package main

import (
	"encoding/json"
	"fmt"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
)

// WebSocket Chat Streaming Example - Real-time LLM responses
//
// Demonstrates using the WebSocket client to send a chat message
// and receive streaming responses, including tool calls.
//
// Requires: ekoDB server running on localhost:8080

func getEnv(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}

func main() {
	fmt.Println("=== WebSocket Chat Streaming Example (Go) ===\n")

	baseURL := getEnv("API_BASE_URL", "http://localhost:8080")
	wsURL := getEnv("WS_BASE_URL", "ws://localhost:8080")
	apiKey := getEnv("API_BASE_KEY", "a-test-api-key-from-ekodb")

	// Create HTTP client for session management
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		fmt.Printf("Failed to create client: %v\n", err)
		return
	}

	// Create a chat session
	systemPrompt := "You are a helpful assistant."
	session, err := client.CreateChatSession(ekodb.CreateChatSessionRequest{
		SystemPrompt: &systemPrompt,
	})
	if err != nil {
		fmt.Printf("Failed to create chat session: %v\n", err)
		return
	}
	chatID := session.ChatID
	fmt.Printf("Created chat session: %s\n", chatID)

	// Connect WebSocket
	ws, err := client.WebSocket(wsURL)
	if err != nil {
		fmt.Printf("Failed to connect WebSocket: %v\n", err)
		return
	}
	defer ws.Close()

	// Send a chat message and stream the response
	fmt.Println("\nSending message: 'What is the capital of France?'")
	eventCh, err := ws.ChatSend(chatID, "What is the capital of France?")
	if err != nil {
		fmt.Printf("Failed to send chat: %v\n", err)
		return
	}

	fullResponse := ""
	for event := range eventCh {
		switch event.Type {
		case "chunk":
			fullResponse += event.Content
			fmt.Print(event.Content)

		case "end":
			fmt.Printf("\n\n--- Stream ended ---\n")
			fmt.Printf("Message ID: %s\n", event.MessageID)
			fmt.Printf("Execution time: %dms\n", event.ExecutionTimeMs)
			if event.TokenUsage != nil {
				fmt.Printf("Token usage: %s\n", string(event.TokenUsage))
			}

		case "toolCall":
			argsStr := string(event.Arguments)
			fmt.Printf("\n[Tool Call] %s(%s)\n", event.ToolName, argsStr)
			// Send tool result back
			err := ws.SendToolResult(chatID, event.CallID, true,
				map[string]string{"result": "Tool executed successfully"}, "")
			if err != nil {
				fmt.Printf("Failed to send tool result: %v\n", err)
			}

		case "error":
			fmt.Printf("\n[Error] %s\n", event.Error)
		}
	}

	if len(fullResponse) > 200 {
		fullResponse = fullResponse[:200]
	}
	fmt.Printf("\nFull response: %s...\n", fullResponse)
}

func chatWithClientTools() {
	fmt.Println("\n=== Chat with Client Tools ===\n")

	baseURL := getEnv("API_BASE_URL", "http://localhost:8080")
	wsURL := getEnv("WS_BASE_URL", "ws://localhost:8080")
	apiKey := getEnv("API_BASE_KEY", "a-test-api-key-from-ekodb")

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		fmt.Printf("Failed to create client: %v\n", err)
		return
	}
	ws, err := client.WebSocket(wsURL)
	if err != nil {
		fmt.Printf("Failed to connect: %v\n", err)
		return
	}
	defer ws.Close()

	systemPrompt := "You are a helpful assistant with access to tools."
	session, err := client.CreateChatSession(ekodb.CreateChatSessionRequest{
		SystemPrompt: &systemPrompt,
	})
	if err != nil {
		fmt.Printf("Failed to create session: %v\n", err)
		return
	}
	chatID := session.ChatID

	// Register client-side tools
	err = ws.RegisterClientTools(chatID, []ekodb.ClientToolDefinition{
		{
			Name:        "get_weather",
			Description: "Get the current weather for a location",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"location": map[string]string{"type": "string"},
				},
				"required": []string{"location"},
			},
		},
	})
	if err != nil {
		fmt.Printf("Failed to register tools: %v\n", err)
		return
	}
	fmt.Println("Registered client tools")

	eventCh, err := ws.ChatSend(chatID, "What's the weather in Paris?")
	if err != nil {
		fmt.Printf("Failed to send: %v\n", err)
		return
	}

	for event := range eventCh {
		switch event.Type {
		case "chunk":
			fmt.Print(event.Content)
		case "toolCall":
			fmt.Printf("\n[Tool Call] %s(%s)\n", event.ToolName, string(event.Arguments))
			ws.SendToolResult(chatID, event.CallID, true,
				map[string]string{"temperature": "22°C", "condition": "Sunny"}, "")
		case "end":
			fmt.Printf("\n--- Done (%dms) ---\n", event.ExecutionTimeMs)
		case "error":
			fmt.Printf("\n[Error] %s\n", event.Error)
		}
	}
}

// Ensure json is used (for the Arguments field)
var _ = json.Marshal
