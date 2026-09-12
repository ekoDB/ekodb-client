package io.ekodb.client.functions

import kotlinx.serialization.EncodeDefault
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonClassDiscriminator
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.int
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put

/**
 * Build the structural placeholder `{"type": "Parameter", "name": <name>}`
 * that ekoDB's `resolve_json_parameters` recognizes inside
 * [FunctionStageConfig.Insert.record], [FunctionStageConfig.Update.updates],
 * [FunctionStageConfig.UpdateById.updates],
 * [FunctionStageConfig.FindOneAndUpdate.updates],
 * [FunctionStageConfig.BatchInsert.records], and any QueryExpression
 * filter value.
 *
 * This is the structural alternative to the text-level `"{{name}}"`
 * placeholder form — use it when the parameter is a whole-object record
 * or a value whose type would be lost on a raw-JSON round-trip (Binary,
 * DateTime, UUID, Decimal, Duration, Number, Set, Vector). Requires
 * ekoDB >= 0.41.0 for the mutation-stage parameter-resolution
 * consistency fix.
 *
 * Example — items_create function:
 * ```kotlin
 * val createItem = UserFunction(
 *     label = "items_create",
 *     name = "Create item",
 *     parameters = mapOf("record" to ParameterDefinition(required = true)),
 *     functions = listOf(
 *         FunctionStageConfig.Insert(
 *             collection = "items",
 *             record = parameterRef("record"),
 *         ),
 *     ),
 * )
 * ```
 */
fun parameterRef(name: String): JsonObject = buildJsonObject {
    put("type", "Parameter")
    put("name", name)
}

enum class QueryConditionOperator(val wireValue: String) {
    Eq("Eq"),
    Ne("Ne"),
    Gt("Gt"),
    Gte("Gte"),
    Lt("Lt"),
    Lte("Lte"),
    In("In"),
    NotIn("NotIn"),
    Contains("Contains"),
    StartsWith("StartsWith"),
    EndsWith("EndsWith"),
    Equals("Equals"),
    Equal("Equal"),
    NotEquals("NotEquals"),
    NotEqual("NotEqual"),
    GreaterThan("GreaterThan"),
    LessThan("LessThan"),
    GreaterThanOrEqual("GreaterThanOrEqual"),
    LessThanOrEqual("LessThanOrEqual"),
}

enum class QueryLogicalOperator(val wireValue: String) {
    And("And"),
    Or("Or"),
    Not("Not"),
}

private val queryConditionOperators = QueryConditionOperator.entries.mapTo(mutableSetOf()) { it.wireValue }
private val queryLogicalOperators = QueryLogicalOperator.entries.mapTo(mutableSetOf()) { it.wireValue }

/** Build a valid condition expression for Query, Update, and Delete filters. */
fun queryCondition(field: String, operator: QueryConditionOperator, value: JsonElement): JsonObject = buildJsonObject {
    put("type", "Condition")
    put("content", buildJsonObject {
        put("field", field)
        put("operator", operator.wireValue)
        put("value", value)
    })
}

/** Build a valid logical expression from already-tagged child expressions. */
fun queryLogical(operator: QueryLogicalOperator, expressions: List<JsonObject>): JsonObject =
    validateQueryExpression(
        buildJsonObject {
            put("type", "Logical")
            put("content", buildJsonObject {
                put("operator", operator.wireValue)
                put("expressions", JsonArray(expressions))
            })
        },
    )

/** Validate raw JSON at stage construction so malformed filters never reach the server. */
fun validateQueryExpression(expression: JsonObject): JsonObject {
    val type = expression["type"]?.jsonPrimitive?.content
        ?: throw IllegalArgumentException("query expression must contain a string `type`")
    val content = expression["content"] as? JsonObject
        ?: throw IllegalArgumentException("query expression must contain an object `content`")

    when (type) {
        "Condition" -> {
            require((content["field"] as? JsonPrimitive)?.isString == true) {
                "Condition content requires string `field`"
            }
            val operator = content["operator"]?.jsonPrimitive?.content
            require(operator != null) {
                "Condition content requires string `operator`"
            }
            require(operator in queryConditionOperators) { "unsupported condition operator `$operator`" }
            require("value" in content) { "Condition content requires `value`" }
        }
        "Logical" -> {
            val operator = content["operator"]?.jsonPrimitive?.content
            require(operator != null) {
                "Logical content requires string `operator`"
            }
            val expressions = content["expressions"] as? JsonArray
                ?: throw IllegalArgumentException("Logical content requires array `expressions`")
            require(operator in queryLogicalOperators) { "unsupported logical operator `$operator`" }
            require(expressions.isNotEmpty()) { "logical operator `$operator` requires expressions" }
            require(operator != "Not" || expressions.size == 1) {
                "logical operator `Not` requires exactly one expression"
            }
            expressions.forEach {
                validateQueryExpression(
                    it as? JsonObject
                        ?: throw IllegalArgumentException("logical child must be an object"),
                )
            }
        }
        else -> throw IllegalArgumentException(
            "query expression `type` must be `Condition` or `Logical`",
        )
    }
    return expression
}

