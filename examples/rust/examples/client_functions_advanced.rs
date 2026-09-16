//! Advanced Functions Example - Query, Sort, Limit, Group
//!
//! Demonstrates advanced query and aggregation operations using simple patterns

use ekodb_client::{
    Client, Error, FieldType, Function, GroupFunctionConfig, GroupFunctionOp, Record, UserFunction,
};
use serde_json::json;
use std::error::Error as StdError;

fn is_not_found(error: &Error) -> bool {
    matches!(error, Error::NotFound | Error::Api { code: 404, .. })
}

fn number(record: &Record, key: &str) -> Result<f64, Box<dyn StdError>> {
    let field = match record.get(key) {
        Some(FieldType::Object(wrapper)) => wrapper.get("value"),
        value => value,
    };
    match field {
        Some(FieldType::Integer(value)) => Ok(*value as f64),
        Some(FieldType::Float(value)) => Ok(*value),
        value => Err(format!("field '{key}' was not numeric: {value:?}").into()),
    }
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

async fn run_examples(
    client: &Client,
    script_ids: &mut Vec<String>,
) -> Result<(), Box<dyn StdError>> {
    println!("🚀 ekoDB Rust Advanced Functions Example\n");

    // Setup test data
    println!("📋 Setting up test data...");
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

    let mut inserted_ids = Vec::new();
    for product in &products {
        let mut record = Record::new();
        if let Some(obj) = product.as_object() {
            for (key, value) in obj {
                record.insert(key.clone(), value.clone());
            }
        }
        let inserted = client.insert("advanced_products_rs", record, None).await?;
        let id = inserted
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("product insert did not return an ID")?;
        if inserted_ids.iter().any(|existing| existing == id) {
            return Err(format!("duplicate inserted product ID: {id}").into());
        }
        inserted_ids.push(id.to_string());
    }
    println!("✅ Created {} products\n", products.len());

    // Example 1: List All Products
    println!("📝 Example 1: List All Products\n");
    let script1 = UserFunction::new("list_all_products_adv_rs", "List All Products")
        .with_tag("products")
        .with_tag("list")
        .with_function(Function::FindAll {
            collection: "advanced_products_rs".to_string(),
        });

    let script_id1 = save_or_update(client, script1).await?;
    script_ids.push(script_id1);
    println!("✅ Function saved");

    let result1 = client
        .call_function("list_all_products_adv_rs", None)
        .await?;
    println!("📊 Found {} products", result1.records.len());
    if result1.records.len() != 8 {
        return Err(format!("expected 8 products, found {}", result1.records.len()).into());
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result1.stats.execution_time_ms
    );

    // Example 2: Group Products by Category
    println!("📝 Example 2: Group Products by Category\n");
    let script2 = UserFunction::new("products_by_category_rs", "Products by Category")
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

    let script_id2 = save_or_update(client, script2).await?;
    script_ids.push(script_id2);
    println!("✅ Function saved");

    let result2 = client
        .call_function("products_by_category_rs", None)
        .await?;
    println!("📊 Category breakdown:");
    let mut electronics = None;
    let mut furniture = None;
    for record in &result2.records {
        println!("   {:?}", record);
        let aggregate = (
            number(record, "count")? as i64,
            number(record, "avg_price")?,
        );
        match record.get_string("category") {
            Some("Electronics") => electronics = Some(aggregate),
            Some("Furniture") => furniture = Some(aggregate),
            value => return Err(format!("unexpected category group: {value:?}").into()),
        }
    }
    if result2.records.len() != 2
        || electronics != Some((5, 367.0))
        || furniture
            .map(|(count, average)| count != 3 || (average - 1097.0 / 3.0).abs() > 1e-9)
            .unwrap_or(true)
    {
        return Err(format!(
            "unexpected aggregates: Electronics={electronics:?}, Furniture={furniture:?}"
        )
        .into());
    }
    println!(
        "⏱️  Execution time: {}ms\n",
        result2.stats.execution_time_ms
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    dotenv::dotenv().ok();
    let base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        std::env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;
    let collection = "advanced_products_rs";
    if let Err(error) = client.delete_collection(collection).await {
        if !is_not_found(&error) {
            return Err(error.into());
        }
    }
    let mut script_ids = Vec::new();
    let operation_result = run_examples(&client, &mut script_ids).await;
    println!("🧹 Cleaning up...");
    let mut cleanup_errors = Vec::new();
    for script_id in script_ids.iter().rev() {
        if let Err(error) = client.delete_function(script_id).await {
            cleanup_errors.push(format!("function {script_id}: {error}"));
        }
    }
    if let Err(error) = client.delete_collection(collection).await {
        if !is_not_found(&error) {
            cleanup_errors.push(format!("collection {collection}: {error}"));
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
    println!("✅ Cleanup complete\n");
    println!("✅ All advanced script examples finished!");
    Ok(())
}
