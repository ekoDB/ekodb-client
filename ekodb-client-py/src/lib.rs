//! Python bindings for ekoDB client library
//!
//! This module provides Python bindings for the ekoDB Rust client library using PyO3.

use ekodb_client::{
    ChatMessageRequest, ChatResponse, Client as RustClient, CollectionConfig,
    CreateChatSessionRequest, FieldType, GetMessagesQuery,
    GetMessagesResponse, ListSessionsQuery, ListSessionsResponse, Query as RustQuery,
    RateLimitInfo as RustRateLimitInfo, Record as RustRecord, SerializationFormat as RustSerializationFormat,
    UpdateSessionRequest, WebSocketClient as RustWebSocketClient,
};
use serde_json;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3_asyncio::tokio::future_into_py;

// Create a custom exception for rate limiting
create_exception!(ekodb_client, RateLimitError, PyException, "Rate limit exceeded");

/// Serialization format for client-server communication
#[pyclass]
#[derive(Clone, Copy)]
enum SerializationFormat {
    /// JSON format (default, human-readable)
    Json,
    /// MessagePack format (binary, 2-3x faster)
    MessagePack,
}

impl From<SerializationFormat> for RustSerializationFormat {
    fn from(format: SerializationFormat) -> Self {
        match format {
            SerializationFormat::Json => RustSerializationFormat::Json,
            SerializationFormat::MessagePack => RustSerializationFormat::MessagePack,
        }
    }
}

/// Rate limit information from the server
#[pyclass]
#[derive(Clone)]
struct RateLimitInfo {
    /// Maximum requests allowed per window
    #[pyo3(get)]
    limit: usize,
    /// Requests remaining in current window
    #[pyo3(get)]
    remaining: usize,
    /// Unix timestamp when the rate limit resets
    #[pyo3(get)]
    reset: i64,
}

#[pymethods]
impl RateLimitInfo {
    /// Check if approaching rate limit (less than 10% remaining)
    fn is_near_limit(&self) -> bool {
        let threshold = (self.limit as f64 * 0.1) as usize;
        self.remaining <= threshold
    }

    /// Check if the rate limit has been exceeded
    fn is_exceeded(&self) -> bool {
        self.remaining == 0
    }

    /// Get the percentage of requests remaining
    fn remaining_percentage(&self) -> f64 {
        (self.remaining as f64 / self.limit as f64) * 100.0
    }

    fn __repr__(&self) -> String {
        format!(
            "RateLimitInfo(limit={}, remaining={}, reset={})",
            self.limit, self.remaining, self.reset
        )
    }
}

impl From<RustRateLimitInfo> for RateLimitInfo {
    fn from(info: RustRateLimitInfo) -> Self {
        RateLimitInfo {
            limit: info.limit,
            remaining: info.remaining,
            reset: info.reset,
        }
    }
}

/// Python wrapper for ekoDB Client
#[pyclass]
struct Client {
    inner: RustClient,
}

#[pymethods]
impl Client {
    /// Create a new ekoDB client
    ///
    /// Args:
    ///     base_url: The base URL of the ekoDB server
    ///     api_key: Your API key
    ///     should_retry: Enable automatic retries (default: True)
    ///     max_retries: Maximum number of retry attempts (default: 3)
    ///     timeout_secs: Request timeout in seconds (default: 30)
    ///     format: Serialization format (default: SerializationFormat.Json)
    ///             Use SerializationFormat.MessagePack for 2-3x better performance
    ///
    /// Returns:
    ///     A new Client instance
    #[staticmethod]
    #[pyo3(signature = (base_url, api_key, should_retry=true, max_retries=3, timeout_secs=30, format=None))]
    fn new(
        base_url: String,
        api_key: String,
        should_retry: bool,
        max_retries: usize,
        timeout_secs: u64,
        format: Option<SerializationFormat>,
    ) -> PyResult<Self> {
        let mut builder = RustClient::builder()
            .base_url(&base_url)
            .api_key(&api_key)
            .should_retry(should_retry)
            .max_retries(max_retries)
            .timeout(std::time::Duration::from_secs(timeout_secs));
        
        if let Some(fmt) = format {
            builder = builder.serialization_format(fmt.into());
        }
        
        let client = builder.build()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create client: {}", e)))?;

        Ok(Client { inner: client })
    }

