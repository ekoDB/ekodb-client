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

func getAuthToken() (string, error) {
	// Load environment variables from the .env file
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

	// Get authentication token
	token, err := getAuthToken()
	if err != nil {
		log.Fatal("Authentication failed:", err)
	}
	fmt.Println("✓ Authentication successful")

	// Insert test data with TTL
	fmt.Println("\n=== Insert Test Data with TTL ===")
	insertData := map[string]interface{}{
		"name":         "WebSocket TTL Test",
		"value":        42,
		"ttl_duration": "1h",
	}
	insertBody, _ := json.Marshal(insertData)

	req, _ := http.NewRequest("POST", baseURL+"/api/insert/ws_ttl_test", bytes.NewBuffer(insertBody))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		log.Fatal("Insert failed:", err)
	}
	defer resp.Body.Close()

	var insertResult map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&insertResult)
	fmt.Printf("✓ Inserted document with TTL: %v\n", insertResult["id"])

	// Connect to WebSocket
	fmt.Println("\n=== Query via WebSocket ===")
	u, _ := url.Parse(wsURL + "/api/ws")
	headers := http.Header{}
	headers.Add("Authorization", "Bearer "+token)

	conn, _, err := websocket.DefaultDialer.Dial(u.String(), headers)
	if err != nil {
		log.Fatal("WebSocket connection failed:", err)
	}
	defer conn.Close()
	fmt.Println("✓ WebSocket connected")

	// Send query
	messageID := fmt.Sprintf("%d", time.Now().UnixNano())
	query := map[string]interface{}{
		"type":      "FindAll",
		"messageId": messageID,
		"payload": map[string]interface{}{
			"collection": "ws_ttl_test",
		},
	}

	if err := conn.WriteJSON(query); err != nil {
		log.Fatal("Failed to send query:", err)
	}

	// Read response
	var response map[string]interface{}
	if err := conn.ReadJSON(&response); err != nil {
		log.Fatal("Failed to read response:", err)
	}

	if payload, ok := response["payload"].(map[string]interface{}); ok {
		if data, ok := payload["data"].([]interface{}); ok {
			fmt.Printf("✓ Retrieved %d record(s) via WebSocket\n", len(data))
		}
	}

	fmt.Println("\n✓ WebSocket TTL example completed successfully")
}
