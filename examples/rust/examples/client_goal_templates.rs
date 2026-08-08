//! ekoDB Rust Client - Goal Template CRUD Example
//!
//! Demonstrates creating, listing, getting, updating, and deleting goal templates.
//!
//! Run with: `cargo run --example client_goal_templates`

use ekodb_client::Client;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB Goal Template CRUD Example (Rust) ===\n");

    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    // 1. Create a goal template
    println!("--- Creating goal template ---");
    let template = client
        .goal_template_create(json!({
            "title": "Data Migration",
            "description": "Template for migrating data between schemas",
            "steps": [
                {"description": "Analyze source schema"},
                {"description": "Create target schema"},
                {"description": "Migrate records"},
                {"description": "Validate results"},
            ],
        }))
        .await?;
    let template_id = template["id"].as_str().unwrap_or_default();
    println!(
        "Created template: {} (id: {})",
        template["title"], template_id
    );

    // 2. List all templates
    println!("\n--- Listing templates ---");
    let list = client.goal_template_list().await?;
    println!("Templates: {}", list);

    // 3. Get template by ID
    println!("\n--- Getting template ---");
    let fetched = client.goal_template_get(template_id).await?;
    println!("Fetched: {}", fetched["title"]);

    // 4. Update template
    println!("\n--- Updating template ---");
    let updated = client
        .goal_template_update(
            template_id,
            json!({"description": "Updated: comprehensive data migration workflow"}),
        )
        .await?;
    println!("Updated description: {}", updated["description"]);

    // 5. Delete template
    println!("\n--- Deleting template ---");
    client.goal_template_delete(template_id).await?;
    println!("Template deleted successfully");

    println!("\n✓ Goal template CRUD example completed");
    Ok(())
}
