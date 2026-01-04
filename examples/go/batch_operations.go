package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
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
	godotenv.Load()
	baseURL = os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey = os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}
}

func getAuthToken() (string, error) {
	if authToken != "" {
		return authToken, nil
	}

	body, _ := json.Marshal(map[string]string{"api_key": apiKey})
	resp, err := http.Post(baseURL+"/api/auth/token", "application/json", bytes.NewBuffer(body))
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	var result map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&result)
	authToken = result["token"].(string)
	return authToken, nil
}

func request(method, path string, body interface{}) (map[string]interface{}, error) {
	token, err := getAuthToken()
	if err != nil {
		return nil, err
	}

	var reqBody io.Reader
	if body != nil {
		jsonBody, _ := json.Marshal(body)
		reqBody = bytes.NewBuffer(jsonBody)
	}

	req, _ := http.NewRequest(method, baseURL+path, reqBody)
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode == 404 {
		return nil, nil
	}

	respBody, _ := io.ReadAll(resp.Body)
	if len(respBody) == 0 {
		return map[string]interface{}{}, nil
	}

	var result map[string]interface{}
	json.Unmarshal(respBody, &result)
	return result, nil
}

func main() {
	fmt.Println("=== Batch Operations (Direct HTTP) ===\n")

	_, err := getAuthToken()
	if err != nil {
		fmt.Printf("Auth failed: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("✓ Authentication successful")

	// Example 1: Batch Insert
	fmt.Println("\n=== Batch Insert ===")
	records := []map[string]interface{}{
		{"name": "User 1", "email": "user1@example.com", "age": 25},
		{"name": "User 2", "email": "user2@example.com", "age": 30},
		{"name": "User 3", "email": "user3@example.com", "age": 35},
		{"name": "User 4", "email": "user4@example.com", "age": 40},
		{"name": "User 5", "email": "user5@example.com", "age": 45},
	}

	inserts := make([]map[string]interface{}, len(records))
	for i, r := range records {
		inserts[i] = map[string]interface{}{"data": r}
	}

	batchInsertData := map[string]interface{}{
		"inserts":       inserts,
		"bypass_ripple": false,
	}

	insertResult, _ := request("POST", "/api/batch/insert/batch_users", batchInsertData)
	insertedCount := 0
	if successful, ok := insertResult["successful"].([]interface{}); ok {
		insertedCount = len(successful)
	}
	fmt.Printf("✓ Batch inserted %d records\n", insertedCount)

	// Example 2: Create test records for update/delete
	fmt.Println("\n=== Creating test records for update/delete ===")
	var ids []string
	for i := 0; i < 3; i++ {
		doc, _ := request("POST", "/api/insert/batch_users", map[string]interface{}{
			"name":  fmt.Sprintf("Test User %d", i),
			"value": i,
		})
		if doc != nil {
			ids = append(ids, doc["id"].(string))
		}
	}
	fmt.Printf("Created %d test records\n", len(ids))

	// Example 3: Batch Update
	fmt.Println("\n=== Batch Update ===")
	updates := make([]map[string]interface{}, len(ids))
	for i, id := range ids {
		updates[i] = map[string]interface{}{
			"id":   id,
			"data": map[string]interface{}{"name": fmt.Sprintf("Updated User %d", i), "status": "active"},
		}
	}

	batchUpdateData := map[string]interface{}{
		"updates":       updates,
		"bypass_ripple": false,
	}

	updateResult, _ := request("PUT", "/api/batch/update/batch_users", batchUpdateData)
	updatedCount := 0
	if successful, ok := updateResult["successful"].([]interface{}); ok {
		updatedCount = len(successful)
	}
	fmt.Printf("✓ Batch updated %d records\n", updatedCount)

	// Example 4: Batch Delete
	fmt.Println("\n=== Batch Delete ===")
	deletes := make([]map[string]interface{}, len(ids))
	for i, id := range ids {
		deletes[i] = map[string]interface{}{"id": id}
	}

	batchDeleteData := map[string]interface{}{
		"deletes":       deletes,
		"bypass_ripple": false,
	}

	deleteResult, _ := request("DELETE", "/api/batch/delete/batch_users", batchDeleteData)
	deletedCount := 0
	if successful, ok := deleteResult["successful"].([]interface{}); ok {
		deletedCount = len(successful)
	}
	fmt.Printf("✓ Batch deleted %d records\n", deletedCount)

	// Verify the deletes
	verifyDelete, _ := request("GET", "/api/find/batch_users/"+ids[0], nil)
	if verifyDelete == nil {
		fmt.Println("✓ Verified: Records successfully deleted (not found)")
	} else {
		fmt.Println("✗ Warning: Record still exists after delete!")
	}

	fmt.Println("\n✓ All batch operations completed successfully")
}
