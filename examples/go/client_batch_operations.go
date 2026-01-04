// Batch Operations - Using ekodb-client-go library
//
// This example demonstrates batch operations using the Go client library.
// Compare with batch_operations.go to see the difference!

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
	fmt.Println("✓ Client created")

	collection := "client_batch_operations_go"

	// Example 1: Batch Insert
	fmt.Println("\n=== Batch Insert ===")
	records := []ekodb.Record{
		{"name": "User 1", "email": "user1@example.com", "age": 25},
		{"name": "User 2", "email": "user2@example.com", "age": 30},
		{"name": "User 3", "email": "user3@example.com", "age": 35},
		{"name": "User 4", "email": "user4@example.com", "age": 40},
		{"name": "User 5", "email": "user5@example.com", "age": 45},
	}

	inserted, err := client.BatchInsert(collection, records)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Batch inserted %d records\n", len(inserted))

	// Collect IDs
	ids := make([]string, len(inserted))
	for i, rec := range inserted {
		ids[i] = rec["id"].(string)
	}

	// Verify
	limit := 100
	allDocs, err := client.Find(collection, ekodb.Query{Limit: &limit})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Verified: Found %d total records in collection\n", len(allDocs))

	// Example 2: Batch Update
	fmt.Println("\n=== Batch Update ===")
	updates := make(map[string]ekodb.Record)
	for i := 0; i < 3 && i < len(ids); i++ {
		updates[ids[i]] = ekodb.Record{
			"status":  "active",
			"updated": true,
		}
	}

	updated, err := client.BatchUpdate(collection, updates)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Batch updated %d records\n", len(updated))

	// Example 3: Batch Delete
	fmt.Println("\n=== Batch Delete ===")
	deleteIDs := ids[:3]
	deletedCount, err := client.BatchDelete(collection, deleteIDs)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Batch deleted %d records\n", deletedCount)

	// Cleanup: Delete the collection
	fmt.Println("\n=== Cleanup ===")
	if err := client.DeleteCollection(collection); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Deleted collection")

	fmt.Println("\n✓ All batch operations completed successfully")
}
