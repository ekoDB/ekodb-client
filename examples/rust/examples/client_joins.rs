//! Join Operations example - Demonstrates joining data across multiple collections
//!
//! This example shows how to use ekoDB's join functionality to query
//! related data across multiple collections, similar to SQL joins.

use ekodb_client::{Client, FieldType, QueryBuilder, Record};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Create ekoDB client
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("=== ekoDB Rust Client - Join Operations Example ===\n");

    // Use unique collection names
    let users_collection = "joins_users_rust";
    let departments_collection = "joins_departments_rust";
    let profiles_collection = "joins_profiles_rust";
    let orders_collection = "joins_orders_rust";

    // Cleanup any existing test collections
    let _ = client.delete_collection(users_collection).await;
    let _ = client.delete_collection(departments_collection).await;
    let _ = client.delete_collection(profiles_collection).await;
    let _ = client.delete_collection(orders_collection).await;

    // Setup: Create sample data
    println!("=== Setting up sample data ===");

    // Create users
    let mut user1 = Record::new();
    user1.insert("name", "Alice Johnson");
    user1.insert("email", "alice@example.com");
    user1.insert("department_id", "dept-001");

    let user1_result = client.insert(users_collection, user1).await?;
    let user1_id = match user1_result.get("id") {
        Some(FieldType::String(id)) => id.clone(),
        _ => return Err("No ID returned".into()),
    };

    let mut user2 = Record::new();
    user2.insert("name", "Bob Smith");
    user2.insert("email", "bob@example.com");
    user2.insert("department_id", "dept-002");

    let user2_result = client.insert(users_collection, user2).await?;
    let user2_id = match user2_result.get("id") {
        Some(FieldType::String(id)) => id.clone(),
        _ => return Err("No ID returned".into()),
    };

    // Create departments
    let mut dept1 = Record::new();
    dept1.insert("id", "dept-001");
    dept1.insert("name", "Engineering");
    dept1.insert("location", "Building A");
    client.insert(departments_collection, dept1).await?;

    let mut dept2 = Record::new();
    dept2.insert("id", "dept-002");
    dept2.insert("name", "Sales");
    dept2.insert("location", "Building B");
    client.insert(departments_collection, dept2).await?;

    // Create orders
    let mut order1 = Record::new();
    order1.insert("user_id", user1_id.clone());
    order1.insert("product", "Laptop");
    order1.insert("amount", 1200);
    order1.insert("status", "completed");
    client.insert(orders_collection, order1).await?;

    let mut order2 = Record::new();
    order2.insert("user_id", user1_id.clone());
    order2.insert("product", "Mouse");
    order2.insert("amount", 25);
    order2.insert("status", "completed");
    client.insert(orders_collection, order2).await?;

    let mut order3 = Record::new();
    order3.insert("user_id", user2_id.clone());
    order3.insert("product", "Keyboard");
    order3.insert("amount", 75);
    order3.insert("status", "pending");
    client.insert(orders_collection, order3).await?;

    println!("✓ Sample data created\n");

    // Example 1: Simple single collection join
    println!("=== Example 1: Single collection join (users with departments) ===");

    let join1 = json!({
        "collection": departments_collection,
        "local_field": "department_id",
        "foreign_field": "id",
        "as": "department"
    });

    let query1 = QueryBuilder::new().join(join1).limit(10).build();

    let results1 = client.find(users_collection, query1).await?;
    println!("✓ Found {} users with department data", results1.len());
    for user in &results1 {
        if let Some(FieldType::Object(fields)) = user.get("name") {
            if let Some(FieldType::String(name)) = fields.get("value") {
                println!("  - {}", name);
            }
        }
    }
    println!();

    // Example 2: Join with filtering
    println!("=== Example 2: Join with filtering ===");

    let join2 = json!({
        "collection": departments_collection,
        "local_field": "department_id",
        "foreign_field": "id",
        "as": "department"
    });

    let query2 = QueryBuilder::new()
        .eq("department_id", "dept-001")
        .join(join2)
        .build();

    let results2 = client.find(users_collection, query2).await?;
    println!("✓ Found {} users in Engineering", results2.len());
    for user in &results2 {
        if let Some(FieldType::Object(fields)) = user.get("name") {
            if let Some(FieldType::String(name)) = fields.get("value") {
                println!("  - {}", name);
            }
        }
    }
    println!();

    // Example 3: Join with profiles
    println!("=== Example 3: Join with user profiles ===");

    // Create profiles
    let mut profile1 = Record::new();
    profile1.insert("id", user1_id.clone());
    profile1.insert("bio", "Senior Software Engineer");
    client.insert(profiles_collection, profile1).await?;

    let mut profile2 = Record::new();
    profile2.insert("id", user2_id.clone());
    profile2.insert("bio", "Sales Manager");
    client.insert(profiles_collection, profile2).await?;

    let join3 = json!({
        "collection": departments_collection,
        "local_field": "department_id",
        "foreign_field": "id",
        "as": "department"
    });

    let query3 = QueryBuilder::new().join(join3).limit(10).build();

    let results3 = client.find(users_collection, query3).await?;
    println!("✓ Found {} users with department join", results3.len());
    for user in &results3 {
        if let Some(FieldType::Object(fields)) = user.get("name") {
            if let Some(FieldType::String(name)) = fields.get("value") {
                println!("  - {}", name);
            }
        }
    }
    println!();

    // Example 4: Join orders with user data
    println!("=== Example 4: Join orders with user data ===");

    let join4 = json!({
        "collection": users_collection,
        "local_field": "user_id",
        "foreign_field": "id",
        "as": "user"
    });

    let query4 = QueryBuilder::new()
        .eq("status", "completed")
        .join(join4)
        .build();

    let results4 = client.find(orders_collection, query4).await?;
    println!("✓ Found {} completed orders", results4.len());
    for order in &results4 {
        if let Some(FieldType::Object(product_fields)) = order.get("product") {
            if let Some(FieldType::String(product)) = product_fields.get("value") {
                if let Some(FieldType::Object(amount_fields)) = order.get("amount") {
                    if let Some(FieldType::Integer(amount)) = amount_fields.get("value") {
                        println!("  - {} (${})", product, amount);
                    }
                }
            }
        }
    }
    println!();

    // Example 5: Complex join with multiple conditions
    println!("=== Example 5: Complex join with multiple conditions ===");

    let join5 = json!({
        "collection": departments_collection,
        "local_field": "department_id",
        "foreign_field": "id",
        "as": "department"
    });

    let query5 = QueryBuilder::new()
        .contains("email", "@example.com")
        .join(join5)
        .sort_asc("name")
        .limit(5)
        .build();

    let results5 = client.find(users_collection, query5).await?;
    println!("✓ Found {} users with example.com emails", results5.len());
    for user in &results5 {
        if let Some(FieldType::Object(name_fields)) = user.get("name") {
            if let Some(FieldType::String(name)) = name_fields.get("value") {
                if let Some(FieldType::Object(email_fields)) = user.get("email") {
                    if let Some(FieldType::String(email)) = email_fields.get("value") {
                        println!("  - {} ({})", name, email);
                    }
                }
            }
        }
    }
    println!();

    // Cleanup
    println!("=== Cleanup ===");
    client.delete_collection(users_collection).await?;
    client.delete_collection(departments_collection).await?;
    client.delete_collection(profiles_collection).await?;
    client.delete_collection(orders_collection).await?;
    println!("✓ Deleted test collections");

    println!("\n✓ Join operations example completed successfully");

    Ok(())
}
