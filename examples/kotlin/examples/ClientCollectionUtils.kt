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
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    val collection = "collection_utils_test_kt"

    println("=== ekoDB Kotlin Client - Collection Utilities Example ===\n")

    try {
        // Example 1: Check if collection exists (should be false initially)
        println("=== Check Collection Exists (Before Creation) ===")
        try {
            val exists = client.collectionExists(collection)
            println("Collection '$collection' exists: $exists")
        } catch (e: Exception) {
            println("CollectionExists error: ${e.message}")
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
            }
        }
        println("Created 5 test documents")

        // Example 3: Check if collection exists now (should be true)
        println("\n=== Check Collection Exists (After Creation) ===")
        try {
            val exists = client.collectionExists(collection)
            println("Collection '$collection' exists: $exists")
        } catch (e: Exception) {
            println("CollectionExists error: ${e.message}")
        }

        // Example 4: Count documents in collection
        println("\n=== Count Documents ===")
        try {
            val count = client.count(collection)
            println("Document count in '$collection': $count")
        } catch (e: Exception) {
            println("Count error: ${e.message}")
        }

        // Example 5: Get collection metadata
        println("\n=== Get Collection Metadata ===")
        try {
            val metadata = client.getCollection(collection)
            println("Collection metadata: $metadata")
        } catch (e: Exception) {
            println("GetCollection error: ${e.message}")
        }

        // Example 6: List all collections
        println("\n=== List Collections ===")
        try {
            val collections = client.listCollections()
            println("All collections (${collections.size}):")
            collections.forEach { println("  - $it") }
        } catch (e: Exception) {
            println("ListCollections error: ${e.message}")
        }

        // Example 7: Check non-existent collection
        println("\n=== Check Non-Existent Collection ===")
        try {
            val exists = client.collectionExists("nonexistent_collection_xyz")
            println("Collection 'nonexistent_collection_xyz' exists: $exists")
        } catch (e: Exception) {
            println("CollectionExists error: ${e.message}")
        }

    } finally {
        // Cleanup: Delete the test collection
        println("\n=== Cleanup ===")
        try {
            client.deleteCollection(collection)
            println("Deleted collection '$collection'")
        } catch (e: Exception) {
            println("Could not delete collection: ${e.message}")
        }

        client.close()
        println("\n=== Collection Utilities Example Complete ===")
    }
}
