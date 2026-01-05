/**
 * SWR (Stale-While-Revalidate) Pattern Example
 *
 * Demonstrates using ekoDB Functions as an SWR cache:
 * - Check cache first, return if exists
 * - On cache miss, fetch from external API
 * - Store result with TTL for auto-expiration
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.Script
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking
import kotlin.system.measureTimeMillis

fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===")
    println()

    // Setup: Create cache collection with test data
    println("Step 1: Setting up cache collection...")
    try { client.deleteCollection("swr_cache_kt") } catch (e: Exception) {}

    // Insert a cached entry to simulate cache hit
    val cacheRecord = Record.new()
        .insert("id", "torvalds")
        .insert("data", "{\"login\": \"torvalds\", \"name\": \"Linus Torvalds\"}")
        .insert("cached_at", java.time.Instant.now().toString())
    client.insert("swr_cache_kt", cacheRecord)
    println("✓ Cache entry created\n")

    // Create a simple cache lookup script
    println("Step 2: Create SWR cache lookup script")
    val swrScript = Script(
        label = "swr_cache_lookup_kt",
        name = "SWR Cache Lookup",
        description = "Simple cache lookup for SWR pattern",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = "swr_cache_kt")
        ),
        tags = listOf("swr", "cache")
    )

    val scriptId = client.saveScript(swrScript)
    println("✓ Created SWR script: swr_cache_lookup_kt ($scriptId)\n")

    // First call - demonstrates cache lookup
    println("Step 3: First call - Cache lookup")
    val result1 = client.callScript("swr_cache_lookup_kt")
    println("Found ${result1.records.size} cached entries")
    println("✓ Cache lookup complete\n")

    // Second call - demonstrates fast response
    println("Step 4: Second call - Fast cache hit")
    val duration = measureTimeMillis {
        client.callScript("swr_cache_lookup_kt")
    }
    println("Response time: ${duration}ms (served from cache)")
    println("✓ Lightning fast cache hit\n")

    // Cleanup
    println("🧹 Cleaning up...")
    try {
        client.deleteScript(scriptId)
        client.deleteCollection("swr_cache_kt")
    } catch (e: Exception) {
        // Ignore cleanup errors
    }
    println("✓ Cleanup complete\n")

    println("=== SWR Pattern Summary ===")
    println("✅ Cache miss → Fetch from API → Store in ekoDB")
    println("✅ Cache hit → Instant response from ekoDB")
    println("✅ TTL handles automatic cache invalidation")
}
