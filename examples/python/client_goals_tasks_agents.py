"""ekoDB Python Client - Goals, Tasks, and Agents Integration Example

Demonstrates the full lifecycle for goals, tasks, and agents:
  - Goal CRUD, search, step lifecycle, complete, approve, reject
  - Task CRUD, lifecycle (start, succeed, fail, pause, resume, due)
  - Agent CRUD, lookup by name and deployment
"""

import asyncio
import os
from datetime import datetime, timezone

from ekodb_client import Client


async def main():
    print("=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY")
    if not api_key:
        raise ValueError("API_BASE_KEY environment variable is required")

    client = Client.new(base_url, api_key)

    # ── Goals ──────────────────────────────────────────────────────────────

    # 1. Create a goal
    print("--- goal_create ---")
    goal = await client.goal_create(
        {
            "title": "Deploy v3",
            "description": "Deploy version 3 to production",
            "steps": [
                {"description": "Build Docker image", "status": "pending"},
                {"description": "Run migrations", "status": "pending"},
                {"description": "Smoke test", "status": "pending"},
            ],
            "priority": "high",
        }
    )
    goal_id = goal["id"]
    print(f"Created goal: {goal.get('title')} (id: {goal_id})")

    # 2. List goals
    print("\n--- goal_list ---")
    goals = await client.goal_list()
    print(f"Goals: {goals}")

    # 3. Get goal by ID
    print("\n--- goal_get ---")
    fetched_goal = await client.goal_get(goal_id)
    print(f"Fetched: {fetched_goal.get('title')}")

    # 4. Update goal
    print("\n--- goal_update ---")
    updated_goal = await client.goal_update(
        goal_id, {"description": "Deploy version 3 to production (updated)"}
    )
    print(f"Updated description: {updated_goal.get('description')}")

    # 5. Search goals
    print("\n--- goal_search ---")
    search_results = await client.goal_search("Deploy")
    print(f"Search results: {search_results}")

    # 6. Goal step lifecycle: start -> complete -> fail
    print("\n--- goal_step_start (step 0) ---")
    step_started = await client.goal_step_start(goal_id, 0)
    print(f"Step 0 started: {step_started}")

    print("\n--- goal_step_complete (step 0) ---")
    step_completed = await client.goal_step_complete(
        goal_id, 0, {"result": "Docker image built: sha256:abc123"}
    )
    print(f"Step 0 completed: {step_completed}")

    print("\n--- goal_step_fail (step 1) ---")
    step_failed = await client.goal_step_fail(
        goal_id, 1, {"error": "Migration failed: column already exists"}
    )
    print(f"Step 1 failed: {step_failed}")

    # 7. Complete the goal
    print("\n--- goal_complete ---")
    completed_goal = await client.goal_complete(
        goal_id, {"summary": "Deployed with migration workaround"}
    )
    print(f"Goal completed: {completed_goal.get('status')}")

    # 8. Approve the goal
    print("\n--- goal_approve ---")
    approved_goal = await client.goal_approve(goal_id)
    print(f"Goal approved: {approved_goal.get('status')}")

    # 9. Create a second goal to test reject
    print("\n--- goal_reject ---")
    goal2 = await client.goal_create(
        {"title": "Rejected Goal", "description": "This will be rejected"}
    )
    goal2_id = goal2["id"]
    rejected_goal = await client.goal_reject(goal2_id, {"reason": "Budget constraints"})
    print(f"Goal rejected: {rejected_goal.get('status')}")

    # ── Tasks ──────────────────────────────────────────────────────────────

    # 10. Create a task
    print("\n--- task_create ---")
    task = await client.task_create(
        {
            "name": "Nightly Backup",
            "cron": "0 2 * * *",
            "description": "Backup all databases nightly",
            "max_consecutive_failures": 3,
        }
    )
    task_id = task["id"]
    print(f"Created task: {task.get('name')} (id: {task_id})")

    # 11. List tasks
    print("\n--- task_list ---")
    tasks = await client.task_list()
    print(f"Tasks: {tasks}")

    # 12. Get task by ID
    print("\n--- task_get ---")
    fetched_task = await client.task_get(task_id)
    print(f"Fetched: {fetched_task.get('name')}")

    # 13. Start task
    print("\n--- task_start ---")
    started_task = await client.task_start(task_id)
    print(f"Task started: {started_task.get('status')}")

    # 14. Succeed task
    print("\n--- task_succeed ---")
    succeeded_task = await client.task_succeed(
        task_id, {"output": "Backup completed: 1.5GB"}
    )
    print(f"Task succeeded: {succeeded_task}")

    # 15. Start and pause task
    print("\n--- task_pause ---")
    await client.task_start(task_id)
    paused_task = await client.task_pause(task_id)
    print(f"Task paused: {paused_task.get('status')}")

    # 16. Resume task
    print("\n--- task_resume ---")
    resumed_task = await client.task_resume(
        task_id, {"next_run": "2026-03-22T02:00:00Z"}
    )
    print(f"Task resumed: {resumed_task.get('status')}")

    # 17. Start and fail task
    print("\n--- task_fail ---")
    await client.task_start(task_id)
    failed_task = await client.task_fail(task_id, {"error": "Disk full"})
    print(f"Task failed: {failed_task}")

    # 18. Task due
    print("\n--- task_due ---")
    now = datetime.now(timezone.utc).isoformat()
    due_tasks = await client.task_due(now)
    print(f"Due tasks: {due_tasks}")

    # 19. Delete task
    print("\n--- task_delete ---")
    await client.task_delete(task_id)
    print("Task deleted")

    # ── Agents ─────────────────────────────────────────────────────────────

    # 20. Create an agent
    print("\n--- agent_create ---")
    agent = await client.agent_create(
        {
            "name": "CodeReviewer",
            "system_prompt": "You review code for correctness and style.",
            "deployment_id": "deploy_test",
            "model": "gpt-4o",
            "tools": ["web_search", "file_read"],
        }
    )
    agent_id = agent["id"]
    print(f"Created agent: {agent.get('name')} (id: {agent_id})")

    # 21. List agents
    print("\n--- agent_list ---")
    agents = await client.agent_list()
    print(f"Agents: {agents}")

    # 22. Get agent by ID
    print("\n--- agent_get ---")
    fetched_agent = await client.agent_get(agent_id)
    print(f"Fetched: {fetched_agent.get('name')}")

    # 23. Get agent by name
    print("\n--- agent_get_by_name ---")
    by_name = await client.agent_get_by_name("CodeReviewer")
    print(f"By name: {by_name.get('name')}")

    # 24. Update agent
    print("\n--- agent_update ---")
    updated_agent = await client.agent_update(
        agent_id, {"system_prompt": "You review code. Be concise."}
    )
    print(f"Updated agent prompt: {updated_agent.get('system_prompt')}")

    # 25. Agents by deployment
    print("\n--- agents_by_deployment ---")
    deploy_agents = await client.agents_by_deployment("deploy_test")
    print(f"Agents in deploy_test: {deploy_agents}")

    # 26. Delete agent
    print("\n--- agent_delete ---")
    await client.agent_delete(agent_id)
    print("Agent deleted")

    # ── Cleanup ────────────────────────────────────────────────────────────

    print("\n--- cleanup ---")
    await client.goal_delete(goal_id)
    await client.goal_delete(goal2_id)
    print("Goals cleaned up")

    print("\n=== All goals/tasks/agents operations completed successfully ===")


if __name__ == "__main__":
    asyncio.run(main())
