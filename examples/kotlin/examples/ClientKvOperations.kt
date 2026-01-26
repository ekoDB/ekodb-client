package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.delay
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

/**
 * Key-Value operations example - Store and retrieve key-value pairs
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"
    
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
        
        // Example 5: Batch set multiple keys
        println("=== KV Batch Set ===")
        val batchEntries = listOf(
            Triple("config:db", buildJsonObject { put("host", "localhost"); put("port", 5432) }, null),
            Triple("config:cache", buildJsonObject { put("ttl", 3600); put("enabled", true) }, null),
            Triple("config:api", buildJsonObject { put("timeout", 30); put("retries", 3) }, null)
        )
        val setResults = client.kvBatchSet(batchEntries)
        println("✓ Batch set ${setResults.size} keys")
        setResults.forEach { (key, wasSet) ->
            println("  $key: ${if (wasSet) "success" else "failed"}")
        }
        println()
        
        // Example 6: Batch get multiple keys
        println("=== KV Batch Get ===")
        val keys = listOf("config:db", "config:cache", "config:api")
        val batchValues = client.kvBatchGet(keys)
        println("✓ Batch retrieved ${batchValues.size} values")
        batchValues.forEachIndexed { index, value ->
            println("  ${keys[index]}: $value")
        }
        println()
        
        // Example 7: Check if key exists
        println("=== KV Exists ===")
        val exists = client.kvExists("user:123")
        println("✓ Key exists: $exists\n")
        
        // Example 8: Find keys with pattern
        println("=== KV Find (Pattern Query) ===")
        val configResults = client.kvFind(pattern = "config:.*")
        println("✓ Found ${configResults.size} keys matching 'config:.*'\n")
        
        // Example 9: Query all keys
        println("=== KV Query (Alias for Find) ===")
        val allResults = client.kvQuery()
        println("✓ Total keys in store: ${allResults.size}\n")
        
        // Example 10: Delete a key
        println("=== KV Delete ===")
        client.kvDelete("user:123")
        println("✓ Deleted key: user:123\n")
        
        // Example 11: Verify deletion with kvExists
        println("=== Verify Deletion ===")
        val existsAfter = client.kvExists("user:123")
        println("✓ Key exists after delete: $existsAfter\n")
        
        // Cleanup with batch delete
        println("=== KV Batch Delete ===")
        val deleteResults = client.kvBatchDelete(keys)
        println("✓ Batch deleted ${deleteResults.size} keys")
        deleteResults.forEach { (key, wasDeleted) ->
            println("  $key: ${if (wasDeleted) "deleted" else "not found"}")
        }
        
    } finally {
        client.close()
        println("\n=== Example Complete ===")
    }
}
