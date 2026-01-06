//! Public API types for ekoDB client
//!
//! These types represent the data structures used in the ekoDB API.
//! They are intentionally separate from server internals.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

/// Serialization format for client-server communication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    /// JSON format (default, human-readable)
    Json,
    /// MessagePack format (binary, faster)
    MessagePack,
}

/// Flexible numeric value that can be Integer, Float, or Decimal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberValue {
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// Decimal value
    Decimal(Decimal),
}

/// Field type representing all supported data types in ekoDB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldType {
    /// String value
    String(String),
    /// 64-bit signed integer
    Integer(i64),
    /// 64-bit floating point number
    Float(f64),
    /// Flexible numeric type (can be Integer, Float, or Decimal)
    Number(NumberValue),
    /// Boolean value
    Boolean(bool),
    /// Nested object (key-value map)
    Object(HashMap<String, FieldType>),
    /// Ordered array of values
    Array(Vec<FieldType>),
    /// Unordered set (automatically deduplicated)
    Set(Vec<FieldType>),
    /// Vector embeddings for similarity search
    Vector(Vec<FieldType>),
    /// ISO-8601 datetime
    DateTime(DateTime<Utc>),
    /// UUID
    UUID(Uuid),
    /// High-precision decimal number
    Decimal(Decimal),
    /// Time duration
    Duration(Duration),
    /// Binary data (base64 encoded)
    Binary(Vec<u8>),
    /// Raw bytes
    Bytes(Vec<u8>),
    /// Null value
    Null,
}

impl FieldType {
    /// Create a String field
    pub fn string(s: impl Into<String>) -> Self {
        FieldType::String(s.into())
    }

    /// Create an Integer field
    pub fn integer(i: i64) -> Self {
        FieldType::Integer(i)
    }

    /// Create a Float field
    pub fn float(f: f64) -> Self {
        FieldType::Float(f)
    }

    /// Create a Number field from an integer
    pub fn number_int(i: i64) -> Self {
        FieldType::Number(NumberValue::Integer(i))
    }

    /// Create a Number field from a float
    pub fn number_float(f: f64) -> Self {
        FieldType::Number(NumberValue::Float(f))
    }

    /// Create a Number field from a decimal
    pub fn number_decimal(d: Decimal) -> Self {
        FieldType::Number(NumberValue::Decimal(d))
    }

    /// Create a Boolean field
    pub fn boolean(b: bool) -> Self {
        FieldType::Boolean(b)
    }

    /// Create an Array field
    pub fn array(items: Vec<FieldType>) -> Self {
        FieldType::Array(items)
    }

    /// Create a Set field (automatically deduplicated by server)
    pub fn set(items: Vec<FieldType>) -> Self {
        FieldType::Set(items)
    }

    /// Create a Vector field for embeddings
    pub fn vector(embeddings: Vec<f64>) -> Self {
        FieldType::Vector(embeddings.into_iter().map(FieldType::Float).collect())
    }

    /// Create a DateTime field
    pub fn datetime(dt: DateTime<Utc>) -> Self {
        FieldType::DateTime(dt)
    }

    /// Create a UUID field
    pub fn uuid(uuid: Uuid) -> Self {
        FieldType::UUID(uuid)
    }

    /// Create a Decimal field
    pub fn decimal(d: Decimal) -> Self {
        FieldType::Decimal(d)
    }

    /// Create a Null field
    pub fn null() -> Self {
        FieldType::Null
    }
}

/// A record in ekoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    /// Record fields
    #[serde(flatten)]
    pub fields: HashMap<String, FieldType>,
}

impl Record {
    /// Create a new empty record
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Insert a field into the record
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<FieldType>) {
        self.fields.insert(key.into(), value.into());
    }

    /// Insert a field and return self for fluent chaining
    ///
    /// This is a builder-style method that allows chaining field insertions.
    ///
    /// # Example
    ///
    /// ```
    /// # use ekodb_client::Record;
    /// let record = Record::new()
    ///     .field("name", "John Doe")
    ///     .field("age", 30)
    ///     .field("active", true);
    /// ```
    pub fn field(mut self, key: impl Into<String>, value: impl Into<FieldType>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }

    /// Get a field from the record
    pub fn get(&self, key: &str) -> Option<&FieldType> {
        self.fields.get(key)
    }

    /// Remove a field from the record
    pub fn remove(&mut self, key: &str) -> Option<FieldType> {
        self.fields.remove(key)
    }

    /// Check if a field exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    /// Get the number of fields
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Check if the record is empty
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Set TTL duration for this record
    ///
    /// Supported formats:
    /// - Duration strings: "30s", "5m", "1h", "1d", "2w"
    /// - Integer seconds as string: "3600"
    /// - ISO8601 timestamp: "2024-12-31T23:59:59Z"
    pub fn with_ttl(mut self, duration: impl Into<String>) -> Self {
        self.fields
            .insert("ttl".to_string(), FieldType::String(duration.into()));
        self
    }

    /// Set TTL with update-on-access behavior
    ///
    /// If true, TTL resets when the record is accessed
    pub fn with_ttl_update_on_access(
        mut self,
        duration: impl Into<String>,
        update_on_access: bool,
    ) -> Self {
        self.fields
            .insert("ttl".to_string(), FieldType::String(duration.into()));
        self.fields.insert(
            "ttl_update_on_access".to_string(),
            FieldType::Boolean(update_on_access),
        );
        self
    }
}

