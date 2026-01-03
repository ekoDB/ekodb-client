//! Key-Value operations using ekodb_client library
//!
//! This example demonstrates KV store operations using the ekodb_client library.
//!
//! Compare with kv_operations.rs to see the difference!

use ekodb_client::Client;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Example 1: Set a key-value pair
    println!("\n=== KV Set ===");
    client
        .kv_set(
            "session:user123",
            json!({
                "userId": 123,
                "username": "john_doe"
            }),
        )
        .await?;
    println!("✓ Set key: session:user123");

    // Example 2: Get a key-value pair
    println!("\n=== KV Get ===");
    if let Some(value) = client.kv_get("session:user123").await? {
        println!("Retrieved value: {:?}", value);
    }

    // Example 3: Set multiple keys
    println!("\n=== Set Multiple Keys ===");
    let keys = vec!["cache:product:1", "cache:product:2", "cache:product:3"];

    for (i, key) in keys.iter().enumerate() {
        client
            .kv_set(
                key,
                json!({
                    "name": format!("Product {}", i + 1),
                    "price": 29.99 + (i as f64 * 10.0)
                }),
            )
            .await?;
    }
    println!("✓ Set {} keys", keys.len());

    // Example 4: Get multiple keys
    println!("\n=== Get Multiple Keys ===");
    for key in &keys {
        if let Some(value) = client.kv_get(key).await? {
            println!("{}: {:?}", key, value);
        }
    }

    // Example 5: Check if key exists
    println!("\n=== KV Exists ===");
    let exists = client.kv_exists("session:user123").await?;
    println!("Key exists: {}", exists);

    // Example 6: Find keys with pattern
    println!("\n=== KV Find (Pattern Query) ===");
    let cache_results = client.kv_find(Some("cache:product:.*"), false).await?;
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
    client.kv_delete("session:user123").await?;
    println!("✓ Deleted key: session:user123");

    // Verify deletion with kv_exists
    let exists_after = client.kv_exists("session:user123").await?;
    println!("✓ Verified: Key exists after delete: {}", exists_after);

    // Example 9: Delete multiple keys
    println!("\n=== Delete Multiple Keys ===");
    for key in &keys {
        client.kv_delete(key).await?;
    }
    println!("✓ Deleted {} keys", keys.len());

    println!("\n✓ All KV operations completed successfully");

    Ok(())
}
