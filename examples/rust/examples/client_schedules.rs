//! ekoDB Rust Client - Schedule Management Example
//!
//! Demonstrates the full schedule lifecycle: create, list, get, update,
//! trigger, pause, resume, and delete. It creates a temporary no-op function
//! so the immediate trigger exercises a complete, standalone lifecycle.
//!
//! Run with: `cargo run --example client_schedules`

use ekodb_client::{Client, Function, UserFunction};
use serde_json::json;
use std::collections::HashMap;
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

    let function_label = format!("schedule_noop_rust_{}", std::process::id());
    let function_id = client
        .save_function(
            UserFunction::new(&function_label, "Schedule example no-op").with_function(
                Function::Return {
                    fields: HashMap::from([("triggered".to_string(), json!(true))]),
                    status_code: Some(200),
                },
            ),
        )
        .await?;

    let mut schedule_id = None;
    let operations: Result<(), Box<dyn Error>> = async {
        // 1. Create a schedule
        println!("--- Creating schedule ---");
        let schedule = client
            .create_schedule(json!({
                "name": "nightly-cleanup",
                "description": "Remove expired sessions and temp files",
                "function_label": function_label,
                "cron_expression": "0 0 3 * * *",
                "timezone": "UTC",
                "enabled": true,
            }))
            .await?;
        schedule_id = Some(schedule["id"].as_str().unwrap_or_default().to_string());
        let schedule_id = schedule_id.as_deref().unwrap_or_default();
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
        Ok(())
    }
    .await;

    let schedule_cleanup = match schedule_id.as_deref() {
        Some(id) => client.delete_schedule(id).await,
        None => Ok(()),
    };
    let function_cleanup = client.delete_function(&function_id).await;
    operations?;
    schedule_cleanup?;
    println!("Schedule deleted successfully");
    function_cleanup?;

    println!("\n=== All schedule operations completed ===");
    Ok(())
}
