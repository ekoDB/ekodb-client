//! Collection management using ekodb_client library
//!
//! This example demonstrates collection operations using the ekodb_client library.
//!
//! Compare with collection_management.rs to see the difference!

use ekodb_client::{Client, Record};
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

    let collection = "client_collection_management_rust";

    // Example 1: Create collection (via insert)
    println!("\n=== Create Collection (via insert) ===");
    let mut record = Record::new();
    record.insert("name", "Demo Record");
    record.insert("value", 42);

    let inserted = client.insert(collection, record).await?;
    if let Some(ekodb_client::FieldType::String(id)) = inserted.get("id") {
        println!("Collection created with first record: {:?}", id);
    }

    // Example 2: List all collections
    println!("\n=== List Collections ===");
    let collections = client.list_collections().await?;
    println!("Total collections: {}", collections.len());
    println!(
        "Sample collections: {:?}",
        collections.iter().take(5).collect::<Vec<_>>()
    );

    // Example 3: Count documents in collection
    println!("\n=== Count Documents ===");
    let count = client.count_documents(collection).await?;
    println!("Document count: {}", count);

    // Example 4: Check if collection exists
    println!("\n=== Check Collection Exists ===");
    let exists = client.collection_exists(collection).await?;
    println!("Collection exists: {}", exists);

    // Example 5: Delete collection
    println!("\n=== Delete Collection ===");
    client.delete_collection(collection).await?;
    println!("Collection deleted successfully");

    // Example 6: Verify deletion
    println!("\n=== Verify Deletion ===");
    let still_exists = client.collection_exists(collection).await?;
    println!("Collection still exists: {}", still_exists);

    println!("\n✓ All collection management operations completed successfully");

    Ok(())
}
