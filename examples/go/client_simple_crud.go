// Simple CRUD Operations - Using ekodb-client-go library
//
// This example demonstrates CRUD operations using the Go client library.
// Compare with ttl-caching/simple_crud.go to see the difference!

package main

import (
	"encoding/base64"
	"fmt"
	"log"
	"os"
	"time"

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

	// Example 1: Insert a document with various field types
	fmt.Println("\n=== Insert Document ===")
	record := ekodb.Record{
		"name":       "Test Record",
		"value":      42,
		"active":     true,
		"price":      99.99,
		"created_at": time.Now().Format(time.RFC3339),
		"user_id":    "550e8400-e29b-41d4-a716-446655440000",
		"tags":       []string{"tag1", "tag2", "tag3"},
		"metadata":   map[string]interface{}{"key": "value", "nested": map[string]interface{}{"deep": true}},
		"embedding":  []float64{0.1, 0.2, 0.3, 0.4, 0.5},
		"categories": []string{"electronics", "computers"},
		"data":       base64.StdEncoding.EncodeToString([]byte("hello world")),
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

	// Example 2b: Extract field values using type-specific getValue utilities
	fmt.Println("\n=== Extract Field Values (All Types) ===")
	// ekoDB returns fields as map[string]interface{}{"type": "String", "value": "actual_value"}
	// Use type-specific getValue utilities to extract values
	name := ekodb.GetStringValue(found["name"])
	valueNum, _ := ekodb.GetIntValue(found["value"])
	active := ekodb.GetBoolValue(found["active"])
	price := ekodb.GetDecimalValue(found["price"])
	createdAt := ekodb.GetDateTimeValue(found["created_at"])
	userId := ekodb.GetUUIDValue(found["user_id"])
	tags := ekodb.GetArrayValue(found["tags"])
	metadata := ekodb.GetObjectValue(found["metadata"])
	embedding := ekodb.GetVectorValue(found["embedding"])
	categories := ekodb.GetSetValue(found["categories"])
	data := ekodb.GetBytesValue(found["data"])

	fmt.Println("Extracted values:")
	fmt.Printf("  name (String): %s\n", name)
	fmt.Printf("  value (Integer): %d\n", valueNum)
	fmt.Printf("  active (Boolean): %v\n", active)
	fmt.Printf("  price (Decimal): %f\n", price)
	fmt.Printf("  created_at (DateTime): %v\n", createdAt)
	fmt.Printf("  user_id (UUID): %s\n", userId)
	fmt.Printf("  tags (Array): %v\n", tags)
	fmt.Printf("  metadata (Object): %v\n", metadata)
	fmt.Printf("  embedding (Vector): %v\n", embedding)
	fmt.Printf("  categories (Set): %v\n", categories)
	fmt.Printf("  data (Bytes): %d bytes\n", len(data))

	// Or extract all fields at once using ExtractRecord()
	plainRecord := ekodb.ExtractRecord(found)
	fmt.Printf("Plain record: %v\n", plainRecord)

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
