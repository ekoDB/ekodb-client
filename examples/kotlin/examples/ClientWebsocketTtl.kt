package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout

/**
 * WebSocket TTL example - WebSocket operations with TTL documents
 */
fun main() = runBlocking {
    val dotenv = dotenv { ignoreIfMissing = true }
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val wsUrl = dotenv["WS_BASE_URL"] ?: "ws://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb-0123456789"
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_websocket_ttl_example"
    
    println("=== ekoDB Kotlin Client - WebSocket TTL Example ===\n")
    
    try {
        // Step 1: Insert test data with TTL via HTTP
        println("=== Insert Test Data with TTL ===")
        
        val record = Record.new()
            .insert("name", "WebSocket TTL Test")
            .insert("value", 42)
            .insert("created_at", System.currentTimeMillis())
        
        val inserted = client.insertWithTtl(collection, record, "1h")
        println("✓ Inserted document with TTL: ${inserted["id"]}\n")
        
        // Step 2: Query via WebSocket
        println("=== Query via WebSocket ===")
        
        val wsClient = client.websocket(wsUrl)
        wsClient.connect()
        println("✓ WebSocket connected")
        
        try {
            withTimeout(5000) {
                val result = wsClient.findAll(collection)
                println("✓ Retrieved data via WebSocket:")
                println("  $result\n")
            }
        } catch (e: Exception) {
            println("⚠ WebSocket query: ${e.message}\n")
        }
        
        wsClient.close()
        println("✓ WebSocket closed\n")
        
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
        println("\n✓ WebSocket TTL example completed successfully")
        println("\n💡 Note: Documents with TTL will automatically expire after the specified duration")
    }
}
