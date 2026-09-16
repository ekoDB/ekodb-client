package io.ekodb.client.examples

/**
 * ekoDB Kotlin Client - Native SWR Function Example
 *
 * Demonstrates the native SWR (Stale-While-Revalidate) function that
 * encapsulates the entire cache-aside pattern in a single operation.
 */

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.ParameterDefinition
import io.ekodb.client.functions.UserFunction
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlin.system.measureTimeMillis

private const val BASIC_LABEL = "github_user_native_kt"
private const val AUDIT_LABEL = "product_swr_audit_kt"
private const val PIPELINE_LABEL = "user_enrichment_pipeline_kt"
private const val DYNAMIC_LABEL = "flexible_cache_kt"
private const val AUDIT_COLLECTION = "swr_audit_trail_kt"
private const val ENRICHED_COLLECTION = "enriched_users_kt"

private fun isAlreadyExistsError(e: Exception): Boolean {
    val msg = e.message ?: return false
    return msg.contains("status 409") || msg.contains("already exists")
}

private fun isNotFoundError(e: Exception): Boolean =
    e.message.orEmpty().contains("not found", ignoreCase = true)

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

fun exampleBasicSWR(client: EkoDBClient): String = runBlocking {
    println("\nExample 1: Basic Native SWR")
    println("─".repeat(80))
    println("Single function replaces KvGet → If → HttpRequest → KvSet pipeline")

    // Create function with native SWR function
    val basicSWRScript = UserFunction(
        label = BASIC_LABEL,
        name = "GitHub User Lookup (Native SWR)",
        description = "Fetches GitHub user data with automatic caching using native SWR",
        parameters = mapOf(
            "username" to ParameterDefinition(
                required = true,
                description = "GitHub username to fetch",
                paramType = "String"
            )
        ),
        functions = listOf(
            FunctionStageConfig.SWR(
                cache_key = "github:user:kt:{{username}}",
                ttl = JsonPrimitive("15m"),
                url = "https://api.github.com/users/{{username}}",
                method = "GET",
                headers = mapOf("User-Agent" to "ekoDB-Client"),
                body = null,
                timeout_seconds = null,
                output_field = "user_data",
                collection = null
            )
        ),
        tags = listOf("github", "swr", "native")
    )

    val funcId = saveOrUpdate(client, basicSWRScript)
    println("✓ Created native SWR function: $BASIC_LABEL ($funcId)")

    // First call - cache miss
    println("\nFirst call (cache miss - will fetch from GitHub API):")
    var recordCount1 = 0
    val duration1 = measureTimeMillis {
        val result1 = client.callFunction(BASIC_LABEL, mapOf("username" to JsonPrimitive("torvalds")))
        recordCount1 = result1.records.size
    }
    println("  Response time: ${duration1}ms")
    println("  Records returned: $recordCount1")

    // Second call - cache hit
    println("\nSecond call (cache hit - instant from KV store):")
    val duration2 = measureTimeMillis {
        client.callFunction(BASIC_LABEL, mapOf("username" to JsonPrimitive("torvalds")))
    }
    println("  Response time: ${duration2}ms")
    val speedup = if (duration2 > 0) duration1.toDouble() / duration2.toDouble() else 0.0
    println("  Speedup: ${"%.1f".format(speedup)}x faster 🚀")
    println()

    funcId
}

fun exampleAuditTrail(client: EkoDBClient): String = runBlocking {
    println("\nExample 2: SWR with Built-in Audit Trail")
    println("─".repeat(80))
    println("Optional collection parameter for automatic request logging")

    // Create function with audit trail
    val auditSWRScript = UserFunction(
        label = AUDIT_LABEL,
        name = "Product API with Audit (Native SWR)",
        description = "Caches product data and logs all requests automatically",
        parameters = mapOf(
            "product_id" to ParameterDefinition(
                required = true,
                description = "Product ID to fetch",
                paramType = "String"
            )
        ),
        functions = listOf(
            FunctionStageConfig.SWR(
                cache_key = "product:kt:{{product_id}}",
                ttl = JsonPrimitive("1h"),
                url = "https://fakestoreapi.com/products/{{product_id}}",
                method = "GET",
                headers = null,
                body = null,
                timeout_seconds = null,
                output_field = "product",
                collection = AUDIT_COLLECTION
            )
        ),
        tags = listOf("products", "audit")
    )

    val auditFuncId = saveOrUpdate(client, auditSWRScript)
    println("✓ Created SWR function with audit trail: $AUDIT_LABEL ($auditFuncId)")

    println("\nFetching product (will create audit trail entry):")
    val productResult = client.callFunction(AUDIT_LABEL, mapOf("product_id" to JsonPrimitive("1")))
    println("  ✓ Product fetched and cached")
    println("  ✓ Audit record created in '$AUDIT_COLLECTION' collection")
    println("  Records: ${productResult.records.size}\n")

    auditFuncId
}

