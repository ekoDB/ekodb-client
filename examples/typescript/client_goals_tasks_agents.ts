// ekoDB TypeScript Client - Goals, Tasks, and Agents Integration Example
//
// Exercises the full lifecycle: create, list, get, update, search, complete,
// approve, reject, step operations, task scheduling, and agent management.

import { EkoDBClient } from "@ekodb/ekodb-client";

async function main() {
  console.log("=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;
  if (!apiKey) throw new Error("API_BASE_KEY environment variable is required");

  const client = new EkoDBClient(baseUrl, apiKey);

  // ========================================================================
  // GOALS
  // ========================================================================

  // 1. Create a goal
  console.log("--- Creating goal ---");
  const goal = await client.goalCreate({
    title: "Deploy v2.0",
    description: "Ship version 2.0 to production",
    steps: [
      { description: "Run test suite" },
      { description: "Build release artifacts" },
      { description: "Deploy to staging" },
      { description: "Deploy to production" },
    ],
  });
  const goalId = goal.id as string;
  console.log(`Created goal: ${goal.title} (id: ${goalId})`);

  // 2. List all goals
  console.log("\n--- Listing goals ---");
  const goalList = await client.goalList();
  console.log("Goals:", JSON.stringify(goalList, null, 2));

  // 3. Get goal by ID
  console.log("\n--- Getting goal ---");
  const fetched = await client.goalGet(goalId);
  console.log(`Fetched: ${fetched.title}`);

  // 4. Update goal
  console.log("\n--- Updating goal ---");
  const updated = await client.goalUpdate(goalId, {
    description: "Ship version 2.0 with hot-fix patches",
  });
  console.log(`Updated description: ${updated.description}`);

  // 5. Search goals
  console.log("\n--- Searching goals ---");
  const searchResults = await client.goalSearch("deploy");
  console.log("Search results:", JSON.stringify(searchResults, null, 2));

  // 6. Step operations: start, complete, fail
  console.log("\n--- Goal step: start step 0 ---");
  const stepStarted = await client.goalStepStart(goalId, 0);
  console.log(`Step 0 started on goal ${stepStarted.id}`);

  console.log("--- Goal step: complete step 0 ---");
  const stepCompleted = await client.goalStepComplete(goalId, 0, {
    result: "All 527 tests passed",
  });
  console.log(`Step 0 completed on goal ${stepCompleted.id}`);

  console.log("--- Goal step: fail step 1 ---");
  const stepFailed = await client.goalStepFail(goalId, 1, {
    error: "Build timed out after 10 minutes",
  });
  console.log(`Step 1 failed on goal ${stepFailed.id}`);

  // 7. Complete the goal
  console.log("\n--- Completing goal ---");
  const completed = await client.goalComplete(goalId, {
    summary: "Deployed successfully after retrying build step",
  });
  console.log(`Goal status: ${completed.status}`);

  // 8. Approve the goal
  console.log("--- Approving goal ---");
  const approved = await client.goalApprove(goalId);
  console.log(`Goal status after approve: ${approved.status}`);

  // 9. Create another goal to reject
  console.log("\n--- Creating goal to reject ---");
  const rejectGoal = await client.goalCreate({
    title: "Bad Plan",
    description: "This plan is flawed",
    steps: [{ description: "Do something wrong" }],
  });
  const rejectGoalId = rejectGoal.id as string;

  console.log("--- Rejecting goal ---");
  const rejected = await client.goalReject(rejectGoalId, {
    reason: "Plan does not meet requirements",
  });
  console.log(`Goal status after reject: ${rejected.status}`);

  // ========================================================================
  // TASKS
  // ========================================================================

  console.log("\n\n--- Creating task ---");
  const task = await client.taskCreate({
    name: "Hourly Health Check",
    cron: "0 * * * *",
    action: "health_check",
    config: { endpoint: "/health" },
  });
  const taskId = task.id as string;
  console.log(`Created task: ${task.name} (id: ${taskId})`);

  console.log("\n--- Listing tasks ---");
  const taskList = await client.taskList();
  console.log("Tasks:", JSON.stringify(taskList, null, 2));

  console.log("\n--- Getting task ---");
  const fetchedTask = await client.taskGet(taskId);
  console.log(`Fetched: ${fetchedTask.name}`);

  console.log("\n--- Starting task ---");
  const started = await client.taskStart(taskId);
  console.log(`Task status: ${started.status}`);

  console.log("\n--- Succeeding task ---");
  const succeeded = await client.taskSucceed(taskId, {
    output: "Health check passed: 200 OK",
  });
  console.log(`Task status after succeed: ${succeeded.status}`);

  console.log("\n--- Pausing task ---");
  const paused = await client.taskPause(taskId);
  console.log(`Task status after pause: ${paused.status}`);

  console.log("\n--- Resuming task ---");
  const resumed = await client.taskResume(taskId, {});
  console.log(`Task status after resume: ${resumed.status}`);

  console.log("\n--- Failing task ---");
  await client.taskStart(taskId);
  const failed = await client.taskFail(taskId, {
    error: "Connection timeout to /health",
  });
  console.log(`Task status after fail: ${failed.status}`);

  console.log("\n--- Getting due tasks ---");
  const dueTasks = await client.taskDue(new Date().toISOString());
  console.log("Due tasks:", JSON.stringify(dueTasks, null, 2));

  console.log("\n--- Deleting task ---");
  await client.taskDelete(taskId);
  console.log("Task deleted successfully");

  // ========================================================================
  // AGENTS
  // ========================================================================

  console.log("\n\n--- Creating agent ---");
  const agent = await client.agentCreate({
    name: "SupportBot",
    system_prompt: "You are a helpful customer support agent.",
    model: "gpt-4",
    deployment_id: "deploy_prod_1",
  });
  const agentId = agent.id as string;
  console.log(`Created agent: ${agent.name} (id: ${agentId})`);

  console.log("\n--- Listing agents ---");
  const agentList = await client.agentList();
  console.log("Agents:", JSON.stringify(agentList, null, 2));

  console.log("\n--- Getting agent by ID ---");
  const fetchedAgent = await client.agentGet(agentId);
  console.log(`Fetched: ${fetchedAgent.name}`);

  console.log("\n--- Getting agent by name ---");
  const byName = await client.agentGetByName("SupportBot");
  console.log(`By name: ${byName.name} (id: ${byName.id})`);

  console.log("\n--- Updating agent ---");
  const updatedAgent = await client.agentUpdate(agentId, {
    system_prompt: "You are an expert customer support agent with empathy.",
  });
  console.log(`Updated agent: ${updatedAgent.name}`);

  console.log("\n--- Getting agents by deployment ---");
  const byDeployment = await client.agentsByDeployment("deploy_prod_1");
  console.log("Agents in deployment:", JSON.stringify(byDeployment, null, 2));

  console.log("\n--- Deleting agent ---");
  await client.agentDelete(agentId);
  console.log("Agent deleted successfully");

  // ========================================================================
  // CLEANUP
  // ========================================================================

  console.log("\n--- Cleanup: deleting goals ---");
  await client.goalDelete(goalId);
  await client.goalDelete(rejectGoalId);
  console.log("Goals deleted successfully");

  console.log("\n=== All goals, tasks & agents operations completed ===");
}

main().catch(console.error);
