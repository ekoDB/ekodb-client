// Search Functions Example - Basic Search Operations
//
// Demonstrates simple search and query operations using scripts

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
	collection := "search_docs_go"
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

	fmt.Println("🚀 ekoDB Go Search Functions Example")
	fmt.Println()

	// Setup test data
	fmt.Println("📋 Setting up test data...")
	client.DeleteCollection(collection)

	documents := []map[string]interface{}{
		{"title": "Introduction to Machine Learning", "content": "Machine learning is a subset of AI.", "category": "AI", "tags": []string{"ml", "ai"}},
		{"title": "Database Design Principles", "content": "Good database design involves normalization.", "category": "Database", "tags": []string{"database", "design"}},
		{"title": "Vector Databases Explained", "content": "Vector databases store data based on similarity.", "category": "Database", "tags": []string{"vector", "database"}},
		{"title": "Natural Language Processing", "content": "NLP enables computers to understand language.", "category": "AI", "tags": []string{"nlp", "ai"}},
		{"title": "Getting Started with ekoDB", "content": "ekoDB is a high-performance database.", "category": "Database", "tags": []string{"ekodb", "tutorial"}},
	}

	for _, doc := range documents {
		if _, err := client.Insert(collection, doc); err != nil {
			return err
		}
		collectionOwned = true
	}
	fmt.Printf("✅ Inserted %d documents\n\n", len(documents))

	// Example 1: List All Documents
	fmt.Println("📝 Example 1: List All Documents")
	fmt.Println()

	script1 := ekodb.UserFunction{
		Label:      "list_all_docs_go",
		Name:       "List All Documents",
		Version:    func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions:  []ekodb.FunctionStageConfig{ekodb.StageFindAll(collection)},
		Tags:       []string{"search", "list"},
	}
	scriptID1, err := saveOrUpdateFn(client, script1)
	if err != nil {
		return err
	}
	scriptIDs = append(scriptIDs, scriptID1)
	fmt.Println("✅ Function saved")

	result1, err := client.CallFunction("list_all_docs_go", nil)
	if err != nil {
		return err
	}
	if result1 != nil {
		fmt.Printf("📊 Found %d documents\n", len(result1.Records))
		for i, record := range result1.Records {
			title := record["title"]
			category := record["category"]
			fmt.Printf("   %d. %v (%v)\n", i+1, title, category)
		}
		fmt.Printf("⏱️  Execution time: %vms\n\n", result1.Stats.ExecutionTimeMs)
	}

	// Example 2: Count Documents by Category
	fmt.Println("📝 Example 2: Count Documents by Category")
	fmt.Println()

	script2 := ekodb.UserFunction{
		Label:      "docs_by_category_go",
		Name:       "Documents by Category",
		Version:    func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll(collection),
			ekodb.StageGroup([]string{"category"}, []ekodb.GroupFunctionConfig{
				{OutputField: "count", Operation: "Count"},
			}),
		},
		Tags: []string{"search", "analytics"},
	}
	scriptID2, err := saveOrUpdateFn(client, script2)
	if err != nil {
		return err
	}
	scriptIDs = append(scriptIDs, scriptID2)
	fmt.Println("✅ Function saved")

	result2, err := client.CallFunction("docs_by_category_go", nil)
	if err != nil {
		return err
	}
	if result2 != nil {
		fmt.Println("📊 Documents by category:")
		for _, record := range result2.Records {
			fmt.Printf("   %v\n", record)
		}
		fmt.Printf("⏱️  Execution time: %vms\n\n", result2.Stats.ExecutionTimeMs)
	}

	fmt.Println("✅ All search script examples finished!")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
