package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"

	"github.com/joho/godotenv"
)

func getAuthToken() (string, error) {
	baseURL := os.Getenv("API_BASE_URL")
	apiKey := os.Getenv("API_BASE_KEY")
	authReq := map[string]string{"api_key": apiKey}
	body, _ := json.Marshal(authReq)

	resp, err := http.Post(baseURL+"/api/auth/token", "application/json", bytes.NewBuffer(body))
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	var result map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&result)
	return result["token"].(string), nil
}

func makeRequest(method, path string, token string, data interface{}) (map[string]interface{}, error) {
	baseURL := os.Getenv("API_BASE_URL")
	var body io.Reader
	if data != nil {
		jsonData, _ := json.Marshal(data)
		body = bytes.NewBuffer(jsonData)
	}

	req, err := http.NewRequest(method, baseURL+path, body)
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

	var result map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&result)
	return result, nil
}

func main() {
	// Load environment variables
	err := godotenv.Load()
	if err != nil {
		log.Fatal(fmt.Sprintf("✗ Failed to load environment variables: %v", err))
	}

	// Get authentication token
	token, err := getAuthToken()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Authentication successful\n")

	// Setup test data
	fmt.Println("=== Setup: Creating Test Accounts ===")
	alice, err := makeRequest("POST", "/api/insert/test_accounts", token, map[string]interface{}{
		"account_id": "ACC001",
		"name":       "Alice",
		"balance":    1000,
	})
	if err != nil {
		log.Fatal(err)
	}
	aliceID := alice["id"].(string)
	fmt.Printf("Created Alice: $1000 - ID: %s\n", aliceID)

	bob, err := makeRequest("POST", "/api/insert/test_accounts", token, map[string]interface{}{
		"account_id": "ACC002",
		"name":       "Bob",
		"balance":    500,
	})
	if err != nil {
		log.Fatal(err)
	}
	bobID := bob["id"].(string)
	fmt.Printf("Created Bob: $500 - ID: %s\n\n", bobID)

	// Example 1: Begin Transaction
	fmt.Println("=== Example 1: Begin Transaction ===")
	txData, err := makeRequest("POST", "/api/transactions", token, map[string]interface{}{
		"isolation_level": "ReadCommitted",
	})
	if err != nil {
		log.Fatal(err)
	}
	txID := txData["transaction_id"].(string)
	fmt.Printf("Transaction ID: %s\n\n", txID)

	// Example 2: Operations with transaction_id
	fmt.Println("=== Example 2: Operations with transaction_id ===")
	_, err = makeRequest("PUT", "/api/batch/update/test_accounts?transaction_id="+url.QueryEscape(txID), token, map[string]interface{}{
		"updates": []map[string]interface{}{
			{"id": aliceID, "data": map[string]interface{}{"balance": 800}},
			{"id": bobID, "data": map[string]interface{}{"balance": 700}},
		},
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Updated Alice: $1000 → $800")
	fmt.Println("Updated Bob: $500 → $700\n")

	// Example 3: Transaction Status
	fmt.Println("=== Example 3: Transaction Status ===")
	status, err := makeRequest("GET", "/api/transactions/"+txID, token, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Status: %v\n", status["state"])
	fmt.Printf("Operations: %.0f\n\n", status["operations_count"])

	// Example 4: Commit Transaction
	fmt.Println("=== Example 4: Commit Transaction ===")
	_, err = makeRequest("POST", "/api/transactions/"+txID+"/commit", token, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Transaction committed\n")

	// Verification
	fmt.Println("=== Verification ===")
	aliceFinal, err := makeRequest("GET", "/api/find/test_accounts/"+aliceID, token, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Alice: %v\n", aliceFinal["balance"])

	bobFinal, err := makeRequest("GET", "/api/find/test_accounts/"+bobID, token, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Bob: %v\n\n", bobFinal["balance"])

	// Example 5: Rollback
	fmt.Println("=== Example 5: Rollback ===")
	txData2, err := makeRequest("POST", "/api/transactions", token, map[string]interface{}{
		"isolation_level": "ReadCommitted",
	})
	if err != nil {
		log.Fatal(err)
	}
	txID2 := txData2["transaction_id"].(string)
	fmt.Printf("New transaction: %s\n", txID2)

	_, err = makeRequest("PUT", "/api/batch/update/test_accounts?transaction_id="+url.QueryEscape(txID2), token, map[string]interface{}{
		"updates": []map[string]interface{}{
			{"id": bobID, "data": map[string]interface{}{"balance": 600}},
		},
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Updated Bob: $700 → $600 (in transaction)")

	_, err = makeRequest("POST", "/api/transactions/"+txID2+"/rollback", token, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Transaction rolled back")

	bobAfter, err := makeRequest("GET", "/api/find/test_accounts/"+bobID, token, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Bob after rollback: %v\n\n", bobAfter["balance"])

	// Cleanup
	fmt.Println("=== Cleanup ===")
	_, _ = makeRequest("DELETE", "/api/delete/test_accounts/"+aliceID, token, nil)
	_, _ = makeRequest("DELETE", "/api/delete/test_accounts/"+bobID, token, nil)
	fmt.Println("✓ Deleted test accounts\n")

	fmt.Println("✓ All transaction examples completed")
}
