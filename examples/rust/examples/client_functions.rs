///! Saved Functions Example for ekoDB Rust Client
///!
///! Demonstrates creating, managing, and executing saved functions
use ekodb_client::{
    Client, FieldType, FunctionStageConfig, GroupFunctionConfig, GroupFunctionOp,
    ParameterDefinition, Record, SavedFunction,
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

    println!("=== ekoDB Rust Client - Functions Example ===\n");

    // Setup test data
    setup_test_data(&client).await?;

    // Run examples and track IDs
    let get_active_users_id = simple_query_function(&client).await?;
    parameterized_function(&client).await?;
    let user_stats_id = aggregation_function(&client).await?;
    function_management(&client, &get_active_users_id, &user_stats_id).await?;

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

async fn simple_query_function(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 1: Simple Query Function\n");

    // Test absolute minimum - just FindAll
    let function = SavedFunction::new("get_active_users", "Get Active Users").with_stage(
        FunctionStageConfig::FindAll {
            collection: "users".to_string(),
        },
    );

    let id = client.save_function(function).await?;
    println!("✅ Function saved: {}", id);

    let result = client.call_function(&id, None).await?;
    println!("📊 Found {} active users\n", result.records.len());

    Ok(id)
}

async fn parameterized_function(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 2: Parameterized Function\n");

    let function = SavedFunction::new("get_users_by_status", "Get Users By Status")
        .with_parameter(
            ParameterDefinition::new("status")
                .with_default(FieldType::String("active".to_string())),
        )
        .with_parameter(ParameterDefinition::new("limit").with_default(FieldType::Integer(10)))
        .with_stage(FunctionStageConfig::FindAll {
            collection: "users".to_string(),
        });

    let id = client.save_function(function).await?;
    println!("✅ Function saved: {}", id);

    let mut params = HashMap::new();
    params.insert(
        "status".to_string(),
        FieldType::String("active".to_string()),
    );
    params.insert("limit".to_string(), FieldType::Integer(3));

    let result = client
        .call_function("get_users_by_status", Some(params))
        .await?;
    println!("📊 Found {} users (limited)\n", result.records.len());

    Ok(())
}

async fn aggregation_function(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 3: Aggregation Function\n");

    let function = SavedFunction::new("user_stats", "User Statistics")
        .with_stage(FunctionStageConfig::FindAll {
            collection: "users".to_string(),
        })
        .with_stage(FunctionStageConfig::Group {
            by_fields: vec!["status".to_string()],
            functions: vec![
                GroupFunctionConfig::new("count", GroupFunctionOp::Count),
                GroupFunctionConfig::new("avg_score", GroupFunctionOp::Average)
                    .with_input_field("score"),
            ],
        });

    let id = client.save_function(function).await?;
    println!("✅ Function saved: {}", id);

    let result = client.call_function("user_stats", None).await?;
    println!("📊 Statistics: {} groups\n", result.records.len());

    Ok(id)
}

async fn function_management(
    client: &Client,
    get_active_users_id: &str,
    user_stats_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 4: Function Management\n");

    // List all functions
    let functions = client.list_functions(None).await?;
    println!("📋 Total functions: {}", functions.len());

    // Get specific function (requires ID, not label)
    let func = client.get_function(get_active_users_id).await?;
    println!("🔍 Retrieved function: {}", func.name);

    // Update function (requires ID, not label)
    let updated = SavedFunction::new("get_active_users", "Get Active Users (Updated)")
        .with_description("Updated description")
        .with_version("1.1")
        .with_stage(FunctionStageConfig::FindAll {
            collection: "users".to_string(),
        })
        .with_tag("users");

    client.update_function(get_active_users_id, updated).await?;
    println!("✏️  Function updated");

    // Delete function (requires ID, not label)
    client.delete_function(user_stats_id).await?;
    println!("🗑️  Function deleted\n");

    println!("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID returned from save");
    println!("ℹ️  Only CALL can use either ID or label");

    println!();

    Ok(())
}
