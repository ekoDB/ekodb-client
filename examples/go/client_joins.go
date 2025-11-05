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
		log.Printf("Warning: .env file not found")
	}

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}

	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}

	// Create client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	fmt.Println("=== Join Operations Examples ===\n")

	// Use unique collection names
	usersCollection := "joins_users_client_go"
	departmentsCollection := "joins_departments_client_go"
	profilesCollection := "joins_profiles_client_go"
	ordersCollection := "joins_orders_client_go"

	// Cleanup any existing test collections
	_ = client.DeleteCollection(usersCollection)
	_ = client.DeleteCollection(departmentsCollection)
	_ = client.DeleteCollection(profilesCollection)
	_ = client.DeleteCollection(ordersCollection)

	// Setup: Create sample data
	fmt.Println("Setting up sample data...")

	// Create departments first (need their IDs for users)
	dept1, _ := client.Insert(departmentsCollection, ekodb.Record{
		"name":     "Engineering",
		"location": "Building A",
	})

	dept2, _ := client.Insert(departmentsCollection, ekodb.Record{
		"name":     "Sales",
		"location": "Building B",
	})

	// Create users with actual department IDs
	user1, _ := client.Insert(usersCollection, ekodb.Record{
		"name":          "Alice Johnson",
		"email":         "alice@example.com",
		"department_id": dept1["id"],
	})

	user2, _ := client.Insert(usersCollection, ekodb.Record{
		"name":          "Bob Smith",
		"email":         "bob@example.com",
		"department_id": dept2["id"],
	})

	// Create orders
	client.Insert(ordersCollection, ekodb.Record{
		"user_id": user1["id"],
		"product": "Laptop",
		"amount":  1200,
		"status":  "completed",
	})

	client.Insert(ordersCollection, ekodb.Record{
		"user_id": user1["id"],
		"product": "Mouse",
		"amount":  25,
		"status":  "completed",
	})

	client.Insert(ordersCollection, ekodb.Record{
		"user_id": user2["id"],
		"product": "Keyboard",
		"amount":  75,
		"status":  "pending",
	})

	fmt.Println("✅ Sample data created\n")

	// Example 1: Simple single collection join
	fmt.Println("1. Single collection join (users with departments):")
	join1 := ekodb.NewSingleJoin(departmentsCollection, "department_id", "id", "department")

	query1 := ekodb.NewQueryBuilder().
		Join(join1.ToMap()).
		Limit(10).
		Build()

	results1, err := client.Find(usersCollection, query1)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d users with department data\n", len(results1))
		for _, user := range results1 {
			name := getStringValue(user, "name")
			// Join returns an array, get first element
			deptName := getJoinedStringValue(user, "department", "name")
			fmt.Printf("  - %s: %s\n", name, deptName)
		}
	}
	fmt.Println()

	// Example 2: Join with filtering
	fmt.Println("2. Join with filtering:")
	join2 := ekodb.NewSingleJoin(departmentsCollection, "department_id", "id", "department")

	query2 := ekodb.NewQueryBuilder().
		Eq("department_id", dept1["id"]).
		Join(join2.ToMap()).
		Build()

	results2, err := client.Find(usersCollection, query2)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d users in Engineering\n", len(results2))
		for _, user := range results2 {
			name := getStringValue(user, "name")
			// Join returns an array, get first element
			location := getJoinedStringValue(user, "department", "location")
			fmt.Printf("  - %s: %s\n", name, location)
		}
	}
	fmt.Println()

	// Example 3: Join with user profiles
	fmt.Println("3. Join with user profiles:")

	// Create profiles
	client.Insert(profilesCollection, ekodb.Record{
		"user_id": user1["id"],
		"bio":     "Senior Software Engineer",
		"skills":  []string{"JavaScript", "TypeScript", "React"},
	})

	client.Insert(profilesCollection, ekodb.Record{
		"user_id": user2["id"],
		"bio":     "Sales Manager",
		"skills":  []string{"Negotiation", "CRM", "Communication"},
	})

	join3 := ekodb.NewSingleJoin(profilesCollection, "id", "user_id", "profile")

	query3 := ekodb.NewQueryBuilder().
		Join(join3.ToMap()).
		Limit(10).
		Build()

	results3, err := client.Find(usersCollection, query3)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d users with profile data\n", len(results3))
		for _, user := range results3 {
			name := getStringValue(user, "name")
			// Join returns an array, get first element
			bio := getJoinedStringValue(user, "profile", "bio")
			fmt.Printf("  - %s: %s\n", name, bio)
		}
	}
	fmt.Println()

	// Example 4: Join orders with user data
	fmt.Println("4. Join orders with user data:")
	join4 := ekodb.NewSingleJoin(usersCollection, "user_id", "id", "user")

	query4 := ekodb.NewQueryBuilder().
		Eq("status", "completed").
		Join(join4.ToMap()).
		Build()

	results4, err := client.Find(ordersCollection, query4)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d completed orders\n", len(results4))
		for _, order := range results4 {
			product := getStringValue(order, "product")
			amount := getFloatValue(order, "amount")
			// Join returns an array, get first element
			userName := getJoinedStringValue(order, "user", "name")
			fmt.Printf("  - %s ($%.0f) by %s\n", product, amount, userName)
		}
	}
	fmt.Println()

	// Example 5: Complex join with multiple conditions
	fmt.Println("5. Complex join with multiple conditions:")
	join5 := ekodb.NewSingleJoin(departmentsCollection, "department_id", "id", "department")

	query5 := ekodb.NewQueryBuilder().
		Contains("email", "@example.com").
		Join(join5.ToMap()).
		SortAscending("name").
		Limit(5).
		Build()

	results5, err := client.Find(usersCollection, query5)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Found %d users with example.com emails\n", len(results5))
		for _, user := range results5 {
			name := getStringValue(user, "name")
			email := getStringValue(user, "email")
			// Join returns an array, get first element
			location := getJoinedStringValue(user, "department", "location")
			fmt.Printf("  - %s (%s): %s\n", name, email, location)
		}
	}
	fmt.Println()

	// Cleanup
	fmt.Println("=== Cleanup ===")
	client.DeleteCollection(usersCollection)
	client.DeleteCollection(departmentsCollection)
	client.DeleteCollection(profilesCollection)
	client.DeleteCollection(ordersCollection)
	fmt.Println("✅ Deleted test collections")

	fmt.Println("\n✅ Join operations examples completed!")
}

