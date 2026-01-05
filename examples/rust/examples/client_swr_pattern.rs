//! SWR (Stale-While-Revalidate) Pattern Example
//!
//! Demonstrates using ekoDB Functions as an SWR cache:
//! - Check cache first, return if exists
//! - On cache miss, fetch from external API
//! - Store result with TTL for auto-expiration

use ekodb_client::{Client, FieldType, Function, ParameterDefinition, Script, ScriptCondition};
use serde_json::json;
use std::collections::HashMap;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        std::env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n");

    println!("Step 1: Create SWR function that acts as edge cache");

    let mut parameters = HashMap::new();
    parameters.insert(
        "username".to_string(),
        ParameterDefinition::new("username")
            .required()
            .with_description("GitHub username to fetch"),
    );
    parameters.insert(
        "ttl".to_string(),
        ParameterDefinition::new("ttl")
            .required()
            .with_description("Cache TTL in seconds"),
    );
    parameters.insert(
        "cached_at".to_string(),
        ParameterDefinition::new("cached_at")
            .required()
            .with_description("UTC timestamp when cached"),
    );

    let swr_script = Script {
        label: "fetch_github_user_rs".to_string(),
        name: "Fetch GitHub User with Cache".to_string(),
        description: Some("SWR pattern: Check cache, fetch from GitHub API if stale".to_string()),
        version: Some("1.0".to_string()),
        parameters,
        functions: vec![
            Function::FindById {
                collection: "github_cache_rs".to_string(),
                record_id: "{{username}}".to_string(),
            },
            Function::If {
                condition: ScriptCondition::HasRecords,
                then_functions: vec![Box::new(Function::Project {
                    fields: vec!["data".to_string(), "cached_at".to_string()],
                    exclude: false,
                })],
                else_functions: Some(vec![
                    Box::new(Function::HttpRequest {
                        url: "https://api.github.com/users/{{username}}".to_string(),
                        method: "GET".to_string(),
                        headers: Some(HashMap::from([(
                            "User-Agent".to_string(),
                            "ekoDB-SWR-Example".to_string(),
                        )])),
                        body: None,
                        timeout_seconds: None,
                        output_field: None,
                    }),
                    Box::new(Function::Insert {
                        collection: "github_cache_rs".to_string(),
                        record: json!({
                            "id": {"type": "String", "value": "{{username}}"},
                            "data": {"type": "Object", "value": "{{http_response}}"},
                            "cached_at": {"type": "String", "value": "{{cached_at}}"}
                        }),
                        bypass_ripple: Some(false),
                        ttl: Some(json!("{{ttl}}")),
                    }),
                ]),
            },
        ],
        tags: vec!["swr".to_string(), "github".to_string(), "cache".to_string()],
        created_at: None,
        updated_at: None,
    };

    let script_id = client.save_script(swr_script).await?;
    println!(
        "✓ Created SWR script: fetch_github_user_rs ({})\n",
        script_id
    );

    println!("Step 2: First call - Cache miss, fetches from GitHub API");
    let mut params1 = HashMap::new();
    params1.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );
    params1.insert("ttl".to_string(), FieldType::Integer(300));
    params1.insert(
        "cached_at".to_string(),
        FieldType::String(chrono::Utc::now().to_rfc3339()),
    );
    let result1 = client
        .call_script("fetch_github_user_rs", Some(params1))
        .await?;
    println!("Result: {:?}", result1.stats);
    println!("✓ Data fetched from external API and cached\n");

    println!("Step 3: Second call - Cache hit, instant response from ekoDB");
    let start = Instant::now();
    let mut params2 = HashMap::new();
    params2.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );
    params2.insert("ttl".to_string(), FieldType::Integer(300));
    params2.insert(
        "cached_at".to_string(),
        FieldType::String(chrono::Utc::now().to_rfc3339()),
    );
    let _ = client
        .call_script("fetch_github_user_rs", Some(params2))
        .await?;
    let duration = start.elapsed();
    println!(
        "Response time: {}ms (served from cache)",
        duration.as_millis()
    );
    println!("✓ Lightning fast cache hit\n");

    // Cleanup
    println!("🧹 Cleaning up...");
    let _ = client.delete_script(&script_id).await;
    let _ = client.delete_collection("github_cache_rs").await;
    println!("✓ Cleanup complete\n");

    println!("=== SWR Pattern Summary ===");
    println!("✅ Cache miss → Fetch from API → Store in ekoDB");
    println!("✅ Cache hit → Instant response from ekoDB");
    println!("✅ TTL handles automatic cache invalidation");

    Ok(())
}
