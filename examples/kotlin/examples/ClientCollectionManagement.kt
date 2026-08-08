package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

/**
 * Collection management example - Create, list, and manage collections
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_collection_example"
    
    println("=== ekoDB Kotlin Client - Collection Management Example ===\n")
    
    try {
        // Example 1: List all collections
        println("=== List Collections ===")
        val collections = client.listCollections()
        println("✓ Found ${collections.size} collections")
        collections.take(5).forEach { println("  - $it") }
        println()
        
        // Example 2: Check if collection exists
        println("=== Check Collection Existence ===")
        val exists = client.collectionExists(collection)
        println("Collection '$collection' exists: $exists\n")
        
        // Example 3: Get collection metadata (if exists)
        if (exists) {
            println("=== Get Collection Metadata ===")
            try {
                val metadata = client.getCollection(collection)
                println("✓ Collection metadata:")
                println("  $metadata\n")
            } catch (e: Exception) {
                println("⚠ Could not get metadata: ${e.message}\n")
            }
        }
        
        // Example 4: Delete collection if exists
        if (exists) {
            println("=== Delete Collection ===")
            client.deleteCollection(collection)
            println("✓ Deleted collection: $collection\n")
        }
        
        // Example 5: Create collection with schema
        println("=== Create Collection with Schema ===")
        val schema = buildJsonObject {
            put("fields", buildJsonObject {
                put("name", buildJsonObject {
                    put("field_type", "String")
                    put("required", true)
                })
                put("age", buildJsonObject {
                    put("field_type", "Integer")
                    put("required", false)
                })
            })
        }
        
        client.createCollection(collection, schema)
        println("✓ Created collection with schema: $collection\n")
        
        // Example 6: Get schema
        println("=== Get Collection Schema ===")
        val retrievedSchema = client.getSchema(collection)
        println("✓ Schema: $retrievedSchema\n")
        
    } finally {
        // Cleanup
        println("=== Cleanup ===")
        try {
            client.deleteCollection(collection)
            println("✓ Deleted collection: $collection")
        } catch (e: Exception) {
            println("⚠ Could not delete collection: ${e.message}")
        }
        
        client.close()
        println("\n=== Example Complete ===")
    }
}
