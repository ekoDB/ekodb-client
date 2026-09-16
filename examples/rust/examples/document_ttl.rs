use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::time::Duration;

const COLLECTION: &str = "ttl_cache_example_rs";

async fn delete_collection(
    client: &Client,
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .delete(format!("{base_url}/api/collections/{COLLECTION}"))
        .bearer_auth(token)
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::NOT_FOUND {
        response.error_for_status()?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ekoDB Document TTL Example (Rust) ===");
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Get authentication token
    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;
    let auth_response: Value = client
        .post(&format!("{}/api/auth/token", base_url))
        .json(&json!({"api_key": api_key}))
        .send()
        .await?
        .json()
        .await?;

    let token = auth_response["token"].as_str().unwrap();
    println!("✓ Authentication successful");

    delete_collection(&client, &base_url, token).await?;
    let operation_result = run_operations(&client, &base_url, token).await;
    let cleanup_result = delete_collection(&client, &base_url, token).await;
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), Ok(())) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup),
        (Ok(()), Ok(())) => {}
    }

    println!("\n✓ All document TTL operations completed successfully");
    Ok(())
}

async fn run_operations(
    client: &Client,
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Insert document with TTL
    println!("\n=== Insert Document with TTL (1 hour) ===");
    let doc1: Value = client
        .post(format!("{base_url}/api/insert/{COLLECTION}"))
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
    let doc1_id = doc1["id"].as_str().ok_or("insert response omitted id")?;
    println!("✓ Inserted document: {doc1_id}");

    // Example 2: Insert with shorter TTL (integer seconds)
    println!("\n=== Insert Document with TTL (5 minutes - integer) ===");
    let doc2: Value = client
        .post(format!("{base_url}/api/insert/{COLLECTION}"))
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
    let doc2_id = doc2["id"].as_str().ok_or("insert response omitted id")?;
    println!("✓ Inserted document: {doc2_id}");

    // Example 3: Insert with duration string format
    println!("\n=== Insert Document with TTL (30 minutes - duration string) ===");
    let doc3: Value = client
        .post(format!("{base_url}/api/insert/{COLLECTION}"))
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
    let doc3_id = doc3["id"].as_str().ok_or("insert response omitted id")?;
    println!("✓ Inserted document with duration string TTL: {doc3_id}");

    // Example 3: Query documents
    println!("\n=== Query Documents ===");
    let docs: Vec<Value> = client
        .post(format!("{base_url}/api/find/{COLLECTION}"))
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
        .put(format!("{base_url}/api/update/{COLLECTION}/{doc1_id}"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({"value": "updated_value"}))
        .send()
        .await?
        .error_for_status()?;
    println!("✓ Updated document");

    // Example 6: Delete document
    println!("\n=== Delete Document ===");
    client
        .delete(format!("{base_url}/api/delete/{COLLECTION}/{doc1_id}"))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .error_for_status()?;
    println!("✓ Deleted document");

    // doc2_id and doc3_id are validated above; the owned collection cleanup
    // removes them together even if a preceding operation fails.
    let _ = (doc2_id, doc3_id);
    Ok(())
}
