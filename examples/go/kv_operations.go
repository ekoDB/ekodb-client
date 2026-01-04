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
	fmt.Println("=== Key-Value Operations (Direct HTTP) ===\n")

	_, err := getAuthToken()
	if err != nil {
		fmt.Printf("Auth failed: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("✓ Authentication successful")

	// Example 1: Set a key-value pair
	fmt.Println("\n=== KV Set ===")
	request("POST", "/api/kv/set/session:user123", map[string]interface{}{
		"value": map[string]interface{}{
			"userId":   123,
			"username": "john_doe",
		},
	})
	fmt.Println("✓ Set key: session:user123")

	// Example 2: Get a key-value pair
	fmt.Println("\n=== KV Get ===")
	getValue, _ := request("GET", "/api/kv/get/session:user123", nil)
	if getValue != nil {
		fmt.Printf("Retrieved value: %v\n", getValue["value"])
	}

	// Example 3: Set multiple keys
	fmt.Println("\n=== Set Multiple Keys ===")
	keys := []string{"cache:product:1", "cache:product:2", "cache:product:3"}

	for i, key := range keys {
		request("POST", "/api/kv/set/"+key, map[string]interface{}{
			"value": map[string]interface{}{
				"name":  fmt.Sprintf("Product %d", i+1),
				"price": 29.99 + float64(i)*10.0,
			},
		})
	}
	fmt.Printf("✓ Set %d keys\n", len(keys))

	// Example 4: Get multiple keys
	fmt.Println("\n=== Get Multiple Keys ===")
	for _, key := range keys {
		result, _ := request("GET", "/api/kv/get/"+key, nil)
		if result != nil {
			fmt.Printf("%s: %v\n", key, result["value"])
		}
	}

	// Example 5: Delete a key
	fmt.Println("\n=== KV Delete ===")
	request("DELETE", "/api/kv/delete/session:user123", nil)
	fmt.Println("✓ Deleted key: session:user123")

	// Verify deletion
	verifyDelete, _ := request("GET", "/api/kv/get/session:user123", nil)
	if verifyDelete == nil {
		fmt.Println("✓ Verified: Key successfully deleted (not found)")
	} else {
		fmt.Println("✗ Warning: Key still exists after delete!")
	}

	// Example 6: Delete multiple keys
	fmt.Println("\n=== Delete Multiple Keys ===")
	for _, key := range keys {
		request("DELETE", "/api/kv/delete/"+key, nil)
	}
	fmt.Printf("✓ Deleted %d keys\n", len(keys))

	fmt.Println("\n✓ All KV operations completed successfully")
}
