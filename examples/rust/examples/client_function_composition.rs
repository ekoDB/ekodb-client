//! Function Composition Example for ekoDB Rust Client
//!
//! Demonstrates calling Functions within Functions using CallFunction
//! Shows how to build reusable logic blocks and compose complex workflows
use ekodb_client::{
    extract_record, get_string_value, Client, Error, FieldType, Function, FunctionCondition,
    ParameterDefinition, Record, UserFunction,
};
use serde_json::Value;
use std::{collections::HashMap, env};

const TEST_COLLECTION: &str = "function_composition_users_rs";
const FETCH_USER: &str = "fetch_user_rs";
const GET_USER_WRAPPER: &str = "get_user_wrapper_rs";
const FETCH_AND_STORE_USER: &str = "fetch_and_store_user_rs";
const SWR_USER: &str = "swr_user_rs";
const VALIDATE_USER: &str = "validate_user_rs";
const FETCH_SLIM_USER: &str = "fetch_slim_user_rs";
const GET_VERIFIED_USER: &str = "get_verified_user_rs";
const USER_CACHE_KEY: &str = "user_cache_rs:{{user_id}}";
const USER_CACHE_ENTRY: &str = "user_cache_rs:1";

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

fn swr_payload(record: &Record) -> Result<Value, Box<dyn std::error::Error>> {
    let json = serde_json::to_value(record)?;
    let value = unwrap_typed(
        json.get("value")
            .ok_or("SWR record did not include value")?,
    );
    if value["id"] != 1 || value["name"] != "Leanne Graham" {
        return Err(format!("unexpected SWR payload: {value}").into());
    }
    Ok(value.clone())
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("=== ekoDB Function Composition Examples ===\n");

    if let Err(error) = client.delete_collection(TEST_COLLECTION).await {
        if !is_not_found(&error) {
            return Err(error.into());
        }
    }
    if client.kv_exists(USER_CACHE_ENTRY).await? {
        client.kv_delete(USER_CACHE_ENTRY).await?;
    }

    let mut function_ids = Vec::new();
    let operation_result = async {
        setup_test_data(&client).await?;
        basic_composition_example(&client, &mut function_ids).await?;
        swr_with_composition_example(&client, &mut function_ids).await?;
        nested_composition_example(&client, &mut function_ids).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;

    let mut cleanup_errors = Vec::new();
    for function_id in function_ids.iter().rev() {
        if let Err(error) = client.delete_function(function_id).await {
            cleanup_errors.push(format!("function {function_id}: {error}"));
        }
    }
    if let Err(error) = client.delete_collection(TEST_COLLECTION).await {
        if !is_not_found(&error) {
            cleanup_errors.push(format!("collection {TEST_COLLECTION}: {error}"));
        }
    }
    match client.kv_exists(USER_CACHE_ENTRY).await {
        Ok(true) => {
            if let Err(error) = client.kv_delete(USER_CACHE_ENTRY).await {
                cleanup_errors.push(format!("KV key {USER_CACHE_ENTRY}: {error}"));
            }
        }
        Ok(false) => {}
        Err(error) => cleanup_errors.push(format!("KV key {USER_CACHE_ENTRY}: {error}")),
    }

    match (operation_result, cleanup_errors.is_empty()) {
        (Err(primary), false) => {
            return Err(std::io::Error::other(format!(
                "{primary}; cleanup also failed: {}",
                cleanup_errors.join("; ")
            ))
            .into());
        }
        (Err(primary), true) => return Err(primary),
        (Ok(()), false) => {
            return Err(std::io::Error::other(format!(
                "cleanup failed: {}",
                cleanup_errors.join("; ")
            ))
            .into());
        }
        (Ok(()), true) => {}
    }

    println!("\n✅ Cleanup complete");
    println!("✅ All composition examples completed!");
    Ok(())
}

async fn setup_test_data(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Setting up test data...\n");

    // Create users
    let mut inserted_ids = Vec::new();
    for i in 1..=5 {
        let mut record = Record::new();
        record.insert("user_code", FieldType::String(format!("user_{}", i)));
        record.insert("name", FieldType::String(format!("User {}", i)));
        record.insert(
            "department",
            FieldType::String(if i <= 2 { "engineering" } else { "sales" }.to_string()),
        );
        record.insert("score", FieldType::Integer(i * 20));
        let inserted = client.insert(TEST_COLLECTION, record, None).await?;
        let id = inserted
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("composition user insert did not return an ID")?;
        if inserted_ids.iter().any(|existing| existing == id) {
            return Err(format!("duplicate composition user ID: {id}").into());
        }
        inserted_ids.push(id.to_string());
    }

    println!("✅ Test data ready\n");
    Ok(())
}

// ============================================================================
// Example 1: Basic Composition - Reusable Building Blocks
// ============================================================================

async fn basic_composition_example(
    client: &Client,
    function_ids: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 1: Basic Function Composition\n");
    println!("Building reusable functions that call each other...\n");

    // Step 1: Create a reusable "fetch_user" function
    // This is a simple, reusable building block
    let fetch_user = UserFunction::new(FETCH_USER, "Fetch user by code")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::FindOne {
            collection: TEST_COLLECTION.to_string(),
            key: "user_code".to_string(),
            value: serde_json::json!("{{user_code}}"),
        });

    function_ids.push(save_or_update(client, fetch_user).await?);
    println!("✅ Saved reusable function: {FETCH_USER}");

    // Step 2: Create a wrapper function that CALLS the base function
    // This demonstrates composability - reusing logic instead of duplicating it
    let get_user_wrapper = UserFunction::new(GET_USER_WRAPPER, "Wrapper that calls fetch_user")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::CallFunction {
            function_label: FETCH_USER.to_string(),
            params: None, // Inherits user_code from parent scope
        })
        .with_function(Function::Project {
            fields: vec!["name".to_string(), "department".to_string()],
            exclude: false,
        });

    function_ids.push(save_or_update(client, get_user_wrapper).await?);
    println!(
        "✅ Saved composed function: {GET_USER_WRAPPER} (calls {FETCH_USER} + projects fields)\n"
    );

    // Step 3: Call the composed function
    let mut params = HashMap::new();
    params.insert(
        "user_code".to_string(),
        FieldType::String("user_1".to_string()),
    );
    let result = client.call_function(GET_USER_WRAPPER, Some(params)).await?;
    println!("📊 Result from composed function:");
    println!("   Records: {}", result.records.len());
    if result.records.len() != 1 {
        return Err(format!("expected one composed user, got {}", result.records.len()).into());
    }
    if let Some(record) = result.records.first() {
        let record_json = serde_json::to_value(record)?;
        let extracted = extract_record(&record_json);

        let name = get_string_value(&extracted["name"]).unwrap_or_else(|| "Unknown".to_string());
        let department =
            get_string_value(&extracted["department"]).unwrap_or_else(|| "Unknown".to_string());

        println!("   Name: {}", name);
        println!("   Department: {}\n", department);
        if name != "User 1" || department != "engineering" {
            return Err(format!("unexpected composed user: {record:?}").into());
        }
    }

    println!("🎯 Key Benefit: fetch_user can be reused by ANY function!");
    println!("   No code duplication, single source of truth\n");

    Ok(())
}

