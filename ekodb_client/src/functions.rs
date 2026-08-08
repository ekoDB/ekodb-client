//! Functions API for ekoDB client
//!
//! This module provides types and methods for working with functions,
//! allowing you to create, manage, and execute server-side sequences of Functions.

use crate::types::FieldType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Build the structural placeholder `{"type": "Parameter", "name": <name>}`
/// that ekoDB's `resolve_json_parameters` recognizes inside
/// `Function::Insert.record`, `Function::Update.updates`,
/// `Function::UpdateById.updates`, `Function::FindOneAndUpdate.updates`,
/// `Function::BatchInsert.records`, and any `QueryExpression` filter value.
///
/// This is the structural alternative to the text-level `"{{name}}"`
/// placeholder form — use it when the parameter is a whole-object record or
/// a value whose type would be lost on a raw-JSON round-trip (Binary,
/// DateTime, UUID, Decimal, Duration, Number, Set, Vector). Requires
/// ekoDB >= 0.41.0 for the mutation-stage parameter-resolution
/// consistency fix.
///
/// # Example
///
/// ```
/// use ekodb_client::{parameter_ref, Function, UserFunction};
///
/// let create = UserFunction::new("items_create", "Create item")
///     .with_function(Function::Insert {
///         collection: "items".to_string(),
///         record: parameter_ref("record"),
///         bypass_ripple: None,
///         ttl: None,
///     });
/// ```
pub fn parameter_ref(name: impl Into<String>) -> serde_json::Value {
    serde_json::json!({
        "type": "Parameter",
        "name": name.into(),
    })
}

/// A reusable sequence of Functions stored in ekoDB.
/// Called by label via the `call_function` chat tool or REST API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFunction {
    /// Unique identifier (ekoDB-generated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// User-defined label (unique identifier)
    pub label: String,

    /// Human-readable name
    pub name: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Version string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Parameter definitions (keyed by parameter name)
    #[serde(default)]
    pub parameters: HashMap<String, ParameterDefinition>,

    /// Functions to execute in sequence
    pub functions: Vec<Function>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Creation timestamp (server-managed, read-only)
    #[serde(skip_serializing)]
    pub created_at: Option<DateTime<Utc>>,

    /// Last update timestamp (server-managed, read-only)
    #[serde(skip_serializing)]
    pub updated_at: Option<DateTime<Utc>>,

    /// REST method this function answers (`"GET"`, `"POST"`, etc.).
    /// Pair with `http_path` to expose the function under the
    /// path-routed dispatcher. Requires ekoDB >= 0.42.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,

    /// REST path pattern (e.g. `/users/:id`). Path segments
    /// prefixed with `:` are extracted into the function's params
    /// map. Requires ekoDB >= 0.42.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_path: Option<String>,
}

