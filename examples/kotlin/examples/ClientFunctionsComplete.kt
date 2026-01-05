/**
 * Complete Scripts Example - ekoDB Scripts
 * Demonstrates: FindAll, Group, Count, Multi-stage Pipelines
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

    println("🚀 ekoDB Kotlin Complete Scripts Example")
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

    val scriptIds = mutableListOf<String>()

    try {
        // Example 1: FindAll + Group (Simple Aggregation)
        println("📝 Example 1: FindAll + Group (Simple Aggregation)")
        println()

        val script1 = Script(
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

        val scriptId1 = client.saveScript(script1)
        println("✅ Script saved: $scriptId1")
        scriptIds.add(scriptId1)

        val result1 = client.callScript("product_stats_kt")
        println("📊 Found ${result1.records.size} category groups\n")

        // Example 2: Simple Product Listing
        println("📝 Example 2: Simple Product Listing")
        println()

        val script2 = Script(
            label = "list_all_products_kt",
            name = "List All Products",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "complete_products_kt")
            ),
            tags = listOf("products", "list")
        )

        val scriptId2 = client.saveScript(script2)
        println("✅ Script saved")
        scriptIds.add(scriptId2)

        val result2 = client.callScript("list_all_products_kt")
        println("📊 Found ${result2.records.size} products\n")

        // Example 3: Count by Category
        println("📝 Example 3: Count by Category")
        println()

        val script3 = Script(
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

        val scriptId3 = client.saveScript(script3)
        println("✅ Script saved")
        scriptIds.add(scriptId3)

        val result3 = client.callScript("count_by_category_kt")
        println("📊 Found ${result3.records.size} categories\n")

        // Example 4: Multi-Stage Pipeline
        println("📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)")
        println()

        val script4 = Script(
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

        val scriptId4 = client.saveScript(script4)
        println("✅ Script saved")
        scriptIds.add(scriptId4)

        val result4 = client.callScript("product_summary_kt")
        println("📊 Pipeline executed with ${result4.records.size} results\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (scriptId in scriptIds) {
            try {
                client.deleteScript(scriptId)
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
        println("✅ All complete script examples finished!")
        println()
        println("💡 This example demonstrates ekoDB's Script system:")
        println("   ✅ FindAll operations")
        println("   ✅ Group aggregations (Count, Average)")
        println("   ✅ Multi-stage pipelines (FindAll → Group → Count)")
        println("   ✅ Script management (save, call, delete)")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
}
