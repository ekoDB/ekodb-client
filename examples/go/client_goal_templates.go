// ekoDB Go Client - Goal Template CRUD Example
//
// Demonstrates creating, listing, getting, updating, and deleting goal templates.

package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func run() (runErr error) {
	fmt.Print("=== ekoDB Goal Template CRUD Example (Go) ===\n\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		return errors.New("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}

	// 1. Create a goal template
	fmt.Println("--- Creating goal template ---")
	templateTitle := "Data Migration"
	template, err := client.GoalTemplateCreate(map[string]interface{}{
		"title":       templateTitle,
		"description": "Template for migrating data between schemas",
		"steps": []map[string]interface{}{
			{"description": "Analyze source schema"},
			{"description": "Create target schema"},
			{"description": "Migrate records"},
			{"description": "Validate results"},
		},
	})
	if err != nil {
		return fmt.Errorf("create template: %w", err)
	}
	templateID := ekodb.GetStringValue(template["id"])
	if templateID == "" {
		return errors.New("create template response contained no ID")
	}
	templateDeleted := false
	defer func() {
		if !templateDeleted {
			if cleanupErr := client.GoalTemplateDelete(templateID); cleanupErr != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete template %s: %w", templateID, cleanupErr))
			}
		}
	}()
	fmt.Printf("Created template: %s (id: %s)\n", templateTitle, templateID)

	// 2. List all templates
	fmt.Println("\n--- Listing templates ---")
	list, err := client.GoalTemplateList()
	if err != nil {
		return fmt.Errorf("list templates: %w", err)
	}
	if !responseContainsID(list, templateID) {
		return fmt.Errorf("template list did not contain created ID %s: %v", templateID, list)
	}
	fmt.Printf("Templates: %v\n", list)

	// 3. Get template by ID
	fmt.Println("\n--- Getting template ---")
	fetched, err := client.GoalTemplateGet(templateID)
	if err != nil {
		return fmt.Errorf("get template: %w", err)
	}
	if title := ekodb.GetStringValue(fetched["title"]); title != templateTitle {
		return fmt.Errorf("fetched template title was %q, expected %q", title, templateTitle)
	}
	fmt.Printf("Fetched: %s\n", ekodb.GetStringValue(fetched["title"]))

	// 4. Update template
	fmt.Println("\n--- Updating template ---")
	updated, err := client.GoalTemplateUpdate(templateID, map[string]interface{}{
		"description": "Updated: comprehensive data migration workflow",
	})
	if err != nil {
		return fmt.Errorf("update template: %w", err)
	}
	wantDescription := "Updated: comprehensive data migration workflow"
	if description := ekodb.GetStringValue(updated["description"]); description != wantDescription {
		return fmt.Errorf("updated template description was %q, expected %q", description, wantDescription)
	}
	fmt.Printf("Updated description: %s\n", ekodb.GetStringValue(updated["description"]))

	// 5. Delete template
	fmt.Println("\n--- Deleting template ---")
	err = client.GoalTemplateDelete(templateID)
	if err != nil {
		return fmt.Errorf("delete template: %w", err)
	}
	templateDeleted = true
	fmt.Println("Template deleted successfully")

	fmt.Println("\n✓ Goal template CRUD example completed")
	return nil
}

func responseContainsID(value interface{}, id string) bool {
	switch typed := value.(type) {
	case []interface{}:
		for _, item := range typed {
			if responseContainsID(item, id) {
				return true
			}
		}
	case map[string]interface{}:
		if ekodb.GetStringValue(typed["id"]) == id {
			return true
		}
		for _, key := range []string{"items", "templates"} {
			if responseContainsID(typed[key], id) {
				return true
			}
		}
	}
	return false
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
