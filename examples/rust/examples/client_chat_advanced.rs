// ekoDB Rust Client - Advanced Chat Features Example
//
// This example demonstrates advanced chat features:
// - Message regeneration
// - Message editing
// - Message deletion
// - Session merging
// - Forgotten messages (exclude from context)

use ekodb_client::{
    Client, MergeSessionsRequest, MergeStrategy, ToggleForgottenRequest, UpdateMessageRequest,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ekoDB Advanced Chat Features Example ===\n");

    // Load environment variables
    dotenv::dotenv().ok();

    // Create client
    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key = env::var("API_BASE_KEY").expect("API_BASE_KEY must be set");

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    let collection = "client_chat_advanced_rust";

    // Insert sample data
    println!("=== Inserting Sample Data ===");
    let mut record = ekodb_client::Record::new();
    record.insert("name", "ekoDB");
    record.insert("description", "High-performance database product");
    record.insert("price", 99);
    client.insert(collection, record).await?;
    println!("✓ Inserted sample product\n");

    // Create a chat session
    println!("=== Creating Chat Session ===");
    let session = client
        .create_chat_session(
            ekodb_client::CreateChatSessionRequest::new("openai")
                .model("gpt-4")
                .collection(ekodb_client::CollectionConfig {
                    collection_name: collection.to_string(),
                    fields: vec![],
                    search_options: None,
                })
                .system_prompt("You are a helpful product assistant."),
        )
        .await?;
    let chat_id = session.chat_id.clone();
    println!("✓ Created session: {}\n", chat_id);

    // Send initial message
    println!("=== Sending Initial Message ===");
    let msg1 = client
        .chat_message(
            &chat_id,
            ekodb_client::ChatMessageRequest::new("What products are available?"),
        )
        .await?;
    println!("✓ Message sent");
    if !msg1.responses.is_empty() {
        println!("  Response: {}\n", msg1.responses[0]);
    }

    // Send a second message to have more context
    client
        .chat_message(
            &chat_id,
            ekodb_client::ChatMessageRequest::new("What is the price of ekoDB?"),
        )
        .await?;
    println!("✓ Second message sent");

    // Get the messages to find the user and assistant message IDs
    let messages = client
        .get_chat_session_messages(
            &chat_id,
            ekodb_client::GetMessagesQuery {
                limit: Some(10),
                skip: None,
                sort: Some("desc".to_string()), // Get newest first
            },
        )
        .await?;

    // Find the most recent assistant and user messages
    let assistant_message_id = messages
        .messages
        .iter()
        .find(|msg| {
            // Role is wrapped in an Object with type/value structure
            if let Some(ekodb_client::FieldType::Object(role_obj)) = msg.get("role") {
                if let Some(ekodb_client::FieldType::String(role)) = role_obj.get("value") {
                    return role == "assistant";
                }
            }
            false
        })
        .and_then(|msg| {
            // ID is a direct String field
            if let Some(ekodb_client::FieldType::String(id)) = msg.get("id") {
                Some(id.clone())
            } else {
                None
            }
        })
        .expect("Could not find assistant message");

    let user_message_id = messages
        .messages
        .iter()
        .find(|msg| {
            if let Some(ekodb_client::FieldType::Object(role_obj)) = msg.get("role") {
                if let Some(ekodb_client::FieldType::String(role)) = role_obj.get("value") {
                    return role == "user";
                }
            }
            false
        })
        .and_then(|msg| {
            if let Some(ekodb_client::FieldType::String(id)) = msg.get("id") {
                Some(id.clone())
            } else {
                None
            }
        })
        .expect("Could not find user message");

    // Feature 1: Regenerate Message
    println!("=== Feature 1: Regenerate AI Response ===");
    let regenerated = client
        .regenerate_chat_message(&chat_id, &assistant_message_id)
        .await?;
    println!("✓ Message regenerated");
    if !regenerated.responses.is_empty() {
        println!("  New response: {}\n", regenerated.responses[0]);
    }

    // Feature 2: Update Message Content
    println!("=== Feature 2: Edit Message ===");
    client
        .update_chat_message(
            &chat_id,
            &user_message_id,
            UpdateMessageRequest {
                content: "What is the price of ekoDB?".to_string(),
            },
        )
        .await?;
    println!("✓ Message content updated\n");

    // Feature 3: Toggle Forgotten Status
    println!("=== Feature 3: Mark Message as Forgotten ===");
    client
        .toggle_forgotten_message(
            &chat_id,
            &user_message_id,
            ToggleForgottenRequest { forgotten: true },
        )
        .await?;
    println!("✓ Message marked as forgotten (excluded from LLM context)\n");

    // Unmark as forgotten
    client
        .toggle_forgotten_message(
            &chat_id,
            &user_message_id,
            ToggleForgottenRequest { forgotten: false },
        )
        .await?;
    println!("✓ Message unmarked as forgotten\n");

    // Feature 4: Session Merging
    println!("=== Feature 4: Merge Chat Sessions ===");

    // Create a second session
    let session2 = client
        .create_chat_session(
            ekodb_client::CreateChatSessionRequest::new("openai")
                .model("gpt-4")
                .collection(ekodb_client::CollectionConfig {
                    collection_name: "products".to_string(),
                    fields: vec![],
                    search_options: None,
                }),
        )
        .await?;
    let chat_id2 = session2.chat_id.clone();
    println!("✓ Created second session: {}", chat_id2);

    // Send a message in the second session
    client
        .chat_message(
            &chat_id2,
            ekodb_client::ChatMessageRequest::new("Tell me more about the features"),
        )
        .await?;
    println!("✓ Sent message in second session");

    // Merge sessions
    let merge_result = client
        .merge_chat_sessions(MergeSessionsRequest {
            source_chat_ids: vec![chat_id2.clone()],
            target_chat_id: chat_id.clone(),
            merge_strategy: MergeStrategy::Chronological,
        })
        .await?;
    println!("✓ Sessions merged successfully");
    println!(
        "  Total messages in merged session: {}\n",
        merge_result.message_count
    );

    // Feature 5: Delete Message
    println!("=== Feature 5: Delete Message ===");
    client
        .delete_chat_message(&chat_id, &user_message_id)
        .await?;
    println!("✓ Message deleted\n");

    // Verify message count after deletion
    let session_details = client.get_chat_session(&chat_id).await?;
    println!("✓ Messages remaining: {}\n", session_details.message_count);

    // Cleanup
    println!("=== Cleanup ===");
    client.delete_chat_session(&chat_id).await?;
    println!("✓ Deleted session");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection\n");

    println!("✓ All advanced chat features demonstrated successfully!");

    Ok(())
}
