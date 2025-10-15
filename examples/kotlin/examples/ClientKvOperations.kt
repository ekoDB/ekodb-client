package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.delay
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

/**
 * Key-Value operations example - Store and retrieve key-value pairs
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    println("=== ekoDB Kotlin Client - KV Operations Example ===\n")
    
    try {
        // Example 1: Set a key-value pair
        println("=== KV Set ===")
        client.kvSet("user:123", buildJsonObject {
            put("name", "Alice")
            put("email", "alice@example.com")
            put("role", "admin")
        })
        println("✓ Set key: user:123\n")
        
        // Example 2: Get a value
        println("=== KV Get ===")
        val value = client.kvGet("user:123")
        println("✓ Retrieved value: $value\n")
        
        // Example 3: Set with TTL
        println("=== KV Set with TTL ===")
        client.kvSetWithTtl("session:abc123", buildJsonObject {
            put("user_id", "123")
            put("created_at", System.currentTimeMillis())
        }, "10s")
        println("✓ Set key with 10s TTL: session:abc123\n")
        
        // Example 4: Verify TTL key exists
        println("=== Verify TTL Key ===")
        val sessionValue = client.kvGet("session:abc123")
        println("✓ Session value: $sessionValue")
        println("  (Will expire in 10 seconds)\n")
        
        // Example 5: Set multiple keys
        println("=== Set Multiple Keys ===")
        val keys = listOf("config:db", "config:cache", "config:api")
        keys.forEachIndexed { index, key ->
            client.kvSet(key, buildJsonObject {
                put("setting", "value_$index")
                put("enabled", true)
            })
        }
        println("✓ Set ${keys.size} configuration keys\n")
        
        // Example 6: Delete a key
        println("=== KV Delete ===")
        client.kvDelete("user:123")
        println("✓ Deleted key: user:123\n")
        
        // Example 7: Verify deletion
        println("=== Verify Deletion ===")
        val deletedValue = client.kvGet("user:123")
        if (deletedValue == null) {
            println("✓ Confirmed key was deleted\n")
        } else {
            println("⚠ Key still exists (unexpected)\n")
        }
        
        // Note: TTL expiration is automatic - the key will expire after 10 seconds
        // We don't wait to verify to keep the example fast
        
        // Cleanup remaining keys
        println("=== Cleanup ===")
        keys.forEach { key ->
            try {
                client.kvDelete(key)
            } catch (e: Exception) {
                // Ignore errors
            }
        }
        println("✓ Cleaned up test keys")
        
    } finally {
        client.close()
        println("\n=== Example Complete ===")
    }
}
