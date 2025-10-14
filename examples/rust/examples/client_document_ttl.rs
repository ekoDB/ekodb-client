//! Document TTL operations using ekodb_client library
//!
//! This example demonstrates TTL (Time To Live) operations using the ekodb_client library.
//!
//! Compare with document_ttl.rs to see the difference!

use ekodb_client::{Client, Query, Record};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Create ekoDB client
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("✓ Client created");

    let collection = "client_document_ttl_rust";

    // Example 1: Insert document with TTL (1 hour)
    println!("\n=== Insert Document with TTL (1 hour) ===");
    let mut record1 = Record::new();
    record1.insert("key", "session_token");
    record1.insert("value", "abc123");
    let record1 = record1.with_ttl_update_on_access("1h", true);

    let doc1 = client.insert(collection, record1).await?;
    if let Some(ekodb_client::FieldType::String(id)) = doc1.get("id") {
        println!("✓ Inserted document: {:?}", id);
    }

    // Example 2: Insert with shorter TTL (5 minutes)
    println!("\n=== Insert Document with TTL (5 minutes) ===");
    let mut record2 = Record::new();
    record2.insert("key", "temp_data");
    record2.insert("important", true);
    let record2 = record2.with_ttl("5m");

    let doc2 = client.insert(collection, record2).await?;
    println!("✓ Inserted document: {:?}", doc2.get("id"));

    // Example 3: Query documents
    println!("\n=== Query Documents ===");
    let query = Query::new().limit(10);
    let docs = client.find(collection, query).await?;
    println!("✓ Found {} documents with TTL", docs.len());

    // Example 4: Update document (TTL persists)
    println!("\n=== Update Document ===");
    if let Some(ekodb_client::FieldType::String(id)) = doc1.get("id") {
        let mut updates = Record::new();
        updates.insert("value", "updated_value");

        client.update(collection, id, updates).await?;
        println!("✓ Updated document");
    }

    // Example 5: Delete document
    println!("\n=== Delete Document ===");
    if let Some(id) = doc2.get("id") {
        if let ekodb_client::FieldType::String(id_str) = id {
            client.delete(collection, id_str).await?;
            println!("✓ Deleted document");
        }
    }

    // Cleanup: Delete the collection
    println!("\n=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection");

    println!("\n✓ All document TTL operations completed successfully");
    println!(
        "\n💡 Note: Documents with TTL will automatically expire after the specified duration"
    );

    Ok(())
}
