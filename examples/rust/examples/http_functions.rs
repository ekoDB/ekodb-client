/*
Functions Example using Direct HTTP Requests

Demonstrates using functions with raw HTTP/reqwest API
No client library required
*/

use serde_json::{json, Value};
use std::env;
use std::sync::OnceLock;
use std::time::Duration;

static AUTH_TOKEN: OnceLock<String> = OnceLock::new();
const TEST_COLLECTION: &str = "functions_users_http_rs";
const ACTIVE_LABEL: &str = "get_active_users_http_rs";
const PAGINATED_LABEL: &str = "get_active_users_paginated_http_rs";
const HIGH_SCORE_LABEL: &str = "get_high_scoring_active_users_http_rs";
const STATS_LABEL: &str = "user_stats_http_rs";

fn http_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
}

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

    let client = http_client()?;
    let response = client
        .post(format!("{}/api/auth/token", base_url))
        .json(&json!({"api_key": api_key}))
        .send()
        .await
        .map_err(|error| format!("POST /api/auth/token: {error}"))?;

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

    let client = http_client()?;
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

    let response = req
        .send()
        .await
        .map_err(|error| format!("{method} {path}: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("HTTP {}: {}", status, text).into());
    }

    let data: Value = response.json().await?;
    Ok(data)
}

/// Save a stored function idempotently over raw HTTP.
///
/// POSTs the definition to `/api/functions`; if the server responds 409
/// (label already exists), PUTs the same definition to
/// `/api/functions/{label}` (the endpoint accepts id-or-label) and returns the
/// existing function's id so the caller's later GET/UPDATE/DELETE-by-id flow
/// keeps working. `label` must match the `label` field in `function`.
async fn save_or_update(
    function: Value,
    label: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = get_base_url();
    let token = get_auth_token().await?;
    let client = http_client()?;

    let response = client
        .post(format!("{}/api/functions", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&function)
        .send()
        .await
        .map_err(|error| format!("POST /api/functions ({label}): {error}"))?;

    if response.status() == reqwest::StatusCode::CONFLICT {
        // Label already exists — update via PUT and recover the id by label.
        request("PUT", &format!("/api/functions/{}", label), Some(function)).await?;
        println!("ℹ️  Function '{}' already existed — updated instead", label);
        let existing = request("GET", &format!("/api/functions/{}", label), None).await?;
        return Ok(existing["id"].as_str().unwrap_or(label).to_string());
    }

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("HTTP {}: {}", status, text).into());
    }

    let _: Value = response.json().await?;
    let saved = request("GET", &format!("/api/functions/{}", label), None).await?;
    saved["id"]
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("saved function '{label}' response did not contain an id").into())
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

        request(
            "POST",
            &format!("/api/insert/{}", TEST_COLLECTION),
            Some(record),
        )
        .await?;
    }

    println!("✅ Test data ready\n");
    Ok(())
}

async fn simple_query_function() -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 1: Simple Query Function with Filter\n");

    let function1 = json!({
        "label": ACTIVE_LABEL,
        "name": "Get Active Users",
        "description": "Query users with active status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": TEST_COLLECTION,
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

    // Save script (idempotent: update if the label already exists)
    let id = save_or_update(function1, ACTIVE_LABEL).await?;
    println!("✅ Function saved: {}", id);

    // Call script (can use label)
    let call_result = request(
        "POST",
        &format!("/api/functions/{}", ACTIVE_LABEL),
        Some(json!({})),
    )
    .await?;
    let records = call_result["records"].as_array().unwrap();
    println!("📊 Found {} active users\n", records.len());

    Ok(id)
}

async fn parameterized_pagination_function() -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 2: Parameterized Pagination with Limit/Skip\n");

    let function2 = json!({
        "label": PAGINATED_LABEL,
        "name": "Get Active Users (Paginated)",
        "version": "1.0",
        "parameters": {
            "page_size": {
                "type": "Integer",
                "required": false,
                "default": 5
            },
            "page_offset": {
                "type": "Integer",
                "required": false,
                "default": 0
            }
        },
        "functions": [{
            "type": "Query",
            "collection": TEST_COLLECTION,
            "filter": {
                "type": "Condition",
                "content": {
                    "field": "status",
                    "operator": "Eq",
                    "value": "active"
                }
            },
            "sort": [{"field": "score", "ascending": false}],
            "limit": "{{page_size}}",
            "skip": "{{page_offset}}"
        }],
        "tags": ["users", "pagination"]
    });

    let id = save_or_update(function2, PAGINATED_LABEL).await?;
    println!("✅ Function saved: {}", id);

    // Call with page 1 (first 3 users)
    let call_result = request(
        "POST",
        &format!("/api/functions/{}", PAGINATED_LABEL),
        Some(json!({"page_size": 3, "page_offset": 0})),
    )
    .await?;
    let records = call_result["records"].as_array().unwrap();
    println!("📊 Page 1: Found {} users (limit=3, skip=0)", records.len());

    // Call with page 2 (next 3 users)
    let call_result = request(
        "POST",
        &format!("/api/functions/{}", PAGINATED_LABEL),
        Some(json!({"page_size": 3, "page_offset": 3})),
    )
    .await?;
    let records = call_result["records"].as_array().unwrap();
    println!(
        "📊 Page 2: Found {} users (limit=3, skip=3)\n",
        records.len()
    );

    Ok(id)
}

