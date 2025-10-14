//! WebSocket TTL operations using ekodb_client library
//!
//! This example demonstrates WebSocket queries with TTL documents using the ekodb_client library.
//!
//! Compare with websocket_ttl.rs to see the difference!

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

    let collection = "client_websocket_ttl_rust";

    // Step 1: Insert test data with TTL via HTTP
    println!("\n=== Insert Test Data with TTL ===");
    let mut record = Record::new();
    record.insert("name", "WebSocket TTL Test");
    record.insert("value", 42);
    let record = record.with_ttl("1h");

    let inserted = client.insert(collection, record).await?;
    println!("✓ Inserted document with TTL: {:?}", inserted.get("id"));

    // Step 2: Query via WebSocket
    println!("\n=== Query via WebSocket ===");
    let ws_client = client.websocket(&ws_url).await?;
    println!("✓ WebSocket connected");

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

    println!("\n✓ WebSocket TTL example completed successfully");
    println!(
        "\n💡 Note: Documents with TTL will automatically expire after the specified duration"
    );

    Ok(())
}
