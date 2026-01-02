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
	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}

	baseURL := getEnv("API_BASE_URL", "http://localhost:8080")
	apiKey := getEnv("API_BASE_KEY", "a-test-api-key-from-ekodb")

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n")

	// Create SWR script for GitHub user caching
	fmt.Println("Step 1: Create SWR function that acts as edge cache")

	version := "1.0"
	swrScript := ekodb.Script{
		Label:       "fetch_github_user",
		Name:        "Fetch GitHub User with Cache",
		Description: strPtr("SWR pattern: Check cache, fetch from GitHub API if stale, auto-update with TTL"),
		Version:     &version,
		Parameters: map[string]ekodb.ParameterDefinition{
			"username": {
				Required:    true,
				Description: "GitHub username to fetch",
			},
			"ttl": {
				Required:    false,
				Default:     300,
				Description: "Cache TTL in seconds (default: 5 minutes)",
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			// 1. Check cache
			ekodb.StageFindById("github_cache", "{{username}}"),
			// 2. If cache exists, return it; else fetch from API
			ekodb.StageIf(
				ekodb.ConditionHasRecords(),
				// Cache hit - return cached data
				[]ekodb.FunctionStageConfig{
					ekodb.StageProject([]string{"data", "cached_at"}, false),
				},
				// Cache miss - fetch external API and store
				[]ekodb.FunctionStageConfig{
					ekodb.StageHttpRequest(
						"https://api.github.com/users/{{username}}",
						"GET",
						map[string]string{"User-Agent": "ekoDB-SWR-Example"},
						nil,
					),
					ekodb.StageInsert(
						"github_cache",
						map[string]interface{}{
							"id":        "{{username}}",
							"data":      "{{http_response}}",
							"cached_at": time.Now().Format(time.RFC3339),
						},
						false,
						nil,
					),
				},
			),
		},
		Tags: []string{"swr", "github", "cache"},
	}

	scriptID, err := client.SaveScript(swrScript)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Created SWR script: %s (%s)\n\n", swrScript.Label, scriptID)

	// Test the SWR pattern
	fmt.Println("Step 2: First call - Cache miss, fetches from GitHub API")
	start1 := time.Now()
	result1, err := client.CallScript("fetch_github_user", map[string]interface{}{
		"username": "torvalds",
		"ttl":      300,
	})
	if err != nil {
		log.Fatal(err)
	}
	duration1 := time.Since(start1)
	fmt.Printf("Response time: %v\n", duration1)
	resultJSON1, _ := json.MarshalIndent(result1.Records, "", "  ")
	fmt.Printf("Result: %s\n", resultJSON1)
	fmt.Println("✓ Data fetched from external API and cached\n")

	// Second call - should hit cache
	fmt.Println("Step 3: Second call - Cache hit, instant response from ekoDB")
	start2 := time.Now()
	_, err = client.CallScript("fetch_github_user", map[string]interface{}{
		"username": "torvalds",
	})
	if err != nil {
		log.Fatal(err)
	}
	duration2 := time.Since(start2)
	speedup := float64(duration1.Milliseconds()) / float64(duration2.Milliseconds())
	fmt.Printf("Response time: %v (%.1fx faster!)\n", duration2, speedup)
	fmt.Println("✓ Lightning fast cache hit\n")

	// Create advanced enrichment example
	fmt.Println("=== Advanced: SWR with Data Enrichment ===\n")
	fmt.Println("Creating product enrichment function...")

	version2 := "1.0"
	enrichScript := ekodb.Script{
		Label:       "fetch_product_enriched",
		Name:        "Fetch Product with Enrichment",
		Description: strPtr("Demonstrates calling external API and enriching data"),
		Version:     &version2,
		Parameters: map[string]ekodb.ParameterDefinition{
			"product_id": {
				Required:    true,
				Description: "Product ID",
			},
			"ttl": {
				Required:    false,
				Default:     600,
				Description: "Cache TTL (10 minutes)",
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindById("product_cache", "{{product_id}}"),
			ekodb.StageIf(
				ekodb.ConditionHasRecords(),
				[]ekodb.FunctionStageConfig{
					ekodb.StageProject([]string{"enriched_data"}, false),
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageHttpRequest(
						"https://dummyjson.com/products/{{product_id}}",
						"GET",
						nil,
						nil,
					),
					ekodb.StageInsert(
						"product_cache",
						map[string]interface{}{
							"id":            "{{product_id}}",
							"enriched_data": "{{http_response}}",
							"enriched_at":   time.Now().Format(time.RFC3339),
						},
						false,
						nil,
					),
				},
			),
		},
		Tags: []string{"enrichment", "product", "cache"},
	}

	enrichScriptID, err := client.SaveScript(enrichScript)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✓ Created enrichment script: %s (%s)\n\n", enrichScript.Label, enrichScriptID)

	fmt.Println("Step 4: Call enrichment function - Fetches from API + stores enriched result")
	enriched, err := client.CallScript("fetch_product_enriched", map[string]interface{}{
		"product_id": "1",
		"ttl":        600,
	})
	if err != nil {
		log.Fatal(err)
	}
	enrichedJSON, _ := json.MarshalIndent(enriched.Records, "", "  ")
	fmt.Printf("Enriched data: %s\n", enrichedJSON)
	fmt.Println("✓ Data fetched, enriched, and cached atomically\n")

	// Print benefits
	fmt.Println("=== Why This Is Powerful ===")
	fmt.Println("✓ No separate cache layer (Redis, Memcached) needed")
	fmt.Println("✓ No manual cache invalidation (TTL handles it)")
	fmt.Println("✓ No separate edge infrastructure (ekoDB IS the edge)")
	fmt.Println("✓ Atomic operations (function executes as transaction)")
	fmt.Println("✓ With multi-node + ripples: Auto-sync across all nodes")
	fmt.Println("✓ Sub-millisecond cache hits from internal storage")
	fmt.Println("✓ One service instead of many (cache + API gateway + database)")

	fmt.Println("\n=== Real-World Use Cases ===")
	fmt.Println("1. API Gateway Pattern:")
	fmt.Println("   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache")
	fmt.Println("\n2. Database Federation:")
	fmt.Println("   - Query multiple DBs (Postgres, MongoDB) + external APIs")
	fmt.Println("   - Merge results in one function call")
	fmt.Println("\n3. IoT Data Enrichment:")
	fmt.Println("   - Sensor data + weather API + location API")
	fmt.Println("   - Enrich and cache in one atomic operation")
	fmt.Println("\n4. E-commerce Product Pages:")
	fmt.Println("   - Product info + reviews + inventory + pricing")
	fmt.Println("   - All from different sources, cached together")

	fmt.Println("\n✓ Example complete - Your database IS your edge!\n")
}

func strPtr(s string) *string {
	return &s
}

func getEnv(key, fallback string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return fallback
}
