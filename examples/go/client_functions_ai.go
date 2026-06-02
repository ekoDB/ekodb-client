// AI Functions Example - Chat and Embed Operations
//
// Demonstrates AI operations in scripts:
// - Chat completions with context
// - Embedding generation
// - Simple AI workflows

package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

// saveOrUpdateFn saves a function, or — if the label already exists (HTTP 409)
// — updates it in place and recovers the encrypted ID via a GET by label.
func saveOrUpdateFn(client *ekodb.Client, fn ekodb.UserFunction) (string, error) {
	id, err := client.SaveFunction(fn)
	if err == nil {
		return id, nil
	}
	var httpErr *ekodb.HTTPError
	if errors.As(err, &httpErr) && httpErr.StatusCode == 409 {
		if uerr := client.UpdateFunction(fn.Label, fn); uerr != nil {
			return "", uerr
		}
		fmt.Printf("Function '%s' already existed — updated instead\n", fn.Label)
		existing, gerr := client.GetFunction(fn.Label)
		if gerr != nil {
			return "", gerr
		}
		if existing.ID == nil {
			return "", fmt.Errorf("function %q has no id after update", fn.Label)
		}
		return *existing.ID, nil
	}
	return "", err
}

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

	fmt.Println("🚀 ekoDB Go AI Functions Example")
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

	model1 := "gpt-4o-mini"
	temp1 := 0.7
	script1 := ekodb.UserFunction{
		Label:       "ai_assistant_go",
		Name:        "AI Chat Assistant",
		Description: func() *string { s := "Simple AI chat completion"; return &s }(),
		Version:     func() *string { s := "1.0"; return &s }(),
		Parameters:  map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageChat(
				[]ekodb.ChatMessage{
					{Role: "system", Content: "You are a helpful database assistant. Be concise."},
					{Role: "user", Content: "What are the benefits of using vector databases?"},
				},
				&model1,
				&temp1,
			),
		},
		Tags: []string{"ai", "chat"},
	}
	scriptID1, _ := saveOrUpdateFn(client, script1)
	scriptIDs = append(scriptIDs, scriptID1)
	fmt.Println("✅ Chat script saved")

	result1, _ := client.CallFunction("ai_assistant_go", nil)
	if result1 != nil {
		fmt.Println("📊 AI Response generated")
		fmt.Printf("⏱️  Execution time: %vms\n\n", result1.Stats.ExecutionTimeMs)
	}

	// Example 2: Embed Generation
	fmt.Println("📝 Example 2: Generate Embeddings")
	fmt.Println()

	model2 := "text-embedding-3-small"
	script2 := ekodb.UserFunction{
		Label:       "generate_embedding_go",
		Name:        "Generate Embedding",
		Description: func() *string { s := "Generate embedding for text"; return &s }(),
		Version:     func() *string { s := "1.0"; return &s }(),
		Parameters: map[string]ekodb.ParameterDefinition{
			"text": {Required: true, Description: "Text to embed"},
		},
		Functions: []ekodb.FunctionStageConfig{ekodb.StageEmbed("{{text}}", &model2)},
		Tags:      []string{"ai", "embed"},
	}
	scriptID2, _ := saveOrUpdateFn(client, script2)
	scriptIDs = append(scriptIDs, scriptID2)
	fmt.Println("✅ Embed script saved")

	result2, _ := client.CallFunction("generate_embedding_go", map[string]interface{}{
		"text": "ekoDB is a powerful database",
	})
	if result2 != nil {
		fmt.Println("📊 Embedding generated")
		fmt.Printf("⏱️  Execution time: %vms\n\n", result2.Stats.ExecutionTimeMs)
	}

	// Cleanup
	fmt.Println("🧹 Cleaning up...")
	for _, scriptID := range scriptIDs {
		client.DeleteFunction(scriptID)
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