    /// Insert a document into a collection
    /// 
    /// Args:
    ///     collection: Collection name
    ///     record: Document data as a dict
    ///     ttl: Optional TTL duration (e.g., "30s", "5m", "1h", "1d")
    fn insert<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        record: &PyDict,
        ttl: Option<String>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<&'py PyAny> {
        let mut rust_record = dict_to_record(record)?;
        
        // Add TTL if provided
        if let Some(ttl_duration) = ttl {
            rust_record = rust_record.with_ttl(ttl_duration);
        }
        
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let result = client
                .insert(&collection, rust_record, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Insert failed: {}", e)))?;

            Python::with_gil(|py| record_to_dict(py, &result))
        })
    }

    /// Find a document by ID
    fn find_by_id<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        bypass_ripple: Option<bool>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let result = client
                .find_by_id(&collection, &id, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Find failed: {}", e)))?;

            Python::with_gil(|py| record_to_dict(py, &result))
        })
    }

    /// Find documents matching a query
    /// 
    /// Args:
    ///     collection: Collection name
    ///     query: Optional query dict with filters, joins, etc.
    ///     limit: Optional limit (deprecated, use query dict instead)
    fn find<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        query: Option<&PyDict>,
        limit: Option<usize>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();
        let query_json = if let Some(q) = query {
            Some(dict_to_json(q)?)
        } else {
            None
        };

        future_into_py::<_, PyObject>(py, async move {
            let mut rust_query = RustQuery::new();
            
            // Apply limit from parameter if provided (for backward compatibility)
            if let Some(l) = limit {
                rust_query = rust_query.limit(l);
            }
            
            // Parse query dict if provided
            if let Some(q) = query_json {
                rust_query = serde_json::from_value(q).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to parse query: {}", e))
                })?;
            }

            let results = client
                .find(&collection, rust_query, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Find failed: {}", e)))?;

            Python::with_gil(|py| {
                let list = PyList::empty(py);
                for record in results {
                    list.append(record_to_dict(py, &record)?)?;
                }
                Ok(list.into())
            })
        })
    }

    /// Update a document
    fn update<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        updates: &PyDict,
        bypass_ripple: Option<bool>,
    ) -> PyResult<&'py PyAny> {
        let rust_updates = dict_to_record(updates)?;
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let result = client
                .update(&collection, &id, rust_updates, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Update failed: {}", e)))?;

            Python::with_gil(|py| record_to_dict(py, &result))
        })
    }

    /// Delete a document
    fn delete<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        bypass_ripple: Option<bool>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            client
                .delete(&collection, &id, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    /// List all collections
    fn list_collections<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let collections = client
                .list_collections()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("List collections failed: {}", e)))?;

            Python::with_gil(|py| {
                let list = PyList::empty(py);
                for name in collections {
                    list.append(name)?;
                }
                Ok(list.into())
            })
        })
    }

    /// Batch insert multiple documents
    #[pyo3(signature = (collection, records, bypass_ripple=None))]
    fn batch_insert<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        records: Vec<&PyDict>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<&'py PyAny> {
        let rust_records: Result<Vec<RustRecord>, _> = records
            .iter()
            .map(|d| dict_to_record(d))
            .collect();
        let rust_records = rust_records?;
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let results = client
                .batch_insert(&collection, rust_records, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Batch insert failed: {}", e)))?;

            Python::with_gil(|py| {
                let list = PyList::empty(py);
                for record in results {
                    list.append(record_to_dict(py, &record)?)?;
                }
                Ok(list.into())
            })
        })
    }

    /// Batch update multiple documents
    #[pyo3(signature = (collection, updates, bypass_ripple=None))]
    fn batch_update<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        updates: Vec<(&str, &PyDict)>, // Vec of (id, record) pairs
        bypass_ripple: Option<bool>,
    ) -> PyResult<&'py PyAny> {
        let rust_updates: Result<Vec<(String, RustRecord)>, PyErr> = updates
            .iter()
            .map(|(id, d)| Ok((id.to_string(), dict_to_record(d)?)))
            .collect();
        let rust_updates = rust_updates?;
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let results = client
                .batch_update(&collection, rust_updates, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Batch update failed: {}", e)))?;

            Python::with_gil(|py| {
                let list = PyList::empty(py);
                for record in results {
                    list.append(record_to_dict(py, &record)?)?;
                }
                Ok(list.into())
            })
        })
    }

    /// Batch delete multiple documents by IDs
    #[pyo3(signature = (collection, ids, bypass_ripple=None))]
    fn batch_delete<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        ids: Vec<String>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let deleted_count = client
                .batch_delete(&collection, ids, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Batch delete failed: {}", e)))?;

            Python::with_gil(|py| Ok(deleted_count.into_py(py)))
        })
    }

    /// Delete a collection
    fn delete_collection<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            client
                .delete_collection(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete collection failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    /// Create a collection with optional schema
    /// 
    /// Args:
    ///     collection: Collection name
    ///     schema: Optional schema dict with field definitions
    fn create_collection<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        schema: Option<&PyDict>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();
        let schema_config = if let Some(s) = schema {
            let schema_json = dict_to_json(s)?;
            serde_json::from_value(schema_json).map_err(|e| {
                PyValueError::new_err(format!("Failed to parse schema: {}", e))
            })?
        } else {
            ekodb_client::Schema::default()
        };

        future_into_py::<_, PyObject>(py, async move {
            client
                .create_collection(&collection, schema_config)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Create collection failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    /// Get collection schema
    /// 
    /// Args:
    ///     collection: Collection name
    fn get_schema<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let schema = client
                .get_schema(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get schema failed: {}", e)))?;

            Python::with_gil(|py| {
                let schema_json = serde_json::to_value(&schema).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize schema: {}", e))
                })?;
                json_to_pydict(py, &schema_json)
            })
        })
    }

    /// Get collection metadata
    /// 
    /// Args:
    ///     collection: Collection name
    fn get_collection<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let metadata = client
                .get_collection(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get collection failed: {}", e)))?;

            Python::with_gil(|py| {
                let metadata_json = serde_json::to_value(&metadata).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize metadata: {}", e))
                })?;
                json_to_pydict(py, &metadata_json)
            })
        })
    }

    /// Search documents with full-text, vector, or hybrid search
    /// 
    /// Args:
    ///     collection: Collection name
    ///     search_query: Search query dict with query text, fields, weights, etc.
    fn search<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        search_query: &PyDict,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();
        let query_json = dict_to_json(search_query)?;

        future_into_py::<_, PyObject>(py, async move {
            let search_query = serde_json::from_value(query_json).map_err(|e| {
                PyRuntimeError::new_err(format!("Failed to parse search query: {}", e))
            })?;

            let results = client
                .search(&collection, search_query)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Search failed: {}", e)))?;

            Python::with_gil(|py| {
                let results_json = serde_json::to_value(&results).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize results: {}", e))
                })?;
                json_to_pydict(py, &results_json)
            })
        })
    }

    /// Set a key-value pair
    fn kv_set<'py>(
        &self,
        py: Python<'py>,
        key: String,
        value: &PyDict,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();
        // Convert Python dict to JSON value
        let json_str = serde_json::to_string(&dict_to_json(value)?)
            .map_err(|e| PyRuntimeError::new_err(format!("JSON error: {}", e)))?;
        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| PyRuntimeError::new_err(format!("JSON parse error: {}", e)))?;

        future_into_py::<_, PyObject>(py, async move {
            client
                .kv_set(&key, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV set failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    /// Get a value by key
    fn kv_get<'py>(
        &self,
        py: Python<'py>,
        key: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let result = client
                .kv_get(&key)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV get failed: {}", e)))?;

            Python::with_gil(|py| {
                match result {
                    Some(value) => {
                        // Convert JSON value to Python dict
                        let json_str = serde_json::to_string(&value)
                            .map_err(|e| PyRuntimeError::new_err(format!("JSON error: {}", e)))?;
                        let dict = PyDict::new(py);
                        // Parse and set as string for now
                        dict.set_item("value", json_str)?;
                        Ok(dict.into())
                    }
                    None => Ok(py.None()),
                }
            })
        })
    }

    /// Delete a key
    fn kv_delete<'py>(
        &self,
        py: Python<'py>,
        key: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            client
                .kv_delete(&key)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV delete failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    // ========== Chat Methods ==========

    // Note: The chat() method has been removed. Use create_chat_session() and chat_message() instead.

    /// Create a new chat session
    fn create_chat_session<'py>(
        &self,
        py: Python<'py>,
        collections: Vec<(String, Vec<String>)>,
        llm_provider: String,
        llm_model: Option<String>,
        system_prompt: Option<String>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let collection_configs: Vec<CollectionConfig> = collections
                .into_iter()
                .map(|(name, _fields)| CollectionConfig {
                    collection_name: name,
                    fields: vec![],
                    search_options: None,
                })
                .collect();

            let request = CreateChatSessionRequest {
                collections: collection_configs,
                llm_provider,
                llm_model,
                system_prompt,
                bypass_ripple: None,
                parent_id: None,
                branch_point_idx: None,
                max_context_messages: None,
            };

            let result = client
                .create_chat_session(request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Create session failed: {}", e)))?;

            Python::with_gil(|py| chat_response_to_dict(py, &result))
        })
    }

    /// Send a message in an existing chat session
    fn chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let request = ChatMessageRequest {
                message,
                bypass_ripple: None,
                force_summarize: None,
            };

            let result = client
                .chat_message(&chat_id, request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Chat message failed: {}", e)))?;

            Python::with_gil(|py| chat_response_to_dict(py, &result))
        })
    }

    /// List all chat sessions
    fn list_chat_sessions<'py>(
        &self,
        py: Python<'py>,
        limit: Option<usize>,
        skip: Option<usize>,
        sort: Option<String>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let query = ListSessionsQuery { limit, skip, sort };

            let result = client
                .list_chat_sessions(query)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("List sessions failed: {}", e)))?;

            Python::with_gil(|py| list_sessions_response_to_dict(py, &result))
        })
    }

    /// Get messages from a chat session
    fn get_chat_session_messages<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        limit: Option<usize>,
        skip: Option<usize>,
        sort: Option<String>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let query = GetMessagesQuery { limit, skip, sort };

            let result = client
                .get_chat_session_messages(&chat_id, query)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get messages failed: {}", e)))?;

            Python::with_gil(|py| get_messages_response_to_dict(py, &result))
        })
    }

    /// Get a chat session by ID
    fn get_chat_session<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let result = client
                .get_chat_session(&chat_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get session failed: {}", e)))?;

            Python::with_gil(|py| {
                let dict = PyDict::new(py);
                dict.set_item("session", record_to_dict(py, &result.session)?)?;
                dict.set_item("message_count", result.message_count)?;
                Ok(dict.into())
            })
        })
    }

    /// Update a chat session
    fn update_chat_session<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        system_prompt: Option<String>,
        llm_model: Option<String>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let request = UpdateSessionRequest {
                system_prompt,
                llm_model,
                collections: None,
                title: None,
            };

            let result = client
                .update_chat_session(&chat_id, request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Update session failed: {}", e)))?;

            Python::with_gil(|py| {
                let dict = PyDict::new(py);
                dict.set_item("session", record_to_dict(py, &result.session)?)?;
                dict.set_item("message_count", result.message_count)?;
                Ok(dict.into())
            })
        })
    }

    /// Branch a chat session
    fn branch_chat_session<'py>(
        &self,
        py: Python<'py>,
        parent_id: String,
        branch_point_idx: usize,
        collections: Vec<(String, Vec<String>)>,
        llm_provider: String,
        llm_model: Option<String>,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let collection_configs: Vec<CollectionConfig> = collections
                .into_iter()
                .map(|(name, _fields)| CollectionConfig {
                    collection_name: name,
                    fields: vec![],
                    search_options: None,
                })
                .collect();

            let request = CreateChatSessionRequest {
                collections: collection_configs,
                llm_provider,
                llm_model,
                system_prompt: None,
                bypass_ripple: None,
                parent_id: Some(parent_id),
                branch_point_idx: Some(branch_point_idx),
                max_context_messages: None,
            };

            let result = client
                .branch_chat_session(request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Branch session failed: {}", e)))?;

            Python::with_gil(|py| chat_response_to_dict(py, &result))
        })
    }

    /// Delete a chat session
    fn delete_chat_session<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            client
                .delete_chat_session(&chat_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete session failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    /// Regenerate an AI response message
    fn regenerate_chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let result = client
                .regenerate_chat_message(&chat_id, &message_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Regenerate message failed: {}", e)))?;

            Python::with_gil(|py| chat_response_to_dict(py, &result))
        })
    }

    /// Update a specific message
    fn update_chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
        content: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let request = ekodb_client::UpdateMessageRequest { content };
            
            client
                .update_chat_message(&chat_id, &message_id, request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Update message failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    /// Delete a specific message
    fn delete_chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            client
                .delete_chat_message(&chat_id, &message_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete message failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    /// Toggle the "forgotten" status of a message
    fn toggle_forgotten_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
        forgotten: bool,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let request = ekodb_client::ToggleForgottenRequest { forgotten };
            
            client
                .toggle_forgotten_message(&chat_id, &message_id, request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Toggle forgotten failed: {}", e)))?;

            Python::with_gil(|py| Ok(py.None()))
        })
    }

    /// Merge multiple chat sessions into one
    fn merge_chat_sessions<'py>(
        &self,
        py: Python<'py>,
        source_chat_ids: Vec<String>,
        target_chat_id: String,
        merge_strategy: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            use ekodb_client::MergeStrategy;
            
            let strategy = match merge_strategy.as_str() {
                "Chronological" => MergeStrategy::Chronological,
                "Summarized" => MergeStrategy::Summarized,
                "LatestOnly" => MergeStrategy::LatestOnly,
                _ => return Err(PyRuntimeError::new_err(format!("Invalid merge strategy: {}. Valid options: Chronological, Summarized, LatestOnly", merge_strategy))),
            };

            let request = ekodb_client::MergeSessionsRequest {
                source_chat_ids: source_chat_ids,
                target_chat_id: target_chat_id,
                merge_strategy: strategy,
            };

            let result = client
                .merge_chat_sessions(request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Merge sessions failed: {}", e)))?;

            Python::with_gil(|py| {
                let dict = PyDict::new(py);
                dict.set_item("session", record_to_dict(py, &result.session)?)?;
                dict.set_item("message_count", result.message_count)?;
                Ok(dict.into())
            })
        })
    }

    /// Create a WebSocket connection
    fn websocket<'py>(
        &self,
        py: Python<'py>,
        ws_url: String,
    ) -> PyResult<&'py PyAny> {
        let client = self.inner.clone();

        future_into_py::<_, PyObject>(py, async move {
            let ws_client = client
                .websocket(&ws_url)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("WebSocket connection failed: {}", e)))?;

            Python::with_gil(|py| {
                // Return a WebSocketClient wrapper
                let ws_wrapper = WebSocketClient {
                    inner: Some(ws_client),
                };
                Ok(Py::new(py, ws_wrapper)?.into_py(py))
            })
        })
    }
}

