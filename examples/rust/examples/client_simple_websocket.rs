//! Simple WebSocket operations using ekodb_client library
//!
//! This example demonstrates WebSocket queries using the ekodb_client library.
//!
//! Compare with simple_websocket.rs to see the difference!

use ekodb_client::{Client, Record};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Step 1: Insert some test data first via HTTP
    println!("\n=== Inserting Test Data ===");
    let mut record = Record::new();
    record.insert("name", "WebSocket Test Record");
    record.insert("value", 42);
    record.insert("active", true);

    let inserted = client.insert(collection, record).await?;
    if let Some(ekodb_client::FieldType::String(id)) = inserted.get("id") {
        println!("✓ Inserted test record: {}", id);
    }

    // Step 2: Connect to WebSocket and query data
    println!("\n=== Connecting to WebSocket ===");
    let ws_client = client.websocket(&ws_url).await?;
    println!("✓ WebSocket connected");

    // Step 3: Query data via WebSocket
    println!("\n=== Querying Data via WebSocket ===");
    let records = ws_client.find_all(collection).await?;
    println!("✓ Retrieved {} record(s) via WebSocket", records.len());

    // Show some of the data
    for (i, record) in records.iter().take(3).enumerate() {
        println!("  Record {}: {} fields", i + 1, record.len());
    }

    // Cleanup: Delete the collection
    println!("\n=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection");

    println!("\n✓ WebSocket example completed successfully");

    Ok(())
}
