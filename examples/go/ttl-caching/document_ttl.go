package main

import (
	"bytes"
	"encoding/json"
	"fmt"
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
		log.Fatal("Authentication failed:", err)
	}
	fmt.Println("✓ Authentication successful")

	client := &http.Client{}

	// Example 1: Insert document with TTL (1 hour)
	fmt.Println("\n=== Insert Document with TTL (1 hour) ===")
	doc1Data := map[string]interface{}{
		"key":   "session_token",
		"value": "abc123",
		"ttl":   3600,
	}
	doc1JSON, _ := json.Marshal(doc1Data)

	req, _ := http.NewRequest("POST", baseURL+"/api/insert/ttl_cache", bytes.NewBuffer(doc1JSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err := client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	var doc1Result map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&doc1Result)
	fmt.Printf("✓ Inserted document: %v\n", doc1Result["id"])

	// Example 2: Insert with shorter TTL (integer seconds)
	fmt.Println("\n=== Insert Document with TTL (5 minutes - integer) ===")
	doc2Data := map[string]interface{}{
		"key":   "temp_data",
		"value": map[string]bool{"important": true},
		"ttl":   300,
	}
	doc2JSON, _ := json.Marshal(doc2Data)

	req, _ = http.NewRequest("POST", baseURL+"/api/insert/ttl_cache", bytes.NewBuffer(doc2JSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err = client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	var doc2Result map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&doc2Result)
	fmt.Printf("✓ Inserted document: %v\n", doc2Result["id"])

	// Example 3: Insert with duration string format
	fmt.Println("\n=== Insert Document with TTL (30 minutes - duration string) ===")
	doc3Data := map[string]interface{}{
		"key":   "duration_test",
		"value": "testing duration strings",
		"ttl":   "30m",
	}
	doc3JSON, _ := json.Marshal(doc3Data)

	req, _ = http.NewRequest("POST", baseURL+"/api/insert/ttl_cache", bytes.NewBuffer(doc3JSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err = client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	var doc3Result map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&doc3Result)
	fmt.Printf("✓ Inserted document with duration string TTL: %v\n", doc3Result["id"])

	// Example 4: Query documents
	fmt.Println("\n=== Query Documents ===")
	queryData := map[string]interface{}{"limit": 10}
	queryJSON, _ := json.Marshal(queryData)

	req, _ = http.NewRequest("POST", baseURL+"/api/find/ttl_cache", bytes.NewBuffer(queryJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err = client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	var docs []interface{}
	json.NewDecoder(resp.Body).Decode(&docs)
	fmt.Printf("✓ Found %d documents with TTL\n", len(docs))

	// Example 5: Update document
	fmt.Println("\n=== Update Document ===")
	updateData := map[string]interface{}{"value": "updated_value"}
	updateJSON, _ := json.Marshal(updateData)

	req, _ = http.NewRequest("PUT", fmt.Sprintf("%s/api/update/ttl_cache/%v", baseURL, doc1Result["id"]), bytes.NewBuffer(updateJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err = client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()
	fmt.Println("✓ Updated document")

	// Example 6: Delete document
	fmt.Println("\n=== Delete Document ===")
	if doc1Result["id"] != nil {
		req, _ = http.NewRequest("DELETE", fmt.Sprintf("%s/api/delete/ttl_cache/%v", baseURL, doc1Result["id"]), nil)
		req.Header.Set("Authorization", "Bearer "+token)

		resp, err = client.Do(req)
		if err != nil {
			log.Fatal(err)
		}
		defer resp.Body.Close()
		fmt.Println("✓ Deleted document")
	} else {
		fmt.Println("✓ Skipped delete (no ID returned)")
	}

	fmt.Println("\n✓ All document TTL operations completed successfully")
}
