/**
 * Functions Example for ekoDB Go Client
 *
 * Demonstrates creating, managing, and executing scripts with the Go client.
 * Covers: FindAll, Group, Project, Count, and Function management operations.
 */

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func setupTestData(client *ekodb.Client) error {
	fmt.Println("📋 Setting up test data...")

	for i := 1; i <= 10; i++ {
		record := map[string]interface{}{
			"name":   fmt.Sprintf("User %d", i),
			"age":    20 + i,
			"status": map[string]interface{}{"active": i%2 == 0, "inactive": i%2 != 0}[fmt.Sprintf("%v", i%2 == 0)],
			"score":  i * 10,
		}
		if i%2 == 0 {
			record["status"] = "active"
		} else {
			record["status"] = "inactive"
		}

		if _, err := client.Insert("users", record); err != nil {
			return err
		}
	}

	fmt.Println("✅ Test data ready\n")
	return nil
}

func simpleQueryScript(client *ekodb.Client) (string, error) {
	fmt.Println("📝 Example 1: Simple Query Function\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:       "get_active_users",
		Name:        "Get Active Users",
		Description: strPtr("Retrieve all active users"),
		Version:     &version,
		Parameters:  map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
		},
		Tags: []string{"users", "query"},
	}

	scriptID, err := client.SaveFunction(script)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Function saved: %s\n", scriptID)

	result, err := client.CallFunction("get_active_users", nil)
	if err != nil {
		return "", err
	}
	fmt.Printf("📊 Found %d records\n", len(result.Records))
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return scriptID, nil
}

func parameterizedScript(client *ekodb.Client) error {
	fmt.Println("📝 Example 2: Parameterized Function\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:   "get_users_by_status",
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
			ekodb.StageFindAll("users"),
		},
		Tags: []string{"users", "parameterized"},
	}

	_, err := client.SaveFunction(script)
	if err != nil {
		return err
	}
	fmt.Println("✅ Function saved")

	params := map[string]interface{}{
		"status": "active",
		"limit":  3,
	}
	result, err := client.CallFunction("get_users_by_status", params)
	if err != nil {
		return err
	}
	fmt.Printf("📊 Found %d users (limited)\n", len(result.Records))
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return nil
}

func aggregationScript(client *ekodb.Client) (string, error) {
	fmt.Println("📝 Example 3: Aggregation Function\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:      "user_stats",
		Name:       "User Statistics",
		Version:    &version,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
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

	scriptID, err := client.SaveFunction(script)
	if err != nil {
		return "", err
	}
	fmt.Println("✅ Function saved")

	result, err := client.CallFunction("user_stats", nil)
	if err != nil {
		return "", err
	}
	fmt.Printf("📊 Statistics: %d groups\n", len(result.Records))
	for _, record := range result.Records {
		fmt.Printf("   %v\n", record)
	}
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return scriptID, nil
}

func scriptManagement(client *ekodb.Client, getActiveUsersID, userStatsID string) error {
	fmt.Println("📝 Example 4: function Management\n")

	// List all scripts
	scripts, err := client.ListFunctions(nil)
	if err != nil {
		return err
	}
	fmt.Printf("📋 Total scripts: %d\n", len(scripts))

	// Get specific script (use encrypted ID)
	script, err := client.GetFunction(getActiveUsersID)
	if err != nil {
		return err
	}
	fmt.Printf("🔍 Retrieved script: %s\n", script.Name)

	// Update script (use encrypted ID)
	updatedVersion := "1.1"
	updatedScript := ekodb.UserFunction{
		Label:       "get_active_users",
		Name:        "Get Active Users (Updated)",
		Description: strPtr("Updated description"),
		Version:     &updatedVersion,
		Parameters:  map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
		},
		Tags: []string{"users"},
	}
	if err := client.UpdateFunction(getActiveUsersID, updatedScript); err != nil {
		return err
	}
	fmt.Println("✏️  function updated")

	// Delete script (use ID) - handle error gracefully
	if err := client.DeleteFunction(userStatsID); err != nil {
		fmt.Println("ℹ️  function delete skipped (may not exist)")
	} else {
		fmt.Println("🗑️  function deleted")
	}
	fmt.Println()

	fmt.Println("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
	fmt.Println("ℹ️  Only CALL can use either ID or label\n")

	return nil
}

func multiStageScript(client *ekodb.Client) error {
	fmt.Println("📝 Example 5: Multi-Stage Pipeline\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:   "top_users",
		Name:    "Top Performing Users",
		Version: &version,
		Parameters: map[string]ekodb.ParameterDefinition{
			"min_score": {
				Required: false,
				Default:  50,
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
			ekodb.StageProject([]string{"name", "score", "status"}, false),
		},
		Tags: []string{"analytics", "reporting"},
	}

	_, err := client.SaveFunction(script)
	if err != nil {
		return err
	}
	fmt.Println("✅ Multi-stage script saved")

	params := map[string]interface{}{"min_score": 50}
	result, err := client.CallFunction("top_users", params)
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
	fmt.Println("📝 Example 6: Count Users\n")

	version := "1.0"
	script := ekodb.UserFunction{
		Label:      "count_users",
		Name:       "Count All Users",
		Version:    &version,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
			ekodb.StageCount("count"),
		},
		Tags: []string{"users", "count"},
	}

	_, err := client.SaveFunction(script)
	if err != nil {
		return err
	}
	fmt.Println("✅ Count script saved")

	result, err := client.CallFunction("count_users", nil)
	if err != nil {
		return err
	}
	count := 0
	if len(result.Records) > 0 {
		if c, ok := result.Records[0]["count"].(float64); ok {
			count = int(c)
		} else if c, ok := result.Records[0]["count"].(int); ok {
			count = c
		}
	}
	fmt.Printf("📊 Total user count: %d\n", count)
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return nil
}

func cleanup(client *ekodb.Client) error {
	fmt.Println("🧹 Cleaning up...")

	// Delete test collection
	if err := client.DeleteCollection("users"); err != nil {
		return err
	}
	fmt.Println("✅ Deleted collection")

	// List and delete all test scripts
	scripts, err := client.ListFunctions(nil)
	if err != nil {
		return err
	}
	for _, script := range scripts {
		if len(script.Label) > 4 && (script.Label[:4] == "get_" || script.Label[:5] == "user_" ||
			script.Label[:4] == "top_" || script.Label[:6] == "count_") {
			if script.ID != nil {
				_ = client.DeleteFunction(*script.ID)
			}
		}
	}
	fmt.Println("✅ Deleted test scripts\n")

	return nil
}

func main() {
	fmt.Println("🚀 ekoDB Functions Example (Go Client)\n")

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
		log.Fatalf("❌ Failed to create client: %v", err)
	}
	fmt.Println("✅ Client initialized\n")

	if err := setupTestData(client); err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	getActiveUsersID, err := simpleQueryScript(client)
	if err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	if err := parameterizedScript(client); err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	userStatsID, err := aggregationScript(client)
	if err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	if err := scriptManagement(client, getActiveUsersID, userStatsID); err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	if err := multiStageScript(client); err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	if err := countScript(client); err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	if err := cleanup(client); err != nil {
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
