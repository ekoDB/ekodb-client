package io.ekodb.client

import io.ekodb.client.admin.*
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.HttpRequestRetry
import io.ktor.http.*
import io.ktor.http.content.TextContent
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import java.io.File
import kotlin.test.*

class AdminContractTest {
    private val headers = headersOf(HttpHeaders.ContentType, "application/json")
    private suspend fun withAdmin(engine: MockEngine, action: suspend (EkoDBAdminClient) -> Unit) {
        val http = HttpClient(engine)
        val admin = EkoDBAdminClient("https://example.test", "admin-secret", httpClient = http)
        try {
            action(admin)
        } finally {
            admin.close(); http.close()
        }
    }
    private fun edit(value: JsonElement?): PermissionEdit = when (value) {
        null -> PermissionEdit.Keep
        JsonNull -> PermissionEdit.Clear
        else -> PermissionEdit.Set(value.jsonArray.map { it.jsonPrimitive.content })
    }

    @Test fun recordedThreeStateContract() = runBlocking {
        val fixture = Json.parseToJsonElement(File("../test-fixtures/admin-contract-0.79.2.json").readText()).jsonObject
        assertEquals(MIN_SERVER_VERSION_FOR_PERMISSION_EDITS, fixture["minimum_server_version"]!!.jsonPrimitive.content)
        for (case in fixture["permission_edits"]!!.jsonArray) {
            val body = case.jsonObject["request"]!!.jsonObject
            val response = case.jsonObject["response"]!!.jsonObject
            var health = 0; var updates = 0
            val engine = MockEngine { request ->
                if (request.url.encodedPath == "/api/auth/token") {
                    respond("""{"token":"jwt-secret"}""", headers = headers)
                } else {
                    assertEquals("Bearer jwt-secret", request.headers[HttpHeaders.Authorization])
                    when (request.url.encodedPath) {
                        "/api/health" -> { health++; respond("""{"version":"0.79.2"}""", headers = headers) }
                        "/api/auth/permissions" -> {
                            updates++; assertEquals(HttpMethod.Put, request.method)
                            assertEquals(body, Json.parseToJsonElement((request.body as TextContent).text))
                            respond(response.toString(), headers = headers)
                        }
                        else -> error("Unexpected route")
                    }
                }
            }
            withAdmin(engine) { admin ->
                val input = UpdateApiKeyCollectionPermissionsRequest("target-secret", "users", body["read"]!!.jsonPrimitive.boolean, body["write"]!!.jsonPrimitive.boolean, edit(body["select_fields"]), edit(body["exclude_fields"]), edit(body["writable_fields"]))
                repeat(2) {
                    val actual = admin.updateApiKeyCollectionPermissions(input).collectionPermissions.getValue("users")
                    val expected = response["collection_permissions"]!!.jsonObject["users"]!!.jsonObject
                    assertEquals(expected["read"]!!.jsonPrimitive.boolean, actual.read)
                    assertEquals(expected["write"]!!.jsonPrimitive.boolean, actual.write)
                    fun fields(name: String): List<String>? = expected[name].takeUnless { it == JsonNull }?.jsonArray?.map { it.jsonPrimitive.content }
                    assertEquals(fields("select_fields"), actual.selectFields)
                    assertEquals(fields("exclude_fields"), actual.excludeFields)
                    assertEquals(fields("writable_fields"), actual.writableFields)
                }
            }
            assertEquals(1, health); assertEquals(2, updates)
        }
    }

    @Test fun operationsDefaultsLegacyMetadataAndEncodedRevoke() = runBlocking {
        val record = """{"key":"new-secret","is_admin":false,"collection_permissions":{"users":["read"]},"label":"disposable","ripple_warning":"Incomplete propagation","future_policy":{"flag":true}}"""
        val paths = mutableListOf<String>()
        val engine = MockEngine { request ->
            paths += request.url.encodedPath
            when (request.url.encodedPath) {
                "/api/auth/token" -> respond("""{"token":"jwt-secret"}""", headers = headers)
                "/api/auth/register" -> {
                    assertEquals(Json.parseToJsonElement("""{"api_key":"admin-secret","is_admin":false,"collection_permissions":{}}"""), Json.parseToJsonElement((request.body as TextContent).text))
                    respond(record, headers = headers)
                }
                "/api/auth/keys" -> respond("[$record]", headers = headers)
                "/api/auth/permissions" -> respond(record, headers = headers)
                else -> { assertEquals(HttpMethod.Delete, request.method); respond("", HttpStatusCode.NoContent, headers) }
            }
        }
        withAdmin(engine) { admin ->
            val key = admin.createApiKey()
            assertFalse(key.isAdmin); assertEquals("Incomplete propagation", key.rippleWarning)
            assertNotNull(key.metadata["future_policy"])
            assertTrue(key.collectionPermissions.getValue("users").read)
            assertFalse(key.collectionPermissions.getValue("users").write)
            assertEquals(1, admin.listApiKeys().size)
            assertFalse(admin.getPermissions().isAdmin)
            admin.revokeApiKey("secret/ ?#%+&=雪")
        }
        assertEquals("/api/auth/keys/secret%2F%20%3F%23%25%2B%26%3D%E9%9B%AA", paths.last())
    }

