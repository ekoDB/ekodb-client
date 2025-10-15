package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout

/**
 * Simple WebSocket example - Real-time subscriptions
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val wsUrl = System.getenv("WS_BASE_URL") ?: "ws://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_websocket_example"
    
    println("=== ekoDB Kotlin Client - Simple WebSocket Example ===\n")
    
    try {
        // Create WebSocket client
        println("=== Create WebSocket Client ===")
        val wsClient = client.websocket(wsUrl)
        println("✓ WebSocket client created\n")
        
        // Connect to WebSocket
        println("=== Connect to WebSocket ===")
        wsClient.connect()
        println("✓ Connected to WebSocket\n")
        
        // Insert a test record first
        println("=== Insert Test Record ===")
        val record = Record.new()
            .insert("name", "Test User")
            .insert("status", "active")
        
        client.insert(collection, record)
        println("✓ Inserted test record\n")
        
        // Test findAll via WebSocket
        println("=== Find All via WebSocket ===")
        try {
            withTimeout(5000) {
                val result = wsClient.findAll(collection)
                println("✓ WebSocket findAll result:")
                println("  $result\n")
            }
        } catch (e: Exception) {
            println("⚠ WebSocket findAll: ${e.message}\n")
        }
        
        // Close WebSocket
        println("=== Close WebSocket ===")
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
        println("\n=== Example Complete ===")
    }
}
