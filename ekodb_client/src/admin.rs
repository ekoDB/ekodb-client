//! API-key administration, separate from data CRUD. Returned records contain secrets.
use crate::{AuthManager, Error, Result};
use reqwest::{Client as HttpClient, Method, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::OnceCell;
use url::Url;

/// Earlier servers erase restrictions when updating a grant.
pub const MIN_SERVER_VERSION_FOR_PERMISSION_EDITS: &str = "0.80.0";
pub const MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS: &str = "0.79.2";
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
pub struct LegacyUpdateApiKeyCollectionPermissionsRequest {
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateApiKeyCollectionPermissionsRequest {
    pub key_id: String,
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
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchApiKeyCollectionPermissionsRequest {
    pub key_id: String,
    pub collection: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_boolean"
    )]
    pub read: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_boolean"
    )]
    pub write: Option<bool>,
    #[serde(default, skip_serializing_if = "PermissionEdit::is_keep")]
    pub select_fields: PermissionEdit,
    #[serde(default, skip_serializing_if = "PermissionEdit::is_keep")]
    pub exclude_fields: PermissionEdit,
    #[serde(default, skip_serializing_if = "PermissionEdit::is_keep")]
    pub writable_fields: PermissionEdit,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateApiKeyKvPermissionsRequest {
    pub key_id: String,
    pub prefix: String,
    pub read: bool,
    pub write: bool,
}
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchApiKeyKvPermissionsRequest {
    pub key_id: String,
    pub prefix: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_boolean"
    )]
    pub read: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_boolean"
    )]
    pub write: Option<bool>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct KvPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    pub read: bool,
    pub write: bool,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct PermissionChange<T> {
    #[serde(deserialize_with = "required_nullable")]
    pub before: Option<T>,
    #[serde(deserialize_with = "required_nullable")]
    pub after: Option<T>,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}
