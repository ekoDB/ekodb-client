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
	"sort"
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

	// Example 1: Select specific fields - only get id, name, email
	fmt.Println("Example 1: Select specific fields (id, name, email only)")
	usersQuery := ekodb.NewQueryBuilder().
		Eq("status", "active").
		SelectFields("id", "name", "email").
		Limit(10).
		Build()

	users, err := client.Find(testCollection, usersQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}
	fmt.Printf("  Found %d active users\n", len(users))
	if len(users) > 0 {
		fields := getFieldNames(users[0])
		fmt.Printf("  Fields returned: %v\n", fields)
		name := ekodb.GetStringValue(users[0]["name"])
		email := ekodb.GetStringValue(users[0]["email"])
		fmt.Printf("  First user: %s <%s>\n", name, email)
	}

	// Example 2: Exclude sensitive fields - hide password, api_key, secret_token
	fmt.Println("\nExample 2: Exclude sensitive fields (password, api_key, secret_token)")
	adminsQuery := ekodb.NewQueryBuilder().
		Eq("user_role", "admin").
		ExcludeFields("password", "api_key", "secret_token").
		Build()

	admins, err := client.Find(testCollection, adminsQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}
	fmt.Printf("  Found %d admins\n", len(admins))
	if len(admins) > 0 {
		_, hasPassword := admins[0]["password"]
		_, hasApiKey := admins[0]["api_key"]
		_, hasToken := admins[0]["secret_token"]
		fmt.Println("  Sensitive fields excluded:")
		fmt.Printf("    - password: %s\n", excludedStatus(hasPassword))
		fmt.Printf("    - api_key: %s\n", excludedStatus(hasApiKey))
		fmt.Printf("    - secret_token: %s\n", excludedStatus(hasToken))
		fields := getFieldNames(admins[0])
		fmt.Printf("  Fields returned: %v\n", fields)
	}

	// Example 3: Complex query with projection - active users with profile fields
	fmt.Println("\nExample 3: Complex query with projection (active users, ages 18-65)")
	activeUsersQuery := ekodb.NewQueryBuilder().
		Eq("status", "active").
		Gte("age", 18).
		Lt("age", 65).
		SelectFields("id", "name", "email", "age", "created_at").
		SortDescending("age").
		Limit(50).
		Build()

	activeUsers, err := client.Find(testCollection, activeUsersQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}
	fmt.Printf("  Found %d active users (ages 18-65)\n", len(activeUsers))
	for _, user := range activeUsers {
		name := ekodb.GetStringValue(user["name"])
		age, _ := ekodb.GetIntValue(user["age"])
		fmt.Printf("    - %s (age %d)\n", name, age)
	}

	// Example 4: Query inactive users with profile fields
	fmt.Println("\nExample 4: Query inactive users with profile fields")
	inactiveQuery := ekodb.NewQueryBuilder().
		Eq("status", "inactive").
		SelectFields("id", "name", "email", "bio").
		Build()

	inactiveUsers, err := client.Find(testCollection, inactiveQuery)
	if err != nil {
		log.Fatalf("Find failed: %v", err)
	}
	fmt.Printf("  Found %d inactive users\n", len(inactiveUsers))
	for _, user := range inactiveUsers {
		name := ekodb.GetStringValue(user["name"])
		bio := ekodb.GetStringValue(user["bio"])
		fmt.Printf("    - %s: %s\n", name, bio)
	}

	// Example 5: Compare full vs projected data - demonstrates bandwidth savings
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

	if len(fullUsers) > 0 && len(projectedUsers) > 0 {
		fullFields := getFieldNames(fullUsers[0])
		projectedFields := getFieldNames(projectedUsers[0])

		fmt.Println("  Full query:")
		fmt.Printf("    - %d fields per record\n", len(fullFields))
		fmt.Printf("    - Fields: %v\n", fullFields)
		fmt.Println("  Projected query:")
		fmt.Printf("    - %d fields per record\n", len(projectedFields))
		fmt.Printf("    - Fields: %v\n", projectedFields)
		savings := 100 - (len(projectedFields) * 100 / max(len(fullFields), 1))
		fmt.Printf("  Bandwidth savings: ~%d%% fewer fields\n", savings)
	}

	// Cleanup
	fmt.Println("\nCleaning up test data...")
	if err := client.DeleteCollection(testCollection); err != nil {
		log.Printf("Warning: Failed to delete collection: %v", err)
	} else {
		fmt.Println("Cleanup complete")
	}

	fmt.Println("\nAll projection examples completed successfully!")
}

// Helper function to get sorted field names from a record
func getFieldNames(record ekodb.Record) []string {
	fields := make([]string, 0, len(record))
	for k := range record {
		fields = append(fields, k)
	}
	sort.Strings(fields)
	return fields
}

// Helper function to return exclusion status string
func excludedStatus(present bool) string {
	if present {
		return "PRESENT (unexpected!)"
	}
	return "excluded"
}

// Helper function for max
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
