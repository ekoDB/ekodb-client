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
use crate::types::{Query, Record};
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
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new(
        base_url: &str,
        timeout: Duration,
        max_retries: u32,
        should_retry: bool,
    ) -> Result<Self> {
        let client = ReqwestClient::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| Error::Connection(e.to_string()))?;

        let base_url = Url::parse(base_url)?;
        let retry_policy = RetryPolicy::new(max_retries);

        Ok(Self {
            client,
            base_url,
            retry_policy,
            should_retry,
        })
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
    pub async fn insert(&self, collection: &str, record: Record, token: &str) -> Result<Record> {
        let url = self.base_url.join(&format!("/api/insert/{}", collection))?;

        self.execute_with_retry(|| async {
            let response = self
                .client
                .post(url.clone())
                .header("Authorization", format!("Bearer {}", token))
                .json(&record)
                .send()
                .await?;

            self.handle_response(response).await
        })
        .await
    }

    /// Find records
    pub async fn find(&self, collection: &str, query: Query, token: &str) -> Result<Vec<Record>> {
        let url = self.base_url.join(&format!("/api/find/{}", collection))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&query)
                    .send()
                    .await?;

                self.handle_response(response).await
            })
            .await
    }

    /// Find a record by ID
    pub async fn find_by_id(&self, collection: &str, id: &str, token: &str) -> Result<Record> {
        let url = self
            .base_url
            .join(&format!("/api/find/{}/{}", collection, id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                self.handle_response(response).await
            })
            .await
    }

    /// Update a record
    pub async fn update(
        &self,
        collection: &str,
        id: &str,
        record: Record,
        token: &str,
    ) -> Result<Record> {
        let url = self
            .base_url
            .join(&format!("/api/update/{}/{}", collection, id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .put(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&record)
                    .send()
                    .await?;

                self.handle_response(response).await
            })
            .await
    }

    /// Delete a record
    pub async fn delete(&self, collection: &str, id: &str, token: &str) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("/api/delete/{}/{}", collection, id))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                // Server returns the deleted record, but we discard it
                let _deleted: Record = self.handle_response(response).await?;
                Ok(())
            })
            .await
    }

    /// Batch insert records
    pub async fn batch_insert(
        &self,
        collection: &str,
        records: Vec<Record>,
        token: &str,
    ) -> Result<Vec<Record>> {
        let url = self
            .base_url
            .join(&format!("/api/batch/insert/{}", collection))?;

        // Convert to the format the server expects
        #[derive(Serialize)]
        struct BatchInsertItem {
            data: Record,
        }

        #[derive(Serialize)]
        struct BatchInsertQuery {
            inserts: Vec<BatchInsertItem>,
        }

        let batch_data = BatchInsertQuery {
            inserts: records
                .into_iter()
                .map(|r| BatchInsertItem { data: r })
                .collect(),
        };

        #[derive(Deserialize)]
        struct BatchOperationResult {
            successful: Vec<String>,
            #[allow(dead_code)]
            failed: Vec<serde_json::Value>,
        }

        let result: BatchOperationResult = self
            .retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&batch_data)
                    .send()
                    .await?;

                self.handle_response(response).await
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
    ) -> Result<Vec<Record>> {
        let url = self
            .base_url
            .join(&format!("/api/batch/update/{}", collection))?;

        // Convert to the format the server expects
        #[derive(Serialize)]
        struct BatchUpdateItem {
            #[serde(rename = "id")]
            id: String,
            data: Record,
        }

        #[derive(Serialize)]
        struct BatchUpdateQuery {
            updates: Vec<BatchUpdateItem>,
        }

        let batch_data = BatchUpdateQuery {
            updates: updates
                .into_iter()
                .map(|(id, data)| BatchUpdateItem { id, data })
                .collect(),
        };

        #[derive(Deserialize)]
        struct BatchOperationResult {
            successful: Vec<String>,
            #[allow(dead_code)]
            failed: Vec<serde_json::Value>,
        }

        let result: BatchOperationResult = self
            .retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .put(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&batch_data)
                    .send()
                    .await?;

                self.handle_response(response).await
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
    ) -> Result<u64> {
        let url = self
            .base_url
            .join(&format!("/api/batch/delete/{}", collection))?;

        // Convert to the format the server expects
        #[derive(Serialize)]
        struct BatchDeleteItem {
            #[serde(rename = "id")]
            id: String,
        }

        #[derive(Serialize)]
        struct BatchDeleteQuery {
            deletes: Vec<BatchDeleteItem>,
        }

        let batch_data = BatchDeleteQuery {
            deletes: ids.into_iter().map(|id| BatchDeleteItem { id }).collect(),
        };

        #[derive(Deserialize)]
        struct BatchOperationResult {
            successful: Vec<String>,
            #[allow(dead_code)]
            failed: Vec<serde_json::Value>,
        }

        let result: BatchOperationResult = self
            .retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&batch_data)
                    .send()
                    .await?;

                self.handle_response(response).await
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
                    .send()
                    .await?;

                self.handle_response(response).await
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
                    .send()
                    .await?;

                // Server might return empty or success message
                let _: serde_json::Value = self.handle_response(response).await?;
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
                    .json(&KvSetRequest {
                        value: value.clone(),
                    })
                    .send()
                    .await?;

                let _: serde_json::Value = self.handle_response(response).await?;
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
                    .send()
                    .await?;

                self.handle_response::<KvGetResponse>(response).await
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
                    .send()
                    .await?;

                let _: serde_json::Value = self.handle_response(response).await?;
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&search_query)
                    .send()
                    .await?;

                self.handle_response(response).await
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
                    .json(&schema)
                    .send()
                    .await?;

                let _: serde_json::Value = self.handle_response(response).await?;
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
                    .send()
                    .await?;

                self.handle_response(response).await
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
                    .send()
                    .await?;

                self.handle_response(response).await
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
    async fn handle_response<T: for<'de> Deserialize<'de>>(&self, response: Response) -> Result<T> {
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

                // Get the response text first for better error messages
                let text = response.text().await?;
                serde_json::from_str(&text).map_err(|e| {
                    Error::Validation(format!(
                        "Failed to parse response as JSON: {}. Response was: {}",
                        e,
                        text.chars().take(200).collect::<String>()
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                self.handle_response(response).await
            })
            .await
    }

    /// Get specific chat model info
    pub async fn get_chat_model(&self, model_name: &str, token: &str) -> Result<Vec<String>> {
        let url = self
            .base_url
            .join(&format!("/api/chat_models/{}", model_name))?;

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .put(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .get(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .put(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .patch(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await?;

                self.handle_response(response).await
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

        self.retry_policy
            .execute(|| async {
                let response = self
                    .client
                    .post(url.clone())
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                self.handle_response(response).await
            })
            .await
    }
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}
