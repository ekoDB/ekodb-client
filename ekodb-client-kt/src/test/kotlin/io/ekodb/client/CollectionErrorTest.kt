package io.ekodb.client

import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.test.runTest
import kotlinx.serialization.json.*
import kotlin.test.*

class CollectionErrorTest {
    private fun client(statuses: List<HttpStatusCode>, body: String, called: () -> Unit): EkoDBClient {
        var attempt = 0
        val engine = MockEngine { request ->
            if (request.url.encodedPath == "/api/auth/token") {
                respond("""{"token":"test-token"}""", headers = headersOf(HttpHeaders.ContentType, "application/json"))
            } else {
                called()
                assertEquals("/api/collections/disposable", request.url.encodedPath)
                assertEquals(HttpMethod.Post, request.method)
                val status = statuses[minOf(attempt++, statuses.lastIndex)]
                respond(body, status, headersOf(HttpHeaders.ContentType, "text/plain"))
            }
        }
        return EkoDBClient.builder().baseUrl("http://localhost").apiKey("test-key").maxRetries(3)
            .httpClient(HttpClient(engine) { install(ContentNegotiation) { json() } }).build()
    }

    @Test
    fun `400 preserves exact body and is not retried`() = runTest {
        val body = """{"error":"Invalid field type: 'vector'"}"""
        var calls = 0
        val client = client(listOf(HttpStatusCode.BadRequest), body) { calls++ }
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.createCollection("disposable", buildJsonObject {}) }
            assertEquals(400, error.statusCode)
            assertEquals(body, error.responseBody)
            assertEquals("Request failed with status 400: $body", error.message)
            assertEquals(1, calls)
        } finally {
            client.close()
        }
    }

    @Test
    fun `exhausted server error cannot silently succeed and retains plain text body`() = runTest {
        var calls = 0
        val client = client(listOf(HttpStatusCode.ServiceUnavailable), "storage unavailable") { calls++ }
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.createCollection("disposable", buildJsonObject {}) }
            assertEquals(503, error.statusCode)
            assertEquals("storage unavailable", error.responseBody)
            assertEquals(3, calls)
        } finally {
            client.close()
        }
    }

    @Test
    fun `transient server error still retries to successful creation`() = runTest {
        var calls = 0
        val client = client(listOf(HttpStatusCode.ServiceUnavailable, HttpStatusCode.Created), "") { calls++ }
        try {
            client.createCollection("disposable", buildJsonObject {})
            assertEquals(2, calls)
        } finally {
            client.close()
        }
    }
}
