// Collection Utilities Example - Using ekodb-client-go library
//
// This example demonstrates collection utility methods:
// - CollectionExists: Check if a collection exists
// - CountDocuments: Count documents in a collection

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

	collection := "collection_utils_test"

	// Example 1: Check if collection exists (should be false initially)
	fmt.Println("\n=== Check Collection Exists (Before Creation) ===")
	exists, err := client.CollectionExists(collection)
	if err != nil {
		log.Printf("CollectionExists error: %v", err)
	} else {
		fmt.Printf("Collection '%s' exists: %v\n", collection, exists)
	}

	// Example 2: Create some test documents
	fmt.Println("\n=== Creating Test Documents ===")
	for i := 1; i <= 5; i++ {
		record := ekodb.Record{
			"name":  fmt.Sprintf("Test Document %d", i),
			"index": i,
		}
		_, err := client.Insert(collection, record)
		if err != nil {
			log.Printf("Insert error: %v", err)
		}
	}
	fmt.Println("Created 5 test documents")

	// Example 3: Check if collection exists now (should be true)
	fmt.Println("\n=== Check Collection Exists (After Creation) ===")
	exists, err = client.CollectionExists(collection)
	if err != nil {
		log.Printf("CollectionExists error: %v", err)
	} else {
		fmt.Printf("Collection '%s' exists: %v\n", collection, exists)
	}

	// Example 4: Count documents in collection
	fmt.Println("\n=== Count Documents ===")
	count, err := client.CountDocuments(collection)
	if err != nil {
		log.Printf("CountDocuments error: %v", err)
	} else {
		fmt.Printf("Document count in '%s': %d\n", collection, count)
	}

	// Example 5: Check non-existent collection
	fmt.Println("\n=== Check Non-Existent Collection ===")
	exists, err = client.CollectionExists("nonexistent_collection_xyz")
	if err != nil {
		log.Printf("CollectionExists error: %v", err)
	} else {
		fmt.Printf("Collection 'nonexistent_collection_xyz' exists: %v\n", exists)
	}

	// Cleanup: Delete the test collection
	fmt.Println("\n=== Cleanup ===")
	err = client.DeleteCollection(collection)
	if err != nil {
		log.Printf("DeleteCollection error: %v", err)
	} else {
		fmt.Printf("Deleted collection '%s'\n", collection)
	}

	fmt.Println("\n✓ Collection Utilities example complete")
}
