package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/gorilla/websocket"
	"github.com/joho/godotenv"
)

var (
	baseURL string
	wsURL   string
	apiKey  string
)

func init() {
	godotenv.Load()
	baseURL = os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	wsURL = os.Getenv("WS_BASE_URL")
	if wsURL == "" {
		wsURL = "ws://localhost:8080"
	}
	apiKey = os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}
}

func getAuthToken() (string, error) {
	body, _ := json.Marshal(map[string]string{"api_key": apiKey})
	resp, err := http.Post(baseURL+"/api/auth/token", "application/json", bytes.NewBuffer(body))
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	var result map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&result)
	return result["token"].(string), nil
}

func insertTestData(token string) (map[string]interface{}, error) {
	body, _ := json.Marshal(map[string]interface{}{
		"name":   "WebSocket Test Record",
		"value":  42,
		"active": true,
	})

	req, _ := http.NewRequest("POST", baseURL+"/api/insert/websocket_test", bytes.NewBuffer(body))
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
	fmt.Println("=== Simple WebSocket Operations (Direct API) ===\n")

	// Step 1: Get authentication token
	token, err := getAuthToken()
	if err != nil {
		fmt.Printf("Auth failed: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("✓ Authentication successful")

	// Step 2: Insert test data first
	fmt.Println("\n=== Inserting Test Data ===")
	insertResult, err := insertTestData(token)
	if err != nil {
		fmt.Printf("Insert failed: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("✓ Inserted test record: %s\n", insertResult["id"])

	// Step 3: Connect to WebSocket
	fmt.Println("\n=== Connecting to WebSocket ===")

	header := http.Header{}
	header.Add("Authorization", "Bearer "+token)

	conn, _, err := websocket.DefaultDialer.Dial(wsURL+"/api/ws", header)
	if err != nil {
		fmt.Printf("WebSocket connection failed: %v\n", err)
		os.Exit(1)
	}
	defer conn.Close()
	fmt.Println("✓ WebSocket connected")

	// Step 4: Query data via WebSocket
	fmt.Println("\n=== Querying Data via WebSocket ===")
	messageID := fmt.Sprintf("%d", time.Now().UnixNano())

	message := map[string]interface{}{
		"type":      "FindAll",
		"messageId": messageID,
		"payload": map[string]interface{}{
			"collection": "websocket_test",
		},
	}

	err = conn.WriteJSON(message)
	if err != nil {
		fmt.Printf("Failed to send message: %v\n", err)
		os.Exit(1)
	}

	// Wait for response
	conn.SetReadDeadline(time.Now().Add(10 * time.Second))
	_, responseBytes, err := conn.ReadMessage()
	if err != nil {
		fmt.Printf("Failed to read response: %v\n", err)
		os.Exit(1)
	}

	var response map[string]interface{}
	json.Unmarshal(responseBytes, &response)
	prettyResponse, _ := json.MarshalIndent(response, "", "  ")
	fmt.Printf("Response: %s\n", prettyResponse)

	if payload, ok := response["payload"].(map[string]interface{}); ok {
		if data, ok := payload["data"].([]interface{}); ok {
			fmt.Printf("✓ Retrieved %d record(s) via WebSocket\n", len(data))
		}
	}

	fmt.Println("\n✓ WebSocket example completed successfully")
}
