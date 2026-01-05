//! Search Scripts Example - Basic Search Operations
//!
//! Demonstrates simple search and query operations using scripts

use ekodb_client::{
    Client, FieldType, Function, GroupFunctionConfig, GroupFunctionOp, Record, Script,
};
use serde_json::json;

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

    println!("🚀 ekoDB Rust Search Scripts Example\n");

    // Setup test data
    println!("📋 Setting up test data...");
    let _ = client.delete_collection("search_docs_rs").await;

    let documents = vec![
        json!({"title": "Introduction to Machine Learning", "content": "Machine learning is a subset of AI.", "category": "AI", "tags": ["ml", "ai"]}),
        json!({"title": "Database Design Principles", "content": "Good database design involves normalization.", "category": "Database", "tags": ["database", "design"]}),
        json!({"title": "Vector Databases Explained", "content": "Vector databases store data based on similarity.", "category": "Database", "tags": ["vector", "database"]}),
        json!({"title": "Natural Language Processing", "content": "NLP enables computers to understand language.", "category": "AI", "tags": ["nlp", "ai"]}),
        json!({"title": "Getting Started with ekoDB", "content": "ekoDB is a high-performance database.", "category": "Database", "tags": ["ekodb", "tutorial"]}),
    ];

    for doc in &documents {
        let mut record = Record::new();
        record.insert(
            "title",
            FieldType::String(doc["title"].as_str().unwrap().to_string()),
        );
        record.insert(
            "content",
            FieldType::String(doc["content"].as_str().unwrap().to_string()),
        );
        record.insert(
            "category",
            FieldType::String(doc["category"].as_str().unwrap().to_string()),
        );
        if let Some(tags) = doc["tags"].as_array() {
            let tag_strings: Vec<String> = tags
                .iter()
                .filter_map(|t| t.as_str().map(|s| s.to_string()))
                .collect();
            record.insert(
                "tags",
                FieldType::Array(tag_strings.into_iter().map(FieldType::String).collect()),
            );
        }
        client.insert("search_docs_rs", record, None).await?;
    }
    println!("✅ Inserted {} documents\n", documents.len());

    let mut script_ids: Vec<String> = Vec::new();

    // Example 1: List All Documents
    println!("📝 Example 1: List All Documents\n");
    let script1 = Script::new("list_all_docs_rs", "List All Documents")
        .with_version("1.0")
        .with_function(Function::FindAll {
            collection: "search_docs_rs".to_string(),
        })
        .with_tag("search")
        .with_tag("list");
    let script_id1 = client.save_script(script1).await?;
    script_ids.push(script_id1);
    println!("✅ Script saved");

    let result1 = client.call_script("list_all_docs_rs", None).await?;
    println!("📊 Found {} documents", result1.records.len());
    for (i, record) in result1.records.iter().enumerate() {
        let title = match record.fields.get("title") {
            Some(FieldType::String(s)) => s.as_str(),
            _ => "?",
        };
        let category = match record.fields.get("category") {
            Some(FieldType::String(s)) => s.as_str(),
            _ => "?",
        };
        println!("   {}. {} ({})", i + 1, title, category);
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result1.stats.execution_time_ms
    );

    // Example 2: Count Documents by Category
    println!("📝 Example 2: Count Documents by Category\n");
    let script2 = Script::new("docs_by_category_rs", "Documents by Category")
        .with_version("1.0")
        .with_function(Function::FindAll {
            collection: "search_docs_rs".to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["category".to_string()],
            functions: vec![GroupFunctionConfig::new("count", GroupFunctionOp::Count)],
        })
        .with_tag("search")
        .with_tag("analytics");
    let script_id2 = client.save_script(script2).await?;
    script_ids.push(script_id2);
    println!("✅ Script saved");

    let result2 = client.call_script("docs_by_category_rs", None).await?;
    println!("📊 Documents by category:");
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
    let _ = client.delete_collection("search_docs_rs").await;
    println!("✅ Cleanup complete\n");

    println!("✅ All search script examples finished!");

    Ok(())
}
