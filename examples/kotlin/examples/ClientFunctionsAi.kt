/**
 * AI Functions Example - Chat and Embed Operations
 *
 * Demonstrates AI operations in functions:
 * - Chat completions with context
 * - Embedding generation
 * - Simple AI workflows
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.UserFunction
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.ChatMessage
import io.ekodb.client.functions.ParameterDefinition
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("🚀 ekoDB Kotlin AI Functions Example")
    println()

    // Setup test data
    println("📋 Setting up test data...")
    try { client.deleteCollection("ai_articles_kt") } catch (e: Exception) {}

    val articles = listOf(
        Record.new().insert("title", "Getting Started with ekoDB").insert("content", "ekoDB is a high-performance database...").insert("status", "published"),
        Record.new().insert("title", "Advanced Query Patterns").insert("content", "Learn how to build complex queries...").insert("status", "published")
    )

    for (article in articles) {
        client.insert("ai_articles_kt", article)
    }
    println("✅ Created ${articles.size} articles\n")

    val funcIds = mutableListOf<String>()

    try {
        // Example 1: Simple Chat Completion
        println("📝 Example 1: Simple Chat Completion")
        println()

        val func1 = UserFunction(
            label = "ai_assistant_kt",
            name = "AI Chat Assistant",
            description = "Simple AI chat completion",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.Chat(
                    messages = listOf(
                        ChatMessage(role = "system", content = "You are a helpful database assistant. Be concise."),
                        ChatMessage(role = "user", content = "What are the benefits of using vector databases?")
                    ),
                    model = "gpt-4o-mini",
                    temperature = 0.7
                )
            ),
            tags = listOf("ai", "chat")
        )
        val funcId1 = client.saveFunction(func1)
        funcIds.add(funcId1)
        println("✅ Chat function saved")

        val result1 = client.callFunction("ai_assistant_kt")
        println("🤖 AI Response:")
        if (result1.records.isNotEmpty()) {
            println("   ${result1.records[0]}")
        }
        println("⏱️  Execution time: ${result1.stats.execution_time_ms}ms\n")

        // Example 2: Embed Generation
        println("📝 Example 2: Generate Embeddings")
        println()

        val func2 = UserFunction(
            label = "generate_embedding_kt",
            name = "Generate Embedding",
            description = "Generate embedding for text",
            version = "1.0",
            parameters = mapOf(
                "text" to ParameterDefinition(
                    required = true,
                    description = "Text to embed"
                )
            ),
            functions = listOf(
                FunctionStageConfig.Embed(
                    input_field = "text",
                    output_field = "embedding"
                )
            ),
            tags = listOf("ai", "embed")
        )
        val funcId2 = client.saveFunction(func2)
        funcIds.add(funcId2)
        println("✅ Embed function saved")

        val params = buildJsonObject {
            put("text", "ekoDB is a powerful database")
        }
        val result2 = client.callFunction("generate_embedding_kt", params)
        println("📊 Embedding generated")
        println("⏱️  Execution time: ${result2.stats.execution_time_ms}ms\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (funcId in funcIds) {
            try { client.deleteFunction(funcId) } catch (e: Exception) {}
        }
        try { client.deleteCollection("ai_articles_kt") } catch (e: Exception) {}
        println("✅ Cleanup complete")

        println()
        println("✅ All AI function examples finished!")
        println()
        println("💡 This example demonstrates:")
        println("   ✅ Chat completions with system/user messages")
        println("   ✅ Embedding generation for text")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
}
