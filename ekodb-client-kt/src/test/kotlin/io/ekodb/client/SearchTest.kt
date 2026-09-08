package io.ekodb.client

import io.ekodb.client.types.DistanceMetric
import io.ekodb.client.types.SearchQuery
import io.ekodb.client.types.SearchResponse
import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.http.*
import io.ktor.http.content.TextContent
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.*
import java.io.File
import kotlin.test.*

class SearchTest {
    private val json = Json { ignoreUnknownKeys = true }
    private val vector = listOf(0.1, 0.2, 0.3)
    private val response = """{"results":[{"record":{"id":"r1","_score":0.2,"new_field":{"x":1}},"score":0.95,"matched_fields":["title","body"],"future_hit":true}],"total":1,"execution_time_ms":12,"future_metadata":true}"""
    private fun wire(query: SearchQuery) = json.encodeToJsonElement(query).jsonObject
    private fun expected(body: String) = json.parseToJsonElement(body)

    @Test
    fun `simple text and vector query include query even when empty and omit unset options`() {
        assertEquals(expected("""{"query":"hello"}"""), wire(SearchQuery("hello")))
        assertEquals(expected("""{"query":""}"""), wire(SearchQuery()))
        assertEquals(expected("""{"query":"","vector":[0.1,0.2,0.3]}"""), wire(SearchQuery(vector = vector)))
    }

    @Test
    fun `full text options use canonical snake case and retain false values`() {
        val query = SearchQuery("enginer", language = "english", caseSensitive = false, fuzzy = true,
            minScore = 0.2, fields = "title,body", weights = "title:2.5,body:1.5",
            enableStemming = true, boostExact = false, maxEditDistance = 2, limit = 10)
        assertEquals(expected("""{"query":"enginer","language":"english","case_sensitive":false,"fuzzy":true,"min_score":0.2,"fields":"title,body","weights":"title:2.5,body:1.5","enable_stemming":true,"boost_exact":false,"max_edit_distance":2,"limit":10}"""), wire(query))
    }

    @Test
    fun `all closed metrics serialize and invalid metrics fail decoding`() {
        for ((metric, name) in listOf(DistanceMetric.COSINE to "cosine", DistanceMetric.EUCLIDEAN to "euclidean", DistanceMetric.DOT_PRODUCT to "dotproduct")) {
            assertEquals(JsonPrimitive(name), wire(SearchQuery(vectorMetric = metric))["vector_metric"])
            assertEquals(metric, json.decodeFromJsonElement<SearchQuery>(wire(SearchQuery(vectorMetric = metric))).vectorMetric)
        }
        assertFailsWith<SerializationException> {
            json.decodeFromString<SearchQuery>("""{"query":"","vector_metric":"dot"}""")
        }
    }

    @Test
    fun `full combined request equals manual canonical JSON`() {
        val query = SearchQueryBuilder("hello").language("english").caseSensitive(true).fuzzy(false)
            .minScore(0.1).fields(listOf("title", "body")).weights(linkedMapOf("title" to 2.5, "body" to 1.5))
            .enableStemming(false).boostExact(true).maxEditDistance(1).bypassCache(true).bypassRipple(false)
            .limit(7).vector(vector).vectorField("title_embedding").vectorMetric(DistanceMetric.EUCLIDEAN)
            .vectorK(9).vectorThreshold(0.25).textWeight(0.7).vectorWeight(0.3)
            .selectFields(listOf("title")).excludeFields(listOf("embedding"))
            .filters { eq("category", "ml") }.build()
        assertEquals(expected("""{"query":"hello","language":"english","case_sensitive":true,"fuzzy":false,"min_score":0.1,"fields":"title,body","weights":"title:2.5,body:1.5","enable_stemming":false,"boost_exact":true,"max_edit_distance":1,"bypass_cache":true,"bypass_ripple":false,"limit":7,"vector":[0.1,0.2,0.3],"vector_field":"title_embedding","vector_metric":"euclidean","vector_k":9,"vector_threshold":0.25,"text_weight":0.7,"vector_weight":0.3,"select_fields":["title"],"exclude_fields":["embedding"],"filters":{"type":"Condition","content":{"field":"category","operator":"Eq","value":"ml"}}}"""), wire(query))
    }

