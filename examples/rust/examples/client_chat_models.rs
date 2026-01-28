//! Chat Models API Example - Using ekodb_client library
//!
//! This example demonstrates how to retrieve available chat models
//! from the ekoDB server organized by provider.

use ekodb_client::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Create ekoDB client
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("✓ Client created");

    // Example 1: Get All Chat Models
    println!("\n=== Get All Chat Models ===");
    match client.get_chat_models().await {
        Ok(models) => {
            println!("Available chat models by provider:");
            println!("\nOpenAI models ({}):", models.openai.len());
            for model in &models.openai {
                println!("  - {}", model);
            }
            println!("\nAnthropic models ({}):", models.anthropic.len());
            for model in &models.anthropic {
                println!("  - {}", model);
            }
            println!("\nPerplexity models ({}):", models.perplexity.len());
            for model in &models.perplexity {
                println!("  - {}", model);
            }
        }
        Err(e) => println!("GetChatModels error: {}", e),
    }

    // Example 2: Get Models for Specific Provider
    println!("\n=== Get Models for Specific Provider ===");
    let providers = ["openai", "anthropic", "perplexity"];

    for provider in providers {
        match client.get_chat_model(provider).await {
            Ok(models) => {
                println!("\n{} models ({}):", provider, models.len());
                for model in models.iter().take(5) {
                    println!("  - {}", model);
                }
                if models.len() > 5 {
                    println!("  ... and {} more", models.len() - 5);
                }
            }
            Err(e) => println!("GetChatModel({}) error: {}", provider, e),
        }
    }

    println!("\n✓ Chat Models API example complete");

    Ok(())
}
