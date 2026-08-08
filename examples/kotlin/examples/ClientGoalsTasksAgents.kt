package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray

/**
 * Goals, Tasks, and Agents integration example.
 *
 * Exercises the full lifecycle of goals (including steps), tasks, and agents.
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB Kotlin Client - Goals, Tasks & Agents Example ===\n")

    try {
        // ── Goals ────────────────────────────────────────────────────────────

        // Create a goal with steps
        println("--- Creating goal ---")
        val goal = client.goalCreate(buildJsonObject {
            put("title", "Deploy v2.0")
            put("description", "Ship the next major release")
            putJsonArray("steps") {
                add(buildJsonObject { put("description", "Run test suite") })
                add(buildJsonObject { put("description", "Build release artifacts") })
                add(buildJsonObject { put("description", "Deploy to production") })
            }
        })
        val goalId = goal["id"].toString().trim('"')
        println("Created goal: $goalId")

        // List goals
        println("\n--- Listing goals ---")
        val goalList = client.goalList()
        println("Goals: $goalList")

        // Get goal by ID
        println("\n--- Getting goal ---")
        val fetched = client.goalGet(goalId)
        println("Fetched: ${fetched["title"]}")

        // Update goal
        println("\n--- Updating goal ---")
        val updated = client.goalUpdate(goalId, buildJsonObject {
            put("description", "Ship v2.0 with full test coverage")
        })
        println("Updated description: ${updated["description"]}")

        // Search goals
        println("\n--- Searching goals ---")
        val searchResult = client.goalSearch("deploy")
        println("Search results: $searchResult")

        // Step lifecycle: start -> complete, start -> fail
        println("\n--- Goal step lifecycle ---")
        val stepStarted = client.goalStepStart(goalId, 0)
        println("Step 0 started: ${stepStarted["id"]}")

        val stepCompleted = client.goalStepComplete(goalId, 0, buildJsonObject {
            put("result", "All 253 tests passed")
        })
        println("Step 0 completed: ${stepCompleted["id"]}")

        val step1Started = client.goalStepStart(goalId, 1)
        println("Step 1 started: ${step1Started["id"]}")

        val step1Failed = client.goalStepFail(goalId, 1, buildJsonObject {
            put("error", "Build timeout after 300s")
        })
        println("Step 1 failed: ${step1Failed["id"]}")

        // Complete the goal
        println("\n--- Completing goal ---")
        val completed = client.goalComplete(goalId, buildJsonObject {
            put("summary", "Deployed successfully despite build hiccup")
        })
        println("Goal status after complete: ${completed["status"]}")

        // Approve the goal
        println("\n--- Approving goal ---")
        val approved = client.goalApprove(goalId)
        println("Goal status after approve: ${approved["status"]}")

        // Create another goal to test rejection
        println("\n--- Creating goal for rejection ---")
        val goal2 = client.goalCreate(buildJsonObject {
            put("title", "Risky migration")
        })
        val goal2Id = goal2["id"].toString().trim('"')

        val rejected = client.goalReject(goal2Id, buildJsonObject {
            put("reason", "Not enough test coverage")
        })
        println("Goal status after reject: ${rejected["status"]}")

        // ── Tasks ────────────────────────────────────────────────────────────

        println("\n--- Creating task ---")
        val task = client.taskCreate(buildJsonObject {
            put("name", "Nightly Backup")
            put("cron", "0 2 * * *")
            put("description", "Full database backup every night at 2 AM")
        })
        val taskId = task["id"].toString().trim('"')
        println("Created task: $taskId")

        println("\n--- Listing tasks ---")
        val taskList = client.taskList()
        println("Tasks: $taskList")

        println("\n--- Getting task ---")
        val fetchedTask = client.taskGet(taskId)
        println("Task name: ${fetchedTask["name"]}")

        println("\n--- Starting task ---")
        val started = client.taskStart(taskId)
        println("Task status: ${started["status"]}")

        println("\n--- Succeeding task ---")
        val succeeded = client.taskSucceed(taskId, buildJsonObject {
            put("output", "Backup completed: 42 collections, 1.2 GB")
        })
        println("Task status after succeed: ${succeeded["status"]}")

        // Create another task to test pause/resume/fail
        val task2 = client.taskCreate(buildJsonObject {
            put("name", "Index Rebuild")
            put("cron", "0 4 * * 0")
        })
        val task2Id = task2["id"].toString().trim('"')

        println("\n--- Pausing task ---")
        val paused = client.taskPause(task2Id)
        println("Task status: ${paused["status"]}")

        println("\n--- Resuming task ---")
        val resumed = client.taskResume(task2Id, buildJsonObject {})
        println("Task status: ${resumed["status"]}")

        println("\n--- Failing task ---")
        client.taskFail(task2Id, buildJsonObject {
            put("error", "Disk full")
        })
        println("Task marked as failed")

        println("\n--- Checking due tasks ---")
        val dueTasks = client.taskDue("2099-01-01T00:00:00Z")
        println("Due tasks: $dueTasks")

        println("\n--- Deleting tasks ---")
        client.taskDelete(taskId)
        client.taskDelete(task2Id)
        println("Tasks deleted")

        // ── Agents ───────────────────────────────────────────────────────────

        println("\n--- Creating agent ---")
        val agent = client.agentCreate(buildJsonObject {
            put("name", "DataBot")
            put("system_prompt", "You are a data analysis assistant.")
            put("deployment_id", "deploy_kt_example")
        })
        val agentId = agent["id"].toString().trim('"')
        println("Created agent: $agentId — ${agent["name"]}")

        println("\n--- Listing agents ---")
        val agentList = client.agentList()
        println("Agents: $agentList")

        println("\n--- Getting agent by ID ---")
        val fetchedAgent = client.agentGet(agentId)
        println("Agent: ${fetchedAgent["name"]}")

        println("\n--- Getting agent by name ---")
        val byName = client.agentGetByName("DataBot")
        println("By name: ${byName["name"]}")

        println("\n--- Updating agent ---")
        val updatedAgent = client.agentUpdate(agentId, buildJsonObject {
            put("system_prompt", "You are an expert data analyst.")
        })
        println("Updated agent system_prompt")

        println("\n--- Agents by deployment ---")
        val byDeploy = client.agentsByDeployment("deploy_kt_example")
        println("Agents for deployment: $byDeploy")

        println("\n--- Deleting agent ---")
        client.agentDelete(agentId)
        println("Agent deleted")

        // ── Cleanup goals ────────────────────────────────────────────────────

        println("\n--- Cleanup: deleting goals ---")
        client.goalDelete(goalId)
        client.goalDelete(goal2Id)
        println("Goals deleted")

    } finally {
        client.close()
        println("\n=== Example Complete ===")
    }
}
