//! SWR (Stale-While-Revalidate) Pattern Example
//!
//! Demonstrates using ekoDB Functions as an SWR cache:
//! - Check cache first, return if exists
//! - On cache miss, fetch from external API
//! - Store result with TTL for auto-expiration

use ekodb_client::{
    Client, Error, FieldType, Function, FunctionCondition, ParameterDefinition, UserFunction,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::time::Instant;

const COLLECTION: &str = "user_cache_rs";
const FUNCTION_LABEL: &str = "fetch_api_user_rs";

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
}

fn unwrap_typed(value: &Value) -> &Value {
    if let Value::Object(object) = value {
        if object.len() == 2 && object.contains_key("type") && object.contains_key("value") {
            return unwrap_typed(&object["value"]);
        }
    }
    value
}

fn user_payload(record: &ekodb_client::Record) -> Result<Value, Box<dyn StdError>> {
    let json = serde_json::to_value(record)?;
    let data = unwrap_typed(
        json.get("data")
            .ok_or("SWR record did not include a data field")?,
    );
    if data["id"] != 1 || data["name"] != "Leanne Graham" {
        return Err(format!("unexpected SWR user payload: {data}").into());
    }
    Ok(data.clone())
}

/// Save a function idempotently: if the label already exists (HTTP 409),
/// update the existing definition instead, then return its id.
async fn save_or_update(
    client: &Client,
    function: UserFunction,
) -> Result<String, Box<dyn std::error::Error>> {
    let label = function.label.clone();
    match client.save_function(function.clone()).await {
        Ok(id) => Ok(id),
        Err(ekodb_client::Error::Api { code: 409, .. }) => {
            client.update_function(&label, function).await?;
            println!("ℹ️  Function '{}' already existed — updated instead", label);
            let existing = client.get_function(&label).await?;
            existing.id.ok_or_else(|| {
                std::io::Error::other(format!("updated function '{label}' did not include an id"))
                    .into()
            })
        }
        Err(e) => Err(Box::new(e)),
    }
}

