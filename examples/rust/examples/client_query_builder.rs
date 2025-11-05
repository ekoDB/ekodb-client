//! Query Builder Example
//!
//! This example demonstrates the QueryBuilder for constructing complex queries
//! with operators, sorting, pagination, and joins.
//!
//! Prerequisites:
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_query_builder`

use ekodb_client::{Client, QueryBuilder, Record};
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=== ekoDB Query Builder Example ===\n");

    // Create client
    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let collection = "query_builder_client_rust";

    // Step 1: Insert sample data
    println!("=== Inserting Sample Data ===");

    let users = vec![
        ("Alice", 25, "alice@example.com", "active", 1500),
        ("Bob", 30, "bob@example.com", "active", 2000),
        ("Charlie", 35, "charlie@example.com", "inactive", 1000),
        ("David", 28, "david@example.com", "active", 1800),
        ("Eve", 32, "eve@example.com", "pending", 1200),
    ];

    let user_count = users.len();
    for (name, age, email, status, score) in users {
        let mut user = Record::new();
        user.insert("name", name);
        user.insert("age", age);
        user.insert("email", email);
        user.insert("status", status);
        user.insert("score", score);
        client.insert(collection, user, None).await?;
    }
    println!("✓ Inserted {} users\n", user_count);

    // Step 2: Simple equality query
    println!("=== Simple Equality Query ===");
    let query = QueryBuilder::new().eq("status", "active").build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Found {} active users", results.len());
    for user in &results {
        println!("  - {:?}", user.get("name"));
    }
    println!();

    // Step 3: Range query
    println!("=== Range Query (age >= 28 AND age < 35) ===");
    let query = QueryBuilder::new().gte("age", 28).lt("age", 35).build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Found {} users in age range", results.len());
    for user in &results {
        println!("  - {:?}", user.get("name"));
    }
    println!();

    // Step 4: IN operator
    println!("=== IN Operator ===");
    let query = QueryBuilder::new()
        .in_array("status", vec![json!("active"), json!("pending")])
        .build();

    let results = client.find(collection, query, None).await?;
    println!(
        "✓ Found {} users with status active or pending",
        results.len()
    );
    for user in &results {
        println!("  - {:?}", user.get("name"));
    }
    println!();

    // Step 5: NOT IN operator
    println!("=== NOT IN Operator ===");
    let query = QueryBuilder::new()
        .nin("status", vec![json!("inactive")])
        .build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Found {} users not inactive", results.len());
    println!();

    // Step 6: String pattern matching
    println!("=== String Pattern Matching ===");
    let query = QueryBuilder::new()
        .contains("email", "@example.com")
        .build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Found {} users with @example.com email", results.len());
    println!();

    // Step 7: Regex query
    println!("=== Regex Query ===");
    let query = QueryBuilder::new()
        .regex("name", "^[A-C]") // Names starting with A, B, or C
        .build();

    let results = client.find(collection, query, None).await?;
    println!(
        "✓ Found {} users with names starting with A-C",
        results.len()
    );
    for user in &results {
        println!("  - {:?}", user.get("name"));
    }
    println!();

    // Step 8: Complex query with multiple conditions
    println!("=== Complex Query (active AND age >= 28 AND score > 1500) ===");
    let query = QueryBuilder::new()
        .eq("status", "active")
        .gte("age", 28)
        .gt("score", 1500)
        .build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Found {} users matching all conditions", results.len());
    for user in &results {
        println!("  - {:?}", user.get("name"));
    }
    println!();

    // Step 9: OR query
    println!("=== OR Query ===");
    let query = QueryBuilder::new()
        .or(vec![
            json!({
                "type": "Condition",
                "content": {
                    "field": "age",
                    "operator": "Lt",
                    "value": 28
                }
            }),
            json!({
                "type": "Condition",
                "content": {
                    "field": "age",
                    "operator": "Gt",
                    "value": 32
                }
            }),
        ])
        .build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Found {} users with age < 28 OR age > 32", results.len());
    for user in &results {
        println!("  - {:?}", user.get("name"));
    }
    println!();

    // Step 10: Sorting
    println!("=== Sorted Query (by score descending) ===");
    let query = QueryBuilder::new().sort_desc("score").limit(3).build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Top 3 users by score:");
    for (i, user) in results.iter().enumerate() {
        println!("  {}. {:?}", i + 1, user.get("name"));
    }
    println!();

    // Step 11: Pagination
    println!("=== Pagination (page 2, size 2) ===");
    let query = QueryBuilder::new()
        .sort_asc("name")
        .page(1, 2) // Page 1 (0-indexed), 2 items per page
        .build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Page 2 results:");
    for user in &results {
        println!("  - {:?}", user.get("name"));
    }
    println!();

    // Step 12: NOT operator
    println!("=== NOT Operator ===");
    let query = QueryBuilder::new()
        .not(json!({
            "type": "Condition",
            "content": {
                "field": "status",
                "operator": "Eq",
                "value": "inactive"
            }
        }))
        .build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Found {} users NOT inactive", results.len());
    println!();

    // Step 13: Complex chained query
    println!("=== Complex Chained Query ===");
    let query = QueryBuilder::new()
        .eq("status", "active")
        .gte("age", 25)
        .contains("email", "@example.com")
        .sort_desc("score")
        .limit(5)
        .bypass_cache(true)
        .build();

    let results = client.find(collection, query, None).await?;
    println!("✓ Found {} users with all conditions", results.len());
    println!();

    // Cleanup: Delete the collection
    println!("=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection\n");

    println!("✓ All query builder operations completed successfully");

    Ok(())
}