/**
 * Function pipeline stage configuration
 * Using polymorphic serialization with "type" as discriminator
 */
@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("type")
sealed class FunctionStageConfig {
    companion object {
        /**
         * Filter a collection.
         *
         * Shorthand for a [Query] carrying only `filter`. There is no separate
         * `Filter` stage server-side — filtering, sorting, limiting and
         * skipping are all fields on `Query`.
         *
         * This replaces a `Filter` subclass that serialized as
         * `{"type":"Filter"}`, which the server has no variant for. Because a
         * function's stage array deserializes as a unit, one such stage
         * rejected the ENTIRE function. It could not simply be retagged
         * `@SerialName("Query")`, since two subclasses of one sealed hierarchy
         * cannot share a serial name — hence a factory rather than a class.
         *
         * Use [Query] directly when you need more than one of these at once;
         * it takes them together and produces a single stage.
         */
        fun filter(collection: String, filter: JsonObject): FunctionStageConfig =
            Query(collection = collection, filter = filter)

        /**
         * Sort a collection. Shorthand for a [Query] carrying only `sort`.
         *
         * Takes typed [SortFieldConfig] rather than raw JSON so `ascending`
         * cannot be omitted — the server requires it, and a bare
         * `{"field": "..."}` is rejected, taking the whole function with it.
         */
        fun sort(collection: String, sort: List<SortFieldConfig>): FunctionStageConfig =
            Query(
                collection = collection,
                sort = sort.map { Json.encodeToJsonElement(SortFieldConfig.serializer(), it).jsonObject },
            )

        /** Limit a collection read. Shorthand for a [Query] with only `limit`. */
        fun limit(collection: String, limit: Int): FunctionStageConfig =
            Query(collection = collection, limit = limit)

        /** Skip rows of a collection read. Shorthand for a [Query] with only `skip`. */
        fun skip(collection: String, skip: Int): FunctionStageConfig =
            Query(collection = collection, skip = skip)
    }

