package main

/*
Functions Example using Direct HTTP Requests

Demonstrates using scripts with raw HTTP/net/http API
No client library required
*/

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/joho/godotenv"
)

var (
	baseURL   string
	apiKey    string
	authToken string
)

const testCollection = "functions_users_http_go"
const activeLabel = "get_active_users_http_go"
const paginatedLabel = "get_active_users_paginated_http_go"
const statsLabel = "user_stats_http_go"

func init() {
	godotenv.Load("../.env")
	baseURL = getEnv("API_BASE_URL", "http://localhost:8080")
	apiKey = getEnv("API_BASE_KEY", "a-test-api-key-from-ekodb")
}

func getEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

func getAuthToken() (string, error) {
	if authToken != "" {
		return authToken, nil
	}

	body := map[string]string{"api_key": apiKey}
	jsonBody, _ := json.Marshal(body)

	resp, err := http.Post(baseURL+"/api/auth/token", "application/json", bytes.NewBuffer(jsonBody))
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	var result map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return "", err
	}

	authToken = result["token"].(string)
	return authToken, nil
}

func request(method, path string, body interface{}) (interface{}, error) {
	token, err := getAuthToken()
	if err != nil {
		return nil, err
	}

	var reqBody io.Reader
	if body != nil {
		jsonBody, _ := json.Marshal(body)
		reqBody = bytes.NewBuffer(jsonBody)
	}

	req, err := http.NewRequest(method, baseURL+path, reqBody)
	if err != nil {
		return nil, err
	}

	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		bodyBytes, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("HTTP %d: %s", resp.StatusCode, string(bodyBytes))
	}

	var result interface{}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, err
	}

	return result, nil
}

func functionRecords(raw interface{}) ([]interface{}, error) {
	result, ok := raw.(map[string]interface{})
	if !ok {
		return nil, fmt.Errorf("function response has type %T, expected object", raw)
	}
	records, ok := result["records"].([]interface{})
	if !ok {
		return nil, fmt.Errorf("function response records has type %T, expected array", result["records"])
	}
	return records, nil
}

func httpValue(value interface{}) interface{} {
	if wrapped, ok := value.(map[string]interface{}); ok {
		if inner, exists := wrapped["value"]; exists {
			return httpValue(inner)
		}
	}
	return value
}

func assertScores(records []interface{}, expected []float64) error {
	if len(records) != len(expected) {
		return fmt.Errorf("received %d records, expected %d", len(records), len(expected))
	}
	for index, raw := range records {
		record, ok := raw.(map[string]interface{})
		if !ok {
			return fmt.Errorf("record %d has type %T, expected object", index, raw)
		}
		score, ok := httpValue(record["score"]).(float64)
		if !ok || score != expected[index] {
			return fmt.Errorf("record %d score was %v, expected %.0f", index, record["score"], expected[index])
		}
	}
	return nil
}

// saveOrUpdateFunctionHTTP saves a function via POST /api/functions, or — if a
// function with the same label already exists (HTTP 409) — updates it in place
// via PUT /api/functions/{label}. This makes the example idempotent across
// repeated runs while still exercising both the create and update paths.
//
// It returns the function's encrypted ID so downstream GET/UPDATE/DELETE calls
// (which require the encrypted ID, not the label) keep working: on a fresh
// create the ID comes from the POST response; on the 409→update path it is
// recovered with a GET /api/functions/{label} (the server accepts the label).
func saveOrUpdateFunctionHTTP(function map[string]interface{}) (string, error) {
	label, _ := function["label"].(string)

	saveResultRaw, err := request("POST", "/api/functions", function)
	if err == nil {
		saveResult := saveResultRaw.(map[string]interface{})
		return saveResult["id"].(string), nil
	}

	// The request helper formats 4xx/5xx errors as "HTTP <code>: <body>".
	if !strings.Contains(err.Error(), "HTTP 409") {
		return "", err
	}

	if _, perr := request("PUT", "/api/functions/"+label, function); perr != nil {
		return "", perr
	}
	fmt.Printf("ℹ️  Function '%s' already existed — updated instead\n", label)

	fnRaw, gerr := request("GET", "/api/functions/"+label, nil)
	if gerr != nil {
		return "", gerr
	}
	fn := fnRaw.(map[string]interface{})
	id, ok := fn["id"].(string)
	if !ok {
		return "", fmt.Errorf("function %q has no id after update", label)
	}
	return id, nil
}

