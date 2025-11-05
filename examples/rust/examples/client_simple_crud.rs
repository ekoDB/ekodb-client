//! Simple CRUD operations using ekodb_client library
//!
//! This example demonstrates the same operations as simple_crud.rs
//! but using the ekodb_client library for cleaner, type-safe code.
//!
//! Compare with simple_crud.rs to see the difference!

use ekodb_client::{Client, FieldType, Record};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Create ekoDB client (automatically handles token exchange)
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("✓ Client created (token exchange happens automatically)");

    let collection = "client_simple_crud_rust";

    // Example 1: Insert a document
    println!("\n=== Insert Document ===");
    let mut record = Record::new();
    record.insert("name", "Test Record");
    record.insert("value", 42);
    record.insert("active", true);

    let inserted = client.insert(collection, record, None).await?;
    println!("Inserted: {:?}", inserted);

    // Extract the ID
    let doc_id = if let Some(id_field) = inserted.get("id") {
        match id_field {
            FieldType::String(id) => id.clone(),
            _ => return Err("Expected string ID".into()),
        }
    } else {
        return Err("No ID returned".into());
    };

    // Example 2: Find by ID
    println!("\n=== Find by ID ===");
    let found_doc = client.find_by_id(collection, &doc_id, None).await?;
    println!("Found: {:?}", found_doc);

    // Example 3: Find with query
    println!("\n=== Find with Query ===");
    use ekodb_client::Query;

    let query = Query::new()
        .filter(serde_json::json!({
            "type": "Condition",
            "content": {
                "field": "active",
                "operator": "Eq",
                "value": true
            }
        }))
        .limit(10);

    let docs = client.find(collection, query, None).await?;
    println!("Found documents: {:?}", docs);

    // Example 4: Update document
    println!("\n=== Update Document ===");
    let mut updates = Record::new();
    updates.insert("name", "Updated Record");
    updates.insert("value", 100);

    let updated = client.update(collection, &doc_id, updates, None).await?;
    println!("Updated: {:?}", updated);

    // Example 5: Delete document
    println!("\n=== Delete Document ===");
    client.delete(collection, &doc_id, None).await?;
    println!("Deleted document");

    // Cleanup: Delete the collection
    println!("\n=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection");

    println!("\n✓ All CRUD operations completed successfully");

    Ok(())
}
