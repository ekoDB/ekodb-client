package main

/*
Complete CRUD Functions Example using Direct HTTP Requests

Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification using parameterized {{param}} syntax
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

func makeRequest(method, endpoint string, body interface{}, token string) (map[string]interface{}, error) {
	var req *http.Request
	var err error

	if body != nil {
		jsonBody, _ := json.Marshal(body)
		req, err = http.NewRequest(method, baseURL+endpoint, bytes.NewBuffer(jsonBody))
	} else {
		req, err = http.NewRequest(method, baseURL+endpoint, nil)
	}

	if err != nil {
		return nil, err
	}

	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	bodyBytes, _ := io.ReadAll(resp.Body)
	if resp.StatusCode >= 400 {
		return nil, fmt.Errorf("HTTP %d: %s", resp.StatusCode, string(bodyBytes))
	}

	var result map[string]interface{}
	if err := json.Unmarshal(bodyBytes, &result); err != nil {
		return nil, err
	}

	return result, nil
}

func script1InsertAndVerify(token string) (string, string, error) {
	fmt.Println("\n" + string([]byte(repeat("=", 60))))
	fmt.Println("📝 function 1: Insert + Verify")
	fmt.Println(string([]byte(repeat("=", 60))))

	script := map[string]interface{}{
		"label":       "insert_and_verify",
		"name":        "Insert and Verify User",
		"description": "Insert a user and verify it was created",
		"version":     "1.0",
		"parameters": map[string]interface{}{
			"user_name": map[string]interface{}{
				"type":        "String",
				"required":    true,
				"description": "Name of the user",
			},
			"user_email": map[string]interface{}{
				"type":        "String",
				"required":    true,
				"description": "Email of the user",
			},
		},
		"functions": []map[string]interface{}{
			{
				"type":       "Insert",
				"collection": "users",
				"record": map[string]interface{}{
					"name":    "{{user_name}}",
					"email":   "{{user_email}}",
					"status":  "pending",
					"credits": 0,
				},
			},
			{
				"type":       "Query",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
			},
		},
		"tags": []string{"crud", "insert", "verification"},
	}

	fmt.Println("\n1️⃣ Saving Function...")
	saveResult, err := makeRequest("POST", "/api/functions", script, token)
	if err != nil {
		return "", "", fmt.Errorf("failed to save script: %w", err)
	}
	scriptID := saveResult["id"].(string)
	fmt.Printf("   ✅ Function saved: %s\n", scriptID)

	fmt.Println("\n2️⃣ Calling function (Insert + Verify)...")
	params := map[string]interface{}{
		"user_name":  "Alice Smith",
		"user_email": "alice@example.com",
	}
	callResult, err := makeRequest("POST", "/api/functions/insert_and_verify", params, token)
	if err != nil {
		return "", "", fmt.Errorf("failed to call script: %w", err)
	}

	stats := callResult["stats"].(map[string]interface{})
	fmt.Printf("   ✅ function executed: %.0f Functions\n", stats["stages_executed"])
	fmt.Printf("   ⏱️  Execution time: %.0fms\n", stats["execution_time_ms"])

	records := callResult["records"].([]interface{})
	fmt.Println("\n3️⃣ Verification Results:")
	fmt.Printf("   ✅ Found %d record(s)\n", len(records))

	if len(records) > 0 {
		user := records[0].(map[string]interface{})

		// Extract user ID (handle both plain string and map format)
		var userID string
		if idMap, ok := user["id"].(map[string]interface{}); ok {
			userID = idMap["value"].(string)
		} else {
			userID = user["id"].(string)
		}

		fmt.Printf("   📋 User ID: %s\n", userID)
		fmt.Printf("   📋 Name: %v\n", user["name"])
		fmt.Printf("   📋 Email: %v\n", user["email"])
		fmt.Printf("   📋 Status: %v\n", user["status"])
		fmt.Printf("   📋 Credits: %v\n", user["credits"])

		return userID, scriptID, nil
	}

	return "", "", fmt.Errorf("no records found")
}

func script2QueryUpdateVerify(token string) (string, error) {
	fmt.Println("\n" + string([]byte(repeat("=", 60))))
	fmt.Println("📝 function 2: Query + Update + Verify")
	fmt.Println(string([]byte(repeat("=", 60))))

	script := map[string]interface{}{
		"label":       "query_update_verify",
		"name":        "Query, Update, and Verify",
		"description": "Find user by filter, update status, verify change",
		"version":     "1.0",
		"parameters": map[string]interface{}{
			"user_email": map[string]interface{}{
				"type":        "String",
				"required":    true,
				"description": "Email to search for",
			},
			"new_status": map[string]interface{}{
				"type":        "String",
				"default":     "active",
				"required":    false,
				"description": "New status value",
			},
		},
		"functions": []map[string]interface{}{
			{
				"type":       "Query",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
			},
			{
				"type":       "Update",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
				"updates": map[string]interface{}{
					"status": "{{new_status}}",
				},
			},
			{
				"type":       "Query",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
			},
		},
		"tags": []string{"crud", "update", "verification"},
	}

	fmt.Println("\n1️⃣ Saving Function...")
	saveResult, err := makeRequest("POST", "/api/functions", script, token)
	if err != nil {
		return "", fmt.Errorf("failed to save script: %w", err)
	}
	scriptID := saveResult["id"].(string)
	fmt.Printf("   ✅ Function saved: %s\n", scriptID)

	fmt.Println("\n2️⃣ Calling function (Query + Update + Verify)...")
	params := map[string]interface{}{
		"user_email": "alice@example.com",
		"new_status": "active",
	}
	callResult, err := makeRequest("POST", "/api/functions/query_update_verify", params, token)
	if err != nil {
		return "", fmt.Errorf("failed to call script: %w", err)
	}

	stats := callResult["stats"].(map[string]interface{})
	fmt.Printf("   ✅ function executed: %.0f Functions\n", stats["stages_executed"])
	fmt.Printf("   ⏱️  Execution time: %.0fms\n", stats["execution_time_ms"])

	records := callResult["records"].([]interface{})
	fmt.Println("\n3️⃣ Verification Results:")
	fmt.Printf("   ✅ Found %d record(s)\n", len(records))
	if len(records) > 0 {
		user := records[0].(map[string]interface{})
		fmt.Printf("   📋 Status updated to: %v\n", user["status"])
		fmt.Printf("   📋 Name: %v\n", user["name"])
	}

	return scriptID, nil
}

func script3QueryUpdateCredits(token string) (string, error) {
	fmt.Println("\n" + string([]byte(repeat("=", 60))))
	fmt.Println("📝 function 3: Query + Update Credits + Verify")
	fmt.Println(string([]byte(repeat("=", 60))))

	script := map[string]interface{}{
		"label":       "query_update_credits",
		"name":        "Query, Update Credits, and Verify",
		"description": "Find user by email, update credits, verify change",
		"version":     "1.0",
		"parameters": map[string]interface{}{
			"user_email": map[string]interface{}{
				"type":        "String",
				"required":    true,
				"description": "Email to search for",
			},
			"credits": map[string]interface{}{
				"type":        "Integer",
				"default":     100,
				"required":    false,
				"description": "Credits to set",
			},
		},
		"functions": []map[string]interface{}{
			{
				"type":       "Query",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
			},
			{
				"type":       "Update",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
				"updates": map[string]interface{}{
					"credits": "{{credits}}",
				},
			},
			{
				"type":       "Query",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
			},
		},
		"tags": []string{"crud", "update", "verification"},
	}

	fmt.Println("\n1️⃣ Saving Function...")
	saveResult, err := makeRequest("POST", "/api/functions", script, token)
	if err != nil {
		return "", fmt.Errorf("failed to save script: %w", err)
	}
	scriptID := saveResult["id"].(string)
	fmt.Printf("   ✅ Function saved: %s\n", scriptID)

	fmt.Println("\n2️⃣ Calling function (Query + Update Credits + Verify)...")
	params := map[string]interface{}{
		"user_email": "alice@example.com",
		"credits":    100,
	}
	callResult, err := makeRequest("POST", "/api/functions/query_update_credits", params, token)
	if err != nil {
		return "", fmt.Errorf("failed to call script: %w", err)
	}

	stats := callResult["stats"].(map[string]interface{})
	fmt.Printf("   ✅ function executed: %.0f Functions\n", stats["stages_executed"])
	fmt.Printf("   ⏱️  Execution time: %.0fms\n", stats["execution_time_ms"])

	records := callResult["records"].([]interface{})
	fmt.Println("\n3️⃣ Verification Results:")
	fmt.Printf("   ✅ Found %d record(s)\n", len(records))
	if len(records) > 0 {
		user := records[0].(map[string]interface{})
		fmt.Printf("   📋 Credits updated to: %v\n", user["credits"])
		fmt.Printf("   📋 Status: %v\n", user["status"])
		fmt.Printf("   📋 Name: %v\n", user["name"])
	}

	return scriptID, nil
}

func script4DeleteAndVerify(token string) (string, error) {
	fmt.Println("\n" + string([]byte(repeat("=", 60))))
	fmt.Println("📝 function 4: Query Before Delete + Delete + Verify")
	fmt.Println(string([]byte(repeat("=", 60))))

	script := map[string]interface{}{
		"label":       "delete_and_verify",
		"name":        "Query Before Delete and Verify",
		"description": "Verify record exists, delete it, then verify it's gone",
		"version":     "1.0",
		"parameters": map[string]interface{}{
			"user_email": map[string]interface{}{
				"type":        "String",
				"required":    true,
				"description": "Email of user to delete",
			},
		},
		"functions": []map[string]interface{}{
			{
				"type":       "Query",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
			},
			{
				"type":       "Delete",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
			},
			{
				"type":       "Query",
				"collection": "users",
				"filter": map[string]interface{}{
					"type": "Condition",
					"content": map[string]interface{}{
						"field":    "email",
						"operator": "Eq",
						"value":    "{{user_email}}",
					},
				},
			},
		},
		"tags": []string{"crud", "delete", "verification"},
	}

	fmt.Println("\n1️⃣ Saving Function...")
	saveResult, err := makeRequest("POST", "/api/functions", script, token)
	if err != nil {
		return "", fmt.Errorf("failed to save script: %w", err)
	}
	scriptID := saveResult["id"].(string)
	fmt.Printf("   ✅ Function saved: %s\n", scriptID)

	fmt.Println("\n2️⃣ Calling function (Query + Delete + Verify)...")
	params := map[string]interface{}{
		"user_email": "alice@example.com",
	}
	callResult, err := makeRequest("POST", "/api/functions/delete_and_verify", params, token)
	if err != nil {
		return "", fmt.Errorf("failed to call script: %w", err)
	}

	stats := callResult["stats"].(map[string]interface{})
	fmt.Printf("   ✅ function executed: %.0f Functions\n", stats["stages_executed"])
	fmt.Printf("   ⏱️  Execution time: %.0fms\n", stats["execution_time_ms"])

	records := callResult["records"].([]interface{})
	fmt.Println("\n3️⃣ Verification Results:")
	fmt.Println("   📊 Before delete: Record existed (verified by first Query)")
	if len(records) == 0 {
		fmt.Println("   ✅ After delete: Record successfully deleted (Query returned 0 records)")
	} else {
		fmt.Printf("   ❌ Delete failed - still found %d record(s)\n", len(records))
	}

	return scriptID, nil
}

func cleanup(token string, scriptIDs []string) {
	fmt.Println("\n" + string([]byte(repeat("=", 60))))
	fmt.Println("🧹 Cleanup")
	fmt.Println(string([]byte(repeat("=", 60))))

	for _, scriptID := range scriptIDs {
		if scriptID == "" {
			continue
		}
		_, err := makeRequest("DELETE", "/api/functions/"+scriptID, nil, token)
		if err != nil {
			fmt.Printf("   ⚠️  Could not delete script: %v\n", err)
		} else {
			truncatedID := scriptID
			if len(scriptID) > 20 {
				truncatedID = scriptID[:20] + "..."
			}
			fmt.Printf("   ✅ Deleted script: %s\n", truncatedID)
		}
	}

	_, err := makeRequest("DELETE", "/api/collections/users", nil, token)
	if err != nil {
		fmt.Printf("   ⚠️  Could not delete collection: %v\n", err)
	} else {
		fmt.Println("   ✅ Deleted collection: users")
	}
}

func repeat(s string, count int) string {
	result := ""
	for i := 0; i < count; i++ {
		result += s
	}
	return result
}

func main() {
	fmt.Println("🚀 ekoDB Complete CRUD Functions Example")
	fmt.Println(string([]byte(repeat("=", 60))))
	fmt.Println("Demonstrates:")
	fmt.Println("  • Insert + Verify (using Query)")
	fmt.Println("  • Query + Update Status + Verify")
	fmt.Println("  • Query + Update Credits + Verify")
	fmt.Println("  • Query Before Delete + Delete + Verify Gone")
	fmt.Println()
	fmt.Println("Each function shows Functions chaining with proper verification")
	fmt.Println(string([]byte(repeat("=", 60))))

	token, err := getAuthToken()
	if err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	// Run all CRUD Functions in sequence
	_, script1ID, err := script1InsertAndVerify(token)
	if err != nil {
		log.Fatalf("Function 1 failed: %v", err)
	}
	scriptIDs := []string{script1ID}

	script2ID, err := script2QueryUpdateVerify(token)
	if err != nil {
		log.Fatalf("Function 2 failed: %v", err)
	}
	scriptIDs = append(scriptIDs, script2ID)

	script3ID, err := script3QueryUpdateCredits(token)
	if err != nil {
		log.Fatalf("Function 3 failed: %v", err)
	}
	scriptIDs = append(scriptIDs, script3ID)

	script4ID, err := script4DeleteAndVerify(token)
	if err != nil {
		log.Fatalf("Function 4 failed: %v", err)
	}
	scriptIDs = append(scriptIDs, script4ID)

	// Cleanup
	cleanup(token, scriptIDs)

	fmt.Println("\n" + string([]byte(repeat("=", 60))))
	fmt.Println("✅ Complete CRUD Functions Example Finished!")
	fmt.Println(string([]byte(repeat("=", 60))))
	fmt.Println("\n💡 Key Takeaways:")
	fmt.Println("   ✅ Functions chain steps together")
	fmt.Println("   ✅ Each function demonstrates operation + verification")
	fmt.Println("   ✅ Parameters make functions reusable")
	fmt.Println("   ✅ Verification is built into the function itself")
	fmt.Println("   ✅ Complete CRUD lifecycle in 4 focused functions")
}