impl UserFunction {
    /// Create a new UserFunction
    pub fn new(label: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: None,
            label: label.into(),
            name: name.into(),
            description: None,
            version: None,
            parameters: HashMap::new(),
            functions: Vec::new(),
            tags: Vec::new(),
            created_at: None,
            updated_at: None,
            http_method: None,
            http_path: None,
        }
    }

    /// Expose this function under the REST path-router. `method` is
    /// e.g. `"GET"` / `"POST"`; `path` is a pattern like
    /// `"/users/:id"` (path segments starting with `:` become params
    /// at call time). Requires ekoDB >= 0.42.0.
    pub fn with_http_route(mut self, method: impl Into<String>, path: impl Into<String>) -> Self {
        self.http_method = Some(method.into());
        self.http_path = Some(path.into());
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Add a parameter definition
    pub fn with_parameter(mut self, param: ParameterDefinition) -> Self {
        self.parameters.insert(param.name.clone(), param);
        self
    }

    /// Add a Function
    pub fn with_function(mut self, function: Function) -> Self {
        self.functions.push(function);
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// Parameter definition for a function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    /// Parameter name (used as key in HashMap, not serialized)
    #[serde(skip_serializing, default)]
    pub name: String,

    /// Whether this parameter is required
    #[serde(default)]
    pub required: bool,

    /// Default value if not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<FieldType>,

    /// Parameter description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ParameterDefinition {
    /// Create a new parameter definition
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: false,
            default: None,
            description: None,
        }
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Set default value
    pub fn with_default(mut self, default: FieldType) -> Self {
        self.default = Some(default);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Condition evaluation for function control flow (If statements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum FunctionCondition {
    /// Check if field equals value in current records
    FieldEquals {
        field: String,
        value: serde_json::Value,
    },
    /// Check if field exists in current records
    FieldExists { field: String },
    /// Check if we have any records
    HasRecords,
    /// Check if record count equals N
    CountEquals { count: usize },
    /// Check if record count > N
    CountGreaterThan { count: usize },
    /// Check if record count < N
    CountLessThan { count: usize },
    /// Logical AND
    And { conditions: Vec<FunctionCondition> },
    /// Logical OR
    Or { conditions: Vec<FunctionCondition> },
    /// Logical NOT
    Not { condition: Box<FunctionCondition> },
}

/// Function step in a pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
#[allow(
    clippy::enum_variant_names,
    clippy::vec_box,
    clippy::upper_case_acronyms
)]
pub enum Function {
    /// Find all records in collection
    FindAll { collection: String },

    /// Query records with advanced options
    Query {
        collection: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        filter: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sort: Option<Vec<SortFieldConfig>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        skip: Option<serde_json::Value>,
    },

    /// Project specific fields
    Project { fields: Vec<String>, exclude: bool },

    /// Group records with functions
    Group {
        by_fields: Vec<String>,
        functions: Vec<GroupFunctionConfig>,
    },

    /// Count records
    Count { output_field: String },

    /// Find record by ID
    FindById {
        collection: String,
        record_id: String,
    },

    /// Find one record by key/value
    FindOne {
        collection: String,
        key: String,
        value: serde_json::Value,
    },

    /// Insert a record
    Insert {
        collection: String,
        record: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        bypass_ripple: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl: Option<serde_json::Value>,
    },

    /// Update records matching filter
    Update {
        collection: String,
        filter: serde_json::Value,
        updates: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        bypass_ripple: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl: Option<serde_json::Value>,
    },

    /// Update record by ID
    UpdateById {
        collection: String,
        record_id: String,
        updates: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        bypass_ripple: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl: Option<serde_json::Value>,
    },

    /// Find one record and update atomically
    FindOneAndUpdate {
        collection: String,
        filter: serde_json::Value,
        updates: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        bypass_ripple: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl: Option<serde_json::Value>,
    },

    /// Update with actions (increment/decrement)
    UpdateWithAction {
        collection: String,
        filter: serde_json::Value,
        actions: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        bypass_ripple: Option<bool>,
    },

    /// Delete records matching filter
    Delete {
        collection: String,
        filter: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        bypass_ripple: Option<bool>,
    },

    /// Delete record by ID
    DeleteById {
        collection: String,
        record_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bypass_ripple: Option<bool>,
    },

    /// Batch insert records
    BatchInsert {
        collection: String,
        records: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        bypass_ripple: Option<bool>,
    },

    /// Batch delete records
    BatchDelete {
        ids: serde_json::Value,
        #[serde(default)]
        bypass_ripple: bool,
    },

    /// HTTP request
    HttpRequest {
        url: String,
        #[serde(default = "default_method")]
        method: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout_seconds: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_field: Option<String>,
    },

    /// Vector search
    VectorSearch {
        query_vector: Vec<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<serde_json::Value>,
    },

    /// Text search
    TextSearch {
        collection: String,
        query_text: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        fields: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fuzzy: Option<bool>,
    },

    /// Hybrid search (text + vector)
    HybridSearch {
        text_query: String,
        vector_query: Vec<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<serde_json::Value>,
    },

    /// AI Chat completion
    Chat {
        messages: Vec<ChatMessage>,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        temperature: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_tokens: Option<i32>,
    },

    /// Generate embeddings for field in records
    Embed {
        input_field: String,
        output_field: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
    },

    /// Conditional execution
    If {
        condition: FunctionCondition,
        then_functions: Vec<Box<Function>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        else_functions: Option<Vec<Box<Function>>>,
    },

    /// For each record, execute Functions
    ForEach { functions: Vec<Box<Function>> },

    /// Call a saved UserFunction by label
    CallFunction {
        function_label: String,
        params: Option<HashMap<String, serde_json::Value>>,
    },

    /// Create a savepoint for partial rollback
    CreateSavepoint { name: String },

    /// Rollback to a specific savepoint
    RollbackToSavepoint { name: String },

    /// Release a savepoint (no longer needed)
    ReleaseSavepoint { name: String },

    // =========================================================================
    // KV Store Operations
    // =========================================================================
    /// Get a value from the KV store
    /// Returns {value: <data>} on hit, {value: null} on miss
    KvGet { key: serde_json::Value },

    /// Set a value in the KV store
    KvSet {
        key: serde_json::Value,
        value: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl: Option<serde_json::Value>,
    },

    /// Delete a key from the KV store
    KvDelete { key: serde_json::Value },

    /// Check if a key exists in the KV store
    KvExists {
        key: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_field: Option<String>,
    },

    /// Query the KV store with a pattern
    KvQuery {
        #[serde(skip_serializing_if = "Option::is_none")]
        pattern: Option<serde_json::Value>,
        #[serde(default)]
        include_expired: bool,
    },

    /// SWR (Stale-While-Revalidate) pattern for external API caching
    /// Automatically handles: KV cache check → HTTP request → KV cache set → optional audit storage
    SWR {
        cache_key: String,
        ttl: serde_json::Value,
        url: String,
        method: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout_seconds: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_field: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        collection: Option<String>,
    },

    /// Bcrypt-hash a plaintext value and write the result onto every
    /// record in the working data (creates a single result record if the
    /// working set is empty). Use in a compound `users_register` function.
    /// Requires ekoDB >= 0.41.0.
    BcryptHash {
        /// Plaintext to hash, typically `"{{password}}"`.
        plain: String,
        /// bcrypt cost factor (4..=31). Defaults to 12 when None.
        #[serde(skip_serializing_if = "Option::is_none")]
        cost: Option<u32>,
        /// Field name to write the bcrypt hash string into.
        output_field: String,
    },

    /// Verify a plaintext against a bcrypt hash stored on the first
    /// record in the working data; writes a boolean result into
    /// `output_field`. Pair with an `If` stage for login flows.
    /// Requires ekoDB >= 0.41.0.
    BcryptVerify {
        /// Plaintext to verify, typically `"{{password}}"`.
        plain: String,
        /// Name of the field on the current record holding the stored
        /// bcrypt hash (e.g. `"password_hash"`).
        hash_field: String,
        /// Field name to write the boolean result into.
        output_field: String,
    },

    /// Generate a cryptographically-random token and add it to every
    /// record in the working data. Requires ekoDB >= 0.41.0.
    RandomToken {
        /// Number of random bytes (1..=1024).
        bytes: usize,
        /// "hex" | "base64" | "base64url" (default "hex").
        #[serde(skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
        /// Field name to write the encoded token into.
        output_field: String,
    },

    /// Sign a JWT and write the resulting token to every working
    /// record. `claims` is the payload; `iat` and `exp` are
    /// auto-stamped when `expires_in_secs` is set. Pair with
    /// `BcryptVerify` to issue a session token after login. Use
    /// `"{{env.JWT_SECRET}}"` for `secret` so the LLM never sees
    /// the operator-owned signing key. Requires ekoDB >= 0.42.0.
    JwtSign {
        /// JWT payload claims (raw JSON values).
        claims: HashMap<String, serde_json::Value>,
        /// Signing secret. Typically `"{{env.JWT_SECRET}}"`.
        secret: String,
        /// "HS256" | "HS384" | "HS512" (default "HS256").
        #[serde(skip_serializing_if = "Option::is_none")]
        algorithm: Option<String>,
        /// Lifetime in seconds. Auto-stamps `iat` + `exp` when set.
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_in_secs: Option<i64>,
        /// Field name to write the signed JWT into.
        output_field: String,
    },

    /// Verify a JWT held in `token_field` on the first working
    /// record. On success, writes the decoded claims object into
    /// `output_field`. On failure (invalid signature, malformed,
    /// expired, missing token), writes `null`. Branch with
    /// `If { FieldEquals { field: output_field, value: null } }` to
    /// reject. Requires ekoDB >= 0.42.0.
    JwtVerify {
        /// Field on the working record holding the JWT string.
        token_field: String,
        /// Verification secret. Must match the signing secret.
        secret: String,
        /// Expected algorithm (default "HS256").
        #[serde(skip_serializing_if = "Option::is_none")]
        algorithm: Option<String>,
        /// Field name to write the decoded claims object into.
        output_field: String,
    },

    /// Send a transactional email through a provider's REST API.
    /// Today only `provider = "sendgrid"` is supported. Pull the
    /// API key from `{{env.SENDGRID_API_KEY}}` so the LLM never
    /// sees the operator-owned secret. Result envelope
    /// `{provider_status, provider_message, provider}` is written
    /// to `output_field` (defaults to `"email_send"`).
    /// Requires ekoDB >= 0.42.0.
    EmailSend {
        /// Recipient email.
        to: String,
        /// Subject line.
        subject: String,
        /// Plain-text or HTML body (set `html: Some(true)` for HTML).
        body: String,
        /// Sender email (must be verified with the provider).
        from: String,
        /// Optional reply-to header.
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to: Option<String>,
        /// Provider API key (typically `"{{env.SENDGRID_API_KEY}}"`).
        api_key: String,
        /// Provider name. `None` defaults to `"sendgrid"`.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider: Option<String>,
        /// When true, body is sent as `text/html`. Default: false.
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<bool>,
        /// Field name for the result envelope. Defaults to
        /// `"email_send"`.
        #[serde(skip_serializing_if = "Option::is_none")]
        output_field: Option<String>,
    },

    // =========================================================================
    // Crypto Primitives (Hmac, AES, UUID, TOTP, Base64, Hex, Slugify)
    // =========================================================================
    /// HMAC-SHA256/384/512 message authentication. Pair with
    /// `HmacVerify` for inbound webhook signing or pre-signed URL
    /// generation. Requires ekoDB >= 0.42.0.
    HmacSign {
        input: String,
        secret: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        algorithm: Option<String>,
        output_field: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
    },
    /// HMAC verification (constant-time). Writes a boolean.
    HmacVerify {
        input: String,
        provided_mac: String,
        secret: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
        output_field: String,
    },
    /// AES-256-GCM authenticated encryption. Writes
    /// `{ciphertext, nonce}` (both base64) to `output_field`.
    AesEncrypt {
        plaintext: String,
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        key_encoding: Option<String>,
        output_field: String,
    },
    /// AES-256-GCM authenticated decryption. Reads the envelope
    /// from `ciphertext_field`, writes the recovered plaintext or
    /// `null` (fail-closed).
    AesDecrypt {
        ciphertext_field: String,
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        key_encoding: Option<String>,
        output_field: String,
    },
    /// Generate a v4 UUID into `output_field`.
    UuidGenerate { output_field: String },
    /// Generate a TOTP code (RFC 6238) from a base32 secret.
    TotpGenerate {
        secret: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        digits: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        period: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        algorithm: Option<String>,
        output_field: String,
    },
    /// Verify a user-submitted TOTP code; tolerates `skew`
    /// time-steps either side (default 1).
    TotpVerify {
        code: String,
        secret: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        digits: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        period: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        skew: Option<u8>,
        output_field: String,
    },
    /// Base64 encode (`url_safe = Some(true)` for URL-safe / no-pad).
    Base64Encode {
        input: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        url_safe: Option<bool>,
        output_field: String,
    },
    /// Base64 decode → UTF-8 string. Fail-closed.
    Base64Decode {
        input: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        url_safe: Option<bool>,
        output_field: String,
    },
    /// Hex encode (lowercase).
    HexEncode { input: String, output_field: String },
    /// Hex decode → UTF-8 string. Fail-closed.
    HexDecode { input: String, output_field: String },
    /// URL-friendly slug.
    Slugify { input: String, output_field: String },

    // =========================================================================
    // Concurrency Primitives (idempotency / rate limit / lock)
    // =========================================================================
    /// Idempotency-key claim. Atomically claims the key (KV SETNX
    /// with TTL); on first call writes `{claimed: true, key}`, on
    /// replay writes `{claimed: false, key, response}`.
    /// Requires ekoDB >= 0.42.0.
    IdempotencyClaim {
        key: String,
        ttl_secs: u64,
        output_field: String,
    },
    /// Fixed-window rate-limit gate. Increments a counter under
    /// `rate:<key>:<window-floor>`; over-limit either errors
    /// (`on_exceed = "fail"`, default) or writes `allowed: false`
    /// (`on_exceed = "skip"`).
    RateLimit {
        key: String,
        limit: u64,
        window_secs: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        on_exceed: Option<String>,
        output_field: String,
    },
    /// Distributed-lock acquire (token-fenced). On success writes
    /// `{acquired: true, token}`; pass that token to `LockRelease`.
    LockAcquire {
        key: String,
        ttl_secs: u64,
        output_field: String,
    },
    /// Distributed-lock release; only deletes the key when the
    /// stored token matches `token` (prevents foreign release).
    LockRelease {
        key: String,
        token: String,
        output_field: String,
    },

    // =========================================================================
    // Error Handling & Control Flow
    // =========================================================================
    /// Try/Catch error handling for graceful failure recovery.
    /// Executes try_functions, and if any fail, executes catch_functions.
    TryCatch {
        try_functions: Vec<Box<Function>>,
        catch_functions: Vec<Box<Function>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_error_field: Option<String>,
    },

    /// Execute multiple functions in parallel (concurrently).
    /// All functions run simultaneously, results are merged.
    Parallel {
        functions: Vec<Box<Function>>,
        wait_for_all: bool,
    },

    /// Sleep/delay execution for rate limiting or timing control.
    Sleep { duration_ms: serde_json::Value },

    // =========================================================================
    // Response Formatting
    // =========================================================================
    /// Return a shaped response (final output formatting).
    /// Constructs the final response object from current execution context.
    Return {
        fields: HashMap<String, serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status_code: Option<u16>,
    },

    // =========================================================================
    // Data Validation
    // =========================================================================
    /// Validate data against a JSON schema before processing.
    Validate {
        schema: serde_json::Value,
        data_field: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        on_error: Option<Vec<Box<Function>>>,
    },
}

fn default_method() -> String {
    "GET".to_string()
}

/// Chat message for AI operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

/// Group function configuration for Group stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupFunctionConfig {
    pub output_field: String,
    pub operation: GroupFunctionOp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field: Option<String>,
}

