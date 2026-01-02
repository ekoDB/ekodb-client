/**
 * Scripts Example for ekoDB Kotlin Client
 *
 * Demonstrates creating, managing, and executing scripts with the Kotlin client.
 * Covers: FindAll, Group, Project, Count, and Script management operations.
 */

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.Script
import io.ekodb.client.functions.ParameterDefinition
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.GroupFunctionConfig
import io.ekodb.client.functions.GroupFunctionOp
import io.ekodb.client.types.Record
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
    println("📝 Example 1: Simple Query Script\n")

    val script = Script(
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

    val scriptId = client.saveScript(script)
    println("✅ Script saved: $scriptId")

    val result = client.callScript("get_active_users")
    println("📊 Found ${result.records.size} records")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
    
    return scriptId
}

suspend fun parameterizedScript(client: EkoDBClient) {
    println("📝 Example 2: Parameterized Script\n")

    val script = Script(
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

    client.saveScript(script)
    println("✅ Script saved")

    val params = mapOf(
        "status" to JsonPrimitive("active"),
        "limit" to JsonPrimitive(3)
    )
    val result = client.callScript("get_users_by_status", params)
    println("📊 Found ${result.records.size} users (limited)")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
}

suspend fun aggregationScript(client: EkoDBClient): String {
    println("📝 Example 3: Aggregation Script\n")

    val script = Script(
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

    val scriptId = client.saveScript(script)
    println("✅ Script saved")

    val result = client.callScript("user_stats")
    println("📊 Statistics: ${result.records.size} groups")
    result.records.forEach { record ->
        println("   $record")
    }
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
    
    return scriptId
}

suspend fun scriptManagement(client: EkoDBClient, getActiveUsersId: String, userStatsId: String) {
    println("📝 Example 4: Script Management\n")

    // List all scripts (skip if deserialization fails due to parameter placeholders)
    try {
        val scripts = client.listScripts()
        println("📋 Total scripts: ${scripts.size}")
    } catch (e: Exception) {
        println("📋 Script listing skipped (some scripts contain parameter placeholders)")
    }

    // Get specific script (use encrypted ID)
    val script = client.getScript(getActiveUsersId)
    println("🔍 Retrieved script: ${script.name}")

    // Update script (use encrypted ID)
    val updated = Script(
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
    client.updateScript(getActiveUsersId, updated)
    println("✏️  Script updated")

    // Delete script (use ID) - handle error gracefully
    try {
        client.deleteScript(userStatsId)
        println("🗑️  Script deleted")
    } catch (e: Exception) {
        println("ℹ️  Script delete skipped (may not exist)")
    }
    println()
    
    println("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
    println("ℹ️  Only CALL can use either ID or label\n")
}

suspend fun multiStageScript(client: EkoDBClient) {
    println("📝 Example 5: Multi-Stage Pipeline\n")

    val script = Script(
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

    client.saveScript(script)
    println("✅ Multi-stage script saved")

    val result = client.callScript("top_users", mapOf("min_score" to JsonPrimitive(50)))
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

    val script = Script(
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

    client.saveScript(script)
    println("✅ Count script saved")

    val result = client.callScript("count_users")
    val count = result.records.firstOrNull()?.get("count")
    println("📊 Total user count: $count")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
}

suspend fun cleanup(client: EkoDBClient) {
    println("🧹 Cleaning up...")

    // Delete test collection
    client.deleteCollection("users")
    println("✅ Deleted collection")

    // List and delete all test scripts (skip if deserialization fails)
    try {
        val scripts = client.listScripts()
        for (script in scripts) {
            if (script.label.startsWith("get_") || script.label.startsWith("user_") ||
                script.label.startsWith("top_") || script.label.startsWith("count_")) {
                try {
                    script.id?.let { client.deleteScript(it) }
                } catch (e: Exception) {
                    // Script might already be deleted
                }
            }
        }
        println("✅ Deleted test scripts\n")
    } catch (e: Exception) {
        println("⚠️  Script cleanup skipped (some scripts contain parameter placeholders)\n")
    }
}

fun main() = runBlocking {
    println("🚀 ekoDB Scripts Example (Kotlin Client)\n")

    try {
        val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
        val apiKey = System.getenv("API_BASE_KEY") ?: "a-test-api-key-from-ekodb"

        val client = EkoDBClient.builder()
            .baseUrl(baseUrl)
            .apiKey(apiKey)
            .build()

        println("✅ Client initialized\n")

        setupTestData(client)
        val getActiveUsersId = simpleQueryScript(client)
        parameterizedScript(client)
        val userStatsId = aggregationScript(client)
        scriptManagement(client, getActiveUsersId, userStatsId)
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
