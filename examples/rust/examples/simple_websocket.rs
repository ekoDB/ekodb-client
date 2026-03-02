use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let ws_url = env::var("WS_BASE_URL").unwrap_or_else(|_| "ws://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Step 1: Get authentication token via HTTP
    let client = reqwest::Client::new();
    let token_response = client
        .post(&format!("{}/api/auth/token", base_url))
        .json(&json!({ "api_key": api_key }))
        .send()
        .await?;

    let token_data: Value = token_response.json().await?;
    let token = token_data["token"].as_str().unwrap();
    println!("✓ Authentication successful");

    // Step 2: Insert some test data first
    println!("\n=== Inserting Test Data ===");
    let insert_response = client
        .post(&format!("{}/api/insert/websocket_test", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "name": "WebSocket Test Record",
            "value": 42,
            "active": true
        }))
        .send()
        .await?;

    let insert_result: Value = insert_response.json().await?;
    println!("✓ Inserted test record: {}", insert_result["id"]);

    // Step 3: Connect to WebSocket with Authorization header
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

    // Create request with auth header
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

    println!("✓ WebSocket connected");

    let (mut write, mut read) = ws_stream.split();

    // Step 4: Query data via WebSocket
    println!("\n=== Querying Data via WebSocket ===");
    let message_id = format!(
        "{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    );

    write
        .send(Message::Text(
            json!({
                "type": "FindAll",
                "messageId": message_id,
                "payload": {
                    "collection": "websocket_test"
                }
            })
            .to_string(),
        ))
        .await?;

    // Wait for response - loop to handle potential ping/pong or empty messages
    let timeout = std::time::Duration::from_secs(5);
    let start = std::time::Instant::now();
    let mut received_data = false;

    while start.elapsed() < timeout && !received_data {
        match tokio::time::timeout(std::time::Duration::from_millis(500), read.next()).await {
            Ok(Some(msg)) => {
                match msg {
                    Ok(msg) => {
                        if let Ok(text) = msg.into_text() {
                            if !text.is_empty() {
                                match serde_json::from_str::<Value>(&text) {
                                    Ok(response) => {
                                        // Check if this is a response with data
                                        if response.get("payload").is_some() {
                                            println!(
                                                "Response: {}",
                                                serde_json::to_string_pretty(&response)?
                                            );

                                            // Show the data we got back
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
                                            received_data = true;
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to parse JSON: {}", e);
                                        eprintln!("Raw message: {}", text);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
            Ok(None) => {
                // Stream ended
                break;
            }
            Err(_) => {
                // Timeout on this read, continue looping
                continue;
            }
        }
    }

    if !received_data {
        eprintln!("Warning: No data response received within timeout");
    }

    println!("\n✓ WebSocket example completed successfully");

    Ok(())
}
