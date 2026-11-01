package io.ekodb.client

import io.ekodb.client.admin.*
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.*
import io.ktor.http.*
import io.ktor.http.content.TextContent
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import java.io.File
import kotlin.test.*

class AdminPermissionResourcesTest {
    private val headers = headersOf(HttpHeaders.ContentType, "application/json")
    private val fixture = Json.parseToJsonElement(File("../test-fixtures/admin-contract-0.80.0.json").readText()).jsonObject
    private fun edit(value: JsonElement?): PermissionEdit = when (value) {
        null -> PermissionEdit.Keep
        JsonNull -> PermissionEdit.Clear
        else -> PermissionEdit.Set(value.jsonArray.map { it.jsonPrimitive.content })
    }

    @Test fun modernFacetRequestsAndChangesMatchSharedFixture() = runBlocking<Unit> {
        for (caseValue in fixture.getValue("permission_changes").jsonArray) {
            val case = caseValue.jsonObject
            val body = case.getValue("body").takeUnless { it == JsonNull }?.jsonObject
            val response = case.getValue("response").jsonObject
            var probes = 0
            var changes = 0
            val http = HttpClient(MockEngine { request ->
                when (request.url.encodedPath) {
                    "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                    "/api/auth/keys/me/permissions" -> { probes++; respond(fixture.getValue("permissions").toString(), headers = headers) }
                    case.getValue("path").jsonPrimitive.content -> {
                        changes++
                        assertEquals(case.getValue("method").jsonPrimitive.content, request.method.value)
                        if (body != null) assertEquals(body, Json.parseToJsonElement((request.body as TextContent).text))
                        respond(response.toString(), headers = headers)
                    }
                    else -> error("Unexpected route")
                }
            })
            val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
            try {
                assertTrue(admin.usesKeyPermissionRoutes())
                assertTrue(admin.usesKeyPermissionRoutes())
                val keyId = fixture.getValue("key_id").jsonPrimitive.content
                val name = case.getValue("name").jsonPrimitive.content
                val method = case.getValue("method").jsonPrimitive.content
                val actual: PermissionChange<*> = if (case.getValue("facet").jsonPrimitive.content == "collections") {
                    when (method) {
                        "PUT" -> admin.putApiKeyCollectionPermissions(UpdateApiKeyCollectionPermissionsRequest(keyId, name, body!!.getValue("read").jsonPrimitive.boolean, body.getValue("write").jsonPrimitive.boolean, edit(body["select_fields"]), edit(body["exclude_fields"]), edit(body["writable_fields"])))
                        "PATCH" -> admin.patchApiKeyCollectionPermissions(PatchApiKeyCollectionPermissionsRequest(keyId, name, body?.get("read")?.jsonPrimitive?.boolean, body?.get("write")?.jsonPrimitive?.boolean, edit(body?.get("select_fields")), edit(body?.get("exclude_fields")), edit(body?.get("writable_fields"))))
                        else -> admin.deleteApiKeyCollectionPermissions(keyId, name)
                    }
                } else {
                    when (method) {
                        "PUT" -> admin.putApiKeyKvPermissions(UpdateApiKeyKvPermissionsRequest(keyId, name, body!!.getValue("read").jsonPrimitive.boolean, body.getValue("write").jsonPrimitive.boolean))
                        "PATCH" -> admin.patchApiKeyKvPermissions(PatchApiKeyKvPermissionsRequest(keyId, name, body?.get("read")?.jsonPrimitive?.boolean, body?.get("write")?.jsonPrimitive?.boolean))
                        else -> admin.deleteApiKeyKvPermissions(keyId, name)
                    }
                }
                fun raw(value: Any?): JsonElement = when (value) {
                    null -> JsonNull
                    is CollectionPermissions -> value.metadata
                    is KvPermissions -> value.metadata
                    else -> error("Unexpected grant")
                }
                assertEquals(response["before"], raw(actual.before))
                assertEquals(response["after"], raw(actual.after))
                assertEquals(1, probes)
                assertEquals(1, changes)
            } finally {
                admin.close(); http.close()
            }
        }
    }

