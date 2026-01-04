///! Function Composition Example for ekoDB Rust Client
///!
///! Demonstrates calling Functions within Functions using CallFunction
///! Shows how to build reusable logic blocks and compose complex workflows
use ekodb_client::{
    Client, FieldType, Function, ParameterDefinition, Record, Script, ScriptCondition,
};
use std::{collections::HashMap, env};

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

    // Setup test data
    setup_test_data(&client).await?;

    // Example 1: Basic composability
    basic_composition_example(&client).await?;

    // Example 2: SWR pattern with reusable enrichment
    swr_with_composition_example(&client).await?;

    // Example 3: Multi-level composition
    nested_composition_example(&client).await?;

    println!("\n✅ All composition examples completed!");
    Ok(())
}

async fn setup_test_data(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Setting up test data...\n");

    // Create users
    for i in 1..=5 {
        let mut record = Record::new();
        record.insert("user_code", FieldType::String(format!("user_{}", i)));
        record.insert("name", FieldType::String(format!("User {}", i)));
        record.insert(
            "department",
            FieldType::String(if i <= 2 { "engineering" } else { "sales" }.to_string()),
        );
        record.insert("score", FieldType::Integer(i * 20));
        client.insert("users", record, None).await?;
    }

    println!("✅ Test data ready\n");
    Ok(())
}

// ============================================================================
// Example 1: Basic Composition - Reusable Building Blocks
// ============================================================================

async fn basic_composition_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 1: Basic Function Composition\n");
    println!("Building reusable functions that call each other...\n");

    // Step 1: Create a reusable "fetch_user" function
    // This is a simple, reusable building block
    let fetch_user = Script::new("fetch_user", "Fetch user by code")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::FindOne {
            collection: "users".to_string(),
            key: "user_code".to_string(),
            value: serde_json::json!("{{user_code}}"),
        });

    client.save_script(fetch_user).await?;
    println!("✅ Saved reusable function: fetch_user");

    // Step 2: Create a wrapper function that CALLS the base function
    // This demonstrates composability - reusing logic instead of duplicating it
    let get_user_wrapper = Script::new("get_user_wrapper", "Wrapper that calls fetch_user")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::CallFunction {
            function_label: "fetch_user".to_string(),
            params: Some({
                let mut map = HashMap::new();
                map.insert("user_code".to_string(), serde_json::json!("{{user_code}}"));
                map
            }),
        })
        .with_function(Function::Project {
            fields: vec!["name".to_string(), "department".to_string()],
            exclude: false,
        });

    client.save_script(get_user_wrapper).await?;
    println!("✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)\n");

    // Step 3: Call the composed function
    let mut params = HashMap::new();
    params.insert(
        "user_code".to_string(),
        FieldType::String("user_1".to_string()),
    );
    let result = client.call_script("get_user_wrapper", Some(params)).await?;
    println!("📊 Result from composed function:");
    println!("   Records: {}", result.records.len());
    if let Some(record) = result.records.first() {
        println!("   Name: {:?}", record.get("name"));
        println!("   Department: {:?}\n", record.get("department"));
    }

    println!("🎯 Key Benefit: fetch_user can be reused by ANY function!");
    println!("   No code duplication, single source of truth\n");

    Ok(())
}

// ============================================================================
// Example 2: SWR Pattern with Reusable Functions
// ============================================================================

