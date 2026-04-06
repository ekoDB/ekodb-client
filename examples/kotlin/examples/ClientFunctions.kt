/**
 * Functions Example for ekoDB Kotlin Client
 *
 * Demonstrates creating, managing, and executing functions with the Kotlin client.
 * Covers: FindAll, Group, Project, Count, and Function management operations.
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.UserFunction
import io.ekodb.client.functions.ParameterDefinition
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.GroupFunctionConfig
import io.ekodb.client.functions.GroupFunctionOp
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject

suspend fun setupTestData(client: EkoDBClient) {
    println("📋 Setting up test data...")

    for (i in 1..10) {
        val record = Record.new()
            .insert("name", "User $i")
            .insert("age", 20 + i)
            .insert("status", if (i % 2 == 0) "active" else "inactive")
            .insert("score", i * 10)
        client.insert("users", record)
    }

    println("✅ Test data ready\n")
}

suspend fun simpleQueryScript(client: EkoDBClient): String {
    println("📝 Example 1: Simple Query Function\n")

    val func = UserFunction(
        label = "get_active_users",
        name = "Get Active Users",
        description = "Retrieve all active users",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = "users")
        ),
        tags = listOf("users", "query")
    )

    val funcId = client.saveFunction(func)
    println("✅ Function saved: $funcId")

    val result = client.callFunction("get_active_users")
    println("📊 Found ${result.records.size} records")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
    
    return funcId
}

suspend fun parameterizedScript(client: EkoDBClient) {
    println("📝 Example 2: Parameterized Function\n")

    val func = UserFunction(
        label = "get_users_by_status",
        name = "Get Users By Status",
        version = "1.0",
        parameters = mapOf(
            "status" to ParameterDefinition(
                required = false,
                default = JsonPrimitive("active"),
                description = "Filter by user status"
            ),
            "limit" to ParameterDefinition(
                required = false,
                default = JsonPrimitive(10),
                description = "Maximum number of results"
            )
        ),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = "users")
        ),
        tags = listOf("users", "parameterized")
    )

    client.saveFunction(func)
    println("✅ Function saved")

    val params = mapOf(
        "status" to JsonPrimitive("active"),
        "limit" to JsonPrimitive(3)
    )
    val result = client.callFunction("get_users_by_status", params)
    println("📊 Found ${result.records.size} users (limited)")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
}

suspend fun aggregationScript(client: EkoDBClient): String {
    println("📝 Example 3: Aggregation Function\n")

    val func = UserFunction(
        label = "user_stats",
        name = "User Statistics",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = "users"),
            FunctionStageConfig.Group(
                by_fields = listOf("status"),
                functions = listOf(
                    GroupFunctionConfig(
                        output_field = "count",
                        operation = GroupFunctionOp.Count
                    ),
                    GroupFunctionConfig(
                        output_field = "avg_score",
                        operation = GroupFunctionOp.Average,
                        input_field = "score"
                    )
                )
            )
        ),
        tags = listOf("analytics")
    )

    val funcId = client.saveFunction(func)
    println("✅ Function saved")

    val result = client.callFunction("user_stats")
    println("📊 Statistics: ${result.records.size} groups")
    result.records.forEach { record ->
        println("   $record")
    }
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
    
    return funcId
}

suspend fun functionManagement(client: EkoDBClient, getActiveUsersId: String, userStatsId: String) {
    println("📝 Example 4: function Management\n")

    // List all functions (skip if deserialization fails due to parameter placeholders)
    try {
        val funcs = client.listFunctions()
        println("📋 Total functions: ${funcs.size}")
    } catch (e: Exception) {
        println("📋 function listing skipped (some functions contain parameter placeholders)")
    }

    // Get specific function (use encrypted ID)
    val func = client.getFunction(getActiveUsersId)
    println("🔍 Retrieved function: ${func.name}")

    // Update function (use encrypted ID)
    val updated = UserFunction(
        label = "get_active_users",
        name = "Get Active Users (Updated)",
        description = "Updated description",
        version = "1.1",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = "users")
        ),
        tags = listOf("users")
    )
    client.updateFunction(getActiveUsersId, updated)
    println("✏️  function updated")

    // Delete function (use ID) - handle error gracefully
    try {
        client.deleteFunction(userStatsId)
        println("🗑️  function deleted")
    } catch (e: Exception) {
        println("ℹ️  function delete skipped (may not exist)")
    }
    println()
    
    println("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
    println("ℹ️  Only CALL can use either ID or label\n")
}

suspend fun multiStageScript(client: EkoDBClient) {
    println("📝 Example 5: Multi-Stage Pipeline\n")

    val func = UserFunction(
        label = "top_users",
        name = "Top Performing Users",
        version = "1.0",
        parameters = mapOf(
            "min_score" to ParameterDefinition(
                required = false,
                default = JsonPrimitive(50)
            )
        ),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = "users"),
            FunctionStageConfig.Project(
                fields = listOf("name", "score", "status"),
                exclude = false
            )
        ),
        tags = listOf("analytics", "reporting")
    )

    client.saveFunction(func)
    println("✅ Multi-stage function saved")

    val result = client.callFunction("top_users", mapOf("min_score" to JsonPrimitive(50)))
    println("📊 Pipeline executed ${result.stats.stages_executed} stages")
    println("⏱️  Total execution time: ${result.stats.execution_time_ms}ms")
    println("📈 Stage breakdown:")
    result.stats.stage_stats.forEachIndexed { index, stage ->
        println("   ${index + 1}. ${stage.stage}: ${stage.execution_time_ms}ms (${stage.input_count} → ${stage.output_count} records)")
    }
    println()
}

suspend fun countScript(client: EkoDBClient) {
    println("📝 Example 6: Count Users\n")

    val func = UserFunction(
        label = "count_users",
        name = "Count All Users",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = "users"),
            FunctionStageConfig.Count(output_field = "count")
        ),
        tags = listOf("users", "count")
    )

    client.saveFunction(func)
    println("✅ Count function saved")

    val result = client.callFunction("count_users")
    val count = result.records.firstOrNull()?.get("count")
    println("📊 Total user count: $count")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
}

suspend fun cleanup(client: EkoDBClient) {
    println("🧹 Cleaning up...")

    // Delete test collection
    client.deleteCollection("users")
    println("✅ Deleted collection")

    // List and delete all test functions (skip if deserialization fails)
    try {
        val funcs = client.listFunctions()
        for (fn in funcs) {
            if (fn.label.startsWith("get_") || fn.label.startsWith("user_") ||
                fn.label.startsWith("top_") || fn.label.startsWith("count_")) {
                try {
                    fn.id?.let { client.deleteFunction(it) }
                } catch (e: Exception) {
                    // function might already be deleted
                }
            }
        }
        println("✅ Deleted test functions\n")
    } catch (e: Exception) {
        println("⚠️  function cleanup skipped (some functions contain parameter placeholders)\n")
    }
}

fun main() = runBlocking {
    println("🚀 ekoDB Functions Example (Kotlin Client)\n")

    try {
        val dotenv = dotenv()
        val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
        val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

        val client = EkoDBClient.builder()
            .baseUrl(baseUrl)
            .apiKey(apiKey)
            .build()

        println("✅ Client initialized\n")

        setupTestData(client)
        val getActiveUsersId = simpleQueryScript(client)
        parameterizedScript(client)
        val userStatsId = aggregationScript(client)
        functionManagement(client, getActiveUsersId, userStatsId)
        multiStageScript(client)
        countScript(client)
        cleanup(client)

        println("✅ All examples completed successfully!")
        println("\n💡 Key Advantages of Using the Client:")
        println("   • Automatic token management")
        println("   • Type-safe Stage builders")
        println("   • Built-in error handling")
    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        throw e
    }
}
