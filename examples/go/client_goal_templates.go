// ekoDB Go Client - Goal Template CRUD Example
//
// Demonstrates creating, listing, getting, updating, and deleting goal templates.

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Goal Template CRUD Example (Go) ===\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		log.Fatal("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	// 1. Create a goal template
	fmt.Println("--- Creating goal template ---")
	template, err := client.GoalTemplateCreate(map[string]interface{}{
		"title":       "Data Migration",
		"description": "Template for migrating data between schemas",
		"steps": []map[string]interface{}{
			{"description": "Analyze source schema"},
			{"description": "Create target schema"},
			{"description": "Migrate records"},
			{"description": "Validate results"},
		},
	})
	if err != nil {
		log.Fatalf("Failed to create template: %v", err)
	}
	templateID := template["id"].(string)
	fmt.Printf("Created template: %s (id: %s)\n", template["title"], templateID)

	// 2. List all templates
	fmt.Println("\n--- Listing templates ---")
	list, err := client.GoalTemplateList()
	if err != nil {
		log.Fatalf("Failed to list templates: %v", err)
	}
	fmt.Printf("Templates: %v\n", list)

	// 3. Get template by ID
	fmt.Println("\n--- Getting template ---")
	fetched, err := client.GoalTemplateGet(templateID)
	if err != nil {
		log.Fatalf("Failed to get template: %v", err)
	}
	fmt.Printf("Fetched: %s\n", fetched["title"])

	// 4. Update template
	fmt.Println("\n--- Updating template ---")
	updated, err := client.GoalTemplateUpdate(templateID, map[string]interface{}{
		"description": "Updated: comprehensive data migration workflow",
	})
	if err != nil {
		log.Fatalf("Failed to update template: %v", err)
	}
	fmt.Printf("Updated description: %s\n", updated["description"])

	// 5. Delete template
	fmt.Println("\n--- Deleting template ---")
	err = client.GoalTemplateDelete(templateID)
	if err != nil {
		log.Fatalf("Failed to delete template: %v", err)
	}
	fmt.Println("Template deleted successfully")

	fmt.Println("\n✓ Goal template CRUD example completed")
}
