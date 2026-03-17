package io.ekodb.client

import kotlinx.serialization.json.*
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.assertNotNull

class WebSocketClientTest {

    // ========================================================================
    // MutationNotification Tests
    // ========================================================================

    @Test
    fun `MutationNotification construction`() {
        val notification = MutationNotification(
            collection = "users",
            event = "insert",
            recordIds = listOf("1", "2", "3"),
            timestamp = "2026-03-13T00:00:00Z"
        )
        assertEquals("users", notification.collection)
        assertEquals("insert", notification.event)
        assertEquals(3, notification.recordIds.size)
        assertEquals("1", notification.recordIds[0])
        assertEquals("2026-03-13T00:00:00Z", notification.timestamp)
    }

    @Test
    fun `MutationNotification with records`() {
        val records = buildJsonArray {
            add(buildJsonObject { put("id", "1"); put("name", "Alice") })
        }
        val notification = MutationNotification(
            collection = "users",
            event = "update",
            recordIds = listOf("1"),
            records = records,
            timestamp = "2026-03-13T00:00:00Z"
        )
        val notifRecords = notification.records
        assertNotNull(notifRecords)
        assertEquals(1, notifRecords.jsonArray.size)
    }

    @Test
    fun `MutationNotification equality`() {
        val n1 = MutationNotification("col", "insert", listOf("1"), null, "ts")
        val n2 = MutationNotification("col", "insert", listOf("1"), null, "ts")
        assertEquals(n1, n2)
    }

    // ========================================================================
    // ChatStreamEvent Tests
    // ========================================================================

    @Test
    fun `ChatStreamEvent Chunk`() {
        val event = ChatStreamEvent.Chunk("Hello world")
        assertEquals("Hello world", event.content)
    }

    @Test
    fun `ChatStreamEvent End`() {
        val tokenUsage = buildJsonObject {
            put("prompt_tokens", 10)
            put("completion_tokens", 20)
        }
        val event = ChatStreamEvent.End(
            messageId = "msg-1",
            tokenUsage = tokenUsage,
            executionTimeMs = 500
        )
        assertEquals("msg-1", event.messageId)
        assertEquals(500, event.executionTimeMs)
        assertNotNull(event.tokenUsage)
    }

    @Test
    fun `ChatStreamEvent ToolCall`() {
        val args = buildJsonObject { put("location", "NYC") }
        val event = ChatStreamEvent.ToolCall(
            chatId = "chat-1",
            callId = "call-123",
            toolName = "get_weather",
            arguments = args
        )
        assertEquals("get_weather", event.toolName)
        assertEquals("call-123", event.callId)
        assertEquals("chat-1", event.chatId)
    }

    @Test
    fun `ChatStreamEvent Error`() {
        val event = ChatStreamEvent.Error("Model unavailable")
        assertEquals("Model unavailable", event.error)
    }

    @Test
    fun `ChatStreamEvent sealed class pattern matching`() {
        val events: List<ChatStreamEvent> = listOf(
            ChatStreamEvent.Chunk("hi"),
            ChatStreamEvent.End("msg-1", executionTimeMs = 100),
            ChatStreamEvent.Error("fail"),
            ChatStreamEvent.ToolCall("c1", "call-1", "tool1", buildJsonObject {})
        )

        var chunks = 0
        var ends = 0
        var errors = 0
        var toolCalls = 0

        for (event in events) {
            when (event) {
                is ChatStreamEvent.Chunk -> chunks++
                is ChatStreamEvent.End -> ends++
                is ChatStreamEvent.Error -> errors++
                is ChatStreamEvent.ToolCall -> toolCalls++
            }
        }

        assertEquals(1, chunks)
        assertEquals(1, ends)
        assertEquals(1, errors)
        assertEquals(1, toolCalls)
    }

    // ========================================================================
    // ClientToolDefinition Tests
    // ========================================================================

    @Test
    fun `ClientToolDefinition construction`() {
        val params = buildJsonObject {
            put("type", "object")
            put("properties", buildJsonObject {
                put("location", buildJsonObject { put("type", "string") })
            })
        }
        val tool = ClientToolDefinition(
            name = "get_weather",
            description = "Get current weather",
            parameters = params
        )
        assertEquals("get_weather", tool.name)
        assertEquals("Get current weather", tool.description)
        assertNotNull(tool.parameters)
    }

    @Test
    fun `ClientToolDefinition equality`() {
        val params = buildJsonObject { put("type", "object") }
        val t1 = ClientToolDefinition("tool", "desc", params)
        val t2 = ClientToolDefinition("tool", "desc", params)
        assertEquals(t1, t2)
    }

    // ========================================================================
    // ChatSendOptions Tests
    // ========================================================================

    @Test
    fun `ChatSendOptions defaults`() {
        val opts = ChatSendOptions()
        assertEquals(null, opts.bypassRipple)
        assertEquals(null, opts.clientTools)
        assertEquals(null, opts.maxIterations)
        assertEquals(null, opts.confirmTools)
        assertEquals(null, opts.excludeTools)
    }

    @Test
    fun `ChatSendOptions with values`() {
        val opts = ChatSendOptions(
            bypassRipple = true,
            maxIterations = 5,
            excludeTools = listOf("shell_exec")
        )
        assertEquals(true, opts.bypassRipple)
        assertEquals(5, opts.maxIterations)
        assertEquals(listOf("shell_exec"), opts.excludeTools)
    }

    // ========================================================================
    // SubscribeOptions Tests
    // ========================================================================

    @Test
    fun `SubscribeOptions defaults`() {
        val opts = SubscribeOptions()
        assertEquals(null, opts.filterField)
        assertEquals(null, opts.filterValue)
    }

    @Test
    fun `SubscribeOptions with filter`() {
        val opts = SubscribeOptions(filterField = "status", filterValue = "active")
        assertEquals("status", opts.filterField)
        assertEquals("active", opts.filterValue)
    }

    // ========================================================================
    // WebSocketMessage Tests
    // ========================================================================

    @Test
    fun `WebSocketMessage data class`() {
        val payload = buildJsonObject { put("collection", "test") }
        val msg = WebSocketMessage("FindAll", "123", payload)
        assertEquals("FindAll", msg.type)
        assertEquals("123", msg.messageId)
        assertEquals("test", msg.payload["collection"]?.jsonPrimitive?.content)
    }
}