fun examplePipelineEnrichment(client: EkoDBClient): String = runBlocking {
    println("\nExample 3: SWR in Multi-Function Pipeline")
    println("─".repeat(80))
    println("Fetch external data → Process → Store in collection")

    // Create enrichment pipeline
    val pipelineScript = UserFunction(
        label = PIPELINE_LABEL,
        name = "User Data Enrichment Pipeline",
        description = "Fetches external API data and stores enriched results",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                required = true,
                description = "User ID to enrich",
                paramType = "String"
            )
        ),
        functions = listOf(
            // Step 1: Fetch from external API with caching (30 min TTL)
            FunctionStageConfig.SWR(
                cache_key = "api:user:kt:{{user_id}}",
                ttl = JsonPrimitive("30m"),
                url = "https://jsonplaceholder.typicode.com/users/{{user_id}}",
                method = "GET",
                headers = null,
                body = null,
                timeout_seconds = null,
                output_field = "user_data",
                collection = null
            ),
            // Step 2: Store enriched data in collection (24 hour TTL)
            FunctionStageConfig.Insert(
                collection = ENRICHED_COLLECTION,
                record = buildJsonObject {
                    put("user_id", buildJsonObject {
                        put("type", JsonPrimitive("String"))
                        put("value", JsonPrimitive("{{user_id}}"))
                    })
                    put("source_data", buildJsonObject {
                        put("type", JsonPrimitive("Object"))
                        put("value", JsonPrimitive("{{user_data}}"))
                    })
                },
                bypass_ripple = false,
                ttl = 86400
            )
        ),
        tags = listOf("enrichment", "pipeline")
    )

    val pipelineScriptId = saveOrUpdate(client, pipelineScript)
    println("✓ Created enrichment pipeline: $PIPELINE_LABEL ($pipelineScriptId)")

    println("\nRunning pipeline:")
    val enrichResult = client.callFunction(PIPELINE_LABEL, mapOf("user_id" to JsonPrimitive("1")))
    println("  ✓ Data fetched from API (cached 30m)")
    println("  ✓ Enriched data stored in '$ENRICHED_COLLECTION' (TTL 24h)")
    println("  Pipeline returned ${enrichResult.records.size} records\n")

    pipelineScriptId
}

fun exampleDynamicTTL(client: EkoDBClient): String = runBlocking {
    println("\nExample 4: Dynamic TTL Configuration")
    println("─".repeat(80))
    println("TTL as parameter - supports duration strings, integers, ISO timestamps")

    // Create function with dynamic TTL
    val dynamicTTLScript = UserFunction(
        label = DYNAMIC_LABEL,
        name = "Flexible Cache TTL (Native SWR)",
        description = "Demonstrates parameterized TTL values",
        parameters = mapOf(
            "resource_id" to ParameterDefinition(
                required = true,
                description = "Resource to fetch",
                paramType = "String"
            ),
            "ttl" to ParameterDefinition(
                required = true,
                description = "Cache duration (e.g., '5m', '1h', '30s')",
                paramType = "String"
            )
        ),
        functions = listOf(
            FunctionStageConfig.SWR(
                cache_key = "resource:kt:{{resource_id}}",
                ttl = JsonPrimitive("{{ttl}}"),
                url = "https://jsonplaceholder.typicode.com/posts/{{resource_id}}",
                method = "GET",
                headers = null,
                body = null,
                timeout_seconds = null,
                output_field = null,
                collection = null
            )
        ),
        tags = listOf("dynamic")
    )

    val dynamicFuncId = saveOrUpdate(client, dynamicTTLScript)
    println("✓ Created dynamic TTL function: $DYNAMIC_LABEL ($dynamicFuncId)")

    // Test with different TTLs
    val ttlTests = listOf(
        "5m" to "5 minutes",
        "1h" to "1 hour",
        "30s" to "30 seconds"
    )

    for ((ttlValue, description) in ttlTests) {
        client.callFunction(
            DYNAMIC_LABEL,
            mapOf(
                "resource_id" to JsonPrimitive("test"),
                "ttl" to JsonPrimitive(ttlValue)
            )
        )
        println("  ✓ Cached with TTL: $ttlValue ($description)")
    }

    dynamicFuncId
}

