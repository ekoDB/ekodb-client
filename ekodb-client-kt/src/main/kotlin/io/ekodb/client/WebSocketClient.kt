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
import org.msgpack.core.MessageBufferPacker
import org.msgpack.core.MessagePack
import org.msgpack.core.MessageUnpacker
import org.msgpack.value.ValueType

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
        val executionTimeMs: Long = 0,
        /** Model's context window size in tokens. */
        val contextWindow: Int? = null
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
/** In-memory schema cache with TTL for primary_key_alias resolution. */
class SchemaCache(
    private val enabled: Boolean = false,
    private val maxEntries: Int = 100,
    private val ttlMs: Long = 300_000
) {
    private data class Entry(val primaryKeyAlias: String, val version: Long, val cachedAt: Long)

    private val entries = mutableMapOf<String, Entry>()
    private val lruOrder = mutableListOf<String>()

    fun isEnabled(): Boolean = enabled

    fun getAlias(collection: String): String? {
        if (!enabled) return null
        val entry = entries[collection] ?: return null
        if (System.currentTimeMillis() - entry.cachedAt > ttlMs) {
            entries.remove(collection)
            lruOrder.remove(collection)
            return null
        }
        touchLRU(collection)
        return entry.primaryKeyAlias
    }

    fun insert(collection: String, primaryKeyAlias: String, version: Long) {
        if (!enabled) return
        val isNew = !entries.containsKey(collection)
        entries[collection] = Entry(primaryKeyAlias, version, System.currentTimeMillis())
        if (isNew) {
            lruOrder.add(collection)
            while (lruOrder.size > maxEntries) {
                val oldest = lruOrder.removeFirst()
                entries.remove(oldest)
            }
        } else {
            touchLRU(collection)
        }
    }

    fun invalidate(collection: String) {
        entries.remove(collection)
        lruOrder.remove(collection)
    }

    fun invalidateAll() {
        entries.clear()
        lruOrder.clear()
    }

    fun handleSchemaChanged(collection: String, version: Long, primaryKeyAlias: String) {
        if (!enabled) return
        val existing = entries[collection]
        if (existing != null && version <= existing.version) return
        insert(collection, primaryKeyAlias, version)
    }

    val size: Int get() = entries.size

    private fun touchLRU(collection: String) {
        lruOrder.remove(collection)
        lruOrder.add(collection)
    }
}

/** Extract record ID trying custom aliases, then "id", then "_id". */
fun extractRecordId(record: JsonObject, extraCandidates: List<String> = emptyList()): String? {
    for (key in extraCandidates + listOf("id", "_id")) {
        val value = record[key] ?: continue
        if (value is JsonPrimitive && value.isString) return value.content
        // Handle typed wrapper {"type":"String","value":"..."}
        if (value is JsonObject) {
            val inner = value["value"]
            if (inner is JsonPrimitive && inner.isString) return inner.content
        }
    }
    return null
}

