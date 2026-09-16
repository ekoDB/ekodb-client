// ekoDB Go Client - Chat Message Stream (SSE) Example
//
// Demonstrates streaming chat responses via HTTP SSE (Server-Sent Events).
// This is a simpler alternative to WebSocket streaming that doesn't require
// a persistent connection.

package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func run() (runErr error) {
	fmt.Print("=== ekoDB Chat Message Stream (SSE) Example (Go) ===\n\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		return fmt.Errorf("API_BASE_KEY environment variable is required")
	}

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
		return fmt.Errorf("create session: %w", err)
	}
	defer func() {
		if err := client.DeleteChatSession(session.ChatID); err != nil {
			runErr = errors.Join(runErr, fmt.Errorf("delete chat session: %w", err))
		}
	}()
	fmt.Printf("Created session: %s\n", session.ChatID)

	// Stream a chat message via SSE
	fmt.Print("\nStreaming response for: 'What is ekoDB?'\n\n")
	eventCh, err := client.ChatMessageStream(context.Background(), session.ChatID, ekodb.ChatMessageRequest{
		Message: "What is ekoDB?",
	})
	if err != nil {
		return fmt.Errorf("start stream: %w", err)
	}

	completed := false
	contentBytes := 0
	content := ""
	messageID := ""
	for event := range eventCh {
		switch event.Type {
		case "chunk":
			fmt.Print(event.Content)
			contentBytes += len(event.Content)
			content += event.Content
		case "end":
			completed = true
			messageID = event.MessageID
			fmt.Printf("\n\n--- Stream complete ---\n")
			fmt.Printf("Message ID: %s\n", event.MessageID)
			fmt.Printf("Execution time: %dms\n", event.ExecutionTimeMs)
			if event.ContextWindow > 0 {
				fmt.Printf("Context window: %d tokens\n", event.ContextWindow)
			}
		case "error":
			return fmt.Errorf("chat stream failed: %s", event.Error)
		}
	}
	if !completed {
		return fmt.Errorf("chat stream ended before an end event")
	}
	if strings.TrimSpace(content) == "" || strings.TrimSpace(messageID) == "" {
		return fmt.Errorf("chat stream completed without content or message ID: bytes=%d message_id=%q", contentBytes, messageID)
	}

	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("\n✓ Chat message stream example completed")
}
