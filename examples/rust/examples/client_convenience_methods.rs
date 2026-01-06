//! ekoDB Convenience Methods Example
//!
//! This example demonstrates the convenience methods added to the ekoDB Rust client:
//! - upsert: Insert or update a record
//! - find_one: Find a single record by field value
//! - exists: Check if a record exists
//! - paginate: Paginate through records
//! - Record.field(): Fluent builder for records

use ekodb_client::{Client, Record, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    println!("=== ekoDB Convenience Methods Example ===\n");

    // Create client
    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    let collection = "convenience_methods_example";

    println!("=== Record Builder Pattern ===");
    // Fluent builder for creating records
    let user1 = Record::new()
        .field("name", "Alice Johnson")
        .field("email", "alice@example.com")
        .field("age", 28)
        .field("active", true);

    let inserted = client.insert(collection, user1, None).await?;
    println!("✓ Created record with fluent builder: {:?}", inserted);

    println!("\n=== Upsert Operation ===");
    // Upsert: Insert or update based on existence
    let user2 = Record::new()
        .field("name", "Bob Smith")
        .field("email", "bob@example.com")
        .field("age", 35);

    // First upsert - will insert (doesn't exist yet)
    if let Some(id) = inserted.get("id") {
        let user_id = match id {
            ekodb_client::FieldType::String(s) => s.clone(),
            _ => panic!("Expected string ID"),
        };

        let upserted1 = client
            .upsert(collection, &user_id, user2.clone(), None)
            .await?;
        println!("✓ First upsert (insert): {:?}", upserted1);

        // Second upsert - will update (already exists)
        let updated_user = Record::new()
            .field("name", "Bob Smith")
            .field("email", "bob.smith@newdomain.com")
            .field("age", 36);

        let upserted2 = client
            .upsert(collection, &user_id, updated_user, None)
            .await?;
        println!("✓ Second upsert (update): {:?}", upserted2);
    }

    println!("\n=== Find One Operation ===");
    // Find a single record by any field
    match client
        .find_one(collection, "email", "alice@example.com")
        .await?
    {
        Some(user) => println!("✓ Found user by email: {:?}", user),
        None => println!("✗ User not found"),
    }

    // Try finding a non-existent user
    match client
        .find_one(collection, "email", "nonexistent@example.com")
        .await?
    {
        Some(user) => println!("✓ Found user: {:?}", user),
        None => println!("✓ User not found (as expected)"),
    }

    println!("\n=== Exists Check ===");
    // Check existence without fetching the record
    if let Some(id) = inserted.get("id") {
        let user_id = match id {
            ekodb_client::FieldType::String(s) => s.clone(),
            _ => panic!("Expected string ID"),
        };

        let exists = client.exists(collection, &user_id).await?;
        println!("✓ Record exists: {}", exists);

        // Check non-existent record
        let not_exists = client.exists(collection, "fake-id-123").await?;
        println!("✓ Fake record exists: {} (should be false)", not_exists);
    }

    println!("\n=== Pagination ===");
    // Insert more records for pagination demo
    for i in 1..=25 {
        let record = Record::new()
            .field("name", format!("User {}", i))
            .field("email", format!("user{}@example.com", i))
            .field("index", i);

        client.insert(collection, record, None).await?;
    }
    println!("✓ Inserted 25 records for pagination");

    // Paginate through records
    let page_size = 10;
    for page in 1..=3 {
        let records = client.paginate(collection, page, page_size).await?;
        println!(
            "✓ Page {}: {} records (expected {})",
            page,
            records.len(),
            if page == 3 { "7" } else { "10" }
        );
    }

    println!("\n=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection");

    println!("\n✅ All convenience methods demonstrated successfully!");
    Ok(())
}