class WebSocketClient(
    private val wsUrl: String,
    private val token: String,
    private val httpClient: HttpClient
) {
    private var session: DefaultClientWebSocketSession? = null
    private var dispatcherJob: Job? = null
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)

    /**
     * Per-connection wire format, set by [negotiateFormat] on every [connect]:
     * true once the server has Welcomed msgpack, so frames are sent/received as
     * binary msgpack; false (JSON text) otherwise, including against an older
     * server that never Welcomes. `@Volatile` for visibility across the
     * connect, send, and dispatcher coroutines. Keeps the transport fully
     * back-compatible.
     */
    @Volatile
    private var binary = false
    var schemaCache: SchemaCache? = null

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
        // Cancel any existing dispatcher before reconnecting
        dispatcherJob?.let {
            it.cancel()
            it.join()
        }

        val url = if (wsUrl.contains("/api/ws")) {
            wsUrl
        } else {
            val baseUrl = wsUrl.trimEnd('/')
            "$baseUrl/api/ws"
        }

        session = httpClient.webSocketSession(url) {
            headers.append("Authorization", "Bearer $token")
        }

        // Negotiate the wire format before the dispatcher starts so the Welcome
        // frame is consumed here, not routed as a response.
        negotiateFormat()

        spawnDispatcher()
    }

    /**
     * Additive capability handshake: offer msgpack and, if the server Welcomes
     * it, switch this connection to binary msgpack frames; otherwise stay on
     * JSON text. The Welcome (a text frame) is read with a timeout so an older
     * server that never answers — or answers with an Error — simply leaves the
     * connection on JSON. Best-effort and never throws: JSON always works.
     */
    private suspend fun negotiateFormat() {
        binary = false
        val s = session ?: return
        try {
            s.send(Frame.Text("""{"type":"Hello","payload":{"formats":["msgpack","json"]}}"""))
            // Only caps the wait when no Welcome comes (a silent/old server); it
            // returns immediately when one does. 2s comfortably exceeds the
            // handshake round-trip even on high-latency links.
            val frame = withTimeoutOrNull(2000) { s.incoming.receive() }
            if (frame is Frame.Text) {
                val msg = Json.parseToJsonElement(frame.readText()).jsonObject
                val type = msg["type"]?.jsonPrimitive?.content
                val format = msg["payload"]?.jsonObject?.get("format")?.jsonPrimitive?.content
                if (type == "Welcome" && format == "msgpack") {
                    binary = true
                }
            }
        } catch (_: Exception) {
            binary = false
        }
    }

    private fun spawnDispatcher() {
        val currentSession = session ?: return
        dispatcherJob = scope.launch {
            try {
                for (frame in currentSession.incoming) {
                    // A binary frame is msgpack (the server only sends binary
                    // once it has Welcomed msgpack); a text frame is JSON.
                    // Decode by frame type so the routed object is identical
                    // regardless of negotiated transport.
                    val msg: JsonObject? = when (frame) {
                        is Frame.Text -> try {
                            Json.parseToJsonElement(frame.readText()).jsonObject
                        } catch (_: Exception) {
                            null
                        }
                        is Frame.Binary -> try {
                            msgpackToJsonElement(frame.readBytes()).jsonObject
                        } catch (_: Exception) {
                            null
                        }
                        else -> null
                    }
                    if (msg != null) routeMessage(msg)
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
            "SchemaChanged" -> {
                val payload = msg["payload"]?.jsonObject
                if (payload != null) {
                    schemaCache?.handleSchemaChanged(
                        payload["collection"]?.jsonPrimitive?.content ?: "",
                        payload["version"]?.jsonPrimitive?.long ?: 0,
                        payload["primary_key_alias"]?.jsonPrimitive?.content ?: "id"
                    )
                }
            }
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
            executionTimeMs = payload["execution_time_ms"]?.jsonPrimitive?.long ?: 0,
            contextWindow = payload["context_window"]?.jsonPrimitive?.int
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

        sendFrame(s, request)
        return deferred.await()
    }

    /**
     * Send a request on the active session using the negotiated format: binary
     * msgpack when the server Welcomed it, JSON text otherwise. The single write
     * point so every request honors the negotiated transport.
     */
    private suspend fun sendFrame(s: DefaultClientWebSocketSession, request: JsonObject) {
        if (binary) {
            s.send(Frame.Binary(true, jsonElementToMsgpack(request)))
        } else {
            s.send(Frame.Text(request.toString()))
        }
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
     * Build the `Unsubscribe` wire frame. Factored out so the exact JSON shape
     * (`type` tag, top-level `messageId`, nested `payload.collection`) can be
     * asserted in a unit test without a live socket. This mirrors the frame the
     * TypeScript and Go clients send.
     */
    internal fun buildUnsubscribeFrame(collection: String, messageId: String): JsonObject =
        buildJsonObject {
            put("type", "Unsubscribe")
            put("messageId", messageId)
            put("payload", buildJsonObject {
                put("collection", collection)
            })
        }

    /**
     * Build the CancelChat frame. Extracted so its wire shape (`type` tag,
     * top-level `messageId`, nested `payload.chat_id`) can be asserted in a unit
     * test without a live socket.
     */
    internal fun buildCancelChatFrame(chatId: String, messageId: String): JsonObject =
        buildJsonObject {
            put("type", "CancelChat")
            put("messageId", messageId)
            put("payload", buildJsonObject {
                put("chat_id", chatId)
            })
        }

    /**
     * Stop receiving mutation notifications for a collection.
     *
     * Intentional teardown: closes and drops the local subscription channel (so
     * a collected Flow completes) and sends a best-effort `Unsubscribe` frame to
     * the server so it stops streaming this collection on this connection. If
     * the socket is already gone the local teardown suffices, since the server
     * drops all subscriptions when the connection closes. Safe to call for a
     * collection that is not currently subscribed (no-op).
     *
     * A unique `messageId` is attached to the frame purely so the server's ack
     * carries a correlation id: `Unsubscribe` registers no pending request, so
     * the dispatcher finds no match and silently discards the ack.
     */
    suspend fun unsubscribe(collection: String) {
        // Local teardown: close and remove the subscription channel so any
        // collector of the returned Flow completes.
        val channel = mutex.withLock {
            subscriptions.remove(collection)
        }
        channel?.close()

        // Best-effort server frame. If we're not connected the local teardown
        // above is sufficient, so absence of a session is a silent no-op.
        val s = session ?: return
        val request = buildUnsubscribeFrame(collection, genMessageId())
        try {
            sendFrame(s, request)
        } catch (e: CancellationException) {
            throw e
        } catch (_: Throwable) {
            // Best-effort: the socket may be closing. Local teardown already
            // happened, so ignore the send failure rather than fail teardown.
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

        sendFrame(s, request)

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

        sendFrame(s, request)
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

        sendFrame(s, request)
    }

    /**
     * Cancel an in-flight streaming chat. Fire-and-forget: tells the server to
     * stop generating tokens for the given chat.
     */
    suspend fun cancelChat(chatId: String) {
        val s = session ?: throw IllegalStateException("Not connected. Call connect() first.")

        // Include a messageId so any server ack carries a correlation id. Without
        // it, the dispatcher's single-pending fallback could misroute a
        // CancelChat ack to an unrelated pending request (same reason unsubscribe
        // attaches one). This frame is fire-and-forget, so the ack is simply
        // ignorable once it can be correlated.
        val request = buildCancelChatFrame(chatId, genMessageId())

        sendFrame(s, request)
    }

    /**
     * Stateless raw LLM completion via WebSocket.
     *
     * Sends a RawComplete message and waits for the Success response.
     * Preferred over HTTP: the persistent WSS connection is already
     * authenticated and won't be killed by reverse proxy timeouts.
     */
    suspend fun rawCompletion(
        systemPrompt: String,
        message: String,
        provider: String? = null,
        model: String? = null,
        maxTokens: Int? = null,
    ): String {
        val messageId = genMessageId()
        val request = buildJsonObject {
            put("type", "RawComplete")
            put("messageId", messageId)
            put("payload", buildJsonObject {
                put("system_prompt", systemPrompt)
                put("message", message)
                provider?.let { put("provider", it) }
                model?.let { put("model", it) }
                maxTokens?.let { put("max_tokens", it) }
            })
        }
        val payload = sendRequest(request)
        return payload["data"]?.jsonObject
            ?.get("content")?.jsonPrimitive?.content
            ?: ""
    }

    /**
     * Close the WebSocket connection and clean up resources.
     */
    suspend fun close() {
        dispatcherJob?.let {
            it.cancel()
            it.join()
        }
        dispatcherJob = null
        session?.close()
        session = null
        pendingRequests.clear()
        subscriptions.clear()
        chatStreams.clear()
    }

    // =========================================================================
    // WS CRUD Methods — Full Parity with Server
    // =========================================================================

    private suspend fun sendCRUD(msgType: String, payload: JsonObject): JsonObject {
        val messageId = genMessageId()
        val request = buildJsonObject {
            put("type", msgType)
            put("messageId", messageId)
            put("payload", payload)
        }
        return sendRequest(request)
    }

    /** Extract record ID using schema cache alias. */
    fun extractId(collection: String, record: JsonObject): String? {
        val alias = schemaCache?.getAlias(collection)
        return extractRecordId(record, if (alias != null) listOf(alias) else emptyList())
    }

    /** Insert a single record via WebSocket. */
    suspend fun insert(collection: String, record: JsonObject, bypassRipple: Boolean? = null): JsonObject =
        sendCRUD("Insert", buildJsonObject {
            put("collection", collection)
            put("record", record)
            bypassRipple?.let { put("bypass_ripple", it) }
        })

    /** Query records via WebSocket. */
    suspend fun query(
        collection: String,
        filter: JsonElement? = null,
        sort: JsonElement? = null,
        limit: Int? = null,
        skip: Int? = null
    ): JsonObject = sendCRUD("Query", buildJsonObject {
        put("collection", collection)
        filter?.let { put("filter", it) }
        sort?.let { put("sort", it) }
        limit?.let { put("limit", it) }
        skip?.let { put("skip", it) }
    })

    /** Find a record by ID via WebSocket. */
    suspend fun findById(collection: String, id: String): JsonObject =
        sendCRUD("FindById", buildJsonObject {
            put("collection", collection)
            put("id", id)
        })

    /** Update a record by ID via WebSocket. */
    suspend fun update(collection: String, id: String, record: JsonObject, bypassRipple: Boolean? = null): JsonObject =
        sendCRUD("Update", buildJsonObject {
            put("collection", collection)
            put("id", id)
            put("record", record)
            bypassRipple?.let { put("bypass_ripple", it) }
        })

    /** Delete a record by ID via WebSocket. */
    suspend fun delete(collection: String, id: String, bypassRipple: Boolean? = null): JsonObject =
        sendCRUD("Delete", buildJsonObject {
            put("collection", collection)
            put("id", id)
            bypassRipple?.let { put("bypass_ripple", it) }
        })

    /** Batch insert multiple records via WebSocket. */
    suspend fun batchInsert(collection: String, records: List<JsonObject>, bypassRipple: Boolean? = null): JsonObject =
        sendCRUD("BatchInsert", buildJsonObject {
            put("collection", collection)
            put("records", JsonArray(records))
            bypassRipple?.let { put("bypass_ripple", it) }
        })

    /** Batch update multiple records via WebSocket. */
    suspend fun batchUpdate(collection: String, updates: List<JsonArray>, bypassRipple: Boolean? = null): JsonObject =
        sendCRUD("BatchUpdate", buildJsonObject {
            put("collection", collection)
            put("updates", JsonArray(updates))
            bypassRipple?.let { put("bypass_ripple", it) }
        })

    /** Batch delete records by IDs via WebSocket. */
    suspend fun batchDelete(collection: String, ids: List<String>, bypassRipple: Boolean? = null): JsonObject =
        sendCRUD("BatchDelete", buildJsonObject {
            put("collection", collection)
            put("ids", JsonArray(ids.map { JsonPrimitive(it) }))
            bypassRipple?.let { put("bypass_ripple", it) }
        })

    /** Full-text search via WebSocket. */
    suspend fun textSearch(collection: String, query: String, fields: List<String>? = null, limit: Int? = null): JsonObject =
        sendCRUD("TextSearch", buildJsonObject {
            put("collection", collection)
            put("query", query)
            if (fields != null || limit != null) {
                put("options", buildJsonObject {
                    fields?.let { put("fields", JsonArray(it.map { f -> JsonPrimitive(f) })) }
                    limit?.let { put("limit", it) }
                })
            }
        })

    /** Get distinct values for a field via WebSocket. */
    suspend fun distinctValues(collection: String, field: String, filter: JsonElement? = null): JsonObject =
        sendCRUD("DistinctValues", buildJsonObject {
            put("collection", collection)
            put("field", field)
            filter?.let { put("filter", it) }
        })

    /** Apply an atomic field action via WebSocket. */
    suspend fun updateWithAction(
        collection: String, id: String, action: String, field: String, value: JsonElement? = null
    ): JsonObject = sendCRUD("UpdateWithAction", buildJsonObject {
        put("collection", collection)
        put("id", id)
        put("action", action)
        put("field", field)
        value?.let { put("value", it) }
    })

    /** Create a collection with optional schema via WebSocket. */
    suspend fun createCollection(name: String, schema: JsonObject? = null): JsonObject =
        sendCRUD("CreateCollection", buildJsonObject {
            put("name", name)
            put("schema", schema ?: buildJsonObject {})
        })

    /** List all collections via WebSocket. */
    suspend fun listCollections(): JsonObject =
        sendCRUD("GetCollections", buildJsonObject {})

    /** Delete a collection via WebSocket. */
    suspend fun deleteCollection(name: String): JsonObject =
        sendCRUD("DeleteCollection", buildJsonObject {
            put("name", name)
        })
}

@Serializable
data class WebSocketMessage(
    val type: String,
    val messageId: String,
    val payload: JsonObject
)

// ============================================================================
// MessagePack <-> JSON transcoding for the WebSocket binary transport.
//
// `internal` (not private) and top-level (no instance state) so the round-trip
// can be unit-tested directly. The wire shape is value-identical to JSON: a
// msgpack bin (a Binary field) decodes to a number array, matching the server's
// JSON representation of binary fields, so decoded data is the same regardless
// of the negotiated transport.
// ============================================================================

/**
 * Pack a [JsonElement] into a MessagePack byte array. Maps use string keys
 * (matching the server's named-field encoding). Numbers are packed as integers
 * when they have no fractional part, else as doubles.
 */
internal fun jsonElementToMsgpack(element: JsonElement): ByteArray {
    val packer = MessagePack.newDefaultBufferPacker()
    packElement(packer, element)
    val bytes = packer.toByteArray()
    packer.close()
    return bytes
}

private fun packElement(packer: MessageBufferPacker, el: JsonElement) {
    when (el) {
        is JsonNull -> packer.packNil()
        is JsonObject -> {
            packer.packMapHeader(el.size)
            for ((k, v) in el) {
                packer.packString(k)
                packElement(packer, v)
            }
        }
        is JsonArray -> {
            packer.packArrayHeader(el.size)
            for (v in el) packElement(packer, v)
        }
        is JsonPrimitive -> {
            when {
                el.isString -> packer.packString(el.content)
                el.booleanOrNull != null -> packer.packBoolean(el.boolean)
                el.longOrNull != null -> packer.packLong(el.long)
                el.doubleOrNull != null -> packer.packDouble(el.double)
                else -> packer.packString(el.content)
            }
        }
    }
}

/**
 * Unpack a MessagePack byte array into a [JsonElement], value-identical to the
 * JSON wire shape. A msgpack bin (a Binary field) is decoded as a number array
 * — matching the server's JSON representation of binary fields.
 */
internal fun msgpackToJsonElement(bytes: ByteArray): JsonElement {
    val unpacker = MessagePack.newDefaultUnpacker(bytes)
    val el = unpackValue(unpacker)
    unpacker.close()
    return el
}

private fun unpackValue(unpacker: MessageUnpacker): JsonElement {
    return when (unpacker.nextFormat.valueType) {
        ValueType.NIL -> {
            unpacker.unpackNil()
            JsonNull
        }
        ValueType.BOOLEAN -> JsonPrimitive(unpacker.unpackBoolean())
        ValueType.INTEGER -> JsonPrimitive(unpacker.unpackLong())
        ValueType.FLOAT -> JsonPrimitive(unpacker.unpackDouble())
        ValueType.STRING -> JsonPrimitive(unpacker.unpackString())
        ValueType.BINARY -> {
            val len = unpacker.unpackBinaryHeader()
            val arr = ByteArray(len)
            unpacker.readPayload(arr)
            JsonArray(arr.map { JsonPrimitive(it.toInt() and 0xFF) })
        }
        ValueType.ARRAY -> {
            val len = unpacker.unpackArrayHeader()
            val list = ArrayList<JsonElement>(len)
            repeat(len) { list.add(unpackValue(unpacker)) }
            JsonArray(list)
        }
        ValueType.MAP -> {
            val len = unpacker.unpackMapHeader()
            val map = LinkedHashMap<String, JsonElement>(len)
            repeat(len) {
                val key = unpackValue(unpacker)
                val keyStr = (key as? JsonPrimitive)?.content ?: key.toString()
                map[keyStr] = unpackValue(unpacker)
            }
            JsonObject(map)
        }
        else -> {
            unpacker.skipValue()
            JsonNull
        }
    }
}
