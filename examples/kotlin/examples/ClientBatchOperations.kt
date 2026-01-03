package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

/**
 * Batch operations example - Insert, update, and delete multiple records efficiently
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_batch_example"
    
    println("=== ekoDB Kotlin Client - Batch Operations Example ===\n")
    
    try {
        // Example 1: Batch Insert
        println("=== Batch Insert ===")
        val records = (1..5).map { i ->
            Record.new()
                .insert("name", "User $i")
                .insert("email", "user$i@example.com")
                .insert("score", i * 10)
                .insert("active", true)
        }
        
        val insertResult = client.batchInsert(collection, records)
        println("✓ Inserted ${insertResult.successful.size} records")
        
        // Extract IDs from successful inserts
        // Using helper pattern that's also available as getRecordId() in Utils.kt
        val insertedIds = insertResult.successful.mapNotNull { record ->
            when (val idField = record["id"]) {
                is FieldType.StringValue -> idField.value
                else -> null
            }
        }
        println("  IDs: ${insertedIds.take(3).joinToString(", ")}...\n")
        
        // Example 2: Batch Update
        println("=== Batch Update ===")
        val updates = insertedIds.take(3).map { id ->
            id to Record.new()
                .insert("score", 100)
                .insert("updated", true)
        }
        
        val updateResult = client.batchUpdate(collection, updates)
        println("✓ Updated ${updateResult.size} records\n")
        
        // Example 3: Batch Delete
        println("=== Batch Delete ===")
        val deleteIds = insertedIds.takeLast(2)
        val deletedCount = client.batchDelete(collection, deleteIds)
        println("✓ Deleted $deletedCount records\n")
        
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