// ============================================================================
// Example 2: SWR Pattern with Reusable Functions
// ============================================================================

async fn swr_with_composition_example(
    client: &Client,
    function_ids: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 2: SWR Pattern with Function Composition\n");
    println!("Using KV cache + CallFunction for fast cache-aside pattern...\n");

    // Step 1: Create reusable "fetch_and_store" function
    // Using jsonplaceholder.typicode.com - a reliable free API for testing
    // This function fetches from API and stores in KV cache
    let fetch_and_store =
        UserFunction::new(FETCH_AND_STORE_USER, "Fetch user from API and cache in KV")
            .with_parameter(ParameterDefinition::new("user_id").required())
            .with_function(Function::HttpRequest {
                url: "https://jsonplaceholder.typicode.com/users/{{user_id}}".to_string(),
                method: "GET".to_string(),
                headers: Some({
                    let mut headers = HashMap::new();
                    headers.insert("Accept".to_string(), "application/json".to_string());
                    headers
                }),
                body: None,
                timeout_seconds: None,
                output_field: None,
            })
            // Store in KV cache (much faster than collection for cache lookups)
            .with_function(Function::KvSet {
                key: serde_json::json!(USER_CACHE_KEY),
                value: serde_json::json!("{{http_response}}"),
                ttl: Some(serde_json::json!(300)), // 5 minute cache
            });

    function_ids.push(save_or_update(client, fetch_and_store).await?);
    println!("✅ Saved reusable function: {FETCH_AND_STORE_USER} (uses KV)");

    // Step 2: Create SWR function that CALLS the reusable fetch function
    // Pattern: KV cache check → populate if missing → return
    let swr_user = UserFunction::new(SWR_USER, "SWR pattern for user data (KV-based)")
        .with_parameter(ParameterDefinition::new("user_id").required())
        // Check KV cache first (O(1) lookup - much faster than FindById)
        .with_function(Function::KvGet {
            key: serde_json::json!(USER_CACHE_KEY),
        })
        .with_function(Function::If {
            // KvGet returns { value: ... } on hit, { value: null } on miss
            // So we check if "value" is not null to detect cache hit
            condition: FunctionCondition::Not {
                condition: Box::new(FunctionCondition::FieldEquals {
                    field: "value".to_string(),
                    value: serde_json::Value::Null,
                }),
            },
            then_functions: vec![
                // Cache hit - project the value field
                Box::new(Function::Project {
                    fields: vec!["value".to_string()],
                    exclude: false,
                }),
            ],
            else_functions: Some(vec![
                // Cache miss - call reusable function to fetch and store
                Box::new(Function::CallFunction {
                    function_label: FETCH_AND_STORE_USER.to_string(),
                    params: Some({
                        let mut p = HashMap::new();
                        p.insert("user_id".to_string(), serde_json::json!("{{user_id}}"));
                        p
                    }),
                }),
                // After storing, retrieve the cached value to return it
                Box::new(Function::KvGet {
                    key: serde_json::json!(USER_CACHE_KEY),
                }),
                Box::new(Function::Project {
                    fields: vec!["value".to_string()],
                    exclude: false,
                }),
            ]),
        });

    function_ids.push(save_or_update(client, swr_user).await?);
    println!("✅ Saved SWR function using composition: {SWR_USER}\n");

    // Step 3: Test the SWR pattern - First call (cache miss)
    println!("First call (cache miss - will fetch from API):");
    let mut params = HashMap::new();
    params.insert("user_id".to_string(), FieldType::String("1".to_string()));

    let start = std::time::Instant::now();
    let result1 = client.call_function(SWR_USER, Some(params.clone())).await?;
    let duration1 = start.elapsed();

    println!("   ⏱️  Duration: {:?}", duration1);
    println!("   📊 Records: {}", result1.records.len());
    if result1.records.len() != 1 {
        return Err(format!(
            "expected one cache-miss record, got {}",
            result1.records.len()
        )
        .into());
    }
    if let Some(record) = result1.records.first() {
        if let Ok(json) = serde_json::to_string_pretty(record) {
            let preview: String = json.chars().take(200).collect();
            println!("   📦 Data: {}...\n", preview);
        }
    } else {
        println!();
    }
    let payload1 = swr_payload(&result1.records[0])?;

    // Step 4: Second call (cache hit)
    println!("Second call (cache hit - from cache):");
    let start = std::time::Instant::now();
    let result2 = client.call_function(SWR_USER, Some(params)).await?;
    let duration2 = start.elapsed();

    println!("   ⏱️  Duration: {:?}", duration2);
    println!("   📊 Records: {}", result2.records.len());
    if result2.records.len() != 1 {
        return Err(format!(
            "expected one cache-hit record, got {}",
            result2.records.len()
        )
        .into());
    }
    if let Some(record) = result2.records.first() {
        if let Ok(json) = serde_json::to_string_pretty(record) {
            let preview: String = json.chars().take(200).collect();
            println!("   📦 Data: {}...", preview);
        }
    }
    if swr_payload(&result2.records[0])? != payload1 {
        return Err("cache hit returned different user data".into());
    }
    let duration2_millis = duration2.as_secs_f64() * 1000.0;
    println!(
        "   🚀 Cache speedup: {:.1}x faster!\n",
        (duration1.as_secs_f64() * 1000.0) / duration2_millis.max(f64::EPSILON)
    );

    Ok(())
}

