// KV Precision Example - Float vs Decimal Comparison
//
// This example demonstrates the difference between using Go floats
// (which lose precision) vs ekoDB's Decimal type (which preserves precision).
//
// Key takeaway: Use FieldDecimal() for monetary values and other cases
// where precision matters.

package main

import (
	"errors"
	"fmt"
	"log"
	"math"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

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

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}
	floatKeys := []string{"product:float:1:go", "product:float:2:go", "product:float:3:go"}
	decimalKeys := []string{"product:decimal:1:go", "product:decimal:2:go", "product:decimal:3:go"}
	allKeys := append(append([]string{}, floatKeys...), decimalKeys...)
	allKeys = append(allKeys, "precision:float:go", "precision:decimal:go")
	if _, err := client.KVBatchDelete(allKeys); err != nil {
		return fmt.Errorf("initial KV cleanup: %w", err)
	}
	defer func() {
		fmt.Println("\n=== Cleanup ===")
		if _, cleanupErr := client.KVBatchDelete(allKeys); cleanupErr != nil {
			runErr = errors.Join(runErr, fmt.Errorf("cleanup test keys: %w", cleanupErr))
		} else {
			fmt.Println("✓ Cleaned up test keys")
		}
	}()

	fmt.Print("=== KV Precision: Float vs Decimal ===\n\n")

	// =========================================================================
	// Test 1: Float Precision Loss
	// =========================================================================
	fmt.Println("=== Test 1: Using Go Floats (LOSES PRECISION) ===")

	floatProducts := []map[string]interface{}{
		{"key": floatKeys[0], "value": map[string]interface{}{"name": "Widget A", "price": 29.99}},
		{"key": floatKeys[1], "value": map[string]interface{}{"name": "Widget B", "price": 39.99}},
		{"key": floatKeys[2], "value": map[string]interface{}{"name": "Widget C", "price": 49.99}},
	}

	// Store with float values
	_, err = client.KVBatchSet(floatProducts)
	if err != nil {
		return fmt.Errorf("batch set float products: %w", err)
	}
	fmt.Println("✓ Stored products with float prices")

	// Retrieve and show the precision loss
	// Note: KV values come back with type wrappers like { "type": "Float", "value": 29.99 }
	floatResults, err := client.KVBatchGet(floatKeys)
	if err != nil {
		return fmt.Errorf("batch get float products: %w", err)
	}
	if len(floatResults) != len(floatKeys) {
		return fmt.Errorf("batch get returned %d float products, expected %d", len(floatResults), len(floatKeys))
	}

	fmt.Println("\nRetrieved float prices:")
	expectedPrices := []float64{29.99, 39.99, 49.99}
	names := []string{"Widget A", "Widget B", "Widget C"}
	for i, val := range floatResults {
		actualPrice := ekodb.GetFloatValue(val["price"])
		if actualPrice != expectedPrices[i] {
			return fmt.Errorf("float price mismatch for %s: got %v, expected %v", names[i], actualPrice, expectedPrices[i])
		}
		fmt.Printf("  %s: $%v (expected $%v) ✓\n", names[i], actualPrice, expectedPrices[i])
	}

	// =========================================================================
	// Test 2: Decimal Precision Preserved
	// =========================================================================
	fmt.Println("\n=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===")

	decimalProducts := []map[string]interface{}{
		{"key": decimalKeys[0], "value": map[string]interface{}{"name": "Widget A", "price": ekodb.FieldDecimal("29.99")}},
		{"key": decimalKeys[1], "value": map[string]interface{}{"name": "Widget B", "price": ekodb.FieldDecimal("39.99")}},
		{"key": decimalKeys[2], "value": map[string]interface{}{"name": "Widget C", "price": ekodb.FieldDecimal("49.99")}},
	}

	// Store with decimal values
	_, err = client.KVBatchSet(decimalProducts)
	if err != nil {
		return fmt.Errorf("batch set decimal products: %w", err)
	}
	fmt.Println("✓ Stored products with decimal prices")

	// Retrieve and show precision is preserved
	// Decimal values come back as { "type": "Decimal", "value": "29.99" } - GetDecimalValue extracts it
	decimalResults, err := client.KVBatchGet(decimalKeys)
	if err != nil {
		return fmt.Errorf("batch get decimal products: %w", err)
	}
	if len(decimalResults) != len(decimalKeys) {
		return fmt.Errorf("batch get returned %d decimal products, expected %d", len(decimalResults), len(decimalKeys))
	}

	expectedDecimals := []string{"29.99", "39.99", "49.99"}
	fmt.Println("\nRetrieved decimal prices:")
	for i, val := range decimalResults {
		actualPrice := ekodb.GetStringValue(val["price"])
		if actualPrice != expectedDecimals[i] {
			return fmt.Errorf("decimal price mismatch for %s: got %q, expected %q", names[i], actualPrice, expectedDecimals[i])
		}
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
	if math.Abs(floatSum-119.97) > 1e-9 {
		return fmt.Errorf("float sum was %.17g, expected 119.97", floatSum)
	}
	fmt.Printf("  Float sum: $%v (expected $119.97)\n", floatSum)

	// Decimal sum (GetDecimalValue handles string parsing)
	var decimalSum float64 = 0
	for _, val := range decimalResults {
		decimalSum += ekodb.GetDecimalValue(val["price"])
	}
	if math.Abs(decimalSum-119.97) > 1e-9 {
		return fmt.Errorf("decimal sum was %.17g, expected 119.97", decimalSum)
	}
	fmt.Printf("  Decimal sum: $%.2f (expected $119.97)\n", decimalSum)

	// =========================================================================
	// Test 4: More extreme precision example
	// =========================================================================
	fmt.Println("\n=== Test 4: Extreme Precision Example ===")

	// Store a value that floats can't represent exactly
	err = client.KVSet("precision:float:go", map[string]interface{}{"amount": 0.1 + 0.2})
	if err != nil {
		return fmt.Errorf("set float precision: %w", err)
	}
	err = client.KVSet("precision:decimal:go", map[string]interface{}{"amount": ekodb.FieldDecimal("0.30")})
	if err != nil {
		return fmt.Errorf("set decimal precision: %w", err)
	}

	floatPrecision, err := client.KVGet("precision:float:go")
	if err != nil {
		return fmt.Errorf("get float precision value: %w", err)
	}
	decimalPrecision, err := client.KVGet("precision:decimal:go")
	if err != nil {
		return fmt.Errorf("get decimal precision value: %w", err)
	}

	floatMap, ok := ekodb.GetValue(floatPrecision).(map[string]interface{})
	if !ok {
		return fmt.Errorf("float precision value has unexpected shape: %T", ekodb.GetValue(floatPrecision))
	}
	floatAmount := ekodb.GetFloatValue(floatMap["amount"])
	if floatAmount == 0 {
		return errors.New("float precision amount was missing or zero")
	}
	fmt.Printf("  Float 0.1 + 0.2 = %.17f (should be 0.3)\n", floatAmount)

	decimalMap, ok := ekodb.GetValue(decimalPrecision).(map[string]interface{})
	if !ok {
		return fmt.Errorf("decimal precision value has unexpected shape: %T", ekodb.GetValue(decimalPrecision))
	}
	decimalAmount := ekodb.GetStringValue(decimalMap["amount"])
	if decimalAmount != "0.30" {
		return fmt.Errorf("decimal precision mismatch: got %q, expected %q", decimalAmount, "0.30")
	}
	fmt.Printf("  Decimal \"0.30\" = %s (exact!)\n", decimalAmount)

	fmt.Println("\n=== Summary ===")
	fmt.Println("✅ Use FieldDecimal() for monetary values, percentages, and")
	fmt.Println("   any case where floating-point errors are unacceptable.")
	fmt.Println("✅ FieldDecimal() stores values as strings internally,")
	fmt.Println("   preserving exact precision across all operations.")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
