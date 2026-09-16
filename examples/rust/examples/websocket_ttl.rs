use futures::{SinkExt, StreamExt};
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

const COLLECTION: &str = "ws_ttl_test_rs";

async fn delete_collection(
    client: &Client,
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .delete(format!("{base_url}/api/collections/{COLLECTION}"))
        .bearer_auth(token)
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::NOT_FOUND {
        response.error_for_status()?;
    }
    Ok(())
}

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

    delete_collection(&client, &base_url, token).await?;
    let operation_result = run_example(&client, &base_url, &ws_url, token).await;
    let cleanup_result = delete_collection(&client, &base_url, token).await;
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), Ok(())) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup),
        (Ok(()), Ok(())) => {}
    }
    println!("\n✓ WebSocket TTL example completed successfully");
    Ok(())
}

async fn run_example(
    client: &Client,
    base_url: &str,
    ws_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Insert test data with TTL
    println!("\n=== Insert Test Data with TTL ===");
    let doc: Value = client
        .post(format!("{base_url}/api/insert/{COLLECTION}"))
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
    let parsed = Url::parse(&ws_url_full)?;
    let host = parsed
        .host_str()
        .map(|h| match parsed.port() {
            Some(p) => format!("{}:{}", h, p),
            None => h.to_string(),
        })
        .unwrap_or_else(|| "localhost:8080".to_string());

    // Create request with auth header and WebSocket headers
    let request = tokio_tungstenite::tungstenite::http::Request::builder()
        .uri(&ws_url_full)
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

    // Send query
    write
        .send(Message::Text(
            json!({
                "type": "FindAll",
                "messageId": "1",
                "payload": {
                    "collection": COLLECTION
                }
            })
            .to_string()
            .into(),
        ))
        .await?;

    // Wait for one complete response and fail rather than reporting a false success.
    let msg = tokio::time::timeout(std::time::Duration::from_secs(10), read.next())
        .await
        .map_err(|_| "WebSocket response timed out")?
        .ok_or("WebSocket closed before returning data")??;
    let text = msg.into_text()?;
    let response: Value = serde_json::from_str(&text)?;
    let records = response
        .pointer("/payload/data")
        .and_then(Value::as_array)
        .ok_or("WebSocket response did not contain payload.data")?;
    if records.len() != 1 {
        return Err(format!("expected exactly 1 WebSocket record, got {}", records.len()).into());
    }
    println!("✓ Retrieved 1 record via WebSocket");

    for (i, record) in records.iter().enumerate() {
        println!("\nRecord {}:", i + 1);
        if let Some(obj) = record.as_object() {
            for (key, value) in obj {
                println!("  {}: {}", key, value);
            }
        }
    }

    Ok(())
}
