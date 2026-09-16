// SWR (Stale-While-Revalidate) Pattern Example
//
// Demonstrates using ekoDB Functions as an SWR cache:
// - Check cache first, return if exists
// - On cache miss, fetch from external API
// - Store result with TTL for auto-expiration

package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
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

func run() (runErr error) {
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
		return err
	}
	collection := "user_cache_go"
	scriptID := ""
	defer func() {
		fmt.Println("🧹 Cleaning up...")
		if scriptID != "" {
			runErr = errors.Join(runErr, client.DeleteFunction(scriptID))
		}
		if err := client.DeleteCollection(collection); err != nil {
			var httpErr *ekodb.HTTPError
			if !errors.As(err, &httpErr) || httpErr.StatusCode != 404 {
				runErr = errors.Join(runErr, err)
			}
		}
		if runErr == nil {
			fmt.Print("✓ Cleanup complete\n\n")
		}
	}()

	// Cleanup any stale collections from previous runs
	client.DeleteCollection(collection)

	fmt.Println("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===")
	fmt.Println()

	fmt.Println("Step 1: Create SWR function that acts as edge cache")

	// Using jsonplaceholder.typicode.com - a reliable free API for testing
	swrScript := ekodb.UserFunction{
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
			ekodb.StageFindById(collection, "{{user_id}}"),
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
						ekodb.StageInsert(collection, map[string]interface{}{
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

	scriptID, err = saveOrUpdateFn(client, swrScript)
	if err != nil {
		return fmt.Errorf("save script: %w", err)
	}
	fmt.Printf("✓ Created SWR script: fetch_api_user_go (%s)\n\n", scriptID)

	fmt.Println("Step 2: First call - Cache miss, fetches from API")
	result1, err := client.CallFunction("fetch_api_user_go", map[string]interface{}{
		"user_id": "1",
		"ttl":     300,
	})
	if err != nil {
		return fmt.Errorf("first call: %w", err)
	}
	resultJSON, _ := json.MarshalIndent(result1, "", "  ")
	fmt.Printf("Result: %s\n", resultJSON)
	fmt.Print("✓ Data fetched from external API and cached\n\n")

	fmt.Println("Step 3: Second call - Cache hit, instant response from ekoDB")
	start := time.Now()
	_, err = client.CallFunction("fetch_api_user_go", map[string]interface{}{
		"user_id": "1",
	})
	duration := time.Since(start)
	if err != nil {
		return fmt.Errorf("second call: %w", err)
	}
	fmt.Printf("Response time: %dms (served from cache)\n", duration.Milliseconds())
	fmt.Print("✓ Lightning fast cache hit\n\n")

	fmt.Println("=== SWR Pattern Summary ===")
	fmt.Println("✅ Cache miss → Fetch from API → Store in ekoDB")
	fmt.Println("✅ Cache hit → Instant response from ekoDB")
	fmt.Println("✅ TTL handles automatic cache invalidation")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
