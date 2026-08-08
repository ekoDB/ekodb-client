//! ekoDB Rust Client - Goals, Tasks & Agents Integration Example
//!
//! Demonstrates the full lifecycle of goals (with steps), tasks, and agents.
//!
//! Run with: `cargo run --example client_goals_tasks_agents`

use ekodb_client::Client;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB Goals, Tasks & Agents Integration Example (Rust) ===\n");

    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    // =========================================================================
    // GOALS
    // =========================================================================

    // 1. Create a goal
    println!("--- Goal: create ---");
    let goal = client
        .goal_create(json!({
            "title": "Deploy v2.0 to production",
            "description": "Full release cycle for version 2.0",
            "priority": "high",
            "steps": [
                {"description": "Run integration tests"},
                {"description": "Build release artifacts"},
                {"description": "Deploy to staging"},
                {"description": "Deploy to production"},
            ],
        }))
        .await?;
    let goal_id = goal["id"].as_str().unwrap_or_default().to_string();
    println!("Created goal: {} (id: {})", goal["title"], goal_id);

    // 2. List goals
    println!("\n--- Goal: list ---");
    let goals = client.goal_list().await?;
    println!("Goals: {}", goals);

    // 3. Get goal by ID
    println!("\n--- Goal: get ---");
    let fetched_goal = client.goal_get(&goal_id).await?;
    println!("Fetched goal: {}", fetched_goal["title"]);

    // 4. Update goal
    println!("\n--- Goal: update ---");
    let updated_goal = client
        .goal_update(
            &goal_id,
            json!({"description": "Updated: full v2.0 release with rollback plan"}),
        )
        .await?;
    println!("Updated description: {}", updated_goal["description"]);

    // 5. Search goals
    println!("\n--- Goal: search ---");
    let search_results = client.goal_search("deploy").await?;
    println!("Search results: {}", search_results);

    // 6. Goal step lifecycle
    println!("\n--- Goal step: start (step 0) ---");
    let step_started = client.goal_step_start(&goal_id, 0).await?;
    println!("Step 0 started: {}", step_started);

    println!("\n--- Goal step: complete (step 0) ---");
    let step_completed = client
        .goal_step_complete(&goal_id, 0, json!({"result": "All 247 tests passed"}))
        .await?;
    println!("Step 0 completed: {}", step_completed);

    println!("\n--- Goal step: start (step 1) ---");
    client.goal_step_start(&goal_id, 1).await?;
    println!("Step 1 started");

    println!("\n--- Goal step: fail (step 1) ---");
    let step_failed = client
        .goal_step_fail(&goal_id, 1, json!({"error": "Build timeout after 300s"}))
        .await?;
    println!("Step 1 failed: {}", step_failed);

    // 7. Complete the goal (moves to pending_review)
    println!("\n--- Goal: complete ---");
    let completed = client
        .goal_complete(
            &goal_id,
            json!({"summary": "Partial completion — step 1 failed"}),
        )
        .await?;
    println!("Goal completed (pending review): {}", completed);

    // 8. Approve the goal
    println!("\n--- Goal: approve ---");
    let approved = client.goal_approve(&goal_id).await?;
    println!("Goal approved: {}", approved);

    // 9. Create a second goal for rejection flow
    println!("\n--- Goal: create (for rejection) ---");
    let goal2 = client
        .goal_create(json!({
            "title": "Refactor auth module",
            "description": "Rewrite JWT handling",
        }))
        .await?;
    let goal2_id = goal2["id"].as_str().unwrap_or_default().to_string();
    println!("Created goal2: {} (id: {})", goal2["title"], goal2_id);

    // 10. Complete and reject goal2
    println!("\n--- Goal: complete (goal2) ---");
    client
        .goal_complete(&goal2_id, json!({"summary": "Completed refactor"}))
        .await?;

    println!("\n--- Goal: reject ---");
    let rejected = client
        .goal_reject(
            &goal2_id,
            json!({"reason": "Breaks backward compatibility"}),
        )
        .await?;
    println!("Goal rejected: {}", rejected);

    // =========================================================================
    // TASKS
    // =========================================================================

    // 11. Create a task
    println!("\n--- Task: create ---");
    let task = client
        .task_create(json!({
            "title": "Nightly backup",
            "description": "Full database backup to S3",
            "schedule": "0 2 * * *",
            "max_retries": 3,
            "due_at": "2026-03-22T02:00:00Z",
        }))
        .await?;
    let task_id = task["id"].as_str().unwrap_or_default().to_string();
    println!("Created task: {} (id: {})", task["title"], task_id);

    // 12. List tasks
    println!("\n--- Task: list ---");
    let tasks = client.task_list().await?;
    println!("Tasks: {}", tasks);

    // 13. Get task by ID
    println!("\n--- Task: get ---");
    let fetched_task = client.task_get(&task_id).await?;
    println!("Fetched task: {}", fetched_task["title"]);

    // 14. Start the task
    println!("\n--- Task: start ---");
    let started = client.task_start(&task_id).await?;
    println!("Task started: {}", started);

    // 15. Succeed the task
    println!("\n--- Task: succeed ---");
    let succeeded = client
        .task_succeed(
            &task_id,
            json!({"output": "Backup completed: 2.3 GB uploaded"}),
        )
        .await?;
    println!("Task succeeded: {}", succeeded);

    // 16. Start again, then pause
    println!("\n--- Task: start (round 2) ---");
    client.task_start(&task_id).await?;

    println!("\n--- Task: pause ---");
    let paused = client.task_pause(&task_id).await?;
    println!("Task paused: {}", paused);

    // 17. Resume the task
    println!("\n--- Task: resume ---");
    let resumed = client
        .task_resume(&task_id, json!({"reason": "Maintenance window ended"}))
        .await?;
    println!("Task resumed: {}", resumed);

    // 18. Fail the task
    println!("\n--- Task: fail ---");
    let failed = client
        .task_fail(&task_id, json!({"error": "S3 bucket access denied"}))
        .await?;
    println!("Task failed: {}", failed);

    // 19. Check due tasks
    println!("\n--- Task: due ---");
    let due_tasks = client.task_due("2026-03-22T03:00:00Z").await?;
    println!("Due tasks: {}", due_tasks);

    // 20. Delete the task
    println!("\n--- Task: delete ---");
    client.task_delete(&task_id).await?;
    println!("Task deleted");

    // =========================================================================
    // AGENTS
    // =========================================================================

    // 21. Create an agent
    println!("\n--- Agent: create ---");
    let agent = client
        .agent_create(json!({
            "name": "backup-agent",
            "description": "Handles nightly backups and restores",
            "deployment_id": "deploy-abc-123",
            "capabilities": ["backup", "restore", "verify"],
            "status": "active",
        }))
        .await?;
    let agent_id = agent["id"].as_str().unwrap_or_default().to_string();
    println!("Created agent: {} (id: {})", agent["name"], agent_id);

    // 22. List agents
    println!("\n--- Agent: list ---");
    let agents = client.agent_list().await?;
    println!("Agents: {}", agents);

    // 23. Get agent by ID
    println!("\n--- Agent: get ---");
    let fetched_agent = client.agent_get(&agent_id).await?;
    println!("Fetched agent: {}", fetched_agent["name"]);

    // 24. Get agent by name
    println!("\n--- Agent: get_by_name ---");
    let by_name = client.agent_get_by_name("backup-agent").await?;
    println!("Found by name: {}", by_name["name"]);

    // 25. Update agent
    println!("\n--- Agent: update ---");
    let updated_agent = client
        .agent_update(
            &agent_id,
            json!({"description": "Handles backups, restores, and disaster recovery"}),
        )
        .await?;
    println!(
        "Updated agent description: {}",
        updated_agent["description"]
    );

    // 26. Agents by deployment
    println!("\n--- Agent: agents_by_deployment ---");
    let deployment_agents = client.agents_by_deployment("deploy-abc-123").await?;
    println!("Agents in deployment: {}", deployment_agents);

    // 27. Delete agent
    println!("\n--- Agent: delete ---");
    client.agent_delete(&agent_id).await?;
    println!("Agent deleted");

    // =========================================================================
    // CLEANUP
    // =========================================================================

    // 28. Delete goals
    println!("\n--- Goal: delete ---");
    client.goal_delete(&goal_id).await?;
    println!("Goal 1 deleted");
    client.goal_delete(&goal2_id).await?;
    println!("Goal 2 deleted");

    println!("\n=== All goals, tasks & agents operations completed ===");
    Ok(())
}
