//! Comprehensive unit tests for ekoDB client
//!
//! These tests use mockito to mock HTTP responses and test the client
//! without requiring a running ekoDB server.
//!
//! Run with: `cargo test -p ekodb_client --test unit_tests`

use ekodb_client::{Client, Error, FieldType, Query, QueryBuilder, Record, SearchQuery};
use mockito::{Matcher, Server};
use serde_json::json;

// ============================================================================
// Test Helpers
// ============================================================================

/// Create a test client pointing to mock server
async fn create_test_client(server: &Server) -> Client {
    Client::builder()
        .base_url(server.url())
        .api_key("test-api-key")
        .should_retry(false) // Disable retries for predictable tests
        .serialization_format(ekodb_client::SerializationFormat::Json) // Use JSON for mock tests
        .build()
        .expect("Failed to create test client")
}

/// Setup mock for token endpoint
fn mock_token_endpoint(server: &mut Server) -> mockito::Mock {
    server
        .mock("POST", "/api/auth/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"token": "test-jwt-token"}).to_string())
        .create()
}

// ============================================================================
// Client Builder Tests
// ============================================================================

#[test]
fn test_client_builder_requires_base_url() {
    let result = Client::builder().api_key("test-key").build();

    assert!(result.is_err());
    match result {
        Err(Error::InvalidConfig(msg)) => {
            assert!(msg.contains("base_url"), "Error should mention base_url");
        }
        _ => panic!("Expected InvalidConfig error"),
    }
}

#[test]
fn test_client_builder_requires_api_key() {
    let result = Client::builder().base_url("http://localhost:8080").build();

    assert!(result.is_err());
    match result {
        Err(Error::InvalidConfig(msg)) => {
            assert!(msg.contains("api_key"), "Error should mention api_key");
        }
        _ => panic!("Expected InvalidConfig error"),
    }
}

#[test]
fn test_client_builder_validates_url() {
    let result = Client::builder()
        .base_url("not-a-valid-url")
        .api_key("test-key")
        .build();

    assert!(result.is_err());
}

#[test]
fn test_client_builder_accepts_valid_config() {
    let result = Client::builder()
        .base_url("http://localhost:8080")
        .api_key("test-key")
        .build();

    assert!(result.is_ok());
}

#[test]
fn test_client_builder_accepts_all_options() {
    let result = Client::builder()
        .base_url("http://localhost:8080")
        .api_key("test-key")
        .timeout(std::time::Duration::from_secs(60))
        .max_retries(5)
        .should_retry(true)
        .serialization_format(ekodb_client::SerializationFormat::Json)
        .build();

    assert!(result.is_ok());
}

// ============================================================================
// Health Check Tests
// ============================================================================

#[tokio::test]
async fn test_health_check_success() {
    let mut server = Server::new_async().await;

    let _m = server
        .mock("GET", "/api/health")
        .with_status(200)
        .with_body(json!({"status": "healthy"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.health_check().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_health_check_failure() {
    let mut server = Server::new_async().await;

    let _m = server
        .mock("GET", "/api/health")
        .with_status(503)
        .with_body("Service Unavailable")
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.health_check().await;

    assert!(result.is_err());
}

// ============================================================================
// Insert Tests
// ============================================================================

#[tokio::test]
async fn test_insert_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _insert_mock = server
        .mock("POST", "/api/insert/users")
        .match_header("Authorization", "Bearer test-jwt-token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "user_123",
                "name": "John Doe",
                "age": 30
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let mut record = Record::new();
    record.insert("name", "John Doe");
    record.insert("age", 30i64);

    let result = client.insert("users", record, None).await;

    assert!(result.is_ok());
    let inserted = result.unwrap();
    assert!(inserted.get("id").is_some());
}

#[tokio::test]
async fn test_insert_with_bypass_ripple() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _insert_mock = server
        .mock("POST", "/api/insert/users")
        .match_query(Matcher::UrlEncoded("bypass_ripple".into(), "true".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "user_123"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let mut record = Record::new();
    record.insert("name", "Test");

    let options = ekodb_client::options::InsertOptions::new().bypass_ripple(true);
    let result = client.insert("users", record, Some(options)).await;
    assert!(result.is_ok());
}

// ============================================================================
// Find Tests
// ============================================================================

#[tokio::test]
async fn test_find_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _find_mock = server
        .mock("POST", "/api/find/users")
        .match_header("Authorization", "Bearer test-jwt-token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {"id": "user_1", "name": "Alice", "age": 25},
                {"id": "user_2", "name": "Bob", "age": 30}
            ])
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let query = Query::new().limit(10);
    let result = client.find("users", query, None).await;

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 2);
}

#[tokio::test]
async fn test_find_empty_result() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _find_mock = server
        .mock("POST", "/api/find/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("[]")
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let query = Query::new();
    let result = client.find("users", query, None).await;

    assert!(result.is_ok());
    let records = result.unwrap();
    assert!(records.is_empty());
}

// ============================================================================
// Find By ID Tests
// ============================================================================

#[tokio::test]
async fn test_find_by_id_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _find_mock = server
        .mock("GET", "/api/find/users/user_123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "user_123", "name": "Alice"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.find_by_id("users", "user_123", None).await;

    assert!(result.is_ok());
    let record = result.unwrap();
    assert!(record.get("id").is_some());
}

// ============================================================================
// Update Tests
// ============================================================================

#[tokio::test]
async fn test_update_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _update_mock = server
        .mock("PUT", "/api/update/users/user_123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "user_123",
                "name": "Alice Updated",
                "age": 26
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let mut record = Record::new();
    record.insert("name", "Alice Updated");
    record.insert("age", 26i64);

    let result = client.update("users", "user_123", record, None).await;

    assert!(result.is_ok());
}

// ============================================================================
// Delete Tests
// ============================================================================

#[tokio::test]
async fn test_delete_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _delete_mock = server
        .mock("DELETE", "/api/delete/users/user_123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "user_123", "deleted": true}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.delete("users", "user_123", None).await;

    assert!(result.is_ok());
}

// ============================================================================
// Batch Operations Tests
// ============================================================================

#[tokio::test]
async fn test_batch_insert_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _batch_mock = server
        .mock("POST", "/api/batch/insert/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "successful": ["id_1", "id_2", "id_3"],
                "failed": []
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let records: Vec<Record> = (0..3)
        .map(|i| {
            let mut r = Record::new();
            r.insert("name", format!("User {}", i));
            r
        })
        .collect();

    let result = client.batch_insert("users", records, None).await;

    assert!(result.is_ok());
    let inserted = result.unwrap();
    assert_eq!(inserted.len(), 3);
}

#[tokio::test]
async fn test_batch_delete_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _batch_mock = server
        .mock("DELETE", "/api/batch/delete/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "successful": ["id_1", "id_2"],
                "failed": []
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let ids = vec!["id_1".to_string(), "id_2".to_string()];
    let result = client.batch_delete("users", ids, None).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
}

// ============================================================================
// Collections Tests
// ============================================================================

#[tokio::test]
async fn test_list_collections_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _collections_mock = server
        .mock("GET", "/api/collections")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "collections": ["users", "posts", "comments"]
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.list_collections().await;

    assert!(result.is_ok());
    let collections = result.unwrap();
    assert_eq!(collections.len(), 3);
    assert!(collections.contains(&"users".to_string()));
}

// ============================================================================
// KV Store Tests
// ============================================================================

#[tokio::test]
async fn test_kv_set_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("POST", "/api/kv/set/my_key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"success": true}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client
        .kv_set("my_key", json!({"data": "value"}), None)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_kv_get_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("GET", "/api/kv/get/my_key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"value": {"data": "stored_value"}}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.kv_get("my_key").await;

    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_some());
}

#[tokio::test]
async fn test_kv_delete_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("DELETE", "/api/kv/delete/my_key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"deleted": true}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.kv_delete("my_key").await;

    assert!(result.is_ok());
}

// ============================================================================
// KV Batch Operations Tests
// ============================================================================

#[tokio::test]
async fn test_kv_batch_get_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("POST", "/api/kv/batch/get")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {"data": "value1"},
                {"data": "value2"},
                {"data": "value3"}
            ])
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
    let result = client.kv_batch_get(keys).await;

    assert!(result.is_ok());
    let values = result.unwrap();
    assert_eq!(values.len(), 3);
}