    @Test
    fun `filters exactly reuse find expressions including logical combinations`() {
        val configure: QueryBuilder.() -> Unit = {
            eq("category", "ml")
            gte("year", 2025)
            or(listOf(QueryBuilder.condition("status", "Eq", "active"), QueryBuilder.condition("status", "Eq", "draft")))
        }
        val find = QueryBuilder().apply(configure).build()
        assertEquals(find.filter, wire(SearchQueryBuilder().filters(configure).build())["filters"])
        assertEquals(find.filter, wire(SearchQuery(filters = find.filter))["filters"])
        assertFalse(wire(SearchQueryBuilder().filters {}.build()).containsKey("filters"))
    }

    @Test
    fun `builder results are independent and copy collection inputs`() {
        val input = mutableListOf(0.1)
        val builder = SearchQueryBuilder().vector(input).limit(3)
        val first = builder.build()
        input.add(0.2)
        builder.limit(5)
        assertEquals(listOf(0.1), first.vector)
        assertEquals(3, first.limit)
        assertEquals(5, builder.build().limit)
    }

    @Test
    fun `response retains scores matched fields timing and dynamic document fields`() {
        val decoded = json.decodeFromString<SearchResponse>(response)
        assertEquals(1L, decoded.total)
        assertEquals(12L, decoded.executionTimeMs)
        val hit = decoded.results.single()
        assertEquals(0.95, hit.score)
        assertEquals(listOf("title", "body"), hit.matchedFields)
        assertEquals(JsonPrimitive(0.2), hit.record["_score"])
        assertEquals(expected("""{"x":1}"""), hit.record["new_field"])
    }

    @Test
    fun `response permits missing timing and empty results but requires scores`() {
        val decoded = json.decodeFromString<SearchResponse>("""{"results":[],"total":0}""")
        assertNull(decoded.executionTimeMs)
        assertTrue(decoded.results.isEmpty())
        assertFailsWith<SerializationException> {
            json.decodeFromString<SearchResponse>("""{"results":[{"record":{},"matched_fields":[]}],"total":1}""")
        }
    }

    private fun testClient(format: SerializationFormat = SerializationFormat.JSON, handler: MockRequestHandleScope.(io.ktor.client.request.HttpRequestData) -> io.ktor.client.request.HttpResponseData): EkoDBClient {
        val engine = MockEngine { request ->
            if (request.url.encodedPath == "/api/auth/token") {
                respond("""{"token":"test-token"}""", headers = headersOf(HttpHeaders.ContentType, "application/json"))
            } else {
                handler(request)
            }
        }
        return EkoDBClient.builder().baseUrl("http://localhost:8080").apiKey("test")
            .format(format).httpClient(HttpClient(engine) { install(ContentNegotiation) { json(json) } }).build()
    }

    @Test
    fun `typed and raw search send identical JSON and force JSON in both format modes`() = runBlocking {
        for (format in SerializationFormat.entries) {
            val query = SearchQuery(vector = vector, vectorField = "named", vectorMetric = DistanceMetric.COSINE)
            var calls = 0
            val client = testClient(format) { request ->
                calls++
                assertEquals("/api/search/docs%2Fprivate", request.url.encodedPath)
                assertEquals(HttpMethod.Post, request.method)
                assertEquals("Bearer test-token", request.headers[HttpHeaders.Authorization])
                assertEquals("application/json", request.headers[HttpHeaders.Accept])
                assertEquals(ContentType.Application.Json, request.body.contentType?.withoutParameters())
                assertEquals(wire(query), expected((request.body as TextContent).text))
                respond(response, headers = headersOf(HttpHeaders.ContentType, "application/json"))
            }
            try {
                assertEquals(0.95, client.search("docs/private", query).results.single().score)
                assertTrue(client.search("docs/private", wire(query)).containsKey("future_metadata"))
                assertEquals(2, calls)
            } finally {
                client.close()
            }
        }
    }

