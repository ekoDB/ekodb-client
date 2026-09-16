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
	"errors"
	"fmt"
	"log"
	"os"
	"strings"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ All advanced chat features demonstrated successfully!")
}

func run() (runErr error) {
	fmt.Print("=== ekoDB Advanced Chat Features Example ===\n\n")

	// Load environment variables
	_ = godotenv.Load()

	// Create client
	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		return errors.New("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}

	collection := "client_chat_advanced_go"
	chatIDs := []string{}
	collectionCreated := false
	defer func() {
		fmt.Println("=== Cleanup ===")
		for i := len(chatIDs) - 1; i >= 0; i-- {
			chatID := chatIDs[i]
			if cleanupErr := client.DeleteChatSession(chatID); cleanupErr != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete chat session %s: %w", chatID, cleanupErr))
			} else {
				fmt.Printf("✓ Deleted chat session: %s\n", chatID)
			}
		}
		if collectionCreated {
			if cleanupErr := client.DeleteCollection(collection); cleanupErr != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete collection %s: %w", collection, cleanupErr))
			} else {
				fmt.Print("✓ Deleted collection\n\n")
			}
		}
	}()

	// Insert sample data
	fmt.Println("=== Inserting Sample Data ===")
	collectionCreated = true
	_, err = client.Insert(collection, ekodb.Record{
		"name":        "ekoDB",
		"description": "High-performance database product",
		"price":       99,
	})
	if err != nil {
		return fmt.Errorf("insert sample product: %w", err)
	}
	fmt.Print("✓ Inserted sample product\n\n")

	// Create a chat session
	fmt.Println("=== Creating Chat Session ===")
	llmModel := "gpt-4o-mini"
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
		return fmt.Errorf("create chat session: %w", err)
	}
	chatID := session.ChatID
	chatIDs = append(chatIDs, chatID)
	fmt.Printf("✓ Created session: %s\n\n", chatID)

	// Send initial message
	fmt.Println("=== Sending Initial Message ===")
	msg1, err := client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What products are available?",
	})
	if err != nil {
		return fmt.Errorf("send initial message: %w", err)
	}
	if msg1.MessageID == "" || len(msg1.Responses) == 0 || strings.TrimSpace(msg1.Responses[0]) == "" {
		return errors.New("initial chat message returned no usable response")
	}
	fmt.Println("✓ Message sent")
	fmt.Printf("  Response: %s\n\n", msg1.Responses[0])

	// Send a second message
	_, err = client.ChatMessage(chatID, ekodb.ChatMessageRequest{
		Message: "What is the price of ekoDB?",
	})
	if err != nil {
		return fmt.Errorf("send second message: %w", err)
	}
	fmt.Print("✓ Second message sent\n\n")

	// Get messages to find user and assistant message IDs
	messages, err := client.GetChatSessionMessages(chatID, &ekodb.GetMessagesQuery{
		Limit: ekodb.IntPtr(10),
		Sort:  ekodb.StringPtr("desc"),
	})
	if err != nil {
		return fmt.Errorf("get messages: %w", err)
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
		return errors.New("could not find assistant message")
	}
	if userMessageID == "" {
		return errors.New("could not find user message")
	}

	// Feature 1: Regenerate Message
	fmt.Println("=== Feature 1: Regenerate AI Response ===")
	regenerated, err := client.RegenerateChatMessage(chatID, assistantMessageID)
	if err != nil {
		return fmt.Errorf("regenerate message: %w", err)
	}
	if regenerated.MessageID == "" || len(regenerated.Responses) == 0 || strings.TrimSpace(regenerated.Responses[0]) == "" {
		return errors.New("regenerated chat message returned no usable response")
	}
	fmt.Println("✓ Message regenerated")
	fmt.Printf("  New response: %s\n\n", regenerated.Responses[0])

	// Feature 2: Update Message Content
	fmt.Println("=== Feature 2: Edit Message ===")
	err = client.UpdateChatMessage(chatID, userMessageID, "What is the updated price of ekoDB?")
	if err != nil {
		return fmt.Errorf("update message: %w", err)
	}
	fmt.Print("✓ Message content updated\n\n")

	// Feature 3: Toggle Forgotten Status
	fmt.Println("=== Feature 3: Mark Message as Forgotten ===")
	err = client.ToggleForgottenMessage(chatID, userMessageID, true)
	if err != nil {
		return fmt.Errorf("mark message forgotten: %w", err)
	}
	fmt.Print("✓ Message marked as forgotten (excluded from LLM context)\n\n")

	// Unmark as forgotten
	err = client.ToggleForgottenMessage(chatID, userMessageID, false)
	if err != nil {
		return fmt.Errorf("unmark message forgotten: %w", err)
	}
	fmt.Print("✓ Message unmarked as forgotten\n\n")

	// Feature 4: Session Merging
	fmt.Println("=== Feature 4: Merge Chat Sessions ===")

	// Create a second session
	session2, err := client.CreateChatSession(ekodb.CreateChatSessionRequest{
		Collections: []ekodb.CollectionConfig{
			{
				CollectionName: collection,
				Fields:         []interface{}{}, // Empty = search all fields
			},
		},
		LLMProvider: "openai",
		LLMModel:    &llmModel,
	})
	if err != nil {
		return fmt.Errorf("create second session: %w", err)
	}
	chatID2 := session2.ChatID
	chatIDs = append(chatIDs, chatID2)
	fmt.Printf("✓ Created second session: %s\n", chatID2)

	// Send a message in the second session
	_, err = client.ChatMessage(chatID2, ekodb.ChatMessageRequest{
		Message: "Tell me more about the features",
	})
	if err != nil {
		return fmt.Errorf("send second-session message: %w", err)
	}
	fmt.Println("✓ Sent message in second session")

	// Merge sessions
	mergeResult, err := client.MergeChatSessions(ekodb.MergeSessionsRequest{
		SourceChatIDs: []string{chatID2},
		TargetChatID:  chatID,
		MergeStrategy: ekodb.MergeStrategyChronological,
	})
	if err != nil {
		return fmt.Errorf("merge sessions: %w", err)
	}
	if mergeResult.MessageCount == 0 {
		return errors.New("merged session reported zero messages")
	}
	fmt.Println("✓ Sessions merged successfully")
	fmt.Printf("  Total messages in merged session: %d\n\n", mergeResult.MessageCount)

	// Feature 5: Delete Message
	fmt.Println("=== Feature 5: Delete Message ===")
	err = client.DeleteChatMessage(chatID, userMessageID)
	if err != nil {
		return fmt.Errorf("delete message: %w", err)
	}
	fmt.Print("✓ Message deleted\n\n")

	// Verify message count after deletion
	sessionDetails, err := client.GetChatSession(chatID)
	if err != nil {
		return fmt.Errorf("get merged session: %w", err)
	}
	fmt.Printf("✓ Messages remaining: %d\n\n", sessionDetails.MessageCount)

	return nil
}
