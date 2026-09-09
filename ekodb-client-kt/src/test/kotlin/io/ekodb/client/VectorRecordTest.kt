package io.ekodb.client

import io.ekodb.client.types.*
import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.http.*
import io.ktor.http.content.TextContent
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.test.runTest
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.*
import java.io.File
import kotlin.test.*

class VectorRecordTest {
    private val vector = listOf(1.0, 0.0, 0.0)
    private fun record() = EkoRecord().insert("label", "vector-golden").insert("count", 7)
        .insert("eligible", true).insert("ordinary", FieldType.array(vector.map { FieldType.float(it) }))
        .insert("embedding", FieldType.vector(vector))
    private fun golden() = Json.parseToJsonElement(File("../test-fixtures/vector-record.json").readText())

    @Test
    fun `vector record preserves its type instead of the historical plain array`() {
        // Reproduced before changing the serializer: expected the tagged value,
        // but RecordSerializer -> FieldTypeSerializer emitted {"embedding":[1.0,0.0,0.0]}.
        assertEquals(golden(), Json.encodeToJsonElement(record()))
    }

    @Test
    fun `ordinary array and vector round trip distinctly`() {
        val decoded = Json.decodeFromJsonElement<EkoRecord>(golden())
        assertEquals(record(), decoded)
        assertIs<FieldType.ArrayValue>(decoded["ordinary"])
        assertIs<FieldType.VectorValue>(decoded["embedding"])
        assertEquals(JsonArray(vector.map { JsonPrimitive(it) }), Json.encodeToJsonElement(vector))
        assertEquals(JsonArray(vector.map { JsonPrimitive(it) }), Json.encodeToJsonElement(vector.toTypedArray()))
    }

    @Test
    fun `nested objects and arrays retain vectors`() {
        val nested: FieldType = FieldType.obj(mapOf("items" to FieldType.array(listOf(FieldType.vector(vector)))))
        assertEquals(nested, Json.decodeFromJsonElement<FieldType>(Json.encodeToJsonElement(nested)))
    }

    @Test
    fun `empty vector retains tag and round trips`() {
        // The live unconstrained Vector schema accepts this value; no useful
        // cosine-search behavior for zero-dimensional vectors is implied.
        val empty: FieldType = FieldType.vector(emptyList())
        val wire = Json.parseToJsonElement("""{"type":"Vector","value":[]}""")
        assertEquals(wire, Json.encodeToJsonElement(empty))
        assertEquals(empty, Json.decodeFromJsonElement<FieldType>(wire))
    }

    @Test
    fun `default JSON encoder still rejects nonfinite vector coordinates`() {
        for (invalid in listOf(Double.NaN, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY)) {
            assertFailsWith<SerializationException> {
                Json.encodeToJsonElement<FieldType>(FieldType.vector(listOf(invalid)))
            }
        }
    }

    @Test
    fun `invalid tagged vector contents fail decoding`() {
        for (value in listOf("[true]", "[null]", "[{}]", "null", "{}")) {
            assertFailsWith<SerializationException> {
                Json.decodeFromString<FieldType>("""{"type":"Vector","value":$value}""")
            }
        }
    }

    @Test
    fun `unknown envelopes and extra object fields are preserved`() {
        for (wire in listOf(
            """{"type":"FutureVector","value":[1],"future":true}""",
            """{"type":"Vector","value":[1],"future":true}""",
            """{"future":{"embedding":{"type":"Vector","value":[1.0]}}}"""
        )) {
            val value = Json.parseToJsonElement(wire)
            assertEquals(value, Json.encodeToJsonElement(Json.decodeFromJsonElement<FieldType>(value)))
        }
    }

    private fun client(handler: MockRequestHandleScope.(io.ktor.client.request.HttpRequestData) -> io.ktor.client.request.HttpResponseData): EkoDBClient {
        val engine = MockEngine { request ->
            if (request.url.encodedPath == "/api/auth/token") {
                respond("""{"token":"test-token"}""", headers = headersOf(HttpHeaders.ContentType, "application/json"))
            } else {
                handler(request)
            }
        }
        return EkoDBClient.builder().baseUrl("http://localhost").apiKey("test-key").maxRetries(3)
            .httpClient(HttpClient(engine) { install(ContentNegotiation) { json() } }).build()
    }

    @Test
    fun `insert update and both upsert branches send complete golden body`() = runTest {
        val paths = mutableListOf<String>()
        val client = client { request ->
            assertEquals(golden(), Json.parseToJsonElement((request.body as TextContent).text))
            assertEquals(ContentType.Application.Json, request.body.contentType?.withoutParameters())
            paths.add("${request.method.value} ${request.url.encodedPath}")
            if (request.url.encodedPath.endsWith("/missing")) {
                respond("not found", HttpStatusCode.NotFound)
            } else {
                respond(golden().toString(), headers = headersOf(HttpHeaders.ContentType, "application/json"))
            }
        }
        try {
            assertEquals(record(), client.insert("vectors", record()))
            assertEquals(record(), client.update("vectors", "existing", record()))
            assertEquals(record(), client.upsert("vectors", "existing", record()))
            assertEquals(record(), client.upsert("vectors", "missing", record()))
            assertEquals(listOf("POST /api/insert/vectors", "PUT /api/update/vectors/existing",
                "PUT /api/update/vectors/existing", "PUT /api/update/vectors/missing", "POST /api/insert/vectors"), paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `bad array insert preserves 400 body without retry`() = runTest {
        val body = """{"error":"Type mismatch for field 'embedding': expected Vector, got Array"}"""
        var calls = 0
        val client = client { request ->
            calls++
            assertIs<JsonArray>(Json.parseToJsonElement((request.body as TextContent).text).jsonObject["embedding"])
            respond(body, HttpStatusCode.BadRequest, headers = headersOf(HttpHeaders.ContentType, "application/json"))
        }
        try {
            val bad = EkoRecord().insert("embedding", FieldType.array(vector.map { FieldType.float(it) }))
            val error = assertFailsWith<EkoDBHttpException> { client.insert("vectors", bad) }
            assertEquals(400, error.statusCode)
            assertEquals(body, error.responseBody)
            assertEquals(1, calls)
        } finally {
            client.close()
        }
    }

    @Test
    fun `search vector stays a numeric array`() = runTest {
        val client = client { request ->
            assertEquals("/api/search/vectors", request.url.encodedPath)
            assertEquals(Json.parseToJsonElement("""{"query":"","vector":[1.0,0.0,0.0],"vector_field":"embedding","vector_metric":"cosine"}"""),
                Json.parseToJsonElement((request.body as TextContent).text))
            respond("""{"results":[],"total":0}""", headers = headersOf(HttpHeaders.ContentType, "application/json"))
        }
        try {
            client.search("vectors", SearchQuery(vector = vector, vectorField = "embedding", vectorMetric = DistanceMetric.COSINE))
        } finally {
            client.close()
        }
    }
}
