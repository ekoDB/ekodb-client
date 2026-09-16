//! KV Store & Wrapped Types Example for ekoDB Rust Client
//!
//! Demonstrates: KV operations in scripts, wrapped type field builders
use ekodb_client::{
    Client, Error, FieldType, Function, ParameterDefinition, Query, Record, SerializationFormat,
    UserFunction,
};
use rust_decimal::Decimal;
use serde_json::Value;
use std::{collections::HashMap, env, error::Error as StdError, str::FromStr};
use uuid::Uuid;

const ORDERS: &str = "orders_example_rs";
const PRODUCTS: &str = "products_example_rs";
const SESSION_KEY: &str = "kv_wrapped:rs:user:session:123";
const CACHE_KEY: &str = "kv_wrapped:rs:cache:product:456";
const PRODUCT_KEY: &str = "kv_wrapped:rs:product:cache:789";
const ORDER_KEY: &str = "kv_wrapped:rs:order:status:c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6";

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
}

fn unwrap_typed(value: &Value) -> &Value {
    if let Value::Object(object) = value {
        if object.len() == 2 && object.contains_key("type") && object.contains_key("value") {
            return unwrap_typed(&object["value"]);
        }
    }
    value
}

fn function_value(result: &ekodb_client::FunctionResult) -> Result<Value, Box<dyn StdError>> {
    if result.records.len() != 1 {
        return Err(format!("expected one function record, got {}", result.records.len()).into());
    }
    let record = serde_json::to_value(&result.records[0])?;
    let value = unwrap_typed(
        record
            .get("value")
            .ok_or("function record had no value field")?,
    );
    Ok(value.clone())
}

async fn delete_kv_if_exists(client: &Client, key: &str) -> Result<(), Error> {
    if client.kv_exists(key).await? {
        client.kv_delete(key).await?;
    }
    Ok(())
}

/// Save a function idempotently: if the label already exists (HTTP 409),
/// update the existing definition instead, then return its id.
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    println!("🚀 ekoDB Rust KV Store & Wrapped Types Example\n");
    println!("📋 Demonstrates:");
    println!("   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)");
    println!("   • KV store operations (get, set, delete, exists, query)");
    println!("   • KV operations within scripts");
    println!("   • Combined wrapped types + KV workflows\n");

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .serialization_format(SerializationFormat::Json)
        .build()?;

    for collection in [ORDERS, PRODUCTS] {
        if let Err(error) = client.delete_collection(collection).await {
            if !is_not_found(&error) {
                return Err(error.into());
            }
        }
    }
    for key in [SESSION_KEY, CACHE_KEY, PRODUCT_KEY, ORDER_KEY] {
        delete_kv_if_exists(&client, key).await?;
    }

    let mut script_ids = Vec::new();
    let operation_result: Result<(), Box<dyn StdError>> = async {
        // Wrapped Types Examples
        wrapped_types_insert(&client).await?;
        wrapped_types_in_script(&client, &mut script_ids).await?;

        // KV Store Examples
        kv_basic_operations(&client).await?;
        kv_script_operations(&client, &mut script_ids).await?;

        // Combined Example
        combined_example(&client, &mut script_ids).await?;
        Ok(())
    }
    .await;

    let cleanup_result = cleanup(&client, &script_ids).await;
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), Ok(())) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup),
        (Ok(()), Ok(())) => {}
    }

    println!("✅ All KV & Wrapped Types examples completed!");
    println!("\n💡 Key takeaways:");
    println!("   ✅ Use FieldType variants for type-safe wrapped values");
    println!("   ✅ FieldType::Decimal preserves precision (no floating point errors)");
    println!("   ✅ KV store is great for caching and quick lookups");
    println!("   ✅ KV operations work within scripts");

    Ok(())
}

// =============================================================================
// Wrapped Types Examples
// =============================================================================

