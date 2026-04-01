package io.ekodb.client.types

import kotlinx.serialization.*
import kotlinx.serialization.json.*

// ========== Tool Configuration ==========

/**
 * Controls how the LLM decides whether to use tools.
 *
 * ```kotlin
 * val choice = ToolChoice(type = "auto")
 * val forced = ToolChoice(type = "tool", name = "web_search")
 * ```
 */
@Serializable
data class ToolChoice(
    val type: String,    // "auto", "none", "required", "tool"
    val name: String? = null  // only used when type is "tool"
)

/**
 * Configuration for which tools are available in a chat session.
 */
@Serializable
data class ToolConfig(
    val enabled: Boolean = false,
    @SerialName("allowed_tools")
    val allowedTools: List<String>? = null,
    @SerialName("allowed_collections")
    val allowedCollections: List<String>? = null,
    @SerialName("max_iterations")
    val maxIterations: Int? = null,
    @SerialName("allow_write_operations")
    val allowWriteOperations: Boolean? = null,
    @SerialName("tool_choice")
    val toolChoice: ToolChoice? = null,
)

// ========== Collection & Search Config ==========

/**
 * Configuration for searching a specific collection in chat context.
 */
@Serializable
data class CollectionConfig(
    @SerialName("collection_name")
    val collectionName: String,
    val fields: List<FieldSearchOptions> = emptyList(),
    @SerialName("search_options")
    val searchOptions: TextSearchOptions? = null,
)

/**
 * Field-specific search options.
 */
@Serializable
data class FieldSearchOptions(
    val field: String,
    @SerialName("search_options")
    val searchOptions: TextSearchOptions? = null,
)

/**
 * Text search options for chat context retrieval.
 */
@Serializable
data class TextSearchOptions(
    val language: String? = null,
    @SerialName("case_sensitive")
    val caseSensitive: Boolean? = null,
    @SerialName("fuzzy_match")
    val fuzzyMatch: Boolean? = null,
    @SerialName("min_score")
    val minScore: Double? = null,
    @SerialName("enable_stemming")
    val enableStemming: Boolean? = null,
    @SerialName("boost_exact_matches")
    val boostExactMatches: Boolean? = null,
    @SerialName("max_edit_distance")
    val maxEditDistance: Int? = null,
)

// ========== Chat Request/Response ==========

/**
 * Request to send a one-shot chat message (creates a new session).
 */
@Serializable
data class ChatRequest(
    val message: String,
    val collections: List<CollectionConfig> = emptyList(),
    @SerialName("llm_provider")
    val llmProvider: String,
    @SerialName("llm_model")
    val llmModel: String? = null,
    @SerialName("system_prompt")
    val systemPrompt: String? = null,
)

/**
 * Token usage statistics from LLM providers.
 */
@Serializable
data class TokenUsage(
    @SerialName("prompt_tokens")
    val promptTokens: Int? = null,
    @SerialName("completion_tokens")
    val completionTokens: Int? = null,
    @SerialName("total_tokens")
    val totalTokens: Int? = null,
)

/**
 * Context snippet from search results used as chat context.
 */
@Serializable
data class ContextSnippet(
    val collection: String,
    val record: JsonElement,
    val score: Double,
    @SerialName("matched_fields")
    val matchedFields: List<String>,
)

/**
 * Response from a chat request.
 */
@Serializable
data class ChatResponse(
    @SerialName("chat_id")
    val chatId: String,
    @SerialName("message_id")
    val messageId: String,
    val responses: List<String>,
    @SerialName("context_snippets")
    val contextSnippets: List<ContextSnippet> = emptyList(),
    @SerialName("execution_time_ms")
    val executionTimeMs: Long,
    @SerialName("token_usage")
    val tokenUsage: TokenUsage? = null,
)

// ========== Session Management ==========

/**
 * Request to create a new chat session.
 */
@Serializable
data class CreateChatSessionRequest(
    val collections: List<CollectionConfig> = emptyList(),
    @SerialName("llm_provider")
    val llmProvider: String,
    @SerialName("llm_model")
    val llmModel: String? = null,
    @SerialName("system_prompt")
    val systemPrompt: String? = null,
    @SerialName("agent_id")
    val agentId: String? = null,
    @SerialName("bypass_ripple")
    val bypassRipple: Boolean? = null,
    @SerialName("parent_id")
    val parentId: String? = null,
    @SerialName("branch_point_idx")
    val branchPointIdx: Int? = null,
    @SerialName("max_context_messages")
    val maxContextMessages: Int? = null,
    @SerialName("max_tokens")
    val maxTokens: Int? = null,
    val temperature: Float? = null,
    @SerialName("tool_config")
    val toolConfig: ToolConfig? = null,
)

/**
 * Chat session details.
 */
