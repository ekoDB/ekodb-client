// KV Store & Wrapped Types Example for ekoDB Go Client
//
// Demonstrates: KV operations in scripts, wrapped type field builders

package main

import (
	"fmt"
	"log"
	"os"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}

	fmt.Println("🚀 ekoDB Go KV Store & Wrapped Types Example\n")
	fmt.Println("📋 Demonstrates:")
	fmt.Println("   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)")
	fmt.Println("   • KV store operations (get, set, delete, exists, query)")
	fmt.Println("   • KV operations within scripts")
	fmt.Println("   • Combined wrapped types + KV workflows\n")

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
		log.Fatalf("Failed to create client: %v", err)
	}

	var scriptIds []string

	// Wrapped Types Examples
	if err := wrappedTypesInsert(client); err != nil {
		log.Fatalf("Wrapped types insert failed: %v", err)
	}

	if id, err := wrappedTypesInScript(client); err != nil {
		log.Fatalf("Wrapped types script failed: %v", err)
	} else {
		scriptIds = append(scriptIds, id)
	}

	// KV Store Examples
	if err := kvBasicOperations(client); err != nil {
		log.Fatalf("KV basic operations failed: %v", err)
	}

	if id, err := kvScriptOperations(client); err != nil {
		log.Fatalf("KV script operations failed: %v", err)
	} else {
		scriptIds = append(scriptIds, id)
	}

	// Combined Example
	if id, err := combinedExample(client); err != nil {
		log.Fatalf("Combined example failed: %v", err)
	} else {
		scriptIds = append(scriptIds, id)
	}

	// Cleanup
	cleanup(client, scriptIds)

	fmt.Println("✅ All KV & Wrapped Types examples completed!")
	fmt.Println("\n💡 Key takeaways:")
	fmt.Println("   ✅ Use Field* helpers for type-safe wrapped values")
	fmt.Println("   ✅ FieldDecimal() preserves precision (no floating point errors)")
	fmt.Println("   ✅ KV store is great for caching and quick lookups")
	fmt.Println("   ✅ StageKv*() functions work within scripts")
}

// =============================================================================
// Wrapped Types Examples
// =============================================================================

func wrappedTypesInsert(client *ekodb.Client) error {
	fmt.Println("📝 Example 1: Inserting Records with Wrapped Types\n")

	// Insert an order with wrapped types for precise data handling
	order := map[string]interface{}{
		"order_id":        ekodb.FieldUUID("550e8400-e29b-41d4-a716-446655440000"),
		"total":           ekodb.FieldDecimal("1234.56"), // Precise decimal
		"created_at":      ekodb.FieldDateTime(time.Now()),
		"processing_time": ekodb.FieldDuration(3600000), // 1 hour in ms
		"quantity":        ekodb.FieldNumber(42),
		"tags":            ekodb.FieldSet([]string{"priority", "express"}),
		"embedding":       ekodb.FieldVector([]float64{0.1, 0.2, 0.3, 0.4}),
		"metadata":        ekodb.FieldObject(map[string]interface{}{"source": "web"}),
	}

	result, err := client.Insert("orders_example", order)
	if err != nil {
		return err
	}
	fmt.Printf("✅ Inserted order: %v\n", result["id"])

	// Insert products with wrapped types
	products := []map[string]interface{}{
		{
			"sku":        ekodb.FieldUUID("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"),
			"name":       "Laptop Pro",
			"price":      ekodb.FieldDecimal("1299.99"),
			"stock":      ekodb.FieldInteger(15),
			"rating":     ekodb.FieldFloat(4.8),
			"available":  ekodb.FieldBoolean(true),
			"categories": ekodb.FieldSet([]string{"electronics", "computers"}),
		},
		{
			"sku":        ekodb.FieldUUID("b1ffcd00-0d1c-5fg9-cc7e-7cc0ce491b22"),
			"name":       "Wireless Mouse",
			"price":      ekodb.FieldDecimal("29.99"),
			"stock":      ekodb.FieldInteger(150),
			"rating":     ekodb.FieldFloat(4.5),
			"available":  ekodb.FieldBoolean(true),
			"categories": ekodb.FieldSet([]string{"electronics", "accessories"}),
		},
	}

	for _, product := range products {
		if _, err := client.Insert("products_example", product); err != nil {
			return err
		}
	}
	fmt.Printf("✅ Inserted %d products with wrapped types\n\n", len(products))
	return nil
}

func wrappedTypesInScript(client *ekodb.Client) (string, error) {
	fmt.Println("📝 Example 2: Script with Wrapped Type Parameters\n")

	script := ekodb.Script{
		Label:       "create_order_with_types_go",
		Name:        "Create Order with Wrapped Types (Go)",
		Description: strPtr("Demonstrates wrapped types in script insert operations"),
		Version:     strPtr("1.0"),
		Parameters: map[string]ekodb.ParameterDefinition{
			"order_total": {
				Required: true,
			},
			"order_id": {
				Required: true,
			},
			"timestamp": {
				Required:    true,
				Description: "Current UTC timestamp (ISO 8601)",
			},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageInsert("script_orders", map[string]interface{}{
				"order_id":   "{{order_id}}",
				"total":      map[string]interface{}{"type": "Decimal", "value": "{{order_total}}"},
				"created_at": "{{timestamp}}",
				"status":     "pending",
			}, false, nil),
		},
		Tags: []string{"orders", "wrapped-types"},
	}

	id, err := client.SaveScript(script)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Script saved: %s\n", id)

	result, err := client.CallScript("create_order_with_types_go", map[string]interface{}{
		"order_total": "599.99",
		"order_id":    fmt.Sprintf("order_%d", time.Now().UnixNano()),
		"timestamp":   time.Now().Format(time.RFC3339),
	})
	if err != nil {
		return id, err
	}
	fmt.Println("📊 Created order via script")
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return id, nil
}