impl GroupFunctionConfig {
    /// Create a new group function
    pub fn new(output_field: impl Into<String>, operation: GroupFunctionOp) -> Self {
        Self {
            output_field: output_field.into(),
            operation,
            input_field: None,
        }
    }

    /// Set the input field
    pub fn with_input_field(mut self, field: impl Into<String>) -> Self {
        self.input_field = Some(field.into());
        self
    }
}

/// Group function operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GroupFunctionOp {
    Sum,
    Average,
    Count,
    Min,
    Max,
    First,
    Last,
    Push,
}

/// Sort field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortFieldConfig {
    pub field: String,
    #[serde(default = "default_ascending")]
    pub ascending: bool,
}

impl SortFieldConfig {
    /// Create a new sort field (ascending by default)
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            ascending: true,
        }
    }

    /// Set descending order
    pub fn descending(mut self) -> Self {
        self.ascending = false;
        self
    }

    /// Set ascending order
    pub fn ascending(mut self) -> Self {
        self.ascending = true;
        self
    }
}

fn default_ascending() -> bool {
    true
}

/// Function execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResult {
    /// Resulting records from the pipeline
    pub records: Vec<crate::Record>,

    /// Statistics about the function execution
    pub stats: FunctionStats,
}

/// Statistics about function execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionStats {
    /// Number of input records
    pub input_count: usize,

    /// Number of output records
    pub output_count: usize,

    /// Execution time in milliseconds
    pub execution_time_ms: u128,

    /// Number of stages executed
    pub stages_executed: usize,

    /// Per-stage statistics
    pub stage_stats: Vec<StageStats>,
}

