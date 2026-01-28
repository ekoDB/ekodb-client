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

func requestArray(method, path string, body interface{}) ([]map[string]interface{}, error) {
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
		return []map[string]interface{}{}, nil
	}

	var result []map[string]interface{}
	json.Unmarshal(respBody, &result)
	return result, nil
}

func main() {
	fmt.Println("=== Simple CRUD Operations (Direct HTTP) ===\n")

	_, err := getAuthToken()
	if err != nil {
		fmt.Printf("Auth failed: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("✓ Authentication successful")

	// Example 1: Insert a document
	fmt.Println("\n=== Insert Document ===")
	doc, _ := request("POST", "/api/insert/test_collection", map[string]interface{}{
		"name":   "Test Record",
		"value":  42,
		"active": true,
	})
	fmt.Printf("Inserted: %v\n", doc)
	docID := doc["id"].(string)

	// Example 2: Find by ID
	fmt.Println("\n=== Find by ID ===")
	foundDoc, _ := request("GET", "/api/find/test_collection/"+docID, nil)
	fmt.Printf("Found: %v\n", foundDoc)

	// Example 3: Find with query
	fmt.Println("\n=== Find with Query ===")
	docs, _ := requestArray("POST", "/api/find/test_collection", map[string]interface{}{
		"filter": map[string]interface{}{
			"type": "Condition",
			"content": map[string]interface{}{
				"field":    "active",
				"operator": "Eq",
				"value":    true,
			},
		},
		"limit": 10,
	})
	fmt.Printf("Found documents: %v\n", docs)

	// Example 4: Update document
	fmt.Println("\n=== Update Document ===")
	updated, _ := request("PUT", "/api/update/test_collection/"+docID, map[string]interface{}{
		"name":  "Updated Record",
		"value": 100,
	})
	fmt.Printf("Updated: %v\n", updated)

	// Example 5: Delete document
	fmt.Println("\n=== Delete Document ===")
	request("DELETE", "/api/delete/test_collection/"+docID, nil)
	fmt.Println("Deleted document")

	fmt.Println("\n✓ All CRUD operations completed successfully")
}
