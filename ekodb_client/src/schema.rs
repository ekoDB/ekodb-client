//! Schema management for collections
//!
//! This module provides types and utilities for defining and managing
//! collection schemas with field types, constraints, and indexes.

use crate::types::FieldType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Index configuration for a field
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum IndexConfig {
    /// Full-text search index with inverted index
    Text {
        #[serde(default = "default_language")]
        language: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        analyzer: Option<String>,
    },
    /// Vector similarity search index (HNSW)
    Vector {
        #[serde(default = "default_vector_algorithm")]
        algorithm: VectorIndexAlgorithm,
        #[serde(default = "default_distance_metric")]
        metric: DistanceMetric,
        #[serde(default = "default_hnsw_m")]
        m: usize,
        #[serde(default = "default_hnsw_ef_construction")]
        ef_construction: usize,
    },
    /// B-tree index for range queries and exact matches
    BTree,
    /// Hash index for exact matches only (faster than BTree)
    Hash,
}

fn default_language() -> String {
    "english".to_string()
}

fn default_vector_algorithm() -> VectorIndexAlgorithm {
    VectorIndexAlgorithm::Flat
}

fn default_distance_metric() -> DistanceMetric {
    DistanceMetric::Cosine
}

fn default_hnsw_m() -> usize {
    16
}

fn default_hnsw_ef_construction() -> usize {
    200
}

/// Vector index algorithm
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VectorIndexAlgorithm {
    /// Simple flat index (brute force)
    Flat,
    /// Hierarchical Navigable Small World
    HNSW,
    /// Inverted File Index (for future)
    IVF,
}

/// Distance metric for vector similarity
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    DotProduct,
}

/// Field type schema with constraints
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FieldTypeSchema {
    /// Field type (e.g., "string", "number", "boolean")
    pub field_type: String,

    /// Default value for the field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<FieldType>,

    /// Whether the field must be unique across records
    #[serde(default)]
    pub unique: bool,

    /// Whether the field is required
    #[serde(default)]
    pub required: bool,

    /// Allowed enum values
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<FieldType>,

    /// Maximum value (for numbers/dates)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<FieldType>,

    /// Minimum value (for numbers/dates)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<FieldType>,

    /// Regex pattern for string validation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    /// Index configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<IndexConfig>,
}

impl FieldTypeSchema {
    /// Create a new field schema with just the type
    pub fn new(field_type: impl Into<String>) -> Self {
        Self {
            field_type: field_type.into(),
            default: None,
            unique: false,
            required: false,
            enums: Vec::new(),
            max: None,
            min: None,
            regex: None,
            index: None,
        }
    }

    /// Set the field as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Set the field as unique
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    /// Set a default value
    pub fn default_value(mut self, value: FieldType) -> Self {
        self.default = Some(value);
        self
    }

    /// Set enum values
    pub fn enums(mut self, values: Vec<FieldType>) -> Self {
        self.enums = values;
        self
    }

    /// Set min/max range
    pub fn range(mut self, min: Option<FieldType>, max: Option<FieldType>) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Set regex pattern
    pub fn pattern(mut self, regex: impl Into<String>) -> Self {
        self.regex = Some(regex.into());
        self
    }

    /// Add an index
    pub fn with_index(mut self, index: IndexConfig) -> Self {
        self.index = Some(index);
        self
    }
}

/// Collection schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Field definitions
    pub fields: HashMap<String, FieldTypeSchema>,

    /// Schema version
    #[serde(default)]
    pub version: u32,

    /// Creation timestamp
    #[serde(default)]
    pub created_at: DateTime<Utc>,

    /// Last modification timestamp
    #[serde(default)]
    pub last_modified: DateTime<Utc>,

    /// Whether to bypass ripple replication
    #[serde(default)]
    pub bypass_ripple: bool,
}

impl Schema {
    /// Create a new empty schema
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            version: 1,
            created_at: Utc::now(),
            last_modified: Utc::now(),
            bypass_ripple: true,
        }
    }

    /// Add a field to the schema
    pub fn add_field(mut self, name: impl Into<String>, field: FieldTypeSchema) -> Self {
        self.fields.insert(name.into(), field);
        self
    }

    /// Set bypass_ripple flag
    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = bypass;
        self
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}

