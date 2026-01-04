//! Python bindings for ekoDB client library
//!
//! This module provides Python bindings for the ekoDB Rust client library using PyO3.

use ekodb_client::{
    ChatMessageRequest, ChatResponse, Client as RustClient, CollectionConfig,
    CreateChatSessionRequest, FieldType, GetMessagesQuery, GetMessagesResponse,
    ListSessionsQuery, ListSessionsResponse, Query as RustQuery,
    RateLimitInfo as RustRateLimitInfo, Record as RustRecord,
    Script as RustScript, SerializationFormat as RustSerializationFormat,
    UpdateSessionRequest, WebSocketClient as RustWebSocketClient,
};
use serde_json;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};
use pyo3_async_runtimes::tokio::future_into_py;

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
    #[pyo3(signature = (collection, record, ttl=None, bypass_ripple=None))]
    fn insert<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        record: &Bound<'py, PyDict>,
        ttl: Option<String>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut rust_record = dict_to_record(record)?;
        
        // Add TTL if provided
        if let Some(ttl_duration) = ttl {
            rust_record = rust_record.with_ttl(ttl_duration);
        }
        
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let result = client
                .insert(&collection, rust_record, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Insert failed: {}", e)))?;

            Ok(Python::attach(|py| record_to_dict(py, &result))?)
        })
    }

    /// Find a document by ID
    #[pyo3(signature = (collection, id, bypass_ripple=None))]
    fn find_by_id<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let result = client
                .find_by_id(&collection, &id, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Find failed: {}", e)))?;

            Python::attach(|py| record_to_dict(py, &result))
        })
    }

    /// Find documents matching a query
    /// 
    /// Args:
    ///     collection: Collection name
    ///     query: Optional query dict with filters, joins, etc.
    ///     limit: Optional limit (deprecated, use query dict instead)
    #[pyo3(signature = (collection, query=None, limit=None, bypass_ripple=None))]
    fn find<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        query: Option<&Bound<'py, PyDict>>,
        limit: Option<usize>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let query_json = if let Some(q) = query {
            Some(dict_to_json(q)?)
        } else {
            None
        };

        future_into_py::<_, Py<PyAny>>(py, async move {
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

            Python::attach(|py| {
                let list = PyList::empty(py);
                for record in results {
                    list.append(record_to_dict(py, &record)?)?;
                }
                Ok(list.into())
            })
        })
    }

    /// Update a document
    #[pyo3(signature = (collection, id, updates, bypass_ripple=None))]
    fn update<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        updates: &Bound<'py, PyDict>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rust_updates = dict_to_record(updates)?;
        let client = self.inner.clone();

        future_into_py(py, async move {
            let result = client
                .update(&collection, &id, rust_updates, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Update failed: {}", e)))?;

            Python::attach(|py| record_to_dict(py, &result))
        })
    }

    /// Delete a document
    #[pyo3(signature = (collection, id, bypass_ripple=None))]
    fn delete<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            client
                .delete(&collection, &id, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// List all collections
    fn list_collections<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let collections = client
                .list_collections()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("List collections failed: {}", e)))?;

            Python::attach(|py| {
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
        records: Vec<Bound<'py, PyDict>>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rust_records: Result<Vec<RustRecord>, _> = records
            .iter()
            .map(|d| dict_to_record(d))
            .collect();
        let rust_records = rust_records?;
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let results = client
                .batch_insert(&collection, rust_records, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Batch insert failed: {}", e)))?;

            Python::attach(|py| {
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
        updates: Vec<(String, Bound<'py, PyDict>)>, // Vec of (id, record) pairs
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rust_updates: Result<Vec<(String, RustRecord)>, PyErr> = updates
            .iter()
            .map(|(id, d)| Ok((id.clone(), dict_to_record(d)?)))
            .collect();
        let rust_updates = rust_updates?;
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let results = client
                .batch_update(&collection, rust_updates, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Batch update failed: {}", e)))?;

            Python::attach(|py| {
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
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let deleted_count = client
                .batch_delete(&collection, ids, bypass_ripple)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Batch delete failed: {}", e)))?;

            Python::attach(|py| Ok(PyInt::new(py, deleted_count).into()))
        })
    }

    /// Delete a collection
    fn delete_collection<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            client
                .delete_collection(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete collection failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Restore a deleted record from trash
    /// 
    /// Args:
    ///     collection: Collection name
    ///     id: Record ID to restore
    /// 
    /// Returns:
    ///     True if the record was restored, False if not found
    fn restore_deleted<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let restored = client
                .restore_deleted(&collection, &id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Restore deleted failed: {}", e)))?;

            Python::attach(|py| {
                Ok(PyBool::new(py, restored).as_borrowed().to_owned().into())
            })
        })
    }

    /// Restore all deleted records in a collection from trash
    /// 
    /// Args:
    ///     collection: Collection name
    /// 
    /// Returns:
    ///     Number of records restored
    fn restore_collection<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let count = client
                .restore_collection(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Restore collection failed: {}", e)))?;

            Python::attach(|py| Ok(PyInt::new(py, count as i64).into()))
        })
    }

    /// Health check - verify the ekoDB server is responding
    /// 
    /// Returns:
    ///     True if the server is healthy, False otherwise
    fn health_check<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let result = client
                .health_check()
                .await;

            Python::attach(|py| {
                Ok(PyBool::new(py, result.is_ok()).as_borrowed().to_owned().into())
            })
        })
    }

    /// Create a collection with optional schema
    /// 
    /// Args:
    ///     collection: Collection name
    ///     schema: Optional schema dict with field definitions
    #[pyo3(signature = (collection, schema=None))]
    fn create_collection<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        schema: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let schema_config = if let Some(s) = schema {
            let schema_json = dict_to_json(s)?;
            serde_json::from_value(schema_json).map_err(|e| {
                PyValueError::new_err(format!("Failed to parse schema: {}", e))
            })?
        } else {
            ekodb_client::Schema::default()
        };

        future_into_py(py, async move {
            client
                .create_collection(&collection, schema_config)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Create collection failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
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
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let schema = client
                .get_schema(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get schema failed: {}", e)))?;

            Python::attach(|py| {
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
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let metadata = client
                .get_collection(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get collection failed: {}", e)))?;

            Python::attach(|py| {
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
        search_query: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let query_json = dict_to_json(search_query)?;

        future_into_py(py, async move {
            let search_query = serde_json::from_value(query_json).map_err(|e| {
                PyRuntimeError::new_err(format!("Failed to parse search query: {}", e))
            })?;

            let results = client
                .search(&collection, search_query)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Search failed: {}", e)))?;

            Python::attach(|py| {
                let results_json = serde_json::to_value(&results).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize results: {}", e))
                })?;
                json_to_pydict(py, &results_json)
            })
        })
    }

    /// Text-only search (full-text search helper)
    ///
    /// Args:
    ///     collection: Collection name
    ///     query_text: Search query text
    ///     limit: Maximum number of results
    fn text_search<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        query_text: String,
        limit: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let results = client
                .text_search(&collection, &query_text, limit)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Text search failed: {}", e)))?;

            Python::attach(|py| {
                let results_json = serde_json::to_value(&results).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize results: {}", e))
                })?;
                json_to_pydict(py, &results_json)
            })
        })
    }

    /// Hybrid search (combines text + vector search)
    ///
    /// Args:
    ///     collection: Collection name
    ///     query_text: Search query text
    ///     query_vector: List of floats representing the query embedding
    ///     limit: Maximum number of results
    fn hybrid_search<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        query_text: String,
        query_vector: Vec<f64>,
        limit: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let results = client
                .hybrid_search(&collection, &query_text, query_vector, limit)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Hybrid search failed: {}", e)))?;

            Python::attach(|py| {
                let results_json = serde_json::to_value(&results).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize results: {}", e))
                })?;
                json_to_pydict(py, &results_json)
            })
        })
    }

    /// Find all records in a collection (with limit)
    ///
    /// Args:
    ///     collection: Collection name
    ///     limit: Maximum number of records to return
    fn find_all<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        limit: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let results = client
                .find_all(&collection, limit)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Find all failed: {}", e)))?;

            Python::attach(|py| {
                let results_json = serde_json::to_value(&results).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize results: {}", e))
                })?;
                json_to_pydict(py, &results_json)
            })
        })
    }

    /// Generate embeddings for text using AI
    ///
    /// Args:
    ///     text: The text to generate embeddings for
    ///     model: The embedding model to use (e.g., "text-embedding-3-small")
    fn embed<'py>(
        &self,
        py: Python<'py>,
        text: String,
        model: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let embedding = client
                .embed(&text, &model)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Embed failed: {}", e)))?;

            Python::attach(|py| {
                let list = PyList::new(py, &embedding)?;
                Ok(list.into())
            })
        })
    }

    /// Set a key-value pair
    fn kv_set<'py>(
        &self,
        py: Python<'py>,
        key: String,
        value: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        // Convert Python dict to JSON value
        let json_str = serde_json::to_string(&dict_to_json(value)?)
            .map_err(|e| PyRuntimeError::new_err(format!("JSON error: {}", e)))?;
        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| PyRuntimeError::new_err(format!("JSON parse error: {}", e)))?;

        future_into_py(py, async move {
            client
                .kv_set(&key, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV set failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Get a value by key
    fn kv_get<'py>(
        &self,
        py: Python<'py>,
        key: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let result = client
                .kv_get(&key)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV get failed: {}", e)))?;

            Python::attach(|py| {
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
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            client
                .kv_delete(&key)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV delete failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Check if a key exists
    fn kv_exists<'py>(
        &self,
        py: Python<'py>,
        key: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let exists = client
                .kv_exists(&key)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV exists failed: {}", e)))?;

            Python::attach(|py| {
                Ok(PyBool::new(py, exists).as_borrowed().to_owned().into())
            })
        })
    }

    /// Query/find KV entries with pattern matching
    #[pyo3(signature = (pattern=None, include_expired=false))]
    fn kv_find<'py>(
        &self,
        py: Python<'py>,
        pattern: Option<String>,
        include_expired: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let result = client
                .kv_find(pattern.as_deref(), include_expired)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV find failed: {}", e)))?;

            Python::attach(|py| {
                let list = PyList::empty(py);
                for item in result {
                    let py_value = json_to_pydict(py, &item)?;
                    list.append(py_value)?;
                }
                Ok(list.into())
            })
        })
    }

    /// Query KV store with pattern (alias for kv_find)
    #[pyo3(signature = (pattern=None, include_expired=false))]
    fn kv_query<'py>(
        &self,
        py: Python<'py>,
        pattern: Option<String>,
        include_expired: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.kv_find(py, pattern, include_expired)
    }

    // ========== Transaction Methods ==========

    /// Begin a new transaction
    #[pyo3(signature = (isolation_level="ReadCommitted"))]
    fn begin_transaction<'py>(
        &self,
        py: Python<'py>,
        isolation_level: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let isolation_level = isolation_level.to_string();

        future_into_py(py, async move {
            let result = client
                .begin_transaction(&isolation_level)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Begin transaction failed: {}", e)))?;

            Ok(result)
        })
    }

    /// Get transaction status
    fn get_transaction_status<'py>(
        &self,
        py: Python<'py>,
        transaction_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let result = client
                .get_transaction_status(&transaction_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get transaction status failed: {}", e)))?;

            Python::attach(|py| {
                let dict = PyDict::new(py);
                if let Some(state) = result.get("state").and_then(|v| v.as_str()) {
                    dict.set_item("state", state)?;
                }
                if let Some(count) = result.get("operations_count").and_then(|v| v.as_i64()) {
                    dict.set_item("operations_count", count)?;
                }
                Ok(dict.into_any().unbind())
            })
        })
    }

    /// Commit a transaction
    fn commit_transaction<'py>(
        &self,
        py: Python<'py>,
        transaction_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            client
                .commit_transaction(&transaction_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Commit transaction failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Rollback a transaction
    fn rollback_transaction<'py>(
        &self,
        py: Python<'py>,
        transaction_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            client
                .rollback_transaction(&transaction_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Rollback transaction failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    // ========== Chat Methods ==========

    // Note: The chat() method has been removed. Use create_chat_session() and chat_message() instead.

    /// Create a new chat session
    #[pyo3(signature = (collections, llm_provider, llm_model=None, system_prompt=None))]
    fn create_chat_session<'py>(
        &self,
        py: Python<'py>,
        collections: Vec<(String, Vec<String>)>,
        llm_provider: String,
        llm_model: Option<String>,
        system_prompt: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
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

            Python::attach(|py| chat_response_to_dict(py, &result))
        })
    }

    /// Send a message in an existing chat session
    fn chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let request = ChatMessageRequest {
                message,
                bypass_ripple: None,
                force_summarize: None,
            };

            let result = client
                .chat_message(&chat_id, request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Chat message failed: {}", e)))?;

            Python::attach(|py| chat_response_to_dict(py, &result))
        })
    }

    /// List all chat sessions
    #[pyo3(signature = (limit=None, skip=None, sort=None))]
    fn list_chat_sessions<'py>(
        &self,
        py: Python<'py>,
        limit: Option<usize>,
        skip: Option<usize>,
        sort: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let query = ListSessionsQuery { limit, skip, sort };

            let result = client
                .list_chat_sessions(query)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("List sessions failed: {}", e)))?;

            Python::attach(|py| list_sessions_response_to_dict(py, &result))
        })
    }

    /// Get messages from a chat session
    #[pyo3(signature = (chat_id, limit=None, skip=None, sort=None))]
    fn get_chat_session_messages<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        limit: Option<usize>,
        skip: Option<usize>,
        sort: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let query = GetMessagesQuery { limit, skip, sort };

            let result = client
                .get_chat_session_messages(&chat_id, query)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get messages failed: {}", e)))?;

            Python::attach(|py| get_messages_response_to_dict(py, &result))
        })
    }

    /// Get a chat session by ID
    fn get_chat_session<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let result = client
                .get_chat_session(&chat_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get session failed: {}", e)))?;

            Python::attach(|py| {
                let dict = PyDict::new(py);
                dict.set_item("session", record_to_dict(py, &result.session)?)?;
                dict.set_item("message_count", result.message_count)?;
                Ok(dict.into())
            })
        })
    }

    /// Update a chat session
    #[pyo3(signature = (chat_id, system_prompt=None, llm_model=None))]
    fn update_chat_session<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        system_prompt: Option<String>,
        llm_model: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
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

            Python::attach(|py| {
                let dict = PyDict::new(py);
                dict.set_item("session", record_to_dict(py, &result.session)?)?;
                dict.set_item("message_count", result.message_count)?;
                Ok(dict.into())
            })
        })
    }

    /// Branch a chat session
    #[pyo3(signature = (parent_id, branch_point_idx, collections, llm_provider, llm_model=None))]
    fn branch_chat_session<'py>(
        &self,
        py: Python<'py>,
        parent_id: String,
        branch_point_idx: usize,
        collections: Vec<(String, Vec<String>)>,
        llm_provider: String,
        llm_model: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
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

            Python::attach(|py| chat_response_to_dict(py, &result))
        })
    }

    /// Delete a chat session
    fn delete_chat_session<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            client
                .delete_chat_session(&chat_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete session failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Regenerate an AI response message
    fn regenerate_chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let result = client
                .regenerate_chat_message(&chat_id, &message_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Regenerate message failed: {}", e)))?;

            Python::attach(|py| chat_response_to_dict(py, &result))
        })
    }

    /// Update a specific message
    fn update_chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
        content: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let request = ekodb_client::UpdateMessageRequest { content };
            
            client
                .update_chat_message(&chat_id, &message_id, request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Update message failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Delete a specific message
    fn delete_chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            client
                .delete_chat_message(&chat_id, &message_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete message failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Toggle the "forgotten" status of a message
    fn toggle_forgotten_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
        forgotten: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let request = ekodb_client::ToggleForgottenRequest { forgotten };
            
            client
                .toggle_forgotten_message(&chat_id, &message_id, request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Toggle forgotten failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Merge multiple chat sessions into one
    fn merge_chat_sessions<'py>(
        &self,
        py: Python<'py>,
        source_chat_ids: Vec<String>,
        target_chat_id: String,
        merge_strategy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
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

            Python::attach(|py| {
                let dict = PyDict::new(py);
                dict.set_item("session", record_to_dict(py, &result.session)?)?;
                dict.set_item("message_count", result.message_count)?;
                Ok(dict.into())
            })
        })
    }

    // ========================================================================
    // SCRIPTS API
    // ========================================================================

    /// Save a new script definition
    ///
    /// Args:
    ///     script: Script definition as a dict
    ///
    /// Returns:
    ///     Script ID string
    fn save_script<'py>(
        &self,
        py: Python<'py>,
        script: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let script_json = serde_json::to_string(&pydict_to_json(py, script)?)
            .map_err(|e| PyValueError::new_err(format!("Invalid script definition: {}", e)))?;
        
        future_into_py::<_, Py<PyAny>>(py, async move {
            let script: RustScript = serde_json::from_str(&script_json)
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to parse script: {}", e)))?;
            
            let id = client
                .save_script(script)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Save script failed: {}", e)))?;
            
            Python::attach(|py| Ok(PyString::new(py, &id).into()))
        })
    }

    /// Get a script by ID
    ///
    /// Args:
    ///     id: Script ID
    ///
    /// Returns:
    ///     Script definition as a dict
    fn get_script<'py>(
        &self,
        py: Python<'py>,
        id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        
        future_into_py(py, async move {
            let script = client
                .get_script(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get script failed: {}", e)))?;
            
            let json = serde_json::to_value(&script)
                .map_err(|e| PyRuntimeError::new_err(format!("Serialization failed: {}", e)))?;
            
            Python::attach(|py| json_to_pydict(py, &json))
        })
    }

    /// List all scripts, optionally filtered by tags
    ///
    /// Args:
    ///     tags: Optional list of tags to filter by
    ///
    /// Returns:
    ///     List of script definitions
    fn list_scripts<'py>(
        &self,
        py: Python<'py>,
        tags: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        
        future_into_py(py, async move {
            let scripts = client
                .list_scripts(tags)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("List scripts failed: {}", e)))?;
            
            let json = serde_json::to_value(&scripts)
                .map_err(|e| PyRuntimeError::new_err(format!("Serialization failed: {}", e)))?;
            
            Python::attach(|py| json_to_pydict(py, &json))
        })
    }

    /// Update an existing script
    ///
    /// Args:
    ///     id: Script ID
    ///     script: Updated script definition as a dict
    fn update_script<'py>(
        &self,
        py: Python<'py>,
        script_id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let script_json = serde_json::to_string(&pydict_to_json(py, data)?)
            .map_err(|e| PyValueError::new_err(format!("Invalid script definition: {}", e)))?;
        
        future_into_py(py, async move {
            let script: RustScript = serde_json::from_str(&script_json)
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to parse script: {}", e)))?;
            
            client
                .update_script(&script_id, script)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Update script failed: {}", e)))?;
            
            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Delete a script by ID
    ///
    /// Args:
    ///     id: Script ID
    fn delete_script<'py>(
        &self,
        py: Python<'py>,
        id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        
        future_into_py(py, async move {
            client
                .delete_script(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete script failed: {}", e)))?;
            
            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Call a saved script by ID or label
    ///
    /// Args:
    ///     script_id_or_label: Script ID or label name
    ///     params: Optional parameters as a dict
    ///
    /// Returns:
    ///     Script execution result with records and metadata
    #[pyo3(signature = (script_id_or_label, params=None))]
    fn call_script<'py>(
        &self,
        py: Python<'py>,
        script_id_or_label: String,
        params: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let params_map = if let Some(p) = params {
            Some(pydict_to_fieldtype_map(py, p)?)
        } else {
            None
        };
        
        future_into_py::<_, Py<PyAny>>(py, async move {
            let result = client
                .call_script(&script_id_or_label, params_map)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Call script failed: {}", e)))?;
            
            Python::attach(|py| {
                let dict = PyDict::new(py);
                
                // Convert records
                let records_list = PyList::empty(py);
                for record in &result.records {
                    // Convert Record to serde_json::Value first
                    let record_value = serde_json::to_value(record).unwrap();
                    records_list.append(json_to_pydict(py, &record_value)?)?;
                }
                dict.set_item("records", records_list)?;
                
                // Convert stats (not metadata)
                let stats = json_to_pydict(py, &serde_json::to_value(&result.stats).unwrap())?;
                dict.set_item("stats", stats)?;
                
                Ok(dict.into())
            })
        })
    }

    /// Create a WebSocket connection
    fn websocket<'py>(
        &self,
        py: Python<'py>,
        ws_url: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let ws_client = client
                .websocket(&ws_url)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("WebSocket connection failed: {}", e)))?;

            Python::attach(|py| {
                // Return a WebSocketClient wrapper
                let ws_wrapper = WebSocketClient {
                    inner: Some(ws_client),
                };
                Ok(Py::new(py, ws_wrapper)?.into())
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
    ) -> PyResult<Bound<'py, PyAny>> {
        // Clone the WebSocket client before moving into async block
        let ws_client = match &self.inner {
            Some(client) => client.clone(),
            None => return Err(PyRuntimeError::new_err("WebSocket client not initialized")),
        };

        future_into_py::<_, Py<PyAny>>(py, async move {
            let records = ws_client
                .find_all(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("WebSocket find_all failed: {}", e)))?;

            Python::attach(|py| {
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
fn chat_response_to_dict(py: Python, response: &ChatResponse) -> PyResult<Py<PyAny>> {
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
fn list_sessions_response_to_dict(py: Python, response: &ListSessionsResponse) -> PyResult<Py<PyAny>> {
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
fn get_messages_response_to_dict(py: Python, response: &GetMessagesResponse) -> PyResult<Py<PyAny>> {
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
fn py_to_json(value: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
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
    } else if let Ok(list) = value.cast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(py_to_json(&item)?);
        }
        Ok(serde_json::Value::Array(arr))
    } else if let Ok(dict) = value.cast::<PyDict>() {
        dict_to_json(dict)
    } else {
        Ok(serde_json::Value::Null)
    }
}

/// Convert Python dict to JSON value
fn dict_to_json(dict: &Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    let mut map = serde_json::Map::new();
    
    for (key, value) in dict.iter() {
        let key_str: String = key.extract()?;
        map.insert(key_str, py_to_json(&value)?);
    }
    
    Ok(serde_json::Value::Object(map))
}

/// Convert Python value to FieldType recursively
fn py_to_field_type(value: &Bound<'_, PyAny>) -> PyResult<FieldType> {
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
    } else if let Ok(list) = value.cast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(py_to_field_type(&item)?);
        }
        Ok(FieldType::Array(arr))
    } else if let Ok(dict) = value.cast::<PyDict>() {
        let mut map = std::collections::HashMap::new();
        for (k, v) in dict.iter() {
            let key_str: String = k.extract()?;
            map.insert(key_str, py_to_field_type(&v)?);
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
fn dict_to_record(dict: &Bound<'_, PyDict>) -> PyResult<RustRecord> {
    let mut record = RustRecord::default();
    
    for (key, value) in dict.iter() {
        let key_str: String = key.extract()?;
        let field_value = py_to_field_type(&value)?;
        record.fields.insert(key_str, field_value);
    }

    Ok(record)
}

/// Convert FieldType to Python object recursively
fn field_type_to_py(py: Python, value: &FieldType) -> PyResult<Py<PyAny>> {
    match value {
        FieldType::String(s) => Ok(PyString::new(py, s).into()),
        FieldType::Integer(i) => Ok(PyInt::new(py, *i).into()),
        FieldType::Float(f) => Ok(PyFloat::new(py, *f).into()),
        FieldType::Boolean(b) => Ok(PyBool::new(py, *b).as_borrowed().to_owned().into()),
        FieldType::Null => Ok(py.None()),
        FieldType::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(field_type_to_py(py, item)?)?;
            }
            Ok(list.into())
        }
        FieldType::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, field_type_to_py(py, v)?)?;
            }
            Ok(dict.into())
        }
        // For all other complex types, convert to string representation
        _ => {
            let value_str = format!("{:?}", value);
            Ok(PyString::new(py, &value_str).into())
        }
    }
}

/// Convert Rust Record to Python dict
fn record_to_dict(py: Python, record: &RustRecord) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);

    for (key, value) in record.fields.iter() {
        dict.set_item(key, field_type_to_py(py, value)?)?;
    }

    Ok(dict.into())
}

