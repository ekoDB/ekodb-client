//! Collection Utilities Example - Using ekodb_client library
//!
//! This example demonstrates collection utility methods:
//! - collection_exists: Check if a collection exists
//! - count_documents: Count documents in a collection
//! - list_collections: List all collections
//! - delete_collection: Delete a collection

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

    let collection = "collection_utils_test_rust";

    // Example 1: Check if collection exists (should be false initially)
    println!("\n=== Check Collection Exists (Before Creation) ===");
    let exists = client.collection_exists(collection).await?;
    println!("Collection '{}' exists: {}", collection, exists);

    // Example 2: Create some test documents
    println!("\n=== Creating Test Documents ===");
    for i in 1..=5 {
        let mut record = Record::new();
        record.insert("name", format!("Test Document {}", i));
        record.insert("index", i);
        client.insert(collection, record, None).await?;
    }
    println!("Created 5 test documents");

    // Example 3: Check if collection exists now (should be true)
    println!("\n=== Check Collection Exists (After Creation) ===");
    let exists = client.collection_exists(collection).await?;
    println!("Collection '{}' exists: {}", collection, exists);

    // Example 4: Count documents in collection
    println!("\n=== Count Documents ===");
    let count = client.count_documents(collection).await?;
    println!("Document count in '{}': {}", collection, count);

    // Example 5: List all collections (our test collection should appear)
    println!("\n=== List Collections ===");
    let collections = client.list_collections().await?;
    println!("Total collections: {}", collections.len());
    for name in &collections {
        let marker = if name == collection {
            " <-- our test"
        } else {
            ""
        };
        println!("  - {}{}", name, marker);
    }

    // Example 6: Check non-existent collection
    println!("\n=== Check Non-Existent Collection ===");
    let exists = client
        .collection_exists("nonexistent_collection_xyz")
        .await?;
    println!("Collection 'nonexistent_collection_xyz' exists: {}", exists);

    // Cleanup: Delete the test collection
    println!("\n=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("Deleted collection '{}'", collection);

    println!("\n✓ Collection Utilities example complete");

    Ok(())
}
