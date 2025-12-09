package main

/*
Scripts Example using Direct HTTP Requests

Demonstrates using scripts with raw HTTP/net/http API
No client library required
*/

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/joho/godotenv"
)

var (
	baseURL   string
	apiKey    string
	authToken string
)

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
		if _, err := request("POST", "/api/insert/users", record); err != nil {
			return err
		}
	}

	fmt.Println("✅ Test data ready\n")
	return nil
}

func simpleQueryFunction() (string, error) {
	fmt.Println("📝 Example 1: Simple Query Function with Filter\n")

	function1 := map[string]interface{}{
		"label":       "get_active_users",
		"name":        "Get Active Users",
		"description": "Query users with active status",
		"version":     "1.0",
		"parameters":  map[string]interface{}{},
		"functions": []map[string]interface{}{
			{
				"type":       "Query",
				"collection": "users",
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

	// Save script
	saveResultRaw, err := request("POST", "/api/functions", function1)
	if err != nil {
		return "", err
	}
	saveResult := saveResultRaw.(map[string]interface{})
	fmt.Printf("✅ Script saved: %s\n", saveResult["id"])

	// Call script (can use label)
	callResultRaw, err := request("POST", "/api/functions/get_active_users", map[string]interface{}{})
	if err != nil {
		return "", err
	}
	callResult := callResultRaw.(map[string]interface{})

	records := callResult["records"].([]interface{})
	fmt.Printf("📊 Found %d active users\n\n", len(records))

	return saveResult["id"].(string), nil
}

func parameterizedPaginationFunction() error {
	fmt.Println("📝 Example 2: Parameterized Pagination with Limit/Skip\n")

	function2 := map[string]interface{}{
		"label":   "get_active_users_paginated",
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
				"collection": "users",
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

	saveResultRaw, err := request("POST", "/api/functions", function2)
	if err != nil {
		return err
	}
	saveResult := saveResultRaw.(map[string]interface{})
	fmt.Printf("✅ Script saved: %s\n", saveResult["id"])

	// Call with page 1 (first 3 users)
	callResultRaw, err := request("POST", "/api/functions/get_active_users_paginated", map[string]interface{}{
		"page_size":   3,
		"page_offset": 0,
	})
	if err != nil {
		return err
	}
	callResult := callResultRaw.(map[string]interface{})
	records := callResult["records"].([]interface{})
	fmt.Printf("📊 Page 1: Found %d users (limit=3, skip=0)\n", len(records))

	// Call with page 2 (next 3 users)
	callResultRaw, err = request("POST", "/api/functions/get_active_users_paginated", map[string]interface{}{
		"page_size":   3,
		"page_offset": 3,
	})
	if err != nil {
		return err
	}
	callResult = callResultRaw.(map[string]interface{})
	records = callResult["records"].([]interface{})
	fmt.Printf("📊 Page 2: Found %d users (limit=3, skip=3)\n\n", len(records))

	return nil
}

func aggregationFunction() (string, error) {
	fmt.Println("📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)\n")

	function3 := map[string]interface{}{
		"label":      "user_stats",
		"name":       "User Statistics by Status",
		"version":    "1.0",
		"parameters": map[string]interface{}{},
		"functions": []map[string]interface{}{
			{
				"type":       "Query",
				"collection": "users",
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

	saveResultRaw, err := request("POST", "/api/functions", function3)
	if err != nil {
		return "", err
	}
	saveResult := saveResultRaw.(map[string]interface{})
	fmt.Printf("✅ Script saved: %s\n", saveResult["id"])

	// Call script
	callResultRaw, err := request("POST", "/api/functions/user_stats", map[string]interface{}{})
	if err != nil {
		return "", err
	}
	callResult := callResultRaw.(map[string]interface{})

	records := callResult["records"].([]interface{})
	fmt.Printf("📊 Pipeline Results: Filtered (age>20) → Grouped by status → %d groups\n", len(records))
	for _, record := range records {
		jsonData, _ := json.Marshal(record)
		fmt.Printf("   %s\n", string(jsonData))
	}
	fmt.Println()

	return saveResult["id"].(string), nil
}

func functionManagement(getActiveUsersID, userStatsID string) error {
	fmt.Println("📝 Example 4: Function Management\n")

	// List all scripts
	scripts, err := request("GET", "/api/functions", nil)
	if err != nil {
		return err
	}

	scriptsArray := scripts.([]interface{})
	fmt.Printf("📋 Total scripts: %d\n", len(scriptsArray))

	// Get specific script (requires encrypted ID)
	scriptResult, err := request("GET", "/api/functions/"+getActiveUsersID, nil)
	if err != nil {
		return err
	}
	script := scriptResult.(map[string]interface{})
	fmt.Printf("🔍 Retrieved script: %s\n", script["name"])

	// Update script (requires encrypted ID)
	updated := map[string]interface{}{
		"label":       "get_active_users",
		"name":        "Get Active Users (Updated)",
		"description": "Updated description",
		"version":     "1.1",
		"parameters":  map[string]interface{}{},
		"functions": []map[string]interface{}{
			{"type": "FindAll", "collection": "users"},
		},
		"tags": []string{"users"},
	}
	if _, err := request("PUT", "/api/functions/"+getActiveUsersID, updated); err != nil {
		return err
	}
	fmt.Println("✏️  Script updated")

	// Delete script (requires encrypted ID)
	if _, err := request("DELETE", "/api/functions/"+userStatsID, nil); err != nil {
		return err
	}
	fmt.Println("🗑️  Script deleted\n")

	fmt.Println("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
	fmt.Println("ℹ️  Only CALL can use either ID or label\n")

	return nil
}

func main() {
	fmt.Println("🚀 ekoDB Scripts Example (Go/HTTP)\n")

	if err := setupTestData(); err != nil {
		fmt.Printf("❌ Error: %v\n", err)
		os.Exit(1)
	}

	getActiveUsersID, err := simpleQueryFunction()
	if err != nil {
		log.Fatal(err)
	}

	if err := parameterizedPaginationFunction(); err != nil {
		log.Fatal(err)
	}

	userStatsID, err := aggregationFunction()
	if err != nil {
		log.Fatal(err)
		os.Exit(1)
	}

	if err := functionManagement(getActiveUsersID, userStatsID); err != nil {
		fmt.Printf("❌ Error: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("✅ All examples completed!")
}
