// Key-Value Operations - Using ekodb-client-go library
//
// This example demonstrates KV operations using the Go client library.
// Compare with kv_operations.go to see the difference!

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

	// Example 1: Set a key
	fmt.Println("\n=== KV Set ===")
	sessionData := map[string]interface{}{
		"userId":   123,
		"username": "john_doe",
	}
	if err := client.KVSet("session:user123", sessionData); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Set key: session:user123")

	// Example 2: Get a key
	fmt.Println("\n=== KV Get ===")
	value, err := client.KVGet("session:user123")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Retrieved value: %v\n", value)

	// Example 3: Batch set multiple keys
	fmt.Println("\n=== KV Batch Set ===")
	batchEntries := []map[string]interface{}{
		{"key": "cache:product:1", "value": map[string]interface{}{"name": "Product 1", "price": 29.99}},
		{"key": "cache:product:2", "value": map[string]interface{}{"name": "Product 2", "price": 39.99}},
		{"key": "cache:product:3", "value": map[string]interface{}{"name": "Product 3", "price": 49.99}},
	}
	setResults, err := client.KVBatchSet(batchEntries)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Batch set %d keys\n", len(setResults))
	for _, result := range setResults {
		key := result[0].(string)
		wasSet := result[1].(bool)
		status := "failed"
		if wasSet {
			status = "success"
		}
		fmt.Printf("  %s: %s\n", key, status)
	}

	// Example 4: Batch get multiple keys
	fmt.Println("\n=== KV Batch Get ===")
	keys := []string{"cache:product:1", "cache:product:2", "cache:product:3"}
	batchValues, err := client.KVBatchGet(keys)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Batch retrieved %d values\n", len(batchValues))
	for i, val := range batchValues {
		fmt.Printf("  %s: %v\n", keys[i], val)
	}

	// Example 5: Check if key exists
	fmt.Println("\n=== KV Exists ===")
	exists, err := client.KVExists("session:user123")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Key exists: %v\n", exists)

	// Example 6: Find keys with pattern
	fmt.Println("\n=== KV Find (Pattern Query) ===")
	cacheResults, err := client.KVFind("cache:product:.*", false)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Found %d keys matching 'cache:product:.*'\n", len(cacheResults))

	// Example 7: Query all keys
	fmt.Println("\n=== KV Query (Alias for Find) ===")
	allResults, err := client.KVQuery("", false)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Total keys in store: %d\n", len(allResults))

	// Example 8: Delete a key
	fmt.Println("\n=== KV Delete ===")
	if err := client.KVDelete("session:user123"); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Deleted key: session:user123")

	// Verify deletion with KVExists
	existsAfter, _ := client.KVExists("session:user123")
	fmt.Printf("✓ Verified: Key exists after delete: %v\n", existsAfter)

	// Example 9: Batch delete multiple keys
	fmt.Println("\n=== KV Batch Delete ===")
	deleteResults, err := client.KVBatchDelete(keys)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Batch deleted %d keys\n", len(deleteResults))
	for _, result := range deleteResults {
		key := result[0].(string)
		wasDeleted := result[1].(bool)
		status := "not found"
		if wasDeleted {
			status = "deleted"
		}
		fmt.Printf("  %s: %s\n", key, status)
	}

	fmt.Println("\n✓ All KV operations completed successfully")
}
