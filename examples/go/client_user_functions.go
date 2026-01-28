// User Functions API Example - Using ekodb-client-go library
//
// This example demonstrates CRUD operations for User Functions.
// User Functions are reusable sequences of Functions that can be called by Scripts.

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

	// Example 1: Create a User Function
	fmt.Println("\n=== Create User Function ===")
	description := "Fetches all users and filters by active status"
	version := "1.0.0"
	userFunc := ekodb.UserFunction{
		Label:       "get_active_users",
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
	if err != nil {
		log.Printf("SaveUserFunction error: %v", err)
	} else {
		fmt.Printf("Created user function with ID: %s\n", funcID)
	}

	// Example 2: Get User Function by label
	fmt.Println("\n=== Get User Function ===")
	retrieved, err := client.GetUserFunction("get_active_users")
	if err != nil {
		log.Printf("GetUserFunction error: %v", err)
	} else {
		fmt.Printf("Retrieved: %s - %s\n", retrieved.Label, retrieved.Name)
		if retrieved.Description != nil {
			fmt.Printf("Description: %s\n", *retrieved.Description)
		}
	}

	// Example 3: List all User Functions
	fmt.Println("\n=== List All User Functions ===")
	allFuncs, err := client.ListUserFunctions(nil)
	if err != nil {
		log.Printf("ListUserFunctions error: %v", err)
	} else {
		fmt.Printf("Found %d user functions:\n", len(allFuncs))
		for _, f := range allFuncs {
			fmt.Printf("  - %s: %s\n", f.Label, f.Name)
		}
	}

	// Example 4: List User Functions by tag
	fmt.Println("\n=== List User Functions by Tag ===")
	taggedFuncs, err := client.ListUserFunctions([]string{"users"})
	if err != nil {
		log.Printf("ListUserFunctions (with tags) error: %v", err)
	} else {
		fmt.Printf("Found %d user functions with 'users' tag:\n", len(taggedFuncs))
		for _, f := range taggedFuncs {
			fmt.Printf("  - %s\n", f.Label)
		}
	}

	// Example 5: Update User Function
	fmt.Println("\n=== Update User Function ===")
	updatedDesc := "Fetches all users - updated description"
	userFunc.Description = &updatedDesc
	err = client.UpdateUserFunction("get_active_users", userFunc)
	if err != nil {
		log.Printf("UpdateUserFunction error: %v", err)
	} else {
		fmt.Println("User function updated successfully")
	}

	// Example 6: Delete User Function
	fmt.Println("\n=== Delete User Function ===")
	err = client.DeleteUserFunction("get_active_users")
	if err != nil {
		log.Printf("DeleteUserFunction error: %v", err)
	} else {
		fmt.Println("User function deleted successfully")
	}

	fmt.Println("\n✓ User Functions API example complete")
}
