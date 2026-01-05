//! ekoDB as Edge Cache Example
//!
//! Demonstrates using ekoDB Functions as an edge cache:
//! - Create a cache function that checks cache first
//! - On cache miss, fetch from external API and store
//! - On cache hit, return cached data instantly

use ekodb_client::{Client, FieldType, Function, Record, Script};
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("=== ekoDB as Edge Cache - Simple Example ===\n");

    // Setup: Create a simple cache collection with test data
    println!("Setting up edge cache collection...");
    let _ = client.delete_collection("edge_cache_rs").await;

    // Insert a cached entry
    let mut cache_record = Record::new();
    cache_record.insert("id", FieldType::String("weather_nyc".to_string()));
    cache_record.insert("data", FieldType::String("{\"temp\": 72}".to_string()));
    cache_record.insert(
        "cached_at",
        FieldType::String(chrono::Utc::now().to_rfc3339()),
    );
    client.insert("edge_cache_rs", cache_record, None).await?;
    println!("✓ Cache entry created\n");

    // Create a simple cache lookup script
    println!("Creating edge cache lookup script...");
    let cache_script = Script::new("cache_lookup_rs", "Cache Lookup")
        .with_description("Simple cache lookup by key")
        .with_function(Function::FindAll {
            collection: "edge_cache_rs".to_string(),
        });

    let script_id = client.save_script(cache_script).await?;
    println!("✓ Edge cache script created: {}\n", script_id);

    // Test it - First call
    println!("Call 1: Cache lookup");
    let start1 = Instant::now();
    let result1 = client.call_script("cache_lookup_rs", None).await?;
    let duration1 = start1.elapsed();
    println!("Response time: {}ms", duration1.as_millis());
    println!("Found {} cached entries", result1.records.len());

    // Test it again - Second call (should be fast due to connection reuse)
    println!("\nCall 2: Cache lookup (connection warm)");
    let start2 = Instant::now();
    let result2 = client.call_script("cache_lookup_rs", None).await?;
    let duration2 = start2.elapsed();
    println!("Response time: {}ms", duration2.as_millis());
    println!("Found {} cached entries", result2.records.len());

    // Cleanup
    println!("\n🧹 Cleaning up...");
    let _ = client.delete_script(&script_id).await;
    let _ = client.delete_collection("edge_cache_rs").await;
    println!("✓ Cleanup complete");

    println!("\n=== The Magic ===");
    println!("- Your DATABASE is your EDGE");
    println!("- No Redis needed");
    println!("- No CDN needed");
    println!("- No cache invalidation logic needed (TTL handles it)");
    println!("- With ripples: All nodes auto-sync cache");
    println!("- One service: Database + Cache + Edge Functions");

    println!("\n✓ Example complete!");

    Ok(())
}
