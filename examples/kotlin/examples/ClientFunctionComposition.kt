/**
 * Function Composition Example for ekoDB Kotlin Client
 *
 * Demonstrates calling Functions within Functions using CallFunction
 * Shows how to build reusable logic blocks and compose complex workflows
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.*
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonNull
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlin.system.measureTimeMillis

private const val TEST_COLLECTION = "function_composition_users_kt"
private const val FETCH_USER_LABEL = "fc_fetch_user_kt"
private const val GET_USER_WRAPPER_LABEL = "fc_get_user_wrapper_kt"
private const val FETCH_AND_STORE_LABEL = "fc_fetch_store_user_kt"
private const val SWR_USER_LABEL = "fc_swr_user_kt"
private const val VALIDATE_USER_LABEL = "fc_validate_user_kt"
private const val FETCH_SLIM_LABEL = "fc_fetch_slim_user_kt"
private const val GET_VERIFIED_LABEL = "fc_get_verified_user_kt"
private const val USER_CACHE_KEY = "fc:user_cache_kt:1"

private fun isAlreadyExistsError(e: Exception): Boolean {
    val msg = e.message ?: return false
    return msg.contains("status 409") || msg.contains("already exists")
}

private fun isNotFoundError(error: Throwable): Boolean =
    error.message?.let { it.contains("status 404") || it.contains("not found", ignoreCase = true) } == true

private suspend fun saveOrUpdate(
    client: EkoDBClient,
    func: UserFunction,
    functionIds: MutableSet<String>
): String {
    val id = try {
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
    functionIds += id
    return id
}

fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB Function Composition Examples ===\n")
    val functionIds = linkedSetOf<String>()
    var failure: Throwable? = null
    try {
        setupCompositionTestData(client)
        basicCompositionExample(client, functionIds)
        swrCompositionExample(client, functionIds)
        nestedCompositionExample(client, functionIds)
    } catch (error: Throwable) {
        println("❌ Error: ${error.message}")
        failure = error
    } finally {
        println("🧹 Cleaning up...")
        for (id in functionIds.reversed()) {
            try {
                client.deleteFunction(id)
            } catch (cleanupError: Throwable) {
                if (!isNotFoundError(cleanupError)) {
                    failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
                }
            }
        }
        try {
            client.kvDelete(USER_CACHE_KEY)
        } catch (cleanupError: Throwable) {
            if (!isNotFoundError(cleanupError)) {
                failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
            }
        }
        try {
            client.deleteCollection(TEST_COLLECTION)
        } catch (cleanupError: Throwable) {
            if (!isNotFoundError(cleanupError)) {
                failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
            }
        }
        try {
            client.close()
        } catch (cleanupError: Throwable) {
            failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
        }
        if (failure == null) println("✅ Cleanup complete")
    }
    failure?.let { throw it }
    println("\n✅ All composition examples completed!")
}

suspend fun setupCompositionTestData(client: EkoDBClient) {
    println("📋 Setting up test data...\n")

    try {
        client.deleteCollection(TEST_COLLECTION)
    } catch (error: Throwable) {
        if (!isNotFoundError(error)) throw error
    }

    for (i in 1..5) {
        val record = Record.new()
            .insert("id", "user_$i")
            .insert("name", "User $i")
            .insert("department", if (i <= 2) "engineering" else "sales")
            .insert("score", i * 20)
        client.insert(TEST_COLLECTION, record)
    }

    check(client.countDocuments(TEST_COLLECTION) == 5L) { "Expected exactly 5 composition users" }

    println("✅ Test data ready\n")
}

suspend fun basicCompositionExample(client: EkoDBClient, functionIds: MutableSet<String>) {
    println("📝 Example 1: Basic Function Composition\n")
    println("Building reusable functions that call each other...\n")

    // Step 1: Create reusable "fetch_user" function
    val fetchUser = UserFunction(
        label = FETCH_USER_LABEL,
        name = "Fetch user by ID",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                required = true
            )
        ),
        functions = listOf(
            FunctionStageConfig.FindById(
                collection = TEST_COLLECTION,
                record_id = "{{user_id}}"
            )
        )
    )

    saveOrUpdate(client, fetchUser, functionIds)
    println("✅ Saved reusable function: fetch_user")

    // Step 2: Create wrapper that CALLS fetch_user
    val getUserWrapper = UserFunction(
        label = GET_USER_WRAPPER_LABEL,
        name = "Wrapper that calls fetch_user",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                required = true
            )
        ),
        functions = listOf(
            FunctionStageConfig.CallFunction(
                function_label = FETCH_USER_LABEL,
                params = null // Inherits user_id from parent scope
            ),
            FunctionStageConfig.Project(
                fields = listOf("name", "department"),
                exclude = false
            )
        )
    )

    saveOrUpdate(client, getUserWrapper, functionIds)
    println("✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)\n")

    // Step 3: Call the composed function
    val params = mapOf("user_id" to JsonPrimitive("user_1"))
    val result = client.callFunction(GET_USER_WRAPPER_LABEL, params)
    check(result.records.size == 1) { "Expected one composed user result" }

    println("📊 Result from composed function:")
    println("   Records: ${result.records.size}")
    if (result.records.isNotEmpty()) {
        val record = result.records[0]
        println("   Name: ${record["name"]}")
        println("   Department: ${record["department"]}\n")
    }

    println("🎯 Key Benefit: fetch_user can be reused by ANY function!")
    println("   No code duplication, single source of truth\n")
}

suspend fun swrCompositionExample(client: EkoDBClient, functionIds: MutableSet<String>) {
    println("📝 Example 2: SWR Pattern with Function Composition\n")
    println("Using KV cache + CallFunction for fast cache-aside pattern...\n")

    // Step 1: Create reusable fetch and store function
    // Using jsonplaceholder.typicode.com - a reliable free API for testing
    // This function fetches from API and stores in KV cache
    val fetchAndStore = UserFunction(
        label = FETCH_AND_STORE_LABEL,
        name = "Fetch user from API and cache in KV",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                required = true
            )
        ),
        functions = listOf(
            FunctionStageConfig.HttpRequest(
                url = "https://jsonplaceholder.typicode.com/users/{{user_id}}",
                method = "GET",
                headers = mapOf("Accept" to "application/json"),
                timeout_seconds = 15,
                output_field = "fetched_user"
            ),
            // Store in KV cache (much faster than collection for cache lookups)
            FunctionStageConfig.KvSet(
                key = "fc:user_cache_kt:{{user_id}}",
                value = kotlinx.serialization.json.JsonPrimitive("{{fetched_user}}"),
                ttl = 300 // 5 minute cache
            )
        )
    )

    saveOrUpdate(client, fetchAndStore, functionIds)
    println("✅ Saved reusable function: fetch_and_store_user (uses KV)")

    // Step 2: Create SWR function that CALLS the reusable function
    // Pattern: KV cache check → populate if missing → return
    val swrUser = UserFunction(
        label = SWR_USER_LABEL,
        name = "SWR pattern for user data (KV-based)",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                required = true
            )
        ),
        functions = listOf(
            // Check KV cache first (O(1) lookup - much faster than FindById)
            FunctionStageConfig.KvGet(
                key = "fc:user_cache_kt:{{user_id}}"
            ),
            FunctionStageConfig.If(
                // KvGet returns { value: ... } on hit, { value: null } on miss
                // So we check if "value" is not null to detect cache hit
                condition = FunctionCondition.Not(
                    FunctionCondition.FieldEquals(field = "value", fieldValue = JsonNull)
                ),
                then_functions = listOf(
                    // Cache hit - project the value field
                    FunctionStageConfig.Project(
                        fields = listOf("value"),
                        exclude = false
                    )
                ),
                else_functions = listOf(
                    // Cache miss - call reusable function to fetch and store
                    // Explicitly pass user_id to the function
                    FunctionStageConfig.CallFunction(
                        function_label = FETCH_AND_STORE_LABEL,
                        params = buildJsonObject {
                            put("user_id", "{{user_id}}")
                        }
                    )
                )
            )
        )
    )

    saveOrUpdate(client, swrUser, functionIds)
    println("✅ Saved SWR function using composition: swr_user\n")

    // Step 3: Test cache miss
    println("First call (cache miss - will fetch from API):")
    val params = mapOf("user_id" to JsonPrimitive("1"))

    lateinit var result1: io.ekodb.client.functions.FunctionResult
    val duration1 = measureTimeMillis {
        result1 = client.callFunction(SWR_USER_LABEL, params)
    }
    check(result1.records.size == 1) { "Expected one cache-miss result" }
    println("   ⏱️  Duration: ${duration1}ms")
    println("   📊 Records: ${result1.records.size}\n")

    // Step 4: Test cache hit
    println("Second call (cache hit - from cache):")
    lateinit var result2: io.ekodb.client.functions.FunctionResult
    val duration2 = measureTimeMillis {
        result2 = client.callFunction(SWR_USER_LABEL, params)
    }
    check(result2.records.size == 1) { "Expected one cache-hit result" }
    println("   ⏱️  Duration: ${duration2}ms")
    println("   📊 Records: ${result2.records.size}")
    if (duration2 > 0) {
        val speedup = duration1.toDouble() / duration2.toDouble()
        println("   🚀 Cache speedup: ${"%.1f".format(speedup)}x faster!\n")
    }
}

suspend fun nestedCompositionExample(client: EkoDBClient, functionIds: MutableSet<String>) {
    println("📝 Example 3: Multi-Level Function Composition\n")
    println("Building complex workflows from small, reusable pieces...\n")

    // Level 1: Base function
    val validateUser = UserFunction(
        label = VALIDATE_USER_LABEL,
        name = "Check if user exists",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                required = true
            )
        ),
        functions = listOf(
            FunctionStageConfig.FindById(
                collection = TEST_COLLECTION,
                record_id = "{{user_id}}"
            )
        )
    )

    saveOrUpdate(client, validateUser, functionIds)
    println("✅ Level 1 function: validate_user")

    // Level 2: Calls validate_user + projects
    val fetchSlim = UserFunction(
        label = FETCH_SLIM_LABEL,
        name = "Validate and slim down user",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                required = true
            )
        ),
        functions = listOf(
            FunctionStageConfig.CallFunction(
                function_label = VALIDATE_USER_LABEL,
                params = null // Inherits user_id from parent scope
            ),
            FunctionStageConfig.Project(
                fields = listOf("name", "department"),
                exclude = false
            )
        )
    )

    saveOrUpdate(client, fetchSlim, functionIds)
    println("✅ Level 2 function: fetch_slim_user (calls validate_user)")

    // Level 3: Calls fetch_slim (demonstrates 3-level nesting)
    val getVerifiedUser = UserFunction(
        label = GET_VERIFIED_LABEL,
        name = "Get verified and validated user",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                required = true
            )
        ),
        functions = listOf(
            FunctionStageConfig.CallFunction(
                function_label = FETCH_SLIM_LABEL,
                params = null // Inherits user_id from parent scope
            )
        )
    )

    saveOrUpdate(client, getVerifiedUser, functionIds)
    println("✅ Level 3 function: get_verified_user (calls fetch_slim_user)\n")

    // Execute 3-level nested composition
    val params = mapOf("user_id" to JsonPrimitive("user_1"))
    val result = client.callFunction(GET_VERIFIED_LABEL, params)
    check(result.records.size == 1) { "Expected one nested composition result" }

    println("📊 Result from 3-level nested composition:")
    println("   Records: ${result.records.size}")
    if (result.records.isNotEmpty()) {
        val record = result.records[0]
        println("   Name: ${record["name"]}")
        println("   Department: ${record["department"]}\n")
    }

    println("🎯 Key Benefit: Each function is independently testable and reusable!")
    println("   - validate_user: Used in 100 different workflows")
    println("   - fetch_slim_user: Used in 50 workflows")
    println("   - get_verified_user: Specific workflow\n")
}
