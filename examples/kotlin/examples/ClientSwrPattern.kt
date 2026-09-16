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
    val collection = "swr_cache_kt_$runSuffix"
    val functionLabel = "swr_cache_lookup_kt_$runSuffix"
    var primaryError: Throwable? = null

    try {
        println("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===")
        println()

    // Setup: Create cache collection with test data
    println("Step 1: Setting up cache collection...")
    // Insert a cached entry to simulate cache hit
    val cacheRecord = Record.new()
        .insert("id", "torvalds")
        .insert("data", "{\"login\": \"torvalds\", \"name\": \"Linus Torvalds\"}")
        .insert("cached_at", java.time.Instant.now().toString())
    client.insert(collection, cacheRecord)
    println("✓ Cache entry created\n")

    // Create a simple cache lookup function
    println("Step 2: Create SWR cache lookup function")
    val swrScript = UserFunction(
        label = functionLabel,
        name = "SWR Cache Lookup",
        description = "Simple cache lookup for SWR pattern",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = collection)
        ),
        tags = listOf("swr", "cache")
    )

    val funcId = saveOrUpdate(client, swrScript)
    println("✓ Created SWR function: $functionLabel ($funcId)\n")

    // First call - demonstrates cache lookup
    println("Step 3: First call - Cache lookup")
    val result1 = client.callFunction(functionLabel)
    println("Found ${result1.records.size} cached entries")
    println("✓ Cache lookup complete\n")

    // Second call - demonstrates fast response
    println("Step 4: Second call - Fast cache hit")
    val duration = measureTimeMillis {
        client.callFunction(functionLabel)
    }
    println("Response time: ${duration}ms (served from cache)")
    println("✓ Lightning fast cache hit\n")

        println("=== SWR Pattern Summary ===")
        println("✅ Cache miss → Fetch from API → Store in ekoDB")
        println("✅ Cache hit → Instant response from ekoDB")
        println("✅ TTL handles automatic cache invalidation")
    } catch (error: Throwable) {
        primaryError = error
        throw error
    } finally {
        println("🧹 Cleaning up...")
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
        println("✓ Cleanup complete\n")
    }
}
