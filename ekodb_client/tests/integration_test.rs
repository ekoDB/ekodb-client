//! Integration tests for ekodb_client
//!
//! These tests require a running ekoDB server.
//!
//! To run these tests:
//! 1. Start the ekoDB server: `make run`
//! 2. Set environment variables:
//!    ```bash
//!    export API_BASE_URL="http://localhost:8080"
//!    export API_BASE_KEY="a-test-api-key-from-ekodb"
//!    ```
//! 3. Run the ignored tests:
//!    ```bash
//!    cargo test -p ekodb_client --test integration_test -- --ignored
//!    ```

use ekodb_client::{Client, Query, Record};

#[tokio::test]
#[ignore] // Ignore by default since it requires a running server
async fn test_client_builder() {
    let result = Client::builder()
        .base_url("http://localhost:8080")
        .api_key("test-token")
        .build();

    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_insert_and_find() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .base_url("http://localhost:8080")
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    // Insert a test record
    let mut record = Record::new();
    record.insert("test_field", "test_value");
    record.insert("number", 42);

    let inserted = client.insert("test_collection", record).await?;
    assert!(inserted.get("id").is_some());

    // Find the record
    let query = Query::new();
    let results = client.find("test_collection", query).await?;
    assert!(!results.is_empty());

    Ok(())
}

#[test]
fn test_record_operations() {
    let mut record = Record::new();

    // Test insert
    record.insert("name", "John");
    record.insert("age", 30);

    // Test get
    assert!(record.get("name").is_some());
    assert!(record.get("age").is_some());
    assert!(record.get("nonexistent").is_none());

    // Test contains_key
    assert!(record.contains_key("name"));
    assert!(!record.contains_key("nonexistent"));

    // Test len
    assert_eq!(record.len(), 2);
    assert!(!record.is_empty());

    // Test remove
    let removed = record.remove("age");
    assert!(removed.is_some());
    assert_eq!(record.len(), 1);
}

#[test]
fn test_field_type_conversions() {
    use ekodb_client::FieldType;

    // Test From implementations
    let s: FieldType = "hello".into();
    assert!(matches!(s, FieldType::String(_)));

    let i: FieldType = 42i64.into();
    assert!(matches!(i, FieldType::Integer(42)));

    let f: FieldType = 3.14f64.into();
    assert!(matches!(f, FieldType::Float(_)));

    let b: FieldType = true.into();
    assert!(matches!(b, FieldType::Boolean(true)));
}
