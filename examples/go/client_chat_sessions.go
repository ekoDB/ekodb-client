// ekoDB Go Client - Chat Session Management Example
//
// This example demonstrates advanced chat session management including:
// - Creating sessions
// - Sending messages
// - Retrieving messages
// - Updating sessions
// - Branching sessions
// - Listing sessions
// - Deleting sessions

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
	fmt.Print("=== ekoDB Chat Session Management Example ===\n\n")

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

	collection := "client_chat_sessions_go"
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
		"description": "A high-performance database product",
		"price":       99,
	})
	if err != nil {
		return fmt.Errorf("insert sample data: %w", err)
	}
	fmt.Print("✓ Inserted sample product\n\n")

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

	// Step 3: Send messages in the session
	fmt.Println("=== Sending Messages ===")
	msg1, err := client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What products are available?",
	})
	if err != nil {
		return fmt.Errorf("send first message: %w", err)
	}
	fmt.Println("✓ Message 1 sent")
	if msg1.MessageID == "" || len(msg1.Responses) == 0 || strings.TrimSpace(msg1.Responses[0]) == "" {
		return fmt.Errorf("first message returned no usable response")
	}
	fmt.Printf("  Response: %s\n\n", msg1.Responses[0])

	msg2, err := client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What is the price?",
	})
	if err != nil {
		return fmt.Errorf("send second message: %w", err)
	}
	fmt.Println("✓ Message 2 sent")
	if msg2.MessageID == "" || len(msg2.Responses) == 0 || strings.TrimSpace(msg2.Responses[0]) == "" {
		return fmt.Errorf("second message returned no usable response")
	}
	fmt.Printf("  Response: %s\n\n", msg2.Responses[0])

	// Step 4: Get session messages
	fmt.Println("=== Retrieving Session Messages ===")
	limit := 10
	sort := "asc"
	messagesResponse, err := client.GetChatSessionMessages(chatID, &ekodb.GetMessagesQuery{
		Limit: &limit,
		Sort:  &sort,
	})
	if err != nil {
		return fmt.Errorf("get messages: %w", err)
	}
	fmt.Printf("✓ Retrieved %d messages\n\n", len(messagesResponse.Messages))
	if len(messagesResponse.Messages) < 4 {
		return fmt.Errorf("retrieved %d messages, expected at least 4", len(messagesResponse.Messages))
	}

	// Step 5: Update session
	fmt.Println("=== Updating Session ===")
	newPrompt := "You are an expert product consultant."
	_, err = client.UpdateChatSession(chatID, ekodb.UpdateSessionRequest{
		SystemPrompt: &newPrompt,
	})
	if err != nil {
		return fmt.Errorf("update session: %w", err)
	}
	fmt.Print("✓ Session updated\n\n")

	// Step 6: Branch the session
	fmt.Println("=== Branching Session ===")
	branchIdx := 0
	branched, err := client.BranchChatSession(ekodb.CreateChatSessionRequest{
		Collections: []ekodb.CollectionConfig{
			{
				CollectionName: "products",
				Fields:         []interface{}{}, // Empty = search all fields
			},
		},
		LLMProvider:    "openai",
		LLMModel:       &llmModel,
		ParentID:       &chatID,
		BranchPointIdx: &branchIdx,
	})
	if err != nil {
		return fmt.Errorf("branch session: %w", err)
	}
	branchID := branched.ChatID
	branchDeleted := false
	defer func() {
		if !branchDeleted {
			if err := client.DeleteChatSession(branchID); err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete branch session: %w", err))
			}
		}
	}()
	fmt.Printf("✓ Created branch: %s\n", branchID)
	fmt.Printf("  Parent: %s\n\n", chatID)

	// Step 7: List all sessions
	fmt.Println("=== Listing Sessions ===")
	listLimit := 10
	listSort := "desc"
	sessionsList, err := client.ListChatSessions(&ekodb.ListSessionsQuery{
		Limit: &listLimit,
		Sort:  &listSort,
	})
	if err != nil {
		return fmt.Errorf("list sessions: %w", err)
	}
	fmt.Printf("✓ Found %d sessions\n", len(sessionsList.Sessions))
	for i, s := range sessionsList.Sessions {
		title := "Untitled"
		if s.Title != nil {
			title = *s.Title
		}
		fmt.Printf("  Session %d: %s (%s)\n", i+1, s.ChatID, title)
	}
	fmt.Println()

	// Step 8: Get session details
	fmt.Println("=== Getting Session Details ===")
	sessionDetails, err := client.GetChatSession(chatID)
	if err != nil {
		return fmt.Errorf("get session: %w", err)
	}
	fmt.Println("✓ Session details retrieved")
	fmt.Printf("  Messages: %d\n\n", sessionDetails.MessageCount)

	// Step 9: Delete branch session
	fmt.Println("=== Deleting Branch Session ===")
	if err := client.DeleteChatSession(branchID); err != nil {
		return fmt.Errorf("delete branch session: %w", err)
	}
	branchDeleted = true
	fmt.Printf("✓ Deleted branch session: %s\n\n", branchID)

	// Cleanup
	fmt.Println("=== Cleanup ===")
	if err := client.DeleteChatSession(chatID); err != nil {
		return fmt.Errorf("delete chat session: %w", err)
	}
	chatDeleted = true
	fmt.Println("✓ Deleted session")
	if err := client.DeleteCollection(collection); err != nil {
		return fmt.Errorf("delete collection: %w", err)
	}
	collectionDeleted = true
	fmt.Print("✓ Deleted collection\n\n")

	fmt.Println("✓ All session management operations completed successfully")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
