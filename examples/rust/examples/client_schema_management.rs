//! Schema Management Example
//!
//! This example demonstrates basic schema management operations.
//!
//! Prerequisites:
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_schema_management`

use ekodb_client::{Client, FieldTypeSchema, Record, Schema};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=== ekoDB Schema Management Example ===\n");

    // Create client
    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let collection = "schema_client_rust";

    // Step 1: Create a collection with a simple schema
    println!("=== Creating Collection with Schema ===");

    // Delete collection if it exists (cleanup from previous runs)
    let _ = client.delete_collection(collection).await;

    let schema = Schema::new()
        .add_field("title", FieldTypeSchema::new("String").required())
        .add_field("email", FieldTypeSchema::new("String").required())
        .add_field("age", FieldTypeSchema::new("Integer"))
        .add_field("status", FieldTypeSchema::new("String"));

    client.create_collection(collection, schema).await?;
    println!("✓ Created collection '{}' with schema\n", collection);

    // Step 2: Insert documents that conform to schema
    println!("=== Inserting Valid Documents ===");

    let mut user1 = Record::new();
    user1.insert("title", "Software Engineer");
    user1.insert("email", "alice@example.com");
    user1.insert("age", 28);
    user1.insert("status", "active");

    let inserted = client.insert(collection, user1).await?;
    println!("✓ Inserted user 1: {:?}", inserted.get("id"));

    let mut user2 = Record::new();
    user2.insert("title", "Product Manager");
    user2.insert("email", "bob@example.com");
    user2.insert("age", 32);
    user2.insert("status", "pending");

    let inserted = client.insert(collection, user2).await?;
    println!("✓ Inserted user 2: {:?}\n", inserted.get("id"));

    // Step 3: Get schema
    println!("=== Getting Schema ===");

    let schema = client.get_schema(collection).await?;
    println!("✓ Schema for {}:", collection);
    for (name, field) in &schema.fields {
        println!("  - {}: {}", name, field.field_type);
        if field.required {
            println!("    (required)");
        }
    }
    println!();

    // Step 4: List all collections
    println!("=== Listing Collections ===");

    let collections = client.list_collections().await?;
    println!("✓ Total collections: {}", collections.len());
    println!(
        "  Sample: {:?}",
        collections.iter().take(5).collect::<Vec<_>>()
    );
    println!();

    // Cleanup: Delete the collection
    println!("=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection\n");

    println!("✓ All schema management operations completed successfully");

    Ok(())
}
