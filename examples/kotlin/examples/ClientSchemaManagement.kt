package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

/**
 * Schema management example - Advanced schema operations
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_schema_example"
    
    println("=== ekoDB Kotlin Client - Schema Management Example ===\n")
    
    try {
        // Step 1: Create collection with schema
        println("=== Creating Collection with Schema ===")
        
        // Delete if exists
        try {
            client.deleteCollection(collection)
        } catch (e: Exception) {
            // Ignore if doesn't exist
        }
        
        val schema = buildJsonObject {
            put("fields", buildJsonObject {
                put("title", buildJsonObject {
                    put("field_type", "String")
                    put("required", true)
                })
                put("email", buildJsonObject {
                    put("field_type", "String")
                    put("required", true)
                })
                put("age", buildJsonObject {
                    put("field_type", "Integer")
                    put("required", false)
                })
                put("status", buildJsonObject {
                    put("field_type", "String")
                    put("required", false)
                })
            })
        }
        
        client.createCollection(collection, schema)
        println("✓ Created collection '$collection' with schema\n")
        
        // Step 2: Insert valid documents
        println("=== Inserting Valid Documents ===")
        
        val user1 = Record.new()
            .insert("title", "Software Engineer")
            .insert("email", "alice@example.com")
            .insert("age", 28)
            .insert("status", "active")
        
        val inserted1 = client.insert(collection, user1)
        println("✓ Inserted user 1: ${inserted1["id"]}")
        
        val user2 = Record.new()
            .insert("title", "Product Manager")
            .insert("email", "bob@example.com")
            .insert("age", 32)
            .insert("status", "pending")
        
        val inserted2 = client.insert(collection, user2)
        println("✓ Inserted user 2: ${inserted2["id"]}\n")
        
        // Step 3: Get schema
        println("=== Getting Schema ===")
        
        val retrievedSchema = client.getSchema(collection)
        println("✓ Schema for $collection:")
        val fields = retrievedSchema["fields"]
        println("  Fields: $fields\n")
        
        // Step 4: List all collections
        println("=== Listing Collections ===")
        
        val collections = client.listCollections()
        println("✓ Total collections: ${collections.size}")
        println("  Sample: ${collections.take(5)}\n")
        
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
        println("\n✓ All schema management operations completed successfully")
    }
}
