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
			"status": map[bool]string{true: "active", false: "inactive"}[i%2 == 0],
			"score":  i * 10,
		}
		_, err := client.Insert("users", record)
		if err != nil {
			return err
		}
	}

	fmt.Println("✅ Test data ready\n")
	return nil
}

func simpleQueryFunction(client *ekodb.Client) (string, error) {
	fmt.Println("📝 Example 1: Simple Query Function\n")

	function := ekodb.SavedFunction{
		Label:       "get_active_users",
		Name:        "Get Active Users",
		Description: stringPtr("Retrieve all active users"),
		Version:     "1.0",
		Parameters:  map[string]ekodb.ParameterDefinition{},
		Pipeline: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
		},
		Tags: []string{"users", "query"},
	}

	id, err := client.SaveFunction(function)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Function saved: %s\n", id)

	result, err := client.CallFunction("get_active_users", nil)
	if err != nil {
		return "", err
	}
	fmt.Printf("📊 Found %d active users\n\n", len(result.Records))

	return id, nil
}

func parameterizedFunction(client *ekodb.Client) error {
	fmt.Println("📝 Example 2: Parameterized Function\n")

	function := ekodb.SavedFunction{
		Label:   "get_users_by_status",
		Name:    "Get Users By Status",
		Version: "1.0",
		Parameters: map[string]ekodb.ParameterDefinition{
			"status": {
				Required: false,
				Default:  "active",
			},
			"limit": {
				Required: false,
				Default:  10,
			},
		},
		Pipeline: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
		},
		Tags: []string{"users", "parameterized"},
	}

	id, err := client.SaveFunction(function)
	if err != nil {
		return err
	}
	fmt.Printf("✅ Function saved: %s\n", id)

	params := map[string]interface{}{
		"status": "active",
		"limit":  3,
	}
	result, err := client.CallFunction("get_users_by_status", params)
	if err != nil {
		return err
	}
	fmt.Printf("📊 Found %d users (limited)\n\n", len(result.Records))

	return nil
}

func aggregationFunction(client *ekodb.Client) (string, error) {
	fmt.Println("📝 Example 3: Aggregation Function\n")

	function := ekodb.SavedFunction{
		Label:      "user_stats",
		Name:       "User Statistics",
		Version:    "1.0",
		Parameters: map[string]ekodb.ParameterDefinition{},
		Pipeline: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
			ekodb.StageGroup(
				[]string{"status"},
				[]ekodb.GroupFunctionConfig{
					{OutputField: "count", Operation: ekodb.GroupFunctionCount},
					{OutputField: "avg_score", Operation: ekodb.GroupFunctionAverage, InputField: stringPtr("score")},
				},
			),
		},
		Tags: []string{"analytics"},
	}

	id, err := client.SaveFunction(function)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Function saved: %s\n", id)

	result, err := client.CallFunction("user_stats", nil)
	if err != nil {
		return "", err
	}
	fmt.Printf("📊 Statistics: %d groups\n", len(result.Records))
	for _, record := range result.Records {
		fmt.Printf("   %+v\n", record)
	}
	fmt.Println()

	return id, nil
}

func functionManagement(client *ekodb.Client, getActiveUsersID, userStatsID string) error {
	fmt.Println("📝 Example 4: Function Management\n")

	// List all functions
	functions, err := client.ListFunctions(nil)
	if err != nil {
		return err
	}
	fmt.Printf("📋 Total functions: %d\n", len(functions))

	// Get specific function (requires ID, not label)
	function, err := client.GetFunction(getActiveUsersID)
	if err != nil {
		return err
	}
	fmt.Printf("🔍 Retrieved function: %s\n", function.Name)

	// Update function (requires ID, not label)
	updated := ekodb.SavedFunction{
		Label:       "get_active_users",
		Name:        "Get Active Users (Updated)",
		Description: stringPtr("Updated description"),
		Version:     "1.1",
		Parameters:  map[string]ekodb.ParameterDefinition{},
		Pipeline: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("users"),
		},
		Tags: []string{"users"},
	}
	err = client.UpdateFunction(getActiveUsersID, updated)
	if err != nil {
		return err
	}
	fmt.Println("✏️  Function updated")

	// Delete function (requires ID, not label)
	err = client.DeleteFunction(userStatsID)
	if err != nil {
		return err
	}
	fmt.Println("🗑️  Function deleted\n")

	fmt.Println("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
	fmt.Println("ℹ️  Only CALL can use either ID or label\n")

	return nil
}

func stringPtr(s string) *string {
	return &s
}

func main() {
	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}

	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}

	// Create ekoDB client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Client created (token exchange happens automatically)")

	// Run examples
	if err := setupTestData(client); err != nil {
		log.Fatalf("❌ Setup failed: %v", err)
	}

	getActiveUsersID, err := simpleQueryFunction(client)
	if err != nil {
		log.Fatalf("❌ Example 1 failed: %v", err)
	}

	if err := parameterizedFunction(client); err != nil {
		log.Fatalf("❌ Example 2 failed: %v", err)
	}

	userStatsID, err := aggregationFunction(client)
	if err != nil {
		log.Fatalf("❌ Example 3 failed: %v", err)
	}

	if err := functionManagement(client, getActiveUsersID, userStatsID); err != nil {
		log.Fatalf("❌ Example 4 failed: %v", err)
	}

	fmt.Println("✅ All examples completed!")
}
