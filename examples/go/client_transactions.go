package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

// Transaction Example using ekoDB Go Client
//
// Demonstrates:
// - Begin transaction with the server default or an explicit isolation level
// - Operations within a transaction
// - Transaction status checking
// - Commit transaction
// - Rollback transaction

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ All client transaction examples completed")
}

func run() (runErr error) {
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
		return errors.New("API_BASE_KEY environment variable not set")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}
	fmt.Print("✓ Client created\n\n")

	collection := "client_transactions_accounts_go"
	activeTransactions := []string{}
	collectionCreated := false
	defer func() {
		fmt.Println("=== Cleanup ===")
		for i := len(activeTransactions) - 1; i >= 0; i-- {
			if cleanupErr := client.RollbackTransaction(activeTransactions[i]); cleanupErr != nil {
				runErr = errors.Join(runErr, fmt.Errorf("rollback transaction %s: %w", activeTransactions[i], cleanupErr))
			}
		}
		if collectionCreated {
			if cleanupErr := client.DeleteCollection(collection); cleanupErr != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete collection %s: %w", collection, cleanupErr))
			} else {
				fmt.Print("✓ Deleted test account collection\n\n")
			}
		}
	}()

	// Setup test data
	fmt.Println("=== Setup: Creating Test Accounts ===")
	collectionCreated = true
	alice, err := client.Insert(collection, map[string]interface{}{
		"account_id": "ACC001",
		"name":       "Alice",
		"balance":    1000,
	})
	if err != nil {
		return fmt.Errorf("insert Alice: %w", err)
	}
	aliceID := ekodb.GetStringValue(alice["id"])
	if aliceID == "" {
		return errors.New("no ID returned for Alice")
	}
	fmt.Printf("Created Alice: $1000 - ID: %s\n", aliceID)

	bob, err := client.Insert(collection, map[string]interface{}{
		"account_id": "ACC002",
		"name":       "Bob",
		"balance":    500,
	})
	if err != nil {
		return fmt.Errorf("insert Bob: %w", err)
	}
	bobID := ekodb.GetStringValue(bob["id"])
	if bobID == "" {
		return errors.New("no ID returned for Bob")
	}
	fmt.Printf("Created Bob: $500 - ID: %s\n\n", bobID)

	// Example 1: Begin Transaction
	fmt.Println("=== Example 1: Begin Transaction ===")
	txID, err := client.BeginTransaction()
	if err != nil {
		return fmt.Errorf("begin transaction: %w", err)
	}
	activeTransactions = append(activeTransactions, txID)
	fmt.Printf("Transaction ID (server-default isolation): %s\n\n", txID)

	// Example 2: Operations within Transaction
	fmt.Println("=== Example 2: Operations within Transaction ===")
	if _, err := client.Update(collection, aliceID, ekodb.Record{"balance": 800}, ekodb.UpdateOptions{TransactionId: &txID}); err != nil {
		return fmt.Errorf("stage Alice update: %w", err)
	}
	if _, err := client.Update(collection, bobID, ekodb.Record{"balance": 700}, ekodb.UpdateOptions{TransactionId: &txID}); err != nil {
		return fmt.Errorf("stage Bob update: %w", err)
	}
	aliceInTx, err := client.FindByID(collection, aliceID, ekodb.FindByIDOptions{TransactionId: &txID})
	if err != nil {
		return fmt.Errorf("read Alice in transaction: %w", err)
	}
	bobInTx, err := client.FindByID(collection, bobID, ekodb.FindByIDOptions{TransactionId: &txID})
	if err != nil {
		return fmt.Errorf("read Bob in transaction: %w", err)
	}
	aliceBalance, aliceOK := ekodb.GetIntValue(aliceInTx["balance"])
	bobBalance, bobOK := ekodb.GetIntValue(bobInTx["balance"])
	if !aliceOK || !bobOK || aliceBalance != 800 || bobBalance != 700 {
		return fmt.Errorf("transactional balances were Alice=%v Bob=%v; expected 800/700", aliceInTx["balance"], bobInTx["balance"])
	}
	fmt.Println("Updated Alice: $1000 → $800")
	fmt.Print("Updated Bob: $500 → $700\n\n")

	// Example 3: Transaction Status
	fmt.Println("=== Example 3: Transaction Status ===")
	status, err := client.GetTransactionStatus(txID)
	if err != nil {
		return fmt.Errorf("get transaction status: %w", err)
	}
	fmt.Printf("Status: %v\n", status["state"])
	fmt.Printf("Operations: %v\n\n", status["operations_count"])
	operations, operationsOK := ekodb.GetIntValue(status["operations_count"])
	if status["state"] != "Active" || !operationsOK || operations != 2 {
		return fmt.Errorf("unexpected transaction status: %v", status)
	}

	// Example 4: Commit Transaction
	fmt.Println("=== Example 4: Commit Transaction ===")
	if err := client.CommitTransaction(txID); err != nil {
		return fmt.Errorf("commit transaction: %w", err)
	}
	activeTransactions = activeTransactions[:len(activeTransactions)-1]
	fmt.Print("✓ Transaction committed\n\n")
	committedAlice, err := client.FindByID(collection, aliceID)
	if err != nil {
		return fmt.Errorf("read committed Alice: %w", err)
	}
	committedBob, err := client.FindByID(collection, bobID)
	if err != nil {
		return fmt.Errorf("read committed Bob: %w", err)
	}
	aliceBalance, aliceOK = ekodb.GetIntValue(committedAlice["balance"])
	bobBalance, bobOK = ekodb.GetIntValue(committedBob["balance"])
	if !aliceOK || !bobOK || aliceBalance != 800 || bobBalance != 700 {
		return errors.New("committed balances were not 800/700")
	}
	fmt.Print("✓ Verified committed balances: Alice=$800, Bob=$700\n\n")

	// Example 5: Rollback Demo
	fmt.Println("=== Example 5: Rollback Demo ===")
	txID2, err := client.BeginTransaction("READ_COMMITTED")
	if err != nil {
		return fmt.Errorf("begin rollback transaction: %w", err)
	}
	activeTransactions = append(activeTransactions, txID2)
	fmt.Printf("New transaction: %s\n", txID2)
	if _, err := client.Update(collection, bobID, ekodb.Record{"balance": 600}, ekodb.UpdateOptions{TransactionId: &txID2}); err != nil {
		return fmt.Errorf("stage rollback update: %w", err)
	}
	bobInRollbackTx, err := client.FindByID(collection, bobID, ekodb.FindByIDOptions{TransactionId: &txID2})
	if err != nil {
		return fmt.Errorf("read Bob in rollback transaction: %w", err)
	}
	rollbackBalance, ok := ekodb.GetIntValue(bobInRollbackTx["balance"])
	if !ok || rollbackBalance != 600 {
		return fmt.Errorf("rollback transaction balance was %v; expected 600", bobInRollbackTx["balance"])
	}
	fmt.Println("Updated Bob: $700 → $600 (in transaction)")

	status2, err := client.GetTransactionStatus(txID2)
	if err != nil {
		return fmt.Errorf("get rollback transaction status: %w", err)
	}
	fmt.Printf("Status before rollback: %v\n", status2["state"])
	operations, operationsOK = ekodb.GetIntValue(status2["operations_count"])
	if status2["state"] != "Active" || !operationsOK || operations != 1 {
		return fmt.Errorf("unexpected rollback transaction status: %v", status2)
	}

	if err := client.RollbackTransaction(txID2); err != nil {
		return fmt.Errorf("rollback transaction: %w", err)
	}
	activeTransactions = activeTransactions[:len(activeTransactions)-1]
	fmt.Print("✓ Transaction rolled back\n\n")
	bobAfterRollback, err := client.FindByID(collection, bobID)
	if err != nil {
		return fmt.Errorf("read Bob after rollback: %w", err)
	}
	rollbackBalance, ok = ekodb.GetIntValue(bobAfterRollback["balance"])
	if !ok || rollbackBalance != 700 {
		return fmt.Errorf("Bob's post-rollback balance was %v; expected 700", bobAfterRollback["balance"])
	}
	fmt.Print("✓ Verified Bob remains $700 after rollback\n\n")

	return nil
}
