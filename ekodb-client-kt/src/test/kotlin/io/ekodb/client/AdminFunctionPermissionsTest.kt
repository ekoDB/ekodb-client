package io.ekodb.client

import io.ekodb.client.admin.*
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.Principal
import io.ekodb.client.functions.RunAs
import io.ekodb.client.functions.UserFunction
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.request.HttpRequestData
import io.ktor.http.*
import io.ktor.http.content.TextContent
import io.ktor.serialization.kotlinx.json.json
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import java.io.File
import kotlin.test.*

class AdminFunctionPermissionsTest {
    private val headers = headersOf(HttpHeaders.ContentType, "application/json")
    private val fixture = Json.parseToJsonElement(File("../test-fixtures/admin-function-permissions-contract.json").readText()).jsonObject
    private val keyId = fixture.getValue("key_id").jsonPrimitive.content
    private val adminKey = "synthetic-admin-key-0123456789-abcdef"
    private val modernPermissions = """{"admin":true,"collections":{},"kv":[],"functions":[]}"""

    private fun bodyOf(request: HttpRequestData): JsonElement? = (request.body as? TextContent)?.text?.let(Json::parseToJsonElement)

    /** Answers from [routes] by "METHOD path"; records every request but the token exchange. */
    private fun admin(
        routes: Map<String, Pair<Int, String>>,
        recorded: MutableList<HttpRequestData> = mutableListOf(),
        probe: Pair<Int, String> = 200 to modernPermissions,
    ): Pair<EkoDBAdminClient, HttpClient> {
        val http = HttpClient(MockEngine { request ->
            val path = request.url.encodedPath
            when {
                path == "/api/auth/token" -> respond("""{"token":"jwt-synthetic"}""", headers = headers)
                path == "/api/auth/keys/me/permissions" -> respond(probe.second, HttpStatusCode.fromValue(probe.first), headers)
                else -> {
                    recorded.add(request)
                    val (status, body) = routes["${request.method.value} $path"] ?: (404 to """{"error":"synthetic-secret"}""")
                    respond(body, HttpStatusCode.fromValue(status), headers)
                }
            }
        })
        return EkoDBAdminClient("https://example.test", adminKey, httpClient = http) to http
    }

    private inline fun withAdmin(
        routes: Map<String, Pair<Int, String>>,
        recorded: MutableList<HttpRequestData> = mutableListOf(),
        probe: Pair<Int, String> = 200 to modernPermissions,
        block: (EkoDBAdminClient) -> Unit,
    ) {
        val (client, http) = admin(routes, recorded, probe)
        try {
            block(client)
        } finally {
            client.close(); http.close()
        }
    }

    private fun patterns(grants: List<FunctionPermissions>?) = grants?.map { it.pattern }

    @Test fun registrationSendsFunctionPermissionsAndReadsTheEchoedMap() = runBlocking<Unit> {
        val recorded = mutableListOf<HttpRequestData>()
        withAdmin(mapOf("POST /api/auth/register" to (200 to fixture.getValue("registration_response").toString())), recorded) { client ->
            val key = runBlocking {
                client.createApiKey(CreateApiKeyRequest(
                    label = "report-runner",
                    functionPermissions = mapOf(
                        "report_*" to listOf(FunctionAction.Call),
                        "nightly_close" to listOf(FunctionAction.Call),
                        "audit_only" to emptyList(),
                    ),
                ))
            }
            assertEquals(fixture.getValue("registration_request"), bodyOf(recorded.single()))
            assertEquals(fixture.getValue("expected_registered_patterns").jsonArray.map { it.jsonPrimitive.content }, patterns(key.functionPermissions))
        }
    }

