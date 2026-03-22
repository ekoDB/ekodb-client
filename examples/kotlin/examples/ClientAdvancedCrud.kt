package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive

/**
 * Advanced CRUD operations example.
 *
 * Exercises updateWithAction, updateWithActionSequence, restoreRecord, and restoreCollection.
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    val collection = "kotlin_advanced_crud_example"

    println("=== ekoDB Kotlin Client - Advanced CRUD Example ===\n")

    try {
        // Insert a record to work with
        println("--- Inserting base record ---")
        val record = Record.new()
            .insert("name", "Counter Record")
            .insert("views", 10)
            .insert("score", 100.0)
            .insert("tags", FieldType.array(listOf(
                FieldType.string("kotlin"),
                FieldType.string("example")
            )))

        val inserted = client.insert(collection, record)
        val recordId = (inserted["id"] as? FieldType.StringValue)?.value
            ?: error("No ID returned")
        println("Inserted: $recordId")

        // 1. updateWithAction - increment a counter
        println("\n--- updateWithAction: increment views ---")
        val incremented = client.updateWithAction(
            collection, recordId,
            action = "increment",
            field = "views",
            value = JsonPrimitive(5)
        )
        println("After increment: $incremented")

        // 2. updateWithAction - push to array
        println("\n--- updateWithAction: push to tags ---")
        val pushed = client.updateWithAction(
            collection, recordId,
            action = "push",
            field = "tags",
            value = JsonPrimitive("advanced")
        )
        println("After push: $pushed")

        // 3. updateWithActionSequence - multiple atomic actions
        println("\n--- updateWithActionSequence: multiple operations ---")
        val sequenced = client.updateWithActionSequence(
            collection, recordId,
            actions = listOf(
                Triple("increment", "views", JsonPrimitive(100)),
                Triple("decrement", "score", JsonPrimitive(25)),
                Triple("push", "tags", JsonPrimitive("sequenced"))
            )
        )
        println("After sequence: $sequenced")

        // 4. Delete the record (moves to trash)
        println("\n--- Deleting record (soft delete) ---")
        client.delete(collection, recordId)
        println("Record deleted (in trash)")

        // 5. Restore the single record
        println("\n--- Restoring record from trash ---")
        val restored = client.restoreRecord(collection, recordId)
        println("Record restored: $restored")

        // Verify restore
        val afterRestore = client.findById(collection, recordId)
        println("Verified restored record: ${afterRestore["name"]}")

        // 6. Delete the record again, then restore the whole collection
        println("\n--- Soft-deleting record again ---")
        client.delete(collection, recordId)
        println("Record deleted")

        println("\n--- Restoring entire collection ---")
        val restoredCount = client.restoreCollection(collection)
        println("Records restored: $restoredCount")

    } finally {
        // Cleanup
        println("\n--- Cleanup ---")
        try {
            client.deleteCollection(collection)
            println("Deleted collection: $collection")
        } catch (e: Exception) {
            println("Could not delete collection: ${e.message}")
        }

        client.close()
        println("\n=== Example Complete ===")
    }
}