/// Collection metadata with analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMetadata {
    /// Schema definition
    pub collection: Schema,

    /// Analytics data (if available)
    #[serde(default)]
    pub analytics: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type_schema_builder() {
        let field = FieldTypeSchema::new("string")
            .required()
            .unique()
            .pattern("^[a-z]+$");

        assert_eq!(field.field_type, "string");
        assert!(field.required);
        assert!(field.unique);
        assert_eq!(field.regex, Some("^[a-z]+$".to_string()));
    }

    #[test]
    fn test_field_with_default_value() {
        let field = FieldTypeSchema::new("number").default_value(FieldType::Integer(0));

        assert_eq!(field.default, Some(FieldType::Integer(0)));
    }

    #[test]
    fn test_field_with_enums() {
        let field = FieldTypeSchema::new("string").enums(vec![
            FieldType::String("active".to_string()),
            FieldType::String("inactive".to_string()),
        ]);

        assert_eq!(field.enums.len(), 2);
    }

    #[test]
    fn test_field_with_range() {
        let field = FieldTypeSchema::new("number")
            .range(Some(FieldType::Integer(0)), Some(FieldType::Integer(100)));

        assert_eq!(field.min, Some(FieldType::Integer(0)));
        assert_eq!(field.max, Some(FieldType::Integer(100)));
    }

    #[test]
    fn test_field_with_text_index() {
        let field = FieldTypeSchema::new("string").with_index(IndexConfig::Text {
            language: "english".to_string(),
            analyzer: None,
        });

        assert!(field.index.is_some());
        match field.index.unwrap() {
            IndexConfig::Text { language, .. } => {
                assert_eq!(language, "english");
            }
            _ => panic!("Expected Text index"),
        }
    }

    #[test]
    fn test_field_with_vector_index() {
        let field = FieldTypeSchema::new("vector").with_index(IndexConfig::Vector {
            algorithm: VectorIndexAlgorithm::HNSW,
            metric: DistanceMetric::Cosine,
            m: 16,
            ef_construction: 200,
        });

        assert!(field.index.is_some());
        match field.index.unwrap() {
            IndexConfig::Vector {
                algorithm,
                metric,
                m,
                ef_construction,
            } => {
                assert_eq!(algorithm, VectorIndexAlgorithm::HNSW);
                assert_eq!(metric, DistanceMetric::Cosine);
                assert_eq!(m, 16);
                assert_eq!(ef_construction, 200);
            }
            _ => panic!("Expected Vector index"),
        }
    }

    #[test]
    fn test_schema_builder() {
        let schema = Schema::new()
            .add_field("name", FieldTypeSchema::new("string").required())
            .add_field("email", FieldTypeSchema::new("string").unique())
            .add_field("age", FieldTypeSchema::new("number"))
            .bypass_ripple(false);

        assert_eq!(schema.fields.len(), 3);
        assert!(schema.fields.contains_key("name"));
        assert!(schema.fields.contains_key("email"));
        assert!(schema.fields.contains_key("age"));
        assert_eq!(schema.bypass_ripple, false);
    }

    #[test]
    fn test_schema_default() {
        let schema = Schema::default();
        assert_eq!(schema.fields.len(), 0);
        assert_eq!(schema.version, 1);
        assert!(schema.bypass_ripple);
    }

    #[test]
    fn test_schema_serialization() {
        let schema = Schema::new().add_field("name", FieldTypeSchema::new("string").required());

        let json = serde_json::to_value(&schema).unwrap();
        assert!(json["fields"]["name"]["required"].as_bool().unwrap());
        assert_eq!(json["fields"]["name"]["field_type"], "string");
    }

    #[test]
    fn test_index_config_serialization() {
        let index = IndexConfig::Text {
            language: "english".to_string(),
            analyzer: None,
        };

        let json = serde_json::to_value(&index).unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["language"], "english");
    }

    #[test]
    fn test_vector_index_algorithm() {
        let flat = VectorIndexAlgorithm::Flat;
        let hnsw = VectorIndexAlgorithm::HNSW;
        let ivf = VectorIndexAlgorithm::IVF;

        let json_flat = serde_json::to_value(&flat).unwrap();
        let json_hnsw = serde_json::to_value(&hnsw).unwrap();
        let json_ivf = serde_json::to_value(&ivf).unwrap();

        assert_eq!(json_flat, "flat");
        assert_eq!(json_hnsw, "hnsw");
        assert_eq!(json_ivf, "ivf");
    }

    #[test]
    fn test_distance_metric() {
        let cosine = DistanceMetric::Cosine;
        let euclidean = DistanceMetric::Euclidean;
        let dot = DistanceMetric::DotProduct;

        let json_cosine = serde_json::to_value(&cosine).unwrap();
        let json_euclidean = serde_json::to_value(&euclidean).unwrap();
        let json_dot = serde_json::to_value(&dot).unwrap();

        assert_eq!(json_cosine, "cosine");
        assert_eq!(json_euclidean, "euclidean");
        assert_eq!(json_dot, "dotproduct");
    }
}
