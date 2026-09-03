package io.ekodb.client

import io.ekodb.client.types.ChatModels
import io.ekodb.client.types.ChatProviderState
import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * `GET /api/chat_models` carries a `gemini` list and a per-provider `providers`
 * status map beside the three original lists. Both are defaulted so a server
 * that predates them still parses; an unknown status name is kept rather than
 * rejected so a newer server cannot break the client.
 */
class ChatModelsTest {
    private val json = Json { ignoreUnknownKeys = true }

    @Test
    fun `carries gemini and the per-provider status`() {
        val models = json.decodeFromString<ChatModels>(
            """
            {
              "openai": [],
              "anthropic": ["claude-sonnet-4-5"],
              "perplexity": ["sonar"],
              "gemini": ["gemini-2.5-flash"],
              "providers": {
                "anthropic": {"status": "ok", "verified": true, "model_count": 1},
                "openai": {"status": "auth_failed", "verified": true, "http_status": 401,
                           "message": "Failed to fetch OpenAI models: 401 Unauthorized"},
                "perplexity": {"status": "ok", "verified": false,
                               "message": "static model list; key not verified"}
              }
            }
            """.trimIndent()
        )
        assertEquals(listOf("gemini-2.5-flash"), models.gemini)
        val openai = models.providers.getValue("openai")
        assertEquals(ChatProviderState.AUTH_FAILED, openai.status)
        assertTrue(openai.verified)
        assertEquals(401, openai.httpStatus)
        assertEquals("Failed to fetch OpenAI models: 401 Unauthorized", openai.message)
        assertNull(openai.modelCount)
        assertFalse(openai.isUsable)
        val anthropic = models.providers.getValue("anthropic")
        assertEquals(ChatProviderState.OK, anthropic.status)
        assertEquals(1, anthropic.modelCount)
        assertTrue(anthropic.isUsable)
        assertFalse(models.providers.getValue("perplexity").verified)
    }

    @Test
    fun `a server without gemini or providers still parses`() {
        val models = json.decodeFromString<ChatModels>(
            """{"openai":["gpt-4o"],"anthropic":[],"perplexity":[]}"""
        )
        assertEquals(listOf("gpt-4o"), models.openai)
        assertTrue(models.gemini.isEmpty())
        assertTrue(models.providers.isEmpty())
    }

    @Test
    fun `an unknown status name is kept rather than rejected`() {
        val models = json.decodeFromString<ChatModels>(
            """{"openai":[],"anthropic":[],"perplexity":[],"gemini":[],
                "providers":{"openai":{"status":"brand_new","verified":false}}}"""
        )
        assertEquals(ChatProviderState.UNKNOWN, models.providers.getValue("openai").status)
    }
}
