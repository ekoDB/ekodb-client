package io.ekodb.client.examples

import io.ekodb.client.*
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking
import java.util.*

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
        // Create - Insert document with various field types
        println("=== Create ===")
        val user = Record.new()
            .insert("name", "Alice Johnson")
            .insert("email", "alice@example.com")
            .insert("age", 28)
            .insert("active", true)
            .insert("price", 99.99)
            .insert("created_at", Date().toString())
            .insert("user_id", "550e8400-e29b-41d4-a716-446655440000")
            .insert("tags", FieldType.array(listOf(
                FieldType.string("tag1"),
                FieldType.string("tag2"),
                FieldType.string("tag3")
            )))
            .insert("metadata", FieldType.obj(mapOf(
                "key" to FieldType.string("value"),
                "nested" to FieldType.obj(mapOf("deep" to FieldType.boolean(true)))
            )))
            .insert("embedding", FieldType.vector(listOf(0.1, 0.2, 0.3, 0.4, 0.5)))
            .insert("categories", FieldType.set(listOf(
                FieldType.string("electronics"),
                FieldType.string("computers")
            )))
            .insert("data", FieldType.binary(java.util.Base64.getDecoder().decode("aGVsbG8gd29ybGQ=")))
        
        val inserted = client.insert(collection, user)
        println("✓ Inserted user: $inserted")
        
        val userId = (inserted["id"] as? FieldType.StringValue)?.value
            ?: error("No ID returned")
        println("  User ID: $userId\n")
        
        // Read
        println("=== Read ===")
        val found = client.findById(collection, userId)
        println("✓ Found user by ID: $found\n")
        
        // Read - Extract field values using type-specific getValue utilities
        println("=== Extract Field Values (All Types) ===")
        // ekoDB returns fields as Map with "type" and "value"
        // Use type-specific getValue functions to extract values
        val name = getValue<String>(found["name"])
        val email = getValue<String>(found["email"])
        val age = getValue<Int>(found["age"])
        val active = getValue<Boolean>(found["active"])
        val price = getDecimalValue(found["price"])
        val createdAt = getDateTimeValue(found["created_at"])
        val userIdField = getUUIDValue(found["user_id"])
        val tags = getArrayValue(found["tags"])
        val metadata = getObjectValue(found["metadata"])
        val embedding = getVectorValue(found["embedding"])
        val categories = getSetValue(found["categories"])
        val data = getBytesValue(found["data"])
        
        println("Extracted values:")
        println("  name (String): $name")
        println("  email (String): $email")
        println("  age (Integer): $age")
        println("  active (Boolean): $active")
        println("  price (Decimal): $price")
        println("  created_at (DateTime): $createdAt")
        println("  user_id (UUID): $userIdField")
        println("  tags (Array): $tags")
        println("  metadata (Object): $metadata")
        println("  embedding (Vector): $embedding")
        println("  categories (Set): $categories")
        println("  data (Bytes): ${data?.size ?: 0} bytes")
        
        // Record already contains all fields
        println("Record fields: ${found.keys.joinToString(", ")}\n")
        
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