func setupTestData() error {
	fmt.Println("📋 Setting up test data...")

	for i := 1; i <= 10; i++ {
		status := "inactive"
		if i%2 == 0 {
			status = "active"
		}
		record := map[string]interface{}{
			"name":   fmt.Sprintf("User %d", i),
			"age":    20 + i,
			"status": status,
			"score":  i * 10,
		}
		if _, err := request("POST", "/api/insert/"+testCollection, record); err != nil {
			return err
		}
	}

	fmt.Print("✅ Test data ready\n\n")
	return nil
}

func simpleQueryFunction() (string, error) {
	fmt.Print("📝 Example 1: Simple Query Function with Filter\n\n")

	function1 := map[string]interface{}{
		"label":       activeLabel,
		"name":        "Get Active Users",
		"description": "Query users with active status",
		"version":     "1.0",
		"parameters":  map[string]interface{}{},
		"functions": []map[string]interface{}{
			{
				"type":       "Query",
				"collection": testCollection,
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "status",
						"operator": "Eq",
						"value":    "active",
					},
				},
			},
		},
		"tags": []string{"users", "query"},
	}

	// Save script (idempotent: update in place if the label already exists)
	functionID, err := saveOrUpdateFunctionHTTP(function1)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Function saved: %s\n", functionID)

	// Call script (can use label)
	callResultRaw, err := request("POST", "/api/functions/"+activeLabel, map[string]interface{}{})
	if err != nil {
		return "", err
	}
	records, err := functionRecords(callResultRaw)
	if err != nil {
		return functionID, err
	}
	if len(records) != 5 {
		return functionID, fmt.Errorf("active-users function returned %d records, expected 5", len(records))
	}
	for index, raw := range records {
		record, ok := raw.(map[string]interface{})
		if !ok || httpValue(record["status"]) != "active" {
			return functionID, fmt.Errorf("active-users result %d had unexpected status: %v", index, raw)
		}
	}
	fmt.Printf("📊 Found %d active users\n\n", len(records))

	return functionID, nil
}

func parameterizedPaginationFunction() (string, error) {
	fmt.Print("📝 Example 2: Parameterized Pagination with Limit/Skip\n\n")

	function2 := map[string]interface{}{
		"label":   paginatedLabel,
		"name":    "Get Active Users (Paginated)",
		"version": "1.0",
		"parameters": map[string]interface{}{
			"page_size": map[string]interface{}{
				"type":     "Integer",
				"required": false,
				"default":  5,
			},
			"page_offset": map[string]interface{}{
				"type":     "Integer",
				"required": false,
				"default":  0,
			},
		},
		"functions": []map[string]interface{}{
			{
				"type":       "Query",
				"collection": testCollection,
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "status",
						"operator": "Eq",
						"value":    "active",
					},
				},
				"sort":  []map[string]interface{}{{"field": "score", "ascending": false}},
				"limit": "{{page_size}}",
				"skip":  "{{page_offset}}",
			},
		},
		"tags": []string{"users", "pagination"},
	}

	functionID, err := saveOrUpdateFunctionHTTP(function2)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Function saved: %s\n", functionID)

	// Call with page 1 (first 3 users)
	callResultRaw, err := request("POST", "/api/functions/"+paginatedLabel, map[string]interface{}{
		"page_size":   3,
		"page_offset": 0,
	})
	if err != nil {
		return "", err
	}
	records, err := functionRecords(callResultRaw)
	if err != nil {
		return functionID, err
	}
	if len(records) != 3 {
		return functionID, fmt.Errorf("page 1 returned %d users, expected 3", len(records))
	}
	if err := assertScores(records, []float64{100, 80, 60}); err != nil {
		return functionID, fmt.Errorf("page 1: %w", err)
	}
	fmt.Printf("📊 Page 1: Found %d users (limit=3, skip=0)\n", len(records))

	// Call with page 2 (next 3 users)
	callResultRaw, err = request("POST", "/api/functions/"+paginatedLabel, map[string]interface{}{
		"page_size":   3,
		"page_offset": 3,
	})
	if err != nil {
		return "", err
	}
	records, err = functionRecords(callResultRaw)
	if err != nil {
		return functionID, err
	}
	if len(records) != 2 {
		return functionID, fmt.Errorf("page 2 returned %d users, expected 2", len(records))
	}
	if err := assertScores(records, []float64{40, 20}); err != nil {
		return functionID, fmt.Errorf("page 2: %w", err)
	}
	fmt.Printf("📊 Page 2: Found %d users (limit=3, skip=3)\n\n", len(records))

	return functionID, nil
}

