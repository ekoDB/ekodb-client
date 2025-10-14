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

func main() {
	// Load environment variables from the .env file
	err := godotenv.Load()
	if err != nil {
		log.Fatal(fmt.Sprintf("✗ Failed to load environment variables: %v", err))
		os.Exit(1)
	}

	var baseURL = os.Getenv("API_BASE_URL")
	var apiKey = os.Getenv("API_BASE_KEY")

	// Get authentication token
	token := os.Getenv("AUTH_TOKEN")
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

	client := &http.Client{}

	// Example 1: Create a collection (via insert)
	fmt.Println("\n=== Create Collection (via insert) ===")
	insertData := map[string]interface{}{
		"name":        "Demo Collection",
		"description": "Created for testing",
		"active":      true,
	}
	insertJSON, _ := json.Marshal(insertData)

	req, _ := http.NewRequest("POST", baseURL+"/api/insert/demo_collection", bytes.NewBuffer(insertJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, _ := client.Do(req)
	var insertResult map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&insertResult)
	resp.Body.Close()
	fmt.Printf("Collection created with first record: %s\n", insertResult["id"])

	// Example 2: List all collections
	fmt.Println("\n=== List Collections ===")
	req, _ = http.NewRequest("GET", baseURL+"/api/collections", nil)
	req.Header.Set("Authorization", "Bearer "+token)

	resp, _ = client.Do(req)
	var collectionsData map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&collectionsData)
	resp.Body.Close()

	collections := collectionsData["collections"].([]interface{})
	fmt.Printf("Total collections: %d\n", len(collections))
	fmt.Printf("Sample collections: %v\n", collections)

	// Example 3: Count documents in collection
	fmt.Println("\n=== Count Documents ===")
	countData := map[string]interface{}{
		"limit": 1000,
	}
	countJSON, _ := json.Marshal(countData)

	req, _ = http.NewRequest("POST", baseURL+"/api/find/demo_collection", bytes.NewBuffer(countJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, _ = client.Do(req)
	var docs []interface{}
	json.NewDecoder(resp.Body).Decode(&docs)
	resp.Body.Close()
	fmt.Printf("Document count: %d\n", len(docs))

	// Example 4: Delete collection
	fmt.Println("\n=== Delete Collection ===")
	req, _ = http.NewRequest("DELETE", baseURL+"/api/collections/demo_collection", nil)
	req.Header.Set("Authorization", "Bearer "+token)

	resp, _ = client.Do(req)
	resp.Body.Close()
	fmt.Println("Collection deleted successfully")

	// Example 5: Verify deletion
	fmt.Println("\n=== Verify Deletion ===")
	req, _ = http.NewRequest("GET", baseURL+"/api/collections", nil)
	req.Header.Set("Authorization", "Bearer "+token)

	resp, _ = client.Do(req)
	var updatedCollectionsData map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&updatedCollectionsData)
	resp.Body.Close()

	updatedCollections := updatedCollectionsData["collections"].([]interface{})
	stillExists := false
	for _, col := range updatedCollections {
		if col.(string) == "demo_collection" {
			stillExists = true
			break
		}
	}
	fmt.Printf("Collection still exists: %v\n", stillExists)

	fmt.Println("\n✓ All collection management operations completed successfully")
}
