//! Python bindings for ekoDB client library
//!
//! This module provides Python bindings for the ekoDB Rust client library using PyO3.

use ekodb_client::{
    ChatMessageRequest, ChatResponse, Client as RustClient, CollectionConfig,
    CreateChatSessionRequest, DistinctValuesQuery as RustDistinctValuesQuery, FieldType,
    RawCompletionRequest as RustRawCompletionRequest,
    GetMessagesQuery, GetMessagesResponse, ListSessionsQuery, ListSessionsResponse,
    Query as RustQuery, RateLimitInfo as RustRateLimitInfo, Record as RustRecord,
    Script as RustScript, SearchQuery as RustSearchQuery,
    SerializationFormat as RustSerializationFormat, UpdateSessionRequest,
    UserFunction as RustUserFunction, WebSocketClient as RustWebSocketClient,
};
use serde_json;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple};
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
    ///     bypass_ripple: Optional flag to bypass ripple propagation
    ///     transaction_id: Optional transaction ID for atomic operations
    ///     bypass_cache: Optional flag to bypass cache
    #[pyo3(signature = (collection, record, ttl=None, bypass_ripple=None, transaction_id=None, bypass_cache=None))]
    fn insert<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        record: &Bound<'py, PyDict>,
        ttl: Option<String>,
        bypass_ripple: Option<bool>,
        transaction_id: Option<String>,
        bypass_cache: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rust_record = dict_to_record(record)?;

        // Build InsertOptions from Python parameters
        let options = if ttl.is_some() || bypass_ripple.is_some() || transaction_id.is_some() || bypass_cache.is_some() {
            let mut opts = ekodb_client::options::InsertOptions::new();
            if let Some(t) = ttl {
                opts = opts.ttl(t);
            }
            if let Some(br) = bypass_ripple {
                opts = opts.bypass_ripple(br);
            }
            if let Some(tx) = transaction_id {
                opts = opts.transaction_id(tx);
            }
            if let Some(bc) = bypass_cache {
                opts = opts.bypass_cache(bc);
            }
            Some(opts)
        } else {
            None
        };

        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let result = client
                .insert(&collection, rust_record, options)
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
    ///
    /// Args:
    ///     bypass_ripple: Optional flag to bypass ripple propagation
    ///     transaction_id: Optional transaction ID for atomic operations
    ///     bypass_cache: Optional flag to bypass cache
    #[pyo3(signature = (collection, id, updates, bypass_ripple=None, transaction_id=None, bypass_cache=None))]
    fn update<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        updates: &Bound<'py, PyDict>,
        bypass_ripple: Option<bool>,
        transaction_id: Option<String>,
        bypass_cache: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rust_updates = dict_to_record(updates)?;

        // Build UpdateOptions from Python parameters
        let options = if bypass_ripple.is_some() || transaction_id.is_some() || bypass_cache.is_some() {
            let mut opts = ekodb_client::options::UpdateOptions::new();
            if let Some(br) = bypass_ripple {
                opts = opts.bypass_ripple(br);
            }
            if let Some(tx) = transaction_id {
                opts = opts.transaction_id(tx);
            }
            if let Some(bc) = bypass_cache {
                opts = opts.bypass_cache(bc);
            }
            Some(opts)
        } else {
            None
        };

        let client = self.inner.clone();

        future_into_py(py, async move {
            let result = client
                .update(&collection, &id, rust_updates, options)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Update failed: {}", e)))?;

            Python::attach(|py| record_to_dict(py, &result))
        })
    }

    /// Apply an atomic field action to a single field of a record.
    ///
    /// Use this instead of `update()` for safe concurrent modifications like
    /// incrementing counters, pushing to arrays, or arithmetic operations.
    ///
    /// Args:
    ///     collection: Collection name
    ///     id: Record ID
    ///     action: The atomic action (increment, decrement, multiply, divide, modulo,
    ///             push, pop, shift, unshift, remove, append, clear)
    ///     field: The field name to apply the action to
    ///     value: The value for the action (omit for pop/shift/clear)
    #[pyo3(signature = (collection, id, action, field, value=None))]
    fn update_with_action<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        action: String,
        field: String,
        value: Option<&Bound<'py, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let field_value = match value {
            Some(v) => py_to_field_type(v)?,
            None => FieldType::Null,
        };
        let client = self.inner.clone();

        future_into_py(py, async move {
            let result = client
                .update_with_action(&collection, &id, &action, &field, field_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Action failed: {}", e)))?;

            Python::attach(|py| record_to_dict(py, &result))
        })
    }

    /// Apply a sequence of atomic field actions to a record in a single request.
    ///
    /// All actions are applied atomically — the record is fetched once, all actions
    /// run in order, and the result is persisted in a single update.
    ///
    /// Args:
    ///     collection: Collection name
    ///     id: Record ID
    ///     actions: List of (action, field, value) tuples
    #[pyo3(signature = (collection, id, actions))]
    fn update_with_action_sequence<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        actions: Vec<(String, String, Bound<'py, PyAny>)>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rust_actions: Vec<(String, String, FieldType)> = actions
            .iter()
            .map(|(action, field, value)| {
                let fv = py_to_field_type(value)?;
                Ok((action.clone(), field.clone(), fv))
            })
            .collect::<PyResult<Vec<_>>>()?;

        let client = self.inner.clone();

        future_into_py(py, async move {
            let result = client
                .update_with_action_sequence(&collection, &id, rust_actions)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Action sequence failed: {}", e)))?;

            Python::attach(|py| record_to_dict(py, &result))
        })
    }

    /// Delete a document
    ///
    /// Args:
    ///     bypass_ripple: Optional flag to bypass ripple propagation
    ///     transaction_id: Optional transaction ID for atomic operations
    #[pyo3(signature = (collection, id, bypass_ripple=None, transaction_id=None))]
    fn delete<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        bypass_ripple: Option<bool>,
        transaction_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let _ = transaction_id; // Acknowledge unused param
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

    // ========== Convenience Methods ==========

    /// Insert or update a record (upsert operation)
    ///
    /// Attempts to update the record first. If the record doesn't exist (NotFound error),
    /// it will be inserted instead. This provides atomic insert-or-update semantics.
    ///
    /// Args:
    ///     collection: Collection name
    ///     id: Record ID
    ///     record: Document data as a dict
    ///     bypass_ripple: Optional flag to bypass ripple effects
    ///
    /// Returns:
    ///     The inserted or updated record as a dict
    fn upsert<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
        record: &Bound<'py, PyDict>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rust_record = dict_to_record(record)?;
        let client = self.inner.clone();

        // Build options from bypass_ripple
        let update_options = bypass_ripple.map(|br| {
            ekodb_client::options::UpdateOptions::new().bypass_ripple(br)
        });
        let insert_options = bypass_ripple.map(|br| {
            ekodb_client::options::InsertOptions::new().bypass_ripple(br)
        });

        future_into_py::<_, Py<PyAny>>(py, async move {
            // Try update first
            match client.update(&collection, &id, rust_record.clone(), update_options).await {
                Ok(updated) => {
                    Python::attach(|py| record_to_dict(py, &updated))
                }
                Err(e) => {
                    // Check if it's a NotFound error
                    let error_msg = e.to_string();
                    if error_msg.contains("Not found") || error_msg.contains("404") {
                        // Record doesn't exist, insert it
                        let inserted = client
                            .insert(&collection, rust_record, insert_options)
                            .await
                            .map_err(|e| PyRuntimeError::new_err(format!("Upsert insert failed: {}", e)))?;
                        Python::attach(|py| record_to_dict(py, &inserted))
                    } else {
                        // Other error, propagate it
                        Err(PyRuntimeError::new_err(format!("Upsert failed: {}", e)))
                    }
                }
            }
        })
    }

    /// Find a single record by field value
    /// 
    /// Convenience method for finding one record matching a specific field value.
    /// Returns None if no record matches, or the first matching record.
    /// 
    /// Args:
    ///     collection: Collection name
    ///     field: Field name to search
    ///     value: Value to match (any JSON-serializable type)
    /// 
    /// Returns:
    ///     The matching record as a dict, or None if not found
    fn find_one<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        field: String,
        value: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        
        // Convert Python value to JSON
        let json_value = py_to_json(&value)?;

        future_into_py::<_, Py<PyAny>>(py, async move {
            // Build query with eq filter and limit 1
            let filter_json = serde_json::json!({
                "type": "Condition",
                "content": {
                    "field": field,
                    "operator": "Eq",
                    "value": json_value
                }
            });

            let mut query = RustQuery::new();
            query.filter = Some(filter_json);
            query.limit = Some(1);

            let results = client
                .find(&collection, query, None)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Find one failed: {}", e)))?;

            Python::attach(|py| {
                if let Some(record) = results.first() {
                    record_to_dict(py, record)
                } else {
                    Ok(py.None())
                }
            })
        })
    }

    /// Check if a record exists by ID
    /// 
    /// This is more efficient than fetching the record when you only need to check existence.
    /// 
    /// Args:
    ///     collection: Collection name
    ///     id: Record ID to check
    /// 
    /// Returns:
    ///     True if the record exists, False if it doesn't
    fn exists<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            match client.find_by_id(&collection, &id, None).await {
                Ok(_) => Python::attach(|py| Ok(PyBool::new(py, true).as_borrowed().to_owned().into())),
                Err(e) => {
                    let error_msg = e.to_string().to_lowercase();
                    if error_msg.contains("not found") || error_msg.contains("404") {
                        Python::attach(|py| Ok(PyBool::new(py, false).as_borrowed().to_owned().into()))
                    } else {
                        Err(PyRuntimeError::new_err(format!("Exists check failed: {}", e)))
                    }
                }
            }
        })
    }

    /// Paginate through records
    /// 
    /// Convenience method for pagination with page numbers (1-indexed).
    /// 
    /// Args:
    ///     collection: Collection name
    ///     page: Page number (1-indexed, i.e., first page is 1)
    ///     page_size: Number of records per page
    /// 
    /// Returns:
    ///     List of records for the requested page
    fn paginate<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        page: usize,
        page_size: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            // Page 1 = skip 0, Page 2 = skip page_size, etc.
            let skip = if page > 0 { (page - 1) * page_size } else { 0 };

            let mut query = RustQuery::new();
            query.limit = Some(page_size);
            query.skip = Some(skip);

            let results = client
                .find(&collection, query, None)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Paginate failed: {}", e)))?;

            Python::attach(|py| {
                let list = PyList::empty(py);
                for record in results {
                    list.append(record_to_dict(py, &record)?)?;
                }
                Ok(list.into())
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

    /// Search documents with full-text, vector, or hybrid search with all available parameters
    /// 
    /// Args:
    ///     collection: Collection name
    ///     query: Search query text (required)
    ///     language: Language for stemming (optional)
    ///     case_sensitive: Case-sensitive search (optional)
    ///     fuzzy: Enable fuzzy matching (optional)
    ///     min_score: Minimum score threshold (optional)
    ///     fields: Fields to search in (optional)
    ///     weights: Field weights as string "field:weight,field2:weight2" (optional)
    ///     enable_stemming: Enable stemming (optional)
    ///     boost_exact: Boost exact matches (optional)
    ///     max_edit_distance: Maximum edit distance for fuzzy (optional)
    ///     vector: Query vector for semantic search (optional)
    ///     vector_field: Field containing vectors (optional)
    ///     vector_metric: Similarity metric (optional)
    ///     vector_k: Number of vector results (optional)
    ///     vector_threshold: Minimum similarity threshold (optional)
    ///     text_weight: Weight for text search in hybrid (optional)
    ///     vector_weight: Weight for vector search in hybrid (optional)
    ///     bypass_ripple: Bypass ripple cache (optional)
    ///     bypass_cache: Bypass cache (optional)
    ///     limit: Maximum number of results (optional)
    ///     select_fields: Fields to include in results (optional)
    ///     exclude_fields: Fields to exclude from results (optional)
    #[pyo3(signature = (collection, query, language=None, case_sensitive=None, fuzzy=None, min_score=None, fields=None, weights=None, enable_stemming=None, boost_exact=None, max_edit_distance=None, vector=None, vector_field=None, vector_metric=None, vector_k=None, vector_threshold=None, text_weight=None, vector_weight=None, bypass_ripple=None, bypass_cache=None, limit=None, select_fields=None, exclude_fields=None))]
    fn search<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        query: String,
        language: Option<String>,
        case_sensitive: Option<bool>,
        fuzzy: Option<bool>,
        min_score: Option<f64>,
        fields: Option<String>,
        weights: Option<String>,
        enable_stemming: Option<bool>,
        boost_exact: Option<bool>,
        max_edit_distance: Option<u32>,
        vector: Option<Vec<f64>>,
        vector_field: Option<String>,
        vector_metric: Option<String>,
        vector_k: Option<usize>,
        vector_threshold: Option<f64>,
        text_weight: Option<f64>,
        vector_weight: Option<f64>,
        bypass_ripple: Option<bool>,
        bypass_cache: Option<bool>,
        limit: Option<usize>,
        select_fields: Option<Vec<String>>,
        exclude_fields: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        // Build SearchQuery with all parameters
        let search_query = RustSearchQuery {
            query,
            language,
            case_sensitive,
            fuzzy,
            min_score,
            fields,
            weights,
            enable_stemming,
            boost_exact,
            max_edit_distance,
            bypass_ripple,
            bypass_cache,
            limit,
            vector,
            vector_field,
            vector_metric,
            vector_k,
            vector_threshold,
            text_weight,
            vector_weight,
            select_fields,
            exclude_fields,
        };

        future_into_py(py, async move {
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

    /// Get distinct (unique) values for a field across a collection.
    ///
    /// Returns a dict: { collection, field, values: [...], count: N }
    ///
    /// Args:
    ///     collection: Collection name
    ///     field: Field to get distinct values for
    ///     filter: Optional filter dict (same format as find())
    ///     bypass_ripple: Optional flag to bypass ripple propagation
    ///     bypass_cache: Optional flag to bypass cache
    #[pyo3(signature = (collection, field, filter=None, bypass_ripple=None, bypass_cache=None))]
    fn distinct_values<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        field: String,
        filter: Option<&Bound<'py, PyDict>>,
        bypass_ripple: Option<bool>,
        bypass_cache: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let filter_json = if let Some(f) = filter {
            Some(dict_to_json(f)?)
        } else {
            None
        };

        future_into_py(py, async move {
            let mut query = RustDistinctValuesQuery::new();
            if let Some(f) = filter_json {
                query = query.filter(f);
            }
            if let Some(br) = bypass_ripple {
                query = query.bypass_ripple(br);
            }
            if let Some(bc) = bypass_cache {
                query = query.bypass_cache(bc);
            }

            let resp = client
                .distinct_values(&collection, &field, query)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("distinct_values failed: {}", e)))?;

            Python::attach(|py| {
                let resp_json = serde_json::to_value(&resp).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize response: {}", e))
                })?;
                json_to_pydict(py, &resp_json)
            })
        })
    }

    /// Stateless raw LLM completion — no session, no history, no RAG.
    ///
    /// Sends a system prompt and user message directly to the LLM via ekoDB
    /// and returns the raw text response. Returns a dict: { content: "..." }
    ///
    /// Args:
    ///     system_prompt: System prompt passed verbatim to the LLM
    ///     message: User message passed verbatim to the LLM
    ///     provider: Optional LLM provider override
    ///     model: Optional model name override
    ///     max_tokens: Optional max tokens override
    #[pyo3(signature = (system_prompt, message, provider=None, model=None, max_tokens=None))]
    fn raw_completion<'py>(
        &self,
        py: Python<'py>,
        system_prompt: String,
        message: String,
        provider: Option<String>,
        model: Option<String>,
        max_tokens: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let request = RustRawCompletionRequest {
                system_prompt,
                message,
                provider,
                model,
                max_tokens,
            };

            let resp = client
                .raw_completion(request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("raw_completion failed: {}", e)))?;

            Python::attach(|py| {
                let resp_json = serde_json::to_value(&resp).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize response: {}", e))
                })?;
                json_to_pydict(py, &resp_json)
            })
        })
    }

    /// Stateless raw LLM completion via SSE streaming.
    ///
    /// Same as raw_completion() but uses Server-Sent Events to keep the
    /// connection alive. Preferred for deployed instances where reverse proxies
    /// may kill idle HTTP connections before the LLM responds.
    ///
    /// Args:
    ///     system_prompt: System instructions for the LLM
    ///     message: User message to send
    ///     provider: LLM provider (optional, uses server default)
    ///     model: Model name (optional, uses server default)
    ///     max_tokens: Maximum tokens in response (optional)
    ///
    /// Returns:
    ///     dict with "content" key containing the LLM response text
    fn raw_completion_stream<'py>(
        &self,
        py: Python<'py>,
        system_prompt: String,
        message: String,
        provider: Option<String>,
        model: Option<String>,
        max_tokens: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let request = RustRawCompletionRequest {
                system_prompt,
                message,
                provider,
                model,
                max_tokens,
            };

            let resp = client
                .raw_completion_stream(request)
                .await
                .map_err(|e| {
                    PyRuntimeError::new_err(format!("raw_completion_stream failed: {}", e))
                })?;

            Python::attach(|py| {
                let resp_json = serde_json::to_value(&resp).map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to serialize response: {}", e))
                })?;
                json_to_pydict(py, &resp_json)
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

    /// Generate embeddings for multiple texts in a single batch request
    ///
    /// Args:
    ///     texts: List of texts to generate embeddings for
    ///     model: The embedding model to use (e.g., "text-embedding-3-small")
    fn embed_batch<'py>(
        &self,
        py: Python<'py>,
        texts: Vec<String>,
        model: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let embeddings = client
                .embed_batch(texts, &model)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Embed batch failed: {}", e)))?;

            Python::attach(|py| {
                let outer = PyList::empty(py);
                for embedding in &embeddings {
                    let inner = PyList::new(py, embedding)?;
                    outer.append(inner)?;
                }
                Ok(outer.into())
            })
        })
    }

    /// Set a key-value pair
    ///
    /// Args:
    ///     key: The key
    ///     value: The value as a dict
    ///     ttl: Optional TTL duration (e.g., "30s", "5m", "1h")
    #[pyo3(signature = (key, value, ttl=None))]
    fn kv_set<'py>(
        &self,
        py: Python<'py>,
        key: String,
        value: &Bound<'py, PyDict>,
        ttl: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        // Convert Python dict to JSON value
        let json_str = serde_json::to_string(&dict_to_json(value)?)
            .map_err(|e| PyRuntimeError::new_err(format!("JSON error: {}", e)))?;
        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| PyRuntimeError::new_err(format!("JSON parse error: {}", e)))?;

        future_into_py(py, async move {
            client
                .kv_set(&key, json_value, ttl.as_deref())
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

    /// Batch get multiple keys
    /// 
    /// Args:
    ///     keys: List of keys to retrieve
    /// 
    /// Returns:
    ///     List of records corresponding to the keys
    fn kv_batch_get<'py>(
        &self,
        py: Python<'py>,
        keys: Vec<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let results = client
                .kv_batch_get(keys)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV batch get failed: {}", e)))?;

            Python::attach(|py| {
                let list = PyList::empty(py);
                for record in results {
                    list.append(record_to_dict(py, &record)?)?;
                }
                Ok(list.into())
            })
        })
    }

    /// Batch set multiple key-value pairs.
    /// Note: TTL from the first entry with a valid TTL is applied to all entries (server limitation).
    ///
    /// Args:
    ///     entries: List of dicts with 'key', 'value', and optional 'ttl' fields
    ///              (ttl from first entry applies to all)
    ///
    /// Returns:
    ///     List of tuples [(key, was_set), ...]
    fn kv_batch_set<'py>(
        &self,
        py: Python<'py>,
        entries: Vec<Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        
        let mut keys = Vec::new();
        let mut values = Vec::new();
        let mut ttl = None;
        
        for entry in entries {
            let key: String = entry.get_item("key")?
                .ok_or_else(|| PyValueError::new_err("Entry missing 'key' field"))?
                .extract()?;
            let value_bound = entry.get_item("value")?
                .ok_or_else(|| PyValueError::new_err("Entry missing 'value' field"))?;
            
            // Convert value to Record
            let value_dict = value_bound.cast::<PyDict>()
                .map_err(|_| PyValueError::new_err("Entry 'value' must be a dict"))?;
            
            keys.push(key);
            values.push(dict_to_record(value_dict)?);
            
            // Server applies a single TTL to all entries - use first entry's TTL if provided
            if ttl.is_none() {
                if let Ok(Some(ttl_val)) = entry.get_item("ttl") {
                    ttl = Some(ttl_val.extract::<i64>()?);
                }
            }
        }

        future_into_py::<_, Py<PyAny>>(py, async move {
            let results = client
                .kv_batch_set(keys.clone(), values, ttl)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV batch set failed: {}", e)))?;

            Python::attach(|py| {
                let list = PyList::empty(py);
                for (key, was_set) in results {
                    let py_key = PyString::new(py, &key);
                    let py_bool = PyBool::new(py, was_set);
                    let tuple = PyTuple::new(py, &[py_key.as_any(), py_bool.as_any()])?;
                    list.append(tuple)?;
                }
                Ok(list.into())
            })
        })
    }

    /// Batch delete multiple keys
    /// 
    /// Args:
    ///     keys: List of keys to delete
    /// 
    /// Returns:
    ///     List of tuples [(key, was_deleted), ...]
    fn kv_batch_delete<'py>(
        &self,
        py: Python<'py>,
        keys: Vec<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let results = client
                .kv_batch_delete(keys.clone())
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("KV batch delete failed: {}", e)))?;

            Python::attach(|py| {
                let list = PyList::empty(py);
                for (key, was_deleted) in results {
                    let py_key = PyString::new(py, &key);
                    let py_bool = PyBool::new(py, was_deleted);
                    let tuple = PyTuple::new(py, &[py_key.as_any(), py_bool.as_any()])?;
                    list.append(tuple)?;
                }
                Ok(list.into())
            })
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
    #[pyo3(signature = (collections, llm_provider, llm_model=None, system_prompt=None, max_context_messages=None, bypass_ripple=None, max_tokens=None, temperature=None))]
    fn create_chat_session<'py>(
        &self,
        py: Python<'py>,
        collections: Vec<(String, Vec<String>)>,
        llm_provider: String,
        llm_model: Option<String>,
        system_prompt: Option<String>,
        max_context_messages: Option<usize>,
        bypass_ripple: Option<bool>,
        max_tokens: Option<i32>,
        temperature: Option<f32>,
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
                bypass_ripple,
                parent_id: None,
                branch_point_idx: None,
                max_context_messages,
                max_tokens,
                temperature,
                tool_config: None,
            };

            let result = client
                .create_chat_session(request)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Create session failed: {}", e)))?;

            Python::attach(|py| chat_response_to_dict(py, &result))
        })
    }

    /// Send a message in an existing chat session
    #[pyo3(signature = (chat_id, message, bypass_ripple=None, force_summarize=None, max_iterations=None))]
    fn chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message: String,
        bypass_ripple: Option<bool>,
        force_summarize: Option<bool>,
        max_iterations: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let request = ChatMessageRequest {
                message,
                bypass_ripple,
                force_summarize,
                max_iterations,
                tool_config: None,
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
    #[pyo3(signature = (chat_id, system_prompt=None, llm_model=None, title=None, max_context_messages=None, bypass_ripple=None))]
    fn update_chat_session<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        system_prompt: Option<String>,
        llm_model: Option<String>,
        title: Option<String>,
        max_context_messages: Option<usize>,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let request = UpdateSessionRequest {
                system_prompt,
                llm_model,
                collections: None,
                max_context_messages,
                bypass_ripple,
                title,
                memory: None,
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
                max_tokens: None,
                temperature: None,
                tool_config: None,
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
    #[pyo3(signature = (source_chat_ids, target_chat_id, merge_strategy, bypass_ripple=None))]
    fn merge_chat_sessions<'py>(
        &self,
        py: Python<'py>,
        source_chat_ids: Vec<String>,
        target_chat_id: String,
        merge_strategy: String,
        bypass_ripple: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            use ekodb_client::MergeStrategy;

            let strategy = match merge_strategy.as_str() {
                "Chronological" => MergeStrategy::Chronological,
                "Summarized" => MergeStrategy::Summarized,
                "LatestOnly" => MergeStrategy::LatestOnly,
                "Interleaved" => MergeStrategy::Interleaved,
                _ => return Err(PyRuntimeError::new_err(format!("Invalid merge strategy: {}. Valid options: Chronological, Summarized, LatestOnly, Interleaved", merge_strategy))),
            };

            let request = ekodb_client::MergeSessionsRequest {
                source_chat_ids: source_chat_ids,
                target_chat_id: target_chat_id,
                merge_strategy: strategy,
                bypass_ripple,
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

    // ========================================================================
    // CHAT MODELS API
    // ========================================================================

    /// Get all available chat models from all providers
    ///
    /// Returns:
    ///     Dict with provider names as keys and lists of model names as values
    /// Get all built-in server-side chat tool definitions.
    ///
    /// Returns a list of dicts with name, description, and parameters fields.
    /// Used by planning agents to discover available tools dynamically.
    fn get_chat_tools<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let tools = client
                .get_chat_tools()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get chat tools failed: {}", e)))?;

            Python::attach(|py| {
                let list = pyo3::types::PyList::empty(py);
                for tool in tools {
                    let json_str = serde_json::to_string(&tool)
                        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
                    let obj: Py<PyAny> = py
                        .import("json")?
                        .call_method1("loads", (json_str,))?
                        .into();
                    list.append(obj)?;
                }
                Ok(list.into())
            })
        })
    }

    fn get_chat_models<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let models = client
                .get_chat_models()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get chat models failed: {}", e)))?;

            Python::attach(|py| {
                let dict = PyDict::new(py);
                dict.set_item("openai", models.openai)?;
                dict.set_item("anthropic", models.anthropic)?;
                dict.set_item("perplexity", models.perplexity)?;
                Ok(dict.into())
            })
        })
    }

    /// Get available models for a specific provider
    ///
    /// Args:
    ///     provider_name: Name of the provider (e.g., "openai", "anthropic")
    ///
    /// Returns:
    ///     List of model names for the specified provider
    fn get_chat_model<'py>(
        &self,
        py: Python<'py>,
        provider_name: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let models = client
                .get_chat_model(&provider_name)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get chat model failed: {}", e)))?;

            Python::attach(|py| {
                let list = PyList::new(py, &models)?;
                Ok(list.into())
            })
        })
    }

    /// Get a specific chat message by ID
    ///
    /// Args:
    ///     chat_id: Chat session ID
    ///     message_id: Message ID
    ///
    /// Returns:
    ///     Message record as a dict
    fn get_chat_message<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let message = client
                .get_chat_message(&chat_id, &message_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get chat message failed: {}", e)))?;

            Python::attach(|py| record_to_dict(py, &message))
        })
    }

    // ========================================================================
    // USER FUNCTIONS API
    // ========================================================================

    /// Save a new user function definition
    ///
    /// Args:
    ///     user_function: User function definition as a dict
    ///
    /// Returns:
    ///     User function ID string
    fn save_user_function<'py>(
        &self,
        py: Python<'py>,
        user_function: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let func_json = serde_json::to_string(&pydict_to_json(py, user_function)?)
            .map_err(|e| PyValueError::new_err(format!("Invalid user function definition: {}", e)))?;

        future_into_py::<_, Py<PyAny>>(py, async move {
            let user_func: RustUserFunction = serde_json::from_str(&func_json)
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to parse user function: {}", e)))?;

            let id = client
                .save_user_function(user_func)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Save user function failed: {}", e)))?;

            Python::attach(|py| Ok(PyString::new(py, &id).into()))
        })
    }

    /// Get a user function by label
    ///
    /// Args:
    ///     label: User function label (unique identifier)
    ///
    /// Returns:
    ///     User function definition as a dict
    fn get_user_function<'py>(
        &self,
        py: Python<'py>,
        label: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let user_func = client
                .get_user_function(&label)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get user function failed: {}", e)))?;

            let json = serde_json::to_value(&user_func)
                .map_err(|e| PyRuntimeError::new_err(format!("Serialization failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &json))
        })
    }

    /// List all user functions, optionally filtered by tags
    ///
    /// Args:
    ///     tags: Optional list of tags to filter by
    ///
    /// Returns:
    ///     List of user function definitions
    #[pyo3(signature = (tags=None))]
    fn list_user_functions<'py>(
        &self,
        py: Python<'py>,
        tags: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            let user_funcs = client
                .list_user_functions(tags)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("List user functions failed: {}", e)))?;

            let json = serde_json::to_value(&user_funcs)
                .map_err(|e| PyRuntimeError::new_err(format!("Serialization failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &json))
        })
    }

    /// Update an existing user function by label
    ///
    /// Args:
    ///     label: User function label (unique identifier)
    ///     user_function: Updated user function definition as a dict
    fn update_user_function<'py>(
        &self,
        py: Python<'py>,
        label: String,
        user_function: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let func_json = serde_json::to_string(&pydict_to_json(py, user_function)?)
            .map_err(|e| PyValueError::new_err(format!("Invalid user function definition: {}", e)))?;

        future_into_py(py, async move {
            let user_func: RustUserFunction = serde_json::from_str(&func_json)
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to parse user function: {}", e)))?;

            client
                .update_user_function(&label, user_func)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Update user function failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Delete a user function by label
    ///
    /// Args:
    ///     label: User function label (unique identifier)
    fn delete_user_function<'py>(
        &self,
        py: Python<'py>,
        label: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py(py, async move {
            client
                .delete_user_function(&label)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Delete user function failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    // ========================================================================
    // COLLECTION UTILITIES
    // ========================================================================

    /// Check if a collection exists
    ///
    /// Args:
    ///     collection: Collection name
    ///
    /// Returns:
    ///     True if the collection exists, False otherwise
    fn collection_exists<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let exists = client
                .collection_exists(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Collection exists check failed: {}", e)))?;

            Python::attach(|py| Ok(PyBool::new(py, exists).as_borrowed().to_owned().into()))
        })
    }

    /// Count documents in a collection
    ///
    /// Args:
    ///     collection: Collection name
    ///
    /// Returns:
    ///     Number of documents in the collection
    fn count_documents<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let count = client
                .count_documents(&collection)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Count documents failed: {}", e)))?;

            Python::attach(|py| Ok(PyInt::new(py, count as i64).into()))
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

    /// Refresh the authentication token (clears cache and fetches new JWT)
    fn refresh_token<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py::<_, Py<PyAny>>(py, async move {
            let token = client
                .refresh_token()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Token refresh failed: {}", e)))?;
            Python::attach(|py| Ok(PyString::new(py, &token).into()))
        })
    }

    /// Clear the cached authentication token
    fn clear_token_cache<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py::<_, Py<PyAny>>(py, async move {
            client.clear_token_cache().await;
            Python::attach(|py| Ok(py.None()))
        })
    }

    // ── Goal CRUD ──────────────────────────────────────────────────────

    /// Create a new goal
    ///
    /// Args:
    ///     data: Goal definition as a dict
    ///
    /// Returns:
    ///     The created goal as a dict
    fn goal_create<'py>(
        &self,
        py: Python<'py>,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .goal_create(json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_create failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// List all goals
    ///
    /// Returns:
    ///     A dict containing the list of goals
    fn goal_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .goal_list()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_list failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Get a goal by ID
    ///
    /// Args:
    ///     id: Goal ID
    ///
    /// Returns:
    ///     The goal as a dict
    fn goal_get<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .goal_get(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_get failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Update an existing goal
    ///
    /// Args:
    ///     id: Goal ID
    ///     data: Updated goal fields as a dict
    ///
    /// Returns:
    ///     The updated goal as a dict
    fn goal_update<'py>(
        &self,
        py: Python<'py>,
        id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .goal_update(&id, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_update failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Delete a goal by ID
    ///
    /// Args:
    ///     id: Goal ID
    fn goal_delete<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            client
                .goal_delete(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_delete failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Search goals by query string
    ///
    /// Args:
    ///     query: Search query
    ///
    /// Returns:
    ///     A dict containing matching goals
    fn goal_search<'py>(&self, py: Python<'py>, query: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .goal_search(&query)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_search failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    // ── Goal Lifecycle ─────────────────────────────────────────────────

    /// Mark a goal as complete
    ///
    /// Args:
    ///     id: Goal ID
    ///     data: Completion data as a dict
    ///
    /// Returns:
    ///     The completed goal as a dict
    fn goal_complete<'py>(
        &self,
        py: Python<'py>,
        id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .goal_complete(&id, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_complete failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Approve a goal
    ///
    /// Args:
    ///     id: Goal ID
    ///
    /// Returns:
    ///     The approved goal as a dict
    fn goal_approve<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .goal_approve(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_approve failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Reject a goal
    ///
    /// Args:
    ///     id: Goal ID
    ///     data: Rejection reason/data as a dict
    ///
    /// Returns:
    ///     The rejected goal as a dict
    fn goal_reject<'py>(
        &self,
        py: Python<'py>,
        id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .goal_reject(&id, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_reject failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    // ── Goal Step Lifecycle ────────────────────────────────────────────

    /// Start a goal step
    ///
    /// Args:
    ///     id: Goal ID
    ///     step_index: Index of the step to start
    ///
    /// Returns:
    ///     The updated goal as a dict
    fn goal_step_start<'py>(
        &self,
        py: Python<'py>,
        id: String,
        step_index: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .goal_step_start(&id, step_index)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_step_start failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Complete a goal step
    ///
    /// Args:
    ///     id: Goal ID
    ///     step_index: Index of the step to complete
    ///     data: Step completion data as a dict
    ///
    /// Returns:
    ///     The updated goal as a dict
    fn goal_step_complete<'py>(
        &self,
        py: Python<'py>,
        id: String,
        step_index: usize,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .goal_step_complete(&id, step_index, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_step_complete failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Fail a goal step
    ///
    /// Args:
    ///     id: Goal ID
    ///     step_index: Index of the step that failed
    ///     data: Failure data as a dict
    ///
    /// Returns:
    ///     The updated goal as a dict
    fn goal_step_fail<'py>(
        &self,
        py: Python<'py>,
        id: String,
        step_index: usize,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .goal_step_fail(&id, step_index, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("goal_step_fail failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    // ── Task CRUD ──────────────────────────────────────────────────────

    /// Create a new scheduled task
    ///
    /// Args:
    ///     data: Task definition as a dict
    ///
    /// Returns:
    ///     The created task as a dict
    fn task_create<'py>(
        &self,
        py: Python<'py>,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .task_create(json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_create failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// List all scheduled tasks
    ///
    /// Returns:
    ///     A dict containing the list of tasks
    fn task_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .task_list()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_list failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Get a task by ID
    ///
    /// Args:
    ///     id: Task ID
    ///
    /// Returns:
    ///     The task as a dict
    fn task_get<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .task_get(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_get failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Update an existing task
    ///
    /// Args:
    ///     id: Task ID
    ///     data: Updated task fields as a dict
    ///
    /// Returns:
    ///     The updated task as a dict
    fn task_update<'py>(
        &self,
        py: Python<'py>,
        id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .task_update(&id, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_update failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Delete a task by ID
    ///
    /// Args:
    ///     id: Task ID
    fn task_delete<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            client
                .task_delete(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_delete failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Get tasks that are due at the given time
    ///
    /// Args:
    ///     now: ISO 8601 timestamp string
    ///
    /// Returns:
    ///     A dict containing due tasks
    fn task_due<'py>(&self, py: Python<'py>, now: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .task_due(&now)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_due failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    // ── Task Lifecycle ─────────────────────────────────────────────────

    /// Start a task
    ///
    /// Args:
    ///     id: Task ID
    ///
    /// Returns:
    ///     The started task as a dict
    fn task_start<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .task_start(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_start failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Mark a task as succeeded
    ///
    /// Args:
    ///     id: Task ID
    ///     data: Success data as a dict
    ///
    /// Returns:
    ///     The succeeded task as a dict
    fn task_succeed<'py>(
        &self,
        py: Python<'py>,
        id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .task_succeed(&id, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_succeed failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Mark a task as failed
    ///
    /// Args:
    ///     id: Task ID
    ///     data: Failure data as a dict
    ///
    /// Returns:
    ///     The failed task as a dict
    fn task_fail<'py>(
        &self,
        py: Python<'py>,
        id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .task_fail(&id, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_fail failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Pause a task
    ///
    /// Args:
    ///     id: Task ID
    ///
    /// Returns:
    ///     The paused task as a dict
    fn task_pause<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .task_pause(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_pause failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Resume a paused task
    ///
    /// Args:
    ///     id: Task ID
    ///     data: Resume data as a dict
    ///
    /// Returns:
    ///     The resumed task as a dict
    fn task_resume<'py>(
        &self,
        py: Python<'py>,
        id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .task_resume(&id, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("task_resume failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    // ── Agent CRUD ─────────────────────────────────────────────────────

    /// Create a new agent
    ///
    /// Args:
    ///     data: Agent definition as a dict
    ///
    /// Returns:
    ///     The created agent as a dict
    fn agent_create<'py>(
        &self,
        py: Python<'py>,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .agent_create(json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("agent_create failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// List all agents
    ///
    /// Returns:
    ///     A dict containing the list of agents
    fn agent_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .agent_list()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("agent_list failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Get an agent by ID
    ///
    /// Args:
    ///     id: Agent ID
    ///
    /// Returns:
    ///     The agent as a dict
    fn agent_get<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .agent_get(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("agent_get failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Get an agent by name
    ///
    /// Args:
    ///     name: Agent name
    ///
    /// Returns:
    ///     The agent as a dict
    fn agent_get_by_name<'py>(&self, py: Python<'py>, name: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .agent_get_by_name(&name)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("agent_get_by_name failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Update an existing agent
    ///
    /// Args:
    ///     id: Agent ID
    ///     data: Updated agent fields as a dict
    ///
    /// Returns:
    ///     The updated agent as a dict
    fn agent_update<'py>(
        &self,
        py: Python<'py>,
        id: String,
        data: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let json_value = py_to_json(data.as_any())?;

        future_into_py(py, async move {
            let result = client
                .agent_update(&id, json_value)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("agent_update failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }

    /// Delete an agent by ID
    ///
    /// Args:
    ///     id: Agent ID
    fn agent_delete<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            client
                .agent_delete(&id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("agent_delete failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Get agents by deployment ID
    ///
    /// Args:
    ///     deployment_id: Deployment ID
    ///
    /// Returns:
    ///     A dict containing agents for the deployment
    fn agents_by_deployment<'py>(&self, py: Python<'py>, deployment_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        future_into_py(py, async move {
            let result = client
                .agents_by_deployment(&deployment_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("agents_by_deployment failed: {}", e)))?;

            Python::attach(|py| json_to_pydict(py, &result))
        })
    }
}

/// Python wrapper for WebSocket Client
#[pyclass]
struct WebSocketClient {
    inner: Option<RustWebSocketClient>,
}

/// Python wrapper for subscription receiver
#[pyclass]
struct SubscriptionReceiver {
    inner: std::sync::Arc<tokio::sync::Mutex<tokio::sync::mpsc::Receiver<ekodb_client::websocket::MutationNotificationPayload>>>,
}

/// Python wrapper for chat stream receiver
#[pyclass]
struct ChatStreamReceiver {
    inner: std::sync::Arc<tokio::sync::Mutex<tokio::sync::mpsc::Receiver<ekodb_client::websocket::ChatStreamEvent>>>,
}

#[pymethods]
impl SubscriptionReceiver {
    /// Receive the next mutation notification. Returns None when the subscription ends.
    fn recv<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rx = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let mut guard = rx.lock().await;
            match guard.recv().await {
                Some(notification) => {
                    Python::attach(|py| {
                        let dict = PyDict::new(py);
                        dict.set_item("collection", &notification.collection)?;
                        dict.set_item("event", &notification.event)?;
                        let ids = PyList::empty(py);
                        for id in &notification.record_ids {
                            ids.append(id)?;
                        }
                        dict.set_item("record_ids", ids)?;
                        dict.set_item("timestamp", &notification.timestamp)?;
                        if let Some(ref records) = notification.records {
                            dict.set_item("records", json_to_pydict(py, records)?)?;
                        }
                        Ok(dict.into())
                    })
                }
                None => {
                    Python::attach(|py| Ok(py.None()))
                }
            }
        })
    }
}

#[pymethods]
impl ChatStreamReceiver {
    /// Receive the next chat stream event. Returns None when the stream ends.
    fn recv<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rx = self.inner.clone();

        future_into_py::<_, Py<PyAny>>(py, async move {
            let mut guard = rx.lock().await;
            match guard.recv().await {
                Some(event) => {
                    Python::attach(|py| {
                        let dict = PyDict::new(py);
                        match &event {
                            ekodb_client::websocket::ChatStreamEvent::Chunk(content) => {
                                dict.set_item("type", "chunk")?;
                                dict.set_item("content", content)?;
                            }
                            ekodb_client::websocket::ChatStreamEvent::End {
                                message_id,
                                token_usage,
                                tool_call_history,
                                execution_time_ms,
                            } => {
                                dict.set_item("type", "end")?;
                                dict.set_item("message_id", message_id)?;
                                dict.set_item("execution_time_ms", *execution_time_ms)?;
                                if let Some(ref tu) = token_usage {
                                    dict.set_item("token_usage", json_to_pydict(py, tu)?)?;
                                }
                                if let Some(ref tch) = tool_call_history {
                                    dict.set_item("tool_call_history", json_to_pydict(py, tch)?)?;
                                }
                            }
                            ekodb_client::websocket::ChatStreamEvent::ToolCall {
                                call_id,
                                tool_name,
                                arguments,
                            } => {
                                dict.set_item("type", "tool_call")?;
                                dict.set_item("call_id", call_id)?;
                                dict.set_item("tool_name", tool_name)?;
                                dict.set_item("arguments", json_to_pydict(py, arguments)?)?;
                            }
                            ekodb_client::websocket::ChatStreamEvent::Error(error) => {
                                dict.set_item("type", "error")?;
                                dict.set_item("error", error)?;
                            }
                        }
                        Ok(dict.into())
                    })
                }
                None => {
                    Python::attach(|py| Ok(py.None()))
                }
            }
        })
    }
}

#[pymethods]
impl WebSocketClient {
    /// Find all records in a collection via WebSocket
    fn find_all<'py>(
        &self,
        py: Python<'py>,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
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

    /// Subscribe to mutation notifications on a collection.
    /// Returns a SubscriptionReceiver for receiving events.
    #[pyo3(signature = (collection, filter_field=None, filter_value=None))]
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        collection: String,
        filter_field: Option<String>,
        filter_value: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let ws_client = match &self.inner {
            Some(client) => client.clone(),
            None => return Err(PyRuntimeError::new_err("WebSocket client not initialized")),
        };

        future_into_py::<_, Py<PyAny>>(py, async move {
            let rx = ws_client
                .subscribe(
                    &collection,
                    filter_field.as_deref(),
                    filter_value.as_deref(),
                )
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Subscribe failed: {}", e)))?;

            Python::attach(|py| {
                let receiver = SubscriptionReceiver {
                    inner: std::sync::Arc::new(tokio::sync::Mutex::new(rx)),
                };
                Ok(Py::new(py, receiver)?.into())
            })
        })
    }

    /// Send a chat message and receive streaming responses.
    /// Returns a ChatStreamReceiver for receiving events.
    #[pyo3(signature = (chat_id, message, client_tools=None, max_iterations=None, confirm_tools=None, exclude_tools=None))]
    fn chat_send<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        message: String,
        client_tools: Option<Vec<(String, String, Py<PyAny>)>>,
        max_iterations: Option<u32>,
        confirm_tools: Option<Vec<String>>,
        exclude_tools: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let ws_client = match &self.inner {
            Some(client) => client.clone(),
            None => return Err(PyRuntimeError::new_err("WebSocket client not initialized")),
        };

        // Convert client tools from Python tuples to Rust structs
        let rust_tools: Option<Vec<ekodb_client::websocket::ClientToolDefinition>> = match client_tools {
            Some(tools) => {
                let mut converted = Vec::with_capacity(tools.len());
                for (name, description, params) in tools {
                    let parameters = Python::attach(|py| {
                        let params_bound = params.bind(py);
                        py_to_json(params_bound)
                    }).map_err(|e| PyRuntimeError::new_err(format!("Failed to convert tool parameters for '{}': {}", name, e)))?;
                    converted.push(ekodb_client::websocket::ClientToolDefinition {
                        name,
                        description,
                        parameters,
                    });
                }
                Some(converted)
            }
            None => None,
        };

        future_into_py::<_, Py<PyAny>>(py, async move {
            let rx = ws_client
                .chat_send_with_tools(
                    &chat_id,
                    &message,
                    rust_tools,
                    max_iterations,
                    confirm_tools,
                    exclude_tools,
                )
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Chat send failed: {}", e)))?;

            Python::attach(|py| {
                let receiver = ChatStreamReceiver {
                    inner: std::sync::Arc::new(tokio::sync::Mutex::new(rx)),
                };
                Ok(Py::new(py, receiver)?.into())
            })
        })
    }

    /// Register client-side tools for a chat session.
    fn register_client_tools<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        tools: Vec<(String, String, Py<PyAny>)>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let ws_client = match &self.inner {
            Some(client) => client.clone(),
            None => return Err(PyRuntimeError::new_err("WebSocket client not initialized")),
        };

        let mut rust_tools: Vec<ekodb_client::websocket::ClientToolDefinition> = Vec::with_capacity(tools.len());
        for (name, description, params) in tools {
            let parameters = Python::attach(|py| {
                let params_bound = params.bind(py);
                py_to_json(params_bound)
            }).map_err(|e| PyRuntimeError::new_err(format!("Failed to convert tool parameters for '{}': {}", name, e)))?;
            rust_tools.push(ekodb_client::websocket::ClientToolDefinition {
                name,
                description,
                parameters,
            });
        }

        future_into_py::<_, Py<PyAny>>(py, async move {
            ws_client
                .register_client_tools(&chat_id, rust_tools)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Register tools failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
        })
    }

    /// Send a tool result back to the server during a chat stream.
    #[pyo3(signature = (chat_id, call_id, success, result=None, error=None))]
    fn send_tool_result<'py>(
        &self,
        py: Python<'py>,
        chat_id: String,
        call_id: String,
        success: bool,
        result: Option<Py<PyAny>>,
        error: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let ws_client = match &self.inner {
            Some(client) => client.clone(),
            None => return Err(PyRuntimeError::new_err("WebSocket client not initialized")),
        };

        let json_result: Option<serde_json::Value> = match result {
            Some(r) => {
                let val = Python::attach(|py| {
                    let r_bound = r.bind(py);
                    py_to_json(r_bound)
                }).map_err(|e| PyRuntimeError::new_err(format!("Failed to convert tool result: {}", e)))?;
                Some(val)
            }
            None => None,
        };

        future_into_py::<_, Py<PyAny>>(py, async move {
            ws_client
                .send_tool_result(&chat_id, &call_id, success, json_result, error)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Send tool result failed: {}", e)))?;

            Python::attach(|py| Ok(py.None()))
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
        record.insert(key_str, field_value);
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

    for (key, value) in record.iter() {
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
    m.add_class::<SubscriptionReceiver>()?;
    m.add_class::<ChatStreamReceiver>()?;
    m.add_class::<RateLimitInfo>()?;
    m.add_class::<SerializationFormat>()?;
    m.add("RateLimitError", m.py().get_type::<RateLimitError>())?;
    Ok(())
}