    @Test fun registrationWithoutGrantsKeepsTheOldBodyAndReadsNull() = runBlocking<Unit> {
        val recorded = mutableListOf<HttpRequestData>()
        val response = fixture.getValue("registration_response_without_function_permissions").toString()
        withAdmin(mapOf("POST /api/auth/register" to (200 to response)), recorded) { client ->
            val key = runBlocking { client.createApiKey(CreateApiKeyRequest(label = "report-runner")) }
            assertFalse(bodyOf(recorded.single())!!.jsonObject.containsKey("function_permissions"))
            assertNull(key.functionPermissions)
        }
    }

    @Test fun registrationRefusesAnInvalidPatternBeforeAnyRequest() = runBlocking<Unit> {
        val recorded = mutableListOf<HttpRequestData>()
        withAdmin(emptyMap(), recorded) { client ->
            for (pattern in listOf("", "re*port", "report_**", "*report")) {
                val error = assertFailsWith<IllegalArgumentException>(pattern) {
                    runBlocking { client.createApiKey(CreateApiKeyRequest(functionPermissions = mapOf(pattern to listOf(FunctionAction.Call)))) }
                }
                assertEquals("Invalid function pattern", error.message)
            }
        }
        assertTrue(recorded.isEmpty())
    }

    @Test fun keyListingsAndPermissionReadsCarryFunctionGrants() = runBlocking<Unit> {
        val grants = fixture.getValue("function_grants").jsonArray.map { it.jsonObject.getValue("pattern").jsonPrimitive.content }
        withAdmin(
            mapOf("GET /api/auth/keys" to (200 to JsonArray(listOf(fixture.getValue("key_list_record"))).toString())),
            probe = 200 to fixture.getValue("permissions").toString(),
        ) { client ->
            runBlocking {
                assertEquals(grants, patterns(client.listApiKeys().single().functionPermissions))
                assertEquals(grants, patterns(client.getPermissions().functionPermissions))
            }
        }
        withAdmin(emptyMap(), probe = 200 to fixture.getValue("permissions_without_functions").toString()) { client ->
            runBlocking { assertEquals(emptyList(), client.getPermissions().functionPermissions) }
        }
    }

    @Test fun listAndGetReadTheFunctionGrantRoutes() = runBlocking<Unit> {
        val recorded = mutableListOf<HttpRequestData>()
        val routes = mapOf(
            "GET /api/auth/keys/$keyId/permissions/functions" to (200 to fixture.getValue("function_grants").toString()),
            "GET /api/auth/keys/$keyId/permissions/functions/report_%2A" to (200 to """{"pattern":"report_*"}"""),
        )
        withAdmin(routes, recorded) { client ->
            runBlocking {
                assertEquals(listOf("nightly_close", "report_*"), patterns(client.listApiKeyFunctionPermissions(keyId)))
                assertEquals("report_*", client.getApiKeyFunctionPermissions(keyId, "report_*").pattern)
                val error = assertFailsWith<IllegalStateException> { client.getApiKeyFunctionPermissions(keyId, "nope") }
                assertEquals("Admin request failed (HTTP 404)", error.message)
            }
        }
    }

    @Test fun grantChangesFollowTheRecordedRoutesBodiesAndReplies() = runBlocking<Unit> {
        for (caseValue in fixture.getValue("function_grant_changes").jsonArray) {
            val case = caseValue.jsonObject
            val method = case.getValue("method").jsonPrimitive.content
            val path = case.getValue("path").jsonPrimitive.content
            val response = case.getValue("response").jsonObject
            val recorded = mutableListOf<HttpRequestData>()
            withAdmin(mapOf("$method $path" to (case.getValue("status").jsonPrimitive.int to response.toString())), recorded) { client ->
                val pattern = case.getValue("pattern").jsonPrimitive.content
                val change = runBlocking {
                    if (method == "PUT") client.putApiKeyFunctionPermissions(keyId, pattern) else client.deleteApiKeyFunctionPermissions(keyId, pattern)
                }
                val sent = recorded.single()
                assertEquals(method, sent.method.value, case["name"].toString())
                assertEquals(path, sent.url.encodedPath, case["name"].toString())
                val expectedBody = case.getValue("body").takeUnless { it == JsonNull }
                assertEquals(expectedBody, bodyOf(sent), case["name"].toString())
                assertEquals(response["before"]?.takeUnless { it == JsonNull }?.jsonObject?.get("pattern")?.jsonPrimitive?.content, change.before?.pattern)
                assertEquals(response["after"]?.takeUnless { it == JsonNull }?.jsonObject?.get("pattern")?.jsonPrimitive?.content, change.after?.pattern)
                assertEquals(response["ripple_warning"], change.metadata["ripple_warning"])
            }
        }
    }

