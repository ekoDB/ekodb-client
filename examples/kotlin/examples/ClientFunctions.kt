/**
 * Functions Example for ekoDB Kotlin Client
 *
 * Demonstrates creating, managing, and executing functions with the Kotlin client.
 * Covers: FindAll, Group, Project, Count, and Function management operations.
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.getValue
import io.ekodb.client.functions.UserFunction
import io.ekodb.client.functions.ParameterDefinition
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.GroupFunctionConfig
import io.ekodb.client.functions.GroupFunctionOp
import io.ekodb.client.functions.QueryConditionOperator
import io.ekodb.client.functions.queryCondition
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject

private const val TEST_COLLECTION = "client_functions_users_kt"
private const val ACTIVE_LABEL = "client_functions_get_active_users_kt"
private const val STATUS_LABEL = "client_functions_get_users_by_status_kt"
private const val STATS_LABEL = "client_functions_user_stats_kt"
private const val TOP_LABEL = "client_functions_top_users_kt"
private const val COUNT_LABEL = "client_functions_count_users_kt"

/**
 * Detect the server's "label already exists" rejection.
 *
 * The server now returns HTTP 409 Conflict when POST /api/functions is called
 * with a label that already exists. The client surfaces that as an exception
 * whose message contains both "status 409" and "already exists".
 */
private fun isAlreadyExistsError(e: Exception): Boolean {
    val msg = e.message ?: return false
    return msg.contains("status 409") || msg.contains("already exists")
}

private fun isNotFoundError(error: Throwable): Boolean =
    error.message?.let { it.contains("status 404") || it.contains("not found", ignoreCase = true) } == true

/**
 * Idempotent save: create the function, or PUT-update it if its label already
 * exists. Returns the function's encrypted ID either way (looked up by label on
 * the update path), so downstream get/update/delete-by-id keeps working.
 */
private suspend fun saveOrUpdate(
    client: EkoDBClient,
    func: UserFunction,
    functionIds: MutableSet<String>
): String {
    val id = try {
        client.saveFunction(func)
    } catch (e: Exception) {
        if (isAlreadyExistsError(e)) {
            // Label already exists — update the existing definition (PUT by label).
            client.updateFunction(func.label, func)
            println("ℹ️  Function '${func.label}' already existed — updated instead")
            // Resolve the encrypted ID by label so cleanup-by-id still works.
            client.getFunction(func.label).id
                ?: throw IllegalStateException("No ID returned for function '${func.label}'")
        } else {
            throw e
        }
    }
    functionIds += id
    return id
}

suspend fun setupTestData(client: EkoDBClient) {
    println("📋 Setting up test data...")

    try {
        client.deleteCollection(TEST_COLLECTION)
    } catch (error: Throwable) {
        if (!isNotFoundError(error)) throw error
    }

    for (i in 1..10) {
        val record = Record.new()
            .insert("name", "User $i")
            .insert("age", 20 + i)
            .insert("status", if (i % 2 == 0) "active" else "inactive")
            .insert("score", i * 10)
        client.insert(TEST_COLLECTION, record)
    }

    check(client.countDocuments(TEST_COLLECTION) == 10L) { "Expected exactly 10 test users" }

    println("✅ Test data ready\n")
}

suspend fun simpleQueryScript(client: EkoDBClient, functionIds: MutableSet<String>): String {
    println("📝 Example 1: Simple Query Function\n")

    val func = UserFunction(
        label = ACTIVE_LABEL,
        name = "Get Active Users",
        description = "Retrieve all active users",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.Query(
                collection = TEST_COLLECTION,
                filter = queryCondition("status", QueryConditionOperator.Eq, JsonPrimitive("active"))
            )
        ),
        tags = listOf("users", "query")
    )

    val funcId = saveOrUpdate(client, func, functionIds)
    println("✅ Function saved: $funcId")

    val result = client.callFunction(ACTIVE_LABEL)
    check(result.records.size == 5) { "Expected 5 active users, got ${result.records.size}" }
    println("📊 Found ${result.records.size} records")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
    
    return funcId
}

