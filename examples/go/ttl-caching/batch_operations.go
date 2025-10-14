package main

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

func main() {
	// Load environment variables from the .env file
	err := godotenv.Load()
	if err != nil {
		log.Fatal(fmt.Sprintf("✗ Failed to load environment variables: %v", err))
		os.Exit(1)
	}

	// Get authentication token
	token := os.Getenv("AUTH_TOKEN")
	var baseURL = os.Getenv("API_BASE_URL")
	var apiKey = os.Getenv("API_BASE_KEY")
	if token == "" {
		tokenResp, err := http.Post(baseURL+"/api/auth/token", "application/json",
			bytes.NewBuffer([]byte(fmt.Sprintf(`{"api_key":"%s"}`, apiKey))))
		if err != nil {
			log.Fatal(err)
		}
		defer tokenResp.Body.Close()

		var tokenData map[string]interface{}
		json.NewDecoder(tokenResp.Body).Decode(&tokenData)
		token = tokenData["token"].(string)
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
	for i, record := range records {
		inserts[i] = map[string]interface{}{"data": record}
	}

	batchInsertData := map[string]interface{}{
		"inserts":       inserts,
		"bypass_ripple": false,
	}

	insertJSON, _ := json.Marshal(batchInsertData)
	req, _ := http.NewRequest("POST", baseURL+"/api/batch/insert/batch_users", bytes.NewBuffer(insertJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	var insertResult map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&insertResult)
	fmt.Println("Batch insert response:", insertResult)
	fmt.Println("✓ Batch insert completed")

	// Example 2: Create test records for update/delete
	fmt.Println("\n=== Creating test records for update/delete ===")
	ids := []string{}
	for i := 0; i < 3; i++ {
		data := map[string]interface{}{"name": fmt.Sprintf("Test User %d", i), "value": i}
		dataJSON, _ := json.Marshal(data)

		req, _ := http.NewRequest("POST", baseURL+"/api/insert/batch_users", bytes.NewBuffer(dataJSON))
		req.Header.Set("Authorization", "Bearer "+token)
		req.Header.Set("Content-Type", "application/json")

		resp, _ := client.Do(req)
		var doc map[string]interface{}
		json.NewDecoder(resp.Body).Decode(&doc)
		ids = append(ids, doc["id"].(string))
		resp.Body.Close()
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

	updateJSON, _ := json.Marshal(batchUpdateData)
	req, _ = http.NewRequest("PUT", baseURL+"/api/batch/update/batch_users", bytes.NewBuffer(updateJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, _ = client.Do(req)
	var updateResult map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&updateResult)
	resp.Body.Close()
	fmt.Println("Batch update response:", updateResult)
	fmt.Println("✓ Batch update completed")

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

	deleteJSON, _ := json.Marshal(batchDeleteData)
	req, _ = http.NewRequest("DELETE", baseURL+"/api/batch/delete/batch_users", bytes.NewBuffer(deleteJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, _ = client.Do(req)
	var deleteResult map[string]interface{}
	body, _ := io.ReadAll(resp.Body)
	json.Unmarshal(body, &deleteResult)
	resp.Body.Close()
	fmt.Println("Batch delete response:", deleteResult)
	fmt.Println("✓ Batch delete completed")

	fmt.Println("\n✓ All batch operations completed successfully")
}
