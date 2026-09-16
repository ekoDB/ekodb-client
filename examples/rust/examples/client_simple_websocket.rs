//! Simple WebSocket operations using ekodb_client library
//!
//! This example demonstrates WebSocket queries using the ekodb_client library.
//!
//! Compare with simple_websocket.rs to see the difference!

use ekodb_client::{Client, Error, FieldType, Record};
use std::env;
use std::error::Error as StdError;

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
}

fn field<'a>(record: &'a Record, key: &str) -> Result<&'a FieldType, Box<dyn StdError>> {
    let value = record
        .get(key)
        .ok_or_else(|| format!("missing WebSocket field '{key}'"))?;
    if let FieldType::Object(wrapper) = value {
        return wrapper
            .get("value")
            .ok_or_else(|| format!("field '{key}' wrapper has no value").into());
    }
    Ok(value)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let ws_url = env::var("WS_BASE_URL").unwrap_or_else(|_| "ws://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Create ekoDB client
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("✓ Client created");

    let collection = "client_simple_websocket_rust";

    if let Err(error) = client.delete_collection(collection).await {
        if !is_not_found(&error) {
            return Err(error.into());
        }
    }

    let operation_result: Result<(), Box<dyn StdError>> = async {
        // Step 1: Insert some test data first via HTTP
        println!("\n=== Inserting Test Data ===");
        let mut record = Record::new();
        record.insert("name", "WebSocket Test Record");
        record.insert("value", 42);
        record.insert("active", true);

        let inserted = client.insert(collection, record, None).await?;
        let inserted_id = inserted
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("insert response did not include a non-empty ID")?
            .to_string();
        println!("✓ Inserted test record: {}", inserted_id);

        // Step 2: Connect to WebSocket and query data
        println!("\n=== Connecting to WebSocket ===");
        let ws_client = client.websocket(&ws_url).await?;
        println!("✓ WebSocket connected");

        // Step 3: Query data via WebSocket
        println!("\n=== Querying Data via WebSocket ===");
        let records = ws_client.find_all(collection).await?;
        println!("✓ Retrieved {} record(s) via WebSocket", records.len());
        if records.len() != 1 {
            return Err(format!("expected 1 WebSocket record, got {}", records.len()).into());
        }
        let received = &records[0];
        if received.get_string("id") != Some(inserted_id.as_str()) {
            return Err("WebSocket record ID did not match the inserted ID".into());
        }
        match field(received, "name")? {
            FieldType::String(value) if value == "WebSocket Test Record" => {}
            value => return Err(format!("unexpected WebSocket name: {value:?}").into()),
        }
        match field(received, "value")? {
            FieldType::Integer(42) => {}
            value => return Err(format!("unexpected WebSocket value: {value:?}").into()),
        }
        match field(received, "active")? {
            FieldType::Boolean(true) => {}
            value => return Err(format!("unexpected WebSocket active field: {value:?}").into()),
        }

        // Show the data
        for (i, record) in records.iter().take(3).enumerate() {
            println!("\nRecord {}:", i + 1);
            for (key, value) in record.iter() {
                // Handle both direct values and wrapped Object variants with type metadata
                match value {
                    FieldType::String(s) => println!("  {}: \"{}\"", key, s),
                    FieldType::Integer(n) => println!("  {}: {}", key, n),
                    FieldType::Boolean(b) => println!("  {}: {}", key, b),
                    FieldType::Object(map) => {
                        // Extract value from {"type": "...", "value": ...} structure
                        if let Some(inner_value) = map.get("value") {
                            match inner_value {
                                FieldType::String(s) => println!("  {}: \"{}\"", key, s),
                                FieldType::Integer(n) => println!("  {}: {}", key, n),
                                FieldType::Boolean(b) => println!("  {}: {}", key, b),
                                _ => println!("  {}: {:?}", key, inner_value),
                            }
                        } else {
                            println!("  {}: {{object}}", key);
                        }
                    }
                    _ => println!("  {}: {:?}", key, value),
                }
            }
        }

        Ok(())
    }
    .await;

    // Cleanup: Delete the collection
    println!("\n=== Cleanup ===");
    let cleanup_result = client.delete_collection(collection).await;
    if cleanup_result.is_ok() {
        println!("✓ Deleted collection");
    }

    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), _) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup.into()),
        (Ok(()), Ok(())) => {}
    }

    println!("\n✓ WebSocket example completed successfully");

    Ok(())
}
