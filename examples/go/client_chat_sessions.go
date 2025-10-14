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
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Chat Session Management Example ===\n")

	// Load environment variables
	_ = godotenv.Load()

	// Create client
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

	collection := "client_chat_sessions_go"

	// Step 1: Insert sample data
	fmt.Println("=== Inserting Sample Data ===")
	_, err = client.Insert(collection, ekodb.Record{
		"name":        "ekoDB",
		"description": "A high-performance database product",
		"price":       99,
	})
	if err != nil {
		log.Fatalf("Failed to insert: %v", err)
	}
	fmt.Println("✓ Inserted sample product\n")

	// Step 2: Create a chat session
	fmt.Println("=== Creating Chat Session ===")
	llmModel := "gpt-4"
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
		log.Fatalf("Failed to create session: %v", err)
	}
	chatID := session.ChatID
	fmt.Printf("✓ Created session: %s\n\n", chatID)

	// Step 3: Send messages in the session
	fmt.Println("=== Sending Messages ===")
	msg1, err := client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What products are available?",
	})
	if err != nil {
		log.Fatalf("Failed to send message: %v", err)
	}
	fmt.Println("✓ Message 1 sent")
	if len(msg1.Responses) > 0 {
		fmt.Printf("  Response: %s\n\n", msg1.Responses[0])
	} else {
		fmt.Println("  Response: No response\n")
	}

	msg2, err := client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What is the price?",
	})
	if err != nil {
		log.Fatalf("Failed to send message: %v", err)
	}
	fmt.Println("✓ Message 2 sent")
	if len(msg2.Responses) > 0 {
		fmt.Printf("  Response: %s\n\n", msg2.Responses[0])
	} else {
		fmt.Println("  Response: No response\n")
	}

	// Step 4: Get session messages
	fmt.Println("=== Retrieving Session Messages ===")
	limit := 10
	sort := "asc"
	messagesResponse, err := client.GetChatSessionMessages(chatID, &ekodb.GetMessagesQuery{
		Limit: &limit,
		Sort:  &sort,
	})
	if err != nil {
		log.Fatalf("Failed to get messages: %v", err)
	}
	fmt.Printf("✓ Retrieved %d messages\n\n", len(messagesResponse.Messages))

	// Step 5: Update session
	fmt.Println("=== Updating Session ===")
	newPrompt := "You are an expert product consultant."
	_, err = client.UpdateChatSession(chatID, ekodb.UpdateSessionRequest{
		SystemPrompt: &newPrompt,
	})
	if err != nil {
		log.Fatalf("Failed to update session: %v", err)
	}
	fmt.Println("✓ Session updated\n")

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
		log.Fatalf("Failed to branch session: %v", err)
	}
	branchID := branched.ChatID
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
		log.Fatalf("Failed to list sessions: %v", err)
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
		log.Fatalf("Failed to get session: %v", err)
	}
	fmt.Println("✓ Session details retrieved")
	fmt.Printf("  Messages: %d\n\n", sessionDetails.MessageCount)

	// Step 9: Delete branch session
	fmt.Println("=== Deleting Branch Session ===")
	err = client.DeleteChatSession(branchID)
	if err != nil {
		log.Fatalf("Failed to delete session: %v", err)
	}
	fmt.Printf("✓ Deleted branch session: %s\n\n", branchID)

	// Cleanup: Delete the collection (chat sessions are managed by server)
	fmt.Println("=== Cleanup ===")
	if err := client.DeleteCollection(collection); err != nil {
		log.Fatalf("Failed to delete collection: %v", err)
	}
	fmt.Println("✓ Deleted collection\n")

	fmt.Println("✓ All session management operations completed successfully")
}