suspend fun parameterizedScript(client: EkoDBClient, functionIds: MutableSet<String>): String {
    println("📝 Example 2: Parameterized Function\n")

    val func = UserFunction(
        label = STATUS_LABEL,
        name = "Get Users By Status",
        version = "1.0",
        parameters = mapOf(
            "status" to ParameterDefinition(
                required = false,
                default = JsonPrimitive("active"),
                description = "Filter by user status"
            )
        ),
        functions = listOf(
            FunctionStageConfig.Query(
                collection = TEST_COLLECTION,
                filter = queryCondition("status", QueryConditionOperator.Eq, JsonPrimitive("{{status}}")),
                limit = 3
            )
        ),
        tags = listOf("users", "parameterized")
    )

    val funcId = saveOrUpdate(client, func, functionIds)
    println("✅ Function saved")

    val params = mapOf("status" to JsonPrimitive("active"))
    val result = client.callFunction(STATUS_LABEL, params)
    check(result.records.size == 3) { "Expected 3 limited users, got ${result.records.size}" }
    check(result.records.all { getValue<String>(it["status"]) == "active" }) {
        "Parameterized query returned a non-active user"
    }
    println("📊 Found ${result.records.size} users (limited)")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
    return funcId
}

suspend fun aggregationScript(client: EkoDBClient, functionIds: MutableSet<String>): String {
    println("📝 Example 3: Aggregation Function\n")

    val func = UserFunction(
        label = STATS_LABEL,
        name = "User Statistics",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = TEST_COLLECTION),
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

    val funcId = saveOrUpdate(client, func, functionIds)
    println("✅ Function saved")

    val result = client.callFunction(STATS_LABEL)
    check(result.records.size == 2) { "Expected active and inactive groups" }
    val groupCounts = result.records.associate {
        requireNotNull(getValue<String>(it["status"])) to requireNotNull(getValue<Long>(it["count"]))
    }
    check(groupCounts == mapOf("active" to 5L, "inactive" to 5L)) {
        "Unexpected status counts: $groupCounts"
    }
    println("📊 Statistics: ${result.records.size} groups")
    result.records.forEach { record ->
        println("   $record")
    }
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
    
    return funcId
}

suspend fun functionManagement(client: EkoDBClient, getActiveUsersId: String, userStatsId: String) {
    println("📝 Example 4: function Management\n")

    val funcs = client.listFunctions()
    println("📋 Total functions: ${funcs.size}")

    // Get specific function (use encrypted ID)
    val func = client.getFunction(getActiveUsersId)
    println("🔍 Retrieved function: ${func.name}")

    // Update function (use encrypted ID)
    val updated = UserFunction(
        label = ACTIVE_LABEL,
        name = "Get Active Users (Updated)",
        description = "Updated description",
        version = "1.1",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = TEST_COLLECTION)
        ),
        tags = listOf("users")
    )
    client.updateFunction(getActiveUsersId, updated)
    println("✏️  function updated")

    // Delete function (use ID)
    client.deleteFunction(userStatsId)
    println("🗑️  function deleted")
    println()
    
    println("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
    println("ℹ️  Only CALL can use either ID or label\n")
}

