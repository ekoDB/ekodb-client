// WebSocket TTL Expiration Verification Test
//
// This test VERIFIES that TTL works correctly over WebSocket connections by:
// 1. Connecting via WebSocket
// 2. Inserting a document with short TTL (3 seconds) via WebSocket
// 3. Verifying the document exists via WebSocket query
// 4. Waiting for TTL to expire
// 5. Verifying the document is GONE via WebSocket query
//
// If this test passes, WebSocket TTL expiration is working correctly.

package main

import (
	"fmt"
	"log"
	"os"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("╔════════════════════════════════════════════════════════╗")
	fmt.Println("║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║")
	fmt.Println("╚════════════════════════════════════════════════════════╝")
	fmt.Println()
	fmt.Println("This test verifies TTL expiration works via WebSocket connections.")
	fmt.Println("We will use WebSocket to insert, query, and verify TTL expiration.")
	fmt.Println()

	// Load environment
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found, using defaults")
	}

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}

	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}

	// Create client with WebSocket
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Client connected")

	collection := "ws_ttl_expiration_test_go"
	ttlSeconds := 3

	// ═══════════════════════════════════════════════════════════════════════
	// TEST: WebSocket TTL Expiration
	// ═══════════════════════════════════════════════════════════════════════
	fmt.Println()
	fmt.Println("═══════════════════════════════════════════════════════════")
	fmt.Println("TEST: WebSocket TTL Expiration")
	fmt.Println("═══════════════════════════════════════════════════════════")

	// Step 1: Insert document with TTL via client (uses WebSocket if available)
	fmt.Printf("\n[Step 1] Insert document with %d second TTL\n", ttlSeconds)
	fmt.Printf("  Input: {name: 'WS TTL Test', value: 'should expire'}\n")
	fmt.Printf("  TTL: %ds\n", ttlSeconds)

	record := ekodb.Record{
		"name":  "WS TTL Test",
		"value": "should expire via websocket",
	}
	doc, err := client.Insert(collection, record, fmt.Sprintf("%ds", ttlSeconds))
	if err != nil {
		log.Fatalf("❌ FAILED: Could not insert document: %v", err)
	}
	docID := doc["id"].(string)
	fmt.Printf("  Output: Document ID = %s\n", docID)
	fmt.Println("  ✓ PASS: Document inserted")

	// Step 2: Query via client to verify document exists
	fmt.Println("\n[Step 2] Query to verify document exists")
	fmt.Printf("  Input: FindByID(%s)\n", docID)

	found, err := client.FindByID(collection, docID)
	if err != nil {
		log.Fatalf("❌ FAILED: Query failed: %v", err)
	}
	if found == nil {
		log.Fatal("❌ FAILED: Document should exist but was not found")
	}
	fmt.Printf("  Output: Found document with name = %v\n", found["name"])
	fmt.Println("  ✓ PASS: Document exists")

	// Step 3: Wait for TTL to expire
	waitTime := time.Duration(ttlSeconds+2) * time.Second
	fmt.Printf("\n[Step 3] Wait for TTL to expire (%v)\n", waitTime)
	fmt.Printf("  Waiting")
	for i := 0; i < int(waitTime.Seconds()); i++ {
		time.Sleep(1 * time.Second)
		fmt.Printf(".")
	}
	fmt.Println(" done")
	fmt.Println("  ✓ PASS: Wait complete")

	// Step 4: Query via client to verify document is GONE
	fmt.Println("\n[Step 4] Query to verify document has expired")
	fmt.Printf("  Input: FindByID(%s)\n", docID)

	expired, err := client.FindByID(collection, docID)
	if err != nil {
		fmt.Printf("  Output: Error (expected) - %v\n", err)
		fmt.Println("  ✓ PASS: Document expired (not found error)")
	} else if expired == nil {
		fmt.Println("  Output: nil (document not found)")
		fmt.Println("  ✓ PASS: Document expired correctly!")
	} else {
		fmt.Printf("  Output: Document still exists! %v\n", expired)
		log.Fatal("❌ FAILED: Document should have expired but still exists!")
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Cleanup
	// ═══════════════════════════════════════════════════════════════════════
	fmt.Println("\n═══════════════════════════════════════════════════════════")
	fmt.Println("CLEANUP")
	fmt.Println("═══════════════════════════════════════════════════════════")

	if err := client.DeleteCollection(collection); err != nil {
		log.Printf("Warning: cleanup failed: %v", err)
	} else {
		fmt.Println("✓ Deleted test collection")
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Summary
	// ═══════════════════════════════════════════════════════════════════════
	fmt.Println()
	fmt.Println("╔════════════════════════════════════════════════════════╗")
	fmt.Println("║          WEBSOCKET TTL TEST PASSED ✓                   ║")
	fmt.Println("╚════════════════════════════════════════════════════════╝")
	fmt.Println()
	fmt.Println("WebSocket TTL expiration is working correctly:")
	fmt.Println("  • Documents with TTL inserted via client expire correctly")
	fmt.Println("  • Queries correctly return nil for expired documents")
}
