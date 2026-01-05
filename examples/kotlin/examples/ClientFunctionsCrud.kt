/**
 * CRUD Scripts Example - Basic Data Operations
 *
 * Demonstrates basic CRUD operations using scripts:
 * - FindAll queries
 * - Group aggregations
 * - Simple data transformations
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.Script
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.GroupFunctionConfig
import io.ekodb.client.functions.GroupFunctionOp
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("🚀 ekoDB Kotlin CRUD Scripts Example")
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

    val scriptIds = mutableListOf<String>()

    try {
        // Example 1: List All Users
        println("📝 Example 1: List All Users")
        println()

        val script1 = Script(
            label = "list_all_users_kt",
            name = "List All Users",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "crud_users_kt")
            ),
            tags = listOf("users", "list")
        )
        val scriptId1 = client.saveScript(script1)
        scriptIds.add(scriptId1)
        println("✅ Script saved")

        val result1 = client.callScript("list_all_users_kt")
        println("📊 Found ${result1.records.size} users")
        println("⏱️  Execution time: ${result1.stats.execution_time_ms}ms\n")

        // Example 2: Count Users by Status
        println("📝 Example 2: Count Users by Status")
        println()

        val script2 = Script(
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
        val scriptId2 = client.saveScript(script2)
        scriptIds.add(scriptId2)
        println("✅ Script saved")

        val result2 = client.callScript("users_by_status_kt")
        println("📊 User counts by status:")
        result2.records.forEach { println("   $it") }
        println("⏱️  Execution time: ${result2.stats.execution_time_ms}ms\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (scriptId in scriptIds) {
            try { client.deleteScript(scriptId) } catch (e: Exception) {}
        }
        try { client.deleteCollection("crud_users_kt") } catch (e: Exception) {}
        println("✅ Cleanup complete")

        println()
        println("✅ All CRUD script examples finished!")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
}
