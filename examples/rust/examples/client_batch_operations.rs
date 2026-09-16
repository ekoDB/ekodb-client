//! Batch operations using ekodb_client library
//!
//! This example demonstrates batch insert, update, and delete operations
//! using the ekodb_client library.
//!
//! Compare with batch_operations.rs to see the difference!

use ekodb_client::{Client, Error, FieldType, Query, Record};
use std::env;
use std::error::Error as StdError;

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
}

fn field<'a>(record: &'a Record, key: &str) -> Result<&'a FieldType, Box<dyn StdError>> {
    let value = record
        .get(key)
        .ok_or_else(|| format!("missing field '{key}'"))?;
    if let FieldType::Object(wrapper) = value {
        return wrapper
            .get("value")
            .ok_or_else(|| format!("field '{key}' wrapper has no value").into());
    }
    Ok(value)
}

fn integer(record: &Record, key: &str) -> Result<i64, Box<dyn StdError>> {
    match field(record, key)? {
        FieldType::Integer(value) => Ok(*value),
        other => Err(format!("field '{key}' was not an integer: {other:?}").into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
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

    if let Err(error) = client.delete_collection(collection).await {
        if !is_not_found(&error) {
            return Err(error.into());
        }
    }

    let operation_result: Result<(), Box<dyn StdError>> = async {
        // Example 1: Batch Insert (using individual inserts for now)
        // TODO: Add BatchBuilder support to client library
        println!("\n=== Batch Insert (via multiple inserts) ===");
        let mut inserted_ids = Vec::new();

        for i in 1..=5 {
            let mut record = Record::new();
            record.insert("name", format!("User {}", i));
            record.insert("email", format!("user{}@example.com", i));
            record.insert("age", 20 + (i * 5));

            let result = client.insert(collection, record, None).await?;
            let id = result
                .get_string("id")
                .filter(|id| !id.is_empty())
                .ok_or("insert response did not include a non-empty ID")?;
            if inserted_ids.iter().any(|existing| existing == id) {
                return Err(format!("insert returned duplicate ID {id}").into());
            }
            inserted_ids.push(id.to_string());
        }

        if inserted_ids.len() != 5 {
            return Err(format!("expected 5 inserted IDs, got {}", inserted_ids.len()).into());
        }
        println!("✓ Inserted {} records", inserted_ids.len());

        // Verify the inserts
        let docs = client
            .find(collection, Query::new().limit(100), None)
            .await?;
        println!(
            "✓ Verified: Found {} total records in collection",
            docs.len()
        );
        if docs.len() != 5 {
            return Err(format!("expected 5 inserted records, found {}", docs.len()).into());
        }

        // Example 2: Update records
        println!("\n=== Update Records ===");
        for (i, id) in inserted_ids.iter().take(3).enumerate() {
            let mut updates = Record::new();
            updates.insert("status", "active");
            updates.insert("updated_index", i as i64);

            client.update(collection, id, updates, None).await?;
            let stored = client.find_by_id(collection, id, None).await?;
            if stored.get_string("status") != Some("active")
                || integer(&stored, "updated_index")? != i as i64
            {
                return Err(format!("record {id} did not contain the requested update").into());
            }
        }
        println!("✓ Updated 3 records");

        // Example 3: Delete records
        println!("\n=== Delete Records ===");
        for id in inserted_ids.iter().take(3) {
            client.delete(collection, id, None).await?;
        }
        println!("✓ Deleted 3 records");

        let remaining = client
            .find(collection, Query::new().limit(100), None)
            .await?;
        if remaining.len() != 2 {
            return Err(
                format!("expected 2 records after delete, found {}", remaining.len()).into(),
            );
        }
        for record in &remaining {
            let id = record
                .get_string("id")
                .ok_or("remaining record did not include an ID")?;
            if inserted_ids[..3].iter().any(|deleted| deleted == id) {
                return Err(format!("deleted record {id} was still returned").into());
            }
        }

        Ok(())
    }
    .await;

    // Cleanup: Delete the collection
    println!("\n=== Cleanup ===");
    let cleanup_result = client.delete_collection(collection).await;
    if cleanup_result.is_ok() {
        println!("✓ Deleted collection");
    }

    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), _) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup.into()),
        (Ok(()), Ok(())) => {}
    }

    println!("\n✓ All batch operations completed successfully");

    Ok(())
}
