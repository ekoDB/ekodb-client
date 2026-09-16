/**
 * Functions Example for ekoDB Go Client
 *
 * Demonstrates creating, managing, and executing scripts with the Go client.
 * Covers: FindAll, Group, Project, Count, and Function management operations.
 */

package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"strings"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

const (
	functionsCollection = "functions_users_client_go"
	activeUsersLabel    = "get_active_users_client_go"
	usersByStatusLabel  = "get_users_by_status_client_go"
	userStatsLabel      = "user_stats_client_go"
	topUsersLabel       = "top_users_client_go"
	countUsersLabel     = "count_users_client_go"
)

// saveOrUpdateScript saves a function, or — if a function with the same label
// already exists (HTTP 409) — updates it in place via PUT by label, making the
// example idempotent across repeated runs while still exercising both the
// create and update paths.
//
// It always returns the function's encrypted ID so downstream GET/UPDATE/DELETE
// calls (which require the encrypted ID, not the label) keep working: on a
// fresh create the ID comes straight from the save response; on the 409→update
// path it is recovered by looking the function up by label (the server's GET
// /api/functions/{id_or_label} accepts the label).
func saveOrUpdateScript(client *ekodb.Client, script ekodb.UserFunction) (string, error) {
	id, err := client.SaveFunction(script)
	if err == nil {
		return id, nil
	}
	var httpErr *ekodb.HTTPError
	if errors.As(err, &httpErr) && httpErr.StatusCode == 409 {
		if uerr := client.UpdateFunction(script.Label, script); uerr != nil {
			return "", uerr
		}
		fmt.Printf("ℹ️  Function '%s' already existed — updated instead\n", script.Label)
		existing, gerr := client.GetFunction(script.Label)
		if gerr != nil {
			return "", gerr
		}
		if existing.ID == nil {
			return "", fmt.Errorf("function %q has no id after update", script.Label)
		}
		return *existing.ID, nil
	}
	return "", err
}

func setupTestData(client *ekodb.Client) error {
	fmt.Println("📋 Setting up test data...")

	for i := 1; i <= 10; i++ {
		status := "inactive"
		if i%2 == 0 {
			status = "active"
		}
		record := map[string]interface{}{
			"name":   fmt.Sprintf("User %d", i),
			"age":    20 + i,
			"status": status,
			"score":  i * 10,
		}

		if _, err := client.Insert(functionsCollection, record); err != nil {
			return err
		}
	}

	fmt.Print("✅ Test data ready\n\n")
	return nil
}

func simpleQueryScript(client *ekodb.Client) (string, error) {
	fmt.Print("📝 Example 1: Simple Query Function\n\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:       activeUsersLabel,
		Name:        "Get Active Users",
		Description: strPtr("Retrieve all active users"),
		Version:     &version,
		Parameters:  map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageQuery(functionsCollection, map[string]interface{}{
				"type": "Condition",
				"content": map[string]interface{}{
					"field": "status", "operator": "Eq", "value": "active",
				},
			}, nil, nil, nil),
		},
		Tags: []string{"users", "query"},
	}

	scriptID, err := saveOrUpdateScript(client, script)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Function saved: %s\n", scriptID)

	result, err := client.CallFunction(activeUsersLabel, nil)
	if err != nil {
		return "", err
	}
	if len(result.Records) != 5 {
		return scriptID, fmt.Errorf("active-users function returned %d records, expected 5", len(result.Records))
	}
	fmt.Printf("📊 Found %d records\n", len(result.Records))
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return scriptID, nil
}