// Helper function to extract string values from FieldType wrapped values
func getStringValue(record ekodb.Record, field string) string {
	if val, ok := record[field]; ok {
		// Check if it's a FieldType wrapper
		if m, ok := val.(map[string]interface{}); ok {
			if value, ok := m["value"]; ok {
				if str, ok := value.(string); ok {
					return str
				}
			}
		}
		// Direct string value
		if str, ok := val.(string); ok {
			return str
		}
	}
	return "Unknown"
}

// Helper function to extract float values from FieldType wrapped values
func getFloatValue(record ekodb.Record, field string) float64 {
	if val, ok := record[field]; ok {
		// Check if it's a FieldType wrapper
		if m, ok := val.(map[string]interface{}); ok {
			if value, ok := m["value"]; ok {
				if num, ok := value.(float64); ok {
					return num
				}
			}
		}
		// Direct numeric value
		if num, ok := val.(float64); ok {
			return num
		}
		if num, ok := val.(int); ok {
			return float64(num)
		}
	}
	return 0
}

// Helper function to extract string values from joined arrays
func getJoinedStringValue(record ekodb.Record, joinField, field string) string {
	if val, ok := record[joinField]; ok {
		// Join returns an array
		if arr, ok := val.([]interface{}); ok {
			if len(arr) > 0 {
				if obj, ok := arr[0].(map[string]interface{}); ok {
					return getStringValue(obj, field)
				}
			}
		}
	}
	return "Unknown"
}
