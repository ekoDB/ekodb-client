//! HTTP client implementation for ekoDB API

use crate::chat::{
    ChatMessageRequest, ChatResponse, ChatSessionResponse, CreateChatSessionRequest,
    GetMessagesQuery, ListSessionsQuery, ListSessionsResponse, MergeSessionsRequest, Models,
    ToggleForgottenRequest, UpdateMessageRequest, UpdateSessionRequest,
};
use crate::client::RateLimitInfo;
use crate::error::{Error, Result};
use crate::retry::RetryPolicy;
use crate::schema::{CollectionMetadata, Schema};
use crate::search::{SearchQuery, SearchResponse};
use crate::types::{Query, Record, SerializationFormat};
use reqwest::{Client as ReqwestClient, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

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
        let client = ReqwestClient::builder()
            .timeout(timeout)
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
        record: Record,
        bypass_ripple: Option<bool>,
        token: &str,
    ) -> Result<Record> {
        let url_path = if let Some(bypass) = bypass_ripple {
            format!("/api/insert/{}?bypass_ripple={}", collection, bypass)
        } else {
            format!("/api/insert/{}", collection)
        };
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

        self.retry_policy
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
        bypass_ripple: Option<bool>,
        token: &str,
    ) -> Result<Record> {
        let url_path = if let Some(bypass) = bypass_ripple {
            format!("/api/update/{}/{}?bypass_ripple={}", collection, id, bypass)
        } else {
            format!("/api/update/{}/{}", collection, id)
        };
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
                record.insert("id", id);
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
                record.insert("id", id);
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
        let url = self.base_url.join("/api/collections")?;

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
                    .header("Accept", "application/json") // Force JSON for metadata operations
                    .send()
                    .await?;

                // Force JSON deserialization for metadata operations
                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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
                let bytes = response.bytes().await.map_err(Error::Http)?;
                let _: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(Error::Serialization)?;
                Ok(())
            })
            .await
    }

    /// Set a key-value pair
    pub async fn kv_set(&self, key: &str, value: serde_json::Value, token: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/api/kv/set/{}", key))?;

        #[derive(Serialize)]
        struct KvSetRequest {
            value: serde_json::Value,
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
                    })
                    .send()
                    .await?;

                // Force JSON for KV operations (stores serde_json::Value)
                let bytes = response.bytes().await.map_err(Error::Http)?;
                let _: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(Error::Serialization)?;
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
                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice::<KvGetResponse>(&bytes).map_err(Error::Serialization)
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
                let bytes = response.bytes().await.map_err(Error::Http)?;
                let _: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(Error::Serialization)?;
                Ok(())
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
                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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
                let bytes = response.bytes().await.map_err(Error::Http)?;
                let _: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(Error::Serialization)?;
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
                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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
                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
            })
            .await
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
                    let stream_reader = StreamReader::new(byte_stream.map(|result| {
                        result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                    }));
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
                let error_body: ErrorResponse = response.json().await?;
                Err(Error::Auth(error_body.message))
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
            })
            .await
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
            })
            .await
    }

    // ========================================================================
    // SCRIPTS API
    // ========================================================================

    /// Save a Script definition
    pub async fn save_script(
        &self,
        script: crate::functions::Script,
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
                    .json(&script)
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

    /// Get a Script by ID
    pub async fn get_script(&self, id: &str, token: &str) -> Result<crate::functions::Script> {
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
            })
            .await
    }

    /// List all Scripts (optionally filtered by tags)
    pub async fn list_scripts(
        &self,
        tags: Option<Vec<String>>,
        token: &str,
    ) -> Result<Vec<crate::functions::Script>> {
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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
            })
            .await
    }

    /// Update an existing Script by ID
    pub async fn update_script(
        &self,
        id: &str,
        script: crate::functions::Script,
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
                    .json(&script)
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

    /// Delete a Script by ID
    pub async fn delete_script(&self, id: &str, token: &str) -> Result<()> {
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

    /// Call a saved Script by ID or label
    pub async fn call_script(
        &self,
        script_id_or_label: &str,
        params: Option<std::collections::HashMap<String, crate::types::FieldType>>,
        token: &str,
    ) -> Result<crate::functions::FunctionResult> {
        let url = self
            .base_url
            .join(&format!("/api/functions/{}", script_id_or_label))?;

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
        let url = self.base_url.join("/api/user-functions")?;

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
        let url = self
            .base_url
            .join(&format!("/api/user-functions/{}", label))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
            })
            .await
    }

    /// List all UserFunctions (optionally filtered by tags)
    pub async fn list_user_functions(
        &self,
        tags: Option<Vec<String>>,
        token: &str,
    ) -> Result<Vec<crate::functions::UserFunction>> {
        let mut url = self.base_url.join("/api/user-functions")?;

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

                let bytes = response.bytes().await.map_err(Error::Http)?;
                serde_json::from_slice(&bytes).map_err(Error::Serialization)
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
        let url = self
            .base_url
            .join(&format!("/api/user-functions/{}", label))?;

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
        let url = self
            .base_url
            .join(&format!("/api/user-functions/{}", label))?;

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
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}
