package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.QueryBuilder
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

/**
 * Simple CRUD operations example
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_users_example"
    
    println("=== ekoDB Kotlin Client - Simple CRUD Example ===\n")
    
    try {
        // Create
        println("=== Create ===")
        val user = Record.new()
            .insert("name", "Alice Johnson")
            .insert("email", "alice@example.com")
            .insert("age", 28)
            .insert("active", true)
        
        val inserted = client.insert(collection, user)
        println("✓ Inserted user: $inserted")
        
        val userId = (inserted["id"] as? FieldType.StringValue)?.value
            ?: error("No ID returned")
        println("  User ID: $userId\n")
        
        // Read
        println("=== Read ===")
        val found = client.findById(collection, userId)
        println("✓ Found user by ID: $found\n")
        
        // Update
        println("=== Update ===")
        val updates = Record.new()
            .insert("age", 29)
            .insert("city", "San Francisco")
        
        val updated = client.update(collection, userId, updates)
        println("✓ Updated user: $updated\n")
        
        // Query
        println("=== Query ===")
        val query = QueryBuilder.new()
            .gte("age", 25)
            .eq("active", true)
            .limit(10)
            .build()
        
        val results = client.find(collection, query)
        println("✓ Found ${results.size} users matching query")
        results.forEach { println("  - $it") }
        println()
        
        // Delete
        println("=== Delete ===")
        client.delete(collection, userId)
        println("✓ Deleted user with ID: $userId\n")
        
        // Verify deletion
        try {
            client.findById(collection, userId)
            println("⚠ User still exists (unexpected)")
        } catch (e: Exception) {
            println("✓ Confirmed user was deleted\n")
        }
        
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
