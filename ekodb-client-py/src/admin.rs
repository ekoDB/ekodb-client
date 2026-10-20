use crate::json_to_pydict;
use ekodb_client::admin::{
    AdminClient as RustAdmin, CreateApiKeyRequest, UpdateApiKeyCollectionPermissionsRequest,
};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
    types::PyDict,
};
use pyo3_async_runtimes::tokio::future_into_py;
use serde_json::Value;
use std::{future::Future, time::Duration};

#[pyclass]
struct AdminClient {
    inner: RustAdmin,
}
fn finish<'py>(
    py: Python<'py>,
    future: impl Future<Output = ekodb_client::Result<Value>> + Send + 'static,
) -> PyResult<Bound<'py, PyAny>> {
    future_into_py(py, async move {
        let value = future
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Python::attach(|py| json_to_pydict(py, &value))
    })
}
fn value<T: serde::Serialize>(v: T) -> ekodb_client::Result<Value> {
    serde_json::to_value(v)
        .map_err(|_| ekodb_client::Error::Validation("Invalid admin response".into()))
}
fn request_json(py: Python<'_>, request: Bound<'_, PyDict>) -> PyResult<Value> {
    let encoded = py
        .import("json")
        .and_then(|module| module.call_method1("dumps", (request,)))
        .and_then(|value| value.extract::<String>())
        .map_err(|_| PyValueError::new_err("Invalid admin request"))?;
    serde_json::from_str(&encoded).map_err(|_| PyValueError::new_err("Invalid admin request"))
}
#[pymethods]
impl AdminClient {
    #[new]
    #[pyo3(signature=(base_url,api_key,timeout_secs=30))]
    fn new(base_url: &str, api_key: &str, timeout_secs: u64) -> PyResult<Self> {
        Ok(Self {
            inner: RustAdmin::with_timeout(base_url, api_key, Duration::from_secs(timeout_secs))
                .map_err(|_| PyValueError::new_err("Invalid admin configuration"))?,
        })
    }
    #[pyo3(signature=(request=None))]
    fn create_api_key<'py>(
        &self,
        py: Python<'py>,
        request: Option<Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let input = match request {
            Some(v) => request_json(py, v)?,
            None => serde_json::json!({}),
        };
        let request: CreateApiKeyRequest = serde_json::from_value(input).map_err(|_| {
            PyValueError::new_err("Invalid admin key request; registration requires action arrays")
        })?;
        let client = self.inner.clone();
        finish(
            py,
            async move { value(client.create_api_key(&request).await?) },
        )
    }
    fn list_api_keys<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        finish(py, async move { value(client.list_api_keys().await?) })
    }
    fn revoke_api_key<'py>(&self, py: Python<'py>, api_key: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        finish(py, async move {
            client.revoke_api_key(&api_key).await?;
            Ok(Value::Null)
        })
    }
    fn get_permissions<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        finish(py, async move { value(client.get_permissions().await?) })
    }
    fn update_api_key_collection_permissions<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let input = request_json(py, request)?;
        let request: UpdateApiKeyCollectionPermissionsRequest = serde_json::from_value(input)
            .map_err(|_| PyValueError::new_err("Invalid collection permission update"))?;
        let client = self.inner.clone();
        finish(py, async move {
            value(
                client
                    .update_api_key_collection_permissions(&request)
                    .await?,
            )
        })
    }
}
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<AdminClient>()?;
    module.add(
        "MIN_SERVER_VERSION_FOR_PERMISSION_EDITS",
        ekodb_client::admin::MIN_SERVER_VERSION_FOR_PERMISSION_EDITS,
    )
}
