use reqwest::Client;
use serde_json::{json, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Get authentication token
    let client = Client::new();
    let auth_response: Value = client
        .post(&format!("{}/api/auth/token", base_url))
        .json(&json!({"api_key": api_key}))
        .send()
        .await?
        .json()
        .await?;

    let token = auth_response["token"].as_str().unwrap();
    println!("✓ Authentication successful");

    // Example 1: Insert document with TTL
    println!("\n=== Insert Document with TTL (1 hour) ===");
    let doc1: Value = client
        .post(&format!("{}/api/insert/ttl_cache", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "key": "session_token",
            "value": "abc123",
            "ttl": 3600
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("✓ Inserted document: {}", doc1["id"]);

    // Example 2: Insert with shorter TTL (integer seconds)
    println!("\n=== Insert Document with TTL (5 minutes - integer) ===");
    let doc2: Value = client
        .post(&format!("{}/api/insert/ttl_cache", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "key": "temp_data",
            "value": "short-lived data",
            "ttl": 300
        }))
        .send()
        .await?
        .json()
        .await?;
    println!(
        "✓ Inserted document: {}",
        doc2.get("id").unwrap_or(&Value::Null)
    );

    // Example 3: Insert with duration string format
    println!("\n=== Insert Document with TTL (30 minutes - duration string) ===");
    let doc3: Value = client
        .post(&format!("{}/api/insert/ttl_cache", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "key": "duration_test",
            "value": "testing duration strings",
            "ttl": "30m"
        }))
        .send()
        .await?
        .json()
        .await?;
    println!(
        "✓ Inserted document with duration string TTL: {}",
        doc3.get("id").unwrap_or(&Value::Null)
    );

    // Example 3: Query documents
    println!("\n=== Query Documents ===");
    let docs: Vec<Value> = client
        .post(&format!("{}/api/find/ttl_cache", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({"limit": 10}))
        .send()
        .await?
        .json()
        .await?;
    println!("✓ Found {} documents with TTL", docs.len());

    // Example 4: Update document
    println!("\n=== Update Document ===");
    client
        .put(&format!("{}/api/update/ttl_cache/{}", base_url, doc1["id"]))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({"value": "updated_value"}))
        .send()
        .await?;
    println!("✓ Updated document");

    // Example 6: Delete document
    println!("\n=== Delete Document ===");
    client
        .delete(&format!(
            "{}/api/delete/ttl_cache/{}",
            base_url,
            doc1.get("id").unwrap_or(&Value::Null)
        ))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    println!("✓ Deleted document");

    println!("\n✓ All document TTL operations completed successfully");

    Ok(())
}