// =============================================================================
// KV Store Examples
// =============================================================================

func kvBasicOperations(client *ekodb.Client) error {
	fmt.Println("📝 Example 3: Basic KV Store Operations\n")

	// Set a simple value
	if err := client.KVSet("user:session:123", map[string]interface{}{
		"userId": "user_abc",
		"role":   "admin",
	}); err != nil {
		return err
	}
	fmt.Println("✅ Set session data")

	// Get the value back
	session, err := client.KVGet("user:session:123")
	if err != nil {
		return err
	}
	fmt.Printf("📊 Retrieved session: %v\n", session)

	// Check if key exists
	exists, err := client.KVExists("user:session:123")
	if err != nil {
		return err
	}
	fmt.Printf("🔍 Key exists: %v\n", exists)

	// Set with TTL (1 hour)
	if err := client.KVSet("cache:product:456", map[string]interface{}{
		"name":  "Cached Product",
		"price": 99.99,
	}); err != nil {
		return err
	}
	fmt.Println("✅ Set cached data with 1 hour TTL")

	// Delete a key
	if err := client.KVDelete("user:session:123"); err != nil {
		return err
	}
	fmt.Println("🗑️  Deleted session")

	return nil
}

func kvScriptOperations(client *ekodb.Client) (string, error) {
	fmt.Println("📝 Example 4: KV Operations in Scripts\n")

	ttl := int64(3600)
	script := ekodb.Script{
		Label:       "cached_product_lookup_go",
		Name:        "Cached Product Lookup (Go)",
		Description: strPtr("Uses KV store for caching within a script"),
		Version:     strPtr("1.0"),
		Parameters: map[string]ekodb.ParameterDefinition{
			"product_key":  {Required: true},
			"product_data": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageKvSet("{{product_key}}", "{{product_data}}", &ttl),
			ekodb.StageKvGet("{{product_key}}"),
		},
		Tags: []string{"kv", "caching"},
	}

	id, err := client.SaveScript(script)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Script saved: %s\n", id)

	result, err := client.CallScript("cached_product_lookup_go", map[string]interface{}{
		"product_key":  "product:cache:789",
		"product_data": map[string]interface{}{"name": "Test Product", "price": 49.99},
	})
	if err != nil {
		return id, err
	}
	fmt.Println("📊 Cached and retrieved product data")
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return id, nil
}

// =============================================================================
// Combined Example
// =============================================================================

func combinedExample(client *ekodb.Client) (string, error) {
	fmt.Println("📝 Example 5: Combined Wrapped Types + KV Script\n")

	ttl := int64(86400) // 24 hours
	script := ekodb.Script{
		Label:       "process_order_with_cache_go",
		Name:        "Process Order with Cache (Go)",
		Description: strPtr("Demonstrates combined KV and wrapped type usage"),
		Version:     strPtr("1.0"),
		Parameters: map[string]ekodb.ParameterDefinition{
			"order_id":  {Required: true},
			"total":     {Required: true},
			"timestamp": {Required: true, Description: "Current UTC timestamp (ISO 8601)"},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageKvSet("order:status:{{order_id}}", map[string]interface{}{
				"status":     "processing",
				"updated_at": "{{timestamp}}",
			}, &ttl),
			ekodb.StageInsert("processed_orders", map[string]interface{}{
				"order_id":   "{{order_id}}",
				"total":      map[string]interface{}{"type": "Decimal", "value": "{{total}}"},
				"created_at": "{{timestamp}}",
				"status":     "processing",
			}, false, nil),
			ekodb.StageKvGet("order:status:{{order_id}}"),
		},
		Tags: []string{"orders", "kv", "wrapped-types"},
	}

	id, err := client.SaveScript(script)
	if err != nil {
		return "", err
	}
	fmt.Printf("✅ Script saved: %s\n", id)

	result, err := client.CallScript("process_order_with_cache_go", map[string]interface{}{
		"order_id":  "c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6",
		"total":     "299.99",
		"timestamp": time.Now().Format(time.RFC3339),
	})
	if err != nil {
		return id, err
	}
	fmt.Println("📊 Processed order with caching")
	fmt.Printf("⏱️  Stages executed: %d\n", result.Stats.StagesExecuted)
	fmt.Printf("⏱️  Execution time: %dms\n\n", result.Stats.ExecutionTimeMs)

	return id, nil
}

// =============================================================================
// Cleanup
// =============================================================================

func cleanup(client *ekodb.Client, scriptIds []string) {
	fmt.Println("🧹 Cleaning up...")

	for _, id := range scriptIds {
		_ = client.DeleteScript(id)
	}

	_ = client.DeleteCollection("orders_example")
	_ = client.DeleteCollection("products_example")
	_ = client.DeleteCollection("script_orders")
	_ = client.DeleteCollection("processed_orders")

	_ = client.KVDelete("cache:product:456")
	_ = client.KVDelete("product:cache:789")
	_ = client.KVDelete("order:status:c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6")

	fmt.Println("✅ Cleanup complete\n")
}

func strPtr(s string) *string {
	return &s
}
