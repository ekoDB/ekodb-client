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

func main() {
	godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	wsURL := os.Getenv("WS_BASE_URL")
	if wsURL == "" {
		wsURL = "ws://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}

	fmt.Println("=== WebSocket Subscription Example (Go) ===")
	fmt.Println()

	collection := "ws_subscribe_example_go"

	// Step 1: Authenticate
	authBody, _ := json.Marshal(map[string]string{"api_key": apiKey})
	authResp, err := http.Post(baseURL+"/api/auth/token", "application/json", bytes.NewBuffer(authBody))
	if err != nil {
		fmt.Printf("Auth failed: %v\n", err)
		os.Exit(1)
	}
	defer authResp.Body.Close()

	var authResult map[string]interface{}
	json.NewDecoder(authResp.Body).Decode(&authResult)
	token := authResult["token"].(string)
	fmt.Println("✓ Authentication successful")

	// Step 2: Connect to WebSocket
	header := http.Header{}
	header.Set("Authorization", "Bearer "+token)

	conn, _, err := websocket.DefaultDialer.Dial(wsURL+"/api/ws", header)
	if err != nil {
		fmt.Printf("WebSocket connection failed: %v\n", err)
		os.Exit(1)
	}
	defer conn.Close()
	fmt.Println("✓ WebSocket connected")

	// Step 3: Subscribe to collection
	fmt.Printf("\n=== Subscribing to '%s' ===\n", collection)

	subscribeMsg := map[string]interface{}{
		"type": "Subscribe",
		"payload": map[string]interface{}{
			"collection": collection,
		},
	}
	if err := conn.WriteJSON(subscribeMsg); err != nil {
		fmt.Printf("WebSocket subscribe request failed: %v\n", err)
		os.Exit(1)
	}

	var subResponse map[string]interface{}
	if err := conn.ReadJSON(&subResponse); err != nil {
		fmt.Printf("WebSocket subscribe response read failed: %v\n", err)
		os.Exit(1)
	}
	payload := subResponse["payload"].(map[string]interface{})
	data := payload["data"].(map[string]interface{})
	fmt.Printf("✓ Subscribed (subscription_id: %s)\n", data["subscription_id"])

	// Step 4: Start a goroutine to listen for notifications
	notifications := make(chan map[string]interface{}, 10)
	go func() {
		for {
			var msg map[string]interface{}
			err := conn.ReadJSON(&msg)
			if err != nil {
				return
			}
			if msg["type"] == "MutationNotification" {
				notifications <- msg
			}
		}
	}()

	// Helper to insert a record via REST API
	doInsert := func(record map[string]interface{}) (map[string]interface{}, error) {
		body, _ := json.Marshal(record)
		req, _ := http.NewRequest("POST", baseURL+"/api/insert/"+collection, bytes.NewBuffer(body))
		req.Header.Set("Authorization", "Bearer "+token)
		req.Header.Set("Content-Type", "application/json")

		resp, err := (&http.Client{}).Do(req)
		if err != nil {
			return nil, err
		}
		defer resp.Body.Close()

		var result map[string]interface{}
		json.NewDecoder(resp.Body).Decode(&result)
		return result, nil
	}

	// Step 5: Insert records via REST API to trigger notifications
	fmt.Println("\n=== Performing mutations to trigger notifications ===")

	fmt.Println("Inserting record 1...")
	result1, _ := doInsert(map[string]interface{}{
		"name":   "Alice",
		"role":   "engineer",
		"active": true,
	})
	fmt.Printf("✓ Inserted: %s\n", result1["id"])

	// Wait for notification
	select {
	case notification := <-notifications:
		p := notification["payload"].(map[string]interface{})
		fmt.Println("\n  📡 Notification received:")
		fmt.Printf("     Event:      %s\n", p["event"])
		fmt.Printf("     Collection: %s\n", p["collection"])
		ids := p["record_ids"].([]interface{})
		idStrs := make([]string, len(ids))
		for i, id := range ids {
			idStrs[i] = id.(string)
		}
		fmt.Printf("     Record IDs: %v\n", idStrs)
		fmt.Printf("     Timestamp:  %s\n", p["timestamp"])
	case <-time.After(5 * time.Second):
		fmt.Println("  ⏳ No notification within timeout")
	}

	fmt.Println("\nInserting record 2...")
	result2, _ := doInsert(map[string]interface{}{
		"name":   "Bob",
		"role":   "designer",
		"active": true,
	})
	fmt.Printf("✓ Inserted: %s\n", result2["id"])

	select {
	case notification := <-notifications:
		p := notification["payload"].(map[string]interface{})
		fmt.Println("\n  📡 Notification received:")
		fmt.Printf("     Event:      %s\n", p["event"])
		ids := p["record_ids"].([]interface{})
		idStrs := make([]string, len(ids))
		for i, id := range ids {
			idStrs[i] = id.(string)
		}
		fmt.Printf("     Record IDs: %v\n", idStrs)
	case <-time.After(5 * time.Second):
		fmt.Println("  ⏳ No notification within timeout")
	}

	// Step 6: Unsubscribe
	fmt.Println("\n=== Unsubscribing ===")
	unsubMsg := map[string]interface{}{
		"type": "Unsubscribe",
		"payload": map[string]interface{}{
			"collection": collection,
		},
	}
	if err := conn.WriteJSON(unsubMsg); err != nil {
		fmt.Printf("WebSocket unsubscribe failed: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("✓ Unsubscribed")

	fmt.Println("\n✓ WebSocket subscription example completed successfully")
}
