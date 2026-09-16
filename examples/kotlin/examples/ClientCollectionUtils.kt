package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking

/**
 * Collection Utilities example - Demonstrates collection utility methods:
 * - collectionExists: Check if a collection exists
 * - count: Count documents in a collection
 * - listCollections: List all collections
 * - getCollection: Get collection metadata
 * - deleteCollection: Delete a collection
 */
fun main(): Unit = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    val collection = "collection_utils_test_kt"
    val nonexistentCollection = "nonexistent_collection_xyz_${System.nanoTime()}"
    var failure: Throwable? = null

    println("=== ekoDB Kotlin Client - Collection Utilities Example ===\n")

    try {
        // Make the example deterministic across repeat runs while keeping the
        // collectionExists calls below as the behavior being demonstrated.
        if (client.collectionExists(collection)) {
            client.deleteCollection(collection)
        }

        // Example 1: Check if collection exists (should be false initially)
        println("=== Check Collection Exists (Before Creation) ===")
        try {
            val exists = client.collectionExists(collection)
            println("Collection '$collection' exists: $exists")
            check(!exists) { "Expected '$collection' not to exist before creation" }
        } catch (e: Exception) {
            println("CollectionExists error: ${e.message}")
            throw e
        }

        // Example 2: Create some test documents to implicitly create the collection
        println("\n=== Creating Test Documents ===")
        for (i in 1..5) {
            val record = Record.new()
                .insert("name", "Test Document $i")
                .insert("index", i)
            try {
                client.insert(collection, record)
            } catch (e: Exception) {
                println("Insert error: ${e.message}")
                throw e
            }
        }
        println("Created 5 test documents")

        // Example 3: Check if collection exists now (should be true)
        println("\n=== Check Collection Exists (After Creation) ===")
        try {
            val exists = client.collectionExists(collection)
            println("Collection '$collection' exists: $exists")
            check(exists) { "Expected '$collection' to exist after creation" }
        } catch (e: Exception) {
            println("CollectionExists error: ${e.message}")
            throw e
        }

        // Example 4: Count documents in collection
        println("\n=== Count Documents ===")
        try {
            val count = client.count(collection)
            check(count == 5L) { "Expected 5 documents in '$collection', found $count" }
            println("Document count in '$collection': $count")
        } catch (e: Exception) {
            println("Count error: ${e.message}")
            throw e
        }

        // Example 5: Get collection metadata
        println("\n=== Get Collection Metadata ===")
        try {
            val metadata = client.getCollection(collection)
            println("Collection metadata: $metadata")
        } catch (e: Exception) {
            println("GetCollection error: ${e.message}")
            throw e
        }

        // Example 6: List all collections
        println("\n=== List Collections ===")
        try {
            val collections = client.listCollections()
            println("All collections (${collections.size}):")
            collections.forEach { println("  - $it") }
        } catch (e: Exception) {
            println("ListCollections error: ${e.message}")
            throw e
        }

        // Example 7: Check non-existent collection
        println("\n=== Check Non-Existent Collection ===")
        try {
            val exists = client.collectionExists(nonexistentCollection)
            println("Collection '$nonexistentCollection' exists: $exists")
            check(!exists) { "Expected '$nonexistentCollection' not to exist" }
        } catch (e: Exception) {
            println("CollectionExists error: ${e.message}")
            throw e
        }
    } catch (error: Throwable) {
        failure = error
    } finally {
        // Cleanup: Delete the test collection
        println("\n=== Cleanup ===")
        try {
            client.deleteCollection(collection)
            println("Deleted collection '$collection'")
        } catch (e: Exception) {
            println("Could not delete collection: ${e.message}")
            val primary = failure
            if (primary == null) failure = e else primary.addSuppressed(e)
        }

        try {
            client.close()
        } catch (closeError: Throwable) {
            val primary = failure
            if (primary == null) failure = closeError else primary.addSuppressed(closeError)
        }
    }
    failure?.let { throw it }
    println("\n=== Collection Utilities Example Complete ===")
}