/// Statistics for a single stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageStats {
    /// Stage name
    pub stage: String,

    /// Input count to stage
    pub input_count: usize,

    /// Output count from stage
    pub output_count: usize,

    /// Execution time for this stage in milliseconds
    pub execution_time_ms: u128,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn try_catch_round_trip() {
        let stage = Function::TryCatch {
            try_functions: vec![Box::new(Function::FindAll {
                collection: "users".to_string(),
            })],
            catch_functions: vec![Box::new(Function::Insert {
                collection: "errors".to_string(),
                record: json!({"msg": "failed"}),
                bypass_ripple: None,
                ttl: None,
            })],
            output_error_field: Some("api_error".to_string()),
        };
        let json = serde_json::to_string(&stage).unwrap();
        let parsed: Function = serde_json::from_str(&json).unwrap();
        match parsed {
            Function::TryCatch {
                try_functions,
                catch_functions,
                output_error_field,
            } => {
                assert_eq!(try_functions.len(), 1);
                assert_eq!(catch_functions.len(), 1);
                assert_eq!(output_error_field, Some("api_error".to_string()));
            }
            other => panic!("Expected TryCatch, got {:?}", other),
        }
    }

    #[test]
    fn parallel_round_trip() {
        let stage = Function::Parallel {
            functions: vec![
                Box::new(Function::FindAll {
                    collection: "a".to_string(),
                }),
                Box::new(Function::FindAll {
                    collection: "b".to_string(),
                }),
            ],
            wait_for_all: false,
        };
        let json = serde_json::to_string(&stage).unwrap();
        let parsed: Function = serde_json::from_str(&json).unwrap();
        match parsed {
            Function::Parallel {
                functions,
                wait_for_all,
            } => {
                assert_eq!(functions.len(), 2);
                assert!(!wait_for_all);
            }
            other => panic!("Expected Parallel, got {:?}", other),
        }
    }

    #[test]
    fn sleep_round_trip() {
        let stage = Function::Sleep {
            duration_ms: json!(1000),
        };
        let json = serde_json::to_string(&stage).unwrap();
        let parsed: Function = serde_json::from_str(&json).unwrap();
        match parsed {
            Function::Sleep { duration_ms } => {
                assert_eq!(duration_ms, json!(1000));
            }
            other => panic!("Expected Sleep, got {:?}", other),
        }
    }

    #[test]
    fn sleep_with_placeholder() {
        let stage = Function::Sleep {
            duration_ms: json!("{{delay}}"),
        };
        let json = serde_json::to_string(&stage).unwrap();
        assert!(json.contains("{{delay}}"));
        let parsed: Function = serde_json::from_str(&json).unwrap();
        match parsed {
            Function::Sleep { duration_ms } => {
                assert_eq!(duration_ms, json!("{{delay}}"));
            }
            other => panic!("Expected Sleep, got {:?}", other),
        }
    }

    #[test]
    fn return_round_trip() {
        let mut fields = HashMap::new();
        fields.insert("message".to_string(), json!("ok"));
        fields.insert("user_id".to_string(), json!("{{id}}"));
        let stage = Function::Return {
            fields,
            status_code: Some(201),
        };
        let json = serde_json::to_string(&stage).unwrap();
        let parsed: Function = serde_json::from_str(&json).unwrap();
        match parsed {
            Function::Return {
                fields,
                status_code,
            } => {
                assert_eq!(fields.get("message"), Some(&json!("ok")));
                assert_eq!(status_code, Some(201));
            }
            other => panic!("Expected Return, got {:?}", other),
        }
    }

    #[test]
    fn return_omits_status_code_when_none() {
        let mut fields = HashMap::new();
        fields.insert("success".to_string(), json!(true));
        let stage = Function::Return {
            fields,
            status_code: None,
        };
        let json = serde_json::to_string(&stage).unwrap();
        assert!(!json.contains("status_code"));
    }

    #[test]
    fn validate_round_trip() {
        let stage = Function::Validate {
            schema: json!({"type": "object", "required": ["name"]}),
            data_field: "{{input}}".to_string(),
            on_error: Some(vec![Box::new(Function::FindAll {
                collection: "errors".to_string(),
            })]),
        };
        let json = serde_json::to_string(&stage).unwrap();
        let parsed: Function = serde_json::from_str(&json).unwrap();
        match parsed {
            Function::Validate {
                schema,
                data_field,
                on_error,
            } => {
                assert_eq!(schema, json!({"type": "object", "required": ["name"]}));
                assert_eq!(data_field, "{{input}}");
                assert_eq!(on_error.unwrap().len(), 1);
            }
            other => panic!("Expected Validate, got {:?}", other),
        }
    }

    #[test]
    fn validate_omits_on_error_when_none() {
        let stage = Function::Validate {
            schema: json!({"type": "object"}),
            data_field: "record".to_string(),
            on_error: None,
        };
        let json = serde_json::to_string(&stage).unwrap();
        assert!(!json.contains("on_error"));
    }

    /// Regression test: the server may store functions with newer stage types
    /// (Return, TryCatch, etc.). When the client deserializes a list of
    /// UserFunctions, it must not choke on these variants.
    #[test]
    fn deserialize_user_function_with_return_stage() {
        let json = r#"{
            "id": "abc123",
            "label": "api_handler",
            "name": "API Handler",
            "functions": [
                {"type": "FindAll", "collection": "users"},
                {"type": "Return", "fields": {"data": "{{result}}"}, "status_code": 200}
            ]
        }"#;
        let uf: UserFunction = serde_json::from_str(json).unwrap();
        assert_eq!(uf.functions.len(), 2);
        match &uf.functions[1] {
            Function::Return {
                fields,
                status_code,
            } => {
                assert!(fields.contains_key("data"));
                assert_eq!(*status_code, Some(200));
            }
            other => panic!("Expected Return, got {:?}", other),
        }
    }
}
