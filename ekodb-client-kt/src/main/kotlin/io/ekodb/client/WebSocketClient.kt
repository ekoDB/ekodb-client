package io.ekodb.client

import io.ktor.client.*
import io.ktor.client.plugins.websocket.*
import io.ktor.websocket.*
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.*

/** Mutation notification from a subscription. */
data class MutationNotification(
    val collection: String,
    val event: String,
    val recordIds: List<String>,
    val records: JsonElement? = null,
    val timestamp: String
)

/** A streaming chat event. */
sealed class ChatStreamEvent {
    data class Chunk(val content: String) : ChatStreamEvent()
    data class End(
        val messageId: String,
        val tokenUsage: JsonElement? = null,
        val toolCallHistory: JsonElement? = null,
        val executionTimeMs: Long = 0
    ) : ChatStreamEvent()

    data class ToolCall(
        val chatId: String,
        val callId: String,
        val toolName: String,
        val arguments: JsonElement
    ) : ChatStreamEvent()

    data class Error(val error: String) : ChatStreamEvent()
}

/** Definition for a client-side tool the LLM can call. */
data class ClientToolDefinition(
    val name: String,
    val description: String,
    val parameters: JsonElement
)

/** Options for chatSend. */
data class ChatSendOptions(
    val bypassRipple: Boolean? = null,
    val clientTools: List<ClientToolDefinition>? = null,
    val maxIterations: Int? = null,
    val confirmTools: List<String>? = null,
    val excludeTools: List<String>? = null
)

/** Options for subscribe. */
data class SubscribeOptions(
    val filterField: String? = null,
    val filterValue: String? = null
)

/**
 * WebSocket client for real-time communication with ekoDB.
 * Includes full dispatcher for subscriptions, chat streaming, and tool calling.
 */
