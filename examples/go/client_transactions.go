package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

// Transaction Example using ekoDB Go Client
//
// Demonstrates:
// - Begin transaction with isolation level
// - Operations within a transaction
// - Transaction status checking
// - Commit transaction
// - Rollback transaction

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
		log.Fatal("API_BASE_KEY environment variable not set")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Client created\n")

	// Setup test data
	fmt.Println("=== Setup: Creating Test Accounts ===")
	alice, err := client.Insert("test_accounts", map[string]interface{}{
		"account_id": "ACC001",
		"name":       "Alice",
		"balance":    1000,
	})
	if err != nil {
		log.Fatal(err)
	}
	aliceID := alice["id"].(string)
	fmt.Printf("Created Alice: $1000 - ID: %s\n", aliceID)

	bob, err := client.Insert("test_accounts", map[string]interface{}{
		"account_id": "ACC002",
		"name":       "Bob",
		"balance":    500,
	})
	if err != nil {
		log.Fatal(err)
	}
	bobID := bob["id"].(string)
	fmt.Printf("Created Bob: $500 - ID: %s\n\n", bobID)

	// Example 1: Begin Transaction
	fmt.Println("=== Example 1: Begin Transaction ===")
	txID, err := client.BeginTransaction("READ_COMMITTED")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Transaction ID: %s\n\n", txID)

	// Example 2: Operations within Transaction
	fmt.Println("=== Example 2: Operations within Transaction ===")
	fmt.Println("Updated Alice: $1000 → $800 (simulated)")
	fmt.Println("Updated Bob: $500 → $700 (simulated)\n")

	// Example 3: Transaction Status
	fmt.Println("=== Example 3: Transaction Status ===")
	status, err := client.GetTransactionStatus(txID)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Status: %v\n", status["state"])
	fmt.Printf("Operations: %v\n\n", status["operations_count"])

	// Example 4: Commit Transaction
	fmt.Println("=== Example 4: Commit Transaction ===")
	if err := client.CommitTransaction(txID); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Transaction committed\n")

	// Example 5: Rollback Demo
	fmt.Println("=== Example 5: Rollback Demo ===")
	txID2, err := client.BeginTransaction("READ_COMMITTED")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("New transaction: %s\n", txID2)

	status2, _ := client.GetTransactionStatus(txID2)
	fmt.Printf("Status before rollback: %v\n", status2["state"])

	if err := client.RollbackTransaction(txID2); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Transaction rolled back\n")

	// Cleanup
	fmt.Println("=== Cleanup ===")
	client.Delete("test_accounts", aliceID)
	client.Delete("test_accounts", bobID)
	fmt.Println("✓ Deleted test accounts\n")

	fmt.Println("✓ All client transaction examples completed")
}
