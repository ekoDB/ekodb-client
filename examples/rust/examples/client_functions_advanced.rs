//! Advanced Scripts Example - Query, Sort, Limit, Group
//!
//! Demonstrates advanced query and aggregation operations using simple patterns

use ekodb_client::{Client, Function, GroupFunctionConfig, GroupFunctionOp, Record, Script};
use serde_json::json;

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

    println!("🚀 ekoDB Rust Advanced Scripts Example\n");

    // Setup test data
    println!("📋 Setting up test data...");
    let _ = client.delete_collection("advanced_products_rs").await;

    let products = vec![
        json!({"name": "Laptop Pro", "category": "Electronics", "price": 1299, "stock": 15, "rating": 4.8}),
        json!({"name": "Wireless Mouse", "category": "Electronics", "price": 29, "stock": 45, "rating": 4.5}),
        json!({"name": "Desk Chair", "category": "Furniture", "price": 349, "stock": 8, "rating": 4.2}),
        json!({"name": "Standing Desk", "category": "Furniture", "price": 599, "stock": 12, "rating": 4.7}),
        json!({"name": "USB-C Cable", "category": "Electronics", "price": 19, "stock": 100, "rating": 4.3}),
        json!({"name": "Monitor 27\"", "category": "Electronics", "price": 399, "stock": 20, "rating": 4.6}),
        json!({"name": "Bookshelf", "category": "Furniture", "price": 149, "stock": 5, "rating": 4.1}),
        json!({"name": "Keyboard", "category": "Electronics", "price": 89, "stock": 30, "rating": 4.4}),
    ];

    for product in &products {
        let mut record = Record::new();
        if let Some(obj) = product.as_object() {
            for (key, value) in obj {
                record.insert(key.clone(), value.clone());
            }
        }
        client.insert("advanced_products_rs", record, None).await?;
    }
    println!("✅ Created {} products\n", products.len());

    let mut script_ids: Vec<String> = Vec::new();

    // Example 1: List All Products
    println!("📝 Example 1: List All Products\n");
    let script1 = Script::new("list_all_products_adv_rs", "List All Products")
        .with_tag("products")
        .with_tag("list")
        .with_function(Function::FindAll {
            collection: "advanced_products_rs".to_string(),
        });

    let script_id1 = client.save_script(script1).await?;
    script_ids.push(script_id1);
    println!("✅ Script saved");

    let result1 = client.call_script("list_all_products_adv_rs", None).await?;
    println!("📊 Found {} products", result1.records.len());
    println!(
        "⏱️  Execution time: {}ms\n",
        result1.stats.execution_time_ms
    );

    // Example 2: Group Products by Category
    println!("📝 Example 2: Group Products by Category\n");
    let script2 = Script::new("products_by_category_rs", "Products by Category")
        .with_tag("products")
        .with_tag("analytics")
        .with_function(Function::FindAll {
            collection: "advanced_products_rs".to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["category".to_string()],
            functions: vec![
                GroupFunctionConfig::new("count", GroupFunctionOp::Count),
                GroupFunctionConfig::new("avg_price", GroupFunctionOp::Average)
                    .with_input_field("price"),
            ],
        });

    let script_id2 = client.save_script(script2).await?;
    script_ids.push(script_id2);
    println!("✅ Script saved");

    let result2 = client.call_script("products_by_category_rs", None).await?;
    println!("📊 Category breakdown:");
    for record in &result2.records {
        println!("   {:?}", record);
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result2.stats.execution_time_ms
    );

    // Cleanup
    println!("🧹 Cleaning up...");
    for script_id in script_ids {
        let _ = client.delete_script(&script_id).await;
    }
    let _ = client.delete_collection("advanced_products_rs").await;
    println!("✅ Cleanup complete\n");

    println!("✅ All advanced script examples finished!");

    Ok(())
}
