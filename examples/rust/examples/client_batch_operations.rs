//! Batch operations using ekodb_client library
//!
//! This example demonstrates batch insert, update, and delete operations
//! using the ekodb_client library.
//!
//! Compare with batch_operations.rs to see the difference!

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

    let collection = "client_batch_operations_rust";

    // Example 1: Batch Insert (using individual inserts for now)
    // TODO: Add BatchBuilder support to client library
    println!("\n=== Batch Insert (via multiple inserts) ===");
    let mut inserted_ids = Vec::new();

    for i in 1..=5 {
        let mut record = Record::new();
        record.insert("name", format!("User {}", i));
        record.insert("email", format!("user{}@example.com", i));
        record.insert("age", 20 + (i * 5));

        let result = client.insert(collection, record).await?;

        // Extract ID
        if let Some(ekodb_client::FieldType::String(id)) = result.get("id") {
            inserted_ids.push(id.clone());
        }
    }

    println!("✓ Inserted {} records", inserted_ids.len());

    // Verify the inserts
    use ekodb_client::Query;
    let query = Query::new().limit(100);
    let docs = client.find(collection, query).await?;
    println!(
        "✓ Verified: Found {} total records in collection",
        docs.len()
    );

    // Example 2: Update records
    println!("\n=== Update Records ===");
    for (i, id) in inserted_ids.iter().take(3).enumerate() {
        let mut updates = Record::new();
        updates.insert("status", "active");
        updates.insert("updated_index", i as i64);

        client.update(collection, id, updates).await?;
    }
    println!("✓ Updated 3 records");

    // Example 3: Delete records
    println!("\n=== Delete Records ===");
    for id in inserted_ids.iter().take(3) {
        client.delete(collection, id).await?;
    }
    println!("✓ Deleted 3 records");

    // Cleanup: Delete the collection
    println!("\n=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection");

    println!("\n✓ All batch operations completed successfully");

    Ok(())
}
