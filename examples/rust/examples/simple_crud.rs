use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Get authentication token
    let client = reqwest::Client::new();
    let token_response = client
        .post(&format!("{}/api/auth/token", base_url))
        .json(&json!({ "api_key": api_key }))
        .send()
        .await?;

    let token_data: Value = token_response.json().await?;
    let token = token_data["token"].as_str().unwrap();
    println!("✓ Authentication successful");

    // Create HTTP client with default headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Example 1: Insert a document
    println!("\n=== Insert Document ===");
    let doc: Value = client
        .post(&format!("{}/api/insert/test_collection", base_url))
        .json(&json!({
            "name": "Test Record",
            "value": 42,
            "active": true
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("Inserted: {:?}", doc);
    let doc_id = doc["id"].as_str().unwrap();

    // Example 2: Find by ID
    println!("\n=== Find by ID ===");
    let found_doc: Value = client
        .get(&format!("{}/api/find/test_collection/{}", base_url, doc_id))
        .send()
        .await?
        .json()
        .await?;
    println!("Found: {:?}", found_doc);

    // Example 3: Find with query
    println!("\n=== Find with Query ===");
    let docs: Value = client
        .post(&format!("{}/api/find/test_collection", base_url))
        .json(&json!({
            "filter": {
                "type": "Condition",
                "content": {
                    "field": "active",
                    "operator": "Eq",
                    "value": true
                }
            },
            "limit": 10
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("Found documents: {:?}", docs);

    // Example 4: Update document
    println!("\n=== Update Document ===");
    let updated: Value = client
        .put(&format!(
            "{}/api/update/test_collection/{}",
            base_url, doc_id
        ))
        .json(&json!({
            "name": "Updated Record",
            "value": 100
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("Updated: {:?}", updated);

    // Example 5: Delete document
    println!("\n=== Delete Document ===");
    client
        .delete(&format!(
            "{}/api/delete/test_collection/{}",
            base_url, doc_id
        ))
        .send()
        .await?;
    println!("Deleted document");

    println!("\n✓ All CRUD operations completed successfully");

    Ok(())
}
