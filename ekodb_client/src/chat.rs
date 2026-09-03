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

/// Available LLM models from different providers, and why each list looks
/// the way it does.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Models {
    pub openai: Vec<String>,
    pub anthropic: Vec<String>,
    pub perplexity: Vec<String>,
    /// Google Gemini models. Empty from a server that predates the field.
    #[serde(default)]
    pub gemini: Vec<String>,
    /// Per-provider status, keyed by provider name (`"openai"`, `"anthropic"`,
    /// `"perplexity"`, `"gemini"`). A rejected key reports `auth_failed`
    /// where a missing one reports `not_configured`, so an empty list is
    /// never ambiguous. Empty from a server that predates the map.
    #[serde(default)]
    pub providers: std::collections::BTreeMap<String, ProviderStatus>,
}

/// A provider's state on `GET /api/chat_models`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderState {
    /// The provider listed its models with the configured key.
    Ok,
    /// No key (for an OpenAI-compatible endpoint: no key and no URL).
    NotConfigured,
    /// The provider rejected the key (401).
    AuthFailed,
    /// The key is accepted but may not use this resource or region (403).
    PermissionDenied,
    /// The account cannot pay (402, or a quota / spend-limit code).
    Billing,
    /// The provider is rate limiting the server (429).
    RateLimited,
    /// The provider answered 5xx or an unusable body.
    Unavailable,
    /// Nothing answered: DNS, connect, TLS, or a timeout.
    Unreachable,
    /// The provider refused the request itself (any other 4xx).
    RequestError,
    /// A status this client version does not know: the server is newer.
    #[serde(other)]
    Unknown,
}

/// One provider's row in [`Models::providers`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderStatus {
    pub status: ProviderState,
    /// True when the status is the provider's own answer about the configured
    /// key. A 5xx, a refused connection, or a missing key says nothing about it.
    pub verified: bool,
    /// The provider's own HTTP status, when it answered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    /// The provider's own message, when it answered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// How many models were listed, when the status is `ok`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_count: Option<usize>,
}

impl ProviderStatus {
    /// True when the provider listed its models with the configured key.
    pub fn is_usable(&self) -> bool {
        self.status == ProviderState::Ok
    }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
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
            agent_id: None,
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

    /// Set the agent ID for this session
    pub fn agent_id(mut self, id: impl Into<String>) -> Self {
        self.agent_id = Some(id.into());
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub message_count: usize,
}

/// Inline multimodal attachment for a chat message. `mime_type`
/// follows IANA (`image/png`, `application/pdf`, etc); `data` is
/// the base64-encoded payload. ekoDB routes large files through
/// per-provider File APIs server-side, so the client always sends
/// base64 regardless of size.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub mime_type: String,
    pub data: String,
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
    /// Client-side tool definitions. When provided over SSE, ekoDB merges these
    /// with built-in tools and routes calls back via `__client_tool_call` events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_tools: Option<Vec<ClientToolDef>>,
    /// Tools that require client confirmation before server-side execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_tools: Option<Vec<String>>,
    /// Tools to exclude from the LLM's tool list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_tools: Option<Vec<String>>,
    /// Multimodal attachments sent with this turn. Under ~20 MB each
    /// stay inline; larger ones are routed through the provider's
    /// File API on the server side. Wire format matches the server's
    /// `AgentChatRequest.attachments`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
}

