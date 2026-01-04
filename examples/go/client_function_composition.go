/**
 * Function Composition Example for ekoDB Go Client
 *
 * Demonstrates calling Functions within Functions using CallFunction
 * Shows how to build reusable logic blocks and compose complex workflows
 */

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

func setupTestData(client *ekodb.Client) error {
	fmt.Println("📋 Setting up test data...\n")

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

		if _, err := client.Insert("users", record); err != nil {
			return err
		}
	}

	fmt.Println("✅ Test data ready\n")
	return nil
}

func basicCompositionExample(client *ekodb.Client) error {
	fmt.Println("📝 Example 1: Basic Function Composition\n")
	fmt.Println("Building reusable functions that call each other...\n")

	// Step 1: Create reusable "fetch_user" function
	fetchUser := ekodb.Script{
		Label: "fetch_user",
		Name:  "Fetch user by code",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindOne("users", "user_code", "{{user_code}}"),
		},
		Tags: []string{},
	}

	if _, err := client.SaveScript(fetchUser); err != nil {
		return err
	}
	fmt.Println("✅ Saved reusable function: fetch_user")

	// Step 2: Create wrapper that CALLS fetch_user
	getUserWrapper := ekodb.Script{
		Label: "get_user_wrapper",
		Name:  "Wrapper that calls fetch_user",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageCallFunction("fetch_user", map[string]interface{}{
				"user_code": "{{user_code}}",
			}),
			ekodb.StageProject([]string{"name", "department"}, false),
		},
	}

	if _, err := client.SaveScript(getUserWrapper); err != nil {
		return err
	}
	fmt.Println("✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)\n")

	// Step 3: Call the composed function
	params := map[string]interface{}{
		"user_code": "user_1",
	}
	result, err := client.CallScript("get_user_wrapper", params)
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
	fmt.Println("   No code duplication, single source of truth\n")

	return nil
}

func swrCompositionExample(client *ekodb.Client) error {
	fmt.Println("📝 Example 2: SWR Pattern with Function Composition\n")
	fmt.Println("Using CallFunction to replace inline logic in SWR pattern...\n")

	// Step 1: Create reusable fetch and store function
	ttl := int64(300)
	fetchAndStore := ekodb.Script{
		Label: "fetch_and_store_github",
		Name:  "Fetch from GitHub and store",
		Parameters: map[string]ekodb.ParameterDefinition{
			"username": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageHttpRequest(
				"https://api.github.com/users/{{username}}",
				"GET",
				map[string]string{"User-Agent": "ekoDB-Client"},
				nil,
			),
			ekodb.StageInsert(
				"github_cache",
				map[string]interface{}{
					"id":   map[string]interface{}{"type": "String", "value": "{{username}}"},
					"data": map[string]interface{}{"type": "Object", "value": "{{http_response}}"},
				},
				false,
				&ttl,
			),
		},
	}

	if _, err := client.SaveScript(fetchAndStore); err != nil {
		return err
	}
	fmt.Println("✅ Saved reusable function: fetch_and_store_github")

	// Step 2: Create SWR function that CALLS the reusable function
	swrGithub := ekodb.Script{
		Label: "swr_github_user",
		Name:  "SWR pattern using reusable functions",
		Parameters: map[string]ekodb.ParameterDefinition{
			"username": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindById("github_cache", "{{username}}"),
			ekodb.StageIf(
				ekodb.ConditionHasRecords(),
				[]ekodb.FunctionStageConfig{
					ekodb.StageProject([]string{"data"}, false),
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageCallFunction("fetch_and_store_github", map[string]interface{}{
						"username": "{{username}}",
					}),
				},
			),
		},
	}

	if _, err := client.SaveScript(swrGithub); err != nil {
		return err
	}
	fmt.Println("✅ Saved SWR function using composition: swr_github_user\n")

	// Step 3: Test cache miss
	fmt.Println("First call (cache miss - will fetch from GitHub):")
	params := map[string]interface{}{
		"username": "torvalds",
	}

	start1 := time.Now()
	result1, err := client.CallScript("swr_github_user", params)
	if err != nil {
		return err
	}
	duration1 := time.Since(start1)

	fmt.Printf("   ⏱️  Duration: %v\n", duration1)
	fmt.Printf("   📊 Records: %d\n\n", len(result1.Records))

	// Step 4: Test cache hit
	fmt.Println("Second call (cache hit - from cache):")
	start2 := time.Now()
	result2, err := client.CallScript("swr_github_user", params)
	if err != nil {
		return err
	}
	duration2 := time.Since(start2)

	fmt.Printf("   ⏱️  Duration: %v\n", duration2)
	fmt.Printf("   📊 Records: %d\n", len(result2.Records))
	if duration2.Milliseconds() > 0 {
		speedup := float64(duration1.Milliseconds()) / float64(duration2.Milliseconds())
		fmt.Printf("   🚀 Cache speedup: %.1fx faster!\n\n", speedup)
	}

	return nil
}

