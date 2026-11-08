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

/** The longest function grant pattern the server accepts, in bytes. */
const val MAX_FUNCTION_PATTERN_BYTES = 256
private const val REQUIREMENT = "Permission updates need ekoDB 0.79.2 or later because earlier servers remove a grant's field restrictions when it is updated"

sealed interface PermissionEdit {
    data object Keep : PermissionEdit
    data object Clear : PermissionEdit
    data class Set(val fields: List<String>) : PermissionEdit
}
enum class ApiKeyAction { Read, Write }

/** The one right a function grant carries. */
enum class FunctionAction { Call }
data class CreateApiKeyRequest(
    val label: String? = null,
    val description: String? = null,
    val isAdmin: Boolean = false,
    val collectionPermissions: Map<String, List<ApiKeyAction>> = emptyMap(),
    /**
     * Stored-function call grants: a function label, a label prefix ending in
     * one `*`, or `*`, each with `[FunctionAction.Call]`. A pattern given an
     * empty list grants nothing. Omitted from the request when empty.
     */
    val functionPermissions: Map<String, List<FunctionAction>> = emptyMap(),
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

/**
 * A server-returned stored-function call grant. [pattern] is a function label, a label
 * prefix ending in one `*`, or `*`; holding the grant is the right to call.
 */
data class FunctionPermissions(val pattern: String, val metadata: JsonObject = JsonObject(emptyMap()))
data class PermissionChange<T>(val before: T?, val after: T?, val metadata: JsonObject = JsonObject(emptyMap()))
typealias CollectionPermissionChange = PermissionChange<CollectionPermissions>
typealias KvPermissionChange = PermissionChange<KvPermissions>
typealias FunctionPermissionChange = PermissionChange<FunctionPermissions>
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
    /** Empty when the key holds no function grant, and when the server predates function grants. */
    val functionPermissions: List<FunctionPermissions> = emptyList(),
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
    /**
     * Null when the response carries none. Registration echoes the request map and key
     * listings return grants; both read as grants sorted by pattern, and a pattern registered
     * with no rights grants nothing.
     */
    val functionPermissions: List<FunctionPermissions>? = null,
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
        request.functionPermissions.keys.forEach(::validateFunctionPattern)
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
            if (request.functionPermissions.isNotEmpty()) {
                put("function_permissions", buildJsonObject {
                    request.functionPermissions.forEach { (pattern, rights) ->
                        put(pattern, JsonArray(rights.map { JsonPrimitive(it.name.lowercase()) }))
                    }
                })
            }
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
    suspend fun listApiKeyFunctionPermissions(keyId: String): List<FunctionPermissions> =
        parseFunctionList(transport.adminRequest(HttpMethod.Get, permissionPath(keyId, "functions")))
    suspend fun getApiKeyFunctionPermissions(keyId: String, pattern: String): FunctionPermissions {
        validateFunctionPattern(pattern)
        return parseFunction(transport.adminRequest(HttpMethod.Get, permissionPath(keyId, "functions", pattern)))
    }
    private suspend fun functionChange(method: HttpMethod, keyId: String, pattern: String, body: JsonObject?): FunctionPermissionChange {
        validateFunctionPattern(pattern)
        val result = transport.adminRequest(method, permissionPath(keyId, "functions", pattern), body, readDeleteBody = true)
        return parseChange(result, ::parseFunction)
    }

    /** Gives the key the right to call the functions [pattern] covers. */
    suspend fun putApiKeyFunctionPermissions(keyId: String, pattern: String): FunctionPermissionChange =
        functionChange(HttpMethod.Put, keyId, pattern, buildJsonObject { put("call", true) })

    /** Removes the key's grant on exactly [pattern]. */
    suspend fun deleteApiKeyFunctionPermissions(keyId: String, pattern: String): FunctionPermissionChange =
        functionChange(HttpMethod.Delete, keyId, pattern, null)
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
                val health = transport.adminRequest(HttpMethod.Get, "/api/health") as? JsonObject
                version = (health?.get("version") as? JsonPrimitive)?.takeIf { it.isString }?.content
                versionRead = versionParts(version) != null
            }
            require(supportedVersion(version)) { REQUIREMENT }
        }
        return parseKey(transport.adminRequest(HttpMethod.Put, "/api/auth/permissions", body))
    }
    override fun close() { transport.close(); secureClient.close(); if (ownsBaseClient) baseClient.close() }
}
private fun versionParts(version: String?): List<Long>? {
    val match = Regex("^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)(?:\\+[0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*)?$").matchEntire(version ?: return null) ?: return null
    return match.groupValues.drop(1).map { it.toLongOrNull() ?: return null }
}
private fun supportedVersion(version: String?): Boolean {
    val parts = versionParts(version) ?: return false
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
    return ApiKey(key, boolean(record["is_admin"]), grants(record["collection_permissions"]), text(record["label"]), text(record["description"]), text(record["ripple_warning"]), JsonObject(record.filterKeys { it != "key" }), id, record["kv_permissions"]?.takeUnless { it == JsonNull }?.let(::parseKvList), record["function_permissions"]?.takeUnless { it == JsonNull }?.let(::parseFunctionGrants))
}

