//! Document TTL operations using ekodb_client library
//!
//! This example demonstrates TTL (Time To Live) operations using the ekodb_client library.
//!
//! Compare with document_ttl.rs to see the difference!

use ekodb_client::{Client, Error, Query, Record};
use std::env;
use std::error::Error as StdError;

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
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

    let collection = "client_document_ttl_rust";

    if let Err(error) = client.delete_collection(collection).await {
        if !is_not_found(&error) {
            return Err(error.into());
        }
    }

    let operation_result: Result<(), Box<dyn StdError>> = async {
        // Example 1: Insert document with TTL (1 hour)
        println!("\n=== Insert Document with TTL (1 hour) ===");
        let mut record1 = Record::new();
        record1.insert("key", "session_token");
        record1.insert("value", "abc123");
        let record1 = record1.with_ttl_update_on_access("1h", true);

        let doc1 = client.insert(collection, record1, None).await?;
        let doc1_id = doc1
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("first insert response did not include a non-empty ID")?
            .to_string();
        println!("✓ Inserted document: {:?}", doc1_id);

        // Example 2: Insert with shorter TTL (5 minutes)
        println!("\n=== Insert Document with TTL (5 minutes) ===");
        let mut record2 = Record::new();
        record2.insert("key", "temp_data");
        record2.insert("important", true);
        let record2 = record2.with_ttl("5m");

        let doc2 = client.insert(collection, record2, None).await?;
        let doc2_id = doc2
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("second insert response did not include a non-empty ID")?
            .to_string();
        if doc2_id == doc1_id {
            return Err("the two inserts returned the same ID".into());
        }
        println!("✓ Inserted document: {:?}", doc2.get("id"));

        // Example 3: Query documents
        println!("\n=== Query Documents ===");
        let docs = client
            .find(collection, Query::new().limit(10), None)
            .await?;
        println!("✓ Found {} documents with TTL", docs.len());
        let mut ids: Vec<&str> = docs
            .iter()
            .map(|doc| doc.get_string("id").ok_or("queried document had no ID"))
            .collect::<Result<_, _>>()?;
        ids.sort_unstable();
        let mut expected = vec![doc1_id.as_str(), doc2_id.as_str()];
        expected.sort_unstable();
        if ids != expected {
            return Err(format!("TTL query returned unexpected IDs: {ids:?}").into());
        }

        // Example 4: Update document (TTL persists)
        println!("\n=== Update Document ===");
        let mut updates = Record::new();
        updates.insert("value", "updated_value");

        client.update(collection, &doc1_id, updates, None).await?;
        let stored = client.find_by_id(collection, &doc1_id, None).await?;
        if stored.get_string("value") != Some("updated_value") {
            return Err("updated TTL document did not contain the requested value".into());
        }
        println!("✓ Updated document");

        // Example 5: Delete document
        println!("\n=== Delete Document ===");
        client.delete(collection, &doc2_id, None).await?;
        println!("✓ Deleted document");
        let remaining = client
            .find(collection, Query::new().limit(10), None)
            .await?;
        if remaining.len() != 1 || remaining[0].get_string("id") != Some(doc1_id.as_str()) {
            return Err("delete did not leave exactly the first TTL document".into());
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

    println!("\n✓ All document TTL operations completed successfully");
    println!(
        "\n💡 Note: Documents with TTL will automatically expire after the specified duration"
    );

    Ok(())
}
