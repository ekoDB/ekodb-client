//! Full-text and vector search support for ekoDB
//!
//! This module provides comprehensive search capabilities including:
//! - Full-text search with fuzzy matching
//! - Vector/semantic search
//! - Hybrid search (text + vector)
//! - Field weighting and boosting

use serde::{Deserialize, Serialize};

/// Helper function to deserialize fields from either String or Vec<String>
fn deserialize_fields<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value: Option<Value> = Option::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(Value::String(s)) => Ok(Some(s)),
        Some(Value::Array(arr)) => {
            let strings: Result<Vec<String>, _> = arr
                .into_iter()
                .map(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .ok_or_else(|| Error::custom("Array elements must be strings"))
                })
                .collect();
            Ok(Some(strings?.join(",")))
        }
        Some(_) => Err(Error::custom("fields must be a string or array of strings")),
    }
}

/// Helper function to deserialize weights from either String or Object
fn deserialize_weights<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value: Option<Value> = Option::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(Value::String(s)) => Ok(Some(s)),
        Some(Value::Object(map)) => {
            // Convert object to "field:weight,field2:weight2" format
            let weights: Vec<String> = map
                .into_iter()
                .filter_map(|(key, val)| val.as_f64().map(|weight| format!("{}:{}", key, weight)))
                .collect();

            if weights.is_empty() {
                return Err(Error::custom("weights object must contain numeric values"));
            }

            Ok(Some(weights.join(",")))
        }
        Some(_) => Err(Error::custom(
            "weights must be a string or object with numeric values",
        )),
    }
}

/// Search query for full-text and vector search
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchQuery {
    /// Search query string
    pub query: String,

    /// Language for stemming (e.g., "english", "spanish", "french")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Case-sensitive search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,

    /// Enable fuzzy matching (typo tolerance)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<bool>,

    /// Minimum score threshold (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,

    /// Fields to search in (comma-separated or array)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_fields"
    )]
    pub fields: Option<String>,

    /// Field weights (format: "field1:2.0,field2:1.5" or object)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_weights"
    )]
    pub weights: Option<String>,

    /// Enable stemming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_stemming: Option<bool>,

    /// Boost exact matches
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_exact: Option<bool>,

    /// Maximum edit distance for fuzzy matching (0-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_edit_distance: Option<u32>,

    /// Bypass ripple cache
    #[serde(default)]
    pub bypass_ripple: Option<bool>,

    /// Bypass cache
    #[serde(default)]
    pub bypass_cache: Option<bool>,

    /// Maximum number of results to return
    #[serde(default)]
    pub limit: Option<usize>,

    // Vector search parameters
    /// Query vector for semantic search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector: Option<Vec<f64>>,

    /// Field containing vectors (default: "embedding")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_field: Option<String>,

    /// Similarity metric: "cosine", "euclidean", "dotproduct"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_metric: Option<String>,

    /// Number of vector results (k-nearest neighbors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_k: Option<usize>,

    /// Minimum similarity threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_threshold: Option<f64>,

    // Hybrid search parameters
    /// Weight for text search (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_weight: Option<f64>,

    /// Weight for vector search (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_weight: Option<f64>,

    // Field projection
    /// Select specific fields to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_fields: Option<Vec<String>>,

    /// Exclude specific fields from results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_fields: Option<Vec<String>>,
}

impl SearchQuery {
    /// Create a new search query with the given query string
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }

    /// Set the language for stemming
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Enable case-sensitive search
    pub fn case_sensitive(mut self, enabled: bool) -> Self {
        self.case_sensitive = Some(enabled);
        self
    }

    /// Enable fuzzy matching
    pub fn fuzzy(mut self, enabled: bool) -> Self {
        self.fuzzy = Some(enabled);
        self
    }

    /// Set minimum score threshold
    pub fn min_score(mut self, score: f64) -> Self {
        self.min_score = Some(score);
        self
    }

    /// Set fields to search in
    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    /// Set field weights
    pub fn weights(mut self, weights: impl Into<String>) -> Self {
        self.weights = Some(weights.into());
        self
    }

    /// Enable stemming
    pub fn enable_stemming(mut self, enabled: bool) -> Self {
        self.enable_stemming = Some(enabled);
        self
    }

    /// Boost exact matches
    pub fn boost_exact(mut self, enabled: bool) -> Self {
        self.boost_exact = Some(enabled);
        self
    }

    /// Set maximum edit distance for fuzzy matching
    pub fn max_edit_distance(mut self, distance: u32) -> Self {
        self.max_edit_distance = Some(distance);
        self
    }

    /// Set query vector for semantic search
    pub fn vector(mut self, vector: Vec<f64>) -> Self {
        self.vector = Some(vector);
        self
    }

    /// Set vector field name
    pub fn vector_field(mut self, field: impl Into<String>) -> Self {
        self.vector_field = Some(field.into());
        self
    }

    /// Set vector similarity metric
    pub fn vector_metric(mut self, metric: impl Into<String>) -> Self {
        self.vector_metric = Some(metric.into());
        self
    }

    /// Set number of vector results (k-nearest neighbors)
    pub fn vector_k(mut self, k: usize) -> Self {
        self.vector_k = Some(k);
        self
    }

    /// Set minimum similarity threshold
    pub fn vector_threshold(mut self, threshold: f64) -> Self {
        self.vector_threshold = Some(threshold);
        self
    }

    /// Set text search weight for hybrid search
    pub fn text_weight(mut self, weight: f64) -> Self {
        self.text_weight = Some(weight);
        self
    }

    /// Set vector search weight for hybrid search
    pub fn vector_weight(mut self, weight: f64) -> Self {
        self.vector_weight = Some(weight);
        self
    }

    /// Bypass ripple cache
    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    /// Bypass cache
    pub fn bypass_cache(mut self, bypass: bool) -> Self {
        self.bypass_cache = Some(bypass);
        self
    }

    /// Set maximum number of results to return
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Select specific fields to return
    pub fn select_fields(mut self, fields: Vec<String>) -> Self {
        self.select_fields = Some(fields);
        self
    }

    /// Exclude specific fields from results
    pub fn exclude_fields(mut self, fields: Vec<String>) -> Self {
        self.exclude_fields = Some(fields);
        self
    }
}

