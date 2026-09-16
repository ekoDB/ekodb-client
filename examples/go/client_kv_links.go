// ekoDB Go Client - KV Links Integration Example
//
// Exercises KV link operations:
//   KVSet (create entries) -> KVLink -> KVGetLinks -> KVUnlink

package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

const kvLinksKey = "team:engineering:go"

func run() (runErr error) {
	fmt.Print("=== ekoDB KV Links Integration Example (Go) ===\n\n")

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

	collection := "kv_links_example_go"
	kvDeleted := false
	collectionDeleted := false
	defer func() {
		if !kvDeleted {
			if err := client.KVDelete(kvLinksKey); err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("cleanup KV key %s: %w", kvLinksKey, err))
			}
		}
		if !collectionDeleted {
			if err := client.DeleteCollection(collection); err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("cleanup collection %s: %w", collection, err))
			}
		}
	}()

	// 1. Insert some documents to link to
	fmt.Println("--- Inserting documents ---")
	doc1, err := client.Insert(collection, ekodb.Record{
		"name":  "Alice",
		"email": "alice@example.com",
	})
	if err != nil {
		return fmt.Errorf("insert doc1: %w", err)
	}
	doc1ID := doc1["id"].(string)
	fmt.Printf("Inserted doc1: %s\n", doc1ID)

	doc2, err := client.Insert(collection, ekodb.Record{
		"name":  "Bob",
		"email": "bob@example.com",
	})
	if err != nil {
		return fmt.Errorf("insert doc2: %w", err)
	}
	doc2ID := doc2["id"].(string)
	fmt.Printf("Inserted doc2: %s\n", doc2ID)

	// 2. Set a KV entry
	fmt.Println("\n--- Setting KV entry ---")
	if err := client.KVSet(kvLinksKey, map[string]interface{}{
		"name": "Engineering",
		"lead": "Alice",
	}); err != nil {
		return fmt.Errorf("KVSet: %w", err)
	}
	fmt.Printf("KV entry set: %s\n", kvLinksKey)

	// 3. Link the KV key to both documents
	fmt.Println("\n--- Linking KV key to documents ---")
	link1, err := client.KVLink(kvLinksKey, collection, doc1ID)
	if err != nil {
		return fmt.Errorf("KVLink doc1: %w", err)
	}
	fmt.Printf("Linked doc1: %v\n", link1)

	link2, err := client.KVLink(kvLinksKey, collection, doc2ID)
	if err != nil {
		return fmt.Errorf("KVLink doc2: %w", err)
	}
	fmt.Printf("Linked doc2: %v\n", link2)

	// 4. Get all links for the key
	fmt.Println("\n--- Getting links ---")
	links, err := client.KVGetLinks(kvLinksKey)
	if err != nil {
		return fmt.Errorf("KVGetLinks: %w", err)
	}
	fmt.Printf("Links for %s: %v\n", kvLinksKey, links)

	// 5. Unlink one document
	fmt.Println("\n--- Unlinking doc2 ---")
	unlinkResult, err := client.KVUnlink(kvLinksKey, collection, doc2ID)
	if err != nil {
		return fmt.Errorf("KVUnlink: %w", err)
	}
	fmt.Printf("Unlink result: %v\n", unlinkResult)

	// 6. Verify links after unlink
	fmt.Println("\n--- Verifying links after unlink ---")
	linksAfter, err := client.KVGetLinks(kvLinksKey)
	if err != nil {
		return fmt.Errorf("KVGetLinks after unlink: %w", err)
	}
	fmt.Printf("Links after unlink: %v\n", linksAfter)

	// Cleanup
	fmt.Println("\n--- Cleanup ---")
	if _, err := client.KVUnlink(kvLinksKey, collection, doc1ID); err != nil {
		return fmt.Errorf("KVUnlink remaining doc1: %w", err)
	}
	if err := client.KVDelete(kvLinksKey); err != nil {
		return fmt.Errorf("delete KV key: %w", err)
	}
	kvDeleted = true
	if err := client.DeleteCollection(collection); err != nil {
		return fmt.Errorf("delete collection: %w", err)
	}
	collectionDeleted = true
	fmt.Println("Cleaned up all resources")

	fmt.Println("\n=== All KV link operations completed successfully ===")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
