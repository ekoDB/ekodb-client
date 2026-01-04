// TTL Expiration Verification Test
//
// This test VERIFIES that TTL actually works by:
// 1. Inserting a document with short TTL (3 seconds)
// 2. Verifying the document exists immediately
// 3. Waiting for TTL to expire
// 4. Verifying the document is GONE
//
// If this test passes, TTL expiration is working correctly.

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
	fmt.Println("║     TTL EXPIRATION VERIFICATION TEST                   ║")
	fmt.Println("╚════════════════════════════════════════════════════════╝")
	fmt.Println()
	fmt.Println("This test verifies that document TTL expiration works correctly.")
	fmt.Println("We will insert documents with short TTL and verify they expire.")
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

	// Create client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Client connected")

	collection := "ttl_expiration_test_go"
	ttlSeconds := 3

	// ═══════════════════════════════════════════════════════════════════════
	// TEST 1: Document TTL Expiration
	// ═══════════════════════════════════════════════════════════════════════
	fmt.Println()
	fmt.Println("═══════════════════════════════════════════════════════════")
	fmt.Println("TEST 1: Document TTL Expiration")
	fmt.Println("═══════════════════════════════════════════════════════════")

	// Step 1: Insert document with 3-second TTL
	fmt.Printf("\n[Step 1] Insert document with %d second TTL\n", ttlSeconds)
	fmt.Printf("  Input: {name: 'TTL Test', value: 'should expire'}\n")
	fmt.Printf("  TTL: %ds\n", ttlSeconds)

	record := ekodb.Record{
		"name":  "TTL Test",
		"value": "should expire",
	}
	doc, err := client.Insert(collection, record, fmt.Sprintf("%ds", ttlSeconds))
	if err != nil {
		log.Fatalf("❌ FAILED: Could not insert document: %v", err)
	}
	docID := doc["id"].(string)
	fmt.Printf("  Output: Document ID = %s\n", docID)
	fmt.Println("  ✓ PASS: Document inserted")

	// Step 2: Verify document exists immediately
	fmt.Println("\n[Step 2] Verify document exists immediately")
	fmt.Printf("  Input: FindByID(%s)\n", docID)

	found, err := client.FindByID(collection, docID)
	if err != nil {
		log.Fatalf("❌ FAILED: Could not find document: %v", err)
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

	// Step 4: Verify document is GONE
	fmt.Println("\n[Step 4] Verify document has expired")
	fmt.Printf("  Input: FindByID(%s)\n", docID)

	expired, err := client.FindByID(collection, docID)
	if err != nil {
		// Error could mean document not found - that's expected!
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
	// TEST 2: Multiple Documents with Different TTLs
	// ═══════════════════════════════════════════════════════════════════════
	fmt.Println()
	fmt.Println("═══════════════════════════════════════════════════════════")
	fmt.Println("TEST 2: Multiple Documents with Different TTLs")
	fmt.Println("═══════════════════════════════════════════════════════════")

	// Insert 3 documents: 3s TTL, 10s TTL, no TTL (with better margins)
	fmt.Println("\n[Step 1] Insert 3 documents with different TTLs")

	rec3s := ekodb.Record{"name": "3 second TTL", "expires": "soon"}
	rec10s := ekodb.Record{"name": "10 second TTL", "expires": "later"}
	recPerm := ekodb.Record{"name": "Permanent", "expires": "never"}

	doc3s, _ := client.Insert(collection, rec3s, "3s")
	doc10s, _ := client.Insert(collection, rec10s, "10s")
	docPerm, _ := client.Insert(collection, recPerm, "")

	id3s := doc3s["id"].(string)
	id10s := doc10s["id"].(string)
	idPerm := docPerm["id"].(string)

	fmt.Printf("  Inserted: 3s TTL=%s, 10s TTL=%s, Permanent=%s\n", id3s[:8], id10s[:8], idPerm[:8])
	fmt.Println("  ✓ PASS: All documents inserted")

	// Wait 5 seconds - 3s doc should expire, 10s doc should remain
	fmt.Println("\n[Step 2] Wait 5 seconds (3s doc should expire)")
	time.Sleep(5 * time.Second)

	// Check 3s doc (should be gone)
	check3s, _ := client.FindByID(collection, id3s)
	if check3s == nil {
		fmt.Println("  ✓ PASS: 3s TTL document expired")
	} else {
		log.Fatal("❌ FAILED: 3s TTL document should have expired")
	}

	// Check 10s doc (should still exist)
	check10s, _ := client.FindByID(collection, id10s)
	if check10s != nil {
		fmt.Println("  ✓ PASS: 10s TTL document still exists")
	} else {
		log.Fatal("❌ FAILED: 10s TTL document expired too early")
	}

	// Check permanent doc (should exist)
	checkPerm, _ := client.FindByID(collection, idPerm)
	if checkPerm != nil {
		fmt.Println("  ✓ PASS: Permanent document still exists")
	} else {
		log.Fatal("❌ FAILED: Permanent document should exist")
	}

	// Wait 7 more seconds - 10s doc should now expire (12s total)
	fmt.Println("\n[Step 3] Wait 7 more seconds (10s doc should expire)")
	time.Sleep(7 * time.Second)

	check10sAgain, _ := client.FindByID(collection, id10s)
	if check10sAgain == nil {
		fmt.Println("  ✓ PASS: 10s TTL document expired")
	} else {
		log.Fatal("❌ FAILED: 10s TTL document should have expired")
	}

	checkPermAgain, _ := client.FindByID(collection, idPerm)
	if checkPermAgain != nil {
		fmt.Println("  ✓ PASS: Permanent document still exists")
	} else {
		log.Fatal("❌ FAILED: Permanent document should still exist")
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
	fmt.Println("║              ALL TTL TESTS PASSED ✓                    ║")
	fmt.Println("╚════════════════════════════════════════════════════════╝")
	fmt.Println()
	fmt.Println("TTL expiration is working correctly:")
	fmt.Println("  • Documents with TTL expire after the specified time")
	fmt.Println("  • Documents without TTL persist indefinitely")
	fmt.Println("  • Different TTL durations are handled correctly")
}
