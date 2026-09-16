// User Functions API Example - Using ekodb-client-go library
//
// This example demonstrates CRUD operations for User Functions.
// User Functions are reusable sequences of Functions that can be called by functions.

package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

// isAlreadyExists reports whether err is a 409 Conflict (the function label
// already exists) returned by the ekoDB functions API.
func isAlreadyExists(err error) bool {
	var httpErr *ekodb.HTTPError
	return errors.As(err, &httpErr) && httpErr.StatusCode == 409
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
		apiKey = "a-test-api-key-from-ekodb"
	}

	// Create ekoDB client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return err
	}
	fmt.Println("✓ Client created")
	label := "get_active_users_go"
	owned := false
	defer func() {
		if owned {
			runErr = errors.Join(runErr, client.DeleteUserFunction(label))
		}
	}()

	// Example 1: Create a User Function
	fmt.Println("\n=== Create User Function ===")
	description := "Fetches all users and filters by active status"
	version := "1.0.0"
	userFunc := ekodb.UserFunction{
		Label:       label,
		Name:        "Get Active Users",
		Description: &description,
		Version:     &version,
		Parameters: map[string]ekodb.ParameterDefinition{
			"collection": {
				Required:    true,
				Description: "Collection to query",
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindAll("{{collection}}"),
		},
		Tags: []string{"users", "query"},
	}

	funcID, err := client.SaveUserFunction(userFunc)
	if isAlreadyExists(err) {
		// Idempotent: the label already exists, so update it instead (PUT by label).
		if uerr := client.UpdateUserFunction(userFunc.Label, userFunc); uerr != nil {
			return fmt.Errorf("update existing user function: %w", uerr)
		} else {
			owned = true
			fmt.Printf("User function '%s' already existed — updated instead\n", userFunc.Label)
		}
	} else if err != nil {
		return fmt.Errorf("save user function: %w", err)
	} else {
		owned = true
		fmt.Printf("Created user function with ID: %s\n", funcID)
	}

	// Example 2: Get User Function by label
	fmt.Println("\n=== Get User Function ===")
	retrieved, err := client.GetUserFunction(label)
	if err != nil {
		return fmt.Errorf("get user function: %w", err)
	}
	fmt.Printf("Retrieved: %s - %s\n", retrieved.Label, retrieved.Name)
	if retrieved.Description != nil {
		fmt.Printf("Description: %s\n", *retrieved.Description)
	}

	// Example 3: List all User Functions
	fmt.Println("\n=== List All User Functions ===")
	allFuncs, err := client.ListUserFunctions(nil)
	if err != nil {
		return fmt.Errorf("list user functions: %w", err)
	}
	fmt.Printf("Found %d user functions:\n", len(allFuncs))
	for _, f := range allFuncs {
		fmt.Printf("  - %s: %s\n", f.Label, f.Name)
	}

	// Example 4: List User Functions by tag
	fmt.Println("\n=== List User Functions by Tag ===")
	taggedFuncs, err := client.ListUserFunctions([]string{"users"})
	if err != nil {
		return fmt.Errorf("list tagged user functions: %w", err)
	}
	fmt.Printf("Found %d user functions with 'users' tag:\n", len(taggedFuncs))
	for _, f := range taggedFuncs {
		fmt.Printf("  - %s\n", f.Label)
	}

	// Example 5: Update User Function
	fmt.Println("\n=== Update User Function ===")
	updatedDesc := "Fetches all users - updated description"
	userFunc.Description = &updatedDesc
	err = client.UpdateUserFunction(label, userFunc)
	if err != nil {
		return fmt.Errorf("update user function: %w", err)
	}
	fmt.Println("User function updated successfully")

	// Example 6: Delete User Function
	fmt.Println("\n=== Delete User Function ===")
	err = client.DeleteUserFunction(label)
	if err != nil {
		return fmt.Errorf("delete user function: %w", err)
	}
	owned = false
	fmt.Println("User function deleted successfully")

	fmt.Println("\n✓ User Functions API example complete")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
