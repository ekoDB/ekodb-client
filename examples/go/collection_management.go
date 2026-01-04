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
	fmt.Println("=== Collection Management (Direct HTTP) ===\n")

	_, err := getAuthToken()
	if err != nil {
		fmt.Printf("Auth failed: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("✓ Authentication successful")

	// Example 1: Create a collection (via insert)
	fmt.Println("\n=== Create Collection (via insert) ===")
	insertResult, _ := request("POST", "/api/insert/demo_collection", map[string]interface{}{
		"name":        "Demo Collection",
		"description": "Created for testing",
		"active":      true,
	})
	fmt.Printf("Collection created with first record: %s\n", insertResult["id"])

	// Example 2: List all collections
	fmt.Println("\n=== List Collections ===")
	collectionsData, _ := request("GET", "/api/collections", nil)
	collections := []string{}
	if colls, ok := collectionsData["collections"].([]interface{}); ok {
		for _, c := range colls {
			collections = append(collections, c.(string))
		}
	}
	fmt.Printf("Total collections: %d\n", len(collections))
	if len(collections) > 5 {
		fmt.Printf("Sample collections: %v\n", collections[:5])
	} else {
		fmt.Printf("Sample collections: %v\n", collections)
	}

	// Example 3: Delete collection
	fmt.Println("\n=== Delete Collection ===")
	request("DELETE", "/api/collections/demo_collection", nil)
	fmt.Println("Collection deleted successfully")

	// Example 4: Verify deletion
	fmt.Println("\n=== Verify Deletion ===")
	updatedCollectionsData, _ := request("GET", "/api/collections", nil)
	stillExists := false
	if colls, ok := updatedCollectionsData["collections"].([]interface{}); ok {
		for _, c := range colls {
			if c.(string) == "demo_collection" {
				stillExists = true
				break
			}
		}
	}
	fmt.Printf("Collection still exists: %v\n", stillExists)

	fmt.Println("\n✓ All collection management operations completed successfully")
}
