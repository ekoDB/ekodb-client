///! Functions Example for ekoDB Rust Client
///!
///! Demonstrates creating, managing, and executing functions
use ekodb_client::{
    Client, FieldType, Function, GroupFunctionConfig, GroupFunctionOp, ParameterDefinition,
    QueryConditionOperator, QueryExpression, Record, SortFieldConfig, UserFunction,
};
use std::{collections::HashMap, env};

const TEST_COLLECTION: &str = "functions_users_client_rs";
const ACTIVE_LABEL: &str = "get_active_users_client_rs";
const UPDATED_ACTIVE_LABEL: &str = "get_active_users_updated_client_rs";
const STATUS_LABEL: &str = "get_users_by_status_client_rs";
const STATS_LABEL: &str = "user_stats_client_rs";

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
    let _ = client.delete_collection(TEST_COLLECTION).await;
    let operation_result: Result<(), Box<dyn std::error::Error>> = async {
        setup_test_data(&client).await?;
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
        Ok(())
    }
    .await;

    let mut cleanup_errors = Vec::new();
    for label in [
        ACTIVE_LABEL,
        UPDATED_ACTIVE_LABEL,
        STATUS_LABEL,
        STATS_LABEL,
    ] {
        if let Err(error) = client.delete_function(label).await {
            if !error.to_string().contains("not found") {
                cleanup_errors.push(format!("function {label}: {error}"));
            }
        }
    }
    if let Err(error) = client.delete_collection(TEST_COLLECTION).await {
        cleanup_errors.push(format!("collection {TEST_COLLECTION}: {error}"));
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
    println!("\n✅ All examples completed!");
    Ok(())
}

/// Save a function idempotently: if the label already exists (HTTP 409),
/// update the existing definition instead, then return its id so the rest of
/// the example (get/update/delete by id) continues to work unchanged.
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
            // We don't get a fresh id back from update; fetch by label to recover it.
            let existing = client.get_function(&label).await?;
            Ok(existing.id.unwrap_or(label))
        }
        Err(e) => Err(Box::new(e)),
    }
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
        client.insert(TEST_COLLECTION, record, None).await?;
    }

    println!("✅ Test data ready\n");
    Ok(())
}

async fn simple_query_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 1: Simple Query Function\n");

    let script =
        UserFunction::new(ACTIVE_LABEL, "Get Active Users").with_function(Function::Query {
            collection: TEST_COLLECTION.to_string(),
            filter: Some(QueryExpression::condition(
                "status",
                QueryConditionOperator::Eq,
                serde_json::json!("active"),
            )),
            sort: None,
            limit: None,
            skip: None,
        });

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved: {}", id);

    let result = client.call_function(ACTIVE_LABEL, None).await?;
    println!("📊 Found {} active users\n", result.records.len());

    Ok(id)
}

async fn parameterized_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 2: Parameterized Function\n");

    let script = UserFunction::new(STATUS_LABEL, "Get Users By Status")
        .with_parameter(
            ParameterDefinition::new("status")
                .with_default(FieldType::String("active".to_string())),
        )
        .with_parameter(ParameterDefinition::new("limit").with_default(FieldType::Integer(10)))
        .with_function(Function::Query {
            collection: TEST_COLLECTION.to_string(),
            filter: Some(QueryExpression::condition(
                "status",
                QueryConditionOperator::Eq,
                serde_json::json!("{{status}}"),
            )),
            sort: Some(vec![SortFieldConfig::new("score").descending()]),
            limit: Some(serde_json::json!("{{limit}}")),
            skip: None,
        });

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved: {}", id);

    let mut params = HashMap::new();
    params.insert(
        "status".to_string(),
        FieldType::String("active".to_string()),
    );
    params.insert("limit".to_string(), FieldType::Integer(3));

    let result = client.call_function(STATUS_LABEL, Some(params)).await?;
    println!("📊 Found {} users (limited)\n", result.records.len());

    Ok(id)
}

async fn aggregation_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 3: Aggregation Function\n");

    let script = UserFunction::new(STATS_LABEL, "User Statistics")
        .with_function(Function::FindAll {
            collection: TEST_COLLECTION.to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["status".to_string()],
            functions: vec![
                GroupFunctionConfig::new("count", GroupFunctionOp::Count),
                GroupFunctionConfig::new("avg_score", GroupFunctionOp::Average)
                    .with_input_field("score"),
            ],
        });

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved: {}", id);

    let result = client.call_function(STATS_LABEL, None).await?;
    println!("📊 Statistics: {} groups\n", result.records.len());

    Ok(id)
}

async fn script_management(
    client: &Client,
    get_active_users_id: &str,
    _get_users_by_status_id: &str,
    user_stats_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 4: Function Management\n");

    // List all functions
    let functions = client.list_functions(None).await?;
    println!("📋 Total functions: {}", functions.len());

    // Get specific function by ID
    let function = client.get_function(get_active_users_id).await?;
    println!("🔍 Retrieved function: {}", function.name);

    // Update function by ID
    let updated = UserFunction::new(UPDATED_ACTIVE_LABEL, "Get Active Users (Updated)")
        .with_description("Updated description")
        .with_version("1.1")
        .with_function(Function::FindAll {
            collection: TEST_COLLECTION.to_string(),
        })
        .with_tag("users");

    client.update_function(get_active_users_id, updated).await?;
    println!("✏️  Function updated");

    // Delete function by ID
    client.delete_function(user_stats_id).await?;
    println!("🗑️  Function deleted\n");

    println!("ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.");

    println!();

    Ok(())
}
