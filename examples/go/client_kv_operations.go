// Key-Value Operations - Using ekodb-client-go library
//
// This example demonstrates KV operations using the Go client library.
// Compare with ttl-caching/kv_operations.go to see the difference!

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

	// Example 3: Set multiple keys
	fmt.Println("\n=== Set Multiple Keys ===")
	keys := []string{"cache:product:1", "cache:product:2", "cache:product:3"}
	for i, key := range keys {
		data := map[string]interface{}{
			"name":  fmt.Sprintf("Product %d", i+1),
			"price": 29.99 + float64(i*10),
		}
		if err := client.KVSet(key, data); err != nil {
			log.Fatal(err)
		}
	}
	fmt.Printf("✓ Set %d keys\n", len(keys))

	// Example 4: Get multiple keys
	fmt.Println("\n=== Get Multiple Keys ===")
	for _, key := range keys {
		value, err := client.KVGet(key)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("%s: %v\n", key, value)
	}

	// Example 5: Delete a key
	fmt.Println("\n=== KV Delete ===")
	if err := client.KVDelete("session:user123"); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Deleted key: session:user123")

	// Verify deletion
	_, err = client.KVGet("session:user123")
	if err != nil {
		fmt.Println("✓ Verified: Key successfully deleted (not found)")
	}

	// Example 6: Delete multiple keys
	fmt.Println("\n=== Delete Multiple Keys ===")
	for _, key := range keys {
		if err := client.KVDelete(key); err != nil {
			log.Fatal(err)
		}
	}
	fmt.Printf("✓ Deleted %d keys\n", len(keys))

	fmt.Println("\n✓ All KV operations completed successfully")
}
