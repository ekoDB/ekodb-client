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

    // Example 1: Create a collection (via insert)
    println!("\n=== Create Collection (via insert) ===");
    let insert_data = json!({
        "name": "Demo Collection",
        "description": "Created for testing",
        "active": true
    });

    let insert_response = client
        .post(&format!("{}/api/insert/demo_collection", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&insert_data)
        .send()
        .await?;

    let insert_result: Value = insert_response.json().await?;
    println!(
        "Collection created with first record: {}",
        insert_result["id"]
    );

    // Example 2: List all collections
    println!("\n=== List Collections ===");
    let collections_response = client
        .get(&format!("{}/api/collections", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let collections_data: Value = collections_response.json().await?;
    let collections = collections_data["collections"].as_array().unwrap();
    println!("Total collections: {}", collections.len());

    let collection_names: Vec<String> = collections
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();
    println!("Sample collections: {:?}", collection_names);

    // Example 3: Count documents in collection
    println!("\n=== Count Documents ===");
    let count_response = client
        .post(&format!("{}/api/find/demo_collection", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({"limit": 1000}))
        .send()
        .await?;

    let docs: Vec<Value> = count_response.json().await?;
    println!("Document count: {}", docs.len());

    // Example 4: Delete collection
    println!("\n=== Delete Collection ===");
    client
        .delete(&format!("{}/api/collections/demo_collection", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    println!("Collection deleted successfully");

    // Example 5: Verify deletion
    println!("\n=== Verify Deletion ===");
    let verify_response = client
        .get(&format!("{}/api/collections", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let updated_collections_data: Value = verify_response.json().await?;
    let updated_collections = updated_collections_data["collections"].as_array().unwrap();
    let updated_collection_names: Vec<String> = updated_collections
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();
    let still_exists = updated_collection_names.contains(&"demo_collection".to_string());
    println!("Collection still exists: {}", still_exists);

    println!("\n✓ All collection management operations completed successfully");

    Ok(())
}
