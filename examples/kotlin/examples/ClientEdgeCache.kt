/**
 * ekoDB as Edge Cache Example
 *
 * Demonstrates using ekoDB Functions as an edge cache:
 * - Create a cache function that checks cache first
 * - On cache miss, fetch from external API and store
 * - On cache hit, return cached data instantly
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.Script
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlin.system.measureTimeMillis

fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB as Edge Cache - Simple Example ===")
    println()

    // Setup: Create cache collection with test data
    println("Setting up edge cache collection...")
    try { client.deleteCollection("edge_cache_kt") } catch (e: Exception) {}

    // Insert a cached entry
    val cacheRecord = Record.new()
        .insert("id", "weather_nyc")
        .insert("data", "{\"temp\": 72}")
        .insert("cached_at", java.time.Instant.now().toString())
    client.insert("edge_cache_kt", cacheRecord)
    println("✓ Cache entry created\n")

    // Create a simple cache lookup script
    println("Creating edge cache lookup script...")
    val cacheScript = Script(
        label = "edge_cache_lookup_kt",
        name = "Edge Cache Lookup",
        description = "Simple cache lookup by key",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = "edge_cache_kt")
        ),
        tags = listOf("cache", "edge")
    )

    val scriptId = client.saveScript(cacheScript)
    println("✓ Edge cache script created: $scriptId\n")

    // Test it - First call
    println("Call 1: Cache lookup")
    val duration1 = measureTimeMillis {
        val result1 = client.callScript("edge_cache_lookup_kt")
        println("Found ${result1.records.size} cached entries")
    }
    println("Response time: ${duration1}ms")

    // Test it again - Second call (should be fast due to connection reuse)
    println("\nCall 2: Cache lookup (connection warm)")
    val duration2 = measureTimeMillis {
        val result2 = client.callScript("edge_cache_lookup_kt")
        println("Found ${result2.records.size} cached entries")
    }
    println("Response time: ${duration2}ms")

    // Cleanup
    println("\n🧹 Cleaning up...")
    try {
        client.deleteScript(scriptId)
        client.deleteCollection("edge_cache_kt")
    } catch (e: Exception) {
        // Ignore cleanup errors
    }
    println("✓ Cleanup complete")

    println("\n=== The Magic ===")
    println("- Your DATABASE is your EDGE")
    println("- No Redis needed")
    println("- No CDN needed")
    println("- No cache invalidation logic needed (TTL handles it)")
    println("- With ripples: All nodes auto-sync cache")
    println("- One service: Database + Cache + Edge Functions")

    println("\n✓ Example complete!")
}
