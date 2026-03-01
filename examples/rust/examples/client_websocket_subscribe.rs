//! WebSocket Subscription Example - Real-time mutation notifications
//!
//! Demonstrates subscribing to collection changes via WebSocket and
//! receiving push notifications when records are inserted, updated, or deleted.
//!
//! Requires: ekoDB server running on localhost:8080

use futures_util::{SinkExt, StreamExt};
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

    let collection = "ws_subscribe_example_rs";

    // Step 1: Authenticate
    let http_client = reqwest::Client::new();
    let token_response = http_client
        .post(&format!("{}/api/auth/token", base_url))
        .json(&json!({ "api_key": api_key }))
        .send()
        .await?;

    let token_data: Value = token_response.json().await?;
    let token = token_data["token"].as_str().unwrap().to_string();
    println!("✓ Authentication successful");

    // Step 2: Connect to WebSocket
    println!("\n=== Connecting to WebSocket ===");
    let url = format!("{}/api/ws", ws_url);

    let request = tokio_tungstenite::tungstenite::http::Request::builder()
        .uri(&url)
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
    let (mut write, mut read) = ws_stream.split();
    println!("✓ WebSocket connected");

    // Step 3: Subscribe to collection
    println!("\n=== Subscribing to '{}' ===", collection);

    let subscribe_msg = json!({
        "type": "Subscribe",
        "payload": { "collection": collection }
    });
    write
        .send(Message::Text(subscribe_msg.to_string().into()))
        .await?;

    // Read subscription confirmation
    if let Some(Ok(Message::Text(text))) = read.next().await {
        let response: Value = serde_json::from_str(&text)?;
        if let Some(sub_id) = response["payload"]["data"]["subscription_id"].as_str() {
            println!("✓ Subscribed (subscription_id: {})", sub_id);
        }
    }

    // Step 4: Spawn a task to listen for notifications
    let (notify_tx, mut notify_rx) = tokio::sync::mpsc::channel::<Value>(10);

    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                    if parsed["type"] == "MutationNotification" {
                        let _ = notify_tx.send(parsed).await;
                    }
                }
            }
        }
    });

    // Step 5: Insert records via REST API to trigger notifications
    println!("\n=== Performing mutations to trigger notifications ===");

    println!("Inserting record 1...");
    let insert_res = http_client
        .post(&format!("{}/api/insert/{}", base_url, collection))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "name": "Alice",
            "role": "engineer",
            "active": true
        }))
        .send()
        .await?;
    let insert_data: Value = insert_res.json().await?;
    println!("✓ Inserted: {}", insert_data["id"]);

    // Wait for notification
    match tokio::time::timeout(std::time::Duration::from_secs(5), notify_rx.recv()).await {
        Ok(Some(notification)) => {
            let payload = &notification["payload"];
            println!("\n  📡 Notification received:");
            println!("     Event:      {}", payload["event"]);
            println!("     Collection: {}", payload["collection"]);
            println!("     Record IDs: {}", payload["record_ids"]);
            println!("     Timestamp:  {}", payload["timestamp"]);
        }
        _ => println!("  ⏳ No notification within timeout"),
    }

    println!("\nInserting record 2...");
    let insert_res2 = http_client
        .post(&format!("{}/api/insert/{}", base_url, collection))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "name": "Bob",
            "role": "designer",
            "active": true
        }))
        .send()
        .await?;
    let insert_data2: Value = insert_res2.json().await?;
    println!("✓ Inserted: {}", insert_data2["id"]);

    match tokio::time::timeout(std::time::Duration::from_secs(5), notify_rx.recv()).await {
        Ok(Some(notification)) => {
            let payload = &notification["payload"];
            println!("\n  📡 Notification received:");
            println!("     Event:      {}", payload["event"]);
            println!("     Record IDs: {}", payload["record_ids"]);
        }
        _ => println!("  ⏳ No notification within timeout"),
    }

    // Step 6: Unsubscribe
    println!("\n=== Unsubscribing ===");
    let unsub_msg = json!({
        "type": "Unsubscribe",
        "payload": { "collection": collection }
    });
    write
        .send(Message::Text(unsub_msg.to_string().into()))
        .await?;
    println!("✓ Unsubscribed");

    // Cleanup
    read_task.abort();
    println!("\n✓ WebSocket subscription example completed successfully");

    Ok(())
}