suspend fun multiStageScript(client: EkoDBClient, functionIds: MutableSet<String>): String {
    println("📝 Example 5: Multi-Stage Pipeline\n")

    val func = UserFunction(
        label = TOP_LABEL,
        name = "Top Performing Users",
        version = "1.0",
        parameters = mapOf(
            "min_score" to ParameterDefinition(
                required = false,
                default = JsonPrimitive(50)
            )
        ),
        functions = listOf(
            FunctionStageConfig.Query(
                collection = TEST_COLLECTION,
                filter = queryCondition("score", QueryConditionOperator.Gte, JsonPrimitive("{{min_score}}"))
            ),
            FunctionStageConfig.Project(
                fields = listOf("name", "score", "status"),
                exclude = false
            )
        ),
        tags = listOf("analytics", "reporting")
    )

    val funcId = saveOrUpdate(client, func, functionIds)
    println("✅ Multi-stage function saved")

    val result = client.callFunction(TOP_LABEL, mapOf("min_score" to JsonPrimitive(50)))
    check(result.records.size == 6) { "Expected 6 users with score >= 50" }
    println("📊 Pipeline executed ${result.stats.stages_executed} stages")
    println("⏱️  Total execution time: ${result.stats.execution_time_ms}ms")
    println("📈 Stage breakdown:")
    result.stats.stage_stats.forEachIndexed { index, stage ->
        println("   ${index + 1}. ${stage.stage}: ${stage.execution_time_ms}ms (${stage.input_count} → ${stage.output_count} records)")
    }
    println()
    return funcId
}

suspend fun countScript(client: EkoDBClient, functionIds: MutableSet<String>): String {
    println("📝 Example 6: Count Users\n")

    val func = UserFunction(
        label = COUNT_LABEL,
        name = "Count All Users",
        version = "1.0",
        parameters = emptyMap(),
        functions = listOf(
            FunctionStageConfig.FindAll(collection = TEST_COLLECTION),
            FunctionStageConfig.Count(output_field = "count")
        ),
        tags = listOf("users", "count")
    )

    val funcId = saveOrUpdate(client, func, functionIds)
    println("✅ Count function saved")

    val result = client.callFunction(COUNT_LABEL)
    val count = result.records.firstOrNull()?.get("count")
    check(getValue<Long>(count) == 10L) { "Expected count function to return 10, got $count" }
    println("📊 Total user count: $count")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")
    return funcId
}

suspend fun cleanup(client: EkoDBClient, functionIds: Collection<String>): List<Throwable> {
    println("🧹 Cleaning up...")
    val errors = mutableListOf<Throwable>()
    var functionCleanupFailed = false
    for (id in functionIds) {
        try {
            client.deleteFunction(id)
        } catch (error: Throwable) {
            if (!isNotFoundError(error)) {
                functionCleanupFailed = true
                errors += error
            }
        }
    }
    if (functionIds.isNotEmpty() && !functionCleanupFailed) println("✅ Deleted test functions")
    try {
        client.deleteCollection(TEST_COLLECTION)
        println("✅ Deleted collection")
    } catch (error: Throwable) {
        if (!isNotFoundError(error)) errors += error
    }
    println()
    return errors
}

fun main() = runBlocking {
    println("🚀 ekoDB Functions Example (Kotlin Client)\n")

    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"
    val client = EkoDBClient.builder().baseUrl(baseUrl).apiKey(apiKey).build()
    val functionIds = linkedSetOf<String>()
    var failure: Throwable? = null

    try {
        println("✅ Client initialized\n")

        setupTestData(client)
        val getActiveUsersId = simpleQueryScript(client, functionIds)
        parameterizedScript(client, functionIds)
        val userStatsId = aggregationScript(client, functionIds)
        functionManagement(client, getActiveUsersId, userStatsId)
        functionIds.remove(userStatsId)
        multiStageScript(client, functionIds)
        countScript(client, functionIds)
    } catch (error: Throwable) {
        println("❌ Error: ${error.message}")
        failure = error
    } finally {
        for (cleanupError in cleanup(client, functionIds)) {
            failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
        }
        try {
            client.close()
        } catch (cleanupError: Throwable) {
            failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
        }
    }
    failure?.let { throw it }
    println("✅ All examples completed successfully!")
    println("\n💡 Key Advantages of Using the Client:")
    println("   • Automatic token management")
    println("   • Type-safe Stage builders")
    println("   • Built-in error handling")
}
