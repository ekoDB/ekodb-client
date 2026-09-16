//! Key-Value operations using ekodb_client library
//!
//! This example demonstrates KV store operations using the ekodb_client library.
//!
//! Compare with kv_operations.rs to see the difference!

use ekodb_client::Client;
use serde_json::json;
use std::env;

const SESSION_KEY: &str = "kv_operations:rs:session:user123";
const PRODUCT_KEYS: [&str; 3] = [
    "kv_operations:rs:cache:product:1",
    "kv_operations:rs:cache:product:2",
    "kv_operations:rs:cache:product:3",
];

async fn run_examples(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("✓ Client created");

    // Example 1: Set a key-value pair
    println!("\n=== KV Set ===");
    client
        .kv_set(
            SESSION_KEY,
            json!({
                "userId": 123,
                "username": "john_doe"
            }),
            None,
        )
        .await?;
    println!("✓ Set key: session:user123");

    // Example 2: Get a key-value pair
    println!("\n=== KV Get ===");
    if let Some(value) = client.kv_get(SESSION_KEY).await? {
        println!("Retrieved value: {:?}", value);
    }

    // Example 3: Batch set multiple keys
    println!("\n=== KV Batch Set ===");
    use ekodb_client::Record;
    let keys: Vec<String> = PRODUCT_KEYS.iter().map(|key| (*key).to_string()).collect();
    let mut values = Vec::new();
    for i in 0..3 {
        let mut record = Record::new();
        record.insert("name".to_string(), json!(format!("Product {}", i + 1)));
        record.insert("price".to_string(), json!(29.99 + (i as f64 * 10.0)));
        values.push(record);
    }

    let set_results = client.kv_batch_set(keys.clone(), values, None).await?;
    println!("✓ Batch set {} keys", set_results.len());
    for (key, was_set) in &set_results {
        println!("  {}: {}", key, if *was_set { "success" } else { "failed" });
    }

    // Example 4: Batch get multiple keys
    println!("\n=== KV Batch Get ===");
    let batch_values = client.kv_batch_get(keys.clone()).await?;
    println!("✓ Batch retrieved {} values", batch_values.len());
    for (i, value) in batch_values.iter().enumerate() {
        println!("  {}: {:?}", keys[i], value);
    }

    // Example 5: Check if key exists
    println!("\n=== KV Exists ===");
    let exists = client.kv_exists(SESSION_KEY).await?;
    println!("Key exists: {}", exists);

    // Example 6: Find keys with pattern
    println!("\n=== KV Find (Pattern Query) ===");
    let cache_results = client
        .kv_find(Some("kv_operations:rs:cache:product:.*"), false)
        .await?;
    println!(
        "Found {} keys matching 'cache:product:.*'",
        cache_results.len()
    );

    // Example 7: Query all keys
    println!("\n=== KV Query (Alias for Find) ===");
    let all_results = client.kv_query(None, false).await?;
    println!("Total keys in store: {}", all_results.len());

    // Example 8: Delete a key
    println!("\n=== KV Delete ===");
    client.kv_delete(SESSION_KEY).await?;
    println!("✓ Deleted key: session:user123");

    // Verify deletion with kv_exists
    let exists_after = client.kv_exists(SESSION_KEY).await?;
    println!("✓ Verified: Key exists after delete: {}", exists_after);

    // Example 9: Batch delete multiple keys
    println!("\n=== KV Batch Delete ===");
    let delete_results = client.kv_batch_delete(keys.clone()).await?;
    println!("✓ Batch deleted {} keys", delete_results.len());
    for (key, was_deleted) in &delete_results {
        println!(
            "  {}: {}",
            key,
            if *was_deleted { "deleted" } else { "not found" }
        );
    }

    Ok(())
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
    let keys: Vec<&str> = std::iter::once(SESSION_KEY).chain(PRODUCT_KEYS).collect();
    for key in &keys {
        if client.kv_exists(key).await? {
            client.kv_delete(key).await?;
        }
    }

    let operation_result = run_examples(&client).await;
    let mut cleanup_errors = Vec::new();
    for key in keys {
        match client.kv_exists(key).await {
            Ok(true) => {
                if let Err(error) = client.kv_delete(key).await {
                    cleanup_errors.push(format!("KV key {key}: {error}"));
                }
            }
            Ok(false) => {}
            Err(error) => cleanup_errors.push(format!("KV key {key}: {error}")),
        }
    }
    match (operation_result, cleanup_errors.is_empty()) {
        (Err(primary), false) => {
            return Err(format!(
                "{primary}; cleanup also failed: {}",
                cleanup_errors.join("; ")
            )
            .into())
        }
        (Err(primary), true) => return Err(primary),
        (Ok(()), false) => {
            return Err(format!("cleanup failed: {}", cleanup_errors.join("; ")).into())
        }
        (Ok(()), true) => {}
    }
    println!("\n✓ All KV operations completed successfully");
    Ok(())
}
