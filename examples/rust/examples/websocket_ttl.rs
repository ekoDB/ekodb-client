use futures::{SinkExt, StreamExt};
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let ws_url = env::var("WS_BASE_URL").unwrap_or_else(|_| "ws://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Get authentication token
    let client = Client::new();
    let auth_response: Value = client
        .post(&format!("{}/api/auth/token", base_url))
        .json(&json!({"api_key": api_key}))
        .send()
        .await?
        .json()
        .await?;

    let token = auth_response["token"].as_str().unwrap();
    println!("✓ Authentication successful");

    // Insert test data with TTL
    println!("\n=== Insert Test Data with TTL ===");
    let doc: Value = client
        .post(&format!("{}/api/insert/ws_ttl_test", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "name": "WebSocket TTL Test",
            "value": 42,
            "ttl": 3600
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("✓ Inserted document with TTL: {:?}", doc["id"]);

    // Connect to WebSocket
    println!("\n=== Query via WebSocket ===");
    let ws_url_full = format!("{}/api/ws", ws_url);

    // Create request with auth header and WebSocket headers
    let request = tokio_tungstenite::tungstenite::http::Request::builder()
        .uri(&ws_url_full)
        .header("Authorization", format!("Bearer {}", token))
        .header("Host", "localhost:8080")
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
    println!("✓ WebSocket connected");

    let (mut write, mut read) = ws_stream.split();

    // Send query
    write
        .send(Message::Text(
            json!({
                "type": "FindAll",
                "messageId": "1",
                "payload": {
                    "collection": "ws_ttl_test"
                }
            })
            .to_string(),
        ))
        .await?;

    // Wait for response
    if let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            if let Ok(text) = msg.into_text() {
                if !text.is_empty() {
                    match serde_json::from_str::<Value>(&text) {
                        Ok(response) => {
                            if let Some(payload) = response.get("payload") {
                                if let Some(data) = payload.get("data") {
                                    if let Some(arr) = data.as_array() {
                                        println!(
                                            "✓ Retrieved {} record(s) via WebSocket",
                                            arr.len()
                                        );
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                        }
                    }
                }
            }
        }
    }

    println!("\n✓ WebSocket TTL example completed successfully");

    Ok(())
}