async fn complex_filter_function() -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 3: Complex Filter with Multiple Conditions\n");

    let function2 = json!({
        "label": HIGH_SCORE_LABEL,
        "name": "Get High Scoring Active Users",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": TEST_COLLECTION,
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

    let id = save_or_update(function2, HIGH_SCORE_LABEL).await?;
    println!("✅ Function saved: {}", id);

    // Call the function
    let call_result = request(
        "POST",
        &format!("/api/functions/{}", HIGH_SCORE_LABEL),
        Some(json!({})),
    )
    .await?;
    let records = call_result["records"].as_array().unwrap();
    println!(
        "📊 Found {} users (status=active, score>50, sorted by score)\n",
        records.len()
    );

    Ok(id)
}

async fn aggregation_function() -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)\n");

    let function3 = json!({
        "label": STATS_LABEL,
        "name": "User Statistics by Status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": TEST_COLLECTION,
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

    let id = save_or_update(function3, STATS_LABEL).await?;
    println!("✅ Function saved: {}", id);

    let call_result = request(
        "POST",
        &format!("/api/functions/{}", STATS_LABEL),
        Some(json!({})),
    )
    .await?;
    let records = call_result["records"].as_array().unwrap();
    println!(
        "📊 Pipeline Results: Filtered (age>20) → Grouped by status → {} groups",
        records.len()
    );
    for record in records {
        println!("   {}", serde_json::to_string(record)?);
    }
    println!();

    Ok(id)
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
        "label": ACTIVE_LABEL,
        "name": "Get Active Users (Updated)",
        "description": "Updated description",
        "version": "1.1",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": TEST_COLLECTION}
        ],
        "tags": ["users"],
    });
    request(
        "PUT",
        &format!("/api/functions/{}", get_active_users_id),
        Some(updated),
    )
    .await?;
    println!("✏️  Function updated");

    // Delete script (requires encrypted ID)
    request("DELETE", &format!("/api/functions/{}", user_stats_id), None).await?;
    println!("🗑️  Function deleted\n");

    println!("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID");
    println!("ℹ️  Only CALL can use either ID or label\n");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ekoDB Functions Example (Rust/HTTP)\n");

    dotenv::dotenv().ok();

    if let Err(error) = request(
        "DELETE",
        &format!("/api/collections/{}", TEST_COLLECTION),
        None,
    )
    .await
    {
        if !error.to_string().contains("HTTP 404") && !error.to_string().contains("not found") {
            return Err(error);
        }
    }

    let mut function_ids = Vec::new();
    let operation_result: Result<(), Box<dyn std::error::Error>> = async {
        setup_test_data().await?;
        let get_active_users_id = simple_query_function().await?;
        function_ids.push(get_active_users_id.clone());
        function_ids.push(parameterized_pagination_function().await?);
        function_ids.push(complex_filter_function().await?);
        let user_stats_id = aggregation_function().await?;
        function_ids.push(user_stats_id.clone());
        function_management(&get_active_users_id, &user_stats_id).await?;
        println!("✅ All examples completed!");
        Ok(())
    }
    .await;

    let mut cleanup_errors = Vec::new();
    match request("GET", "/api/functions", None).await {
        Ok(functions) => {
            if let Some(functions) = functions.as_array() {
                for function in functions {
                    let is_ours = function["label"].as_str().is_some_and(|label| {
                        [ACTIVE_LABEL, PAGINATED_LABEL, HIGH_SCORE_LABEL, STATS_LABEL]
                            .contains(&label)
                    });
                    if is_ours {
                        if let Some(id) = function["id"].as_str() {
                            if !function_ids.iter().any(|existing| existing == id) {
                                function_ids.push(id.to_string());
                            }
                        }
                    }
                }
            }
        }
        Err(error) => cleanup_errors.push(format!("discover functions: {error}")),
    }
    for id in function_ids {
        if let Err(error) = request("DELETE", &format!("/api/functions/{id}"), None).await {
            if !error.to_string().contains("HTTP 404") && !error.to_string().contains("not found") {
                cleanup_errors.push(format!("function {id}: {error}"));
            }
        }
    }
    if let Err(error) = request(
        "DELETE",
        &format!("/api/collections/{}", TEST_COLLECTION),
        None,
    )
    .await
    {
        if !error.to_string().contains("HTTP 404") && !error.to_string().contains("not found") {
            cleanup_errors.push(format!("collection {TEST_COLLECTION}: {error}"));
        }
    }

    if let Err(error) = operation_result {
        if !cleanup_errors.is_empty() {
            eprintln!("Cleanup also failed: {}", cleanup_errors.join("; "));
        }
        return Err(error);
    }
    if !cleanup_errors.is_empty() {
        return Err(format!("Cleanup failed: {}", cleanup_errors.join("; ")).into());
    }
    Ok(())
}