func parameterizedScript(client *ekodb.Client) error {
	fmt.Print("📝 Example 2: Parameterized Function\n\n")

	version := "1.0"
	queryStage := ekodb.StageQuery(functionsCollection, map[string]interface{}{
		"type": "Condition",
		"content": map[string]interface{}{
			"field": "status", "operator": "Eq", "value": "{{status}}",
		},
	}, nil, nil, nil)
	queryStage.Data["limit"] = "{{limit}}"
	script := ekodb.UserFunction{
		Label:   usersByStatusLabel,
		Name:    "Get Users By Status",
		Version: &version,
		Parameters: map[string]ekodb.ParameterDefinition{
			"status": {
				Required:    false,
				Default:     "active",
				Description: "Filter by user status",
			},
			"limit": {
				Required:    false,
				Default:     10,
				Description: "Maximum number of results",
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			queryStage,
		},
		Tags: []string{"users", "parameterized"},
	}

	_, err := saveOrUpdateScript(client, script)
	if err != nil {
		return err
	}
	fmt.Println("✅ Function saved")

	params := map[string]interface{}{
		"status": "active",
		"limit":  3,
	}
	result, err := client.CallFunction(usersByStatusLabel, params)
	if err != nil {
		return err
	}
	if len(result.Records) != 3 {
		return fmt.Errorf("parameterized function returned %d records, expected 3", len(result.Records))
	}
	fmt.Printf("📊 Found %d users (limited)\n", len(result.Records))
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return nil
}

func aggregationScript(client *ekodb.Client) (string, error) {
	fmt.Print("📝 Example 3: Aggregation Function\n\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:      userStatsLabel,
		Name:       "User Statistics",
		Version:    &version,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll(functionsCollection),
			ekodb.StageGroup(
				[]string{"status"},
				[]ekodb.GroupFunctionConfig{
					{
						OutputField: "count",
						Operation:   ekodb.GroupFunctionCount,
					},
					{
						OutputField: "avg_score",
						Operation:   ekodb.GroupFunctionAverage,
						InputField:  strPtr("score"),
					},
				},
			),
		},
		Tags: []string{"analytics"},
	}

	scriptID, err := saveOrUpdateScript(client, script)
	if err != nil {
		return "", err
	}
	fmt.Println("✅ Function saved")

	result, err := client.CallFunction(userStatsLabel, nil)
	if err != nil {
		return "", err
	}
	if len(result.Records) != 2 {
		return scriptID, fmt.Errorf("aggregation returned %d groups, expected 2", len(result.Records))
	}
	fmt.Printf("📊 Statistics: %d groups\n", len(result.Records))
	for _, record := range result.Records {
		fmt.Printf("   %v\n", record)
	}
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return scriptID, nil
}

func scriptManagement(client *ekodb.Client, getActiveUsersID, userStatsID string) error {
	fmt.Print("📝 Example 4: Function Management\n\n")

	// List all functions
	functions, err := client.ListFunctions(nil)
	if err != nil {
		return err
	}
	fmt.Printf("📋 Total functions: %d\n", len(functions))

	// Get specific function (use encrypted ID)
	fn, err := client.GetFunction(getActiveUsersID)
	if err != nil {
		return err
	}
	fmt.Printf("🔍 Retrieved function: %s\n", fn.Name)

	// Update function (use encrypted ID)
	updatedVersion := "1.1"
	updatedScript := ekodb.UserFunction{
		Label:       activeUsersLabel,
		Name:        "Get Active Users (Updated)",
		Description: strPtr("Updated description"),
		Version:     &updatedVersion,
		Parameters:  map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll(functionsCollection),
		},
		Tags: []string{"users"},
	}
	if err := client.UpdateFunction(getActiveUsersID, updatedScript); err != nil {
		return err
	}
	fmt.Println("✏️  Function updated")

	// Delete function (use ID) - handle error gracefully
	if err := client.DeleteFunction(userStatsID); err != nil {
		fmt.Println("ℹ️  Function delete skipped (may not exist)")
		return fmt.Errorf("delete user statistics function: %w", err)
	} else {
		fmt.Println("🗑️  Function deleted")
	}
	fmt.Println()

	fmt.Println("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
	fmt.Print("ℹ️  Only CALL can use either ID or label\n\n")

	return nil
}

