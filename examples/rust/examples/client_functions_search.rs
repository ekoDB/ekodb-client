//! Search Functions Example - Basic Search Operations
//!
//! Demonstrates simple search and query operations using user functions

use ekodb_client::{
    extract_record, get_string_value, Client, Error, FieldType, Function, GroupFunctionConfig,
    GroupFunctionOp, Record, UserFunction,
};
use serde_json::json;
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
    println!("🚀 ekoDB Rust Search Functions Example\n");

    // Setup test data
    println!("📋 Setting up test data...");
    let documents = vec![
        json!({"title": "Introduction to Machine Learning", "content": "Machine learning is a subset of AI.", "category": "AI", "tags": ["ml", "ai"]}),
        json!({"title": "Database Design Principles", "content": "Good database design involves normalization.", "category": "Database", "tags": ["database", "design"]}),
        json!({"title": "Vector Databases Explained", "content": "Vector databases store data based on similarity.", "category": "Database", "tags": ["vector", "database"]}),
        json!({"title": "Natural Language Processing", "content": "NLP enables computers to understand language.", "category": "AI", "tags": ["nlp", "ai"]}),
        json!({"title": "Getting Started with ekoDB", "content": "ekoDB is a high-performance database.", "category": "Database", "tags": ["ekodb", "tutorial"]}),
    ];

    let mut inserted_ids = Vec::new();
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
        let inserted = client.insert("search_docs_rs", record, None).await?;
        let id = inserted
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("document insert did not return an ID")?;
        if inserted_ids.iter().any(|existing| existing == id) {
            return Err(format!("duplicate inserted document ID: {id}").into());
        }
        inserted_ids.push(id.to_string());
    }
    println!("✅ Inserted {} documents\n", documents.len());

    // Example 1: List All Documents
    println!("📝 Example 1: List All Documents\n");
    let script1 = UserFunction::new("list_all_docs_rs", "List All Documents")
        .with_version("1.0")
        .with_function(Function::FindAll {
            collection: "search_docs_rs".to_string(),
        })
        .with_tag("search")
        .with_tag("list");
    let script_id1 = save_or_update(client, script1).await?;
    script_ids.push(script_id1);
    println!("✅ Function saved");

    let result1 = client.call_function("list_all_docs_rs", None).await?;
    println!("📊 Found {} documents", result1.records.len());
    if result1.records.len() != 5 {
        return Err(format!("expected 5 documents, found {}", result1.records.len()).into());
    }
    let mut returned_titles = std::collections::HashSet::new();
    for (i, record) in result1.records.iter().enumerate() {
        // Convert Record to JSON and extract values
        let record_json = serde_json::to_value(record)?;
        let extracted = extract_record(&record_json);

        let title = get_string_value(&extracted["title"])
            .ok_or("search result did not include a string title")?;
        let category = get_string_value(&extracted["category"])
            .ok_or("search result did not include a string category")?;
        if category != "AI" && category != "Database" {
            return Err(format!("unexpected document category: {category}").into());
        }
        returned_titles.insert(title.clone());

        println!("   {}. {} ({})", i + 1, title, category);
    }
    let expected_titles: std::collections::HashSet<String> = documents
        .iter()
        .map(|doc| doc["title"].as_str().unwrap().to_string())
        .collect();
    if returned_titles != expected_titles {
        return Err(format!("unexpected document titles: {returned_titles:?}").into());
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result1.stats.execution_time_ms
    );

    // Example 2: Count Documents by Category
    println!("📝 Example 2: Count Documents by Category\n");
    let script2 = UserFunction::new("docs_by_category_rs", "Documents by Category")
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
    let script_id2 = save_or_update(client, script2).await?;
    script_ids.push(script_id2);
    println!("✅ Function saved");

    let result2 = client.call_function("docs_by_category_rs", None).await?;
    println!("📊 Documents by category:");
    let mut ai = None;
    let mut database = None;
    for record in &result2.records {
        println!("   {:?}", record);
        match record.get_string("category") {
            Some("AI") => ai = Some(integer(record, "count")?),
            Some("Database") => database = Some(integer(record, "count")?),
            value => return Err(format!("unexpected category group: {value:?}").into()),
        }
    }
    if ai != Some(2) || database != Some(3) || result2.records.len() != 2 {
        return Err(format!("unexpected category counts: AI={ai:?}, Database={database:?}").into());
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
    let collection = "search_docs_rs";
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
            .into())
        }
        (Err(primary), true) => return Err(primary),
        (Ok(()), false) => {
            return Err(format!("cleanup failed: {}", cleanup_errors.join("; ")).into())
        }
        (Ok(()), true) => {}
    }
    println!("✅ Cleanup complete\n");
    println!("✅ All search script examples finished!");
    Ok(())
}
