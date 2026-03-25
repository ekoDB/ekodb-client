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
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
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
    private val format: SerializationFormat = SerializationFormat.JSON, // Default to JSON (CBOR has serialization compatibility issues)
    injectedClient: HttpClient? = null // Optional injected client for testing
) {
    // CBOR serializer for MessagePack format
    private val cbor = Cbor {
        ignoreUnknownKeys = true
    }
    
    private val client: HttpClient = injectedClient ?: HttpClient(CIO) {
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
            connectTimeoutMillis = timeout * 1000
        }
        
        install(Logging) {
            logger = Logger.DEFAULT
            level = LogLevel.INFO
        }
        
        install(WebSockets)
    }
    
    private var authToken: String? = null
    private var tokenExpiry: Long = 0
    private var lastRateLimitInfo: RateLimitInfo? = null

    /**
     * Get the last observed rate limit information.
     * Returns null if no rate limit headers have been received yet.
     */
    fun getRateLimitInfo(): RateLimitInfo? = lastRateLimitInfo

    /**
     * Check if the client is near the rate limit threshold (< 10% remaining).
     */
    fun isNearRateLimit(): Boolean = lastRateLimitInfo?.isNearLimit() ?: false

    /**
     * Extract rate limit info from response headers.
     */
    private fun extractRateLimitInfo(response: HttpResponse) {
        val limit = response.headers["X-RateLimit-Limit"]?.toIntOrNull()
        val remaining = response.headers["X-RateLimit-Remaining"]?.toIntOrNull()
        val reset = response.headers["X-RateLimit-Reset"]?.toLongOrNull()
        if (limit != null && remaining != null && reset != null) {
            lastRateLimitInfo = RateLimitInfo(limit, remaining, reset)
        }
    }
    
    /**
     * Get authentication token (exchanges API key for JWT).
     *
     * Returns a cached token if it has more than 60 seconds of validity remaining.
     * Otherwise fetches a new one via `/api/auth/token`. This means callers
     * never need to handle token refresh themselves -- every `getToken()` call
     * returns a token that is valid for at least 60 more seconds.
     */
    private suspend fun getToken(): String {
        // Return cached token if available and not about to expire
        authToken?.let { cached ->
            val now = System.currentTimeMillis() / 1000
            // Refresh 60s before expiry to avoid edge-case races
            if (now + 60 < tokenExpiry) {
                return cached
            }
        }

        // Exchange API key for JWT token
        val response = client.post("$baseUrl/api/auth/token") {
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

        // Extract expiry from JWT payload (base64-decode the middle segment)
        tokenExpiry = extractJWTExpiry(token)
            ?: (System.currentTimeMillis() / 1000 + 3600) // Fallback: assume 1 hour from now

        authToken = token
        return token
    }

    /**
     * Extract the `exp` claim from a JWT without verifying the signature.
     * Returns the Unix timestamp (seconds) of expiry, or null if parsing fails.
     */
    internal fun extractJWTExpiry(token: String): Long? {
        val parts = token.split(".")
        if (parts.size != 3) return null
        return try {
            val payloadBytes = java.util.Base64.getUrlDecoder().decode(parts[1])
            val payload = Json.parseToJsonElement(String(payloadBytes)).jsonObject
            payload["exp"]?.jsonPrimitive?.long
        } catch (_: Exception) {
            null
        }
    }
    
    /**
     * Refresh the authentication token
     */
    suspend fun refreshToken(): String {
        clearTokenCache()
        return getToken()
    }
    
    /**
     * Clear the cached authentication token and expiry
     */
    fun clearTokenCache() {
        authToken = null
        tokenExpiry = 0
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
    suspend fun insert(
        collection: String,
        record: Record,
        ttl: String? = null,
        bypassRipple: Boolean? = null,
        transactionId: String? = null,
        bypassCache: Boolean? = null
    ): Record {
        val token = getToken()
        // Add TTL to record if provided
        val finalRecord = if (ttl != null) record.withTtl(ttl) else record
        
        // Build query parameters
        val params = mutableListOf<String>()
        bypassRipple?.let { params.add("bypass_ripple=$it") }
        transactionId?.let { params.add("transaction_id=$it") }
        
        val url = if (params.isNotEmpty()) {
            "$baseUrl/api/insert/$collection?${params.joinToString("&")}"
        } else {
            "$baseUrl/api/insert/$collection"
        }
        val response = executeWithRetry {
            client.post(url) {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(finalRecord)
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
            client.get("$baseUrl/api/find/$collection/$id") {
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
     * Find a record by ID with field projection.
     *
     * @param collection The collection name
     * @param id The record ID
     * @param selectFields Fields to include in the result (null for all)
     * @param excludeFields Fields to exclude from the result (null for none)
     */
    suspend fun findByIdWithProjection(
        collection: String,
        id: String,
        selectFields: List<String>? = null,
        excludeFields: List<String>? = null
    ): Record {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/find/$collection/$id") {
                header("Authorization", "Bearer $token")
                selectFields?.let { parameter("select_fields", it.joinToString(",")) }
                excludeFields?.let { parameter("exclude_fields", it.joinToString(",")) }
            }
        }

        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Find by ID with projection failed with status ${response.status}: $errorBody")
        }

        return response.body()
    }

    /**
     * Find records with a query
     */
    suspend fun find(collection: String, query: Query): List<Record> {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/find/$collection") {
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
    suspend fun update(
        collection: String,
        id: String,
        updates: Record,
        bypassRipple: Boolean? = null,
        transactionId: String? = null,
        bypassCache: Boolean? = null,
        selectFields: List<String>? = null,
        excludeFields: List<String>? = null
    ): Record {
        val token = getToken()
        // Build query parameters
        val params = mutableListOf<String>()
        bypassRipple?.let { params.add("bypass_ripple=$it") }
        transactionId?.let { params.add("transaction_id=$it") }
        
        val url = if (params.isNotEmpty()) {
            "$baseUrl/api/update/$collection/$id?${params.joinToString("&")}"
        } else {
            "$baseUrl/api/update/$collection/$id"
        }
        val response = executeWithRetry {
            client.put(url) {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(updates)
            }
        }
        return response.body()
    }
    
    /**
     * Apply an atomic field action to a single field of a record.
     *
     * Use this instead of [update] for safe concurrent modifications like
     * incrementing counters, pushing to arrays, or arithmetic operations.
     *
     * @param collection Collection name
     * @param id Record ID
     * @param action The atomic action: increment, decrement, multiply, divide, modulo,
     *               push, pop, shift, unshift, remove, append, clear
     * @param field The field name to apply the action to
     * @param value The value for the action (null for pop/shift/clear)
     */
    suspend fun updateWithAction(
        collection: String,
        id: String,
        action: String,
        field: String,
        value: JsonElement? = null
    ): Record {
        val token = getToken()
        val url = "$baseUrl/api/update/$collection/$id/action/$action"
        val body = buildJsonObject {
            put("field", field)
            put("value", value ?: JsonNull)
        }
        val response = executeWithRetry {
            client.put(url) {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(body)
            }
        }
        return response.body()
    }

    /**
     * Apply a sequence of atomic field actions to a record in a single request.
     *
     * All actions are applied atomically — the record is fetched once, all actions
     * run in order, and the result is persisted in a single update.
     *
     * @param collection Collection name
     * @param id Record ID
     * @param actions List of Triple(action, field, value)
     */
    suspend fun updateWithActionSequence(
        collection: String,
        id: String,
        actions: List<Triple<String, String, JsonElement>>
    ): Record {
        val token = getToken()
        val url = "$baseUrl/api/update/sequence/$collection/$id"
        val body = buildJsonArray {
            for ((action, field, value) in actions) {
                add(buildJsonArray {
                    add(action)
                    add(field)
                    add(value)
                })
            }
        }
        val response = executeWithRetry {
            client.put(url) {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(body)
            }
        }
        return response.body()
    }

    /**
     * Delete a record
     */
    suspend fun delete(
        collection: String,
        id: String,
        bypassRipple: Boolean? = null,
        transactionId: String? = null
    ) {
        val token = getToken()
        // Build query parameters
        val params = mutableListOf<String>()
        bypassRipple?.let { params.add("bypass_ripple=$it") }
        transactionId?.let { params.add("transaction_id=$it") }
        
        val url = if (params.isNotEmpty()) {
            "$baseUrl/api/delete/$collection/$id?${params.joinToString("&")}"
        } else {
            "$baseUrl/api/delete/$collection/$id"
        }
        val response = executeWithRetry {
            client.delete(url) {
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
    suspend fun batchInsert(collection: String, records: List<Record>, bypassRipple: Boolean? = null): BatchResult {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/batch/insert/$collection") {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(buildJsonObject {
                    put("inserts", JsonArray(records.map { record ->
                        buildJsonObject {
                            put("data", Json.encodeToJsonElement(record))
                            if (bypassRipple != null) {
                                put("bypass_ripple", bypassRipple)
                            }
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
                Record().apply { this["id"] = FieldType.string(id) }
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
     * @param bypassRipple Optional flag to bypass ripple effects
     */
    suspend fun batchUpdate(
        collection: String, 
        updates: List<Pair<String, Record>>,
        transactionId: String? = null,
        bypassRipple: Boolean? = null
    ): List<Record> {
        val token = getToken()
        val urlPath = if (transactionId != null) {
            "$baseUrl/api/batch/update/$collection?transaction_id=$transactionId"
        } else {
            "$baseUrl/api/batch/update/$collection"
        }
        val response = executeWithRetry {
            client.put(urlPath) {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(buildJsonObject {
                    put("updates", JsonArray(updates.map { (id, data) ->
                        buildJsonObject {
                            put("id", id)
                            put("data", Json.encodeToJsonElement(data))
                            if (bypassRipple != null) {
                                put("bypass_ripple", bypassRipple)
                            }
                        }
                    }))
                })
            }
        }
        
        val result = response.body<JsonObject>()
        val successful = result["successful"]?.jsonArray ?: JsonArray(emptyList())
        return successful.map { idElement ->
            val id = idElement.jsonPrimitive.content
            Record().apply { this["id"] = FieldType.string(id) }
        }
    }
    
    /**
     * Batch delete records by IDs
     */
    suspend fun batchDelete(collection: String, ids: List<String>, bypassRipple: Boolean? = null): Long {
        val token = getToken()
        val response = executeWithRetry {
            client.delete("$baseUrl/api/batch/delete/$collection") {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(buildJsonObject {
                    put("deletes", JsonArray(ids.map { id ->
                        buildJsonObject {
                            put("id", id)
                            if (bypassRipple != null) {
                                put("bypass_ripple", bypassRipple)
                            }
                        }
                    }))
                })
            }
        }
        
        val result = response.body<JsonObject>()
        val successful = result["successful"]?.jsonArray ?: JsonArray(emptyList())
        return successful.size.toLong()
    }
    
    // ========== Convenience Methods ==========
    
    /**
     * Insert or update a record (upsert operation)
     * 
     * Attempts to update the record first. If the record doesn't exist (404 error),
     * it will be inserted instead. This provides atomic insert-or-update semantics.
     * 
     * @param collection Collection name
     * @param id Record ID
     * @param record Record data to insert or update
     * @param bypassRipple Optional flag to bypass ripple effects
     * @return The inserted or updated record
     */
    suspend fun upsert(
        collection: String,
        id: String,
        record: Record,
        bypassRipple: Boolean? = null
    ): Record {
        return try {
            // Try update first
            update(collection, id, record, bypassRipple)
        } catch (e: Exception) {
            // Check if it's a NotFound error
            if (e.message?.contains("404") == true || e.message?.contains("Not found") == true) {
                // Record doesn't exist, insert it
                insert(collection, record, ttl = null, bypassRipple = bypassRipple)
            } else {
                // Other error, propagate it
                throw e
            }
        }
    }
    
    /**
     * Find a single record by field value
     * 
     * Convenience method for finding one record matching a specific field value.
     * Returns null if no record matches, or the first matching record.
     * 
     * @param collection Collection name
     * @param field Field name to search
     * @param value Value to match (any JSON-serializable type)
     * @return The matching record or null if not found
     */
    suspend fun findOne(collection: String, field: String, value: Any): Record? {
        val query = QueryBuilder()
            .eq(field, value)
            .limit(1)
            .build()
        
        val results = find(collection, query)
        return results.firstOrNull()
    }
    
    /**
     * Check if a record exists by ID
     * 
     * This is more efficient than fetching the record when you only need to check existence.
     * 
     * @param collection Collection name
     * @param id Record ID to check
     * @return true if the record exists, false if it doesn't
     */
    suspend fun exists(collection: String, id: String): Boolean {
        return try {
            findById(collection, id)
            true
        } catch (e: Exception) {
            if (e.message?.contains("404") == true || e.message?.contains("Not found") == true) {
                false
            } else {
                throw e
            }
        }
    }
    
    /**
     * Paginate through records
     * 
     * Convenience method for pagination with page numbers (1-indexed).
     * 
     * @param collection Collection name
     * @param page Page number (1-indexed, i.e., first page is 1)
     * @param pageSize Number of records per page
     * @return List of records for the requested page
     */
    suspend fun paginate(collection: String, page: Int, pageSize: Int): List<Record> {
        // Page 1 = skip 0, Page 2 = skip pageSize, etc.
        val skip = if (page > 0) (page - 1) * pageSize else 0
        
        val query = QueryBuilder()
            .limit(pageSize)
            .skip(skip)
            .build()
        
        return find(collection, query)
    }
    
    /**
     * Count documents in a collection
     */
    suspend fun count(collection: String): Long {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/collections/$collection") {
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
            client.get("$baseUrl/api/collections") {
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
            client.delete("$baseUrl/api/collections/$collection") {
                header("Authorization", "Bearer $token")
            }
        }
    }
    
    /**
     * Restore a deleted record from trash
     * Records remain in trash for 30 days before permanent deletion
     */
    suspend fun restoreRecord(collection: String, id: String): Boolean {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/trash/$collection/$id") {
                header("Authorization", "Bearer $token")
            }
        }
        val result: JsonObject = response.body()
        return result["status"]?.toString()?.contains("restored") == true
    }
    
    /**
     * Restore all deleted records in a collection from trash
     * Returns the number of records restored
     */
    suspend fun restoreCollection(collection: String): Long {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/trash/$collection") {
                header("Authorization", "Bearer $token")
            }
        }
        val result: JsonObject = response.body()
        return result["records_restored"]?.toString()?.toLongOrNull() ?: 0L
    }
    
    /**
     * Health check - verify the ekoDB server is responding
     */
    suspend fun health(): Boolean {
        return try {
            val response = client.get("$baseUrl/api/health")
            val result: JsonObject = response.body()
            result["status"]?.toString()?.contains("ok") == true
        } catch (e: Exception) {
            false
        }
    }
    
    /**
     * Create a collection with schema
     */
    suspend fun createCollection(collection: String, schema: JsonObject) {
        val token = getToken()
        executeWithRetry {
            client.post("$baseUrl/api/collections/$collection") {
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
            client.get("$baseUrl/api/collections/$collection") {
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
            client.get("$baseUrl/api/schemas/$collection") {
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
     * Count documents in a collection
     */
    suspend fun countDocuments(collection: String): Long {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/$collection/count") {
                header("Authorization", "Bearer $token")
            }
        }
        val body = response.bodyAsText()
        val json = Json.parseToJsonElement(body).jsonObject
        return json["count"]?.jsonPrimitive?.long ?: 0L
    }
    
    /**
     * Search documents in a collection
     */
    suspend fun search(collection: String, searchQuery: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/search/$collection") {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(searchQuery)
            }
        }
        return response.body()
    }
    
    /**
     * Get distinct (unique) values for a field across a collection.
     *
     * Results are deduplicated and sorted alphabetically by the server.
     *
     * @param collection Collection name
     * @param field Field to get distinct values for
     * @param filter Optional filter expression (same format as find())
     * @param bypassRipple Optional flag to bypass ripple propagation
     * @param bypassCache Optional flag to bypass cache
     * @return JsonObject with keys: collection, field, values (array), count (int)
     *
     * Example:
     * ```kotlin
     * // All distinct statuses
     * val resp = client.distinctValues("orders", "status")
     *
     * // Statuses for US orders only
     * val filter = buildJsonObject {
     *     put("type", "Condition")
     *     put("content", buildJsonObject {
     *         put("field", "region")
     *         put("operator", "Eq")
     *         put("value", "us")
     *     })
     * }
     * val resp = client.distinctValues("orders", "status", filter = filter)
     * ```
     */
    suspend fun distinctValues(
        collection: String,
        field: String,
        filter: JsonObject? = null,
        bypassRipple: Boolean? = null,
        bypassCache: Boolean? = null,
    ): JsonObject {
        val token = getToken()
        val body = buildJsonObject {
            if (filter != null) put("filter", filter)
            if (bypassRipple != null) put("bypass_ripple", bypassRipple)
            if (bypassCache != null) put("bypass_cache", bypassCache)
        }
        val response = executeWithRetry {
            client.post("$baseUrl/api/distinct/$collection/$field") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                header("Accept", ContentType.Application.Json.toString())
                setBody(body)
            }
        }
        return response.body()
    }

    /**
     * Stateless raw LLM completion — no session, no history, no RAG.
     *
     * Sends a system prompt and user message directly to the LLM via ekoDB
     * and returns the raw text response without any context injection or
     * conversation management. Use this for structured-output tasks such as
     * planning where the response must be parsed programmatically.
     *
     * @param systemPrompt System prompt passed verbatim to the LLM
     * @param message User message passed verbatim to the LLM
     * @param provider Optional LLM provider override
     * @param model Optional model name override
     * @param maxTokens Optional max tokens override
     * @return JsonObject with "content" field containing the LLM response
     *
     * @example
     * ```kotlin
     * val resp = client.rawCompletion(
     *     systemPrompt = "You are a helpful assistant.",
     *     message = "Summarize this in JSON.",
     *     maxTokens = 2048
     * )
     * println(resp["content"]?.jsonPrimitive?.content)
     * ```
     */
    suspend fun rawCompletion(
        systemPrompt: String,
        message: String,
        provider: String? = null,
        model: String? = null,
        maxTokens: Int? = null,
    ): JsonObject {
        val token = getToken()
        val body = buildJsonObject {
            put("system_prompt", systemPrompt)
            put("message", message)
            if (provider != null) put("provider", provider)
            if (model != null) put("model", model)
            if (maxTokens != null) put("max_tokens", maxTokens)
        }
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/complete") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                header("Accept", ContentType.Application.Json.toString())
                setBody(body)
            }
        }
        return response.body()
    }

    /**
     * Stateless raw LLM completion via SSE streaming.
     *
     * Same as [rawCompletion] but uses Server-Sent Events to keep the
     * connection alive. Preferred for deployed instances where reverse proxies
     * may kill idle HTTP connections before the LLM responds.
     *
     * @param systemPrompt System instructions for the LLM
     * @param message User message to send
     * @param provider LLM provider (optional, uses server default)
     * @param model Model name (optional, uses server default)
     * @param maxTokens Maximum tokens in response (optional)
     * @return JsonObject with "content" key containing the LLM response text
     *
     * ```kotlin
     * val resp = client.rawCompletionStream(
     *     systemPrompt = "You are a helpful assistant.",
     *     message = "Summarize this in JSON.",
     *     maxTokens = 2048
     * )
     * println(resp["content"]?.jsonPrimitive?.content)
     * ```
     */
    suspend fun rawCompletionStream(
        systemPrompt: String,
        message: String,
        provider: String? = null,
        model: String? = null,
        maxTokens: Int? = null,
    ): JsonObject {
        val token = getToken()
        val body = buildJsonObject {
            put("system_prompt", systemPrompt)
            put("message", message)
            if (provider != null) put("provider", provider)
            if (model != null) put("model", model)
            if (maxTokens != null) put("max_tokens", maxTokens)
        }
        val response = client.post("$baseUrl/api/chat/complete/stream") {
            header("Authorization", "Bearer $token")
            contentType(ContentType.Application.Json)
            header("Accept", ContentType.Text.EventStream.toString())
            setBody(body)
        }
        if (response.status != io.ktor.http.HttpStatusCode.OK) {
            val errBody = response.bodyAsText()
            throw RuntimeException("SSE raw completion failed (${response.status}): $errBody")
        }
        val sseBody = response.bodyAsText()
        var content = ""
        var lastError: String? = null
        for (line in sseBody.lines()) {
            if (!line.startsWith("data:")) continue
            val dataStr = line.removePrefix("data:").trim()
            if (dataStr.isEmpty()) continue
            try {
                val eventData = Json.parseToJsonElement(dataStr).jsonObject
                eventData["token"]?.jsonPrimitive?.content?.let { content += it }
                eventData["content"]?.jsonPrimitive?.content?.let { content = it }
                eventData["error"]?.jsonPrimitive?.content?.let { lastError = it }
            } catch (_: Exception) {
                // skip malformed SSE data
            }
        }
        if (lastError != null) throw RuntimeException("LLM error: $lastError")
        return buildJsonObject { put("content", content) }
    }

    /**
     * Stateless raw LLM completion via SSE with a per-token callback.
     *
     * Identical to [rawCompletionStream] but invokes [onToken] for every
     * token received, enabling real-time progress display.
     */
    suspend fun rawCompletionStreamWithProgress(
        systemPrompt: String,
        message: String,
        provider: String? = null,
        model: String? = null,
        maxTokens: Int? = null,
        onToken: (String) -> Unit,
    ): JsonObject {
        val token = getToken()
        val body = buildJsonObject {
            put("system_prompt", systemPrompt)
            put("message", message)
            if (provider != null) put("provider", provider)
            if (model != null) put("model", model)
            if (maxTokens != null) put("max_tokens", maxTokens)
        }
        val response = client.post("$baseUrl/api/chat/complete/stream") {
            header("Authorization", "Bearer $token")
            contentType(ContentType.Application.Json)
            header("Accept", ContentType.Text.EventStream.toString())
            setBody(body)
        }
        if (response.status != io.ktor.http.HttpStatusCode.OK) {
            val errBody = response.bodyAsText()
            throw RuntimeException("SSE raw completion failed (${response.status}): $errBody")
        }
        val sseBody = response.bodyAsText()
        var content = ""
        var lastError: String? = null
        for (line in sseBody.lines()) {
            if (!line.startsWith("data:")) continue
            val dataStr = line.removePrefix("data:").trim()
            if (dataStr.isEmpty()) continue
            try {
                val eventData = Json.parseToJsonElement(dataStr).jsonObject
                eventData["token"]?.jsonPrimitive?.content?.let {
                    content += it
                    onToken(it)
                }
                eventData["content"]?.jsonPrimitive?.content?.let { content = it }
                eventData["error"]?.jsonPrimitive?.content?.let { lastError = it }
            } catch (_: Exception) {
                // skip malformed SSE data
            }
        }
        if (lastError != null) throw RuntimeException("LLM error: $lastError")
        return buildJsonObject { put("content", content) }
    }

    /**
     * Key-Value: Set a key-value pair
     */
    suspend fun kvSet(key: String, value: JsonElement) {
        val token = getToken()
        executeWithRetry {
            client.post("$baseUrl/api/kv/set/$key") {
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
            client.post("$baseUrl/api/kv/set/$key") {
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
            client.get("$baseUrl/api/kv/get/$key") {
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
            client.delete("$baseUrl/api/kv/delete/$key") {
                header("Authorization", "Bearer $token")
            }
        }
        
        if (!response.status.isSuccess() && response.status != HttpStatusCode.NotFound) {
            val errorBody = response.bodyAsText()
            throw Exception("KV delete failed with status ${response.status}: $errorBody")
        }
    }
    
    /**
     * Key-Value: Batch get multiple keys
     */
    suspend fun kvBatchGet(keys: List<String>): List<JsonElement> {
        val token = getToken()
        val body = buildJsonObject {
            put("keys", JsonArray(keys.map { JsonPrimitive(it) }))
        }
        
        val response = executeWithRetry {
            client.post("$baseUrl/api/kv/batch/get") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(body)
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("KV batch get failed with status ${response.status}: $errorBody")
        }
        
        return response.body<List<JsonElement>>()
    }
    
    /**
     * Key-Value: Batch set multiple key-value pairs.
     * Note: TTL from the first entry with a valid TTL is applied to all entries (server limitation).
     * @param entries List of Triple(key, value, ttl) - ttl from first entry applies to all
     */
    suspend fun kvBatchSet(entries: List<Triple<String, JsonElement, Int?>>): List<Pair<String, Boolean>> {
        val token = getToken()
        val keys = entries.map { it.first }
        val values = entries.map { buildJsonObject { put("value", it.second) } }
        // Server applies a single TTL to all entries - use first entry's TTL if provided
        val ttl = entries.firstOrNull()?.third
        
        val body = buildJsonObject {
            put("keys", JsonArray(keys.map { JsonPrimitive(it) }))
            put("values", JsonArray(values))
            ttl?.let { put("ttl", JsonPrimitive(it)) }
        }
        
        val response = executeWithRetry {
            client.post("$baseUrl/api/kv/batch/set") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(body)
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("KV batch set failed with status ${response.status}: $errorBody")
        }
        
        val result = response.body<List<JsonArray>>()
        return result.map { arr -> 
            Pair(arr[0].jsonPrimitive.content, arr[1].jsonPrimitive.boolean)
        }
    }
    
    /**
     * Key-Value: Batch delete multiple keys
     */
    suspend fun kvBatchDelete(keys: List<String>): List<Pair<String, Boolean>> {
        val token = getToken()
        val body = buildJsonObject {
            put("keys", JsonArray(keys.map { JsonPrimitive(it) }))
        }
        
        val response = executeWithRetry {
            client.delete("$baseUrl/api/kv/batch/delete") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(body)
            }
        }
        
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("KV batch delete failed with status ${response.status}: $errorBody")
        }
        
        val result = response.body<List<JsonArray>>()
        return result.map { arr -> 
            Pair(arr[0].jsonPrimitive.content, arr[1].jsonPrimitive.boolean)
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
            client.post("$baseUrl/api/kv/find") {
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
            client.post("$baseUrl/api/transactions") {
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
            client.get("$baseUrl/api/transactions/$transactionId") {
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
            client.post("$baseUrl/api/transactions/$transactionId/commit") {
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
            client.post("$baseUrl/api/transactions/$transactionId/rollback") {
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

                // Extract rate limit info from headers
                extractRateLimitInfo(response)

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

                // Throw on client errors (4xx) that weren't handled above
                if (response.status.value in 400..499) {
                    val errorBody = response.bodyAsText()
                    throw Exception("Request failed with status ${response.status.value}: $errorBody")
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
    /**
     * Get all built-in server-side chat tool definitions.
     * Returns a list of tool objects with name, description, and parameters fields.
     * Used by planning agents to discover available tools dynamically.
     */
    suspend fun getChatTools(): JsonArray {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/tools") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }

    suspend fun getChatModels(): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat_models") {
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
            client.get("$baseUrl/api/chat_models/$modelName") {
                header("Authorization", "Bearer $token")
            }
        }
        return response.body()
    }
    
    /**
     * Execute a tool via ekoDB's server-side tool pipeline.
     *
     * Calls POST /api/chat/tools/execute which goes through the same
     * execute_tool function as the LLM tool-calling loop — with all
     * collection filtering, permission enforcement, and internal collection
     * blocking. No LLM round-trip.
     *
     * @return The tool result if executed, or null if the server doesn't
     * support the endpoint (older ekoDB versions).
     */
    suspend fun executeTool(
        toolName: String,
        params: JsonObject,
        chatId: String? = null
    ): JsonObject? {
        val token = getToken()
        val body = buildJsonObject {
            put("tool", toolName)
            put("params", params)
            if (chatId != null) {
                put("chat_id", chatId)
            }
        }

        return try {
            val response = executeWithRetry {
                client.post("$baseUrl/api/chat/tools/execute") {
                    header("Authorization", "Bearer $token")
                    contentType(ContentType.Application.Json)
                    header("Accept", ContentType.Application.Json.toString())
                    setBody(body)
                }
            }
            val result: JsonObject = response.body()
            val success = result["success"]?.jsonPrimitive?.booleanOrNull ?: false
            if (success) {
                result["result"]?.jsonObject
            } else {
                val error = result["error"]?.jsonPrimitive?.contentOrNull ?: "tool execution failed"
                throw RuntimeException(error)
            }
        } catch (e: Exception) {
            if (e.message?.contains("404") == true || e.message?.contains("405") == true) {
                null // Server doesn't have the endpoint (404) or route mismatch (405)
            } else {
                throw e
            }
        }
    }

    /**
     * Create a new chat session
     */
    suspend fun createChatSession(request: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat") {
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
            client.get("$baseUrl/api/chat/$chatId") {
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
            client.get("$baseUrl/api/chat") {
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
            client.put("$baseUrl/api/chat/$chatId") {
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
            client.delete("$baseUrl/api/chat/$chatId") {
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
            client.post("$baseUrl/api/chat/branch") {
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
            client.post("$baseUrl/api/chat/merge") {
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
            client.post("$baseUrl/api/chat/$chatId/messages") {
                header("Authorization", "Bearer $token")
                contentType(getContentTypeForRequest())
                header("Accept", getContentTypeForRequest().toString())
                setBody(request)
            }
        }
        return response.body()
    }

    /**
     * Send a message in an existing chat session via SSE streaming.
     *
     * Returns a Flow that emits [ChatStreamEvent] objects as they arrive:
     * - [ChatStreamEvent.Chunk] for each token
     * - [ChatStreamEvent.End] when the response is complete
     * - [ChatStreamEvent.Error] on failure
     *
     * Preferred over [chatMessage] for long-running responses where reverse
     * proxies may kill idle HTTP connections before the LLM responds.
     *
     * ```kotlin
     * client.chatMessageStream(chatId, request).collect { event ->
     *     when (event) {
     *         is ChatStreamEvent.Chunk -> print(event.content)
     *         is ChatStreamEvent.End -> println("\nDone: ${event.messageId}")
     *         is ChatStreamEvent.Error -> System.err.println(event.error)
     *         else -> {}
     *     }
     * }
     * ```
     */
    fun chatMessageStream(chatId: String, request: JsonObject): Flow<ChatStreamEvent> = flow {
        val token = getToken()
        val response = client.post("$baseUrl/api/chat/$chatId/messages/stream") {
            header("Authorization", "Bearer $token")
            contentType(ContentType.Application.Json)
            header("Accept", ContentType.Text.EventStream.toString())
            setBody(request)
        }
        if (response.status != io.ktor.http.HttpStatusCode.OK) {
            val errBody = response.bodyAsText()
            emit(ChatStreamEvent.Error("SSE chat message stream failed (${response.status}): $errBody"))
            return@flow
        }
        val sseBody = response.bodyAsText()
        for (line in sseBody.lines()) {
            if (!line.startsWith("data:")) continue
            val dataStr = line.removePrefix("data:").trim()
            if (dataStr.isEmpty()) continue
            try {
                val eventData = Json.parseToJsonElement(dataStr).jsonObject
                val error = eventData["error"]?.jsonPrimitive?.content
                if (error != null) {
                    emit(ChatStreamEvent.Error(error))
                } else if (eventData.containsKey("message_id") && eventData.containsKey("content")) {
                    // Done event — has full content + message_id
                    emit(ChatStreamEvent.End(
                        messageId = eventData["message_id"]!!.jsonPrimitive.content,
                        tokenUsage = eventData["token_usage"],
                        executionTimeMs = eventData["execution_time_ms"]?.jsonPrimitive?.long ?: 0L,
                        contextWindow = eventData["context_window"]?.jsonPrimitive?.int,
                    ))
                } else {
                    eventData["token"]?.jsonPrimitive?.content?.let {
                        emit(ChatStreamEvent.Chunk(it))
                    }
                }
            } catch (_: Exception) {
                // skip malformed SSE data
            }
        }
    }

    /**
     * Get messages from a chat session
     */
    suspend fun getChatSessionMessages(chatId: String, query: JsonObject = buildJsonObject {}): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/$chatId/messages") {
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
            client.get("$baseUrl/api/chat/$chatId/messages/$messageId") {
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
            client.put("$baseUrl/api/chat/$chatId/messages/$messageId") {
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
            client.delete("$baseUrl/api/chat/$chatId/messages/$messageId") {
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
            client.post("$baseUrl/api/chat/$chatId/messages/$messageId/regenerate") {
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
            client.patch("$baseUrl/api/chat/$chatId/messages/$messageId/forgotten") {
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
        return WebSocketClient(wsUrl, token, client)
    }
    
    /**
     * Close the client and release resources
     */
    fun close() {
        client.close()
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
        private var httpClient: HttpClient? = null // For testing
        
        fun baseUrl(url: String) = apply { this.baseUrl = url }
        fun apiKey(key: String) = apply { this.apiKey = key }
        fun timeout(seconds: Long) = apply { this.timeout = seconds }
        fun maxRetries(retries: Int) = apply { this.maxRetries = retries }
        
        /**
         * Set serialization format (default: JSON for compatibility, MESSAGEPACK experimental)
         * MESSAGEPACK uses CBOR encoding which is faster but has compatibility issues with custom types
         */
        fun format(format: SerializationFormat) = apply { this.format = format }
        
        /**
         * Set a custom HTTP client (for testing with mock engines)
         */
        fun httpClient(client: HttpClient) = apply { this.httpClient = client }
        
        fun build(): EkoDBClient {
            require(apiKey.isNotEmpty()) { "API key is required" }
            return EkoDBClient(baseUrl, apiKey, timeout, maxRetries, format, httpClient)
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
            client.post("$baseUrl/api/functions") {
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
            client.get("$baseUrl/api/functions/$id") {
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
            client.get(url) {
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
            client.put("$baseUrl/api/functions/$id") {
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
            client.delete("$baseUrl/api/functions/$id") {
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
            client.post("$baseUrl/api/functions/$labelOrId") {
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
    
    // ========================================================================
    // USER FUNCTIONS API
    // ========================================================================

    /**
     * Save a new user function
     * User functions are reusable sequences of Functions that can be called by Scripts.
     * @param userFunction The user function definition as JSON
     * @return The ID of the created user function
     */
    suspend fun saveUserFunction(userFunction: JsonObject): String {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/functions") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(userFunction)
            }
        }

        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }

        val result = response.body<JsonObject>()
        return result["id"]?.jsonPrimitive?.content
            ?: throw IllegalStateException("No user function ID returned")
    }

    /**
     * Get a user function by label
     * @param label The user function label
     * @return The user function definition
     */
    suspend fun getUserFunction(label: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/functions/$label") {
                bearerAuth(token)
            }
        }

        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }

        return response.body()
    }

    /**
     * List all user functions, optionally filtered by tags
     * @param tags Optional list of tags to filter by
     * @return List of user functions
     */
    suspend fun listUserFunctions(tags: List<String>? = null): List<JsonObject> {
        val token = getToken()
        val url = if (tags != null && tags.isNotEmpty()) {
            "$baseUrl/api/functions?tags=${tags.joinToString(",")}"
        } else {
            "$baseUrl/api/functions"
        }
        val response = executeWithRetry {
            client.get(url) {
                bearerAuth(token)
            }
        }

        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }

        return response.body<List<JsonObject>>()
    }

    /**
     * Update an existing user function by label
     * @param label The user function label
     * @param userFunction The updated user function definition
     */
    suspend fun updateUserFunction(label: String, userFunction: JsonObject) {
        val token = getToken()
        val response = executeWithRetry {
            client.put("$baseUrl/api/functions/$label") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(userFunction)
            }
        }

        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
    }

    /**
     * Delete a user function by label
     * @param label The user function label
     */
    suspend fun deleteUserFunction(label: String) {
        val token = getToken()
        val response = executeWithRetry {
            client.delete("$baseUrl/api/functions/$label") {
                bearerAuth(token)
            }
        }

        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
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
        val response = embedRequest(buildJsonObject {
            put("text", text)
            put("model", model)
        })
        val embeddings = response["embeddings"]?.jsonArray
            ?: throw Exception("No embeddings in response")
        if (embeddings.isEmpty()) throw Exception("No embedding returned")
        return embeddings[0].jsonArray.map { it.jsonPrimitive.double }
    }

    /**
     * Generate embeddings for multiple texts in a single request
     *
     * @param texts List of texts to embed
     * @param model Embedding model name
     * @return List of embedding vectors
     *
     * ```kotlin
     * val embeddings = client.embedBatch(listOf("Hello", "World"), "text-embedding-3-small")
     * println("Generated ${embeddings.size} embeddings")
     * ```
     */
    suspend fun embedBatch(texts: List<String>, model: String): List<List<Double>> {
        val response = embedRequest(buildJsonObject {
            putJsonArray("texts") {
                texts.forEach { add(JsonPrimitive(it)) }
            }
            put("model", model)
        })
        val embeddings = response["embeddings"]?.jsonArray
            ?: throw Exception("No embeddings in response")
        return embeddings.map { arr -> arr.jsonArray.map { it.jsonPrimitive.double } }
    }

    private suspend fun embedRequest(body: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/embed") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(body)
            }
        }
        if (!response.status.isSuccess()) {
            val errorBody = response.bodyAsText()
            throw Exception("Embed request failed with status ${response.status}: $errorBody")
        }
        return response.body<JsonObject>()
    }
    
    /**
     * Perform full-text search on a collection
     * 
     * Searches documents using keyword matching with fuzzy matching and stemming.
     * 
     * @param collection Collection name to search
     * @param queryText Search query text
     * @param limit Maximum number of results to return
     * @return List of matching records
     * 
     * @example
     * ```kotlin
     * val results = client.textSearch("messages", "ownership system", 10)
     * ```
     */
    suspend fun textSearch(
        collection: String,
        queryText: String,
        limit: Int
    ): List<JsonObject> {
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
    
    // ========================================================================
    // ── Goal CRUD ──
    // ========================================================================

    /**
     * Create a new goal.
     *
     * @param data Goal definition as a JSON object
     * @return The created goal
     */
    suspend fun goalCreate(data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/goals") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * List all goals.
     *
     * @return A JSON object containing the list of goals
     */
    suspend fun goalList(): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/goals") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Get a goal by ID.
     *
     * @param id Goal ID
     * @return The goal object
     */
    suspend fun goalGet(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/goals/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Update a goal by ID.
     *
     * @param id Goal ID
     * @param data Updated goal fields as a JSON object
     * @return The updated goal
     */
    suspend fun goalUpdate(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.put("$baseUrl/api/chat/goals/$id") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Delete a goal by ID.
     *
     * @param id Goal ID
     */
    suspend fun goalDelete(id: String) {
        val token = getToken()
        val response = executeWithRetry {
            client.delete("$baseUrl/api/chat/goals/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
    }

    // ── Goal Templates ──

    /**
     * Create a new goal template.
     *
     * @param data Template definition as a JSON object
     * @return The created goal template
     */
    suspend fun goalTemplateCreate(data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/goal-templates") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * List all goal templates.
     *
     * @return A JSON object containing the list of goal templates
     */
    suspend fun goalTemplateList(): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/goal-templates") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Get a goal template by ID.
     *
     * @param id Goal template ID
     * @return The goal template object
     */
    suspend fun goalTemplateGet(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/goal-templates/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Update a goal template by ID.
     *
     * @param id Goal template ID
     * @param data Updated template fields as a JSON object
     * @return The updated goal template
     */
    suspend fun goalTemplateUpdate(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.put("$baseUrl/api/chat/goal-templates/$id") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Delete a goal template by ID.
     *
     * @param id Goal template ID
     */
    suspend fun goalTemplateDelete(id: String) {
        val token = getToken()
        val response = executeWithRetry {
            client.delete("$baseUrl/api/chat/goal-templates/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
    }

    /**
     * Search goals by query string.
     *
     * @param query Search query
     * @return Matching goals
     */
    suspend fun goalSearch(query: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/goals/search") {
                bearerAuth(token)
                parameter("q", query)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    // ── Goal Lifecycle ──

    /**
     * Mark a goal as complete.
     *
     * @param id Goal ID
     * @param data Completion details (e.g., outcome summary)
     * @return The updated goal
     */
    suspend fun goalComplete(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/goals/$id/complete") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Approve a completed goal.
     *
     * @param id Goal ID
     * @return The updated goal
     */
    suspend fun goalApprove(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/goals/$id/approve") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Reject a completed goal.
     *
     * @param id Goal ID
     * @param data Rejection details (e.g., reason)
     * @return The updated goal
     */
    suspend fun goalReject(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/goals/$id/reject") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    // ── Goal Step Lifecycle ──

    /**
     * Start a goal step.
     *
     * @param id Goal ID
     * @param stepIndex Zero-based step index
     * @return The updated goal
     */
    suspend fun goalStepStart(id: String, stepIndex: Int): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/goals/$id/steps/$stepIndex/start") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Complete a goal step.
     *
     * @param id Goal ID
     * @param stepIndex Zero-based step index
     * @param data Completion details for the step
     * @return The updated goal
     */
    suspend fun goalStepComplete(id: String, stepIndex: Int, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/goals/$id/steps/$stepIndex/complete") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Fail a goal step.
     *
     * @param id Goal ID
     * @param stepIndex Zero-based step index
     * @param data Failure details for the step
     * @return The updated goal
     */
    suspend fun goalStepFail(id: String, stepIndex: Int, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/goals/$id/steps/$stepIndex/fail") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    // ========================================================================
    // ── Task CRUD ──
    // ========================================================================

    /**
     * Create a new task.
     *
     * @param data Task definition as a JSON object
     * @return The created task
     */
    suspend fun taskCreate(data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/tasks") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * List all tasks.
     *
     * @return A JSON object containing the list of tasks
     */
    suspend fun taskList(): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/tasks") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Get a task by ID.
     *
     * @param id Task ID
     * @return The task object
     */
    suspend fun taskGet(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/tasks/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Update a task by ID.
     *
     * @param id Task ID
     * @param data Updated task fields as a JSON object
     * @return The updated task
     */
    suspend fun taskUpdate(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.put("$baseUrl/api/chat/tasks/$id") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Delete a task by ID.
     *
     * @param id Task ID
     */
    suspend fun taskDelete(id: String) {
        val token = getToken()
        val response = executeWithRetry {
            client.delete("$baseUrl/api/chat/tasks/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
    }

    /**
     * Get tasks that are due.
     *
     * @param now ISO-8601 timestamp representing the current time
     * @return Tasks that are due at or before the given time
     */
    suspend fun taskDue(now: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/tasks/due") {
                bearerAuth(token)
                parameter("now", now)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    // ── Task Lifecycle ──

    /**
     * Start a task.
     *
     * @param id Task ID
     * @return The updated task
     */
    suspend fun taskStart(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/tasks/$id/start") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Mark a task as succeeded.
     *
     * @param id Task ID
     * @param data Success details (e.g., output summary)
     * @return The updated task
     */
    suspend fun taskSucceed(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/tasks/$id/succeed") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Mark a task as failed.
     *
     * @param id Task ID
     * @param data Failure details (e.g., error message)
     * @return The updated task
     */
    suspend fun taskFail(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/tasks/$id/fail") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Pause a running task.
     *
     * @param id Task ID
     * @return The updated task
     */
    suspend fun taskPause(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/tasks/$id/pause") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Resume a paused task.
     *
     * @param id Task ID
     * @param data Resume details
     * @return The updated task
     */
    suspend fun taskResume(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/tasks/$id/resume") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    // ========================================================================
    // ── Agent CRUD ──
    // ========================================================================

    /**
     * Create a new agent.
     *
     * @param data Agent definition as a JSON object
     * @return The created agent
     */
    suspend fun agentCreate(data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/chat/agents") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * List all agents.
     *
     * @return A JSON object containing the list of agents
     */
    suspend fun agentList(): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/agents") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Get an agent by ID.
     *
     * @param id Agent ID
     * @return The agent object
     */
    suspend fun agentGet(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/agents/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Get an agent by name.
     *
     * @param name Agent name
     * @return The agent object
     */
    suspend fun agentGetByName(name: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/agents/by-name/$name") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Update an agent by ID.
     *
     * @param id Agent ID
     * @param data Updated agent fields as a JSON object
     * @return The updated agent
     */
    suspend fun agentUpdate(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.put("$baseUrl/api/chat/agents/$id") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /**
     * Delete an agent by ID.
     *
     * @param id Agent ID
     */
    suspend fun agentDelete(id: String) {
        val token = getToken()
        val response = executeWithRetry {
            client.delete("$baseUrl/api/chat/agents/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
    }

    /**
     * List agents by deployment ID.
     *
     * @param deploymentId Deployment ID
     * @return Agents associated with the given deployment
     */
    suspend fun agentsByDeployment(deploymentId: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/chat/agents/by-deployment/$deploymentId") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    // ── KV Document Linking ──────────────────────────────────────────────────

    /** Get documents linked to a KV key */
    suspend fun kvGetLinks(key: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/kv/links/$key") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /** Link a document to a KV key */
    suspend fun kvLink(key: String, collection: String, documentId: String): JsonObject {
        val token = getToken()
        val body = buildJsonObject {
            put("key", key)
            put("collection", collection)
            put("document_id", documentId)
        }
        val response = executeWithRetry {
            client.post("$baseUrl/api/kv/link") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(body)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /** Unlink a document from a KV key */
    suspend fun kvUnlink(key: String, collection: String, documentId: String): JsonObject {
        val token = getToken()
        val body = buildJsonObject {
            put("key", key)
            put("collection", collection)
            put("document_id", documentId)
        }
        val response = executeWithRetry {
            client.post("$baseUrl/api/kv/unlink") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(body)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    // ── Schedule Management ──────────────────────────────────────────────────

    /** Create a new schedule */
    suspend fun createSchedule(data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/schedules") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /** List all schedules */
    suspend fun listSchedules(): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/schedules") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /** Get a schedule by ID */
    suspend fun getSchedule(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.get("$baseUrl/api/schedules/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /** Update a schedule */
    suspend fun updateSchedule(id: String, data: JsonObject): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.put("$baseUrl/api/schedules/$id") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
                setBody(data)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /** Delete a schedule */
    suspend fun deleteSchedule(id: String) {
        val token = getToken()
        val response = executeWithRetry {
            client.delete("$baseUrl/api/schedules/$id") {
                bearerAuth(token)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
    }

    /** Pause a schedule */
    suspend fun pauseSchedule(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/schedules/$id/pause") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
    }

    /** Resume a schedule */
    suspend fun resumeSchedule(id: String): JsonObject {
        val token = getToken()
        val response = executeWithRetry {
            client.post("$baseUrl/api/schedules/$id/resume") {
                bearerAuth(token)
                contentType(ContentType.Application.Json)
            }
        }
        if (response.status.value >= 400) {
            val errorText = response.bodyAsText()
            throw IllegalStateException("Server error ${response.status.value}: $errorText")
        }
        return response.body<JsonObject>()
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

/**
 * Rate limit information from the server.
 * Extracted from X-RateLimit-* response headers.
 */
data class RateLimitInfo(
    /** Maximum requests allowed per window */
    val limit: Int,
    /** Remaining requests in current window */
    val remaining: Int,
    /** Unix timestamp when the limit resets */
    val reset: Long
) {
    /** True if less than 10% of the rate limit quota remains. */
    fun isNearLimit(): Boolean {
        val threshold = (limit * 0.1).toInt()
        return remaining <= threshold
    }

    /** True if the rate limit has been exceeded. */
    fun isExceeded(): Boolean = remaining == 0

    /** Percentage of rate limit quota remaining (0.0 - 100.0). */
    fun remainingPercentage(): Double {
        if (limit == 0) return 0.0
        return (remaining.toDouble() / limit.toDouble()) * 100.0
    }
}
