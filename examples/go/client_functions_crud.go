// CRUD Functions Example - Basic Data Operations
//
// Demonstrates basic CRUD operations using scripts:
// - FindAll queries
// - Group aggregations
// - Simple data transformations

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found, using defaults")
	}

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
		log.Fatal(err)
	}

	fmt.Println("🚀 ekoDB Go CRUD Functions Example")
	fmt.Println()

	// Setup test data
	fmt.Println("📋 Setting up test data...")
	client.DeleteCollection("crud_users_go")

	for i := 1; i <= 10; i++ {
		status := "active"
		if i%3 == 0 {
			status = "inactive"
		}
		role := "user"
		if i <= 3 {
			role = "admin"
		}
		client.Insert("crud_users_go", map[string]interface{}{
			"name":   fmt.Sprintf("User %d", i),
			"email":  fmt.Sprintf("user%d@example.com", i),
			"status": status,
			"score":  i * 10,
			"role":   role,
		})
	}
	fmt.Println("✅ Created 10 test users\n")

	var scriptIDs []string

	// Example 1: List All Users
	fmt.Println("📝 Example 1: List All Users")
	fmt.Println()

	script1 := ekodb.UserFunction{
		Label:      "list_all_users_go",
		Name:       "List All Users",
		Version:    func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("crud_users_go"),
			ekodb.StageProject([]string{"name", "email", "status"}, false),
		},
		Tags: []string{"users", "query"},
	}
	scriptID1, _ := client.SaveFunction(script1)
	scriptIDs = append(scriptIDs, scriptID1)
	fmt.Println("✅ Function saved")

	result1, _ := client.CallFunction("list_all_users_go", nil)
	if result1 != nil {
		fmt.Printf("📊 Found %d users\n", len(result1.Records))
		fmt.Printf("⏱️  Execution time: %vms\n\n", result1.Stats.ExecutionTimeMs)
	}

	// Example 2: Count Users by Status
	fmt.Println("📝 Example 2: Count Users by Status")
	fmt.Println()

	script2 := ekodb.UserFunction{
		Label:      "users_by_status_go",
		Name:       "Users by Status",
		Version:    func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("crud_users_go"),
			ekodb.StageGroup([]string{"status"}, []ekodb.GroupFunctionConfig{
				{OutputField: "count", Operation: "Count"},
			}),
		},
		Tags: []string{"users", "analytics"},
	}
	scriptID2, _ := client.SaveFunction(script2)
	scriptIDs = append(scriptIDs, scriptID2)
	fmt.Println("✅ Function saved")

	result2, _ := client.CallFunction("users_by_status_go", nil)
	if result2 != nil {
		fmt.Println("📊 User counts by status:")
		for _, record := range result2.Records {
			fmt.Printf("   %v\n", record)
		}
		fmt.Printf("⏱️  Execution time: %vms\n\n", result2.Stats.ExecutionTimeMs)
	}

	// Cleanup
	fmt.Println("🧹 Cleaning up...")
	for _, scriptID := range scriptIDs {
		client.DeleteFunction(scriptID)
	}
	client.DeleteCollection("crud_users_go")
	fmt.Println("✅ Cleanup complete")
	fmt.Println()
	fmt.Println("✅ All CRUD script examples finished!")
}
