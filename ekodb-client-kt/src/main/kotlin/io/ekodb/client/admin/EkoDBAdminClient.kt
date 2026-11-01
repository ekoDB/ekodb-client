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

const val MIN_SERVER_VERSION_FOR_PERMISSION_EDITS = "0.80.0"
const val MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS = "0.79.2"
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
data class LegacyUpdateApiKeyCollectionPermissionsRequest(
    val apiKey: String,
    val collection: String,
    val read: Boolean,
    val write: Boolean,
    val selectFields: PermissionEdit = PermissionEdit.Keep,
    val excludeFields: PermissionEdit = PermissionEdit.Keep,
    val writableFields: PermissionEdit = PermissionEdit.Keep,
)
data class UpdateApiKeyCollectionPermissionsRequest(
    val keyId: String,
    val collection: String,
    val read: Boolean,
    val write: Boolean,
    val selectFields: PermissionEdit = PermissionEdit.Keep,
    val excludeFields: PermissionEdit = PermissionEdit.Keep,
    val writableFields: PermissionEdit = PermissionEdit.Keep,
)
data class PatchApiKeyCollectionPermissionsRequest(
    val keyId: String,
    val collection: String,
    val read: Boolean? = null,
    val write: Boolean? = null,
    val selectFields: PermissionEdit = PermissionEdit.Keep,
    val excludeFields: PermissionEdit = PermissionEdit.Keep,
    val writableFields: PermissionEdit = PermissionEdit.Keep,
)
data class UpdateApiKeyKvPermissionsRequest(val keyId: String, val prefix: String, val read: Boolean, val write: Boolean)
data class PatchApiKeyKvPermissionsRequest(val keyId: String, val prefix: String, val read: Boolean? = null, val write: Boolean? = null)
data class KvPermissions(val read: Boolean, val write: Boolean, val prefix: String? = null, val metadata: JsonObject = JsonObject(emptyMap()))
data class PermissionChange<T>(val before: T?, val after: T?, val metadata: JsonObject = JsonObject(emptyMap()))
typealias CollectionPermissionChange = PermissionChange<CollectionPermissions>
typealias KvPermissionChange = PermissionChange<KvPermissions>
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
    val kvPermissions: List<KvPermissions> = emptyList(),
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
    val id: String? = null,
    val kvPermissions: List<KvPermissions>? = null,
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
    private val permissionMutex = Mutex()
    private var permissionRoutes: Boolean? = null
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
    suspend fun usesKeyPermissionRoutes(): Boolean = permissionMutex.withLock {
        permissionRoutes?.let { return@withLock it }
        val modern = try {
            parsePermissions(transport.adminRequest(HttpMethod.Get, "/api/auth/keys/me/permissions"), true)
            true
        } catch (error: IllegalStateException) {
            if (error.message in listOf("Admin request failed (HTTP 404)", "Admin request failed (HTTP 405)")) false else throw error
        }
        permissionRoutes = modern
        modern
    }
    suspend fun getPermissions(): ApiKeyPermissions = getApiKeyPermissions("me")
    suspend fun getApiKeyPermissions(keyId: String): ApiKeyPermissions {
        validateKeyId(keyId)
        val modern = usesKeyPermissionRoutes()
        require(modern || keyId == "me") { "Reading another key by id needs modern permission routes" }
        val path = if (modern) "/api/auth/keys/$keyId/permissions" else "/api/auth/permissions"
        return parsePermissions(transport.adminRequest(HttpMethod.Get, path), modern)
    }
    private suspend fun permissionPath(keyId: String, facet: String, name: String? = null): String {
        validateKeyId(keyId)
        name?.let { validateFacet(it) }
        require(usesKeyPermissionRoutes()) { "This operation needs modern permission routes" }
        return "/api/auth/keys/$keyId/permissions/$facet" + (name?.let { "/${encodeSegment(it)}" } ?: "")
    }
    suspend fun listApiKeyCollectionPermissions(keyId: String): Map<String, CollectionPermissions> =
        grants(transport.adminRequest(HttpMethod.Get, permissionPath(keyId, "collections")))
    suspend fun getApiKeyCollectionPermissions(keyId: String, collection: String): CollectionPermissions =
        parseGrant(transport.adminRequest(HttpMethod.Get, permissionPath(keyId, "collections", collection)))
    suspend fun listApiKeyKvPermissions(keyId: String, prefix: String? = null): List<KvPermissions> {
        val path = permissionPath(keyId, "kv") + (prefix?.let { "?prefix=${encodeSegment(it)}" } ?: "")
        return parseKvList(transport.adminRequest(HttpMethod.Get, path))
    }
    suspend fun getApiKeyKvPermissions(keyId: String, prefix: String): KvPermissions =
        parseKv(transport.adminRequest(HttpMethod.Get, permissionPath(keyId, "kv", prefix)))
    private suspend fun collectionChange(method: HttpMethod, keyId: String, collection: String, body: JsonObject?): CollectionPermissionChange {
        val result = transport.adminRequest(method, permissionPath(keyId, "collections", collection), body, readDeleteBody = true)
        return parseChange(result, ::parseGrant)
    }
    suspend fun putApiKeyCollectionPermissions(request: UpdateApiKeyCollectionPermissionsRequest): CollectionPermissionChange =
        collectionChange(HttpMethod.Put, request.keyId, request.collection, grantBody(request.read, request.write, request.selectFields, request.excludeFields, request.writableFields))
    suspend fun updateApiKeyCollectionPermissions(request: UpdateApiKeyCollectionPermissionsRequest): CollectionPermissionChange =
        collectionChange(HttpMethod.Patch, request.keyId, request.collection, grantBody(request.read, request.write, request.selectFields, request.excludeFields, request.writableFields))
    suspend fun patchApiKeyCollectionPermissions(request: PatchApiKeyCollectionPermissionsRequest): CollectionPermissionChange =
        collectionChange(HttpMethod.Patch, request.keyId, request.collection, grantBody(request.read, request.write, request.selectFields, request.excludeFields, request.writableFields))
    suspend fun deleteApiKeyCollectionPermissions(keyId: String, collection: String): CollectionPermissionChange =
        collectionChange(HttpMethod.Delete, keyId, collection, null)
    private suspend fun kvChange(method: HttpMethod, keyId: String, prefix: String, read: Boolean?, write: Boolean?): KvPermissionChange {
        val body = if (method == HttpMethod.Delete) {
            null
        } else {
            buildJsonObject {
                read?.let { put("read", it) }; write?.let { put("write", it) }
            }
        }
        return parseChange(transport.adminRequest(method, permissionPath(keyId, "kv", prefix), body, readDeleteBody = true), ::parseKv)
    }
    suspend fun putApiKeyKvPermissions(request: UpdateApiKeyKvPermissionsRequest): KvPermissionChange {
        require(request.read || request.write) { "A KV grant needs read or write permission" }
        return kvChange(HttpMethod.Put, request.keyId, request.prefix, request.read, request.write)
    }
    suspend fun patchApiKeyKvPermissions(request: PatchApiKeyKvPermissionsRequest): KvPermissionChange =
        kvChange(HttpMethod.Patch, request.keyId, request.prefix, request.read, request.write)
    suspend fun deleteApiKeyKvPermissions(keyId: String, prefix: String): KvPermissionChange =
        kvChange(HttpMethod.Delete, keyId, prefix, null, null)
    suspend fun legacyUpdateApiKeyCollectionPermissions(request: LegacyUpdateApiKeyCollectionPermissionsRequest): ApiKey {
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
        require(!usesKeyPermissionRoutes()) { "Legacy permission edits are unavailable on this server" }
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
    val minimum = MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS.split(".").map { it.toLongOrNull() ?: return false }
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
    val id = text(record["id"])
    if (id != null && !Regex("[0-9a-f]{32}").matches(id)) invalid()
    return ApiKey(key, boolean(record["is_admin"]), grants(record["collection_permissions"]), text(record["label"]), text(record["description"]), text(record["ripple_warning"]), JsonObject(record.filterKeys { it != "key" }), id, record["kv_permissions"]?.takeUnless { it == JsonNull }?.let(::parseKvList))
}