    @Test fun registrationAndListingExposeIdsAndModernPermissionsPreserveKvAndLimits() = runBlocking<Unit> {
        val http = HttpClient(MockEngine { request ->
            when (request.url.encodedPath) {
                "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                "/api/auth/register" -> respond(fixture.getValue("key").toString(), headers = headers)
                "/api/auth/keys" -> respond(JsonArray(listOf(fixture.getValue("key"))).toString(), headers = headers)
                "/api/auth/keys/me/permissions" -> respond(fixture.getValue("permissions").toString(), headers = headers)
                else -> error("Unexpected route")
            }
        })
        val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
        try {
            assertEquals(fixture.getValue("key_id").jsonPrimitive.content, admin.createApiKey().id)
            assertEquals(fixture.getValue("key_id").jsonPrimitive.content, admin.listApiKeys().single().id)
            val permissions = admin.getPermissions()
            assertFailsWith<IllegalArgumentException> { admin.putApiKeyKvPermissions(UpdateApiKeyKvPermissionsRequest("me", "session:", false, false)) }
            assertFalse(permissions.isAdmin)
            assertEquals("session:", permissions.kvPermissions.single().prefix)
            assertEquals(listOf("name"), permissions.collectionPermissions.getValue("tenant:a/").selectFields)
            assertFailsWith<IllegalArgumentException> { admin.getApiKeyPermissions("synthetic-target-key-0123456789-abcdef") }
            assertFailsWith<IllegalArgumentException> { admin.legacyUpdateApiKeyCollectionPermissions(LegacyUpdateApiKeyCollectionPermissionsRequest("synthetic-target-key-0123456789-abcdef", "users", true, false)) }
        } finally {
            admin.close(); http.close()
        }
    }

    @Test fun fallbackIsRestrictedTo404And405AndFailedProbesAreRetried() = runBlocking<Unit> {
        for (status in listOf(403, 404, 405, 500, 503)) {
            var probes = 0
            var legacyReads = 0
            val http = HttpClient(MockEngine { request ->
                when (request.url.encodedPath) {
                    "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                    "/api/auth/keys/me/permissions" -> { probes++; respond("{}", HttpStatusCode.fromValue(status), headers) }
                    "/api/auth/permissions" -> { legacyReads++; respond("""{"is_admin":false,"collection_permissions":{},"kv_permissions":null}""", headers = headers) }
                    else -> error("Unexpected route")
                }
            })
            val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
            try {
                repeat(2) {
                    if (status == 404 || status == 405) {
                        assertFalse(admin.getPermissions().isAdmin)
                    } else {
                        assertFailsWith<IllegalStateException> { admin.getPermissions() }
                    }
                }
                assertEquals(if (status == 404 || status == 405) 1 else 2, probes)
                assertEquals(if (status == 404 || status == 405) 2 else 0, legacyReads)
            } finally {
                admin.close(); http.close()
            }
        }
    }

    @Test fun nullableRegistrationKvGrantsArePreserved() = runBlocking<Unit> {
        val record = JsonObject(fixture.getValue("key").jsonObject + ("kv_permissions" to JsonNull))
        val http = HttpClient(MockEngine { request ->
            when (request.url.encodedPath) {
                "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                "/api/auth/register" -> respond(record.toString(), headers = headers)
                "/api/auth/keys" -> respond(JsonArray(listOf(record)).toString(), headers = headers)
                else -> error("Unexpected route")
            }
        })
        val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
        try {
            assertNull(admin.createApiKey().kvPermissions)
            assertNull(admin.listApiKeys().single().kvPermissions)
        } finally {
            admin.close(); http.close()
        }
    }

