// SWR (Stale-While-Revalidate) Pattern Example
//
// Demonstrates using ekoDB Functions as an SWR cache:
// - Check cache first, return if exists
// - On cache miss, fetch from external API
// - Store result with TTL for auto-expiration

package main

import (
	"encoding/json"
	"fmt"
	"log"
	"os"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
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

	// Cleanup any stale collections from previous runs
	client.DeleteCollection("user_cache_go")

	// Create collection without schema to allow any data structure
	client.CreateCollection("user_cache_go", ekodb.Schema{})

	fmt.Println("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===")
	fmt.Println()

	fmt.Println("Step 1: Create SWR function that acts as edge cache")

	// Using jsonplaceholder.typicode.com - a reliable free API for testing
	swrScript := ekodb.Script{
		Label:       "fetch_api_user_go",
		Name:        "Fetch User with Cache",
		Description: func() *string { s := "SWR pattern: Check cache, fetch from API if stale"; return &s }(),
		Version:     func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_id": {
				Required:    true,
				Description: "User ID to fetch",
			},
			"ttl": {
				Required:    false,
				Default:     300,
				Description: "Cache TTL in seconds",
			},
		},
		Tags: []string{"swr", "user", "cache"},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindById("user_cache_go", "{{user_id}}"),
			{
				Stage: "If",
				Data: map[string]interface{}{
					"condition": map[string]interface{}{"type": "HasRecords"},
					"then_functions": []interface{}{
						ekodb.StageProject([]string{"data", "cached_at"}, false),
					},
					"else_functions": []interface{}{
						ekodb.StageHttpRequest(
							"https://jsonplaceholder.typicode.com/users/{{user_id}}",
							"GET",
							map[string]string{"Accept": "application/json"},
							nil,
						),
						ekodb.StageInsert("user_cache_go", map[string]interface{}{
							"id": "{{user_id}}",
							"data": map[string]interface{}{
								"type":  "Object",
								"value": "{{http_response}}",
							},
							"cached_at": time.Now().Format(time.RFC3339),
						}, false, nil),
					},
				},
			},
		},
	}

	scriptID, err := client.SaveScript(swrScript)
	if err != nil {
		log.Printf("Save script error: %v", err)
		return
	}
	fmt.Printf("✓ Created SWR script: fetch_api_user_go (%s)\n\n", scriptID)

	fmt.Println("Step 2: First call - Cache miss, fetches from API")
	result1, err := client.CallScript("fetch_api_user_go", map[string]interface{}{
		"user_id": "1",
		"ttl":     300,
	})
	if err != nil {
		log.Printf("Call script error: %v", err)
	} else {
		resultJSON, _ := json.MarshalIndent(result1, "", "  ")
		fmt.Printf("Result: %s\n", resultJSON)
		fmt.Println("✓ Data fetched from external API and cached\n")
	}

	fmt.Println("Step 3: Second call - Cache hit, instant response from ekoDB")
	start := time.Now()
	_, err = client.CallScript("fetch_api_user_go", map[string]interface{}{
		"user_id": "1",
	})
	duration := time.Since(start)
	if err != nil {
		log.Printf("Call script error: %v", err)
	} else {
		fmt.Printf("Response time: %dms (served from cache)\n", duration.Milliseconds())
		fmt.Println("✓ Lightning fast cache hit\n")
	}

	// Cleanup
	fmt.Println("🧹 Cleaning up...")
	client.DeleteScript(scriptID)
	client.DeleteCollection("user_cache_go")
	fmt.Println("✓ Cleanup complete\n")

	fmt.Println("=== SWR Pattern Summary ===")
	fmt.Println("✅ Cache miss → Fetch from API → Store in ekoDB")
	fmt.Println("✅ Cache hit → Instant response from ekoDB")
	fmt.Println("✅ TTL handles automatic cache invalidation")
}