async fn wrapped_types_insert(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 1: Inserting Records with Wrapped Types\n");

    // Insert an order with wrapped types for precise data handling
    let mut order = Record::new();
    order.insert(
        "order_id",
        FieldType::UUID(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()),
    );
    order.insert(
        "total",
        FieldType::Decimal(Decimal::from_str("1234.56").unwrap()),
    );
    order.insert("created_at", FieldType::DateTime(chrono::Utc::now()));
    order.insert("quantity", FieldType::Integer(42));
    order.insert(
        "tags",
        FieldType::Set(vec![
            FieldType::String("priority".to_string()),
            FieldType::String("express".to_string()),
        ]),
    );

    let result = client.insert(ORDERS, order, None).await?;
    if result
        .get_string("id")
        .filter(|id| !id.is_empty())
        .is_none()
    {
        return Err("order insert did not return an ID".into());
    }
    println!("✅ Inserted order: {:?}", result.get("id"));

    // Insert products with wrapped types
    let products = vec![
        ("Laptop Pro", "1299.99", 15, 4.8, true),
        ("Wireless Mouse", "29.99", 150, 4.5, true),
    ];

    let mut product_ids = Vec::new();
    for (name, price, stock, rating, available) in products {
        let mut product = Record::new();
        product.insert("name", FieldType::String(name.to_string()));
        product.insert(
            "price",
            FieldType::Decimal(Decimal::from_str(price).unwrap()),
        );
        product.insert("stock", FieldType::Integer(stock));
        product.insert("rating", FieldType::Float(rating));
        product.insert("available", FieldType::Boolean(available));

        let inserted = client.insert(PRODUCTS, product, None).await?;
        let id = inserted
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("product insert did not return an ID")?;
        if product_ids.iter().any(|existing| existing == id) {
            return Err(format!("duplicate product ID: {id}").into());
        }
        product_ids.push(id.to_string());
    }

    let stored = client.find(PRODUCTS, Query::new().limit(10), None).await?;
    if stored.len() != 2 {
        return Err(format!("expected 2 products, found {}", stored.len()).into());
    }

    println!("✅ Inserted 2 products with wrapped types\n");
    Ok(())
}

async fn wrapped_types_in_script(
    client: &Client,
    script_ids: &mut Vec<String>,
) -> Result<(), Box<dyn StdError>> {
    println!("📝 Example 2: function with Wrapped Type Parameters\n");

    let script = UserFunction::new(
        "create_order_with_types_rs",
        "Create Order with Wrapped Types (Rust)",
    )
    .with_description("Demonstrates wrapped types in script insert operations")
    .with_version("1.0")
    .with_parameter(ParameterDefinition::new("order_total").required())
    .with_function(Function::FindAll {
        collection: PRODUCTS.to_string(),
    });

    let id = save_or_update(client, script).await?;
    script_ids.push(id.clone());
    println!("✅ Function saved: {}", id);

    let mut params = HashMap::new();
    params.insert(
        "order_total".to_string(),
        FieldType::String("599.99".to_string()),
    );

    let result = client
        .call_function("create_order_with_types_rs", Some(params))
        .await?;
    if result.records.len() != 2 {
        return Err(format!(
            "wrapped-types function returned {} records",
            result.records.len()
        )
        .into());
    }
    println!("📊 function executed");
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(())
}

// =============================================================================
// KV Store Examples
// =============================================================================

async fn kv_basic_operations(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 3: Basic KV Store Operations\n");

    // Set a simple value
    let mut session_data = HashMap::new();
    session_data.insert(
        "userId".to_string(),
        serde_json::Value::String("user_abc".to_string()),
    );
    session_data.insert(
        "role".to_string(),
        serde_json::Value::String("admin".to_string()),
    );

    client
        .kv_set(
            SESSION_KEY,
            serde_json::Value::Object(session_data.into_iter().collect()),
            None,
        )
        .await?;
    println!("✅ Set session data");

    // Get the value back
    let session = client.kv_get(SESSION_KEY).await?;
    println!("📊 Retrieved session: {:?}", session);
    let session_value = session
        .as_ref()
        .map(unwrap_typed)
        .ok_or("session KV value was missing")?;
    if session_value["userId"] != "user_abc" || session_value["role"] != "admin" {
        return Err(format!("unexpected session KV value: {session_value}").into());
    }

    // Check if key exists
    let exists = client.kv_exists(SESSION_KEY).await?;
    println!("🔍 Key exists: {}", exists);
    if !exists {
        return Err("session KV key did not exist after kv_set".into());
    }

    // Set with TTL (1 hour)
    let mut cache_data = HashMap::new();
    cache_data.insert(
        "name".to_string(),
        serde_json::Value::String("Cached Product".to_string()),
    );
    cache_data.insert(
        "price".to_string(),
        serde_json::Value::Number(serde_json::Number::from_f64(99.99).unwrap()),
    );

    client
        .kv_set(
            CACHE_KEY,
            serde_json::Value::Object(cache_data.into_iter().collect()),
            Some("1h"),
        )
        .await?;
    println!("✅ Set cached data");

    // Delete a key
    client.kv_delete(SESSION_KEY).await?;
    println!("🗑️  Deleted session\n");
    if client.kv_exists(SESSION_KEY).await? {
        return Err("session KV key still existed after delete".into());
    }

    Ok(())
}

