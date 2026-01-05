//! Scripts API for ekoDB client
//!
//! This module provides types and methods for working with Scripts,
//! allowing you to create, manage, and execute server-side sequences of Functions.

use crate::types::FieldType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Script definition with Functions and parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
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

    /// Creation timestamp (server-managed, don't send from client)
    #[serde(skip_serializing, skip_deserializing)]
    pub created_at: Option<DateTime<Utc>>,

    /// Last update timestamp (server-managed, don't send from client)
    #[serde(skip_serializing, skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Script {
    /// Create a new Script
    pub fn new(label: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            name: name.into(),
            description: None,
            version: None,
            parameters: HashMap::new(),
            functions: Vec::new(),
            tags: Vec::new(),
            created_at: None, // Server will set this
            updated_at: None, // Server will set this
        }
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

    /// Add a Function to the Script
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

/// A UserFunction is a reusable sequence of Functions that can be called by Scripts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFunction {
    /// Unique identifier (ekoDB-generated)
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

    /// Parameter definitions
    #[serde(default)]
    pub parameters: HashMap<String, ParameterDefinition>,

    /// Functions to execute in sequence
    pub functions: Vec<Function>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Creation timestamp (server-managed)
    #[serde(skip_serializing, skip_deserializing)]
    pub created_at: Option<DateTime<Utc>>,

    /// Last update timestamp (server-managed)
    #[serde(skip_serializing, skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
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
        }
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

/// Condition evaluation for Script control flow (If statements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ScriptCondition {
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
    And { conditions: Vec<ScriptCondition> },
    /// Logical OR
    Or { conditions: Vec<ScriptCondition> },
    /// Logical NOT
    Not { condition: Box<ScriptCondition> },
}

/// Function in a Script
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
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
        condition: ScriptCondition,
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
    KvGet {
        key: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_field: Option<String>,
    },

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
