//! Saved Functions API for ekoDB client
//!
//! This module provides types and methods for working with saved functions,
//! allowing you to create, manage, and execute server-side data processing pipelines.

use crate::types::FieldType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A saved function definition with pipeline stages and parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedFunction {
    /// User-defined label (unique identifier)
    pub label: String,

    /// Human-readable name
    pub name: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Version string
    #[serde(default = "default_version")]
    pub version: String,

    /// Parameter definitions (keyed by parameter name)
    #[serde(default)]
    pub parameters: HashMap<String, ParameterDefinition>,

    /// Pipeline stages to execute
    pub pipeline: Vec<FunctionStageConfig>,

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

fn default_version() -> String {
    "1.0".to_string()
}

impl SavedFunction {
    /// Create a new saved function
    pub fn new(label: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            name: name.into(),
            description: None,
            version: default_version(),
            parameters: HashMap::new(),
            pipeline: Vec::new(),
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
        self.version = version.into();
        self
    }

    /// Add a parameter definition
    pub fn with_parameter(mut self, param: ParameterDefinition) -> Self {
        self.parameters.insert(param.name.clone(), param);
        self
    }

    /// Add a pipeline stage
    pub fn with_stage(mut self, stage: FunctionStageConfig) -> Self {
        self.pipeline.push(stage);
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

/// Represents a value that can be a literal or a parameter reference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ParameterValue {
    /// Direct value (any FieldType)
    Literal(FieldType),
    /// Parameter reference (e.g., "{{limit}}")
    Parameter(String),
}

impl ParameterValue {
    /// Create a literal value
    pub fn literal(value: FieldType) -> Self {
        ParameterValue::Literal(value)
    }

    /// Create a parameter reference
    pub fn parameter(name: impl Into<String>) -> Self {
        ParameterValue::Parameter(name.into())
    }
}

/// Function pipeline stage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum FunctionStageConfig {
    /// Find all records in collection
    FindAll { collection: String },

    /// Query records with advanced options
    Query {
        collection: String,
        expression: serde_json::Value,
    },

    /// Project specific fields
    Project { fields: Vec<String> },

    /// Group records with functions
    Group {
        by_fields: Vec<String>,
        functions: Vec<GroupFunctionConfig>,
    },

    /// Count records
    Count,

    /// Insert a record
    Insert {
        data: serde_json::Value,
        #[serde(default)]
        bypass_ripple: bool,
    },

    /// Update a record
    Update {
        id: serde_json::Value,
        data: serde_json::Value,
        #[serde(default)]
        bypass_ripple: bool,
    },

    /// Delete a record
    Delete {
        id: serde_json::Value,
        #[serde(default)]
        bypass_ripple: bool,
    },

    /// Batch insert records
    BatchInsert {
        records: serde_json::Value,
        #[serde(default)]
        bypass_ripple: bool,
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
    },

    /// Vector search
    VectorSearch {
        query_vector: Vec<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<serde_json::Value>,
    },

    /// Text search
    TextSearch {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<serde_json::Value>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        system_prompt: Option<String>,
        messages: Vec<ChatMessage>,
        #[serde(skip_serializing_if = "Option::is_none")]
        temperature: Option<f32>,
    },

    /// Generate embeddings
    Embed {
        texts: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
    },

    /// Conditional execution
    If {
        condition: serde_json::Value,
        then_stages: Vec<FunctionStageConfig>,
        #[serde(skip_serializing_if = "Option::is_none")]
        else_stages: Option<Vec<FunctionStageConfig>>,
    },

    /// Loop over items
    ForEach {
        items: serde_json::Value,
        #[serde(rename = "var")]
        variable: String,
        stages: Vec<FunctionStageConfig>,
    },
}

fn default_method() -> String {
    "GET".to_string()
}

/// Chat message for AI operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ParameterValue,
    pub content: ParameterValue,
}

impl ChatMessage {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: ParameterValue::Literal(FieldType::String("system".to_string())),
            content: ParameterValue::Literal(FieldType::String(content.into())),
        }
    }

    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ParameterValue::Literal(FieldType::String("user".to_string())),
            content: ParameterValue::Literal(FieldType::String(content.into())),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ParameterValue::Literal(FieldType::String("assistant".to_string())),
            content: ParameterValue::Literal(FieldType::String(content.into())),
        }
    }

    /// Create a message with parameter references
    pub fn with_params(role: ParameterValue, content: ParameterValue) -> Self {
        Self { role, content }
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
