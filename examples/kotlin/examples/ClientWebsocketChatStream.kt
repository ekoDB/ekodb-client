package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.ChatStreamEvent
import io.ekodb.client.ClientToolDefinition
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
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val wsUrl = dotenv["WS_BASE_URL"] ?: "ws://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== WebSocket Chat Streaming Example (Kotlin) ===\n")

    try {
        // Create a chat session via HTTP
        val session = client.createChatSession(buildJsonObject {
            put("system_prompt", "You are a helpful assistant.")
        })
        val chatId = session["chat_id"]?.jsonPrimitive?.content ?: error("No chat_id")
        println("Created chat session: $chatId")

        // Connect WebSocket
        val ws = client.websocket(wsUrl)

        // Send a chat message and stream the response
        println("\nSending message: 'What is the capital of France?'")
        val stream = ws.chatSend(chatId, "What is the capital of France?")

        val fullResponse = StringBuilder()
        stream.collect { event ->
            when (event) {
                is ChatStreamEvent.Chunk -> {
                    fullResponse.append(event.content)
                    print(event.content)
                }
                is ChatStreamEvent.End -> {
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
                }
            }
        }

        val preview = fullResponse.toString().take(200)
        println("\nFull response: $preview...")

        ws.close()
    } catch (e: Exception) {
        println("Error: ${e.message}")
    }
}