func nestedCompositionExample(client *ekodb.Client) error {
	fmt.Println("📝 Example 3: Multi-Level Function Composition\n")
	fmt.Println("Building complex workflows from small, reusable pieces...\n")

	// Level 1: Base function
	validateUser := ekodb.Script{
		Label: "validate_user",
		Name:  "Check if user exists",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindOne("users", "user_code", "{{user_code}}"),
		},
		Tags: []string{},
	}

	if _, err := client.SaveScript(validateUser); err != nil {
		return err
	}
	fmt.Println("✅ Level 1 function: validate_user")

	// Level 2: Calls validate_user + projects
	fetchSlim := ekodb.Script{
		Label: "fetch_slim_user",
		Name:  "Validate and slim down user",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageCallFunction("validate_user", map[string]interface{}{
				"user_code": "{{user_code}}",
			}),
			ekodb.StageProject([]string{"name", "department"}, false),
		},
	}

	if _, err := client.SaveScript(fetchSlim); err != nil {
		return err
	}
	fmt.Println("✅ Level 2 function: fetch_slim_user (calls validate_user)")

	// Level 3: Calls fetch_slim + counts
	countUser := ekodb.Script{
		Label: "count_validated_user",
		Name:  "Get validated user and count",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_code": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageCallFunction("fetch_slim_user", map[string]interface{}{
				"user_code": "{{user_code}}",
			}),
			ekodb.StageCount("record_count"),
		},
	}

	if _, err := client.SaveScript(countUser); err != nil {
		return err
	}
	fmt.Println("✅ Level 3 function: count_validated_user (calls fetch_slim_user)\n")

	// Execute 3-level nested composition
	params := map[string]interface{}{
		"user_code": "user_1",
	}
	result, err := client.CallScript("count_validated_user", params)
	if err != nil {
		return err
	}

	fmt.Println("📊 Result from 3-level nested composition:")
	fmt.Printf("   Records: %d\n", len(result.Records))
	if len(result.Records) > 0 {
		record := result.Records[0]
		nameJSON, _ := json.Marshal(record["name"])
		deptJSON, _ := json.Marshal(record["department"])
		countJSON, _ := json.Marshal(record["record_count"])
		fmt.Printf("   Name: %s\n", nameJSON)
		fmt.Printf("   Department: %s\n", deptJSON)
		fmt.Printf("   Record count: %s\n\n", countJSON)
	}

	fmt.Println("🎯 Key Benefit: Each function is independently testable and reusable!")
	fmt.Println("   - validate_user: Used in 100 different workflows")
	fmt.Println("   - fetch_slim_user: Used in 50 workflows")
	fmt.Println("   - count_validated_user: Specific workflow\n")

	return nil
}

func main() {
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
		log.Fatalf("Failed to create client: %v", err)
	}

	fmt.Println("=== ekoDB Function Composition Examples ===\n")

	// Setup test data
	if err := setupTestData(client); err != nil {
		log.Fatalf("Failed to setup test data: %v", err)
	}

	// Run examples
	if err := basicCompositionExample(client); err != nil {
		log.Fatalf("Basic composition example failed: %v", err)
	}

	if err := swrCompositionExample(client); err != nil {
		log.Fatalf("SWR composition example failed: %v", err)
	}

	if err := nestedCompositionExample(client); err != nil {
		log.Fatalf("Nested composition example failed: %v", err)
	}

	fmt.Println("\n✅ All composition examples completed!")
}
