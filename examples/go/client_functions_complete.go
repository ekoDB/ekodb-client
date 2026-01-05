// Complete Scripts Example - ekoDB Scripts
// Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func strPtr(s string) *string {
	return &s
}

func main() {
	// Load environment
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

	fmt.Println("🚀 ekoDB Go Complete Scripts Example")
	fmt.Println()
	fmt.Println("📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines")
	fmt.Println()

	// Setup test data
	fmt.Println("📋 Setting up complete test data...")

	products := []map[string]interface{}{
		{"name": "Laptop Pro", "category": "Electronics", "price": 1299, "stock": 15, "rating": 4.8},
		{"name": "Wireless Mouse", "category": "Electronics", "price": 29, "stock": 45, "rating": 4.5},
		{"name": "Desk Chair", "category": "Furniture", "price": 349, "stock": 8, "rating": 4.2},
		{"name": "Standing Desk", "category": "Furniture", "price": 599, "stock": 12, "rating": 4.7},
		{"name": "Monitor 27\"", "category": "Electronics", "price": 399, "stock": 20, "rating": 4.6},
	}

	for _, product := range products {
		_, err := client.Insert("complete_products_go", product)
		if err != nil {
			log.Printf("Insert error: %v", err)
		}
	}
	fmt.Printf("✅ Created %d products\n\n", len(products))

	var scriptIDs []string

	// Example 1: FindAll + Group (Simple Aggregation)
	fmt.Println("📝 Example 1: FindAll + Group (Simple Aggregation)")
	fmt.Println()

	version := "1.0"
	script1 := ekodb.Script{
		Label:      "product_stats_go",
		Name:       "Product Statistics",
		Version:    &version,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("complete_products_go"),
			ekodb.StageGroup([]string{"category"}, []ekodb.GroupFunctionConfig{
				{OutputField: "count", Operation: "Count"},
				{OutputField: "avg_price", Operation: "Average", InputField: strPtr("price")},
			}),
		},
		Tags: []string{"products", "analytics"},
	}

	scriptID1, err := client.SaveScript(script1)
	if err != nil {
		log.Printf("Save script error: %v", err)
	} else {
		fmt.Printf("✅ Script saved: %s\n", scriptID1)
		scriptIDs = append(scriptIDs, scriptID1)

		result1, err := client.CallScript("product_stats_go", nil)
		if err != nil {
			log.Printf("Call script error: %v", err)
		} else {
			fmt.Printf("📊 Found %d category stats\n\n", len(result1.Records))
		}
	}

	// Example 2: Simple Product Listing
	fmt.Println("📝 Example 2: Simple Product Listing")
	fmt.Println()

	script2 := ekodb.Script{
		Label:      "list_all_products_go",
		Name:       "List All Products",
		Version:    &version,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("complete_products_go"),
		},
		Tags: []string{"products", "list"},
	}

	scriptID2, err := client.SaveScript(script2)
	if err != nil {
		log.Printf("Save script error: %v", err)
	} else {
		fmt.Println("✅ Script saved")
		scriptIDs = append(scriptIDs, scriptID2)

		result2, err := client.CallScript("list_all_products_go", nil)
		if err != nil {
			log.Printf("Call script error: %v", err)
		} else {
			fmt.Printf("📊 Found %d products\n\n", len(result2.Records))
		}
	}

	// Example 3: Count by Category
	fmt.Println("📝 Example 3: Count by Category")
	fmt.Println()

	script3 := ekodb.Script{
		Label:      "count_by_category_go",
		Name:       "Count Products by Category",
		Version:    &version,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("complete_products_go"),
			ekodb.StageGroup([]string{"category"}, []ekodb.GroupFunctionConfig{
				{OutputField: "count", Operation: "Count"},
			}),
		},
		Tags: []string{"products", "analytics"},
	}

	scriptID3, err := client.SaveScript(script3)
	if err != nil {
		log.Printf("Save script error: %v", err)
	} else {
		fmt.Println("✅ Script saved")
		scriptIDs = append(scriptIDs, scriptID3)

		result3, err := client.CallScript("count_by_category_go", nil)
		if err != nil {
			log.Printf("Call script error: %v", err)
		} else {
			fmt.Printf("📊 Found %d categories\n\n", len(result3.Records))
		}
	}

	// Example 4: Multi-Stage Pipeline
	fmt.Println("📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)")
	fmt.Println()

	script4 := ekodb.Script{
		Label:      "product_summary_go",
		Name:       "Product Summary Report",
		Version:    &version,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("complete_products_go"),
			ekodb.StageGroup([]string{"category"}, []ekodb.GroupFunctionConfig{
				{OutputField: "count", Operation: "Count"},
				{OutputField: "avg_price", Operation: "Average", InputField: strPtr("price")},
			}),
			ekodb.StageCount("total_categories"),
		},
		Tags: []string{"products", "analytics"},
	}

	scriptID4, err := client.SaveScript(script4)
	if err != nil {
		log.Printf("Save script error: %v", err)
	} else {
		fmt.Println("✅ Script saved")
		scriptIDs = append(scriptIDs, scriptID4)

		result4, err := client.CallScript("product_summary_go", nil)
		if err != nil {
			log.Printf("Call script error: %v", err)
		} else {
			fmt.Printf("📊 Pipeline executed %d stages\n\n", result4.Stats.StagesExecuted)
		}
	}

	// Cleanup
	fmt.Println("🧹 Cleaning up...")
	for _, scriptID := range scriptIDs {
		client.DeleteScript(scriptID)
	}
	client.DeleteCollection("complete_products_go")
	fmt.Println("✅ Cleanup complete")

	fmt.Println()
	fmt.Println("✅ All complete script examples finished!")
	fmt.Println()
	fmt.Println("💡 This example demonstrates ekoDB's Script system:")
	fmt.Println("   ✅ FindAll operations")
	fmt.Println("   ✅ Group aggregations (Count, Average)")
	fmt.Println("   ✅ Multi-stage pipelines (FindAll → Group → Count)")
	fmt.Println("   ✅ Script management (save, call, delete)")
}
