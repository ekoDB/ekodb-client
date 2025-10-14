// Simple CRUD Operations - Using ekodb-client-go library
//
// This example demonstrates CRUD operations using the Go client library.
// Compare with ttl-caching/simple_crud.go to see the difference!

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

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

	collection := "client_simple_crud_go"

	// Example 1: Insert a document
	fmt.Println("\n=== Insert Document ===")
	record := ekodb.Record{
		"name":   "Test Record",
		"value":  42,
		"active": true,
	}

	inserted, err := client.Insert(collection, record)
	if err != nil {
		log.Fatal(err)
	}
	docID := inserted["id"].(string)
	fmt.Printf("Inserted: %v\n", inserted)

	// Example 2: Find by ID
	fmt.Println("\n=== Find by ID ===")
	found, err := client.FindByID(collection, docID)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Found: %v\n", found)

	// Example 3: Find with query
	fmt.Println("\n=== Find with Query ===")
	limit := 10
	query := ekodb.Query{
		Limit: &limit,
	}
	docs, err := client.Find(collection, query)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Found documents: %d\n", len(docs))

	// Example 4: Update document
	fmt.Println("\n=== Update Document ===")
	updateData := ekodb.Record{
		"name":  "Updated Record",
		"value": 100,
	}
	updated, err := client.Update(collection, docID, updateData)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Updated: %v\n", updated)

	// Example 5: Delete document
	fmt.Println("\n=== Delete Document ===")
	if err := client.Delete(collection, docID); err != nil {
		log.Fatal(err)
	}
	fmt.Println("Deleted document")

	// Cleanup: Delete the collection
	fmt.Println("\n=== Cleanup ===")
	if err := client.DeleteCollection(collection); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Deleted collection")

	fmt.Println("\n✓ All CRUD operations completed successfully")
}
