package io.ekodb.client.functions

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonClassDiscriminator

/**
 * Function pipeline stage configuration
 * Using polymorphic serialization with "type" as discriminator
 */
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
        val expression: JsonObject
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Project")
    data class Project(
        val fields: List<String>
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Group")
    data class Group(
        val by_fields: List<String>,
        val functions: List<GroupFunctionConfig>
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Count")
    object Count : FunctionStageConfig()
    
    @Serializable
    @SerialName("Insert")
    data class Insert(
        val collection: String,
        val data: JsonObject,
        val bypass_ripple: Boolean = false
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Delete")
    data class Delete(
        val collection: String,
        val id: ParameterValue,
        val bypass_ripple: Boolean = false
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("BatchInsert")
    data class BatchInsert(
        val collection: String,
        val records: List<JsonObject>,
        val bypass_ripple: Boolean = false
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("BatchDelete")
    data class BatchDelete(
        val collection: String,
        val ids: List<ParameterValue>,
        val bypass_ripple: Boolean = false
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("HttpRequest")
    data class HttpRequest(
        val url: String,
        val method: String = "GET",
        val headers: Map<String, String>? = null,
        val body: JsonElement? = null
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("VectorSearch")
    data class VectorSearch(
        val collection: String,
        val query_vector: List<Double>,
        val options: JsonObject? = null
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("TextSearch")
    data class TextSearch(
        val collection: String,
        val query: String,
        val options: JsonObject? = null
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("HybridSearch")
    data class HybridSearch(
        val collection: String,
        val text_query: String,
        val vector_query: List<Double>,
        val options: JsonObject? = null
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Chat")
    data class Chat(
        val messages: List<ChatMessage>,
        val model: String? = null,
        val temperature: Double? = null
    ) : FunctionStageConfig()
    
    @Serializable
    @SerialName("Embed")
    data class Embed(
        val texts: JsonElement,
        val model: String? = null
    ) : FunctionStageConfig()
}

/**
 * Chat message for AI operations
 */
@Serializable
data class ChatMessage(
    val role: ParameterValue,
    val content: ParameterValue
) {
    companion object {
        fun system(content: String) = ChatMessage(
            ParameterValue.literal(kotlinx.serialization.json.JsonPrimitive("system")),
            ParameterValue.literal(kotlinx.serialization.json.JsonPrimitive(content))
        )
        
        fun user(content: String) = ChatMessage(
            ParameterValue.literal(kotlinx.serialization.json.JsonPrimitive("user")),
            ParameterValue.literal(kotlinx.serialization.json.JsonPrimitive(content))
        )
        
        fun assistant(content: String) = ChatMessage(
            ParameterValue.literal(kotlinx.serialization.json.JsonPrimitive("assistant")),
            ParameterValue.literal(kotlinx.serialization.json.JsonPrimitive(content))
        )
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
@Serializable
data class SortFieldConfig(
    val field: String,
    val ascending: Boolean = true
)
