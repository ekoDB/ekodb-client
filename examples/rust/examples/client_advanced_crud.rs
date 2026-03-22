//! ekoDB Rust Client - Advanced CRUD Example
//!
//! Demonstrates atomic field actions, action sequences, and record/collection
//! restore (undelete) operations.
//!
//! Run with: `cargo run --example client_advanced_crud`

use ekodb_client::{Client, FieldType, Record};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB Advanced CRUD Example (Rust) ===\n");

    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let collection = "advanced_crud_example_rust";

    // =========================================================================
    // Setup: Insert a document with numeric and array fields
    // =========================================================================

    println!("--- Inserting base document ---");
    let mut record = Record::new();
    record.insert("name", "Widget Counter");
    record.insert("views", 100);
    record.insert("score", 4.5);
    record.insert("label", "prefix");
    record.insert(
        "tags",
        FieldType::array(vec![
            FieldType::string("rust"),
            FieldType::string("database"),
        ]),
    );

    let inserted = client.insert(collection, record, None).await?;
    let doc_id = inserted.get_string("id").unwrap_or_default().to_string();
    println!(
        "Inserted: {:?} (id: {})",
        inserted.get_string("name"),
        doc_id
    );

    // =========================================================================
    // update_with_action — single atomic field actions
    // =========================================================================

    // 1. Increment a numeric field
    println!("\n--- update_with_action: increment views by 10 ---");
    let result = client
        .update_with_action(
            collection,
            &doc_id,
            "increment",
            "views",
            FieldType::integer(10),
        )
        .await?;
    println!("views after increment: {:?}", result.get("views"));

    // 2. Decrement a numeric field
    println!("\n--- update_with_action: decrement views by 3 ---");
    let result = client
        .update_with_action(
            collection,
            &doc_id,
            "decrement",
            "views",
            FieldType::integer(3),
        )
        .await?;
    println!("views after decrement: {:?}", result.get("views"));

    // 3. Multiply a numeric field
    println!("\n--- update_with_action: multiply score by 2 ---");
    let result = client
        .update_with_action(
            collection,
            &doc_id,
            "multiply",
            "score",
            FieldType::float(2.0),
        )
        .await?;
    println!("score after multiply: {:?}", result.get("score"));

    // 4. Push to an array field
    println!("\n--- update_with_action: push 'ekodb' to tags ---");
    let result = client
        .update_with_action(
            collection,
            &doc_id,
            "push",
            "tags",
            FieldType::string("ekodb"),
        )
        .await?;
    println!("tags after push: {:?}", result.get("tags"));

    // 5. Append to a string field
    println!("\n--- update_with_action: append '-suffix' to label ---");
    let result = client
        .update_with_action(
            collection,
            &doc_id,
            "append",
            "label",
            FieldType::string("-suffix"),
        )
        .await?;
    println!("label after append: {:?}", result.get("label"));

    // 6. Pop from an array field
    println!("\n--- update_with_action: pop last element from tags ---");
    let result = client
        .update_with_action(collection, &doc_id, "pop", "tags", FieldType::Null)
        .await?;
    println!("tags after pop: {:?}", result.get("tags"));

    // 7. Remove a specific value from an array
    println!("\n--- update_with_action: remove 'rust' from tags ---");
    let result = client
        .update_with_action(
            collection,
            &doc_id,
            "remove",
            "tags",
            FieldType::string("rust"),
        )
        .await?;
    println!("tags after remove: {:?}", result.get("tags"));

    // =========================================================================
    // update_with_action_sequence — multiple atomic actions in one request
    // =========================================================================

    println!("\n--- update_with_action_sequence: increment views + push tag + append label ---");
    let actions = vec![
        (
            "increment".to_string(),
            "views".to_string(),
            FieldType::integer(50),
        ),
        (
            "push".to_string(),
            "tags".to_string(),
            FieldType::string("batch"),
        ),
        (
            "append".to_string(),
            "label".to_string(),
            FieldType::string("-v2"),
        ),
    ];
    let result = client
        .update_with_action_sequence(collection, &doc_id, actions)
        .await?;
    println!("views after sequence: {:?}", result.get("views"));
    println!("tags after sequence: {:?}", result.get("tags"));
    println!("label after sequence: {:?}", result.get("label"));

    // =========================================================================
    // restore_deleted — undelete a single record
    // =========================================================================

    println!("\n--- Delete then restore_deleted ---");
    client.delete(collection, &doc_id, None).await?;
    println!("Deleted document {}", doc_id);

    let restored = client.restore_deleted(collection, &doc_id).await?;
    println!("Restored: {} (expected true)", restored);

    // Verify restore
    let fetched = client.find_by_id(collection, &doc_id, None).await?;
    println!("Verified restored name: {:?}", fetched.get("name"));

    // =========================================================================
    // restore_collection — undelete all records in a collection
    // =========================================================================

    println!("\n--- Insert, delete multiple, then restore_collection ---");
    let mut doc2 = Record::new();
    doc2.insert("name", "Second Widget");
    doc2.insert("views", 0);
    let inserted2 = client.insert(collection, doc2, None).await?;
    let doc2_id = inserted2.get_string("id").unwrap_or_default().to_string();

    let mut doc3 = Record::new();
    doc3.insert("name", "Third Widget");
    doc3.insert("views", 0);
    let inserted3 = client.insert(collection, doc3, None).await?;
    let doc3_id = inserted3.get_string("id").unwrap_or_default().to_string();

    // Delete all three
    client.delete(collection, &doc_id, None).await?;
    client.delete(collection, &doc2_id, None).await?;
    client.delete(collection, &doc3_id, None).await?;
    println!("Deleted 3 documents");

    let count = client.restore_collection(collection).await?;
    println!("Restored {} records from collection trash", count);

    // =========================================================================
    // Cleanup
    // =========================================================================

    println!("\n--- Cleanup ---");
    client.delete(collection, &doc_id, None).await?;
    client.delete(collection, &doc2_id, None).await?;
    client.delete(collection, &doc3_id, None).await?;
    client.delete_collection(collection).await?;
    println!("Deleted collection");

    println!("\n=== All advanced CRUD operations completed ===");
    Ok(())
}