/// Python wrapper for WebSocket Client
#[pyclass]
struct WebSocketClient {
    inner: Option<RustWebSocketClient>,
}

#[pymethods]
impl WebSocketClient {
    /// Find all records in a collection via WebSocket
    fn find_all<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<&'py PyAny> {
        // Clone the WebSocket client before moving into async block
        let ws_client = match &self.inner {
            Some(client) => client.clone(),
            None => return Err(PyRuntimeError::new_err("WebSocket client not initialized")),
        };

        future_into_py::<_, PyObject>(py, async move {
            let records = ws_client
                .find_all(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("WebSocket find_all failed: {}", e)))?;

            Python::with_gil(|py| {
                let list = PyList::empty(py);
                for record in records {
                    list.append(record_to_dict(py, &record)?)?;
                }
                Ok(list.into())
            })
        })
    }
}

/// Convert ChatResponse to Python dict
fn chat_response_to_dict(py: Python, response: &ChatResponse) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("chat_id", &response.chat_id)?;
    dict.set_item("message_id", &response.message_id)?;
    
    let responses_list = PyList::empty(py);
    for r in &response.responses {
        responses_list.append(r)?;
    }
    dict.set_item("responses", responses_list)?;
    
    dict.set_item("execution_time_ms", response.execution_time_ms)?;
    
    if let Some(ref token_usage) = response.token_usage {
        let token_dict = PyDict::new(py);
        token_dict.set_item("prompt_tokens", token_usage.prompt_tokens)?;
        token_dict.set_item("completion_tokens", token_usage.completion_tokens)?;
        token_dict.set_item("total_tokens", token_usage.total_tokens)?;
        dict.set_item("token_usage", token_dict)?;
    }
    
    Ok(dict.into())
}

