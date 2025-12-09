use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

async fn get_auth_token(
    client: &Client,
    base_url: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .post(format!("{}/api/auth/token", base_url))
        .json(&json!({"api_key": api_key}))
        .send()
        .await?;

    let data: Value = response.json().await?;
    Ok(data["token"].as_str().unwrap().to_string())
}

async fn make_request(
    client: &Client,
    method: &str,
    path: &str,
    base_url: &str,
    token: &str,
    data: Option<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("{}{}", base_url, path);
    let mut request = match method {
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        _ => client.get(&url),
    };

    request = request
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json");

    if let Some(body) = data {
        request = request.json(&body);
    }

    let response = request.send().await?;
    let result: Value = response.json().await?;
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::new();
    let token = get_auth_token(&client, &base_url, &api_key).await?;
    println!("✓ Authentication successful\n");

    // Setup test data
    println!("=== Setup: Creating Test Accounts ===");
    let alice = make_request(
        &client,
        "POST",
        "/api/insert/test_accounts",
        &base_url,
        &token,
        Some(json!({
            "account_id": "ACC001",
            "name": "Alice",
            "balance": 1000
        })),
    )
    .await?;
    let alice_id = alice["id"].as_str().unwrap();
    println!("Created Alice: $1000 - ID: {}", alice_id);

    let bob = make_request(
        &client,
        "POST",
        "/api/insert/test_accounts",
        &base_url,
        &token,
        Some(json!({
            "account_id": "ACC002",
            "name": "Bob",
            "balance": 500
        })),
    )
    .await?;
    let bob_id = bob["id"].as_str().unwrap();
    println!("Created Bob: $500 - ID: {}\n", bob_id);

    // Example 1: Begin Transaction
    println!("=== Example 1: Begin Transaction ===");
    let tx_data = make_request(
        &client,
        "POST",
        "/api/transactions",
        &base_url,
        &token,
        Some(json!({
            "isolation_level": "ReadCommitted"
        })),
    )
    .await?;
    let tx_id = tx_data["transaction_id"].as_str().unwrap();
    println!("Transaction ID: {}\n", tx_id);

    // Example 2: Operations with transaction_id
    println!("=== Example 2: Operations with transaction_id ===");
    make_request(
        &client,
        "PUT",
        &format!("/api/batch/update/test_accounts?transaction_id={}", tx_id),
        &base_url,
        &token,
        Some(json!({
            "updates": [
                {"id": alice_id, "data": {"balance": 800}},
                {"id": bob_id, "data": {"balance": 700}}
            ]
        })),
    )
    .await?;
    println!("Updated Alice: $1000 → $800");
    println!("Updated Bob: $500 → $700\n");

    // Example 3: Transaction Status
    println!("=== Example 3: Transaction Status ===");
    let status = make_request(
        &client,
        "GET",
        &format!("/api/transactions/{}", tx_id),
        &base_url,
        &token,
        None,
    )
    .await?;
    println!("Status: {}", status["state"]);
    println!("Operations: {}\n", status["operations_count"]);

    // Example 4: Commit Transaction
    println!("=== Example 4: Commit Transaction ===");
    make_request(
        &client,
        "POST",
        &format!("/api/transactions/{}/commit", tx_id),
        &base_url,
        &token,
        None,
    )
    .await?;
    println!("✓ Transaction committed\n");

    // Verification
    println!("=== Verification ===");
    let alice_final = make_request(
        &client,
        "GET",
        &format!("/api/find/test_accounts/{}", alice_id),
        &base_url,
        &token,
        None,
    )
    .await?;
    println!("Alice: {}", alice_final["balance"]);

    let bob_final = make_request(
        &client,
        "GET",
        &format!("/api/find/test_accounts/{}", bob_id),
        &base_url,
        &token,
        None,
    )
    .await?;
    println!("Bob: {}\n", bob_final["balance"]);

    // Example 5: Rollback
    println!("=== Example 5: Rollback ===");
    let tx_data2 = make_request(
        &client,
        "POST",
        "/api/transactions",
        &base_url,
        &token,
        Some(json!({
            "isolation_level": "ReadCommitted"
        })),
    )
    .await?;
    let tx_id2 = tx_data2["transaction_id"].as_str().unwrap();
    println!("New transaction: {}", tx_id2);

    make_request(
        &client,
        "PUT",
        &format!("/api/batch/update/test_accounts?transaction_id={}", tx_id2),
        &base_url,
        &token,
        Some(json!({
            "updates": [
                {"id": bob_id, "data": {"balance": 600}}
            ]
        })),
    )
    .await?;
    println!("Updated Bob: $700 → $600 (in transaction)");

    make_request(
        &client,
        "POST",
        &format!("/api/transactions/{}/rollback", tx_id2),
        &base_url,
        &token,
        None,
    )
    .await?;
    println!("✓ Transaction rolled back");

    let bob_after = make_request(
        &client,
        "GET",
        &format!("/api/find/test_accounts/{}", bob_id),
        &base_url,
        &token,
        None,
    )
    .await?;
    println!("Bob after rollback: {}\n", bob_after["balance"]);

    // Cleanup
    println!("=== Cleanup ===");
    make_request(
        &client,
        "DELETE",
        &format!("/api/delete/test_accounts/{}", alice_id),
        &base_url,
        &token,
        None,
    )
    .await?;
    make_request(
        &client,
        "DELETE",
        &format!("/api/delete/test_accounts/{}", bob_id),
        &base_url,
        &token,
        None,
    )
    .await?;
    println!("✓ Deleted test accounts\n");

    println!("✓ All transaction examples completed");

    Ok(())
}