#[tokio::test]
async fn test_kv_batch_set_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("POST", "/api/kv/batch/set")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([["key1", true], ["key2", true], ["key3", true]]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
    let mut values = Vec::new();
    for i in 1..=3 {
        let mut record = Record::new();
        record.insert("data".to_string(), FieldType::String(format!("value{}", i)));
        values.push(record);
    }

    let result = client.kv_batch_set(keys, values, None).await;

    assert!(result.is_ok());
    let results = result.unwrap();
    assert_eq!(results.len(), 3);
    assert!(results[0].1);
    assert!(results[1].1);
    assert!(results[2].1);
}

#[tokio::test]
async fn test_kv_batch_set_with_ttl() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("POST", "/api/kv/batch/set")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([["key1", true], ["key2", true]]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let keys = vec!["key1".to_string(), "key2".to_string()];
    let mut values = Vec::new();
    for i in 1..=2 {
        let mut record = Record::new();
        record.insert("data".to_string(), FieldType::String(format!("value{}", i)));
        values.push(record);
    }

    let result = client.kv_batch_set(keys, values, Some(3600)).await;

    assert!(result.is_ok());
    let results = result.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn test_kv_batch_delete_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("DELETE", "/api/kv/batch/delete")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([["key1", true], ["key2", true], ["key3", false]]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
    let result = client.kv_batch_delete(keys).await;

    assert!(result.is_ok());
    let results = result.unwrap();
    assert_eq!(results.len(), 3);
    assert!(results[0].1); // key1 deleted
    assert!(results[1].1); // key2 deleted
    assert!(!results[2].1); // key3 not found
}

#[tokio::test]
async fn test_kv_batch_get_empty() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("POST", "/api/kv/batch/get")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.kv_batch_get(vec![]).await;

    assert!(result.is_ok());
    let values = result.unwrap();
    assert_eq!(values.len(), 0);
}

#[tokio::test]
async fn test_kv_batch_set_partial_failure() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("POST", "/api/kv/batch/set")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([["key1", true], ["key2", false], ["key3", true]]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
    let mut values = Vec::new();
    for i in 1..=3 {
        let mut record = Record::new();
        record.insert("data".to_string(), FieldType::String(format!("value{}", i)));
        values.push(record);
    }

    let result = client.kv_batch_set(keys, values, None).await;

    assert!(result.is_ok());
    let results = result.unwrap();
    assert!(results[0].1); // key1 succeeded
    assert!(!results[1].1); // key2 failed
    assert!(results[2].1); // key3 succeeded
}

// ============================================================================
// Transaction Tests
// ============================================================================

#[tokio::test]
async fn test_begin_transaction_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _tx_mock = server
        .mock("POST", "/api/transactions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"transaction_id": "tx_123456"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.begin_transaction("ReadCommitted").await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "tx_123456");
}

#[tokio::test]
async fn test_commit_transaction_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _tx_mock = server
        .mock("POST", "/api/transactions/tx_123/commit")
        .with_status(200)
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.commit_transaction("tx_123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_rollback_transaction_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _tx_mock = server
        .mock("POST", "/api/transactions/tx_123/rollback")
        .with_status(200)
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.rollback_transaction("tx_123").await;

    assert!(result.is_ok());
}

// ============================================================================
// Authentication Tests
// ============================================================================

