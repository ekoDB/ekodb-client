///! KV Store & Wrapped Types Example for ekoDB Rust Client
///!
///! Demonstrates: KV operations in scripts, wrapped type field builders
use ekodb_client::{
    Client, FieldType, Function, ParameterDefinition, Record, SerializationFormat, UserFunction,
};
use rust_decimal::Decimal;
use std::{collections::HashMap, env, str::FromStr};
use uuid::Uuid;

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

    let mut script_ids = Vec::new();

    // Wrapped Types Examples
    wrapped_types_insert(&client).await?;

    if let Ok(id) = wrapped_types_in_script(&client).await {
        script_ids.push(id);
    }

    // KV Store Examples
    kv_basic_operations(&client).await?;

    if let Ok(id) = kv_script_operations(&client).await {
        script_ids.push(id);
    }

    // Combined Example
    if let Ok(id) = combined_example(&client).await {
        script_ids.push(id);
    }

    // Cleanup
    cleanup(&client, &script_ids).await?;

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

    let result = client.insert("orders_example_rs", order, None).await?;
    println!("✅ Inserted order: {:?}", result.get("id"));

    // Insert products with wrapped types
    let products = vec![
        ("Laptop Pro", "1299.99", 15, 4.8, true),
        ("Wireless Mouse", "29.99", 150, 4.5, true),
    ];

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

        client.insert("products_example_rs", product, None).await?;
    }

    println!("✅ Inserted 2 products with wrapped types\n");
    Ok(())
}

async fn wrapped_types_in_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 2: function with Wrapped Type Parameters\n");

    let script = UserFunction::new(
        "create_order_with_types_rs",
        "Create Order with Wrapped Types (Rust)",
    )
    .with_description("Demonstrates wrapped types in script insert operations")
    .with_version("1.0")
    .with_parameter(ParameterDefinition::new("order_total").required())
    .with_function(Function::FindAll {
        collection: "products_example_rs".to_string(),
    });

    let id = client.save_function(script).await?;
    println!("✅ Function saved: {}", id);

    let mut params = HashMap::new();
    params.insert(
        "order_total".to_string(),
        FieldType::String("599.99".to_string()),
    );

    let result = client
        .call_function("create_order_with_types_rs", Some(params))
        .await?;
    println!("📊 function executed");
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
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
            "user:session:123",
            serde_json::Value::Object(session_data.into_iter().collect()),
            None,
        )
        .await?;
    println!("✅ Set session data");

    // Get the value back
    let session = client.kv_get("user:session:123").await?;
    println!("📊 Retrieved session: {:?}", session);

    // Check if key exists
    let exists = client.kv_exists("user:session:123").await?;
    println!("🔍 Key exists: {}", exists);

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
            "cache:product:456",
            serde_json::Value::Object(cache_data.into_iter().collect()),
            Some("1h"),
        )
        .await?;
    println!("✅ Set cached data");

    // Delete a key
    client.kv_delete("user:session:123").await?;
    println!("🗑️  Deleted session\n");

    Ok(())
}

async fn kv_script_operations(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
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

    let id = client.save_function(script).await?;
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
            "product:cache:789",
            serde_json::Value::Object(product_data.into_iter().collect()),
            None,
        )
        .await?;

    let mut params = HashMap::new();
    params.insert(
        "product_key".to_string(),
        FieldType::String("product:cache:789".to_string()),
    );

    let result = client
        .call_function("cached_product_lookup_rs", Some(params))
        .await?;
    println!("📊 Cached and retrieved product data");
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

// =============================================================================
// Combined Example
// =============================================================================

async fn combined_example(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
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
        key: serde_json::Value::String("order:status:{{order_id}}".to_string()),
    });

    let id = client.save_function(script).await?;
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
            "order:status:c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6",
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
    println!("📊 Processed order with caching");
    println!("⏱️  Stages executed: {}", result.stats.stages_executed);
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

// =============================================================================
// Cleanup
// =============================================================================

async fn cleanup(client: &Client, script_ids: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧹 Cleaning up...");

    for id in script_ids {
        let _ = client.delete_function(id).await;
    }

    let _ = client.delete_collection("orders_example_rs").await;
    let _ = client.delete_collection("products_example_rs").await;

    let _ = client.kv_delete("cache:product:456").await;
    let _ = client.kv_delete("product:cache:789").await;
    let _ = client
        .kv_delete("order:status:c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6")
        .await;

    println!("✅ Cleanup complete\n");
    Ok(())
}
