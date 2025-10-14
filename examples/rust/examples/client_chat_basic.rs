//! Basic Chat Example
//!
//! This example demonstrates basic chat functionality with ekoDB's AI integration.
//! It shows how to send a simple chat message with context search.
//!
//! Prerequisites:
//! - Set OPENAI_API_KEY, ANTHROPIC_API_KEY, or PERPLEXITY_API_KEY in .env
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_chat_basic`

use ekodb_client::{
    ChatMessageRequest, Client, CollectionConfig, CreateChatSessionRequest, Record,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=== ekoDB Chat Basic Example ===\n");

    // Create client
    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let collection = "client_chat_basic_rust";

    // Step 1: Insert some sample data for context
    println!("=== Inserting Sample Data ===");
    let mut doc1 = Record::new();
    doc1.insert("title", "Introduction to ekoDB");
    doc1.insert("content", "ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration.");
    doc1.insert("category", "documentation");
    client.insert(collection, doc1).await?;

    let mut doc2 = Record::new();
    doc2.insert("title", "Search Features");
    doc2.insert("content", "ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.");
    doc2.insert("category", "features");
    client.insert(collection, doc2).await?;

    let mut doc3 = Record::new();
    doc3.insert("title", "AI Chat Integration");
    doc3.insert("content", "The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context.");
    doc3.insert("category", "features");
    client.insert(collection, doc3).await?;

    println!("✓ Inserted 3 sample documents\n");

    // Step 2: Create a chat session
    println!("=== Creating Chat Session ===");
    let session_request = CreateChatSessionRequest {
        collections: vec![CollectionConfig {
            collection_name: collection.to_string(),
            fields: vec![],
            search_options: None,
        }],
        llm_provider: "openai".to_string(),
        llm_model: Some("gpt-4".to_string()),
        system_prompt: Some("You are a helpful assistant for ekoDB documentation.".to_string()),
        parent_id: None,
        branch_point_idx: None,
        max_context_messages: Some(10),
        bypass_ripple: Some(false),
    };

    let session = client.create_chat_session(session_request).await?;
    let chat_id = &session.chat_id;
    println!("✓ Created session: {}\n", chat_id);

    // Step 3: Send a chat message
    println!("=== Sending Chat Message ===");
    let message_request = ChatMessageRequest::new("What is ekoDB and what features does it have?");
    let response = client.chat_message(chat_id, message_request).await?;

    println!("Message ID: {}", response.message_id);
    println!("\n=== AI Response ===");
    for (i, resp) in response.responses.iter().enumerate() {
        println!("Response {}: {}", i + 1, resp);
    }

    println!(
        "\n=== Context Used ({} snippets) ===",
        response.context_snippets.len()
    );
    for (i, snippet) in response.context_snippets.iter().enumerate() {
        println!("\nSnippet {}:", i + 1);
        println!("  Collection: {}", snippet.collection);
        println!("  Score: {:.4}", snippet.score);
        println!("  Matched Fields: {:?}", snippet.matched_fields);
        println!("  Record: {:?}", snippet.record);
    }

    println!("\nExecution Time: {}ms", response.execution_time_ms);

    // Cleanup: Delete the collection (chat session is managed by server)
    println!("\n=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection");

    println!("\n✓ Chat completed successfully");

    Ok(())
}