func aggregationFunction() (string, error) {
	fmt.Print("📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)\n\n")

	function3 := map[string]interface{}{
		"label":      statsLabel,
		"name":       "User Statistics by Status",
		"version":    "1.0",
		"parameters": map[string]interface{}{},
		"functions": []map[string]interface{}{
			{
				"type":       "Query",
				"collection": testCollection,
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "age",
						"operator": "Gt",
						"value":    20,
					},
				},
			},
			{
				"type":      "Group",
				"by_fields": []string{"status"},
				"functions": []map[string]interface{}{
					{"output_field": "count", "operation": "Count"},
					{
						"output_field": "avg_score",
						"operation":    "Average",
						"input_field":  "score",
					},
					{
						"output_field": "max_score",
						"operation":    "Max",
						"input_field":  "score",
					},
				},
			},
		},
		"tags": []string{"analytics", "pipeline"},
	}

	functionID, err := saveOrUpdateFunctionHTTP(function3)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Function saved: %s\n", functionID)

	// Call script
	callResultRaw, err := request("POST", "/api/functions/"+statsLabel, map[string]interface{}{})
	if err != nil {
		return "", err
	}
	records, err := functionRecords(callResultRaw)
	if err != nil {
		return functionID, err
	}
	if len(records) != 2 {
		return functionID, fmt.Errorf("aggregation returned %d groups, expected 2", len(records))
	}
	expected := map[string][3]float64{
		"active":   {5, 60, 100},
		"inactive": {5, 50, 90},
	}
	for _, raw := range records {
		record, ok := raw.(map[string]interface{})
		if !ok {
			return functionID, fmt.Errorf("aggregation record has type %T, expected object", raw)
		}
		status, _ := httpValue(record["status"]).(string)
		want, ok := expected[status]
		if !ok {
			return functionID, fmt.Errorf("aggregation returned unexpected status %q", status)
		}
		count, countOK := httpValue(record["count"]).(float64)
		average, averageOK := httpValue(record["avg_score"]).(float64)
		maximum, maximumOK := httpValue(record["max_score"]).(float64)
		if !countOK || !averageOK || !maximumOK || count != want[0] || average != want[1] || maximum != want[2] {
			return functionID, fmt.Errorf("aggregation for %s was count/avg/max=%v/%v/%v, expected %.0f/%.0f/%.0f", status, record["count"], record["avg_score"], record["max_score"], want[0], want[1], want[2])
		}
	}
	fmt.Printf("📊 Pipeline Results: Filtered (age>20) → Grouped by status → %d groups\n", len(records))
	for _, record := range records {
		jsonData, _ := json.Marshal(record)
		fmt.Printf("   %s\n", string(jsonData))
	}
	fmt.Println()

	return functionID, nil
}

