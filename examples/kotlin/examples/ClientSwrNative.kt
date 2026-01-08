package io.ekodb.examples

/**
 * ekoDB Kotlin Client - Native SWR Function Example
 *
 * Demonstrates the native SWR (Stale-While-Revalidate) function that
 * encapsulates the entire cache-aside pattern in a single operation.
 */

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.ParameterDefinition
import io.ekodb.client.functions.Script
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlin.system.measureTimeMillis

fun exampleBasicSWR(client: EkoDBClient): String = runBlocking {
    println("\nExample 1: Basic Native SWR")
    println("─".repeat(80))
    println("Single function replaces KvGet → If → HttpRequest → KvSet pipeline")

    // Create script with native SWR function
    val basicSWRScript = Script(
        label = "github_user_native",
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
                cache_key = "github:user:{{username}}",
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

    val scriptId = client.saveScript(basicSWRScript)
    println("✓ Created native SWR script: github_user_native ($scriptId)")

    // First call - cache miss
    println("\nFirst call (cache miss - will fetch from GitHub API):")
    var recordCount1 = 0
    val duration1 = measureTimeMillis {
        val result1 = client.callScript("github_user_native", mapOf("username" to JsonPrimitive("torvalds")))
        recordCount1 = result1.records.size
    }
    println("  Response time: ${duration1}ms")
    println("  Records returned: $recordCount1")

    // Second call - cache hit
    println("\nSecond call (cache hit - instant from KV store):")
    val duration2 = measureTimeMillis {
        client.callScript("github_user_native", mapOf("username" to JsonPrimitive("torvalds")))
    }
    println("  Response time: ${duration2}ms")
    val speedup = if (duration2 > 0) duration1.toDouble() / duration2.toDouble() else 0.0
    println("  Speedup: ${"%.1f".format(speedup)}x faster 🚀")
    println()

    scriptId
}

fun exampleAuditTrail(client: EkoDBClient): String = runBlocking {
    println("\nExample 2: SWR with Built-in Audit Trail")
    println("─".repeat(80))
    println("Optional collection parameter for automatic request logging")

    // Create script with audit trail
    val auditSWRScript = Script(
        label = "product_swr_audit",
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
                cache_key = "product:{{product_id}}",
                ttl = JsonPrimitive("1h"),
                url = "https://fakestoreapi.com/products/{{product_id}}",
                method = "GET",
                headers = null,
                body = null,
                timeout_seconds = null,
                output_field = "product",
                collection = "swr_audit_trail"
            )
        ),
        tags = listOf("products", "audit")
    )

    val auditScriptId = client.saveScript(auditSWRScript)
    println("✓ Created SWR script with audit trail: product_swr_audit ($auditScriptId)")

    println("\nFetching product (will create audit trail entry):")
    val productResult = client.callScript("product_swr_audit", mapOf("product_id" to JsonPrimitive("1")))
    println("  ✓ Product fetched and cached")
    println("  ✓ Audit record created in 'swr_audit_trail' collection")
    println("  Records: ${productResult.records.size}\n")

    auditScriptId
}

fun examplePipelineEnrichment(client: EkoDBClient): String = runBlocking {
    println("\nExample 3: SWR in Multi-Function Pipeline")
    println("─".repeat(80))
    println("Fetch external data → Process → Store in collection")

    // Create enrichment pipeline
    val pipelineScript = Script(
        label = "user_enrichment_pipeline",
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
                cache_key = "api:user:{{user_id}}",
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
                collection = "enriched_users",
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

    val pipelineScriptId = client.saveScript(pipelineScript)
    println("✓ Created enrichment pipeline: user_enrichment_pipeline ($pipelineScriptId)")

    println("\nRunning pipeline:")
    val enrichResult = client.callScript("user_enrichment_pipeline", mapOf("user_id" to JsonPrimitive("1")))
    println("  ✓ Data fetched from API (cached 30m)")
    println("  ✓ Enriched data stored in 'enriched_users' (TTL 24h)")
    println("  Pipeline returned ${enrichResult.records.size} records\n")

    pipelineScriptId
}

fun exampleDynamicTTL(client: EkoDBClient): String = runBlocking {
    println("\nExample 4: Dynamic TTL Configuration")
    println("─".repeat(80))
    println("TTL as parameter - supports duration strings, integers, ISO timestamps")

    // Create script with dynamic TTL
    val dynamicTTLScript = Script(
        label = "flexible_cache",
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
                cache_key = "resource:{{resource_id}}",
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

    val dynamicScriptId = client.saveScript(dynamicTTLScript)
    println("✓ Created dynamic TTL script: flexible_cache ($dynamicScriptId)")

    // Test with different TTLs
    val ttlTests = listOf(
        "5m" to "5 minutes",
        "1h" to "1 hour",
        "30s" to "30 seconds"
    )

    for ((ttlValue, description) in ttlTests) {
        client.callScript(
            "flexible_cache",
            mapOf(
                "resource_id" to JsonPrimitive("test"),
                "ttl" to JsonPrimitive(ttlValue)
            )
        )
        println("  ✓ Cached with TTL: $ttlValue ($description)")
    }

    dynamicScriptId
}

suspend fun cleanup(client: EkoDBClient, scriptIds: List<String>) {
    println("\n🧹 Cleaning up...")
    try {
        scriptIds.forEach { scriptId ->
            client.deleteScript(scriptId)
        }
        println("✓ Deleted ${scriptIds.size} test scripts")
    } catch (e: Exception) {
        println("⚠ Cleanup error (non-critical): ${e.message}")
    }
}

fun main() = runBlocking {
    println("🚀 ekoDB Kotlin Client - Native SWR Function Examples\n")
    println("📋 Demonstrates:")
    println("   • Single-function SWR pattern (replaces 4-step pipeline)")
    println("   • Automatic cache checking, HTTP fetching, and cache setting")
    println("   • Built-in audit trail support")
    println("   • Duration string TTLs ('15m', '1h', '30s')")
    println("   • Multi-function pipeline integration")
    println("   • Dynamic TTL configuration\n")

    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    val scriptIds = mutableListOf<String>()

    try {
        // Run examples
        scriptIds.add(exampleBasicSWR(client))
        scriptIds.add(exampleAuditTrail(client))
        scriptIds.add(examplePipelineEnrichment(client))
        scriptIds.add(exampleDynamicTTL(client))

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
    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    } finally {
        cleanup(client, scriptIds)
    }

    println("\n✅ All examples completed!")
}
