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

	fmt.Println("=== Schema Management Examples ===\n")

	// Use unique collection names
	usersCollection := "schema_users_client_go"
	productsCollection := "schema_products_client_go"
	documentsCollection := "schema_documents_client_go"
	employeesCollection := "schema_employees_client_go"

	// Cleanup: Delete collections if they exist from previous runs
	client.DeleteCollection(usersCollection)
	client.DeleteCollection(productsCollection)
	client.DeleteCollection(documentsCollection)
	client.DeleteCollection(employeesCollection)

	// Example 1: Create a simple user schema
	fmt.Println("1. Creating user schema with basic fields:")
	userSchema := ekodb.NewSchemaBuilder().
		AddField("name", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			Pattern("^[a-zA-Z ]+$").
			Build()).
		AddField("email", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			Unique().
			Pattern("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$").
			Build()).
		AddField("age", ekodb.NewFieldTypeSchemaBuilder("Integer").
			Range(0, 150).
			Build()).
		AddField("status", ekodb.NewFieldTypeSchemaBuilder("String").
			DefaultValue("active").
			Enums([]interface{}{"active", "inactive", "suspended"}).
			Build()).
		Build()

	if err := client.CreateCollection(usersCollection, userSchema); err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Println("✅ User schema created\n")
	}

	// Example 2: Schema with text index for full-text search
	fmt.Println("2. Creating product schema with text index:")
	productSchema := ekodb.NewSchemaBuilder().
		AddField("name", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			TextIndex("english").
			Build()).
		AddField("description", ekodb.NewFieldTypeSchemaBuilder("String").
			TextIndex("english").
			Build()).
		AddField("price", ekodb.NewFieldTypeSchemaBuilder("Float").
			Required().
			Range(0, nil).
			Build()).
		AddField("category", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			BTreeIndex().
			Build()).
		AddField("sku", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			Unique().
			HashIndex().
			Build()).
		Build()

	if err := client.CreateCollection(productsCollection, productSchema); err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Println("✅ Product schema with indexes created\n")
	}

	// Example 3: Schema with vector index for semantic search
	fmt.Println("3. Creating document schema with vector index:")
	documentSchema := ekodb.NewSchemaBuilder().
		AddField("title", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			TextIndex("english").
			Build()).
		AddField("content", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			TextIndex("english").
			Build()).
		AddField("embedding", ekodb.NewFieldTypeSchemaBuilder("Array").
			VectorIndex(ekodb.VectorIndexHNSW, ekodb.DistanceMetricCosine, 16, 200).
			Build()).
		AddField("tags", ekodb.NewFieldTypeSchemaBuilder("Array").
			Build()).
		AddField("created_at", ekodb.NewFieldTypeSchemaBuilder("DateTime").
			Build()).
		Build()

	if err := client.CreateCollection(documentsCollection, documentSchema); err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Println("✅ Document schema with vector index created\n")
	}

	// Example 4: Get collection schema
	fmt.Println("4. Retrieving collection schema:")
	schema, err := client.GetSchema(usersCollection)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Schema fields: %d fields\n", len(schema.Fields))
		if schema.Version != nil {
			fmt.Printf("Schema version: %d\n", *schema.Version)
		}
	}
	fmt.Println()

	// Example 5: Get collection metadata
	fmt.Println("5. Retrieving collection metadata:")
	metadata, err := client.GetCollection(usersCollection)
	if err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Printf("Collection has %d fields\n", len(metadata.Collection.Fields))
	}
	fmt.Println()

	// Example 6: Complex schema with all constraint types
	fmt.Println("6. Creating employee schema with all constraint types:")
	employeeSchema := ekodb.NewSchemaBuilder().
		AddField("employee_id", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			Unique().
			Pattern("^EMP-[0-9]{6}$").
			HashIndex().
			Build()).
		AddField("first_name", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			Build()).
		AddField("last_name", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			Build()).
		AddField("department", ekodb.NewFieldTypeSchemaBuilder("String").
			Required().
			Enums([]interface{}{"Engineering", "Sales", "Marketing", "HR", "Finance"}).
			BTreeIndex().
			Build()).
		AddField("salary", ekodb.NewFieldTypeSchemaBuilder("Float").
			Required().
			Range(30000, 500000).
			Build()).
		AddField("hire_date", ekodb.NewFieldTypeSchemaBuilder("DateTime").
			Required().
			Build()).
		AddField("is_active", ekodb.NewFieldTypeSchemaBuilder("Boolean").
			DefaultValue(true).
			Build()).
		AddField("bio", ekodb.NewFieldTypeSchemaBuilder("String").
			TextIndex("english").
			Build()).
		BypassRipple(false).
		Build()

	if err := client.CreateCollection(employeesCollection, employeeSchema); err != nil {
		log.Printf("Error: %v", err)
	} else {
		fmt.Println("✅ Employee schema with all constraints created\n")
	}

	fmt.Println("✅ Schema management examples completed!")
}
