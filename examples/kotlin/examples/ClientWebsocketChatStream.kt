package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.ChatStreamEvent
import io.ekodb.client.ClientToolDefinition
import io.ekodb.client.WebSocketClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*

/**
 * WebSocket Chat Streaming Example - Real-time LLM responses
 *
 * Demonstrates using the WebSocket client to send a chat message
 * and receive streaming responses, including tool calls.
 *
 * Requires: ekoDB server running on localhost:8080
 */
fun main(): Unit = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val wsUrl = dotenv["WS_BASE_URL"] ?: "ws://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== WebSocket Chat Streaming Example (Kotlin) ===\n")

    var cleanupChatId: String? = null
    var cleanupWs: WebSocketClient? = null
    var failure: Throwable? = null
    try {
        // Create a chat session via HTTP
        val session = client.createChatSession(buildJsonObject {
            putJsonArray("collections") {}
            put("llm_provider", "openai")
            put("llm_model", "gpt-4o-mini")
            put("system_prompt", "You are a helpful assistant.")
        })
        val chatId = session["chat_id"]?.jsonPrimitive?.content ?: error("No chat_id")
        cleanupChatId = chatId
        println("Created chat session: $chatId")

        // Connect WebSocket
        val ws = client.websocket(wsUrl)
        cleanupWs = ws
        ws.connect()

        // Send a chat message and stream the response
        println("\nSending message: 'What is the capital of France?'")
        val stream = ws.chatSend(chatId, "What is the capital of France?")

        val fullResponse = StringBuilder()
        var completed = false
        stream.collect { event ->
            when (event) {
                is ChatStreamEvent.Chunk -> {
                    fullResponse.append(event.content)
                    print(event.content)
                }
                is ChatStreamEvent.End -> {
                    completed = true
                    println("\n\n--- Stream ended ---")
                    println("Message ID: ${event.messageId}")
                    println("Execution time: ${event.executionTimeMs}ms")
                    event.tokenUsage?.let { println("Token usage: $it") }
                }
                is ChatStreamEvent.ToolCall -> {
                    println("\n[Tool Call] ${event.toolName}(${event.arguments})")
                    // Send tool result back
                    ws.sendToolResult(
                        chatId,
                        event.callId,
                        true,
                        buildJsonObject { put("result", "Tool executed successfully") }
                    )
                }
                is ChatStreamEvent.Error -> {
                    println("\n[Error] ${event.error}")
                    error("WebSocket chat failed: ${event.error}")
                }
            }
        }
        check(completed) { "WebSocket chat ended before an end event" }

        val preview = fullResponse.toString().take(200)
        println("\nFull response: $preview...")

    } catch (error: Throwable) {
        println("Error: ${error.message}")
        failure = error
    } finally {
        try {
            cleanupWs?.close()
        } catch (cleanupError: Throwable) {
            val primary = failure
            if (primary == null) failure = cleanupError else primary.addSuppressed(cleanupError)
        }
        try {
            cleanupChatId?.let { client.deleteChatSession(it) }
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
}
