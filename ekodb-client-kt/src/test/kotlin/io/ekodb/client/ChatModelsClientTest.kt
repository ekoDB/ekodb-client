package io.ekodb.client

import io.ekodb.client.types.ChatProviderState
import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * `chatModels()` is the typed form of `GET /api/chat_models`: the four lists
 * plus the `providers` status map, decoded so a rejected key reads as
 * [ChatProviderState.AUTH_FAILED] and a status this build does not know as
 * [ChatProviderState.UNKNOWN], never as a decode failure.
 */
class ChatModelsClientTest {
    private fun clientServing(body: String): EkoDBClient {
        val engine = MockEngine { request ->
            val content = if (request.url.encodedPath.contains("/api/auth/token")) {
                """{"token":"mock_jwt_token"}"""
            } else {
                body
            }
            respond(
                content = content,
                status = HttpStatusCode.OK,
                headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString()),
            )
        }
        val http = HttpClient(engine) {
            install(ContentNegotiation) { json(Json { ignoreUnknownKeys = true }) }
        }
        return EkoDBClient.builder()
            .baseUrl("https://test.ekodb.net")
            .apiKey("test_api_key")
            .httpClient(http)
            .build()
    }

    @Test
    fun `decodes the lists and the provider statuses`() = runBlocking {
        val models = clientServing(
            """{"openai":[],"anthropic":["claude-sonnet-4-5"],"perplexity":[],"gemini":["gemini-2.5-flash"],
                "providers":{
                  "anthropic":{"status":"ok","verified":true,"model_count":1},
                  "openai":{"status":"auth_failed","verified":true,"http_status":401,"message":"Incorrect API key provided"},
                  "perplexity":{"status":"not_configured","verified":false},
                  "gemini":{"status":"brand_new","verified":false}}}""",
        ).chatModels()
        assertEquals(listOf("claude-sonnet-4-5"), models.anthropic)
        assertEquals(listOf("gemini-2.5-flash"), models.gemini)
        assertEquals(ChatProviderState.AUTH_FAILED, models.providers.getValue("openai").status)
        assertEquals(401, models.providers.getValue("openai").httpStatus)
        assertFalse(models.providers.getValue("openai").isUsable)
        assertTrue(models.providers.getValue("anthropic").isUsable)
        assertEquals(ChatProviderState.NOT_CONFIGURED, models.providers.getValue("perplexity").status)
        assertEquals(ChatProviderState.UNKNOWN, models.providers.getValue("gemini").status)
    }

    @Test
    fun `an older server without gemini or providers still decodes`() = runBlocking {
        val models = clientServing("""{"openai":["gpt-4o"],"anthropic":[],"perplexity":[]}""").chatModels()
        assertEquals(listOf("gpt-4o"), models.openai)
        assertTrue(models.gemini.isEmpty())
        assertTrue(models.providers.isEmpty())
    }
}
