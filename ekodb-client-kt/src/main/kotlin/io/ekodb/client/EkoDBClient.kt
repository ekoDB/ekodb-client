package io.ekodb.client

import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Query
import io.ekodb.client.types.Record
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.*
import io.ktor.client.plugins.compression.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.logging.*
import io.ktor.client.plugins.websocket.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.delay
import kotlinx.serialization.json.*
import kotlin.time.Duration.Companion.seconds

/**
 * ekoDB Client for Kotlin
 * 
 * Official Kotlin client library for ekoDB - A high-performance database with
 * intelligent caching, real-time capabilities, and automatic optimization.
 */
class EkoDBClient private constructor(
    private val baseUrl: String,
    private val apiKey: String,
    private val timeout: Long = 30,
    private val maxRetries: Int = 3
) {
    private val httpClient = HttpClient(CIO) {
        install(ContentNegotiation) {
            json(Json {
                prettyPrint = true
                isLenient = true
                ignoreUnknownKeys = true
            })
        }
        
        // Enable compression with proper content negotiation
        // Client sends Accept-Encoding: gzip, server compresses only if it accepts
        install(ContentEncoding) {
            gzip()
            deflate()
        }
        
        install(HttpTimeout) {
            requestTimeoutMillis = timeout * 1000
            connectTimeoutMillis = timeout * 1000
            socketTimeoutMillis = timeout * 1000
        }
        
        install(Logging) {
            logger = Logger.DEFAULT
            level = LogLevel.INFO
        }
        
        install(WebSockets)
    }
    
    private var authToken: String? = null
    
    /**
     * Get authentication token (exchanges API key for JWT)
     */
    private suspend fun getToken(): String {
        // Return cached token if available
        authToken?.let { return it }
        
        // Exchange API key for JWT token
        val response = httpClient.post("$baseUrl/api/auth/token") {
            contentType(ContentType.Application.Json)
            setBody(buildJsonObject {
                put("api_key", apiKey)
            })
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Authentication failed: $errorBody")
        }
        
        val tokenData = response.body<JsonObject>()
        val token = tokenData["token"]?.jsonPrimitive?.content
            ?: throw Exception("No token in authentication response")
        
        authToken = token
        return token
    }
    
    /**
     * Refresh the authentication token
     */
    suspend fun refreshToken(): String {
        clearTokenCache()
        return getToken()
    }
    
    /**
     * Clear the cached authentication token
     */
    fun clearTokenCache() {
        authToken = null
    }
    
    /**
     * Insert a record into a collection
     */
    suspend fun insert(collection: String, record: Record): Record {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/insert/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(record)
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Insert failed with status ${response.status}: $errorBody")
        }
        
        return response.body()
    }
    
    /**
     * Insert a record with TTL
     */
    suspend fun insertWithTtl(collection: String, record: Record, ttl: String): Record {
        return insert(collection, record.withTtl(ttl))
    }
    
    /**
     * Find a record by ID
     */
    suspend fun findById(collection: String, id: String): Record {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/find/$collection/$id") {
                header("Authorization", "Bearer $token")
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Find by ID failed with status ${response.status}: $errorBody")
        }
        
        return response.body()
    }
    
    /**
     * Find records with a query
     */
    suspend fun find(collection: String, query: Query): List<Record> {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/find/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(query)
            }
        }
        return response.body()
    }
    
    /**
     * Find all records in a collection
     */
    suspend fun findAll(collection: String): List<Record> {
        return find(collection, Query.new())
    }
    
    /**
     * Update a record
     */
    suspend fun update(collection: String, id: String, updates: Record): Record {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.put("$baseUrl/api/update/$collection/$id") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(updates)
            }
        }
        return response.body()
    }
    
    /**
     * Delete a record
     */
    suspend fun delete(collection: String, id: String) {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.delete("$baseUrl/api/delete/$collection/$id") {
                header("Authorization", "Bearer $token")
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Delete failed with status ${response.status}: $errorBody")
        }
    }
    
    /**
     * Batch insert records
     */
    suspend fun batchInsert(collection: String, records: List<Record>): BatchResult {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/batch/insert/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(buildJsonObject {
                    put("inserts", JsonArray(records.map { record ->
                        buildJsonObject {
                            put("data", Json.encodeToJsonElement(record))
                        }
                    }))
                })
            }
        }
        
        val result = response.body<JsonObject>()
        val successful = result["successful"]?.jsonArray ?: JsonArray(emptyList())
        val failed = result["failed"]?.jsonArray ?: JsonArray(emptyList())
        
        return BatchResult(
            successful = successful.map { idElement ->
                val id = idElement.jsonPrimitive.content
                Record().apply { fields["id"] = FieldType.string(id) }
            },
            failed = failed.map { errorElement ->
                BatchError(0, errorElement.toString())
            }
        )
    }
    
    /**
     * Batch update records
     */
    suspend fun batchUpdate(collection: String, updates: List<Pair<String, Record>>): List<Record> {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.put("$baseUrl/api/batch/update/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(buildJsonObject {
                    put("updates", JsonArray(updates.map { (id, data) ->
                        buildJsonObject {
                            put("id", id)
                            put("data", Json.encodeToJsonElement(data))
                        }
                    }))
                })
            }
        }
        
        val result = response.body<JsonObject>()
        val successful = result["successful"]?.jsonArray ?: JsonArray(emptyList())
        return successful.map { idElement ->
            val id = idElement.jsonPrimitive.content
            Record().apply { fields["id"] = FieldType.string(id) }
        }
    }
    
    /**
     * Batch delete records by IDs
     */
    suspend fun batchDelete(collection: String, ids: List<String>): Long {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.delete("$baseUrl/api/batch/delete/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(buildJsonObject {
                    put("deletes", JsonArray(ids.map { id ->
                        buildJsonObject {
                            put("id", id)
                        }
                    }))
                })
            }
        }
        
        val result = response.body<JsonObject>()
        val successful = result["successful"]?.jsonArray ?: JsonArray(emptyList())
        return successful.size.toLong()
    }
    
    /**
     * Count documents in a collection
     */
    suspend fun count(collection: String): Long {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/collections/$collection") {
                header("Authorization", "Bearer $token")
            }
        }
        // this must return the collection but getting a collection by collection name returns all the stats, we need to return the amount of records in the collection
        val json = response.body<JsonObject>()
        return json["count"].toString().toLong()
    }
    
    /**
     * List all collections
     */
    suspend fun listCollections(): List<String> {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/collections") {
                header("Authorization", "Bearer $token")
            }
        }
        val result = response.body<JsonObject>()
        val collections = result["collections"]?.jsonArray ?: JsonArray(emptyList())
        return collections.map { it.jsonPrimitive.content }
    }
    
    /**
     * Delete a collection
     */
    suspend fun deleteCollection(collection: String) {
        val token = getToken()
        executeWithRetry {
            httpClient.delete("$baseUrl/api/collections/$collection") {
                header("Authorization", "Bearer $token")
            }
        }
    }
    
    /**
     * Create a collection with schema
     */
    suspend fun createCollection(collection: String, schema: JsonObject) {
        val token = getToken()
        executeWithRetry {
            httpClient.post("$baseUrl/api/collections/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(schema)
            }
        }
    }
    
    /**
     * Get collection metadata
     */
    suspend fun getCollection(collection: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/collections/$collection") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }
    
    /**
     * Get collection schema
     */
    suspend fun getSchema(collection: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/schemas/$collection") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }
    
    /**
     * Check if a collection exists
     */
    suspend fun collectionExists(collection: String): Boolean {
        return try {
            val collections = listCollections()
            collections.contains(collection)
        } catch (e: Exception) {
            false
        }
    }
    
    /**
     * Search documents in a collection
     */
    suspend fun search(collection: String, searchQuery: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/search/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(searchQuery)
            }
        }
        return response.body()
    }
    
    /**
     * Key-Value: Set a key-value pair
     */
    suspend fun kvSet(key: String, value: JsonElement) {
        val token = getToken()
        executeWithRetry {
            httpClient.post("$baseUrl/api/kv/set/$key") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(buildJsonObject {
                    put("value", value)
                })
            }
        }
    }
    
    /**
     * Key-Value: Set with TTL
     */
    suspend fun kvSetWithTtl(key: String, value: JsonElement, ttl: String) {
        val token = getToken()
        executeWithRetry {
            httpClient.post("$baseUrl/api/kv/set/$key") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                parameter("ttl", ttl)
                setBody(buildJsonObject {
                    put("value", value)
                })
            }
        }
    }
    
    /**
     * Key-Value: Get a value
     */
    suspend fun kvGet(key: String): Any? {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/kv/get/$key") {
                header("Authorization", "Bearer $token")
            }
        }
        
        if (response.status == HttpStatusCode.NotFound) {
            return null
        }
        
        val result = response.body<JsonObject>()
        return result["value"]
    }
    
    /**
     * Key-Value: Delete a key
     */
    suspend fun kvDelete(key: String) {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.delete("$baseUrl/api/kv/delete/$key") {
                header("Authorization", "Bearer $token")
            }
        }
        
        if (!response.status.isSuccess() && response.status != HttpStatusCode.NotFound) {
            val errorBody = response.bodyAsText()
            throw Exception("KV delete failed with status ${response.status}: $errorBody")
        }
    }
    
    /**
     * Execute request with retry logic
     */
    private suspend fun executeWithRetry(
        block: suspend () -> HttpResponse
    ): HttpResponse {
        var lastException: Exception? = null
        
        repeat(maxRetries) { attempt ->
            try {
                val response = block()
                
                // Check for rate limiting
                if (response.status == HttpStatusCode.TooManyRequests) {
                    val retryAfter = response.headers["Retry-After"]?.toLongOrNull() ?: 1
                    delay(retryAfter.seconds)
                    return@repeat
                }
                
                // Check for server errors that should be retried
                if (response.status.value in 500..599) {
                    if (attempt < maxRetries - 1) {
                        delay((2.0.pow(attempt) * 1000).toLong())
                        return@repeat
                    }
                }
                
                return response
            } catch (e: Exception) {
                lastException = e
                if (attempt < maxRetries - 1) {
                    delay((2.0.pow(attempt) * 1000).toLong())
                }
            }
        }
        
        throw lastException ?: Exception("Request failed after $maxRetries attempts")
    }
    
    private fun Double.pow(n: Int): Double {
        var result = 1.0
        repeat(n) { result *= this }
        return result
    }
    
    // ========================================================================
    // Chat/AI Methods
    // ========================================================================
    
    /**
     * Get all available chat models
     */
    suspend fun getChatModels(): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/chat_models") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }
    
    /**
     * Get specific chat model info
     */
    suspend fun getChatModel(modelName: String): JsonArray {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/chat_models/$modelName") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }
    
    /**
     * Create a new chat session
     */
    suspend fun createChatSession(request: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/chat") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(request)
            }
        }
        return response.body()
    }
    
    /**
     * Get a chat session by ID
     */
    suspend fun getChatSession(chatId: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/chat/$chatId") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }
    
    /**
     * List all chat sessions
     */
    suspend fun listChatSessions(query: JsonObject = buildJsonObject {}): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/chat") {
                header("Authorization", "Bearer $token")
                query.forEach { (key, value) ->
                    parameter(key, value.toString())
                }
            }
        }
        return response.body()
    }
    
    /**
     * Update chat session metadata
     */
    suspend fun updateChatSession(chatId: String, request: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.put("$baseUrl/api/chat/$chatId") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(request)
            }
        }
        return response.body()
    }
    
    /**
     * Delete a chat session
     */
    suspend fun deleteChatSession(chatId: String) {
        val token = getToken()
        executeWithRetry {
            httpClient.delete("$baseUrl/api/chat/$chatId") {
                header("Authorization", "Bearer $token")
            }
        }
    }
    
    /**
     * Branch a chat session from an existing one
     */
    suspend fun branchChatSession(request: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/chat/branch") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(request)
            }
        }
        return response.body()
    }
    
    /**
     * Merge multiple chat sessions
     */
    suspend fun mergeChatSessions(request: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/chat/merge") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(request)
            }
        }
        return response.body()
    }
    
    /**
     * Send a message in an existing chat session
     */
    suspend fun chatMessage(chatId: String, request: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/chat/$chatId/messages") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(request)
            }
        }
        return response.body()
    }
    
    /**
     * Get messages from a chat session
     */
    suspend fun getChatSessionMessages(chatId: String, query: JsonObject = buildJsonObject {}): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/chat/$chatId/messages") {
                header("Authorization", "Bearer $token")
                query.forEach { (key, value) ->
                    parameter(key, value.toString())
                }
            }
        }
        return response.body()
    }
    
    /**
     * Get a specific message by ID
     */
    suspend fun getChatMessage(chatId: String, messageId: String): Record {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/chat/$chatId/messages/$messageId") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }
    
    /**
     * Update a chat message
     */
    suspend fun updateChatMessage(chatId: String, messageId: String, request: JsonObject) {
        val token = getToken()
        executeWithRetry {
            httpClient.put("$baseUrl/api/chat/$chatId/messages/$messageId") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(request)
            }
        }
    }
    
    /**
     * Delete a chat message
     */
    suspend fun deleteChatMessage(chatId: String, messageId: String) {
        val token = getToken()
        executeWithRetry {
            httpClient.delete("$baseUrl/api/chat/$chatId/messages/$messageId") {
                header("Authorization", "Bearer $token")
            }
        }
    }
    
    /**
     * Regenerate a chat message
     */
    suspend fun regenerateChatMessage(chatId: String, messageId: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/chat/$chatId/messages/$messageId/regenerate") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }
    
    /**
     * Toggle forgotten status of a message
     */
    suspend fun toggleForgottenMessage(chatId: String, messageId: String, request: JsonObject): Record {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.patch("$baseUrl/api/chat/$chatId/messages/$messageId/forgotten") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(request)
            }
        }
        return response.body()
    }
    
    /**
     * Create a WebSocket client for real-time operations
     */
    suspend fun websocket(wsUrl: String): WebSocketClient {
        val token = getToken()
        return WebSocketClient(wsUrl, token, httpClient)
    }
    
    /**
     * Close the client and release resources
     */
    fun close() {
        httpClient.close()
    }
    
    /**
     * Builder for creating EkoDBClient instances
     */
    class Builder {
        private var baseUrl: String = "http://localhost:8080"
        private var apiKey: String = ""
        private var timeout: Long = 30
        private var maxRetries: Int = 3
        
        fun baseUrl(url: String) = apply { this.baseUrl = url }
        fun apiKey(key: String) = apply { this.apiKey = key }
        fun timeout(seconds: Long) = apply { this.timeout = seconds }
        fun maxRetries(retries: Int) = apply { this.maxRetries = retries }
        
        fun build(): EkoDBClient {
            require(apiKey.isNotEmpty()) { "API key is required" }
            return EkoDBClient(baseUrl, apiKey, timeout, maxRetries)
        }
    }
    
    companion object {
        fun builder() = Builder()
        
        const val VERSION = "0.1.0"
    }
}

/**
 * Batch operation result
 */
data class BatchResult(
    val successful: List<Record>,
    val failed: List<BatchError>
)

/**
 * Batch operation error
 */
data class BatchError(
    val index: Int,
    val error: String
)
