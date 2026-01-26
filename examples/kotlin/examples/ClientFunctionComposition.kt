/**
 * Function Composition Example for ekoDB Kotlin Client
 *
 * Demonstrates calling Functions within Functions using CallFunction
 * Shows how to build reusable logic blocks and compose complex workflows
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.*
import io.ekodb.client.types.FieldType
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

    println("=== ekoDB Function Composition Examples ===\n")

    setupTestData(client)
    basicCompositionExample(client)
    swrCompositionExample(client)
    nestedCompositionExample(client)

    println("\n✅ All composition examples completed!")
}

suspend fun setupTestData(client: EkoDBClient) {
    println("📋 Setting up test data...\n")

    for (i in 1..5) {
        val record = mapOf(
            "id" to FieldType.String("user_$i"),
            "name" to FieldType.String("User $i"),
            "department" to FieldType.String(if (i <= 2) "engineering" else "sales"),
            "score" to FieldType.Integer(i * 20)
        )
        client.insert("users", record)
    }

    println("✅ Test data ready\n")
}

suspend fun basicCompositionExample(client: EkoDBClient) {
    println("📝 Example 1: Basic Function Composition\n")
    println("Building reusable functions that call each other...\n")

    // Step 1: Create reusable "fetch_user" function
    val fetchUser = Script(
        label = "fetch_user",
        name = "Fetch user by ID",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                name = "user_id",
                required = true
            )
        ),
        functions = listOf(
            Function.FindById(
                collection = "users",
                recordId = "{{user_id}}"
            )
        )
    )

    client.saveScript(fetchUser)
    println("✅ Saved reusable function: fetch_user")

    // Step 2: Create wrapper that CALLS fetch_user
    val getUserWrapper = Script(
        label = "get_user_wrapper",
        name = "Wrapper that calls fetch_user",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                name = "user_id",
                required = true
            )
        ),
        functions = listOf(
            Function.CallFunction(
                functionLabel = "fetch_user",
                params = mapOf("user_id" to "{{user_id}}")
            ),
            Function.Project(
                fields = listOf("name", "department"),
                exclude = false
            )
        )
    )

    client.saveScript(getUserWrapper)
    println("✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)\n")

    // Step 3: Call the composed function
    val params = mapOf("user_id" to FieldType.String("user_1"))
    val result = client.callScript("get_user_wrapper", params)

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

suspend fun swrCompositionExample(client: EkoDBClient) {
    println("📝 Example 2: SWR Pattern with Function Composition\n")
    println("Using CallFunction to replace inline logic in SWR pattern...\n")

    // Step 1: Create reusable fetch and store function
    // Using jsonplaceholder.typicode.com - a reliable free API for testing
    val fetchAndStore = Script(
        label = "fetch_and_store_user",
        name = "Fetch user from API and cache",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                name = "user_id",
                required = true
            )
        ),
        functions = listOf(
            Function.HttpRequest(
                url = "https://jsonplaceholder.typicode.com/users/{{user_id}}",
                method = "GET",
                headers = mapOf("Accept" to "application/json")
            ),
            Function.Insert(
                collection = "user_cache",
                record = mapOf(
                    "cache_key" to mapOf("type" to "String", "value" to "{{user_id}}"),
                    "data" to mapOf("type" to "Object", "value" to "{{http_response}}")
                ),
                ttl = 300 // 5 minute cache
            )
        )
    )

    client.saveScript(fetchAndStore)
    println("✅ Saved reusable function: fetch_and_store_user")

    // Step 2: Create SWR function that CALLS the reusable function
    val swrUser = Script(
        label = "swr_user",
        name = "SWR pattern for user data",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                name = "user_id",
                required = true
            )
        ),
        functions = listOf(
            Function.FindById(
                collection = "user_cache",
                recordId = "{{user_id}}"
            ),
            Function.If(
                condition = ScriptCondition.HasRecords,
                thenFunctions = listOf(
                    Function.Project(
                        fields = listOf("data"),
                        exclude = false
                    )
                ),
                elseFunctions = listOf(
                    Function.CallFunction(
                        functionLabel = "fetch_and_store_user",
                        params = mapOf("user_id" to "{{user_id}}")
                    )
                )
            )
        )
    )

    client.saveScript(swrUser)
    println("✅ Saved SWR function using composition: swr_user\n")

    // Step 3: Test cache miss
    println("First call (cache miss - will fetch from API):")
    val params = mapOf("user_id" to FieldType.String("1"))

    val duration1 = measureTimeMillis {
        val result1 = client.callScript("swr_user", params)
        println("   ⏱️  Duration: ${duration1}ms")
        println("   📊 Records: ${result1.records.size}\n")
    }

    // Step 4: Test cache hit
    println("Second call (cache hit - from cache):")
    val duration2 = measureTimeMillis {
        val result2 = client.callScript("swr_user", params)
        println("   ⏱️  Duration: ${duration2}ms")
        println("   📊 Records: ${result2.records.size}")
        if (duration2 > 0) {
            val speedup = duration1.toDouble() / duration2.toDouble()
            println("   🚀 Cache speedup: ${"%.1f".format(speedup)}x faster!\n")
        }
    }
}

suspend fun nestedCompositionExample(client: EkoDBClient) {
    println("📝 Example 3: Multi-Level Function Composition\n")
    println("Building complex workflows from small, reusable pieces...\n")

    // Level 1: Base function
    val validateUser = Script(
        label = "validate_user",
        name = "Check if user exists",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                name = "user_id",
                required = true
            )
        ),
        functions = listOf(
            Function.FindById(
                collection = "users",
                recordId = "{{user_id}}"
            )
        )
    )

    client.saveScript(validateUser)
    println("✅ Level 1 function: validate_user")

    // Level 2: Calls validate_user + projects
    val fetchSlim = Script(
        label = "fetch_slim_user",
        name = "Validate and slim down user",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                name = "user_id",
                required = true
            )
        ),
        functions = listOf(
            Function.CallFunction(
                functionLabel = "validate_user",
                params = mapOf("user_id" to "{{user_id}}")
            ),
            Function.Project(
                fields = listOf("name", "department"),
                exclude = false
            )
        )
    )

    client.saveScript(fetchSlim)
    println("✅ Level 2 function: fetch_slim_user (calls validate_user)")

    // Level 3: Calls fetch_slim (demonstrates 3-level nesting)
    val getVerifiedUser = Script(
        label = "get_verified_user",
        name = "Get verified and validated user",
        parameters = mapOf(
            "user_id" to ParameterDefinition(
                name = "user_id",
                required = true
            )
        ),
        functions = listOf(
            Function.CallFunction(
                functionLabel = "fetch_slim_user",
                params = mapOf("user_id" to "{{user_id}}")
            )
        )
    )

    client.saveScript(getVerifiedUser)
    println("✅ Level 3 function: get_verified_user (calls fetch_slim_user)\n")

    // Execute 3-level nested composition
    val params = mapOf("user_id" to FieldType.String("user_1"))
    val result = client.callScript("get_verified_user", params)

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
