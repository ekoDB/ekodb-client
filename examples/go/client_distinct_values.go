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
		log.Printf("Warning: .env file not found")
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
		log.Fatalf("Failed to create client: %v", err)
	}

	fmt.Println("=== Distinct Values Example ===\n")

	collection := "distinct_values_example_go"

	// Cleanup from previous runs
	_ = client.DeleteCollection(collection)

	// ---------------------------------------------------------------------------
	// Step 1: Insert sample data
	// ---------------------------------------------------------------------------
	fmt.Println("Inserting sample products...")

	products := []ekodb.Record{
		{"name": "Widget A", "category": "electronics", "status": "active"},
		{"name": "Widget B", "category": "electronics", "status": "active"},
		{"name": "Gadget X", "category": "electronics", "status": "discontinued"},
		{"name": "Book 1", "category": "books", "status": "active"},
		{"name": "Book 2", "category": "books", "status": "active"},
		{"name": "Book 3", "category": "books", "status": "archived"},
		{"name": "Shirt M", "category": "clothing", "status": "active"},
		{"name": "Shirt L", "category": "clothing", "status": "active"},
	}

	for _, p := range products {
		if _, err := client.Insert(collection, p); err != nil {
			log.Fatalf("Insert failed: %v", err)
		}
	}
	fmt.Printf("Inserted %d products\n\n", len(products))

	// ---------------------------------------------------------------------------
	// Step 2: All distinct categories (no filter)
	// ---------------------------------------------------------------------------
	fmt.Println("=== Distinct Categories (all products) ===")
	resp, err := client.DistinctValues(collection, "category", ekodb.DistinctValuesQuery{})
	if err != nil {
		log.Fatalf("DistinctValues failed: %v", err)
	}
	fmt.Printf("Found %d distinct categories:\n", resp.Count)
	for _, v := range resp.Values {
		fmt.Printf("  - %v\n", v)
	}
	fmt.Println()

	// ---------------------------------------------------------------------------
	// Step 3: All distinct statuses (no filter)
	// ---------------------------------------------------------------------------
	fmt.Println("=== Distinct Statuses (all products) ===")
	resp, err = client.DistinctValues(collection, "status", ekodb.DistinctValuesQuery{})
	if err != nil {
		log.Fatalf("DistinctValues failed: %v", err)
	}
	fmt.Printf("Found %d distinct statuses:\n", resp.Count)
	for _, v := range resp.Values {
		fmt.Printf("  - %v\n", v)
	}
	fmt.Println()

	// ---------------------------------------------------------------------------
	// Step 4: Distinct statuses within electronics only
	// ---------------------------------------------------------------------------
	fmt.Println("=== Distinct Statuses in Electronics ===")
	resp, err = client.DistinctValues(collection, "status", ekodb.DistinctValuesQuery{
		Filter: map[string]interface{}{
			"type": "Condition",
			"content": map[string]interface{}{
				"field":    "category",
				"operator": "Eq",
				"value":    "electronics",
			},
		},
	})
	if err != nil {
		log.Fatalf("DistinctValues failed: %v", err)
	}
	fmt.Printf("Found %d distinct statuses for electronics:\n", resp.Count)
	for _, v := range resp.Values {
		fmt.Printf("  - %v\n", v)
	}
	fmt.Println()

	// ---------------------------------------------------------------------------
	// Cleanup
	// ---------------------------------------------------------------------------
	if err := client.DeleteCollection(collection); err != nil {
		log.Printf("Cleanup warning: %v", err)
	}
	fmt.Println("Cleanup done.")
}
