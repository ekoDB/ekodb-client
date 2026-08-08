//! ekoDB Rust Client - Chat Message Stream (SSE) Example
//!
//! Demonstrates streaming chat responses via HTTP SSE (Server-Sent Events).
//! This is a simpler alternative to WebSocket streaming.
//!
//! Run with: `cargo run --example client_chat_message_stream`

use ekodb_client::{ChatMessageRequest, Client, CreateChatSessionRequest};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB Chat Message Stream (SSE) Example (Rust) ===\n");

    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    // Create a chat session
    let session = client
        .create_chat_session(
            CreateChatSessionRequest::new("openai").system_prompt("You are a helpful assistant."),
        )
        .await?;
    println!("Created session: {}", session.chat_id);

    // Stream a chat message via SSE
    println!("\nStreaming response for: 'What is ekoDB?'\n");
    let mut rx = client
        .chat_message_stream(&session.chat_id, ChatMessageRequest::new("What is ekoDB?"))
        .await?;

    use ekodb_client::websocket::ChatStreamEvent;
    while let Some(event) = rx.recv().await {
        match event {
            ChatStreamEvent::Chunk(text) => {
                print!("{}", text);
            }
            ChatStreamEvent::End {
                message_id,
                execution_time_ms,
                context_window,
                ..
            } => {
                println!("\n\n--- Stream complete ---");
                println!("Message ID: {}", message_id);
                println!("Execution time: {}ms", execution_time_ms);
                if let Some(cw) = context_window {
                    println!("Context window: {} tokens", cw);
                }
            }
            ChatStreamEvent::Error(err) => {
                eprintln!("\nError: {}", err);
            }
            _ => {}
        }
    }

    println!("\n✓ Chat message stream example completed");
    Ok(())
}
