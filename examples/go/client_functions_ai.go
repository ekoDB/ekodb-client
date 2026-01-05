// AI Scripts Example - Chat and Embed Operations
//
// Demonstrates AI operations in scripts:
// - Chat completions with context
// - Embedding generation
// - Simple AI workflows

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found, using defaults")
	}

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}

	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("🚀 ekoDB Go AI Scripts Example")
	fmt.Println()

	// Setup test data
	fmt.Println("📋 Setting up test data...")
	client.DeleteCollection("ai_articles_go")

	articles := []map[string]interface{}{
		{"title": "Getting Started with ekoDB", "content": "ekoDB is a high-performance database...", "status": "published"},
		{"title": "Advanced Query Patterns", "content": "Learn how to build complex queries...", "status": "published"},
	}

	for _, article := range articles {
		client.Insert("ai_articles_go", article)
	}
	fmt.Printf("✅ Created %d articles\n\n", len(articles))

	var scriptIDs []string

	// Example 1: Simple Chat Completion
	fmt.Println("📝 Example 1: Simple Chat Completion")
	fmt.Println()

	script1 := map[string]interface{}{
		"label":       "ai_assistant_go",
		"name":        "AI Chat Assistant",
		"description": "Simple AI chat completion",
		"version":     "1.0",
		"parameters":  map[string]interface{}{},
		"functions": []interface{}{
			ekodb.Stage.Chat(
				[]map[string]interface{}{
					ekodb.ChatMessage.System("You are a helpful database assistant. Be concise."),
					ekodb.ChatMessage.User("What are the benefits of using vector databases?"),
				},
				"gpt-4",
				0.7,
			),
		},
		"tags": []string{"ai", "chat"},
	}
	scriptID1, _ := client.SaveScript(script1)
	scriptIDs = append(scriptIDs, scriptID1)
	fmt.Println("✅ Chat script saved")

	result1, _ := client.CallScript("ai_assistant_go", nil)
	fmt.Println("🤖 AI Response:")
	if records, ok := result1["records"].([]interface{}); ok && len(records) > 0 {
		if record, ok := records[0].(map[string]interface{}); ok {
			if response, ok := record["response"]; ok {
				fmt.Printf("   %v\n", response)
			}
		}
	}
	if stats, ok := result1["stats"].(map[string]interface{}); ok {
		fmt.Printf("⏱️  Execution time: %vms\n\n", stats["execution_time_ms"])
	}

	// Example 2: Embed Generation
	fmt.Println("📝 Example 2: Generate Embeddings")
	fmt.Println()

	script2 := map[string]interface{}{
		"label":       "generate_embedding_go",
		"name":        "Generate Embedding",
		"description": "Generate embedding for text",
		"version":     "1.0",
		"parameters": map[string]interface{}{
			"text": map[string]interface{}{"required": true, "description": "Text to embed"},
		},
		"functions": []interface{}{ekodb.Stage.Embed("{{text}}")},
		"tags":      []string{"ai", "embed"},
	}
	scriptID2, _ := client.SaveScript(script2)
	scriptIDs = append(scriptIDs, scriptID2)
	fmt.Println("✅ Embed script saved")

	result2, _ := client.CallScript("generate_embedding_go", map[string]interface{}{
		"text": "ekoDB is a powerful database",
	})
	fmt.Println("📊 Embedding generated")
	if stats, ok := result2["stats"].(map[string]interface{}); ok {
		fmt.Printf("⏱️  Execution time: %vms\n\n", stats["execution_time_ms"])
	}

	// Cleanup
	fmt.Println("🧹 Cleaning up...")
	for _, scriptID := range scriptIDs {
		client.DeleteScript(scriptID)
	}
	client.DeleteCollection("ai_articles_go")
	fmt.Println("✅ Cleanup complete")
	fmt.Println()
	fmt.Println("✅ All AI script examples finished!")
	fmt.Println()
	fmt.Println("💡 This example demonstrates:")
	fmt.Println("   ✅ Chat completions with system/user messages")
	fmt.Println("   ✅ Embedding generation for text")
}
