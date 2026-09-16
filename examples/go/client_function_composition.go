/**
 * Function Composition Example for ekoDB Go Client
 *
 * Demonstrates calling Functions within Functions using CallFunction
 * Shows how to build reusable logic blocks and compose complex workflows
 */

package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

const compositionCollection = "function_composition_users_go"

var compositionLabels = []string{
	"fetch_user_go", "get_user_wrapper_go", "fetch_and_store_user_go", "swr_user_go",
	"validate_user_go", "fetch_slim_user_go", "get_verified_user_go",
}

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

func setupTestData(client *ekodb.Client) error {
	fmt.Print("📋 Setting up test data...\n\n")

	for i := 1; i <= 5; i++ {
		dept := "sales"
		if i <= 2 {
			dept = "engineering"
		}

		record := map[string]interface{}{
			"user_code":  fmt.Sprintf("user_%d", i),
			"name":       fmt.Sprintf("User %d", i),
			"department": dept,
			"score":      i * 20,
		}

		if _, err := client.Insert(compositionCollection, record); err != nil {
			return err
		}
	}

	fmt.Print("✅ Test data ready\n\n")
	return nil
}

func basicCompositionExample(client *ekodb.Client) error {
	fmt.Print("📝 Example 1: Basic Function Composition\n\n")
	fmt.Print("Building reusable functions that call each other...\n\n")

	// Step 1: Create reusable "fetch_user" function
	fetchUser := ekodb.UserFunction{
		Label: compositionLabels[0],
		Name:  "Fetch user by code",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindOne(compositionCollection, "user_code", "{{user_code}}"),
		},
		Tags: []string{},
	}

	if _, err := saveOrUpdateFn(client, fetchUser); err != nil {
		return err
	}
	fmt.Println("✅ Saved reusable function: fetch_user")

	// Step 2: Create wrapper that CALLS fetch_user
	getUserWrapper := ekodb.UserFunction{
		Label: compositionLabels[1],
		Name:  "Wrapper that calls fetch_user",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			// params nil - inherits user_code from parent scope
			ekodb.StageCallFunction(compositionLabels[0], nil),
			ekodb.StageProject([]string{"name", "department"}, false),
		},
	}

	if _, err := saveOrUpdateFn(client, getUserWrapper); err != nil {
		return err
	}
	fmt.Print("✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)\n\n")

	// Step 3: Call the composed function
	params := map[string]interface{}{
		"user_code": "user_1",
	}
	result, err := client.CallFunction(compositionLabels[1], params)
	if err != nil {
		return err
	}

	fmt.Println("📊 Result from composed function:")
	fmt.Printf("   Records: %d\n", len(result.Records))
	if len(result.Records) > 0 {
		record := result.Records[0]
		nameJSON, _ := json.Marshal(record["name"])
		deptJSON, _ := json.Marshal(record["department"])
		fmt.Printf("   Name: %s\n", nameJSON)
		fmt.Printf("   Department: %s\n\n", deptJSON)
	}

	fmt.Println("🎯 Key Benefit: fetch_user can be reused by ANY function!")
	fmt.Print("   No code duplication, single source of truth\n\n")

	return nil
}