    @Test fun refusedGrantChangesSurfaceTheStatusWithoutTheBody() = runBlocking<Unit> {
        for (caseValue in fixture.getValue("refusals").jsonArray) {
            val case = caseValue.jsonObject
            val method = case.getValue("method").jsonPrimitive.content
            val status = case.getValue("status").jsonPrimitive.int
            val routes = mapOf("$method /api/auth/keys/$keyId/permissions/functions/report_%2A" to (status to """{"error":"synthetic-secret"}"""))
            withAdmin(routes) { client ->
                val error = assertFailsWith<IllegalStateException>(case["name"].toString()) {
                    runBlocking {
                        if (method == "PUT") client.putApiKeyFunctionPermissions(keyId, "report_*") else client.deleteApiKeyFunctionPermissions(keyId, "report_*")
                    }
                }
                assertEquals("Admin request failed (HTTP $status)", error.message)
            }
        }
    }

    @Test fun invalidPatternsFailBeforeAnyRequest() = runBlocking<Unit> {
        val recorded = mutableListOf<HttpRequestData>()
        withAdmin(emptyMap(), recorded) { client ->
            for (caseValue in fixture.getValue("rejected_patterns").jsonArray) {
                val case = caseValue.jsonObject
                val pattern = case["value"]?.jsonPrimitive?.content ?: "a".repeat(case.getValue("length").jsonPrimitive.int)
                runBlocking {
                    for (call in listOf<suspend () -> Unit>(
                        { client.getApiKeyFunctionPermissions(keyId, pattern) },
                        { client.putApiKeyFunctionPermissions(keyId, pattern) },
                        { client.deleteApiKeyFunctionPermissions(keyId, pattern) },
                    )) {
                        val error = assertFailsWith<IllegalArgumentException>(case["name"].toString()) { call() }
                        assertEquals("Invalid function pattern", error.message)
                    }
                }
            }
            assertFailsWith<IllegalArgumentException> { runBlocking { client.putApiKeyFunctionPermissions("synthetic-target-key-0123456789-abcdef", "report_*") } }
        }
        assertTrue(recorded.isEmpty())
    }

    @Test fun theLongestPatternIsAcceptedAndBytesNotCharactersAreCounted() = runBlocking<Unit> {
        assertEquals(fixture.getValue("accepted_pattern_length").jsonPrimitive.int, MAX_FUNCTION_PATTERN_BYTES)
        val pattern = "a".repeat(MAX_FUNCTION_PATTERN_BYTES - 1) + "*"
        val path = "/api/auth/keys/$keyId/permissions/functions/" + "a".repeat(MAX_FUNCTION_PATTERN_BYTES - 1) + "%2A"
        val reply = """{"before":null,"after":{"pattern":"$pattern"}}"""
        withAdmin(mapOf("PUT $path" to (200 to reply))) { client ->
            runBlocking {
                assertEquals(pattern, client.putApiKeyFunctionPermissions(keyId, pattern).after?.pattern)
                assertFailsWith<IllegalArgumentException> { client.putApiKeyFunctionPermissions(keyId, "é".repeat(MAX_FUNCTION_PATTERN_BYTES / 2 + 1)) }
            }
        }
    }

    @Test fun functionGrantRoutesNeedKeyAddressedServers() = runBlocking<Unit> {
        withAdmin(emptyMap(), probe = 404 to "{}") { client ->
            assertFailsWith<IllegalArgumentException> { runBlocking { client.putApiKeyFunctionPermissions("me", "report_*") } }
        }
    }