    @Test fun versionFloorAndFailedReadCache() = runBlocking {
        for (version in listOf("", "0.79.0", "0.79.1", "0.79.2-rc.1", "09.79.2", "0.79.2+", "0.79.2", "0.79.3", "1.0.0")) {
            var health = 0; var puts = 0
            val allowed = version in listOf("0.79.2", "0.79.3", "1.0.0")
            val engine = MockEngine { request ->
                when (request.url.encodedPath) {
                    "/api/auth/token" -> respond("""{"token":"jwt-secret"}""", headers = headers)
                    "/api/health" -> { health++; respond(buildJsonObject { put("version", version) }.toString(), headers = headers) }
                    else -> { puts++; respond("""{"key":"new-secret","is_admin":false,"collection_permissions":{}}""", headers = headers) }
                }
            }
            withAdmin(engine) { admin ->
                repeat(2) {
                    val request = UpdateApiKeyCollectionPermissionsRequest("target-secret", "users", true, false)
                    if (allowed) {
                        admin.updateApiKeyCollectionPermissions(request)
                    } else {
                        assertFailsWith<IllegalArgumentException> { admin.updateApiKeyCollectionPermissions(request) }
                    }
                }
            }
            assertEquals(1, health); assertEquals(if (allowed) 2 else 0, puts)
        }
    }

    @Test fun safeErrorsNoAmbiguousRetryAndBoundedRefresh() = runBlocking {
        for (status in listOf(401, 403, 429, 500, 503, 307)) {
            var calls = 0
            val engine = MockEngine { request ->
                calls++
                if (request.url.encodedPath == "/api/auth/token") {
                    respond("""{"token":"jwt-secret"}""", headers = headers)
                } else {
                    respond("admin-secret jwt-secret target-secret", HttpStatusCode.fromValue(status), headersOf(HttpHeaders.ContentType to listOf("application/json"), HttpHeaders.Location to listOf("https://elsewhere.test/")))
                }
            }
            withAdmin(engine) { admin ->
                val error = assertFailsWith<IllegalStateException> { admin.createApiKey() }
                assertFalse(error.toString().contains("secret")); assertNull(error.cause)
                assertTrue(error.message!!.contains(status.toString()))
            }
            assertEquals(if (status == 401) 4 else 2, calls)
        }
    }

    @Test fun invalidInputsAndMalformedResponseAreSafe() = runBlocking {
        var calls = 0
        val engine = MockEngine { request ->
            calls++
            if (request.url.encodedPath == "/api/auth/token") {
                respond("""{"token":"jwt-secret"}""", headers = headers)
            } else {
                respond("target-secret not JSON", headers = headers)
            }
        }
        withAdmin(engine) { admin ->
            assertFailsWith<IllegalArgumentException> { admin.updateApiKeyCollectionPermissions(UpdateApiKeyCollectionPermissionsRequest("target-secret", "users", true, false, PermissionEdit.Set(listOf(" ")))) }
            assertEquals(0, calls)
            val error = assertFailsWith<IllegalStateException> { admin.listApiKeys() }
            assertFalse(error.toString().contains("secret")); assertNull(error.cause)
            assertEquals(2, calls)
        }
    }

    @Test fun injectedRetryPluginCannotReplayRegistration() = runBlocking {
        var registrations = 0
        val engine = MockEngine { request ->
            if (request.url.encodedPath == "/api/auth/token") {
                respond("""{"token":"jwt-secret"}""", headers = headers)
            } else { registrations++; respond("failed", HttpStatusCode.ServiceUnavailable, headers) }
        }
        val http = HttpClient(engine) { install(HttpRequestRetry) { retryOnServerErrors(maxRetries = 2); delayMillis { 0 } } }
        val admin = EkoDBAdminClient("https://example.test", "admin-secret", httpClient = http)
        try {
            assertFailsWith<IllegalStateException> { admin.createApiKey() }; assertEquals(1, registrations)
        } finally {
            admin.close(); http.close()
        }
    }
}