func swrCompositionExample(client *ekodb.Client) error {
	fmt.Print("📝 Example 2: SWR Pattern with Function Composition\n\n")
	fmt.Print("Using KV cache + CallFunction for fast cache-aside pattern...\n\n")

	// Step 1: Create reusable fetch and store function
	// Using jsonplaceholder.typicode.com - a reliable free API for testing
	// This function fetches from API and stores in KV cache
	ttl := int64(300)
	fetchAndStore := ekodb.UserFunction{
		Label: compositionLabels[2],
		Name:  "Fetch user from API and cache in KV",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_id": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageHttpRequest(
				"https://jsonplaceholder.typicode.com/users/{{user_id}}",
				"GET",
				map[string]string{"Accept": "application/json"},
				nil,
			),
			// Store in KV cache (much faster than collection for cache lookups)
			ekodb.StageKvSet("function_composition_user_cache_go:{{user_id}}", "{{http_response}}", &ttl),
		},
	}

	if _, err := saveOrUpdateFn(client, fetchAndStore); err != nil {
		return err
	}
	fmt.Println("✅ Saved reusable function: fetch_and_store_user (uses KV)")

	// Step 2: Create SWR function that CALLS the reusable function
	// Pattern: KV cache check → populate if missing → return
	swrUser := ekodb.UserFunction{
		Label: compositionLabels[3],
		Name:  "SWR pattern for user data (KV-based)",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_id": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			// Check KV cache first (O(1) lookup - much faster than FindById)
			ekodb.StageKvGet("function_composition_user_cache_go:{{user_id}}"),
			ekodb.StageIf(
				// KvGet returns { value: ... } on hit, { value: null } on miss
				// So we check if "value" is not null to detect cache hit
				ekodb.ConditionNot(ekodb.ConditionFieldEquals("value", nil)),
				[]ekodb.FunctionStageConfig{
					// Cache hit - project the value field
					ekodb.StageProject([]string{"value"}, false),
				},
				[]ekodb.FunctionStageConfig{
					// Cache miss - call reusable function to fetch and store
					ekodb.StageCallFunction(compositionLabels[2], map[string]any{
						"user_id": "{{user_id}}",
					}),
					// After storing, retrieve the cached value to return it
					ekodb.StageKvGet("function_composition_user_cache_go:{{user_id}}"),
					ekodb.StageProject([]string{"value"}, false),
				},
			),
		},
	}

	if _, err := saveOrUpdateFn(client, swrUser); err != nil {
		return err
	}
	fmt.Print("✅ Saved SWR function using composition: swr_user\n\n")

	// Step 3: Test cache miss
	fmt.Println("First call (cache miss - will fetch from API):")
	params := map[string]interface{}{
		"user_id": "1",
	}

	start1 := time.Now()
	result1, err := client.CallFunction(compositionLabels[3], params)
	if err != nil {
		return err
	}
	duration1 := time.Since(start1)

	fmt.Printf("   ⏱️  Duration: %v\n", duration1)
	fmt.Printf("   📊 Records: %d\n", len(result1.Records))
	if len(result1.Records) > 0 {
		dataJSON, _ := json.MarshalIndent(result1.Records[0], "      ", "  ")
		preview := string(dataJSON)
		if len(preview) > 200 {
			preview = preview[:200] + "..."
		}
		fmt.Printf("   📦 Data: %s\n\n", preview)
	} else {
		fmt.Println()
	}

	// Step 4: Test cache hit
	fmt.Println("Second call (cache hit - from cache):")
	start2 := time.Now()
	result2, err := client.CallFunction(compositionLabels[3], params)
	if err != nil {
		return err
	}
	duration2 := time.Since(start2)

	fmt.Printf("   ⏱️  Duration: %v\n", duration2)
	fmt.Printf("   📊 Records: %d\n", len(result2.Records))
	if len(result2.Records) > 0 {
		dataJSON, _ := json.MarshalIndent(result2.Records[0], "      ", "  ")
		preview := string(dataJSON)
		if len(preview) > 200 {
			preview = preview[:200] + "..."
		}
		fmt.Printf("   📦 Data: %s\n", preview)
	}
	if duration2.Milliseconds() > 0 {
		speedup := float64(duration1.Milliseconds()) / float64(duration2.Milliseconds())
		fmt.Printf("   🚀 Cache speedup: %.1fx faster!\n\n", speedup)
	}

	return nil
}