    @Serializable
    @SerialName("FindAll")
    data class FindAll(
        val collection: String
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Query")
    data class Query(
        val collection: String,
        val filter: JsonObject? = null,
        val sort: List<JsonObject>? = null,
        val limit: Int? = null,
        val skip: Int? = null
    ) : FunctionStageConfig() {
        init {
            filter?.let(::validateQueryExpression)
        }
    }

    @Serializable
    @SerialName("Project")
    data class Project(
        val fields: List<String>,
        @EncodeDefault val exclude: Boolean = false
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Group")
    data class Group(
        val by_fields: List<String>,
        val functions: List<GroupFunctionConfig>
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Count")
    data class Count(
        @EncodeDefault val output_field: String = "count"
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Insert")
    data class Insert(
        val collection: String,
        val record: JsonObject,
        @EncodeDefault val bypass_ripple: Boolean = false,
        val ttl: Long? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Update")
    data class Update(
        val collection: String,
        val filter: JsonObject,
        val updates: JsonObject,
        @EncodeDefault val bypass_ripple: Boolean = false,
        val ttl: Long? = null
    ) : FunctionStageConfig() {
        init {
            validateQueryExpression(filter)
        }
    }

    @Serializable
    @SerialName("UpdateById")
    data class UpdateById(
        val collection: String,
        val record_id: String,
        val updates: JsonObject,
        @EncodeDefault val bypass_ripple: Boolean = false,
        val ttl: Long? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Delete")
    data class Delete(
        val collection: String,
        val filter: JsonObject,
        @EncodeDefault val bypass_ripple: Boolean = false
    ) : FunctionStageConfig() {
        init {
            validateQueryExpression(filter)
        }
    }

    @Serializable
    @SerialName("DeleteById")
    data class DeleteById(
        val collection: String,
        val record_id: String,
        @EncodeDefault val bypass_ripple: Boolean = false
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("BatchInsert")
    data class BatchInsert(
        val collection: String,
        val records: List<JsonObject>,
        @EncodeDefault val bypass_ripple: Boolean = false
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("BatchDelete")
    data class BatchDelete(
        val collection: String,
        val record_ids: List<String>,
        @EncodeDefault val bypass_ripple: Boolean = false
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("HttpRequest")
    data class HttpRequest(
        val url: String,
        @EncodeDefault val method: String = "GET",
        val headers: Map<String, String>? = null,
        val body: JsonElement? = null,
        val timeout_seconds: Long? = null,
        val output_field: String? = null,
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("VectorSearch")
    data class VectorSearch(
        val collection: String,
        val query_vector: List<Double>,
        val limit: Int? = null,
        val threshold: Double? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("TextSearch")
    data class TextSearch(
        val collection: String,
        val query_text: String,
        val fields: List<String>? = null,
        val limit: Int? = null,
        val fuzzy: Boolean? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("HybridSearch")
    data class HybridSearch(
        val collection: String,
        val query_text: String,
        val query_vector: List<Double>? = null,
        val limit: Int? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Chat")
    data class Chat(
        val messages: List<ChatMessage>,
        val model: String? = null,
        val temperature: Double? = null,
        val max_tokens: Int? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Embed")
    data class Embed(
        val input_field: String,
        val output_field: String,
        val model: JsonElement? = null // Flexible like TypeScript - accepts any JSON
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("FindById")
    data class FindById(
        val collection: String,
        val record_id: String
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("FindOne")
    data class FindOne(
        val collection: String,
        val key: String,
        val value: JsonElement
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("FindOneAndUpdate")
    data class FindOneAndUpdate(
        val collection: String,
        val record_id: String,
        val updates: JsonObject,
        @EncodeDefault val bypass_ripple: Boolean = false,
        val ttl: Long? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("UpdateWithAction")
    data class UpdateWithAction(
        val collection: String,
        val record_id: String,
        val action: String,
        val field: String,
        val value: JsonElement,
        @EncodeDefault val bypass_ripple: Boolean = false
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Upsert")
    data class Upsert(
        val collection: String,
        val key: String,
        val value: JsonElement,
        val record: JsonObject,
        @EncodeDefault val bypass_ripple: Boolean = false,
        val ttl: Long? = null,
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Increment")
    data class Increment(
        val collection: String,
        val record_id: String,
        val field: String,
        val by: JsonElement? = null,
        @EncodeDefault val bypass_ripple: Boolean = false,
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("Push")
    data class Push(
        val collection: String,
        val record_id: String,
        val field: String,
        val value: JsonElement,
        @EncodeDefault val bypass_ripple: Boolean = false,
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("SetField")
    data class SetField(
        val field: String,
        val value: JsonElement,
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("AddFields")
    data class AddFields(
        val fields: List<JsonObject>,
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("CurrentDatetime")
    data class CurrentDatetime(
        val output_field: String,
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("If")
    data class If(
        val condition: FunctionCondition,
        val then_functions: List<FunctionStageConfig>,
        val else_functions: List<FunctionStageConfig>? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("ForEach")
    data class ForEach(
        val functions: List<FunctionStageConfig>
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("CallFunction")
    data class CallFunction(
        val function_label: String,
        val params: JsonObject? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("CreateSavepoint")
    data class CreateSavepoint(
        val name: String
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("RollbackToSavepoint")
    data class RollbackToSavepoint(
        val name: String
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("ReleaseSavepoint")
    data class ReleaseSavepoint(
        val name: String
    ) : FunctionStageConfig()

    // =========================================================================
    // KV Store Operations
    // =========================================================================

    /** Returns {value: <data>} on hit, {value: null} on miss */
    @Serializable
    @SerialName("KvGet")
    data class KvGet(
        val key: String
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("KvSet")
    data class KvSet(
        val key: String,
        val value: JsonElement,
        val ttl: Long? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("KvDelete")
    data class KvDelete(
        val key: String
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("KvExists")
    data class KvExists(
        val key: String,
        val output_field: String? = null
    ) : FunctionStageConfig()

    @Serializable
    @SerialName("KvQuery")
    data class KvQuery(
        val pattern: String? = null,
        @EncodeDefault val include_expired: Boolean = false
    ) : FunctionStageConfig()

    /**
     * SWR (Stale-While-Revalidate) pattern for external API caching.
     * Automatically handles: KV cache check → HTTP request → KV cache set → optional audit storage.
     *
     * @param cache_key KV key for caching (supports parameter substitution like "user:{{user_id}}")
     * @param ttl Cache TTL - supports duration strings ("15m", "1h"), integers (seconds), or ISO timestamps
     * @param url HTTP URL to fetch from (supports parameter substitution)
     * @param method HTTP method (default: "GET")
     * @param headers Optional HTTP headers
     * @param body Optional HTTP request body
     * @param timeout_seconds Optional HTTP timeout
     * @param output_field Field name for response in enriched params (default: "response")
     * @param collection Optional collection for audit trail storage
     */
    @Serializable
    @SerialName("SWR")
    data class SWR(
        val cache_key: String,
        val ttl: JsonElement,
        val url: String,
        @EncodeDefault val method: String = "GET",
        val headers: Map<String, String>? = null,
        val body: JsonElement? = null,
        val timeout_seconds: Int? = null,
        val output_field: String? = null,
        val collection: String? = null
    ) : FunctionStageConfig()

    /**
     * Bcrypt-hash a plaintext value and write the result into every record
     * in the working data as [output_field]. Use in a compound
     * `users_register` function between input validation and Insert.
     *
     * [plain] is typically a `"{{password}}"` placeholder — the substituter
     * replaces it with the call-time param before this stage runs. [cost]
     * is the bcrypt work factor (4..=31); leave null for the ekoDB
     * default (12).
     *
     * Requires ekoDB >= 0.41.0.
     */
    @Serializable
    @SerialName("BcryptHash")
    data class BcryptHash(
        val plain: String,
        val cost: Int? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /**
     * Verify a plaintext against a bcrypt hash stored on the first record
     * in the working data. Writes a boolean result into [output_field] on
     * every working record. Pair with [FunctionCondition.FieldEquals] inside
     * an If stage to branch on login success / failure.
     *
     * [plain] is typically `"{{password}}"`. [hash_field] is the name of
     * the field on the current record holding the stored bcrypt hash
     * (e.g. `"password_hash"`).
     *
     * Requires ekoDB >= 0.41.0.
     */
    @Serializable
    @SerialName("BcryptVerify")
    data class BcryptVerify(
        val plain: String,
        val hash_field: String,
        val output_field: String
    ) : FunctionStageConfig()

    /**
     * Generate a cryptographically-random token and add it to every record
     * in the working data. [bytes] must be in 1..=1024. [encoding] is one
     * of `"hex"` (default), `"base64"`, or `"base64url"`; leave null to
     * accept the server default.
     *
     * Requires ekoDB >= 0.41.0.
     */
    @Serializable
    @SerialName("RandomToken")
    data class RandomToken(
        val bytes: Int,
        val encoding: String? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /**
     * Sign a JWT and write the resulting token to every working
     * record. Pair with [BcryptVerify] to issue a session token
     * after login. Use `"{{env.JWT_SECRET}}"` for [secret] so the
     * LLM never sees the operator-owned signing key. `iat` and
     * `exp` are auto-stamped when [expires_in_secs] is set.
     *
     * [algorithm] is one of `"HS256"` (default), `"HS384"`,
     * `"HS512"`; leave null to accept the server default.
     *
     * Requires ekoDB >= 0.42.0.
     */
    @Serializable
    @SerialName("JwtSign")
    data class JwtSign(
        val claims: Map<String, kotlinx.serialization.json.JsonElement>,
        val secret: String,
        val algorithm: String? = null,
        val expires_in_secs: Long? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /**
     * Verify a JWT held in [token_field] on the first working
     * record. On success writes the decoded claims object into
     * [output_field]; on failure writes JSON `null`. Branch with
     * [FunctionCondition.FieldEquals] matching `null` to reject.
     *
     * Requires ekoDB >= 0.42.0.
     */
    @Serializable
    @SerialName("JwtVerify")
    data class JwtVerify(
        val token_field: String,
        val secret: String,
        val algorithm: String? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /**
     * Send a transactional email through a provider's REST API.
     * Today only `provider = "sendgrid"` is supported. Use
     * `"{{env.SENDGRID_API_KEY}}"` for [api_key] so the LLM never
     * sees the operator-owned secret. Set [html] to true to send
     * `text/html`. The result envelope `{provider_status,
     * provider_message, provider}` is written to [output_field]
     * (default `"email_send"`).
     *
     * Requires ekoDB >= 0.42.0.
     */
    @Serializable
    @SerialName("EmailSend")
    data class EmailSend(
        val to: String,
        val subject: String,
        val body: String,
        val from: String,
        val reply_to: String? = null,
        val api_key: String,
        val provider: String? = null,
        val html: Boolean? = null,
        val output_field: String? = null
    ) : FunctionStageConfig()

    /** HMAC-SHA256/384/512 sign. Requires ekoDB >= 0.42.0. */
    @Serializable
    @SerialName("HmacSign")
    data class HmacSign(
        val input: String,
        val secret: String,
        val algorithm: String? = null,
        val output_field: String,
        val encoding: String? = null
    ) : FunctionStageConfig()

    /** HMAC verify (constant-time). Writes a boolean. */
    @Serializable
    @SerialName("HmacVerify")
    data class HmacVerify(
        val input: String,
        val provided_mac: String,
        val secret: String,
        val algorithm: String? = null,
        val encoding: String? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /** AES-256-GCM encrypt; writes `{ciphertext, nonce}` envelope. */
    @Serializable
    @SerialName("AesEncrypt")
    data class AesEncrypt(
        val plaintext: String,
        val key: String,
        val key_encoding: String? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /** AES-256-GCM decrypt; reads envelope from `ciphertext_field`. */
    @Serializable
    @SerialName("AesDecrypt")
    data class AesDecrypt(
        val ciphertext_field: String,
        val key: String,
        val key_encoding: String? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /** Generate a v4 UUID into `output_field`. */
    @Serializable
    @SerialName("UuidGenerate")
    data class UuidGenerate(val output_field: String) : FunctionStageConfig()

    /** TOTP code generation (RFC 6238). */
    @Serializable
    @SerialName("TotpGenerate")
    data class TotpGenerate(
        val secret: String,
        val digits: Int? = null,
        val period: Long? = null,
        val algorithm: String? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /** TOTP verify; tolerates `skew` time-steps either side (default 1). */
    @Serializable
    @SerialName("TotpVerify")
    data class TotpVerify(
        val code: String,
        val secret: String,
        val digits: Int? = null,
        val period: Long? = null,
        val algorithm: String? = null,
        val skew: Int? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /** Base64 encode (`url_safe = true` for URL-safe / no-pad). */
    @Serializable
    @SerialName("Base64Encode")
    data class Base64Encode(
        val input: String,
        val url_safe: Boolean? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /** Base64 decode → UTF-8. Fail-closed. */
    @Serializable
    @SerialName("Base64Decode")
    data class Base64Decode(
        val input: String,
        val url_safe: Boolean? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /** Hex encode (lowercase). */
    @Serializable
    @SerialName("HexEncode")
    data class HexEncode(val input: String, val output_field: String) : FunctionStageConfig()

    /** Hex decode → UTF-8. Fail-closed. */
    @Serializable
    @SerialName("HexDecode")
    data class HexDecode(val input: String, val output_field: String) : FunctionStageConfig()

    /** URL-friendly slug. */
    @Serializable
    @SerialName("Slugify")
    data class Slugify(val input: String, val output_field: String) : FunctionStageConfig()

    /**
     * Idempotency-key claim (KV SETNX with TTL). Requires ekoDB >= 0.42.0.
     */
    @Serializable
    @SerialName("IdempotencyClaim")
    data class IdempotencyClaim(
        val key: String,
        val ttl_secs: Long,
        val output_field: String
    ) : FunctionStageConfig()

    /** Fixed-window rate-limit gate. */
    @Serializable
    @SerialName("RateLimit")
    data class RateLimit(
        val key: String,
        val limit: Long,
        val window_secs: Long,
        val on_exceed: String? = null,
        val output_field: String
    ) : FunctionStageConfig()

    /** Distributed-lock acquire (token-fenced). */
    @Serializable
    @SerialName("LockAcquire")
    data class LockAcquire(
        val key: String,
        val ttl_secs: Long,
        val output_field: String
    ) : FunctionStageConfig()

    /** Distributed-lock release; only releases on token match. */
    @Serializable
    @SerialName("LockRelease")
    data class LockRelease(
        val key: String,
        val token: String,
        val output_field: String
    ) : FunctionStageConfig()

    /**
     * Try/Catch error handling for graceful failure recovery.
     * Executes [try_functions], and if any fail, executes [catch_functions].
     *
     * [output_error_field] is the field name to store error details (default: "error").
     */
    @Serializable
    @SerialName("TryCatch")
    data class TryCatch(
        val try_functions: List<FunctionStageConfig>,
        val catch_functions: List<FunctionStageConfig>,
        val output_error_field: String? = null
    ) : FunctionStageConfig()

    /**
     * Execute multiple functions in parallel (concurrently).
     * All functions run simultaneously, results are merged.
     *
     * If [wait_for_all] is true, waits for all to complete; if false, returns
     * on first completion.
     */
    @Serializable
    @SerialName("Parallel")
    data class Parallel(
        val functions: List<FunctionStageConfig>,
        @EncodeDefault val wait_for_all: Boolean = true
    ) : FunctionStageConfig()

    /**
     * Sleep/delay execution for rate limiting or timing control.
     *
     * [duration_ms] is the duration in milliseconds — an integer or a
     * `"{{delay_param}}"` placeholder string.
     */
    @Serializable
    @SerialName("Sleep")
    data class Sleep(
        val duration_ms: JsonElement
    ) : FunctionStageConfig()

    /**
     * Return a shaped response (final output formatting).
     * Constructs the final response object from current execution context.
     *
     * [fields] are the fields to include in the response (supports `{{param}}`
     * substitution). [status_code] is the HTTP status code (default: 200).
     */
    @Serializable
    @SerialName("Return")
    data class Return(
        val fields: Map<String, JsonElement>,
        val status_code: Int? = null
    ) : FunctionStageConfig()

    /**
     * Validate data against a JSON schema before processing.
     *
     * [schema] is the JSON Schema to validate against. [data_field] is the
     * field containing data to validate. [on_error] are optional functions to
     * execute on validation failure.
     */
    @Serializable
    @SerialName("Validate")
    data class Validate(
        val schema: JsonElement,
        val data_field: String,
        val on_error: List<FunctionStageConfig>? = null
    ) : FunctionStageConfig()
}

/**
 * Condition for function control flow (If statements)
 * Uses adjacently-tagged format: { "type": "...", "value": { ...data } }
 * Unit variants (HasRecords) have no value field.
 */
@Serializable(with = FunctionConditionSerializer::class)
sealed class FunctionCondition {
    object HasRecords : FunctionCondition()
    data class FieldEquals(val field: String, val fieldValue: JsonElement) : FunctionCondition()
    data class FieldExists(val field: String) : FunctionCondition()
    data class FieldGreaterThan(val field: String, val fieldValue: JsonElement) : FunctionCondition()
    data class FieldLessThan(val field: String, val fieldValue: JsonElement) : FunctionCondition()
    data class FieldGreaterThanOrEqual(val field: String, val fieldValue: JsonElement) : FunctionCondition()
    data class FieldLessThanOrEqual(val field: String, val fieldValue: JsonElement) : FunctionCondition()
    data class CountEquals(val count: Int) : FunctionCondition()
    data class CountGreaterThan(val count: Int) : FunctionCondition()
    data class CountLessThan(val count: Int) : FunctionCondition()
    data class And(val conditions: List<FunctionCondition>) : FunctionCondition()
    data class Or(val conditions: List<FunctionCondition>) : FunctionCondition()
    data class Not(val condition: FunctionCondition) : FunctionCondition()
}

/**
 * Custom serializer for FunctionCondition that produces adjacently-tagged format.
 */
object FunctionConditionSerializer : kotlinx.serialization.KSerializer<FunctionCondition> {
    override val descriptor: kotlinx.serialization.descriptors.SerialDescriptor =
        kotlinx.serialization.descriptors.buildClassSerialDescriptor("FunctionCondition") {
            element("type", kotlinx.serialization.descriptors.PrimitiveSerialDescriptor("type", kotlinx.serialization.descriptors.PrimitiveKind.STRING))
            element("value", JsonObject.serializer().descriptor, isOptional = true)
        }

    override fun serialize(encoder: kotlinx.serialization.encoding.Encoder, value: FunctionCondition) {
        val jsonEncoder = encoder as kotlinx.serialization.json.JsonEncoder
        val jsonObject = when (value) {
            is FunctionCondition.HasRecords -> buildJsonObject { put("type", "HasRecords") }
            is FunctionCondition.FieldEquals -> buildJsonObject {
                put("type", "FieldEquals")
                put("value", buildJsonObject {
                    put("field", value.field)
                    put("value", value.fieldValue)
                })
            }
            is FunctionCondition.FieldExists -> buildJsonObject {
                put("type", "FieldExists")
                put("value", buildJsonObject { put("field", value.field) })
            }
            is FunctionCondition.FieldGreaterThan -> fieldComparison("FieldGreaterThan", value.field, value.fieldValue)
            is FunctionCondition.FieldLessThan -> fieldComparison("FieldLessThan", value.field, value.fieldValue)
            is FunctionCondition.FieldGreaterThanOrEqual ->
                fieldComparison("FieldGreaterThanOrEqual", value.field, value.fieldValue)
            is FunctionCondition.FieldLessThanOrEqual ->
                fieldComparison("FieldLessThanOrEqual", value.field, value.fieldValue)
            is FunctionCondition.CountEquals -> buildJsonObject {
                put("type", "CountEquals")
                put("value", buildJsonObject { put("count", value.count) })
            }
            is FunctionCondition.CountGreaterThan -> buildJsonObject {
                put("type", "CountGreaterThan")
                put("value", buildJsonObject { put("count", value.count) })
            }
            is FunctionCondition.CountLessThan -> buildJsonObject {
                put("type", "CountLessThan")
                put("value", buildJsonObject { put("count", value.count) })
            }
            is FunctionCondition.And -> buildJsonObject {
                put("type", "And")
                put("value", buildJsonObject {
                    put("conditions", kotlinx.serialization.json.Json.encodeToJsonElement(
                        kotlinx.serialization.builtins.ListSerializer(FunctionConditionSerializer),
                        value.conditions
                    ))
                })
            }
            is FunctionCondition.Or -> buildJsonObject {
                put("type", "Or")
                put("value", buildJsonObject {
                    put("conditions", kotlinx.serialization.json.Json.encodeToJsonElement(
                        kotlinx.serialization.builtins.ListSerializer(FunctionConditionSerializer),
                        value.conditions
                    ))
                })
            }
            is FunctionCondition.Not -> buildJsonObject {
                put("type", "Not")
                put("value", buildJsonObject {
                    put("condition", kotlinx.serialization.json.Json.encodeToJsonElement(
                        FunctionConditionSerializer,
                        value.condition
                    ))
                })
            }
        }
        jsonEncoder.encodeJsonElement(jsonObject)
    }

    override fun deserialize(decoder: kotlinx.serialization.encoding.Decoder): FunctionCondition {
        val jsonDecoder = decoder as kotlinx.serialization.json.JsonDecoder
        val jsonObject = jsonDecoder.decodeJsonElement().jsonObject
        val type = jsonObject["type"]?.jsonPrimitive?.content ?: error("Missing type field")
        val valueObj = jsonObject["value"]?.jsonObject

        requireOnlyKeys(
            jsonObject,
            if (type == "HasRecords") setOf("type") else setOf("type", "value"),
            type,
        )

        return when (type) {
            "HasRecords" -> FunctionCondition.HasRecords
            "FieldEquals" -> {
                val v = valueObj ?: error("Missing value for FieldEquals")
                requireOnlyKeys(v, setOf("field", "value"), type)
                FunctionCondition.FieldEquals(
                    v["field"]?.jsonPrimitive?.content ?: error("Missing field"),
                    v["value"] ?: error("Missing value")
                )
            }
            "FieldExists" -> {
                val v = valueObj ?: error("Missing value for FieldExists")
                requireOnlyKeys(v, setOf("field"), type)
                FunctionCondition.FieldExists(v["field"]?.jsonPrimitive?.content ?: error("Missing field"))
            }
            "FieldGreaterThan" -> decodeFieldComparison(valueObj, type) { field, value ->
                FunctionCondition.FieldGreaterThan(field, value)
            }
            "FieldLessThan" -> decodeFieldComparison(valueObj, type) { field, value ->
                FunctionCondition.FieldLessThan(field, value)
            }
            "FieldGreaterThanOrEqual" ->
                decodeFieldComparison(valueObj, type) { field, value ->
                    FunctionCondition.FieldGreaterThanOrEqual(field, value)
                }
            "FieldLessThanOrEqual" ->
                decodeFieldComparison(valueObj, type) { field, value ->
                    FunctionCondition.FieldLessThanOrEqual(field, value)
                }
            "CountEquals" -> {
                val v = valueObj ?: error("Missing value for CountEquals")
                requireOnlyKeys(v, setOf("count"), type)
                FunctionCondition.CountEquals(v["count"]?.jsonPrimitive?.int ?: error("Missing count"))
            }
            "CountGreaterThan" -> {
                val v = valueObj ?: error("Missing value for CountGreaterThan")
                requireOnlyKeys(v, setOf("count"), type)
                FunctionCondition.CountGreaterThan(v["count"]?.jsonPrimitive?.int ?: error("Missing count"))
            }
            "CountLessThan" -> {
                val v = valueObj ?: error("Missing value for CountLessThan")
                requireOnlyKeys(v, setOf("count"), type)
                FunctionCondition.CountLessThan(v["count"]?.jsonPrimitive?.int ?: error("Missing count"))
            }
            "And" -> {
                val v = valueObj ?: error("Missing value for And")
                requireOnlyKeys(v, setOf("conditions"), type)
                val conditions = kotlinx.serialization.json.Json.decodeFromJsonElement(
                    kotlinx.serialization.builtins.ListSerializer(FunctionConditionSerializer),
                    v["conditions"] ?: error("Missing conditions")
                )
                FunctionCondition.And(conditions)
            }
            "Or" -> {
                val v = valueObj ?: error("Missing value for Or")
                requireOnlyKeys(v, setOf("conditions"), type)
                val conditions = kotlinx.serialization.json.Json.decodeFromJsonElement(
                    kotlinx.serialization.builtins.ListSerializer(FunctionConditionSerializer),
                    v["conditions"] ?: error("Missing conditions")
                )
                FunctionCondition.Or(conditions)
            }
            "Not" -> {
                val v = valueObj ?: error("Missing value for Not")
                requireOnlyKeys(v, setOf("condition"), type)
                val condition = kotlinx.serialization.json.Json.decodeFromJsonElement(
                    FunctionConditionSerializer,
                    v["condition"] ?: error("Missing condition")
                )
                FunctionCondition.Not(condition)
            }
            else -> error("Unknown condition type: $type")
        }
    }

    private fun fieldComparison(type: String, field: String, fieldValue: JsonElement): JsonObject =
        buildJsonObject {
            put("type", type)
            put("value", buildJsonObject {
                put("field", field)
                put("value", fieldValue)
            })
        }

    private fun decodeFieldComparison(
        value: JsonObject?,
        type: String,
        create: (String, JsonElement) -> FunctionCondition,
    ): FunctionCondition {
        val fields = value ?: error("Missing value for $type")
        requireOnlyKeys(fields, setOf("field", "value"), type)
        return create(
            fields["field"]?.jsonPrimitive?.content ?: error("Missing field"),
            fields["value"] ?: error("Missing value"),
        )
    }

    private fun requireOnlyKeys(value: JsonObject, allowed: Set<String>, type: String) {
        val unknown = value.keys - allowed
        require(unknown.isEmpty()) {
            "$type condition contains unknown field(s): ${unknown.sorted().joinToString()}"
        }
    }
}

/**
 * Chat message for AI operations
 */
@Serializable
data class ChatMessage(
    val role: String,
    val content: String
) {
    companion object {
        fun system(content: String) = ChatMessage("system", content)
        fun user(content: String) = ChatMessage("user", content)
        fun assistant(content: String) = ChatMessage("assistant", content)
    }
}

/**
 * Group function configuration for Group stage
 */
@Serializable
data class GroupFunctionConfig(
    val output_field: String,
    val operation: GroupFunctionOp,
    val input_field: String? = null
)

/**
 * Group function operation types
 */
@Serializable
enum class GroupFunctionOp {
    Sum,
    Average,
    Count,
    Min,
    Max,
    First,
    Last,
    Push,
    AddToSet,
    StandardDeviation,
    ApproxDistinct,
}

/**
 * Sort field configuration
 */
@OptIn(ExperimentalSerializationApi::class)
@Serializable
data class SortFieldConfig(
    val field: String,
    @EncodeDefault val ascending: Boolean = true
)
