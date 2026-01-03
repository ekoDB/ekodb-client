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
import io.ktor.http.content.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.util.*
import kotlinx.coroutines.delay
import kotlinx.serialization.json.*
import kotlinx.serialization.cbor.Cbor
import kotlinx.serialization.encodeToByteArray
import kotlinx.serialization.decodeFromByteArray
import kotlinx.serialization.ExperimentalSerializationApi
import kotlin.time.Duration.Companion.seconds

/**
 * Serialization format for client-server communication
 */
enum class SerializationFormat {
    /** JSON format (default, human-readable, fully supported) */
    JSON,
    /** MessagePack format (binary, experimental) - uses CBOR encoding. Has compatibility issues with custom types. */
    MESSAGEPACK
}

/**
 * ekoDB Client for Kotlin
 * 
 * Official Kotlin client library for ekoDB - A high-performance database with
 * intelligent caching, real-time capabilities, and automatic optimization.
 */
@OptIn(ExperimentalSerializationApi::class)
class EkoDBClient private constructor(
    private val baseUrl: String,
    private val apiKey: String,
    private val timeout: Long = 30,
    private val maxRetries: Int = 3,
    private val format: SerializationFormat = SerializationFormat.JSON // Default to JSON (CBOR has serialization compatibility issues)
) {
    // CBOR serializer for MessagePack format
    private val cbor = Cbor {
        ignoreUnknownKeys = true
    }
    
    private val httpClient = HttpClient(CIO) {
        install(ContentNegotiation) {
            json(Json {
                prettyPrint = true
                isLenient = true
                ignoreUnknownKeys = true
                // No global classDiscriminator - let sealed classes define their own
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
            contentType(getContentTypeForRequest())
            header("Accept", getContentTypeForRequest().toString())
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
     * Determine if a path should always use JSON (metadata endpoints)
     * CRUD operations support MessagePack/CBOR, but metadata/KV/auth/chat endpoints are JSON-only
     */
    private fun shouldUseJSON(path: String): Boolean {
        val jsonOnlyPaths = listOf(
            "/api/collections",
            "/api/kv",
            "/api/auth",
            "/api/health",
            "/api/config",
            "/api/schema",
            "/api/analytics",
            "/api/ripples",
            "/api/chat"
        )
        
        return jsonOnlyPaths.any { prefix ->
            path == prefix || path.startsWith("$prefix/")
        }
    }
    
    /**
     * Get the ContentType based on the serialization format and request URL
     */
    private fun HttpRequestBuilder.getContentTypeForRequest(): ContentType {
        val path = url.encodedPath
        return if (shouldUseJSON(path)) {
            ContentType.Application.Json
        } else {
            when (this@EkoDBClient.format) {
                SerializationFormat.JSON -> ContentType.Application.Json
                SerializationFormat.MESSAGEPACK -> ContentType("application", "msgpack")
            }
        }
    }
    
    /**
     * Set request body with proper serialization based on format and request URL
     */
    private fun HttpRequestBuilder.setBodyWithFormat(data: Any) {
        // Get path from the request URL
        val path = url.encodedPath
        val forceJSON = shouldUseJSON(path)
        
        when {
            forceJSON || this@EkoDBClient.format == SerializationFormat.JSON -> {
                // Use JSON serialization
                when (data) {
                    is JsonElement -> {
                        val jsonString = Json.encodeToString(JsonElement.serializer(), data)
                        setBody(TextContent(jsonString, ContentType.Application.Json))
                    }
                    else -> setBody(data)  // Let ContentNegotiation handle @Serializable types
                }
            }
            this@EkoDBClient.format == SerializationFormat.MESSAGEPACK -> {
                // Manually serialize to CBOR bytes for CRUD operations only
                val cborBytes = when (data) {
                    is JsonElement -> {
                        // Convert JsonElement to JSON string first, then to CBOR-compatible structure
                        // JsonElement can be JsonObject, JsonArray, JsonPrimitive, or JsonNull
                        val jsonString = Json.encodeToString(JsonElement.serializer(), data)
                        this@EkoDBClient.cbor.encodeToByteArray(JsonElement.serializer(), Json.decodeFromString(jsonString))
                    }
                    is Record -> {
                        // Serialize Record using its custom serializer
                        this@EkoDBClient.cbor.encodeToByteArray(Record.serializer(), data)
                    }
                    is Query -> {
                        // Serialize Query using its custom serializer
                        this@EkoDBClient.cbor.encodeToByteArray(Query.serializer(), data)
                    }
                    else -> {
                        // CBOR only supports known @Serializable types
                        // For other types, throw clear error instead of attempting dynamic resolution
                        throw IllegalArgumentException(
                            "MessagePack/CBOR serialization only supports Record, Query, and JsonElement types. " +
                            "Received unsupported type: ${data::class.qualifiedName}. " +
                            "Use SerializationFormat.JSON instead or ensure your type is @Serializable and explicitly handled."
                        )
                    }
                }
                setBody(ByteArrayContent(cborBytes, ContentType("application", "msgpack")))
            }
        }
    }
    
    /**
     * Parse response body based on format
     */
    private suspend inline fun <reified T> HttpResponse.bodyWithFormat(): T {
        return when (this@EkoDBClient.format) {
            SerializationFormat.JSON -> body()
            SerializationFormat.MESSAGEPACK -> {
                val bytes = body<ByteArray>()
                val jsonElement = this@EkoDBClient.cbor.decodeFromByteArray(JsonElement.serializer(), bytes)
                Json.decodeFromJsonElement(jsonElement)
            }
        }
    }
    
    /**
     * Insert a record into a collection
     */
    suspend fun insert(collection: String, record: Record): Record {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/insert/$collection") {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBodyWithFormat(record)
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Insert failed with status ${response.status}: $errorBody")
        }
        
        return response.bodyWithFormat()
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
     * @param collection The collection name
     * @param updates List of (id, data) pairs to update
     * @param transactionId Optional transaction ID for transactional updates
     */
    suspend fun batchUpdate(
        collection: String, 
        updates: List<Pair<String, Record>>,
        transactionId: String? = null
    ): List<Record> {
        val token = getToken()
        val urlPath = if (transactionId != null) {
            "$baseUrl/api/batch/update/$collection?transaction_id=$transactionId"
        } else {
            "$baseUrl/api/batch/update/$collection"
        }
        val response = executeWithRetry {
            httpClient.put(urlPath) {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
     * Key-Value: Check if a key exists
     */
    suspend fun kvExists(key: String): Boolean {
        return try {
            val result = kvGet(key)
            result != null
        } catch (e: Exception) {
            false
        }
    }
    
    /**
     * Key-Value: Find/query entries with pattern matching
     */
    suspend fun kvFind(pattern: String? = null, includeExpired: Boolean = false): List<JsonElement> {
        val token = getToken()
        val body = buildJsonObject {
            pattern?.let { put("pattern", it) }
            put("include_expired", includeExpired)
        }
        
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/kv/find") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(body)
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("KV find failed with status ${response.status}: $errorBody")
        }
        
        return response.body<List<JsonElement>>()
    }
    
    /**
     * Key-Value: Query entries with pattern (alias for kvFind)
     */
    suspend fun kvQuery(pattern: String? = null, includeExpired: Boolean = false): List<JsonElement> {
        return kvFind(pattern, includeExpired)
    }
    
    // ============================================================================
    // Transaction Operations
    // ============================================================================
    
    /**
     * Begin a new transaction
     * @param isolationLevel Transaction isolation level (default: "ReadCommitted")
     * @return Transaction ID
     */
    suspend fun beginTransaction(isolationLevel: String = "ReadCommitted"): String {
        val token = getToken()
        
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/transactions") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(mapOf("isolation_level" to isolationLevel))
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Begin transaction failed with status ${response.status}: $errorBody")
        }
        
        val result = response.body<JsonElement>()
        return result.jsonObject["transaction_id"]?.jsonPrimitive?.content
            ?: throw Exception("No transaction_id in response")
    }
    
    /**
     * Get transaction status
     * @param transactionId The transaction ID
     * @return Transaction status map
     */
    suspend fun getTransactionStatus(transactionId: String): Map<String, Any?> {
        val token = getToken()
        
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/transactions/$transactionId") {
                header("Authorization", "Bearer $token")
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Get transaction status failed with status ${response.status}: $errorBody")
        }
        
        val result = response.body<JsonElement>()
        return mapOf(
            "state" to result.jsonObject["state"]?.jsonPrimitive?.content,
            "operations_count" to result.jsonObject["operations_count"]?.jsonPrimitive?.int
        )
    }
    
    /**
     * Commit a transaction
     * @param transactionId The transaction ID to commit
     */
    suspend fun commitTransaction(transactionId: String) {
        val token = getToken()
        
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/transactions/$transactionId/commit") {
                header("Authorization", "Bearer $token")
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Commit transaction failed with status ${response.status}: $errorBody")
        }
    }
    
    /**
     * Rollback a transaction
     * @param transactionId The transaction ID to rollback
     */
    suspend fun rollbackTransaction(transactionId: String) {
        val token = getToken()
        
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/transactions/$transactionId/rollback") {
                header("Authorization", "Bearer $token")
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Rollback transaction failed with status ${response.status}: $errorBody")
        }
    }
    
    /**
     * Execute request with retry logic
     */
    private suspend fun executeWithRetry(
        block: suspend () -> HttpResponse
    ): HttpResponse {
        var lastException: Exception? = null
        var tokenRefreshed = false
        
        repeat(maxRetries) { attempt ->
            try {
                val response = block()
                
                // Check for unauthorized (401) - try refreshing token once
                if (response.status == HttpStatusCode.Unauthorized && !tokenRefreshed) {
                    println("Authentication failed, refreshing token...")
                    refreshToken()
                    tokenRefreshed = true
                    return@repeat
                }
                
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
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
        private var format: SerializationFormat = SerializationFormat.JSON // Default to JSON (CBOR has compatibility issues)
        
        fun baseUrl(url: String) = apply { this.baseUrl = url }
        fun apiKey(key: String) = apply { this.apiKey = key }
        fun timeout(seconds: Long) = apply { this.timeout = seconds }
        fun maxRetries(retries: Int) = apply { this.maxRetries = retries }
        
        /**
         * Set serialization format (default: JSON for compatibility, MESSAGEPACK experimental)
         * MESSAGEPACK uses CBOR encoding which is faster but has compatibility issues with custom types
         */
        fun format(format: SerializationFormat) = apply { this.format = format }
        
        fun build(): EkoDBClient {
            require(apiKey.isNotEmpty()) { "API key is required" }
            return EkoDBClient(baseUrl, apiKey, timeout, maxRetries, format)
        }
    }
    
    // ========================================================================
    // SCRIPTS API
    // ========================================================================
    
    /**
     * Save a new script definition
     */
    suspend fun saveScript(script: io.ekodb.client.functions.Script): String {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/functions") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(script)
            }
        }
        
        // Check for error response
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        
        val result = response.body<JsonObject>()
        return result["id"]?.jsonPrimitive?.content 
            ?: throw IllegalStateException("No script ID returned")
    }
    
    /**
     * Get a script by ID
     */
    suspend fun getScript(id: String): io.ekodb.client.functions.Script {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.get("$baseUrl/api/functions/$id") {
                bearerAuth(token)
            }
        }
        return response.body<io.ekodb.client.functions.Script>()
    }
    
    /**
     * List all scripts, optionally filtered by tags
     */
    suspend fun listScripts(tags: List<String>? = null): List<io.ekodb.client.functions.Script> {
        val token = getToken()
        val url = if (tags != null) {
            "$baseUrl/api/functions?tags=${tags.joinToString(",")}"
        } else {
            "$baseUrl/api/functions"
        }
        val response = executeWithRetry {
            httpClient.get(url) {
                bearerAuth(token)
            }
        }
        return response.body<List<io.ekodb.client.functions.Script>>()
    }
    
    /**
     * Update an existing script by ID
     */
    suspend fun updateScript(id: String, script: io.ekodb.client.functions.Script) {
        val token = getToken()
        executeWithRetry {
            httpClient.put("$baseUrl/api/functions/$id") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(script)
            }
        }
    }
    
    /**
     * Delete a script by ID
     */
    suspend fun deleteScript(id: String) {
        val token = getToken()
        executeWithRetry {
            httpClient.delete("$baseUrl/api/functions/$id") {
                bearerAuth(token)
            }
        }
    }
    
    /**
     * Call a script by label or ID
     * @param labelOrId Script label or encrypted ID
     * @param params Optional parameters for the script
     */
    suspend fun callScript(
        labelOrId: String,
        params: Map<String, JsonElement>? = null
    ): io.ekodb.client.functions.FunctionResult {
        val token = getToken()
        val response = executeWithRetry {
            httpClient.post("$baseUrl/api/functions/$labelOrId") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                // Send parameters directly, or empty map if no params
                setBody(params ?: emptyMap<String, JsonElement>())
            }
        }
        return response.body<io.ekodb.client.functions.FunctionResult>()
    }
    
    /**
     * Call a script (deprecated - use callScript with labelOrId)
     * @deprecated Use callScript(labelOrId, params) instead
     */
    @Deprecated("Collection parameter is not used for scripts", ReplaceWith("callScript(label, params)"))
    suspend fun callScript(
        collection: String,
        label: String,
        params: Map<String, JsonElement>? = null
    ): io.ekodb.client.functions.FunctionResult {
        return callScript(label, params)
    }
    
    // ========== RAG Helper Methods ==========
    
    /**
     * Generate embeddings for text using ekoDB's native Functions
     * 
     * This helper simplifies embedding generation by:
     * 1. Creating a temporary collection with the text
     * 2. Running a Script with FindAll + Embed Functions
     * 3. Extracting and returning the embedding vector
     * 4. Cleaning up temporary resources
     * 
     * @param text The text to generate embeddings for
     * @param model The embedding model to use (e.g., "text-embedding-3-small")
     * @return List of floats representing the embedding vector
     * 
     * @example
     * ```kotlin
     * val embedding = client.embed("Hello world", "text-embedding-3-small")
     * println("Generated ${embedding.size} dimensions")
     * ```
     */
    suspend fun embed(text: String, model: String): List<Double> {
        val tempCollection = "embed_temp_${System.currentTimeMillis()}_${(Math.random() * 1000000).toInt()}"
        
        try {
            // Insert temporary record with the text
            val record = Record()
            record.insert("text", text)
            insert(tempCollection, record)
            
            // Create Script with FindAll + Embed Functions
            val tempLabel = "embed_script_${System.currentTimeMillis()}_${(Math.random() * 1000000).toInt()}"
            val script = io.ekodb.client.functions.Script(
                label = tempLabel,
                name = "Generate Embedding",
                description = "Temporary script for embedding generation",
                version = "1.0",
                parameters = emptyMap(),
                functions = listOf(
                    io.ekodb.client.functions.FunctionStageConfig.FindAll(
                        collection = tempCollection
                    ),
                    io.ekodb.client.functions.FunctionStageConfig.Embed(
                        input_field = "text",
                        output_field = "embedding",
                        model = JsonPrimitive(model)
                    )
                ),
                tags = emptyList()
            )
            
            // Save and execute the script
            val scriptId = saveScript(script)
            val result = callScript(scriptId)
            
            // Clean up
            try { deleteScript(scriptId) } catch (_: Exception) {}
            try { deleteCollection(tempCollection) } catch (_: Exception) {}
            
            // Extract embedding from result
            if (result.records.isNotEmpty()) {
                val record = result.records[0]
                val embedding = record["embedding"]
                if (embedding is JsonArray) {
                    return embedding.map { it.jsonPrimitive.double }
                }
            }
            
            throw Exception("Failed to extract embedding from result")
        } catch (e: Exception) {
            // Ensure cleanup even on error
            try { deleteCollection(tempCollection) } catch (_: Exception) {}
            throw e
        }
    }
    
    /**
     * Perform text search without embeddings
     * 
     * Simplified text search with full-text matching, fuzzy search, and stemming.
     * 
     * @param collection Collection name to search
     * @param queryText Search query text
     * @param limit Maximum number of results to return
     * @return List of matching records
     * 
     * @example
     * ```kotlin
     * val results = client.textSearch("documents", "ownership system", 10)
     * ```
     */
    suspend fun textSearch(collection: String, queryText: String, limit: Int): List<JsonObject> {
        val searchQuery = buildJsonObject {
            put("query", queryText)
            put("limit", limit)
        }
        
        val response = search(collection, searchQuery)
        val results = response["results"]?.jsonArray ?: return emptyList()
        
        return results.map { result ->
            result.jsonObject["record"]?.jsonObject ?: buildJsonObject {}
        }
    }
    
    /**
     * Perform hybrid search combining text and vector search
     * 
     * Combines semantic similarity (vector search) with keyword matching (text search)
     * for more accurate and relevant results.
     * 
     * @param collection Collection name to search
     * @param queryText Search query text
     * @param queryVector Embedding vector for semantic search
     * @param limit Maximum number of results to return
     * @return List of matching records
     * 
     * @example
     * ```kotlin
     * val embedding = client.embed(query, "text-embedding-3-small")
     * val results = client.hybridSearch("documents", query, embedding, 5)
     * ```
     */
    suspend fun hybridSearch(
        collection: String,
        queryText: String,
        queryVector: List<Double>,
        limit: Int
    ): List<JsonObject> {
        val searchQuery = buildJsonObject {
            put("query", queryText)
            put("vector", JsonArray(queryVector.map { JsonPrimitive(it) }))
            put("limit", limit)
        }
        
        val response = search(collection, searchQuery)
        val results = response["results"]?.jsonArray ?: return emptyList()
        
        return results.map { result ->
            result.jsonObject["record"]?.jsonObject ?: buildJsonObject {}
        }
    }
    
    /**
     * Find all records in a collection with a limit
     * 
     * Simplified method to query all documents in a collection.
     * 
     * @param collection Collection name
     * @param limit Maximum number of records to return
     * @return List of records
     * 
     * @example
     * ```kotlin
     * val allMessages = client.findAllWithLimit("messages", 1000)
     * println("Found ${allMessages.size} messages")
     * ```
     */
    suspend fun findAllWithLimit(collection: String, limit: Int): List<Record> {
        val query = QueryBuilder()
            .limit(limit)
            .build()
        return find(collection, query)
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
