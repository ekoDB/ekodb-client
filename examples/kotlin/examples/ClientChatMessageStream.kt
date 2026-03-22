// ekoDB Kotlin Client - Chat Message Stream (SSE) Example
//
// Demonstrates streaming chat responses via HTTP SSE (Server-Sent Events).

import io.ekodb.client.EkoDBClient
import io.ekodb.client.ChatStreamEvent
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    println("=== ekoDB Chat Message Stream (SSE) Example (Kotlin) ===\n")

    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY")
        ?: throw IllegalStateException("API_BASE_KEY environment variable is required")

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    // Create a chat session
    val session = client.createChatSession(buildJsonObject {
        put("system_prompt", "You are a helpful assistant.")
    })
    val chatId = session["chat_id"].toString().trim('"')
    println("Created session: $chatId")

    // Stream a chat message via SSE
    println("\nStreaming response for: 'What is ekoDB?'\n")
    val events = client.chatMessageStream(chatId, buildJsonObject {
        put("message", "What is ekoDB?")
    })

    events.collect { event ->
        when (event) {
            is ChatStreamEvent.Chunk -> print(event.content)
            is ChatStreamEvent.End -> {
                println("\n\n--- Stream complete ---")
                println("Message ID: ${event.messageId}")
                println("Execution time: ${event.executionTimeMs}ms")
                event.contextWindow?.let {
                    println("Context window: $it tokens")
                }
            }
            is ChatStreamEvent.ToolCall -> {
                println("[Tool] ${event.toolName}")
            }
            is ChatStreamEvent.Error -> {
                println("Error: ${event.error}")
            }
        }
    }

    client.close()
    println("\n✓ Chat message stream example completed")
}
