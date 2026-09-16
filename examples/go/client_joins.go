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

func run() (runErr error) {
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
		return fmt.Errorf("create client: %w", err)
	}

	fmt.Print("=== Join Operations Examples ===\n\n")

	// Use unique collection names
	usersCollection := "joins_users_client_go"
	departmentsCollection := "joins_departments_client_go"
	profilesCollection := "joins_profiles_client_go"
	ordersCollection := "joins_orders_client_go"

	// Cleanup any existing test collections
	collections := []string{usersCollection, departmentsCollection, profilesCollection, ordersCollection}
	for _, collection := range collections {
		if err := client.DeleteCollection(collection); err != nil && !joinNotFound(err) {
			return fmt.Errorf("initial cleanup %s: %w", collection, err)
		}
	}
	defer func() {
		fmt.Println("=== Cleanup ===")
		var cleanupErrors []error
		for _, collection := range collections {
			if err := client.DeleteCollection(collection); err != nil && !joinNotFound(err) {
				cleanupErrors = append(cleanupErrors, fmt.Errorf("delete %s: %w", collection, err))
			}
		}
		if cleanupErr := errors.Join(cleanupErrors...); cleanupErr != nil {
			runErr = errors.Join(runErr, cleanupErr)
		} else {
			fmt.Println("✅ Deleted test collections")
		}
	}()

	// Setup: Create sample data
	fmt.Println("Setting up sample data...")

	// Create departments first (need their IDs for users)
	dept1, err := client.Insert(departmentsCollection, ekodb.Record{
		"name":     "Engineering",
		"location": "Building A",
	})
	if err != nil {
		return fmt.Errorf("insert Engineering department: %w", err)
	}

	dept2, err := client.Insert(departmentsCollection, ekodb.Record{
		"name":     "Sales",
		"location": "Building B",
	})
	if err != nil {
		return fmt.Errorf("insert Sales department: %w", err)
	}

	// Create users with actual department IDs
	user1, err := client.Insert(usersCollection, ekodb.Record{
		"name":          "Alice Johnson",
		"email":         "alice@example.com",
		"department_id": dept1["id"],
	})
	if err != nil {
		return fmt.Errorf("insert Alice: %w", err)
	}

	user2, err := client.Insert(usersCollection, ekodb.Record{
		"name":          "Bob Smith",
		"email":         "bob@example.com",
		"department_id": dept2["id"],
	})
	if err != nil {
		return fmt.Errorf("insert Bob: %w", err)
	}

	// Create orders
	if _, err := client.Insert(ordersCollection, ekodb.Record{
		"user_id": user1["id"],
		"product": "Laptop",
		"amount":  1200,
		"status":  "completed",
	}); err != nil {
		return fmt.Errorf("insert Laptop order: %w", err)
	}

	if _, err := client.Insert(ordersCollection, ekodb.Record{
		"user_id": user1["id"],
		"product": "Mouse",
		"amount":  25,
		"status":  "completed",
	}); err != nil {
		return fmt.Errorf("insert Mouse order: %w", err)
	}

	if _, err := client.Insert(ordersCollection, ekodb.Record{
		"user_id": user2["id"],
		"product": "Keyboard",
		"amount":  75,
		"status":  "pending",
	}); err != nil {
		return fmt.Errorf("insert Keyboard order: %w", err)
	}

	fmt.Print("✅ Sample data created\n\n")

	// Example 1: Simple single collection join
	fmt.Println("1. Single collection join (users with departments):")
	join1 := ekodb.NewSingleJoin(departmentsCollection, "department_id", "id", "department")

	query1 := ekodb.NewQueryBuilder().
		Join(join1.ToMap()).
		Limit(10).
		Build()

	results1, err := client.Find(usersCollection, query1)
	if err != nil {
		return fmt.Errorf("users/departments join: %w", err)
	}
	if len(results1) != 2 {
		return fmt.Errorf("users/departments join returned %d rows, expected 2", len(results1))
	}
	expectedDepartments := map[string]string{"Alice Johnson": "Engineering", "Bob Smith": "Sales"}
	fmt.Printf("Found %d users with department data\n", len(results1))
	for _, user := range results1 {
		name := getStringValue(user, "name")
		deptName := getJoinedStringValue(user, "department", "name")
		if expectedDepartments[name] != deptName {
			return fmt.Errorf("department join for %q was %q, expected %q", name, deptName, expectedDepartments[name])
		}
		fmt.Printf("  - %s: %s\n", name, deptName)
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
		return fmt.Errorf("filtered department join: %w", err)
	}
	if len(results2) != 1 || getStringValue(results2[0], "name") != "Alice Johnson" || getJoinedStringValue(results2[0], "department", "location") != "Building A" {
		return fmt.Errorf("filtered department join returned unexpected rows: %v", results2)
	}
	fmt.Printf("Found %d users in Engineering\n", len(results2))
	fmt.Printf("  - %s: %s\n", getStringValue(results2[0], "name"), getJoinedStringValue(results2[0], "department", "location"))
	fmt.Println()

	// Example 3: Join with user profiles
	fmt.Println("3. Join with user profiles:")

	// Create profiles
	if _, err := client.Insert(profilesCollection, ekodb.Record{
		"user_id": user1["id"],
		"bio":     "Senior Software Engineer",
		"skills":  []string{"JavaScript", "TypeScript", "React"},
	}); err != nil {
		return fmt.Errorf("insert Alice profile: %w", err)
	}

	if _, err := client.Insert(profilesCollection, ekodb.Record{
		"user_id": user2["id"],
		"bio":     "Sales Manager",
		"skills":  []string{"Negotiation", "CRM", "Communication"},
	}); err != nil {
		return fmt.Errorf("insert Bob profile: %w", err)
	}

	join3 := ekodb.NewSingleJoin(profilesCollection, "id", "user_id", "profile")

	query3 := ekodb.NewQueryBuilder().
		Join(join3.ToMap()).
		Limit(10).
		Build()

	results3, err := client.Find(usersCollection, query3)
	if err != nil {
		return fmt.Errorf("profiles join: %w", err)
	}
	if len(results3) != 2 {
		return fmt.Errorf("profiles join returned %d rows, expected 2", len(results3))
	}
	fmt.Printf("Found %d users with profile data\n", len(results3))
	for _, user := range results3 {
		name := getStringValue(user, "name")
		bio := getJoinedStringValue(user, "profile", "bio")
		if bio == "Unknown" || bio == "" {
			return fmt.Errorf("profile join for %q returned no bio", name)
		}
		fmt.Printf("  - %s: %s\n", name, bio)
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
		return fmt.Errorf("orders/users join: %w", err)
	}
	if len(results4) != 2 {
		return fmt.Errorf("orders/users join returned %d rows, expected 2", len(results4))
	}
	fmt.Printf("Found %d completed orders\n", len(results4))
	for _, order := range results4 {
		product := getStringValue(order, "product")
		amount := ekodb.GetFloatValue(order["amount"])
		expectedAmount := map[string]float64{"Laptop": 1200, "Mouse": 25}
		if amount != expectedAmount[product] || expectedAmount[product] == 0 {
			return fmt.Errorf("unexpected completed order %q amount: %v", product, amount)
		}
		userName := getJoinedStringValue(order, "user", "name")
		if userName != "Alice Johnson" {
			return fmt.Errorf("order %q joined to %q, expected Alice Johnson", product, userName)
		}
		fmt.Printf("  - %s ($%.0f) by %s\n", product, amount, userName)
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
		return fmt.Errorf("complex join: %w", err)
	}
	if len(results5) != 2 {
		return fmt.Errorf("complex join returned %d rows, expected 2", len(results5))
	}
	fmt.Printf("Found %d users with example.com emails\n", len(results5))
	for _, user := range results5 {
		name := getStringValue(user, "name")
		email := getStringValue(user, "email")
		location := getJoinedStringValue(user, "department", "location")
		if !strings.HasSuffix(email, "@example.com") || location == "Unknown" {
			return fmt.Errorf("complex join returned unexpected user: %v", user)
		}
		fmt.Printf("  - %s (%s): %s\n", name, email, location)
	}
	fmt.Println()

	return nil
}

func joinNotFound(err error) bool {
	var httpErr *ekodb.HTTPError
	return (errors.As(err, &httpErr) && httpErr.StatusCode == 404) || strings.Contains(strings.ToLower(err.Error()), "not found")
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
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
