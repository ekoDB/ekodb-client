// ekoDB Go Client - Advanced Chat Features Example
//
// This example demonstrates advanced chat features:
// - Message regeneration
// - Message editing
// - Message deletion
// - Session merging
// - Forgotten messages (exclude from context)

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Advanced Chat Features Example ===\n")

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

	collection := "client_chat_advanced_go"

	// Insert sample data
	fmt.Println("=== Inserting Sample Data ===")
	_, err = client.Insert(collection, ekodb.Record{
		"name":        "ekoDB",
		"description": "High-performance database product",
		"price":       99,
	})
	if err != nil {
		log.Fatalf("Failed to insert: %v", err)
	}
	fmt.Println("✓ Inserted sample product\n")

	// Create a chat session
	fmt.Println("=== Creating Chat Session ===")
	llmModel := "gpt-4"
	systemPrompt := "You are a helpful product assistant."

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

	// Send initial message
	fmt.Println("=== Sending Initial Message ===")
	msg1, err := client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What products are available?",
	})
	if err != nil {
		log.Fatalf("Failed to send message: %v", err)
	}
	fmt.Println("✓ Message sent")
	if len(msg1.Responses) > 0 {
		fmt.Printf("  Response: %s\n\n", msg1.Responses[0])
	}

	// Send a second message
	_, err = client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What is the price of ekoDB?",
	})
	if err != nil {
		log.Fatalf("Failed to send second message: %v", err)
	}
	fmt.Println("✓ Second message sent\n")

	// Get messages to find user and assistant message IDs
	messages, err := client.GetChatSessionMessages(chatID, &ekodb.GetMessagesQuery{
		Limit: ekodb.IntPtr(10),
		Sort:  ekodb.StringPtr("desc"),
	})
	if err != nil {
		log.Fatalf("Failed to get messages: %v", err)
	}

	var assistantMessageID string
	var userMessageID string
	for _, msg := range messages.Messages {
		// Role is wrapped in an Object with type/value structure
		var role string
		if roleObj, ok := msg["role"].(map[string]interface{}); ok {
			// Try the standard ekoDB field format: {"type": "String", "value": "assistant"}
			if roleVal, ok := roleObj["value"].(string); ok {
				role = roleVal
			} else if roleVal, ok := roleObj["value"].(string); ok {
				role = roleVal
			}
		} else if roleStr, ok := msg["role"].(string); ok {
			role = roleStr
		}

		if role == "assistant" && assistantMessageID == "" {
			if id, ok := msg["id"].(string); ok {
				assistantMessageID = id
			}
		}

		if role == "user" && userMessageID == "" {
			if id, ok := msg["id"].(string); ok {
				userMessageID = id
			}
		}
	}
	if assistantMessageID == "" {
		log.Fatal("Could not find assistant message")
	}
	if userMessageID == "" {
		log.Fatal("Could not find user message")
	}

	// Feature 1: Regenerate Message
	fmt.Println("=== Feature 1: Regenerate AI Response ===")
	regenerated, err := client.RegenerateChatMessage(chatID, assistantMessageID)
	if err != nil {
		log.Fatalf("Failed to regenerate message: %v", err)
	}
	fmt.Println("✓ Message regenerated")
	if len(regenerated.Responses) > 0 {
		fmt.Printf("  New response: %s\n\n", regenerated.Responses[0])
	}

	// Feature 2: Update Message Content
	fmt.Println("=== Feature 2: Edit Message ===")
	err = client.UpdateChatMessage(chatID, userMessageID, "What is the updated price of ekoDB?")
	if err != nil {
		log.Fatalf("Failed to update message: %v", err)
	}
	fmt.Println("✓ Message content updated\n")

	// Feature 3: Toggle Forgotten Status
	fmt.Println("=== Feature 3: Mark Message as Forgotten ===")
	err = client.ToggleForgottenMessage(chatID, userMessageID, true)
	if err != nil {
		log.Fatalf("Failed to toggle forgotten: %v", err)
	}
	fmt.Println("✓ Message marked as forgotten (excluded from LLM context)\n")

	// Unmark as forgotten
	err = client.ToggleForgottenMessage(chatID, userMessageID, false)
	if err != nil {
		log.Fatalf("Failed to toggle forgotten: %v", err)
	}
	fmt.Println("✓ Message unmarked as forgotten\n")

	// Feature 4: Session Merging
	fmt.Println("=== Feature 4: Merge Chat Sessions ===")

	// Create a second session
	session2, err := client.CreateChatSession(ekodb.CreateChatSessionRequest{
		Collections: []ekodb.CollectionConfig{
			{
				CollectionName: "products",
				Fields:         []interface{}{}, // Empty = search all fields
			},
		},
		LLMProvider: "openai",
		LLMModel:    &llmModel,
	})
	if err != nil {
		log.Fatalf("Failed to create second session: %v", err)
	}
	chatID2 := session2.ChatID
	fmt.Printf("✓ Created second session: %s\n", chatID2)

	// Send a message in the second session
	_, err = client.ChatMessage(chatID2, ekodb.ChatMessageRequest{
		Message: "Tell me more about the features",
	})
	if err != nil {
		log.Fatalf("Failed to send message: %v", err)
	}
	fmt.Println("✓ Sent message in second session")

	// Merge sessions
	mergeResult, err := client.MergeChatSessions(ekodb.MergeSessionsRequest{
		SourceChatIDs: []string{chatID2},
		TargetChatID:  chatID,
		MergeStrategy: ekodb.MergeStrategyChronological,
	})
	if err != nil {
		log.Fatalf("Failed to merge sessions: %v", err)
	}
	fmt.Println("✓ Sessions merged successfully")
	fmt.Printf("  Total messages in merged session: %d\n\n", mergeResult.MessageCount)

	// Feature 5: Delete Message
	fmt.Println("=== Feature 5: Delete Message ===")
	err = client.DeleteChatMessage(chatID, userMessageID)
	if err != nil {
		log.Fatalf("Failed to delete message: %v", err)
	}
	fmt.Println("✓ Message deleted\n")

	// Verify message count after deletion
	sessionDetails, err := client.GetChatSession(chatID)
	if err != nil {
		log.Fatalf("Failed to get session: %v", err)
	}
	fmt.Printf("✓ Messages remaining: %d\n\n", sessionDetails.MessageCount)

	// Cleanup
	fmt.Println("=== Cleanup ===")
	err = client.DeleteChatSession(chatID)
	if err != nil {
		log.Fatalf("Failed to delete session: %v", err)
	}
	fmt.Println("✓ Deleted session")

	if err := client.DeleteCollection(collection); err != nil {
		log.Fatalf("Failed to delete collection: %v", err)
	}
	fmt.Println("✓ Deleted collection\n")

	fmt.Println("✓ All advanced chat features demonstrated successfully!")
}
