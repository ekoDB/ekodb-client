// Simple WebSocket Operations - Using ekodb-client-go library
//
// This example demonstrates WebSocket queries using the Go client library.
// Compare with ttl-caching/simple_websocket.go to see the difference!

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

	wsURL := os.Getenv("WS_BASE_URL")
	if wsURL == "" {
		wsURL = "ws://localhost:8080"
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

	collection := "client_simple_websocket_go"

	// Step 1: Insert test data via HTTP
	fmt.Println("\n=== Inserting Test Data ===")
	record := ekodb.Record{
		"name":   "WebSocket Test Record",
		"value":  42,
		"active": true,
	}
	inserted, err := client.Insert(collection, record)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Inserted test record: %v\n", inserted["id"])

	// Step 2: Connect to WebSocket
	fmt.Println("\n=== Connecting to WebSocket ===")
	ws, err := client.WebSocket(wsURL)
	if err != nil {
		log.Fatal(err)
	}
	defer ws.Close()
	fmt.Println("✓ WebSocket connected")

	// Step 3: Query data via WebSocket
	fmt.Println("\n=== Querying Data via WebSocket ===")
	records, err := ws.FindAll(collection)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Retrieved %d record(s) via WebSocket\n", len(records))

	// Show some data
	for i, rec := range records {
		if i >= 3 {
			break
		}
		fmt.Printf("  Record %d: %d fields\n", i+1, len(rec))
	}

	// Cleanup: Delete the collection
	fmt.Println("\n=== Cleanup ===")
	if err := client.DeleteCollection(collection); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Deleted collection")

	fmt.Println("\n✓ WebSocket example completed successfully")
}
