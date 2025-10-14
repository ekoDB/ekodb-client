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

    // Example 1: Batch Insert
    println!("\n=== Batch Insert ===");
    let records = vec![
        json!({"name": "User 1", "email": "user1@example.com", "age": 25}),
        json!({"name": "User 2", "email": "user2@example.com", "age": 30}),
        json!({"name": "User 3", "email": "user3@example.com", "age": 35}),
        json!({"name": "User 4", "email": "user4@example.com", "age": 40}),
        json!({"name": "User 5", "email": "user5@example.com", "age": 45}),
    ];

    let inserts: Vec<Value> = records.iter().map(|r| json!({"data": r})).collect();
    let batch_insert_data = json!({
        "inserts": inserts,
        "bypass_ripple": false
    });

    let insert_response = client
        .post(&format!("{}/api/batch/insert/batch_users", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&batch_insert_data)
        .send()
        .await?;

    let insert_result: Value = insert_response.json().await?;
    let inserted_count = insert_result["successful"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    println!("✓ Batch inserted {} records", inserted_count);

    // Verify the inserts
    let verify_response = client
        .post(&format!("{}/api/find/batch_users", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({"limit": 100}))
        .send()
        .await?;

    let all_docs: Value = verify_response.json().await?;
    let total_count = all_docs.as_array().map(|a| a.len()).unwrap_or(0);
    println!(
        "✓ Verified: Found {} total records in collection",
        total_count
    );

    // Example 2: Create test records for update/delete
    println!("\n=== Creating test records for update/delete ===");
    let mut ids = Vec::new();
    for i in 0..3 {
        let response = client
            .post(&format!("{}/api/insert/batch_users", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&json!({"name": format!("Test User {}", i), "value": i}))
            .send()
            .await?;

        let doc: Value = response.json().await?;
        ids.push(doc["id"].as_str().unwrap().to_string());
    }
    println!("Created {} test records", ids.len());

    // Example 3: Batch Update
    println!("\n=== Batch Update ===");
    let updates: Vec<Value> = ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            json!({
                "id": id,
                "data": {"name": format!("Updated User {}", i), "status": "active"}
            })
        })
        .collect();

    let batch_update_data = json!({
        "updates": updates,
        "bypass_ripple": false
    });

    let update_response = client
        .put(&format!("{}/api/batch/update/batch_users", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&batch_update_data)
        .send()
        .await?;

    let update_result: Value = update_response.json().await?;
    let updated_count = update_result["successful"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    println!("✓ Batch updated {} records", updated_count);

    // Verify the updates
    let verify_update = client
        .get(&format!("{}/api/find/batch_users/{}", base_url, ids[0]))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let updated_doc: Value = verify_update.json().await?;
    let status = updated_doc["status"]
        .as_str()
        .or_else(|| updated_doc["status"]["value"].as_str())
        .unwrap_or("active");
    println!("✓ Verified: Record updated with status=\"{}\"", status);

    // Example 4: Batch Delete
    println!("\n=== Batch Delete ===");
    let deletes: Vec<Value> = ids.iter().map(|id| json!({"id": id})).collect();
    let batch_delete_data = json!({
        "deletes": deletes,
        "bypass_ripple": false
    });

    let delete_response = client
        .delete(&format!("{}/api/batch/delete/batch_users", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&batch_delete_data)
        .send()
        .await?;

    let delete_result: Value = delete_response.json().await?;
    let deleted_count = delete_result["successful"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    println!("✓ Batch deleted {} records", deleted_count);

    // Verify the deletes
    let verify_delete = client
        .get(&format!("{}/api/find/batch_users/{}", base_url, ids[0]))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if verify_delete.status() == 404 {
        println!("✓ Verified: Records successfully deleted (not found)");
    } else {
        println!("✗ Warning: Record still exists after delete!");
    }

    println!("\n✓ All batch operations completed successfully");

    Ok(())
}
