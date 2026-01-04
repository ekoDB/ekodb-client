package io.ekodb.client

import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue
import kotlin.test.assertFalse

/**
 * Unit tests for ekoDB Kotlin client - EkoDBClient methods
 * Tests client methods with mock HTTP responses
 */
class EkoDBClientTest {

    private val testBaseUrl = "https://test.ekodb.io"
    private val testApiKey = "test_api_key_123"

    // Helper to create mock engine that returns token first, then the actual response
    private fun createMockEngine(responseBody: String, statusCode: HttpStatusCode = HttpStatusCode.OK): MockEngine {
        var requestCount = 0
        return MockEngine { request ->
            requestCount++
            // First request is always token request
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = responseBody,
                    status = statusCode,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            }
        }
    }

    // Helper to create test client with mock engine
    private fun createTestClient(mockEngine: MockEngine): EkoDBClient {
        val mockHttpClient = HttpClient(mockEngine) {
            install(ContentNegotiation) {
                json(Json {
                    ignoreUnknownKeys = true
                    isLenient = true
                })
            }
        }
        return EkoDBClient.builder()
            .baseUrl(testBaseUrl)
            .apiKey(testApiKey)
            .httpClient(mockHttpClient)
            .build()
    }

    // ========================================================================
    // Health Check Tests
    // ========================================================================

    @Test
    fun `health returns true when server is healthy`() = runBlocking {
        val mockEngine = MockEngine { request ->
            respond(
                content = """{"status": "ok"}""",
                status = HttpStatusCode.OK,
                headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
            )
        }
        val client = createTestClient(mockEngine)
        val result = client.health()
        assertTrue(result)
    }

    @Test
    fun `health returns false when server is unhealthy`() = runBlocking {
        val mockEngine = MockEngine { request ->
            respond(
                content = """{"error": "unhealthy"}""",
                status = HttpStatusCode.ServiceUnavailable,
                headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
            )
        }
        val client = createTestClient(mockEngine)
        val result = client.health()
        assertFalse(result)
    }

    // ========================================================================
    // Collection Tests
    // ========================================================================

    @Test
    fun `listCollections returns collection names`() = runBlocking {
        val mockEngine = createMockEngine("""{"collections": ["users", "orders", "products"]}""")
        val client = createTestClient(mockEngine)
        val result = client.listCollections()
        assertNotNull(result)
        assertEquals(3, result.size)
        assertTrue(result.contains("users"))
    }

    @Test
    fun `collectionExists returns true for existing collection`() = runBlocking {
        val mockEngine = createMockEngine("""{"collections": ["users", "orders"]}""")
        val client = createTestClient(mockEngine)
        val result = client.collectionExists("users")
        assertTrue(result)
    }

    @Test
    fun `collectionExists returns false for non-existing collection`() = runBlocking {
        val mockEngine = createMockEngine("""{"collections": ["users", "orders"]}""")
        val client = createTestClient(mockEngine)
        val result = client.collectionExists("nonexistent")
        assertFalse(result)
    }

    // ========================================================================
    // Restore Tests
    // ========================================================================

    @Test
    fun `restoreRecord returns true on success`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "restored"}""")
        val client = createTestClient(mockEngine)
        val result = client.restoreRecord("users", "deleted_user_123")
        assertTrue(result)
    }

    @Test
    fun `restoreCollection returns count`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "restored", "records_restored": 5}""")
        val client = createTestClient(mockEngine)
        val result = client.restoreCollection("users")
        assertEquals(5, result)
    }

    // ========================================================================
    // Transaction Tests
    // ========================================================================

    @Test
    fun `beginTransaction returns transaction id`() = runBlocking {
        val mockEngine = createMockEngine("""{"transaction_id": "tx_123456"}""")
        val client = createTestClient(mockEngine)
        val result = client.beginTransaction()
        assertNotNull(result)
        assertEquals("tx_123456", result)
    }

    @Test
    fun `getTransactionStatus returns result`() = runBlocking {
        val mockEngine = createMockEngine("""{"transaction_id": "tx_123", "status": "active"}""")
        val client = createTestClient(mockEngine)
        val result = client.getTransactionStatus("tx_123")
        assertNotNull(result)
        // Just verify we got a result back
        assertTrue(result.isNotEmpty())
    }

    // ========================================================================
    // Chat Tests
    // ========================================================================

    @Test
    fun `createChatSession returns session`() = runBlocking {
        val mockEngine = createMockEngine("""{"chat_id": "chat_123", "created": true}""")
        val client = createTestClient(mockEngine)
        val request = buildJsonObject {
            put("llm_provider", "openai")
            putJsonArray("collections") {
                add(buildJsonObject { put("collection_name", "documents") })
            }
        }
        val result = client.createChatSession(request)
        assertNotNull(result)
        assertEquals("chat_123", result["chat_id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `getChatSession returns session details`() = runBlocking {
        val mockEngine = createMockEngine("""{"chat_id": "chat_123", "llm_provider": "openai"}""")
        val client = createTestClient(mockEngine)
        val result = client.getChatSession("chat_123")
        assertNotNull(result)
        assertEquals("chat_123", result["chat_id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `listChatSessions returns sessions`() = runBlocking {
        val mockEngine = createMockEngine("""{"sessions": [{"chat_id": "chat_1"}, {"chat_id": "chat_2"}]}""")
        val client = createTestClient(mockEngine)
        val result = client.listChatSessions()
        assertNotNull(result)
    }

    // ========================================================================
    // Search Tests
    // ========================================================================

    @Test
    fun `search returns results`() = runBlocking {
        val mockEngine = createMockEngine("""{"results": [{"id": "doc_1", "score": 0.95}], "total": 1}""")
        val client = createTestClient(mockEngine)
        val searchQuery = buildJsonObject {
            put("query", "search term")
        }
        val result = client.search("documents", searchQuery)
        assertNotNull(result)
    }

    // ========================================================================
    // Schema Tests
    // ========================================================================

    @Test
    fun `getCollection returns collection metadata`() = runBlocking {
        val mockEngine = createMockEngine("""{"collection": "users", "count": 100}""")
        val client = createTestClient(mockEngine)
        val result = client.getCollection("users")
        assertNotNull(result)
    }

    @Test
    fun `getSchema returns schema`() = runBlocking {
        val mockEngine = createMockEngine("""{"fields": {"name": {"field_type": "String"}}}""")
        val client = createTestClient(mockEngine)
        val result = client.getSchema("users")
        assertNotNull(result)
    }
}
