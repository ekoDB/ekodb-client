//! API-key administration, separate from data CRUD. Returned records contain secrets.
use crate::{AuthManager, Error, Result};
use reqwest::{Client as HttpClient, Method, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::OnceCell;
use url::Url;

/// Earlier servers erase restrictions when updating a grant.
pub const MIN_SERVER_VERSION_FOR_PERMISSION_EDITS: &str = "0.79.2";
const REQUIREMENT: &str = "Permission updates need ekoDB 0.79.2 or later because earlier servers remove a grant's field restrictions when it is updated";

/// A restriction edit: omit, clear, or replace (including an empty array).
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum PermissionEdit {
    #[default]
    Keep,
    Clear,
    Set(Vec<String>),
}
impl PermissionEdit {
    fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }
    fn valid(&self) -> bool {
        match self {
            Self::Set(fields) => fields.iter().all(|v| !v.trim().is_empty()),
            _ => true,
        }
    }
}
impl Serialize for PermissionEdit {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::Set(fields) => fields.serialize(serializer),
            _ => serializer.serialize_none(),
        }
    }
}
impl<'de> Deserialize<'de> for PermissionEdit {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        Ok(match Option::<Vec<String>>::deserialize(deserializer)? {
            Some(v) => Self::Set(v),
            None => Self::Clear,
        })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyAction {
    Read,
    Write,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default)]
    pub collection_permissions: HashMap<String, Vec<ApiKeyAction>>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateApiKeyCollectionPermissionsRequest {
    pub api_key: String,
    pub collection: String,
    pub read: bool,
    pub write: bool,
    #[serde(default, skip_serializing_if = "PermissionEdit::is_keep")]
    pub select_fields: PermissionEdit,
    #[serde(default, skip_serializing_if = "PermissionEdit::is_keep")]
    pub exclude_fields: PermissionEdit,
    #[serde(default, skip_serializing_if = "PermissionEdit::is_keep")]
    pub writable_fields: PermissionEdit,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct CollectionPermissions {
    pub read: bool,
    pub write: bool,
    pub select_fields: Option<Vec<String>>,
    pub exclude_fields: Option<Vec<String>>,
    pub writable_fields: Option<Vec<String>>,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}
impl<'de> Deserialize<'de> for CollectionPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Grant {
            read: bool,
            write: bool,
            select_fields: Option<Vec<String>>,
            exclude_fields: Option<Vec<String>>,
            writable_fields: Option<Vec<String>>,
            #[serde(flatten)]
            metadata: HashMap<String, Value>,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire {
            Actions(Vec<ApiKeyAction>),
            Grant(Grant),
        }
        Ok(match Wire::deserialize(d)? {
            Wire::Actions(actions) => Self {
                read: actions.contains(&ApiKeyAction::Read),
                write: actions.contains(&ApiKeyAction::Write),
                select_fields: None,
                exclude_fields: None,
                writable_fields: None,
                metadata: HashMap::new(),
            },
            Wire::Grant(g) => Self {
                read: g.read,
                write: g.write,
                select_fields: g.select_fields,
                exclude_fields: g.exclude_fields,
                writable_fields: g.writable_fields,
                metadata: g.metadata,
            },
        })
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ApiKeyPermissions {
    pub is_admin: bool,
    pub collection_permissions: HashMap<String, CollectionPermissions>,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}
/// Contains a secret; never log or print this record.
#[derive(Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub is_admin: bool,
    pub collection_permissions: HashMap<String, CollectionPermissions>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub ripple_warning: Option<String>,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}
#[derive(Clone)]
pub struct AdminClient {
    http: HttpClient,
    auth: Arc<AuthManager>,
    base_url: Url,
    api_key: String,
    version: Arc<OnceCell<Option<String>>>,
}
impl AdminClient {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        Self::with_timeout(base_url, api_key, Duration::from_secs(30))
    }
    pub fn with_timeout(base_url: &str, api_key: &str, timeout: Duration) -> Result<Self> {
        let base_url = Url::parse(base_url)
            .map_err(|_| Error::InvalidConfig("Invalid admin base URL".into()))?;
        if !matches!(base_url.scheme(), "http" | "https")
            || base_url.host_str().is_none()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
            || api_key.trim().is_empty()
            || timeout.is_zero()
        {
            return Err(Error::InvalidConfig("Invalid admin configuration".into()));
        }
        let http = HttpClient::builder()
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| safe_error())?;
        let auth = Arc::new(AuthManager::new(
            api_key.into(),
            base_url.clone(),
            http.clone(),
        ));
        Ok(Self {
            http,
            auth,
            base_url,
            api_key: api_key.into(),
            version: Arc::new(OnceCell::new()),
        })
    }
    async fn send(&self, method: Method, path: &str, body: Option<Value>) -> Result<Value> {
        let url = self.base_url.join(path).map_err(|_| safe_error())?;
        let mut token = self
            .auth
            .get_token()
            .await
            .map_err(|_| Error::Authentication("Admin authentication failed".into()))?;
        for attempt in 0..2 {
            if token.trim().is_empty() {
                return Err(Error::Authentication(
                    "Invalid admin authentication response".into(),
                ));
            }
            let mut request = self
                .http
                .request(method.clone(), url.clone())
                .bearer_auth(&token)
                .header("Accept", "application/json");
            if let Some(value) = &body {
                request = request.json(value);
            }
            let response = request.send().await.map_err(|_| safe_error())?;
            let status = response.status();
            if status == StatusCode::UNAUTHORIZED && attempt == 0 {
                token = self
                    .auth
                    .refresh_token()
                    .await
                    .map_err(|_| Error::Authentication("Admin authentication failed".into()))?;
                continue;
            }
            if !status.is_success() {
                return Err(Error::api(status.as_u16(), "Admin request failed"));
            }
            if status == StatusCode::NO_CONTENT || method == Method::DELETE {
                return Ok(Value::Null);
            }
            return response.json().await.map_err(|_| invalid_response());
        }
        Err(safe_error())
    }
    pub async fn create_api_key(&self, request: &CreateApiKeyRequest) -> Result<ApiKey> {
        let mut body = serde_json::to_value(request).map_err(|_| safe_error())?;
        body["api_key"] = json!(self.api_key);
        parse_key(
            self.send(Method::POST, "/api/auth/register", Some(body))
                .await?,
        )
    }
    pub async fn list_api_keys(&self) -> Result<Vec<ApiKey>> {
        let value = self.send(Method::GET, "/api/auth/keys", None).await?;
        let values = value.as_array().ok_or_else(invalid_response)?;
        values.iter().cloned().map(parse_key).collect()
    }
    pub async fn revoke_api_key(&self, key: &str) -> Result<()> {
        if key.trim().is_empty() {
            return Err(Error::Validation("An API key is required".into()));
        }
        let encoded = key
            .as_bytes()
            .iter()
            .map(|b| {
                if b.is_ascii_alphanumeric() || b"-._~".contains(b) {
                    (*b as char).to_string()
                } else {
                    format!("%{b:02X}")
                }
            })
            .collect::<String>();
        self.send(Method::DELETE, &format!("/api/auth/keys/{encoded}"), None)
            .await?;
        Ok(())
    }
    pub async fn get_permissions(&self) -> Result<ApiKeyPermissions> {
        decode(
            self.send(Method::GET, "/api/auth/permissions", None)
                .await?,
        )
    }
    pub async fn update_api_key_collection_permissions(
        &self,
        request: &UpdateApiKeyCollectionPermissionsRequest,
    ) -> Result<ApiKey> {
        if request.api_key.trim().is_empty()
            || request.collection.trim().is_empty()
            || ![
                &request.select_fields,
                &request.exclude_fields,
                &request.writable_fields,
            ]
            .iter()
            .all(|v| v.valid())
        {
            return Err(Error::Validation(
                "Invalid collection permission update".into(),
            ));
        }
        let version = self
            .version
            .get_or_init(|| async {
                self.send(Method::GET, "/api/health", None)
                    .await
                    .ok()
                    .and_then(|v| v.get("version").and_then(Value::as_str).map(str::to_owned))
            })
            .await;
        if !version.as_deref().is_some_and(supported_version) {
            return Err(Error::Validation(REQUIREMENT.into()));
        }
        let body = serde_json::to_value(request).map_err(|_| safe_error())?;
        parse_key(
            self.send(Method::PUT, "/api/auth/permissions", Some(body))
                .await?,
        )
    }
}
fn version_parts(version: &str) -> Option<Vec<u64>> {
    let stable = version
        .split_once('+')
        .map_or(version, |(stable, _)| stable);
    if let Some((_, build)) = version.split_once('+') {
        if build.split('.').any(|part| {
            part.is_empty() || !part.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        }) {
            return None;
        }
    }
    let mut parts = Vec::new();
    for part in stable.split('.') {
        if part.is_empty()
            || (part.len() > 1 && part.starts_with('0'))
            || !part.bytes().all(|b| b.is_ascii_digit())
        {
            return None;
        }
        let Ok(number) = part.parse::<u64>() else {
            return None;
        };
        parts.push(number);
    }
    if parts.len() == 3 { Some(parts) } else { None }
}
fn supported_version(version: &str) -> bool {
    match (
        version_parts(version),
        version_parts(MIN_SERVER_VERSION_FOR_PERMISSION_EDITS),
    ) {
        (Some(current), Some(minimum)) => current >= minimum,
        _ => false,
    }
}
fn safe_error() -> Error {
    Error::Connection("Admin request failed".into())
}
fn invalid_response() -> Error {
    Error::Validation("Invalid admin response".into())
}
fn decode<T: DeserializeOwned>(value: Value) -> Result<T> {
    serde_json::from_value(value).map_err(|_| invalid_response())
}
fn parse_key(value: Value) -> Result<ApiKey> {
    let key: ApiKey = decode(value)?;
    if key.key.is_empty() {
        return Err(invalid_response());
    }
    Ok(key)
}
