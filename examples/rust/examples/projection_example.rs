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

    // Example 1: Select specific fields
    println!("Example 1: Select specific fields");
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
    println!("Fetched {} users with only 3 fields each", users.len());

    // Example 2: Exclude sensitive fields
    println!("\nExample 2: Exclude sensitive fields");
    let admins_query = QueryBuilder::new()
        .eq("user_role", "admin")
        .exclude_fields(vec![
            "password".to_string(),
            "api_key".to_string(),
            "secret_token".to_string(),
        ])
        .build();

    let admins = client.find(TEST_COLLECTION, admins_query, None).await?;
    println!("Fetched {} admins without sensitive data", admins.len());
    // Verify sensitive fields are excluded
    if !admins.is_empty() {
        let has_password = admins[0].get("password").is_some();
        println!("  Password field excluded: {}", !has_password);
    }

    // Example 3: Complex query with projection
    println!("\nExample 3: Complex query with projection");
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
        .sort_desc("created_at")
        .limit(50)
        .build();

    let active_users = client
        .find(TEST_COLLECTION, active_users_query, None)
        .await?;
    println!(
        "Fetched {} active users with profile fields",
        active_users.len()
    );

    // Example 4: Find by ID with projection
    println!("\nExample 4: Find by ID with projection");
    if !inserted_ids.is_empty() {
        let profile_query = QueryBuilder::new()
            .eq("id", inserted_ids[0].as_str())
            .select_fields(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
                "bio".to_string(),
                "avatar_url".to_string(),
            ])
            .build();

        let user_profiles = client.find(TEST_COLLECTION, profile_query, None).await?;
        if !user_profiles.is_empty() {
            let name = match user_profiles[0].get("name") {
                Some(FieldType::String(s)) => s.as_str(),
                _ => "N/A",
            };
            println!("Fetched user profile: {}", name);
        } else {
            println!("Fetched user profile: N/A");
        }
    }

    // Example 5: Compare full vs projected data
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

    let full_fields = if !full_users.is_empty() {
        full_users[0].len()
    } else {
        0
    };
    let projected_fields = if !projected_users.is_empty() {
        projected_users[0].len()
    } else {
        0
    };

    println!("Full query returned {} fields per user", full_fields);
    println!(
        "Projected query returned {} fields per user",
        projected_fields
    );

    // Cleanup
    println!("\nCleaning up test data...");
    client.delete_collection(TEST_COLLECTION).await?;
    println!("Cleanup complete");

    println!("\nAll projection examples completed successfully!");

    Ok(())
}
