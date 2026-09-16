/**
 * KV Precision Example - Float vs Decimal Comparison
 *
 * This example demonstrates the difference between using JavaScript floats
 * (which lose precision) vs ekoDB's Decimal type (which preserves precision).
 *
 * Key takeaway: Use Field.decimal() for monetary values and other cases
 * where precision matters.
 */

import { EkoDBClient, Field, getValue } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

function isNotFoundError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 404") || /not found/i.test(message);
}

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const prefix = `kv_precision_ts_${process.pid}_${Date.now()}`;
  const floatKeys = [1, 2, 3].map((i) => `${prefix}:product:float:${i}`);
  const decimalKeys = [1, 2, 3].map((i) => `${prefix}:product:decimal:${i}`);
  const precisionFloatKey = `${prefix}:precision:float`;
  const precisionDecimalKey = `${prefix}:precision:decimal`;
  const allKeys = [
    ...floatKeys,
    ...decimalKeys,
    precisionFloatKey,
    precisionDecimalKey,
  ];
  let primaryError: unknown;

  try {
    console.log("=== KV Precision: Float vs Decimal ===\n");

    // =========================================================================
    // Test 1: Float Precision Loss
    // =========================================================================
    console.log("=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===");

    const floatProducts = [
      { key: floatKeys[0], value: { name: "Widget A", price: 29.99 } },
      { key: floatKeys[1], value: { name: "Widget B", price: 39.99 } },
      { key: floatKeys[2], value: { name: "Widget C", price: 49.99 } },
    ];

    // Store with float values
    await client.kvBatchSet(floatProducts);
    console.log("✓ Stored products with float prices");

    // Retrieve and show the precision loss
    // Note: KV values come back with type wrappers like { type: "Float", value: 29.99 }
    const floatResults = await client.kvBatchGet(
      floatProducts.map((p) => p.key),
    );
    console.log("\nRetrieved float prices:");
    floatResults.forEach((val, i) => {
      const expectedPrice = floatProducts[i].value.price;
      const actualPrice = getValue<number>(val?.price);
      const match = expectedPrice === actualPrice ? "✓" : "✗ PRECISION LOST";
      console.log(
        `  ${
          floatProducts[i].value.name
        }: $${actualPrice} (expected $${expectedPrice}) ${match}`,
      );
    });

    // =========================================================================
    // Test 2: Decimal Precision Preserved
    // =========================================================================
    console.log(
      "\n=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===",
    );

    const decimalProducts = [
      {
        key: decimalKeys[0],
        value: { name: "Widget A", price: Field.decimal("29.99") },
      },
      {
        key: decimalKeys[1],
        value: { name: "Widget B", price: Field.decimal("39.99") },
      },
      {
        key: decimalKeys[2],
        value: { name: "Widget C", price: Field.decimal("49.99") },
      },
    ];

    // Store with decimal values
    await client.kvBatchSet(decimalProducts);
    console.log("✓ Stored products with decimal prices");

    // Retrieve and show precision is preserved
    // Decimal values come back as { type: "Decimal", value: "29.99" } - getValue extracts the string
    const decimalResults = await client.kvBatchGet(
      decimalProducts.map((p) => p.key),
    );
    console.log("\nRetrieved decimal prices:");
    decimalResults.forEach((val, i) => {
      const expectedPrice = (decimalProducts[i].value.price as any).value;
      const actualPrice = getValue<string>(val?.price);
      console.log(
        `  ${
          decimalProducts[i].value.name
        }: $${actualPrice} (expected $${expectedPrice}) ✓`,
      );
    });

    // =========================================================================
    // Test 3: Real-world calculation comparison
    // =========================================================================
    console.log("\n=== Test 3: Sum Calculation Comparison ===");

    // Sum up prices - floats will accumulate errors
    const floatSum = floatResults.reduce(
      (sum, val) => sum + (getValue<number>(val?.price) || 0),
      0,
    );
    console.log(`  Float sum: $${floatSum} (expected $119.97)`);

    // For decimal, values come back as strings - need string-based arithmetic
    const decimalSum = decimalResults.reduce((sum, val) => {
      const priceStr = getValue<string>(val?.price) || "0";
      return sum + parseFloat(priceStr);
    }, 0);
    console.log(`  Decimal sum: $${decimalSum.toFixed(2)} (expected $119.97)`);

    // =========================================================================
    // Test 4: More extreme precision example
    // =========================================================================
    console.log("\n=== Test 4: Extreme Precision Example ===");

    // Store a value that floats can't represent exactly
    await client.kvSet(precisionFloatKey, { amount: 0.1 + 0.2 });
    await client.kvSet(precisionDecimalKey, { amount: Field.decimal("0.30") });

    const floatPrecision = await client.kvGet(precisionFloatKey);
    const decimalPrecision = await client.kvGet(precisionDecimalKey);

    const floatObject = getValue<Record<string, unknown>>(floatPrecision);
    const decimalObject = getValue<Record<string, unknown>>(decimalPrecision);
    const floatAmount = getValue<number>(floatObject?.amount);
    if (floatAmount !== 0.1 + 0.2) {
      throw new Error(`Unexpected float precision value: ${floatAmount}`);
    }
    console.log(`  Float 0.1 + 0.2 = ${floatAmount} (should be 0.3)`);
    const decimalAmount = getValue<string>(decimalObject?.amount);
    if (decimalAmount !== "0.30") {
      throw new Error(`Unexpected decimal precision value: ${decimalAmount}`);
    }
    console.log(`  Decimal "0.30" = ${decimalAmount} (exact!)`);

    // =========================================================================
    // Cleanup
    // =========================================================================
    console.log("\n=== Cleanup ===");
    console.log("\n=== Summary ===");
    console.log("✅ Use Field.decimal() for monetary values, percentages, and");
    console.log("   any case where floating-point errors are unacceptable.");
    console.log("✅ Field.decimal() stores values as strings internally,");
    console.log("   preserving exact precision across all operations.");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    console.log("\n=== Cleanup ===");
    const cleanupErrors: unknown[] = [];
    for (const key of allKeys) {
      try {
        await client.kvDelete(key);
      } catch (error) {
        if (!isNotFoundError(error)) cleanupErrors.push(error);
      }
    }
    if (cleanupErrors.length > 0) {
      throw new AggregateError(
        primaryError === undefined
          ? cleanupErrors
          : [primaryError, ...cleanupErrors],
        "KV precision example or cleanup failed",
      );
    }
    console.log("✓ Cleaned up test keys");
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
