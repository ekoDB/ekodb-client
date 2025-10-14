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

func getAuthToken() (string, error) {
	var baseURL = os.Getenv("API_BASE_URL")
	var apiKey = os.Getenv("API_BASE_KEY")
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
	var baseURL = os.Getenv("API_BASE_URL")
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
	// Load environment variables from the .env file
	err := godotenv.Load()
	if err != nil {
		log.Fatal(fmt.Sprintf("✗ Failed to load environment variables: %v", err))
		os.Exit(1)
	}

	var baseURL = os.Getenv("API_BASE_URL")

	// Get authentication token
	token, err := getAuthToken()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Authentication successful")

	// Example 1: Insert a document
	fmt.Println("\n=== Insert Document ===")
	doc, err := makeRequest("POST", "/api/insert/test_collection", token, map[string]interface{}{
		"name":   "Test Record",
		"value":  42,
		"active": true,
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Inserted: %v\n", doc)
	docID := doc["id"].(string)

	// Example 2: Find by ID
	fmt.Println("\n=== Find by ID ===")
	foundDoc, err := makeRequest("GET", "/api/find/test_collection/"+docID, token, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Found: %v\n", foundDoc)

	// Example 3: Find with query
	fmt.Println("\n=== Find with Query ===")
	queryBody := map[string]interface{}{
		"filter": map[string]interface{}{
			"type": "Condition",
			"content": map[string]interface{}{
				"field":    "active",
				"operator": "Eq",
				"value":    true,
			},
		},
		"limit": 10,
	}
	jsonData, _ := json.Marshal(queryBody)

	req, _ := http.NewRequest("POST", baseURL+"/api/find/test_collection", bytes.NewBuffer(jsonData))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	var docs []interface{}
	json.NewDecoder(resp.Body).Decode(&docs)
	fmt.Printf("Found %d documents\n", len(docs))

	// Example 4: Update document
	fmt.Println("\n=== Update Document ===")
	updated, err := makeRequest("PUT", "/api/update/test_collection/"+docID, token, map[string]interface{}{
		"name":  "Updated Record",
		"value": 100,
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Updated: %v\n", updated)

	// Example 5: Delete document
	fmt.Println("\n=== Delete Document ===")
	_, err = makeRequest("DELETE", "/api/delete/test_collection/"+docID, token, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Deleted document")

	fmt.Println("\n✓ All CRUD operations completed successfully")
}
