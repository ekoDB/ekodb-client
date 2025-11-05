//! Chat Session Management Example
//!
//! This example demonstrates how to create and manage persistent chat sessions.
//! It shows session creation, messaging, branching, and history management.
//!
//! Prerequisites:
//! - Set OPENAI_API_KEY, ANTHROPIC_API_KEY, or PERPLEXITY_API_KEY in .env
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_chat_sessions`

use ekodb_client::{
    ChatMessageRequest, Client, CollectionConfig, CreateChatSessionRequest, GetMessagesQuery,
    ListSessionsQuery, Record, UpdateSessionRequest,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=== ekoDB Chat Session Management Example ===\n");

    // Create client
    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let collection = "client_chat_sessions_rust";

    // Step 1: Insert sample data
    println!("=== Inserting Sample Data ===");
    let mut doc = Record::new();
    doc.insert("product", "ekoDB");
    doc.insert(
        "description",
        "A high-performance database product with AI capabilities",
    );
    doc.insert("price", 99);
    client.insert(collection, doc, None).await?;
    println!("✓ Inserted sample product\n");

    // Step 2: Create a chat session
    println!("=== Creating Chat Session ===");
    let session_request = CreateChatSessionRequest::new("openai")
        .model("gpt-4")
        .system_prompt("You are a helpful assistant for product information.")
        .collection(CollectionConfig {
            collection_name: collection.to_string(),
            fields: vec![],
            search_options: None,
        });

    let session = client.create_chat_session(session_request).await?;
    let chat_id = session.chat_id.clone();

    println!("✓ Created session: {}", chat_id);

    // Step 3: Send messages in the session
    println!("=== Sending Messages ===");

    let msg1 = client
        .chat_message(
            &chat_id,
            ChatMessageRequest::new("What products are available?"),
        )
        .await?;
    println!("✓ Message 1 sent");
    println!(
        "  Response: {}\n",
        msg1.responses.first().unwrap_or(&String::new())
    );

    let msg2 = client
        .chat_message(&chat_id, ChatMessageRequest::new("What is the price?"))
        .await?;
    println!("✓ Message 2 sent");
    println!(
        "  Response: {}\n",
        msg2.responses.first().unwrap_or(&String::new())
    );

    // Step 4: Get session messages
    println!("=== Retrieving Session Messages ===");
    let messages_response = client
        .get_chat_session_messages(&chat_id, GetMessagesQuery::new().limit(10).sort("asc"))
        .await?;
    println!("✓ Retrieved {} messages", messages_response.messages.len());
    for (i, msg) in messages_response.messages.iter().enumerate() {
        if let Some(ekodb_client::FieldType::String(role)) = msg.get("role") {
            if let Some(ekodb_client::FieldType::String(content)) = msg.get("content") {
                println!(
                    "  Message {}: [{}] {}",
                    i + 1,
                    role,
                    if content.len() > 60 {
                        format!("{}...", &content[..60])
                    } else {
                        content.to_string()
                    }
                );
            }
        }
    }
    println!();

    // Step 5: Update session metadata
    println!("=== Updating Session ===");
    client
        .update_chat_session(
            &chat_id,
            UpdateSessionRequest::new().system_prompt("You are an expert product consultant."),
        )
        .await?;
    println!("✓ Session updated\n");

    // Step 6: Branch the session (branch at message index 0)
    println!("=== Branching Session ===");
    let branch_request = CreateChatSessionRequest::new("openai")
        .model("gpt-4")
        .branch_from(chat_id.to_string(), 0);

    let branched = client.branch_chat_session(branch_request).await?;
    let branch_id = branched.chat_id.clone();

    println!("✓ Created branch: {}", branch_id);
    println!("  Parent: {}\n", chat_id);

    // Step 7: List all sessions
    println!("=== Listing Sessions ===");
    let sessions = client
        .list_chat_sessions(ListSessionsQuery::new().limit(10).sort("desc"))
        .await?;
    println!("✓ Found {} sessions", sessions.total);
    for (i, sess) in sessions.sessions.iter().take(3).enumerate() {
        println!(
            "  Session {}: {} ({})",
            i + 1,
            sess.chat_id,
            sess.title.as_ref().unwrap_or(&"Untitled".to_string())
        );
    }
    println!();

    // Step 8: Get specific session
    println!("=== Getting Session Details ===");
    let session_details = client.get_chat_session(&chat_id).await?;
    println!("✓ Session details retrieved");
    println!("  Messages: {}\n", session_details.message_count);

    // Step 9: Delete the branched session
    println!("=== Deleting Branch Session ===");
    client.delete_chat_session(&branch_id).await?;
    println!("✓ Deleted branch session: {}\n", branch_id);

    // Cleanup: Delete the collection (chat sessions are managed by server)
    println!("=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection\n");

    println!("✓ All session management operations completed successfully");

    Ok(())
}