#[tokio::test]
async fn test_token_refresh() {
    let mut server = Server::new_async().await;

    // First token request
    let _token_mock = server
        .mock("POST", "/api/auth/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"token": "new-jwt-token"}).to_string())
        .expect(2) // Expect 2 calls - initial + refresh
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    // Clear cache and refresh
    client.clear_token_cache().await;
    let result = client.refresh_token().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_authentication_failure() {
    let mut server = Server::new_async().await;

    let _token_mock = server
        .mock("POST", "/api/auth/token")
        .with_status(401)
        .with_body("Invalid API key")
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    // Try an operation that requires authentication
    let result = client.list_collections().await;

    assert!(result.is_err());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_server_error_handling() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _error_mock = server
        .mock("POST", "/api/insert/users")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let mut record = Record::new();
    record.insert("name", "Test");

    let result = client.insert("users", record, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_not_found_error() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _error_mock = server
        .mock("GET", "/api/find/users/nonexistent")
        .with_status(404)
        .with_body(json!({"error": "Not found"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.find_by_id("users", "nonexistent", None).await;

    assert!(result.is_err());
}

// ============================================================================
// Record Tests
// ============================================================================

#[test]
fn test_record_new_is_empty() {
    let record = Record::new();
    assert!(record.is_empty());
    assert_eq!(record.len(), 0);
}

#[test]
fn test_record_insert_and_get() {
    let mut record = Record::new();
    record.insert("name", "John");
    record.insert("age", 30i64);

    assert_eq!(record.len(), 2);
    assert!(record.contains_key("name"));
    assert!(record.contains_key("age"));
    assert!(!record.contains_key("email"));
}

#[test]
fn test_record_remove() {
    let mut record = Record::new();
    record.insert("name", "John");

    let removed = record.remove("name");
    assert!(removed.is_some());
    assert!(!record.contains_key("name"));
}

#[test]
fn test_record_with_ttl() {
    let record = Record::new().with_ttl("30m");
    assert!(record.contains_key("ttl"));
}

#[test]
fn test_record_with_ttl_update_on_access() {
    let record = Record::new().with_ttl_update_on_access("1h", true);
    assert!(record.contains_key("ttl"));
    assert!(record.contains_key("ttl_update_on_access"));
}

// ============================================================================
// FieldType Tests
// ============================================================================

#[test]
fn test_field_type_string() {
    let field: FieldType = "hello".into();
    match field {
        FieldType::String(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected String"),
    }
}

#[test]
fn test_field_type_integer() {
    let field: FieldType = 42i64.into();
    match field {
        FieldType::Integer(i) => assert_eq!(i, 42),
        _ => panic!("Expected Integer"),
    }
}

#[test]
fn test_field_type_i32() {
    let field: FieldType = 42i32.into();
    match field {
        FieldType::Integer(i) => assert_eq!(i, 42),
        _ => panic!("Expected Integer"),
    }
}

#[test]
fn test_field_type_float() {
    let field: FieldType = 3.15f64.into();
    match field {
        FieldType::Float(f) => assert!((f - 3.15).abs() < f64::EPSILON),
        _ => panic!("Expected Float"),
    }
}

#[test]
fn test_field_type_boolean() {
    let field: FieldType = true.into();
    match field {
        FieldType::Boolean(b) => assert!(b),
        _ => panic!("Expected Boolean"),
    }
}

#[test]
fn test_field_type_factory_methods() {
    let s = FieldType::string("test");
    assert!(matches!(s, FieldType::String(_)));

    let i = FieldType::integer(42);
    assert!(matches!(i, FieldType::Integer(42)));

    let f = FieldType::float(3.15);
    assert!(matches!(f, FieldType::Float(_)));

    let b = FieldType::boolean(true);
    assert!(matches!(b, FieldType::Boolean(true)));

    let n = FieldType::null();
    assert!(matches!(n, FieldType::Null));
}

#[test]
fn test_field_type_vector() {
    let vec = FieldType::vector(vec![0.1, 0.2, 0.3]);
    match vec {
        FieldType::Vector(v) => assert_eq!(v.len(), 3),
        _ => panic!("Expected Vector"),
    }
}

// ============================================================================
// Query Tests
// ============================================================================

#[test]
fn test_query_default() {
    let query = Query::new();
    assert!(query.filter.is_none());
    assert!(query.sort.is_none());
    assert!(query.limit.is_none());
    assert!(query.skip.is_none());
}

#[test]
fn test_query_with_all_options() {
    let query = Query::new()
        .filter(json!({"status": "active"}))
        .sort(json!([{"field": "created_at", "ascending": false}]))
        .limit(10)
        .skip(20)
        .bypass_cache(true)
        .bypass_ripple(false);

    assert!(query.filter.is_some());
    assert!(query.sort.is_some());
    assert_eq!(query.limit, Some(10));
    assert_eq!(query.skip, Some(20));
    assert_eq!(query.bypass_cache, Some(true));
    assert_eq!(query.bypass_ripple, Some(false));
}

// ============================================================================
// QueryBuilder Tests
// ============================================================================

#[test]
fn test_query_builder_eq() {
    let query = QueryBuilder::new().eq("status", "active").build();

    assert!(query.filter.is_some());
    let filter = query.filter.unwrap();
    assert_eq!(filter["type"], "Condition");
    assert_eq!(filter["content"]["operator"], "Eq");
}

#[test]
fn test_query_builder_comparison_operators() {
    let query = QueryBuilder::new()
        .gt("age", 18)
        .lt("age", 65)
        .gte("score", 80)
        .lte("score", 100)
        .build();

    assert!(query.filter.is_some());
    let filter = query.filter.unwrap();
    // Multiple filters are combined with AND
    assert_eq!(filter["type"], "Logical");
    assert_eq!(filter["content"]["operator"], "And");
}

#[test]
fn test_query_builder_in_array() {
    let query = QueryBuilder::new()
        .in_array("status", vec![json!("active"), json!("pending")])
        .build();

    assert!(query.filter.is_some());
}

#[test]
fn test_query_builder_string_operators() {
    let query = QueryBuilder::new()
        .contains("email", "@example.com")
        .starts_with("name", "John")
        .ends_with("domain", ".com")
        .build();

    assert!(query.filter.is_some());
}

#[test]
fn test_query_builder_logical_operators() {
    let query = QueryBuilder::new()
        .or(vec![
            json!({"type": "Condition", "content": {"field": "status", "operator": "Eq", "value": "active"}}),
            json!({"type": "Condition", "content": {"field": "status", "operator": "Eq", "value": "pending"}}),
        ])
        .build();

    assert!(query.filter.is_some());
    let filter = query.filter.unwrap();
    assert_eq!(filter["type"], "Logical");
    assert_eq!(filter["content"]["operator"], "Or");
}

#[test]
fn test_query_builder_sorting() {
    let query = QueryBuilder::new()
        .sort_desc("created_at")
        .sort_asc("name")
        .build();

    assert!(query.sort.is_some());
    let sort = query.sort.unwrap();
    let sort_arr = sort.as_array().unwrap();
    assert_eq!(sort_arr.len(), 2);
}

#[test]
fn test_query_builder_pagination() {
    let query = QueryBuilder::new().page(2, 20).build();

    assert_eq!(query.limit, Some(20));
    assert_eq!(query.skip, Some(40)); // page 2 * 20 items = skip 40
}

#[test]
fn test_query_builder_flags() {
    let query = QueryBuilder::new()
        .bypass_cache(true)
        .bypass_ripple(true)
        .build();

    assert_eq!(query.bypass_cache, Some(true));
    assert_eq!(query.bypass_ripple, Some(true));
}

// ============================================================================
// SearchQuery Tests
// ============================================================================

#[test]
fn test_search_query_builder() {
    let query = SearchQuery::new("rust programming")
        .language("english")
        .fuzzy(true)
        .min_score(0.5)
        .limit(10);

    assert_eq!(query.query, "rust programming");
    assert_eq!(query.language, Some("english".to_string()));
    assert_eq!(query.fuzzy, Some(true));
    assert_eq!(query.min_score, Some(0.5));
    assert_eq!(query.limit, Some(10));
}

#[test]
fn test_search_query_vector_params() {
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
fn test_search_query_hybrid() {
    let query = SearchQuery::new("test").text_weight(0.7).vector_weight(0.3);

    assert_eq!(query.text_weight, Some(0.7));
    assert_eq!(query.vector_weight, Some(0.3));
}

#[test]
fn test_search_query_text_options() {
    let query = SearchQuery::new("test")
        .fields("name,description")
        .weights("name:2.0,description:1.0")
        .enable_stemming(true)
        .boost_exact(true)
        .case_sensitive(false)
        .max_edit_distance(2);

    assert_eq!(query.fields, Some("name,description".to_string()));
    assert_eq!(query.weights, Some("name:2.0,description:1.0".to_string()));
    assert_eq!(query.enable_stemming, Some(true));
    assert_eq!(query.boost_exact, Some(true));
    assert_eq!(query.case_sensitive, Some(false));
    assert_eq!(query.max_edit_distance, Some(2));
}

// ============================================================================
// Error Type Tests
// ============================================================================

#[test]
fn test_error_is_retryable() {
    assert!(Error::Timeout.is_retryable());
    assert!(Error::Connection("test".to_string()).is_retryable());
    assert!(Error::ServiceUnavailable("test".to_string()).is_retryable());
    assert!(
        Error::RateLimit {
            retry_after_secs: 30
        }
        .is_retryable()
    );

    assert!(!Error::NotFound.is_retryable());
    assert!(!Error::api(401, "unauthorized").is_retryable());
    assert!(!Error::Validation("test".to_string()).is_retryable());
}

#[test]
fn test_error_retry_delay() {
    assert_eq!(
        Error::RateLimit {
            retry_after_secs: 60
        }
        .retry_delay_secs(),
        Some(60)
    );
    assert_eq!(
        Error::ServiceUnavailable("test".to_string()).retry_delay_secs(),
        Some(10)
    );
    assert_eq!(Error::Timeout.retry_delay_secs(), Some(5));
    assert_eq!(
        Error::Connection("test".to_string()).retry_delay_secs(),
        Some(3)
    );
    assert_eq!(Error::NotFound.retry_delay_secs(), None);
}

#[test]
fn test_error_display() {
    let err = Error::api(404, "Resource not found");
    assert_eq!(format!("{}", err), "API error (404): Resource not found");

    let err = Error::Timeout;
    assert_eq!(format!("{}", err), "Operation timed out");

    let err = Error::NotFound;
    assert_eq!(format!("{}", err), "Record not found");
}

// ============================================================================
// Restore Operations Tests
// ============================================================================

#[tokio::test]
async fn test_restore_deleted_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _restore_mock = server
        .mock("POST", "/api/trash/users/record_123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"status": "restored"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.restore_deleted("users", "record_123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_restore_collection_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _restore_mock = server
        .mock("POST", "/api/trash/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"status": "restored", "records_restored": 5}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.restore_collection("users").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);
}

// ============================================================================
// KV Advanced Tests
// ============================================================================

#[tokio::test]
async fn test_kv_exists_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _kv_mock = server
        .mock("GET", "/api/kv/get/test_key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"value": "exists"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.kv_exists("test_key").await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

// Note: kv_exists_not_found requires specific error handling - covered by integration tests

// ============================================================================
// Batch Update Test
// ============================================================================

// Note: batch_update and find_all require complex types - covered by integration tests

// ============================================================================
// Transaction Status Test
// ============================================================================

#[tokio::test]
async fn test_get_transaction_status_success() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _tx_mock = server
        .mock("GET", "/api/transactions/tx_123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"transaction_id": "tx_123", "status": "active"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.get_transaction_status("tx_123").await;
    assert!(result.is_ok());
}

// Note: Functions, Chat, Search tests require complex mock setup
// These are covered by integration tests

// ============================================================================
// Convenience Methods Tests
// ============================================================================

#[tokio::test]
async fn test_upsert_inserts_when_not_found() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    // Mock update endpoint returning 404
    let _update_mock = server
        .mock("PUT", "/api/update/users/user123")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(json!({"error": "Not found"}).to_string())
        .create_async()
        .await;

    // Mock insert endpoint succeeding
    let _insert_mock = server
        .mock("POST", "/api/insert/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "user123", "name": "John Doe"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let record = Record::new().field("name", "John Doe");
    let result = client.upsert("users", "user123", record, None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_upsert_updates_when_exists() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    // Mock update endpoint succeeding
    let _update_mock = server
        .mock("PUT", "/api/update/users/user123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "user123", "name": "John Doe Updated"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let record = Record::new().field("name", "John Doe Updated");
    let result = client.upsert("users", "user123", record, None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_find_one_returns_some_when_found() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _find_mock = server
        .mock("POST", "/api/find/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([{"id": "user123", "email": "test@example.com"}]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.find_one("users", "email", "test@example.com").await;

    assert!(result.is_ok());
    let record = result.unwrap();
    assert!(record.is_some());
}

#[tokio::test]
async fn test_find_one_returns_none_when_not_found() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _find_mock = server
        .mock("POST", "/api/find/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client
        .find_one("users", "email", "notfound@example.com")
        .await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_exists_returns_true_when_found() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _find_mock = server
        .mock("GET", "/api/find/users/user123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "user123", "name": "John"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.exists("users", "user123").await;

    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_exists_returns_false_when_not_found() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _find_mock = server
        .mock("GET", "/api/find/users/user123")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(json!({"error": "Not found"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.exists("users", "user123").await;

    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_paginate_calculates_skip_correctly() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    // Page 2 with page_size 10 should skip 10 records
    let _find_mock = server
        .mock("POST", "/api/find/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([{"id": "user11", "name": "User 11"}]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.paginate("users", 2, 10).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn test_paginate_first_page_skips_zero() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    // Page 1 with page_size 10 should skip 0 records
    let _find_mock = server
        .mock("POST", "/api/find/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([{"id": "user1", "name": "User 1"}]).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client.paginate("users", 1, 10).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn test_record_field_builder() {
    let record = Record::new()
        .field("name", "John Doe")
        .field("age", 30)
        .field("active", true);

    assert_eq!(record.len(), 3);
    assert!(record.contains_key("name"));
    assert!(record.contains_key("age"));
    assert!(record.contains_key("active"));
}

#[test]
fn test_record_field_builder_chaining() {
    let record = Record::new()
        .field("field1", "value1")
        .field("field2", 42)
        .field("field3", true)
        .field("field4", "value4");

    assert_eq!(record.len(), 4);
}

#[test]
fn test_record_field_builder_empty() {
    let record = Record::new();
    assert!(record.is_empty());
    assert_eq!(record.len(), 0);
}

// ============================================================================
// SWR Function Tests
// ============================================================================

#[test]
fn test_swr_function_serialization() {
    use ekodb_client::Function;
    use std::collections::HashMap;

    let swr = Function::SWR {
        cache_key: "user:{{user_id}}".to_string(),
        ttl: json!("15m"),
        url: "https://api.example.com/users/{{user_id}}".to_string(),
        method: "GET".to_string(),
        headers: Some(HashMap::from([(
            "User-Agent".to_string(),
            "ekoDB-Client".to_string(),
        )])),
        body: None,
        timeout_seconds: Some(30),
        output_field: Some("user_data".to_string()),
        collection: None,
    };

    let serialized = serde_json::to_value(&swr).expect("Should serialize");

    assert_eq!(serialized["cache_key"], "user:{{user_id}}");
    assert_eq!(serialized["ttl"], "15m");
    assert_eq!(
        serialized["url"],
        "https://api.example.com/users/{{user_id}}"
    );
    assert_eq!(serialized["method"], "GET");
    assert_eq!(serialized["timeout_seconds"], 30);
    assert_eq!(serialized["output_field"], "user_data");
}

#[test]
fn test_swr_function_with_audit_collection() {
    use ekodb_client::Function;

    let swr = Function::SWR {
        cache_key: "product:{{id}}".to_string(),
        ttl: json!("1h"),
        url: "https://api.example.com/products/{{id}}".to_string(),
        method: "GET".to_string(),
        headers: None,
        body: None,
        timeout_seconds: None,
        output_field: Some("product".to_string()),
        collection: Some("swr_audit_trail".to_string()),
    };

    let serialized = serde_json::to_value(&swr).expect("Should serialize");

    assert_eq!(serialized["collection"], "swr_audit_trail");
}

#[test]
fn test_swr_function_with_post_body() {
    use ekodb_client::Function;

    let swr = Function::SWR {
        cache_key: "api:{{resource}}".to_string(),
        ttl: json!(900), // Integer seconds
        url: "https://api.example.com/resource".to_string(),
        method: "POST".to_string(),
        headers: None,
        body: Some(json!({"query": "{{query}}"})),
        timeout_seconds: Some(60),
        output_field: None,
        collection: None,
    };

    let serialized = serde_json::to_value(&swr).expect("Should serialize");

    assert_eq!(serialized["method"], "POST");
    assert_eq!(serialized["body"]["query"], "{{query}}");
    assert_eq!(serialized["ttl"], 900);
}

#[test]
fn test_swr_function_ttl_formats() {
    use ekodb_client::Function;

    // Duration string
    let swr1 = Function::SWR {
        cache_key: "test".to_string(),
        ttl: json!("30m"),
        url: "https://example.com".to_string(),
        method: "GET".to_string(),
        headers: None,
        body: None,
        timeout_seconds: None,
        output_field: None,
        collection: None,
    };
    let ser1 = serde_json::to_value(&swr1).unwrap();
    assert_eq!(ser1["ttl"], "30m");

    // Integer seconds
    let swr2 = Function::SWR {
        cache_key: "test".to_string(),
        ttl: json!(1800),
        url: "https://example.com".to_string(),
        method: "GET".to_string(),
        headers: None,
        body: None,
        timeout_seconds: None,
        output_field: None,
        collection: None,
    };
    let ser2 = serde_json::to_value(&swr2).unwrap();
    assert_eq!(ser2["ttl"], 1800);

    // ISO timestamp
    let swr3 = Function::SWR {
        cache_key: "test".to_string(),
        ttl: json!("2024-12-31T23:59:59Z"),
        url: "https://example.com".to_string(),
        method: "GET".to_string(),
        headers: None,
        body: None,
        timeout_seconds: None,
        output_field: None,
        collection: None,
    };
    let ser3 = serde_json::to_value(&swr3).unwrap();
    assert_eq!(ser3["ttl"], "2024-12-31T23:59:59Z");
}

#[test]
fn test_swr_function_deserialization() {
    use ekodb_client::Function;

    let json_data = json!({
        "type": "SWR",
        "cache_key": "user:123",
        "ttl": "15m",
        "url": "https://api.example.com/users/123",
        "method": "GET",
        "output_field": "user_data"
    });

    let function: Function = serde_json::from_value(json_data).expect("Should deserialize");

    match function {
        Function::SWR {
            cache_key,
            ttl,
            url,
            method,
            output_field,
            ..
        } => {
            assert_eq!(cache_key, "user:123");
            assert_eq!(ttl, json!("15m"));
            assert_eq!(url, "https://api.example.com/users/123");
            assert_eq!(method, "GET");
            assert_eq!(output_field, Some("user_data".to_string()));
        }
        _ => panic!("Expected SWR function"),
    }
}

// ============================================================================
// Atomic Field Action Tests
// ============================================================================

#[tokio::test]
async fn test_update_with_action_increment() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _action_mock = server
        .mock("PUT", "/api/update/counters/rec_1/action/increment")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "rec_1",
                "views": 42
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client
        .update_with_action(
            "counters",
            "rec_1",
            "increment",
            "views",
            FieldType::Number(ekodb_client::NumberValue::Integer(1)),
        )
        .await;

    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.get_string("id").unwrap(), "rec_1");
}

#[tokio::test]
async fn test_update_with_action_push() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _action_mock = server
        .mock("PUT", "/api/update/lists/rec_2/action/push")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "rec_2",
                "tags": ["rust", "new-tag"]
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client
        .update_with_action(
            "lists",
            "rec_2",
            "push",
            "tags",
            FieldType::String("new-tag".into()),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_with_action_clear() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _action_mock = server
        .mock("PUT", "/api/update/data/rec_3/action/clear")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "rec_3", "temp": 0}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client
        .update_with_action("data", "rec_3", "clear", "temp", FieldType::Null)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_with_action_not_found() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _action_mock = server
        .mock("PUT", "/api/update/counters/missing/action/increment")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(json!({"error": "Record not found"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let result = client
        .update_with_action(
            "counters",
            "missing",
            "increment",
            "views",
            FieldType::Number(ekodb_client::NumberValue::Integer(1)),
        )
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_with_action_sequence() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _seq_mock = server
        .mock("PUT", "/api/update/sequence/game/player_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "player_1",
                "score": 110,
                "lives": 2,
                "log": ["hit"]
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let actions = vec![
        (
            "increment".to_string(),
            "score".to_string(),
            FieldType::Number(ekodb_client::NumberValue::Integer(10)),
        ),
        (
            "decrement".to_string(),
            "lives".to_string(),
            FieldType::Number(ekodb_client::NumberValue::Integer(1)),
        ),
        (
            "push".to_string(),
            "log".to_string(),
            FieldType::String("hit".into()),
        ),
    ];

    let result = client
        .update_with_action_sequence("game", "player_1", actions)
        .await;

    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.get_string("id").unwrap(), "player_1");
}

#[tokio::test]
async fn test_update_with_action_sequence_error() {
    let mut server = Server::new_async().await;

    let _token_mock = mock_token_endpoint(&mut server);

    let _seq_mock = server
        .mock("PUT", "/api/update/sequence/data/rec_x")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            json!({"error": "Failed to apply action 'increment': field not numeric"}).to_string(),
        )
        .create_async()
        .await;

    let client = create_test_client(&server).await;

    let actions = vec![(
        "increment".to_string(),
        "name".to_string(),
        FieldType::Number(ekodb_client::NumberValue::Integer(1)),
    )];

    let result = client
        .update_with_action_sequence("data", "rec_x", actions)
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Goal CRUD Tests
// ============================================================================

#[tokio::test]
async fn test_goal_create() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/goals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1", "title": "Test Goal", "status": "active"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.goal_create(json!({"title": "Test Goal"})).await;
    assert!(result.is_ok());
    let goal = result.unwrap();
    assert_eq!(goal["id"], "goal_1");
    assert_eq!(goal["title"], "Test Goal");
}

#[tokio::test]
async fn test_goal_list() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/goals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"goals": [{"id": "goal_1"}, {"id": "goal_2"}]}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.goal_list().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_goal_get() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/goals/goal_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1", "title": "Test Goal"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.goal_get("goal_1").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["id"], "goal_1");
}

#[tokio::test]
async fn test_goal_update() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("PUT", "/api/chat/goals/goal_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1", "title": "Updated"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .goal_update("goal_1", json!({"title": "Updated"}))
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["title"], "Updated");
}

#[tokio::test]
async fn test_goal_delete() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("DELETE", "/api/chat/goals/goal_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"ok": true}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.goal_delete("goal_1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_goal_search() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock(
            "GET",
            Matcher::Regex(r"/api/chat/goals/search\?q=.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"goals": [{"id": "goal_1"}]}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.goal_search("test query").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_goal_complete() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/goals/goal_1/complete")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1", "status": "pending_review"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .goal_complete("goal_1", json!({"summary": "Done"}))
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "pending_review");
}

#[tokio::test]
async fn test_goal_approve() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/goals/goal_1/approve")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1", "status": "in_progress"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.goal_approve("goal_1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_goal_reject() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/goals/goal_1/reject")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1", "status": "failed"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .goal_reject("goal_1", json!({"reason": "Bad plan"}))
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "failed");
}

#[tokio::test]
async fn test_goal_step_start() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/goals/goal_1/steps/0/start")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.goal_step_start("goal_1", 0).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_goal_step_complete() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/goals/goal_1/steps/0/complete")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .goal_step_complete("goal_1", 0, json!({"result": "Step done"}))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_goal_step_fail() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/goals/goal_1/steps/0/fail")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "goal_1"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .goal_step_fail("goal_1", 0, json!({"error": "Step failed"}))
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// Task CRUD Tests
// ============================================================================

#[tokio::test]
async fn test_task_create() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/tasks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "task_1", "name": "Test Task", "status": "active"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .task_create(json!({"name": "Test Task", "cron": "0 * * * *"}))
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["id"], "task_1");
}

#[tokio::test]
async fn test_task_list() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/tasks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"tasks": []}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.task_list().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_task_get() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/tasks/task_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "task_1", "name": "Test Task"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.task_get("task_1").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["id"], "task_1");
}

#[tokio::test]
async fn test_task_update() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("PUT", "/api/chat/tasks/task_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "task_1", "name": "Updated"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .task_update("task_1", json!({"name": "Updated"}))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_task_delete() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("DELETE", "/api/chat/tasks/task_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"ok": true}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.task_delete("task_1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_task_due() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock(
            "GET",
            Matcher::Regex(r"/api/chat/tasks/due\?now=.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"tasks": []}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.task_due("2026-03-20T00:00:00Z").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_task_start() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/tasks/task_1/start")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "task_1", "status": "running"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.task_start("task_1").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "running");
}

#[tokio::test]
async fn test_task_succeed() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/tasks/task_1/succeed")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "task_1", "status": "active"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .task_succeed("task_1", json!({"output": "Success"}))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_task_fail() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/tasks/task_1/fail")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "task_1", "status": "active"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .task_fail("task_1", json!({"error": "Timeout"}))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_task_pause() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/tasks/task_1/pause")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "task_1", "status": "paused"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.task_pause("task_1").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "paused");
}

#[tokio::test]
async fn test_task_resume() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/tasks/task_1/resume")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "task_1", "status": "active"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .task_resume("task_1", json!({"next_run": "2026-03-21T00:00:00Z"}))
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// Agent CRUD Tests
// ============================================================================

#[tokio::test]
async fn test_agent_create() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("POST", "/api/chat/agents")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "agent_1", "name": "TestAgent"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .agent_create(json!({"name": "TestAgent", "system_prompt": "You help."}))
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["name"], "TestAgent");
}

#[tokio::test]
async fn test_agent_list() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/agents")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"agents": []}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.agent_list().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agent_get() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/agents/agent_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "agent_1", "name": "TestAgent"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.agent_get("agent_1").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["id"], "agent_1");
}

#[tokio::test]
async fn test_agent_get_by_name() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/agents/by-name/TestAgent")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "agent_1", "name": "TestAgent"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.agent_get_by_name("TestAgent").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["name"], "TestAgent");
}

#[tokio::test]
async fn test_agent_update() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("PUT", "/api/chat/agents/agent_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "agent_1", "name": "Updated"}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client
        .agent_update("agent_1", json!({"name": "Updated"}))
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["name"], "Updated");
}

