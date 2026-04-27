//! HTTP client implementation for ekoDB API

use crate::chat::{
    ChatMessageRequest, ChatResponse, ChatSessionResponse, CreateChatSessionRequest, EmbedRequest,
    EmbedResponse, GetMessagesQuery, ListSessionsQuery, ListSessionsResponse, MergeSessionsRequest,
    Models, RawCompletionRequest, RawCompletionResponse, ToggleForgottenRequest,
    UpdateMessageRequest, UpdateSessionRequest,
};
use crate::client::RateLimitInfo;
use crate::error::{Error, Result};
use crate::retry::RetryPolicy;
use crate::schema::{CollectionMetadata, Schema};
use crate::search::{DistinctValuesQuery, DistinctValuesResponse, SearchQuery, SearchResponse};
use crate::types::{FieldType, Query, Record, SerializationFormat};
use reqwest::{Client as ReqwestClient, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

/// Body for the single-field atomic action endpoint.
#[derive(Serialize, Deserialize, Clone)]
struct UpdateWithActionBody {
    field: String,
    value: FieldType,
}

/// HTTP client for making requests to ekoDB API
pub struct HttpClient {
    client: ReqwestClient,
    base_url: Url,
    retry_policy: RetryPolicy,
    should_retry: bool,
    format: SerializationFormat,
}

impl HttpClient {
    /// Create a new HTTP client with specific serialization format
    pub fn new(
        base_url: &str,
        timeout: Duration,
        max_retries: u32,
        should_retry: bool,
        format: SerializationFormat,
    ) -> Result<Self> {
        // Use connect_timeout (not timeout) so streaming responses (SSE, WebSocket)
        // aren't killed mid-stream. The overall timeout would kill the entire request
        // including incremental streaming — connect_timeout only limits the initial
        // TCP connection phase.
        let client = ReqwestClient::builder()
            .connect_timeout(timeout)
            .gzip(true) // Enable automatic gzip decompression
            .build()
            .map_err(|e| Error::Connection(e.to_string()))?;

        let base_url = Url::parse(base_url)?;
        let retry_policy = RetryPolicy::new(max_retries);

        Ok(Self {
            client,
            base_url,
            retry_policy,
            should_retry,
            format,
        })
    }

    /// Health Check
    pub async fn health_check(&self) -> Result<()> {
        let response = self
            .client
            .get(self.base_url.join("/api/health").unwrap())
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Connection(format!(
                "Health check failed: {}",
                response.status()
            )))
        }
    }

    /// Check if a path should use JSON instead of MessagePack
    /// Only CRUD operations (insert/update/delete/batch) support MessagePack
    /// Everything else (search, collections, kv, auth, chat) uses JSON
    fn should_use_json(path: &str) -> bool {
        // ONLY these operations support MessagePack
        let msgpack_paths = [
            "/api/insert/",
            "/api/batch/insert/",
            "/api/update/",
            "/api/batch/update/",
            "/api/delete/",
            "/api/batch/delete/",
        ];

        // Check if path starts with any MessagePack-supported operation
        for prefix in &msgpack_paths {
            if path.starts_with(prefix) {
                return false; // Use MessagePack
            }
        }

        // Everything else uses JSON
        true
    }

    /// Serialize data based on the client's format and path
    /// Only CRUD operations use MessagePack, everything else uses JSON
    fn serialize<T: Serialize>(&self, path: &str, data: &T) -> Result<Vec<u8>> {
        let use_json = Self::should_use_json(path) || self.format == SerializationFormat::Json;

        if use_json {
            serde_json::to_vec(data).map_err(Error::Serialization)
        } else {
            rmp_serde::to_vec(data)
                .map_err(|e| Error::Validation(format!("MessagePack serialization error: {}", e)))
        }
    }

    /// Deserialize data based on the client's format and path
    /// Only CRUD operations use MessagePack, everything else uses JSON
    fn deserialize<'a, T: Deserialize<'a>>(&self, path: &str, data: &'a [u8]) -> Result<T> {
        let use_json = Self::should_use_json(path) || self.format == SerializationFormat::Json;

        if use_json {
            serde_json::from_slice(data).map_err(Error::Serialization)
        } else {
            rmp_serde::from_slice(data)
                .map_err(|e| Error::Validation(format!("MessagePack deserialization error: {}", e)))
        }
    }

    /// Get content-type header for the current format and path
    fn content_type(&self, path: &str) -> &'static str {
        let use_json = Self::should_use_json(path) || self.format == SerializationFormat::Json;

        if use_json {
            "application/json"
        } else {
            "application/msgpack"
        }
    }

    /// Add format headers (Content-Type and Accept) to a request builder
    /// Note: reqwest automatically handles gzip compression with the gzip feature enabled
    fn add_format_headers(
        &self,
        path: &str,
        builder: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        let content_type = self.content_type(path);
        builder
            .header("Content-Type", content_type)
            .header("Accept", content_type)
        // Accept-Encoding is automatically handled by reqwest when gzip feature is enabled
    }

    /// Execute a request with optional retry logic
    async fn execute_with_retry<F, Fut, T>(&self, mut f: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        if self.should_retry {
            self.retry_policy.execute(f).await
        } else {
            f().await
        }
    }

    /// Insert a record
    pub async fn insert(
        &self,
        collection: &str,
        mut record: Record,
        options: Option<crate::options::InsertOptions>,
        token: &str,
    ) -> Result<Record> {
        // Add TTL to record if specified
        if let Some(ref opts) = options {
            if let Some(ref ttl) = opts.ttl {
                record.insert("ttl".to_string(), serde_json::Value::String(ttl.clone()));
            }
        }

        // Build URL with query parameters
        let mut url_path = format!("/api/insert/{}", collection);
        let mut params = vec![];

        if let Some(ref opts) = options {
            if let Some(bypass) = opts.bypass_ripple {
                params.push(format!("bypass_ripple={}", bypass));
            }
            if let Some(ref tx_id) = opts.transaction_id {
                params.push(format!("transaction_id={}", tx_id));
            }
            if let Some(bypass_cache) = opts.bypass_cache {
                params.push(format!("bypass_cache={}", bypass_cache));
            }
        }

        if !params.is_empty() {
            url_path = format!("{}?{}", url_path, params.join("&"));
        }

        let url = self.base_url.join(&url_path)?;
        let body = self.serialize(&url_path, &record)?;

        self.execute_with_retry(|| async {
            let response = self
                .add_format_headers(
                    &url_path,
                    self.client
                        .post(url.clone())
                        .header("Authorization", format!("Bearer {}", token)),
                )
                .body(body.clone())
                .send()
                .await?;

            self.handle_response(&url_path, response).await
        })
        .await
    }

    /// Find records
    pub async fn find(
        &self,
        collection: &str,
        query: Query,
        token: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<Vec<Record>> {
        let url_path = if let Some(bypass) = bypass_ripple {
            format!("/api/find/{}?bypass_ripple={}", collection, bypass)
        } else {
            format!("/api/find/{}", collection)
        };
        let url = self.base_url.join(&url_path)?;
        let body = self.serialize(&url_path, &query)?;

        self.execute_with_retry(|| async {
            let response = self
                .add_format_headers(
                    &url_path,
                    self.client
                        .post(url.clone())
                        .header("Authorization", format!("Bearer {}", token)),
                )
                .body(body.clone())
                .send()
                .await?;

            self.handle_response(&url_path, response).await
        })
        .await
    }

    /// Find a record by ID
    pub async fn find_by_id(
        &self,
        collection: &str,
        id: &str,
        token: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<Record> {
        let url_path = if let Some(bypass) = bypass_ripple {
            format!("/api/find/{}/{}?bypass_ripple={}", collection, id, bypass)
        } else {
            format!("/api/find/{}/{}", collection, id)
        };
        let url = self.base_url.join(&url_path)?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .add_format_headers(
                        &url_path,
                        self.client
                            .get(url.clone())
                            .header("Authorization", format!("Bearer {}", token)),
                    )
                    .send()
                    .await?;

                self.handle_response(&url_path, response).await
            })
            .await
    }

    /// Update a record
    pub async fn update(
        &self,
        collection: &str,
        id: &str,
        record: Record,
        options: Option<crate::options::UpdateOptions>,
        token: &str,
    ) -> Result<Record> {
        // Build URL with query parameters
        let mut url_path = format!("/api/update/{}/{}", collection, id);
        let mut params = vec![];

        if let Some(ref opts) = options {
            if let Some(bypass) = opts.bypass_ripple {
                params.push(format!("bypass_ripple={}", bypass));
            }
            if let Some(ref tx_id) = opts.transaction_id {
                params.push(format!("transaction_id={}", tx_id));
            }
            if let Some(bypass_cache) = opts.bypass_cache {
                params.push(format!("bypass_cache={}", bypass_cache));
            }
        }

        if !params.is_empty() {
            url_path = format!("{}?{}", url_path, params.join("&"));
        }

        let url = self.base_url.join(&url_path)?;
        let body = self.serialize(&url_path, &record)?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .add_format_headers(
                        &url_path,
                        self.client
                            .put(url.clone())
                            .header("Authorization", format!("Bearer {}", token)),
                    )
                    .body(body.clone())
                    .send()
                    .await?;

                self.handle_response(&url_path, response).await
            })
            .await
    }

    /// Apply an atomic field action to a single field of a record.
    ///
    /// Actions: increment, decrement, multiply, divide, modulo, push, pop,
    /// shift, unshift, remove, append, clear.
    pub async fn update_with_action(
        &self,
        collection: &str,
        id: &str,
        action: &str,
        field: &str,
        value: FieldType,
        token: &str,
    ) -> Result<Record> {
        let url_path = format!("/api/update/{}/{}/action/{}", collection, id, action);
        let url = self.base_url.join(&url_path)?;
        let body = self.serialize(
            &url_path,
            &UpdateWithActionBody {
                field: field.to_string(),
                value,
            },
        )?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .add_format_headers(
                        &url_path,
                        self.client
                            .put(url.clone())
                            .header("Authorization", format!("Bearer {}", token)),
                    )
                    .body(body.clone())
                    .send()
                    .await?;

                self.handle_response(&url_path, response).await
            })
            .await
    }

    /// Apply a sequence of atomic field actions to a record in a single request.
    ///
    /// Each action is a tuple of (action_name, field_name, value).
    pub async fn update_with_action_sequence(
        &self,
        collection: &str,
        id: &str,
        actions: Vec<(String, String, FieldType)>,
        token: &str,
    ) -> Result<Record> {
        let url_path = format!("/api/update/sequence/{}/{}", collection, id);
        let url = self.base_url.join(&url_path)?;
        let body = self.serialize(&url_path, &actions)?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .add_format_headers(
                        &url_path,
                        self.client
                            .put(url.clone())
                            .header("Authorization", format!("Bearer {}", token)),
                    )
                    .body(body.clone())
                    .send()
                    .await?;

                self.handle_response(&url_path, response).await
            })
            .await
    }

    /// Delete a record
    pub async fn delete(
        &self,
        collection: &str,
        id: &str,
        token: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<()> {
        let url_path = if let Some(bypass) = bypass_ripple {
            format!("/api/delete/{}/{}?bypass_ripple={}", collection, id, bypass)
        } else {
            format!("/api/delete/{}/{}", collection, id)
        };
        let url = self.base_url.join(&url_path)?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .add_format_headers(
                        &url_path,
                        self.client
                            .delete(url.clone())
                            .header("Authorization", format!("Bearer {}", token)),
                    )
                    .send()
                    .await?;

                // Server returns the deleted record, but we discard it
                let _deleted: Record = self.handle_response(&url_path, response).await?;
                Ok(())
            })
            .await
    }

    /// Restore a deleted record from trash
    pub async fn restore_deleted(&self, collection: &str, id: &str, token: &str) -> Result<bool> {
        let url_path = format!("/api/trash/{}/{}", collection, id);
        let url = self.base_url.join(&url_path)?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                let result: serde_json::Value = self.handle_response(&url_path, response).await?;
                Ok(result
                    .get("restored")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false))
            })
            .await
    }

    /// Restore all deleted records in a collection from trash
    pub async fn restore_collection(&self, collection: &str, token: &str) -> Result<usize> {
        let url_path = format!("/api/trash/{}", collection);
        let url = self.base_url.join(&url_path)?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                let result: serde_json::Value = self.handle_response(&url_path, response).await?;
                Ok(result
                    .get("records_restored")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize)
            })
            .await
    }

    /// Batch insert records
    pub async fn batch_insert(
        &self,
        collection: &str,
        records: Vec<Record>,
        token: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<Vec<Record>> {
        let url_path = format!("/api/batch/insert/{}", collection);
        let url = self.base_url.join(&url_path)?;

        // Convert to the format the server expects
        #[derive(Serialize)]
        struct BatchInsertItem {
            data: Record,
            bypass_ripple: Option<bool>,
        }

        #[derive(Serialize)]
        struct BatchInsertQuery {
            inserts: Vec<BatchInsertItem>,
        }

        let batch_data = BatchInsertQuery {
            inserts: records
                .into_iter()
                .map(|r| BatchInsertItem {
                    data: r,
                    bypass_ripple,
                })
                .collect(),
        };

        #[derive(Deserialize)]
        struct BatchOperationResult {
            successful: Vec<String>,
            #[allow(dead_code)]
            failed: Vec<serde_json::Value>,
        }

        let body = self.serialize(&url_path, &batch_data)?;

        let result: BatchOperationResult = self
            .retry_policy
            .execute(|| async {
                let response = self
                    .add_format_headers(
                        &url_path,
                        self.client
                            .post(url.clone())
                            .header("Authorization", format!("Bearer {}", token)),
                    )
                    .body(body.clone())
                    .send()
                    .await?;

                self.handle_response(&url_path, response).await
            })
            .await?;

        // Convert IDs to Record objects with just the ID field
        Ok(result
            .successful
            .into_iter()
            .map(|id| {
                let mut record = Record::new();
                record.insert("id".to_string(), FieldType::String(id));
                record
            })
            .collect())
    }

    /// Batch update records
    pub async fn batch_update(
        &self,
        collection: &str,
        updates: Vec<(String, Record)>, // Vec of (id, record) pairs
        token: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<Vec<Record>> {
        let url_path = format!("/api/batch/update/{}", collection);
        let url = self.base_url.join(&url_path)?;

        // Convert to the format the server expects
        #[derive(Serialize)]
        struct BatchUpdateItem {
            #[serde(rename = "id")]
            id: String,
            data: Record,
            bypass_ripple: Option<bool>,
        }

        #[derive(Serialize)]
        struct BatchUpdateQuery {
            updates: Vec<BatchUpdateItem>,
        }

        let batch_data = BatchUpdateQuery {
            updates: updates
                .into_iter()
                .map(|(id, data)| BatchUpdateItem {
                    id,
                    data,
                    bypass_ripple,
                })
                .collect(),
        };

        #[derive(Deserialize)]
        struct BatchOperationResult {
            successful: Vec<String>,
            #[allow(dead_code)]
            failed: Vec<serde_json::Value>,
        }

        let body = self.serialize(&url_path, &batch_data)?;

        let result: BatchOperationResult = self
            .retry_policy
            .execute(|| async {
                let response = self
                    .add_format_headers(
                        &url_path,
                        self.client
                            .put(url.clone())
                            .header("Authorization", format!("Bearer {}", token)),
                    )
                    .body(body.clone())
                    .send()
                    .await?;

                self.handle_response(&url_path, response).await
            })
            .await?;

        // Convert IDs to Record objects with just the ID field
        Ok(result
            .successful
            .into_iter()
            .map(|id| {
                let mut record = Record::new();
                record.insert("id".to_string(), FieldType::String(id));
                record
            })
            .collect())
    }

    /// Batch delete records by IDs
    pub async fn batch_delete(
        &self,
        collection: &str,
        ids: Vec<String>,
        token: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<u64> {
        let url_path = format!("/api/batch/delete/{}", collection);
        let url = self.base_url.join(&url_path)?;

        // Convert to the format the server expects
        #[derive(Serialize)]
        struct BatchDeleteItem {
            #[serde(rename = "id")]
            id: String,
            bypass_ripple: Option<bool>,
        }

        #[derive(Serialize)]
        struct BatchDeleteQuery {
            deletes: Vec<BatchDeleteItem>,
        }

        let batch_data = BatchDeleteQuery {
            deletes: ids
                .into_iter()
                .map(|id| BatchDeleteItem { id, bypass_ripple })
                .collect(),
        };

        #[derive(Deserialize)]
        struct BatchOperationResult {
            successful: Vec<String>,
            #[allow(dead_code)]
            failed: Vec<serde_json::Value>,
        }

        let body = self.serialize(&url_path, &batch_data)?;

        let result: BatchOperationResult = self
            .retry_policy
            .execute(|| async {
                let response = self
                    .add_format_headers(
                        &url_path,
                        self.client
                            .delete(url.clone())
                            .header("Authorization", format!("Bearer {}", token)),
                    )
                    .body(body.clone())
                    .send()
                    .await?;

                self.handle_response(&url_path, response).await
            })
            .await?;

        Ok(result.successful.len() as u64)
    }

    /// List all collections
    pub async fn list_collections(&self, token: &str) -> Result<Vec<String>> {
        self.list_collections_inner(token, false).await
    }

    /// List collections, optionally excluding internal chat/system collections.
    pub async fn list_collections_filtered(
        &self,
        token: &str,
        exclude_internal: bool,
    ) -> Result<Vec<String>> {
        self.list_collections_inner(token, exclude_internal).await
    }

    /// Shared implementation for listing collections.
    async fn list_collections_inner(
        &self,
        token: &str,
        exclude_internal: bool,
    ) -> Result<Vec<String>> {
        let mut url = self.base_url.join("/api/collections")?;
        if exclude_internal {
            url.set_query(Some("exclude_internal=true"));
        }

        #[derive(Deserialize)]
        struct CollectionsResponse {
            collections: Vec<String>,
        }

        let response: CollectionsResponse = self
            .retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await?;

        Ok(response.collections)
    }

    /// Delete a collection
    pub async fn delete_collection(&self, collection: &str, token: &str) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("/api/collections/{}", collection))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                // Force JSON for metadata operations
                let _: serde_json::Value = Self::json_body(response).await?;
                Ok(())
            })
            .await
    }

    /// Set a key-value pair
    pub async fn kv_set(
        &self,
        key: &str,
        value: serde_json::Value,
        ttl: Option<&str>,
        token: &str,
    ) -> Result<()> {
        let url = self.base_url.join(&format!("/api/kv/set/{}", key))?;

        #[derive(Serialize)]
        struct KvSetRequest {
            value: serde_json::Value,
            #[serde(skip_serializing_if = "Option::is_none")]
            ttl: Option<String>,
        }

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json") // KV uses JSON values
                    .json(&KvSetRequest {
                        value: value.clone(),
                        ttl: ttl.map(|t| t.to_string()),
                    })
                    .send()
                    .await?;

                // Force JSON for KV operations (stores serde_json::Value)
                let _: serde_json::Value = Self::json_body(response).await?;
                Ok(())
            })
            .await
    }

    /// Get a key-value pair
    pub async fn kv_get(&self, key: &str, token: &str) -> Result<Option<serde_json::Value>> {
        let url = self.base_url.join(&format!("/api/kv/get/{}", key))?;

        #[derive(Deserialize)]
        struct KvGetResponse {
            value: Option<serde_json::Value>,
        }

        match self
            .retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json") // KV uses JSON values
                    .send()
                    .await?;

                // Force JSON for KV operations (stores serde_json::Value)
                Self::json_body::<KvGetResponse>(response).await
            })
            .await
        {
            Ok(response) => Ok(response.value),
            Err(Error::NotFound) => Ok(None), // Key doesn't exist, return None
            Err(e) => Err(e),
        }
    }

    /// Delete a key-value pair
    pub async fn kv_delete(&self, key: &str, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/kv/delete/{}", key))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json") // KV uses JSON values
                    .send()
                    .await?;

                // Force JSON for KV operations
                let _: serde_json::Value = Self::json_body(response).await?;
                Ok(())
            })
            .await
    }

    /// Check if a key exists in the KV store
    pub async fn kv_exists(&self, key: &str, token: &str) -> Result<bool> {
        match self.kv_get(key, token).await {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(Error::NotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Batch get multiple keys
    pub async fn kv_batch_get(&self, keys: Vec<String>, token: &str) -> Result<Vec<Record>> {
        let url = self.base_url.join("/api/kv/batch/get")?;

        #[derive(Serialize)]
        struct BatchGetRequest {
            keys: Vec<String>,
        }

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&BatchGetRequest { keys: keys.clone() })
                    .send()
                    .await?;

                let results: Vec<Record> = Self::json_body(response).await?;
                Ok(results)
            })
            .await
    }

    /// Batch set multiple key-value pairs
    pub async fn kv_batch_set(
        &self,
        keys: Vec<String>,
        values: Vec<Record>,
        ttl: Option<i64>,
        token: &str,
    ) -> Result<Vec<(String, bool)>> {
        let url = self.base_url.join("/api/kv/batch/set")?;

        #[derive(Serialize)]
        struct BatchSetRequest {
            keys: Vec<String>,
            values: Vec<Record>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ttl: Option<i64>,
        }

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&BatchSetRequest {
                        keys: keys.clone(),
                        values: values.clone(),
                        ttl,
                    })
                    .send()
                    .await?;

                let results: Vec<(String, bool)> = Self::json_body(response).await?;
                Ok(results)
            })
            .await
    }

    /// Batch delete multiple keys
    pub async fn kv_batch_delete(
        &self,
        keys: Vec<String>,
        token: &str,
    ) -> Result<Vec<(String, bool)>> {
        let url = self.base_url.join("/api/kv/batch/delete")?;

        #[derive(Serialize)]
        struct BatchDeleteRequest {
            keys: Vec<String>,
        }

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&BatchDeleteRequest { keys: keys.clone() })
                    .send()
                    .await?;

                let results: Vec<(String, bool)> = Self::json_body(response).await?;
                Ok(results)
            })
            .await
    }

    /// Query/find KV entries with pattern matching
    pub async fn kv_find(
        &self,
        pattern: Option<&str>,
        include_expired: bool,
        token: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let url = self.base_url.join("/api/kv/find")?;

        #[derive(Serialize)]
        struct KvFindRequest<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            pattern: Option<&'a str>,
            include_expired: bool,
        }

        let request = KvFindRequest {
            pattern,
            include_expired,
        };

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                let result: Vec<serde_json::Value> = Self::json_body(response).await?;
                Ok(result)
            })
            .await
    }

    // ========== Transaction Methods ==========

    /// Begin a new transaction
    pub async fn begin_transaction(&self, isolation_level: &str, token: &str) -> Result<String> {
        let url = self.base_url.join("/api/transactions")?;

        #[derive(Serialize)]
        struct BeginTransactionRequest<'a> {
            isolation_level: &'a str,
        }

        let request = BeginTransactionRequest { isolation_level };

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                let result: serde_json::Value = Self::json_body(response).await?;

                result["transaction_id"]
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| {
                        Error::Serialization(serde::de::Error::custom(
                            "No transaction_id in response",
                        ))
                    })
            })
            .await
    }

    /// Get transaction status
    pub async fn get_transaction_status(
        &self,
        transaction_id: &str,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/transactions/{}", transaction_id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                let result: serde_json::Value = Self::json_body(response).await?;
                Ok(result)
            })
            .await
    }

    /// Commit a transaction
    pub async fn commit_transaction(&self, transaction_id: &str, token: &str) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("/api/transactions/{}/commit", transaction_id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(Error::Http(response.error_for_status().unwrap_err()))
                }
            })
            .await
    }

    /// Rollback a transaction
    pub async fn rollback_transaction(&self, transaction_id: &str, token: &str) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("/api/transactions/{}/rollback", transaction_id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(Error::Http(response.error_for_status().unwrap_err()))
                }
            })
            .await
    }

    /// Perform a full-text search
    pub async fn search(
        &self,
        collection: &str,
        search_query: SearchQuery,
        token: &str,
    ) -> Result<SearchResponse> {
        let url = self.base_url.join(&format!("/api/search/{}", collection))?;

        // Temporarily use JSON for search until MessagePack issue is resolved
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&search_query)
                    .send()
                    .await?;

                // Force JSON for search operations
                Self::json_body(response).await
            })
            .await
    }

    /// Get distinct (unique) values for a field in a collection.
    ///
    /// Calls `POST /api/distinct/{collection}/{field}` with an optional filter body.
    pub async fn distinct_values(
        &self,
        collection: &str,
        field: &str,
        query: DistinctValuesQuery,
        token: &str,
    ) -> Result<DistinctValuesResponse> {
        let url = self
            .base_url
            .join(&format!("/api/distinct/{}/{}", collection, field))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&query)
                    .send()
                    .await?;

                let url_path = format!("/api/distinct/{}/{}", collection, field);
                self.handle_response(&url_path, response).await
            })
            .await
    }

    /// Create a collection with schema
    pub async fn create_collection(
        &self,
        collection: &str,
        schema: Schema,
        token: &str,
    ) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("/api/collections/{}", collection))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&schema)
                    .send()
                    .await?;

                // Force JSON for metadata operations
                let _: serde_json::Value = Self::json_body(response).await?;
                Ok(())
            })
            .await
    }

    /// Get collection metadata and schema
    pub async fn get_collection(
        &self,
        collection: &str,
        token: &str,
    ) -> Result<CollectionMetadata> {
        let url = self
            .base_url
            .join(&format!("/api/collections/{}", collection))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                // Force JSON for metadata operations
                Self::json_body(response).await
            })
            .await
    }

    /// Get collection schema
    pub async fn get_schema(&self, collection: &str, token: &str) -> Result<Schema> {
        let url = self
            .base_url
            .join(&format!("/api/schemas/{}", collection))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                // Force JSON for metadata operations
                Self::json_body(response).await
            })
            .await
    }

    /// Check response status and deserialize JSON body.
    /// Returns typed errors for non-success status codes, consistent with `handle_response`:
    /// - 401 → `Error::TokenExpired`
    /// - 404 → `Error::NotFound`
    /// - 429 → `Error::RateLimit` (retryable)
    /// - 503 → `Error::ServiceUnavailable` (retryable)
    /// - Other non-success → `Error::Api { code, message }`
    async fn json_body<T: for<'de> serde::Deserialize<'de>>(response: Response) -> Result<T> {
        let status = response.status();
        if !status.is_success() {
            return Err(match status {
                StatusCode::UNAUTHORIZED => Error::TokenExpired,
                StatusCode::NOT_FOUND => Error::NotFound,
                StatusCode::TOO_MANY_REQUESTS => {
                    let retry_after = response
                        .headers()
                        .get("retry-after")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(60);
                    Error::RateLimit {
                        retry_after_secs: retry_after,
                    }
                }
                StatusCode::SERVICE_UNAVAILABLE => {
                    let body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Service unavailable".to_string());
                    Error::ServiceUnavailable(extract_error_message(&body))
                }
                _ => {
                    let body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "unknown error".to_string());
                    Error::Api {
                        code: status.as_u16(),
                        message: extract_error_message(&body),
                    }
                }
            });
        }
        let bytes = response.bytes().await.map_err(Error::Http)?;
        serde_json::from_slice(&bytes).map_err(Error::Serialization)
    }

    /// Extract rate limit information from response headers
    fn extract_rate_limit_info(&self, response: &Response) -> Option<RateLimitInfo> {
        let headers = response.headers();

        let limit = headers
            .get("x-ratelimit-limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<usize>().ok())?;

        let remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<usize>().ok())?;

        let reset = headers
            .get("x-ratelimit-reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<i64>().ok())?;

        Some(RateLimitInfo {
            limit,
            remaining,
            reset,
        })
    }

    /// Handle HTTP response and convert to Result
    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        response: Response,
    ) -> Result<T> {
        let status = response.status();

        match status {
            StatusCode::OK | StatusCode::CREATED => {
                // Extract and log rate limit info before consuming the response
                if let Some(rate_limit_info) = self.extract_rate_limit_info(&response) {
                    log::debug!(
                        "Rate limit: {}/{} remaining, resets at {}",
                        rate_limit_info.remaining,
                        rate_limit_info.limit,
                        rate_limit_info.reset
                    );

                    if rate_limit_info.is_near_limit() {
                        log::warn!(
                            "Approaching rate limit: only {} requests remaining ({:.1}%)",
                            rate_limit_info.remaining,
                            rate_limit_info.remaining_percentage()
                        );
                    }
                }

                // Check if response is gzip compressed by looking at Content-Encoding header
                let is_gzipped = response
                    .headers()
                    .get("content-encoding")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.contains("gzip"))
                    .unwrap_or(false);

                let bytes = if is_gzipped {
                    // Use async decompression for gzipped responses
                    use async_compression::tokio::bufread::GzipDecoder;
                    use futures_util::StreamExt;
                    use tokio::io::AsyncReadExt;
                    use tokio_util::io::StreamReader;

                    let byte_stream = response.bytes_stream();
                    let stream_reader = StreamReader::new(
                        byte_stream.map(|result| result.map_err(std::io::Error::other)),
                    );
                    let mut decompressed_reader = GzipDecoder::new(stream_reader);

                    let mut decompressed = Vec::new();
                    decompressed_reader
                        .read_to_end(&mut decompressed)
                        .await
                        .map_err(|e| {
                            Error::Validation(format!("Gzip decompression failed: {}", e))
                        })?;
                    decompressed.into()
                } else {
                    // No compression - read bytes directly
                    response.bytes().await?
                };

                self.deserialize(path, &bytes).map_err(|e| {
                    Error::Validation(format!(
                        "Failed to parse response: {}. First 200 bytes: {:?}",
                        e,
                        bytes.iter().take(200).collect::<Vec<_>>()
                    ))
                })
            }
            StatusCode::UNAUTHORIZED => {
                // Return TokenExpired so client layer can refresh token and retry
                Err(Error::TokenExpired)
            }
            StatusCode::NOT_FOUND => Err(Error::NotFound),
            StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(60);

                Err(Error::RateLimit {
                    retry_after_secs: retry_after,
                })
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                let error_body: ErrorResponse =
                    response.json().await.unwrap_or_else(|_| ErrorResponse {
                        code: 503,
                        message: "Service unavailable".to_string(),
                    });
                Err(Error::ServiceUnavailable(error_body.message))
            }
            _ => {
                // Try to get error text, fallback to status description
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| format!("HTTP {} error", status.as_u16()));

                // Try to parse as ErrorResponse, otherwise use the text
                if let Ok(error_body) = serde_json::from_str::<ErrorResponse>(&error_text) {
                    Err(Error::api(error_body.code, error_body.message))
                } else {
                    Err(Error::api(status.as_u16(), error_text))
                }
            }
        }
    }

    // ========================================================================
    // Chat Operations
    // ========================================================================

    /// Get all available chat models
    pub async fn get_chat_models(&self, token: &str) -> Result<Models> {
        let url = self.base_url.join("/api/chat_models")?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Get all built-in server-side chat tool definitions
    pub async fn get_chat_tools(&self, token: &str) -> Result<Vec<serde_json::Value>> {
        let url = self.base_url.join("/api/chat/tools")?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                self.handle_response("/api/chat/tools", response).await
            })
            .await
    }

    /// Execute a single tool server-side via POST /api/chat/tools/execute.
    /// Goes through ekoDB's tool pipeline with all filtering and permissions.
    pub async fn execute_tool_remote(
        &self,
        tool: &str,
        params: &serde_json::Value,
        chat_id: Option<&str>,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/tools/execute")?;
        let mut body = serde_json::json!({
            "tool": tool,
            "params": params,
        });
        if let Some(cid) = chat_id {
            body["chat_id"] = serde_json::json!(cid);
        }

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .json(&body)
                    .send()
                    .await?;

                self.handle_response("/api/chat/tools/execute", response)
                    .await
            })
            .await
    }

    /// Get specific chat model info
    pub async fn get_chat_model(&self, model_name: &str, token: &str) -> Result<Vec<String>> {
        let url = self
            .base_url
            .join(&format!("/api/chat_models/{}", model_name))?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Generate embeddings for text(s)
    pub async fn embed(&self, request: EmbedRequest, token: &str) -> Result<EmbedResponse> {
        let url = self.base_url.join("/api/embed")?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                let status = response.status();
                let bytes = response.bytes().await.map_err(Error::Http)?;

                if !status.is_success() {
                    let message = String::from_utf8_lossy(&bytes).to_string();
                    return Err(Error::Api {
                        code: status.as_u16(),
                        message,
                    });
                }

                serde_json::from_slice(&bytes).map_err(Error::Serialization)
            })
            .await
    }

    /// Stateless raw LLM completion — no session, no history, no RAG.
    /// Calls POST /api/chat/complete and returns the LLM's raw text response.
    pub async fn raw_completion(
        &self,
        request: RawCompletionRequest,
        token: &str,
    ) -> Result<RawCompletionResponse> {
        let url = self.base_url.join("/api/chat/complete")?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                self.handle_response("/api/chat/complete", response).await
            })
            .await
    }

    /// Stateless raw LLM completion via SSE streaming.
    /// Calls POST /api/chat/complete/stream and collects the full response from SSE events.
    /// Keeps the connection alive with SSE heartbeats so reverse proxies don't kill it.
    pub async fn raw_completion_stream(
        &self,
        request: RawCompletionRequest,
        token: &str,
    ) -> Result<RawCompletionResponse> {
        let url = self.base_url.join("/api/chat/complete/stream")?;

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "text/event-stream")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let code = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api {
                code,
                message: body,
            });
        }

        // Parse SSE events: collect tokens, return content from "done" event
        let body = response.text().await?;
        let mut content = String::new();
        let mut got_done = false;
        let mut last_error: Option<String> = None;

        for line in body.lines() {
            if let Some(data_str) = line.strip_prefix("data:") {
                let data_str = data_str.trim();
                if data_str.is_empty() {
                    continue;
                }
                if let Ok(event_data) = serde_json::from_str::<serde_json::Value>(data_str) {
                    if let Some(token_text) = event_data.get("token").and_then(|v| v.as_str()) {
                        content.push_str(token_text);
                    }
                    if let Some(done_content) = event_data.get("content").and_then(|v| v.as_str()) {
                        content = done_content.to_string();
                        got_done = true;
                    }
                    if let Some(err) = event_data.get("error").and_then(|v| v.as_str()) {
                        last_error = Some(err.to_string());
                    }
                }
            }
        }

        if let Some(err) = last_error {
            return Err(Error::Api {
                code: 500,
                message: err,
            });
        }

        if !got_done && content.is_empty() {
            return Err(Error::Api {
                code: 500,
                message: "SSE stream ended without a done event".to_string(),
            });
        }

        Ok(RawCompletionResponse { content })
    }

    /// Stateless raw LLM completion via SSE with incremental token progress.
    ///
    /// Same as `raw_completion_stream()` but sends each token through the provided
    /// channel as it arrives, allowing callers to show real-time progress.
    /// Returns the final complete response.
    pub async fn raw_completion_stream_with_progress(
        &self,
        request: RawCompletionRequest,
        token: &str,
        progress_tx: tokio::sync::mpsc::Sender<String>,
    ) -> Result<RawCompletionResponse> {
        use futures_util::StreamExt;

        let url = self.base_url.join("/api/chat/complete/stream")?;

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "text/event-stream")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let code = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api {
                code,
                message: body,
            });
        }

        // Stream SSE events incrementally using bytes_stream instead of buffering
        let mut stream = response.bytes_stream();
        let mut buf = String::new();
        let mut content = String::new();
        let mut got_done = false;
        let mut last_error: Option<String> = None;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            buf.push_str(&String::from_utf8_lossy(&chunk));

            // Process complete lines from the buffer
            while let Some(newline_pos) = buf.find('\n') {
                let line = buf[..newline_pos].to_string();
                buf = buf[newline_pos + 1..].to_string();

                if let Some(data_str) = line.strip_prefix("data:") {
                    let data_str = data_str.trim();
                    if data_str.is_empty() {
                        continue;
                    }
                    if let Ok(event_data) = serde_json::from_str::<serde_json::Value>(data_str) {
                        if let Some(token_text) = event_data.get("token").and_then(|v| v.as_str()) {
                            content.push_str(token_text);
                            let _ = progress_tx.send(token_text.to_string()).await;
                        }
                        if let Some(done_content) =
                            event_data.get("content").and_then(|v| v.as_str())
                        {
                            content = done_content.to_string();
                            got_done = true;
                        }
                        if let Some(err) = event_data.get("error").and_then(|v| v.as_str()) {
                            last_error = Some(err.to_string());
                        }
                    }
                }
            }
        }

        if let Some(err) = last_error {
            return Err(Error::Api {
                code: 500,
                message: err,
            });
        }

        if !got_done && content.is_empty() {
            return Err(Error::Api {
                code: 500,
                message: "SSE stream ended without a done event".to_string(),
            });
        }

        Ok(RawCompletionResponse { content })
    }

    /// Stream a chat message via SSE (Server-Sent Events).
    ///
    /// Calls `POST /api/chat/{chat_id}/messages/stream` and yields
    /// `ChatStreamEvent` items through the returned channel as they arrive.
    /// Server-side tools execute normally; client-side tools are not supported.
    pub async fn chat_message_stream(
        &self,
        chat_id: &str,
        request: ChatMessageRequest,
        token: &str,
    ) -> Result<tokio::sync::mpsc::Receiver<crate::websocket::ChatStreamEvent>> {
        use crate::websocket::ChatStreamEvent;
        use futures_util::StreamExt;

        let url = self
            .base_url
            .join(&format!("/api/chat/{}/messages/stream", chat_id))?;

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "text/event-stream")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let code = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api {
                code,
                message: body,
            });
        }

        let (tx, rx) = tokio::sync::mpsc::channel::<ChatStreamEvent>(128);

        // Spawn a task to parse SSE events and forward as ChatStreamEvent
        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            let mut buf = String::new();

            while let Some(chunk_result) = stream.next().await {
                let chunk = match chunk_result {
                    Ok(c) => c,
                    Err(e) => {
                        let _ = tx.send(ChatStreamEvent::Error(e.to_string())).await;
                        return;
                    }
                };
                buf.push_str(&String::from_utf8_lossy(&chunk));

                // Process complete lines
                while let Some(newline_pos) = buf.find('\n') {
                    let line = buf[..newline_pos].to_string();
                    buf = buf[newline_pos + 1..].to_string();

                    if let Some(data_str) = line.strip_prefix("data:") {
                        let data_str = data_str.trim();
                        if data_str.is_empty() {
                            continue;
                        }
                        if let Ok(event_data) = serde_json::from_str::<serde_json::Value>(data_str)
                        {
                            // Token chunk
                            if let Some(token_text) =
                                event_data.get("token").and_then(|v| v.as_str())
                            {
                                if tx
                                    .send(ChatStreamEvent::Chunk(token_text.to_string()))
                                    .await
                                    .is_err()
                                {
                                    return; // receiver dropped
                                }
                            }
                            // Done event
                            if event_data.get("content").is_some() {
                                let message_id = event_data
                                    .get("message_id")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let exec_ms = event_data
                                    .get("execution_time_ms")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(0);
                                let context_window = event_data
                                    .get("context_window")
                                    .and_then(|v| v.as_u64())
                                    .map(|v| v as u32);
                                let token_usage = event_data.get("token_usage").cloned();
                                let tool_call_history =
                                    event_data.get("tool_call_history").cloned();

                                let _ = tx
                                    .send(ChatStreamEvent::End {
                                        message_id,
                                        token_usage,
                                        tool_call_history,
                                        execution_time_ms: exec_ms,
                                        context_window,
                                    })
                                    .await;
                                return;
                            }
                            // Tool-progress event (server emits when a
                            // server-side tool dispatch starts or
                            // finishes — frames carry an `"event":
                            // "tool_call"` or `"tool_result"` discriminator).
                            // We surface tool_call as ChatStreamEvent::ToolCall
                            // so the GUI can render mid-stream "searching X…"
                            // pills. tool_result is dropped here today: the
                            // existing GUI tool-status display already
                            // updates from the tool_call_history arriving
                            // in the End frame, so re-emitting result
                            // events would just duplicate.
                            if let Some(evt_name) = event_data.get("event").and_then(|v| v.as_str())
                                && evt_name == "tool_call"
                            {
                                // Both `tool` and `call_id` must be present
                                // and non-empty — emitting a ToolCall with
                                // empty identifiers makes downstream handling
                                // ambiguous (e.g. submit_chat_tool_result
                                // would land under the wrong call_id). Skip
                                // malformed frames rather than synthesizing
                                // an empty-string ID.
                                let tool_name =
                                    match event_data.get("tool").and_then(|v| v.as_str()) {
                                        Some(s) if !s.is_empty() => s.to_string(),
                                        _ => continue,
                                    };
                                let call_id =
                                    match event_data.get("call_id").and_then(|v| v.as_str()) {
                                        Some(s) if !s.is_empty() => s.to_string(),
                                        _ => continue,
                                    };
                                let arguments = event_data
                                    .get("args")
                                    .cloned()
                                    .unwrap_or(serde_json::Value::Null);
                                // Match the chunk branch's behavior: a
                                // failed send means the receiver dropped, so
                                // exit the read loop instead of spinning.
                                if tx
                                    .send(ChatStreamEvent::ToolCall {
                                        call_id,
                                        tool_name,
                                        arguments,
                                    })
                                    .await
                                    .is_err()
                                {
                                    return;
                                }
                                continue;
                            }
                            // Error event
                            if let Some(err) = event_data.get("error").and_then(|v| v.as_str()) {
                                let _ = tx.send(ChatStreamEvent::Error(err.to_string())).await;
                                return;
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }

    /// Create a new chat session
    pub async fn create_chat_session(
        &self,
        request: CreateChatSessionRequest,
        token: &str,
    ) -> Result<ChatResponse> {
        let url = self.base_url.join("/api/chat")?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Get a chat session by ID
    pub async fn get_chat_session(
        &self,
        chat_id: &str,
        token: &str,
    ) -> Result<ChatSessionResponse> {
        let url = self.base_url.join(&format!("/api/chat/{}", chat_id))?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Submit a client tool result for an in-flight SSE chat stream.
    /// This unblocks ekoDB's tool loop so it can feed the result to the LLM.
    pub async fn submit_chat_tool_result(
        &self,
        chat_id: &str,
        call_id: &str,
        success: bool,
        result: Option<serde_json::Value>,
        error: Option<String>,
        token: &str,
    ) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("/api/chat/{}/tool-result", chat_id))?;

        #[derive(serde::Serialize)]
        struct ToolResultBody {
            call_id: String,
            success: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            result: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error: Option<String>,
        }

        let body = ToolResultBody {
            call_id: call_id.to_string(),
            success,
            result,
            error,
        };

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await?;

                let status = response.status();
                if status == reqwest::StatusCode::UNAUTHORIZED {
                    return Err(crate::error::Error::TokenExpired);
                }
                if !status.is_success() {
                    let mut text = response.text().await.unwrap_or_default();
                    text.truncate(512);
                    return Err(crate::error::Error::Api {
                        code: status.as_u16(),
                        message: format!("tool-result submit failed: {text}"),
                    });
                }
                Ok(())
            })
            .await
    }

    /// List all chat sessions
    pub async fn list_chat_sessions(
        &self,
        query: ListSessionsQuery,
        token: &str,
    ) -> Result<ListSessionsResponse> {
        let mut url = self.base_url.join("/api/chat")?;

        // Add query parameters
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(limit) = query.limit {
                query_pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(skip) = query.skip {
                query_pairs.append_pair("skip", &skip.to_string());
            }
            if let Some(sort) = &query.sort {
                query_pairs.append_pair("sort", sort);
            }
        }

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Update chat session metadata
    pub async fn update_chat_session(
        &self,
        chat_id: &str,
        request: UpdateSessionRequest,
        token: &str,
    ) -> Result<ChatSessionResponse> {
        let url = self.base_url.join(&format!("/api/chat/{}", chat_id))?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .put(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Delete a chat session
    pub async fn delete_chat_session(&self, chat_id: &str, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/chat/{}", chat_id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    let error: ErrorResponse = response.json().await?;
                    Err(Error::api(error.code, error.message))
                }
            })
            .await
    }

    /// Branch a chat session from an existing one
    pub async fn branch_chat_session(
        &self,
        request: CreateChatSessionRequest,
        token: &str,
    ) -> Result<ChatResponse> {
        let url = self.base_url.join("/api/chat/branch")?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Merge multiple chat sessions
    pub async fn merge_chat_sessions(
        &self,
        request: MergeSessionsRequest,
        token: &str,
    ) -> Result<ChatSessionResponse> {
        let url = self.base_url.join("/api/chat/merge")?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Send a message in an existing chat session
    pub async fn chat_message(
        &self,
        chat_id: &str,
        request: ChatMessageRequest,
        token: &str,
    ) -> Result<ChatResponse> {
        let url = self
            .base_url
            .join(&format!("/api/chat/{}/messages", chat_id))?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                let status = response.status();
                let bytes = response.bytes().await.map_err(Error::Http)?;
                if !status.is_success() {
                    // Try to extract error message from response body
                    if let Ok(err_obj) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                        let msg = err_obj["error"].as_str().unwrap_or("unknown error");
                        return Err(Error::Api {
                            code: status.as_u16(),
                            message: msg.to_string(),
                        });
                    }
                    return Err(Error::Api {
                        code: status.as_u16(),
                        message: format!("chat message failed ({})", status),
                    });
                }
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
            })
            .await
    }

    /// Get messages from a chat session
    pub async fn get_chat_session_messages(
        &self,
        chat_id: &str,
        query: GetMessagesQuery,
        token: &str,
    ) -> Result<crate::chat::GetMessagesResponse> {
        let mut url = self
            .base_url
            .join(&format!("/api/chat/{}/messages", chat_id))?;

        // Add query parameters
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(limit) = query.limit {
                query_pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(skip) = query.skip {
                query_pairs.append_pair("skip", &skip.to_string());
            }
            if let Some(sort) = &query.sort {
                query_pairs.append_pair("sort", sort);
            }
        }

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Get a specific message by ID
    pub async fn get_chat_message(
        &self,
        chat_id: &str,
        message_id: &str,
        token: &str,
    ) -> Result<Record> {
        let url = self
            .base_url
            .join(&format!("/api/chat/{}/messages/{}", chat_id, message_id))?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Update a chat message
    pub async fn update_chat_message(
        &self,
        chat_id: &str,
        message_id: &str,
        request: UpdateMessageRequest,
        token: &str,
    ) -> Result<Record> {
        let url = self
            .base_url
            .join(&format!("/api/chat/{}/messages/{}", chat_id, message_id))?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .put(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Delete a chat message
    pub async fn delete_chat_message(
        &self,
        chat_id: &str,
        message_id: &str,
        token: &str,
    ) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("/api/chat/{}/messages/{}", chat_id, message_id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    let error: ErrorResponse = response.json().await?;
                    Err(Error::api(error.code, error.message))
                }
            })
            .await
    }

    /// Toggle message forgotten status
    pub async fn toggle_forgotten_message(
        &self,
        chat_id: &str,
        message_id: &str,
        request: ToggleForgottenRequest,
        token: &str,
    ) -> Result<Record> {
        let url = self.base_url.join(&format!(
            "/api/chat/{}/messages/{}/forgotten",
            chat_id, message_id
        ))?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .patch(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Regenerate a chat message
    pub async fn regenerate_chat_message(
        &self,
        chat_id: &str,
        message_id: &str,
        token: &str,
    ) -> Result<ChatResponse> {
        let url = self.base_url.join(&format!(
            "/api/chat/{}/messages/{}/regenerate",
            chat_id, message_id
        ))?;

        // Force JSON for chat operations
        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    // ========================================================================
    // FUNCTIONS API
    // ========================================================================

    /// Save a UserFunction definition
    pub async fn save_function(
        &self,
        function: crate::functions::UserFunction,
        token: &str,
    ) -> Result<String> {
        let url = self.base_url.join("/api/functions")?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .json(&function)
                    .send()
                    .await?;

                let status = response.status();
                let bytes = response.bytes().await.map_err(Error::Http)?;

                if !status.is_success() {
                    let error_msg = String::from_utf8_lossy(&bytes);
                    return Err(Error::api(
                        status.as_u16(),
                        format!("Server error: {}", error_msg),
                    ));
                }

                if bytes.is_empty() {
                    return Err(Error::api(
                        status.as_u16(),
                        "Empty response from server".to_string(),
                    ));
                }

                #[derive(Deserialize)]
                struct FunctionResponse {
                    status: String,
                    id: String,
                }

                let result: FunctionResponse = serde_json::from_slice(&bytes).map_err(|e| {
                    Error::api(
                        500,
                        format!(
                            "Failed to parse response: {} (body: {})",
                            e,
                            String::from_utf8_lossy(&bytes)
                        ),
                    )
                })?;

                if result.status != "success" {
                    return Err(Error::api(
                        500,
                        format!("Failed to save script: status={}", result.status),
                    ));
                }

                Ok(result.id)
            })
            .await
    }

    /// Get a UserFunction by ID
    pub async fn get_function(
        &self,
        id: &str,
        token: &str,
    ) -> Result<crate::functions::UserFunction> {
        let url = self.base_url.join(&format!("/api/functions/{}", id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// List all functions (optionally filtered by tags)
    pub async fn list_functions(
        &self,
        tags: Option<Vec<String>>,
        token: &str,
    ) -> Result<Vec<crate::functions::UserFunction>> {
        let mut url = self.base_url.join("/api/functions")?;

        if let Some(tags) = tags {
            let tags_query = tags.join(",");
            url.set_query(Some(&format!("tags={}", tags_query)));
        }

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Update an existing UserFunction by ID
    pub async fn update_function(
        &self,
        id: &str,
        function: crate::functions::UserFunction,
        token: &str,
    ) -> Result<()> {
        let url = self.base_url.join(&format!("/api/functions/{}", id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .put(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .json(&function)
                    .send()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    let status = response.status().as_u16();
                    let bytes = response.bytes().await.map_err(Error::Http)?;
                    let error_msg = String::from_utf8_lossy(&bytes);
                    Err(Error::api(status, error_msg.to_string()))
                }
            })
            .await
    }

    /// Delete a UserFunction by ID
    pub async fn delete_function(&self, id: &str, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/functions/{}", id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    let status = response.status().as_u16();
                    let bytes = response.bytes().await.map_err(Error::Http)?;
                    let error_msg = String::from_utf8_lossy(&bytes);
                    Err(Error::api(status, error_msg.to_string()))
                }
            })
            .await
    }

    /// Call a saved UserFunction by ID or label
    pub async fn call_function(
        &self,
        function_id_or_label: &str,
        params: Option<std::collections::HashMap<String, crate::types::FieldType>>,
        token: &str,
    ) -> Result<crate::functions::FunctionResult> {
        let url = self
            .base_url
            .join(&format!("/api/functions/{}", function_id_or_label))?;

        let body = params.unwrap_or_default();

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .json(&body)
                    .send()
                    .await?;

                let status = response.status();
                let bytes = response.bytes().await.map_err(Error::Http)?;

                if !status.is_success() {
                    let error_msg = String::from_utf8_lossy(&bytes);
                    return Err(Error::api(
                        status.as_u16(),
                        format!("Server error: {}", error_msg),
                    ));
                }

                if bytes.is_empty() {
                    return Err(Error::api(
                        status.as_u16(),
                        "Empty response from server".to_string(),
                    ));
                }

                serde_json::from_slice(&bytes).map_err(|e| {
                    Error::api(
                        500,
                        format!(
                            "Failed to parse response: {} (body: {})",
                            e,
                            String::from_utf8_lossy(&bytes)
                        ),
                    )
                })
            })
            .await
    }

    // ========================================================================
    // USER FUNCTIONS API
    // ========================================================================

    /// Save a UserFunction definition
    pub async fn save_user_function(
        &self,
        user_function: crate::functions::UserFunction,
        token: &str,
    ) -> Result<String> {
        let url = self.base_url.join("/api/functions")?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .json(&user_function)
                    .send()
                    .await?;

                let status = response.status();
                let bytes = response.bytes().await.map_err(Error::Http)?;

                if !status.is_success() {
                    let error_msg = String::from_utf8_lossy(&bytes);
                    return Err(Error::api(
                        status.as_u16(),
                        format!("Server error: {}", error_msg),
                    ));
                }

                if bytes.is_empty() {
                    return Err(Error::api(
                        status.as_u16(),
                        "Empty response from server".to_string(),
                    ));
                }

                #[derive(Deserialize)]
                struct FunctionResponse {
                    status: String,
                    id: String,
                }

                let result: FunctionResponse = serde_json::from_slice(&bytes).map_err(|e| {
                    Error::api(
                        500,
                        format!(
                            "Failed to parse response: {} (body: {})",
                            e,
                            String::from_utf8_lossy(&bytes)
                        ),
                    )
                })?;

                if result.status != "success" {
                    return Err(Error::api(
                        500,
                        format!("Failed to save user function: status={}", result.status),
                    ));
                }

                Ok(result.id)
            })
            .await
    }

    /// Get a UserFunction by label
    pub async fn get_user_function(
        &self,
        label: &str,
        token: &str,
    ) -> Result<crate::functions::UserFunction> {
        let url = self.base_url.join(&format!("/api/functions/{}", label))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// List all UserFunctions (optionally filtered by tags)
    pub async fn list_user_functions(
        &self,
        tags: Option<Vec<String>>,
        token: &str,
    ) -> Result<Vec<crate::functions::UserFunction>> {
        let mut url = self.base_url.join("/api/functions")?;

        if let Some(tags) = tags {
            let tags_query = tags.join(",");
            url.set_query(Some(&format!("tags={}", tags_query)));
        }

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                Self::json_body(response).await
            })
            .await
    }

    /// Update an existing UserFunction
    pub async fn update_user_function(
        &self,
        label: &str,
        user_function: crate::functions::UserFunction,
        token: &str,
    ) -> Result<()> {
        let url = self.base_url.join(&format!("/api/functions/{}", label))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .put(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .json(&user_function)
                    .send()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    let status = response.status().as_u16();
                    let bytes = response.bytes().await.map_err(Error::Http)?;
                    let error_msg = String::from_utf8_lossy(&bytes);
                    Err(Error::api(status, error_msg.to_string()))
                }
            })
            .await
    }

    /// Delete a UserFunction by label
    pub async fn delete_user_function(&self, label: &str, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/functions/{}", label))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                if response.status().is_success() {
                    Ok(())
                } else {
                    let status = response.status().as_u16();
                    let bytes = response.bytes().await.map_err(Error::Http)?;
                    let error_msg = String::from_utf8_lossy(&bytes);
                    Err(Error::api(status, error_msg.to_string()))
                }
            })
            .await
    }
    // ── Goal CRUD ────────────────────────────────────────────────────────────

    pub async fn goal_create(
        &self,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/goals")?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/goals", response).await
    }

    pub async fn goal_list(&self, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/goals")?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/goals", response).await
    }

    pub async fn goal_get(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/chat/goals/{}", id))?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/goals/{id}", response).await
    }

    pub async fn goal_update(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/chat/goals/{}", id))?;
        let response = self
            .client
            .put(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/goals/{id}", response).await
    }

    pub async fn goal_delete(&self, id: &str, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/chat/goals/{}", id))?;
        let response = self
            .client
            .delete(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response::<serde_json::Value>("/api/chat/goals/{id}", response)
            .await?;
        Ok(())
    }

    pub async fn goal_search(&self, query: &str, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/goals/search")?;
        let response = self
            .client
            .get(url)
            .query(&[("q", query)])
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/goals/search", response)
            .await
    }

    // ── Goal lifecycle ─────────────────────────────────────────────────────

    pub async fn goal_complete(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/goals/{}/complete", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/goals/{id}/complete", response)
            .await
    }

    pub async fn goal_approve(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/goals/{}/approve", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/goals/{id}/approve", response)
            .await
    }

    pub async fn goal_reject(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/goals/{}/reject", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/goals/{id}/reject", response)
            .await
    }

    // ── Goal step lifecycle ──────────────────────────────────────────────────

    pub async fn goal_step_start(
        &self,
        id: &str,
        step_index: usize,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!(
            "/api/chat/goals/{}/steps/{}/start",
            id, step_index
        ))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/goals/{id}/steps/{index}/start", response)
            .await
    }

    pub async fn goal_step_complete(
        &self,
        id: &str,
        step_index: usize,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!(
            "/api/chat/goals/{}/steps/{}/complete",
            id, step_index
        ))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/goals/{id}/steps/{index}/complete", response)
            .await
    }

    pub async fn goal_step_fail(
        &self,
        id: &str,
        step_index: usize,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/goals/{}/steps/{}/fail", id, step_index))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/goals/{id}/steps/{index}/fail", response)
            .await
    }

    // ── Task CRUD ────────────────────────────────────────────────────────────

    pub async fn task_create(
        &self,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/tasks")?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/tasks", response).await
    }

    pub async fn task_list(&self, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/tasks")?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/tasks", response).await
    }

    pub async fn task_get(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/chat/tasks/{}", id))?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/tasks/{id}", response).await
    }

    pub async fn task_update(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/chat/tasks/{}", id))?;
        let response = self
            .client
            .put(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/tasks/{id}", response).await
    }

    pub async fn task_delete(&self, id: &str, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/chat/tasks/{}", id))?;
        let response = self
            .client
            .delete(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response::<serde_json::Value>("/api/chat/tasks/{id}", response)
            .await?;
        Ok(())
    }

    pub async fn task_due(&self, now: &str, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/tasks/due")?;
        let response = self
            .client
            .get(url)
            .query(&[("now", now)])
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/tasks/due", response).await
    }

    // ── Task lifecycle ─────────────────────────────────────────────────────

    pub async fn task_start(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/tasks/{}/start", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/tasks/{id}/start", response)
            .await
    }

    pub async fn task_succeed(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/tasks/{}/succeed", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/tasks/{id}/succeed", response)
            .await
    }

    pub async fn task_fail(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/tasks/{}/fail", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/tasks/{id}/fail", response)
            .await
    }

    pub async fn task_pause(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/tasks/{}/pause", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/tasks/{id}/pause", response)
            .await
    }

    pub async fn task_resume(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/tasks/{}/resume", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/tasks/{id}/resume", response)
            .await
    }

    // ── Agent CRUD ───────────────────────────────────────────────────────────

    pub async fn agent_create(
        &self,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/agents")?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/agents", response).await
    }

    pub async fn agent_list(&self, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/agents")?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/agents", response).await
    }

    pub async fn agent_get(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/chat/agents/{}", id))?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/agents/{id}", response)
            .await
    }

    pub async fn agent_get_by_name(&self, name: &str, token: &str) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/agents/by-name/{}", name))?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/agents/by-name/{name}", response)
            .await
    }

    pub async fn agent_update(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/chat/agents/{}", id))?;
        let response = self
            .client
            .put(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/agents/{id}", response)
            .await
    }

    pub async fn agent_delete(&self, id: &str, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/chat/agents/{}", id))?;
        let response = self
            .client
            .delete(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response::<serde_json::Value>("/api/chat/agents/{id}", response)
            .await?;
        Ok(())
    }

    pub async fn agents_by_deployment(
        &self,
        deployment_id: &str,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/agents/by-deployment/{}", deployment_id))?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/agents/by-deployment/{id}", response)
            .await
    }

    // ── Goal Template CRUD ─────────────────────────────────────────────────

    pub async fn goal_template_create(
        &self,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/goal-templates")?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/goal-templates", response)
            .await
    }

    pub async fn goal_template_list(&self, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/chat/goal-templates")?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/goal-templates", response)
            .await
    }

    pub async fn goal_template_get(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/goal-templates/{}", id))?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/chat/goal-templates/{id}", response)
            .await
    }

    pub async fn goal_template_update(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/chat/goal-templates/{}", id))?;
        let response = self
            .client
            .put(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/chat/goal-templates/{id}", response)
            .await
    }

    pub async fn goal_template_delete(&self, id: &str, token: &str) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("/api/chat/goal-templates/{}", id))?;
        let response = self
            .client
            .delete(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response::<serde_json::Value>("/api/chat/goal-templates/{id}", response)
            .await?;
        Ok(())
    }

    // ── KV Document Linking ─────────────────────────────────────────────────

    pub async fn kv_get_links(&self, key: &str, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/kv/links/{}", key))?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/kv/links/{key}", response).await
    }

    pub async fn kv_link(
        &self,
        key: &str,
        collection: &str,
        document_id: &str,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/kv/link")?;
        let body = serde_json::json!({
            "key": key,
            "collection": collection,
            "document_id": document_id,
        });
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await?;
        self.handle_response("/api/kv/link", response).await
    }

    pub async fn kv_unlink(
        &self,
        key: &str,
        collection: &str,
        document_id: &str,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/kv/unlink")?;
        let body = serde_json::json!({
            "key": key,
            "collection": collection,
            "document_id": document_id,
        });
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await?;
        self.handle_response("/api/kv/unlink", response).await
    }

    // ── Schedule Management ─────────────────────────────────────────────────

    pub async fn create_schedule(
        &self,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/schedules")?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/schedules", response).await
    }

    pub async fn list_schedules(&self, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join("/api/schedules")?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/schedules", response).await
    }

    pub async fn get_schedule(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/schedules/{}", id))?;
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/schedules/{id}", response).await
    }

    pub async fn update_schedule(
        &self,
        id: &str,
        data: serde_json::Value,
        token: &str,
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join(&format!("/api/schedules/{}", id))?;
        let response = self
            .client
            .put(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
        self.handle_response("/api/schedules/{id}", response).await
    }

    pub async fn delete_schedule(&self, id: &str, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/schedules/{}", id))?;
        let response = self
            .client
            .delete(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response::<serde_json::Value>("/api/schedules/{id}", response)
            .await?;
        Ok(())
    }

    pub async fn pause_schedule(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/schedules/{}/pause", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/schedules/{id}/pause", response)
            .await
    }

    pub async fn resume_schedule(&self, id: &str, token: &str) -> Result<serde_json::Value> {
        let url = self
            .base_url
            .join(&format!("/api/schedules/{}/resume", id))?;
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.handle_response("/api/schedules/{id}/resume", response)
            .await
    }
}

/// Extract the `"message"` or `"error"` field from a JSON error body,
/// falling back to the raw string if parsing fails.
fn extract_error_message(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| {
            v.get("message")
                .or_else(|| v.get("error"))
                .and_then(|m| m.as_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| body.to_string())
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}
