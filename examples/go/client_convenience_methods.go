// ekoDB Convenience Methods Example (Go)
//
// This example demonstrates the convenience methods added to the ekoDB Go client:
// - Upsert: Insert or update a record
// - FindOne: Find a single record by field value
// - Exists: Check if a record exists
// - Paginate: Paginate through records

package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("\n✅ All convenience methods demonstrated successfully!")
}

func run() (runErr error) {
	fmt.Print("=== ekoDB Convenience Methods Example ===\n\n")

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
		return fmt.Errorf("create client: %w", err)
	}
	collection := "convenience_methods_example_go"
	// Start from a deterministic dataset if a previous run was interrupted.
	if err := client.DeleteCollection(collection); err != nil && !isConvenienceNotFound(err) {
		return fmt.Errorf("clear collection %s before run: %w", collection, err)
	}
	defer func() {
		fmt.Println("\n=== Cleanup ===")
		if cleanupErr := client.DeleteCollection(collection); cleanupErr != nil && !isConvenienceNotFound(cleanupErr) {
			runErr = errors.Join(runErr, fmt.Errorf("delete collection %s: %w", collection, cleanupErr))
		} else {
			fmt.Println("✓ Deleted collection")
		}
	}()

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
		return fmt.Errorf("insert native map record: %w", err)
	}
	fmt.Printf("✓ Created record with native map: %v\n", inserted)

	fmt.Println("\n=== Upsert Operation ===")
	userID, ok := inserted["id"].(string)
	if !ok {
		return errors.New("insert response did not contain a string user ID")
	}

	// First upsert - will update (record exists)
	user2 := ekodb.Record{
		"name":  "Alice Johnson",
		"email": "alice.j@newdomain.com",
		"age":   29,
	}

	upserted1, err := client.Upsert(collection, userID, user2)
	if err != nil {
		return fmt.Errorf("update-path upsert: %w", err)
	}
	fmt.Printf("✓ First upsert (update): %v\n", upserted1)

	// Second upsert - will insert (different ID, doesn't exist)
	user3 := ekodb.Record{
		"name":  "Bob Smith",
		"email": "bob@example.com",
		"age":   35,
	}

	missingID := fmt.Sprintf("new-user-%d", time.Now().UnixNano())
	upserted2, err := client.Upsert(collection, missingID, user3)
	if err != nil {
		return fmt.Errorf("insert-path upsert: %w", err)
	}
	fmt.Printf("✓ Second upsert (insert): %v\n", upserted2)
	upsertedID := ekodb.GetStringValue(upserted2["id"])
	if upsertedID == "" {
		return errors.New("insert-path upsert response did not contain a record ID")
	}
	insertedByUpsert, err := client.Exists(collection, upsertedID)
	if err != nil {
		return fmt.Errorf("verify insert-path upsert ID %s: %w", upsertedID, err)
	}
	if !insertedByUpsert {
		return fmt.Errorf("insert-path upsert returned ID %s, but that record was not retrievable", upsertedID)
	}

	fmt.Println("\n=== Find One Operation ===")
	// Find a single record by any field
	foundUser, err := client.FindOne(collection, "email", "alice.j@newdomain.com")
	if err != nil {
		return fmt.Errorf("find updated Alice: %w", err)
	}
	if foundUser == nil {
		return errors.New("updated Alice was not found by email")
	}
	fmt.Printf("✓ Found user by email: %v\n", foundUser)

	// Try finding a non-existent user
	notFound, err := client.FindOne(collection, "email", "nonexistent@example.com")
	if err != nil {
		return fmt.Errorf("find nonexistent user: %w", err)
	}
	if notFound != nil {
		return fmt.Errorf("nonexistent user unexpectedly found: %v", notFound)
	}
	fmt.Println("✓ User not found (as expected)")

	fmt.Println("\n=== Exists Check ===")
	// Check existence without fetching the record
	exists, err := client.Exists(collection, userID)
	if err != nil {
		return fmt.Errorf("check existing user: %w", err)
	}
	if !exists {
		return fmt.Errorf("inserted user %s was not found", userID)
	}
	fmt.Printf("✓ Record exists: %v\n", exists)

	// Check non-existent record
	notExists, err := client.Exists(collection, "fake-id-123")
	if err != nil {
		return fmt.Errorf("check nonexistent user: %w", err)
	}
	if notExists {
		return errors.New("fake-id-123 unexpectedly exists")
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
			return fmt.Errorf("insert pagination record %d: %w", i, err)
		}
	}
	fmt.Println("✓ Inserted 25 records for pagination")
	totalRecords, err := client.CountDocuments(collection)
	if err != nil {
		return fmt.Errorf("count pagination records: %w", err)
	}
	if totalRecords != 27 {
		return fmt.Errorf("pagination setup produced %d records, expected 27", totalRecords)
	}

	// Paginate through records
	pageSize := 10
	for page := 1; page <= 3; page++ {
		records, err := client.Paginate(collection, page, pageSize)
		if err != nil {
			return fmt.Errorf("paginate page %d: %w", page, err)
		}
		expected := pageSize
		if remaining := totalRecords - (page-1)*pageSize; remaining < expected {
			expected = remaining
		}
		if len(records) != expected {
			return fmt.Errorf("page %d returned %d records, expected %d", page, len(records), expected)
		}
		fmt.Printf("✓ Page %d: %d records (expected %d)\n", page, len(records), expected)
	}

	return nil
}

func isConvenienceNotFound(err error) bool {
	var httpErr *ekodb.HTTPError
	return (errors.As(err, &httpErr) && httpErr.StatusCode == 404) ||
		strings.Contains(strings.ToLower(err.Error()), "not found")
}
