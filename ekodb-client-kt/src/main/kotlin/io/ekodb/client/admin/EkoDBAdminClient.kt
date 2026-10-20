package io.ekodb.client.admin

import io.ekodb.client.EkoDBClient
import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.HttpRequestRetry
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.plugins.logging.LogLevel
import io.ktor.client.plugins.logging.Logging
import io.ktor.http.HttpMethod
import io.ktor.serialization.kotlinx.json.json
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.json.*

const val MIN_SERVER_VERSION_FOR_PERMISSION_EDITS = "0.79.2"
private const val REQUIREMENT = "Permission updates need ekoDB 0.79.2 or later because earlier servers remove a grant's field restrictions when it is updated"

sealed interface PermissionEdit {
    data object Keep : PermissionEdit
    data object Clear : PermissionEdit
    data class Set(val fields: List<String>) : PermissionEdit
}
enum class ApiKeyAction { Read, Write }
data class CreateApiKeyRequest(
    val label: String? = null,
    val description: String? = null,
    val isAdmin: Boolean = false,
    val collectionPermissions: Map<String, List<ApiKeyAction>> = emptyMap(),
)
data class UpdateApiKeyCollectionPermissionsRequest(
    val apiKey: String,
    val collection: String,
    val read: Boolean,
    val write: Boolean,
    val selectFields: PermissionEdit = PermissionEdit.Keep,
    val excludeFields: PermissionEdit = PermissionEdit.Keep,
    val writableFields: PermissionEdit = PermissionEdit.Keep,
)
data class CollectionPermissions(
    val read: Boolean,
    val write: Boolean,
    val selectFields: List<String>?,
    val excludeFields: List<String>?,
    val writableFields: List<String>?,
    val metadata: JsonObject,
)
class ApiKeyPermissions internal constructor(
    val isAdmin: Boolean,
    val collectionPermissions: Map<String, CollectionPermissions>,
    val metadata: JsonObject,
)

/** Contains a secret: never log the key or the returned record. */
class ApiKey internal constructor(
    val key: String,
    val isAdmin: Boolean,
    val collectionPermissions: Map<String, CollectionPermissions>,
    val label: String?,
    val description: String?,
    val rippleWarning: String?,
    val metadata: JsonObject,
)

