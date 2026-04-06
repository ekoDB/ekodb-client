/**
 * Complete Functions Example - ekoDB Functions
 * Demonstrates: FindAll, Group, Count, Multi-stage Pipelines
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

    println("🚀 ekoDB Kotlin Complete Functions Example")
    println()
    println("📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines")
    println()

    // Setup test data
    println("📋 Setting up complete test data...")
    try { client.deleteCollection("complete_products_kt") } catch (e: Exception) {}

    val products = listOf(
        Record.new().insert("name", "Laptop Pro").insert("category", "Electronics").insert("price", 1299).insert("stock", 15).insert("rating", 4.8),
        Record.new().insert("name", "Wireless Mouse").insert("category", "Electronics").insert("price", 29).insert("stock", 45).insert("rating", 4.5),
        Record.new().insert("name", "Desk Chair").insert("category", "Furniture").insert("price", 349).insert("stock", 8).insert("rating", 4.2),
        Record.new().insert("name", "Standing Desk").insert("category", "Furniture").insert("price", 599).insert("stock", 12).insert("rating", 4.7),
        Record.new().insert("name", "Monitor 27\"").insert("category", "Electronics").insert("price", 399).insert("stock", 20).insert("rating", 4.6)
    )

    for (product in products) {
        client.insert("complete_products_kt", product)
    }
    println("✅ Created ${products.size} products\n")

    val funcIds = mutableListOf<String>()

    try {
        // Example 1: FindAll + Group (Simple Aggregation)
        println("📝 Example 1: FindAll + Group (Simple Aggregation)")
        println()

        val func1 = UserFunction(
            label = "product_stats_kt",
            name = "Product Statistics",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "complete_products_kt"),
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

        val funcId1 = client.saveFunction(func1)
        println("✅ Function saved: $funcId1")
        funcIds.add(funcId1)

        val result1 = client.callFunction("product_stats_kt")
        println("📊 Found ${result1.records.size} category groups\n")

        // Example 2: Simple Product Listing
        println("📝 Example 2: Simple Product Listing")
        println()

        val func2 = UserFunction(
            label = "list_all_products_kt",
            name = "List All Products",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "complete_products_kt")
            ),
            tags = listOf("products", "list")
        )

        val funcId2 = client.saveFunction(func2)
        println("✅ Function saved")
        funcIds.add(funcId2)

        val result2 = client.callFunction("list_all_products_kt")
        println("📊 Found ${result2.records.size} products\n")

        // Example 3: Count by Category
        println("📝 Example 3: Count by Category")
        println()

        val func3 = UserFunction(
            label = "count_by_category_kt",
            name = "Count Products by Category",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "complete_products_kt"),
                FunctionStageConfig.Group(
                    by_fields = listOf("category"),
                    functions = listOf(
                        GroupFunctionConfig(output_field = "count", operation = GroupFunctionOp.Count)
                    )
                )
            ),
            tags = listOf("products", "analytics")
        )

        val funcId3 = client.saveFunction(func3)
        println("✅ Function saved")
        funcIds.add(funcId3)

        val result3 = client.callFunction("count_by_category_kt")
        println("📊 Found ${result3.records.size} categories\n")

        // Example 4: Multi-Stage Pipeline
        println("📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)")
        println()

        val func4 = UserFunction(
            label = "product_summary_kt",
            name = "Product Summary Report",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "complete_products_kt"),
                FunctionStageConfig.Group(
                    by_fields = listOf("category"),
                    functions = listOf(
                        GroupFunctionConfig(output_field = "count", operation = GroupFunctionOp.Count),
                        GroupFunctionConfig(output_field = "avg_price", operation = GroupFunctionOp.Average, input_field = "price")
                    )
                ),
                FunctionStageConfig.Count(output_field = "total_categories")
            ),
            tags = listOf("products", "analytics")
        )

        val funcId4 = client.saveFunction(func4)
        println("✅ Function saved")
        funcIds.add(funcId4)

        val result4 = client.callFunction("product_summary_kt")
        println("📊 Pipeline executed with ${result4.records.size} results\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (funcId in funcIds) {
            try {
                client.deleteFunction(funcId)
            } catch (e: Exception) {
                // Ignore cleanup errors
            }
        }
        try {
            client.deleteCollection("complete_products_kt")
        } catch (e: Exception) {
            // Ignore cleanup errors
        }
        println("✅ Cleanup complete")

        println()
        println("✅ All complete function examples finished!")
        println()
        println("💡 This example demonstrates ekoDB's function system:")
        println("   ✅ FindAll operations")
        println("   ✅ Group aggregations (Count, Average)")
        println("   ✅ Multi-stage pipelines (FindAll → Group → Count)")
        println("   ✅ Function management (save, call, delete)")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
}
