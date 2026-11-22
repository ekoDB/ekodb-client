package io.ekodb.client

import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.HttpResponseData
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.test.runTest
import kotlin.test.*

/**
 * A call on an existing transaction (commit, rollback, or a savepoint call)
 * answers a failure with `{"error", "transaction_id", "retryable"}`. A failed
 * commit has already discarded its transaction, so the client never re-sends
 * those calls, even with retries enabled: the caller sees the server's own
 * message and flag, and `retryable` means begin a new transaction (#406). Begin
 * and status keep the normal retry policy, and a 401 is sent once more after
 * the token is refreshed.
 */
class TransactionErrorTest {
    /**
     * A client with retries enabled (3 attempts) whose transaction requests are
     * answered by [answer] and recorded in [paths]; token exchanges are recorded
     * in [tokenRequests]. A single recorded request therefore proves the client
     * chose not to re-send.
     */
    private fun client(
        paths: MutableList<String>,
        tokenRequests: MutableList<String> = mutableListOf(),
        answer: MockRequestHandleScope.() -> HttpResponseData,
    ): EkoDBClient {
        val engine = MockEngine { request ->
            if (request.url.encodedPath == "/api/auth/token") {
                tokenRequests.add(request.url.encodedPath)
                respond("""{"token":"test-token"}""", headers = headersOf(HttpHeaders.ContentType, "application/json"))
            } else {
                paths.add(request.url.encodedPath)
                answer()
            }
        }
        return EkoDBClient.builder().baseUrl("http://localhost").apiKey("test-key").maxRetries(3)
            .httpClient(HttpClient(engine) { install(ContentNegotiation) { json() } }).build()
    }

    private fun client(
        status: HttpStatusCode,
        body: String,
        contentType: String,
        paths: MutableList<String>,
    ): EkoDBClient = client(paths) { respond(body, status, headersOf(HttpHeaders.ContentType, contentType)) }

