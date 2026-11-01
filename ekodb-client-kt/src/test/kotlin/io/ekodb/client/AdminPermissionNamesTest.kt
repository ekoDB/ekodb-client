package io.ekodb.client

import io.ekodb.client.admin.*
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.*
import io.ktor.http.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import java.io.File
import kotlin.test.*

class AdminPermissionNamesTest {
    private val headers = headersOf(HttpHeaders.ContentType, "application/json")
    private val fixture = Json.parseToJsonElement(File("../test-fixtures/admin-permission-names.json").readText()).jsonObject
    private val operations = listOf(
        "collections" to HttpMethod.Get,
        "collections" to HttpMethod.Put,
        "collections" to HttpMethod.Patch,
        "collections" to HttpMethod.Delete,
        "kv" to HttpMethod.Get,
        "kv" to HttpMethod.Put,
        "kv" to HttpMethod.Patch,
        "kv" to HttpMethod.Delete,
    )

    private suspend fun invoke(admin: EkoDBAdminClient, index: Int, name: String) {
        when (index) {
            0 -> admin.getApiKeyCollectionPermissions("me", name)
            1 -> admin.putApiKeyCollectionPermissions(UpdateApiKeyCollectionPermissionsRequest("me", name, true, false))
            2 -> admin.patchApiKeyCollectionPermissions(PatchApiKeyCollectionPermissionsRequest("me", name, read = true))
            3 -> admin.deleteApiKeyCollectionPermissions("me", name)
            4 -> admin.getApiKeyKvPermissions("me", name)
            5 -> admin.putApiKeyKvPermissions(UpdateApiKeyKvPermissionsRequest("me", name, true, false))
            6 -> admin.patchApiKeyKvPermissions(PatchApiKeyKvPermissionsRequest("me", name, read = true))
            7 -> admin.deleteApiKeyKvPermissions("me", name)
        }
    }

    @Test fun acceptedFacetNamesReachEveryPublicHttpOperation() = runBlocking<Unit> {
        for (case in fixture.getValue("accepted").jsonArray) {
            val label = case.jsonObject.getValue("name").jsonPrimitive.content
            val name = case.jsonObject.getValue("value").jsonPrimitive.content
            val encoded = name.toByteArray(Charsets.UTF_8).joinToString("") { byte ->
                val value = byte.toInt() and 255
                if (value in 65..90 || value in 97..122 || value in 48..57 || value.toChar() in "-._~") value.toChar().toString() else "%%%02X".format(value)
            }
            var index = 0
            var facets = 0
            val http = HttpClient(MockEngine { request ->
                when (request.url.encodedPath) {
                    "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                    "/api/auth/keys/me/permissions" -> respond("""{"admin":false,"collections":{},"kv":[]}""", headers = headers)
                    else -> {
                        val (facet, method) = operations[index]
                        assertEquals("/api/auth/keys/me/permissions/$facet/$encoded", request.url.encodedPath, label)
                        assertEquals(method, request.method, label)
                        facets++
                        val grant = """{"read":true,"write":false}"""
                        respond(if (method == HttpMethod.Get) grant else """{"before":null,"after":$grant}""", headers = headers)
                    }
                }
            })
            val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
            try {
                for (operation in operations.indices) {
                    index = operation
                    invoke(admin, operation, name)
                }
                assertEquals(operations.size, facets, label)
            } finally {
                admin.close(); http.close()
            }
        }
    }

    @Test fun rejectedFacetNamesFailEveryPublicOperationBeforeHttp() = runBlocking<Unit> {
        var requests = 0
        val http = HttpClient(MockEngine {
            requests++
            error("Unexpected HTTP request")
        })
        val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
        try {
            for (case in fixture.getValue("rejected").jsonArray) {
                val label = case.jsonObject.getValue("name").jsonPrimitive.content
                val name = case.jsonObject.getValue("value").jsonPrimitive.content
                for (operation in operations.indices) {
                    assertFailsWith<IllegalArgumentException>("$label operation $operation") { invoke(admin, operation, name) }
                }
            }
            assertEquals(0, requests)
        } finally {
            admin.close(); http.close()
        }
    }

    @Test fun malformedSurrogatesFailEveryPublicOperationBeforeHttp() = runBlocking<Unit> {
        val cases = listOf(
            "lone high" to "\uD800",
            "lone low" to "\uDC00",
            "embedded high" to "pre\uD800post",
            "embedded low" to "pre\uDC00post",
            "reversed pair" to "\uDC00\uD800",
            "two high" to "\uD800\uD800",
        )
        var requests = 0
        val accepted = mutableListOf<String>()
        val encodedPaths = mutableListOf<String>()
        val http = HttpClient(MockEngine { request ->
            requests++
            when (request.url.encodedPath) {
                "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                "/api/auth/keys/me/permissions" -> respond("""{"admin":false,"collections":{},"kv":[]}""", headers = headers)
                else -> {
                    encodedPaths += request.url.encodedPath
                    val grant = """{"read":true,"write":false}"""
                    respond(if (request.method == HttpMethod.Get) grant else """{"before":null,"after":$grant}""", headers = headers)
                }
            }
        })
        val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
        try {
            for ((label, name) in cases) {
                for (operation in operations.indices) {
                    val failure = runCatching { invoke(admin, operation, name) }.exceptionOrNull()
                    if (failure !is IllegalArgumentException) accepted += "$label operation $operation"
                }
            }
            assertEquals(emptyList(), accepted, "Malformed names were accepted; paths=$encodedPaths")
            assertEquals(0, requests)
        } finally {
            admin.close(); http.close()
        }
    }

    @Test fun replacementCharacterAndSupplementaryPairsRemainDistinctAtHttpBoundary() = runBlocking<Unit> {
        val cases = listOf("\uFFFD" to "%EF%BF%BD", "\uD83D\uDE00" to "%F0%9F%98%80")
        for ((name, encoded) in cases) {
            var index = 0
            var facets = 0
            val http = HttpClient(MockEngine { request ->
                when (request.url.encodedPath) {
                    "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                    "/api/auth/keys/me/permissions" -> respond("""{"admin":false,"collections":{},"kv":[]}""", headers = headers)
                    else -> {
                        val (facet, method) = operations[index]
                        assertEquals("/api/auth/keys/me/permissions/$facet/$encoded", request.url.encodedPath)
                        assertEquals(method, request.method)
                        facets++
                        val grant = """{"read":true,"write":false}"""
                        respond(if (method == HttpMethod.Get) grant else """{"before":null,"after":$grant}""", headers = headers)
                    }
                }
            })
            val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
            try {
                for (operation in operations.indices) {
                    index = operation
                    invoke(admin, operation, name)
                }
                assertEquals(operations.size, facets)
            } finally {
                admin.close(); http.close()
            }
        }
    }
}