class WebSocketClient(
    private val wsUrl: String,
    private val token: String,
    private val httpClient: HttpClient
) {
    private var session: DefaultClientWebSocketSession? = null
    private var dispatcherJob: Job? = null
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)

    // Dispatcher state
    private val mutex = Mutex()
    private val pendingRequests = mutableMapOf<String, CompletableDeferred<JsonObject>>()
    private val subscriptions = mutableMapOf<String, Channel<MutationNotification>>()
    private val chatStreams = mutableMapOf<String, Channel<ChatStreamEvent>>()
    private var registerToolsAck: CompletableDeferred<JsonObject>? = null

    private fun genMessageId(): String {
        return "${System.nanoTime()}"
    }

    /**
     * Connect to the WebSocket server and start the dispatcher.
     */
    suspend fun connect() {
        val url = if (wsUrl.contains("/api/ws")) wsUrl else {
            val baseUrl = wsUrl.trimEnd('/')
            "$baseUrl/api/ws"
        }

        session = httpClient.webSocketSession(url) {
            headers.append("Authorization", "Bearer $token")
        }

        spawnDispatcher()
    }

    private fun spawnDispatcher() {
        val currentSession = session ?: return
        dispatcherJob = scope.launch {
            try {
                for (frame in currentSession.incoming) {
                    if (frame is Frame.Text) {
                        try {
                            val msg = Json.parseToJsonElement(frame.readText()).jsonObject
                            routeMessage(msg)
                        } catch (_: Exception) {
                            // Ignore malformed messages
                        }
                    }
                }
            } finally {
                // Connection closed — notify all waiters
                mutex.withLock {
                    pendingRequests.values.forEach {
                        it.completeExceptionally(Exception("WebSocket connection closed"))
                    }
                    pendingRequests.clear()
                    chatStreams.values.forEach { ch ->
                        ch.trySend(ChatStreamEvent.Error("Connection closed"))
                        ch.close()
                    }
                    chatStreams.clear()
                    subscriptions.values.forEach { it.close() }
                    subscriptions.clear()
                    registerToolsAck?.completeExceptionally(Exception("Connection closed"))
                    registerToolsAck = null
                }
            }
        }
    }

    private suspend fun routeMessage(msg: JsonObject) {
        val type = msg["type"]?.jsonPrimitive?.content ?: return

        when (type) {
            "Success", "Error" -> routeRequestResponse(type, msg)
            "MutationNotification" -> routeMutationNotification(msg)
            "ChatStreamChunk" -> routeChatStreamChunk(msg)
            "ChatStreamEnd" -> routeChatStreamEnd(msg)
            "ChatStreamError" -> routeChatStreamError(msg)
            "ClientToolCall" -> routeClientToolCall(msg)
        }
    }

    private suspend fun routeRequestResponse(type: String, msg: JsonObject) {
        mutex.withLock {
            val payload = msg["payload"]?.jsonObject
            val messageId = payload?.get("message_id")?.jsonPrimitive?.content

            var target: CompletableDeferred<JsonObject>? = null

            if (messageId != null) {
                target = pendingRequests.remove(messageId)
            }

            if (target == null && registerToolsAck != null) {
                target = registerToolsAck
                registerToolsAck = null
            }

            if (target == null && pendingRequests.size == 1) {
                val key = pendingRequests.keys.first()
                target = pendingRequests.remove(key)
            }

            if (target != null) {
                if (type == "Error") {
                    val errMsg = msg["message"]?.jsonPrimitive?.content ?: "Unknown error"
                    target.completeExceptionally(Exception(errMsg))
                } else {
                    target.complete(payload ?: buildJsonObject {})
                }
            }
        }
    }

    private suspend fun routeMutationNotification(msg: JsonObject) {
        val payload = msg["payload"]?.jsonObject ?: return
        val collection = payload["collection"]?.jsonPrimitive?.content ?: return
        val event = payload["event"]?.jsonPrimitive?.content ?: ""
        val recordIds = payload["record_ids"]?.jsonArray?.map { it.jsonPrimitive.content } ?: emptyList()
        val timestamp = payload["timestamp"]?.jsonPrimitive?.content ?: ""

        val notification = MutationNotification(
            collection = collection,
            event = event,
            recordIds = recordIds,
            records = payload["records"],
            timestamp = timestamp
        )

        mutex.withLock {
            subscriptions[collection]?.trySend(notification)
        }
    }

    private fun extractChatId(msg: JsonObject): String? {
        return msg["payload"]?.jsonObject?.get("chat_id")?.jsonPrimitive?.content
    }

    private suspend fun routeChatStreamChunk(msg: JsonObject) {
        val chatId = extractChatId(msg) ?: return
        val content = msg["payload"]?.jsonObject?.get("content")?.jsonPrimitive?.content ?: ""

        mutex.withLock {
            chatStreams[chatId]?.trySend(ChatStreamEvent.Chunk(content))
        }
    }

    private suspend fun routeChatStreamEnd(msg: JsonObject) {
        val chatId = extractChatId(msg) ?: return
        val payload = msg["payload"]?.jsonObject ?: return

        val event = ChatStreamEvent.End(
            messageId = payload["message_id"]?.jsonPrimitive?.content ?: "",
            tokenUsage = payload["token_usage"],
            toolCallHistory = payload["tool_call_history"],
            executionTimeMs = payload["execution_time_ms"]?.jsonPrimitive?.long ?: 0
        )

        mutex.withLock {
            val ch = chatStreams.remove(chatId)
            ch?.trySend(event)
            ch?.close()
        }
    }

    private suspend fun routeChatStreamError(msg: JsonObject) {
        val chatId = extractChatId(msg) ?: return
        val payload = msg["payload"]?.jsonObject ?: return
        val error = payload["error"]?.jsonPrimitive?.content
            ?: payload["message"]?.jsonPrimitive?.content
            ?: "Unknown error"

        mutex.withLock {
            val ch = chatStreams.remove(chatId)
            ch?.trySend(ChatStreamEvent.Error(error))
            ch?.close()
        }
    }

    private suspend fun routeClientToolCall(msg: JsonObject) {
        val chatId = extractChatId(msg) ?: return
        val payload = msg["payload"]?.jsonObject ?: return

        val event = ChatStreamEvent.ToolCall(
            chatId = chatId,
            callId = payload["call_id"]?.jsonPrimitive?.content ?: "",
            toolName = payload["tool_name"]?.jsonPrimitive?.content ?: "",
            arguments = payload["arguments"] ?: buildJsonObject {}
        )

        mutex.withLock {
            chatStreams[chatId]?.trySend(event)
        }
    }

    private suspend fun sendRequest(request: JsonObject): JsonObject {
        val s = session ?: throw IllegalStateException("Not connected. Call connect() first.")
        val messageId = request["messageId"]?.jsonPrimitive?.content
            ?: throw IllegalArgumentException("Request must have a messageId")

        val deferred = CompletableDeferred<JsonObject>()
        mutex.withLock {
            pendingRequests[messageId] = deferred
        }

        s.send(Frame.Text(request.toString()))
        return deferred.await()
    }

    /**
     * Find all records in a collection via WebSocket.
     */
    suspend fun findAll(collection: String): JsonObject {
        val messageId = genMessageId()
        val request = buildJsonObject {
            put("type", "FindAll")
            put("messageId", messageId)
            put("payload", buildJsonObject {
                put("collection", collection)
            })
        }
        return sendRequest(request)
    }

    /**
     * Subscribe to mutation notifications on a collection.
     * Returns a Flow that emits MutationNotification events.
     */
    suspend fun subscribe(
        collection: String,
        options: SubscribeOptions? = null
    ): Flow<MutationNotification> {
        val messageId = genMessageId()
        val request = buildJsonObject {
            put("type", "Subscribe")
            put("messageId", messageId)
            put("payload", buildJsonObject {
                put("collection", collection)
                options?.filterField?.let { put("filter_field", it) }
                options?.filterValue?.let { put("filter_value", it) }
            })
        }

        val channel = Channel<MutationNotification>(64)
        mutex.withLock {
            subscriptions[collection] = channel
        }

        sendRequest(request)

        return flow {
            for (notification in channel) {
                emit(notification)
            }
        }
    }

    /**
     * Send a chat message and receive a streaming response.
     * Returns a Flow that emits ChatStreamEvent objects.
     */
    suspend fun chatSend(
        chatId: String,
        message: String,
        options: ChatSendOptions? = null
    ): Flow<ChatStreamEvent> {
        val s = session ?: throw IllegalStateException("Not connected. Call connect() first.")

        val channel = Channel<ChatStreamEvent>(64)
        mutex.withLock {
            chatStreams[chatId] = channel
        }

        val request = buildJsonObject {
            put("type", "ChatSend")
            put("payload", buildJsonObject {
                put("chat_id", chatId)
                put("message", message)
                options?.bypassRipple?.let { put("bypass_ripple", it) }
                options?.clientTools?.let { tools ->
                    put("client_tools", buildJsonArray {
                        tools.forEach { tool ->
                            add(buildJsonObject {
                                put("name", tool.name)
                                put("description", tool.description)
                                put("parameters", tool.parameters)
                            })
                        }
                    })
                }
                options?.maxIterations?.let { put("max_iterations", it) }
                options?.confirmTools?.let { tools ->
                    put("confirm_tools", buildJsonArray { tools.forEach { add(it) } })
                }
                options?.excludeTools?.let { tools ->
                    put("exclude_tools", buildJsonArray { tools.forEach { add(it) } })
                }
            })
        }

        s.send(Frame.Text(request.toString()))

        return flow {
            for (event in channel) {
                emit(event)
            }
        }
    }

    /**
     * Register client-side tools for a chat session.
     */
    suspend fun registerClientTools(chatId: String, tools: List<ClientToolDefinition>) {
        val s = session ?: throw IllegalStateException("Not connected. Call connect() first.")

        val deferred = CompletableDeferred<JsonObject>()
        mutex.withLock {
            registerToolsAck = deferred
        }

        val request = buildJsonObject {
            put("type", "RegisterClientTools")
            put("payload", buildJsonObject {
                put("chat_id", chatId)
                put("tools", buildJsonArray {
                    tools.forEach { tool ->
                        add(buildJsonObject {
                            put("name", tool.name)
                            put("description", tool.description)
                            put("parameters", tool.parameters)
                        })
                    }
                })
            })
        }

        s.send(Frame.Text(request.toString()))
        deferred.await()
    }

    /**
     * Send a tool result back to the server during a chat stream.
     */
    suspend fun sendToolResult(
        chatId: String,
        callId: String,
        success: Boolean,
        result: JsonElement? = null,
        error: String? = null
    ) {
        val s = session ?: throw IllegalStateException("Not connected. Call connect() first.")

        val request = buildJsonObject {
            put("type", "ClientToolResult")
            put("payload", buildJsonObject {
                put("chat_id", chatId)
                put("call_id", callId)
                put("success", success)
                result?.let { put("result", it) }
                error?.let { put("error", it) }
            })
        }

        s.send(Frame.Text(request.toString()))
    }

    /**
     * Close the WebSocket connection and clean up resources.
     */
    suspend fun close() {
        dispatcherJob?.cancel()
        session?.close()
        session = null
        scope.cancel()
    }
}

@Serializable
data class WebSocketMessage(
    val type: String,
    val messageId: String,
    val payload: JsonObject
)