/// Client tool definition sent with chat messages (SSE path).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientToolDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
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
            client_tools: None,
            confirm_tools: None,
            exclude_tools: None,
            attachments: None,
        }
    }

    /// Attach multimodal inputs (images, PDFs, audio). Each item is
    /// base64-encoded; large items are routed through the provider's
    /// File API server-side.
    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        self.attachments = Some(attachments);
        self
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

    /// Set client-side tool definitions for SSE-path tool routing.
    pub fn client_tools(mut self, tools: Vec<ClientToolDef>) -> Self {
        self.client_tools = Some(tools);
        self
    }

    /// Set tools that require client confirmation before server execution.
    pub fn confirm_tools(mut self, tools: Vec<String>) -> Self {
        self.confirm_tools = Some(tools);
        self
    }

    /// Set tools to exclude from the LLM's tool list.
    pub fn exclude_tools(mut self, tools: Vec<String>) -> Self {
        self.exclude_tools = Some(tools);
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

/// Request to compact a chat session's history on demand.
///
/// Folds the older messages of the session into a single summary message and
/// marks the originals "forgotten" so they stop being replayed, reclaiming
/// context-window budget while keeping a faithful summary in the prompt.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompactChatRequest {
    /// Number of most-recent messages to keep verbatim. Everything older is
    /// folded into the summary. Defaults server-side to the session's
    /// `max_context_messages` (or 50). `0` compacts the entire history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_recent: Option<usize>,
    // No `bypass_ripple`: compaction writes chat-message records, which the
    // server does not ripple (same convention as all chat-message writes).
}

/// Result of compacting a chat session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactChatResponse {
    /// Number of older messages folded into the summary (marked forgotten).
    pub folded: usize,
    /// Number of recent messages kept verbatim.
    pub kept_recent: usize,
    /// Character length of the inserted summary (0 when nothing was folded).
    pub summary_chars: usize,
    /// ID of the inserted synthetic summary message (None when nothing folded).
    pub summary_message_id: Option<String>,
    /// True when there was nothing to fold (history already within keep_recent).
    pub already_compact: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SERVER_WITH_STATUS: &str = r#"{
        "openai": [],
        "anthropic": ["claude-sonnet-4-5"],
        "perplexity": ["sonar"],
        "gemini": ["gemini-2.5-flash"],
        "providers": {
            "anthropic": {"status": "ok", "verified": true, "model_count": 1},
            "gemini": {"status": "ok", "verified": true, "model_count": 1},
            "openai": {"status": "auth_failed", "verified": true, "http_status": 401,
                       "message": "Failed to fetch OpenAI models: 401 Unauthorized"},
            "perplexity": {"status": "ok", "verified": false,
                           "message": "static model list; key not verified"}
        }
    }"#;

    #[test]
    fn models_carry_gemini_and_the_per_provider_status() {
        let models: Models = serde_json::from_str(SERVER_WITH_STATUS).unwrap();
        assert_eq!(models.gemini, vec!["gemini-2.5-flash"]);
        let openai = &models.providers["openai"];
        assert_eq!(openai.status, ProviderState::AuthFailed);
        assert!(openai.verified);
        assert_eq!(openai.http_status, Some(401));
        assert_eq!(
            openai.message.as_deref(),
            Some("Failed to fetch OpenAI models: 401 Unauthorized")
        );
        assert_eq!(openai.model_count, None);
        let anthropic = &models.providers["anthropic"];
        assert_eq!(anthropic.status, ProviderState::Ok);
        assert_eq!(anthropic.model_count, Some(1));
        assert!(!models.providers["perplexity"].verified);
    }

    #[test]
    fn models_from_a_server_without_status_default_to_empty() {
        // A server older than the `providers` map still parses.
        let models: Models =
            serde_json::from_str(r#"{"openai":["gpt-4o"],"anthropic":[],"perplexity":[]}"#)
                .unwrap();
        assert_eq!(models.openai, vec!["gpt-4o"]);
        assert!(models.gemini.is_empty());
        assert!(models.providers.is_empty());
    }

    #[test]
    fn an_unknown_status_name_is_kept_rather_than_rejected() {
        // A newer server may add a status; the client must not fail to parse.
        let status: ProviderStatus =
            serde_json::from_str(r#"{"status":"brand_new","verified":false}"#).unwrap();
        assert_eq!(status.status, ProviderState::Unknown);
    }

    #[test]
    fn provider_status_usable_means_ok() {
        let ok: ProviderStatus =
            serde_json::from_str(r#"{"status":"ok","verified":true,"model_count":3}"#).unwrap();
        assert!(ok.is_usable());
        let rejected: ProviderStatus =
            serde_json::from_str(r#"{"status":"auth_failed","verified":true}"#).unwrap();
        assert!(!rejected.is_usable());
    }

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