#[tokio::test]
async fn test_agent_delete() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("DELETE", "/api/chat/agents/agent_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"ok": true}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.agent_delete("agent_1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agents_by_deployment() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/agents/by-deployment/deploy_1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"agents": [{"id": "agent_1"}]}).to_string())
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.agents_by_deployment("deploy_1").await;
    assert!(result.is_ok());
}

// ============================================================================
// Goal/Task/Agent Error Tests
// ============================================================================

#[tokio::test]
async fn test_goal_get_not_found() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/goals/nonexistent")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body("Not Found")
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.goal_get("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_task_get_not_found() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/tasks/nonexistent")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body("Not Found")
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.task_get("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_agent_get_not_found() {
    let mut server = Server::new_async().await;
    let _token_mock = mock_token_endpoint(&mut server);
    let _mock = server
        .mock("GET", "/api/chat/agents/nonexistent")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body("Not Found")
        .create_async()
        .await;

    let client = create_test_client(&server).await;
    let result = client.agent_get("nonexistent").await;
    assert!(result.is_err());
}

// ============================================================================
// parameter_ref helper — structural placeholders across mutation stages
// ============================================================================
//
// Feature parity with Stage.param (TS), Parameter (Go), parameter_ref
// (Python), and Parameter (Kotlin). Verifies the JSON shape + that
// Function::Insert, UpdateById, Update, and BatchInsert accept a structural
// parameter placeholder in their record/updates/filter fields.
// Requires ekoDB >= 0.41.0 for the server to actually resolve these.

