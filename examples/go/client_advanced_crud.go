// ekoDB Go Client - Advanced CRUD Integration Example
//
// Exercises advanced update and restore operations:
//   UpdateWithAction (increment, push, clear)
//   UpdateWithActionSequence
//   RestoreRecord
//   RestoreCollection

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Advanced CRUD Integration Example (Go) ===\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		log.Fatal("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	collection := "advanced_crud_example_go"

	// ========================================================================
	// Setup: Insert a base record
	// ========================================================================

	fmt.Println("--- Inserting base record ---")
	record, err := client.Insert(collection, ekodb.Record{
		"name":   "Counter",
		"count":  0,
		"tags":   []string{"initial"},
		"scores": []float64{10.0, 20.0},
	})
	if err != nil {
		log.Fatalf("Insert failed: %v", err)
	}
	docID := record["id"].(string)
	fmt.Printf("Inserted record: %s\n", docID)

	// ========================================================================
	// UpdateWithAction: increment
	// ========================================================================

	fmt.Println("\n--- UpdateWithAction: increment ---")
	incremented, err := client.UpdateWithAction(collection, docID, "increment", "count", 5)
	if err != nil {
		log.Fatalf("UpdateWithAction (increment) failed: %v", err)
	}
	fmt.Printf("After increment by 5: count = %v\n", incremented["count"])

	incremented2, err := client.UpdateWithAction(collection, docID, "increment", "count", 3)
	if err != nil {
		log.Fatalf("UpdateWithAction (increment 2) failed: %v", err)
	}
	fmt.Printf("After increment by 3: count = %v\n", incremented2["count"])

	// ========================================================================
	// UpdateWithAction: push
	// ========================================================================

	fmt.Println("\n--- UpdateWithAction: push ---")
	pushed, err := client.UpdateWithAction(collection, docID, "push", "tags", "new-tag")
	if err != nil {
		log.Fatalf("UpdateWithAction (push) failed: %v", err)
	}
	fmt.Printf("After push 'new-tag': tags = %v\n", pushed["tags"])

	pushed2, err := client.UpdateWithAction(collection, docID, "push", "scores", 30.0)
	if err != nil {
		log.Fatalf("UpdateWithAction (push score) failed: %v", err)
	}
	fmt.Printf("After push 30.0: scores = %v\n", pushed2["scores"])

	// ========================================================================
	// UpdateWithAction: decrement
	// ========================================================================

	fmt.Println("\n--- UpdateWithAction: decrement ---")
	decremented, err := client.UpdateWithAction(collection, docID, "decrement", "count", 2)
	if err != nil {
		log.Fatalf("UpdateWithAction (decrement) failed: %v", err)
	}
	fmt.Printf("After decrement by 2: count = %v\n", decremented["count"])

	// ========================================================================
	// UpdateWithActionSequence: multiple atomic actions
	// ========================================================================

	fmt.Println("\n--- UpdateWithActionSequence ---")
	// Atomically: increment count by 10, push a tag, and push a score
	actions := [][3]interface{}{
		{"increment", "count", 10},
		{"push", "tags", "batch-added"},
		{"push", "scores", 99.9},
	}
	sequenced, err := client.UpdateWithActionSequence(collection, docID, actions)
	if err != nil {
		log.Fatalf("UpdateWithActionSequence failed: %v", err)
	}
	fmt.Printf("After sequence: count=%v, tags=%v, scores=%v\n",
		sequenced["count"], sequenced["tags"], sequenced["scores"])

	// ========================================================================
	// RestoreRecord: delete then restore
	// ========================================================================

	fmt.Println("\n--- RestoreRecord ---")

	// Insert a record to delete and restore
	toRestore, err := client.Insert(collection, ekodb.Record{
		"name": "Ephemeral",
	})
	if err != nil {
		log.Fatalf("Insert (for restore) failed: %v", err)
	}
	restoreID := toRestore["id"].(string)
	fmt.Printf("Inserted record for restore: %s\n", restoreID)

	// Delete it
	if err := client.Delete(collection, restoreID); err != nil {
		log.Fatalf("Delete (for restore) failed: %v", err)
	}
	fmt.Println("Record deleted")

	// Restore it
	if err := client.RestoreRecord(collection, restoreID); err != nil {
		log.Fatalf("RestoreRecord failed: %v", err)
	}
	fmt.Println("Record restored")

	// Verify it's back
	restored, err := client.FindByID(collection, restoreID)
	if err != nil {
		log.Fatalf("FindByID after restore failed: %v", err)
	}
	fmt.Printf("Restored record name: %v\n", ekodb.GetStringValue(restored["name"]))

	// ========================================================================
	// RestoreCollection: delete multiple then restore all
	// ========================================================================

	fmt.Println("\n--- RestoreCollection ---")

	// Insert two records
	r1, err := client.Insert(collection, ekodb.Record{"name": "Restore-A"})
	if err != nil {
		log.Fatalf("Insert (restore-a) failed: %v", err)
	}
	r1ID := r1["id"].(string)

	r2, err := client.Insert(collection, ekodb.Record{"name": "Restore-B"})
	if err != nil {
		log.Fatalf("Insert (restore-b) failed: %v", err)
	}
	r2ID := r2["id"].(string)

	// Delete both
	_ = client.Delete(collection, r1ID)
	_ = client.Delete(collection, r2ID)
	fmt.Println("Deleted 2 records")

	// Restore the entire collection's trash
	restoredCount, err := client.RestoreCollection(collection)
	if err != nil {
		log.Fatalf("RestoreCollection failed: %v", err)
	}
	fmt.Printf("Restored %d records from trash\n", restoredCount)

	// ========================================================================
	// Cleanup
	// ========================================================================

	fmt.Println("\n--- Cleanup ---")
	_ = client.Delete(collection, docID)
	_ = client.Delete(collection, restoreID)
	_ = client.Delete(collection, r1ID)
	_ = client.Delete(collection, r2ID)
	_ = client.DeleteCollection(collection)
	fmt.Println("Cleaned up all resources")

	fmt.Println("\n=== All advanced CRUD operations completed successfully ===")
}
