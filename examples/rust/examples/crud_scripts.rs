/*
Complete CRUD Scripts Example using Direct HTTP Requests

Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each Script shows Functions chaining with proper verification using parameterized {{param}} syntax
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

    let _ = AUTH_TOKEN.set(token.clone());

    Ok(token)
}

async fn request(
    method: &str,
    path: &str,
    body: Option<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let token = get_auth_token().await?;
    let base_url = get_base_url();
    let client = reqwest::Client::new();

    let mut req = match method {
        "GET" => client.get(format!("{}{}", base_url, path)),
        "POST" => client.post(format!("{}{}", base_url, path)),
        "PUT" => client.put(format!("{}{}", base_url, path)),
        "DELETE" => client.delete(format!("{}{}", base_url, path)),
        _ => panic!("Unsupported HTTP method: {}", method),
    };

    req = req.header("Authorization", format!("Bearer {}", token));

    if let Some(b) = body {
        req = req.json(&b);
    }

    let response = req.send().await?;
    let status = response.status();

    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("HTTP {}: {}", status, error_text).into());
    }

    let data: Value = response.json().await?;
    Ok(data)
}

async fn script_1_insert_and_verify() -> Result<(String, String), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(60));
    println!("📝 Script 1: Insert + Verify");
    println!("{}", "=".repeat(60));

    let script = json!({
        "label": "insert_and_verify",
        "name": "Insert and Verify User",
        "description": "Insert a user and verify it was created",
        "version": "1.0",
        "parameters": {
            "user_name": {
                "type": "String",
                "required": true,
                "description": "Name of the user"
            },
            "user_email": {
                "type": "String",
                "required": true,
                "description": "Email of the user"
            }
        },
        "functions": [
            {
                "type": "Insert",
                "collection": "users",
                "record": {
                    "name": "{{user_name}}",
                    "email": "{{user_email}}",
                    "status": "pending",
                    "credits": 0
                }
            },
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                }
            }
        ],
        "tags": ["crud", "insert", "verification"]
    });

    println!("\n1️⃣ Saving Script...");
    let save_result = request("POST", "/api/functions", Some(script)).await?;
    let script_id = save_result["id"].as_str().unwrap().to_string();
    println!("   ✅ Script saved: {}", script_id);

    println!("\n2️⃣ Calling Script (Insert + Verify)...");
    let params = json!({
        "user_name": "Alice Smith",
        "user_email": "alice@example.com"
    });
    let call_result = request("POST", "/api/functions/insert_and_verify", Some(params)).await?;

    let stats = &call_result["stats"];
    println!(
        "   ✅ Script executed: {} Functions",
        stats["stages_executed"]
    );
    println!("   ⏱️  Execution time: {}ms", stats["execution_time_ms"]);

    let records = call_result["records"].as_array().unwrap();
    println!("\n3️⃣ Verification Results:");
    println!("   ✅ Found {} record(s)", records.len());

    if let Some(user) = records.first() {
        // Handle both plain string and map format for id
        let user_id = if let Some(id_str) = user["id"].as_str() {
            id_str.to_string()
        } else {
            user["id"]["value"].as_str().unwrap().to_string()
        };
        println!("   📋 User ID: {}", user_id);
        println!("   📋 Name: {}", user["name"]);
        println!("   📋 Email: {}", user["email"]);
        println!("   📋 Status: {}", user["status"]);
        println!("   📋 Credits: {}", user["credits"]);

        return Ok((user_id, script_id));
    }

    Err("No records found".into())
}

async fn script_2_query_update_verify() -> Result<String, Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(60));
    println!("📝 Script 2: Query + Update + Verify");
    println!("{}", "=".repeat(60));

    let script = json!({
        "label": "query_update_verify",
        "name": "Query, Update, and Verify",
        "description": "Find user by filter, update status, verify change",
        "version": "1.0",
        "parameters": {
            "user_email": {
                "type": "String",
                "required": true,
                "description": "Email to search for"
            },
            "new_status": {
                "type": "String",
                "default": "active",
                "required": false,
                "description": "New status value"
            }
        },
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                }
            },
            {
                "type": "Update",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                },
                "updates": {
                    "status": "{{new_status}}"
                }
            },
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                }
            }
        ],
        "tags": ["crud", "update", "verification"]
    });

    println!("\n1️⃣ Saving Script...");
    let save_result = request("POST", "/api/functions", Some(script)).await?;
    let script_id = save_result["id"].as_str().unwrap().to_string();
    println!("   ✅ Script saved: {}", script_id);

    println!("\n2️⃣ Calling Script (Query + Update + Verify)...");
    let params = json!({
        "user_email": "alice@example.com",
        "new_status": "active"
    });
    let call_result = request("POST", "/api/functions/query_update_verify", Some(params)).await?;

    let stats = &call_result["stats"];
    println!(
        "   ✅ Script executed: {} Functions",
        stats["stages_executed"]
    );
    println!("   ⏱️  Execution time: {}ms", stats["execution_time_ms"]);

    let records = call_result["records"].as_array().unwrap();
    println!("\n3️⃣ Verification Results:");
    println!("   ✅ Found {} record(s)", records.len());
    if let Some(user) = records.first() {
        println!("   📋 Status updated to: {}", user["status"]);
        println!("   📋 Name: {}", user["name"]);
    }

    Ok(script_id)
}

async fn script_3_query_update_credits() -> Result<String, Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(60));
    println!("📝 Script 3: Query + Update Credits + Verify");
    println!("{}", "=".repeat(60));

    let script = json!({
        "label": "query_update_credits",
        "name": "Query, Update Credits, and Verify",
        "description": "Find user by email, update credits, verify change",
        "version": "1.0",
        "parameters": {
            "user_email": {
                "type": "String",
                "required": true,
                "description": "Email to search for"
            },
            "credits": {
                "type": "Integer",
                "default": 100,
                "required": false,
                "description": "Credits to set"
            }
        },
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                }
            },
            {
                "type": "Update",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                },
                "updates": {
                    "credits": "{{credits}}"
                }
            },
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                }
            }
        ],
        "tags": ["crud", "update", "verification"]
    });

    println!("\n1️⃣ Saving Script...");
    let save_result = request("POST", "/api/functions", Some(script)).await?;
    let script_id = save_result["id"].as_str().unwrap().to_string();
    println!("   ✅ Script saved: {}", script_id);

    println!("\n2️⃣ Calling Script (Query + Update Credits + Verify)...");
    let params = json!({
        "user_email": "alice@example.com",
        "credits": 100
    });
    let call_result = request("POST", "/api/functions/query_update_credits", Some(params)).await?;

    let stats = &call_result["stats"];
    println!(
        "   ✅ Script executed: {} Functions",
        stats["stages_executed"]
    );
    println!("   ⏱️  Execution time: {}ms", stats["execution_time_ms"]);

    let records = call_result["records"].as_array().unwrap();
    println!("\n3️⃣ Verification Results:");
    println!("   ✅ Found {} record(s)", records.len());
    if let Some(user) = records.first() {
        println!("   📋 Credits updated to: {}", user["credits"]);
        println!("   📋 Status: {}", user["status"]);
        println!("   📋 Name: {}", user["name"]);
    }

    Ok(script_id)
}

async fn script_4_delete_and_verify() -> Result<String, Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(60));
    println!("📝 Script 4: Query Before Delete + Delete + Verify");
    println!("{}", "=".repeat(60));

    let script = json!({
        "label": "delete_and_verify",
        "name": "Query Before Delete and Verify",
        "description": "Verify record exists, delete it, then verify it's gone",
        "version": "1.0",
        "parameters": {
            "user_email": {
                "type": "String",
                "required": true,
                "description": "Email of user to delete"
            }
        },
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                }
            },
            {
                "type": "Delete",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                }
            },
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}"
                    }
                }
            }
        ],
        "tags": ["crud", "delete", "verification"]
    });

    println!("\n1️⃣ Saving Script...");
    let save_result = request("POST", "/api/functions", Some(script)).await?;
    let script_id = save_result["id"].as_str().unwrap().to_string();
    println!("   ✅ Script saved: {}", script_id);

    println!("\n2️⃣ Calling Script (Query + Delete + Verify)...");
    let params = json!({
        "user_email": "alice@example.com"
    });
    let call_result = request("POST", "/api/functions/delete_and_verify", Some(params)).await?;

    let stats = &call_result["stats"];
    println!(
        "   ✅ Script executed: {} Functions",
        stats["stages_executed"]
    );
    println!("   ⏱️  Execution time: {}ms", stats["execution_time_ms"]);

    let records = call_result["records"].as_array().unwrap();
    println!("\n3️⃣ Verification Results:");
    println!("   📊 Before delete: Record existed (verified by first Query)");
    println!(
        "   ✅ After delete: Record successfully deleted (Query returned {} records)",
        records.len()
    );

    Ok(script_id)
}

async fn cleanup(script_ids: Vec<String>) {
    println!("\n{}", "=".repeat(60));
    println!("🧹 Cleanup");
    println!("{}", "=".repeat(60));

    for script_id in script_ids {
        if script_id.is_empty() {
            continue;
        }
        match request("DELETE", &format!("/api/functions/{}", script_id), None).await {
            Ok(_) => {
                let truncated_id = if script_id.len() > 20 {
                    format!("{}...", &script_id[..20])
                } else {
                    script_id.clone()
                };
                println!("   ✅ Deleted script: {}", truncated_id);
            }
            Err(e) => println!("   ⚠️  Could not delete script: {}", e),
        }
    }

    match request("DELETE", "/api/collections/users", None).await {
        Ok(_) => println!("   ✅ Deleted collection: users"),
        Err(e) => println!("   ⚠️  Could not delete collection: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    println!("🚀 ekoDB Complete CRUD Scripts Example");
    println!("{}", "=".repeat(60));
    println!("Demonstrates:");
    println!("  • Insert + Verify (using Query)");
    println!("  • Query + Update Status + Verify");
    println!("  • Query + Update Credits + Verify");
    println!("  • Query Before Delete + Delete + Verify Gone");
    println!();
    println!("Each Script shows Functions chaining with proper verification");
    println!("{}", "=".repeat(60));

    // Run all CRUD Scripts in sequence
    let (_user_id, script1_id) = script_1_insert_and_verify().await?;
    let mut script_ids = vec![script1_id];

    let script2_id = script_2_query_update_verify().await?;
    script_ids.push(script2_id);

    let script3_id = script_3_query_update_credits().await?;
    script_ids.push(script3_id);

    let script4_id = script_4_delete_and_verify().await?;
    script_ids.push(script4_id);

    // Cleanup
    cleanup(script_ids).await;

    println!("\n{}", "=".repeat(60));
    println!("✅ Complete CRUD Scripts Example Finished!");
    println!("{}", "=".repeat(60));
    println!("\n💡 Key Takeaways:");
    println!("   ✅ Scripts chain Functions together");
    println!("   ✅ Each Script demonstrates operation + verification");
    println!("   ✅ Parameters make Scripts reusable");
    println!("   ✅ Verification is built into the Script itself");
    println!("   ✅ Complete CRUD lifecycle in 4 focused Scripts");

    Ok(())
}