async fn swr_with_composition_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 2: SWR Pattern with Function Composition\n");
    println!("Using CallFunction to replace inline logic in SWR pattern...\n");

    // Step 1: Create reusable "fetch_external_data" function
    // This encapsulates all the logic for fetching and storing external data
    let fetch_and_store = Script::new("fetch_and_store_github", "Fetch from GitHub and store")
        .with_parameter(ParameterDefinition::new("username").required())
        .with_function(Function::HttpRequest {
            url: "https://api.github.com/users/{{username}}".to_string(),
            method: "GET".to_string(),
            headers: Some({
                let mut headers = HashMap::new();
                headers.insert("User-Agent".to_string(), "ekoDB-Client".to_string());
                headers
            }),
            body: None,
            timeout_seconds: None,
            output_field: None,
        })
        .with_function(Function::Insert {
            collection: "github_cache".to_string(),
            record: serde_json::json!({
                "id": {"type": "String", "value": "{{username}}"},
                "data": {"type": "Object", "value": "{{http_response}}"}
            }),
            bypass_ripple: None,
            ttl: Some(serde_json::json!(300)), // 5 minute cache
        });

    client.save_script(fetch_and_store).await?;
    println!("✅ Saved reusable function: fetch_and_store_github");

    // Step 2: Create SWR function that CALLS the reusable fetch function
    // Notice how clean this is - no inline HttpRequest or Insert logic!
    let swr_github = Script::new("swr_github_user", "SWR pattern using reusable functions")
        .with_parameter(ParameterDefinition::new("username").required())
        .with_function(Function::FindById {
            collection: "github_cache".to_string(),
            record_id: "{{username}}".to_string(),
        })
        .with_function(Function::If {
            condition: ScriptCondition::HasRecords,
            then_functions: vec![Box::new(Function::Project {
                fields: vec!["data".to_string()],
                exclude: false,
            })],
            else_functions: Some(vec![
                // Instead of inline HttpRequest + Insert, just call the reusable function!
                Box::new(Function::CallFunction {
                    function_label: "fetch_and_store_github".to_string(),
                    params: Some({
                        let mut params = HashMap::new();
                        params.insert("username".to_string(), serde_json::json!("{{username}}"));
                        params
                    }),
                }),
            ]),
        });

    client.save_script(swr_github).await?;
    println!("✅ Saved SWR function using composition: swr_github_user\n");

    // Step 3: Test the SWR pattern - First call (cache miss)
    println!("First call (cache miss - will fetch from GitHub):");
    let mut params = HashMap::new();
    params.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );

    let start = std::time::Instant::now();
    let result1 = client
        .call_script("swr_github_user", Some(params.clone()))
        .await?;
    let duration1 = start.elapsed();

    println!("   ⏱️  Duration: {:?}", duration1);
    println!("   📊 Records: {}\n", result1.records.len());

    // Step 4: Second call (cache hit)
    println!("Second call (cache hit - from cache):");
    let start = std::time::Instant::now();
    let result2 = client.call_script("swr_github_user", Some(params)).await?;
    let duration2 = start.elapsed();

    println!("   ⏱️  Duration: {:?}", duration2);
    println!("   📊 Records: {}", result2.records.len());
    println!(
        "   🚀 Cache speedup: {:.1}x faster!\n",
        duration1.as_millis() as f64 / duration2.as_millis() as f64
    );

    Ok(())
}

// ============================================================================
// Example 3: Nested Multi-Level Composition
// ============================================================================

async fn nested_composition_example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 3: Multi-Level Function Composition\n");
    println!("Building complex workflows from small, reusable pieces...\n");

    // Level 1: Base function - validate user exists
    let validate_user = Script::new("validate_user", "Check if user exists")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::FindOne {
            collection: "users".to_string(),
            key: "user_code".to_string(),
            value: serde_json::json!("{{user_code}}"),
        });

    client.save_script(validate_user).await?;
    println!("✅ Level 1 function: validate_user");

    // Level 2: Calls validate_user + projects fields
    let fetch_slim = Script::new("fetch_slim_user", "Validate and slim down user")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::CallFunction {
            function_label: "validate_user".to_string(),
            params: Some({
                let mut map = HashMap::new();
                map.insert("user_code".to_string(), serde_json::json!("{{user_code}}"));
                map
            }),
        })
        .with_function(Function::Project {
            fields: vec!["name".to_string(), "department".to_string()],
            exclude: false,
        });

    client.save_script(fetch_slim).await?;
    println!("✅ Level 2 function: fetch_slim_user (calls validate_user)");

    // Level 3: Calls fetch_slim_user + counts
    let count_user = Script::new("count_validated_user", "Get validated user and count")
        .with_parameter(ParameterDefinition::new("user_code").required())
        .with_function(Function::CallFunction {
            function_label: "fetch_slim_user".to_string(),
            params: Some({
                let mut map = HashMap::new();
                map.insert("user_code".to_string(), serde_json::json!("{{user_code}}"));
                map
            }),
        })
        .with_function(Function::Count {
            output_field: "record_count".to_string(),
        });

    client.save_script(count_user).await?;
    println!("✅ Level 3 function: count_validated_user (calls fetch_slim_user)\n");

    // Execute the 3-level nested composition
    let mut params = HashMap::new();
    params.insert(
        "user_code".to_string(),
        FieldType::String("user_1".to_string()),
    );
    let result = client
        .call_script("count_validated_user", Some(params))
        .await?;
    println!("📊 Result from 3-level nested composition:");
    println!("   Records: {}", result.records.len());
    if let Some(record) = result.records.first() {
        println!("   Name: {:?}", record.get("name"));
        println!("   Department: {:?}", record.get("department"));
        println!("   Record count: {:?}\n", record.get("record_count"));
    }

    println!("🎯 Key Benefit: Each function is independently testable and reusable!");
    println!("   - validate_user: Used in 100 different workflows");
    println!("   - fetch_enriched_user: Used in 50 workflows");
    println!("   - get_engineering_user: Specific workflow\n");

    Ok(())
}