    // ---- function definitions, refused calls and schedules (the main client) ----

    private fun client(recorded: MutableList<HttpRequestData>, status: Int, body: String): EkoDBClient {
        val engine = MockEngine { request ->
            if (request.url.encodedPath == "/api/auth/token") {
                respond("""{"token":"jwt-synthetic"}""", headers = headers)
            } else {
                recorded.add(request)
                respond(body, HttpStatusCode.fromValue(status), headers)
            }
        }
        val http = HttpClient(engine) { install(ContentNegotiation) { json(Json { ignoreUnknownKeys = true }) } }
        return EkoDBClient.builder().baseUrl("https://example.test").apiKey(adminKey).httpClient(http).build()
    }

    @Test fun typedFunctionsReadRunAsAndCreatedByAndSendRunAs() = runBlocking<Unit> {
        val definition = fixture.getValue("function_definition").toString()
        val read = client(mutableListOf(), 200, definition).getFunction("nightly_close")
        assertEquals(RunAs.Author, read.run_as)
        assertEquals(Principal(keyId, "report-runner", "alice"), read.created_by)

        val listed = client(mutableListOf(), 200, "[$definition, ${fixture.getValue("function_definition_unrecorded")}]").listFunctions()
        assertEquals(listOf(RunAs.Author, null), listed.map { it.run_as })
        assertEquals(listOf(true, false), listed.map { it.created_by != null })

        val recorded = mutableListOf<HttpRequestData>()
        client(recorded, 200, """{"id":"f1"}""").saveFunction(
            UserFunction(label = "nightly_close", name = "Nightly close", functions = listOf(FunctionStageConfig.FindAll(collection = "c")), run_as = RunAs.Caller),
        )
        val sent = bodyOf(recorded.single())!!.jsonObject
        assertEquals("caller", sent.getValue("run_as").jsonPrimitive.content)
        assertFalse(sent.containsKey("created_by"))

        // A run_as other than the two names is a decoding error, never a silent default.
        val invalid = JsonObject(fixture.getValue("function_definition").jsonObject + ("run_as" to JsonPrimitive("root"))).toString()
        assertFails { runBlocking { client(mutableListOf(), 200, invalid).getFunction("nightly_close") } }
    }

    @Test fun rawUserFunctionsKeepTheServerRecordedFields() = runBlocking<Unit> {
        val definition = fixture.getValue("function_definition").jsonObject
        val read = client(mutableListOf(), 200, definition.toString()).getUserFunction("nightly_close")
        assertEquals(definition["created_by"], read["created_by"])
        assertEquals(definition["run_as"], read["run_as"])
        val recorded = mutableListOf<HttpRequestData>()
        client(recorded, 200, """{"id":"f1"}""").saveUserFunction(fixture.getValue("function_save_request").jsonObject)
        assertEquals("caller", bodyOf(recorded.single())!!.jsonObject.getValue("run_as").jsonPrimitive.content)
    }

    @Test fun aRefusedFunctionCallIsTheClientsNormalHttpError() = runBlocking<Unit> {
        val refusal = fixture.getValue("function_call_refusal").jsonObject
        val status = refusal.getValue("status").jsonPrimitive.int
        val error = assertFailsWith<EkoDBHttpException> {
            runBlocking { client(mutableListOf(), status, refusal.getValue("body").toString()).callFunction("nightly_close") }
        }
        assertEquals(403, error.statusCode)
    }

    @Test fun schedulesReturnTheCreatorTheServerRecorded() = runBlocking<Unit> {
        val record = fixture.getValue("schedule_record").jsonObject
        val schedule = client(mutableListOf(), 200, record.toString()).getSchedule(record.getValue("id").jsonPrimitive.content)
        assertEquals(record["created_by"], schedule["created_by"])
    }
}
