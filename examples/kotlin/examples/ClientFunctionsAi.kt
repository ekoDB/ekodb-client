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
import io.ekodb.client.functions.ChatMessage
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.UserFunction
import io.ekodb.client.getArrayValue
import io.ekodb.client.getStringValue
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

    println("🚀 ekoDB Kotlin AI Functions Example")
    println()

    val funcIds = mutableListOf<String>()
    try {
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
        val funcId1 = saveOrUpdate(client, func1)
        funcIds.add(funcId1)
        println("✅ Chat function saved")

        val result1 = client.callFunction("ai_assistant_kt")
        println("🤖 AI Response:")
        val response = result1.records.firstOrNull()?.let { getStringValue(it["response"]) }
            ?.takeIf { it.isNotBlank() }
            ?: error("Chat function returned no response text")
        println("   $response")
        println("⏱️  Execution time: ${result1.stats.execution_time_ms}ms\n")

        // Example 2: Embed Generation
        println("📝 Example 2: Generate Embeddings")
        println()

        val func2 = UserFunction(
            label = "generate_embedding_kt",
            name = "Generate Embedding",
            description = "Generate embedding for text",
            version = "1.0",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "ai_articles_kt"),
                FunctionStageConfig.Embed(
                    input_field = "content",
                    output_field = "embedding"
                )
            ),
            tags = listOf("ai", "embed")
        )
        val funcId2 = saveOrUpdate(client, func2)
        funcIds.add(funcId2)
        println("✅ Embed function saved")

        val result2 = client.callFunction("generate_embedding_kt")
        check(result2.records.size == articles.size) {
            "Embedding function returned ${result2.records.size} records; expected ${articles.size}"
        }
        val dimensions = result2.records.mapIndexed { index, article ->
            getArrayValue(article["embedding"])?.size?.takeIf { it > 0 }
                ?: error("Article ${index + 1} did not contain an embedding vector")
        }
        println("📊 Generated ${dimensions.size} embeddings")
        println("   Dimensions: ${dimensions.first()}")
        println("⏱️  Execution time: ${result2.stats.execution_time_ms}ms\n")

        // Cleanup
        println("🧹 Cleaning up...")
        for (funcId in funcIds) {
            client.deleteFunction(funcId)
        }
        client.deleteCollection("ai_articles_kt")
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
        for (funcId in funcIds) {
            try { client.deleteFunction(funcId) } catch (cleanupError: Exception) { e.addSuppressed(cleanupError) }
        }
        try { client.deleteCollection("ai_articles_kt") } catch (cleanupError: Exception) {
            e.addSuppressed(cleanupError)
        }
        throw e
    } finally {
        client.close()
    }
}
