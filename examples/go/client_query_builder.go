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

	// Create client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	fmt.Println("=== Query Builder Examples ===\n")

	collection := "query_builder_client_go"

	// Cleanup any existing test collection
	_ = client.DeleteCollection(collection)

	// Setup: Insert test data
	fmt.Println("Setting up test data...")

	testUsers := []ekodb.Record{
		{
			"name":    "Alice Johnson",
			"email":   "alice@example.com",
			"age":     28,
			"status":  "active",
			"role":    "user",
			"country": "USA",
		},
		{
			"name":    "Bob Smith",
			"email":   "bob@example.com",
			"age":     35,
			"status":  "active",
			"role":    "admin",
			"country": "Canada",
		},
		{
			"name":    "Charlie Brown",
			"email":   "charlie@test.com",
			"age":     42,
			"status":  "inactive",
			"role":    "moderator",
			"country": "UK",
		},
	}

	for _, user := range testUsers {
		if _, err := client.Insert(collection, user); err != nil {
			log.Printf("Warning: Insert failed (may already exist): %v", err)
		}
	}
	fmt.Println("✅ Test data created\n")

	// Example 1: Simple equality query
	fmt.Println("1. Simple equality query:")
	query1 := ekodb.NewQueryBuilder().
		Eq("status", "active").
		Limit(5).
		Build()

	results1, err := client.Find(collection, query1)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d active users\n\n", len(results1))
	}

	// Example 2: Range query with sorting
	fmt.Println("2. Range query with sorting:")
	query2 := ekodb.NewQueryBuilder().
		Gte("age", 18).
		Lte("age", 65).
		SortDescending("age").
		Limit(10).
		Build()

	results2, err := client.Find(collection, query2)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d users aged 18-65\n\n", len(results2))
	}

	// Example 3: String operations
	fmt.Println("3. String operations:")
	query3 := ekodb.NewQueryBuilder().
		Contains("email", "@example.com").
		SortAscending("name").
		Build()

	results3, err := client.Find(collection, query3)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d users with @example.com emails\n\n", len(results3))
	}

	// Example 4: IN operator
	fmt.Println("4. IN operator:")
	query4 := ekodb.NewQueryBuilder().
		In("role", []interface{}{"admin", "moderator", "editor"}).
		Limit(20).
		Build()

	results4, err := client.Find(collection, query4)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d privileged users\n\n", len(results4))
	}

	// Example 5: Complex query with multiple conditions
	fmt.Println("5. Complex query with multiple conditions:")
	query5 := ekodb.NewQueryBuilder().
		Eq("status", "active").
		Gt("age", 21).
		StartsWith("country", "US").
		Limit(15).
		Build()

	results5, err := client.Find(collection, query5)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d active US users over 21\n\n", len(results5))
	}

	// Example 6: Pagination
	fmt.Println("6. Pagination:")
	query6 := ekodb.NewQueryBuilder().
		Eq("status", "active").
		Page(0, 10).
		SortAscending("name").
		Build()

	results6, err := client.Find(collection, query6)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Page 1: %d users\n\n", len(results6))
	}

	// Example 7: NOT IN operator
	fmt.Println("7. NOT IN operator:")
	query7 := ekodb.NewQueryBuilder().
		Nin("status", []interface{}{"deleted", "banned", "suspended"}).
		Limit(20).
		Build()

	results7, err := client.Find(collection, query7)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d valid users\n\n", len(results7))
	}

	// Example 8: Using bypass flags
	fmt.Println("8. Using bypass flags:")
	query8 := ekodb.NewQueryBuilder().
		Eq("status", "active").
		BypassCache(true).
		Limit(5).
		Build()

	results8, err := client.Find(collection, query8)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d users (bypassed cache)\n\n", len(results8))
	}

	// Cleanup
	fmt.Println("=== Cleanup ===")
	if err := client.DeleteCollection(collection); err != nil {
		log.Printf("Warning: Cleanup failed: %v", err)
	} else {
		fmt.Println("✅ Deleted test collection")
	}

	fmt.Println("\n✅ Query Builder examples completed!")
}