#[test]
fn test_parameter_ref_shape() {
    use ekodb_client::parameter_ref;
    let v = parameter_ref("record");
    assert_eq!(v["type"], "Parameter");
    assert_eq!(v["name"], "record");
}

#[test]
fn test_parameter_ref_arbitrary_name() {
    use ekodb_client::parameter_ref;
    assert_eq!(parameter_ref("user_id")["name"], "user_id");
    assert_eq!(parameter_ref("updates")["name"], "updates");
}

#[test]
fn test_insert_function_accepts_structural_parameter() {
    use ekodb_client::{Function, parameter_ref};
    let stage = Function::Insert {
        collection: "users".to_string(),
        record: parameter_ref("record"),
        bypass_ripple: None,
        ttl: None,
    };

    let json = serde_json::to_value(&stage).expect("serialize");
    assert_eq!(json["type"], "Insert");
    assert_eq!(json["collection"], "users");
    assert_eq!(json["record"]["type"], "Parameter");
    assert_eq!(json["record"]["name"], "record");
}

#[test]
fn test_insert_function_accepts_per_field_placeholders() {
    use ekodb_client::{Function, parameter_ref};
    let stage = Function::Insert {
        collection: "items".to_string(),
        record: serde_json::json!({
            "label": "{{label}}",
            "parent_id": parameter_ref("parent_id"),
            "kind": "item",
            "tags": parameter_ref("tags"),
        }),
        bypass_ripple: None,
        ttl: None,
    };

    let json = serde_json::to_value(&stage).expect("serialize");
    let record = &json["record"];
    assert_eq!(record["label"], "{{label}}");
    assert_eq!(record["kind"], "item");
    assert_eq!(record["parent_id"]["type"], "Parameter");
    assert_eq!(record["parent_id"]["name"], "parent_id");
    assert_eq!(record["tags"]["type"], "Parameter");
    assert_eq!(record["tags"]["name"], "tags");
}

