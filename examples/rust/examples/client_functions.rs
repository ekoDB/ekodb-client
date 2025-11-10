///! Scripts Example for ekoDB Rust Client
///!
///! Demonstrates creating, managing, and executing Scripts
use ekodb_client::{
    Client, FieldType, Function, GroupFunctionConfig, GroupFunctionOp, ParameterDefinition, Record,
    Script,
};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Create ekoDB client
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("=== ekoDB Rust Client - Scripts Example ===\n");

    // Setup test data
    setup_test_data(&client).await?;

    // Run examples and track IDs
    let get_active_users_id = simple_query_script(&client).await?;
    let get_users_by_status_id = parameterized_script(&client).await?;
    let user_stats_id = aggregation_script(&client).await?;
    script_management(
        &client,
        &get_active_users_id,
        &get_users_by_status_id,
        &user_stats_id,
    )
    .await?;

    println!("\n✅ All examples completed!");
    Ok(())
}

async fn setup_test_data(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Setting up test data...");

    for i in 1..=10 {
        let mut record = Record::new();
        record.insert("name", FieldType::String(format!("User {}", i)));
        record.insert("age", FieldType::Integer(20 + i));
        record.insert(
            "status",
            FieldType::String(if i % 2 == 0 { "active" } else { "inactive" }.to_string()),
        );
        record.insert("score", FieldType::Integer(i * 10));
        client.insert("users", record, None).await?;
    }

    println!("✅ Test data ready\n");
    Ok(())
}

async fn simple_query_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 1: Simple Query Script\n");

    // Test absolute minimum - just FindAll
    let script =
        Script::new("get_active_users", "Get Active Users").with_function(Function::FindAll {
            collection: "users".to_string(),
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved: {}", id);

    let result = client.call_script("get_active_users", None).await?;
    println!("📊 Found {} active users\n", result.records.len());

    Ok(id)
}

async fn parameterized_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 2: Parameterized Script\n");

    let script = Script::new("get_users_by_status", "Get Users By Status")
        .with_parameter(
            ParameterDefinition::new("status")
                .with_default(FieldType::String("active".to_string())),
        )
        .with_parameter(ParameterDefinition::new("limit").with_default(FieldType::Integer(10)))
        .with_function(Function::FindAll {
            collection: "users".to_string(),
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved: {}", id);

    let mut params = HashMap::new();
    params.insert(
        "status".to_string(),
        FieldType::String("active".to_string()),
    );
    params.insert("limit".to_string(), FieldType::Integer(3));

    let result = client
        .call_script("get_users_by_status", Some(params))
        .await?;
    println!("📊 Found {} users (limited)\n", result.records.len());

    Ok(id)
}

async fn aggregation_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 3: Aggregation Script\n");

    let script = Script::new("user_stats", "User Statistics")
        .with_function(Function::FindAll {
            collection: "users".to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["status".to_string()],
            functions: vec![
                GroupFunctionConfig::new("count", GroupFunctionOp::Count),
                GroupFunctionConfig::new("avg_score", GroupFunctionOp::Average)
                    .with_input_field("score"),
            ],
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved: {}", id);

    let result = client.call_script("user_stats", None).await?;
    println!("📊 Statistics: {} groups\n", result.records.len());

    Ok(id)
}

async fn script_management(
    client: &Client,
    get_active_users_id: &str,
    _get_users_by_status_id: &str,
    user_stats_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 4: Script Management\n");

    // List all scripts
    let scripts = client.list_scripts(None).await?;
    println!("📋 Total scripts: {}", scripts.len());

    // Get specific script by ID
    let script = client.get_script(get_active_users_id).await?;
    println!("🔍 Retrieved script: {}", script.name);

    // Update script by ID
    let updated = Script::new("get_active_users_updated", "Get Active Users (Updated)")
        .with_description("Updated description")
        .with_version("1.1")
        .with_function(Function::FindAll {
            collection: "users".to_string(),
        })
        .with_tag("users");

    client.update_script(get_active_users_id, updated).await?;
    println!("✏️  Script updated");

    // Delete script by ID
    client.delete_script(user_stats_id).await?;
    println!("🗑️  Script deleted\n");

    println!("ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.");

    println!();

    Ok(())
}
