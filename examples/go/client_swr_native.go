package main

/*
ekoDB Go Client - Native SWR Function Example

Demonstrates the native SWR (Stale-While-Revalidate) function that
encapsulates the entire cache-aside pattern in a single operation.
*/

import (
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

const (
	basicLabel         = "github_user_native_go"
	auditLabel         = "product_swr_audit_go"
	pipelineLabel      = "user_enrichment_pipeline_go"
	dynamicLabel       = "flexible_cache_go"
	auditCollection    = "swr_audit_trail_go"
	enrichedCollection = "enriched_users_go"
)

// saveOrUpdateFn saves a function, or — if the label already exists (HTTP 409)
// — updates it in place and recovers the encrypted ID via a GET by label.
func saveOrUpdateFn(client *ekodb.Client, fn ekodb.UserFunction) (string, error) {
	id, err := client.SaveFunction(fn)
	if err == nil {
		return id, nil
	}
	var httpErr *ekodb.HTTPError
	if errors.As(err, &httpErr) && httpErr.StatusCode == 409 {
		if uerr := client.UpdateFunction(fn.Label, fn); uerr != nil {
			return "", uerr
		}
		fmt.Printf("Function '%s' already existed — updated instead\n", fn.Label)
		existing, gerr := client.GetFunction(fn.Label)
		if gerr != nil {
			return "", gerr
		}
		if existing.ID == nil {
			return "", fmt.Errorf("function %q has no id after update", fn.Label)
		}
		return *existing.ID, nil
	}
	return "", err
}

func exampleBasicSWR(client *ekodb.Client) (string, error) {
	fmt.Println("\nExample 1: Basic Native SWR")
	fmt.Println(strings.Repeat("─", 80))
	fmt.Println("Single function replaces KvGet → If → HttpRequest → KvSet pipeline")

	// Create script with native SWR function
	headers := map[string]string{"User-Agent": "ekoDB-Client"}
	outputField := "user_data"

	basicSWRScript := ekodb.UserFunction{
		Label:       basicLabel,
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
				"github:user:go:{{username}}", // cache_key
				"15m",                         // ttl
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

	scriptID, err := saveOrUpdateFn(client, basicSWRScript)
	if err != nil {
		return "", fmt.Errorf("failed to save script: %w", err)
	}
	fmt.Printf("✓ Created native SWR script: %s (%s)\n", basicLabel, scriptID)

	// First call - cache miss
	fmt.Println("\nFirst call (cache miss - will fetch from GitHub API):")
	start1 := time.Now()
	result1, err := client.CallFunction(basicLabel, map[string]interface{}{
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
	result2, err := client.CallFunction(basicLabel, map[string]interface{}{
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
	collection := auditCollection

	auditSWRScript := ekodb.UserFunction{
		Label:       auditLabel,
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
				"product:go:{{product_id}}", // cache_key
				"1h",                        // ttl
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

	auditScriptID, err := saveOrUpdateFn(client, auditSWRScript)
	if err != nil {
		return "", fmt.Errorf("failed to save audit script: %w", err)
	}
	fmt.Printf("✓ Created SWR script with audit trail: %s (%s)\n", auditLabel, auditScriptID)

	fmt.Println("\nFetching product (will create audit trail entry):")
	productResult, err := client.CallFunction(auditLabel, map[string]interface{}{
		"product_id": "1",
	})
	if err != nil {
		return auditScriptID, fmt.Errorf("product call failed: %w", err)
	}
	fmt.Println("  ✓ Product fetched and cached")
	fmt.Printf("  ✓ Audit record created in '%s' collection\n", auditCollection)
	fmt.Printf("  Records: %d\n\n", len(productResult.Records))

	return auditScriptID, nil
}

func examplePipelineEnrichment(client *ekodb.Client) (string, error) {
	fmt.Println("\nExample 3: SWR in Multi-Function Pipeline")
	fmt.Println(strings.Repeat("─", 80))
	fmt.Println("Fetch external data → Process → Store in collection")

	outputField := "user_data"

	ttl24h := int64(86400) // 24 hours in seconds
	pipelineScript := ekodb.UserFunction{
		Label:       pipelineLabel,
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
				"api:user:go:{{user_id}}", // cache_key
				"30m",                     // ttl
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
				enrichedCollection, // collection
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

	pipelineScriptID, err := saveOrUpdateFn(client, pipelineScript)
	if err != nil {
		return "", fmt.Errorf("failed to save pipeline script: %w", err)
	}
	fmt.Printf("✓ Created enrichment pipeline: %s (%s)\n", pipelineLabel, pipelineScriptID)

	fmt.Println("\nRunning pipeline:")
	enrichResult, err := client.CallFunction(pipelineLabel, map[string]interface{}{
		"user_id": "1",
	})
	if err != nil {
		return pipelineScriptID, fmt.Errorf("pipeline call failed: %w", err)
	}
	fmt.Println("  ✓ Data fetched from API (cached 30m)")
	fmt.Printf("  ✓ Enriched data stored in '%s' (TTL 24h)\n", enrichedCollection)
	fmt.Printf("  Pipeline returned %d records\n\n", len(enrichResult.Records))

	return pipelineScriptID, nil
}

func exampleDynamicTTL(client *ekodb.Client) (string, error) {
	fmt.Println("\nExample 4: Dynamic TTL Configuration")
	fmt.Println(strings.Repeat("─", 80))
	fmt.Println("TTL as parameter - supports duration strings, integers, ISO timestamps")

	dynamicTTLScript := ekodb.UserFunction{
		Label:       dynamicLabel,
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
				"resource:go:{{resource_id}}", // cache_key
				"{{ttl}}",                     // ttl (parameterized)
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

	dynamicScriptID, err := saveOrUpdateFn(client, dynamicTTLScript)
	if err != nil {
		return "", fmt.Errorf("failed to save dynamic TTL script: %w", err)
	}
	fmt.Printf("✓ Created dynamic TTL script: %s (%s)\n", dynamicLabel, dynamicScriptID)

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
		_, err := client.CallFunction(dynamicLabel, map[string]interface{}{
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

func isNotFound(err error) bool {
	var httpErr *ekodb.HTTPError
	return (errors.As(err, &httpErr) && httpErr.StatusCode == 404) ||
		strings.Contains(strings.ToLower(err.Error()), "not found")
}

func startClean(client *ekodb.Client) error {
	var cleanupErrors []string
	for _, label := range []string{basicLabel, auditLabel, pipelineLabel, dynamicLabel} {
		function, err := client.GetFunction(label)
		if err != nil {
			if !isNotFound(err) {
				cleanupErrors = append(cleanupErrors, fmt.Sprintf("function %s lookup: %v", label, err))
			}
			continue
		}
		if function.ID == nil {
			cleanupErrors = append(cleanupErrors, fmt.Sprintf("function %s lookup returned no ID", label))
			continue
		}
		if err := client.DeleteFunction(*function.ID); err != nil && !isNotFound(err) {
			cleanupErrors = append(cleanupErrors, fmt.Sprintf("function %s: %v", label, err))
		}
	}
	for _, key := range []string{
		"github:user:go:torvalds",
		"product:go:1",
		"api:user:go:1",
		"resource:go:test",
	} {
		if err := client.KVDelete(key); err != nil && !isNotFound(err) {
			cleanupErrors = append(cleanupErrors, fmt.Sprintf("KV key %s: %v", key, err))
		}
	}
	for _, collection := range []string{auditCollection, enrichedCollection} {
		if err := client.DeleteCollection(collection); err != nil && !isNotFound(err) {
			cleanupErrors = append(cleanupErrors, fmt.Sprintf("collection %s: %v", collection, err))
		}
	}
	if len(cleanupErrors) > 0 {
		return fmt.Errorf("initial cleanup failed: %s", strings.Join(cleanupErrors, "; "))
	}
	return nil
}

func cleanup(client *ekodb.Client, scriptIDs []string) error {
	fmt.Println("\n🧹 Cleaning up...")
	var cleanupErrors []string
	deletedScripts := 0
	for _, scriptID := range scriptIDs {
		if err := client.DeleteFunction(scriptID); err != nil {
			cleanupErrors = append(cleanupErrors, fmt.Sprintf("function %s: %v", scriptID, err))
		} else {
			deletedScripts++
		}
	}
	for _, key := range []string{
		"github:user:go:torvalds",
		"product:go:1",
		"api:user:go:1",
		"resource:go:test",
	} {
		if err := client.KVDelete(key); err != nil && !isNotFound(err) {
			cleanupErrors = append(cleanupErrors, fmt.Sprintf("KV key %s: %v", key, err))
		}
	}
	for _, collection := range []string{auditCollection, enrichedCollection} {
		if err := client.DeleteCollection(collection); err != nil && !isNotFound(err) {
			cleanupErrors = append(cleanupErrors, fmt.Sprintf("collection %s: %v", collection, err))
		}
	}
	if len(cleanupErrors) > 0 {
		return fmt.Errorf("cleanup failed: %s", strings.Join(cleanupErrors, "; "))
	}
	fmt.Printf("✓ Deleted %d test scripts\n", deletedScripts)
	return nil
}

func run() (runErr error) {
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
		return fmt.Errorf("failed to create client: %w", err)
	}

	if err := startClean(client); err != nil {
		return err
	}

	var scriptIDs []string
	defer func() {
		if cleanupErr := cleanup(client, scriptIDs); cleanupErr != nil {
			runErr = errors.Join(runErr, cleanupErr)
		}
	}()

	// Run examples
	scriptID, err := exampleBasicSWR(client)
	if scriptID != "" {
		scriptIDs = append(scriptIDs, scriptID)
	}
	if err != nil {
		return fmt.Errorf("example 1: %w", err)
	}

	scriptID, err = exampleAuditTrail(client)
	if scriptID != "" {
		scriptIDs = append(scriptIDs, scriptID)
	}
	if err != nil {
		return fmt.Errorf("example 2: %w", err)
	}

	scriptID, err = examplePipelineEnrichment(client)
	if scriptID != "" {
		scriptIDs = append(scriptIDs, scriptID)
	}
	if err != nil {
		return fmt.Errorf("example 3: %w", err)
	}

	scriptID, err = exampleDynamicTTL(client)
	if scriptID != "" {
		scriptIDs = append(scriptIDs, scriptID)
	}
	if err != nil {
		return fmt.Errorf("example 4: %w", err)
	}

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

	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("\n✅ All examples completed!")
}
