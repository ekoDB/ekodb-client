/**
 * Advanced Functions Example - Query, Sort, Limit, Group
 *
 * Demonstrates advanced query and aggregation operations using simple patterns
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

private fun isAlreadyExistsError(e: Exception): Boolean {
    val msg = e.message ?: return false
    return msg.contains("status 409") || msg.contains("already exists")
}

private suspend fun saveOrUpdate(client: EkoDBClient, func: UserFunction): String {
    return try {
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
}

fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("🚀 ekoDB Kotlin Advanced Functions Example")
    println()

    // Setup test data
    println("📋 Setting up test data...")
    try { client.deleteCollection("advanced_products_kt") } catch (e: Exception) {}

    val products = listOf(
        Record.new().insert("name", "Laptop Pro").insert("category", "Electronics").insert("price", 1299).insert("stock", 15).insert("rating", 4.8),
        Record.new().insert("name", "Wireless Mouse").insert("category", "Electronics").insert("price", 29).insert("stock", 45).insert("rating", 4.5),
        Record.new().insert("name", "Desk Chair").insert("category", "Furniture").insert("price", 349).insert("stock", 8).insert("rating", 4.2),
        Record.new().insert("name", "Standing Desk").insert("category", "Furniture").insert("price", 599).insert("stock", 12).insert("rating", 4.7),
        Record.new().insert("name", "USB-C Cable").insert("category", "Electronics").insert("price", 19).insert("stock", 100).insert("rating", 4.3),
        Record.new().insert("name", "Monitor 27\"").insert("category", "Electronics").insert("price", 399).insert("stock", 20).insert("rating", 4.6),
        Record.new().insert("name", "Bookshelf").insert("category", "Furniture").insert("price", 149).insert("stock", 5).insert("rating", 4.1),
        Record.new().insert("name", "Keyboard").insert("category", "Electronics").insert("price", 89).insert("stock", 30).insert("rating", 4.4)
    )

    for (product in products) {
        client.insert("advanced_products_kt", product)
    }
    println("✅ Created ${products.size} products\n")

    val funcIds = mutableListOf<String>()

    try {
        // Example 1: List All Products
        println("📝 Example 1: List All Products")
        println()

        val func1 = UserFunction(
            label = "list_all_products_adv_kt",
            name = "List All Products",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "advanced_products_kt")
            ),
            tags = listOf("products", "list")
        )
        val funcId1 = saveOrUpdate(client, func1)
        funcIds.add(funcId1)
        println("✅ Function saved")

        val result1 = client.callFunction("list_all_products_adv_kt")
        println("📊 Found ${result1.records.size} products")
        println("⏱️  Execution time: ${result1.stats.execution_time_ms}ms\n")

        // Example 2: Group Products by Category
        println("📝 Example 2: Group Products by Category")
        println()

        val func2 = UserFunction(
            label = "products_by_category_kt",
            name = "Products by Category",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "advanced_products_kt"),
                FunctionStageConfig.Group(
                    by_fields = listOf("category"),
                    functions = listOf(
                        GroupFunctionConfig(output_field = "count", operation = GroupFunctionOp.Count),
                        GroupFunctionConfig(output_field = "avg_price", operation = GroupFunctionOp.Average, input_field = "price")
                    )
                )
            ),
            tags = listOf("products", "analytics")
        )
        val funcId2 = saveOrUpdate(client, func2)
        funcIds.add(funcId2)
        println("✅ Function saved")

        val result2 = client.callFunction("products_by_category_kt")
        println("📊 Category breakdown:")
        result2.records.forEach { println("   $it") }
        println("⏱️  Execution time: ${result2.stats.execution_time_ms}ms\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (funcId in funcIds) {
            try { client.deleteFunction(funcId) } catch (e: Exception) {}
        }
        try { client.deleteCollection("advanced_products_kt") } catch (e: Exception) {}
        println("✅ Cleanup complete")

        println()
        println("✅ All advanced function examples finished!")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
}