/// Convert ListSessionsResponse to Python dict
fn list_sessions_response_to_dict(py: Python, response: &ListSessionsResponse) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    
    let sessions_list = PyList::empty(py);
    for session in &response.sessions {
        let session_dict = PyDict::new(py);
        session_dict.set_item("chat_id", &session.chat_id)?;
        session_dict.set_item("created_at", &session.created_at)?;
        session_dict.set_item("updated_at", &session.updated_at)?;
        session_dict.set_item("llm_provider", &session.llm_provider)?;
        session_dict.set_item("llm_model", &session.llm_model)?;
        session_dict.set_item("message_count", session.message_count)?;
        if let Some(ref title) = session.title {
            session_dict.set_item("title", title)?;
        }
        if let Some(ref system_prompt) = session.system_prompt {
            session_dict.set_item("system_prompt", system_prompt)?;
        }
        sessions_list.append(session_dict)?;
    }
    
    dict.set_item("sessions", sessions_list)?;
    dict.set_item("total", response.total)?;
    dict.set_item("returned", response.returned)?;
    
    Ok(dict.into())
}

/// Convert GetMessagesResponse to Python dict
fn get_messages_response_to_dict(py: Python, response: &GetMessagesResponse) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    
    let messages_list = PyList::empty(py);
    for message in &response.messages {
        messages_list.append(record_to_dict(py, message)?)?;
    }
    
    dict.set_item("messages", messages_list)?;
    dict.set_item("total", response.total)?;
    dict.set_item("skip", response.skip)?;
    dict.set_item("returned", response.returned)?;
    if let Some(limit) = response.limit {
        dict.set_item("limit", limit)?;
    }
    
    Ok(dict.into())
}