/// Search result with score and matched fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The matched record
    pub record: serde_json::Value,

    /// Relevance score
    pub score: f64,

    /// Fields that matched the search query
    pub matched_fields: Vec<String>,
}

/// Search response containing results and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Search results
    pub results: Vec<SearchResult>,

    /// Total number of results
    pub total: usize,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_search_query_builder() {
        let query = SearchQuery::new("test query")
            .language("english")
            .fuzzy(true)
            .min_score(0.5);

        assert_eq!(query.query, "test query");
        assert_eq!(query.language, Some("english".to_string()));
        assert_eq!(query.fuzzy, Some(true));
        assert_eq!(query.min_score, Some(0.5));
    }

    #[test]
    fn test_vector_search_params() {
        let query = SearchQuery::new("test")
            .vector(vec![0.1, 0.2, 0.3])
            .vector_field("embedding")
            .vector_metric("cosine")
            .vector_k(5)
            .vector_threshold(0.8);

        assert_eq!(query.vector, Some(vec![0.1, 0.2, 0.3]));
        assert_eq!(query.vector_field, Some("embedding".to_string()));
        assert_eq!(query.vector_metric, Some("cosine".to_string()));
        assert_eq!(query.vector_k, Some(5));
        assert_eq!(query.vector_threshold, Some(0.8));
    }

    #[test]
    fn test_hybrid_search_weights() {
        let query = SearchQuery::new("test").text_weight(0.7).vector_weight(0.3);

        assert_eq!(query.text_weight, Some(0.7));
        assert_eq!(query.vector_weight, Some(0.3));
    }

    #[test]
    fn test_deserialize_fields_from_string() {
        let json = json!({
            "query": "test",
            "fields": "name,email,description"
        });

        let query: SearchQuery = serde_json::from_value(json).unwrap();
        assert_eq!(query.fields, Some("name,email,description".to_string()));
    }

    #[test]
    fn test_deserialize_fields_from_array() {
        let json = json!({
            "query": "test",
            "fields": ["name", "email", "description"]
        });

        let query: SearchQuery = serde_json::from_value(json).unwrap();
        assert_eq!(query.fields, Some("name,email,description".to_string()));
    }

    #[test]
    fn test_deserialize_weights_from_string() {
        let json = json!({
            "query": "test",
            "weights": "name:2.0,email:1.5"
        });

        let query: SearchQuery = serde_json::from_value(json).unwrap();
        assert_eq!(query.weights, Some("name:2.0,email:1.5".to_string()));
    }

    #[test]
    fn test_deserialize_weights_from_object() {
        let json = json!({
            "query": "test",
            "weights": {
                "name": 2.0,
                "email": 1.5,
                "description": 1.0
            }
        });

        let query: SearchQuery = serde_json::from_value(json).unwrap();
        assert!(query.weights.is_some());
        let weights = query.weights.unwrap();
        // Should contain all three fields with weights
        assert!(weights.contains("name:2"));
        assert!(weights.contains("email:1.5"));
        assert!(weights.contains("description:1"));
    }

    #[test]
    fn test_search_query_serialization() {
        let query = SearchQuery::new("test query")
            .language("english")
            .fuzzy(true)
            .fields("name,email");

        let json = serde_json::to_value(&query).unwrap();
        assert_eq!(json["query"], "test query");
        assert_eq!(json["language"], "english");
        assert_eq!(json["fuzzy"], true);
        assert_eq!(json["fields"], "name,email");
    }

    #[test]
    fn test_bypass_flags() {
        let query = SearchQuery::new("test")
            .bypass_cache(true)
            .bypass_ripple(true);

        assert_eq!(query.bypass_cache, Some(true));
        assert_eq!(query.bypass_ripple, Some(true));
    }

    #[test]
    fn test_max_edit_distance() {
        let query = SearchQuery::new("test").max_edit_distance(2);

        assert_eq!(query.max_edit_distance, Some(2));
    }

    #[test]
    fn test_enable_stemming_and_boost_exact() {
        let query = SearchQuery::new("test")
            .enable_stemming(true)
            .boost_exact(true);

        assert_eq!(query.enable_stemming, Some(true));
        assert_eq!(query.boost_exact, Some(true));
    }

    #[test]
    fn test_limit() {
        let query = SearchQuery::new("test").limit(10);

        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn test_limit_serialization() {
        let query = SearchQuery::new("test").limit(5);

        let json = serde_json::to_value(&query).unwrap();
        assert_eq!(json["limit"], 5);
    }
}
