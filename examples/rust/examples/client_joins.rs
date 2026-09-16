//! Join Operations example - Demonstrates joining data across multiple collections
//!
//! This example shows how to use ekoDB's join functionality to query
//! related data across multiple collections, similar to SQL joins.

use ekodb_client::{
    extract_record, get_string_value, Client, Error as EkoError, FieldType, QueryBuilder, Record,
};
use serde_json::json;
use std::env;

const COLLECTIONS: [&str; 4] = [
    "joins_users_rust",
    "joins_departments_rust",
    "joins_profiles_rust",
    "joins_orders_rust",
];

fn is_not_found(error: &EkoError) -> bool {
    matches!(error, EkoError::NotFound | EkoError::Api { code: 404, .. })
}

async fn cleanup(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut errors = Vec::new();
    for collection in COLLECTIONS {
        if let Err(error) = client.delete_collection(collection).await {
            if !is_not_found(&error) {
                errors.push(format!("collection {collection}: {error}"));
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!("cleanup failed: {}", errors.join("; ")).into())
    }
}

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

    cleanup(&client).await?;

    let operation_result: Result<(), Box<dyn std::error::Error>> = async {
        let [users_collection, departments_collection, profiles_collection, orders_collection] =
            COLLECTIONS;

        // Setup: Create sample data
        println!("=== Setting up sample data ===");

        // Create departments first (need their IDs for users)
        let mut dept1 = Record::new();
        dept1.insert("name", "Engineering");
        dept1.insert("location", "Building A");
        let dept1_result = client.insert(departments_collection, dept1, None).await?;
        let dept1_id = match dept1_result.get("id") {
            Some(FieldType::String(id)) => id.clone(),
            _ => return Err("No ID returned".into()),
        };

        let mut dept2 = Record::new();
        dept2.insert("name", "Sales");
        dept2.insert("location", "Building B");
        let dept2_result = client.insert(departments_collection, dept2, None).await?;
        let dept2_id = match dept2_result.get("id") {
            Some(FieldType::String(id)) => id.clone(),
            _ => return Err("No ID returned".into()),
        };

        // Create users with actual department IDs
        let mut user1 = Record::new();
        user1.insert("name", "Alice Johnson");
        user1.insert("email", "alice@example.com");
        user1.insert("department_id", dept1_id.clone());

        let user1_result = client.insert(users_collection, user1, None).await?;
        let user1_id = match user1_result.get("id") {
            Some(FieldType::String(id)) => id.clone(),
            _ => return Err("No ID returned".into()),
        };

        let mut user2 = Record::new();
        user2.insert("name", "Bob Smith");
        user2.insert("email", "bob@example.com");
        user2.insert("department_id", dept2_id.clone());

        let user2_result = client.insert(users_collection, user2, None).await?;
        let user2_id = match user2_result.get("id") {
            Some(FieldType::String(id)) => id.clone(),
            _ => return Err("No ID returned".into()),
        };

        // Create orders
        let mut order1 = Record::new();
        order1.insert("user_id", user1_id.clone());
        order1.insert("product", "Laptop");
        order1.insert("amount", 1200);
        order1.insert("status", "completed");
        client.insert(orders_collection, order1, None).await?;

        let mut order2 = Record::new();
        order2.insert("user_id", user1_id.clone());
        order2.insert("product", "Mouse");
        order2.insert("amount", 25);
        order2.insert("status", "completed");
        client.insert(orders_collection, order2, None).await?;

        let mut order3 = Record::new();
        order3.insert("user_id", user2_id.clone());
        order3.insert("product", "Keyboard");
        order3.insert("amount", 75);
        order3.insert("status", "pending");
        client.insert(orders_collection, order3, None).await?;

        println!("✓ Sample data created\n");

        // Example 1: Simple single collection join
        println!("=== Example 1: Single collection join (users with departments) ===");

        let join1 = json!({
            "collections": [departments_collection],
            "local_field": "department_id",
            "foreign_field": "id",
            "as_field": "department"
        });

        let query1 = QueryBuilder::new().join(join1).limit(10).build();

        let results1 = client.find(users_collection, query1, None).await?;
        println!("✓ Found {} users with department data", results1.len());
        for (index, user) in results1.iter().enumerate() {
            let user_json = serde_json::to_value(user)?;
            let extracted = extract_record(&user_json);

            let has_name = get_string_value(&extracted["name"]).is_some();
            let joined_departments = extracted["department"]
                .as_array()
                .map_or(0, |departments| departments.len());

            println!(
                "  - user {}: name present={}, joined departments={}",
                index + 1,
                has_name,
                joined_departments
            );
        }
        println!();

        // Example 2: Join with filtering
        println!("=== Example 2: Join with filtering ===");

        let join2 = json!({
            "collections": [departments_collection],
            "local_field": "department_id",
            "foreign_field": "id",
            "as_field": "department"
        });

        let query2 = QueryBuilder::new()
            .eq("department_id", dept1_id.clone())
            .join(join2)
            .build();

        let results2 = client.find(users_collection, query2, None).await?;
        println!("✓ Found {} users in Engineering", results2.len());
        for (index, user) in results2.iter().enumerate() {
            let user_json = serde_json::to_value(user)?;
            let extracted = extract_record(&user_json);

            let has_name = get_string_value(&extracted["name"]).is_some();
            let has_department_location = extracted["department"]
                .as_array()
                .and_then(|departments| departments.first())
                .and_then(|department| get_string_value(&department["location"]))
                .is_some();

            println!(
                "  - user {}: name present={}, department location present={}",
                index + 1,
                has_name,
                has_department_location
            );
        }
        println!();

        // Example 3: Join with profiles
        println!("=== Example 3: Join with user profiles ===");

        // Create profiles
        let mut profile1 = Record::new();
        profile1.insert("user_id", user1_id.clone());
        profile1.insert("bio", "Senior Software Engineer");
        client.insert(profiles_collection, profile1, None).await?;

        let mut profile2 = Record::new();
        profile2.insert("user_id", user2_id.clone());
        profile2.insert("bio", "Sales Manager");
        client.insert(profiles_collection, profile2, None).await?;

        let join3 = json!({
            "collections": [profiles_collection],
            "local_field": "id",
            "foreign_field": "user_id",
            "as_field": "profile"
        });

        let query3 = QueryBuilder::new().join(join3).limit(10).build();

        let results3 = client.find(users_collection, query3, None).await?;
        println!("✓ Found {} users with profile data", results3.len());
        for (index, user) in results3.iter().enumerate() {
            let user_json = serde_json::to_value(user)?;
            let extracted = extract_record(&user_json);

            let has_name = get_string_value(&extracted["name"]).is_some();
            let has_profile_bio = extracted["profile"]
                .as_array()
                .and_then(|profiles| profiles.first())
                .and_then(|profile| get_string_value(&profile["bio"]))
                .is_some();

            println!(
                "  - user {}: name present={}, profile bio present={}",
                index + 1,
                has_name,
                has_profile_bio
            );
        }
        println!();

        // Example 4: Join orders with user data
        println!("=== Example 4: Join orders with user data ===");

        let join4 = json!({
            "collections": [users_collection],
            "local_field": "user_id",
            "foreign_field": "id",
            "as_field": "user"
        });

        let query4 = QueryBuilder::new()
            .eq("status", "completed")
            .join(join4)
            .build();

        let results4 = client.find(orders_collection, query4, None).await?;
        println!("✓ Found {} completed orders", results4.len());
        for (index, order) in results4.iter().enumerate() {
            let order_json = serde_json::to_value(order)?;
            let extracted = extract_record(&order_json);

            let has_product = get_string_value(&extracted["product"]).is_some();
            let has_amount = ekodb_client::get_int_value(&extracted["amount"]).is_some();

            // Join returns an array, get first element
            let joined_users = extracted["user"].as_array();
            let has_user_name = joined_users
                .and_then(|users| users.first())
                .and_then(|user| get_string_value(&user["name"]))
                .is_some();

            println!(
                "  - order {}: product present={}, amount present={}, joined user name present={}",
                index + 1,
                has_product,
                has_amount,
                has_user_name
            );
        }
        println!();

        // Example 5: Complex join with multiple conditions
        println!("=== Example 5: Complex join with multiple conditions ===");

        let join5 = json!({
            "collections": [departments_collection],
            "local_field": "department_id",
            "foreign_field": "id",
            "as_field": "department"
        });

        let query5 = QueryBuilder::new()
            .contains("email", "@example.com")
            .join(join5)
            .sort_asc("name")
            .limit(5)
            .build();

        let results5 = client.find(users_collection, query5, None).await?;
        println!("✓ Found {} users with example.com emails", results5.len());
        for (index, user) in results5.iter().enumerate() {
            let user_json = serde_json::to_value(user)?;
            let extracted = extract_record(&user_json);

            let has_name = get_string_value(&extracted["name"]).is_some();
            let email_matches = get_string_value(&extracted["email"])
                .is_some_and(|email| email.ends_with("@example.com"));

            let joined_departments = extracted["department"]
                .as_array()
                .map_or(0, |departments| departments.len());

            println!(
                "  - user {}: name present={}, email domain matched={}, joined departments={}",
                index + 1,
                has_name,
                email_matches,
                joined_departments
            );
        }
        println!();

        Ok(())
    }
    .await;

    println!("=== Cleanup ===");
    let cleanup_result = cleanup(&client).await;
    if cleanup_result.is_ok() {
        println!("✓ Deleted test collections");
    }
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            eprintln!("Cleanup also failed: {cleanup}");
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), Ok(())) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup),
        (Ok(()), Ok(())) => {}
    }

    println!("\n✓ Join operations example completed successfully");

    Ok(())
}
