// ekoDB Go Client - Basic Chat Example
//
// This example demonstrates basic chat functionality with ekoDB.

package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"strings"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func run() (runErr error) {
	fmt.Print("=== ekoDB Chat Basic Example ===\n\n")

	// Load environment variables
	_ = godotenv.Load()

	// Create client
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

	collection := "client_chat_basic_go"
	collectionDeleted := false
	defer func() {
		if !collectionDeleted {
			if err := client.DeleteCollection(collection); err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete collection: %w", err))
			}
		}
	}()

	// Step 1: Insert sample data
	fmt.Println("=== Inserting Sample Data ===")
	_, err = client.Insert(collection, ekodb.Record{
		"name":        "ekoDB",
		"description": "A high-performance database product with AI capabilities",
		"price":       99,
	})
	if err != nil {
		return fmt.Errorf("insert product: %w", err)
	}

	_, err = client.Insert(collection, ekodb.Record{
		"name":        "ekoDB Pro",
		"description": "Enterprise edition product with advanced features",
		"price":       299,
	})
	if err != nil {
		return fmt.Errorf("insert pro product: %w", err)
	}

	_, err = client.Insert(collection, ekodb.Record{
		"name":        "ekoDB Cloud",
		"description": "Fully managed cloud database service product",
		"price":       499,
	})
	if err != nil {
		return fmt.Errorf("insert cloud product: %w", err)
	}
	fmt.Print("✓ Inserted 3 sample documents\n\n")

	// Step 2: Create a chat session
	fmt.Println("=== Creating Chat Session ===")
	llmModel := "gpt-4o-mini"
	systemPrompt := "You are a helpful assistant for product information."

	session, err := client.CreateChatSession(ekodb.CreateChatSessionRequest{
		Collections: []ekodb.CollectionConfig{
			{
				CollectionName: collection,
				Fields:         []interface{}{}, // Empty = search all fields
			},
		},
		LLMProvider:  "openai",
		LLMModel:     &llmModel,
		SystemPrompt: &systemPrompt,
	})
	if err != nil {
		return fmt.Errorf("create session: %w", err)
	}
	chatID := session.ChatID
	chatDeleted := false
	defer func() {
		if !chatDeleted {
			if err := client.DeleteChatSession(chatID); err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete chat session: %w", err))
			}
		}
	}()
	fmt.Printf("✓ Created session: %s\n\n", chatID)

	// Step 3: Send a chat message
	fmt.Println("=== Sending Chat Message ===")
	response, err := client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What products are available and what are their prices?",
	})
	if err != nil {
		return fmt.Errorf("chat: %w", err)
	}
	if response.MessageID == "" || len(response.Responses) == 0 || strings.TrimSpace(response.Responses[0]) == "" {
		return fmt.Errorf("chat returned no usable response: message_id=%q responses=%d", response.MessageID, len(response.Responses))
	}

	fmt.Printf("Message ID: %s\n", response.MessageID)
	fmt.Println("\n=== AI Response ===")
	for _, r := range response.Responses {
		fmt.Println(r)
	}
	fmt.Printf("\n=== Context Used (%d snippets) ===\n", len(response.ContextSnippets))
	for i, snippet := range response.ContextSnippets {
		fmt.Printf("  Snippet %d: %v\n", i+1, snippet)
	}
	fmt.Printf("\nExecution Time: %dms\n", response.ExecutionTimeMs)

	if response.TokenUsage != nil {
		fmt.Println("\n=== Token Usage ===")
		fmt.Printf("Prompt tokens: %d\n", response.TokenUsage.PromptTokens)
		fmt.Printf("Completion tokens: %d\n", response.TokenUsage.CompletionTokens)
		fmt.Printf("Total tokens: %d\n", response.TokenUsage.TotalTokens)
	}

	// Cleanup
	fmt.Println("\n=== Cleanup ===")
	if err := client.DeleteChatSession(chatID); err != nil {
		return fmt.Errorf("delete chat session: %w", err)
	}
	chatDeleted = true
	fmt.Println("✓ Deleted session")
	if err := client.DeleteCollection(collection); err != nil {
		return fmt.Errorf("delete collection: %w", err)
	}
	collectionDeleted = true
	fmt.Println("✓ Deleted collection")

	fmt.Println("\n✓ Chat completed successfully")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
