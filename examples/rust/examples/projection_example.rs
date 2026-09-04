//! Field Projection Examples for Rust Client
//!
//! Demonstrates how to use select_fields and exclude_fields to optimize
//! data transfer and improve query performance.
//!
//! Shows comparison between full object queries and projected queries.

use ekodb_client::{Client, FieldType, QueryBuilder, Record, Result};
use std::env;

const TEST_COLLECTION: &str = "projection_test_users_rs";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("Client created\n");

    // Setup: Create test collection and insert test data
    println!("Setting up test data...");
    // Try to delete existing collection (ignore error if doesn't exist)
    let _ = client.delete_collection(TEST_COLLECTION).await;

    // Insert test users with various fields
    let test_users = vec![
        Record::new()
            .field("name", "Alice Johnson")
            .field("email", "alice@example.com")
            .field("status", "active")
            .field("user_role", "admin")
            .field("age", 30i64)
            .field("bio", "Software engineer")
            .field("avatar_url", "https://example.com/alice.jpg")
            .field("password", "secret123")
            .field("api_key", "key123")
            .field("secret_token", "token123")
            .field("created_at", chrono::Utc::now().to_rfc3339()),
        Record::new()
            .field("name", "Bob Smith")
            .field("email", "bob@example.com")
            .field("status", "active")
            .field("user_role", "user")
            .field("age", 25i64)
            .field("bio", "Designer")
            .field("avatar_url", "https://example.com/bob.jpg")
            .field("password", "secret456")
            .field("api_key", "key456")
            .field("secret_token", "token456")
            .field("created_at", chrono::Utc::now().to_rfc3339()),
        Record::new()
            .field("name", "Carol White")
            .field("email", "carol@example.com")
            .field("status", "inactive")
            .field("user_role", "user")
            .field("age", 35i64)
            .field("bio", "Manager")
            .field("avatar_url", "https://example.com/carol.jpg")
            .field("password", "secret789")
            .field("api_key", "key789")
            .field("secret_token", "token789")
            .field("created_at", chrono::Utc::now().to_rfc3339()),
        Record::new()
            .field("name", "Dave Brown")
            .field("email", "dave@example.com")
            .field("status", "active")
            .field("user_role", "admin")
            .field("age", 45i64)
            .field("bio", "CTO")
            .field("avatar_url", "https://example.com/dave.jpg")
            .field("password", "secretabc")
            .field("api_key", "keyabc")
            .field("secret_token", "tokenabc")
            .field("created_at", chrono::Utc::now().to_rfc3339()),
    ];

    let mut inserted_ids: Vec<String> = Vec::new();
    for user in test_users {
        let result = client.insert(TEST_COLLECTION, user, None).await?;
        if let Some(FieldType::String(id)) = result.get("id") {
            inserted_ids.push(id.clone());
        }
    }
    println!("Inserted {} test users\n", inserted_ids.len());

    // Example 1: Select specific fields - only get id, name, email
    println!("Example 1: Select specific fields (id, name, email only)");
    let users_query = QueryBuilder::new()
        .eq("status", "active")
        .select_fields(vec![
            "id".to_string(),
            "name".to_string(),
            "email".to_string(),
        ])
        .limit(10)
        .build();

    let users = client.find(TEST_COLLECTION, users_query, None).await?;
    println!("  Found {} active users", users.len());
    if !users.is_empty() {
        let fields: Vec<&String> = users[0].keys().collect();
        println!("  Fields returned: {:?}", fields);
        // Show first user's data
        if let Some(FieldType::String(name)) = users[0].get("name") {
            if let Some(FieldType::String(email)) = users[0].get("email") {
                println!("  First user: {} <{}>", name, email);
            }
        }
    }

    // Example 2: Exclude sensitive fields - hide password, api_key, secret_token
    println!("\nExample 2: Exclude sensitive fields (password, api_key, secret_token)");
    let admins_query = QueryBuilder::new()
        .eq("user_role", "admin")
        .exclude_fields(vec![
            "password".to_string(),
            "api_key".to_string(),
            "secret_token".to_string(),
        ])
        .build();

    let admins = client.find(TEST_COLLECTION, admins_query, None).await?;
    println!("  Found {} admins", admins.len());
    if !admins.is_empty() {
        let has_password = admins[0].get("password").is_some();
        let has_api_key = admins[0].get("api_key").is_some();
        let has_token = admins[0].get("secret_token").is_some();
        println!("  Sensitive fields excluded:");
        println!(
            "    - password: {}",
            if has_password {
                "PRESENT (unexpected!)"
            } else {
                "excluded"
            }
        );
        println!(
            "    - api_key: {}",
            if has_api_key {
                "PRESENT (unexpected!)"
            } else {
                "excluded"
            }
        );
        println!(
            "    - secret_token: {}",
            if has_token {
                "PRESENT (unexpected!)"
            } else {
                "excluded"
            }
        );
        let fields: Vec<&String> = admins[0].keys().collect();
        println!("  Fields returned: {:?}", fields);
    }

    // Example 3: Complex query with projection - active users with profile fields
    println!("\nExample 3: Complex query with projection (active users, ages 18-65)");
    let active_users_query = QueryBuilder::new()
        .eq("status", "active")
        .gte("age", 18)
        .lt("age", 65)
        .select_fields(vec![
            "id".to_string(),
            "name".to_string(),
            "email".to_string(),
            "age".to_string(),
            "created_at".to_string(),
        ])
        .sort_desc("age")
        .limit(50)
        .build();

    let active_users = client
        .find(TEST_COLLECTION, active_users_query, None)
        .await?;
    println!("  Found {} active users (ages 18-65)", active_users.len());
    for user in &active_users {
        if let (Some(FieldType::String(name)), Some(FieldType::Integer(age))) =
            (user.get("name"), user.get("age"))
        {
            println!("    - {} (age {})", name, age);
        }
    }

    // Example 4: Query with projection by status
    println!("\nExample 4: Query inactive users with profile fields");
    let inactive_query = QueryBuilder::new()
        .eq("status", "inactive")
        .select_fields(vec![
            "id".to_string(),
            "name".to_string(),
            "email".to_string(),
            "bio".to_string(),
        ])
        .build();

    let inactive_users = client.find(TEST_COLLECTION, inactive_query, None).await?;
    println!("  Found {} inactive users", inactive_users.len());
    for user in &inactive_users {
        if let (Some(FieldType::String(name)), Some(FieldType::String(bio))) =
            (user.get("name"), user.get("bio"))
        {
            println!("    - {}: {}", name, bio);
        }
    }

    // Example 5: Compare full vs projected data - demonstrates bandwidth savings
    println!("\nExample 5: Compare full vs projected data");
    let full_query = QueryBuilder::new().eq("status", "active").build();

    let projected_query = QueryBuilder::new()
        .eq("status", "active")
        .select_fields(vec![
            "id".to_string(),
            "name".to_string(),
            "email".to_string(),
        ])
        .build();

    let full_users = client.find(TEST_COLLECTION, full_query, None).await?;
    let projected_users = client.find(TEST_COLLECTION, projected_query, None).await?;

    if !full_users.is_empty() && !projected_users.is_empty() {
        let full_fields: Vec<&String> = full_users[0].keys().collect();
        let projected_fields: Vec<&String> = projected_users[0].keys().collect();

        println!("  Full query:");
        println!("    - {} fields per record", full_fields.len());
        println!("    - Fields: {:?}", full_fields);
        println!("  Projected query:");
        println!("    - {} fields per record", projected_fields.len());
        println!("    - Fields: {:?}", projected_fields);
        println!(
            "  Bandwidth savings: ~{}% fewer fields",
            100 - (projected_fields.len() * 100 / full_fields.len().max(1))
        );
    }

    // Cleanup
    println!("\nCleaning up test data...");
    client.delete_collection(TEST_COLLECTION).await?;
    println!("Cleanup complete");

    println!("\nAll projection examples completed successfully!");

    Ok(())
}
