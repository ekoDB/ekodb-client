///! Complete Functions Example for ekoDB Rust Client
///!
///! Demonstrates: FindAll, Group, Count, Multi-stage Pipelines
use ekodb_client::{
    Client, FieldType, Function, GroupFunctionConfig, GroupFunctionOp, ParameterDefinition,
    QueryConditionOperator, QueryExpression, Record, UserFunction,
};
use std::{
    collections::{HashMap, HashSet},
    env,
};

const FUNCTION_LABELS: [&str; 6] = [
    "product_stats_rs",
    "list_all_products_rs",
    "count_by_category_rs",
    "top_rated_products_rs",
    "list_with_limit_rs",
    "product_summary_rs",
];

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

    println!("🚀 ekoDB Rust Complete Functions Example\n");
    println!("📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines\n");

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    let mut script_ids = Vec::new();
    let operation_result = async {
        setup_test_data(&client).await?;
        script_ids.push(product_stats_script(&client).await?);
        script_ids.push(list_products_script(&client).await?);
        script_ids.push(category_count_script(&client).await?);
        script_ids.push(top_rated_script(&client).await?);
        script_ids.push(script_with_parameter(&client).await?);
        script_ids.push(multi_stage_pipeline(&client).await?);
        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;

    let cleanup_result = cleanup(&client, &script_ids).await;
    operation_result?;
    cleanup_result?;

    println!("✅ All complete script examples finished!");
    println!("\n💡 This example demonstrates ekoDB's function system:");
    println!("   ✅ FindAll operations");
    println!("   ✅ Group aggregations (Count, Average)");
    println!("   ✅ Multi-stage pipelines (FindAll → Group → Count)");
    println!("   ✅ Parameter definitions");
    println!("   ✅ Function management (save, call, delete)");

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

    let script = UserFunction::new("product_stats_rs", "Product Statistics")
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

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved: {}", id);

    let result = client.call_function("product_stats_rs", None).await?;
    println!("📊 Found {} product groups", result.records.len());
    for record in &result.records {
        println!("   {:?}", record);
    }
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn list_products_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 2: Simple Product Listing\n");

    let script = UserFunction::new("list_all_products_rs", "List All Products")
        .with_tag("products")
        .with_tag("list")
        .with_function(Function::FindAll {
            collection: "complete_products_rs".to_string(),
        });

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved");

    let result = client.call_function("list_all_products_rs", None).await?;
    println!("📊 Found {} products", result.records.len());
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn category_count_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 3: Count by Category\n");

    let script = UserFunction::new("count_by_category_rs", "Count Products by Category")
        .with_tag("products")
        .with_tag("analytics")
        .with_function(Function::FindAll {
            collection: "complete_products_rs".to_string(),
        })
        .with_function(Function::Group {
            by_fields: vec!["category".to_string()],
            functions: vec![GroupFunctionConfig::new("count", GroupFunctionOp::Count)],
        });

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved");

    let result = client.call_function("count_by_category_rs", None).await?;
    println!("📊 Found {} categories", result.records.len());
    for record in &result.records {
        println!("   {:?}", record);
    }
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn top_rated_script(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 4: High Rating Products\n");

    let script = UserFunction::new("top_rated_products_rs", "Top Rated Products")
        .with_tag("products")
        .with_tag("quality")
        .with_function(Function::Query {
            collection: "complete_products_rs".to_string(),
            filter: Some(QueryExpression::condition(
                "rating",
                QueryConditionOperator::Gte,
                serde_json::json!(4.6),
            )),
            sort: None,
            limit: None,
            skip: None,
        });

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved");

    let result = client.call_function("top_rated_products_rs", None).await?;
    if result.records.len() != 3 {
        return Err(format!(
            "top-rated query returned {} products; expected 3",
            result.records.len()
        )
        .into());
    }
    println!("📊 Found {} products", result.records.len());
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn script_with_parameter(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 5: function with Parameter Definition\n");

    let script = UserFunction::new("list_with_limit_rs", "List Products with Limit")
        .with_tag("products")
        .with_tag("list")
        .with_parameter(ParameterDefinition::new("max_items").with_default(FieldType::Integer(5)))
        .with_function(Function::Query {
            collection: "complete_products_rs".to_string(),
            filter: None,
            sort: None,
            limit: Some(serde_json::json!("{{max_items}}")),
            skip: None,
        });

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved");

    let mut params = HashMap::new();
    params.insert("max_items".to_string(), FieldType::Integer(3));

    let result = client
        .call_function("list_with_limit_rs", Some(params))
        .await?;
    if result.records.len() != 3 {
        return Err(format!(
            "parameterized limit returned {} products; expected 3",
            result.records.len()
        )
        .into());
    }
    println!("📊 Found {} products", result.records.len());
    println!("⏱️  Execution time: {}ms\n", result.stats.execution_time_ms);

    Ok(id)
}

async fn multi_stage_pipeline(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)\n");

    let script = UserFunction::new("product_summary_rs", "Product Summary Report")
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

    let id = save_or_update(client, script).await?;
    println!("✅ Function saved");

    let result = client.call_function("product_summary_rs", None).await?;
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

    let mut errors = Vec::new();
    let mut ids: HashSet<String> = script_ids.iter().cloned().collect();
    match client.list_functions(None).await {
        Ok(functions) => {
            for function in functions {
                if FUNCTION_LABELS.contains(&function.label.as_str()) {
                    if let Some(id) = function.id {
                        ids.insert(id);
                    }
                }
            }
        }
        Err(error) => errors.push(format!("list functions for cleanup: {error}")),
    }
    for id in ids {
        if let Err(error) = client.delete_function(&id).await {
            errors.push(format!("function {id}: {error}"));
        }
    }

    if let Err(error) = client.delete_collection("complete_products_rs").await {
        errors.push(format!("collection complete_products_rs: {error}"));
    }

    if !errors.is_empty() {
        return Err(format!("cleanup failed: {}", errors.join("; ")).into());
    }

    println!("✅ Cleanup complete\n");
    Ok(())
}
