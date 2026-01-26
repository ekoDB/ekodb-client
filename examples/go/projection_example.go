// Field Projection Examples for Go Client
//
// Demonstrates how to use SelectFields and ExcludeFields to optimize
// data transfer and improve query performance.
//
// Shows comparison between full object queries and projected queries.

package main

import (
	"fmt"
	"log"
	"os"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

const testCollection = "projection_test_users_go"

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

	// Initialize client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	fmt.Println("Client created\n")

	// Setup: Create test collection and insert test data
	fmt.Println("Setting up test data...")
	// Cleanup any existing test collection
	_ = client.DeleteCollection(testCollection)

	// Insert test users with various fields
	testUsers := []ekodb.Record{
		{
			"name":         "Alice Johnson",
			"email":        "alice@example.com",
			"age":          30,
			"status":       "active",
			"user_role":    "admin",
			"password":     "secret123",
			"api_key":      "key123",
			"secret_token": "token123",
			"bio":          "Software engineer",
			"avatar_url":   "https://example.com/alice.jpg",
			"created_at":   time.Now().Format(time.RFC3339),
		},
		{
			"name":         "Bob Smith",
			"email":        "bob@example.com",
			"age":          25,
			"status":       "active",
			"user_role":    "user",
			"password":     "secret456",
			"api_key":      "key456",
			"secret_token": "token456",
			"bio":          "Designer",
			"avatar_url":   "https://example.com/bob.jpg",
			"created_at":   time.Now().Format(time.RFC3339),
		},
		{
			"name":         "Carol White",
			"email":        "carol@example.com",
			"age":          35,
			"status":       "inactive",
			"user_role":    "user",
			"password":     "secret789",
			"api_key":      "key789",
			"secret_token": "token789",
			"bio":          "Manager",
			"avatar_url":   "https://example.com/carol.jpg",
			"created_at":   time.Now().Format(time.RFC3339),
		},
		{
			"name":         "Dave Brown",
			"email":        "dave@example.com",
			"age":          45,
			"status":       "active",
			"user_role":    "admin",
			"password":     "secretabc",
			"api_key":      "keyabc",
			"secret_token": "tokenabc",
			"bio":          "CTO",
			"avatar_url":   "https://example.com/dave.jpg",
			"created_at":   time.Now().Format(time.RFC3339),
		},
	}

	var insertedIDs []string
	for _, user := range testUsers {
		result, err := client.Insert(testCollection, user)
		if err != nil {
			log.Printf("Warning: Insert failed: %v", err)
		} else if id, ok := result["id"].(string); ok {
			insertedIDs = append(insertedIDs, id)
		}
	}
	fmt.Printf("Inserted %d test users\n\n", len(insertedIDs))

	// Example 1: Select specific fields to reduce data transfer
	fmt.Println("Example 1: Select specific fields")
	usersQuery := ekodb.NewQueryBuilder().
		Eq("status", "active").
		SelectFields("id", "name", "email").
		Limit(10).
		Build()

	users, err := client.Find(testCollection, usersQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}
	fmt.Printf("Fetched %d users with only 3 fields each\n", len(users))

	// Example 2: Exclude sensitive fields
	fmt.Println("\nExample 2: Exclude sensitive fields")
	adminsQuery := ekodb.NewQueryBuilder().
		Eq("user_role", "admin").
		ExcludeFields("password", "api_key", "secret_token").
		Build()

	admins, err := client.Find(testCollection, adminsQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}
	fmt.Printf("Fetched %d admins without sensitive data\n", len(admins))
	// Verify sensitive fields are excluded
	if len(admins) > 0 {
		_, hasPassword := admins[0]["password"]
		fmt.Printf("  Password field excluded: %v\n", !hasPassword)
	}

	// Example 3: Complex query with projection
	fmt.Println("\nExample 3: Complex query with projection")
	activeUsersQuery := ekodb.NewQueryBuilder().
		Eq("status", "active").
		Gte("age", 18).
		Lt("age", 65).
		SelectFields("id", "name", "email", "age", "created_at").
		SortDescending("created_at").
		Limit(50).
		Build()

	activeUsers, err := client.Find(testCollection, activeUsersQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}
	fmt.Printf("Fetched %d active users with profile fields\n", len(activeUsers))

	// Example 4: Find by ID with projection
	fmt.Println("\nExample 4: Find by ID with projection")
	if len(insertedIDs) > 0 {
		userRecord, err := client.FindByIDWithProjection(
			testCollection,
			insertedIDs[0],
			[]string{"id", "name", "email", "bio", "avatar_url"},
			nil,
		)
		if err != nil {
			log.Fatalf("FindByIDWithProjection failed: %v", err)
		}

		name := ekodb.GetStringValue(userRecord["name"])
		fmt.Printf("Fetched user profile: %s\n", name)
	}

	// Example 5: Compare full vs projected data
	fmt.Println("\nExample 5: Compare full vs projected data")
	fullQuery := ekodb.NewQueryBuilder().
		Eq("status", "active").
		Build()

	projectedQuery := ekodb.NewQueryBuilder().
		Eq("status", "active").
		SelectFields("id", "name", "email").
		Build()

	fullUsers, err := client.Find(testCollection, fullQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}

	projectedUsers, err := client.Find(testCollection, projectedQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}

	fullFields := 0
	if len(fullUsers) > 0 {
		fullFields = len(fullUsers[0])
	}
	projectedFields := 0
	if len(projectedUsers) > 0 {
		projectedFields = len(projectedUsers[0])
	}

	fmt.Printf("Full query returned %d fields per user\n", fullFields)
	fmt.Printf("Projected query returned %d fields per user\n", projectedFields)

	// Cleanup
	fmt.Println("\nCleaning up test data...")
	if err := client.DeleteCollection(testCollection); err != nil {
		log.Printf("Warning: Failed to delete collection: %v", err)
	} else {
		fmt.Println("Cleanup complete")
	}

	fmt.Println("\nAll projection examples completed successfully!")
}
