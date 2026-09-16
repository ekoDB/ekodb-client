use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::env;

const COLLECTION: &str = "simple_crud_example_rs";

async fn delete_collection(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .delete(format!("{base_url}/api/collections/{COLLECTION}"))
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::NOT_FOUND {
        response.error_for_status()?;
    }
    Ok(())
}

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

    delete_collection(&client, &base_url).await?;
    let operation_result = run_examples(&client, &base_url).await;
    let cleanup_result = delete_collection(&client, &base_url).await;
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), Ok(())) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup),
        (Ok(()), Ok(())) => {}
    }

    println!("\n✓ All CRUD operations completed successfully");
    Ok(())
}

async fn run_examples(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Insert a document
    println!("\n=== Insert Document ===");
    let doc: Value = client
        .post(format!("{base_url}/api/insert/{COLLECTION}"))
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
        .get(format!("{base_url}/api/find/{COLLECTION}/{doc_id}"))
        .send()
        .await?
        .json()
        .await?;
    println!("Found: {:?}", found_doc);

    // Example 3: Find with query
    println!("\n=== Find with Query ===");
    let docs: Value = client
        .post(format!("{base_url}/api/find/{COLLECTION}"))
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
        .put(format!("{base_url}/api/update/{COLLECTION}/{doc_id}"))
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
        .delete(format!("{base_url}/api/delete/{COLLECTION}/{doc_id}"))
        .send()
        .await?;
    println!("Deleted document");

    Ok(())
}
