// Advanced Scripts Example - Query, Sort, Limit, Group
//
// Demonstrates advanced query and aggregation operations using simple patterns

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

	fmt.Println("🚀 ekoDB Go Advanced Scripts Example")
	fmt.Println()

	// Setup test data
	fmt.Println("📋 Setting up test data...")
	client.DeleteCollection("advanced_products_go")

	products := []map[string]interface{}{
		{"name": "Laptop Pro", "category": "Electronics", "price": 1299, "stock": 15, "rating": 4.8},
		{"name": "Wireless Mouse", "category": "Electronics", "price": 29, "stock": 45, "rating": 4.5},
		{"name": "Desk Chair", "category": "Furniture", "price": 349, "stock": 8, "rating": 4.2},
		{"name": "Standing Desk", "category": "Furniture", "price": 599, "stock": 12, "rating": 4.7},
		{"name": "USB-C Cable", "category": "Electronics", "price": 19, "stock": 100, "rating": 4.3},
		{"name": "Monitor 27\"", "category": "Electronics", "price": 399, "stock": 20, "rating": 4.6},
		{"name": "Bookshelf", "category": "Furniture", "price": 149, "stock": 5, "rating": 4.1},
		{"name": "Keyboard", "category": "Electronics", "price": 89, "stock": 30, "rating": 4.4},
	}

	for _, product := range products {
		client.Insert("advanced_products_go", product)
	}
	fmt.Printf("✅ Created %d products\n\n", len(products))

	var scriptIDs []string

	// Example 1: List All Products
	fmt.Println("📝 Example 1: List All Products")
	fmt.Println()

	version1 := "1.0"
	script1 := ekodb.Script{
		Label:      "list_all_products_adv_go",
		Name:       "List All Products",
		Version:    &version1,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions:  []ekodb.FunctionStageConfig{ekodb.StageFindAll("advanced_products_go")},
		Tags:       []string{"products", "list"},
	}
	scriptID1, _ := client.SaveScript(script1)
	scriptIDs = append(scriptIDs, scriptID1)
	fmt.Println("✅ Script saved")

	result1, _ := client.CallScript("list_all_products_adv_go", nil)
	if result1 != nil {
		fmt.Printf("📊 Found %d products\n", len(result1.Records))
		fmt.Printf("⏱️  Execution time: %vms\n\n", result1.Stats.ExecutionTimeMs)
	}

	// Example 2: Group Products by Category
	fmt.Println("📝 Example 2: Group Products by Category")
	fmt.Println()

	version2 := "1.0"
	script2 := ekodb.Script{
		Label:      "products_by_category_go",
		Name:       "Products by Category",
		Version:    &version2,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("advanced_products_go"),
			ekodb.StageGroup([]string{"category"}, []ekodb.GroupFunctionConfig{
				{OutputField: "count", Operation: "Count"},
				{OutputField: "avg_price", Operation: "Average", InputField: strPtr("price")},
			}),
		},
		Tags: []string{"products", "analytics"},
	}
	scriptID2, _ := client.SaveScript(script2)
	scriptIDs = append(scriptIDs, scriptID2)
	fmt.Println("✅ Script saved")

	result2, _ := client.CallScript("products_by_category_go", nil)
	if result2 != nil {
		fmt.Println("📊 Category breakdown:")
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
	client.DeleteCollection("advanced_products_go")
	fmt.Println("✅ Cleanup complete")
	fmt.Println()
	fmt.Println("✅ All advanced script examples finished!")
}

func strPtr(s string) *string {
	return &s
}
