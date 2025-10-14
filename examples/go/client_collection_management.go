// Collection Management - Using ekodb-client-go library
//
// This example demonstrates collection management using the Go client library.
// Compare with ttl-caching/collection_management.go to see the difference!

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

	collection := "client_collection_management_go"

	// Example 1: Create collection (via insert)
	fmt.Println("\n=== Create Collection (via insert) ===")
	record := ekodb.Record{
		"name":  "Test Collection",
		"value": "Initial record",
	}
	inserted, err := client.Insert(collection, record)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Collection created with first record: %v\n", inserted["id"])

	// Example 2: List collections
	fmt.Println("\n=== List Collections ===")
	collections, err := client.ListCollections()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Total collections: %d\n", len(collections))

	// Show first 5
	displayCount := 5
	if len(collections) < displayCount {
		displayCount = len(collections)
	}
	fmt.Printf("Sample collections: %v\n", collections[:displayCount])

	// Example 3: Count documents in collection
	fmt.Println("\n=== Count Documents ===")
	limit := 1000
	docs, err := client.Find(collection, ekodb.Query{Limit: &limit})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Document count: %d\n", len(docs))

	// Example 4: Delete collection
	fmt.Println("\n=== Delete Collection ===")
	if err := client.DeleteCollection(collection); err != nil {
		log.Fatal(err)
	}
	fmt.Println("Collection deleted successfully")

	// Example 5: Verify deletion
	fmt.Println("\n=== Verify Deletion ===")
	collections, err = client.ListCollections()
	if err != nil {
		log.Fatal(err)
	}

	exists := false
	for _, c := range collections {
		if c == collection {
			exists = true
			break
		}
	}
	fmt.Printf("Collection still exists: %v\n", exists)

	fmt.Println("\n✓ All collection management operations completed successfully")
}