async fn kv_script_operations(
    client: &Client,
    script_ids: &mut Vec<String>,
) -> Result<(), Box<dyn StdError>> {
    println!("📝 Example 4: KV Operations in Functions\n");

    let script = UserFunction::new("cached_product_lookup_rs", "Cached Product Lookup (Rust)")
        .with_description("Uses KV store for caching within a script")
        .with_version("1.0")
        .with_parameter(ParameterDefinition::new("product_key").required())
        .with_tag("kv")
        .with_tag("caching")
        .with_function(Function::KvGet {
            key: serde_json::Value::String("{{product_key}}".to_string()),
        });

    let id = save_or_update(client, script).await?;
    script_ids.push(id.clone());
    println!("✅ Function saved: {}", id);

    // First set some data to retrieve
    let mut product_data = HashMap::new();
    product_data.insert(
        "name".to_string(),
        serde_json::Value::String("Test Product".to_string()),
    );
    product_data.insert(
        "price".to_string(),
        serde_json::Value::Number(serde_json::Number::from_f64(49.99).unwrap()),
    );

    client
        .kv_set(
            PRODUCT_KEY,
            serde_json::Value::Object(product_data.into_iter().collect()),
            None,
        )
        .await?;

    let mut params = HashMap::new();
    params.insert(
        "product_key".to_string(),
        FieldType::String(PRODUCT_KEY.to_string()),
    );

    let result = client
        .call_function("cached_product_lookup_rs", Some(params))
        .await?;
    let value = function_value(&result)?;
    if value["name"] != "Test Product" || value["price"] != 49.99 {
        return Err(format!("unexpected cached product: {value}").into());
    }
    println!("📊 Cached and retrieved product data");
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(())
}

// =============================================================================
// Combined Example
// =============================================================================

async fn combined_example(
    client: &Client,
    script_ids: &mut Vec<String>,
) -> Result<(), Box<dyn StdError>> {
    println!("📝 Example 5: Combined Wrapped Types + KV Function\n");

    let script = UserFunction::new(
        "process_order_with_cache_rs",
        "Process Order with Cache (Rust)",
    )
    .with_description("Demonstrates combined KV and wrapped type usage")
    .with_version("1.0")
    .with_parameter(ParameterDefinition::new("order_id").required())
    .with_tag("orders")
    .with_tag("kv")
    .with_tag("wrapped-types")
    .with_function(Function::KvGet {
        key: serde_json::Value::String("kv_wrapped:rs:order:status:{{order_id}}".to_string()),
    });

    let id = save_or_update(client, script).await?;
    script_ids.push(id.clone());
    println!("✅ Function saved: {}", id);

    // Set order status in KV store
    let mut status_data = HashMap::new();
    status_data.insert(
        "status".to_string(),
        serde_json::Value::String("processing".to_string()),
    );
    status_data.insert(
        "updated_at".to_string(),
        serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
    );

    client
        .kv_set(
            ORDER_KEY,
            serde_json::Value::Object(status_data.into_iter().collect()),
            None,
        )
        .await?;
    let mut params = HashMap::new();
    params.insert(
        "order_id".to_string(),
        FieldType::String("c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6".to_string()),
    );

    let result = client
        .call_function("process_order_with_cache_rs", Some(params))
        .await?;
    let value = function_value(&result)?;
    if value["status"] != "processing" || value["updated_at"].as_str().is_none() {
        return Err(format!("unexpected order status: {value}").into());
    }
    println!("📊 Processed order with caching");
    println!("⏱️  Stages executed: {}", result.stats.stages_executed);
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(())
}

// =============================================================================
// Cleanup
// =============================================================================

async fn cleanup(client: &Client, script_ids: &[String]) -> Result<(), Box<dyn StdError>> {
    println!("🧹 Cleaning up...");

    let mut errors = Vec::new();
    for id in script_ids.iter().rev() {
        if let Err(error) = client.delete_function(id).await {
            errors.push(format!("function {id}: {error}"));
        }
    }

    for collection in [ORDERS, PRODUCTS] {
        if let Err(error) = client.delete_collection(collection).await {
            if !is_not_found(&error) {
                errors.push(format!("collection {collection}: {error}"));
            }
        }
    }
    for key in [SESSION_KEY, CACHE_KEY, PRODUCT_KEY, ORDER_KEY] {
        if let Err(error) = delete_kv_if_exists(client, key).await {
            errors.push(format!("KV key {key}: {error}"));
        }
    }

    if !errors.is_empty() {
        return Err(format!("cleanup failed: {}", errors.join("; ")).into());
    }

    println!("✅ Cleanup complete\n");
    Ok(())
}