    @Test
    fun `409 commit is a retryable EkoDBHttpException sent once`() = runTest {
        val body = """{"error":"write conflict on users/1","transaction_id":"tx-1","retryable":true}"""
        val paths = mutableListOf<String>()
        val client = client(HttpStatusCode.Conflict, body, "application/json", paths)
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.commitTransaction("tx-1") }
            assertEquals(409, error.statusCode)
            assertEquals("write conflict on users/1", error.message)
            assertTrue(error.retryable)
            assertEquals(body, error.responseBody)
            assertEquals(listOf("/api/transactions/tx-1/commit"), paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `400 commit is not retryable and is sent once`() = runTest {
        val body = """{"error":"unique constraint violated","transaction_id":"tx-2","retryable":false}"""
        val paths = mutableListOf<String>()
        val client = client(HttpStatusCode.BadRequest, body, "application/json", paths)
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.commitTransaction("tx-2") }
            assertEquals(400, error.statusCode)
            assertEquals("unique constraint violated", error.message)
            assertFalse(error.retryable)
            assertEquals(listOf("/api/transactions/tx-2/commit"), paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `503 is sent once whatever its flag`() = runTest {
        val cases = listOf(
            """{"error":"at capacity","transaction_id":"tx-3","retryable":false}""" to false,
            """{"error":"shutting down","transaction_id":"tx-3","retryable":true}""" to true,
        )
        for ((body, retryable) in cases) {
            val paths = mutableListOf<String>()
            val client = client(HttpStatusCode.ServiceUnavailable, body, "application/json", paths)
            try {
                val error = assertFailsWith<EkoDBHttpException> { client.commitTransaction("tx-3") }
                assertEquals(503, error.statusCode)
                assertEquals(retryable, error.retryable)
                assertEquals(listOf("/api/transactions/tx-3/commit"), paths)
            } finally {
                client.close()
            }
        }
    }

    @Test
    fun `non-JSON error body falls back to the raw text`() = runTest {
        val paths = mutableListOf<String>()
        val client = client(HttpStatusCode.BadGateway, "upstream connect error", "text/plain", paths)
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.commitTransaction("tx-4") }
            assertEquals(502, error.statusCode)
            assertEquals("upstream connect error", error.message)
            assertEquals("upstream connect error", error.responseBody)
            assertFalse(error.retryable)
            assertEquals(listOf("/api/transactions/tx-4/commit"), paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `commit is not re-sent after a transport failure`() = runTest {
        val paths = mutableListOf<String>()
        val client = client(paths) { throw java.io.IOException("connection reset") }
        try {
            val error = assertFailsWith<Exception> { client.commitTransaction("tx-5") }
            assertTrue(error.message.orEmpty().contains("connection reset"), "got ${error.message}")
            assertEquals(listOf("/api/transactions/tx-5/commit"), paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `every call on an existing transaction throws the typed error`() = runTest {
        val body = """{"error":"transaction tx-9 not found","transaction_id":"tx-9","retryable":false}"""
        val paths = mutableListOf<String>()
        val client = client(HttpStatusCode.NotFound, body, "application/json", paths)
        val calls = listOf<suspend () -> Unit>(
            { client.rollbackTransaction("tx-9") },
            { client.createSavepoint("tx-9", "sp1") },
            { client.rollbackToSavepoint("tx-9", "sp1") },
            { client.releaseSavepoint("tx-9", "sp1") },
        )
        try {
            for (call in calls) {
                val error = assertFailsWith<EkoDBHttpException> { call() }
                assertEquals(404, error.statusCode)
                assertEquals("transaction tx-9 not found", error.message)
                assertFalse(error.retryable)
            }
            assertEquals(calls.size, paths.size)
        } finally {
            client.close()
        }
    }

    @Test
    fun `429 commit is sent once as the typed error`() = runTest {
        val body = """{"error":"too many requests","transaction_id":"tx-6","retryable":true}"""
        val paths = mutableListOf<String>()
        val client = client(paths) {
            respond(
                body,
                HttpStatusCode.TooManyRequests,
                headersOf(HttpHeaders.ContentType to listOf("application/json"), "Retry-After" to listOf("0")),
            )
        }
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.commitTransaction("tx-6") }
            assertEquals(429, error.statusCode)
            assertEquals("too many requests", error.message)
            assertTrue(error.retryable)
            assertEquals(listOf("/api/transactions/tx-6/commit"), paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `429 begin is retried and succeeds`() = runTest {
        val paths = mutableListOf<String>()
        val client = client(paths) {
            if (paths.size == 1) {
                respond("", HttpStatusCode.TooManyRequests, headersOf("Retry-After", "0"))
            } else {
                respond("""{"transaction_id":"tx-new"}""", headers = headersOf(HttpHeaders.ContentType, "application/json"))
            }
        }
        try {
            assertEquals("tx-new", client.beginTransaction())
            // A refused begin started no transaction, so it is sent again.
            assertEquals(listOf("/api/transactions", "/api/transactions"), paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `429 begin after every retry is the normal EkoDBHttpException`() = runTest {
        val paths = mutableListOf<String>()
        val client = client(paths) {
            respond("rate limited", HttpStatusCode.TooManyRequests, headersOf("Retry-After", "0"))
        }
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.beginTransaction() }
            assertEquals(429, error.statusCode)
            assertEquals("Request failed with status 429: rate limited", error.message)
            assertFalse(error.retryable)
            // All three attempts were sent.
            assertEquals(List(3) { "/api/transactions" }, paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `429 status read is retried and succeeds`() = runTest {
        val paths = mutableListOf<String>()
        val client = client(paths) {
            if (paths.size == 1) {
                respond("", HttpStatusCode.TooManyRequests, headersOf("Retry-After", "0"))
            } else {
                respond(
                    """{"state":"Active","operations_count":2}""",
                    headers = headersOf(HttpHeaders.ContentType, "application/json"),
                )
            }
        }
        try {
            assertEquals("Active", client.getTransactionStatus("tx-10")["state"])
            assertEquals(listOf("/api/transactions/tx-10", "/api/transactions/tx-10"), paths)
        } finally {
            client.close()
        }
    }

    @Test
    fun `401 commit refreshes the token once and is sent once more`() = runTest {
        val paths = mutableListOf<String>()
        val tokenRequests = mutableListOf<String>()
        val client = client(paths, tokenRequests) {
            if (paths.size == 1) {
                respond("""{"error":"token expired"}""", HttpStatusCode.Unauthorized)
            } else {
                respond("""{"status":"committed"}""", headers = headersOf(HttpHeaders.ContentType, "application/json"))
            }
        }
        try {
            client.commitTransaction("tx-11")
            assertEquals(List(2) { "/api/transactions/tx-11/commit" }, paths)
            // The first token exchange and the one refresh.
            assertEquals(2, tokenRequests.size)
        } finally {
            client.close()
        }
    }

    @Test
    fun `401 commit twice is sent twice and then throws`() = runTest {
        val paths = mutableListOf<String>()
        val tokenRequests = mutableListOf<String>()
        val client = client(paths, tokenRequests) {
            respond("""{"error":"token expired"}""", HttpStatusCode.Unauthorized)
        }
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.commitTransaction("tx-12") }
            assertEquals(401, error.statusCode)
            assertEquals("token expired", error.message)
            assertEquals(List(2) { "/api/transactions/tx-12/commit" }, paths)
            assertEquals(2, tokenRequests.size)
        } finally {
            client.close()
        }
    }

    @Test
    fun `transactionHttpException ignores a flag that is not a boolean`() {
        val error = transactionHttpException(409, """{"error":"conflict","retryable":"true"}""")
        assertEquals("conflict", error.message)
        assertFalse(error.retryable)
    }

    @Test
    fun `transactionHttpException falls back to the status for an empty body`() {
        val error = transactionHttpException(500, "")
        assertEquals("500 Internal Server Error", error.message)
        assertFalse(error.retryable)
    }

    @Test
    fun `two-argument constructor keeps its message and is not retryable`() {
        val error = EkoDBHttpException(500, "boom")
        assertEquals("Request failed with status 500: boom", error.message)
        assertFalse(error.retryable)
    }

    @Test
    fun `a MessagePack 409 commit carries the decoded message and the retryable flag`() = runTest {
        val packer = org.msgpack.core.MessagePack.newDefaultBufferPacker()
        packer.packMapHeader(3)
        packer.packString("error")
        packer.packString("write conflict on users/1")
        packer.packString("transaction_id")
        packer.packString("tx-1")
        packer.packString("retryable")
        packer.packBoolean(true)
        packer.close()
        val bytes = packer.toByteArray()
        val paths = mutableListOf<String>()
        val client = client(paths) {
            respond(bytes, HttpStatusCode.Conflict, headersOf(HttpHeaders.ContentType, "application/msgpack"))
        }
        try {
            val error = assertFailsWith<EkoDBHttpException> { client.commitTransaction("tx-1") }
            assertEquals(409, error.statusCode)
            assertEquals("write conflict on users/1", error.message)
            assertTrue(error.retryable)
            assertEquals(listOf("/api/transactions/tx-1/commit"), paths)
        } finally {
            client.close()
        }
    }
}
