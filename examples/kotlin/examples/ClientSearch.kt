package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.DistanceMetric
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import io.ekodb.client.types.SearchQuery
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

/** Typed search on current main; requires a local server to run. */
fun main() = runBlocking {
    val dotenv = dotenv()
    val client = EkoDBClient.builder()
        .baseUrl(dotenv["API_BASE_URL"] ?: "http://localhost:8080")
        .apiKey(dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb")
        .build()
    val collection = "kotlin_search_example"
    try {
        val documents = listOf(
            Triple("Rust Programming", "programming", listOf(0.9, 0.1, 0.2)),
            Triple("Python Programming", "programming", listOf(0.8, 0.2, 0.1)),
            Triple("Database Design", "database", listOf(0.1, 0.9, 0.3)),
        )
        for ((title, category, embedding) in documents) {
            client.insert(collection, Record.new()
                .insert("title", title)
                .insert("category", category)
                .insert("embedding", FieldType.vector(embedding)))
        }

        val text = client.search(collection, SearchQuery("programming", limit = 10))
        println("Text results: ${text.total}; execution time: ${text.executionTimeMs} ms")
        text.results.forEach { println("${it.record["title"]}: score=${it.score}, matched=${it.matchedFields}") }

        // Toy vectors demonstrate the request shape, not semantic embedding quality.
        // Production query vectors must match the model/dimensions used for documents.
        val queryVector = listOf(0.85, 0.15, 0.15)
        val vector = client.search(collection, SearchQuery(
            vector = queryVector,
            vectorField = "embedding",
            vectorMetric = DistanceMetric.COSINE,
            vectorK = 10,
            limit = 10,
        ))
        println("Vector results: ${vector.total}")

        val filtered = client.search(collection) {
            vector(queryVector)
            vectorField("embedding")
            vectorMetric(DistanceMetric.COSINE)
            vectorK(10)
            filters { eq("category", "programming") }
        }
        println("Filtered vector results: ${filtered.total}")

        val hybrid = client.search(collection, "programming") {
            vector(queryVector)
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
    } finally {
        try {
            client.deleteCollection(collection)
        } finally {
            client.close()
        }
    }
}
