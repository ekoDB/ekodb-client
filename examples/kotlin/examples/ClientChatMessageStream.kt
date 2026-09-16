// ekoDB Kotlin Client - Chat Message Stream (SSE) Example
//
// Demonstrates streaming chat responses via HTTP SSE (Server-Sent Events).

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.ChatStreamEvent
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.runBlocking

fun main(): Unit = runBlocking {
    println("=== ekoDB Chat Message Stream (SSE) Example (Kotlin) ===\n")

    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY")
        ?: throw IllegalStateException("API_BASE_KEY environment variable is required")

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    var chatId: String? = null
    var failure: Throwable? = null
    try {
        // Create a chat session
        val session = client.createChatSession(buildJsonObject {
            putJsonArray("collections") {}
            put("llm_provider", "openai")
            put("llm_model", "gpt-4o-mini")
            put("system_prompt", "You are a helpful assistant.")
        })
        chatId = session["chat_id"]?.jsonPrimitive?.content
            ?: error("Chat session response did not include chat_id")
        println("Created session: $chatId")

        // Stream a chat message via SSE
        println("\nStreaming response for: 'What is ekoDB?'\n")
        val events = client.chatMessageStream(chatId, buildJsonObject {
            put("message", "What is ekoDB?")
        })

        var completed = false
        events.collect { event ->
            when (event) {
                is ChatStreamEvent.Chunk -> print(event.content)
                is ChatStreamEvent.End -> {
                    completed = true
                    println("\n\n--- Stream complete ---")
                    println("Message ID: ${event.messageId}")
                    println("Execution time: ${event.executionTimeMs}ms")
                    event.contextWindow?.let {
                        println("Context window: $it tokens")
                    }
                }
                is ChatStreamEvent.ToolCall -> println("[Tool] ${event.toolName}")
                is ChatStreamEvent.Error -> {
                    println("Error: ${event.error}")
                    error("Chat stream failed: ${event.error}")
                }
            }
        }
        check(completed) { "Chat stream ended before an end event" }
    } catch (error: Throwable) {
        failure = error
    } finally {
        try {
            chatId?.let { client.deleteChatSession(it) }
        } catch (cleanupError: Throwable) {
            val primary = failure
            if (primary == null) failure = cleanupError else primary.addSuppressed(cleanupError)
        }
        try {
            client.close()
        } catch (cleanupError: Throwable) {
            val primary = failure
            if (primary == null) failure = cleanupError else primary.addSuppressed(cleanupError)
        }
    }
    failure?.let { throw it }
    println("\n✓ Chat message stream example completed")
}
