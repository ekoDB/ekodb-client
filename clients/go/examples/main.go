package main

import (
	"context"
	"fmt"
	"log"

	"github.com/ekodb/ekodb-client/clients/go/ekodb" // This will be replaced
)

func main() {
	// Initialize client
	client := ekodb.NewClient(&ekodb.ClientConfig{
		BaseURL: "http://localhost:8080",
		APIKey:  "test-api-key",
	})

	// Create a test record
	record := ekodb.Record{
		"name": ekodb.TypedValue{Type: ekodb.StringType, Value: "Test User"},
		"age":  ekodb.TypedValue{Type: ekodb.IntegerType, Value: 25},
	}

	// Insert the record
	ctx := context.Background()
	id, err := client.Insert(ctx, "users", record)
	if err != nil {
		log.Fatalf("Failed to insert record: %v", err)
	}
	fmt.Printf("Inserted record with ID: %s\n", id)

	// Retrieve the record
	result, err := client.FindByID(ctx, "users", id)
	if err != nil {
		log.Fatalf("Failed to retrieve record: %v", err)
	}
	fmt.Printf("Retrieved record: %+v\n", result)
}