async fn run_examples(
    client: &Client,
    script_id: &mut Option<String>,
) -> Result<(), Box<dyn StdError>> {
    println!("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n");

    println!("Step 1: Create SWR function that acts as edge cache");

    // Using jsonplaceholder.typicode.com - a reliable free API for testing
    let mut parameters = HashMap::new();
    parameters.insert(
        "user_id".to_string(),
        ParameterDefinition::new("user_id")
            .required()
            .with_description("User ID to fetch"),
    );
    parameters.insert(
        "ttl".to_string(),
        ParameterDefinition::new("ttl")
            .required()
            .with_description("Cache TTL in seconds"),
    );
    parameters.insert(
        "cached_at".to_string(),
        ParameterDefinition::new("cached_at")
            .required()
            .with_description("UTC timestamp when cached"),
    );

    let swr_script = UserFunction {
        id: None,
        label: FUNCTION_LABEL.to_string(),
        name: "Fetch User with Cache".to_string(),
        description: Some("SWR pattern: Check cache, fetch from API if stale".to_string()),
        version: Some("1.0".to_string()),
        parameters,
        functions: vec![
            Function::FindById {
                collection: COLLECTION.to_string(),
                record_id: "{{user_id}}".to_string(),
            },
            Function::If {
                condition: FunctionCondition::HasRecords,
                then_functions: vec![Box::new(Function::Project {
                    fields: vec!["data".to_string(), "cached_at".to_string()],
                    exclude: false,
                })],
                else_functions: Some(vec![
                    Box::new(Function::HttpRequest {
                        url: "https://jsonplaceholder.typicode.com/users/{{user_id}}".to_string(),
                        method: "GET".to_string(),
                        headers: Some(HashMap::from([(
                            "Accept".to_string(),
                            "application/json".to_string(),
                        )])),
                        body: None,
                        timeout_seconds: None,
                        output_field: None,
                    }),
                    Box::new(Function::Insert {
                        collection: COLLECTION.to_string(),
                        record: json!({
                            "id": {"type": "String", "value": "{{user_id}}"},
                            "data": {"type": "Object", "value": "{{http_response}}"},
                            "cached_at": {"type": "String", "value": "{{cached_at}}"}
                        }),
                        bypass_ripple: Some(false),
                        ttl: Some(json!("{{ttl}}")),
                    }),
                ]),
            },
        ],
        tags: vec!["swr".to_string(), "user".to_string(), "cache".to_string()],
        transaction_config: None,
        http_method: None,
        http_path: None,
        created_at: None,
        updated_at: None,
    };

    *script_id = Some(save_or_update(client, swr_script).await?);
    println!(
        "✓ Created SWR script: fetch_api_user_rs ({})\n",
        script_id.as_deref().unwrap()
    );

    println!("Step 2: First call - Cache miss, fetches from API");
    let mut params1 = HashMap::new();
    params1.insert("user_id".to_string(), FieldType::String("1".to_string()));
    params1.insert("ttl".to_string(), FieldType::Integer(300));
    params1.insert(
        "cached_at".to_string(),
        FieldType::String(chrono::Utc::now().to_rfc3339()),
    );
    let result1 = client.call_function(FUNCTION_LABEL, Some(params1)).await?;
    println!("Result: {:?}", result1.stats);
    if result1.records.len() != 1 {
        return Err(format!(
            "expected one cache-miss record, got {}",
            result1.records.len()
        )
        .into());
    }
    let payload1 = user_payload(&result1.records[0])?;
    println!("✓ Data fetched from external API and cached\n");

    println!("Step 3: Second call - Cache hit, instant response from ekoDB");
    let start = Instant::now();
    let mut params2 = HashMap::new();
    params2.insert("user_id".to_string(), FieldType::String("1".to_string()));
    params2.insert("ttl".to_string(), FieldType::Integer(300));
    params2.insert(
        "cached_at".to_string(),
        FieldType::String(chrono::Utc::now().to_rfc3339()),
    );
    let result2 = client.call_function(FUNCTION_LABEL, Some(params2)).await?;
    let duration = start.elapsed();
    println!(
        "Response time: {}ms (served from cache)",
        duration.as_millis()
    );
    if result2.records.len() != 1 {
        return Err(format!(
            "expected one cache-hit record, got {}",
            result2.records.len()
        )
        .into());
    }
    if user_payload(&result2.records[0])? != payload1 {
        return Err("cache hit returned different user data".into());
    }
    println!("✓ Lightning fast cache hit\n");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    dotenv::dotenv().ok();
    let base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        std::env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;
    if let Err(error) = client.delete_collection(COLLECTION).await {
        if !is_not_found(&error) {
            return Err(error.into());
        }
    }

    let mut script_id = None;
    let operation_result = run_examples(&client, &mut script_id).await;
    println!("🧹 Cleaning up...");
    let mut cleanup_errors = Vec::new();
    if let Some(id) = &script_id {
        if let Err(error) = client.delete_function(id).await {
            cleanup_errors.push(format!("function {id}: {error}"));
        }
    }
    if let Err(error) = client.delete_collection(COLLECTION).await {
        if !is_not_found(&error) {
            cleanup_errors.push(format!("collection {COLLECTION}: {error}"));
        }
    }
    match (operation_result, cleanup_errors.is_empty()) {
        (Err(primary), false) => {
            return Err(format!(
                "{primary}; cleanup also failed: {}",
                cleanup_errors.join("; ")
            )
            .into())
        }
        (Err(primary), true) => return Err(primary),
        (Ok(()), false) => {
            return Err(format!("cleanup failed: {}", cleanup_errors.join("; ")).into())
        }
        (Ok(()), true) => {}
    }
    println!("✓ Cleanup complete\n");
    println!("=== SWR Pattern Summary ===");
    println!("✅ Cache miss → Fetch from API → Store in ekoDB");
    println!("✅ Cache hit → Instant response from ekoDB");
    println!("✅ TTL handles automatic cache invalidation");
    Ok(())
}
