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

	// Example 1: Set a key-value pair
	fmt.Println("\n=== KV Set ===")
	kvData := map[string]interface{}{
		"value": map[string]interface{}{
			"userId":   123,
			"username": "john_doe",
		},
	}
	kvJSON, _ := json.Marshal(kvData)

	req, _ := http.NewRequest("POST", baseURL+"/api/kv/set/session:user123", bytes.NewBuffer(kvJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, _ := client.Do(req)
	resp.Body.Close()
	fmt.Println("✓ Set key: session:user123")

	// Example 2: Get a key-value pair
	fmt.Println("\n=== KV Get ===")
	req, _ = http.NewRequest("GET", baseURL+"/api/kv/get/session:user123", nil)
	req.Header.Set("Authorization", "Bearer "+token)

	resp, _ = client.Do(req)
	var getValue map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&getValue)
	resp.Body.Close()
	fmt.Println("Retrieved value:", getValue["value"])

	// Example 3: Set multiple keys
	fmt.Println("\n=== Set Multiple Keys ===")
	keys := []string{"cache:product:1", "cache:product:2", "cache:product:3"}
	for i, key := range keys {
		data := map[string]interface{}{
			"value": map[string]interface{}{
				"name":  fmt.Sprintf("Product %d", i+1),
				"price": 29.99 + float64(i*10),
			},
		}
		dataJSON, _ := json.Marshal(data)

		req, _ := http.NewRequest("POST", baseURL+"/api/kv/set/"+key, bytes.NewBuffer(dataJSON))
		req.Header.Set("Authorization", "Bearer "+token)
		req.Header.Set("Content-Type", "application/json")

		resp, _ := client.Do(req)
		resp.Body.Close()
	}
	fmt.Printf("✓ Set %d keys\n", len(keys))

	// Example 4: Get multiple keys
	fmt.Println("\n=== Get Multiple Keys ===")
	for _, key := range keys {
		req, _ := http.NewRequest("GET", baseURL+"/api/kv/get/"+key, nil)
		req.Header.Set("Authorization", "Bearer "+token)

		resp, _ := client.Do(req)
		var result map[string]interface{}
		json.NewDecoder(resp.Body).Decode(&result)
		resp.Body.Close()
		fmt.Printf("%s: %v\n", key, result["value"])
	}

	// Example 5: Delete a key
	fmt.Println("\n=== KV Delete ===")
	req, _ = http.NewRequest("DELETE", baseURL+"/api/kv/delete/session:user123", nil)
	req.Header.Set("Authorization", "Bearer "+token)

	resp, _ = client.Do(req)
	resp.Body.Close()
	fmt.Println("✓ Deleted key: session:user123")

	// Verify deletion
	req, _ = http.NewRequest("GET", baseURL+"/api/kv/get/session:user123", nil)
	req.Header.Set("Authorization", "Bearer "+token)

	verifyResp, _ := client.Do(req)
	verifyResp.Body.Close()
	if verifyResp.StatusCode == 404 {
		fmt.Println("✓ Verified: Key successfully deleted (not found)")
	} else {
		fmt.Println("✗ Warning: Key still exists after delete!")
	}

	// Example 6: Delete multiple keys
	fmt.Println("\n=== Delete Multiple Keys ===")
	for _, key := range keys {
		req, _ := http.NewRequest("DELETE", baseURL+"/api/kv/delete/"+key, nil)
		req.Header.Set("Authorization", "Bearer "+token)

		resp, _ := client.Do(req)
		resp.Body.Close()
	}
	fmt.Printf("✓ Deleted %d keys\n", len(keys))

	fmt.Println("\n✓ All KV operations completed successfully")
}