@Serializable
data class ChatSession(
    @SerialName("chat_id")
    val chatId: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String,
    @SerialName("llm_provider")
    val llmProvider: String,
    @SerialName("llm_model")
    val llmModel: String,
    val collections: List<CollectionConfig> = emptyList(),
    @SerialName("system_prompt")
    val systemPrompt: String? = null,
    @SerialName("agent_id")
    val agentId: String? = null,
    val title: String? = null,
    @SerialName("message_count")
    val messageCount: Int,
)

/**
 * Response containing session info and message count.
 */
@Serializable
data class ChatSessionResponse(
    val session: JsonObject,
    @SerialName("message_count")
    val messageCount: Int,
)

// ========== Message Operations ==========

/**
 * Request to send a message in an existing session.
 */
@Serializable
data class ChatMessageRequest(
    val message: String,
    @SerialName("bypass_ripple")
    val bypassRipple: Boolean? = null,
    @SerialName("force_summarize")
    val forceSummarize: Boolean? = null,
    @SerialName("max_iterations")
    val maxIterations: Int? = null,
    @SerialName("tool_config")
    val toolConfig: ToolConfig? = null,
    @SerialName("llm_model")
    val llmModel: String? = null,
)

/**
 * Request to update session metadata.
 */
@Serializable
data class UpdateSessionRequest(
    @SerialName("system_prompt")
    val systemPrompt: String? = null,
    @SerialName("llm_model")
    val llmModel: String? = null,
    val collections: List<CollectionConfig>? = null,
    @SerialName("max_context_messages")
    val maxContextMessages: Int? = null,
    @SerialName("bypass_ripple")
    val bypassRipple: Boolean? = null,
    val title: String? = null,
    val memory: JsonElement? = null,
)

/**
 * Request to update a message's content.
 */
@Serializable
data class UpdateMessageRequest(
    val content: String,
)

/**
 * Request to toggle a message's forgotten status.
 */
@Serializable
data class ToggleForgottenRequest(
    val forgotten: Boolean,
)

// ========== Query Parameters ==========

/**
 * Query parameters for getting messages.
 */
@Serializable
data class GetMessagesQuery(
    val limit: Int? = null,
    val skip: Int? = null,
    val sort: String? = null,
)

/**
 * Response containing messages with pagination.
 */
@Serializable
data class GetMessagesResponse(
    val messages: List<JsonObject>,
    val total: Int,
    val skip: Int,
    val limit: Int? = null,
    val returned: Int,
)

/**
 * Query parameters for listing sessions.
 */
@Serializable
data class ListSessionsQuery(
    val limit: Int? = null,
    val skip: Int? = null,
    val sort: String? = null,
)

/**
 * Response containing list of sessions.
 */
@Serializable
data class ListSessionsResponse(
    val sessions: List<ChatSession>,
    val total: Int,
    val returned: Int,
)

// ========== Merge ==========

/**
 * Strategy for merging chat sessions.
 */
@Serializable
enum class MergeStrategy {
    @SerialName("Chronological") Chronological,
    @SerialName("Summarized") Summarized,
    @SerialName("LatestOnly") LatestOnly,
    @SerialName("Interleaved") Interleaved,
}

/**
 * Request to merge multiple chat sessions.
 */
@Serializable
data class MergeSessionsRequest(
    @SerialName("source_chat_ids")
    val sourceChatIds: List<String>,
    @SerialName("target_chat_id")
    val targetChatId: String,
    @SerialName("merge_strategy")
    val mergeStrategy: MergeStrategy,
    @SerialName("bypass_ripple")
    val bypassRipple: Boolean? = null,
)

// ========== Models ==========

/**
 * Available LLM models by provider.
 */
@Serializable
data class ChatModels(
    val openai: List<String> = emptyList(),
    val anthropic: List<String> = emptyList(),
    val perplexity: List<String> = emptyList(),
)

// ========== Raw Completion ==========

/**
 * Request for stateless raw LLM completion — no session, no history, no RAG.
 */
@Serializable
data class RawCompletionRequest(
    @SerialName("system_prompt")
    val systemPrompt: String,
    val message: String,
    val provider: String? = null,
    val model: String? = null,
    @SerialName("max_tokens")
    val maxTokens: Int? = null,
)

/**
 * Response from raw LLM completion.
 */
@Serializable
data class RawCompletionResponse(
    val content: String,
)

// ========== Embeddings ==========

/**
 * Request to generate embeddings.
 */
@Serializable
data class EmbedRequest(
    val text: String? = null,
    val texts: List<String>? = null,
    val model: String? = null,
)

/**
 * Response from embedding generation.
 */
@Serializable
data class EmbedResponse(
    val embeddings: List<List<Double>>,
    val model: String,
    val dimensions: Int,
)
