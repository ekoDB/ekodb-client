//! Test utilities and mock infrastructure for ekoDB client tests
//!
//! This module provides standardized testing patterns and utilities that can be
//! used across all test modules. It serves as a foundation for consistent,
//! maintainable tests.

#[cfg(test)]
pub mod mocks {
    use serde_json::{json, Value};

    /// Standard mock responses for ekoDB API endpoints
    pub struct MockResponses;

    impl MockResponses {
        /// Mock token response from /api/auth/token
        pub fn token_response(token: &str) -> Value {
            json!({
                "token": token
            })
        }

        /// Mock health check response
        pub fn health_ok() -> Value {
            json!({
                "status": "healthy",
                "version": "0.7.0"
            })
        }

        /// Mock insert response with generated ID
        pub fn insert_response(id: &str, data: Value) -> Value {
            let mut response = data;
            if let Value::Object(ref mut map) = response {
                map.insert("id".to_string(), Value::String(id.to_string()));
            }
            response
        }

        /// Mock find response with records
        pub fn find_response(records: Vec<Value>) -> Value {
            json!(records)
        }

        /// Mock find_by_id response
        pub fn find_by_id_response(record: Value) -> Value {
            record
        }

        /// Mock update response
        pub fn update_response(record: Value) -> Value {
            record
        }

        /// Mock delete response
        pub fn delete_response(id: &str) -> Value {
            json!({
                "id": id,
                "deleted": true
            })
        }

        /// Mock batch insert response
        pub fn batch_insert_response(ids: Vec<&str>) -> Value {
            json!({
                "successful": ids,
                "failed": []
            })
        }

        /// Mock batch update response
        pub fn batch_update_response(ids: Vec<&str>) -> Value {
            json!({
                "successful": ids,
                "failed": []
            })
        }

        /// Mock batch delete response
        pub fn batch_delete_response(ids: Vec<&str>) -> Value {
            json!({
                "successful": ids,
                "failed": []
            })
        }

        /// Mock collections list response
        pub fn collections_response(collections: Vec<&str>) -> Value {
            json!({
                "collections": collections
            })
        }

        /// Mock search response
        pub fn search_response(results: Vec<Value>, total: usize) -> Value {
            json!({
                "results": results,
                "total": total,
                "execution_time_ms": 5
            })
        }

        /// Mock KV get response
        pub fn kv_get_response(value: Value) -> Value {
            json!({
                "value": value
            })
        }

        /// Mock transaction begin response
        pub fn transaction_begin_response(transaction_id: &str) -> Value {
            json!({
                "transaction_id": transaction_id
            })
        }

        /// Mock error response
        pub fn error_response(code: u16, message: &str) -> Value {
            json!({
                "error": message,
                "code": code
            })
        }
    }

    /// Test data generators for consistent test fixtures
    pub struct TestData;

    impl TestData {
        /// Create a test record with name and age
        pub fn user_record(name: &str, age: i64) -> Value {
            json!({
                "name": name,
                "age": age
            })
        }

        /// Create a test record with ID
        pub fn user_record_with_id(id: &str, name: &str, age: i64) -> Value {
            json!({
                "id": id,
                "name": name,
                "age": age
            })
        }

        /// Create a test search result
        pub fn search_result(record: Value, score: f64) -> Value {
            json!({
                "record": record,
                "score": score,
                "matched_fields": ["name"]
            })
        }

        /// Create multiple test records
        pub fn user_records(count: usize) -> Vec<Value> {
            (0..count)
                .map(|i| {
                    json!({
                        "id": format!("user_{}", i),
                        "name": format!("User {}", i),
                        "age": 20 + i as i64
                    })
                })
                .collect()
        }

        /// Create a test vector
        pub fn embedding(dimensions: usize) -> Vec<f64> {
            (0..dimensions).map(|i| (i as f64) * 0.1).collect()
        }
    }
}

#[cfg(test)]
pub mod assertions {
    use serde_json::Value;

    /// Assert that a JSON value contains a specific key
    pub fn assert_has_key(value: &Value, key: &str) {
        assert!(
            value.get(key).is_some(),
            "Expected key '{}' not found in {:?}",
            key,
            value
        );
    }

    /// Assert that a JSON value has a specific string field value
    pub fn assert_string_field(value: &Value, key: &str, expected: &str) {
        let actual = value
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("Key '{}' not found or not a string", key));
        assert_eq!(actual, expected, "Field '{}' mismatch", key);
    }

    /// Assert that a JSON value has a specific integer field value
    pub fn assert_int_field(value: &Value, key: &str, expected: i64) {
        let actual = value
            .get(key)
            .and_then(|v| v.as_i64())
            .unwrap_or_else(|| panic!("Key '{}' not found or not an integer", key));
        assert_eq!(actual, expected, "Field '{}' mismatch", key);
    }

    /// Assert that a JSON array has expected length
    pub fn assert_array_len(value: &Value, expected_len: usize) {
        let arr = value.as_array().expect("Expected array");
        assert_eq!(arr.len(), expected_len, "Array length mismatch");
    }
}

#[cfg(test)]
mod tests {
    use super::mocks::*;

    #[test]
    fn test_mock_token_response() {
        let response = MockResponses::token_response("test-token");
        assert_eq!(response["token"], "test-token");
    }

    #[test]
    fn test_mock_insert_response() {
        let data = serde_json::json!({"name": "John"});
        let response = MockResponses::insert_response("123", data);
        assert_eq!(response["id"], "123");
        assert_eq!(response["name"], "John");
    }

    #[test]
    fn test_test_data_user_record() {
        let record = TestData::user_record("John", 30);
        assert_eq!(record["name"], "John");
        assert_eq!(record["age"], 30);
    }

    #[test]
    fn test_test_data_user_records() {
        let records = TestData::user_records(3);
        assert_eq!(records.len(), 3);
        assert_eq!(records[0]["name"], "User 0");
        assert_eq!(records[1]["name"], "User 1");
        assert_eq!(records[2]["name"], "User 2");
    }

    #[test]
    fn test_test_data_embedding() {
        let embedding = TestData::embedding(4);
        assert_eq!(embedding.len(), 4);
        assert!((embedding[0] - 0.0).abs() < f64::EPSILON);
        assert!((embedding[1] - 0.1).abs() < f64::EPSILON);
    }
}
