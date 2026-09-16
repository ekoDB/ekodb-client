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
import io.ekodb.client.functions.UserFunction
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlin.system.measureTimeMillis

private fun isAlreadyExistsError(e: Exception): Boolean {
    val msg = e.message ?: return false
    return msg.contains("status 409") || msg.contains("already exists")
}

private fun isNotFoundError(error: Throwable): Boolean =
    error.message?.let { it.contains("status 404") || it.contains("not found", ignoreCase = true) } == true

private suspend fun saveOrUpdate(client: EkoDBClient, func: UserFunction): String {
    return try {
        client.saveFunction(func)
    } catch (e: Exception) {
        if (isAlreadyExistsError(e)) {
            client.updateFunction(func.label, func)
            println("ℹ️  Function '${func.label}' already existed — updated instead")
            client.getFunction(func.label).id
                ?: throw IllegalStateException("No ID returned for function '${func.label}'")
        } else {
            throw e
        }
    }
}

fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    val runSuffix = System.currentTimeMillis()
    val collection = "edge_cache_kt_$runSuffix"
    val functionLabel = "edge_cache_lookup_kt_$runSuffix"
    var primaryError: Throwable? = null

    try {
        println("=== ekoDB as Edge Cache - Simple Example ===")
        println()

    // Setup: Create cache collection with test data
    println("Setting up edge cache collection...")
    // Insert a cached entry
    val cacheRecord = Record.new()
        .insert("id", "weather_nyc")
        .insert("data", "{\"temp\": 72}")
        .insert("cached_at", java.time.Instant.now().toString())
    client.insert(collection, cacheRecord)
    println("✓ Cache entry created\n")

    // Create a simple cache lookup function
    println("Creating edge cache lookup function...")
    val cacheFunc = UserFunction(
        label = functionLabel,
        name = "Edge Cache Lookup",
        description = "Simple cache lookup by key",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = collection)
        ),
        tags = listOf("cache", "edge")
    )

    val funcId = saveOrUpdate(client, cacheFunc)
    println("✓ Edge cache function created: $funcId\n")

    // Test it - First call
    println("Call 1: Cache lookup")
    val duration1 = measureTimeMillis {
        val result1 = client.callFunction(functionLabel)
        println("Found ${result1.records.size} cached entries")
    }
    println("Response time: ${duration1}ms")

    // Test it again - Second call (should be fast due to connection reuse)
    println("\nCall 2: Cache lookup (connection warm)")
    val duration2 = measureTimeMillis {
        val result2 = client.callFunction(functionLabel)
        println("Found ${result2.records.size} cached entries")
    }
    println("Response time: ${duration2}ms")

        println("\n=== The Magic ===")
        println("- Your DATABASE is your EDGE")
        println("- No Redis needed")
        println("- No CDN needed")
        println("- No cache invalidation logic needed (TTL handles it)")
        println("- With ripples: All nodes auto-sync cache")
        println("- One service: Database + Cache + Edge Functions")
    } catch (error: Throwable) {
        primaryError = error
        throw error
    } finally {
        println("\n🧹 Cleaning up...")
        val cleanupErrors = mutableListOf<Throwable>()
        try {
            val functionId = client.getFunction(functionLabel).id
            if (functionId != null) client.deleteFunction(functionId)
        } catch (error: Throwable) {
            if (!isNotFoundError(error)) cleanupErrors += error
        }
        try {
            client.deleteCollection(collection)
        } catch (error: Throwable) {
            if (!isNotFoundError(error)) cleanupErrors += error
        }
        try {
            client.close()
        } catch (error: Throwable) {
            cleanupErrors += error
        }

        val failure = primaryError
        if (failure != null) {
            cleanupErrors.forEach(failure::addSuppressed)
        } else if (cleanupErrors.isNotEmpty()) {
            val cleanupError = cleanupErrors.first()
            cleanupErrors.drop(1).forEach(cleanupError::addSuppressed)
            throw cleanupError
        }
        println("✓ Cleanup complete")
    }

    println("\n✓ Example complete!")
}