func multiStageScript(client *ekodb.Client) error {
	fmt.Print("📝 Example 5: Multi-Stage Pipeline\n\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:   topUsersLabel,
		Name:    "Top Performing Users",
		Version: &version,
		Parameters: map[string]ekodb.ParameterDefinition{
			"min_score": {
				Required: false,
				Default:  50,
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll(functionsCollection),
			ekodb.StageProject([]string{"name", "score", "status"}, false),
		},
		Tags: []string{"analytics", "reporting"},
	}

	_, err := saveOrUpdateScript(client, script)
	if err != nil {
		return err
	}
	fmt.Println("✅ Multi-stage script saved")

	params := map[string]interface{}{"min_score": 50}
	result, err := client.CallFunction(topUsersLabel, params)
	if err != nil {
		return err
	}
	fmt.Printf("📊 Pipeline executed %d stages\n", result.Stats.StagesExecuted)
	fmt.Printf("⏱️  Total execution time: %dms\n", result.Stats.ExecutionTimeMs)
	fmt.Println("📈 Stage breakdown:")
	for i, stage := range result.Stats.StageStats {
		fmt.Printf("   %d. %s: %dms (%d → %d records)\n",
			i+1, stage.Stage, stage.ExecutionTimeMs, stage.InputCount, stage.OutputCount)
	}
	fmt.Println()

	return nil
}

func countScript(client *ekodb.Client) error {
	fmt.Print("📝 Example 6: Count Users\n\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:      countUsersLabel,
		Name:       "Count All Users",
		Version:    &version,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll(functionsCollection),
			ekodb.StageCount("count"),
		},
		Tags: []string{"users", "count"},
	}

	_, err := saveOrUpdateScript(client, script)
	if err != nil {
		return err
	}
	fmt.Println("✅ Count script saved")

	result, err := client.CallFunction(countUsersLabel, nil)
	if err != nil {
		return err
	}
	count := 0
	if len(result.Records) > 0 {
		if c, ok := ekodb.GetIntValue(result.Records[0]["count"]); ok {
			count = c
		}
	}
	if count != 10 {
		return fmt.Errorf("count function returned %d, expected 10", count)
	}
	fmt.Printf("📊 Total user count: %d\n", count)
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return nil
}

func cleanup(client *ekodb.Client) error {
	fmt.Println("🧹 Cleaning up...")
	var cleanupErrors []error

	if err := client.DeleteCollection(functionsCollection); err != nil && !isNotFound(err) {
		cleanupErrors = append(cleanupErrors, fmt.Errorf("delete collection: %w", err))
	}
	for _, label := range []string{activeUsersLabel, usersByStatusLabel, userStatsLabel, topUsersLabel, countUsersLabel} {
		function, err := client.GetFunction(label)
		if err != nil {
			if !isNotFound(err) {
				cleanupErrors = append(cleanupErrors, fmt.Errorf("lookup function %s: %w", label, err))
			}
			continue
		}
		if function.ID == nil {
			cleanupErrors = append(cleanupErrors, fmt.Errorf("function %s has no ID", label))
			continue
		}
		if err := client.DeleteFunction(*function.ID); err != nil && !isNotFound(err) {
			cleanupErrors = append(cleanupErrors, fmt.Errorf("delete function %s: %w", label, err))
		}
	}
	if err := errors.Join(cleanupErrors...); err != nil {
		return err
	}

	fmt.Println("✅ Deleted collection")
	fmt.Print("✅ Deleted test scripts\n\n")
	return nil
}

func isNotFound(err error) bool {
	var httpErr *ekodb.HTTPError
	return (errors.As(err, &httpErr) && httpErr.StatusCode == 404) ||
		strings.Contains(strings.ToLower(err.Error()), "not found")
}

func run() (runErr error) {
	fmt.Print("🚀 ekoDB Functions Example (Go Client)\n\n")

	// Load environment variables from .env file
	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}

	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("failed to create client: %w", err)
	}
	fmt.Print("✅ Client initialized\n\n")
	if err := cleanup(client); err != nil {
		return fmt.Errorf("initial cleanup: %w", err)
	}
	defer func() {
		if cleanupErr := cleanup(client); cleanupErr != nil {
			runErr = errors.Join(runErr, cleanupErr)
		}
	}()

	if err := setupTestData(client); err != nil {
		return err
	}

	getActiveUsersID, err := simpleQueryScript(client)
	if err != nil {
		return err
	}

	if err := parameterizedScript(client); err != nil {
		return err
	}

	userStatsID, err := aggregationScript(client)
	if err != nil {
		return err
	}

	if err := scriptManagement(client, getActiveUsersID, userStatsID); err != nil {
		return err
	}

	if err := multiStageScript(client); err != nil {
		return err
	}

	if err := countScript(client); err != nil {
		return err
	}
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatalf("❌ Error: %v", err)
	}
	fmt.Println("✅ All examples completed successfully!")
	fmt.Println("\n💡 Key Advantages of Using the Client:")
	fmt.Println("   • Automatic token management")
	fmt.Println("   • Type-safe Stage builders")
	fmt.Println("   • Built-in error handling")
}

// Helper function to create string pointers
func strPtr(s string) *string {
	return &s
}
