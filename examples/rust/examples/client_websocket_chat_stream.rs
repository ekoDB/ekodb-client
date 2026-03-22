//! WebSocket Chat Streaming Example - Real-time LLM responses
//!
//! Demonstrates using the WebSocket client to send a chat message
//! and receive streaming responses, including tool calls.
//!
//! Requires: ekoDB server running on localhost:8080
//!
//! Run with: `cargo run --example client_websocket_chat_stream`

use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    println!("=== WebSocket Chat Streaming Example (Rust) ===\n");

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let ws_url = env::var("WS_BASE_URL").unwrap_or_else(|_| "ws://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let http_client = reqwest::Client::new();

    // Step 1: Authenticate
    let token_response = http_client
        .post(&format!("{}/api/auth/token", base_url))
        .json(&json!({ "api_key": api_key }))
        .send()
        .await?;
    let token_data: Value = token_response.json().await?;
    let token = token_data["token"].as_str().unwrap().to_string();
    println!("✓ Authentication successful");

    // Step 2: Create a chat session
    let session_res = http_client
        .post(&format!("{}/api/chat/sessions", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "system_prompt": "You are a helpful assistant.",
        }))
        .send()
        .await?;
    let session_data: Value = session_res.json().await?;
    let chat_id = session_data["chat_id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    println!("✓ Created chat session: {}", chat_id);

    // Step 3: Connect to WebSocket
    println!("\n=== Connecting to WebSocket ===");
    let url = format!("{}/api/ws", ws_url);
    let parsed = Url::parse(&url)?;
    let host = parsed
        .host_str()
        .map(|h| match parsed.port() {
            Some(p) => format!("{}:{}", h, p),
            None => h.to_string(),
        })
        .unwrap_or_else(|| "localhost:8080".to_string());

    let request = tokio_tungstenite::tungstenite::http::Request::builder()
        .uri(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Host", &host)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header(
            "Sec-WebSocket-Key",
            tokio_tungstenite::tungstenite::handshake::client::generate_key(),
        )
        .body(())
        .unwrap();

    let (ws_stream, _) = connect_async(request).await?;
    let (mut write, mut read) = ws_stream.split();
    println!("✓ WebSocket connected");

    // Step 4: Send chat message via WebSocket
    println!("\n=== Sending message: 'What is the capital of France?' ===");
    let chat_msg = json!({
        "type": "ChatSend",
        "payload": {
            "chat_id": chat_id,
            "message": "What is the capital of France?"
        }
    });
    write
        .send(Message::Text(chat_msg.to_string().into()))
        .await?;

    // Step 5: Stream the response
    let mut full_response = String::new();
    let mut got_ack = false;

    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            let parsed: Value = serde_json::from_str(&text)?;
            let msg_type = parsed["type"].as_str().unwrap_or("");

            match msg_type {
                // Initial acknowledgement
                "Success" if !got_ack => {
                    got_ack = true;
                    println!("✓ Message accepted, streaming...\n");
                }

                // Streaming text chunks
                "ChatStreamChunk" => {
                    let content = parsed["payload"]["content"].as_str().unwrap_or("");
                    full_response.push_str(content);
                    print!("{}", content);
                }

                // Tool call requested
                "ClientToolCall" => {
                    let tool = parsed["payload"]["tool_name"].as_str().unwrap_or("?");
                    let call_id = parsed["payload"]["call_id"].as_str().unwrap_or("");
                    let args = &parsed["payload"]["arguments"];
                    println!("\n[Tool Call] {}({})", tool, args);

                    // Send tool result back
                    let result_msg = json!({
                        "type": "ClientToolResult",
                        "payload": {
                            "chat_id": chat_id,
                            "call_id": call_id,
                            "success": true,
                            "result": {"result": "Tool executed successfully"},
                        }
                    });
                    write
                        .send(Message::Text(result_msg.to_string().into()))
                        .await?;
                }

                // Stream completed
                "ChatStreamEnd" => {
                    let payload = &parsed["payload"];
                    println!("\n\n--- Stream ended ---");
                    println!("Message ID: {}", payload["message_id"]);
                    println!(
                        "Execution time: {}ms",
                        payload["execution_time_ms"].as_u64().unwrap_or(0)
                    );
                    if let Some(usage) = payload.get("token_usage") {
                        println!("Token usage: {}", usage);
                    }
                    if let Some(cw) = payload["context_window"].as_u64() {
                        println!("Context window: {} tokens", cw);
                    }
                    break;
                }

                // Error
                "ChatStreamError" => {
                    let error = parsed["payload"]["error"].as_str().unwrap_or("unknown");
                    println!("\n[Error] {}", error);
                    break;
                }

                _ => {}
            }
        }
    }

    // Truncate for display
    if full_response.len() > 200 {
        full_response.truncate(200);
    }
    println!("\nFull response: {}...", full_response);

    println!("\n✓ WebSocket chat streaming example completed successfully");

    Ok(())
}