class EkoDBAdminClient(
    baseUrl: String,
    private val apiKey: String,
    timeoutSeconds: Long = 30,
    httpClient: HttpClient? = null,
) : AutoCloseable {
    private val ownsBaseClient = httpClient == null
    private val baseClient = httpClient ?: HttpClient(CIO)
    private val secureClient = baseClient.config {
        followRedirects = false
        install(HttpRequestRetry) { noRetry() }
        install(ContentNegotiation) { json() }
        install(Logging) { level = LogLevel.NONE }
    }
    private val transport = EkoDBClient.builder().baseUrl(baseUrl.trimEnd('/')).apiKey(apiKey)
        .timeout(timeoutSeconds).httpClient(secureClient).build()
    private val versionMutex = Mutex()
    private var versionRead = false
    private var version: String? = null

    suspend fun createApiKey(request: CreateApiKeyRequest = CreateApiKeyRequest()): ApiKey {
        val body = buildJsonObject {
            put("api_key", apiKey)
            request.label?.let { put("label", it) }
            request.description?.let { put("description", it) }
            put("is_admin", request.isAdmin)
            put("collection_permissions", buildJsonObject {
                request.collectionPermissions.forEach { (name, actions) ->
                    put(name, JsonArray(actions.map { JsonPrimitive(it.name.lowercase()) }))
                }
            })
        }
        return parseKey(transport.adminRequest(HttpMethod.Post, "/api/auth/register", body))
    }
    suspend fun listApiKeys(): List<ApiKey> {
        val result = transport.adminRequest(HttpMethod.Get, "/api/auth/keys") as? JsonArray ?: invalid()
        return result.map(::parseKey)
    }
    suspend fun revokeApiKey(key: String) {
        require(key.isNotBlank()) { "An API key is required" }
        val encoded = key.toByteArray(Charsets.UTF_8).joinToString("") { byte ->
            val c = byte.toInt() and 255
            if (c in 65..90 || c in 97..122 || c in 48..57 || c.toChar() in "-._~") c.toChar().toString() else "%%%02X".format(c)
        }
        transport.adminRequest(HttpMethod.Delete, "/api/auth/keys/$encoded")
    }
    suspend fun getPermissions(): ApiKeyPermissions {
        val record = transport.adminRequest(HttpMethod.Get, "/api/auth/permissions") as? JsonObject ?: invalid()
        return ApiKeyPermissions(boolean(record["is_admin"]), grants(record["collection_permissions"]), record)
    }
    suspend fun updateApiKeyCollectionPermissions(request: UpdateApiKeyCollectionPermissionsRequest): ApiKey {
        require(request.apiKey.isNotBlank() && request.collection.isNotBlank()) { "Invalid collection permission update" }
        val body = buildJsonObject {
            put("api_key", request.apiKey); put("collection", request.collection)
            put("read", request.read); put("write", request.write)
            listOf("select_fields" to request.selectFields, "exclude_fields" to request.excludeFields, "writable_fields" to request.writableFields).forEach { (field, edit) ->
                when (edit) {
                    PermissionEdit.Keep -> Unit
                    PermissionEdit.Clear -> put(field, JsonNull)
                    is PermissionEdit.Set -> {
                        require(edit.fields.all { it.isNotBlank() }) { "Invalid collection permission update" }
                        put(field, JsonArray(edit.fields.map(::JsonPrimitive)))
                    }
                }
            }
        }
        versionMutex.withLock {
            if (!versionRead) {
                version = try {
                    val health = transport.adminRequest(HttpMethod.Get, "/api/health") as? JsonObject
                    (health?.get("version") as? JsonPrimitive)?.takeIf { it.isString }?.content
                } catch (error: kotlinx.coroutines.CancellationException) {
                    throw error
                } catch (_: Exception) {
                    null
                }
                versionRead = true
            }
            require(supportedVersion(version)) { REQUIREMENT }
        }
        return parseKey(transport.adminRequest(HttpMethod.Put, "/api/auth/permissions", body))
    }
    override fun close() { transport.close(); secureClient.close(); if (ownsBaseClient) baseClient.close() }
}
private fun supportedVersion(version: String?): Boolean {
    val match = Regex("^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)(?:\\+[0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*)?$").matchEntire(version ?: return false) ?: return false
    val parts = match.groupValues.drop(1).map { it.toLongOrNull() ?: return false }
    val minimum = MIN_SERVER_VERSION_FOR_PERMISSION_EDITS.split(".").map { it.toLongOrNull() ?: return false }
    for (i in parts.indices) { if (parts[i] > minimum[i]) return true; if (parts[i] < minimum[i]) return false }
    return true
}
private fun invalid(): Nothing = throw IllegalStateException("Invalid admin response")
private fun boolean(value: JsonElement?): Boolean = (value as? JsonPrimitive)?.takeIf { !it.isString }?.booleanOrNull ?: invalid()
private fun text(value: JsonElement?): String? {
    if (value == null || value == JsonNull) return null
    return (value as? JsonPrimitive)?.takeIf { it.isString }?.content ?: invalid()
}
private fun fields(value: JsonElement?): List<String>? {
    if (value == null || value == JsonNull) return null
    return (value as? JsonArray)?.map { text(it) ?: invalid() } ?: invalid()
}
private fun grants(value: JsonElement?): Map<String, CollectionPermissions> = (value as? JsonObject ?: invalid()).mapValues { (_, grant) ->
    if (grant is JsonArray) {
        val actions = grant.map { text(it) ?: invalid() }
        if (actions.any { it != "read" && it != "write" }) invalid()
        CollectionPermissions("read" in actions, "write" in actions, null, null, null, JsonObject(emptyMap()))
    } else {
        val record = grant as? JsonObject ?: invalid()
        CollectionPermissions(boolean(record["read"]), boolean(record["write"]), fields(record["select_fields"]), fields(record["exclude_fields"]), fields(record["writable_fields"]), record)
    }
}
private fun parseKey(value: JsonElement): ApiKey {
    val record = value as? JsonObject ?: invalid()
    val key = text(record["key"])?.takeIf { it.isNotEmpty() } ?: invalid()
    return ApiKey(key, boolean(record["is_admin"]), grants(record["collection_permissions"]), text(record["label"]), text(record["description"]), text(record["ripple_warning"]), JsonObject(record.filterKeys { it != "key" }))
}
