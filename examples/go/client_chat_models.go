// Chat Models API Example - Using ekodb-client-go library
//
// This example demonstrates the Chat Models API for retrieving available LLM providers and models.

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}

	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}

	// Create ekoDB client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Client created")

	// Example 1: Get all chat models from all providers
	fmt.Println("\n=== Get All Chat Models ===")
	models, err := client.GetChatModels()
	if err != nil {
		log.Printf("GetChatModels error (expected if no LLM configured): %v", err)
	} else {
		fmt.Printf("OpenAI models: %v\n", models.OpenAI)
		fmt.Printf("Anthropic models: %v\n", models.Anthropic)
		fmt.Printf("Perplexity models: %v\n", models.Perplexity)
	}

	// Example 2: Get models for a specific provider
	fmt.Println("\n=== Get OpenAI Models ===")
	openaiModels, err := client.GetChatModel("openai")
	if err != nil {
		log.Printf("GetChatModel error (expected if no LLM configured): %v", err)
	} else {
		fmt.Printf("OpenAI models: %v\n", openaiModels)
	}

	// Example 3: Get Anthropic models
	fmt.Println("\n=== Get Anthropic Models ===")
	anthropicModels, err := client.GetChatModel("anthropic")
	if err != nil {
		log.Printf("GetChatModel error (expected if no LLM configured): %v", err)
	} else {
		fmt.Printf("Anthropic models: %v\n", anthropicModels)
	}

	fmt.Println("\n✓ Chat Models API example complete")
}
