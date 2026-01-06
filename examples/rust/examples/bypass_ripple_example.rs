//! Example demonstrating bypass_ripple usage
//!
//! The bypass_ripple parameter allows you to skip ripple propagation for specific operations,
//! which is useful in multi-node deployments when you want to prevent data from being
//! automatically replicated to other nodes.

use ekodb_client::{Client, Record, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    let collection = "products";

    println!("=== Bypass Ripple Example ===\n");

    // Basic insert (no bypass)
    println!("1. Basic insert (ripple enabled):");
    let record1 = Record::new().field("name", "Product 1").field("price", 100);
    let result1 = client.insert(collection, record1, None).await?;
    println!("   Inserted with ripple: {:?}\n", result1);

    // Insert with bypass_ripple
    println!("2. Insert with bypass_ripple:");
    let record2 = Record::new().field("name", "Product 2").field("price", 200);
    let result2 = client.insert(collection, record2, Some(true)).await?;
    println!("   Inserted with bypass_ripple: {:?}\n", result2);

    // Update with bypass_ripple
    println!("3. Update with bypass_ripple:");
    if let Some(id) = result1.get("id") {
        let user_id = match id {
            ekodb_client::FieldType::String(s) => s.clone(),
            _ => panic!("Expected string ID"),
        };

        let update_record = Record::new().field("price", 150);
        let result3 = client
            .update(collection, &user_id, update_record, Some(true))
            .await?;
        println!("   Updated with bypass_ripple: {:?}\n", result3);

        // Delete with bypass_ripple
        println!("4. Delete with bypass_ripple:");
        client.delete(collection, &user_id, Some(true)).await?;
        println!("   Deleted with bypass_ripple\n");
    }

    // Batch operations with bypass_ripple
    println!("5. Batch insert with bypass_ripple:");
    let records = vec![
        Record::new().field("name", "Batch 1").field("price", 10),
        Record::new().field("name", "Batch 2").field("price", 20),
    ];
    let batch_results = client.batch_insert(collection, records, Some(true)).await?;
    println!(
        "   Batch inserted with bypass_ripple: {} records\n",
        batch_results.len()
    );

    // Upsert with bypass_ripple
    println!("6. Upsert with bypass_ripple:");
    let upsert_record = Record::new()
        .field("name", "Upsert Product")
        .field("price", 500);
    let upsert_result = client
        .upsert(collection, "custom-id", upsert_record, Some(true))
        .await?;
    println!("   Upserted with bypass_ripple: {:?}\n", upsert_result);

    println!("✅ All bypass_ripple operations completed successfully!");

    Ok(())
}
