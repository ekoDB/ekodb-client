//! Raw Completion Stream Example
//!
//! This example demonstrates using SSE streaming for raw LLM completions.
//! SSE streaming keeps the connection alive with heartbeat events, making it
//! ideal for deployed instances where reverse proxies may kill idle connections.
//!
//! Prerequisites:
//! - Set OPENAI_API_KEY or ANTHROPIC_API_KEY on the ekoDB server
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_raw_completion_stream`

use ekodb_client::{Client, RawCompletionRequest};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB Raw Completion Stream (SSE) Example ===\n");

    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    // --- 1. Basic streaming raw completion ---
    println!("--- Basic SSE Raw Completion ---");
    let request = RawCompletionRequest {
        system_prompt: "You are a helpful assistant. Keep responses concise.".to_string(),
        message: "What are the three primary colors?".to_string(),
        provider: None,
        model: None,
        max_tokens: Some(256),
    };

    let response = client.raw_completion_stream(request).await?;
    println!("Response: {}\n", response.content);

    // --- 2. Structured output via SSE ---
    println!("--- Structured Output via SSE ---");
    let request = RawCompletionRequest {
        system_prompt: "Output only valid JSON. No other text.".to_string(),
        message: "Return a JSON array of 3 planets with name and diameter_km fields.".to_string(),
        provider: None,
        model: None,
        max_tokens: Some(512),
    };

    let response = client.raw_completion_stream(request).await?;
    println!("JSON response: {}\n", response.content);

    // --- 3. Comparison: blocking HTTP vs SSE ---
    // On deployed instances behind reverse proxies, raw_completion() may timeout.
    // raw_completion_stream() keeps the connection alive with SSE heartbeats.
    println!("--- Blocking HTTP (for comparison) ---");
    let request = RawCompletionRequest {
        system_prompt: "You are a helpful assistant.".to_string(),
        message: "Say hello in one sentence.".to_string(),
        provider: None,
        model: None,
        max_tokens: Some(64),
    };

    let response = client.raw_completion(request).await?;
    println!("Blocking response: {}\n", response.content);

    println!("=== Done ===");
    Ok(())
}
