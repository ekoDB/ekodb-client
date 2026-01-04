///! Complete Scripts Example for ekoDB Rust Client
///!
///! Demonstrates: FindAll, Group, Count, Multi-stage Pipelines
use ekodb_client::{
    Client, FieldType, Function, GroupFunctionConfig, GroupFunctionOp, ParameterDefinition, Record,
    Script,
};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    println!("🚀 ekoDB Rust Complete Scripts Example\n");
    println!("📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines\n");

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    // Setup test data
    setup_test_data(&client).await?;

    // Run examples and collect IDs for cleanup
    let mut script_ids = Vec::new();

    script_ids.push(product_stats_script(&client).await?);
    script_ids.push(list_products_script(&client).await?);
    script_ids.push(category_count_script(&client).await?);
    script_ids.push(top_rated_script(&client).await?);
    script_ids.push(script_with_parameter(&client).await?);
    script_ids.push(multi_stage_pipeline(&client).await?);

    // Cleanup
    cleanup(&client, &script_ids).await?;

    println!("✅ All complete script examples finished!");
    println!("\n💡 This example demonstrates ekoDB's Script system:");
    println!("   ✅ FindAll operations");
    println!("   ✅ Group aggregations (Count, Average)");
    println!("   ✅ Multi-stage pipelines (FindAll → Group → Count)");
    println!("   ✅ Parameter definitions");
    println!("   ✅ Script management (save, call, delete)");

    Ok(())
}

async fn setup_test_data(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Setting up complete test data...");

    let products = vec![
        ("Laptop Pro", "Electronics", 1299.0, 15, 4.8),
        ("Wireless Mouse", "Electronics", 29.0, 45, 4.5),
        ("Desk Chair", "Furniture", 349.0, 8, 4.2),
        ("Standing Desk", "Furniture", 599.0, 12, 4.7),
        ("Monitor 27\"", "Electronics", 399.0, 20, 4.6),
    ];

    for (name, category, price, stock, rating) in products {
        let mut record = Record::new();
        record.insert("name", FieldType::String(name.to_string()));
        record.insert("category", FieldType::String(category.to_string()));
        record.insert("price", FieldType::Float(price));
        record.insert("stock", FieldType::Integer(stock));
        record.insert("rating", FieldType::Float(rating));
        client.insert("complete_products_rs", record, None).await?;
    }

    println!("✅ Created 5 products\n");
    Ok(())
}

async fn product_stats_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 1: FindAll + Group (Simple Aggregation)\n");

    let script = Script::new("product_stats_rs", "Product Statistics")
        .with_tag("products")
        .with_tag("analytics")
        .with_function(Function::FindAll {
            collection: "complete_products_rs".to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["category".to_string()],
            functions: vec![
                GroupFunctionConfig::new("count", GroupFunctionOp::Count),
                GroupFunctionConfig::new("avg_price", GroupFunctionOp::Average)
                    .with_input_field("price"),
            ],
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved: {}", id);

    let result = client.call_script("product_stats_rs", None).await?;
    println!("📊 Found {} product groups", result.records.len());
    for record in &result.records {
        println!("   {:?}", record);
    }
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn list_products_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 2: Simple Product Listing\n");

    let script = Script::new("list_all_products_rs", "List All Products")
        .with_tag("products")
        .with_tag("list")
        .with_function(Function::FindAll {
            collection: "complete_products_rs".to_string(),
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved");

    let result = client.call_script("list_all_products_rs", None).await?;
    println!("📊 Found {} products", result.records.len());
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn category_count_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 3: Count by Category\n");

    let script = Script::new("count_by_category_rs", "Count Products by Category")
        .with_tag("products")
        .with_tag("analytics")
        .with_function(Function::FindAll {
            collection: "complete_products_rs".to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["category".to_string()],
            functions: vec![GroupFunctionConfig::new("count", GroupFunctionOp::Count)],
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved");

    let result = client.call_script("count_by_category_rs", None).await?;
    println!("📊 Found {} categories", result.records.len());
    for record in &result.records {
        println!("   {:?}", record);
    }
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn top_rated_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 4: High Rating Products\n");

    let script = Script::new("top_rated_products_rs", "Top Rated Products")
        .with_tag("products")
        .with_tag("quality")
        .with_function(Function::FindAll {
            collection: "complete_products_rs".to_string(),
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved");

    let result = client.call_script("top_rated_products_rs", None).await?;
    println!("📊 Found {} products", result.records.len());
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn script_with_parameter(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 5: Script with Parameter Definition\n");

    let script = Script::new("list_with_limit_rs", "List Products with Limit")
        .with_tag("products")
        .with_tag("list")
        .with_parameter(ParameterDefinition::new("max_items").with_default(FieldType::Integer(5)))
        .with_function(Function::FindAll {
            collection: "complete_products_rs".to_string(),
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved");

    let mut params = HashMap::new();
    params.insert("max_items".to_string(), FieldType::Integer(3));

    let result = client
        .call_script("list_with_limit_rs", Some(params))
        .await?;
    println!("📊 Found {} products", result.records.len());
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn multi_stage_pipeline(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)\n");

    let script = Script::new("product_summary_rs", "Product Summary Report")
        .with_tag("products")
        .with_tag("analytics")
        .with_function(Function::FindAll {
            collection: "complete_products_rs".to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["category".to_string()],
            functions: vec![
                GroupFunctionConfig::new("count", GroupFunctionOp::Count),
                GroupFunctionConfig::new("avg_price", GroupFunctionOp::Average)
                    .with_input_field("price"),
            ],
        })
        .with_function(Function::Count {
            output_field: "total_categories".to_string(),
        });

    let id = client.save_script(script).await?;
    println!("✅ Script saved");

    let result = client.call_script("product_summary_rs", None).await?;
    println!(
        "📊 Pipeline executed {} stages",
        result.stats.stages_executed
    );
    println!(
        "⏱️  Total execution time: {}ms\n",
        result.stats.execution_time_ms
    );

    Ok(id)
}

async fn cleanup(client: &Client, script_ids: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧹 Cleaning up...");

    for id in script_ids {
        let _ = client.delete_script(id).await;
    }

    let _ = client.delete_collection("complete_products_rs").await;

    println!("✅ Cleanup complete\n");
    Ok(())
}
