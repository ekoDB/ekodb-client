package main

import (
	"bytes"
	"encoding/json"
	"errors"
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
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return nil, fmt.Errorf("HTTP %d: %s", resp.StatusCode, string(respBody))
	}
	if len(respBody) == 0 {
		return map[string]interface{}{}, nil
	}

	var result map[string]interface{}
	if err := json.Unmarshal(respBody, &result); err != nil {
		return nil, fmt.Errorf("decode response: %w", err)
	}
	return result, nil
}

func run() (runErr error) {
	fmt.Print("=== Batch Operations (Direct HTTP) ===\n\n")

	_, err := getAuthToken()
	if err != nil {
		return fmt.Errorf("auth failed: %w", err)
	}
	fmt.Println("✓ Authentication successful")
	if _, err := request("DELETE", "/api/collections/batch_users_go", nil); err != nil {
		return fmt.Errorf("initial cleanup failed: %w", err)
	}
	defer func() {
		if _, err := request("DELETE", "/api/collections/batch_users_go", nil); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup failed: %w", err))
		}
	}()

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

	insertResult, err := request("POST", "/api/batch/insert/batch_users_go", batchInsertData)
	if err != nil {
		return fmt.Errorf("batch insert: %w", err)
	}
	insertedCount := 0
	if successful, ok := insertResult["successful"].([]interface{}); ok {
		insertedCount = len(successful)
	}
	fmt.Printf("✓ Batch inserted %d records\n", insertedCount)
	if insertedCount != len(records) {
		return fmt.Errorf("expected %d successful inserts, got %d", len(records), insertedCount)
	}

	// Example 2: Create test records for update/delete
	fmt.Println("\n=== Creating test records for update/delete ===")
	var ids []string
	for i := 0; i < 3; i++ {
		doc, err := request("POST", "/api/insert/batch_users_go", map[string]interface{}{
			"name":  fmt.Sprintf("Test User %d", i),
			"value": i,
		})
		if err != nil {
			return fmt.Errorf("create test record %d: %w", i, err)
		}
		id, ok := doc["id"].(string)
		if !ok || id == "" {
			return fmt.Errorf("create test record %d returned no id", i)
		}
		ids = append(ids, id)
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

	updateResult, err := request("PUT", "/api/batch/update/batch_users_go", batchUpdateData)
	if err != nil {
		return fmt.Errorf("batch update: %w", err)
	}
	updatedCount := 0
	if successful, ok := updateResult["successful"].([]interface{}); ok {
		updatedCount = len(successful)
	}
	fmt.Printf("✓ Batch updated %d records\n", updatedCount)
	if updatedCount != len(ids) {
		return fmt.Errorf("expected %d successful updates, got %d", len(ids), updatedCount)
	}

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

	deleteResult, err := request("DELETE", "/api/batch/delete/batch_users_go", batchDeleteData)
	if err != nil {
		return fmt.Errorf("batch delete: %w", err)
	}
	deletedCount := 0
	if successful, ok := deleteResult["successful"].([]interface{}); ok {
		deletedCount = len(successful)
	}
	fmt.Printf("✓ Batch deleted %d records\n", deletedCount)
	if deletedCount != len(ids) {
		return fmt.Errorf("expected %d successful deletes, got %d", len(ids), deletedCount)
	}

	// Verify the deletes
	verifyDelete, err := request("GET", "/api/find/batch_users_go/"+ids[0], nil)
	if err != nil {
		return fmt.Errorf("verify delete: %w", err)
	}
	if verifyDelete == nil {
		fmt.Println("✓ Verified: Records successfully deleted (not found)")
	} else {
		return fmt.Errorf("record still exists after delete")
	}

	return nil
}

func main() {
	if err := run(); err != nil {
		fmt.Printf("Error: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("\n✓ All batch operations completed successfully")
}