    @Test
    fun `raw search still forwards unknown request fields`() = runBlocking {
        val raw = expected("""{"query":"","future_option":{"enabled":true}}""").jsonObject
        val client = testClient { request ->
            assertEquals(raw, expected((request.body as TextContent).text))
            respond(response, headers = headersOf(HttpHeaders.ContentType, "application/json"))
        }
        try {
            assertTrue(client.search("docs", raw).containsKey("future_metadata"))
        } finally {
            client.close()
        }
    }

    @Test
    fun `existing helpers retain request defaults and score injection`() = runBlocking {
        var calls = 0
        val client = testClient { request ->
            val expected = if (calls++ == 0) """{"query":"hello","limit":5}""" else """{"query":"hello","vector":[0.1,0.2,0.3],"limit":5}"""
            assertEquals(expected(expected), expected((request.body as TextContent).text))
            respond(response, headers = headersOf(HttpHeaders.ContentType, "application/json"))
        }
        try {
            assertEquals(JsonPrimitive(0.95), client.textSearch("docs", "hello", 5).single()["_score"])
            assertEquals(JsonPrimitive(0.95), client.hybridSearch("docs", "hello", vector, 5).single()["_score"])
        } finally {
            client.close()
        }
    }

    @Test
    fun `typed DSL exposes custom hybrid options`() = runBlocking {
        val client = testClient { request ->
            assertEquals(expected("""{"query":"hello","vector":[0.1,0.2,0.3],"text_weight":0.8,"vector_weight":0.2}"""), expected((request.body as TextContent).text))
            respond(response, headers = headersOf(HttpHeaders.ContentType, "application/json"))
        }
        try {
            val result = client.search("docs", "hello") { vector(vector); textWeight(0.8); vectorWeight(0.2) }
            assertEquals(1L, result.total)
        } finally {
            client.close()
        }
    }

    @Test
    fun `typed search retries an expired token with the same request`() = runBlocking {
        var calls = 0
        val query = SearchQuery("hello", limit = 3)
        val client = testClient { request ->
            assertEquals(wire(query), expected((request.body as TextContent).text))
            if (calls++ == 0) {
                respond("""{"error":"expired"}""", HttpStatusCode.Unauthorized,
                    headersOf(HttpHeaders.ContentType, "application/json"))
            } else {
                respond(response, headers = headersOf(HttpHeaders.ContentType, "application/json"))
            }
        }
        try {
            assertEquals(1L, client.search("docs", query).total)
            assertEquals(2, calls)
        } finally {
            client.close()
        }
    }

    @Test
    fun `typed search exposes HTTP failures instead of decoding a success response`() = runBlocking {
        val client = testClient {
            respond("""{"error":"missing collection"}""", HttpStatusCode.NotFound,
                headersOf(HttpHeaders.ContentType, "application/json"))
        }
        try {
            val error = assertFailsWith<Exception> { client.search("missing", SearchQuery("hello")) }
            assertTrue(error.message.orEmpty().contains("404"))
        } finally {
            client.close()
        }
    }

    @Test
    fun `numeric weight builder normalizes integral values like sibling builders`() {
        assertEquals("title:2,body:1.5", SearchQueryBuilder().weights(linkedMapOf("title" to 2.0, "body" to 1.5)).build().weights)
    }

    @Test
    fun `cross client golden requests match without normalizing away differences`() {
        val golden = json.parseToJsonElement(File("../test-fixtures/search-requests.json").readText()).jsonObject
        fun base(query: String = "") = SearchQueryBuilder(query).bypassRipple(false).bypassCache(false).limit(10)
        val cases = mapOf(
            "filtered_vector" to base().vector(vector).vectorMetric(DistanceMetric.COSINE).vectorK(10).vectorThreshold(0.25).filters { eq("category", "ml") }.build(),
            "hybrid_custom_weights" to base("machine learning").vector(vector).textWeight(0.7).vectorWeight(0.3).build(),
            "named_vector_field" to base().vector(vector).vectorField("title_embedding").vectorMetric(DistanceMetric.DOT_PRODUCT).vectorK(10).build(),
            "full_text" to base("enginer").fields(listOf("title", "body")).weights(linkedMapOf("title" to 2.5, "body" to 1.5)).fuzzy(true).maxEditDistance(2).build(),
        )
        for ((name, query) in cases) assertEquals(golden[name], wire(query), name)
    }
}
