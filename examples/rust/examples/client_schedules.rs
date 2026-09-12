//! ekoDB Rust Client - Schedule Management Example
//!
//! Demonstrates the full schedule lifecycle: create, list, get, update,
//! trigger, pause, resume, and delete. The referenced function label must
//! already exist on the server.
//!
//! Run with: `cargo run --example client_schedules`

use ekodb_client::Client;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB Schedule Management Example (Rust) ===\n");

    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    // 1. Create a schedule
    println!("--- Creating schedule ---");
    let schedule = client
        .create_schedule(json!({
            "name": "nightly-cleanup",
            "description": "Remove expired sessions and temp files",
            "function_label": "nightly_cleanup",
            "cron_expression": "0 0 3 * * *",
            "timezone": "UTC",
            "enabled": true,
        }))
        .await?;
    let schedule_id = schedule["id"].as_str().unwrap_or_default().to_string();
    println!(
        "Created schedule: {} (id: {})",
        schedule["name"], schedule_id
    );

    // 2. List all schedules
    println!("\n--- Listing schedules ---");
    let schedules = client.list_schedules().await?;
    println!("Schedules: {}", schedules);

    // 3. Get schedule by ID
    println!("\n--- Getting schedule ---");
    let fetched = client.get_schedule(&schedule_id).await?;
    println!("Fetched schedule: {}", fetched["name"]);

    // 4. Update schedule
    println!("\n--- Updating schedule ---");
    let updated = client
        .update_schedule(
            &schedule_id,
            json!({
                "description": "Remove expired sessions, temp files, and orphaned uploads",
                "cron_expression": "0 0 4 * * *",
            }),
        )
        .await?;
    println!("Updated cron: {}", updated["cron_expression"]);
    println!("Updated description: {}", updated["description"]);

    // 5. Trigger the schedule immediately
    println!("\n--- Triggering schedule ---");
    let triggered = client.trigger_schedule(&schedule_id).await?;
    println!("Trigger response: {}", triggered);

    // 6. Pause schedule
    println!("\n--- Pausing schedule ---");
    let paused = client.pause_schedule(&schedule_id).await?;
    println!("Schedule paused: {}", paused);

    // 7. Resume schedule
    println!("\n--- Resuming schedule ---");
    let resumed = client.resume_schedule(&schedule_id).await?;
    println!("Schedule resumed: {}", resumed);

    // 8. Delete schedule
    println!("\n--- Deleting schedule ---");
    client.delete_schedule(&schedule_id).await?;
    println!("Schedule deleted successfully");

    println!("\n=== All schedule operations completed ===");
    Ok(())
}
