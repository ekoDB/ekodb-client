package io.ekodb.client

import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * The SSE chat stream recognises an error frame by its SSE event name as well
 * as by an `error` key, so a frame the server names `error` reaches the
 * consumer as [ChatStreamEvent.Error] whatever its payload calls the message —
 * the `message` fallback in [ChatStreamEvent.Error.fromPayload] is on a live
 * path, not a dead one.
 */
class ChatMessageStreamSseTest {
    private fun clientServing(sse: String): EkoDBClient {
        val engine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token":"mock_jwt_token"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString()),
                )
            } else {
                respond(
                    content = sse,
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Text.EventStream.toString()),
                )
            }
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

    private fun events(sse: String): List<ChatStreamEvent> = runBlocking {
        clientServing(sse)
            .chatMessageStream("c1", buildJsonObject { put("message", "hi") })
            .toList()
    }

    @Test
    fun `an error frame carries the provider classification`() {
        val sse = "event: token\ndata: {\"token\":\"Hel\"}\n\n" +
            "event: error\ndata: {\"error\":\"OpenAI API error 401\",\"error_kind\":\"provider_auth_failed\"," +
            "\"provider\":\"openai\",\"provider_status\":401}\n\n"
        val events = events(sse)
        assertEquals(2, events.size, "$events")
        assertTrue(events[0] is ChatStreamEvent.Chunk)
        val error = events[1] as ChatStreamEvent.Error
        assertEquals("OpenAI API error 401", error.error)
        assertEquals("provider_auth_failed", error.errorKind)
        assertEquals("openai", error.provider)
        assertEquals(401, error.providerStatus)
        assertTrue(error.isProviderFailure)
    }

    @Test
    fun `an error frame whose payload says message is still an error`() {
        val events = events("event: error\ndata: {\"message\":\"boom\"}\n\n")
        assertEquals(listOf<ChatStreamEvent>(ChatStreamEvent.Error("boom")), events)
    }
}
