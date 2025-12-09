/*
Scripts Example using Direct HTTP Requests

Demonstrates using scripts with raw HTTP/reqwest API
No client library required
*/

use serde_json::{json, Value};
use std::env;
use std::sync::OnceLock;

static AUTH_TOKEN: OnceLock<String> = OnceLock::new();

fn get_base_url() -> String {
    env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
}

fn get_api_key() -> String {
    env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string())
}

async fn get_auth_token() -> Result<String, Box<dyn std::error::Error>> {
    if let Some(token) = AUTH_TOKEN.get() {
        return Ok(token.clone());
    }

    let base_url = get_base_url();
    let api_key = get_api_key();

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/auth/token", base_url))
        .json(&json!({"api_key": api_key}))
        .send()
        .await?;

    let data: Value = response.json().await?;
    let token = data["token"].as_str().unwrap().to_string();

    // Store token for reuse (ignore if already set by another thread)
    let _ = AUTH_TOKEN.set(token.clone());

    Ok(token)
}

async fn request(
    method: &str,
    path: &str,
    body: Option<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let base_url = get_base_url();
    let token = get_auth_token().await?;

    let client = reqwest::Client::new();
    let url = format!("{}{}", base_url, path);

    let mut req = match method {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        _ => return Err("Invalid HTTP method".into()),
    };

    req = req.header("Authorization", format!("Bearer {}", token));

    if let Some(body) = body {
        req = req.json(&body);
    }

    let response = req.send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("HTTP {}: {}", status, text).into());
    }

    let data: Value = response.json().await?;
    Ok(data)
}

async fn setup_test_data() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Setting up test data...");

    for i in 1..=10 {
        let record = json!({
            "name": format!("User {}", i),
            "age": 20 + i,
            "status": if i % 2 == 0 { "active" } else { "inactive" },
            "score": i * 10,
        });

        request("POST", "/api/insert/users", Some(record)).await?;
    }

    println!("✅ Test data ready\n");
    Ok(())
}

async fn simple_query_function() -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 1: Simple Query Function with Filter\n");

    let function1 = json!({
        "label": "get_active_users",
        "name": "Get Active Users",
        "description": "Query users with active status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "status",
                        "operator": "Eq",
                        "value": "active",
                    },
                },
            }
        ],
        "tags": ["users", "query"],
    });

    // Save script
    let save_result = request("POST", "/api/functions", Some(function1)).await?;
    let id = save_result["id"].as_str().unwrap();
    println!("✅ Script saved: {}", id);

    // Call script (can use label)
    let call_result = request("POST", "/api/functions/get_active_users", Some(json!({}))).await?;
    let records = call_result["records"].as_array().unwrap();
    println!("📊 Found {} active users\n", records.len());

    Ok(id.to_string())
}

async fn complex_filter_function() -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 2: Complex Filter with Multiple Conditions\n");

    let function2 = json!({
        "label": "get_high_scoring_active_users",
        "name": "Get High Scoring Active Users",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Logical",
                    "content": {
                        "operator": "And",
                        "expressions": [
                            {
                                "type": "Condition",
                                "content": {
                                    "field": "status",
                                    "operator": "Eq",
                                    "value": "active",
                                },
                            },
                            {
                                "type": "Condition",
                                "content": {
                                    "field": "score",
                                    "operator": "Gt",
                                    "value": 50,
                                },
                            },
                        ],
                    },
                },
                "sort": [{"field": "score", "ascending": false}],
                "limit": 10,
            }
        ],
        "tags": ["users", "filter"],
    });

    let save_result = request("POST", "/api/functions", Some(function2)).await?;
    let id = save_result["id"].as_str().unwrap();
    println!("✅ Script saved: {}", id);

    // Call the function
    let call_result = request(
        "POST",
        "/api/functions/get_high_scoring_active_users",
        Some(json!({})),
    )
    .await?;
    let records = call_result["records"].as_array().unwrap();
    println!(
        "📊 Found {} users (status=active, score>50, sorted by score)\n",
        records.len()
    );

    Ok(())
}

async fn aggregation_function() -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)\n");

    let function3 = json!({
        "label": "user_stats",
        "name": "User Statistics by Status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "age",
                        "operator": "Gt",
                        "value": 20,
                    },
                },
            },
            {
                "type": "Group",
                "by_fields": ["status"],
                "functions": [
                    {"output_field": "count", "operation": "Count"},
                    {
                        "output_field": "avg_score",
                        "operation": "Average",
                        "input_field": "score",
                    },
                    {
                        "output_field": "max_score",
                        "operation": "Max",
                        "input_field": "score",
                    },
                ],
            },
        ],
        "tags": ["analytics", "pipeline"],
    });

    let save_result = request("POST", "/api/functions", Some(function3)).await?;
    let id = save_result["id"].as_str().unwrap();
    println!("✅ Script saved: {}", id);

    let call_result = request("POST", "/api/functions/user_stats", Some(json!({}))).await?;
    let records = call_result["records"].as_array().unwrap();
    println!(
        "📊 Pipeline Results: Filtered (age>20) → Grouped by status → {} groups",
        records.len()
    );
    for record in records {
        println!("   {}", serde_json::to_string(record)?);
    }
    println!();

    Ok(id.to_string())
}

async fn function_management(
    get_active_users_id: &str,
    user_stats_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 4: Function Management\n");

    // List all scripts
    let scripts = request("GET", "/api/functions", None).await?;
    let scripts_array = scripts.as_array().unwrap();
    println!("📋 Total scripts: {}", scripts_array.len());

    // Get specific script (requires encrypted ID)
    let script = request(
        "GET",
        &format!("/api/functions/{}", get_active_users_id),
        None,
    )
    .await?;
    let name = script["name"].as_str().unwrap();
    println!("🔍 Retrieved script: {}", name);

    // Update script (requires encrypted ID)
    let updated = json!({
        "label": "get_active_users",
        "name": "Get Active Users (Updated)",
        "description": "Updated description",
        "version": "1.1",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": "users"}
        ],
        "tags": ["users"],
    });
    request(
        "PUT",
        &format!("/api/functions/{}", get_active_users_id),
        Some(updated),
    )
    .await?;
    println!("✏️  Script updated");

    // Delete script (requires encrypted ID)
    request("DELETE", &format!("/api/functions/{}", user_stats_id), None).await?;
    println!("🗑️  Script deleted\n");

    println!("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID");
    println!("ℹ️  Only CALL can use either ID or label\n");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ekoDB Scripts Example (Rust/HTTP)\n");

    dotenv::dotenv().ok();

    setup_test_data().await?;
    let get_active_users_id = simple_query_function().await?;
    complex_filter_function().await?;
    let user_stats_id = aggregation_function().await?;
    function_management(&get_active_users_id, &user_stats_id).await?;

    println!("✅ All examples completed!");

    Ok(())
}