impl Default for Record {
    fn default() -> Self {
        Self::new()
    }
}

/// Query operators for filtering
///
/// ekoDB uses its own operator format, not MongoDB-style $ operators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum QueryOperator {
    /// Equal to
    Eq(FieldType),
    /// Not equal to
    Ne(FieldType),
    /// Greater than
    Gt(FieldType),
    /// Greater than or equal to
    Gte(FieldType),
    /// Less than
    Lt(FieldType),
    /// Less than or equal to
    Lte(FieldType),
    /// In array
    In(Vec<FieldType>),
    /// Not in array
    #[serde(rename = "NotIn")]
    Nin(Vec<FieldType>),
    /// Regex match
    Regex(String),
    /// Exists
    Exists(bool),
}

/// Query for finding records
///
/// Matches the server's FindBody structure with optional fields
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Query {
    /// Filter expression (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,

    /// Sort expression (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<serde_json::Value>,

    /// Limit number of results (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    /// Skip number of results for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<usize>,

    /// Join configuration (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<serde_json::Value>,

    /// Bypass cache (optional)
    #[serde(default)]
    pub bypass_cache: Option<bool>,

    /// Select fields (optional)
    #[serde(default)]
    pub select_fields: Option<Vec<String>>,

    /// Exclude fields (optional)
    #[serde(default)]
    pub exclude_fields: Option<Vec<String>>,

    /// Bypass ripple (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_ripple: Option<bool>,
}

impl Query {
    /// Create a new empty query
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the filter expression
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Set the sort expression
    pub fn sort(mut self, sort: serde_json::Value) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Set the limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the skip for pagination
    pub fn skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }

    /// Set bypass_cache flag
    pub fn bypass_cache(mut self, bypass: bool) -> Self {
        self.bypass_cache = Some(bypass);
        self
    }

    /// Set bypass_ripple flag
    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    /// Set join configuration
    pub fn join(mut self, join: serde_json::Value) -> Self {
        self.join = Some(join);
        self
    }
}

// Implement From traits for convenient conversions
impl From<String> for FieldType {
    fn from(s: String) -> Self {
        FieldType::String(s)
    }
}

impl From<&str> for FieldType {
    fn from(s: &str) -> Self {
        FieldType::String(s.to_string())
    }
}

impl From<i64> for FieldType {
    fn from(i: i64) -> Self {
        FieldType::Integer(i)
    }
}

impl From<i32> for FieldType {
    fn from(i: i32) -> Self {
        FieldType::Integer(i as i64)
    }
}

impl From<f64> for FieldType {
    fn from(f: f64) -> Self {
        FieldType::Float(f)
    }
}

impl From<bool> for FieldType {
    fn from(b: bool) -> Self {
        FieldType::Boolean(b)
    }
}

impl From<DateTime<Utc>> for FieldType {
    fn from(dt: DateTime<Utc>) -> Self {
        FieldType::DateTime(dt)
    }
}

impl From<Uuid> for FieldType {
    fn from(uuid: Uuid) -> Self {
        FieldType::UUID(uuid)
    }
}

impl From<Decimal> for FieldType {
    fn from(d: Decimal) -> Self {
        FieldType::Decimal(d)
    }
}

