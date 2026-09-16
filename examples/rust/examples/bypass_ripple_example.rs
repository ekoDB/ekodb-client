//! Example demonstrating bypass_ripple usage
//!
//! The bypass_ripple parameter allows you to skip ripple propagation for specific operations,
//! which is useful in multi-node deployments when you want to prevent data from being
//! automatically replicated to other nodes.

use ekodb_client::options::{InsertOptions, UpdateOptions, UpsertOptions};
use ekodb_client::{Client, Error, Record};
use std::env;

const COLLECTION: &str = "bypass_ripple_products_rs";

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    if let Err(error) = client.delete_collection(COLLECTION).await {
        if !is_not_found(&error) {
            return Err(error.into());
        }
    }

    let operation_result = run_examples(&client).await;
    let cleanup_result = match client.delete_collection(COLLECTION).await {
        Err(error) if is_not_found(&error) => Ok(()),
        result => result,
    };
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), Ok(())) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup.into()),
        (Ok(()), Ok(())) => {}
    }
    println!("✅ All bypass_ripple operations completed successfully!");
    Ok(())
}

async fn run_examples(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Bypass Ripple Example ===\n");

    // Basic insert (no bypass)
    println!("1. Basic insert (ripple enabled):");
    let record1 = Record::new().field("name", "Product 1").field("price", 100);
    let result1 = client.insert(COLLECTION, record1, None).await?;
    println!("   Inserted with ripple: {:?}\n", result1);

    // Insert with bypass_ripple
    println!("2. Insert with bypass_ripple:");
    let record2 = Record::new().field("name", "Product 2").field("price", 200);
    let insert_opts = InsertOptions::new().bypass_ripple(true);
    let result2 = client
        .insert(COLLECTION, record2, Some(insert_opts))
        .await?;
    println!("   Inserted with bypass_ripple: {:?}\n", result2);

    // Update with bypass_ripple
    println!("3. Update with bypass_ripple:");
    if let Some(id) = result1.get("id") {
        let user_id = match id {
            ekodb_client::FieldType::String(s) => s.clone(),
            _ => panic!("Expected string ID"),
        };

        let update_record = Record::new().field("price", 150);
        let update_opts = UpdateOptions::new().bypass_ripple(true);
        let result3 = client
            .update(COLLECTION, &user_id, update_record, Some(update_opts))
            .await?;
        println!("   Updated with bypass_ripple: {:?}\n", result3);

        // Delete with bypass_ripple
        println!("4. Delete with bypass_ripple:");
        client.delete(COLLECTION, &user_id, Some(true)).await?;
        println!("   Deleted with bypass_ripple\n");
    }

    // Batch operations with bypass_ripple
    println!("5. Batch insert with bypass_ripple:");
    let records = vec![
        Record::new().field("name", "Batch 1").field("price", 10),
        Record::new().field("name", "Batch 2").field("price", 20),
    ];
    let batch_results = client.batch_insert(COLLECTION, records, Some(true)).await?;
    println!(
        "   Batch inserted with bypass_ripple: {} records\n",
        batch_results.len()
    );

    // Upsert with bypass_ripple
    println!("6. Upsert with bypass_ripple:");
    let upsert_record = Record::new()
        .field("name", "Upsert Product")
        .field("price", 500);
    let upsert_opts = UpsertOptions::new().bypass_ripple(true);
    let upsert_result = client
        .upsert(COLLECTION, "custom-id", upsert_record, Some(upsert_opts))
        .await?;
    println!("   Upserted with bypass_ripple: {:?}\n", upsert_result);

    Ok(())
}
