//! CRUD Scripts Example - Basic Data Operations
//!
//! Demonstrates basic CRUD operations using scripts:
//! - FindAll queries
//! - Group aggregations
//! - Simple data transformations

use ekodb_client::{
    Client, FieldType, Function, GroupFunctionConfig, GroupFunctionOp, Record, Script,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        std::env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("🚀 ekoDB Rust CRUD Scripts Example\n");

    // Setup test data
    println!("📋 Setting up test data...");
    let _ = client.delete_collection("crud_users_rs").await;

    for i in 1..=10 {
        let status = if i % 3 == 0 { "inactive" } else { "active" };
        let role = if i <= 3 { "admin" } else { "user" };
        let mut record = Record::new();
        record.insert("name", FieldType::String(format!("User {}", i)));
        record.insert("email", FieldType::String(format!("user{}@example.com", i)));
        record.insert("status", FieldType::String(status.to_string()));
        record.insert("score", FieldType::Integer(i * 10));
        record.insert("role", FieldType::String(role.to_string()));
        client.insert("crud_users_rs", record, None).await?;
    }
    println!("✅ Created 10 test users\n");

    let mut script_ids: Vec<String> = Vec::new();

    // Example 1: List All Users
    println!("📝 Example 1: List All Users\n");
    let script1 = Script::new("list_all_users_rs", "List All Users")
        .with_version("1.0")
        .with_function(Function::FindAll {
            collection: "crud_users_rs".to_string(),
        })
        .with_tag("users")
        .with_tag("list");
    let script_id1 = client.save_script(script1).await?;
    script_ids.push(script_id1);
    println!("✅ Script saved");

    let result1 = client.call_script("list_all_users_rs", None).await?;
    println!("📊 Found {} users", result1.records.len());
    println!(
        "⏱️  Execution time: {}ms\n",
        result1.stats.execution_time_ms
    );

    // Example 2: Count Users by Status
    println!("📝 Example 2: Count Users by Status\n");
    let script2 = Script::new("users_by_status_rs", "Users by Status")
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
    let script_id2 = client.save_script(script2).await?;
    script_ids.push(script_id2);
    println!("✅ Script saved");

    let result2 = client.call_script("users_by_status_rs", None).await?;
    println!("📊 User counts by status:");
    for record in &result2.records {
        println!("   {:?}", record);
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result2.stats.execution_time_ms
    );

    // Cleanup
    println!("🧹 Cleaning up...");
    for script_id in script_ids {
        let _ = client.delete_script(&script_id).await;
    }
    let _ = client.delete_collection("crud_users_rs").await;
    println!("✅ Cleanup complete\n");

    println!("✅ All CRUD script examples finished!");

    Ok(())
}
