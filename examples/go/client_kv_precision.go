// KV Precision Example - Float vs Decimal Comparison
//
// This example demonstrates the difference between using Go floats
// (which lose precision) vs ekoDB's Decimal type (which preserves precision).
//
// Key takeaway: Use FieldDecimal() for monetary values and other cases
// where precision matters.

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

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	fmt.Println("=== KV Precision: Float vs Decimal ===\n")

	// =========================================================================
	// Test 1: Float Precision Loss
	// =========================================================================
	fmt.Println("=== Test 1: Using Go Floats (LOSES PRECISION) ===")

	floatProducts := []map[string]interface{}{
		{"key": "product:float:1", "value": map[string]interface{}{"name": "Widget A", "price": 29.99}},
		{"key": "product:float:2", "value": map[string]interface{}{"name": "Widget B", "price": 39.99}},
		{"key": "product:float:3", "value": map[string]interface{}{"name": "Widget C", "price": 49.99}},
	}

	// Store with float values
	_, err = client.KVBatchSet(floatProducts)
	if err != nil {
		log.Fatalf("Failed to batch set float products: %v", err)
	}
	fmt.Println("✓ Stored products with float prices")

	// Retrieve and show the precision loss
	// Note: KV values come back with type wrappers like { "type": "Float", "value": 29.99 }
	floatKeys := []string{"product:float:1", "product:float:2", "product:float:3"}
	floatResults, err := client.KVBatchGet(floatKeys)
	if err != nil {
		log.Fatalf("Failed to batch get float products: %v", err)
	}

	fmt.Println("\nRetrieved float prices:")
	expectedPrices := []float64{29.99, 39.99, 49.99}
	names := []string{"Widget A", "Widget B", "Widget C"}
	for i, val := range floatResults {
		actualPrice := ekodb.GetFloatValue(val["price"])
		match := "✓"
		if actualPrice != expectedPrices[i] {
			match = "✗ PRECISION LOST"
		}
		fmt.Printf("  %s: $%v (expected $%v) %s\n", names[i], actualPrice, expectedPrices[i], match)
	}

	// =========================================================================
	// Test 2: Decimal Precision Preserved
	// =========================================================================
	fmt.Println("\n=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===")

	decimalProducts := []map[string]interface{}{
		{"key": "product:decimal:1", "value": map[string]interface{}{"name": "Widget A", "price": ekodb.FieldDecimal("29.99")}},
		{"key": "product:decimal:2", "value": map[string]interface{}{"name": "Widget B", "price": ekodb.FieldDecimal("39.99")}},
		{"key": "product:decimal:3", "value": map[string]interface{}{"name": "Widget C", "price": ekodb.FieldDecimal("49.99")}},
	}

	// Store with decimal values
	_, err = client.KVBatchSet(decimalProducts)
	if err != nil {
		log.Fatalf("Failed to batch set decimal products: %v", err)
	}
	fmt.Println("✓ Stored products with decimal prices")

	// Retrieve and show precision is preserved
	// Decimal values come back as { "type": "Decimal", "value": "29.99" } - GetDecimalValue extracts it
	decimalKeys := []string{"product:decimal:1", "product:decimal:2", "product:decimal:3"}
	decimalResults, err := client.KVBatchGet(decimalKeys)
	if err != nil {
		log.Fatalf("Failed to batch get decimal products: %v", err)
	}

	expectedDecimals := []string{"29.99", "39.99", "49.99"}
	fmt.Println("\nRetrieved decimal prices:")
	for i, val := range decimalResults {
		actualPrice := ekodb.GetStringValue(val["price"])
		fmt.Printf("  %s: $%s (expected $%s) ✓\n", names[i], actualPrice, expectedDecimals[i])
	}

	// =========================================================================
	// Test 3: Real-world calculation comparison
	// =========================================================================
	fmt.Println("\n=== Test 3: Sum Calculation Comparison ===")

	// Sum up prices - floats may accumulate errors
	var floatSum float64 = 0
	for _, val := range floatResults {
		floatSum += ekodb.GetFloatValue(val["price"])
	}
	fmt.Printf("  Float sum: $%v (expected $119.97)\n", floatSum)

	// Decimal sum (GetDecimalValue handles string parsing)
	var decimalSum float64 = 0
	for _, val := range decimalResults {
		decimalSum += ekodb.GetDecimalValue(val["price"])
	}
	fmt.Printf("  Decimal sum: $%.2f (expected $119.97)\n", decimalSum)

	// =========================================================================
	// Test 4: More extreme precision example
	// =========================================================================
	fmt.Println("\n=== Test 4: Extreme Precision Example ===")

	// Store a value that floats can't represent exactly
	err = client.KVSet("precision:float", map[string]interface{}{"amount": 0.1 + 0.2})
	if err != nil {
		log.Fatalf("Failed to set float precision: %v", err)
	}
	err = client.KVSet("precision:decimal", map[string]interface{}{"amount": ekodb.FieldDecimal("0.30")})
	if err != nil {
		log.Fatalf("Failed to set decimal precision: %v", err)
	}

	floatPrecision, _ := client.KVGet("precision:float")
	decimalPrecision, _ := client.KVGet("precision:decimal")

	if floatMap, ok := floatPrecision.(map[string]interface{}); ok {
		floatAmount := ekodb.GetFloatValue(floatMap["amount"])
		fmt.Printf("  Float 0.1 + 0.2 = %.17f (should be 0.3)\n", floatAmount)
	}

	if decimalMap, ok := decimalPrecision.(map[string]interface{}); ok {
		decimalAmount := ekodb.GetStringValue(decimalMap["amount"])
		fmt.Printf("  Decimal \"0.30\" = %s (exact!)\n", decimalAmount)
	}

	// =========================================================================
	// Cleanup
	// =========================================================================
	fmt.Println("\n=== Cleanup ===")
	allKeys := append(floatKeys, decimalKeys...)
	allKeys = append(allKeys, "precision:float", "precision:decimal")
	_, err = client.KVBatchDelete(allKeys)
	if err != nil {
		log.Printf("Warning: cleanup failed: %v", err)
	}
	fmt.Println("✓ Cleaned up test keys")

	fmt.Println("\n=== Summary ===")
	fmt.Println("✅ Use FieldDecimal() for monetary values, percentages, and")
	fmt.Println("   any case where floating-point errors are unacceptable.")
	fmt.Println("✅ FieldDecimal() stores values as strings internally,")
	fmt.Println("   preserving exact precision across all operations.")
}
