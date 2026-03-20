// ekoDB Go Client - Raw Completion Stream (SSE) Example
//
// Demonstrates using SSE streaming for raw LLM completions.
// SSE streaming keeps the connection alive with heartbeat events, making it
// ideal for deployed instances where reverse proxies may kill idle connections.

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Raw Completion Stream (SSE) Example ===\n")

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

	// --- 1. Basic streaming raw completion ---
	fmt.Println("--- Basic SSE Raw Completion ---")
	resp, err := client.RawCompletionStream(ekodb.RawCompletionRequest{
		SystemPrompt: "You are a helpful assistant. Keep responses concise.",
		Message:      "What are the three primary colors?",
		MaxTokens:    ekodb.IntPtr(256),
	})
	if err != nil {
		log.Fatalf("RawCompletionStream failed: %v", err)
	}
	fmt.Printf("Response: %s\n\n", resp.Content)

	// --- 2. Structured output via SSE ---
	fmt.Println("--- Structured Output via SSE ---")
	resp, err = client.RawCompletionStream(ekodb.RawCompletionRequest{
		SystemPrompt: "Output only valid JSON. No other text.",
		Message:      "Return a JSON array of 3 planets with name and diameter_km fields.",
		MaxTokens:    ekodb.IntPtr(512),
	})
	if err != nil {
		log.Fatalf("RawCompletionStream failed: %v", err)
	}
	fmt.Printf("JSON response: %s\n\n", resp.Content)

	// --- 3. Comparison: blocking HTTP vs SSE ---
	// On deployed instances behind reverse proxies, RawCompletion() may timeout.
	// RawCompletionStream() keeps the connection alive with SSE heartbeats.
	fmt.Println("--- Blocking HTTP (for comparison) ---")
	resp, err = client.RawCompletion(ekodb.RawCompletionRequest{
		SystemPrompt: "You are a helpful assistant.",
		Message:      "Say hello in one sentence.",
		MaxTokens:    ekodb.IntPtr(64),
	})
	if err != nil {
		log.Fatalf("RawCompletion failed: %v", err)
	}
	fmt.Printf("Blocking response: %s\n\n", resp.Content)

	fmt.Println("=== Done ===")
}