func functionManagement(getActiveUsersID, userStatsID string) error {
	fmt.Print("📝 Example 4: Function Management\n\n")

	// List all functions
	functionsResult, err := request("GET", "/api/functions", nil)
	if err != nil {
		return err
	}

	functionsArray := functionsResult.([]interface{})
	fmt.Printf("📋 Total functions: %d\n", len(functionsArray))

	// Get specific function (requires encrypted ID)
	fnResult, err := request("GET", "/api/functions/"+getActiveUsersID, nil)
	if err != nil {
		return err
	}
	fn := fnResult.(map[string]interface{})
	fmt.Printf("🔍 Retrieved function: %s\n", fn["name"])

	// Update function (requires encrypted ID)
	updated := map[string]interface{}{
		"label":       activeLabel,
		"name":        "Get Active Users (Updated)",
		"description": "Updated description",
		"version":     "1.1",
		"parameters":  map[string]interface{}{},
		"functions": []map[string]interface{}{
			{"type": "FindAll", "collection": testCollection},
		},
		"tags": []string{"users"},
	}
	if _, err := request("PUT", "/api/functions/"+getActiveUsersID, updated); err != nil {
		return err
	}
	fmt.Println("✏️  Function updated")

	// Delete function (requires encrypted ID)
	if _, err := request("DELETE", "/api/functions/"+userStatsID, nil); err != nil {
		return err
	}
	fmt.Print("🗑️  Function deleted\n\n")

	fmt.Println("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
	fmt.Print("ℹ️  Only CALL can use either ID or label\n\n")

	return nil
}

func run() (runErr error) {
	fmt.Print("🚀 ekoDB Functions Example (Go/HTTP)\n\n")

	// Start clean and guarantee cleanup of the language-specific resources.
	if _, err := request("DELETE", "/api/collections/"+testCollection, nil); err != nil && !strings.Contains(err.Error(), "HTTP 404") && !strings.Contains(err.Error(), "not found") {
		return fmt.Errorf("initial collection cleanup: %w", err)
	}
	functionIDs := make([]string, 0, 4)
	defer func() {
		var cleanupErrors []error
		if functionsRaw, err := request("GET", "/api/functions", nil); err == nil {
			if functions, ok := functionsRaw.([]interface{}); ok {
				for _, raw := range functions {
					function, ok := raw.(map[string]interface{})
					if !ok {
						continue
					}
					label, _ := function["label"].(string)
					id, _ := function["id"].(string)
					if (label == activeLabel || label == paginatedLabel || label == statsLabel) && id != "" {
						found := false
						for _, existing := range functionIDs {
							found = found || existing == id
						}
						if !found {
							functionIDs = append(functionIDs, id)
						}
					}
				}
			} else {
				cleanupErrors = append(cleanupErrors, fmt.Errorf("discover functions: response has type %T", functionsRaw))
			}
		} else {
			cleanupErrors = append(cleanupErrors, fmt.Errorf("discover functions for cleanup: %w", err))
		}
		paths := make([]string, 0, len(functionIDs)+1)
		for _, id := range functionIDs {
			paths = append(paths, "/api/functions/"+id)
		}
		paths = append(paths, "/api/collections/"+testCollection)
		for _, path := range paths {
			if _, err := request("DELETE", path, nil); err != nil && !strings.Contains(err.Error(), "HTTP 404") && !strings.Contains(err.Error(), "not found") {
				cleanupErrors = append(cleanupErrors, fmt.Errorf("cleanup %s: %w", path, err))
			}
		}
		runErr = errors.Join(append([]error{runErr}, cleanupErrors...)...)
	}()

	if err := setupTestData(); err != nil {
		return fmt.Errorf("setup test data: %w", err)
	}

	getActiveUsersID, err := simpleQueryFunction()
	if err != nil {
		return err
	}
	functionIDs = append(functionIDs, getActiveUsersID)

	paginatedID, err := parameterizedPaginationFunction()
	if err != nil {
		return err
	}
	functionIDs = append(functionIDs, paginatedID)

	userStatsID, err := aggregationFunction()
	if err != nil {
		return err
	}
	functionIDs = append(functionIDs, userStatsID)

	if err := functionManagement(getActiveUsersID, userStatsID); err != nil {
		return err
	}
	if activeRaw, err := request("GET", "/api/functions/"+activeLabel, nil); err == nil {
		if active, ok := activeRaw.(map[string]interface{}); ok {
			if id, ok := active["id"].(string); ok && id != getActiveUsersID {
				functionIDs = append(functionIDs, id)
			}
		}
	} else {
		return fmt.Errorf("read updated function for cleanup: %w", err)
	}

	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✅ All examples completed!")
}
