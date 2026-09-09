package io.ekodb.client

import io.ekodb.client.types.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.api.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import java.io.File
import java.time.Instant
import java.util.UUID
import kotlin.test.*

/** Opt-in via vectorLiveTest. Only the uniquely named collection is mutated. */
class VectorLiveContractTest {
    @Test
    fun `create insert search filter update and cleanup`() = runBlocking {
        val baseUrl = requireNotNull(System.getenv("EKODB_BASE_URL")) { "EKODB_BASE_URL is required" }.trimEnd('/')
        val apiKey = requireNotNull(System.getenv("EKODB_API_KEY")) { "EKODB_API_KEY is required" }
        var insertRequests = 0
        val countInserts = createClientPlugin("CountVectorInserts") {
            onRequest { request, _ ->
                if (request.url.encodedPath.startsWith("/api/insert/")) insertRequests++
            }
        }
        val transport = HttpClient(CIO) {
            install(ContentNegotiation) { json(Json { ignoreUnknownKeys = true }) }
            install(countInserts)
        }
        val client = EkoDBClient.builder().baseUrl(baseUrl).apiKey(apiKey).httpClient(transport).build()
        val collection = "kotlin_vector_contract_${UUID.randomUUID().toString().replace("-", "").take(12)}"
        val evidence = linkedMapOf<String, JsonElement>(
            "timestamp" to JsonPrimitive(Instant.now().toString()),
            "deployment" to JsonPrimitive("[provisioned host redacted]"),
            "collection" to JsonPrimitive(collection)
        )
        var stage = "health"
        var failure: Throwable? = null
        fun passed(name: String) { evidence[name] = JsonPrimitive("passed") }
        fun record(label: String, eligible: Boolean, vector: List<Double>) = EkoRecord()
            .insert("label", label).insert("eligible", eligible).insert("count", 7)
            .insert("ordinary", FieldType.array(listOf(FieldType.integer(1), FieldType.integer(2))))
            .insert("embedding", FieldType.vector(vector))
        fun label(hit: SearchResult): String? = getValue(Json.decodeFromJsonElement<EkoRecord>(hit.record)["label"])
        val query = SearchQuery(vector = listOf(1.0, 0.0, 0.0), vectorField = "embedding",
            vectorMetric = DistanceMetric.COSINE, vectorK = 2, limit = 2, bypassCache = true)
        try {
            val healthClient = HttpClient { install(ContentNegotiation) { json() } }
            try {
                val health = healthClient.get("$baseUrl/api/health").body<JsonObject>()
                evidence["health"] = buildJsonObject {
                    put("status", health["status"] ?: JsonNull)
                    put("version", health["version"] ?: JsonPrimitive("UNKNOWN"))
                    put("commit", health["commit"] ?: JsonNull)
                }
                assertEquals(JsonPrimitive("ok"), health["status"])
                // The unauthenticated endpoint may hide version/build metadata.
                val auth = healthClient.post("$baseUrl/api/auth/token") {
                    contentType(ContentType.Application.Json)
                    setBody(buildJsonObject { put("api_key", apiKey) })
                }.body<JsonObject>()
                val authenticated = healthClient.get("$baseUrl/api/health") {
                    bearerAuth(auth.getValue("token").jsonPrimitive.content)
                }.body<JsonObject>()
                evidence["authenticated_health"] = buildJsonObject {
                    put("status", authenticated["status"] ?: JsonNull)
                    put("version", authenticated["version"] ?: JsonPrimitive("UNKNOWN"))
                    put("commit", authenticated["commit"] ?: JsonNull)
                }
            } finally {
                healthClient.close()
            }
            stage = "create_readback"
            val schema = SchemaBuilder()
                .addField("label", FieldTypeSchemaBuilder("String").required())
                .addField("eligible", FieldTypeSchemaBuilder("Boolean").required())
                .addField("count", FieldTypeSchemaBuilder("Integer"))
                .addField("ordinary", FieldTypeSchemaBuilder("Array"))
                .addField("embedding", FieldTypeSchemaBuilder("Vector").required().vectorIndex(metric = "cosine"))
                .build()
            client.createCollection(collection, schema)
            val fields = client.getSchema(collection).getValue("fields").jsonObject
            val embedding = fields.getValue("embedding").jsonObject
            assertEquals(JsonPrimitive("Vector"), embedding["field_type"])
            assertEquals(JsonPrimitive("cosine"), embedding.getValue("index").jsonObject["metric"])
            passed(stage)
            stage = "insert"
            val nearest = client.insert(collection, record("nearest", false, listOf(1.0, 0.0, 0.0)))
            client.insert(collection, record("eligible", true, listOf(0.8, 0.6, 0.0)))
            val id = requireNotNull(getValue<String>(nearest["id"]))
            passed(stage)
            stage = "read_vector"
            assertEquals(FieldType.vector(listOf(1.0, 0.0, 0.0)), client.findById(collection, id)["embedding"])
            passed(stage)
            stage = "vector_search"
            val search = client.search(collection, query)
            assertEquals("nearest", label(search.results.first()))
            assertTrue(search.results.first().score.isFinite())
            evidence["nearest_score"] = JsonPrimitive(search.results.first().score)
            passed(stage)
            stage = "filtered_vector_search"
            val filtered = client.search(collection, query.copy(filters = QueryBuilder().eq("eligible", true).build().filter))
            assertEquals(listOf("eligible"), filtered.results.map { label(it) })
            assertTrue(filtered.results.single().score.isFinite())
            evidence["filtered_score"] = JsonPrimitive(filtered.results.single().score)
            passed(stage)
            stage = "upsert_search"
            client.upsert(collection, id, record("nearest", false, listOf(0.0, 1.0, 0.0)))
            assertEquals(FieldType.vector(listOf(0.0, 1.0, 0.0)), client.findById(collection, id)["embedding"])
            assertEquals("eligible", label(client.search(collection, query).results.first()))
            passed(stage)
            stage = "bad_array_insert"
            val bad = record("invalid", true, listOf(1.0, 0.0, 0.0))
                .insert("embedding", FieldType.array(listOf(FieldType.float(1.0), FieldType.float(0.0), FieldType.float(0.0))))
            val beforeBadInsert = insertRequests
            val error = assertFailsWith<EkoDBHttpException> { client.insert(collection, bad) }
            assertEquals(1, insertRequests - beforeBadInsert)
            evidence["bad_array_attempts"] = JsonPrimitive(insertRequests - beforeBadInsert)
            assertEquals(400, error.statusCode)
            assertEquals("""{"error":"Type mismatch for field 'embedding': expected Vector, got Array"}""", error.responseBody)
            evidence["bad_array_status"] = JsonPrimitive(error.statusCode)
            evidence["bad_array_body"] = JsonPrimitive(error.responseBody)
            passed(stage)
        } catch (error: Throwable) {
            failure = error
            evidence["failed_stage"] = JsonPrimitive(stage)
            // Persist no arbitrary server body or exception message: it may echo data.
            evidence["failure_type"] = JsonPrimitive(error.javaClass.simpleName)
            if (error is EkoDBHttpException) evidence["failure_status"] = JsonPrimitive(error.statusCode)
            throw error
        } finally {
            try {
                client.deleteCollection(collection)
                // GET collection metadata may return a fresh empty schema after
                // deletion; use listing and propagate errors when proving absence.
                assertFalse(collection in client.listCollections())
                passed("cleanup_verified_absent")
            } catch (cleanup: Throwable) {
                evidence["cleanup_failure_type"] = JsonPrimitive(cleanup.javaClass.simpleName)
                if (cleanup is EkoDBHttpException) evidence["cleanup_failure_status"] = JsonPrimitive(cleanup.statusCode)
                if (failure != null) failure.addSuppressed(cleanup) else throw cleanup
            } finally {
                client.close()
                File("build/vector-live-contract.json").writeText(Json { prettyPrint = true }.encodeToString(JsonObject(evidence)) + "\n")
            }
        }
    }
}