    @Test fun dotSegmentFacetsFailBeforeHttp() = runBlocking<Unit> {
        var requests = 0
        val http = HttpClient(MockEngine {
            requests++
            error("Unexpected HTTP request")
        })
        val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
        try {
            for (name in listOf(".", "..")) {
                assertFailsWith<IllegalArgumentException> { admin.getApiKeyCollectionPermissions("me", name) }
                assertFailsWith<IllegalArgumentException> { admin.deleteApiKeyCollectionPermissions("me", name) }
                assertFailsWith<IllegalArgumentException> { admin.getApiKeyKvPermissions("me", name) }
                assertFailsWith<IllegalArgumentException> { admin.deleteApiKeyKvPermissions("me", name) }
                assertFailsWith<IllegalArgumentException> { admin.putApiKeyCollectionPermissions(UpdateApiKeyCollectionPermissionsRequest("me", name, true, false)) }
                assertFailsWith<IllegalArgumentException> { admin.patchApiKeyCollectionPermissions(PatchApiKeyCollectionPermissionsRequest("me", name, read = true)) }
                assertFailsWith<IllegalArgumentException> { admin.putApiKeyKvPermissions(UpdateApiKeyKvPermissionsRequest("me", name, true, false)) }
                assertFailsWith<IllegalArgumentException> { admin.patchApiKeyKvPermissions(PatchApiKeyKvPermissionsRequest("me", name, read = true)) }
            }
            assertEquals(0, requests)
        } finally {
            admin.close(); http.close()
        }
    }

    @Test fun facetReadsAndUpdateAliasUsePublicHttpRoutes() = runBlocking<Unit> {
        val id = fixture.getValue("key_id").jsonPrimitive.content
        val root = "/api/auth/keys/$id/permissions"
        val grant = fixture.getValue("permissions").jsonObject.getValue("collections").jsonObject.getValue("tenant:a/")
        val kvs = fixture.getValue("permissions").jsonObject.getValue("kv")
        val http = HttpClient(MockEngine { request ->
            when (request.url.encodedPath) {
                "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                "/api/auth/keys/me/permissions" -> respond(fixture.getValue("permissions").toString(), headers = headers)
                "$root/collections" -> respond(buildJsonObject { put("tenant:a/", grant) }.toString(), headers = headers)
                "$root/collections/tenant%3Aa%2F" -> respond(grant.toString(), headers = headers)
                "$root/kv" -> {
                    assertEquals("prefix=tenant%3Aa%2F%20%2B", request.url.encodedQuery)
                    respond(kvs.toString(), headers = headers)
                }
                "$root/kv/session%3A" -> respond(kvs.jsonArray[0].toString(), headers = headers)
                "$root/collections/users" -> {
                    assertEquals(HttpMethod.Patch, request.method)
                    assertEquals(Json.parseToJsonElement("""{"read":true,"write":false,"select_fields":null}"""), Json.parseToJsonElement((request.body as TextContent).text))
                    respond(buildJsonObject { put("before", JsonNull); put("after", grant); put("future_revision", 3) }.toString(), headers = headers)
                }
                else -> error("Unexpected route")
            }
        })
        val admin = EkoDBAdminClient("https://example.test", "synthetic-admin-key-0123456789-abcdef", httpClient = http)
        try {
            assertTrue(admin.listApiKeyCollectionPermissions(id).getValue("tenant:a/").read)
            assertTrue(admin.getApiKeyCollectionPermissions(id, "tenant:a/").read)
            assertEquals("session:", admin.listApiKeyKvPermissions(id, "tenant:a/ +").single().prefix)
            assertTrue(admin.getApiKeyKvPermissions(id, "session:").read)
            val change = admin.updateApiKeyCollectionPermissions(UpdateApiKeyCollectionPermissionsRequest(id, "users", true, false, selectFields = PermissionEdit.Clear))
            assertEquals(3, change.metadata.getValue("future_revision").jsonPrimitive.int)
        } finally {
            admin.close(); http.close()
        }
    }
}
