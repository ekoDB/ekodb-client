/**
 * Advanced Scripts Example - Query, Sort, Limit, Group
 *
 * Demonstrates advanced query and aggregation operations using simple patterns
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

    println("🚀 ekoDB Kotlin Advanced Scripts Example")
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

    val scriptIds = mutableListOf<String>()

    try {
        // Example 1: List All Products
        println("📝 Example 1: List All Products")
        println()

        val script1 = Script(
            label = "list_all_products_adv_kt",
            name = "List All Products",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "advanced_products_kt")
            ),
            tags = listOf("products", "list")
        )
        val scriptId1 = client.saveScript(script1)
        scriptIds.add(scriptId1)
        println("✅ Script saved")

        val result1 = client.callScript("list_all_products_adv_kt")
        println("📊 Found ${result1.records.size} products")
        println("⏱️  Execution time: ${result1.stats.execution_time_ms}ms\n")

        // Example 2: Group Products by Category
        println("📝 Example 2: Group Products by Category")
        println()

        val script2 = Script(
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
        val scriptId2 = client.saveScript(script2)
        scriptIds.add(scriptId2)
        println("✅ Script saved")

        val result2 = client.callScript("products_by_category_kt")
        println("📊 Category breakdown:")
        result2.records.forEach { println("   $it") }
        println("⏱️  Execution time: ${result2.stats.execution_time_ms}ms\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (scriptId in scriptIds) {
            try { client.deleteScript(scriptId) } catch (e: Exception) {}
        }
        try { client.deleteCollection("advanced_products_kt") } catch (e: Exception) {}
        println("✅ Cleanup complete")

        println()
        println("✅ All advanced script examples finished!")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
}
