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

const sessionKey = "session:user123:go"

var productKeys = []string{"cache:product:1:go", "cache:product:2:go", "cache:product:3:go"}

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
	if resp.StatusCode >= 300 {
		return nil, fmt.Errorf("HTTP %d: %s", resp.StatusCode, respBody)
	}
	if len(respBody) == 0 {
		return map[string]interface{}{}, nil
	}

	var result map[string]interface{}
	json.Unmarshal(respBody, &result)
	return result, nil
}

func run() (runErr error) {
	fmt.Print("=== Key-Value Operations (Direct HTTP) ===\n\n")

	_, err := getAuthToken()
	if err != nil {
		return fmt.Errorf("auth failed: %w", err)
	}
	fmt.Println("✓ Authentication successful")
	defer func() {
		for _, key := range append([]string{sessionKey}, productKeys...) {
			if _, err := request("DELETE", "/api/kv/delete/"+key, nil); err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("cleanup KV key %s: %w", key, err))
			}
		}
	}()

	// Example 1: Set a key-value pair
	fmt.Println("\n=== KV Set ===")
	if _, err := request("POST", "/api/kv/set/"+sessionKey, map[string]interface{}{
		"value": map[string]interface{}{
			"userId":   123,
			"username": "john_doe",
		},
	}); err != nil {
		return fmt.Errorf("set KV key %s: %w", sessionKey, err)
	}
	fmt.Printf("✓ Set key: %s\n", sessionKey)

	// Example 2: Get a key-value pair
	fmt.Println("\n=== KV Get ===")
	getValue, err := request("GET", "/api/kv/get/"+sessionKey, nil)
	if err != nil {
		return fmt.Errorf("get KV key %s: %w", sessionKey, err)
	}
	if getValue != nil {
		fmt.Printf("Retrieved value: %v\n", getValue["value"])
	}

	// Example 3: Set multiple keys
	fmt.Println("\n=== Set Multiple Keys ===")
	keys := productKeys

	for i, key := range keys {
		if _, err := request("POST", "/api/kv/set/"+key, map[string]interface{}{
			"value": map[string]interface{}{
				"name":  fmt.Sprintf("Product %d", i+1),
				"price": 29.99 + float64(i)*10.0,
			},
		}); err != nil {
			return fmt.Errorf("set KV key %s: %w", key, err)
		}
	}
	fmt.Printf("✓ Set %d keys\n", len(keys))

	// Example 4: Get multiple keys
	fmt.Println("\n=== Get Multiple Keys ===")
	for _, key := range keys {
		result, err := request("GET", "/api/kv/get/"+key, nil)
		if err != nil {
			return fmt.Errorf("get KV key %s: %w", key, err)
		}
		if result != nil {
			fmt.Printf("%s: %v\n", key, result["value"])
		}
	}

	// Example 5: Delete a key
	fmt.Println("\n=== KV Delete ===")
	if _, err := request("DELETE", "/api/kv/delete/"+sessionKey, nil); err != nil {
		return fmt.Errorf("delete KV key %s: %w", sessionKey, err)
	}
	fmt.Printf("✓ Deleted key: %s\n", sessionKey)

	// Verify deletion
	verifyDelete, err := request("GET", "/api/kv/get/"+sessionKey, nil)
	if err != nil {
		return fmt.Errorf("verify KV deletion %s: %w", sessionKey, err)
	}
	if verifyDelete == nil {
		fmt.Println("✓ Verified: Key successfully deleted (not found)")
	} else {
		fmt.Println("✗ Warning: Key still exists after delete!")
	}

	// Example 6: Delete multiple keys
	fmt.Println("\n=== Delete Multiple Keys ===")
	for _, key := range keys {
		if _, err := request("DELETE", "/api/kv/delete/"+key, nil); err != nil {
			return fmt.Errorf("delete KV key %s: %w", key, err)
		}
	}
	fmt.Printf("✓ Deleted %d keys\n", len(keys))

	fmt.Println("\n✓ All KV operations completed successfully")
	return nil
}

func main() {
	if err := run(); err != nil {
		fmt.Printf("✗ KV operations failed: %v\n", err)
		os.Exit(1)
	}
}