impl From<serde_json::Value> for FieldType {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => FieldType::Null,
            serde_json::Value::Bool(b) => FieldType::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    FieldType::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    FieldType::Float(f)
                } else {
                    FieldType::String(n.to_string())
                }
            }
            serde_json::Value::String(s) => FieldType::String(s),
            serde_json::Value::Array(arr) => {
                FieldType::Array(arr.into_iter().map(FieldType::from).collect())
            }
            serde_json::Value::Object(obj) => FieldType::Object(
                obj.into_iter()
                    .map(|(k, v)| (k, FieldType::from(v)))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_field_type_string() {
        let field: FieldType = "hello".into();
        assert!(matches!(field, FieldType::String(_)));
    }

    #[test]
    fn test_field_type_integer() {
        let field: FieldType = 42i64.into();
        assert!(matches!(field, FieldType::Integer(42)));
    }

    #[test]
    fn test_field_type_float() {
        let field: FieldType = 3.14f64.into();
        assert!(matches!(field, FieldType::Float(_)));
    }

    #[test]
    fn test_field_type_boolean() {
        let field: FieldType = true.into();
        assert!(matches!(field, FieldType::Boolean(true)));
    }

    #[test]
    fn test_field_type_datetime() {
        let now = Utc::now();
        let field: FieldType = now.into();
        assert!(matches!(field, FieldType::DateTime(_)));
    }

    #[test]
    fn test_field_type_uuid() {
        let uuid = Uuid::new_v4();
        let field: FieldType = uuid.into();
        assert!(matches!(field, FieldType::UUID(_)));
    }

    #[test]
    fn test_field_type_decimal() {
        let decimal = Decimal::new(12345, 2);
        let field: FieldType = decimal.into();
        assert!(matches!(field, FieldType::Decimal(_)));
    }

    #[test]
    fn test_number_value_integer() {
        let num = NumberValue::Integer(42);
        assert!(matches!(num, NumberValue::Integer(42)));
    }

    #[test]
    fn test_number_value_float() {
        let num = NumberValue::Float(3.14);
        assert!(matches!(num, NumberValue::Float(_)));
    }

    #[test]
    fn test_number_value_decimal() {
        let decimal = Decimal::new(12345, 2);
        let num = NumberValue::Decimal(decimal);
        assert!(matches!(num, NumberValue::Decimal(_)));
    }

    #[test]
    fn test_query_operator_eq() {
        let op = QueryOperator::Eq(FieldType::String("test".to_string()));
        assert!(matches!(op, QueryOperator::Eq(_)));
    }

    #[test]
    fn test_query_operator_ne() {
        let op = QueryOperator::Ne(FieldType::Integer(42));
        assert!(matches!(op, QueryOperator::Ne(_)));
    }

    #[test]
    fn test_query_operator_gt() {
        let op = QueryOperator::Gt(FieldType::Integer(10));
        assert!(matches!(op, QueryOperator::Gt(_)));
    }

    #[test]
    fn test_query_operator_gte() {
        let op = QueryOperator::Gte(FieldType::Integer(10));
        assert!(matches!(op, QueryOperator::Gte(_)));
    }

    #[test]
    fn test_query_operator_lt() {
        let op = QueryOperator::Lt(FieldType::Integer(100));
        assert!(matches!(op, QueryOperator::Lt(_)));
    }

    #[test]
    fn test_query_operator_lte() {
        let op = QueryOperator::Lte(FieldType::Integer(100));
        assert!(matches!(op, QueryOperator::Lte(_)));
    }

    #[test]
    fn test_query_operator_in() {
        let op = QueryOperator::In(vec![FieldType::Integer(1), FieldType::Integer(2)]);
        assert!(matches!(op, QueryOperator::In(_)));
    }

    #[test]
    fn test_query_operator_nin() {
        let op = QueryOperator::Nin(vec![FieldType::Integer(1), FieldType::Integer(2)]);
        assert!(matches!(op, QueryOperator::Nin(_)));
    }

    #[test]
    fn test_query_operator_regex() {
        let op = QueryOperator::Regex("^test".to_string());
        assert!(matches!(op, QueryOperator::Regex(_)));
    }

    #[test]
    fn test_query_operator_exists() {
        let op = QueryOperator::Exists(true);
        assert!(matches!(op, QueryOperator::Exists(true)));
    }

    #[test]
    fn test_query_default() {
        let query = Query::default();
        assert!(query.filter.is_none());
        assert!(query.sort.is_none());
        assert!(query.limit.is_none());
        assert!(query.skip.is_none());
    }

    #[test]
    fn test_query_serialization() {
        let query = Query::new()
            .filter(serde_json::json!({"name": "test"}))
            .limit(10);

        let json = serde_json::to_value(&query).unwrap();
        assert!(json["filter"].is_object());
        assert_eq!(json["limit"], 10);
    }

    #[test]
    fn test_query_deserialization() {
        let json = serde_json::json!({
            "filter": {"name": "test"},
            "limit": 10,
            "skip": 5
        });

        let query: Query = serde_json::from_value(json).unwrap();
        assert!(query.filter.is_some());
        assert_eq!(query.limit, Some(10));
        assert_eq!(query.skip, Some(5));
    }

    #[test]
    fn test_record_serialization() {
        let mut record = Record::new();
        record.insert("name", "test");
        record.insert("age", 30);

        let json = serde_json::to_value(&record).unwrap();
        assert!(json.is_object());
    }

    #[test]
    fn test_record_deserialization() {
        let json = serde_json::json!({
            "name": "test",
            "age": 30
        });

        let record: Record = serde_json::from_value(json).unwrap();
        assert!(record.contains_key("name"));
        assert!(record.contains_key("age"));
    }
}
