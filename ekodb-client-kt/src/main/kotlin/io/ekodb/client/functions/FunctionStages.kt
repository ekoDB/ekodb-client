package io.ekodb.client.functions

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.EncodeDefault
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonClassDiscriminator

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
