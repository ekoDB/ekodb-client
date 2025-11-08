/**
 * Saved Functions Example for ekoDB Kotlin Client
 *
 * Demonstrates creating, managing, and executing saved functions
 */

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.*
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

suspend fun simpleFunction(client: EkoDBClient): String {
    println("📝 Example 1: Simple Query Function\n")

    val function = SavedFunction(
        label = "get_active_users",
        name = "Get Active Users",
        description = "Retrieve all active users",
        version = "1.0",
        parameters = emptyMap(),
        pipeline = listOf(
            FunctionStageConfig.FindAll(collection = "users")
        ),
        tags = listOf("users", "query")
    )

    val id = client.saveFunction(function)
    println("✅ Function saved: $id")

    // Call function by label
    val result = client.callFunction("get_active_users")
    println("📊 Found ${result.records.size} active users\n")
    
    return id
}

suspend fun parameterizedFunction(client: EkoDBClient) {
    println("📝 Example 2: Parameterized Function\n")

    val function = SavedFunction(
        label = "get_users_by_status",
        name = "Get Users By Status",
        version = "1.0",
        parameters = mapOf(
            "status" to ParameterDefinition(
                required = false,
                default = JsonPrimitive("active"),
                description = "Filter by status"
            ),
            "limit" to ParameterDefinition(
                required = false,
                default = JsonPrimitive(10),
                description = "Limit number of results"
            )
        ),
        pipeline = listOf(
            FunctionStageConfig.FindAll(collection = "users")
        ),
        tags = listOf("users", "parameterized")
    )

    val id = client.saveFunction(function)
    println("✅ Function saved: $id")

    val params = mapOf(
        "status" to JsonPrimitive("active"),
        "limit" to JsonPrimitive(3)
    )
    val result = client.callFunction("get_users_by_status", params)
    println("📊 Found ${result.records.size} users (limited)\n")
}

suspend fun aggregationFunction(client: EkoDBClient): String {
    println("📝 Example 3: Aggregation Function\n")

    val function = SavedFunction(
        label = "user_stats",
        name = "User Statistics",
        version = "1.0",
        parameters = emptyMap(),
        pipeline = listOf(
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

    val id = client.saveFunction(function)
    println("✅ Function saved: $id")

    val result = client.callFunction("user_stats")
    println("📊 Statistics: ${result.records.size} groups")
    result.records.forEach { record ->
        println("   $record")
    }
    println()
    
    return id
}

suspend fun functionManagement(client: EkoDBClient, functionIds: Map<String, String>) {
    println("📝 Example 4: Function Management\n")

    // List all functions
    val functions = client.listFunctions()
    println("📋 Total functions: ${functions.size}")

    // Get specific function by ID
    val getActiveUsersId = functionIds["get_active_users"] ?: error("Function ID not found")
    val func = client.getFunction(getActiveUsersId)
    println("🔍 Retrieved function: ${func.name}")

    // Update function
    val updated = SavedFunction(
        label = "get_active_users",
        name = "Get Active Users (Updated)",
        description = "Updated description",
        version = "1.1",
        parameters = emptyMap(),
        pipeline = listOf(
            FunctionStageConfig.FindAll(collection = "users")
        ),
        tags = listOf("users")
    )
    client.updateFunction(getActiveUsersId, updated)
    println("✏️  Function updated")

    // Delete function by ID
    val userStatsId = functionIds["user_stats"] ?: error("Function ID not found")
    client.deleteFunction(userStatsId)
    println("🗑️  Function deleted\n")
}

fun main() = runBlocking {
    println("🚀 ekoDB Saved Functions Example (Kotlin)\n")

    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    // Run examples
    setupTestData(client)
    val getActiveUsersId = simpleFunction(client)
    parameterizedFunction(client)
    val userStatsId = aggregationFunction(client)
    
    // Collect function IDs for management operations
    val functionIds = mapOf(
        "get_active_users" to getActiveUsersId,
        "user_stats" to userStatsId
    )
    functionManagement(client, functionIds)

    println("✅ All examples completed!")
}
