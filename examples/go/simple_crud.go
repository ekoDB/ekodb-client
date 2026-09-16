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

const simpleCRUDCollection = "simple_crud_go"

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
	if resp.StatusCode >= 300 {
		return nil, fmt.Errorf("HTTP %d: %s", resp.StatusCode, respBody)
	}
	if len(respBody) == 0 {
		return []map[string]interface{}{}, nil
	}

	var result []map[string]interface{}
	json.Unmarshal(respBody, &result)
	return result, nil
}

func run() (runErr error) {
	fmt.Print("=== Simple CRUD Operations (Direct HTTP) ===\n\n")

	_, err := getAuthToken()
	if err != nil {
		return fmt.Errorf("auth failed: %w", err)
	}
	fmt.Println("✓ Authentication successful")
	defer func() {
		if _, err := request("DELETE", "/api/collections/"+simpleCRUDCollection, nil); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup collection %s: %w", simpleCRUDCollection, err))
		}
	}()

	// Example 1: Insert a document
	fmt.Println("\n=== Insert Document ===")
	doc, err := request("POST", "/api/insert/"+simpleCRUDCollection, map[string]interface{}{
		"name":   "Test Record",
		"value":  42,
		"active": true,
	})
	if err != nil {
		return fmt.Errorf("insert document: %w", err)
	}
	fmt.Printf("Inserted: %v\n", doc)
	docID := doc["id"].(string)

	// Example 2: Find by ID
	fmt.Println("\n=== Find by ID ===")
	foundDoc, err := request("GET", "/api/find/"+simpleCRUDCollection+"/"+docID, nil)
	if err != nil {
		return fmt.Errorf("find document: %w", err)
	}
	fmt.Printf("Found: %v\n", foundDoc)

	// Example 3: Find with query
	fmt.Println("\n=== Find with Query ===")
	docs, err := requestArray("POST", "/api/find/"+simpleCRUDCollection, map[string]interface{}{
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
	if err != nil {
		return fmt.Errorf("query documents: %w", err)
	}
	fmt.Printf("Found documents: %v\n", docs)

	// Example 4: Update document
	fmt.Println("\n=== Update Document ===")
	updated, err := request("PUT", "/api/update/"+simpleCRUDCollection+"/"+docID, map[string]interface{}{
		"name":  "Updated Record",
		"value": 100,
	})
	if err != nil {
		return fmt.Errorf("update document: %w", err)
	}
	fmt.Printf("Updated: %v\n", updated)

	// Example 5: Delete document
	fmt.Println("\n=== Delete Document ===")
	if _, err := request("DELETE", "/api/delete/"+simpleCRUDCollection+"/"+docID, nil); err != nil {
		return fmt.Errorf("delete document: %w", err)
	}
	fmt.Println("Deleted document")

	fmt.Println("\n✓ All CRUD operations completed successfully")
	return nil
}

func main() {
	if err := run(); err != nil {
		fmt.Printf("✗ Simple CRUD failed: %v\n", err)
		os.Exit(1)
	}
}