/// Convert Python value to JSON recursively
fn py_to_json(value: &PyAny) -> PyResult<serde_json::Value> {
    use serde_json::json;
    
    // Check bool BEFORE int (Python bool is subclass of int)
    if let Ok(b) = value.extract::<bool>() {
        Ok(json!(b))
    } else if let Ok(s) = value.extract::<String>() {
        Ok(json!(s))
    } else if let Ok(i) = value.extract::<i64>() {
        Ok(json!(i))
    } else if let Ok(f) = value.extract::<f64>() {
        Ok(json!(f))
    } else if value.is_none() {
        Ok(serde_json::Value::Null)
    } else if let Ok(list) = value.downcast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(py_to_json(item)?);
        }
        Ok(serde_json::Value::Array(arr))
    } else if let Ok(dict) = value.downcast::<PyDict>() {
        dict_to_json(dict)
    } else {
        Ok(serde_json::Value::Null)
    }
}

/// Convert Python dict to JSON value
fn dict_to_json(dict: &PyDict) -> PyResult<serde_json::Value> {
    let mut map = serde_json::Map::new();
    
    for (key, value) in dict.iter() {
        let key_str: String = key.extract()?;
        map.insert(key_str, py_to_json(value)?);
    }
    
    Ok(serde_json::Value::Object(map))
}

