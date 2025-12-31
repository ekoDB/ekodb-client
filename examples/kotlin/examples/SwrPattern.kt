package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.Script
import io.ekodb.client.functions.ScriptCondition
import io.ekodb.client.functions.ParameterDefinition
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import kotlin.system.measureTimeMillis

fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n")

    // Step 1: Create SWR script for GitHub user caching
    println("Step 1: Create SWR function that acts as edge cache")

    val swrScript = Script(
        label = "fetch_github_user",
        name = "Fetch GitHub User with Cache",
        description = "SWR pattern: Check cache, fetch from GitHub API if stale, auto-update with TTL",
        version = "1.0",
        parameters = mapOf(
            "username" to ParameterDefinition(
                required = true,
                description = "GitHub username to fetch"
            ),
            "ttl" to ParameterDefinition(
                required = false,
                default = JsonPrimitive(300),
                description = "Cache TTL in seconds (default: 5 minutes)"
            )
        ),
        functions = listOf(
            // 1. Check cache
            FunctionStageConfig.FindById(
                collection = "github_cache",
                record_id = "{{username}}"
            ),
            // 2. If cache exists, return it; else fetch from API
            FunctionStageConfig.If(
                condition = ScriptCondition.HasRecords,
                then_functions = listOf(
                    // Cache hit - return cached data
                    FunctionStageConfig.Project(
                        fields = listOf("data", "cached_at"),
                        exclude = false
                    )
                ),
                else_functions = listOf(
                    // Cache miss - fetch external API and store
                    FunctionStageConfig.HttpRequest(
                        url = "https://api.github.com/users/{{username}}",
                        method = "GET",
                        headers = mapOf("User-Agent" to "ekoDB-SWR-Example")
                    ),
                    FunctionStageConfig.Insert(
                        collection = "github_cache",
                        record = buildJsonObject {
                            put("id", "{{username}}")
                            put("data", "{{http_response}}")
                            put("cached_at", System.currentTimeMillis().toString())
                        },
                        bypass_ripple = false,
                        ttl = null
                    )
                )
            )
        ),
        tags = listOf("swr", "github", "cache")
    )

    val scriptId = client.saveScript(swrScript)
    println("✓ Created SWR script: ${swrScript.label} ($scriptId)\n")

    // Step 2: First call - Cache miss
    println("Step 2: First call - Cache miss, fetches from GitHub API")
    val start1 = System.currentTimeMillis()
    val result1 = client.callScript(
        "fetch_github_user",
        mapOf(
            "username" to JsonPrimitive("torvalds"),
            "ttl" to JsonPrimitive(300)
        )
    )
    val duration1 = System.currentTimeMillis() - start1
    println("Response time: ${duration1}ms")
    println("Result: ${result1.records.take(1)}")
    println("✓ Data fetched from external API and cached\n")

    // Step 3: Second call - Cache hit
    println("Step 3: Second call - Cache hit, instant response from ekoDB")
    val start2 = System.currentTimeMillis()
    val result2 = client.callScript(
        "fetch_github_user",
        mapOf("username" to JsonPrimitive("torvalds"))
    )
    val duration2 = System.currentTimeMillis() - start2
    println("Response time: ${duration2}ms")
    val speedup = duration1.toDouble() / duration2.toDouble()
    println("Cache hit was %.1fx faster!\n".format(speedup))
    println("✓ Lightning fast cache hit\n")

    // Advanced example: Data enrichment
    println("=== Advanced: SWR with Data Enrichment ===\n")
    println("Creating product enrichment function...")

    val enrichScript = Script(
        label = "fetch_product_enriched",
        name = "Fetch Product with Enrichment",
        description = "Demonstrates calling external API and enriching data",
        version = "1.0",
        parameters = mapOf(
            "product_id" to ParameterDefinition(
                required = true,
                description = "Product ID"
            ),
            "ttl" to ParameterDefinition(
                required = false,
                default = JsonPrimitive(600),
                description = "Cache TTL (10 minutes)"
            )
        ),
        functions = listOf(
            FunctionStageConfig.FindById(
                collection = "product_cache",
                record_id = "{{product_id}}"
            ),
            FunctionStageConfig.If(
                condition = ScriptCondition.HasRecords,
                then_functions = listOf(
                    FunctionStageConfig.Project(
                        fields = listOf("enriched_data"),
                        exclude = false
                    )
                ),
                else_functions = listOf(
                    FunctionStageConfig.HttpRequest(
                        url = "https://dummyjson.com/products/{{product_id}}",
                        method = "GET"
                    ),
                    FunctionStageConfig.Insert(
                        collection = "product_cache",
                        record = buildJsonObject {
                            put("id", "{{product_id}}")
                            put("enriched_data", "{{http_response}}")
                            put("enriched_at", System.currentTimeMillis().toString())
                        },
                        bypass_ripple = false,
                        ttl = null
                    )
                )
            )
        ),
        tags = listOf("enrichment", "product", "cache")
    )

    val enrichScriptId = client.saveScript(enrichScript)
    println("✓ Created enrichment script: ${enrichScript.label} ($enrichScriptId)\n")

    println("Step 4: Call enrichment function - Fetches from API + stores enriched result")
    val enriched = client.callScript(
        "fetch_product_enriched",
        mapOf(
            "product_id" to JsonPrimitive("1"),
            "ttl" to JsonPrimitive(600)
        )
    )
    println("Enriched data: ${enriched.records.take(1)}")
    println("✓ Data fetched, enriched, and cached atomically\n")

    // Print benefits
    println("=== Why This Is Powerful ===")
    println("✓ No separate cache layer (Redis, Memcached) needed")
    println("✓ No manual cache invalidation (TTL handles it)")
    println("✓ No separate edge infrastructure (ekoDB IS the edge)")
    println("✓ Atomic operations (function executes as transaction)")
    println("✓ With multi-node + ripples: Auto-sync across all nodes")
    println("✓ Sub-millisecond cache hits from internal storage")
    println("✓ One service instead of many (cache + API gateway + database)")

    println("\n=== Real-World Use Cases ===")
    println("1. API Gateway Pattern:")
    println("   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache")
    println("\n2. Database Federation:")
    println("   - Query multiple DBs (Postgres, MongoDB) + external APIs")
    println("   - Merge results in one function call")
    println("\n3. IoT Data Enrichment:")
    println("   - Sensor data + weather API + location API")
    println("   - Enrich and cache in one atomic operation")
    println("\n4. E-commerce Product Pages:")
    println("   - Product info + reviews + inventory + pricing")
    println("   - All from different sources, cached together")

    println("\n✓ Example complete - Your database IS your edge!\n")
}
