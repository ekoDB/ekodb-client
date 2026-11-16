//! Schema Management Example
//!
//! This example demonstrates basic schema management operations, including
//! updating constraints on an existing field.
//!
//! Prerequisites:
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_schema_management`

use ekodb_client::{
    Client, Error as EkoError, FieldTypeSchema, Record, Schema, SchemaConstraintUpdate,
};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

const COLLECTION: &str = "schema_client_rust";

fn is_not_found(error: &EkoError) -> bool {
    matches!(error, EkoError::NotFound | EkoError::Api { code: 404, .. })
}

async fn cleanup(client: &Client) -> Result<(), Box<dyn Error>> {
    match client.delete_collection(COLLECTION).await {
        Err(error) if is_not_found(&error) => Ok(()),
        Err(error) => Err(format!("collection {COLLECTION}: {error}").into()),
        Ok(()) => Ok(()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=== ekoDB Schema Management Example ===\n");

    // Create client
    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    cleanup(&client).await?;

    let operation_result: Result<(), Box<dyn Error>> = async {
        let collection = COLLECTION;

        // Step 1: Create a collection with a simple schema
        println!("=== Creating Collection with Schema ===");

        let schema = Schema::new()
            .add_field("title", FieldTypeSchema::new("String").required())
            .add_field("email", FieldTypeSchema::new("String").required())
            .add_field("age", FieldTypeSchema::new("Integer"))
            .add_field("status", FieldTypeSchema::new("String"));

        client.create_collection(collection, schema).await?;
        println!("✓ Created collection '{}' with schema\n", collection);

        // Step 2: Insert documents that conform to schema
        println!("=== Inserting Valid Documents ===");

        let mut user1 = Record::new();
        user1.insert("title", "Software Engineer");
        user1.insert("email", "alice@example.com");
        user1.insert("age", 28);
        user1.insert("status", "active");

        let inserted = client.insert(collection, user1, None).await?;
        println!("✓ Inserted user 1: {:?}", inserted.get("id"));

        let mut user2 = Record::new();
        user2.insert("title", "Product Manager");
        user2.insert("email", "bob@example.com");
        user2.insert("age", 32);
        user2.insert("status", "pending");

        let inserted = client.insert(collection, user2, None).await?;
        println!("✓ Inserted user 2: {:?}\n", inserted.get("id"));

        // Step 3: Get schema
        println!("=== Getting Schema ===");

        let schema = client.get_schema(collection).await?;
        println!("✓ Schema for {}:", collection);
        for (name, field) in &schema.fields {
            println!("  - {}: {}", name, field.field_type);
            if field.required {
                println!("    (required)");
            }
        }
        println!();

        // Step 4: List all collections
        println!("=== Listing Collections ===");

        let collections = client.list_collections().await?;
        println!("✓ Total collections: {}", collections.len());
        println!(
            "  Sample: {:?}",
            collections.iter().take(5).collect::<Vec<_>>()
        );
        println!();

        // Step 5: Update constraints on an existing field. Current servers
        // replace a named field's constraints rather than merging them, so
        // each update sends unique, required and enums together; max, min and
        // regex left out are cleared, and the default is kept.
        println!("=== Updating Constraints on an Existing Field ===");
        let status_update = |enums: Vec<serde_json::Value>| SchemaConstraintUpdate {
            unique: Some(false),
            required: Some(true),
            enums: Some(enums),
            ..Default::default()
        };
        client
            .update_schema_constraints(
                collection,
                HashMap::from([(
                    "status".to_string(),
                    status_update(vec![json!("active"), json!("pending")]),
                )]),
            )
            .await?;
        let narrowed = client.get_schema(collection).await?;
        let status = narrowed
            .fields
            .get("status")
            .ok_or("schema has no status field after the update")?;
        println!(
            "✓ status: required = {}, enums = {:?}",
            status.required, status.enums
        );
        if !status.required || status.enums.len() != 2 {
            return Err(format!("constraint update not applied: {status:?}").into());
        }

        // An empty enums list clears the field's enums.
        client
            .update_schema_constraints(
                collection,
                HashMap::from([("status".to_string(), status_update(vec![]))]),
            )
            .await?;
        let cleared = client.get_schema(collection).await?;
        let status = cleared
            .fields
            .get("status")
            .ok_or("schema has no status field after clearing")?;
        println!("✓ status after clearing: enums = {:?}\n", status.enums);
        if !status.enums.is_empty() {
            return Err(format!("enums were not cleared: {status:?}").into());
        }

        Ok(())
    }
    .await;

    println!("=== Cleanup ===");
    let cleanup_result = cleanup(&client).await;
    if cleanup_result.is_ok() {
        println!("✓ Deleted collection\n");
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

    println!("✓ All schema management operations completed successfully");

    Ok(())
}
