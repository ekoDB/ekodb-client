//! AI Functions Example - Chat and Embed Operations
//!
//! Demonstrates AI operations in scripts:
//! - Chat completions with context
//! - Embedding generation
//! - Simple AI workflows

use ekodb_client::{ChatMessage, Client, FieldType, Function, Record, UserFunction};
use serde_json::json;

const COLLECTION: &str = "ai_articles_rs";

fn vector_len(field: Option<&FieldType>) -> Option<usize> {
    match field {
        Some(FieldType::Vector(values)) | Some(FieldType::Array(values)) => Some(values.len()),
        Some(FieldType::Object(wrapper)) => vector_len(wrapper.get("value")),
        _ => None,
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

    println!("🚀 ekoDB Rust AI Functions Example\n");
    let mut script_ids: Vec<String> = Vec::new();
    let operation_result = async {

    // Setup test data
    println!("📋 Setting up test data...");
    let _ = client.delete_collection(COLLECTION).await;

    let articles = vec![
        json!({"title": "Getting Started with ekoDB", "content": "ekoDB is a high-performance database...", "status": "published"}),
        json!({"title": "Advanced Query Patterns", "content": "Learn how to build complex queries...", "status": "published"}),
    ];

    for article in &articles {
        let mut record = Record::new();
        record.insert(
            "title",
            FieldType::String(article["title"].as_str().unwrap().to_string()),
        );
        record.insert(
            "content",
            FieldType::String(article["content"].as_str().unwrap().to_string()),
        );
        record.insert(
            "status",
            FieldType::String(article["status"].as_str().unwrap().to_string()),
        );
        client.insert(COLLECTION, record, None).await?;
    }
    println!("✅ Created {} articles\n", articles.len());

    // Example 1: Simple Chat Completion
    println!("📝 Example 1: Simple Chat Completion\n");
    let script1 = UserFunction::new("ai_assistant_rs", "AI Chat Assistant")
        .with_description("Simple AI chat completion")
        .with_version("1.0")
        .with_function(Function::Chat {
            messages: vec![
                ChatMessage::system("You are a helpful database assistant. Be concise."),
                ChatMessage::user("What are the benefits of using vector databases?"),
            ],
            model: Some("gpt-4o-mini".to_string()),
            temperature: Some(0.7),
            max_tokens: None,
        })
        .with_tag("ai")
        .with_tag("chat");
    let script_id1 = save_or_update(&client, script1).await?;
    script_ids.push(script_id1.clone());
    println!("✅ Chat script saved");

    let result1 = client.call_function("ai_assistant_rs", None).await?;
    println!("🤖 AI Response:");
    let response = result1
        .records
        .first()
        .and_then(|record| record.get_string("response"))
        .filter(|response| !response.trim().is_empty())
        .ok_or("chat function returned no response text")?;
    println!("   {response}");
    println!(
        "⏱️  Execution time: {}ms\n",
        result1.stats.execution_time_ms
    );

    // Example 2: Embed Generation
    println!("📝 Example 2: Generate Embeddings\n");
    let script2 = UserFunction::new("generate_embedding_rs", "Generate Embedding")
        .with_description("Generate embedding for text")
        .with_version("1.0")
        .with_function(Function::FindAll {
            collection: COLLECTION.to_string(),
        })
        .with_function(Function::Embed {
            input_field: "content".to_string(),
            output_field: "embedding".to_string(),
            model: None,
        })
        .with_tag("ai")
        .with_tag("embed");
    let script_id2 = save_or_update(&client, script2).await?;
    script_ids.push(script_id2.clone());
    println!("✅ Embed script saved");

    let result2 = client.call_function("generate_embedding_rs", None).await?;
    if result2.records.len() != articles.len() {
        return Err(format!(
            "embedding function returned {} records; expected {}",
            result2.records.len(),
            articles.len()
        )
        .into());
    }
    let dimensions = result2
        .records
        .iter()
        .map(|record| vector_len(record.get("embedding")))
        .collect::<Option<Vec<_>>>()
        .ok_or("embedding function returned a record without an embedding vector")?;
    if dimensions.iter().any(|dimensions| *dimensions == 0) {
        return Err("embedding function returned an empty vector".into());
    }
    println!("📊 Generated {} embeddings", dimensions.len());
    println!("   Dimensions: {}", dimensions[0]);
    println!(
        "⏱️  Execution time: {}ms\n",
        result2.stats.execution_time_ms
    );

    Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;

    println!("🧹 Cleaning up...");
    let mut cleanup_errors = Vec::new();
    for script_id in script_ids.iter().rev() {
        if let Err(error) = client.delete_function(script_id).await {
            cleanup_errors.push(format!("function {script_id}: {error}"));
        }
    }
    if let Err(error) = client.delete_collection(COLLECTION).await {
        cleanup_errors.push(format!("collection {COLLECTION}: {error}"));
    }
    if let Err(operation_error) = operation_result {
        if !cleanup_errors.is_empty() {
            eprintln!("⚠️  Cleanup errors: {}", cleanup_errors.join("; "));
        }
        return Err(operation_error);
    }
    if !cleanup_errors.is_empty() {
        return Err(format!("cleanup failed: {}", cleanup_errors.join("; ")).into());
    }
    println!("✅ Cleanup complete\n");

    println!("✅ All AI script examples finished!");
    println!("\n💡 This example demonstrates:");
    println!("   ✅ Chat completions with system/user messages");
    println!("   ✅ Embedding generation for text");

    Ok(())
}
