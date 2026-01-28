//! User Functions API Example - Using ekodb_client library
//!
//! This example demonstrates CRUD operations for User Functions.
//! User Functions are reusable sequences of Functions that can be called by Scripts.

use ekodb_client::{Client, Function, ParameterDefinition, UserFunction};
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

    println!("✓ Client created");

    // Example 1: Create a User Function
    println!("\n=== Create User Function ===");

    let user_func = UserFunction::new("get_active_users_rs", "Get Active Users")
        .with_description("Fetches all users and filters by active status")
        .with_version("1.0.0")
        .with_parameter(ParameterDefinition {
            name: "collection".to_string(),
            required: true,
            description: Some("Collection to query".to_string()),
            default: None,
        })
        .with_function(Function::FindAll {
            collection: "{{collection}}".to_string(),
        })
        .with_tag("users")
        .with_tag("query");

    match client.save_user_function(user_func.clone()).await {
        Ok(func_id) => println!("Created user function with ID: {}", func_id),
        Err(e) => println!("SaveUserFunction error: {}", e),
    }

    // Example 2: Get User Function by label
    println!("\n=== Get User Function ===");
    match client.get_user_function("get_active_users_rs").await {
        Ok(retrieved) => {
            println!("Retrieved: {} - {}", retrieved.label, retrieved.name);
            if let Some(desc) = &retrieved.description {
                println!("Description: {}", desc);
            }
        }
        Err(e) => println!("GetUserFunction error: {}", e),
    }

    // Example 3: List all User Functions
    println!("\n=== List All User Functions ===");
    match client.list_user_functions(None).await {
        Ok(all_funcs) => {
            println!("Found {} user functions:", all_funcs.len());
            for f in all_funcs {
                println!("  - {}: {}", f.label, f.name);
            }
        }
        Err(e) => println!("ListUserFunctions error: {}", e),
    }

    // Example 4: Update User Function
    println!("\n=== Update User Function ===");
    let updated_func = UserFunction::new("get_active_users_rs", "Get Active Users")
        .with_description("Fetches all users - updated description")
        .with_version("1.0.0")
        .with_parameter(ParameterDefinition {
            name: "collection".to_string(),
            required: true,
            description: Some("Collection to query".to_string()),
            default: None,
        })
        .with_function(Function::FindAll {
            collection: "{{collection}}".to_string(),
        })
        .with_tag("users")
        .with_tag("query");

    match client
        .update_user_function("get_active_users_rs", updated_func)
        .await
    {
        Ok(_) => println!("User function updated successfully"),
        Err(e) => println!("UpdateUserFunction error: {}", e),
    }

    // Example 5: Delete User Function
    println!("\n=== Delete User Function ===");
    match client.delete_user_function("get_active_users_rs").await {
        Ok(_) => println!("User function deleted successfully"),
        Err(e) => println!("DeleteUserFunction error: {}", e),
    }

    println!("\n✓ User Functions API example complete");

    Ok(())
}
