//! Main client implementation for ekoDB

use crate::auth::AuthManager;
use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::schema::{CollectionMetadata, Schema};
use crate::search::{SearchQuery, SearchResponse};
use crate::types::{Query, Record};
use std::sync::Arc;
use std::time::Duration;

/// Rate limit information from the server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimitInfo {
    /// Maximum requests allowed per window
    pub limit: usize,
    /// Requests remaining in current window
    pub remaining: usize,
    /// Unix timestamp when the rate limit resets
    pub reset: i64,
}

impl RateLimitInfo {
    /// Check if the rate limit is close to being exceeded
    ///
    /// Returns true if remaining requests are less than 10% of the limit
    pub fn is_near_limit(&self) -> bool {
        let threshold = (self.limit as f64 * 0.1) as usize;
        self.remaining <= threshold
    }

    /// Check if the rate limit has been exceeded
    pub fn is_exceeded(&self) -> bool {
        self.remaining == 0
    }

    /// Get the percentage of requests remaining
    pub fn remaining_percentage(&self) -> f64 {
        (self.remaining as f64 / self.limit as f64) * 100.0
    }
}

/// ekoDB client
#[derive(Clone)]
pub struct Client {
    http: Arc<HttpClient>,
    auth: Arc<AuthManager>,
}

impl Client {
    /// Create a new client builder
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Insert a record into a collection
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `record` - The record to insert
    ///
    /// # Returns
    ///
    /// The inserted record with server-generated fields (e.g., `id`, `_created_at`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::{Client, Record};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_token("your-token")
    ///     .build()?;
    ///
    /// let mut record = Record::new();
    /// record.insert("name", "John Doe");
    /// record.insert("age", 30);
    ///
    /// let result = client.insert("users", record).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn insert(
        &self,
        collection: &str,
        record: Record,
        bypass_ripple: Option<bool>,
    ) -> Result<Record> {
        let token = self.auth.get_token().await?;
        self.http
            .insert(collection, record, bypass_ripple, &token)
            .await
    }

    /// Find records in a collection
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `query` - The query to filter records
    ///
    /// # Returns
    ///
    /// A vector of matching records
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::{Client, Query};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// let query = Query::new()
    ///     .filter(serde_json::json!({
    ///         "type": "Condition",
    ///         "content": {
    ///             "field": "age",
    ///             "operator": "Gte",
    ///             "value": 18
    ///         }
    ///     }))
    ///     .limit(10);
    /// let results = client.find("users", query).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find(
        &self,
        collection: &str,
        query: Query,
        bypass_ripple: Option<bool>,
    ) -> Result<Vec<Record>> {
        let token = self.auth.get_token().await?;
        self.http
            .find(collection, query, &token, bypass_ripple)
            .await
    }

    /// Find a single record by ID
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID
    ///
    /// # Returns
    ///
    /// The record if found, or `Error::NotFound` if not found
    pub async fn find_by_id(
        &self,
        collection: &str,
        id: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<Record> {
        let token = self.auth.get_token().await?;
        self.http
            .find_by_id(collection, id, &token, bypass_ripple)
            .await
    }

    /// Update a record by ID
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID
    /// * `record` - The updated record data
    ///
    /// # Returns
    ///
    /// The updated record
    pub async fn update(
        &self,
        collection: &str,
        id: &str,
        record: Record,
        bypass_ripple: Option<bool>,
    ) -> Result<Record> {
        let token = self.auth.get_token().await?;
        self.http
            .update(collection, id, record, bypass_ripple, &token)
            .await
    }

    /// Delete a record by ID
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID
    ///
    /// # Returns
    ///
    /// `Ok(())` if the record was deleted successfully
    pub async fn delete(
        &self,
        collection: &str,
        id: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http
            .delete(collection, id, &token, bypass_ripple)
            .await
    }

    /// Batch insert multiple documents
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `records` - Vector of records to insert
    ///
    /// # Returns
    ///
    /// Vector of inserted records with their IDs
    pub async fn batch_insert(
        &self,
        collection: &str,
        records: Vec<Record>,
        bypass_ripple: Option<bool>,
    ) -> Result<Vec<Record>> {
        let token = self.auth.get_token().await?;
        self.http
            .batch_insert(collection, records, &token, bypass_ripple)
            .await
    }

    /// Batch update multiple documents
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `updates` - Vector of (id, record) pairs to update
    ///
    /// # Returns
    ///
    /// Vector of updated records
    pub async fn batch_update(
        &self,
        collection: &str,
        updates: Vec<(String, Record)>,
        bypass_ripple: Option<bool>,
    ) -> Result<Vec<Record>> {
        let token = self.auth.get_token().await?;
        self.http
            .batch_update(collection, updates, &token, bypass_ripple)
            .await
    }

    /// Batch delete multiple documents by IDs
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `ids` - Vector of document IDs to delete
    ///
    /// # Returns
    ///
    /// The number of records deleted
    pub async fn batch_delete(
        &self,
        collection: &str,
        ids: Vec<String>,
        bypass_ripple: Option<bool>,
    ) -> Result<u64> {
        let token = self.auth.get_token().await?;
        self.http
            .batch_delete(collection, ids, &token, bypass_ripple)
            .await
    }

    /// Refresh the authentication token
    ///
    /// Clears the cached token and fetches a new one from the server.
    /// This is useful when you receive a 401 Unauthorized error.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// // If you get a 401 error, refresh the token
    /// client.refresh_token().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refresh_token(&self) -> Result<String> {
        self.auth.refresh_token().await
    }

    /// Clear the cached authentication token
    ///
    /// This will force a new token to be fetched on the next request.
    /// Useful for testing or when you know the token has expired.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example(client: &Client) {
    /// // Clear the cached token
    /// client.clear_token_cache().await;
    /// # }
    /// ```
    pub async fn clear_token_cache(&self) {
        self.auth.clear_cache().await
    }

    /// List all collections
    ///
    /// # Returns
    ///
    /// A vector of collection names
    pub async fn list_collections(&self) -> Result<Vec<String>> {
        let token = self.auth.get_token().await?;
        self.http.list_collections(&token).await
    }

    /// Delete a collection
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name to delete
    pub async fn delete_collection(&self, collection: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.delete_collection(collection, &token).await
    }

    /// Count documents in a collection
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    ///
    /// # Returns
    ///
    /// The number of documents in the collection
    pub async fn count_documents(&self, collection: &str) -> Result<usize> {
        let query = Query::new().limit(100000); // Large limit to get all
        let records = self.find(collection, query, None).await?;
        Ok(records.len())
    }

    /// Check if a collection exists
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    ///
    /// # Returns
    ///
    /// `true` if the collection exists, `false` otherwise
    pub async fn collection_exists(&self, collection: &str) -> Result<bool> {
        let collections = self.list_collections().await?;
        Ok(collections.contains(&collection.to_string()))
    }

    /// Set a key-value pair
    ///
    /// # Arguments
    ///
    /// * `key` - The key
    /// * `value` - The value (any JSON-serializable type)
    pub async fn kv_set(&self, key: &str, value: serde_json::Value) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.kv_set(key, value, &token).await
    }

    /// Get a key-value pair
    ///
    /// # Arguments
    ///
    /// * `key` - The key
    ///
    /// # Returns
    ///
    /// The value if found, or `None` if not found
    pub async fn kv_get(&self, key: &str) -> Result<Option<serde_json::Value>> {
        let token = self.auth.get_token().await?;
        self.http.kv_get(key, &token).await
    }

    /// Delete a key-value pair
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete
    pub async fn kv_delete(&self, key: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.kv_delete(key, &token).await
    }

    /// Connect to WebSocket endpoint
    ///
    /// # Arguments
    ///
    /// * `ws_url` - The WebSocket URL (e.g., "ws://localhost:8080/ws")
    ///
    /// # Returns
    ///
    /// A WebSocket client for real-time operations
    pub async fn websocket(&self, ws_url: &str) -> Result<crate::websocket::WebSocketClient> {
        let token = self.auth.get_token().await?;
        crate::websocket::WebSocketClient::new(ws_url, token)
    }

    /// Perform a full-text search
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `search_query` - The search query with options
    ///
    /// # Returns
    ///
    /// Search results with scores and matched fields
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::{Client, SearchQuery};
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// let query = SearchQuery::new("john doe")
    ///     .fields("name,email")
    ///     .fuzzy(true)
    ///     .min_score(0.5);
    ///
    /// let results = client.search("users", query).await?;
    /// println!("Found {} results", results.total);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(
        &self,
        collection: &str,
        search_query: SearchQuery,
    ) -> Result<SearchResponse> {
        let token = self.auth.get_token().await?;
        self.http.search(collection, search_query, &token).await
    }

    /// Create a collection with schema
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `schema` - The schema definition
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::{Client, Schema, FieldTypeSchema};
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// let schema = Schema::new()
    ///     .add_field("name", FieldTypeSchema::new("string").required())
    ///     .add_field("email", FieldTypeSchema::new("string").unique())
    ///     .add_field("age", FieldTypeSchema::new("number"));
    ///
    /// client.create_collection("users", schema).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_collection(&self, collection: &str, schema: Schema) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http
            .create_collection(collection, schema, &token)
            .await
    }

    /// Get collection metadata and schema
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    ///
    /// # Returns
    ///
    /// Collection metadata including schema and analytics
    pub async fn get_collection(&self, collection: &str) -> Result<CollectionMetadata> {
        let token = self.auth.get_token().await?;
        self.http.get_collection(collection, &token).await
    }

    /// Get collection schema
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    ///
    /// # Returns
    ///
    /// The collection schema
    pub async fn get_schema(&self, collection: &str) -> Result<Schema> {
        let token = self.auth.get_token().await?;
        self.http.get_schema(collection, &token).await
    }

    // ========================================================================
    // Chat Operations
    // ========================================================================

    /// Get all available chat models
    ///
    /// # Returns
    ///
    /// List of available models from all providers
    pub async fn get_chat_models(&self) -> Result<crate::chat::Models> {
        let token = self.auth.get_token().await?;
        self.http.get_chat_models(&token).await
    }

    /// Get specific chat model information
    ///
    /// # Arguments
    ///
    /// * `model_name` - Name of the model provider (e.g., "openai", "anthropic")
    pub async fn get_chat_model(&self, model_name: &str) -> Result<Vec<String>> {
        let token = self.auth.get_token().await?;
        self.http.get_chat_model(model_name, &token).await
    }

    /// Create a new chat session
    ///
    /// # Arguments
    ///
    /// * `request` - The session creation request
    ///
    /// # Returns
    ///
    /// The created session information
    pub async fn create_chat_session(
        &self,
        request: crate::chat::CreateChatSessionRequest,
    ) -> Result<crate::chat::ChatResponse> {
        let token = self.auth.get_token().await?;
        self.http.create_chat_session(request, &token).await
    }

    /// Get a chat session by ID
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    pub async fn get_chat_session(
        &self,
        chat_id: &str,
    ) -> Result<crate::chat::ChatSessionResponse> {
        let token = self.auth.get_token().await?;
        self.http.get_chat_session(chat_id, &token).await
    }

    /// List all chat sessions
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters for pagination and sorting
    pub async fn list_chat_sessions(
        &self,
        query: crate::chat::ListSessionsQuery,
    ) -> Result<crate::chat::ListSessionsResponse> {
        let token = self.auth.get_token().await?;
        self.http.list_chat_sessions(query, &token).await
    }

    /// Update chat session metadata
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    /// * `request` - The update request
    pub async fn update_chat_session(
        &self,
        chat_id: &str,
        request: crate::chat::UpdateSessionRequest,
    ) -> Result<crate::chat::ChatSessionResponse> {
        let token = self.auth.get_token().await?;
        self.http
            .update_chat_session(chat_id, request, &token)
            .await
    }

    /// Delete a chat session
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID to delete
    pub async fn delete_chat_session(&self, chat_id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.delete_chat_session(chat_id, &token).await
    }

    /// Branch a chat session from an existing one
    ///
    /// # Arguments
    ///
    /// * `request` - The branch request with parent session info
    pub async fn branch_chat_session(
        &self,
        request: crate::chat::CreateChatSessionRequest,
    ) -> Result<crate::chat::ChatResponse> {
        let token = self.auth.get_token().await?;
        self.http.branch_chat_session(request, &token).await
    }

    /// Merge multiple chat sessions
    ///
    /// # Arguments
    ///
    /// * `request` - The merge request with source and target sessions
    pub async fn merge_chat_sessions(
        &self,
        request: crate::chat::MergeSessionsRequest,
    ) -> Result<crate::chat::ChatSessionResponse> {
        let token = self.auth.get_token().await?;
        self.http.merge_chat_sessions(request, &token).await
    }

    /// Send a message in an existing chat session
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    /// * `request` - The message request
    pub async fn chat_message(
        &self,
        chat_id: &str,
        request: crate::chat::ChatMessageRequest,
    ) -> Result<crate::chat::ChatResponse> {
        let token = self.auth.get_token().await?;
        self.http.chat_message(chat_id, request, &token).await
    }

    /// Get messages from a chat session
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    /// * `query` - Query parameters for pagination and sorting
    pub async fn get_chat_session_messages(
        &self,
        chat_id: &str,
        query: crate::chat::GetMessagesQuery,
    ) -> Result<crate::chat::GetMessagesResponse> {
        let token = self.auth.get_token().await?;
        self.http
            .get_chat_session_messages(chat_id, query, &token)
            .await
    }

    /// Get a specific message by ID
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    /// * `message_id` - The message ID
    pub async fn get_chat_message(&self, chat_id: &str, message_id: &str) -> Result<Record> {
        let token = self.auth.get_token().await?;
        self.http
            .get_chat_message(chat_id, message_id, &token)
            .await
    }

    /// Update a chat message
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    /// * `message_id` - The message ID
    /// * `request` - The update request
    pub async fn update_chat_message(
        &self,
        chat_id: &str,
        message_id: &str,
        request: crate::chat::UpdateMessageRequest,
    ) -> Result<Record> {
        let token = self.auth.get_token().await?;
        self.http
            .update_chat_message(chat_id, message_id, request, &token)
            .await
    }

    /// Delete a chat message
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    /// * `message_id` - The message ID to delete
    pub async fn delete_chat_message(&self, chat_id: &str, message_id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http
            .delete_chat_message(chat_id, message_id, &token)
            .await
    }

    /// Toggle message forgotten status
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    /// * `message_id` - The message ID
    /// * `request` - The toggle request
    pub async fn toggle_forgotten_message(
        &self,
        chat_id: &str,
        message_id: &str,
        request: crate::chat::ToggleForgottenRequest,
    ) -> Result<Record> {
        let token = self.auth.get_token().await?;
        self.http
            .toggle_forgotten_message(chat_id, message_id, request, &token)
            .await
    }

    /// Regenerate a chat message
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The session ID
    /// * `message_id` - The message ID to regenerate
    pub async fn regenerate_chat_message(
        &self,
        chat_id: &str,
        message_id: &str,
    ) -> Result<crate::chat::ChatResponse> {
        let token = self.auth.get_token().await?;
        self.http
            .regenerate_chat_message(chat_id, message_id, &token)
            .await
    }
}

