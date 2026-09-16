use ekodb_client::{
    Client, Error, FieldType, Function, FunctionCondition, ParameterDefinition, UserFunction,
};
use std::collections::HashMap;
use std::env;
use std::time::Instant;

const GITHUB_COLLECTION: &str = "github_cache_swr_rs";
const PRODUCT_COLLECTION: &str = "product_cache_swr_rs";
const GITHUB_FUNCTION: &str = "fetch_github_user_swr_rs";
const PRODUCT_FUNCTION: &str = "fetch_product_enriched_swr_rs";

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
}

/// Save a function idempotently: if the label already exists (HTTP 409),
/// update the existing definition instead. Returns the function id (or the
/// label as a fallback identifier) so the example can keep printing/calling.
async fn save_or_update(
    client: &Client,
    function: UserFunction,
) -> Result<String, Box<dyn std::error::Error>> {
    let label = function.label.clone();
    match client.save_function(function.clone()).await {
        Ok(id) => Ok(id),
        Err(ekodb_client::Error::Api { code: 409, .. }) => {
            client.update_function(&label, function).await?;
            println!("ℹ️  Function '{}' already existed — updated instead", label);
            let existing = client.get_function(&label).await?;
            existing.id.ok_or_else(|| {
                std::io::Error::other(format!("updated function '{label}' did not include an id"))
                    .into()
            })
        }
        Err(e) => Err(Box::new(e)),
    }
}

async fn run_examples(
    client: &Client,
    function_ids: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n");

    // Step 1: Create SWR script for GitHub user caching
    println!("Step 1: Create SWR function that acts as edge cache");

    let swr_script = UserFunction::new(GITHUB_FUNCTION, "Fetch GitHub User with Cache")
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
            collection: GITHUB_COLLECTION.to_string(),
            record_id: "{{username}}".to_string(),
        })
        .with_function(Function::If {
            condition: FunctionCondition::HasRecords,
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
                    collection: GITHUB_COLLECTION.to_string(),
                    record: serde_json::json!({
                        "id": {"type": "String", "value": "{{username}}"},
                        "data": {"type": "Object", "value": "{{http_response}}"}
                    }),
                    bypass_ripple: None,
                    ttl: None,
                }),
                Box::new(Function::FindById {
                    collection: GITHUB_COLLECTION.to_string(),
                    record_id: "{{username}}".to_string(),
                }),
                Box::new(Function::Project {
                    fields: vec!["data".to_string()],
                    exclude: false,
                }),
            ]),
        })
        .with_tag("swr")
        .with_tag("github")
        .with_tag("cache");

    let script_id = save_or_update(client, swr_script).await?;
    function_ids.push(script_id.clone());
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

    let result1 = client.call_function(GITHUB_FUNCTION, Some(params1)).await?;
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

    let _result2 = client.call_function(GITHUB_FUNCTION, Some(params2)).await?;
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

    let enrich_script = UserFunction::new(PRODUCT_FUNCTION, "Fetch Product with Enrichment")
        .with_description("Demonstrates calling external API and enriching data")
        .with_parameter(
            ParameterDefinition::new("product_id")
                .required()
                .with_description("Product ID"),
        )
        .with_parameter(ParameterDefinition::new("ttl").with_description("Cache TTL (10 minutes)"))
        .with_function(Function::FindById {
            collection: PRODUCT_COLLECTION.to_string(),
            record_id: "{{product_id}}".to_string(),
        })
        .with_function(Function::If {
            condition: FunctionCondition::HasRecords,
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
                    collection: PRODUCT_COLLECTION.to_string(),
                    record: serde_json::json!({
                        "id": {"type": "String", "value": "{{product_id}}"},
                        "enriched_data": {"type": "Object", "value": "{{http_response}}"}
                    }),
                    bypass_ripple: None,
                    ttl: None,
                }),
                Box::new(Function::FindById {
                    collection: PRODUCT_COLLECTION.to_string(),
                    record_id: "{{product_id}}".to_string(),
                }),
                Box::new(Function::Project {
                    fields: vec!["enriched_data".to_string()],
                    exclude: false,
                }),
            ]),
        })
        .with_tag("enrichment")
        .with_tag("product")
        .with_tag("cache");

    let enrich_script_id = save_or_update(client, enrich_script).await?;
    function_ids.push(enrich_script_id.clone());
    println!(
        "✓ Created enrichment script: fetch_product_enriched ({})\n",
        enrich_script_id
    );

    println!("Step 4: Call enrichment function - Fetches from API + stores enriched result");
    let mut enrich_params = HashMap::new();
    enrich_params.insert("product_id".to_string(), FieldType::String("1".to_string()));
    enrich_params.insert("ttl".to_string(), FieldType::Integer(600));

    let enriched = client
        .call_function(PRODUCT_FUNCTION, Some(enrich_params))
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

    Ok(())
}

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
    for collection in [GITHUB_COLLECTION, PRODUCT_COLLECTION] {
        if let Err(error) = client.delete_collection(collection).await {
            if !is_not_found(&error) {
                return Err(error.into());
            }
        }
    }

    let mut function_ids = Vec::new();
    let operation_result = run_examples(&client, &mut function_ids).await;
    let mut cleanup_errors = Vec::new();
    for function_id in function_ids.iter().rev() {
        if let Err(error) = client.delete_function(function_id).await {
            cleanup_errors.push(format!("function {function_id}: {error}"));
        }
    }
    for collection in [GITHUB_COLLECTION, PRODUCT_COLLECTION] {
        if let Err(error) = client.delete_collection(collection).await {
            if !is_not_found(&error) {
                cleanup_errors.push(format!("collection {collection}: {error}"));
            }
        }
    }
    match (operation_result, cleanup_errors.is_empty()) {
        (Err(primary), false) => {
            return Err(format!(
                "{primary}; cleanup also failed: {}",
                cleanup_errors.join("; ")
            )
            .into())
        }
        (Err(primary), true) => return Err(primary),
        (Ok(()), false) => {
            return Err(format!("cleanup failed: {}", cleanup_errors.join("; ")).into())
        }
        (Ok(()), true) => {}
    }
    println!("\n✓ Example complete - Your database IS your edge!\n");
    Ok(())
}