private fun validateKeyId(keyId: String) {
    require(keyId == "me" || Regex("[0-9a-f]{32}").matches(keyId)) { "A key id or me is required; key values are not accepted" }
}
private fun validateFacet(value: String) {
    require(value.isNotEmpty() && value != "." && value != ".." && value.none { Character.isISOControl(it) || Character.getType(it) == Character.FORMAT.toInt() }) { "Invalid permission facet" }
}
private fun encodeSegment(value: String): String = value.toByteArray(Charsets.UTF_8).joinToString("") { byte ->
    val c = byte.toInt() and 255
    if (c in 65..90 || c in 97..122 || c in 48..57 || c.toChar() in "-._~") c.toChar().toString() else "%%%02X".format(c)
}
private fun grantBody(read: Boolean?, write: Boolean?, selectFields: PermissionEdit, excludeFields: PermissionEdit, writableFields: PermissionEdit): JsonObject = buildJsonObject {
    read?.let { put("read", it) }; write?.let { put("write", it) }
    listOf("select_fields" to selectFields, "exclude_fields" to excludeFields, "writable_fields" to writableFields).forEach { (name, edit) ->
        when (edit) {
            PermissionEdit.Keep -> Unit
            PermissionEdit.Clear -> put(name, JsonNull)
            is PermissionEdit.Set -> {
                require(edit.fields.all { it.isNotBlank() }) { "Invalid collection permission update" }
                put(name, JsonArray(edit.fields.map(::JsonPrimitive)))
            }
        }
    }
}
private fun parseGrant(value: JsonElement): CollectionPermissions = grants(buildJsonObject { put("grant", value) }).getValue("grant")
private fun parseKv(value: JsonElement): KvPermissions {
    val record = value as? JsonObject ?: invalid()
    return KvPermissions(boolean(record["read"]), boolean(record["write"]), text(record["prefix"]), record)
}
private fun parseKvList(value: JsonElement): List<KvPermissions> = (value as? JsonArray ?: invalid()).map {
    parseKv(it).also { grant -> if (grant.prefix.isNullOrEmpty()) invalid() }
}
private fun parsePermissions(value: JsonElement, modern: Boolean): ApiKeyPermissions {
    val record = value as? JsonObject ?: invalid()
    return ApiKeyPermissions(boolean(record[if (modern) "admin" else "is_admin"]), grants(record[if (modern) "collections" else "collection_permissions"]), record,
        record[if (modern) "kv" else "kv_permissions"]?.takeUnless { !modern && it == JsonNull }?.let(::parseKvList) ?: if (modern) invalid() else emptyList())
}
private fun <T> parseChange(value: JsonElement, parse: (JsonElement) -> T): PermissionChange<T> {
    val record = value as? JsonObject ?: invalid()
    if (!record.containsKey("before") || !record.containsKey("after")) invalid()
    return PermissionChange(record["before"]!!.takeUnless { it == JsonNull }?.let(parse), record["after"]!!.takeUnless { it == JsonNull }?.let(parse), JsonObject(record.filterKeys { it != "before" && it != "after" }))
}