/// Convert Python value to FieldType recursively
fn py_to_field_type(value: &PyAny) -> PyResult<FieldType> {
    // Check bool BEFORE int (Python bool is subclass of int)
    if let Ok(b) = value.extract::<bool>() {
        Ok(FieldType::Boolean(b))
    } else if let Ok(s) = value.extract::<String>() {
        Ok(FieldType::String(s))
    } else if let Ok(i) = value.extract::<i64>() {
        Ok(FieldType::Integer(i))
    } else if let Ok(f) = value.extract::<f64>() {
        Ok(FieldType::Float(f))
    } else if value.is_none() {
        Ok(FieldType::Null)
    } else if let Ok(list) = value.downcast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(py_to_field_type(item)?);
        }
        Ok(FieldType::Array(arr))
    } else if let Ok(dict) = value.downcast::<PyDict>() {
        let mut map = std::collections::HashMap::new();
        for (k, v) in dict.iter() {
            let key_str: String = k.extract()?;
            map.insert(key_str, py_to_field_type(v)?);
        }
        Ok(FieldType::Object(map))
    } else {
        Err(PyValueError::new_err(format!(
            "Unsupported Python type: {:?}",
            value.get_type().name()
        )))
    }
}

/// Convert Python dict to Rust Record
fn dict_to_record(dict: &PyDict) -> PyResult<RustRecord> {
    let mut record = RustRecord::new();

    for (key, value) in dict.iter() {
        let key_str: String = key.extract()?;
        let field_value = py_to_field_type(value)?;
        record.fields.insert(key_str, field_value);
    }

    Ok(record)
}