/// Builder for creating a Client
#[derive(Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<usize>,
    should_retry: Option<bool>,
    serialization_format: Option<crate::types::SerializationFormat>,
}

impl ClientBuilder {
    /// Create a new ClientBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL for the ekoDB server
    ///
    /// # Example
    ///
    /// ```
    /// use ekodb_client::Client;
    ///
    /// let client = Client::builder()
    ///     .base_url("https://api.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .build()?;
    /// # Ok::<(), ekodb_client::Error>(())
    /// ```
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the API key for authentication
    ///
    /// The API key will be exchanged for a JWT token automatically.
    ///
    /// # Example
    ///
    /// ```
    /// use ekodb_client::Client;
    ///
    /// let client = Client::builder()
    ///     .base_url("https://api.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .build()?;
    /// # Ok::<(), ekodb_client::Error>(())
    /// ```
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the API token for authentication (alias for api_key for backward compatibility)
    #[deprecated(since = "0.1.0", note = "Use `api_key` instead")]
    pub fn api_token(mut self, token: impl Into<String>) -> Self {
        self.api_key = Some(token.into());
        self
    }

    /// Set the request timeout
    ///
    /// Default: 30 seconds
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retry attempts
    ///
    /// Default: 3
    pub fn max_retries(mut self, retries: usize) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Enable or disable automatic retries for rate limiting and transient errors
    ///
    /// When enabled (default), the client will automatically retry requests that fail
    /// due to rate limiting (429), service unavailable (503), timeouts, or connection errors.
    /// The retry delay respects the server's `Retry-After` header for rate limits.
    ///
    /// When disabled, all errors are returned immediately to the caller for manual handling.
    ///
    /// Default: true
    ///
    /// # Example
    ///
    /// ```
    /// use ekodb_client::Client;
    ///
    /// // Disable automatic retries
    /// let client = Client::builder()
    ///     .base_url("https://api.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .should_retry(false)
    ///     .build()?;
    /// # Ok::<(), ekodb_client::Error>(())
    /// ```
    pub fn should_retry(mut self, should_retry: bool) -> Self {
        self.should_retry = Some(should_retry);
        self
    }

