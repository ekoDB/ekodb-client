package io.ekodb.client

import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Verifies that every caller-supplied PATH segment is percent-encoded with
 * ktor's [encodeURLPathPart], reaching parity with the Rust, Go, and TypeScript
 * clients. A reserved char (`/`, space, `#`, `?`) in a KV key, function label,
 * chat model name, id, collection, etc. must be encoded into the path rather
 * than producing a broken/ambiguous URL.
 *
 * The MockEngine records the LAST non-auth request so we can inspect the exact
 * encoded path the engine received.
 */
class UrlPathEncodingTest {
    private val testBaseUrl = "https://test.ekodb.net"
    private val testApiKey = "test_api_key_123"

    /**
     * MockEngine that satisfies the token exchange, captures the encoded path of
     * every subsequent request into [captured], and returns [responseBody].
     */
    private class PathCapture {
        var lastEncodedPath: String? = null
    }

    private fun captureClient(capture: PathCapture, responseBody: String = "{}"): EkoDBClient {
        val engine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                capture.lastEncodedPath = request.url.encodedPath
                respond(
                    content = responseBody,
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            }
        }
        val http = HttpClient(engine) {
            install(ContentNegotiation) {
                json(Json { ignoreUnknownKeys = true; isLenient = true })
            }
        }
        return EkoDBClient.builder()
            .baseUrl(testBaseUrl)
            .apiKey(testApiKey)
            .httpClient(http)
            .build()
    }

    // ========================================================================
    // Reserved-char segments must be percent-encoded
    // ========================================================================

    @Test
    fun `kv key with slash is percent-encoded in the path`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = """{"value": null}""")
        client.kvGet("session/abc")
        val path = capture.lastEncodedPath!!
        // The slash in the key must become %2F, not split into a new path segment.
        assertTrue(path.contains("%2F"), "expected %2F in path, got: $path")
        assertEquals("/api/kv/get/session%2Fabc", path)
    }

    @Test
    fun `kv set key with slash is percent-encoded`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture)
        client.kvSet("a/b", JsonPrimitive("v"))
        val path = capture.lastEncodedPath!!
        assertEquals("/api/kv/set/a%2Fb", path)
    }

    @Test
    fun `kv delete key with slash is percent-encoded`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture)
        client.kvDelete("a/b")
        val path = capture.lastEncodedPath!!
        assertEquals("/api/kv/delete/a%2Fb", path)
    }

    @Test
    fun `function label with slash is percent-encoded`() = runBlocking {
        val capture = PathCapture()
        // callFunction deserializes into FunctionResult, so return a minimal valid body.
        val body = """{"records": [], "stats": {"input_count": 0, "output_count": 0, """ +
            """"execution_time_ms": 0, "stages_executed": 0, "stage_stats": []}}"""
        val client = captureClient(capture, responseBody = body)
        client.callFunction("my/label")
        val path = capture.lastEncodedPath!!
        assertTrue(path.contains("%2F"), "expected %2F in path, got: $path")
        assertEquals("/api/functions/my%2Flabel", path)
    }

    @Test
    fun `chat model name with slash is percent-encoded`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = "[]")
        client.getChatModel("anthropic/claude-3")
        val path = capture.lastEncodedPath!!
        assertTrue(path.contains("%2F"), "expected %2F in path, got: $path")
        assertEquals("/api/chat_models/anthropic%2Fclaude-3", path)
    }

    @Test
    fun `agent name with space is percent-encoded`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture)
        client.agentGetByName("my agent")
        val path = capture.lastEncodedPath!!
        // A space encodes to %20 in a path segment.
        assertEquals("/api/chat/agents/by-name/my%20agent", path)
    }

    @Test
    fun `findById id with reserved chars is percent-encoded`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = "{}")
        client.findById("users", "id/with#hash")
        val path = capture.lastEncodedPath!!
        assertTrue(path.contains("%2F"), "expected %2F in path, got: $path")
        assertTrue(path.contains("%23"), "expected %23 (#) in path, got: $path")
        assertEquals("/api/find/users/id%2Fwith%23hash", path)
    }

    @Test
    fun `collection name with slash is percent-encoded on find`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = "[]")
        client.find("odd/collection", io.ekodb.client.types.Query.new())
        val path = capture.lastEncodedPath!!
        assertEquals("/api/find/odd%2Fcollection", path)
    }

    @Test
    fun `chat message path encodes both chatId and messageId`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = "{}")
        client.getChatMessage("chat/1", "msg/2")
        val path = capture.lastEncodedPath!!
        assertEquals("/api/chat/chat%2F1/messages/msg%2F2", path)
    }

    @Test
    fun `distinct path encodes collection and field`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = "{}")
        client.distinctValues("a/b", "c/d")
        val path = capture.lastEncodedPath!!
        assertEquals("/api/distinct/a%2Fb/c%2Fd", path)
    }

    // ========================================================================
    // Normal segments stay unchanged
    // ========================================================================

    @Test
    fun `normal kv key is unchanged`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = """{"value": null}""")
        client.kvGet("plainKey")
        assertEquals("/api/kv/get/plainKey", capture.lastEncodedPath)
    }

    @Test
    fun `normal findById path is unchanged`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = "{}")
        client.findById("users", "abc123")
        assertEquals("/api/find/users/abc123", capture.lastEncodedPath)
    }

    @Test
    fun `static path words are not encoded`() = runBlocking {
        val capture = PathCapture()
        val client = captureClient(capture, responseBody = "{}")
        // by-name is a static word; only the name segment is a caller value.
        client.agentGetByName("plain")
        assertEquals("/api/chat/agents/by-name/plain", capture.lastEncodedPath)
    }
}
