// ekoDB Go Client - Advanced CRUD Integration Example
//
// Exercises advanced update and restore operations:
//   UpdateWithAction (increment, push, clear)
//   UpdateWithActionSequence
//   RestoreRecord
//   RestoreCollection

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
	fmt.Print("=== ekoDB Advanced CRUD Integration Example (Go) ===\n\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		return fmt.Errorf("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}

	collection := "advanced_crud_example_go"
	collectionDeleted := false
	defer func() {
		if !collectionDeleted {
			if err := client.DeleteCollection(collection); err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete collection: %w", err))
			}
		}
	}()

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
		return fmt.Errorf("insert: %w", err)
	}
	docID := record["id"].(string)
	fmt.Printf("Inserted record: %s\n", docID)

	// ========================================================================
	// UpdateWithAction: increment
	// ========================================================================

	fmt.Println("\n--- UpdateWithAction: increment ---")
	incremented, err := client.UpdateWithAction(collection, docID, "increment", "count", 5)
	if err != nil {
		return fmt.Errorf("UpdateWithAction (increment): %w", err)
	}
	fmt.Printf("After increment by 5: count = %v\n", incremented["count"])
	if count, ok := ekodb.GetIntValue(incremented["count"]); !ok || count != 5 {
		return fmt.Errorf("increment by 5 returned count %v, expected 5", incremented["count"])
	}

	incremented2, err := client.UpdateWithAction(collection, docID, "increment", "count", 3)
	if err != nil {
		return fmt.Errorf("UpdateWithAction (increment 2): %w", err)
	}
	fmt.Printf("After increment by 3: count = %v\n", incremented2["count"])
	if count, ok := ekodb.GetIntValue(incremented2["count"]); !ok || count != 8 {
		return fmt.Errorf("second increment returned count %v, expected 8", incremented2["count"])
	}

	// ========================================================================
	// UpdateWithAction: push
	// ========================================================================

	fmt.Println("\n--- UpdateWithAction: push ---")
	pushed, err := client.UpdateWithAction(collection, docID, "push", "tags", "new-tag")
	if err != nil {
		return fmt.Errorf("UpdateWithAction (push): %w", err)
	}
	fmt.Printf("After push 'new-tag': tags = %v\n", pushed["tags"])
	if !containsString(pushed["tags"], "new-tag") {
		return fmt.Errorf("push returned tags %v without new-tag", pushed["tags"])
	}

	pushed2, err := client.UpdateWithAction(collection, docID, "push", "scores", 30.0)
	if err != nil {
		return fmt.Errorf("UpdateWithAction (push score): %w", err)
	}
	fmt.Printf("After push 30.0: scores = %v\n", pushed2["scores"])
	if !containsNumber(pushed2["scores"], 30) {
		return fmt.Errorf("push returned scores %v without 30", pushed2["scores"])
	}

	// ========================================================================
	// UpdateWithAction: decrement
	// ========================================================================

	fmt.Println("\n--- UpdateWithAction: decrement ---")
	decremented, err := client.UpdateWithAction(collection, docID, "decrement", "count", 2)
	if err != nil {
		return fmt.Errorf("UpdateWithAction (decrement): %w", err)
	}
	fmt.Printf("After decrement by 2: count = %v\n", decremented["count"])
	if count, ok := ekodb.GetIntValue(decremented["count"]); !ok || count != 6 {
		return fmt.Errorf("decrement returned count %v, expected 6", decremented["count"])
	}

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
		return fmt.Errorf("UpdateWithActionSequence: %w", err)
	}
	fmt.Printf("After sequence: count=%v, tags=%v, scores=%v\n",
		sequenced["count"], sequenced["tags"], sequenced["scores"])
	if count, ok := ekodb.GetIntValue(sequenced["count"]); !ok || count != 16 {
		return fmt.Errorf("action sequence returned count %v, expected 16", sequenced["count"])
	}
	if !containsString(sequenced["tags"], "batch-added") || !containsNumber(sequenced["scores"], 99.9) {
		return fmt.Errorf("action sequence returned unexpected tags/scores: %v / %v", sequenced["tags"], sequenced["scores"])
	}

	/*
		TODO(ekoDB dev team): Re-enable the record and collection restore examples
		after the server can recover deleted record data. The live API currently
		returns status="success", restored=false and removes the tombstone, leaving
		the record unavailable afterward.

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
	*/

	// ========================================================================
	// Cleanup
	// ========================================================================

	fmt.Println("\n--- Cleanup ---")
	if err := client.Delete(collection, docID); err != nil {
		return fmt.Errorf("delete record cleanup: %w", err)
	}
	if err := client.DeleteCollection(collection); err != nil {
		return fmt.Errorf("DeleteCollection cleanup: %w", err)
	}
	collectionDeleted = true
	fmt.Println("Cleaned up all resources")

	fmt.Println("\n=== All advanced CRUD operations completed successfully ===")
	return nil
}

func containsString(value interface{}, expected string) bool {
	values, ok := ekodb.GetValue(value).([]interface{})
	if !ok {
		if strings, ok := ekodb.GetValue(value).([]string); ok {
			for _, item := range strings {
				if item == expected {
					return true
				}
			}
		}
		return false
	}
	for _, item := range values {
		if ekodb.GetStringValue(item) == expected {
			return true
		}
	}
	return false
}

func containsNumber(value interface{}, expected float64) bool {
	values, ok := ekodb.GetValue(value).([]interface{})
	if !ok {
		return false
	}
	for _, item := range values {
		if ekodb.GetFloatValue(item) == expected {
			return true
		}
	}
	return false
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
