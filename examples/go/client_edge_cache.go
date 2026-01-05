// ekoDB as Edge Cache Example
//
// Demonstrates using ekoDB Functions as an edge cache:
// - Create a cache function that checks cache first
// - On cache miss, fetch from external API and store
// - On cache hit, return cached data instantly

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

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("=== ekoDB as Edge Cache - Simple Example ===")
	fmt.Println()

	// Setup: Create cache collection with test data
	fmt.Println("Setting up edge cache collection...")
	_ = client.DeleteCollection("edge_cache_go")

	// Insert a cached entry
	cacheRecord := map[string]interface{}{
		"id":        "weather_nyc",
		"data":      "{\"temp\": 72}",
		"cached_at": time.Now().Format(time.RFC3339),
	}
	_, err = client.Insert("edge_cache_go", cacheRecord)
	if err != nil {
		log.Fatalf("Failed to insert cache record: %v", err)
	}
	fmt.Println("✓ Cache entry created")
	fmt.Println()

	// Create a simple cache lookup script
	fmt.Println("Creating edge cache lookup script...")
	version := "1.0"
	cacheScript := ekodb.Script{
		Label:       "edge_cache_lookup_go",
		Name:        "Edge Cache Lookup",
		Description: strPtr("Simple cache lookup by key"),
		Version:     &version,
		Parameters:  map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("edge_cache_go"),
		},
		Tags: []string{"cache", "edge"},
	}

	scriptID, err := client.SaveScript(cacheScript)
	if err != nil {
		log.Fatalf("Failed to save script: %v", err)
	}
	fmt.Printf("✓ Edge cache script created: %s\n\n", scriptID)

	// Test it - First call
	fmt.Println("Call 1: Cache lookup")
	start1 := time.Now()
	result1, err := client.CallScript("edge_cache_lookup_go", nil)
	if err != nil {
		log.Fatalf("Script call failed: %v", err)
	}
	duration1 := time.Since(start1)
	fmt.Printf("Response time: %dms\n", duration1.Milliseconds())
	fmt.Printf("Found %d cached entries\n", len(result1.Records))

	// Test it again - Second call (should be fast due to connection reuse)
	fmt.Println("\nCall 2: Cache lookup (connection warm)")
	start2 := time.Now()
	result2, err := client.CallScript("edge_cache_lookup_go", nil)
	if err != nil {
		log.Fatalf("Script call failed: %v", err)
	}
	duration2 := time.Since(start2)
	fmt.Printf("Response time: %dms\n", duration2.Milliseconds())
	fmt.Printf("Found %d cached entries\n", len(result2.Records))

	// Cleanup
	fmt.Println("\n🧹 Cleaning up...")
	_ = client.DeleteScript(scriptID)
	_ = client.DeleteCollection("edge_cache_go")
	fmt.Println("✓ Cleanup complete")

	fmt.Println("\n=== The Magic ===")
	fmt.Println("- Your DATABASE is your EDGE")
	fmt.Println("- No Redis needed")
	fmt.Println("- No CDN needed")
	fmt.Println("- No cache invalidation logic needed (TTL handles it)")
	fmt.Println("- With ripples: All nodes auto-sync cache")
	fmt.Println("- One service: Database + Cache + Edge Functions")

	fmt.Println("\n✓ Example complete!")
}

func strPtr(s string) *string {
	return &s
}
