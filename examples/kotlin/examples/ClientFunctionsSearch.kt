/**
 * Search Scripts Example - Basic Search Operations
 *
 * Demonstrates simple search and query operations using scripts
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.Script
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

    println("🚀 ekoDB Kotlin Search Scripts Example")
    println()

    // Setup test data
    println("📋 Setting up test data...")
    try { client.deleteCollection("search_docs_kt") } catch (e: Exception) {}

    val documents = listOf(
        Record.new().insert("title", "Introduction to Machine Learning").insert("content", "Machine learning is a subset of AI.").insert("category", "AI"),
        Record.new().insert("title", "Database Design Principles").insert("content", "Good database design involves normalization.").insert("category", "Database"),
        Record.new().insert("title", "Vector Databases Explained").insert("content", "Vector databases store data based on similarity.").insert("category", "Database"),
        Record.new().insert("title", "Natural Language Processing").insert("content", "NLP enables computers to understand language.").insert("category", "AI"),
        Record.new().insert("title", "Getting Started with ekoDB").insert("content", "ekoDB is a high-performance database.").insert("category", "Database")
    )

    for (doc in documents) {
        client.insert("search_docs_kt", doc)
    }
    println("✅ Inserted ${documents.size} documents\n")

    val scriptIds = mutableListOf<String>()

    try {
        // Example 1: List All Documents
        println("📝 Example 1: List All Documents")
        println()

        val script1 = Script(
            label = "list_all_docs_kt",
            name = "List All Documents",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "search_docs_kt")
            ),
            tags = listOf("search", "list")
        )
        val scriptId1 = client.saveScript(script1)
        scriptIds.add(scriptId1)
        println("✅ Script saved")

        val result1 = client.callScript("list_all_docs_kt")
        println("📊 Found ${result1.records.size} documents")
        result1.records.forEachIndexed { i, record ->
            println("   ${i + 1}. ${record.get("title")} (${record.get("category")})")
        }
        println("⏱️  Execution time: ${result1.stats.execution_time_ms}ms\n")

        // Example 2: Count Documents by Category
        println("📝 Example 2: Count Documents by Category")
        println()

        val script2 = Script(
            label = "docs_by_category_kt",
            name = "Documents by Category",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "search_docs_kt"),
                FunctionStageConfig.Group(
                    by_fields = listOf("category"),
                    functions = listOf(
                        GroupFunctionConfig(output_field = "count", operation = GroupFunctionOp.Count)
                    )
                )
            ),
            tags = listOf("search", "analytics")
        )
        val scriptId2 = client.saveScript(script2)
        scriptIds.add(scriptId2)
        println("✅ Script saved")

        val result2 = client.callScript("docs_by_category_kt")
        println("📊 Documents by category:")
        result2.records.forEach { println("   $it") }
        println("⏱️  Execution time: ${result2.stats.execution_time_ms}ms\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (scriptId in scriptIds) {
            try { client.deleteScript(scriptId) } catch (e: Exception) {}
        }
        try { client.deleteCollection("search_docs_kt") } catch (e: Exception) {}
        println("✅ Cleanup complete")

        println()
        println("✅ All search script examples finished!")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
}