#[test]
fn test_update_by_id_accepts_structural_parameter() {
    use ekodb_client::{Function, parameter_ref};
    let stage = Function::UpdateById {
        collection: "items".to_string(),
        record_id: "{{id}}".to_string(),
        updates: parameter_ref("updates"),
        bypass_ripple: None,
        ttl: None,
    };

    let json = serde_json::to_value(&stage).expect("serialize");
    assert_eq!(json["type"], "UpdateById");
    assert_eq!(json["record_id"], "{{id}}");
    assert_eq!(json["updates"]["type"], "Parameter");
    assert_eq!(json["updates"]["name"], "updates");
}

#[test]
fn test_update_with_structural_filter_and_updates() {
    use ekodb_client::{Function, parameter_ref};
    let filter = serde_json::json!({
        "type": "Condition",
        "content": {
            "field": "id",
            "operator": "Eq",
            "value": parameter_ref("id"),
        }
    });
    let stage = Function::Update {
        collection: "items".to_string(),
        filter,
        updates: parameter_ref("updates"),
        bypass_ripple: None,
        ttl: None,
    };

    let json = serde_json::to_value(&stage).expect("serialize");
    let filter_value = &json["filter"]["content"]["value"];
    assert_eq!(filter_value["type"], "Parameter");
    assert_eq!(filter_value["name"], "id");
    assert_eq!(json["updates"]["type"], "Parameter");
    assert_eq!(json["updates"]["name"], "updates");
}

