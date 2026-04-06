//! AI Functions Example - Chat and Embed Operations
//!
//! Demonstrates AI operations in scripts:
//! - Chat completions with context
//! - Embedding generation
//! - Simple AI workflows

use ekodb_client::{
    ChatMessage, Client, FieldType, Function, ParameterDefinition, Record, UserFunction,
};
use serde_json::json;
use std::collections::HashMap;

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

    // Setup test data
    println!("📋 Setting up test data...");
    let _ = client.delete_collection("ai_articles_rs").await;

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
        client.insert("ai_articles_rs", record, None).await?;
    }
    println!("✅ Created {} articles\n", articles.len());

    let mut script_ids: Vec<String> = Vec::new();

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
    let script_id1 = client.save_function(script1).await?;
    script_ids.push(script_id1.clone());
    println!("✅ Chat script saved");

    let result1 = client.call_function("ai_assistant_rs", None).await?;
    println!("🤖 AI Response:");
    if let Some(first) = result1.records.first() {
        if let Some(FieldType::String(response)) = first.get("response") {
            println!("   {}", response);
        }
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result1.stats.execution_time_ms
    );

    // Example 2: Embed Generation
    println!("📝 Example 2: Generate Embeddings\n");
    let script2 = UserFunction::new("generate_embedding_rs", "Generate Embedding")
        .with_description("Generate embedding for text")
        .with_version("1.0")
        .with_parameter(
            ParameterDefinition::new("text")
                .required()
                .with_description("Text to embed"),
        )
        .with_function(Function::Embed {
            input_field: "text".to_string(),
            output_field: "embedding".to_string(),
            model: None,
        })
        .with_tag("ai")
        .with_tag("embed");
    let script_id2 = client.save_function(script2).await?;
    script_ids.push(script_id2.clone());
    println!("✅ Embed script saved");

    let mut params = HashMap::new();
    params.insert(
        "text".to_string(),
        FieldType::String("ekoDB is a powerful database".to_string()),
    );
    let result2 = client
        .call_function("generate_embedding_rs", Some(params))
        .await?;
    println!("📊 Embedding generated");
    println!(
        "⏱️  Execution time: {}ms\n",
        result2.stats.execution_time_ms
    );

    // Cleanup
    println!("🧹 Cleaning up...");
    for script_id in script_ids {
        let _ = client.delete_function(&script_id).await;
    }
    let _ = client.delete_collection("ai_articles_rs").await;
    println!("✅ Cleanup complete\n");

    println!("✅ All AI script examples finished!");
    println!("\n💡 This example demonstrates:");
    println!("   ✅ Chat completions with system/user messages");
    println!("   ✅ Embedding generation for text");

    Ok(())
}
