// ekoDB Go Client - KV Links Integration Example
//
// Exercises KV link operations:
//   KVSet (create entries) -> KVLink -> KVGetLinks -> KVUnlink

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB KV Links Integration Example (Go) ===\n")

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

	collection := "kv_links_example_go"

	// 1. Insert some documents to link to
	fmt.Println("--- Inserting documents ---")
	doc1, err := client.Insert(collection, ekodb.Record{
		"name":  "Alice",
		"email": "alice@example.com",
	})
	if err != nil {
		log.Fatalf("Insert doc1 failed: %v", err)
	}
	doc1ID := doc1["id"].(string)
	fmt.Printf("Inserted doc1: %s\n", doc1ID)

	doc2, err := client.Insert(collection, ekodb.Record{
		"name":  "Bob",
		"email": "bob@example.com",
	})
	if err != nil {
		log.Fatalf("Insert doc2 failed: %v", err)
	}
	doc2ID := doc2["id"].(string)
	fmt.Printf("Inserted doc2: %s\n", doc2ID)

	// 2. Set a KV entry
	fmt.Println("\n--- Setting KV entry ---")
	if err := client.KVSet("team:engineering", map[string]interface{}{
		"name": "Engineering",
		"lead": "Alice",
	}); err != nil {
		log.Fatalf("KVSet failed: %v", err)
	}
	fmt.Println("KV entry set: team:engineering")

	// 3. Link the KV key to both documents
	fmt.Println("\n--- Linking KV key to documents ---")
	link1, err := client.KVLink("team:engineering", collection, doc1ID)
	if err != nil {
		log.Fatalf("KVLink doc1 failed: %v", err)
	}
	fmt.Printf("Linked doc1: %v\n", link1)

	link2, err := client.KVLink("team:engineering", collection, doc2ID)
	if err != nil {
		log.Fatalf("KVLink doc2 failed: %v", err)
	}
	fmt.Printf("Linked doc2: %v\n", link2)

	// 4. Get all links for the key
	fmt.Println("\n--- Getting links ---")
	links, err := client.KVGetLinks("team:engineering")
	if err != nil {
		log.Fatalf("KVGetLinks failed: %v", err)
	}
	fmt.Printf("Links for team:engineering: %v\n", links)

	// 5. Unlink one document
	fmt.Println("\n--- Unlinking doc2 ---")
	unlinkResult, err := client.KVUnlink("team:engineering", collection, doc2ID)
	if err != nil {
		log.Fatalf("KVUnlink failed: %v", err)
	}
	fmt.Printf("Unlink result: %v\n", unlinkResult)

	// 6. Verify links after unlink
	fmt.Println("\n--- Verifying links after unlink ---")
	linksAfter, err := client.KVGetLinks("team:engineering")
	if err != nil {
		log.Fatalf("KVGetLinks (after unlink) failed: %v", err)
	}
	fmt.Printf("Links after unlink: %v\n", linksAfter)

	// Cleanup
	fmt.Println("\n--- Cleanup ---")
	_ = client.KVDelete("team:engineering")
	_ = client.Delete(collection, doc1ID)
	_ = client.Delete(collection, doc2ID)
	_ = client.DeleteCollection(collection)
	fmt.Println("Cleaned up all resources")

	fmt.Println("\n=== All KV link operations completed successfully ===")
}
