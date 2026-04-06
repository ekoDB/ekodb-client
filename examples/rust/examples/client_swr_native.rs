//! Native SWR Function Example
//!
//! Demonstrates the new native SWR function in ekoDB that simplifies cache-aside patterns:
//! - Single function call handles cache lookup, HTTP fetch, and storage
//! - Built-in TTL support with duration strings ("15m", "1h", "2h")
//! - Optional audit trail collection for debugging
//! - Automatic enriched parameter population for downstream functions
//!
//! Compare this with client_swr_pattern.rs to see how much simpler the native function is!

use ekodb_client::{Client, FieldType, Function, ParameterDefinition, UserFunction};
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

    println!("=== ekoDB Native SWR Function ===\n");
    println!("This example shows the new simplified SWR function that replaces");
    println!("the manual FindById → If → HttpRequest → Insert pattern.\n");

    // Example 1: Basic SWR with duration string TTL
    println!("Example 1: Basic GitHub User Cache with Native SWR");
    println!("─────────────────────────────────────────────────────\n");

    let basic_swr_script = UserFunction::new("github_user_native", "Native SWR GitHub User Cache")
        .with_description("Demonstrates native SWR function with cache-aside pattern")
        .with_parameter(
            ParameterDefinition::new("username")
                .required()
                .with_description("GitHub username to fetch"),
        )
        .with_function(Function::SWR {
            cache_key: "github:{{username}}".to_string(),
            ttl: json!("15m"), // Duration string TTL
            url: "https://api.github.com/users/{{username}}".to_string(),
            method: "GET".to_string(),
            headers: Some(HashMap::from([(
                "User-Agent".to_string(),
                "ekoDB-Native-SWR/1.0".to_string(),
            )])),
            body: None,
            timeout_seconds: Some(10),
            output_field: Some("github_data".to_string()),
            collection: None, // No audit trail for this example
        })
        .with_tag("swr")
        .with_tag("native")
        .with_tag("github");

    let script_id = client.save_function(basic_swr_script).await?;
    println!(
        "✓ Created native SWR script: github_user_native ({})",
        script_id
    );

    // First call - cache miss
    println!("\nFirst call (cache miss - will fetch from GitHub API):");
    let start1 = Instant::now();
    let mut params1 = HashMap::new();
    params1.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );

    let result1 = client
        .call_function("github_user_native", Some(params1))
        .await?;
    let duration1 = start1.elapsed();
    println!("  Response time: {}ms", duration1.as_millis());
    println!("  Records returned: {}", result1.records.len());
    println!("  ✓ Data fetched from API and cached with 15m TTL");

    // Second call - cache hit
    println!("\nSecond call (cache hit - instant from KV store):");
    let start2 = Instant::now();
    let mut params2 = HashMap::new();
    params2.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );

    let _result2 = client
        .call_function("github_user_native", Some(params2))
        .await?;
    let duration2 = start2.elapsed();
    let speedup = if duration2.as_millis() > 0 {
        duration1.as_millis() as f64 / duration2.as_millis() as f64
    } else {
        0.0
    };
    println!("  Response time: {}ms", duration2.as_millis());
    println!("  Speedup: {:.1}x faster", speedup);
    println!("  ✓ Lightning fast cache hit\n");

    // Example 2: SWR with Audit Trail
    println!("\nExample 2: SWR with Audit Trail Collection");
    println!("─────────────────────────────────────────────────────\n");

    let audit_swr_script = UserFunction::new("product_swr_audit", "Product Cache with Audit Trail")
        .with_description("Native SWR with audit collection for debugging")
        .with_parameter(
            ParameterDefinition::new("product_id")
                .required()
                .with_description("Product ID to fetch"),
        )
        .with_function(Function::SWR {
            cache_key: "product:{{product_id}}".to_string(),
            ttl: json!("1h"), // 1 hour cache
            url: "https://dummyjson.com/products/{{product_id}}".to_string(),
            method: "GET".to_string(),
            headers: None,
            body: None,
            timeout_seconds: None,
            output_field: Some("product_data".to_string()),
            collection: Some("swr_audit_trail".to_string()), // Audit trail enabled
        })
        .with_tag("swr")
        .with_tag("audit");

    let audit_script_id = client.save_function(audit_swr_script).await?;
    println!(
        "✓ Created SWR script with audit trail: product_swr_audit ({})",
        audit_script_id
    );

    println!("\nFetching product (will create audit trail entry):");
    let mut product_params = HashMap::new();
    product_params.insert("product_id".to_string(), FieldType::String("1".to_string()));

    let product_result = client
        .call_function("product_swr_audit", Some(product_params))
        .await?;
    println!("  ✓ Product fetched and cached");
    println!("  ✓ Audit record created in 'swr_audit_trail' collection");
    println!("  Records: {}\n", product_result.records.len());

    // Example 3: SWR with Data Enrichment Pipeline
    println!("\nExample 3: SWR in Multi-Function Pipeline");
    println!("─────────────────────────────────────────────────────\n");
    println!("Fetch external data → Process → Store in collection");

    let pipeline_script =
        UserFunction::new("user_enrichment_pipeline", "User Data Enrichment Pipeline")
            .with_description("Fetch from API, enrich with SWR, store processed result")
            .with_parameter(
                ParameterDefinition::new("user_id")
                    .required()
                    .with_description("User ID to enrich"),
            )
            // Step 1: Fetch user data with SWR caching
            .with_function(Function::SWR {
                cache_key: "user_api:{{user_id}}".to_string(),
                ttl: json!("30m"),
                url: "https://jsonplaceholder.typicode.com/users/{{user_id}}".to_string(),
                method: "GET".to_string(),
                headers: None,
                body: None,
                timeout_seconds: None,
                output_field: Some("user_data".to_string()), // Populates enriched params
                collection: None,
            })
            // Step 2: Insert enriched data into permanent collection
            .with_function(Function::Insert {
                collection: "enriched_users".to_string(),
                record: json!({
                    "user_id": {"type": "String", "value": "{{user_id}}"},
                    "source_data": {"type": "Object", "value": "{{user_data}}"} // Uses enriched param from SWR
                }),
                bypass_ripple: None,
                ttl: Some(json!("24h")), // Different TTL for permanent storage
            })
            .with_tag("enrichment")
            .with_tag("pipeline");

    let pipeline_script_id = client.save_function(pipeline_script).await?;
    println!(
        "✓ Created enrichment pipeline: user_enrichment_pipeline ({})",
        pipeline_script_id
    );

    println!("\nRunning pipeline:");
    let mut enrich_params = HashMap::new();
    enrich_params.insert("user_id".to_string(), FieldType::String("1".to_string()));

    let enrich_result = client
        .call_function("user_enrichment_pipeline", Some(enrich_params))
        .await?;
    println!("  ✓ Data fetched from API (cached 30m)");
    println!("  ✓ Enriched data stored in 'enriched_users' (TTL 24h)");
    println!(
        "  Pipeline returned {} records\n",
        enrich_result.records.len()
    );

    // Example 4: Parameterized TTL
    println!("\nExample 4: Dynamic TTL Configuration");
    println!("─────────────────────────────────────────────────────\n");

    let dynamic_ttl_script = UserFunction::new("flexible_cache", "Flexible Cache with Dynamic TTL")
        .with_description("SWR function with user-controlled TTL")
        .with_parameter(
            ParameterDefinition::new("resource_id")
                .required()
                .with_description("Resource identifier"),
        )
        .with_parameter(
            ParameterDefinition::new("ttl")
                .with_default(FieldType::String("10m".to_string()))
                .with_description("Cache TTL (e.g., '5m', '1h', '2h')"),
        )
        .with_function(Function::SWR {
            cache_key: "resource:{{resource_id}}".to_string(),
            ttl: json!("{{ttl}}"), // User-provided TTL
            url: "https://app.ekodb.io/api/health".to_string(),
            method: "GET".to_string(),
            headers: None,
            body: None,
            timeout_seconds: None,
            output_field: Some("resource".to_string()),
            collection: None,
        })
        .with_tag("dynamic");

    let dynamic_script_id = client.save_function(dynamic_ttl_script).await?;
    println!(
        "✓ Created dynamic TTL script: flexible_cache ({})",
        dynamic_script_id
    );

    // Test with different TTLs
    let ttl_tests = vec![("5m", "5 minutes"), ("1h", "1 hour"), ("30s", "30 seconds")];

    for (ttl_value, description) in ttl_tests {
        let mut dynamic_params = HashMap::new();
        dynamic_params.insert(
            "resource_id".to_string(),
            FieldType::String("test".to_string()),
        );
        dynamic_params.insert("ttl".to_string(), FieldType::String(ttl_value.to_string()));

        let _ = client
            .call_function("flexible_cache", Some(dynamic_params))
            .await?;
        println!("  ✓ Cached with TTL: {} ({})", ttl_value, description);
    }

    println!("\n=== Key Benefits of Native SWR Function ===");
    println!("✅ Simpler: One function instead of 4 (FindById → If → HttpRequest → Insert)");
    println!("✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds");
    println!("✅ Built-in audit: Optional collection parameter for automatic logging");
    println!("✅ Auto-enrichment: output_field populates params for downstream functions");
    println!(
        "✅ Transactional: Works correctly in both transactional and non-transactional contexts"
    );
    println!("✅ KV-optimized: Uses native KV store with proper TTL handling");

    println!("\n=== Performance Comparison ===");
    println!("Old pattern (manual):  ~4 function calls, complex script logic");
    println!("New pattern (native):  ~1 function call, handled by server");
    println!("Result: Simpler code, faster execution, easier maintenance\n");

    // Cleanup
    println!("🧹 Cleaning up test data...");
    let _ = client.delete_function(&script_id).await;
    let _ = client.delete_function(&audit_script_id).await;
    let _ = client.delete_function(&pipeline_script_id).await;
    let _ = client.delete_function(&dynamic_script_id).await;
    let _ = client.delete_collection("swr_audit_trail").await;
    let _ = client.delete_collection("enriched_users").await;
    println!("✓ Cleanup complete\n");

    println!("=== Summary ===");
    println!("The native SWR function provides a production-ready cache-aside pattern");
    println!("with minimal code and maximum flexibility. Use it for:");
    println!("  • API gateway caching");
    println!("  • External API aggregation");
    println!("  • Microservice response caching");
    println!("  • Edge computing patterns");
    println!("  • Real-time data enrichment pipelines\n");

    Ok(())
}
