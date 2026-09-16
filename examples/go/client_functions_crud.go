// CRUD Functions Example - Basic Data Operations
//
// Demonstrates basic CRUD operations using scripts:
// - FindAll queries
// - Group aggregations
// - Simple data transformations

package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

// saveOrUpdateFn saves a function, or — if the label already exists (HTTP 409)
// — updates it in place and recovers the encrypted ID via a GET by label.
func saveOrUpdateFn(client *ekodb.Client, fn ekodb.UserFunction) (string, error) {
	id, err := client.SaveFunction(fn)
	if err == nil {
		return id, nil
	}
	var httpErr *ekodb.HTTPError
	if errors.As(err, &httpErr) && httpErr.StatusCode == 409 {
		if uerr := client.UpdateFunction(fn.Label, fn); uerr != nil {
			return "", uerr
		}
		fmt.Printf("Function '%s' already existed — updated instead\n", fn.Label)
		existing, gerr := client.GetFunction(fn.Label)
		if gerr != nil {
			return "", gerr
		}
		if existing.ID == nil {
			return "", fmt.Errorf("function %q has no id after update", fn.Label)
		}
		return *existing.ID, nil
	}
	return "", err
}

func run() (runErr error) {
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
		return err
	}
	collection := "crud_users_go"
	collectionOwned := false
	var scriptIDs []string
	defer func() {
		fmt.Println("🧹 Cleaning up...")
		for _, id := range scriptIDs {
			runErr = errors.Join(runErr, client.DeleteFunction(id))
		}
		if collectionOwned {
			runErr = errors.Join(runErr, client.DeleteCollection(collection))
		}
		if runErr == nil {
			fmt.Println("✅ Cleanup complete")
		}
	}()

	fmt.Println("🚀 ekoDB Go CRUD Functions Example")
	fmt.Println()

	// Setup test data
	fmt.Println("📋 Setting up test data...")
	client.DeleteCollection(collection)

	for i := 1; i <= 10; i++ {
		status := "active"
		if i%3 == 0 {
			status = "inactive"
		}
		role := "user"
		if i <= 3 {
			role = "admin"
		}
		if _, err := client.Insert(collection, map[string]interface{}{
			"name":   fmt.Sprintf("User %d", i),
			"email":  fmt.Sprintf("user%d@example.com", i),
			"status": status,
			"score":  i * 10,
			"role":   role,
		}); err != nil {
			return err
		}
		collectionOwned = true
	}
	fmt.Print("✅ Created 10 test users\n\n")

	// Example 1: List All Users
	fmt.Println("📝 Example 1: List All Users")
	fmt.Println()

	script1 := ekodb.UserFunction{
		Label:      "list_all_users_go",
		Name:       "List All Users",
		Version:    func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll(collection),
			ekodb.StageProject([]string{"name", "email", "status"}, false),
		},
		Tags: []string{"users", "query"},
	}
	scriptID1, err := saveOrUpdateFn(client, script1)
	if err != nil {
		return err
	}
	scriptIDs = append(scriptIDs, scriptID1)
	fmt.Println("✅ Function saved")

	result1, err := client.CallFunction("list_all_users_go", nil)
	if err != nil {
		return err
	}
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
			ekodb.StageFindAll(collection),
			ekodb.StageGroup([]string{"status"}, []ekodb.GroupFunctionConfig{
				{OutputField: "count", Operation: "Count"},
			}),
		},
		Tags: []string{"users", "analytics"},
	}
	scriptID2, err := saveOrUpdateFn(client, script2)
	if err != nil {
		return err
	}
	scriptIDs = append(scriptIDs, scriptID2)
	fmt.Println("✅ Function saved")

	result2, err := client.CallFunction("users_by_status_go", nil)
	if err != nil {
		return err
	}
	if result2 != nil {
		fmt.Println("📊 User counts by status:")
		for _, record := range result2.Records {
			fmt.Printf("   %v\n", record)
		}
		fmt.Printf("⏱️  Execution time: %vms\n\n", result2.Stats.ExecutionTimeMs)
	}

	fmt.Println("✅ All CRUD script examples finished!")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
