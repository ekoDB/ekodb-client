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

const managedCollection = "collection_management_go"

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
	fmt.Print("=== Collection Management (Direct HTTP) ===\n\n")

	_, err := getAuthToken()
	if err != nil {
		return fmt.Errorf("auth failed: %w", err)
	}
	fmt.Println("✓ Authentication successful")
	defer func() {
		if _, err := request("DELETE", "/api/collections/"+managedCollection, nil); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup collection %s: %w", managedCollection, err))
		}
	}()

	// Example 1: Create a collection (via insert)
	fmt.Println("\n=== Create Collection (via insert) ===")
	insertResult, err := request("POST", "/api/insert/"+managedCollection, map[string]interface{}{
		"name":        "Demo Collection",
		"description": "Created for testing",
		"active":      true,
	})
	if err != nil {
		return fmt.Errorf("create collection: %w", err)
	}
	fmt.Printf("Collection created with first record: %s\n", insertResult["id"])

	// Example 2: List all collections
	fmt.Println("\n=== List Collections ===")
	collectionsData, err := request("GET", "/api/collections", nil)
	if err != nil {
		return fmt.Errorf("list collections: %w", err)
	}
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
	if _, err := request("DELETE", "/api/collections/"+managedCollection, nil); err != nil {
		return fmt.Errorf("delete collection: %w", err)
	}
	fmt.Println("Collection deleted successfully")

	// Example 4: Verify deletion
	fmt.Println("\n=== Verify Deletion ===")
	updatedCollectionsData, err := request("GET", "/api/collections", nil)
	if err != nil {
		return fmt.Errorf("verify collection deletion: %w", err)
	}
	stillExists := false
	if colls, ok := updatedCollectionsData["collections"].([]interface{}); ok {
		for _, c := range colls {
			if c.(string) == managedCollection {
				stillExists = true
				break
			}
		}
	}
	fmt.Printf("Collection still exists: %v\n", stillExists)
	if stillExists {
		return fmt.Errorf("collection %s still exists after deletion", managedCollection)
	}

	fmt.Println("\n✓ All collection management operations completed successfully")
	return nil
}

func main() {
	if err := run(); err != nil {
		fmt.Printf("✗ Collection management failed: %v\n", err)
		os.Exit(1)
	}
}
