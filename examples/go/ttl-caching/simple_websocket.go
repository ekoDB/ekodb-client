package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"net/url"
	"os"
	"time"

	"github.com/gorilla/websocket"
	"github.com/joho/godotenv"
)

func getEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

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
	var wsURL = os.Getenv("WS_BASE_URL")

	// Step 1: Get authentication token
	token, err := getAuthToken()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Authentication successful")

	// Step 2: Insert some test data first
	fmt.Println("\n=== Inserting Test Data ===")
	insertData := map[string]interface{}{
		"name":   "WebSocket Test Record",
		"value":  42,
		"active": true,
	}
	insertJSON, _ := json.Marshal(insertData)

	req, _ := http.NewRequest("POST", baseURL+"/api/insert/websocket_test", bytes.NewBuffer(insertJSON))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	insertResp, err := client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer insertResp.Body.Close()

	var insertResult map[string]interface{}
	json.NewDecoder(insertResp.Body).Decode(&insertResult)
	fmt.Printf("✓ Inserted test record: %s\n", insertResult["id"])

	// Step 3: Connect to WebSocket with Authorization header
	fmt.Println("\n=== Connecting to WebSocket ===")
	u, err := url.Parse(wsURL + "/api/ws")
	if err != nil {
		log.Fatal(err)
	}

	header := http.Header{}
	header.Set("Authorization", "Bearer "+token)

	c, _, err := websocket.DefaultDialer.Dial(u.String(), header)
	if err != nil {
		log.Fatal(err)
	}
	defer c.Close()
	fmt.Println("✓ WebSocket connected")

	// Step 4: Query data via WebSocket
	fmt.Println("\n=== Querying Data via WebSocket ===")
	messageID := fmt.Sprintf("%d", time.Now().UnixNano())

	request := map[string]interface{}{
		"type":      "FindAll",
		"messageId": messageID,
		"payload": map[string]interface{}{
			"collection": "websocket_test",
		},
	}

	err = c.WriteJSON(request)
	if err != nil {
		log.Fatal(err)
	}

	// Wait for response
	var response map[string]interface{}
	err = c.ReadJSON(&response)
	if err != nil {
		log.Fatal(err)
	}

	responseJSON, _ := json.MarshalIndent(response, "", "  ")
	fmt.Printf("Response: %s\n", responseJSON)

	// Show the data we got back
	if payload, ok := response["payload"].(map[string]interface{}); ok {
		if data, ok := payload["data"].([]interface{}); ok {
			fmt.Printf("✓ Retrieved %d record(s) via WebSocket\n", len(data))
		}
	}

	fmt.Println("\n✓ WebSocket example completed successfully")
}
