// ekoDB Convenience Methods Example (Go)
//
// This example demonstrates the convenience methods added to the ekoDB Go client:
// - Upsert: Insert or update a record
// - FindOne: Find a single record by field value
// - Exists: Check if a record exists
// - Paginate: Paginate through records

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Convenience Methods Example ===\n")

	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}

	// Create client
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
	collection := "convenience_methods_example_go"

	fmt.Println("=== Native Map Creation ===")
	// Go uses native maps - no builder needed!
	user1 := ekodb.Record{
		"name":   "Alice Johnson",
		"email":  "alice@example.com",
		"age":    28,
		"active": true,
	}

	inserted, err := client.Insert(collection, user1)
	if err != nil {
		log.Fatalf("Failed to insert: %v", err)
	}
	fmt.Printf("✓ Created record with native map: %v\n", inserted)

	fmt.Println("\n=== Upsert Operation ===")
	userID, ok := inserted["id"].(string)
	if !ok {
		log.Fatal("Failed to get user ID")
	}

	// First upsert - will update (record exists)
	user2 := ekodb.Record{
		"name":  "Alice Johnson",
		"email": "alice.j@newdomain.com",
		"age":   29,
	}

	upserted1, err := client.Upsert(collection, userID, user2)
	if err != nil {
		log.Fatalf("Failed to upsert: %v", err)
	}
	fmt.Printf("✓ First upsert (update): %v\n", upserted1)

	// Second upsert - will insert (different ID, doesn't exist)
	user3 := ekodb.Record{
		"name":  "Bob Smith",
		"email": "bob@example.com",
		"age":   35,
	}

	upserted2, err := client.Upsert(collection, "new-user-id", user3)
	if err != nil {
		log.Fatalf("Failed to upsert: %v", err)
	}
	fmt.Printf("✓ Second upsert (insert): %v\n", upserted2)

	fmt.Println("\n=== Find One Operation ===")
	// Find a single record by any field
	foundUser, err := client.FindOne(collection, "email", "alice.j@newdomain.com")
	if err != nil {
		log.Fatalf("Failed to find one: %v", err)
	}
	if foundUser != nil {
		fmt.Printf("✓ Found user by email: %v\n", foundUser)
	}

	// Try finding a non-existent user
	notFound, err := client.FindOne(collection, "email", "nonexistent@example.com")
	if err != nil {
		log.Fatalf("Failed to find one: %v", err)
	}
	if notFound == nil {
		fmt.Println("✓ User not found (as expected)")
	}

	fmt.Println("\n=== Exists Check ===")
	// Check existence without fetching the record
	exists, err := client.Exists(collection, userID)
	if err != nil {
		log.Fatalf("Failed to check exists: %v", err)
	}
	fmt.Printf("✓ Record exists: %v\n", exists)

	// Check non-existent record
	notExists, err := client.Exists(collection, "fake-id-123")
	if err != nil {
		log.Fatalf("Failed to check exists: %v", err)
	}
	fmt.Printf("✓ Fake record exists: %v (should be false)\n", notExists)

	fmt.Println("\n=== Pagination ===")
	// Insert more records for pagination demo
	for i := 1; i <= 25; i++ {
		record := ekodb.Record{
			"name":  fmt.Sprintf("User %d", i),
			"email": fmt.Sprintf("user%d@example.com", i),
			"index": i,
		}
		_, err := client.Insert(collection, record)
		if err != nil {
			log.Fatalf("Failed to insert pagination record: %v", err)
		}
	}
	fmt.Println("✓ Inserted 25 records for pagination")

	// Paginate through records
	pageSize := 10
	for page := 1; page <= 3; page++ {
		records, err := client.Paginate(collection, page, pageSize)
		if err != nil {
			log.Fatalf("Failed to paginate: %v", err)
		}
		expected := "~7"
		if page < 3 {
			expected = "10"
		}
		fmt.Printf("✓ Page %d: %d records (expected %s)\n", page, len(records), expected)
	}

	fmt.Println("\n=== Cleanup ===")
	err = client.DeleteCollection(collection)
	if err != nil {
		log.Fatalf("Failed to delete collection: %v", err)
	}
	fmt.Println("✓ Deleted collection")

	fmt.Println("\n✅ All convenience methods demonstrated successfully!")
}
