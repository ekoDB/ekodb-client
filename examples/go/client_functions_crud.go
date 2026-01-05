// CRUD Scripts Example - Basic Data Operations
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

	fmt.Println("🚀 ekoDB Go CRUD Scripts Example")
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

	script1 := map[string]interface{}{
		"label":      "list_all_users_go",
		"name":       "List All Users",
		"version":    "1.0",
		"parameters": map[string]interface{}{},
		"functions":  []interface{}{map[string]interface{}{"type": "FindAll", "collection": "crud_users_go"}},
		"tags":       []string{"users", "list"},
	}
	scriptID1, _ := client.SaveScript(script1)
	scriptIDs = append(scriptIDs, scriptID1)
	fmt.Println("✅ Script saved")

	result1, _ := client.CallScript("list_all_users_go", nil)
	if records, ok := result1["records"].([]interface{}); ok {
		fmt.Printf("📊 Found %d users\n", len(records))
	}
	if stats, ok := result1["stats"].(map[string]interface{}); ok {
		fmt.Printf("⏱️  Execution time: %vms\n\n", stats["execution_time_ms"])
	}

	// Example 2: Count Users by Status
	fmt.Println("📝 Example 2: Count Users by Status")
	fmt.Println()

	script2 := map[string]interface{}{
		"label":      "users_by_status_go",
		"name":       "Users by Status",
		"version":    "1.0",
		"parameters": map[string]interface{}{},
		"functions": []interface{}{
			map[string]interface{}{"type": "FindAll", "collection": "crud_users_go"},
			map[string]interface{}{
				"type":      "Group",
				"by_fields": []string{"status"},
				"functions": []interface{}{
					map[string]interface{}{"output_field": "count", "operation": "Count"},
				},
			},
		},
		"tags": []string{"users", "analytics"},
	}
	scriptID2, _ := client.SaveScript(script2)
	scriptIDs = append(scriptIDs, scriptID2)
	fmt.Println("✅ Script saved")

	result2, _ := client.CallScript("users_by_status_go", nil)
	fmt.Println("📊 User counts by status:")
	if records, ok := result2["records"].([]interface{}); ok {
		for _, record := range records {
			fmt.Printf("   %v\n", record)
		}
	}
	if stats, ok := result2["stats"].(map[string]interface{}); ok {
		fmt.Printf("⏱️  Execution time: %vms\n\n", stats["execution_time_ms"])
	}

	// Cleanup
	fmt.Println("🧹 Cleaning up...")
	for _, scriptID := range scriptIDs {
		client.DeleteScript(scriptID)
	}
	client.DeleteCollection("crud_users_go")
	fmt.Println("✅ Cleanup complete")
	fmt.Println()
	fmt.Println("✅ All CRUD script examples finished!")
}
