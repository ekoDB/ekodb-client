package io.ekodb.client

import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonObject
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * The deployment classifies a provider failure on a stream error
 * (`error_kind`, `provider`, `provider_status`, `retry_after_secs`). The event
 * carries every field so a consumer can act on it without string-matching;
 * a plain error stays a plain error.
 */
class ChatStreamErrorTest {
    @Test
    fun `carries the provider failure classification`() {
        val payload = Json.parseToJsonElement(
            """{"chat_id":"c1","error":"OpenAI API error 429 Too Many Requests",
                "error_kind":"provider_rate_limited","provider":"openai",
                "provider_status":429,"retry_after_secs":7}"""
        ).jsonObject
        val event = ChatStreamEvent.Error.fromPayload(payload)
        assertEquals("OpenAI API error 429 Too Many Requests", event.error)
        assertEquals("provider_rate_limited", event.errorKind)
        assertEquals("openai", event.provider)
        assertEquals(429, event.providerStatus)
        assertEquals(7L, event.retryAfterSecs)
        assertTrue(event.isProviderFailure)
    }

    @Test
    fun `a plain error stays bare`() {
        val payload = Json.parseToJsonElement("""{"chat_id":"c1","error":"Model unavailable"}""").jsonObject
        val event = ChatStreamEvent.Error.fromPayload(payload)
        assertEquals("Model unavailable", event.error)
        assertNull(event.errorKind)
        assertNull(event.provider)
        assertNull(event.providerStatus)
        assertNull(event.retryAfterSecs)
        assertFalse(event.isProviderFailure)
        // The one-argument form the rest of the client uses still builds.
        assertEquals(event, ChatStreamEvent.Error("Model unavailable"))
    }

    @Test
    fun `falls back to message when error is absent`() {
        val payload = Json.parseToJsonElement("""{"chat_id":"c1","message":"boom"}""").jsonObject
        assertEquals("boom", ChatStreamEvent.Error.fromPayload(payload).error)
        assertEquals("Unknown error", ChatStreamEvent.Error.fromPayload(Json.parseToJsonElement("{}").jsonObject).error)
    }
}
