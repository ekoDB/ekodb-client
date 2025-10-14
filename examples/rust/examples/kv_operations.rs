use serde_json::{json, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Step 1: Get authentication token
    let client = reqwest::Client::new();
    let token_response = client
        .post(&format!("{}/api/auth/token", base_url))
        .json(&json!({ "api_key": api_key }))
        .send()
        .await?;

    let token_data: Value = token_response.json().await?;
    let token = token_data["token"].as_str().unwrap();
    println!("✓ Authentication successful");

    // Example 1: Set a key-value pair
    println!("\n=== KV Set ===");
    let kv_data = json!({
        "value": {
            "userId": 123,
            "username": "john_doe"
        }
    });

    client
        .post(&format!("{}/api/kv/set/session:user123", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&kv_data)
        .send()
        .await?;

    println!("✓ Set key: session:user123");

    // Example 2: Get a key-value pair
    println!("\n=== KV Get ===");
    let get_response = client
        .get(&format!("{}/api/kv/get/session:user123", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let get_value: Value = get_response.json().await?;
    println!("Retrieved value: {:?}", get_value["value"]);

    // Example 3: Set multiple keys
    println!("\n=== Set Multiple Keys ===");
    let keys = vec!["cache:product:1", "cache:product:2", "cache:product:3"];

    for (i, key) in keys.iter().enumerate() {
        let data = json!({
            "value": {
                "name": format!("Product {}", i + 1),
                "price": 29.99 + (i as f64 * 10.0)
            }
        });

        client
            .post(&format!("{}/api/kv/set/{}", base_url, key))
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await?;
    }
    println!("✓ Set {} keys", keys.len());

    // Example 4: Get multiple keys
    println!("\n=== Get Multiple Keys ===");
    for key in &keys {
        let response = client
            .get(&format!("{}/api/kv/get/{}", base_url, key))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        let result: Value = response.json().await?;
        println!("{}: {:?}", key, result["value"]);
    }

    // Example 5: Delete a key
    println!("\n=== KV Delete ===");
    client
        .delete(&format!("{}/api/kv/delete/session:user123", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    println!("✓ Deleted key: session:user123");

    // Verify deletion
    let verify_response = client
        .get(&format!("{}/api/kv/get/session:user123", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if verify_response.status() == 404 {
        println!("✓ Verified: Key successfully deleted (not found)");
    } else {
        println!("✗ Warning: Key still exists after delete!");
    }

    // Example 6: Delete multiple keys
    println!("\n=== Delete Multiple Keys ===");
    for key in &keys {
        client
            .delete(&format!("{}/api/kv/delete/{}", base_url, key))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
    }
    println!("✓ Deleted {} keys", keys.len());

    println!("\n✓ All KV operations completed successfully");

    Ok(())
}
