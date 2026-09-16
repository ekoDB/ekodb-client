//! WebSocket Subscription Example - Real-time mutation notifications
//!
//! Demonstrates subscribing to collection changes via WebSocket and
//! receiving push notifications when records are inserted, updated, or deleted.
//!
//! Requires: ekoDB server running on localhost:8080

use futures_util::{SinkExt, StreamExt};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::{env, error::Error, time::Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

async fn delete_collection(
    client: &reqwest::Client,
    base_url: &str,
    token: &str,
    collection: &str,
) -> Result<bool, Box<dyn Error>> {
    let response = client
        .delete(format!("{base_url}/api/collections/{collection}"))
        .bearer_auth(token)
        .send()
        .await?;
    if response.status() == StatusCode::NOT_FOUND {
        return Ok(false);
    }
    response.error_for_status()?;
    Ok(true)
}

async fn next_json<S>(socket: &mut S, timeout: Duration) -> Result<Value, Box<dyn Error>>
where
    S: futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
        + futures_util::Sink<Message, Error = tokio_tungstenite::tungstenite::Error>
        + Unpin,
{
    tokio::time::timeout(timeout, async {
        loop {
            let message = socket
                .next()
                .await
                .ok_or("WebSocket closed before the expected message")??;
            match message {
                Message::Text(text) => return Ok(serde_json::from_str(&text)?),
                Message::Ping(payload) => socket.send(Message::Pong(payload)).await?,
                Message::Pong(_) | Message::Frame(_) => {}
                Message::Close(frame) => {
                    return Err(
                        format!("WebSocket closed before the expected message: {frame:?}").into(),
                    );
                }
                Message::Binary(_) => {
                    return Err("expected a text WebSocket message, got binary data".into());
                }
            }
        }
    })
    .await
    .map_err(|_| "timed out waiting for WebSocket message")?
}

fn required_id<'a>(response: &'a Value, operation: &str) -> Result<&'a str, Box<dyn Error>> {
    response["id"]
        .as_str()
        .filter(|id| !id.is_empty())
        .ok_or_else(|| format!("{operation} did not return a non-empty record ID").into())
}

