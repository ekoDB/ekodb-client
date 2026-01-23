package io.ekodb.client

import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import kotlin.test.*

/**
 * Unit tests for KV batch operations in Kotlin client
 */
class KVBatchOperationsTest {

    private val testBaseUrl = "https://test.ekodb.io"
    private val testApiKey = "test_api_key_123"

    private fun createMockEngine(responseBody: String, statusCode: HttpStatusCode = HttpStatusCode.OK): MockEngine {
        return MockEngine { request ->
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

    @Test
    fun `kvBatchGet returns multiple values`() = runBlocking {
        val mockEngine = createMockEngine(
            """[
                {"data": "value1"},
                {"data": "value2"},
                {"data": "value3"}
            ]"""
        )
        val client = createTestClient(mockEngine)

        val keys = listOf("key1", "key2", "key3")
        val results = client.kvBatchGet(keys)

        assertEquals(3, results.size)
        assertNotNull(results[0])
        assertNotNull(results[1])
        assertNotNull(results[2])
    }

    @Test
    fun `kvBatchSet sets multiple keys successfully`() = runBlocking {
        val mockEngine = createMockEngine(
            """[
                ["key1", true],
                ["key2", true],
                ["key3", true]
            ]"""
        )
        val client = createTestClient(mockEngine)

        val entries = listOf(
            Triple("key1", buildJsonObject { put("data", "value1") }, null),
            Triple("key2", buildJsonObject { put("data", "value2") }, null),
            Triple("key3", buildJsonObject { put("data", "value3") }, null)
        )
        val results = client.kvBatchSet(entries)

        assertEquals(3, results.size)
        assertTrue(results[0].second)
        assertTrue(results[1].second)
        assertTrue(results[2].second)
    }

    @Test
    fun `kvBatchSet with TTL sets keys with expiration`() = runBlocking {
        val mockEngine = createMockEngine(
            """[
                ["key1", true],
                ["key2", true]
            ]"""
        )
        val client = createTestClient(mockEngine)

        val entries = listOf(
            Triple("key1", buildJsonObject { put("data", "value1") }, 3600),
            Triple("key2", buildJsonObject { put("data", "value2") }, 3600)
        )
        val results = client.kvBatchSet(entries)

        assertEquals(2, results.size)
        assertTrue(results[0].second)
        assertTrue(results[1].second)
    }

    @Test
    fun `kvBatchDelete deletes multiple keys`() = runBlocking {
        val mockEngine = createMockEngine(
            """[
                ["key1", true],
                ["key2", true],
                ["key3", false]
            ]"""
        )
        val client = createTestClient(mockEngine)

        val keys = listOf("key1", "key2", "key3")
        val results = client.kvBatchDelete(keys)

        assertEquals(3, results.size)
        assertEquals("key1", results[0].first)
        assertTrue(results[0].second)
        assertEquals("key2", results[1].first)
        assertTrue(results[1].second)
        assertEquals("key3", results[2].first)
        assertFalse(results[2].second) // key3 not found
    }

    @Test
    fun `kvBatchGet with empty keys returns empty list`() = runBlocking {
        val mockEngine = createMockEngine("[]")
        val client = createTestClient(mockEngine)

        val results = client.kvBatchGet(emptyList())

        assertEquals(0, results.size)
    }

    @Test
    fun `kvBatchSet handles partial failures`() = runBlocking {
        val mockEngine = createMockEngine(
            """[
                ["key1", true],
                ["key2", false],
                ["key3", true]
            ]"""
        )
        val client = createTestClient(mockEngine)

        val entries = listOf(
            Triple("key1", buildJsonObject { put("data", "value1") }, null),
            Triple("key2", buildJsonObject { put("data", "value2") }, null),
            Triple("key3", buildJsonObject { put("data", "value3") }, null)
        )
        val results = client.kvBatchSet(entries)

        assertEquals(3, results.size)
        assertTrue(results[0].second)  // key1 succeeded
        assertFalse(results[1].second) // key2 failed
        assertTrue(results[2].second)  // key3 succeeded
    }

    @Test
    fun `kvBatchDelete with empty keys returns empty list`() = runBlocking {
        val mockEngine = createMockEngine("[]")
        val client = createTestClient(mockEngine)

        val results = client.kvBatchDelete(emptyList())

        assertEquals(0, results.size)
    }

    @Test
    fun `kvBatchGet handles large batch`() = runBlocking {
        val largeResponse = (1..100).joinToString(",", "[", "]") { """{"data": "value$it"}""" }
        val mockEngine = createMockEngine(largeResponse)
        val client = createTestClient(mockEngine)

        val keys = (1..100).map { "key$it" }
        val results = client.kvBatchGet(keys)

        assertEquals(100, results.size)
    }
}