#[test]
fn test_batch_insert_accepts_per_record_structural_parameters() {
    use ekodb_client::{Function, parameter_ref};
    let stage = Function::BatchInsert {
        collection: "audit_log".to_string(),
        records: serde_json::json!([
            {
                "actor": parameter_ref("user_id"),
                "at": "{{now}}",
                "message": "created"
            },
            {
                "actor": parameter_ref("user_id"),
                "at": "{{now}}",
                "message": "initialized"
            },
        ]),
        bypass_ripple: None,
    };

    let json = serde_json::to_value(&stage).expect("serialize");
    assert_eq!(json["type"], "BatchInsert");
    let records = json["records"].as_array().expect("records is array");
    assert_eq!(records.len(), 2);
    assert_eq!(records[0]["actor"]["name"], "user_id");
    assert_eq!(records[0]["at"], "{{now}}");
    assert_eq!(records[1]["message"], "initialized");
}

// ============================================================================
// Crypto primitives: BcryptHash, BcryptVerify, RandomToken (ekoDB >= 0.41.0)
// ============================================================================
//
// Serialization-shape tests only — runtime behavior is covered by the
// server-side tests in `ekodb/ekodb_server/tests/function_parameters_tests.rs`.

#[test]
fn test_bcrypt_hash_stage_serializes_with_text_placeholder() {
    use ekodb_client::Function;
    let stage = Function::BcryptHash {
        plain: "{{password}}".to_string(),
        cost: Some(12),
        output_field: "password_hash".to_string(),
    };
    let json = serde_json::to_value(&stage).expect("serialize");
    assert_eq!(json["type"], "BcryptHash");
    assert_eq!(json["plain"], "{{password}}");
    assert_eq!(json["cost"], 12);
    assert_eq!(json["output_field"], "password_hash");
}

