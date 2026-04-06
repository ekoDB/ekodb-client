/**
 * CRUD Functions Example - Basic Data Operations
 *
 * Demonstrates basic CRUD operations using functions:
 * - FindAll queries
 * - Group aggregations
 * - Simple data transformations
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.UserFunction
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.GroupFunctionConfig
import io.ekodb.client.functions.GroupFunctionOp
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("🚀 ekoDB Kotlin CRUD Functions Example")
    println()

    // Setup test data
    println("📋 Setting up test data...")
    try { client.deleteCollection("crud_users_kt") } catch (e: Exception) {}

    for (i in 1..10) {
        val status = if (i % 3 == 0) "inactive" else "active"
        val role = if (i <= 3) "admin" else "user"
        val record = Record.new()
            .insert("name", "User $i")
            .insert("email", "user$i@example.com")
            .insert("status", status)
            .insert("score", i * 10)
            .insert("role", role)
        client.insert("crud_users_kt", record)
    }
    println("✅ Created 10 test users\n")

    val funcIds = mutableListOf<String>()

    try {
        // Example 1: List All Users
        println("📝 Example 1: List All Users")
        println()

        val func1 = UserFunction(
            label = "list_all_users_kt",
            name = "List All Users",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "crud_users_kt")
            ),
            tags = listOf("users", "list")
        )
        val funcId1 = client.saveFunction(func1)
        funcIds.add(funcId1)
        println("✅ Function saved")

        val result1 = client.callFunction("list_all_users_kt")
        println("📊 Found ${result1.records.size} users")
        println("⏱️  Execution time: ${result1.stats.execution_time_ms}ms\n")

        // Example 2: Count Users by Status
        println("📝 Example 2: Count Users by Status")
        println()

        val func2 = UserFunction(
            label = "users_by_status_kt",
            name = "Users by Status",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "crud_users_kt"),
                FunctionStageConfig.Group(
                    by_fields = listOf("status"),
                    functions = listOf(
                        GroupFunctionConfig(output_field = "count", operation = GroupFunctionOp.Count)
                    )
                )
            ),
            tags = listOf("users", "analytics")
        )
        val funcId2 = client.saveFunction(func2)
        funcIds.add(funcId2)
        println("✅ Function saved")

        val result2 = client.callFunction("users_by_status_kt")
        println("📊 User counts by status:")
        result2.records.forEach { println("   $it") }
        println("⏱️  Execution time: ${result2.stats.execution_time_ms}ms\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (funcId in funcIds) {
            try { client.deleteFunction(funcId) } catch (e: Exception) {}
        }
        try { client.deleteCollection("crud_users_kt") } catch (e: Exception) {}
        println("✅ Cleanup complete")

        println()
        println("✅ All CRUD function examples finished!")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
}