/// Convert FieldType to Python object recursively
fn field_type_to_py(py: Python, value: &FieldType) -> PyResult<PyObject> {
    match value {
        FieldType::String(s) => Ok(s.to_object(py)),
        FieldType::Integer(i) => Ok(i.to_object(py)),
        FieldType::Float(f) => Ok(f.to_object(py)),
        FieldType::Boolean(b) => Ok(b.to_object(py)),
        FieldType::Null => Ok(py.None()),
        FieldType::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(field_type_to_py(py, item)?)?;
            }
            Ok(list.to_object(py))
        }
        FieldType::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, field_type_to_py(py, v)?)?;
            }
            Ok(dict.to_object(py))
        }
        // For all other complex types, convert to string representation
        _ => {
            let value_str = format!("{:?}", value);
            Ok(value_str.to_object(py))
        }
    }
}

/// Convert Rust Record to Python dict
fn record_to_dict(py: Python, record: &RustRecord) -> PyResult<PyObject> {
    let dict = PyDict::new(py);

    for (key, value) in record.fields.iter() {
        dict.set_item(key, field_type_to_py(py, value)?)?;
    }

    Ok(dict.into())
}

/// Convert JSON value to Python dict
fn json_to_pydict(py: Python, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.to_object(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(s.to_object(py)),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_to_pydict(py, item)?)?;
            }
            Ok(list.to_object(py))
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, json_to_pydict(py, v)?)?;
            }
            Ok(dict.to_object(py))
        }
    }
}

/// ekoDB Python module
#[pymodule]
fn _ekodb_client(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    m.add_class::<WebSocketClient>()?;
    m.add_class::<RateLimitInfo>()?;
    m.add_class::<SerializationFormat>()?;
    m.add("RateLimitError", py.get_type::<RateLimitError>())?;
    Ok(())
}