    /// Set the serialization format for client-server communication
    ///
    /// Supports JSON (default, human-readable) and MessagePack (binary, faster).
    /// MessagePack can provide 2-3x performance improvement over JSON.
    ///
    /// Default: JSON
    ///
    /// # Example
    ///
    /// ```
    /// use ekodb_client::{Client, SerializationFormat};
    ///
    /// // Use MessagePack for better performance
    /// let client = Client::builder()
    ///     .base_url("https://api.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .serialization_format(SerializationFormat::MessagePack)
    ///     .build()?;
    /// # Ok::<(), ekodb_client::Error>(())
    /// ```
    pub fn serialization_format(mut self, format: crate::types::SerializationFormat) -> Self {
        self.serialization_format = Some(format);
        self
    }

    /// Build the client
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing or invalid
    pub fn build(self) -> Result<Client> {
        let base_url_str = self
            .base_url
            .ok_or_else(|| Error::InvalidConfig("base_url is required".to_string()))?;

        let api_key = self
            .api_key
            .ok_or_else(|| Error::InvalidConfig("api_key is required".to_string()))?;

        let timeout = self.timeout.unwrap_or(Duration::from_secs(30));
        let max_retries = self.max_retries.unwrap_or(3);
        let should_retry = self.should_retry.unwrap_or(true); // Default to true
        let format = self
            .serialization_format
            .unwrap_or(crate::types::SerializationFormat::MessagePack); // Default to MessagePack for 2-3x performance

        // Parse base URL
        let base_url = url::Url::parse(&base_url_str)?;

        // Create HTTP client with specified format
        let http = HttpClient::new(
            &base_url_str,
            timeout,
            max_retries as u32,
            should_retry,
            format,
        )?;

        // Create reqwest client for auth
        let reqwest_client = reqwest::Client::builder().timeout(timeout).build()?;

        // Create auth manager with API key
        let auth = AuthManager::new(api_key, base_url, reqwest_client);

        Ok(Client {
            http: Arc::new(http),
            auth: Arc::new(auth),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder_new() {
        let builder = ClientBuilder::new();
        assert!(builder.base_url.is_none());
        assert!(builder.api_key.is_none());
    }

    #[test]
    fn test_client_builder_default() {
        let builder = ClientBuilder::default();
        assert!(builder.base_url.is_none());
        assert!(builder.api_key.is_none());
    }

    #[test]
    fn test_client_builder_with_values() {
        let builder = ClientBuilder::new()
            .base_url("http://localhost:8080")
            .api_key("test-key")
            .timeout(Duration::from_secs(30))
            .max_retries(5);

        assert_eq!(builder.base_url, Some("http://localhost:8080".to_string()));
        assert_eq!(builder.api_key, Some("test-key".to_string()));
        assert_eq!(builder.timeout, Some(Duration::from_secs(30)));
        assert_eq!(builder.max_retries, Some(5));
    }

    #[test]
    fn test_client_builder_missing_base_url() {
        let result = ClientBuilder::new().api_key("test-key").build();

        assert!(result.is_err());
        match result {
            Err(crate::Error::InvalidConfig(msg)) => {
                assert!(msg.contains("base_url"));
            }
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_client_builder_missing_api_key() {
        let result = ClientBuilder::new()
            .base_url("http://localhost:8080")
            .build();

        assert!(result.is_err());
        match result {
            Err(crate::Error::InvalidConfig(msg)) => {
                assert!(msg.contains("api_key"));
            }
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_client_builder_invalid_url() {
        let result = ClientBuilder::new()
            .base_url("not-a-valid-url")
            .api_key("test-key")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_client_builder_valid() {
        let result = ClientBuilder::new()
            .base_url("http://localhost:8080")
            .api_key("test-key")
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_builder_with_custom_timeout() {
        let result = ClientBuilder::new()
            .base_url("http://localhost:8080")
            .api_key("test-key")
            .timeout(Duration::from_secs(60))
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_builder_with_custom_retries() {
        let result = ClientBuilder::new()
            .base_url("http://localhost:8080")
            .api_key("test-key")
            .max_retries(10)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_builder_with_retry_enabled() {
        let result = ClientBuilder::new()
            .base_url("http://localhost:8080")
            .api_key("test-key")
            .should_retry(true)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_builder_with_retry_disabled() {
        let result = ClientBuilder::new()
            .base_url("http://localhost:8080")
            .api_key("test-key")
            .should_retry(false)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_builder_method() {
        let builder = Client::builder();
        assert!(builder.base_url.is_none());
    }

    #[test]
    fn test_query_new() {
        let query = Query::new();
        assert!(query.filter.is_none());
        assert!(query.sort.is_none());
        assert!(query.limit.is_none());
        assert!(query.skip.is_none());
    }

    #[test]
    fn test_query_with_filter() {
        let query = Query::new().filter(serde_json::json!({"name": "test"}));
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_query_with_sort() {
        let query = Query::new().sort(serde_json::json!({"created_at": -1}));
        assert!(query.sort.is_some());
    }

    #[test]
    fn test_query_with_limit() {
        let query = Query::new().limit(10);
        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn test_query_with_skip() {
        let query = Query::new().skip(20);
        assert_eq!(query.skip, Some(20));
    }

    #[test]
    fn test_query_with_bypass_cache() {
        let query = Query::new().bypass_cache(true);
        assert_eq!(query.bypass_cache, Some(true));
    }

    #[test]
    fn test_query_with_bypass_ripple() {
        let query = Query::new().bypass_ripple(true);
        assert_eq!(query.bypass_ripple, Some(true));
    }

    #[test]
    fn test_query_with_join() {
        let join = serde_json::json!({
            "collections": ["users"],
            "local_field": "user_id",
            "foreign_field": "id",
            "as_field": "user"
        });
        let query = Query::new().join(join.clone());
        assert_eq!(query.join, Some(join));
    }

    #[test]
    fn test_query_builder_chaining() {
        let query = Query::new()
            .filter(serde_json::json!({"status": "active"}))
            .sort(serde_json::json!({"created_at": -1}))
            .limit(10)
            .skip(20)
            .bypass_cache(true);

        assert!(query.filter.is_some());
        assert!(query.sort.is_some());
        assert_eq!(query.limit, Some(10));
        assert_eq!(query.skip, Some(20));
        assert_eq!(query.bypass_cache, Some(true));
    }

    #[test]
    fn test_record_new() {
        let record = Record::new();
        assert!(record.is_empty());
        assert_eq!(record.len(), 0);
    }

    #[test]
    fn test_record_insert_and_get() {
        let mut record = Record::new();
        record.insert("name", "test");

        assert!(!record.is_empty());
        assert_eq!(record.len(), 1);
        assert!(record.get("name").is_some());
    }

    #[test]
    fn test_record_contains_key() {
        let mut record = Record::new();
        record.insert("name", "test");

        assert!(record.contains_key("name"));
        assert!(!record.contains_key("age"));
    }

    #[test]
    fn test_rate_limit_info_is_near_limit() {
        let info = RateLimitInfo {
            limit: 1000,
            remaining: 50, // 5% remaining
            reset: 1234567890,
        };
        assert!(info.is_near_limit());

        let info2 = RateLimitInfo {
            limit: 1000,
            remaining: 500, // 50% remaining
            reset: 1234567890,
        };
        assert!(!info2.is_near_limit());
    }

    #[test]
    fn test_rate_limit_info_is_exceeded() {
        let info = RateLimitInfo {
            limit: 1000,
            remaining: 0,
            reset: 1234567890,
        };
        assert!(info.is_exceeded());

        let info2 = RateLimitInfo {
            limit: 1000,
            remaining: 1,
            reset: 1234567890,
        };
        assert!(!info2.is_exceeded());
    }

    #[test]
    fn test_rate_limit_info_remaining_percentage() {
        let info = RateLimitInfo {
            limit: 1000,
            remaining: 250,
            reset: 1234567890,
        };
        assert_eq!(info.remaining_percentage(), 25.0);

        let info2 = RateLimitInfo {
            limit: 1000,
            remaining: 0,
            reset: 1234567890,
        };
        assert_eq!(info2.remaining_percentage(), 0.0);
    }

    #[test]
    fn test_record_remove() {
        let mut record = Record::new();
        record.insert("name", "test");

        let removed = record.remove("name");
        assert!(removed.is_some());
        assert!(!record.contains_key("name"));
    }
}
