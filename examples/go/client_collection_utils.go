// Collection Utilities Example - Using ekodb-client-go library
//
// This example demonstrates collection utility methods:
// - CollectionExists: Check if a collection exists
// - CountDocuments: Count documents in a collection

package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func run() (runErr error) {
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
		return err
	}
	fmt.Println("✓ Client created")

	collection := "collection_utils_test_go"
	collectionDeleted := false
	defer func() {
		if collectionDeleted {
			return
		}
		if err := client.DeleteCollection(collection); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup collection %s: %w", collection, err))
		}
	}()

	// Example 1: Check if collection exists (should be false initially)
	fmt.Println("\n=== Check Collection Exists (Before Creation) ===")
	exists, err := client.CollectionExists(collection)
	if err != nil {
		return fmt.Errorf("check collection before creation: %w", err)
	}
	fmt.Printf("Collection '%s' exists: %v\n", collection, exists)

	// Example 2: Create some test documents
	fmt.Println("\n=== Creating Test Documents ===")
	for i := 1; i <= 5; i++ {
		record := ekodb.Record{
			"name":  fmt.Sprintf("Test Document %d", i),
			"index": i,
		}
		_, err := client.Insert(collection, record)
		if err != nil {
			return fmt.Errorf("insert document %d: %w", i, err)
		}
	}
	fmt.Println("Created 5 test documents")

	// Example 3: Check if collection exists now (should be true)
	fmt.Println("\n=== Check Collection Exists (After Creation) ===")
	exists, err = client.CollectionExists(collection)
	if err != nil {
		return fmt.Errorf("check collection after creation: %w", err)
	}
	fmt.Printf("Collection '%s' exists: %v\n", collection, exists)

	// Example 4: Count documents in collection
	fmt.Println("\n=== Count Documents ===")
	count, err := client.CountDocuments(collection)
	if err != nil {
		return fmt.Errorf("count documents: %w", err)
	}
	fmt.Printf("Document count in '%s': %d\n", collection, count)

	// Example 5: Check non-existent collection
	fmt.Println("\n=== Check Non-Existent Collection ===")
	exists, err = client.CollectionExists("nonexistent_collection_xyz")
	if err != nil {
		return fmt.Errorf("check nonexistent collection: %w", err)
	}
	fmt.Printf("Collection 'nonexistent_collection_xyz' exists: %v\n", exists)

	// Cleanup: Delete the test collection
	fmt.Println("\n=== Cleanup ===")
	err = client.DeleteCollection(collection)
	if err != nil {
		return fmt.Errorf("delete collection: %w", err)
	}
	fmt.Printf("Deleted collection '%s'\n", collection)
	collectionDeleted = true

	fmt.Println("\n✓ Collection Utilities example complete")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