suspend fun cleanupSwrFunctions(client: EkoDBClient) {
    println("\n🧹 Cleaning up...")
    val errors = mutableListOf<String>()
    var deletedFunctions = 0
    listOf(BASIC_LABEL, AUDIT_LABEL, PIPELINE_LABEL, DYNAMIC_LABEL).forEach { label ->
        try {
            val function = client.getFunction(label)
            val id = function.id ?: error("Function '$label' lookup returned no ID")
            client.deleteFunction(id)
            deletedFunctions += 1
        } catch (e: Exception) {
            if (!isNotFoundError(e)) {
                errors.add("function $label: ${e.message}")
            }
        }
    }
    listOf(
        "github:user:kt:torvalds",
        "product:kt:1",
        "api:user:kt:1",
        "resource:kt:test"
    ).forEach { key ->
        try {
            client.kvDelete(key)
        } catch (e: Exception) {
            if (!isNotFoundError(e)) {
                errors.add("KV key $key: ${e.message}")
            }
        }
    }
    listOf(AUDIT_COLLECTION, ENRICHED_COLLECTION).forEach { collection ->
        try {
            client.deleteCollection(collection)
        } catch (e: Exception) {
            if (!isNotFoundError(e)) {
                errors.add("collection $collection: ${e.message}")
            }
        }
    }
    check(errors.isEmpty()) { "Cleanup failed: ${errors.joinToString("; ")}" }
    println("✓ Deleted $deletedFunctions test functions and owned SWR collections")
}

fun main(): Unit = runBlocking {
    println("🚀 ekoDB Kotlin Client - Native SWR Function Examples\n")
    println("📋 Demonstrates:")
    println("   • Single-function SWR pattern (replaces 4-step pipeline)")
    println("   • Automatic cache checking, HTTP fetching, and cache setting")
    println("   • Built-in audit trail support")
    println("   • Duration string TTLs ('15m', '1h', '30s')")
    println("   • Multi-function pipeline integration")
    println("   • Dynamic TTL configuration\n")

    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    var failure: Throwable? = null
    try {
        cleanupSwrFunctions(client)
        // Run examples
        exampleBasicSWR(client)
        exampleAuditTrail(client)
        examplePipelineEnrichment(client)
        exampleDynamicTTL(client)

        // Summary
        println("\n${"=".repeat(80)}")
        println("✅ Key Benefits of Native SWR:")
        println("✅ Single function: Replaces 4-function cache-aside pattern")
        println("✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds")
        println("✅ Built-in audit: Optional collection parameter for automatic logging")
        println("✅ Auto-enrichment: output_field populates params for downstream functions")
        println("✅ Transactional: Works correctly in both transactional and non-transactional contexts")
        println("✅ KV-optimized: Uses native KV store with proper TTL handling")

        println("\n=== Performance Comparison ===")
        println("Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)")
        println("Native SWR:     SWR → Insert (2 functions)")
        println("Result:         60% fewer functions, cleaner code, same behavior 🎯")
    } catch (error: Throwable) {
        println("❌ Error: ${error.message}")
        failure = error
    } finally {
        try {
            cleanupSwrFunctions(client)
        } catch (cleanupError: Throwable) {
            val primary = failure
            if (primary == null) failure = cleanupError else primary.addSuppressed(cleanupError)
        }
        try {
            client.close()
        } catch (cleanupError: Throwable) {
            val primary = failure
            if (primary == null) failure = cleanupError else primary.addSuppressed(cleanupError)
        }
    }

    failure?.let { throw it }

    println("\n✅ All examples completed!")
}
