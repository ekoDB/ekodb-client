// Search Scripts Example - Basic Search Operations
//
// Demonstrates simple search and query operations using scripts

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

	fmt.Println("🚀 ekoDB Go Search Scripts Example")
	fmt.Println()

	// Setup test data
	fmt.Println("📋 Setting up test data...")
	client.DeleteCollection("search_docs_go")

	documents := []map[string]interface{}{
		{"title": "Introduction to Machine Learning", "content": "Machine learning is a subset of AI.", "category": "AI", "tags": []string{"ml", "ai"}},
		{"title": "Database Design Principles", "content": "Good database design involves normalization.", "category": "Database", "tags": []string{"database", "design"}},
		{"title": "Vector Databases Explained", "content": "Vector databases store data based on similarity.", "category": "Database", "tags": []string{"vector", "database"}},
		{"title": "Natural Language Processing", "content": "NLP enables computers to understand language.", "category": "AI", "tags": []string{"nlp", "ai"}},
		{"title": "Getting Started with ekoDB", "content": "ekoDB is a high-performance database.", "category": "Database", "tags": []string{"ekodb", "tutorial"}},
	}

	for _, doc := range documents {
		client.Insert("search_docs_go", doc)
	}
	fmt.Printf("✅ Inserted %d documents\n\n", len(documents))

	var scriptIDs []string

	// Example 1: List All Documents
	fmt.Println("📝 Example 1: List All Documents")
	fmt.Println()

	script1 := ekodb.Script{
		Label:      "list_all_docs_go",
		Name:       "List All Documents",
		Version:    func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions:  []ekodb.FunctionStageConfig{ekodb.StageFindAll("search_docs_go")},
		Tags:       []string{"search", "list"},
	}
	scriptID1, _ := client.SaveScript(script1)
	scriptIDs = append(scriptIDs, scriptID1)
	fmt.Println("✅ Script saved")

	result1, _ := client.CallScript("list_all_docs_go", nil)
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

	script2 := ekodb.Script{
		Label:      "docs_by_category_go",
		Name:       "Documents by Category",
		Version:    func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("search_docs_go"),
			ekodb.StageGroup([]string{"category"}, []ekodb.GroupFunctionConfig{
				{OutputField: "count", Operation: "Count"},
			}),
		},
		Tags: []string{"search", "analytics"},
	}
	scriptID2, _ := client.SaveScript(script2)
	scriptIDs = append(scriptIDs, scriptID2)
	fmt.Println("✅ Script saved")

	result2, _ := client.CallScript("docs_by_category_go", nil)
	if result2 != nil {
		fmt.Println("📊 Documents by category:")
		for _, record := range result2.Records {
			fmt.Printf("   %v\n", record)
		}
		fmt.Printf("⏱️  Execution time: %vms\n\n", result2.Stats.ExecutionTimeMs)
	}

	// Cleanup
	fmt.Println("🧹 Cleaning up...")
	for _, scriptID := range scriptIDs {
		client.DeleteScript(scriptID)
	}
	client.DeleteCollection("search_docs_go")
	fmt.Println("✅ Cleanup complete")
	fmt.Println()
	fmt.Println("✅ All search script examples finished!")
}
