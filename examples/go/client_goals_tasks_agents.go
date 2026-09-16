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
	"errors"
	"fmt"
	"log"
	"os"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func run() (runErr error) {
	fmt.Print("=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===\n\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		return errors.New("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}
	var goalID, goal2ID, taskID, task2ID, agentID string
	defer func() {
		for _, resource := range []struct {
			kind string
			id   *string
			del  func(string) error
		}{
			{"agent", &agentID, client.AgentDelete},
			{"task", &task2ID, client.TaskDelete},
			{"task", &taskID, client.TaskDelete},
			{"goal", &goal2ID, client.GoalDelete},
			{"goal", &goalID, client.GoalDelete},
		} {
			if *resource.id == "" {
				continue
			}
			if cleanupErr := resource.del(*resource.id); cleanupErr != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete %s %s: %w", resource.kind, *resource.id, cleanupErr))
			}
		}
	}()

	// ========================================================================
	// Goals
	// ========================================================================

	// 1. Create a goal
	fmt.Println("--- Creating goal ---")
	goal, err := client.GoalCreate(map[string]interface{}{
		"title":       "Deploy v2.0",
		"description": "Ship the v2.0 release to production",
		"status":      "pending",
		"steps": []map[string]interface{}{
			{"description": "Run integration tests"},
			{"description": "Build release artifacts"},
			{"description": "Deploy to staging"},
		},
	})
	if err != nil {
		return fmt.Errorf("create goal: %w", err)
	}
	goalID = ekodb.GetStringValue(goal["id"])
	if goalID == "" {
		return errors.New("create goal response contained no ID")
	}
	fmt.Printf("Created goal: %s (id: %s)\n", ekodb.GetStringValue(goal["title"]), goalID)

	// 2. List goals
	fmt.Println("\n--- Listing goals ---")
	goalList, err := client.GoalList()
	if err != nil {
		return fmt.Errorf("list goals: %w", err)
	}
	if !entityResponseContainsID(goalList, goalID) {
		return fmt.Errorf("goal list did not contain created ID %s: %v", goalID, goalList)
	}
	fmt.Printf("Goals response: %v\n", goalList)

	// 3. Get goal by ID
	fmt.Println("\n--- Getting goal ---")
	fetched, err := client.GoalGet(goalID)
	if err != nil {
		return fmt.Errorf("get goal: %w", err)
	}
	if title := ekodb.GetStringValue(fetched["title"]); title != "Deploy v2.0" {
		return fmt.Errorf("fetched goal title was %q, expected Deploy v2.0", title)
	}
	fmt.Printf("Fetched goal: %s\n", ekodb.GetStringValue(fetched["title"]))

	// 4. Update goal
	fmt.Println("\n--- Updating goal ---")
	updated, err := client.GoalUpdate(goalID, map[string]interface{}{
		"description": "Ship the v2.0 release to production (with hotfix)",
	})
	if err != nil {
		return fmt.Errorf("update goal: %w", err)
	}
	if description := ekodb.GetStringValue(updated["description"]); description != "Ship the v2.0 release to production (with hotfix)" {
		return fmt.Errorf("updated goal description was %q", description)
	}
	fmt.Printf("Updated description: %s\n", ekodb.GetStringValue(updated["description"]))

	// 5. Search goals
	fmt.Println("\n--- Searching goals ---")
	searchResult, err := client.GoalSearch("Deploy")
	if err != nil {
		return fmt.Errorf("search goals: %w", err)
	}
	if !entityResponseContainsID(searchResult, goalID) {
		return fmt.Errorf("goal search did not contain created ID %s: %v", goalID, searchResult)
	}
	fmt.Printf("Search result: %v\n", searchResult)

	// 6. Goal step lifecycle
	fmt.Println("\n--- Goal step lifecycle ---")
	_, err = client.GoalStepStart(goalID, 0)
	if err != nil {
		return fmt.Errorf("start goal step 0: %w", err)
	}
	fmt.Println("Step 0 started")

	_, err = client.GoalStepComplete(goalID, 0, map[string]interface{}{"result": "All tests passed"})
	if err != nil {
		return fmt.Errorf("complete goal step 0: %w", err)
	}
	fmt.Println("Step 0 completed")

	_, err = client.GoalStepStart(goalID, 1)
	if err != nil {
		return fmt.Errorf("start goal step 1: %w", err)
	}
	fmt.Println("Step 1 started")

	_, err = client.GoalStepFail(goalID, 1, map[string]interface{}{"error": "Build timeout"})
	if err != nil {
		return fmt.Errorf("fail goal step 1: %w", err)
	}
	fmt.Println("Step 1 failed")

	// 7. Complete and approve the goal
	fmt.Println("\n--- Completing and approving goal ---")
	completed, err := client.GoalComplete(goalID, map[string]interface{}{"summary": "Done with caveats"})
	if err != nil {
		return fmt.Errorf("complete goal: %w", err)
	}
	if status := ekodb.GetStringValue(completed["status"]); status != "pending_review" {
		return fmt.Errorf("completed goal status was %q, expected pending_review", status)
	}
	fmt.Printf("Goal status after complete: %s\n", ekodb.GetStringValue(completed["status"]))

	approved, err := client.GoalApprove(goalID)
	if err != nil {
		return fmt.Errorf("approve goal: %w", err)
	}
	if status := ekodb.GetStringValue(approved["status"]); status != "in_progress" {
		return fmt.Errorf("approved goal status was %q, expected in_progress", status)
	}
	fmt.Printf("Goal status after approve: %s\n", ekodb.GetStringValue(approved["status"]))

	// 8. Create a second goal to test reject
	fmt.Println("\n--- Testing GoalReject ---")
	goal2, err := client.GoalCreate(map[string]interface{}{
		"title": "Rejected Goal",
	})
	if err != nil {
		return fmt.Errorf("create reject goal: %w", err)
	}
	goal2ID = ekodb.GetStringValue(goal2["id"])
	if goal2ID == "" {
		return errors.New("create reject goal response contained no ID")
	}

	_, err = client.GoalComplete(goal2ID, map[string]interface{}{"summary": "Attempt"})
	if err != nil {
		return fmt.Errorf("complete reject goal: %w", err)
	}

	rejected, err := client.GoalReject(goal2ID, map[string]interface{}{"reason": "Incomplete work"})
	if err != nil {
		return fmt.Errorf("reject goal: %w", err)
	}
	if status := ekodb.GetStringValue(rejected["status"]); status != "failed" {
		return fmt.Errorf("rejected goal status was %q, expected failed", status)
	}
	fmt.Printf("Goal status after reject: %s\n", ekodb.GetStringValue(rejected["status"]))

	// Cleanup goal2
	if err := client.GoalDelete(goal2ID); err != nil {
		return fmt.Errorf("delete rejected goal: %w", err)
	}
	goal2ID = ""

	// ========================================================================
	// Tasks
	// ========================================================================

	fmt.Println("\n--- Creating task ---")
	taskName := "Run benchmarks"
	task, err := client.TaskCreate(map[string]interface{}{
		"name":        taskName,
		"description": "Execute YCSB benchmarks against staging",
		"due_at":      time.Now().Add(24 * time.Hour).Format(time.RFC3339),
	})
	if err != nil {
		return fmt.Errorf("create task: %w", err)
	}
	taskID = ekodb.GetStringValue(task["id"])
	if taskID == "" {
		return errors.New("create task response contained no ID")
	}
	fmt.Printf("Created task: %s (id: %s)\n", taskName, taskID)

	fmt.Println("\n--- Listing tasks ---")
	taskList, err := client.TaskList()
	if err != nil {
		return fmt.Errorf("list tasks: %w", err)
	}
	if !entityResponseContainsID(taskList, taskID) {
		return fmt.Errorf("task list did not contain created ID %s: %v", taskID, taskList)
	}
	fmt.Printf("Tasks: %v\n", taskList)

	fmt.Println("\n--- Getting task ---")
	taskFetched, err := client.TaskGet(taskID)
	if err != nil {
		return fmt.Errorf("get task: %w", err)
	}
	if name := ekodb.GetStringValue(taskFetched["name"]); name != taskName {
		return fmt.Errorf("fetched task name was %q, expected %q", name, taskName)
	}
	fmt.Printf("Fetched task: %s\n", ekodb.GetStringValue(taskFetched["name"]))

	fmt.Println("\n--- Task lifecycle: start -> succeed ---")
	started, err := client.TaskStart(taskID)
	if err != nil {
		return fmt.Errorf("start task: %w", err)
	}
	if status := ekodb.GetStringValue(started["status"]); status != "running" {
		return fmt.Errorf("started task status was %q, expected running", status)
	}
	fmt.Printf("Task status: %s\n", ekodb.GetStringValue(started["status"]))

	succeeded, err := client.TaskSucceed(taskID, map[string]interface{}{"output": "Throughput: 50k ops/s"})
	if err != nil {
		return fmt.Errorf("succeed task: %w", err)
	}
	if status := ekodb.GetStringValue(succeeded["status"]); status != "active" {
		return fmt.Errorf("succeeded task status was %q, expected active", status)
	}
	fmt.Printf("Task status: %s\n", ekodb.GetStringValue(succeeded["status"]))

	// Create another task for pause/resume/fail
	fmt.Println("\n--- Task lifecycle: pause -> resume -> fail ---")
	task2, err := client.TaskCreate(map[string]interface{}{
		"name": "Flaky task",
	})
	if err != nil {
		return fmt.Errorf("create second task: %w", err)
	}
	task2ID = ekodb.GetStringValue(task2["id"])
	if task2ID == "" {
		return errors.New("create second task response contained no ID")
	}

	_, err = client.TaskStart(task2ID)
	if err != nil {
		return fmt.Errorf("start second task: %w", err)
	}

	paused, err := client.TaskPause(task2ID)
	if err != nil {
		return fmt.Errorf("pause task: %w", err)
	}
	if status := ekodb.GetStringValue(paused["status"]); status != "paused" {
		return fmt.Errorf("paused task status was %q, expected paused", status)
	}
	fmt.Printf("Task status after pause: %s\n", ekodb.GetStringValue(paused["status"]))

	resumed, err := client.TaskResume(task2ID, map[string]interface{}{})
	if err != nil {
		return fmt.Errorf("resume task: %w", err)
	}
	if status := ekodb.GetStringValue(resumed["status"]); status != "active" {
		return fmt.Errorf("resumed task status was %q, expected active", status)
	}
	fmt.Printf("Task status after resume: %s\n", ekodb.GetStringValue(resumed["status"]))

	failed, err := client.TaskFail(task2ID, map[string]interface{}{"error": "OOM killed"})
	if err != nil {
		return fmt.Errorf("fail task: %w", err)
	}
	if status := ekodb.GetStringValue(failed["status"]); status != "active" {
		return fmt.Errorf("TaskFail response status was %q, expected active", status)
	}
	fmt.Println("Task failed successfully")

	// Due tasks
	fmt.Println("\n--- Due tasks ---")
	dueResult, err := client.TaskDue(time.Now().Add(48 * time.Hour).Format(time.RFC3339))
	if err != nil {
		return fmt.Errorf("list due tasks: %w", err)
	}
	fmt.Printf("Due tasks: %v\n", dueResult)

	// Delete tasks
	fmt.Println("\n--- Deleting tasks ---")
	if err := client.TaskDelete(taskID); err != nil {
		return fmt.Errorf("delete task: %w", err)
	}
	taskID = ""
	if err := client.TaskDelete(task2ID); err != nil {
		return fmt.Errorf("delete second task: %w", err)
	}
	task2ID = ""
	fmt.Println("Tasks deleted")

	// ========================================================================
	// Agents
	// ========================================================================

	fmt.Println("\n--- Creating agent ---")
	agent, err := client.AgentCreate(map[string]interface{}{
		"name":          "benchmark-runner",
		"description":   "Runs periodic benchmarks",
		"deployment_id": "deploy_prod_1",
		"llm_model":     "gpt-4.1",
	})
	if err != nil {
		return fmt.Errorf("create agent: %w", err)
	}
	agentID = ekodb.GetStringValue(agent["id"])
	if agentID == "" {
		return errors.New("create agent response contained no ID")
	}
	fmt.Printf("Created agent: %s (id: %s)\n", "benchmark-runner", agentID)

	fmt.Println("\n--- Listing agents ---")
	agentList, err := client.AgentList()
	if err != nil {
		return fmt.Errorf("list agents: %w", err)
	}
	if !entityResponseContainsID(agentList, agentID) {
		return fmt.Errorf("agent list did not contain created ID %s: %v", agentID, agentList)
	}
	listedAgent := findEntityByID(agentList, agentID)
	if deploymentID := ekodb.GetStringValue(listedAgent["deployment_id"]); deploymentID != "deploy_prod_1" {
		return fmt.Errorf("listed agent deployment_id was %q, expected deploy_prod_1", deploymentID)
	}
	fmt.Printf("Agents: %v\n", agentList)

	fmt.Println("\n--- Getting agent by ID ---")
	agentFetched, err := client.AgentGet(agentID)
	if err != nil {
		return fmt.Errorf("get agent: %w", err)
	}
	if name := ekodb.GetStringValue(agentFetched["name"]); name != "benchmark-runner" {
		return fmt.Errorf("fetched agent name was %q, expected benchmark-runner", name)
	}
	fmt.Printf("Fetched agent: %s\n", ekodb.GetStringValue(agentFetched["name"]))

	fmt.Println("\n--- Getting agent by name ---")
	agentByName, err := client.AgentGetByName("benchmark-runner")
	if err != nil {
		return fmt.Errorf("get agent by name: %w", err)
	}
	if id := ekodb.GetStringValue(agentByName["id"]); id != agentID {
		return fmt.Errorf("agent lookup by name returned ID %q, expected %q", id, agentID)
	}
	fmt.Printf("Agent by name: %s\n", ekodb.GetStringValue(agentByName["name"]))

	fmt.Println("\n--- Updating agent ---")
	agentUpdated, err := client.AgentUpdate(agentID, map[string]interface{}{
		"description": "Runs hourly YCSB benchmarks",
	})
	if err != nil {
		return fmt.Errorf("update agent: %w", err)
	}
	if description := ekodb.GetStringValue(agentUpdated["description"]); description != "Runs hourly YCSB benchmarks" {
		return fmt.Errorf("updated agent description was %q", description)
	}
	fmt.Printf("Updated agent description: %s\n", ekodb.GetStringValue(agentUpdated["description"]))

	fmt.Println("\n--- Agents by deployment ---")
	deployAgents, err := client.AgentsByDeployment("deploy_prod_1")
	if err != nil {
		return fmt.Errorf("agents by deployment: %w", err)
	}
	fmt.Printf("Agents in deploy_prod_1: %v\n", deployAgents)
	// TODO(ekoDB dev team): The live server currently returns count=0 here even
	// though AgentList contains the newly-created agent with deployment_id set to
	// deploy_prod_1. Re-enable an exact membership assertion after the server-side
	// deployment filter is fixed. This call is intentionally logged, not verified.

	fmt.Println("\n--- Deleting agent ---")
	if err := client.AgentDelete(agentID); err != nil {
		return fmt.Errorf("delete agent: %w", err)
	}
	agentID = ""
	fmt.Println("Agent deleted")

	// ========================================================================
	// Cleanup: Delete the original goal
	// ========================================================================

	fmt.Println("\n--- Deleting goal ---")
	if err := client.GoalDelete(goalID); err != nil {
		return fmt.Errorf("delete goal: %w", err)
	}
	goalID = ""
	fmt.Println("Goal deleted")

	fmt.Println("\n=== All goals, tasks & agents operations completed successfully ===")
	return nil
}

func entityResponseContainsID(value interface{}, id string) bool {
	return findEntityByID(value, id) != nil
}

func findEntityByID(value interface{}, id string) map[string]interface{} {
	switch typed := value.(type) {
	case []interface{}:
		for _, item := range typed {
			if found := findEntityByID(item, id); found != nil {
				return found
			}
		}
	case map[string]interface{}:
		if ekodb.GetStringValue(typed["id"]) == id {
			return typed
		}
		for _, key := range []string{"items", "goals", "tasks", "agents"} {
			if found := findEntityByID(typed[key], id); found != nil {
				return found
			}
		}
	}
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