private fun validateKeyId(keyId: String) {
    require(keyId == "me" || Regex("[0-9a-f]{32}").matches(keyId)) { "A key id or me is required; key values are not accepted" }
}
private fun validateFacet(value: String) {
    require(value.isNotEmpty() && value != "." && value != ".." && value.codePoints().noneMatch {
        it in 0xD800..0xDFFF || Character.getType(it) == Character.CONTROL.toInt() ||
            it == 0x00AD || it == 0x061C || it == 0x180E ||
            it in 0x200B..0x200F || it in 0x2028..0x202E ||
            it in 0x2060..0x2064 || it in 0x2066..0x2069 || it == 0xFEFF
    }) { "Invalid permission facet" }
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
private fun parseFunction(value: JsonElement): FunctionPermissions {
    val record = value as? JsonObject ?: invalid()
    return FunctionPermissions(text(record["pattern"])?.takeIf { it.isNotEmpty() } ?: invalid(), record)
}
private fun parseFunctionList(value: JsonElement): List<FunctionPermissions> = (value as? JsonArray ?: invalid()).map(::parseFunction)

/**
 * Unicode code-point order, which is the UTF-8 byte order Rust and Go sort by. [String.compareTo]
 * orders UTF-16 code units instead, and puts a character outside the Basic Multilingual Plane
 * before U+E000..U+FFFF.
 */
private val byCodePoint = Comparator<String> { a, b ->
    val x = a.codePoints().toArray()
    val y = b.codePoints().toArray()
    val index = x.indices.firstOrNull { it < y.size && x[it] != y[it] }
    if (index != null) x[index].compareTo(y[index]) else x.size.compareTo(y.size)
}

/**
 * Reads the grants list, or the registration map `pattern -> ["call"]` (empty rights grant nothing).
 * An empty pattern is refused in either form, and both forms read as grants sorted by pattern in
 * code-point order.
 */
private fun parseFunctionGrants(value: JsonElement): List<FunctionPermissions> = when (value) {
    is JsonArray -> parseFunctionList(value).sortedWith(compareBy(byCodePoint) { it.pattern })
    is JsonObject -> value.entries.filter { (pattern, rights) ->
        if (pattern.isEmpty()) invalid()
        val list = rights as? JsonArray ?: invalid()
        if (list.any { text(it) != "call" }) invalid()
        list.isNotEmpty()
    }.map { it.key }.sortedWith(byCodePoint).map { FunctionPermissions(it) }
    else -> invalid()
}

/**
 * The pattern rules the server enforces, checked before any request: a label, a label prefix
 * ending in one `*`, or `*`; non-empty, at most [MAX_FUNCTION_PATTERN_BYTES] bytes, no control
 * or invisible formatting character, and `*` nowhere but last.
 */
private fun validateFunctionPattern(pattern: String) {
    val wellFormed = runCatching { validateFacet(pattern) }.isSuccess
    require(wellFormed && '*' !in pattern.removeSuffix("*") && pattern.toByteArray(Charsets.UTF_8).size <= MAX_FUNCTION_PATTERN_BYTES) {
        "Invalid function pattern"
    }
}
private fun parseKvList(value: JsonElement): List<KvPermissions> = (value as? JsonArray ?: invalid()).map {
    parseKv(it).also { grant -> if (grant.prefix.isNullOrEmpty()) invalid() }
}
private fun parsePermissions(value: JsonElement, modern: Boolean): ApiKeyPermissions {
    val record = value as? JsonObject ?: invalid()
    return ApiKeyPermissions(boolean(record[if (modern) "admin" else "is_admin"]), grants(record[if (modern) "collections" else "collection_permissions"]), record,
        record[if (modern) "kv" else "kv_permissions"]?.takeUnless { !modern && it == JsonNull }?.let(::parseKvList) ?: if (modern) invalid() else emptyList(),
        record[if (modern) "functions" else "function_permissions"]?.takeUnless { it == JsonNull }?.let(::parseFunctionGrants) ?: emptyList())
}
private fun <T> parseChange(value: JsonElement, parse: (JsonElement) -> T): PermissionChange<T> {
    val record = value as? JsonObject ?: invalid()
    if (!record.containsKey("before") || !record.containsKey("after")) invalid()
    return PermissionChange(record["before"]!!.takeUnless { it == JsonNull }?.let(parse), record["after"]!!.takeUnless { it == JsonNull }?.let(parse), JsonObject(record.filterKeys { it != "before" && it != "after" }))
}