// ============================================================================
// Example 3: Nested Multi-Level Composition
// ============================================================================

async fn nested_composition_example(
    client: &Client,
    function_ids: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 3: Multi-Level Function Composition\n");
    println!("Building complex workflows from small, reusable pieces...\n");

    // Level 1: Base function - validate user exists
    let validate_user = UserFunction::new(VALIDATE_USER, "Check if user exists")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::FindOne {
            collection: TEST_COLLECTION.to_string(),
            key: "user_code".to_string(),
            value: serde_json::json!("{{user_code}}"),
        });

    function_ids.push(save_or_update(client, validate_user).await?);
    println!("✅ Level 1 function: {VALIDATE_USER}");

    // Level 2: Calls validate_user + projects fields
    let fetch_slim = UserFunction::new(FETCH_SLIM_USER, "Validate and slim down user")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::CallFunction {
            function_label: VALIDATE_USER.to_string(),
            params: None, // Inherits user_code from parent scope
        })
        .with_function(Function::Project {
            fields: vec!["name".to_string(), "department".to_string()],
            exclude: false,
        });

    function_ids.push(save_or_update(client, fetch_slim).await?);
    println!("✅ Level 2 function: {FETCH_SLIM_USER} (calls {VALIDATE_USER})");

    // Level 3: Calls fetch_slim_user (demonstrates 3-level nesting)
    let get_verified_user = UserFunction::new(GET_VERIFIED_USER, "Get verified and validated user")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::CallFunction {
            function_label: FETCH_SLIM_USER.to_string(),
            params: None, // Inherits user_code from parent scope
        });

    function_ids.push(save_or_update(client, get_verified_user).await?);
    println!("✅ Level 3 function: {GET_VERIFIED_USER} (calls {FETCH_SLIM_USER})\n");

    // Execute the 3-level nested composition
    let mut params = HashMap::new();
    params.insert(
        "user_code".to_string(),
        FieldType::String("user_1".to_string()),
    );
    let result = client
        .call_function(GET_VERIFIED_USER, Some(params))
        .await?;
    println!("📊 Result from 3-level nested composition:");
    println!("   Records: {}", result.records.len());
    if result.records.len() != 1 {
        return Err(format!("expected one nested user, got {}", result.records.len()).into());
    }
    if let Some(record) = result.records.first() {
        let record_json = serde_json::to_value(record)?;
        let extracted = extract_record(&record_json);

        let name = get_string_value(&extracted["name"]).unwrap_or_else(|| "Unknown".to_string());
        let department =
            get_string_value(&extracted["department"]).unwrap_or_else(|| "Unknown".to_string());

        println!("   Name: {}", name);
        println!("   Department: {}\n", department);
        if name != "User 1" || department != "engineering" {
            return Err(format!("unexpected nested user: {record:?}").into());
        }
    }

    println!("🎯 Key Benefit: Each function is independently testable and reusable!");
    println!("   - validate_user: Used in 100 different workflows");
    println!("   - fetch_enriched_user: Used in 50 workflows");
    println!("   - get_engineering_user: Specific workflow\n");

    Ok(())
}
