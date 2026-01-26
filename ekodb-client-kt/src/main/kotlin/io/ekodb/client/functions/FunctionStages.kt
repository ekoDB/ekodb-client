package io.ekodb.client.functions

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.EncodeDefault
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonClassDiscriminator
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.int

/**
 * Function pipeline stage configuration
 * Using polymorphic serialization with "type" as discriminator
 */
@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("type")
sealed class FunctionStageConfig {
    
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
    ) : FunctionStageConfig()
    
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
    ) : FunctionStageConfig()
    
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
    ) : FunctionStageConfig()
    
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
    @SerialName("Filter")
    data class Filter(
        val filter: JsonObject
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Sort")
    data class Sort(
        val sort: List<SortFieldConfig>
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Limit")
    data class Limit(
        val limit: Int
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Skip")
    data class Skip(
        val skip: Int
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("HttpRequest")
    data class HttpRequest(
        val url: String,
        @EncodeDefault val method: String = "GET",
        val headers: Map<String, String>? = null,
        val body: JsonElement? = null
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
        val model: JsonElement? = null  // Flexible like TypeScript - accepts any JSON
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
    @SerialName("If")
    data class If(
        val condition: ScriptCondition,
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
    
    @Serializable
    @SerialName("KvGet")
    data class KvGet(
        val key: String,
        val output_field: String? = null
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
}

/**
 * Condition for Script control flow (If statements)
 * Uses adjacently-tagged format: { "type": "...", "value": { ...data } }
 * Unit variants (HasRecords) have no value field.
 */
@Serializable(with = ScriptConditionSerializer::class)
sealed class ScriptCondition {
    object HasRecords : ScriptCondition()
    data class FieldEquals(val field: String, val fieldValue: JsonElement) : ScriptCondition()
    data class FieldExists(val field: String) : ScriptCondition()
    data class CountEquals(val count: Int) : ScriptCondition()
    data class CountGreaterThan(val count: Int) : ScriptCondition()
    data class CountLessThan(val count: Int) : ScriptCondition()
    data class And(val conditions: List<ScriptCondition>) : ScriptCondition()
    data class Or(val conditions: List<ScriptCondition>) : ScriptCondition()
    data class Not(val condition: ScriptCondition) : ScriptCondition()
}

/**
 * Custom serializer for ScriptCondition that produces adjacently-tagged format.
 */
object ScriptConditionSerializer : kotlinx.serialization.KSerializer<ScriptCondition> {
    override val descriptor: kotlinx.serialization.descriptors.SerialDescriptor =
        kotlinx.serialization.descriptors.buildClassSerialDescriptor("ScriptCondition") {
            element("type", kotlinx.serialization.descriptors.PrimitiveSerialDescriptor("type", kotlinx.serialization.descriptors.PrimitiveKind.STRING))
            element("value", JsonObject.serializer().descriptor, isOptional = true)
        }

    override fun serialize(encoder: kotlinx.serialization.encoding.Encoder, value: ScriptCondition) {
        val jsonEncoder = encoder as kotlinx.serialization.json.JsonEncoder
        val jsonObject = when (value) {
            is ScriptCondition.HasRecords -> buildJsonObject { put("type", "HasRecords") }
            is ScriptCondition.FieldEquals -> buildJsonObject {
                put("type", "FieldEquals")
                put("value", buildJsonObject {
                    put("field", value.field)
                    put("value", value.fieldValue)
                })
            }
            is ScriptCondition.FieldExists -> buildJsonObject {
                put("type", "FieldExists")
                put("value", buildJsonObject { put("field", value.field) })
            }
            is ScriptCondition.CountEquals -> buildJsonObject {
                put("type", "CountEquals")
                put("value", buildJsonObject { put("count", value.count) })
            }
            is ScriptCondition.CountGreaterThan -> buildJsonObject {
                put("type", "CountGreaterThan")
                put("value", buildJsonObject { put("count", value.count) })
            }
            is ScriptCondition.CountLessThan -> buildJsonObject {
                put("type", "CountLessThan")
                put("value", buildJsonObject { put("count", value.count) })
            }
            is ScriptCondition.And -> buildJsonObject {
                put("type", "And")
                put("value", buildJsonObject {
                    put("conditions", kotlinx.serialization.json.Json.encodeToJsonElement(
                        kotlinx.serialization.builtins.ListSerializer(ScriptConditionSerializer),
                        value.conditions
                    ))
                })
            }
            is ScriptCondition.Or -> buildJsonObject {
                put("type", "Or")
                put("value", buildJsonObject {
                    put("conditions", kotlinx.serialization.json.Json.encodeToJsonElement(
                        kotlinx.serialization.builtins.ListSerializer(ScriptConditionSerializer),
                        value.conditions
                    ))
                })
            }
            is ScriptCondition.Not -> buildJsonObject {
                put("type", "Not")
                put("value", buildJsonObject {
                    put("condition", kotlinx.serialization.json.Json.encodeToJsonElement(
                        ScriptConditionSerializer,
                        value.condition
                    ))
                })
            }
        }
        jsonEncoder.encodeJsonElement(jsonObject)
    }

    override fun deserialize(decoder: kotlinx.serialization.encoding.Decoder): ScriptCondition {
        val jsonDecoder = decoder as kotlinx.serialization.json.JsonDecoder
        val jsonObject = jsonDecoder.decodeJsonElement().jsonObject
        val type = jsonObject["type"]?.jsonPrimitive?.content ?: error("Missing type field")
        val valueObj = jsonObject["value"]?.jsonObject

        return when (type) {
            "HasRecords" -> ScriptCondition.HasRecords
            "FieldEquals" -> {
                val v = valueObj ?: error("Missing value for FieldEquals")
                ScriptCondition.FieldEquals(
                    v["field"]?.jsonPrimitive?.content ?: error("Missing field"),
                    v["value"] ?: error("Missing value")
                )
            }
            "FieldExists" -> {
                val v = valueObj ?: error("Missing value for FieldExists")
                ScriptCondition.FieldExists(v["field"]?.jsonPrimitive?.content ?: error("Missing field"))
            }
            "CountEquals" -> {
                val v = valueObj ?: error("Missing value for CountEquals")
                ScriptCondition.CountEquals(v["count"]?.jsonPrimitive?.int ?: error("Missing count"))
            }
            "CountGreaterThan" -> {
                val v = valueObj ?: error("Missing value for CountGreaterThan")
                ScriptCondition.CountGreaterThan(v["count"]?.jsonPrimitive?.int ?: error("Missing count"))
            }
            "CountLessThan" -> {
                val v = valueObj ?: error("Missing value for CountLessThan")
                ScriptCondition.CountLessThan(v["count"]?.jsonPrimitive?.int ?: error("Missing count"))
            }
            "And" -> {
                val v = valueObj ?: error("Missing value for And")
                val conditions = kotlinx.serialization.json.Json.decodeFromJsonElement(
                    kotlinx.serialization.builtins.ListSerializer(ScriptConditionSerializer),
                    v["conditions"] ?: error("Missing conditions")
                )
                ScriptCondition.And(conditions)
            }
            "Or" -> {
                val v = valueObj ?: error("Missing value for Or")
                val conditions = kotlinx.serialization.json.Json.decodeFromJsonElement(
                    kotlinx.serialization.builtins.ListSerializer(ScriptConditionSerializer),
                    v["conditions"] ?: error("Missing conditions")
                )
                ScriptCondition.Or(conditions)
            }
            "Not" -> {
                val v = valueObj ?: error("Missing value for Not")
                val condition = kotlinx.serialization.json.Json.decodeFromJsonElement(
                    ScriptConditionSerializer,
                    v["condition"] ?: error("Missing condition")
                )
                ScriptCondition.Not(condition)
            }
            else -> error("Unknown condition type: $type")
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
    Push
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
