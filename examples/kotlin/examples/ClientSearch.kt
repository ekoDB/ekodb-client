package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.FieldTypeSchemaBuilder
import io.ekodb.client.SchemaBuilder
import io.ekodb.client.getValue
import io.ekodb.client.types.DistanceMetric
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import io.ekodb.client.types.SearchQuery
import io.ekodb.client.types.SearchResponse
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.NonCancellable
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import kotlinx.coroutines.withTimeout
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.decodeFromJsonElement
import kotlinx.serialization.json.put
import java.util.UUID

private data class PairedDocument(
    val title: String,
    val category: String,
    val queryEmbedding: List<Double>,
    val documentEmbedding: List<Double>,
)

/** Typed search on current main; requires a local server to run. */
fun main() = runBlocking {
    val dotenv = dotenv()
    val client = EkoDBClient.builder()
        .baseUrl(dotenv["API_BASE_URL"] ?: "http://localhost:8080")
        .apiKey(dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb")
        .build()
    val collection = "kotlin_search_example_${UUID.randomUUID().toString().replace("-", "").take(12)}"
    var failure: Throwable? = null
    try {
        client.createCollection(collection, SchemaBuilder()
            .addField("title", FieldTypeSchemaBuilder("String").required().textIndex())
            .addField("category", FieldTypeSchemaBuilder("String").required())
            .addField("query_embedding", FieldTypeSchemaBuilder("Vector").required().vectorIndex(metric = "cosine", dimension = 3))
            .addField("document_embedding", FieldTypeSchemaBuilder("Vector").required().vectorIndex(metric = "cosine", dimension = 3))
            .build())
        // Orthogonal toy vectors make the two rankings deterministic. Real query/document
        // vectors need compatible dimensions and a shared dual-encoder model space.
        val documents = listOf(
            PairedDocument("Rust Programming Question", "programming", listOf(1.0, 0.0, 0.0), listOf(0.0, 1.0, 0.0)),
            PairedDocument("Rust Programming Answer", "programming", listOf(0.0, 1.0, 0.0), listOf(1.0, 0.0, 0.0)),
            PairedDocument("Database Design Answer", "database", listOf(0.0, 0.0, 1.0), listOf(0.0, 0.0, 1.0)),
        )
        for (document in documents) {
            client.insert(collection, Record.new()
                .insert("title", document.title)
                .insert("category", document.category)
                .insert("query_embedding", FieldType.vector(document.queryEmbedding))
                .insert("document_embedding", FieldType.vector(document.documentEmbedding)))
        }

        val text = client.search(collection, SearchQuery("programming", limit = 10))
        println("Text results: ${text.total}; execution time: ${text.executionTimeMs} ms")
        text.results.forEach { println("${it.record["title"]}: score=${it.score}, matched=${it.matchedFields}") }

        val queryVector = documents.first().queryEmbedding
        fun topTitle(response: SearchResponse): String? = response.results.firstOrNull()?.let { hit ->
            getValue<String>(Json.decodeFromJsonElement<Record>(hit.record)["title"])
        }

        // The question's query vector finds the complementary answer in document space.
        val directional = client.search(collection, SearchQuery(
            vector = queryVector,
            vectorField = "document_embedding",
            vectorMetric = DistanceMetric.COSINE,
            vectorK = 3,
            limit = 3,
            bypassCache = true,
        ))
        check(topTitle(directional) == "Rust Programming Answer") {
            "Expected the complementary answer first, got ${topTitle(directional)}"
        }

        // Searching the same vector in query space instead finds the question itself.
        val sameField = client.search(collection, SearchQuery(
            vector = queryVector,
            vectorField = "query_embedding",
            vectorMetric = DistanceMetric.COSINE,
            vectorK = 3,
            limit = 3,
            bypassCache = true,
        ))
        check(topTitle(sameField) == "Rust Programming Question") {
            "Expected the source question first, got ${topTitle(sameField)}"
        }
        println("Paired vector search: document=${topTitle(directional)}, query=${topTitle(sameField)}")

        val filtered = client.search(collection) {
            vector(queryVector)
            vectorField("document_embedding")
            vectorMetric(DistanceMetric.COSINE)
            vectorK(10)
            filters { eq("category", "programming") }
        }
        println("Filtered vector results: ${filtered.total}")

        val hybrid = client.search(collection, "programming") {
            vector(queryVector)
            vectorField("document_embedding")
            fields(listOf("title"))
            textWeight(0.7)
            vectorWeight(0.3)
            limit(10)
        }
        println("Custom-weight hybrid results: ${hybrid.total}")

        // Existing raw wire escape hatch preserves unmodeled response metadata.
        val raw = client.search(collection, buildJsonObject {
            put("query", "programming")
            put("limit", 10)
        })
        println("Raw search: $raw")
    } catch (error: Throwable) {
        failure = error
        throw error
    } finally {
        try {
            // A lost create response or canceled search can still leave a collection behind.
            withContext(NonCancellable) {
                withTimeout(120_000) {
                    if (collection in client.listCollections()) {
                        client.deleteCollection(collection)
                        check(collection !in client.listCollections()) { "Example collection $collection was not deleted" }
                    }
                }
            }
        } catch (cleanup: Throwable) {
            val originalFailure = failure
            if (originalFailure != null) originalFailure.addSuppressed(cleanup) else throw cleanup
        } finally {
            client.close()
        }
    }
}