pub type CollectionPermissionChange = PermissionChange<CollectionPermissions>;
pub type KvPermissionChange = PermissionChange<KvPermissions>;
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
    #[serde(default)]
    pub kv_permissions: Vec<KvPermissions>,
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}
/// Contains a secret; never log or print this record.
#[derive(Clone, Serialize, Deserialize)]
pub struct ApiKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub key: String,
    pub is_admin: bool,
    pub collection_permissions: HashMap<String, CollectionPermissions>,
    #[serde(default)]
    pub kv_permissions: Option<Vec<KvPermissions>>,
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
    version: Arc<OnceCell<String>>,
    permission_routes: Arc<OnceCell<bool>>,
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
            permission_routes: Arc::new(OnceCell::new()),
        })
    }
    async fn send(&self, method: Method, path: &str, body: Option<Value>) -> Result<Value> {
        self.send_response(method, path, body, false).await
    }
    async fn send_response(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
        read_delete_body: bool,
    ) -> Result<Value> {
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
            if status == StatusCode::NO_CONTENT || (method == Method::DELETE && !read_delete_body) {
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
    async fn modern_permission_routes(&self) -> Result<bool> {
        Ok(*self
            .permission_routes
            .get_or_try_init(|| async {
                match self
                    .send(Method::GET, "/api/auth/keys/me/permissions", None)
                    .await
                {
                    Ok(value) => {
                        parse_permissions(value, true)?;
                        Ok(true)
                    }
                    Err(Error::Api {
                        code: 404 | 405, ..
                    }) => Ok(false),
                    Err(error) => Err(error),
                }
            })
            .await?)
    }
    pub async fn uses_key_permission_routes(&self) -> Result<bool> {
        self.modern_permission_routes().await
    }
    pub async fn get_permissions(&self) -> Result<ApiKeyPermissions> {
        self.get_api_key_permissions("me").await
    }
    pub async fn get_api_key_permissions(&self, key_id: &str) -> Result<ApiKeyPermissions> {
        validate_key_id(key_id)?;
        let modern = self.modern_permission_routes().await?;
        if !modern && key_id != "me" {
            return Err(Error::Validation(
                "Reading another key by id needs modern permission routes".into(),
            ));
        }
        let path = if modern {
            format!("/api/auth/keys/{key_id}/permissions")
        } else {
            "/api/auth/permissions".into()
        };
        parse_permissions(self.send(Method::GET, &path, None).await?, modern)
    }
    async fn permission_path(
        &self,
        key_id: &str,
        facet: &str,
        name: Option<&str>,
    ) -> Result<String> {
        validate_key_id(key_id)?;
        let suffix = match name {
            Some(value) => {
                validate_facet(value)?;
                format!("/{facet}/{}", encode_segment(value))
            }
            None => format!("/{facet}"),
        };
        if !self.modern_permission_routes().await? {
            return Err(Error::Validation(
                "This operation needs modern permission routes".into(),
            ));
        }
        Ok(format!("/api/auth/keys/{key_id}/permissions{suffix}"))
    }
    pub async fn list_api_key_collection_permissions(
        &self,
        key_id: &str,
    ) -> Result<HashMap<String, CollectionPermissions>> {
        let path = self.permission_path(key_id, "collections", None).await?;
        decode(self.send(Method::GET, &path, None).await?)
    }
    pub async fn get_api_key_collection_permissions(
        &self,
        key_id: &str,
        collection: &str,
    ) -> Result<CollectionPermissions> {
        let path = self
            .permission_path(key_id, "collections", Some(collection))
            .await?;
        decode(self.send(Method::GET, &path, None).await?)
    }
    pub async fn list_api_key_kv_permissions(
        &self,
        key_id: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<KvPermissions>> {
        let mut path = self.permission_path(key_id, "kv", None).await?;
        if let Some(prefix) = prefix {
            path.push_str(&format!("?prefix={}", encode_segment(prefix)));
        }
        parse_kv_list(self.send(Method::GET, &path, None).await?)
    }
    pub async fn get_api_key_kv_permissions(
        &self,
        key_id: &str,
        prefix: &str,
    ) -> Result<KvPermissions> {
        let path = self.permission_path(key_id, "kv", Some(prefix)).await?;
        decode(self.send(Method::GET, &path, None).await?)
    }
    async fn collection_change<T: Serialize>(
        &self,
        method: Method,
        key_id: &str,
        collection: &str,
        request: Option<&T>,
    ) -> Result<CollectionPermissionChange> {
        let mut body = request
            .map(serde_json::to_value)
            .transpose()
            .map_err(|_| safe_error())?;
        if let Some(Value::Object(value)) = &mut body {
            value.remove("key_id");
            value.remove("collection");
        }
        let path = self
            .permission_path(key_id, "collections", Some(collection))
            .await?;
        decode(self.send_response(method, &path, body, true).await?)
    }
    pub async fn put_api_key_collection_permissions(
        &self,
        request: &UpdateApiKeyCollectionPermissionsRequest,
    ) -> Result<CollectionPermissionChange> {
        validate_edits([
            &request.select_fields,
            &request.exclude_fields,
            &request.writable_fields,
        ])?;
        self.collection_change(
            Method::PUT,
            &request.key_id,
            &request.collection,
            Some(request),
        )
        .await
    }
    pub async fn update_api_key_collection_permissions(
        &self,
        request: &UpdateApiKeyCollectionPermissionsRequest,
    ) -> Result<CollectionPermissionChange> {
        validate_edits([
            &request.select_fields,
            &request.exclude_fields,
            &request.writable_fields,
        ])?;
        self.collection_change(
            Method::PATCH,
            &request.key_id,
            &request.collection,
            Some(request),
        )
        .await
    }
    pub async fn patch_api_key_collection_permissions(
        &self,
        request: &PatchApiKeyCollectionPermissionsRequest,
    ) -> Result<CollectionPermissionChange> {
        validate_edits([
            &request.select_fields,
            &request.exclude_fields,
            &request.writable_fields,
        ])?;
        self.collection_change(
            Method::PATCH,
            &request.key_id,
            &request.collection,
            Some(request),
        )
        .await
    }
    pub async fn delete_api_key_collection_permissions(
        &self,
        key_id: &str,
        collection: &str,
    ) -> Result<CollectionPermissionChange> {
        self.collection_change::<Value>(Method::DELETE, key_id, collection, None)
            .await
    }
    async fn kv_change<T: Serialize>(
        &self,
        method: Method,
        key_id: &str,
        prefix: &str,
        request: Option<&T>,
    ) -> Result<KvPermissionChange> {
        let mut body = request
            .map(serde_json::to_value)
            .transpose()
            .map_err(|_| safe_error())?;
        if let Some(Value::Object(value)) = &mut body {
            value.remove("key_id");
            value.remove("prefix");
        }
        let path = self.permission_path(key_id, "kv", Some(prefix)).await?;
        decode(self.send_response(method, &path, body, true).await?)
    }
    pub async fn put_api_key_kv_permissions(
        &self,
        request: &UpdateApiKeyKvPermissionsRequest,
    ) -> Result<KvPermissionChange> {
        if !request.read && !request.write {
            return Err(Error::Validation(
                "A KV grant needs read or write permission".into(),
            ));
        }
        self.kv_change(Method::PUT, &request.key_id, &request.prefix, Some(request))
            .await
    }
    pub async fn patch_api_key_kv_permissions(
        &self,
        request: &PatchApiKeyKvPermissionsRequest,
    ) -> Result<KvPermissionChange> {
        self.kv_change(
            Method::PATCH,
            &request.key_id,
            &request.prefix,
            Some(request),
        )
        .await
    }
    pub async fn delete_api_key_kv_permissions(
        &self,
        key_id: &str,
        prefix: &str,
    ) -> Result<KvPermissionChange> {
        self.kv_change::<Value>(Method::DELETE, key_id, prefix, None)
            .await
    }
    pub async fn legacy_update_api_key_collection_permissions(
        &self,
        request: &LegacyUpdateApiKeyCollectionPermissionsRequest,
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
        if self.modern_permission_routes().await? {
            return Err(Error::Validation(
                "Legacy permission edits are unavailable on this server".into(),
            ));
        }
        let version = self
            .version
            .get_or_try_init(|| async {
                let health = self.send(Method::GET, "/api/health", None).await?;
                let version = health
                    .get("version")
                    .and_then(Value::as_str)
                    .filter(|version| version_parts(version).is_some())
                    .ok_or_else(|| Error::Validation(REQUIREMENT.into()))?;
                Ok::<_, Error>(version.to_owned())
            })
            .await?;
        if !supported_version(version) {
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
        version_parts(MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS),
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
    if key.key.is_empty()
        || key
            .id
            .as_deref()
            .is_some_and(|id| id == "me" || validate_key_id(id).is_err())
    {
        return Err(invalid_response());
    }
    Ok(key)
}

fn validate_key_id(key_id: &str) -> Result<()> {
    if key_id == "me"
        || (key_id.len() == 32
            && key_id
                .bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)))
    {
        return Ok(());
    }
    Err(Error::Validation(
        "A key id or me is required; key values are not accepted".into(),
    ))
}
fn required_nullable<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> std::result::Result<Option<T>, D::Error> {
    Option::<T>::deserialize(deserializer)
}
fn validate_facet(value: &str) -> Result<()> {
    if value.is_empty()
        || matches!(value, "." | "..")
        || value.chars().any(|c| {
            c.is_control()
                || matches!(c,
                    '\u{00ad}' | '\u{061c}' | '\u{180e}'
                    | '\u{200b}'..='\u{200f}'
                    | '\u{2028}'..='\u{202e}'
                    | '\u{2060}'..='\u{2064}'
                    | '\u{2066}'..='\u{2069}'
                    | '\u{feff}'
                )
        })
    {
        return Err(Error::Validation("Invalid permission facet".into()));
    }
    Ok(())
}
fn encode_segment(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(b) {
                (*b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn validate_edits(edits: [&PermissionEdit; 3]) -> Result<()> {
    if edits.iter().all(|edit| edit.valid()) {
        Ok(())
    } else {
        Err(Error::Validation(
            "Invalid collection permission update".into(),
        ))
    }
}
fn parse_permissions(mut value: Value, modern: bool) -> Result<ApiKeyPermissions> {
    if !modern && value.get("kv_permissions") == Some(&Value::Null) {
        value["kv_permissions"] = json!([]);
    }
    if modern {
        let record = value.as_object_mut().ok_or_else(invalid_response)?;
        let admin = record.remove("admin").ok_or_else(invalid_response)?;
        let collections = record.remove("collections").ok_or_else(invalid_response)?;
        let kv = record.remove("kv").ok_or_else(invalid_response)?;
        record.insert("is_admin".into(), admin);
        record.insert("collection_permissions".into(), collections);
        record.insert("kv_permissions".into(), kv);
    }
    let result: ApiKeyPermissions = decode(value)?;
    if result
        .kv_permissions
        .iter()
        .any(|grant| grant.prefix.as_deref().is_none_or(str::is_empty))
    {
        return Err(invalid_response());
    }
    Ok(result)
}
fn parse_kv_list(value: Value) -> Result<Vec<KvPermissions>> {
    let grants: Vec<KvPermissions> = decode(value)?;
    if grants
        .iter()
        .any(|grant| grant.prefix.as_deref().is_none_or(str::is_empty))
    {
        return Err(invalid_response());
    }
    Ok(grants)
}

fn optional_boolean<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<bool>, D::Error> {
    bool::deserialize(deserializer).map(Some)
}
