package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

const bypassRippleCollection = "bypass_ripple_products_go"

// Example demonstrating bypass_ripple usage with the new options API
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

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return err
	}

	collection := bypassRippleCollection
	defer func() {
		if err := client.DeleteCollection(collection); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup collection %s: %w", collection, err))
		}
	}()
	bypass := true

	// Basic insert (no bypass)
	record1 := ekodb.Record{"name": "Product 1", "price": 100}
	result1, err := client.Insert(collection, record1)
	if err != nil {
		return fmt.Errorf("insert with ripple: %w", err)
	}
	fmt.Printf("Inserted with ripple: %v\n", result1)

	// Insert with bypass_ripple
	record2 := ekodb.Record{"name": "Product 2", "price": 200}
	result2, err := client.Insert(collection, record2, ekodb.InsertOptions{
		BypassRipple: &bypass,
	})
	if err != nil {
		return fmt.Errorf("insert with bypass_ripple: %w", err)
	}
	fmt.Printf("Inserted with bypass_ripple: %v\n", result2)

	// Insert with TTL and bypass_ripple
	record3 := ekodb.Record{"name": "Product 3", "price": 300}
	result3, err := client.Insert(collection, record3, ekodb.InsertOptions{
		TTL:          "1h",
		BypassRipple: &bypass,
	})
	if err != nil {
		return fmt.Errorf("insert with TTL and bypass_ripple: %w", err)
	}
	fmt.Printf("Inserted with TTL and bypass_ripple: %v\n", result3)

	// Update with bypass_ripple
	id := result1["id"].(string)
	updateRecord := ekodb.Record{"price": 150}
	result4, err := client.Update(collection, id, updateRecord, ekodb.UpdateOptions{
		BypassRipple: &bypass,
	})
	if err != nil {
		return fmt.Errorf("update with bypass_ripple: %w", err)
	}
	fmt.Printf("Updated with bypass_ripple: %v\n", result4)

	// Delete with bypass_ripple
	if err := client.Delete(collection, id, ekodb.DeleteOptions{
		BypassRipple: &bypass,
	}); err != nil {
		return fmt.Errorf("delete with bypass_ripple: %w", err)
	}
	fmt.Println("Deleted with bypass_ripple")

	// Batch operations with bypass_ripple
	records := []ekodb.Record{
		{"name": "Batch 1", "price": 10},
		{"name": "Batch 2", "price": 20},
	}
	batchResults, err := client.BatchInsert(collection, records, ekodb.BatchInsertOptions{
		BypassRipple: &bypass,
	})
	if err != nil {
		return fmt.Errorf("batch insert with bypass_ripple: %w", err)
	}
	fmt.Printf("Batch inserted with bypass_ripple: %d records\n", len(batchResults))

	// Upsert with bypass_ripple
	upsertRecord := ekodb.Record{"name": "Upsert Product", "price": 500}
	upsertResult, err := client.Upsert(collection, "custom-id", upsertRecord, ekodb.UpsertOptions{
		BypassRipple: &bypass,
	})
	if err != nil {
		return fmt.Errorf("upsert with bypass_ripple: %w", err)
	}
	fmt.Printf("Upserted with bypass_ripple: %v\n", upsertResult)
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
