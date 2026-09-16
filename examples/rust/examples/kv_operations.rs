use serde_json::{json, Value};
use std::env;

const SESSION_KEY: &str = "kv_operations:direct:rs:session:user123";
const PRODUCT_KEYS: [&str; 3] = [
    "kv_operations:direct:rs:cache:product:1",
    "kv_operations:direct:rs:cache:product:2",
    "kv_operations:direct:rs:cache:product:3",
];

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
        .await?
        .error_for_status()?;

    let token_data: Value = token_response.json().await?;
    let token = token_data["token"].as_str().unwrap();
    println!("✓ Authentication successful");

    cleanup_owned_keys(&client, &base_url, token).await?;

    let operation_result = run_examples(&client, &base_url, token).await;
    let cleanup_result = cleanup_owned_keys(&client, &base_url, token).await;

    match (operation_result, cleanup_result) {
        (Ok(()), Ok(())) => {
            println!("\n✓ All KV operations completed successfully");
            Ok(())
        }
        (Err(operation_error), Ok(())) => Err(operation_error),
        (Ok(()), Err(cleanup_error)) => Err(cleanup_error),
        (Err(operation_error), Err(cleanup_error)) => Err(std::io::Error::other(format!(
            "{operation_error}; cleanup also failed: {cleanup_error}"
        ))
        .into()),
    }
}

async fn run_examples(
    client: &reqwest::Client,
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Set a key-value pair
    println!("\n=== KV Set ===");
    let kv_data = json!({
        "value": {
            "userId": 123,
            "username": "john_doe"
        }
    });

    client
        .post(&format!("{}/api/kv/set/{}", base_url, SESSION_KEY))
        .header("Authorization", format!("Bearer {}", token))
        .json(&kv_data)
        .send()
        .await?
        .error_for_status()?;

    println!("✓ Set key: session:user123");

    // Example 2: Get a key-value pair
    println!("\n=== KV Get ===");
    let get_response = client
        .get(&format!("{}/api/kv/get/{}", base_url, SESSION_KEY))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?;

    let get_value: Value = get_response.json().await?;
    println!("Retrieved value: {:?}", get_value["value"]);

    // Example 3: Set multiple keys
    println!("\n=== Set Multiple Keys ===");
    let keys = PRODUCT_KEYS;

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
            .await?
            .error_for_status()?;
    }
    println!("✓ Set {} keys", keys.len());

    // Example 4: Get multiple keys
    println!("\n=== Get Multiple Keys ===");
    for key in &keys {
        let response = client
            .get(&format!("{}/api/kv/get/{}", base_url, key))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?
            .error_for_status()?;

        let result: Value = response.json().await?;
        println!("{}: {:?}", key, result["value"]);
    }

    // Example 5: Delete a key
    println!("\n=== KV Delete ===");
    client
        .delete(&format!("{}/api/kv/delete/{}", base_url, SESSION_KEY))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?;

    println!("✓ Deleted key: session:user123");

    // Verify deletion
    let verify_response = client
        .get(&format!("{}/api/kv/get/{}", base_url, SESSION_KEY))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if verify_response.status() == 404 {
        println!("✓ Verified: Key successfully deleted (not found)");
    } else {
        println!("✗ Warning: Key still exists after delete!");
        return Err(std::io::Error::other("session key still exists after deletion").into());
    }

    // Example 6: Delete multiple keys
    println!("\n=== Delete Multiple Keys ===");
    for key in &keys {
        client
            .delete(&format!("{}/api/kv/delete/{}", base_url, key))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?
            .error_for_status()?;
    }
    println!("✓ Deleted {} keys", keys.len());

    Ok(())
}

async fn cleanup_owned_keys(
    client: &reqwest::Client,
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut errors = Vec::new();
    for key in std::iter::once(SESSION_KEY).chain(PRODUCT_KEYS) {
        match client
            .delete(format!("{base_url}/api/kv/delete/{key}"))
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
        {
            Ok(response)
                if response.status().is_success()
                    || response.status() == reqwest::StatusCode::NOT_FOUND => {}
            Ok(response) => errors.push(format!("{key}: HTTP {}", response.status())),
            Err(error) => errors.push(format!("{key}: {error}")),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "failed to clean owned KV keys: {}",
            errors.join("; ")
        ))
        .into())
    }
}
