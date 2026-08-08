//! Distinct Values Example
//!
//! Demonstrates how to retrieve all unique values for a specific field in a
//! collection, with and without filters.
//!
//! Prerequisites:
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_distinct_values`

use ekodb_client::{Client, DistinctValuesQuery, Record};
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB Distinct Values Example ===\n");

    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let collection = "distinct_values_example_rust";

    // Cleanup from previous runs
    let _ = client.delete_collection(collection).await;

    // -------------------------------------------------------------------------
    // Step 1: Insert sample data
    // -------------------------------------------------------------------------
    println!("=== Inserting Sample Products ===");

    let products = vec![
        ("Widget A", "electronics", "active"),
        ("Widget B", "electronics", "active"),
        ("Gadget X", "electronics", "discontinued"),
        ("Book 1", "books", "active"),
        ("Book 2", "books", "active"),
        ("Book 3", "books", "archived"),
        ("Shirt M", "clothing", "active"),
        ("Shirt L", "clothing", "active"),
    ];

    for (name, category, status) in &products {
        let mut record = Record::new();
        record.insert("name", *name);
        record.insert("category", *category);
        record.insert("status", *status);
        client.insert(collection, record, None).await?;
    }
    println!("Inserted {} products\n", products.len());

    // -------------------------------------------------------------------------
    // Step 2: Get all distinct categories (no filter)
    // -------------------------------------------------------------------------
    println!("=== Distinct Categories (all products) ===");
    let resp = client
        .distinct_values(collection, "category", DistinctValuesQuery::new())
        .await?;
    println!("Found {} distinct categories:", resp.count);
    for v in &resp.values {
        println!("  - {}", v);
    }
    println!();

    // -------------------------------------------------------------------------
    // Step 3: Get distinct statuses (no filter)
    // -------------------------------------------------------------------------
    println!("=== Distinct Statuses (all products) ===");
    let resp = client
        .distinct_values(collection, "status", DistinctValuesQuery::new())
        .await?;
    println!("Found {} distinct statuses:", resp.count);
    for v in &resp.values {
        println!("  - {}", v);
    }
    println!();

    // -------------------------------------------------------------------------
    // Step 4: Distinct statuses within the electronics category only
    // -------------------------------------------------------------------------
    println!("=== Distinct Statuses in Electronics ===");
    let filter = json!({
        "type": "Condition",
        "content": {
            "field": "category",
            "operator": "Eq",
            "value": "electronics"
        }
    });
    let resp = client
        .distinct_values(
            collection,
            "status",
            DistinctValuesQuery::new().filter(filter),
        )
        .await?;
    println!("Found {} distinct statuses for electronics:", resp.count);
    for v in &resp.values {
        println!("  - {}", v);
    }
    println!();

    // -------------------------------------------------------------------------
    // Cleanup
    // -------------------------------------------------------------------------
    client.delete_collection(collection).await?;
    println!("Cleanup done.");

    Ok(())
}
