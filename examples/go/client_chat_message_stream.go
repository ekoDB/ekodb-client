// ekoDB Go Client - Chat Message Stream (SSE) Example
//
// Demonstrates streaming chat responses via HTTP SSE (Server-Sent Events).
// This is a simpler alternative to WebSocket streaming that doesn't require
// a persistent connection.

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Chat Message Stream (SSE) Example (Go) ===\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		log.Fatal("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	// Create a chat session
	systemPrompt := "You are a helpful assistant."
	session, err := client.CreateChatSession(ekodb.CreateChatSessionRequest{
		SystemPrompt: &systemPrompt,
	})
	if err != nil {
		log.Fatalf("Failed to create session: %v", err)
	}
	fmt.Printf("Created session: %s\n", session.ChatID)

	// Stream a chat message via SSE
	fmt.Println("\nStreaming response for: 'What is ekoDB?'\n")
	eventCh, err := client.ChatMessageStream(session.ChatID, ekodb.ChatMessageRequest{
		Message: "What is ekoDB?",
	})
	if err != nil {
		log.Fatalf("Failed to start stream: %v", err)
	}

	for event := range eventCh {
		switch event.Type {
		case "chunk":
			fmt.Print(event.Content)
		case "end":
			fmt.Printf("\n\n--- Stream complete ---\n")
			fmt.Printf("Message ID: %s\n", event.MessageID)
			fmt.Printf("Execution time: %dms\n", event.ExecutionTimeMs)
			if event.ContextWindow > 0 {
				fmt.Printf("Context window: %d tokens\n", event.ContextWindow)
			}
		case "error":
			fmt.Printf("\nError: %s\n", event.Error)
		}
	}

	fmt.Println("\n✓ Chat message stream example completed")
}
