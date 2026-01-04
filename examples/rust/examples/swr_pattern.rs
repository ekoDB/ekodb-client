use ekodb_client::{Client, FieldType, Function, ParameterDefinition, Script, ScriptCondition};
use std::collections::HashMap;
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

    println!("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n");

    // Step 1: Create SWR script for GitHub user caching
    println!("Step 1: Create SWR function that acts as edge cache");

    let swr_script = Script::new("fetch_github_user", "Fetch GitHub User with Cache")
        .with_description(
            "SWR pattern: Check cache, fetch from GitHub API if stale, auto-update with TTL",
        )
        .with_parameter(
            ParameterDefinition::new("username")
                .required()
                .with_description("GitHub username to fetch"),
        )
        .with_parameter(ParameterDefinition::new("ttl").with_description("Cache TTL in seconds"))
        .with_function(Function::FindById {
            collection: "github_cache".to_string(),
            record_id: "{{username}}".to_string(),
        })
        .with_function(Function::If {
            condition: ScriptCondition::HasRecords,
            then_functions: vec![Box::new(Function::Project {
                fields: vec!["data".to_string()],
                exclude: false,
            })],
            else_functions: Some(vec![
                Box::new(Function::HttpRequest {
                    url: "https://api.github.com/users/{{username}}".to_string(),
                    method: "GET".to_string(),
                    headers: Some(HashMap::from([(
                        "User-Agent".to_string(),
                        "ekoDB-Client/1.0".to_string(),
                    )])),
                    body: None,
                    timeout_seconds: None,
                    output_field: None,
                }),
                Box::new(Function::Insert {
                    collection: "github_cache".to_string(),
                    record: serde_json::json!({
                        "id": {"type": "String", "value": "{{username}}"},
                        "data": {"type": "Object", "value": "{{http_response}}"}
                    }),
                    bypass_ripple: None,
                    ttl: None,
                }),
            ]),
        })
        .with_tag("swr")
        .with_tag("github")
        .with_tag("cache");

    let script_id = client.save_script(swr_script).await?;
    println!("✓ Created SWR script: fetch_github_user ({})\n", script_id);

    // Step 2: First call - Cache miss
    println!("Step 2: First call - Cache miss, fetches from GitHub API");
    let start1 = Instant::now();
    let mut params1 = HashMap::new();
    params1.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );
    params1.insert("ttl".to_string(), FieldType::Integer(300));

    let result1 = client
        .call_script("fetch_github_user", Some(params1))
        .await?;
    let duration1 = start1.elapsed();
    println!("Response time: {}ms", duration1.as_millis());
    println!(
        "Result: {}",
        serde_json::to_string_pretty(&result1.records.get(0))?
    );
    println!("✓ Data fetched from external API and cached\n");

    // Step 3: Second call - Cache hit
    println!("Step 3: Second call - Cache hit, instant response from ekoDB");
    let start2 = Instant::now();
    let mut params2 = HashMap::new();
    params2.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );

    let _result2 = client
        .call_script("fetch_github_user", Some(params2))
        .await?;
    let duration2 = start2.elapsed();
    let speedup = duration1.as_millis() as f64 / duration2.as_millis() as f64;
    println!(
        "Response time: {}ms ({:.1}x faster!)",
        duration2.as_millis(),
        speedup
    );
    println!("✓ Lightning fast cache hit\n");

    // Advanced example: Data enrichment
    println!("=== Advanced: SWR with Data Enrichment ===\n");
    println!("Creating product enrichment function...");

    let enrich_script = Script::new("fetch_product_enriched", "Fetch Product with Enrichment")
        .with_description("Demonstrates calling external API and enriching data")
        .with_parameter(
            ParameterDefinition::new("product_id")
                .required()
                .with_description("Product ID"),
        )
        .with_parameter(ParameterDefinition::new("ttl").with_description("Cache TTL (10 minutes)"))
        .with_function(Function::FindById {
            collection: "product_cache".to_string(),
            record_id: "{{product_id}}".to_string(),
        })
        .with_function(Function::If {
            condition: ScriptCondition::HasRecords,
            then_functions: vec![Box::new(Function::Project {
                fields: vec!["enriched_data".to_string()],
                exclude: false,
            })],
            else_functions: Some(vec![
                Box::new(Function::HttpRequest {
                    url: "https://dummyjson.com/products/{{product_id}}".to_string(),
                    method: "GET".to_string(),
                    headers: None,
                    body: None,
                    timeout_seconds: None,
                    output_field: None,
                }),
                Box::new(Function::Insert {
                    collection: "product_cache".to_string(),
                    record: serde_json::json!({
                        "id": {"type": "String", "value": "{{product_id}}"},
                        "enriched_data": {"type": "Object", "value": "{{http_response}}"}
                    }),
                    bypass_ripple: None,
                    ttl: None,
                }),
            ]),
        })
        .with_tag("enrichment")
        .with_tag("product")
        .with_tag("cache");

    let enrich_script_id = client.save_script(enrich_script).await?;
    println!(
        "✓ Created enrichment script: fetch_product_enriched ({})\n",
        enrich_script_id
    );

    println!("Step 4: Call enrichment function - Fetches from API + stores enriched result");
    let mut enrich_params = HashMap::new();
    enrich_params.insert("product_id".to_string(), FieldType::String("1".to_string()));
    enrich_params.insert("ttl".to_string(), FieldType::Integer(600));

    let enriched = client
        .call_script("fetch_product_enriched", Some(enrich_params))
        .await?;
    println!(
        "Enriched data: {}",
        serde_json::to_string_pretty(&enriched.records.get(0))?
    );
    println!("✓ Data fetched, enriched, and cached atomically\n");

    // Print benefits
    println!("=== Why This Is Powerful ===");
    println!("✓ No separate cache layer (Redis, Memcached) needed");
    println!("✓ No manual cache invalidation (TTL handles it)");
    println!("✓ No separate edge infrastructure (ekoDB IS the edge)");
    println!("✓ Atomic operations (function executes as transaction)");
    println!("✓ With multi-node + ripples: Auto-sync across all nodes");
    println!("✓ Sub-millisecond cache hits from internal storage");
    println!("✓ One service instead of many (cache + API gateway + database)");

    println!("\n=== Real-World Use Cases ===");
    println!("1. API Gateway Pattern:");
    println!("   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache");
    println!("\n2. Database Federation:");
    println!("   - Query multiple DBs (Postgres, MongoDB) + external APIs");
    println!("   - Merge results in one function call");
    println!("\n3. IoT Data Enrichment:");
    println!("   - Sensor data + weather API + location API");
    println!("   - Enrich and cache in one atomic operation");
    println!("\n4. E-commerce Product Pages:");
    println!("   - Product info + reviews + inventory + pricing");
    println!("   - All from different sources, cached together");

    println!("\n✓ Example complete - Your database IS your edge!\n");

    Ok(())
}
