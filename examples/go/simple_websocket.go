package main

import (
	"bytes"
	"encoding/json"
	"errors"
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

	req, _ := http.NewRequest("POST", baseURL+"/api/insert/websocket_test_go", bytes.NewBuffer(body))
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

func deleteTestCollection(token string) error {
	req, err := http.NewRequest("DELETE", baseURL+"/api/collections/websocket_test_go", nil)
	if err != nil {
		return err
	}
	req.Header.Set("Authorization", "Bearer "+token)
	resp, err := (&http.Client{}).Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusNotFound && (resp.StatusCode < 200 || resp.StatusCode >= 300) {
		return fmt.Errorf("collection cleanup failed with status %d", resp.StatusCode)
	}
	return nil
}

func run() (runErr error) {
	fmt.Print("=== Simple WebSocket Operations (Direct API) ===\n\n")

	// Step 1: Get authentication token
	token, err := getAuthToken()
	if err != nil {
		return fmt.Errorf("auth failed: %w", err)
	}
	fmt.Println("✓ Authentication successful")
	if err := deleteTestCollection(token); err != nil {
		return fmt.Errorf("initial cleanup failed: %w", err)
	}
	defer func() {
		if err := deleteTestCollection(token); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup failed: %w", err))
		}
	}()

	// Step 2: Insert test data first
	fmt.Println("\n=== Inserting Test Data ===")
	insertResult, err := insertTestData(token)
	if err != nil {
		return fmt.Errorf("insert failed: %w", err)
	}
	fmt.Printf("✓ Inserted test record: %s\n", insertResult["id"])

	// Step 3: Connect to WebSocket
	fmt.Println("\n=== Connecting to WebSocket ===")

	header := http.Header{}
	header.Add("Authorization", "Bearer "+token)

	conn, _, err := websocket.DefaultDialer.Dial(wsURL+"/api/ws", header)
	if err != nil {
		return fmt.Errorf("WebSocket connection failed: %w", err)
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
			"collection": "websocket_test_go",
		},
	}

	err = conn.WriteJSON(message)
	if err != nil {
		return fmt.Errorf("failed to send message: %w", err)
	}

	// Wait for response
	conn.SetReadDeadline(time.Now().Add(10 * time.Second))
	_, responseBytes, err := conn.ReadMessage()
	if err != nil {
		return fmt.Errorf("failed to read response: %w", err)
	}

	var response map[string]interface{}
	if err := json.Unmarshal(responseBytes, &response); err != nil {
		return fmt.Errorf("decode WebSocket response: %w", err)
	}
	prettyResponse, _ := json.MarshalIndent(response, "", "  ")
	fmt.Printf("Response: %s\n", prettyResponse)

	payload, ok := response["payload"].(map[string]interface{})
	if !ok {
		return fmt.Errorf("WebSocket response did not contain payload")
	}
	data, ok := payload["data"].([]interface{})
	if !ok {
		return fmt.Errorf("WebSocket response did not contain payload.data")
	}
	if len(data) != 1 {
		return fmt.Errorf("expected exactly 1 WebSocket record, got %d", len(data))
	}
	fmt.Println("✓ Retrieved 1 record via WebSocket")

	return nil
}

func main() {
	if err := run(); err != nil {
		fmt.Printf("Error: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("\n✓ WebSocket example completed successfully")
}