#[test]
fn test_bcrypt_hash_stage_omits_cost_when_none() {
    use ekodb_client::Function;
    let stage = Function::BcryptHash {
        plain: "{{password}}".to_string(),
        cost: None,
        output_field: "pw_hash".to_string(),
    };
    let json = serde_json::to_value(&stage).expect("serialize");
    assert!(
        json.get("cost").is_none() || json["cost"].is_null(),
        "cost must be omitted when None, got {:?}",
        json.get("cost")
    );
}

#[test]
fn test_bcrypt_verify_stage_serializes() {
    use ekodb_client::Function;
    let stage = Function::BcryptVerify {
        plain: "{{password}}".to_string(),
        hash_field: "password_hash".to_string(),
        output_field: "valid".to_string(),
    };
    let json = serde_json::to_value(&stage).expect("serialize");
    assert_eq!(json["type"], "BcryptVerify");
    assert_eq!(json["plain"], "{{password}}");
    assert_eq!(json["hash_field"], "password_hash");
    assert_eq!(json["output_field"], "valid");
}

#[test]
fn test_random_token_stage_serializes_with_hex_default() {
    use ekodb_client::Function;
    let stage = Function::RandomToken {
        bytes: 32,
        encoding: Some("hex".to_string()),
        output_field: "session_token".to_string(),
    };
    let json = serde_json::to_value(&stage).expect("serialize");
    assert_eq!(json["type"], "RandomToken");
    assert_eq!(json["bytes"], 32);
    assert_eq!(json["encoding"], "hex");
    assert_eq!(json["output_field"], "session_token");
}

#[test]
fn test_random_token_stage_omits_encoding_when_none() {
    use ekodb_client::Function;
    let stage = Function::RandomToken {
        bytes: 16,
        encoding: None,
        output_field: "token".to_string(),
    };
    let json = serde_json::to_value(&stage).expect("serialize");
    assert!(
        json.get("encoding").is_none() || json["encoding"].is_null(),
        "encoding must be omitted when None",
    );
}

#[test]
fn test_crypto_stages_roundtrip_through_serde() {
    use ekodb_client::Function;
    let stages = [
        Function::BcryptHash {
            plain: "{{password}}".to_string(),
            cost: Some(12),
            output_field: "password_hash".to_string(),
        },
        Function::BcryptVerify {
            plain: "{{password}}".to_string(),
            hash_field: "password_hash".to_string(),
            output_field: "valid".to_string(),
        },
        Function::RandomToken {
            bytes: 32,
            encoding: Some("base64url".to_string()),
            output_field: "token".to_string(),
        },
    ];

    for stage in stages {
        let json = serde_json::to_value(&stage).expect("serialize");
        let back: Function = serde_json::from_value(json.clone()).expect("deserialize");
        let json2 = serde_json::to_value(&back).expect("re-serialize");
        assert_eq!(
            json, json2,
            "crypto stage must round-trip through serde unchanged"
        );
    }
}
