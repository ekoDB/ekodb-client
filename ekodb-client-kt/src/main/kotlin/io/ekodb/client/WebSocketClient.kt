package io.ekodb.client

import io.ktor.client.*
import io.ktor.client.plugins.websocket.*
import io.ktor.websocket.*
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.*

/**
 * WebSocket client for real-time communication with ekoDB
 */
class WebSocketClient(
    private val wsUrl: String,
    private val token: String,
    private val httpClient: HttpClient
) {
    private var session: DefaultClientWebSocketSession? = null
    
    /**
     * Connect to the WebSocket server
     */
    suspend fun connect() {
        // Append /api/ws if not present
        val url = if (wsUrl.contains("/api/ws")) wsUrl else {
            val baseUrl = wsUrl.trimEnd('/')
            "$baseUrl/api/ws"
        }
        
        session = httpClient.webSocketSession(url) {
            headers.append("Authorization", "Bearer $token")
        }
    }
    
    /**
     * Subscribe to collection changes
     */
    suspend fun subscribe(collection: String): Flow<JsonObject> = flow {
        val session = session ?: throw IllegalStateException("Not connected. Call connect() first.")
        
        // Send subscribe message
        val subscribeMsg = buildJsonObject {
            put("type", "Subscribe")
            put("messageId", java.util.UUID.randomUUID().toString())
            put("payload", buildJsonObject {
                put("collection", collection)
            })
        }
        
        session.send(Frame.Text(subscribeMsg.toString()))
        
        // Receive messages
        for (frame in session.incoming) {
            if (frame is Frame.Text) {
                val message = Json.parseToJsonElement(frame.readText()).jsonObject
                emit(message)
            }
        }
    }
    
    /**
     * Find all records via WebSocket
     */
    suspend fun findAll(collection: String): JsonObject {
        val session = session ?: throw IllegalStateException("Not connected. Call connect() first.")
        
        val messageId = java.util.UUID.randomUUID().toString()
        val findMsg = buildJsonObject {
            put("type", "FindAll")
            put("messageId", messageId)
            put("payload", buildJsonObject {
                put("collection", collection)
            })
        }
        
        session.send(Frame.Text(findMsg.toString()))
        
        // Wait for response
        for (frame in session.incoming) {
            if (frame is Frame.Text) {
                val response = Json.parseToJsonElement(frame.readText()).jsonObject
                return response
            }
        }
        
        throw Exception("No response received")
    }
    
    /**
     * Close the WebSocket connection
     */
    suspend fun close() {
        session?.close()
        session = null
    }
}

@Serializable
data class WebSocketMessage(
    val type: String,
    val messageId: String,
    val payload: JsonObject
)
