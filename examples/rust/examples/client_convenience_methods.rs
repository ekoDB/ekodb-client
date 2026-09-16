//! ekoDB Convenience Methods Example
//!
//! This example demonstrates the convenience methods added to the ekoDB Rust client:
//! - upsert: Insert or update a record
//! - find_one: Find a single record by field value
//! - exists: Check if a record exists
//! - paginate: Paginate through records
//! - Record.field(): Fluent builder for records

use ekodb_client::{Client, Error, FieldType, Record, Result};
use std::env;

fn record_id(record: &Record) -> Result<String> {
    match record.get("id") {
        Some(FieldType::String(id)) if !id.is_empty() => Ok(id.clone()),
        _ => Err(Error::Validation(
            "record response did not include an id".into(),
        )),
    }
}

fn integer_value(field: Option<&FieldType>) -> Option<i64> {
    match field {
        Some(FieldType::Integer(value)) => Some(*value),
        Some(FieldType::Object(wrapper)) => match wrapper.get("value") {
            Some(FieldType::Integer(value)) => Some(*value),
            _ => None,
        },
        _ => None,
    }
}

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
    let operation_result = async {
        println!("=== Record Builder Pattern ===");
        // Fluent builder for creating records
        let user1 = Record::new()
            .field("name", "Alice Johnson")
            .field("email", "alice@example.com")
            .field("age", 28)
            .field("active", true);

        let inserted = client.insert(collection, user1, None).await?;
        let alice_id = record_id(&inserted)?;
        println!("✓ Created record with fluent builder: {:?}", inserted);

        println!("\n=== Upsert Operation ===");
        // Upsert: Insert or update based on existence
        let user2 = Record::new()
            .field("name", "Bob Smith")
            .field("email", "bob@example.com")
            .field("age", 35);

        // First upsert - will insert (new ID doesn't exist yet)
        let upserted1 = client
            .upsert(collection, "bob-user-id", user2.clone(), None)
            .await?;
        let bob_id = record_id(&upserted1)?;
        println!("✓ First upsert (insert): {:?}", upserted1);

        // Second upsert - will update (same ID, already exists)
        let updated_user = Record::new()
            .field("name", "Bob Smith")
            .field("email", "bob.smith@newdomain.com")
            .field("age", 36);

        let upserted2 = client
            .upsert(collection, &bob_id, updated_user, None)
            .await?;
        if record_id(&upserted2)? != bob_id {
            return Err(Error::Validation(
                "second upsert returned a different record id".into(),
            ));
        }
        let stored_bob = client.find_by_id(collection, &bob_id, None).await?;
        if stored_bob.get_string("email") != Some("bob.smith@newdomain.com")
            || integer_value(stored_bob.get("age")) != Some(36)
        {
            return Err(Error::Validation(
                "second upsert did not update Bob's email and age".into(),
            ));
        }
        println!("✓ Second upsert (update): {:?}", upserted2);

        println!("\n=== Find One Operation ===");
        // Find a single record by any field
        let alice = client
            .find_one(collection, "email", "alice@example.com")
            .await?
            .ok_or_else(|| Error::Validation("Alice was not found by email".into()))?;
        println!("✓ Found user by email: {:?}", alice);

        // Try finding a non-existent user
        if let Some(user) = client
            .find_one(collection, "email", "nonexistent@example.com")
            .await?
        {
            return Err(Error::Validation(format!(
                "unexpectedly found nonexistent user: {user:?}"
            )));
        }
        println!("✓ User not found (as expected)");

        println!("\n=== Exists Check ===");
        // Check existence without fetching the record
        let exists = client.exists(collection, &alice_id).await?;
        if !exists {
            return Err(Error::Validation("Alice should exist".into()));
        }
        println!("✓ Record exists: {exists}");

        // Check non-existent record
        let not_exists = client.exists(collection, "fake-id-123").await?;
        if not_exists {
            return Err(Error::Validation("fake record should not exist".into()));
        }
        println!("✓ Fake record exists: {not_exists} (should be false)");

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
        let total = client.count_documents(collection).await?;
        if total != 27 {
            return Err(Error::Validation(format!(
                "collection contained {total} records; expected 27"
            )));
        }
        for page in 1..=3 {
            let records = client.paginate(collection, page, page_size).await?;
            let expected = if page == 3 { 7 } else { 10 };
            if records.len() != expected {
                return Err(Error::Validation(format!(
                    "page {page} returned {} records; expected {expected}",
                    records.len()
                )));
            }
            println!(
                "✓ Page {}: {} records (expected {})",
                page,
                records.len(),
                expected
            );
        }

        Ok::<(), Error>(())
    }
    .await;

    println!("\n=== Cleanup ===");
    let cleanup_result = client.delete_collection(collection).await;
    operation_result?;
    cleanup_result?;
    println!("✓ Deleted collection");

    println!("\n✅ All convenience methods demonstrated successfully!");
    Ok(())
}
