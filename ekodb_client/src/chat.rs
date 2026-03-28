//! Chat and AI functionality for ekoDB
//!
//! This module provides integration with Large Language Models (LLMs) to enable
//! AI-powered chat functionality over database content.

use crate::types::Record;
use serde::{Deserialize, Serialize};

/// Controls how the LLM decides whether to use tools
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoice {
    /// LLM decides whether to use tools (default)
    Auto,
    /// Never use tools, text response only
    None,
    /// Must use at least one tool
    Required,
    /// Force use of a specific tool by name
    Tool { name: String },
}

/// Configuration for which tools are available in a chat session
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolConfig {
    /// Enable/disable all tools (master switch)
    #[serde(default)]
    pub enabled: bool,
    /// Specific tools to enable (if None, all tools enabled when enabled=true)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,
    /// Collections the tools can access (if None, uses session's collections)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_collections: Option<Vec<String>>,
    /// Maximum iterations for tool calling loop
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_iterations: Option<u32>,
    /// Whether tools can perform write operations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_write_operations: Option<bool>,
    /// Controls how the LLM decides whether to use tools
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

/// Available LLM models from different providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Models {
    pub openai: Vec<String>,
    pub anthropic: Vec<String>,
    pub perplexity: Vec<String>,
}

/// Configuration for searching a specific collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub collection_name: String,
    pub fields: Vec<FieldSearchOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_options: Option<TextSearchOptions>,
}

/// Field-specific search options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSearchOptions {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_options: Option<TextSearchOptions>,
}

/// Text search options for chat context retrieval
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzy_match: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_stemming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_exact_matches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_edit_distance: Option<u32>,
}

/// Request to send a chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub collections: Vec<CollectionConfig>,
    pub llm_provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
}

impl ChatRequest {
    /// Create a new chat request
    pub fn new(message: impl Into<String>, llm_provider: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            collections: Vec::new(),
            llm_provider: llm_provider.into(),
            llm_model: None,
            system_prompt: None,
        }
    }

    /// Add a collection to search for context
    pub fn collection(mut self, collection: CollectionConfig) -> Self {
        self.collections.push(collection);
        self
    }

    /// Set the LLM model
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.llm_model = Some(model.into());
        self
    }

    /// Set a custom system prompt
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }
}

/// Response from a chat request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub chat_id: String,
    pub message_id: String,
    pub responses: Vec<String>,
    pub context_snippets: Vec<ContextSnippet>,
    pub execution_time_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// Token usage information from LLM providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

/// Context snippet from search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSnippet {
    pub collection: String,
    pub record: serde_json::Value,
    pub score: f64,
    pub matched_fields: Vec<String>,
}

/// Request to create a new chat session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatSessionRequest {
    pub collections: Vec<CollectionConfig>,
    pub llm_provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_ripple: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_point_idx: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_context_messages: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
}

impl CreateChatSessionRequest {
    /// Create a new chat session request
    pub fn new(llm_provider: impl Into<String>) -> Self {
        Self {
            collections: Vec::new(),
            llm_provider: llm_provider.into(),
            llm_model: None,
            system_prompt: None,
            bypass_ripple: None,
            parent_id: None,
            branch_point_idx: None,
            max_context_messages: None,
            max_tokens: None,
            temperature: None,
            tool_config: None,
        }
    }

    /// Add a collection to search
    pub fn collection(mut self, collection: CollectionConfig) -> Self {
        self.collections.push(collection);
        self
    }

    /// Set the LLM model
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.llm_model = Some(model.into());
        self
    }

    /// Set a custom system prompt
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Branch from an existing session at a specific message index
    pub fn branch_from(mut self, parent_id: impl Into<String>, branch_point_idx: usize) -> Self {
        self.parent_id = Some(parent_id.into());
        self.branch_point_idx = Some(branch_point_idx);
        self
    }

    /// Set maximum context messages
    pub fn max_context_messages(mut self, max: usize) -> Self {
        self.max_context_messages = Some(max);
        self
    }

    /// Set max tokens for LLM calls
    pub fn max_tokens(mut self, max: i32) -> Self {
        self.max_tokens = Some(max);
        self
    }

    /// Set temperature for LLM calls
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Set tool configuration
    pub fn tool_config(mut self, config: ToolConfig) -> Self {
        self.tool_config = Some(config);
        self
    }
}

/// Response containing chat session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSessionResponse {
    #[serde(default)]
    pub session: Record,
    #[serde(default)]
    pub message_count: usize,
}

/// Chat session details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub chat_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub llm_provider: String,
    pub llm_model: String,
    pub collections: Vec<CollectionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub message_count: usize,
}

/// Request to send a message in an existing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageRequest {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_ripple: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_summarize: Option<bool>,
    /// Maximum tool-calling iterations for this message.
    /// Overrides the server/session default when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iterations: Option<u32>,
    /// Override session tool config for this message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
    /// Per-message LLM model override. When set, uses this model instead of the
    /// session's configured model. Useful for routing simple steps through a
    /// faster/cheaper model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm_model: Option<String>,
}