fn validate_notification<'a>(
    notification: &'a Value,
    collection: &str,
    record_id: &str,
) -> Result<&'a Value, Box<dyn Error>> {
    if notification["type"] != "MutationNotification" {
        return Err(format!("unexpected WebSocket message: {notification}").into());
    }
    let payload = notification["payload"]
        .as_object()
        .ok_or("mutation notification did not include an object payload")?;
    if payload.get("event").and_then(Value::as_str) != Some("insert") {
        return Err(format!("expected insert notification: {notification}").into());
    }
    if payload.get("collection").and_then(Value::as_str) != Some(collection) {
        return Err(format!("notification targeted the wrong collection: {notification}").into());
    }
    let record_ids = payload
        .get("record_ids")
        .and_then(Value::as_array)
        .ok_or("mutation notification did not include record_ids")?;
    if !record_ids.iter().any(|id| id.as_str() == Some(record_id)) {
        return Err(format!("notification did not include inserted ID {record_id}").into());
    }
    if payload.get("timestamp").and_then(Value::as_str).is_none() {
        return Err("mutation notification did not include a timestamp".into());
    }
    Ok(&notification["payload"])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let ws_url = env::var("WS_BASE_URL").unwrap_or_else(|_| "ws://localhost:8080".to_string());
    let api_key = env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".into());
    let collection = "ws_subscribe_example_rs";

    // Step 1: Authenticate
    let http_client = reqwest::Client::new();
    let token_data: Value = http_client
        .post(format!("{base_url}/api/auth/token"))
        .json(&json!({ "api_key": api_key }))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let token = token_data["token"]
        .as_str()
        .filter(|token| !token.is_empty())
        .ok_or("authentication response did not include a token")?
        .to_string();
    println!("✓ Authentication successful");
    delete_collection(&http_client, &base_url, &token, collection).await?;

    let operation_result: Result<(), Box<dyn Error>> = async {
        // Step 2: Connect to WebSocket
        println!("\n=== Connecting to WebSocket ===");
        let url = format!("{ws_url}/api/ws");
        let parsed = Url::parse(&url)?;
        let host = parsed
            .host_str()
            .map(|host| match parsed.port() {
                Some(port) => format!("{host}:{port}"),
                None => host.to_string(),
            })
            .unwrap_or_else(|| "localhost:8080".to_string());
        let request = tokio_tungstenite::tungstenite::http::Request::builder()
            .uri(&url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Host", &host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tokio_tungstenite::tungstenite::handshake::client::generate_key(),
            )
            .body(())?;
        let (mut socket, _) = connect_async(request).await?;
        println!("✓ WebSocket connected");

        // Step 3: Subscribe to collection
        println!("\n=== Subscribing to '{collection}' ===");
        let subscribe_msg = json!({
            "type": "Subscribe",
            "payload": { "collection": collection }
        });
        socket
            .send(Message::Text(subscribe_msg.to_string().into()))
            .await?;
        let response = next_json(&mut socket, Duration::from_secs(10)).await?;
        let sub_id = response["payload"]["data"]["subscription_id"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or_else(|| format!("invalid subscription response: {response}"))?;
        println!("✓ Subscribed (subscription_id: {sub_id})");

        // Step 4: Insert records via REST API to trigger notifications
        println!("\n=== Performing mutations to trigger notifications ===");
        println!("Inserting record 1...");
        let insert_data: Value = http_client
            .post(format!("{base_url}/api/insert/{collection}"))
            .bearer_auth(&token)
            .json(&json!({ "name": "Alice", "role": "engineer", "active": true }))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let record1_id = required_id(&insert_data, "first insert")?;
        println!("✓ Inserted: {record1_id}");
        let notification = match next_json(&mut socket, Duration::from_secs(5)).await {
            Ok(notification) => notification,
            Err(error) => {
                if error.to_string().contains("timed out") {
                    println!("  ⏳ No notification within timeout");
                }
                return Err(error);
            }
        };
        let payload = validate_notification(&notification, collection, record1_id)?;
        println!("\n  📡 Notification received:");
        println!("     Event:      {}", payload["event"]);
        println!("     Collection: {}", payload["collection"]);
        println!("     Record IDs: {}", payload["record_ids"]);
        println!("     Timestamp:  {}", payload["timestamp"]);

        println!("\nInserting record 2...");
        let insert_data2: Value = http_client
            .post(format!("{base_url}/api/insert/{collection}"))
            .bearer_auth(&token)
            .json(&json!({ "name": "Bob", "role": "designer", "active": true }))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let record2_id = required_id(&insert_data2, "second insert")?;
        println!("✓ Inserted: {record2_id}");
        let notification = match next_json(&mut socket, Duration::from_secs(5)).await {
            Ok(notification) => notification,
            Err(error) => {
                if error.to_string().contains("timed out") {
                    println!("  ⏳ No notification within timeout");
                }
                return Err(error);
            }
        };
        let payload = validate_notification(&notification, collection, record2_id)?;
        println!("\n  📡 Notification received:");
        println!("     Event:      {}", payload["event"]);
        println!("     Record IDs: {}", payload["record_ids"]);

        // Step 5: Unsubscribe
        println!("\n=== Unsubscribing ===");
        let unsub_msg = json!({
            "type": "Unsubscribe",
            "payload": { "collection": collection }
        });
        socket
            .send(Message::Text(unsub_msg.to_string().into()))
            .await?;
        let unsub_response = next_json(&mut socket, Duration::from_secs(10)).await?;
        let unsub_data = &unsub_response["payload"]["data"];
        if unsub_data["collection"] != collection
            || unsub_data["found"] != true
            || unsub_data["unsubscribed"] != true
        {
            return Err(format!("invalid unsubscribe response: {unsub_response}").into());
        }
        println!("✓ Unsubscribed");
        Ok(())
    }
    .await;

    println!("\n=== Cleanup ===");
    let cleanup_result = delete_collection(&http_client, &base_url, &token, collection).await;
    if matches!(cleanup_result, Ok(true)) {
        println!("✓ Deleted collection '{collection}'");
    }
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), _) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup),
        (Ok(()), Ok(_)) => {}
    }

    println!("\n✓ WebSocket subscription example completed successfully");
    Ok(())
}
