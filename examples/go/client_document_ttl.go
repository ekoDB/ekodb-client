// Document TTL - Using ekodb-client-go library
//
// This example demonstrates documents with TTL using the Go client library.
// Compare with document_ttl.go to see the difference!

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

	collection := "client_document_ttl_go"

	// Example 1: Insert document with TTL (1 hour)
	fmt.Println("\n=== Insert Document with TTL (1 hour) ===")
	record1 := ekodb.Record{
		"name":  "Cache Entry 1",
		"value": "This will expire in 1 hour",
	}
	doc1, err := client.Insert(collection, record1, "1h")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Inserted document: %v\n", doc1["id"])

	// Example 2: Insert document with TTL (5 minutes)
	fmt.Println("\n=== Insert Document with TTL (5 minutes) ===")
	record2 := ekodb.Record{
		"name":  "Cache Entry 2",
		"value": "This will expire in 5 minutes",
	}
	doc2, err := client.Insert(collection, record2, "5m")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Inserted document: %v\n", doc2["id"])

	// Example 3: Query documents
	fmt.Println("\n=== Query Documents ===")
	limit := 100
	docs, err := client.Find(collection, ekodb.Query{Limit: &limit})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Found %d documents with TTL\n", len(docs))

	// Example 4: Update document (TTL persists)
	fmt.Println("\n=== Update Document ===")
	if doc1ID, ok := doc1["id"].(string); ok {
		updateData := ekodb.Record{
			"name":  "Updated Cache Entry",
			"value": "Updated value",
		}
		_, err := client.Update(collection, doc1ID, updateData)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println("✓ Updated document")
	}

	// Example 5: Delete document
	fmt.Println("\n=== Delete Document ===")
	if doc1ID, ok := doc1["id"].(string); ok {
		if err := client.Delete(collection, doc1ID); err != nil {
			log.Fatal(err)
		}
		fmt.Println("✓ Deleted document")
	}

	// Cleanup: Delete the collection
	fmt.Println("\n=== Cleanup ===")
	if err := client.DeleteCollection(collection); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Deleted collection")

	fmt.Println("\n✓ All document TTL operations completed successfully")
	fmt.Println("\n💡 Note: Documents with TTL will automatically expire after the specified duration")
}
