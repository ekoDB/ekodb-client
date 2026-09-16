//! CRUD Functions Example - Basic Data Operations
//!
//! Demonstrates basic CRUD operations using scripts:
//! - FindAll queries
//! - Group aggregations
//! - Simple data transformations

use ekodb_client::{
    Client, Error, FieldType, Function, GroupFunctionConfig, GroupFunctionOp, Record, UserFunction,
};
use std::error::Error as StdError;

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
}

fn integer(record: &Record, key: &str) -> Result<i64, Box<dyn StdError>> {
    match record.get(key) {
        Some(FieldType::Integer(value)) => Ok(*value),
        Some(FieldType::Object(wrapper)) => match wrapper.get("value") {
            Some(FieldType::Integer(value)) => Ok(*value),
            value => Err(format!("field '{key}' had unexpected wrapped value: {value:?}").into()),
        },
        value => Err(format!("field '{key}' was not an integer: {value:?}").into()),
    }
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
    script_ids: &mut Vec<String>,
) -> Result<(), Box<dyn StdError>> {
    println!("🚀 ekoDB Rust CRUD Functions Example\n");

    // Setup test data
    println!("📋 Setting up test data...");
    let mut user_ids = Vec::new();
    for i in 1..=10 {
        let status = if i % 3 == 0 { "inactive" } else { "active" };
        let role = if i <= 3 { "admin" } else { "user" };
        let mut record = Record::new();
        record.insert("name", FieldType::String(format!("User {}", i)));
        record.insert("email", FieldType::String(format!("user{}@example.com", i)));
        record.insert("status", FieldType::String(status.to_string()));
        record.insert("score", FieldType::Integer(i * 10));
        record.insert("role", FieldType::String(role.to_string()));
        let inserted = client.insert("crud_users_rs", record, None).await?;
        let id = inserted
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("user insert did not return an ID")?;
        if user_ids.iter().any(|existing| existing == id) {
            return Err(format!("duplicate inserted user ID: {id}").into());
        }
        user_ids.push(id.to_string());
    }
    println!("✅ Created 10 test users\n");

    // Example 1: List All Users
    println!("📝 Example 1: List All Users\n");
    let script1 = UserFunction::new("list_all_users_rs", "List All Users")
        .with_version("1.0")
        .with_function(Function::FindAll {
            collection: "crud_users_rs".to_string(),
        })
        .with_tag("users")
        .with_tag("list");
    let script_id1 = save_or_update(client, script1).await?;
    script_ids.push(script_id1);
    println!("✅ Function saved");

    let result1 = client.call_function("list_all_users_rs", None).await?;
    println!("📊 Found {} users", result1.records.len());
    if result1.records.len() != 10 {
        return Err(format!("expected 10 users, found {}", result1.records.len()).into());
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result1.stats.execution_time_ms
    );

    // Example 2: Count Users by Status
    println!("📝 Example 2: Count Users by Status\n");
    let script2 = UserFunction::new("users_by_status_rs", "Users by Status")
        .with_version("1.0")
        .with_function(Function::FindAll {
            collection: "crud_users_rs".to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["status".to_string()],
            functions: vec![GroupFunctionConfig::new("count", GroupFunctionOp::Count)],
        })
        .with_tag("users")
        .with_tag("analytics");
    let script_id2 = save_or_update(client, script2).await?;
    script_ids.push(script_id2);
    println!("✅ Function saved");

    let result2 = client.call_function("users_by_status_rs", None).await?;
    println!("📊 User counts by status:");
    let mut active = None;
    let mut inactive = None;
    for record in &result2.records {
        println!("   {:?}", record);
        match record.get_string("status") {
            Some("active") => active = Some(integer(record, "count")?),
            Some("inactive") => inactive = Some(integer(record, "count")?),
            value => return Err(format!("unexpected status group: {value:?}").into()),
        }
    }
    if active != Some(7) || inactive != Some(3) || result2.records.len() != 2 {
        return Err(
            format!("unexpected status counts: active={active:?}, inactive={inactive:?}").into(),
        );
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result2.stats.execution_time_ms
    );

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
    let collection = "crud_users_rs";
    if let Err(error) = client.delete_collection(collection).await {
        if !is_not_found(&error) {
            return Err(error.into());
        }
    }

    let mut script_ids = Vec::new();
    let operation_result = run_examples(&client, &mut script_ids).await;
    println!("🧹 Cleaning up...");
    let mut cleanup_errors = Vec::new();
    for script_id in script_ids.iter().rev() {
        if let Err(error) = client.delete_function(script_id).await {
            cleanup_errors.push(format!("function {script_id}: {error}"));
        }
    }
    if let Err(error) = client.delete_collection(collection).await {
        if !is_not_found(&error) {
            cleanup_errors.push(format!("collection {collection}: {error}"));
        }
    }
    match (operation_result, cleanup_errors.is_empty()) {
        (Err(primary), false) => {
            return Err(format!(
                "{primary}; cleanup also failed: {}",
                cleanup_errors.join("; ")
            )
            .into());
        }
        (Err(primary), true) => return Err(primary),
        (Ok(()), false) => {
            return Err(format!("cleanup failed: {}", cleanup_errors.join("; ")).into())
        }
        (Ok(()), true) => {}
    }

    println!("✅ Cleanup complete\n");
    println!("✅ All CRUD script examples finished!");
    Ok(())
}