impl ChatMessageRequest {
    /// Create a new chat message request
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            bypass_ripple: None,
            force_summarize: None,
            max_iterations: None,
            tool_config: None,
            llm_model: None,
        }
    }

    /// Force conversation summarization
    pub fn force_summarize(mut self, force: bool) -> Self {
        self.force_summarize = Some(force);
        self
    }

    /// Set maximum tool-calling iterations for this message.
    pub fn max_iterations(mut self, max: u32) -> Self {
        self.max_iterations = Some(max);
        self
    }

    /// Override session tool config for this message.
    pub fn tool_config(mut self, config: ToolConfig) -> Self {
        self.tool_config = Some(config);
        self
    }

    /// Override the session's LLM model for this message only.
    /// Useful for routing simple tool-calling steps through a faster model.
    pub fn llm_model(mut self, model: impl Into<String>) -> Self {
        self.llm_model = Some(model.into());
        self
    }
}

/// Merge strategy for combining chat sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeStrategy {
    Chronological,
    Summarized,
    LatestOnly,
    Interleaved,
}

/// Request to merge multiple chat sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeSessionsRequest {
    pub source_chat_ids: Vec<String>,
    pub target_chat_id: String,
    pub merge_strategy: MergeStrategy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_ripple: Option<bool>,
}

/// Response containing messages with pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessagesResponse {
    pub messages: Vec<Record>,
    pub total: usize,
    pub skip: usize,
    pub limit: Option<usize>,
    pub returned: usize,
}

/// Query parameters for getting messages
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl GetMessagesQuery {
    /// Create a new query
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the skip
    pub fn skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }

    /// Set the sort order ("asc" or "desc")
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }
}

/// Query parameters for listing sessions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListSessionsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl ListSessionsQuery {
    /// Create a new query
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the skip
    pub fn skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }

    /// Set the sort order ("asc" or "desc")
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }
}

/// Response containing list of chat sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSessionsResponse {
    pub sessions: Vec<ChatSession>,
    pub total: usize,
    pub returned: usize,
}

/// Request to update session metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSessionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<CollectionConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_context_messages: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_ripple: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<serde_json::Value>,
}

impl UpdateSessionRequest {
    /// Create a new update request
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the system prompt
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Set the LLM model
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.llm_model = Some(model.into());
        self
    }

    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the collections
    pub fn collections(mut self, collections: Vec<CollectionConfig>) -> Self {
        self.collections = Some(collections);
        self
    }

    /// Set maximum context messages
    pub fn max_context_messages(mut self, max: usize) -> Self {
        self.max_context_messages = Some(max);
        self
    }

    /// Set the memory object
    pub fn memory(mut self, memory: serde_json::Value) -> Self {
        self.memory = Some(memory);
        self
    }
}

/// Request to update a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageRequest {
    pub content: String,
}

/// Request to toggle message forgotten status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleForgottenRequest {
    pub forgotten: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_request_builder() {
        let request = ChatRequest::new("Hello", "openai")
            .model("gpt-4")
            .system_prompt("You are a helpful assistant");

        assert_eq!(request.message, "Hello");
        assert_eq!(request.llm_provider, "openai");
        assert_eq!(request.llm_model, Some("gpt-4".to_string()));
        assert!(request.system_prompt.is_some());
    }

    #[test]
    fn test_create_session_request_builder() {
        let request = CreateChatSessionRequest::new("openai")
            .model("gpt-4")
            .system_prompt("Test prompt");

        assert_eq!(request.llm_provider, "openai");
        assert_eq!(request.llm_model, Some("gpt-4".to_string()));
        assert!(request.system_prompt.is_some());
    }

    #[test]
    fn test_chat_message_request() {
        let request = ChatMessageRequest::new("Hello").force_summarize(true);

        assert_eq!(request.message, "Hello");
        assert_eq!(request.force_summarize, Some(true));
    }

    #[test]
    fn test_get_messages_query() {
        let query = GetMessagesQuery::new().limit(10).skip(5).sort("desc");

        assert_eq!(query.limit, Some(10));
        assert_eq!(query.skip, Some(5));
        assert_eq!(query.sort, Some("desc".to_string()));
    }

    #[test]
    fn test_list_sessions_query() {
        let query = ListSessionsQuery::new().limit(20).sort("asc");

        assert_eq!(query.limit, Some(20));
        assert_eq!(query.sort, Some("asc".to_string()));
    }

    #[test]
    fn test_update_session_request() {
        let request = UpdateSessionRequest::new()
            .title("Updated Title")
            .model("gpt-4-turbo");

        assert_eq!(request.title, Some("Updated Title".to_string()));
        assert_eq!(request.llm_model, Some("gpt-4-turbo".to_string()));
    }
}

/// Request for POST /api/chat/complete — stateless raw LLM completion.
/// No session, no history, no RAG context injection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCompletionRequest {
    /// System prompt passed verbatim to the LLM.
    pub system_prompt: String,
    /// User message passed verbatim to the LLM.
    pub message: String,
    /// LLM provider. Defaults to server's configured default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Model name. Defaults to server's configured default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Max tokens for the LLM response. Defaults to server's configured default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
}

/// Response from POST /api/chat/complete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCompletionResponse {
    /// Raw LLM response text.
    pub content: String,
}

/// Request to generate embeddings directly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Response from embedding generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedResponse {
    pub embeddings: Vec<Vec<f64>>,
    pub model: String,
    pub dimensions: usize,
}
