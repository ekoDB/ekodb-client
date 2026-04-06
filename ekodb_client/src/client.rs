//! Main client implementation for ekoDB

use crate::auth::AuthManager;
use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::schema::{CollectionMetadata, Schema};
use crate::search::{DistinctValuesQuery, DistinctValuesResponse, SearchQuery, SearchResponse};
use crate::types::{FieldType, Query, Record};
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
    /// Opt-in schema cache for primary_key_alias resolution and field validation.
    schema_cache: Arc<crate::schema_cache::SchemaCache>,
    /// Base URL stored for WS URL derivation in connect_ws()
    base_url: String,
}

impl Client {
    /// Create a new client builder
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Health check
    pub async fn health_check(&self) -> Result<()> {
        self.http.health_check().await
    }

    /// Execute an operation with automatic token refresh on TokenExpired errors
    async fn execute_with_token_refresh<F, Fut, T>(&self, mut operation: F) -> Result<T>
    where
        F: FnMut(String) -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // First attempt with current token
        let token = self.auth.get_token().await?;
        match operation(token).await {
            Ok(result) => Ok(result),
            Err(Error::TokenExpired) => {
                // Token expired, refresh and retry once
                log::debug!("Token expired, refreshing and retrying...");
                let new_token = self.auth.refresh_token().await?;
                operation(new_token).await
            }
            Err(e) => Err(e),
        }
    }

    /// Insert a record into a collection
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `record` - The record to insert
    /// * `options` - Optional insert options (TTL, bypass_ripple, transaction_id, bypass_cache)
    ///
    /// # Returns
    ///
    /// The inserted record with server-generated fields (e.g., `id`, `_created_at`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::{Client, Record};
    /// # use ekodb_client::options::InsertOptions;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_key("your-token")
    ///     .build()?;
    ///
    /// let mut record = Record::new();
    /// record.insert("name", "John Doe");
    /// record.insert("age", 30);
    ///
    /// // Simple insert
    /// let result = client.insert("users", record.clone(), None).await?;
    ///
    /// // Insert with TTL (expires in 1 hour)
    /// let options = InsertOptions::new().ttl("1h");
    /// let result = client.insert("sessions", record, Some(options)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn insert(
        &self,
        collection: &str,
        record: Record,
        options: Option<crate::options::InsertOptions>,
    ) -> Result<Record> {
        let collection = collection.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let record = record.clone();
            let http = http.clone();
            let options = options.clone();
            async move { http.insert(&collection, record, options, &token).await }
        })
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
    /// let results = client.find("users", query, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find(
        &self,
        collection: &str,
        query: Query,
        bypass_ripple: Option<bool>,
    ) -> Result<Vec<Record>> {
        let collection = collection.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let query = query.clone();
            let http = http.clone();
            async move { http.find(&collection, query, &token, bypass_ripple).await }
        })
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
        let collection = collection.to_string();
        let id = id.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let id = id.clone();
            let http = http.clone();
            async move {
                http.find_by_id(&collection, &id, &token, bypass_ripple)
                    .await
            }
        })
        .await
    }

    /// Update a record by ID
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID
    /// * `record` - The updated record data
    /// * `options` - Optional update options (bypass_ripple, transaction_id, bypass_cache)
    ///
    /// # Returns
    ///
    /// The updated record
    pub async fn update(
        &self,
        collection: &str,
        id: &str,
        record: Record,
        options: Option<crate::options::UpdateOptions>,
    ) -> Result<Record> {
        let collection = collection.to_string();
        let id = id.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let id = id.clone();
            let record = record.clone();
            let options = options.clone();
            let http = http.clone();
            async move { http.update(&collection, &id, record, options, &token).await }
        })
        .await
    }

    /// Apply an atomic field action to a single field of a record.
    ///
    /// Use this instead of `update()` when you need safe concurrent modifications
    /// like incrementing counters, pushing to arrays, or arithmetic operations.
    ///
    /// # Actions
    ///
    /// - `increment` / `decrement` — add or subtract a number
    /// - `multiply` / `divide` / `modulo` — arithmetic on numeric fields
    /// - `push` / `unshift` — append or prepend a value to an array
    /// - `pop` / `shift` — remove last or first element of an array
    /// - `remove` — remove a specific value from an array
    /// - `append` — concatenate a string onto a string field
    /// - `clear` — reset a field to its zero value (0, "", [])
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID
    /// * `action` - The atomic action to apply
    /// * `field` - The field name to apply the action to
    /// * `value` - The value for the action (use `FieldType::Null` for pop/shift/clear)
    pub async fn update_with_action(
        &self,
        collection: &str,
        id: &str,
        action: &str,
        field: &str,
        value: FieldType,
    ) -> Result<Record> {
        let collection = collection.to_string();
        let id = id.to_string();
        let action = action.to_string();
        let field = field.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let id = id.clone();
            let action = action.clone();
            let field = field.clone();
            let value = value.clone();
            let http = http.clone();
            async move {
                http.update_with_action(&collection, &id, &action, &field, value, &token)
                    .await
            }
        })
        .await
    }

    /// Apply a sequence of atomic field actions to a record in a single request.
    ///
    /// All actions are applied atomically — the record is fetched once, all actions
    /// run in order, and the result is persisted in a single update.
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID
    /// * `actions` - A list of (action, field, value) tuples
    pub async fn update_with_action_sequence(
        &self,
        collection: &str,
        id: &str,
        actions: Vec<(String, String, FieldType)>,
    ) -> Result<Record> {
        let token = self.auth.get_token().await?;
        self.http
            .update_with_action_sequence(collection, id, actions, &token)
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
        let collection = collection.to_string();
        let id = id.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let id = id.clone();
            let http = http.clone();
            async move { http.delete(&collection, &id, &token, bypass_ripple).await }
        })
        .await
    }

    /// Restore a deleted record from trash (undelete)
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID to restore
    ///
    /// # Returns
    ///
    /// `Ok(true)` if the record was restored, `Ok(false)` if no tombstone was found
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_key("your-token")
    ///     .build()?;
    ///
    /// // Delete a record
    /// client.delete("users", "user123", None).await?;
    ///
    /// // Restore it from trash
    /// let restored = client.restore_deleted("users", "user123").await?;
    /// if restored {
    ///     println!("Record restored successfully");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn restore_deleted(&self, collection: &str, id: &str) -> Result<bool> {
        let token = self.auth.get_token().await?;
        self.http.restore_deleted(collection, id, &token).await
    }

    /// Restore all deleted records in a collection from trash
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    ///
    /// # Returns
    ///
    /// Number of records restored
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// let count = client.restore_collection("users").await?;
    /// println!("Restored {} records", count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn restore_collection(&self, collection: &str) -> Result<usize> {
        let token = self.auth.get_token().await?;
        self.http.restore_collection(collection, &token).await
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
        let collection = collection.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let records = records.clone();
            let http = http.clone();
            async move {
                http.batch_insert(&collection, records, &token, bypass_ripple)
                    .await
            }
        })
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
        let collection = collection.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let updates = updates.clone();
            let http = http.clone();
            async move {
                http.batch_update(&collection, updates, &token, bypass_ripple)
                    .await
            }
        })
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
        let collection = collection.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let ids = ids.clone();
            let http = http.clone();
            async move {
                http.batch_delete(&collection, ids, &token, bypass_ripple)
                    .await
            }
        })
        .await
    }

    // ========== Convenience Methods ==========

    /// Insert or update a record (upsert operation)
    ///
    /// Attempts to update the record first. If the record doesn't exist (NotFound error),
    /// it will be inserted instead. This provides atomic insert-or-update semantics.
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID
    /// * `record` - The record data to insert or update
    /// * `options` - Optional upsert options (TTL, bypass_ripple, transaction_id, bypass_cache)
    ///
    /// # Returns
    ///
    /// The inserted or updated record with server-generated fields
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::{Client, Record};
    /// # use ekodb_client::options::UpsertOptions;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// let mut record = Record::new();
    /// record.insert("name", "John Doe");
    /// record.insert("email", "john@example.com");
    ///
    /// // Will update if exists, insert if not
    /// let result = client.upsert("users", "user123", record.clone(), None).await?;
    ///
    /// // With TTL option
    /// let options = UpsertOptions::new().ttl("1h");
    /// let result = client.upsert("sessions", "sess123", record, Some(options)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upsert(
        &self,
        collection: &str,
        id: &str,
        record: Record,
        options: Option<crate::options::UpsertOptions>,
    ) -> Result<Record> {
        // Convert UpsertOptions to UpdateOptions for the update call
        let update_opts = options.as_ref().map(|o| {
            let mut opts = crate::options::UpdateOptions::new();
            if let Some(bypass) = o.bypass_ripple {
                opts = opts.bypass_ripple(bypass);
            }
            if let Some(ref tx_id) = o.transaction_id {
                opts = opts.transaction_id(tx_id.clone());
            }
            if let Some(bypass) = o.bypass_cache {
                opts = opts.bypass_cache(bypass);
            }
            opts
        });

        // Try update first
        match self
            .update(collection, id, record.clone(), update_opts)
            .await
        {
            Ok(updated) => Ok(updated),
            Err(Error::NotFound) => {
                // Record doesn't exist, insert it
                // Convert UpsertOptions to InsertOptions
                let insert_opts = options.map(|o| {
                    let mut opts = crate::options::InsertOptions::new();
                    if let Some(ref ttl) = o.ttl {
                        opts = opts.ttl(ttl.clone());
                    }
                    if let Some(bypass) = o.bypass_ripple {
                        opts = opts.bypass_ripple(bypass);
                    }
                    if let Some(ref tx_id) = o.transaction_id {
                        opts = opts.transaction_id(tx_id.clone());
                    }
                    if let Some(bypass) = o.bypass_cache {
                        opts = opts.bypass_cache(bypass);
                    }
                    opts
                });
                self.insert(collection, record, insert_opts).await
            }
            Err(e) => Err(e),
        }
    }

    /// Find a single record by field value
    ///
    /// Convenience method for finding one record matching a specific field value.
    /// Returns `None` if no record matches, or `Some(Record)` for the first match.
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `field` - The field name to search
    /// * `value` - The value to match
    ///
    /// # Returns
    ///
    /// `Some(Record)` if found, `None` if not found
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// // Find user by email
    /// if let Some(user) = client.find_one("users", "email", "john@example.com").await? {
    ///     println!("Found user: {:?}", user);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_one(
        &self,
        collection: &str,
        field: &str,
        value: impl Into<serde_json::Value>,
    ) -> Result<Option<Record>> {
        use crate::query_builder::QueryBuilder;

        let query = QueryBuilder::new().eq(field, value.into()).limit(1).build();

        let mut results = self.find(collection, query, None).await?;
        Ok(results.pop())
    }

    /// Check if a record exists by ID
    ///
    /// This is more efficient than fetching the record when you only need to check existence.
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `id` - The record ID to check
    ///
    /// # Returns
    ///
    /// `true` if the record exists, `false` if it doesn't
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// if client.exists("users", "user123").await? {
    ///     println!("User exists");
    /// } else {
    ///     println!("User not found");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn exists(&self, collection: &str, id: &str) -> Result<bool> {
        match self.find_by_id(collection, id, None).await {
            Ok(_) => Ok(true),
            Err(Error::NotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Paginate through records
    ///
    /// Convenience method for pagination with page numbers (1-indexed).
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `page` - The page number (1-indexed, i.e., first page is 1)
    /// * `page_size` - Number of records per page
    ///
    /// # Returns
    ///
    /// A vector of records for the requested page
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// // Get page 2 with 10 records per page
    /// let records = client.paginate("users", 2, 10).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn paginate(
        &self,
        collection: &str,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<Record>> {
        use crate::types::Query;

        // Page 1 = skip 0, Page 2 = skip page_size, etc.
        let skip = if page > 0 { (page - 1) * page_size } else { 0 };

        let query = Query {
            filter: None,
            sort: None,
            limit: Some(page_size),
            skip: Some(skip),
            join: None,
            bypass_cache: None,
            bypass_ripple: None,
            select_fields: None,
            exclude_fields: None,
        };

        self.find(collection, query, None).await
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
    /// Get a valid authentication token (JWT).
    ///
    /// Returns a cached token if available, otherwise exchanges the API key
    /// for a new JWT via `/api/auth/token`.
    pub async fn get_token(&self) -> Result<String> {
        self.auth.get_token().await
    }

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
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let http = http.clone();
            async move { http.list_collections(&token).await }
        })
        .await
    }

    /// List collections, excluding internal chat/system collections.
    pub async fn list_user_collections(&self) -> Result<Vec<String>> {
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let http = http.clone();
            async move { http.list_collections_filtered(&token, true).await }
        })
        .await
    }

    /// Delete a collection
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name to delete
    pub async fn delete_collection(&self, collection: &str) -> Result<()> {
        let collection = collection.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let http = http.clone();
            async move { http.delete_collection(&collection, &token).await }
        })
        .await
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
        // Use select_fields to return only _id (minimal data transfer).
        // No dedicated server count endpoint exists, so we fetch IDs only.
        let query = Query {
            select_fields: Some(vec!["_id".to_string()]),
            ..Query::default()
        };
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
    /// * `ttl` - Optional TTL duration (e.g., "30s", "5m", "1h")
    pub async fn kv_set(
        &self,
        key: &str,
        value: serde_json::Value,
        ttl: Option<&str>,
    ) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.kv_set(key, value, ttl, &token).await
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

    /// Check if a key exists in the KV store
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// `true` if the key exists, `false` otherwise
    pub async fn kv_exists(&self, key: &str) -> Result<bool> {
        let token = self.auth.get_token().await?;
        self.http.kv_exists(key, &token).await
    }

    /// Batch get multiple keys
    ///
    /// # Arguments
    ///
    /// * `keys` - Vector of keys to retrieve
    ///
    /// # Returns
    ///
    /// A vector of records corresponding to the keys
    pub async fn kv_batch_get(&self, keys: Vec<String>) -> Result<Vec<Record>> {
        let token = self.auth.get_token().await?;
        self.http.kv_batch_get(keys, &token).await
    }

    /// Batch set multiple key-value pairs
    ///
    /// # Arguments
    ///
    /// * `keys` - Vector of keys
    /// * `values` - Vector of values (must match length of keys)
    /// * `ttl` - Optional TTL in seconds (applied to all entries)
    ///
    /// # Returns
    ///
    /// Vector of tuples (key, was_set) indicating success for each operation
    pub async fn kv_batch_set(
        &self,
        keys: Vec<String>,
        values: Vec<Record>,
        ttl: Option<i64>,
    ) -> Result<Vec<(String, bool)>> {
        let token = self.auth.get_token().await?;
        self.http.kv_batch_set(keys, values, ttl, &token).await
    }

    /// Batch delete multiple keys
    ///
    /// # Arguments
    ///
    /// * `keys` - Vector of keys to delete
    ///
    /// # Returns
    ///
    /// Vector of tuples (key, was_deleted) indicating success for each operation
    pub async fn kv_batch_delete(&self, keys: Vec<String>) -> Result<Vec<(String, bool)>> {
        let token = self.auth.get_token().await?;
        self.http.kv_batch_delete(keys, &token).await
    }

    /// Query/find KV entries with pattern matching
    ///
    /// # Arguments
    ///
    /// * `pattern` - Optional regex pattern for keys (e.g., "cache:user:.*")
    /// * `include_expired` - Whether to include expired entries
    ///
    /// # Returns
    ///
    /// A vector of matching records
    pub async fn kv_find(
        &self,
        pattern: Option<&str>,
        include_expired: bool,
    ) -> Result<Vec<serde_json::Value>> {
        let token = self.auth.get_token().await?;
        self.http.kv_find(pattern, include_expired, &token).await
    }

    /// Alias for kv_find - query KV store with pattern
    pub async fn kv_query(
        &self,
        pattern: Option<&str>,
        include_expired: bool,
    ) -> Result<Vec<serde_json::Value>> {
        self.kv_find(pattern, include_expired).await
    }

    // ========== Transaction Methods ==========

    /// Begin a new transaction
    ///
    /// # Arguments
    ///
    /// * `isolation_level` - Transaction isolation level (e.g., "ReadCommitted")
    ///
    /// # Returns
    ///
    /// The transaction ID
    pub async fn begin_transaction(&self, isolation_level: &str) -> Result<String> {
        let token = self.auth.get_token().await?;
        self.http.begin_transaction(isolation_level, &token).await
    }

    /// Get transaction status
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction ID
    ///
    /// # Returns
    ///
    /// Transaction status including state and operations count
    pub async fn get_transaction_status(&self, transaction_id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http
            .get_transaction_status(transaction_id, &token)
            .await
    }

    /// Commit a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction ID to commit
    pub async fn commit_transaction(&self, transaction_id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.commit_transaction(transaction_id, &token).await
    }

    /// Rollback a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction ID to rollback
    pub async fn rollback_transaction(&self, transaction_id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.rollback_transaction(transaction_id, &token).await
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

    /// Get distinct (unique) values for a field across all records in a collection.
    ///
    /// Results are sorted alphabetically and deduplicated. Supports an optional filter
    /// to restrict which records are examined.
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `field` - The field to get distinct values for
    /// * `query` - Optional query with filter and bypass flags
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::{Client, DistinctValuesQuery};
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// // Get all distinct statuses
    /// let resp = client.distinct_values("orders", "status", DistinctValuesQuery::new()).await?;
    /// println!("Statuses: {:?}", resp.values);
    ///
    /// // With filter
    /// use serde_json::json;
    /// let filter = json!({"type":"Condition","content":{"field":"active","operator":"Eq","value":true}});
    /// let resp = client.distinct_values("users", "role", DistinctValuesQuery::new().filter(filter)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn distinct_values(
        &self,
        collection: &str,
        field: &str,
        query: DistinctValuesQuery,
    ) -> Result<DistinctValuesResponse> {
        let collection = collection.to_string();
        let field = field.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let field = field.clone();
            let query = query.clone();
            let http = http.clone();
            async move {
                http.distinct_values(&collection, &field, query, &token)
                    .await
            }
        })
        .await
    }

    /// Text-only search (full-text search)
    ///
    /// Convenience method for text search without vectors.
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection to search
    /// * `query_text` - The search query text
    /// * `limit` - Maximum number of results
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// let results = client.text_search("articles", "rust programming", 10).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn text_search(
        &self,
        collection: &str,
        query_text: &str,
        limit: usize,
    ) -> Result<Vec<Record>> {
        let search_query = SearchQuery::new(query_text).limit(limit);
        let response = self.search(collection, search_query).await?;

        // Convert SearchResult to Record, injecting _score so callers can access it
        let records: Vec<Record> = response
            .results
            .into_iter()
            .filter_map(|result| {
                let score = result.score;
                let mut record: Record = serde_json::from_value(result.record).ok()?;
                record.insert("_score", score);
                Some(record)
            })
            .collect();

        Ok(records)
    }

    /// Hybrid search (combines text + vector search)
    ///
    /// Performs semantic search using both text and vector embeddings.
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection to search
    /// * `query_text` - The search query text
    /// * `query_vector` - The query embedding vector
    /// * `limit` - Maximum number of results
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// let embedding = vec![0.1, 0.2, 0.3]; // From embed() call
    /// let results = client.hybrid_search(
    ///     "articles",
    ///     "rust programming",
    ///     embedding,
    ///     10
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn hybrid_search(
        &self,
        collection: &str,
        query_text: &str,
        query_vector: Vec<f64>,
        limit: usize,
    ) -> Result<Vec<Record>> {
        let search_query = SearchQuery::new(query_text)
            .vector(query_vector)
            .text_weight(0.5)
            .vector_weight(0.5)
            .limit(limit);

        let response = self.search(collection, search_query).await?;

        // Convert SearchResult to Record, injecting _score so callers can access it
        let records: Vec<Record> = response
            .results
            .into_iter()
            .filter_map(|result| {
                let score = result.score;
                let mut record: Record = serde_json::from_value(result.record).ok()?;
                record.insert("_score", score);
                Some(record)
            })
            .collect();

        Ok(records)
    }

    /// Find all records in a collection
    ///
    /// Convenience method to retrieve all records (up to a limit).
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name
    /// * `limit` - Maximum number of records to return
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// let all_records = client.find_all("users", 100).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_all(&self, collection: &str, limit: usize) -> Result<Vec<Record>> {
        use crate::types::Query;

        let query = Query {
            filter: None,
            sort: None,
            limit: Some(limit),
            skip: None,
            join: None,
            bypass_cache: None,
            bypass_ripple: None,
            select_fields: Some(Vec::new()),
            exclude_fields: Some(Vec::new()),
        };

        self.find(collection, query, None).await
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
        let collection = collection.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let schema = schema.clone();
            let http = http.clone();
            async move { http.create_collection(&collection, schema, &token).await }
        })
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
        let collection = collection.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let collection = collection.clone();
            let http = http.clone();
            async move { http.get_schema(&collection, &token).await }
        })
        .await
    }

    // ========================================================================
    // Tool Dispatch
    // ========================================================================

    /// Execute a tool via ekoDB's server-side tool pipeline.
    ///
    /// Calls `POST /api/chat/tools/execute` which goes through the same
    /// `execute_tool` function as the LLM tool-calling loop — with all
    /// collection filtering, permission enforcement, and internal collection
    /// blocking. No LLM round-trip.
    ///
    /// Returns `Some(Ok(result))` if the tool was executed,
    /// `Some(Err(e))` if execution failed, or `None` if the server doesn't
    /// support the endpoint (older ekoDB versions).
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        params: &serde_json::Value,
        chat_id: Option<&str>,
    ) -> Option<Result<serde_json::Value>> {
        let tool_name = tool_name.to_string();
        let params = params.clone();
        let chat_id = chat_id.map(|s| s.to_string());
        let result = self
            .execute_with_token_refresh(|token| {
                let tool_name = tool_name.clone();
                let params = params.clone();
                let chat_id = chat_id.clone();
                async move {
                    self.http
                        .execute_tool_remote(&tool_name, &params, chat_id.as_deref(), &token)
                        .await
                }
            })
            .await;
        match result {
            Ok(result) => {
                // The server returns a ToolResult with success/result/error fields
                let success = result["success"].as_bool().unwrap_or(false);
                if success {
                    Some(Ok(result["result"].clone()))
                } else {
                    let error = result["error"]
                        .as_str()
                        .unwrap_or("tool execution failed")
                        .to_string();
                    Some(Err(crate::Error::ToolExecution(error)))
                }
            }
            // handle_response returns Error::NotFound for 404
            Err(crate::Error::NotFound) => None,
            // Also handle Error::Api with 405 (method not allowed)
            Err(crate::Error::Api { code: 405, .. }) => None,
            Err(e) => Some(Err(e)),
        }
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

    /// Get all built-in server-side chat tool definitions.
    /// Returns a list of tool objects with `name`, `description`, and `parameters` fields.
    /// Used by planning agents to discover available tools dynamically.
    pub async fn get_chat_tools(&self) -> Result<Vec<serde_json::Value>> {
        let token = self.auth.get_token().await?;
        self.http.get_chat_tools(&token).await
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

    /// Stateless raw LLM completion — no session, no history, no RAG.
    ///
    /// Sends a system prompt and user message directly to the LLM via ekoDB
    /// and returns the raw text response without any context injection or
    /// conversation management. Use this for structured-output tasks such as
    /// planning where the response must be parsed programmatically.
    ///
    /// This is the blocking HTTP variant. For deployed instances behind reverse
    /// proxies, prefer `raw_completion_stream()` (SSE) or use `WebSocketClient::raw_completion()`
    /// (WSS) to avoid proxy timeouts on long-running LLM calls.
    pub async fn raw_completion(
        &self,
        request: crate::chat::RawCompletionRequest,
    ) -> Result<crate::chat::RawCompletionResponse> {
        let token = self.auth.get_token().await?;
        self.http.raw_completion(request, &token).await
    }

    /// Stateless raw LLM completion via SSE streaming.
    ///
    /// Same as `raw_completion()` but uses Server-Sent Events to keep the
    /// connection alive. Preferred for deployed instances where reverse proxies
    /// may kill idle HTTP connections before the LLM responds.
    pub async fn raw_completion_stream(
        &self,
        request: crate::chat::RawCompletionRequest,
    ) -> Result<crate::chat::RawCompletionResponse> {
        let token = self.auth.get_token().await?;
        self.http.raw_completion_stream(request, &token).await
    }

    /// Stateless raw LLM completion via SSE with incremental token progress.
    ///
    /// Same as `raw_completion_stream()` but sends each token through the
    /// provided channel as it arrives, allowing callers to show real-time
    /// progress during long-running LLM calls (e.g., goal plan generation).
    pub async fn raw_completion_stream_with_progress(
        &self,
        request: crate::chat::RawCompletionRequest,
        progress_tx: tokio::sync::mpsc::Sender<String>,
    ) -> Result<crate::chat::RawCompletionResponse> {
        let token = self.auth.get_token().await?;
        self.http
            .raw_completion_stream_with_progress(request, &token, progress_tx)
            .await
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

    /// Submit a client tool result for an in-flight SSE chat stream.
    pub async fn submit_chat_tool_result(
        &self,
        chat_id: &str,
        call_id: &str,
        success: bool,
        result: Option<serde_json::Value>,
        error: Option<String>,
    ) -> Result<()> {
        let chat_id = chat_id.to_string();
        let call_id = call_id.to_string();
        let http = self.http.clone();
        self.execute_with_token_refresh(move |token| {
            let chat_id = chat_id.clone();
            let call_id = call_id.clone();
            let result = result.clone();
            let error = error.clone();
            let http = http.clone();
            async move {
                http.submit_chat_tool_result(&chat_id, &call_id, success, result, error, &token)
                    .await
            }
        })
        .await
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

    /// Stream a chat message via SSE (Server-Sent Events).
    /// Returns a channel that yields `ChatStreamEvent` items as they arrive.
    /// Server-side tools execute normally; client-side tools are not supported over SSE.
    pub async fn chat_message_stream(
        &self,
        chat_id: &str,
        request: crate::chat::ChatMessageRequest,
    ) -> Result<tokio::sync::mpsc::Receiver<crate::websocket::ChatStreamEvent>> {
        let token = self.auth.get_token().await?;
        self.http
            .chat_message_stream(chat_id, request, &token)
            .await
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

    /// Generate embeddings for text using AI (via ekoDB Functions)
    ///
    /// Uses ekoDB's AI integration to generate vector embeddings for semantic search.
    /// Requires OPENAI_API_KEY to be set in the ekoDB server environment.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to generate embeddings for
    /// * `model` - The embedding model to use (e.g., "text-embedding-3-small")
    ///
    /// # Returns
    ///
    /// A vector of f64 values representing the embedding
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::Client;
    /// # async fn example(client: &Client) -> Result<(), ekodb_client::Error> {
    /// let embedding = client.embed("Hello world", "text-embedding-3-small").await?;
    /// println!("Generated {} dimensions", embedding.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn embed(&self, text: &str, model: &str) -> Result<Vec<f64>> {
        let token = self.auth.get_token().await?;
        let request = crate::chat::EmbedRequest {
            text: Some(text.to_string()),
            texts: None,
            model: Some(model.to_string()),
        };
        let response = self.http.embed(request, &token).await?;
        response
            .embeddings
            .into_iter()
            .next()
            .ok_or(crate::Error::Api {
                code: 500,
                message: "No embedding returned".to_string(),
            })
    }

    /// Generate embeddings for multiple texts in a single batch request
    ///
    /// # Arguments
    ///
    /// * `texts` - The texts to generate embeddings for
    /// * `model` - The embedding model to use (e.g., "text-embedding-3-small")
    ///
    /// # Returns
    ///
    /// A vector of embedding vectors
    pub async fn embed_batch(&self, texts: Vec<String>, model: &str) -> Result<Vec<Vec<f64>>> {
        let token = self.auth.get_token().await?;
        let request = crate::chat::EmbedRequest {
            text: None,
            texts: Some(texts),
            model: Some(model.to_string()),
        };
        let response = self.http.embed(request, &token).await?;
        Ok(response.embeddings)
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

    /// Save a new function
    ///
    /// # Arguments
    ///
    /// * `function` - The UserFunction definition to save
    ///
    /// # Returns
    ///
    /// The UserFunction ID assigned by the server
    pub async fn save_function(&self, function: crate::functions::UserFunction) -> Result<String> {
        let token = self.auth.get_token().await?;
        self.http.save_function(function, &token).await
    }

    /// Get a UserFunction by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The UserFunction ID (from save_function)
    ///
    /// # Returns
    ///
    /// The saved UserFunction definition
    pub async fn get_function(&self, id: &str) -> Result<crate::functions::UserFunction> {
        let token = self.auth.get_token().await?;
        self.http.get_function(id, &token).await
    }

    /// List all saved functions, optionally filtered by tags
    ///
    /// # Arguments
    ///
    /// * `tags` - Optional list of tags to filter by
    ///
    /// # Returns
    ///
    /// Vector of saved functions
    pub async fn list_functions(
        &self,
        tags: Option<Vec<String>>,
    ) -> Result<Vec<crate::functions::UserFunction>> {
        let token = self.auth.get_token().await?;
        self.http.list_functions(tags, &token).await
    }

    /// Update an existing function
    ///
    /// # Arguments
    ///
    /// * `id` - The UserFunction ID to update
    /// * `function` - The updated UserFunction definition
    pub async fn update_function(
        &self,
        id: &str,
        function: crate::functions::UserFunction,
    ) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.update_function(id, function, &token).await
    }

    /// Delete a UserFunction by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The UserFunction ID to delete
    pub async fn delete_function(&self, id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.delete_function(id, &token).await
    }

    /// Call a saved function
    ///
    /// # Arguments
    ///
    /// * `label` - The UserFunction label to execute
    /// * `params` - Optional parameters to pass to the function
    ///
    /// # Returns
    ///
    /// UserFunction execution result containing records and metadata
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ekodb_client::{Client, FieldType};
    /// # use std::collections::HashMap;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .base_url("https://your-instance.ekodb.net")
    ///     .api_key("your-token")
    ///     .build()?;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("status".to_string(), FieldType::String("active".to_string()));
    ///
    /// let result = client.call_function("get_active_users", Some(params)).await?;
    /// println!("Found {} records", result.records.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_function(
        &self,
        function_id_or_label: &str,
        params: Option<std::collections::HashMap<String, crate::types::FieldType>>,
    ) -> Result<crate::functions::FunctionResult> {
        let token = self.auth.get_token().await?;
        self.http
            .call_function(function_id_or_label, params, &token)
            .await
    }

    // ========================================================================
    // USER FUNCTIONS API
    // ========================================================================

    /// Save a new UserFunction
    ///
    /// # Arguments
    ///
    /// * `user_function` - The UserFunction definition to save
    ///
    /// # Returns
    ///
    /// The UserFunction ID assigned by the server
    pub async fn save_user_function(
        &self,
        user_function: crate::functions::UserFunction,
    ) -> Result<String> {
        let token = self.auth.get_token().await?;
        self.http.save_user_function(user_function, &token).await
    }

    /// Get a UserFunction by its label
    ///
    /// # Arguments
    ///
    /// * `label` - The UserFunction label (unique identifier)
    ///
    /// # Returns
    ///
    /// The saved UserFunction definition
    pub async fn get_user_function(&self, label: &str) -> Result<crate::functions::UserFunction> {
        let token = self.auth.get_token().await?;
        self.http.get_user_function(label, &token).await
    }

    /// List all saved UserFunctions, optionally filtered by tags
    ///
    /// # Arguments
    ///
    /// * `tags` - Optional list of tags to filter by
    ///
    /// # Returns
    ///
    /// Vector of saved UserFunctions
    pub async fn list_user_functions(
        &self,
        tags: Option<Vec<String>>,
    ) -> Result<Vec<crate::functions::UserFunction>> {
        let token = self.auth.get_token().await?;
        self.http.list_user_functions(tags, &token).await
    }

    /// Update an existing UserFunction
    ///
    /// # Arguments
    ///
    /// * `label` - The UserFunction label to update
    /// * `user_function` - The updated UserFunction definition
    pub async fn update_user_function(
        &self,
        label: &str,
        user_function: crate::functions::UserFunction,
    ) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http
            .update_user_function(label, user_function, &token)
            .await
    }

    /// Delete a UserFunction by its label
    ///
    /// # Arguments
    ///
    /// * `label` - The UserFunction label to delete
    pub async fn delete_user_function(&self, label: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.delete_user_function(label, &token).await
    }

    // ── Goal CRUD ────────────────────────────────────────────────────────────

    pub async fn goal_create(&self, data: serde_json::Value) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_create(data, &token).await
    }

    pub async fn goal_list(&self) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_list(&token).await
    }

    pub async fn goal_get(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_get(id, &token).await
    }

    pub async fn goal_update(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_update(id, data, &token).await
    }

    pub async fn goal_delete(&self, id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.goal_delete(id, &token).await
    }

    pub async fn goal_search(&self, query: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_search(query, &token).await
    }

    // ── Goal lifecycle ─────────────────────────────────────────────────────

    /// Atomically mark a goal as complete (status → pending_review).
    pub async fn goal_complete(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_complete(id, data, &token).await
    }

    /// Atomically approve a goal (status → in_progress).
    pub async fn goal_approve(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_approve(id, &token).await
    }

    /// Atomically reject a goal (status → failed).
    pub async fn goal_reject(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_reject(id, data, &token).await
    }

    // ── Goal step lifecycle ──────────────────────────────────────────────────

    /// Atomically mark a goal step as in_progress.
    pub async fn goal_step_start(&self, id: &str, step_index: usize) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_step_start(id, step_index, &token).await
    }

    /// Atomically mark a goal step as completed with result.
    pub async fn goal_step_complete(
        &self,
        id: &str,
        step_index: usize,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http
            .goal_step_complete(id, step_index, data, &token)
            .await
    }

    /// Atomically mark a goal step as failed with error.
    pub async fn goal_step_fail(
        &self,
        id: &str,
        step_index: usize,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_step_fail(id, step_index, data, &token).await
    }

    // ── Task CRUD ────────────────────────────────────────────────────────────

    pub async fn task_create(&self, data: serde_json::Value) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_create(data, &token).await
    }

    pub async fn task_list(&self) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_list(&token).await
    }

    pub async fn task_get(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_get(id, &token).await
    }

    pub async fn task_update(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_update(id, data, &token).await
    }

    pub async fn task_delete(&self, id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.task_delete(id, &token).await
    }

    pub async fn task_due(&self, now: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_due(now, &token).await
    }

    // ── Task lifecycle ──────────────────────────────────────────────────────

    /// Atomically mark a task as running.
    pub async fn task_start(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_start(id, &token).await
    }

    /// Atomically mark a task as succeeded (increment run_count, reset failures).
    pub async fn task_succeed(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_succeed(id, data, &token).await
    }

    /// Atomically mark a task as failed (increment consecutive_failures).
    pub async fn task_fail(&self, id: &str, data: serde_json::Value) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_fail(id, data, &token).await
    }

    /// Atomically pause a task (status → paused).
    pub async fn task_pause(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_pause(id, &token).await
    }

    /// Atomically resume a task (status → active).
    pub async fn task_resume(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.task_resume(id, data, &token).await
    }

    // ── Agent CRUD ───────────────────────────────────────────────────────────

    pub async fn agent_create(&self, data: serde_json::Value) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.agent_create(data, &token).await
    }

    pub async fn agent_list(&self) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.agent_list(&token).await
    }

    pub async fn agent_get(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.agent_get(id, &token).await
    }

    pub async fn agent_get_by_name(&self, name: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.agent_get_by_name(name, &token).await
    }

    pub async fn agent_update(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.agent_update(id, data, &token).await
    }

    pub async fn agent_delete(&self, id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.agent_delete(id, &token).await
    }

    pub async fn agents_by_deployment(&self, deployment_id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.agents_by_deployment(deployment_id, &token).await
    }

    // ── Goal Template CRUD ──────────────────────────────────────────────────

    pub async fn goal_template_create(&self, data: serde_json::Value) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_template_create(data, &token).await
    }

    pub async fn goal_template_list(&self) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_template_list(&token).await
    }

    pub async fn goal_template_get(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_template_get(id, &token).await
    }

    pub async fn goal_template_update(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.goal_template_update(id, data, &token).await
    }

    pub async fn goal_template_delete(&self, id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.goal_template_delete(id, &token).await
    }

    // ── KV Document Linking ─────────────────────────────────────────────────

    /// Get documents linked to a KV key.
    pub async fn kv_get_links(&self, key: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.kv_get_links(key, &token).await
    }

    /// Link a document to a KV key.
    pub async fn kv_link(
        &self,
        key: &str,
        collection: &str,
        document_id: &str,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http
            .kv_link(key, collection, document_id, &token)
            .await
    }

    /// Unlink a document from a KV key.
    pub async fn kv_unlink(
        &self,
        key: &str,
        collection: &str,
        document_id: &str,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http
            .kv_unlink(key, collection, document_id, &token)
            .await
    }

    // ── Schedule Management ─────────────────────────────────────────────────

    /// Create a new schedule.
    pub async fn create_schedule(&self, data: serde_json::Value) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.create_schedule(data, &token).await
    }

    /// List all schedules.
    pub async fn list_schedules(&self) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.list_schedules(&token).await
    }

    /// Get a schedule by ID.
    pub async fn get_schedule(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.get_schedule(id, &token).await
    }

    /// Update a schedule by ID.
    pub async fn update_schedule(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.update_schedule(id, data, &token).await
    }

    /// Delete a schedule by ID.
    pub async fn delete_schedule(&self, id: &str) -> Result<()> {
        let token = self.auth.get_token().await?;
        self.http.delete_schedule(id, &token).await
    }

    /// Pause a schedule.
    pub async fn pause_schedule(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.pause_schedule(id, &token).await
    }

    /// Resume a schedule.
    pub async fn resume_schedule(&self, id: &str) -> Result<serde_json::Value> {
        let token = self.auth.get_token().await?;
        self.http.resume_schedule(id, &token).await
    }

    // =========================================================================
    // Schema Cache & WebSocket Convenience
    // =========================================================================

    /// Get a reference to the schema cache.
    pub fn schema_cache(&self) -> &crate::schema_cache::SchemaCache {
        &self.schema_cache
    }

    /// Get the schema cache as an Arc (for sharing with WebSocketClient).
    pub fn schema_cache_arc(&self) -> Arc<crate::schema_cache::SchemaCache> {
        self.schema_cache.clone()
    }

    /// Extract the record ID from a JSON record using the cached primary_key_alias.
    ///
    /// If the schema cache has the collection's alias, it's used first.
    /// Falls back to "id" then "_id" if the cache misses.
    pub fn extract_id(&self, collection: &str, record: &serde_json::Value) -> Option<String> {
        let alias = self.schema_cache.get_alias(collection);
        let extra: Vec<&str> = alias.iter().map(|s| s.as_str()).collect();
        crate::utils::extract_record_id(record, &extra)
    }

    /// Subscribe to collection mutations via SSE (Server-Sent Events).
    ///
    /// Returns an `mpsc::Receiver` that yields `MutationNotificationPayload` events.
    /// Also handles `schema_changed` events to auto-invalidate the schema cache.
    ///
    /// Use this when WebSocket connections aren't available (e.g. behind reverse
    /// proxies that block WS upgrades).
    pub async fn subscribe_sse(
        &self,
        collection: &str,
        filter_field: Option<&str>,
        filter_value: Option<&str>,
    ) -> Result<tokio::sync::mpsc::Receiver<crate::websocket::MutationNotificationPayload>> {
        let token = self.auth.get_token().await?;
        let mut parsed_url =
            url::Url::parse(&format!("{}/api/subscribe/{}", self.base_url, collection)).map_err(
                |e| Error::Api {
                    code: 0,
                    message: format!("Invalid SSE URL: {}", e),
                },
            )?;

        {
            let mut pairs = parsed_url.query_pairs_mut();
            if let Some(ff) = filter_field {
                pairs.append_pair("filter_field", ff);
            }
            if let Some(fv) = filter_value {
                pairs.append_pair("filter_value", fv);
            }
        }

        let url = parsed_url.to_string();

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "text/event-stream")
            .send()
            .await
            .map_err(|e| Error::Api {
                code: 0,
                message: format!("SSE connection failed: {}", e),
            })?;

        if !response.status().is_success() {
            return Err(Error::Api {
                code: response.status().as_u16(),
                message: format!("SSE subscribe failed: {}", response.status()),
            });
        }

        let (tx, rx) = tokio::sync::mpsc::channel(256);
        let schema_cache = self.schema_cache.clone();

        // Spawn background task to parse SSE stream
        tokio::spawn(async move {
            use futures_util::StreamExt;
            let mut stream = response.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk) = stream.next().await {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(_) => break,
                };
                let text = String::from_utf8_lossy(&chunk);
                // Normalize CRLF to LF for cross-platform SSE compatibility
                buffer.push_str(&text.replace("\r\n", "\n"));

                // Parse SSE events from buffer (event: ...\ndata: ...\n\n)
                while let Some(end) = buffer.find("\n\n") {
                    let event_block = buffer[..end].to_owned();
                    buffer.drain(..end + 2);

                    let mut event_type = String::new();
                    let mut data_lines: Vec<String> = Vec::new();

                    for line in event_block.lines() {
                        if let Some(val) = line.strip_prefix("event: ") {
                            event_type = val.trim().to_string();
                        } else if let Some(val) = line.strip_prefix("data: ") {
                            data_lines.push(val.trim().to_string());
                        }
                    }

                    let event_data = data_lines.join("\n");

                    match event_type.as_str() {
                        "mutation" => {
                            if let Ok(payload) = serde_json::from_str::<
                                crate::websocket::MutationNotificationPayload,
                            >(&event_data)
                            {
                                if tx.send(payload).await.is_err() {
                                    return; // Receiver dropped
                                }
                            }
                        }
                        "schema_changed" => {
                            if let Ok(sc) = serde_json::from_str::<
                                crate::websocket::SchemaChangedPayload,
                            >(&event_data)
                            {
                                schema_cache.handle_schema_changed(
                                    &sc.collection,
                                    sc.version,
                                    &sc.primary_key_alias,
                                );
                            }
                        }
                        _ => {} // ignore subscribed, error, heartbeat
                    }
                }
            }
        });

        Ok(rx)
    }

    /// Create a WebSocket client connected to this ekoDB instance.
    ///
    /// Derives the WS URL from the base URL (http→ws, https→wss) and uses
    /// the current auth token. The schema cache is automatically attached
    /// for realtime invalidation on SchemaChanged events.
    pub async fn connect_ws(&self) -> Result<crate::websocket::WebSocketClient> {
        let token = self.auth.get_token().await?;

        // Convert http(s) to ws(s)
        let ws_url = self
            .base_url
            .replace("https://", "wss://")
            .replace("http://", "ws://");

        let ws = crate::websocket::WebSocketClient::new(&ws_url, token)?;

        // Attach schema cache for auto-invalidation
        if self.schema_cache.is_enabled() {
            ws.set_schema_cache(self.schema_cache.clone()).await;
        }

        Ok(ws)
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
    schema_cache_enabled: bool,
    schema_cache_ttl: Option<Duration>,
    schema_cache_max: Option<usize>,
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

    /// Enable the in-memory schema cache for primary_key_alias resolution.
    ///
    /// When enabled, CRUD results use cached schema metadata to correctly
    /// extract record IDs regardless of the collection's `primary_key_alias` config.
    /// The cache is invalidated automatically via WS `SchemaChanged` events.
    pub fn schema_cache(mut self, enabled: bool) -> Self {
        self.schema_cache_enabled = enabled;
        self
    }

    /// Set the schema cache TTL (time-to-live) in seconds. Default: 300 (5 min).
    pub fn schema_cache_ttl(mut self, seconds: u64) -> Self {
        self.schema_cache_ttl = Some(Duration::from_secs(seconds));
        self
    }

    /// Set the max number of collections the schema cache holds. Default: 100.
    pub fn schema_cache_max(mut self, max: usize) -> Self {
        self.schema_cache_max = Some(max);
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

        let schema_cache =
            crate::schema_cache::SchemaCache::new(crate::schema_cache::SchemaCacheConfig {
                enabled: self.schema_cache_enabled,
                max_entries: self.schema_cache_max.unwrap_or(100),
                ttl: self.schema_cache_ttl.unwrap_or(Duration::from_secs(300)),
            });

        Ok(Client {
            http: Arc::new(http),
            auth: Arc::new(auth),
            schema_cache: Arc::new(schema_cache),
            base_url: base_url_str,
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
