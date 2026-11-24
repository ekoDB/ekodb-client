package io.ekodb.client

import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.ParameterDefinition
import io.ekodb.client.functions.UserFunction
import io.ktor.client.HttpClient
import io.ktor.client.engine.mock.MockEngine
import io.ktor.client.engine.mock.respond
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.request.HttpRequestData
import io.ktor.http.ContentType
import io.ktor.http.HttpHeaders
import io.ktor.http.HttpStatusCode
import io.ktor.http.content.TextContent
import io.ktor.http.headersOf
import io.ktor.serialization.kotlinx.json.json
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import kotlin.test.Test
import kotlin.test.assertEquals

class FunctionParametersTest {
    private fun client(recorded: MutableList<HttpRequestData>): Pair<EkoDBClient, HttpClient> {
        val http = HttpClient(MockEngine { request ->
            val body = if (request.url.encodedPath == "/api/auth/token") {
                """{"token":"synthetic-token"}"""
            } else {
                recorded.add(request)
                """{"id":"function-id"}"""
            }
            respond(body, HttpStatusCode.OK, headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString()))
        }) {
            install(ContentNegotiation) { json(Json { ignoreUnknownKeys = true }) }
        }
        return EkoDBClient.builder()
            .baseUrl("https://example.test")
            .apiKey("synthetic-api-key-0123456789")
            .httpClient(http)
            .build() to http
    }

    private fun body(request: HttpRequestData): JsonElement =
        Json.parseToJsonElement((request.body as TextContent).text)

    @Suppress("DEPRECATION")
    @Test
    fun typedSaveAndUpdateOmitLegacyParameterType() = runBlocking {
        val recorded = mutableListOf<HttpRequestData>()
        val (client, http) = client(recorded)
        val definition = UserFunction(
            label = "parameter_contract",
            name = "Parameter contract",
            version = "2.0",
            parameters = linkedMapOf(
                "page_size" to ParameterDefinition(default = JsonPrimitive(5), description = "Page size", paramType = "Integer"),
                "page_offset" to ParameterDefinition(default = JsonPrimitive(0), paramType = "Integer"),
                "nullable" to ParameterDefinition(default = JsonNull, paramType = "String"),
                "payload" to ParameterDefinition(default = buildJsonObject { put("param_type", "business-data") }, paramType = "Object"),
                "required_value" to ParameterDefinition(required = true),
                "optional_value" to ParameterDefinition(),
            ),
            functions = listOf(FunctionStageConfig.FindAll("synthetic_rows")),
            tags = listOf("contract"),
        )
        val expected = Json.parseToJsonElement("""{
            "label":"parameter_contract","name":"Parameter contract","version":"2.0",
            "parameters":{
                "page_size":{"default":5,"description":"Page size"},
                "page_offset":{"default":0},
                "nullable":{"default":null},
                "payload":{"default":{"param_type":"business-data"}},
                "required_value":{"required":true},"optional_value":{}
            },
            "functions":[{"type":"FindAll","collection":"synthetic_rows"}],"tags":["contract"]
        }""")
        try {
            assertEquals("function-id", client.saveFunction(definition))
            client.updateFunction("function-id", definition)
            assertEquals(listOf("POST /api/functions", "PUT /api/functions/function-id"), recorded.map { "${it.method.value} ${it.url.encodedPath}" })
            recorded.forEach { assertEquals(expected, body(it)) }
            assertEquals("Integer", definition.parameters.getValue("page_size").paramType)
        } finally {
            client.close(); http.close()
        }
    }

    @Test fun rawSaveAndUpdateOmitOnlyLegacyParameterMetadata() = runBlocking {
        val recorded = mutableListOf<HttpRequestData>()
        val (client, http) = client(recorded)
        val expected = Json.parseToJsonElement("""{
            "label":"raw_parameters","name":"Raw parameters",
            "parameters":{
                "page":{"required":false,"default":0,"description":"Offset"},
                "nullable":{"required":false,"default":null},
                "payload":{"default":{"param_type":"business-data"}},"optional":{}
            },
            "functions":[{"type":"FindAll","collection":"synthetic_rows"}]
        }""").jsonObject
        val parameters = expected.getValue("parameters").jsonObject
        val input = JsonObject(expected + ("parameters" to JsonObject(parameters + (
            "page" to JsonObject(parameters.getValue("page").jsonObject + ("param_type" to JsonPrimitive("Integer")))
        ))))
        try {
            assertEquals("function-id", client.saveUserFunction(input))
            client.updateUserFunction("raw_parameters", input)
            assertEquals(listOf("POST /api/functions", "PUT /api/functions/raw_parameters"), recorded.map { "${it.method.value} ${it.url.encodedPath}" })
            recorded.forEach { assertEquals(expected, body(it)) }
            assertEquals(JsonPrimitive("Integer"), input.getValue("parameters").jsonObject.getValue("page").jsonObject["param_type"])
        } finally {
            client.close(); http.close()
        }
    }
}
