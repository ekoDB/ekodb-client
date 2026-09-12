package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put

/** Retrieve unique field values with and without a typed query-expression filter. */
fun main() = runBlocking {
    val dotenv = dotenv()
    val client = EkoDBClient.builder()
        .baseUrl(dotenv["API_BASE_URL"] ?: "http://localhost:8080")
        .apiKey(dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb")
        .build()
    val collection = "distinct_values_example_kt"

    try {
        try {
            client.deleteCollection(collection)
        } catch (_: Exception) {
            // The collection may not exist on the first run.
        }

        val products = listOf(
            Triple("Widget A", "electronics", "active"),
            Triple("Widget B", "electronics", "discontinued"),
            Triple("Book 1", "books", "active"),
        )
        for ((name, category, status) in products) {
            client.insert(
                collection,
                Record.new()
                    .insert("name", name)
                    .insert("category", category)
                    .insert("status", status)
            )
        }

        val categories = client.distinctValues(collection, "category")
        check(categories["count"]?.jsonPrimitive?.content?.toInt() == 2)
        check(categories["values"]?.jsonArray?.size == 2)

        val electronics = client.distinctValues(
            collection,
            "status",
            filter = buildJsonObject {
                put("type", "Condition")
                put("content", buildJsonObject {
                    put("field", "category")
                    put("operator", "Eq")
                    put("value", "electronics")
                })
            }
        )
        check(electronics["count"]?.jsonPrimitive?.content?.toInt() == 2)
        println("Kotlin distinct-values example passed")
    } finally {
        try {
            client.deleteCollection(collection)
        } finally {
            client.close()
        }
    }
}
