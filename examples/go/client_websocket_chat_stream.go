package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"strings"

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

func runBasic() (runErr error) {
	fmt.Print("=== WebSocket Chat Streaming Example (Go) ===\n\n")

	baseURL := getEnv("API_BASE_URL", "http://localhost:8080")
	wsURL := getEnv("WS_BASE_URL", "ws://localhost:8080")
	apiKey := getEnv("API_BASE_KEY", "a-test-api-key-from-ekodb")

	// Create HTTP client for session management
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}

	// Create a chat session
	systemPrompt := "You are a helpful assistant."
	llmModel := "gpt-4o-mini"
	session, err := client.CreateChatSession(ekodb.CreateChatSessionRequest{
		Collections:  []ekodb.CollectionConfig{},
		LLMProvider:  "openai",
		LLMModel:     &llmModel,
		SystemPrompt: &systemPrompt,
	})
	if err != nil {
		return fmt.Errorf("create chat session: %w", err)
	}
	chatID := session.ChatID
	defer func() {
		if err := client.DeleteChatSession(chatID); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("delete chat session: %w", err))
		}
	}()
	fmt.Printf("Created chat session: %s\n", chatID)

	// Connect WebSocket
	ws, err := client.WebSocket(wsURL)
	if err != nil {
		return fmt.Errorf("connect WebSocket: %w", err)
	}
	defer ws.Close()

	// Send a chat message and stream the response
	fmt.Println("\nSending message: 'What is the capital of France?'")
	eventCh, err := ws.ChatSend(chatID, "What is the capital of France?")
	if err != nil {
		return fmt.Errorf("send chat: %w", err)
	}

	fullResponse := ""
	completed := false
	messageID := ""
	for event := range eventCh {
		switch event.Type {
		case "chunk":
			fullResponse += event.Content
			fmt.Print(event.Content)

		case "end":
			completed = true
			messageID = event.MessageID
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
				return fmt.Errorf("send tool result: %w", err)
			}

		case "error":
			return fmt.Errorf("WebSocket chat failed: %s", event.Error)
		}
	}
	if !completed {
		return fmt.Errorf("WebSocket chat ended before an end event")
	}
	if strings.TrimSpace(fullResponse) == "" || strings.TrimSpace(messageID) == "" {
		return fmt.Errorf("WebSocket chat completed without content or message ID: bytes=%d message_id=%q", len(fullResponse), messageID)
	}

	if len(fullResponse) > 200 {
		fullResponse = fullResponse[:200]
	}
	fmt.Printf("\nFull response: %s...\n", fullResponse)
	return nil
}

func main() {
	if err := runBasic(); err != nil {
		log.Fatal(err)
	}
}

func chatWithClientTools() (runErr error) {
	fmt.Print("\n=== Chat with Client Tools ===\n\n")

	baseURL := getEnv("API_BASE_URL", "http://localhost:8080")
	wsURL := getEnv("WS_BASE_URL", "ws://localhost:8080")
	apiKey := getEnv("API_BASE_KEY", "a-test-api-key-from-ekodb")

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}
	ws, err := client.WebSocket(wsURL)
	if err != nil {
		return fmt.Errorf("connect WebSocket: %w", err)
	}
	defer ws.Close()

	systemPrompt := "You are a helpful assistant with access to tools."
	llmModel := "gpt-4o-mini"
	session, err := client.CreateChatSession(ekodb.CreateChatSessionRequest{
		Collections:  []ekodb.CollectionConfig{},
		LLMProvider:  "openai",
		LLMModel:     &llmModel,
		SystemPrompt: &systemPrompt,
	})
	if err != nil {
		return fmt.Errorf("create session: %w", err)
	}
	chatID := session.ChatID
	defer func() {
		if err := client.DeleteChatSession(chatID); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("delete chat session: %w", err))
		}
	}()

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
		return fmt.Errorf("register tools: %w", err)
	}
	fmt.Println("Registered client tools")

	eventCh, err := ws.ChatSend(chatID, "What's the weather in Paris?")
	if err != nil {
		return fmt.Errorf("send chat: %w", err)
	}

	completed := false
	for event := range eventCh {
		switch event.Type {
		case "chunk":
			fmt.Print(event.Content)
		case "toolCall":
			fmt.Printf("\n[Tool Call] %s(%s)\n", event.ToolName, string(event.Arguments))
			if err := ws.SendToolResult(chatID, event.CallID, true,
				map[string]string{"temperature": "22°C", "condition": "Sunny"}, ""); err != nil {
				return fmt.Errorf("send tool result: %w", err)
			}
		case "end":
			completed = true
			fmt.Printf("\n--- Done (%dms) ---\n", event.ExecutionTimeMs)
		case "error":
			return fmt.Errorf("WebSocket chat failed: %s", event.Error)
		}
	}
	if !completed {
		return fmt.Errorf("WebSocket chat ended before an end event")
	}
	return nil
}