func nestedCompositionExample(client *ekodb.Client) error {
	fmt.Print("📝 Example 3: Multi-Level Function Composition\n\n")
	fmt.Print("Building complex workflows from small, reusable pieces...\n\n")

	// Level 1: Base function
	validateUser := ekodb.UserFunction{
		Label: compositionLabels[4],
		Name:  "Check if user exists",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindOne(compositionCollection, "user_code", "{{user_code}}"),
		},
		Tags: []string{},
	}

	if _, err := saveOrUpdateFn(client, validateUser); err != nil {
		return err
	}
	fmt.Println("✅ Level 1 function: validate_user")

	// Level 2: Calls validate_user + projects
	fetchSlim := ekodb.UserFunction{
		Label: compositionLabels[5],
		Name:  "Validate and slim down user",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			// params nil - inherits user_code from parent scope
			ekodb.StageCallFunction(compositionLabels[4], nil),
			ekodb.StageProject([]string{"name", "department"}, false),
		},
	}

	if _, err := saveOrUpdateFn(client, fetchSlim); err != nil {
		return err
	}
	fmt.Println("✅ Level 2 function: fetch_slim_user (calls validate_user)")

	// Level 3: Calls fetch_slim (demonstrates 3-level nesting)
	getVerifiedUser := ekodb.UserFunction{
		Label: compositionLabels[6],
		Name:  "Get verified and validated user",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			// params nil - inherits user_code from parent scope
			ekodb.StageCallFunction(compositionLabels[5], nil),
		},
	}

	if _, err := saveOrUpdateFn(client, getVerifiedUser); err != nil {
		return err
	}
	fmt.Print("✅ Level 3 function: get_verified_user (calls fetch_slim_user)\n\n")

	// Execute 3-level nested composition
	params := map[string]interface{}{
		"user_code": "user_1",
	}
	result, err := client.CallFunction(compositionLabels[6], params)
	if err != nil {
		return err
	}

	fmt.Println("📊 Result from 3-level nested composition:")
	fmt.Printf("   Records: %d\n", len(result.Records))
	if len(result.Records) > 0 {
		record := result.Records[0]
		nameJSON, _ := json.Marshal(record["name"])
		deptJSON, _ := json.Marshal(record["department"])
		fmt.Printf("   Name: %s\n", nameJSON)
		fmt.Printf("   Department: %s\n\n", deptJSON)
	}

	fmt.Println("🎯 Key Benefit: Each function is independently testable and reusable!")
	fmt.Println("   - validate_user: Used in 100 different workflows")
	fmt.Println("   - fetch_slim_user: Used in 50 workflows")
	fmt.Print("   - get_verified_user: Specific workflow\n\n")

	return nil
}

func run() (runErr error) {
	// Load environment variables
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

	// Initialize client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}
	defer func() {
		for _, label := range compositionLabels {
			function, err := client.GetFunction(label)
			if err != nil {
				if !compositionNotFound(err) {
					runErr = errors.Join(runErr, fmt.Errorf("discover function %s for cleanup: %w", label, err))
				}
				continue
			}
			if function.ID != nil {
				if err := client.DeleteFunction(*function.ID); err != nil {
					runErr = errors.Join(runErr, fmt.Errorf("cleanup function %s: %w", label, err))
				}
			}
		}
		if err := client.DeleteCollection(compositionCollection); err != nil && !compositionNotFound(err) {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup collection %s: %w", compositionCollection, err))
		}
		if err := client.KVDelete("function_composition_user_cache_go:1"); err != nil && !compositionNotFound(err) {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup composition cache: %w", err))
		}
	}()

	fmt.Print("=== ekoDB Function Composition Examples ===\n\n")

	// Setup test data
	if err := setupTestData(client); err != nil {
		return fmt.Errorf("setup test data: %w", err)
	}

	// Run examples
	if err := basicCompositionExample(client); err != nil {
		return fmt.Errorf("basic composition example: %w", err)
	}

	if err := swrCompositionExample(client); err != nil {
		return fmt.Errorf("SWR composition example: %w", err)
	}

	if err := nestedCompositionExample(client); err != nil {
		return fmt.Errorf("nested composition example: %w", err)
	}

	fmt.Println("\n✅ All composition examples completed!")
	return nil
}

func compositionNotFound(err error) bool {
	var httpErr *ekodb.HTTPError
	if errors.As(err, &httpErr) && httpErr.StatusCode == 404 {
		return true
	}
	message := strings.ToLower(err.Error())
	return strings.Contains(message, "not found") || strings.Contains(message, "does not exist")
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