/// Convert JSON value to Python dict
fn json_to_pydict(py: Python, value: &serde_json::Value) -> PyResult<Py<PyAny>> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(PyBool::new(py, *b).as_borrowed().to_owned().into()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(PyInt::new(py, i).into())
            } else if let Some(f) = n.as_f64() {
                Ok(PyFloat::new(py, f).into())
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(PyString::new(py, s).into()),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_to_pydict(py, item)?)?;
            }
            Ok(list.into())
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, json_to_pydict(py, v)?)?;
            }
            Ok(dict.into())
        }
    }
}

/// Convert Python dict to JSON value
fn pydict_to_json(py: Python, dict: &Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    use pyo3::types::{PyBool, PyFloat, PyInt, PyString};
    
    let mut map = serde_json::Map::new();
    for (key, value) in dict.iter() {
        let key_str: String = key.extract()?;
        let json_value = if value.is_none() {
            serde_json::Value::Null
        } else if let Ok(b) = value.cast::<PyBool>() {
            serde_json::Value::Bool(b.is_true())
        } else if let Ok(i) = value.cast::<PyInt>() {
            serde_json::Value::Number(serde_json::Number::from(i.extract::<i64>()?))
        } else if let Ok(f) = value.cast::<PyFloat>() {
            serde_json::Number::from_f64(f.value())
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        } else if let Ok(s) = value.cast::<PyString>() {
            serde_json::Value::String(s.extract()?)
        } else if let Ok(list) = value.cast::<PyList>() {
            let mut arr = Vec::new();
            for item in list {
                if let Ok(d) = item.cast::<PyDict>() {
                    arr.push(pydict_to_json(py, d)?);
                } else {
                    // Try to extract as primitive
                    arr.push(if item.is_none() {
                        serde_json::Value::Null
                    } else if let Ok(s) = item.extract::<String>() {
                        serde_json::Value::String(s)
                    } else if let Ok(i) = item.extract::<i64>() {
                        serde_json::Value::Number(serde_json::Number::from(i))
                    } else if let Ok(f) = item.extract::<f64>() {
                        serde_json::Number::from_f64(f)
                            .map(serde_json::Value::Number)
                            .unwrap_or(serde_json::Value::Null)
                    } else if let Ok(b) = item.extract::<bool>() {
                        serde_json::Value::Bool(b)
                    } else {
                        serde_json::Value::Null
                    });
                }
            }
            serde_json::Value::Array(arr)
        } else if let Ok(d) = value.cast::<PyDict>() {
            pydict_to_json(py, d)?
        } else {
            serde_json::Value::Null
        };
        map.insert(key_str, json_value);
    }
    Ok(serde_json::Value::Object(map))
}

/// Convert Python dict to FieldType map
fn pydict_to_fieldtype_map(
    py: Python,
    dict: &Bound<'_, PyDict>,
) -> PyResult<std::collections::HashMap<String, FieldType>> {
    let json = pydict_to_json(py, dict)?;
    serde_json::from_value(json)
        .map_err(|e| PyValueError::new_err(format!("Failed to convert to FieldType map: {}", e)))
}

/// ekoDB Python module
#[pymodule]
fn _ekodb_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Client>()?;
    m.add_class::<WebSocketClient>()?;
    m.add_class::<RateLimitInfo>()?;
    m.add_class::<SerializationFormat>()?;
    m.add("RateLimitError", m.py().get_type::<RateLimitError>())?;
    Ok(())
}
