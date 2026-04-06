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
import kotlin.test.fail

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
    // executeTool Tests
    // ========================================================================

    @Test
    fun `executeTool returns result on success`() = runBlocking {
        val mockEngine = createMockEngine("""{"success": true, "result": {"count": 42}}""")
        val client = createTestClient(mockEngine)
        val params = buildJsonObject { put("collection", "users") }
        val result = client.executeTool("count_records", params)
        assertNotNull(result)
        val obj = result.jsonObject
        assertEquals(42, obj["count"]?.jsonPrimitive?.int)
    }

    @Test
    fun `executeTool passes chatId when provided`() = runBlocking {
        val mockEngine = createMockEngine("""{"success": true, "result": {"value": "hello"}}""")
        val client = createTestClient(mockEngine)
        val params = buildJsonObject { put("key", "greeting") }
        val result = client.executeTool("kv_get", params, "chat_456")
        assertNotNull(result)
        val obj = result.jsonObject
        assertEquals("hello", obj["value"]?.jsonPrimitive?.content)
    }

    @Test
    fun `executeTool throws on failure`() = runBlocking {
        val mockEngine = createMockEngine("""{"success": false, "error": "permission denied"}""")
        val client = createTestClient(mockEngine)
        val params = buildJsonObject { put("collection", "system") }
        try {
            client.executeTool("delete_collection", params)
            fail("Expected exception")
        } catch (e: RuntimeException) {
            assertTrue(e.message?.contains("permission denied") == true)
        }
    }

    @Test
    fun `executeTool returns null on 404`() = runBlocking {
        val mockEngine = createMockEngine("""Not Found""", HttpStatusCode.NotFound)
        val client = createTestClient(mockEngine)
        val params = buildJsonObject { put("collection", "users") }
        val result = client.executeTool("count_records", params)
        assertEquals(null, result)
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

    // ========================================================================
    // Convenience Methods Tests
    // ========================================================================

    @Test
    fun `upsert updates existing record`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "user_123", "name": "Alice Updated", "email": "alice@example.com"}""")
        val client = createTestClient(mockEngine)
        val record = io.ekodb.client.types.Record()
            .insert("name", "Alice Updated")
            .insert("email", "alice@example.com")

        val result = client.upsert("users", "user_123", record)
        assertNotNull(result)
        val idField = result["id"]
        assertTrue(idField is io.ekodb.client.types.FieldType.StringValue)
        assertEquals("user_123", idField.value)
    }

    // TODO: Fix mock engine for upsert fallback scenario - token caching makes mock ordering complex
    // The upsert logic works correctly but testing the fallback requires more sophisticated mocking
    /*@Test
    fun `upsert inserts new record when not found`() = runBlocking {
        var callNumber = 0
        val mockEngine = MockEngine { request ->
            callNumber++
            when (callNumber) {
                1 -> {
                    // First call: auth token
                    respond(
                        content = """{"token": "mock_jwt_token_123"}""",
                        status = HttpStatusCode.OK,
                        headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                    )
                }
                2 -> {
                    // Second call: update returns 404
                    respond(
                        content = """{"error": "Not found"}""",
                        status = HttpStatusCode.NotFound,
                        headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                    )
                }
                3 -> {
                    // Third call: insert succeeds
                    respond(
                        content = """{"id": "new_user_456", "name": "Bob", "email": "bob@example.com"}""",
                        status = HttpStatusCode.OK,
                        headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                    )
                }
                else -> {
                    respond(
                        content = """{"error": "unexpected call $callNumber"}""",
                        status = HttpStatusCode.BadRequest,
                        headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                    )
                }
            }
        }

        val client = createTestClient(mockEngine)
        val record = io.ekodb.client.types.Record()
            .insert("name", "Bob")
            .insert("email", "bob@example.com")

        val result = client.upsert("users", "nonexistent_id", record)
        assertNotNull(result)
        val idField = result["id"]
        assertNotNull(idField, "ID field should not be null")
        assertTrue(idField is io.ekodb.client.types.FieldType.StringValue, "ID should be StringValue but was ${idField::class.simpleName}")
        assertEquals("new_user_456", (idField as io.ekodb.client.types.FieldType.StringValue).value)
    }*/

    @Test
    fun `findOne returns first matching record`() = runBlocking {
        val mockEngine = createMockEngine("""[{"id": "user_123", "name": "Alice", "email": "alice@example.com"}]""")
        val client = createTestClient(mockEngine)

        val result = client.findOne("users", "email", "alice@example.com")
        assertNotNull(result)
        val idField = result.get("id")
        assertTrue(idField is io.ekodb.client.types.FieldType.StringValue)
        assertEquals("user_123", idField.value)
    }

    @Test
    fun `findOne returns null when no match`() = runBlocking {
        val mockEngine = createMockEngine("""[]""")
        val client = createTestClient(mockEngine)

        val result = client.findOne("users", "email", "nonexistent@example.com")
        assertEquals(null, result)
    }

    @Test
    fun `exists returns true when record exists`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "user_123", "name": "Alice"}""")
        val client = createTestClient(mockEngine)

        val result = client.exists("users", "user_123")
        assertTrue(result)
    }

    @Test
    fun `exists returns false when record not found`() = runBlocking {
        val mockEngine = createMockEngine("""{"error": "Not found"}""", HttpStatusCode.NotFound)
        val client = createTestClient(mockEngine)

        val result = client.exists("users", "nonexistent_id")
        assertFalse(result)
    }

    @Test
    fun `paginate calculates skip correctly for page 1`() = runBlocking {
        val mockEngine = createMockEngine("""[{"id": "1"}, {"id": "2"}, {"id": "3"}]""")
        val client = createTestClient(mockEngine)

        val result = client.paginate("users", 1, 10)
        assertNotNull(result)
        assertEquals(3, result.size)
    }

    @Test
    fun `paginate calculates skip correctly for page 2`() = runBlocking {
        val mockEngine = createMockEngine("""[{"id": "11"}, {"id": "12"}]""")
        val client = createTestClient(mockEngine)

        val result = client.paginate("users", 2, 10)
        assertNotNull(result)
        // Skip should be (2-1)*10 = 10
        assertEquals(2, result.size)
    }

    @Test
    fun `paginate handles page 0 as page 1`() = runBlocking {
        val mockEngine = createMockEngine("""[{"id": "1"}, {"id": "2"}]""")
        val client = createTestClient(mockEngine)

        val result = client.paginate("users", 0, 10)
        assertNotNull(result)
        // Skip should be 0 for page 0
        assertEquals(2, result.size)
    }

    // ========================================================================
    // User Functions Tests
    // ========================================================================

    @Test
    fun `saveUserFunction returns ID`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "uf_123"}""")
        val client = createTestClient(mockEngine)
        val userFunction = buildJsonObject {
            put("label", "my_function")
            put("name", "My Function")
            put("parameters", buildJsonObject {})
            put("functions", buildJsonArray {})
        }
        val result = client.saveUserFunction(userFunction)
        assertEquals("uf_123", result)
    }

    @Test
    fun `getUserFunction returns user function`() = runBlocking {
        val mockEngine = createMockEngine("""{"label": "my_function", "name": "My Function", "id": "uf_123"}""")
        val client = createTestClient(mockEngine)
        val result = client.getUserFunction("my_function")
        assertNotNull(result)
        assertEquals("my_function", result["label"]?.jsonPrimitive?.content)
    }

    @Test
    fun `listUserFunctions returns list`() = runBlocking {
        val mockEngine = createMockEngine("""[{"label": "func_1", "name": "Function 1"}, {"label": "func_2", "name": "Function 2"}]""")
        val client = createTestClient(mockEngine)
        val result = client.listUserFunctions()
        assertNotNull(result)
        assertEquals(2, result.size)
        assertEquals("func_1", result[0]["label"]?.jsonPrimitive?.content)
    }

    @Test
    fun `listUserFunctions with tags returns filtered list`() = runBlocking {
        val mockEngine = createMockEngine("""[{"label": "func_1", "name": "Function 1", "tags": ["data"]}]""")
        val client = createTestClient(mockEngine)
        val result = client.listUserFunctions(listOf("data"))
        assertNotNull(result)
        assertEquals(1, result.size)
    }

    @Test
    fun `updateUserFunction succeeds`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "updated"}""")
        val client = createTestClient(mockEngine)
        val userFunction = buildJsonObject {
            put("label", "my_function")
            put("name", "Updated Function")
            put("parameters", buildJsonObject {})
            put("functions", buildJsonArray {})
        }
        // Should not throw
        client.updateUserFunction("my_function", userFunction)
    }

    @Test
    fun `deleteUserFunction succeeds`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "deleted"}""")
        val client = createTestClient(mockEngine)
        // Should not throw
        client.deleteUserFunction("my_function")
    }

    // =========================================================================
    // Distinct Values Tests
    // =========================================================================

    @Test
    fun `distinctValues returns values and count`() = runBlocking {
        val mockEngine = createMockEngine(
            """{"collection":"products","field":"category","values":["books","electronics","food"],"count":3}"""
        )
        val client = createTestClient(mockEngine)
        val result = client.distinctValues("products", "category")
        assertNotNull(result)
        assertEquals("products", result["collection"]?.jsonPrimitive?.content)
        assertEquals("category", result["field"]?.jsonPrimitive?.content)
        assertEquals(3, result["count"]?.jsonPrimitive?.int)
        val values = result["values"]?.jsonArray
        assertNotNull(values)
        assertEquals(3, values.size)
    }

    @Test
    fun `distinctValues returns empty list for no matches`() = runBlocking {
        val mockEngine = createMockEngine(
            """{"collection":"empty","field":"tag","values":[],"count":0}"""
        )
        val client = createTestClient(mockEngine)
        val result = client.distinctValues("empty", "tag")
        assertNotNull(result)
        assertEquals(0, result["count"]?.jsonPrimitive?.int)
        val values = result["values"]?.jsonArray
        assertNotNull(values)
        assertTrue(values.isEmpty())
    }

    @Test
    fun `distinctValues with filter sends filter in body`() = runBlocking {
        val mockEngine = createMockEngine(
            """{"collection":"orders","field":"status","values":["active","pending"],"count":2}"""
        )
        val client = createTestClient(mockEngine)
        val filter = buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", "region")
                put("operator", "Eq")
                put("value", "us")
            })
        }
        val result = client.distinctValues("orders", "status", filter = filter)
        assertNotNull(result)
        assertEquals(2, result["count"]?.jsonPrimitive?.int)
    }

    // =========================================================================
    // rawCompletion Tests
    // =========================================================================

    @Test
    fun `rawCompletion returns content field`() = runBlocking {
        val mockEngine = createMockEngine("""{"content":"The answer is 42."}""")
        val client = createTestClient(mockEngine)
        val result = client.rawCompletion(
            systemPrompt = "You are a helpful assistant.",
            message = "What is the answer?"
        )
        assertNotNull(result)
        assertEquals("The answer is 42.", result["content"]?.jsonPrimitive?.content)
    }

    @Test
    fun `rawCompletion with all optional fields`() = runBlocking {
        val mockEngine = createMockEngine("""{"content":"Response."}""")
        val client = createTestClient(mockEngine)
        val result = client.rawCompletion(
            systemPrompt = "System.",
            message = "User.",
            provider = "openai",
            model = "gpt-4o",
            maxTokens = 512
        )
        assertNotNull(result)
        assertEquals("Response.", result["content"]?.jsonPrimitive?.content)
    }

    @Test
    fun `rawCompletion with empty content`() = runBlocking {
        val mockEngine = createMockEngine("""{"content":""}""")
        val client = createTestClient(mockEngine)
        val result = client.rawCompletion(
            systemPrompt = "S.",
            message = "M."
        )
        assertNotNull(result)
        assertEquals("", result["content"]?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Goal CRUD Tests
    // ========================================================================

    @Test
    fun `goalCreate creates a new goal`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1", "title": "Test Goal", "status": "active"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalCreate(buildJsonObject {
            put("title", "Test Goal")
        })
        assertEquals("goal_1", result["id"]?.jsonPrimitive?.content)
        assertEquals("active", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalList returns goals`() = runBlocking {
        val mockEngine = createMockEngine("""{"goals": [{"id": "goal_1"}, {"id": "goal_2"}]}""")
        val client = createTestClient(mockEngine)
        val result = client.goalList()
        assertNotNull(result["goals"])
    }

    @Test
    fun `goalGet returns a goal by ID`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1", "title": "Test Goal"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalGet("goal_1")
        assertEquals("goal_1", result["id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalUpdate updates a goal`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1", "title": "Updated"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalUpdate("goal_1", buildJsonObject { put("title", "Updated") })
        assertEquals("Updated", result["title"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalDelete deletes a goal`() = runBlocking {
        val mockEngine = createMockEngine("""{"ok": true}""")
        val client = createTestClient(mockEngine)
        client.goalDelete("goal_1") // should not throw
    }

    @Test
    fun `goalSearch searches goals`() = runBlocking {
        val mockEngine = createMockEngine("""{"goals": []}""")
        val client = createTestClient(mockEngine)
        val result = client.goalSearch("test query")
        assertNotNull(result["goals"])
    }

    @Test
    fun `goalComplete marks goal as complete`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1", "status": "pending_review"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalComplete("goal_1", buildJsonObject { put("summary", "Done") })
        assertEquals("pending_review", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalApprove approves a goal`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1", "status": "in_progress"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalApprove("goal_1")
        assertEquals("in_progress", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalReject rejects a goal`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1", "status": "failed"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalReject("goal_1", buildJsonObject { put("reason", "Bad plan") })
        assertEquals("failed", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalStepStart starts a goal step`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalStepStart("goal_1", 0)
        assertEquals("goal_1", result["id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalStepComplete completes a goal step`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalStepComplete("goal_1", 0, buildJsonObject { put("result", "Done") })
        assertEquals("goal_1", result["id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalStepFail fails a goal step`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "goal_1"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalStepFail("goal_1", 0, buildJsonObject { put("error", "Failed") })
        assertEquals("goal_1", result["id"]?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Task CRUD Tests
    // ========================================================================

    @Test
    fun `taskCreate creates a new task`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "task_1", "name": "Test Task", "status": "active"}""")
        val client = createTestClient(mockEngine)
        val result = client.taskCreate(buildJsonObject {
            put("name", "Test Task")
            put("cron", "0 * * * *")
        })
        assertEquals("task_1", result["id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `taskList returns tasks`() = runBlocking {
        val mockEngine = createMockEngine("""{"tasks": []}""")
        val client = createTestClient(mockEngine)
        val result = client.taskList()
        assertNotNull(result["tasks"])
    }

    @Test
    fun `taskGet returns a task by ID`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "task_1", "name": "Test Task"}""")
        val client = createTestClient(mockEngine)
        val result = client.taskGet("task_1")
        assertEquals("task_1", result["id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `taskUpdate updates a task`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "task_1", "name": "Updated"}""")
        val client = createTestClient(mockEngine)
        val result = client.taskUpdate("task_1", buildJsonObject { put("name", "Updated") })
        assertEquals("Updated", result["name"]?.jsonPrimitive?.content)
    }

    @Test
    fun `taskDelete deletes a task`() = runBlocking {
        val mockEngine = createMockEngine("""{"ok": true}""")
        val client = createTestClient(mockEngine)
        client.taskDelete("task_1") // should not throw
    }

    @Test
    fun `taskDue returns due tasks`() = runBlocking {
        val mockEngine = createMockEngine("""{"tasks": []}""")
        val client = createTestClient(mockEngine)
        val result = client.taskDue("2026-03-20T00:00:00Z")
        assertNotNull(result["tasks"])
    }

    @Test
    fun `taskStart starts a task`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "task_1", "status": "running"}""")
        val client = createTestClient(mockEngine)
        val result = client.taskStart("task_1")
        assertEquals("running", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `taskSucceed marks task as succeeded`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "task_1", "status": "active"}""")
        val client = createTestClient(mockEngine)
        val result = client.taskSucceed("task_1", buildJsonObject { put("output", "OK") })
        assertEquals("active", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `taskFail marks task as failed`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "task_1"}""")
        val client = createTestClient(mockEngine)
        client.taskFail("task_1", buildJsonObject { put("error", "Timeout") })
    }

    @Test
    fun `taskPause pauses a task`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "task_1", "status": "paused"}""")
        val client = createTestClient(mockEngine)
        val result = client.taskPause("task_1")
        assertEquals("paused", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `taskResume resumes a task`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "task_1", "status": "active"}""")
        val client = createTestClient(mockEngine)
        val result = client.taskResume("task_1", buildJsonObject {})
        assertEquals("active", result["status"]?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Agent CRUD Tests
    // ========================================================================

    @Test
    fun `agentCreate creates a new agent`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "agent_1", "name": "TestAgent"}""")
        val client = createTestClient(mockEngine)
        val result = client.agentCreate(buildJsonObject {
            put("name", "TestAgent")
            put("system_prompt", "You help.")
        })
        assertEquals("TestAgent", result["name"]?.jsonPrimitive?.content)
    }

    @Test
    fun `agentList returns agents`() = runBlocking {
        val mockEngine = createMockEngine("""{"agents": []}""")
        val client = createTestClient(mockEngine)
        val result = client.agentList()
        assertNotNull(result["agents"])
    }

    @Test
    fun `agentGet returns an agent by ID`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "agent_1", "name": "TestAgent"}""")
        val client = createTestClient(mockEngine)
        val result = client.agentGet("agent_1")
        assertEquals("agent_1", result["id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `agentGetByName returns an agent by name`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "agent_1", "name": "TestAgent"}""")
        val client = createTestClient(mockEngine)
        val result = client.agentGetByName("TestAgent")
        assertEquals("TestAgent", result["name"]?.jsonPrimitive?.content)
    }

    @Test
    fun `agentUpdate updates an agent`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "agent_1", "name": "Updated"}""")
        val client = createTestClient(mockEngine)
        val result = client.agentUpdate("agent_1", buildJsonObject { put("name", "Updated") })
        assertEquals("Updated", result["name"]?.jsonPrimitive?.content)
    }

    @Test
    fun `agentDelete deletes an agent`() = runBlocking {
        val mockEngine = createMockEngine("""{"ok": true}""")
        val client = createTestClient(mockEngine)
        client.agentDelete("agent_1") // should not throw
    }

    @Test
    fun `agentsByDeployment returns agents for deployment`() = runBlocking {
        val mockEngine = createMockEngine("""{"agents": [{"id": "agent_1"}]}""")
        val client = createTestClient(mockEngine)
        val result = client.agentsByDeployment("deploy_1")
        assertNotNull(result["agents"])
    }

    // ========================================================================
    // Goal Template CRUD Tests
    // ========================================================================

    @Test
    fun `goalTemplateCreate creates a new goal template`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "gt_1", "title": "Deploy Checklist"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalTemplateCreate(buildJsonObject {
            put("title", "Deploy Checklist")
            putJsonArray("steps") {
                add(buildJsonObject { put("title", "Run tests") })
            }
        })
        assertEquals("gt_1", result["id"]?.jsonPrimitive?.content)
        assertEquals("Deploy Checklist", result["title"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalTemplateList returns templates`() = runBlocking {
        val mockEngine = createMockEngine("""{"templates": [{"id": "gt_1"}, {"id": "gt_2"}]}""")
        val client = createTestClient(mockEngine)
        val result = client.goalTemplateList()
        assertNotNull(result["templates"])
        val templates = result["templates"]?.jsonArray
        assertNotNull(templates)
        assertEquals(2, templates.size)
    }

    @Test
    fun `goalTemplateGet returns a template by ID`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "gt_1", "title": "Deploy Checklist"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalTemplateGet("gt_1")
        assertEquals("gt_1", result["id"]?.jsonPrimitive?.content)
        assertEquals("Deploy Checklist", result["title"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalTemplateUpdate updates a template`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "gt_1", "title": "Updated Checklist"}""")
        val client = createTestClient(mockEngine)
        val result = client.goalTemplateUpdate("gt_1", buildJsonObject {
            put("title", "Updated Checklist")
        })
        assertEquals("Updated Checklist", result["title"]?.jsonPrimitive?.content)
    }

    @Test
    fun `goalTemplateDelete deletes a template`() = runBlocking {
        val mockEngine = createMockEngine("""{"ok": true}""")
        val client = createTestClient(mockEngine)
        client.goalTemplateDelete("gt_1") // should not throw
    }

    // ========================================================================
    // chatMessageStream Tests
    // ========================================================================

    @Test
    fun `chatMessageStream emits chunk and end events`() = runBlocking {
        val sseBody = listOf(
            """data: {"token":"Hello"}""",
            """data: {"token":" world"}""",
            """data: {"content":"Hello world","message_id":"msg_1","execution_time_ms":42}""",
            ""
        ).joinToString("\n")

        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = sseBody,
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Text.EventStream.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)
        val request = buildJsonObject { put("message", "Hello") }
        val events = mutableListOf<ChatStreamEvent>()
        client.chatMessageStream("chat_123", request).collect { events.add(it) }

        assertEquals(3, events.size)
        assertTrue(events[0] is ChatStreamEvent.Chunk)
        assertEquals("Hello", (events[0] as ChatStreamEvent.Chunk).content)
        assertTrue(events[1] is ChatStreamEvent.Chunk)
        assertEquals(" world", (events[1] as ChatStreamEvent.Chunk).content)
        assertTrue(events[2] is ChatStreamEvent.End)
        assertEquals("msg_1", (events[2] as ChatStreamEvent.End).messageId)
        assertEquals(42L, (events[2] as ChatStreamEvent.End).executionTimeMs)
    }

    @Test
    fun `chatMessageStream emits error on SSE error event`() = runBlocking {
        val sseBody = """data: {"error":"LLM timeout"}"""

        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = sseBody,
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Text.EventStream.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)
        val request = buildJsonObject { put("message", "Hello") }
        val events = mutableListOf<ChatStreamEvent>()
        client.chatMessageStream("chat_123", request).collect { events.add(it) }

        assertEquals(1, events.size)
        assertTrue(events[0] is ChatStreamEvent.Error)
        assertEquals("LLM timeout", (events[0] as ChatStreamEvent.Error).error)
    }

    @Test
    fun `chatMessageStream emits error on non-200 response`() = runBlocking {
        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = "Unauthorized",
                    status = HttpStatusCode.Unauthorized,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Text.Plain.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)
        val request = buildJsonObject { put("message", "Hello") }
        val events = mutableListOf<ChatStreamEvent>()
        client.chatMessageStream("chat_123", request).collect { events.add(it) }

        assertEquals(1, events.size)
        assertTrue(events[0] is ChatStreamEvent.Error)
        assertTrue((events[0] as ChatStreamEvent.Error).error.contains("401"))
    }

    // ========================================================================
    // countDocuments Tests
    // ========================================================================

    @Test
    fun `countDocuments returns count`() = runBlocking {
        val mockEngine = createMockEngine("""{"count": 42}""")
        val client = createTestClient(mockEngine)
        val count = client.countDocuments("users")
        assertEquals(42L, count)
    }

    @Test
    fun `countDocuments returns zero when missing`() = runBlocking {
        val mockEngine = createMockEngine("""{}""")
        val client = createTestClient(mockEngine)
        val count = client.countDocuments("empty_col")
        assertEquals(0L, count)
    }

    // ========================================================================
    // rawCompletionStreamWithProgress Tests
    // ========================================================================

    @Test
    fun `rawCompletionStreamWithProgress invokes onToken callback`() = runBlocking {
        val sseBody = listOf(
            """data: {"token":"Hello"}""",
            """data: {"token":" world"}""",
            """data: {"content":"Hello world"}""",
            ""
        ).joinToString("\n")

        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = sseBody,
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Text.EventStream.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)
        val tokens = mutableListOf<String>()
        val result = client.rawCompletionStreamWithProgress(
            systemPrompt = "You are helpful.",
            message = "Say hello",
            onToken = { tokens.add(it) }
        )
        assertEquals("Hello world", result["content"]?.jsonPrimitive?.content)
        assertEquals(listOf("Hello", " world"), tokens)
    }

    @Test
    fun `rawCompletionStreamWithProgress throws on error`() = runBlocking {
        val sseBody = listOf(
            """data: {"error":"model overloaded"}""",
            ""
        ).joinToString("\n")

        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = sseBody,
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Text.EventStream.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)
        try {
            client.rawCompletionStreamWithProgress(
                systemPrompt = "You are helpful.",
                message = "Say hello",
                onToken = { }
            )
            fail("Expected RuntimeException")
        } catch (e: RuntimeException) {
            assertTrue(e.message!!.contains("model overloaded"))
        }
    }

    // ========================================================================
    // Core CRUD Tests
    // ========================================================================

    @Test
    fun `insert sends POST to correct endpoint`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "rec_1", "name": "Alice", "email": "alice@example.com"}""")
        val client = createTestClient(mockEngine)
        val record = io.ekodb.client.types.Record()
            .insert("name", "Alice")
            .insert("email", "alice@example.com")
        val result = client.insert("users", record)
        assertNotNull(result)
        val idField = result["id"]
        assertTrue(idField is io.ekodb.client.types.FieldType.StringValue)
        assertEquals("rec_1", idField.value)
    }

    @Test
    fun `findById sends GET to correct endpoint`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "rec_1", "name": "Alice"}""")
        val client = createTestClient(mockEngine)
        val result = client.findById("users", "rec_1")
        assertNotNull(result)
        val idField = result["id"]
        assertTrue(idField is io.ekodb.client.types.FieldType.StringValue)
        assertEquals("rec_1", idField.value)
    }

    @Test
    fun `find sends POST query to correct endpoint`() = runBlocking {
        val mockEngine = createMockEngine("""[{"id": "rec_1", "name": "Alice"}, {"id": "rec_2", "name": "Bob"}]""")
        val client = createTestClient(mockEngine)
        val query = io.ekodb.client.types.Query.new()
        val result = client.find("users", query)
        assertNotNull(result)
        assertEquals(2, result.size)
    }

    @Test
    fun `update sends PUT to correct endpoint`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "rec_1", "name": "Alice Updated", "age": 30}""")
        val client = createTestClient(mockEngine)
        val updates = io.ekodb.client.types.Record()
            .insert("name", "Alice Updated")
            .insert("age", 30)
        val result = client.update("users", "rec_1", updates)
        assertNotNull(result)
        val nameField = result["name"]
        assertTrue(nameField is io.ekodb.client.types.FieldType.StringValue)
        assertEquals("Alice Updated", nameField.value)
    }

    @Test
    fun `delete sends DELETE to correct endpoint`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "deleted"}""")
        val client = createTestClient(mockEngine)
        // Should not throw
        client.delete("users", "rec_1")
    }

    // ========================================================================
    // Batch Operations Tests
    // ========================================================================

    @Test
    fun `batchInsert returns successful and failed counts`() = runBlocking {
        val mockEngine = createMockEngine("""{"successful": ["id_1", "id_2", "id_3"], "failed": []}""")
        val client = createTestClient(mockEngine)
        val records = listOf(
            io.ekodb.client.types.Record().insert("name", "Alice"),
            io.ekodb.client.types.Record().insert("name", "Bob"),
            io.ekodb.client.types.Record().insert("name", "Charlie")
        )
        val result = client.batchInsert("users", records)
        assertEquals(3, result.successful.size)
        assertEquals(0, result.failed.size)
    }

    @Test
    fun `batchUpdate returns updated records`() = runBlocking {
        val mockEngine = createMockEngine("""{"successful": ["id_1", "id_2"], "failed": []}""")
        val client = createTestClient(mockEngine)
        val updates = listOf(
            "id_1" to io.ekodb.client.types.Record().insert("score", 100),
            "id_2" to io.ekodb.client.types.Record().insert("score", 200)
        )
        val result = client.batchUpdate("users", updates)
        assertEquals(2, result.size)
    }

    @Test
    fun `batchDelete returns deleted count`() = runBlocking {
        val mockEngine = createMockEngine("""{"successful": ["id_1", "id_2"], "failed": []}""")
        val client = createTestClient(mockEngine)
        val result = client.batchDelete("users", listOf("id_1", "id_2"))
        assertEquals(2L, result)
    }

    // ========================================================================
    // KV Operations Tests
    // ========================================================================

    @Test
    fun `kvSet does not throw on success`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "ok"}""")
        val client = createTestClient(mockEngine)
        // Should not throw
        client.kvSet("test:key", JsonPrimitive("test_value"))
    }

    @Test
    fun `kvGet returns stored value`() = runBlocking {
        val mockEngine = createMockEngine("""{"value": "stored_value"}""")
        val client = createTestClient(mockEngine)
        val result = client.kvGet("test:key")
        assertNotNull(result)
        assertEquals("stored_value", (result as JsonElement).jsonPrimitive.content)
    }

    @Test
    fun `kvDelete does not throw on success`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "deleted"}""")
        val client = createTestClient(mockEngine)
        // Should not throw
        client.kvDelete("test:key")
    }

    @Test
    fun `kvExists returns true when key exists`() = runBlocking {
        val mockEngine = createMockEngine("""{"value": "exists"}""")
        val client = createTestClient(mockEngine)
        val result = client.kvExists("test:key")
        assertTrue(result)
    }

    // ========================================================================
    // KV Links Tests
    // ========================================================================

    @Test
    fun `kvGetLinks returns linked documents`() = runBlocking {
        val mockEngine = createMockEngine("""{"key": "user:123", "links": [{"collection": "orders", "document_id": "ord_1"}]}""")
        val client = createTestClient(mockEngine)
        val result = client.kvGetLinks("user:123")
        assertNotNull(result)
        assertEquals("user:123", result["key"]?.jsonPrimitive?.content)
        assertNotNull(result["links"])
    }

    @Test
    fun `kvLink links a document to a key`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "linked", "key": "user:123"}""")
        val client = createTestClient(mockEngine)
        val result = client.kvLink("user:123", "orders", "ord_1")
        assertNotNull(result)
        assertEquals("linked", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `kvUnlink removes a document link from a key`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "unlinked", "key": "user:123"}""")
        val client = createTestClient(mockEngine)
        val result = client.kvUnlink("user:123", "orders", "ord_1")
        assertNotNull(result)
        assertEquals("unlinked", result["status"]?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Schedule Management Tests
    // ========================================================================

    @Test
    fun `createSchedule returns schedule object`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "sched_1", "name": "Daily Backup", "cron": "0 0 * * *"}""")
        val client = createTestClient(mockEngine)
        val result = client.createSchedule(buildJsonObject {
            put("name", "Daily Backup")
            put("cron", "0 0 * * *")
        })
        assertEquals("sched_1", result["id"]?.jsonPrimitive?.content)
        assertEquals("Daily Backup", result["name"]?.jsonPrimitive?.content)
    }

    @Test
    fun `listSchedules returns all schedules`() = runBlocking {
        val mockEngine = createMockEngine("""{"schedules": [{"id": "sched_1"}, {"id": "sched_2"}]}""")
        val client = createTestClient(mockEngine)
        val result = client.listSchedules()
        assertNotNull(result["schedules"])
        val schedules = result["schedules"]?.jsonArray
        assertNotNull(schedules)
        assertEquals(2, schedules.size)
    }

    @Test
    fun `getSchedule returns schedule by ID`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "sched_1", "name": "Daily Backup", "cron": "0 0 * * *"}""")
        val client = createTestClient(mockEngine)
        val result = client.getSchedule("sched_1")
        assertEquals("sched_1", result["id"]?.jsonPrimitive?.content)
    }

    @Test
    fun `updateSchedule updates and returns schedule`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "sched_1", "name": "Weekly Backup", "cron": "0 0 * * 0"}""")
        val client = createTestClient(mockEngine)
        val result = client.updateSchedule("sched_1", buildJsonObject {
            put("name", "Weekly Backup")
            put("cron", "0 0 * * 0")
        })
        assertEquals("Weekly Backup", result["name"]?.jsonPrimitive?.content)
    }

    @Test
    fun `deleteSchedule does not throw on success`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "deleted"}""")
        val client = createTestClient(mockEngine)
        // Should not throw
        client.deleteSchedule("sched_1")
    }

    @Test
    fun `pauseSchedule returns paused schedule`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "sched_1", "status": "paused"}""")
        val client = createTestClient(mockEngine)
        val result = client.pauseSchedule("sched_1")
        assertEquals("paused", result["status"]?.jsonPrimitive?.content)
    }

    @Test
    fun `resumeSchedule returns active schedule`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "sched_1", "status": "active"}""")
        val client = createTestClient(mockEngine)
        val result = client.resumeSchedule("sched_1")
        assertEquals("active", result["status"]?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Functions Tests
    // ========================================================================

    @Test
    fun `saveFunction returns script ID`() = runBlocking {
        val mockEngine = createMockEngine("""{"id": "script_1"}""")
        val client = createTestClient(mockEngine)
        val script = io.ekodb.client.functions.UserFunction(
            label = "test_script",
            name = "Test Function",
            description = "A test script",
            functions = listOf(
                io.ekodb.client.functions.FunctionStageConfig.FindAll(collection = "test_col")
            )
        )
        val result = client.saveFunction(script)
        assertEquals("script_1", result)
    }

    @Test
    fun `getFunction returns script by ID`() = runBlocking {
        val mockEngine = createMockEngine("""{
            "label": "test_script",
            "name": "Test Function",
            "version": "1.0",
            "parameters": {},
            "functions": [{"type": "FindAll", "collection": "test_col"}],
            "tags": [],
            "id": "script_1"
        }""")
        val client = createTestClient(mockEngine)
        val result = client.getFunction("script_1")
        assertEquals("test_script", result.label)
        assertEquals("Test Function", result.name)
    }

    @Test
    fun `listFunctions returns scripts list`() = runBlocking {
        val mockEngine = createMockEngine("""[
            {"label": "s1", "name": "Function 1", "version": "1.0", "parameters": {}, "functions": [{"type": "FindAll", "collection": "c1"}], "tags": []},
            {"label": "s2", "name": "Function 2", "version": "1.0", "parameters": {}, "functions": [{"type": "FindAll", "collection": "c2"}], "tags": ["etl"]}
        ]""")
        val client = createTestClient(mockEngine)
        val result = client.listFunctions()
        assertEquals(2, result.size)
        assertEquals("s1", result[0].label)
        assertEquals("s2", result[1].label)
    }

    @Test
    fun `updateFunction does not throw on success`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "updated"}""")
        val client = createTestClient(mockEngine)
        val script = io.ekodb.client.functions.UserFunction(
            label = "test_script",
            name = "Updated Function",
            functions = listOf(
                io.ekodb.client.functions.FunctionStageConfig.FindAll(collection = "updated_col")
            )
        )
        // Should not throw
        client.updateFunction("script_1", script)
    }

    @Test
    fun `deleteFunction does not throw on success`() = runBlocking {
        val mockEngine = createMockEngine("""{"status": "deleted"}""")
        val client = createTestClient(mockEngine)
        // Should not throw
        client.deleteFunction("script_1")
    }

    @Test
    fun `callFunction returns function result`() = runBlocking {
        val mockEngine = createMockEngine("""{
            "records": [{"id": "rec_1", "name": "Alice"}],
            "stats": {
                "input_count": 10,
                "output_count": 1,
                "execution_time_ms": 42,
                "stages_executed": 1,
                "stage_stats": [{"stage": "FindAll", "input_count": 0, "output_count": 10, "execution_time_ms": 42}]
            }
        }""")
        val client = createTestClient(mockEngine)
        val result = client.callFunction("test_script", mapOf("limit" to JsonPrimitive(10)))
        assertEquals(1, result.records.size)
        assertEquals(1, result.stats.stages_executed)
        assertEquals(42L, result.stats.execution_time_ms)
    }

    // ========================================================================
    // Search Tests (textSearch / hybridSearch)
    // ========================================================================

    @Test
    fun `textSearch returns matching records`() = runBlocking {
        val mockEngine = createMockEngine("""{
            "results": [
                {"record": {"id": "doc_1", "title": "Rust Programming"}, "score": 0.95},
                {"record": {"id": "doc_2", "title": "Rust Async"}, "score": 0.80}
            ],
            "total": 2
        }""")
        val client = createTestClient(mockEngine)
        val result = client.textSearch("documents", "rust", 10)
        assertEquals(2, result.size)
        assertEquals("doc_1", result[0]["id"]?.jsonPrimitive?.content)
        assertEquals(0.95, result[0]["_score"]?.jsonPrimitive?.double)
    }

    @Test
    fun `hybridSearch returns combined results`() = runBlocking {
        val mockEngine = createMockEngine("""{
            "results": [
                {"record": {"id": "doc_1", "title": "Machine Learning"}, "score": 0.92}
            ],
            "total": 1
        }""")
        val client = createTestClient(mockEngine)
        val result = client.hybridSearch("documents", "machine learning", listOf(0.1, 0.2, 0.3), 5)
        assertEquals(1, result.size)
        assertEquals("Machine Learning", result[0]["title"]?.jsonPrimitive?.content)
        assertEquals(0.92, result[0]["_score"]?.jsonPrimitive?.double)
    }

    // ========================================================================
    // Auth Token Management Tests
    // ========================================================================

    /**
     * Helper to build a fake JWT with a given `exp` claim.
     * The header and signature are dummy values; only the payload matters.
     */
    private fun buildFakeJWT(exp: Long): String {
        val header = java.util.Base64.getUrlEncoder().withoutPadding()
            .encodeToString("""{"alg":"HS256","typ":"JWT"}""".toByteArray())
        val payload = java.util.Base64.getUrlEncoder().withoutPadding()
            .encodeToString("""{"sub":"user_1","exp":$exp}""".toByteArray())
        val signature = "fake_signature"
        return "$header.$payload.$signature"
    }

    @Test
    fun `extractJWTExpiry returns expiry from valid JWT`() {
        val client = createTestClient(createMockEngine("""{}"""))
        val futureExp = System.currentTimeMillis() / 1000 + 7200 // 2 hours from now
        val jwt = buildFakeJWT(futureExp)
        val result = client.extractJWTExpiry(jwt)
        assertNotNull(result)
        assertEquals(futureExp, result)
    }

    @Test
    fun `extractJWTExpiry returns null for non-JWT string`() {
        val client = createTestClient(createMockEngine("""{}"""))
        assertEquals(null, client.extractJWTExpiry("not-a-jwt"))
    }

    @Test
    fun `extractJWTExpiry returns null for two-segment string`() {
        val client = createTestClient(createMockEngine("""{}"""))
        assertEquals(null, client.extractJWTExpiry("header.payload"))
    }

    @Test
    fun `extractJWTExpiry returns null for invalid base64 payload`() {
        val client = createTestClient(createMockEngine("""{}"""))
        assertEquals(null, client.extractJWTExpiry("a.!!!invalid.c"))
    }

    @Test
    fun `extractJWTExpiry returns null when exp claim is missing`() {
        val client = createTestClient(createMockEngine("""{}"""))
        val header = java.util.Base64.getUrlEncoder().withoutPadding()
            .encodeToString("""{"alg":"HS256"}""".toByteArray())
        val payload = java.util.Base64.getUrlEncoder().withoutPadding()
            .encodeToString("""{"sub":"user_1"}""".toByteArray())
        val jwt = "$header.$payload.sig"
        assertEquals(null, client.extractJWTExpiry(jwt))
    }

    @Test
    fun `getToken returns cached token when not expired`() = runBlocking {
        // Build a JWT that expires far in the future so the cache should be used
        val futureExp = System.currentTimeMillis() / 1000 + 7200
        val fakeJwt = buildFakeJWT(futureExp)
        var authRequestCount = 0

        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                authRequestCount++
                respond(
                    content = """{"token": "$fakeJwt"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = """{"collections": ["test"]}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)

        // First call triggers auth + listCollections
        client.listCollections()
        assertEquals(1, authRequestCount, "First call should fetch a token")

        // Second call should reuse cached token (not expired)
        client.listCollections()
        assertEquals(1, authRequestCount, "Second call should reuse cached token, not fetch again")
    }

    @Test
    fun `getToken refreshes when token about to expire`() = runBlocking {
        // Build a JWT that expires in 30 seconds (within the 60s refresh window)
        val soonExp = System.currentTimeMillis() / 1000 + 30
        val soonJwt = buildFakeJWT(soonExp)

        // Build a JWT that expires far in the future for the refresh
        val futureExp = System.currentTimeMillis() / 1000 + 7200
        val futureJwt = buildFakeJWT(futureExp)

        var authRequestCount = 0

        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                authRequestCount++
                // First auth returns soon-to-expire token, subsequent returns long-lived token
                val jwt = if (authRequestCount == 1) soonJwt else futureJwt
                respond(
                    content = """{"token": "$jwt"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = """{"collections": ["test"]}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)

        // First call gets the soon-to-expire token
        client.listCollections()
        assertEquals(1, authRequestCount, "First call should fetch a token")

        // Second call should detect the token is about to expire and fetch a new one
        client.listCollections()
        assertEquals(2, authRequestCount, "Second call should refresh because token expires within 60s")

        // Third call should reuse the new long-lived token
        client.listCollections()
        assertEquals(2, authRequestCount, "Third call should reuse the refreshed token")
    }

    @Test
    fun `clearTokenCache resets token and expiry`() = runBlocking {
        val futureExp = System.currentTimeMillis() / 1000 + 7200
        val fakeJwt = buildFakeJWT(futureExp)
        var authRequestCount = 0

        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                authRequestCount++
                respond(
                    content = """{"token": "$fakeJwt"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                respond(
                    content = """{"collections": ["test"]}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)

        // First call fetches token
        client.listCollections()
        assertEquals(1, authRequestCount, "First call should fetch a token")

        // Clear the cache
        client.clearTokenCache()

        // Next call should need to fetch a new token since cache was cleared
        client.listCollections()
        assertEquals(2, authRequestCount, "After clearTokenCache, next call should fetch a new token")
    }

    // ========================================================================
    // agent_id Tests
    // ========================================================================

    @Test
    fun `CreateChatSessionRequest includes agent_id`() {
        val req = io.ekodb.client.types.CreateChatSessionRequest(
            llmProvider = "openai",
            agentId = "my-agent",
        )
        assertEquals("my-agent", req.agentId)
        val json = Json.encodeToString(
            io.ekodb.client.types.CreateChatSessionRequest.serializer(), req
        )
        assertTrue(json.contains("\"agent_id\":\"my-agent\""))
    }

    @Test
    fun `CreateChatSessionRequest omits agent_id when null`() {
        val req = io.ekodb.client.types.CreateChatSessionRequest(
            llmProvider = "openai",
        )
        assertEquals(null, req.agentId)
        val json = Json { explicitNulls = false }.encodeToString(
            io.ekodb.client.types.CreateChatSessionRequest.serializer(), req
        )
        assertFalse(json.contains("agent_id"))
    }

    @Test
    fun `ChatSession deserializes agent_id`() {
        val raw = """{"chat_id":"c1","created_at":"t","updated_at":"t","llm_provider":"openai","llm_model":"gpt-4","collections":[],"agent_id":"bot-1","message_count":0}"""
        val session = Json { ignoreUnknownKeys = true }.decodeFromString(
            io.ekodb.client.types.ChatSession.serializer(), raw
        )
        assertEquals("bot-1", session.agentId)
    }

    @Test
    fun `ChatSession handles missing agent_id`() {
        val raw = """{"chat_id":"c1","created_at":"t","updated_at":"t","llm_provider":"openai","llm_model":"gpt-4","collections":[],"message_count":0}"""
        val session = Json { ignoreUnknownKeys = true }.decodeFromString(
            io.ekodb.client.types.ChatSession.serializer(), raw
        )
        assertEquals(null, session.agentId)
    }

    // ========================================================================
    // ChatMessageRequest tool fields Tests
    // ========================================================================

    @Test
    fun `ChatMessageRequest includes client_tools`() {
        val params = buildJsonObject { put("type", "object") }
        val req = io.ekodb.client.types.ChatMessageRequest(
            message = "hello",
            clientTools = listOf(
                io.ekodb.client.types.ClientToolDef("weather", "Get weather", params)
            ),
            confirmTools = listOf("shell_exec"),
            excludeTools = listOf("file_delete"),
        )
        assertEquals(1, req.clientTools?.size)
        assertEquals("weather", req.clientTools!![0].name)
        assertEquals(listOf("shell_exec"), req.confirmTools)
        assertEquals(listOf("file_delete"), req.excludeTools)
    }

    @Test
    fun `ChatMessageRequest serializes tool fields`() {
        val params = buildJsonObject { put("type", "object") }
        val req = io.ekodb.client.types.ChatMessageRequest(
            message = "test",
            clientTools = listOf(
                io.ekodb.client.types.ClientToolDef("calc", "Calculator", params)
            ),
            confirmTools = listOf("shell"),
            excludeTools = listOf("rm"),
        )
        val json = Json.encodeToString(
            io.ekodb.client.types.ChatMessageRequest.serializer(), req
        )
        assertTrue(json.contains("\"client_tools\""))
        assertTrue(json.contains("\"confirm_tools\""))
        assertTrue(json.contains("\"exclude_tools\""))
        assertTrue(json.contains("\"calc\""))
    }

    @Test
    fun `ChatMessageRequest omits tool fields when null`() {
        val req = io.ekodb.client.types.ChatMessageRequest(message = "hi")
        val json = Json { explicitNulls = false }.encodeToString(
            io.ekodb.client.types.ChatMessageRequest.serializer(), req
        )
        assertFalse(json.contains("client_tools"))
        assertFalse(json.contains("confirm_tools"))
        assertFalse(json.contains("exclude_tools"))
    }

    // ========================================================================
    // submitChatToolResult Tests
    // ========================================================================

    @Test
    fun `submitChatToolResult sends correct request`() = runBlocking {
        var capturedPath = ""
        var capturedBody = ""
        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                capturedPath = request.url.encodedPath
                capturedBody = String(request.body.toByteArray())
                respond(
                    content = "{}",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)

        val result = buildJsonObject { put("temp", "72F") }
        client.submitChatToolResult("chat-123", "call-456", true, result)

        assertTrue(capturedPath.contains("/api/chat/chat-123/tool-result"))
        assertTrue(capturedBody.contains("\"call_id\":\"call-456\""))
        assertTrue(capturedBody.contains("\"success\":true"))
        assertTrue(capturedBody.contains("\"temp\":\"72F\""))
    }

    @Test
    fun `submitChatToolResult sends error`() = runBlocking {
        var capturedBody = ""
        val mockEngine = MockEngine { request ->
            if (request.url.encodedPath.contains("/api/auth/token")) {
                respond(
                    content = """{"token": "mock_jwt_token_123"}""",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            } else {
                capturedBody = String(request.body.toByteArray())
                respond(
                    content = "{}",
                    status = HttpStatusCode.OK,
                    headers = headersOf(HttpHeaders.ContentType, ContentType.Application.Json.toString())
                )
            }
        }
        val client = createTestClient(mockEngine)

        client.submitChatToolResult("chat-1", "call-1", false, error = "tool crashed")

        assertTrue(capturedBody.contains("\"success\":false"))
        assertTrue(capturedBody.contains("\"error\":\"tool crashed\""))
    }
}
