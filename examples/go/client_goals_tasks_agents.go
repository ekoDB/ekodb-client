// ekoDB Go Client - Goals, Tasks & Agents Integration Example
//
// Exercises the full lifecycle for Goals, Tasks, and Agents:
//   GoalCreate -> GoalList -> GoalGet -> GoalUpdate -> GoalSearch -> GoalComplete -> GoalApprove
//   GoalStepStart -> GoalStepComplete -> GoalStepFail
//   GoalReject
//   TaskCreate -> TaskList -> TaskGet -> TaskStart -> TaskSucceed -> TaskPause -> TaskResume -> TaskFail -> TaskDue -> TaskDelete
//   AgentCreate -> AgentList -> AgentGet -> AgentGetByName -> AgentUpdate -> AgentsByDeployment -> AgentDelete
//   GoalDelete

package main

import (
	"fmt"
	"log"
	"os"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		log.Fatal("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	// ========================================================================
	// Goals
	// ========================================================================

	// 1. Create a goal
	fmt.Println("--- Creating goal ---")
	goal, err := client.GoalCreate(map[string]interface{}{
		"title":       "Deploy v2.0",
		"description": "Ship the v2.0 release to production",
		"steps": []map[string]interface{}{
			{"description": "Run integration tests"},
			{"description": "Build release artifacts"},
			{"description": "Deploy to staging"},
		},
	})
	if err != nil {
		log.Fatalf("GoalCreate failed: %v", err)
	}
	goalID := goal["id"].(string)
	fmt.Printf("Created goal: %s (id: %s)\n", goal["title"], goalID)

	// 2. List goals
	fmt.Println("\n--- Listing goals ---")
	goalList, err := client.GoalList()
	if err != nil {
		log.Fatalf("GoalList failed: %v", err)
	}
	fmt.Printf("Goals response: %v\n", goalList)

	// 3. Get goal by ID
	fmt.Println("\n--- Getting goal ---")
	fetched, err := client.GoalGet(goalID)
	if err != nil {
		log.Fatalf("GoalGet failed: %v", err)
	}
	fmt.Printf("Fetched goal: %s\n", fetched["title"])

	// 4. Update goal
	fmt.Println("\n--- Updating goal ---")
	updated, err := client.GoalUpdate(goalID, map[string]interface{}{
		"description": "Ship the v2.0 release to production (with hotfix)",
	})
	if err != nil {
		log.Fatalf("GoalUpdate failed: %v", err)
	}
	fmt.Printf("Updated description: %s\n", updated["description"])

	// 5. Search goals
	fmt.Println("\n--- Searching goals ---")
	searchResult, err := client.GoalSearch("Deploy")
	if err != nil {
		log.Fatalf("GoalSearch failed: %v", err)
	}
	fmt.Printf("Search result: %v\n", searchResult)

	// 6. Goal step lifecycle
	fmt.Println("\n--- Goal step lifecycle ---")
	_, err = client.GoalStepStart(goalID, 0)
	if err != nil {
		log.Fatalf("GoalStepStart failed: %v", err)
	}
	fmt.Println("Step 0 started")

	_, err = client.GoalStepComplete(goalID, 0, map[string]interface{}{"result": "All tests passed"})
	if err != nil {
		log.Fatalf("GoalStepComplete failed: %v", err)
	}
	fmt.Println("Step 0 completed")

	_, err = client.GoalStepStart(goalID, 1)
	if err != nil {
		log.Fatalf("GoalStepStart failed: %v", err)
	}
	fmt.Println("Step 1 started")

	_, err = client.GoalStepFail(goalID, 1, map[string]interface{}{"error": "Build timeout"})
	if err != nil {
		log.Fatalf("GoalStepFail failed: %v", err)
	}
	fmt.Println("Step 1 failed")

	// 7. Complete and approve the goal
	fmt.Println("\n--- Completing and approving goal ---")
	completed, err := client.GoalComplete(goalID, map[string]interface{}{"summary": "Done with caveats"})
	if err != nil {
		log.Fatalf("GoalComplete failed: %v", err)
	}
	fmt.Printf("Goal status after complete: %s\n", completed["status"])

	approved, err := client.GoalApprove(goalID)
	if err != nil {
		log.Fatalf("GoalApprove failed: %v", err)
	}
	fmt.Printf("Goal status after approve: %s\n", approved["status"])

	// 8. Create a second goal to test reject
	fmt.Println("\n--- Testing GoalReject ---")
	goal2, err := client.GoalCreate(map[string]interface{}{
		"title": "Rejected Goal",
	})
	if err != nil {
		log.Fatalf("GoalCreate (for reject) failed: %v", err)
	}
	goal2ID := goal2["id"].(string)

	_, err = client.GoalComplete(goal2ID, map[string]interface{}{"summary": "Attempt"})
	if err != nil {
		log.Fatalf("GoalComplete (for reject) failed: %v", err)
	}

	rejected, err := client.GoalReject(goal2ID, map[string]interface{}{"reason": "Incomplete work"})
	if err != nil {
		log.Fatalf("GoalReject failed: %v", err)
	}
	fmt.Printf("Goal status after reject: %s\n", rejected["status"])

	// Cleanup goal2
	_ = client.GoalDelete(goal2ID)

	// ========================================================================
	// Tasks
	// ========================================================================

	fmt.Println("\n--- Creating task ---")
	task, err := client.TaskCreate(map[string]interface{}{
		"name":        "Run benchmarks",
		"description": "Execute YCSB benchmarks against staging",
		"due_at":      time.Now().Add(24 * time.Hour).Format(time.RFC3339),
	})
	if err != nil {
		log.Fatalf("TaskCreate failed: %v", err)
	}
	taskID := task["id"].(string)
	fmt.Printf("Created task: %s (id: %s)\n", task["name"], taskID)

	fmt.Println("\n--- Listing tasks ---")
	taskList, err := client.TaskList()
	if err != nil {
		log.Fatalf("TaskList failed: %v", err)
	}
	fmt.Printf("Tasks: %v\n", taskList)

	fmt.Println("\n--- Getting task ---")
	taskFetched, err := client.TaskGet(taskID)
	if err != nil {
		log.Fatalf("TaskGet failed: %v", err)
	}
	fmt.Printf("Fetched task: %s\n", taskFetched["name"])

	fmt.Println("\n--- Task lifecycle: start -> succeed ---")
	started, err := client.TaskStart(taskID)
	if err != nil {
		log.Fatalf("TaskStart failed: %v", err)
	}
	fmt.Printf("Task status: %s\n", started["status"])

	succeeded, err := client.TaskSucceed(taskID, map[string]interface{}{"output": "Throughput: 50k ops/s"})
	if err != nil {
		log.Fatalf("TaskSucceed failed: %v", err)
	}
	fmt.Printf("Task status: %s\n", succeeded["status"])

	// Create another task for pause/resume/fail
	fmt.Println("\n--- Task lifecycle: pause -> resume -> fail ---")
	task2, err := client.TaskCreate(map[string]interface{}{
		"name": "Flaky task",
	})
	if err != nil {
		log.Fatalf("TaskCreate (task2) failed: %v", err)
	}
	task2ID := task2["id"].(string)

	_, err = client.TaskStart(task2ID)
	if err != nil {
		log.Fatalf("TaskStart (task2) failed: %v", err)
	}

	paused, err := client.TaskPause(task2ID)
	if err != nil {
		log.Fatalf("TaskPause failed: %v", err)
	}
	fmt.Printf("Task status after pause: %s\n", paused["status"])

	resumed, err := client.TaskResume(task2ID, map[string]interface{}{})
	if err != nil {
		log.Fatalf("TaskResume failed: %v", err)
	}
	fmt.Printf("Task status after resume: %s\n", resumed["status"])

	_, err = client.TaskFail(task2ID, map[string]interface{}{"error": "OOM killed"})
	if err != nil {
		log.Fatalf("TaskFail failed: %v", err)
	}
	fmt.Println("Task failed successfully")

	// Due tasks
	fmt.Println("\n--- Due tasks ---")
	dueResult, err := client.TaskDue(time.Now().Add(48 * time.Hour).Format(time.RFC3339))
	if err != nil {
		log.Fatalf("TaskDue failed: %v", err)
	}
	fmt.Printf("Due tasks: %v\n", dueResult)

	// Delete tasks
	fmt.Println("\n--- Deleting tasks ---")
	if err := client.TaskDelete(taskID); err != nil {
		log.Fatalf("TaskDelete failed: %v", err)
	}
	if err := client.TaskDelete(task2ID); err != nil {
		log.Fatalf("TaskDelete (task2) failed: %v", err)
	}
	fmt.Println("Tasks deleted")

	// ========================================================================
	// Agents
	// ========================================================================

	fmt.Println("\n--- Creating agent ---")
	agent, err := client.AgentCreate(map[string]interface{}{
		"name":          "benchmark-runner",
		"description":   "Runs periodic benchmarks",
		"deployment_id": "deploy_prod_1",
	})
	if err != nil {
		log.Fatalf("AgentCreate failed: %v", err)
	}
	agentID := agent["id"].(string)
	fmt.Printf("Created agent: %s (id: %s)\n", agent["name"], agentID)

	fmt.Println("\n--- Listing agents ---")
	agentList, err := client.AgentList()
	if err != nil {
		log.Fatalf("AgentList failed: %v", err)
	}
	fmt.Printf("Agents: %v\n", agentList)

	fmt.Println("\n--- Getting agent by ID ---")
	agentFetched, err := client.AgentGet(agentID)
	if err != nil {
		log.Fatalf("AgentGet failed: %v", err)
	}
	fmt.Printf("Fetched agent: %s\n", agentFetched["name"])

	fmt.Println("\n--- Getting agent by name ---")
	agentByName, err := client.AgentGetByName("benchmark-runner")
	if err != nil {
		log.Fatalf("AgentGetByName failed: %v", err)
	}
	fmt.Printf("Agent by name: %s\n", agentByName["name"])

	fmt.Println("\n--- Updating agent ---")
	agentUpdated, err := client.AgentUpdate(agentID, map[string]interface{}{
		"description": "Runs hourly YCSB benchmarks",
	})
	if err != nil {
		log.Fatalf("AgentUpdate failed: %v", err)
	}
	fmt.Printf("Updated agent description: %s\n", agentUpdated["description"])

	fmt.Println("\n--- Agents by deployment ---")
	deployAgents, err := client.AgentsByDeployment("deploy_prod_1")
	if err != nil {
		log.Fatalf("AgentsByDeployment failed: %v", err)
	}
	fmt.Printf("Agents in deploy_prod_1: %v\n", deployAgents)

	fmt.Println("\n--- Deleting agent ---")
	if err := client.AgentDelete(agentID); err != nil {
		log.Fatalf("AgentDelete failed: %v", err)
	}
	fmt.Println("Agent deleted")

	// ========================================================================
	// Cleanup: Delete the original goal
	// ========================================================================

	fmt.Println("\n--- Deleting goal ---")
	if err := client.GoalDelete(goalID); err != nil {
		log.Fatalf("GoalDelete failed: %v", err)
	}
	fmt.Println("Goal deleted")

	fmt.Println("\n=== All goals, tasks & agents operations completed successfully ===")
}
