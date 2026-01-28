package main

/*
ekoDB Go Client - Native SWR Function Example

Demonstrates the native SWR (Stale-While-Revalidate) function that
encapsulates the entire cache-aside pattern in a single operation.
*/

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func exampleBasicSWR(client *ekodb.Client) (string, error) {
	fmt.Println("\nExample 1: Basic Native SWR")
	fmt.Println(strings.Repeat("─", 80))
	fmt.Println("Single function replaces KvGet → If → HttpRequest → KvSet pipeline")

	// Create script with native SWR function
	headers := map[string]string{"User-Agent": "ekoDB-Client"}
	outputField := "user_data"

	basicSWRScript := ekodb.Script{
		Label:       "github_user_native",
		Name:        "GitHub User Lookup (Native SWR)",
		Description: strPtr("Fetches GitHub user data with automatic caching using native SWR"),
		Parameters: map[string]ekodb.ParameterDefinition{
			"username": {
				Description: "GitHub username to fetch",
				Required:    true,
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageSWR(
				"github:user:{{username}}", // cache_key
				"15m",                      // ttl
				"https://api.github.com/users/{{username}}", // url
				"GET",        // method
				headers,      // headers
				nil,          // body
				nil,          // timeout_seconds
				&outputField, // output_field
				nil,          // collection
			),
		},
		Tags: []string{"github", "swr", "native"},
	}

	scriptID, err := client.SaveScript(basicSWRScript)
	if err != nil {
		return "", fmt.Errorf("failed to save script: %w", err)
	}
	fmt.Printf("✓ Created native SWR script: github_user_native (%s)\n", scriptID)

	// First call - cache miss
	fmt.Println("\nFirst call (cache miss - will fetch from GitHub API):")
	start1 := time.Now()
	result1, err := client.CallScript("github_user_native", map[string]interface{}{
		"username": "torvalds",
	})
	if err != nil {
		return scriptID, fmt.Errorf("first call failed: %w", err)
	}
	duration1 := time.Since(start1).Milliseconds()
	fmt.Printf("  Response time: %dms\n", duration1)
	fmt.Printf("  Records returned: %d\n", len(result1.Records))

	// Second call - cache hit
	fmt.Println("\nSecond call (cache hit - instant from KV store):")
	start2 := time.Now()
	result2, err := client.CallScript("github_user_native", map[string]interface{}{
		"username": "torvalds",
	})
	if err != nil {
		return scriptID, fmt.Errorf("second call failed: %w", err)
	}
	duration2 := time.Since(start2).Milliseconds()
	speedup := float64(duration1) / float64(duration2)
	if duration2 == 0 {
		speedup = 0
	}
	fmt.Printf("  Response time: %dms\n", duration2)
	fmt.Printf("  Speedup: %.1fx faster 🚀\n", speedup)
	fmt.Printf("  Records returned: %d\n\n", len(result2.Records))

	return scriptID, nil
}

func exampleAuditTrail(client *ekodb.Client) (string, error) {
	fmt.Println("\nExample 2: SWR with Built-in Audit Trail")
	fmt.Println(strings.Repeat("─", 80))
	fmt.Println("Optional collection parameter for automatic request logging")

	outputField := "product"
	collection := "swr_audit_trail"

	auditSWRScript := ekodb.Script{
		Label:       "product_swr_audit",
		Name:        "Product API with Audit (Native SWR)",
		Description: strPtr("Caches product data and logs all requests automatically"),
		Parameters: map[string]ekodb.ParameterDefinition{
			"product_id": {
				Description: "Product ID to fetch",
				Required:    true,
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageSWR(
				"product:{{product_id}}", // cache_key
				"1h",                     // ttl
				"https://fakestoreapi.com/products/{{product_id}}", // url
				"GET",        // method
				nil,          // headers
				nil,          // body
				nil,          // timeout_seconds
				&outputField, // output_field
				&collection,  // collection
			),
		},
		Tags: []string{"products", "audit"},
	}

	auditScriptID, err := client.SaveScript(auditSWRScript)
	if err != nil {
		return "", fmt.Errorf("failed to save audit script: %w", err)
	}
	fmt.Printf("✓ Created SWR script with audit trail: product_swr_audit (%s)\n", auditScriptID)

	fmt.Println("\nFetching product (will create audit trail entry):")
	productResult, err := client.CallScript("product_swr_audit", map[string]interface{}{
		"product_id": "1",
	})
	if err != nil {
		return auditScriptID, fmt.Errorf("product call failed: %w", err)
	}
	fmt.Println("  ✓ Product fetched and cached")
	fmt.Println("  ✓ Audit record created in 'swr_audit_trail' collection")
	fmt.Printf("  Records: %d\n\n", len(productResult.Records))

	return auditScriptID, nil
}

func examplePipelineEnrichment(client *ekodb.Client) (string, error) {
	fmt.Println("\nExample 3: SWR in Multi-Function Pipeline")
	fmt.Println(strings.Repeat("─", 80))
	fmt.Println("Fetch external data → Process → Store in collection")

	outputField := "user_data"

	ttl24h := int64(86400) // 24 hours in seconds
	pipelineScript := ekodb.Script{
		Label:       "user_enrichment_pipeline",
		Name:        "User Data Enrichment Pipeline",
		Description: strPtr("Fetches external API data and stores enriched results"),
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_id": {
				Description: "User ID to enrich",
				Required:    true,
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			// Step 1: Fetch from external API with caching (30 min TTL)
			ekodb.StageSWR(
				"api:user:{{user_id}}", // cache_key
				"30m",                  // ttl
				"https://jsonplaceholder.typicode.com/users/{{user_id}}", // url
				"GET",        // method
				nil,          // headers
				nil,          // body
				nil,          // timeout_seconds
				&outputField, // output_field
				nil,          // collection
			),
			// Step 2: Store enriched data in collection (24 hour TTL)
			ekodb.StageInsert(
				"enriched_users", // collection
				map[string]interface{}{
					"user_id":     map[string]interface{}{"type": "String", "value": "{{user_id}}"},
					"source_data": map[string]interface{}{"type": "Object", "value": "{{user_data}}"},
				},
				false,   // bypass_ripple
				&ttl24h, // ttl (seconds)
			),
		},
		Tags: []string{"enrichment", "pipeline"},
	}

	pipelineScriptID, err := client.SaveScript(pipelineScript)
	if err != nil {
		return "", fmt.Errorf("failed to save pipeline script: %w", err)
	}
	fmt.Printf("✓ Created enrichment pipeline: user_enrichment_pipeline (%s)\n", pipelineScriptID)

	fmt.Println("\nRunning pipeline:")
	enrichResult, err := client.CallScript("user_enrichment_pipeline", map[string]interface{}{
		"user_id": "1",
	})
	if err != nil {
		return pipelineScriptID, fmt.Errorf("pipeline call failed: %w", err)
	}
	fmt.Println("  ✓ Data fetched from API (cached 30m)")
	fmt.Println("  ✓ Enriched data stored in 'enriched_users' (TTL 24h)")
	fmt.Printf("  Pipeline returned %d records\n\n", len(enrichResult.Records))

	return pipelineScriptID, nil
}

func exampleDynamicTTL(client *ekodb.Client) (string, error) {
	fmt.Println("\nExample 4: Dynamic TTL Configuration")
	fmt.Println(strings.Repeat("─", 80))
	fmt.Println("TTL as parameter - supports duration strings, integers, ISO timestamps")

	dynamicTTLScript := ekodb.Script{
		Label:       "flexible_cache",
		Name:        "Flexible Cache TTL (Native SWR)",
		Description: strPtr("Demonstrates parameterized TTL values"),
		Parameters: map[string]ekodb.ParameterDefinition{
			"resource_id": {
				Description: "Resource to fetch",
				Required:    true,
			},
			"ttl": {
				Description: "Cache duration (e.g., '5m', '1h', '30s')",
				Required:    true,
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageSWR(
				"resource:{{resource_id}}", // cache_key
				"{{ttl}}",                  // ttl (parameterized)
				"https://jsonplaceholder.typicode.com/posts/{{resource_id}}", // url
				"GET", // method
				nil,   // headers
				nil,   // body
				nil,   // timeout_seconds
				nil,   // output_field
				nil,   // collection
			),
		},
		Tags: []string{"dynamic"},
	}

	dynamicScriptID, err := client.SaveScript(dynamicTTLScript)
	if err != nil {
		return "", fmt.Errorf("failed to save dynamic TTL script: %w", err)
	}
	fmt.Printf("✓ Created dynamic TTL script: flexible_cache (%s)\n", dynamicScriptID)

	// Test with different TTLs
	ttlTests := []struct {
		value       string
		description string
	}{
		{"5m", "5 minutes"},
		{"1h", "1 hour"},
		{"30s", "30 seconds"},
	}

	for _, test := range ttlTests {
		_, err := client.CallScript("flexible_cache", map[string]interface{}{
			"resource_id": "test",
			"ttl":         test.value,
		})
		if err != nil {
			return dynamicScriptID, fmt.Errorf("TTL test failed: %w", err)
		}
		fmt.Printf("  ✓ Cached with TTL: %s (%s)\n", test.value, test.description)
	}

	return dynamicScriptID, nil
}

// strPtr is a helper to create string pointers
func strPtr(s string) *string {
	return &s
}

func cleanup(client *ekodb.Client, scriptIDs []string) {
	fmt.Println("\n🧹 Cleaning up...")
	for _, scriptID := range scriptIDs {
		if err := client.DeleteScript(scriptID); err != nil {
			log.Printf("⚠ Cleanup error (non-critical): %v", err)
		}
	}
	fmt.Printf("✓ Deleted %d test scripts\n", len(scriptIDs))
}

func main() {
	fmt.Println("🚀 ekoDB Go Client - Native SWR Function Examples\n")
	fmt.Println("📋 Demonstrates:")
	fmt.Println("   • Single-function SWR pattern (replaces 4-step pipeline)")
	fmt.Println("   • Automatic cache checking, HTTP fetching, and cache setting")
	fmt.Println("   • Built-in audit trail support")
	fmt.Println("   • Duration string TTLs ('15m', '1h', '30s')")
	fmt.Println("   • Multi-function pipeline integration")
	fmt.Println("   • Dynamic TTL configuration\n")

	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Printf("Warning: .env file not found")
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
		log.Fatalf("❌ Failed to create client: %v", err)
	}

	var scriptIDs []string

	// Run examples
	scriptID, err := exampleBasicSWR(client)
	if err != nil {
		log.Fatalf("❌ Example 1 failed: %v", err)
	}
	scriptIDs = append(scriptIDs, scriptID)

	scriptID, err = exampleAuditTrail(client)
	if err != nil {
		log.Fatalf("❌ Example 2 failed: %v", err)
	}
	scriptIDs = append(scriptIDs, scriptID)

	scriptID, err = examplePipelineEnrichment(client)
	if err != nil {
		log.Fatalf("❌ Example 3 failed: %v", err)
	}
	scriptIDs = append(scriptIDs, scriptID)

	scriptID, err = exampleDynamicTTL(client)
	if err != nil {
		log.Fatalf("❌ Example 4 failed: %v", err)
	}
	scriptIDs = append(scriptIDs, scriptID)

	// Summary
	fmt.Println("\n" + strings.Repeat("=", 80))
	fmt.Println("✅ Key Benefits of Native SWR:")
	fmt.Println("✅ Single function: Replaces 4-function cache-aside pattern")
	fmt.Println("✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds")
	fmt.Println("✅ Built-in audit: Optional collection parameter for automatic logging")
	fmt.Println("✅ Auto-enrichment: output_field populates params for downstream functions")
	fmt.Println("✅ Transactional: Works correctly in both transactional and non-transactional contexts")
	fmt.Println("✅ KV-optimized: Uses native KV store with proper TTL handling")

	fmt.Println("\n=== Performance Comparison ===")
	fmt.Println("Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)")
	fmt.Println("Native SWR:     SWR → Insert (2 functions)")
	fmt.Println("Result:         60% fewer functions, cleaner code, same behavior 🎯")

	cleanup(client, scriptIDs)

	fmt.Println("\n✅ All examples completed!")
}
