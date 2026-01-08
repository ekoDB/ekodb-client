package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.delay

/**
 * Document TTL example - Working with time-to-live for automatic expiration
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_ttl_example"
    
    println("=== ekoDB Kotlin Client - Document TTL Example ===\n")
    
    try {
        // Example 1: Insert document with TTL
        println("=== Insert with TTL ===")
        val record = Record.new()
            .insert("session_id", "abc123")
            .insert("user_id", "user_456")
            .insert("created_at", System.currentTimeMillis())
        
        val inserted = client.insertWithTtl(collection, record, "10s")
        val docId = (inserted["id"] as? FieldType.StringValue)?.value ?: error("No ID returned")
        println("✓ Inserted document with 10s TTL")
        println("  Document ID: $docId\n")
        
        // Example 2: Verify document exists
        println("=== Verify Document Exists ===")
        val found = client.findById(collection, docId)
        println("✓ Document found: ${found.keys.joinToString(", ")}\n")
        
        // Example 3: Insert document with longer TTL
        println("=== Insert with Longer TTL ===")
        val record2 = Record.new()
            .insert("cache_key", "data_xyz")
            .insert("value", "cached_data")
        
        val inserted2 = client.insertWithTtl(collection, record2, "1h")
        val docId2 = (inserted2["id"] as? FieldType.StringValue)?.value ?: error("No ID returned")
        println("✓ Inserted document with 1h TTL")
        println("  Document ID: $docId2\n")
        
        // Note: TTL expiration is automatic - the document will expire after 10 seconds
        // We don't wait to verify to keep the example fast
        println("=== TTL Expiration ===")
        println("✓ Document will automatically expire after 10 seconds\n")
        
        // Example 5: Verify second document still exists
        println("=== Verify Long TTL Document ===")
        val found2 = client.findById(collection, docId2)
        println("✓ Long TTL document still exists: ${found2.keys.joinToString(", ")}\n")
        
        // Example 6: Delete the long TTL document
        println("=== Delete Document ===")
        client.delete(collection, docId2)
        println("✓ Deleted document\n")
        
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
