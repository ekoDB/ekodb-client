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

    // Create departments first (need their IDs for users)
    let mut dept1 = Record::new();
    dept1.insert("name", "Engineering");
    dept1.insert("location", "Building A");
    let dept1_result = client.insert(departments_collection, dept1).await?;
    let dept1_id = match dept1_result.get("id") {
        Some(FieldType::String(id)) => id.clone(),
        _ => return Err("No ID returned".into()),
    };

    let mut dept2 = Record::new();
    dept2.insert("name", "Sales");
    dept2.insert("location", "Building B");
    let dept2_result = client.insert(departments_collection, dept2).await?;
    let dept2_id = match dept2_result.get("id") {
        Some(FieldType::String(id)) => id.clone(),
        _ => return Err("No ID returned".into()),
    };

    // Create users with actual department IDs
    let mut user1 = Record::new();
    user1.insert("name", "Alice Johnson");
    user1.insert("email", "alice@example.com");
    user1.insert("department_id", dept1_id.clone());

    let user1_result = client.insert(users_collection, user1).await?;
    let user1_id = match user1_result.get("id") {
        Some(FieldType::String(id)) => id.clone(),
        _ => return Err("No ID returned".into()),
    };

    let mut user2 = Record::new();
    user2.insert("name", "Bob Smith");
    user2.insert("email", "bob@example.com");
    user2.insert("department_id", dept2_id.clone());

    let user2_result = client.insert(users_collection, user2).await?;
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
        "collections": [departments_collection],
        "local_field": "department_id",
        "foreign_field": "id",
        "as_field": "department"
    });

    let query1 = QueryBuilder::new().join(join1).limit(10).build();

    let results1 = client.find(users_collection, query1).await?;
    println!("✓ Found {} users with department data", results1.len());
    for user in &results1 {
        let name = if let Some(FieldType::Object(fields)) = user.get("name") {
            if let Some(FieldType::String(n)) = fields.get("_field_value") {
                n.clone()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        // Join returns an array, get first element
        let dept_name = if let Some(FieldType::Array(depts)) = user.get("department") {
            if let Some(FieldType::Object(dept)) = depts.first() {
                if let Some(FieldType::Object(name_fields)) = dept.get("name") {
                    if let Some(FieldType::String(dn)) = name_fields.get("_field_value") {
                        dn.clone()
                    } else {
                        "No department".to_string()
                    }
                } else {
                    "No department".to_string()
                }
            } else {
                "No department".to_string()
            }
        } else {
            "No department".to_string()
        };

        println!("  - {}: {}", name, dept_name);
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

    let results2 = client.find(users_collection, query2).await?;
    println!("✓ Found {} users in Engineering", results2.len());
    for user in &results2 {
        let name = if let Some(FieldType::Object(fields)) = user.get("name") {
            if let Some(FieldType::String(n)) = fields.get("_field_value") {
                n.clone()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        // Join returns an array, get first element
        let location = if let Some(FieldType::Array(depts)) = user.get("department") {
            if let Some(FieldType::Object(dept)) = depts.first() {
                if let Some(FieldType::Object(loc_fields)) = dept.get("location") {
                    if let Some(FieldType::String(loc)) = loc_fields.get("_field_value") {
                        loc.clone()
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                }
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        println!("  - {}: {}", name, location);
    }
    println!();

    // Example 3: Join with profiles
    println!("=== Example 3: Join with user profiles ===");

    // Create profiles
    let mut profile1 = Record::new();
    profile1.insert("user_id", user1_id.clone());
    profile1.insert("bio", "Senior Software Engineer");
    client.insert(profiles_collection, profile1).await?;

    let mut profile2 = Record::new();
    profile2.insert("user_id", user2_id.clone());
    profile2.insert("bio", "Sales Manager");
    client.insert(profiles_collection, profile2).await?;

    let join3 = json!({
        "collections": [profiles_collection],
        "local_field": "id",
        "foreign_field": "user_id",
        "as_field": "profile"
    });

    let query3 = QueryBuilder::new().join(join3).limit(10).build();

    let results3 = client.find(users_collection, query3).await?;
    println!("✓ Found {} users with profile data", results3.len());
    for user in &results3 {
        let name = if let Some(FieldType::Object(fields)) = user.get("name") {
            if let Some(FieldType::String(n)) = fields.get("_field_value") {
                n.clone()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        // Join returns an array, get first element
        let bio = if let Some(FieldType::Array(profiles)) = user.get("profile") {
            if let Some(FieldType::Object(profile)) = profiles.first() {
                if let Some(FieldType::Object(bio_fields)) = profile.get("bio") {
                    if let Some(FieldType::String(b)) = bio_fields.get("_field_value") {
                        b.clone()
                    } else {
                        "N/A".to_string()
                    }
                } else {
                    "N/A".to_string()
                }
            } else {
                "N/A".to_string()
            }
        } else {
            "N/A".to_string()
        };

        println!("  - {}: {}", name, bio);
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

    let results4 = client.find(orders_collection, query4).await?;
    println!("✓ Found {} completed orders", results4.len());
    for order in &results4 {
        let product = if let Some(FieldType::Object(fields)) = order.get("product") {
            if let Some(FieldType::String(p)) = fields.get("_field_value") {
                p.clone()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        let amount = if let Some(FieldType::Object(fields)) = order.get("amount") {
            if let Some(FieldType::Integer(a)) = fields.get("_field_value") {
                *a
            } else {
                0
            }
        } else {
            0
        };

        // Join returns an array, get first element
        let user_name = if let Some(FieldType::Array(users)) = order.get("user") {
            if let Some(FieldType::Object(user)) = users.first() {
                if let Some(FieldType::Object(name_fields)) = user.get("name") {
                    if let Some(FieldType::String(n)) = name_fields.get("_field_value") {
                        n.clone()
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                }
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        println!("  - {} (${}) by {}", product, amount, user_name);
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

    let results5 = client.find(users_collection, query5).await?;
    println!("✓ Found {} users with example.com emails", results5.len());
    for user in &results5 {
        let name = if let Some(FieldType::Object(fields)) = user.get("name") {
            if let Some(FieldType::String(n)) = fields.get("_field_value") {
                n.clone()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        let email = if let Some(FieldType::Object(fields)) = user.get("email") {
            if let Some(FieldType::String(e)) = fields.get("_field_value") {
                e.clone()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        // Join returns an array, get first element
        let location = if let Some(FieldType::Array(depts)) = user.get("department") {
            if let Some(FieldType::Object(dept)) = depts.first() {
                if let Some(FieldType::Object(loc_fields)) = dept.get("location") {
                    if let Some(FieldType::String(loc)) = loc_fields.get("_field_value") {
                        loc.clone()
                    } else {
                        "N/A".to_string()
                    }
                } else {
                    "N/A".to_string()
                }
            } else {
                "N/A".to_string()
            }
        } else {
            "N/A".to_string()
        };

        println!("  - {} ({}): {}", name, email, location);
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
