#!/usr/bin/env python3
"""
KV Precision Example - Float vs Decimal Comparison

This example demonstrates the difference between using Python floats
(which lose precision) vs ekoDB's Decimal type (which preserves precision).

Key takeaway: Use field_decimal() for monetary values and other cases
where precision matters.
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv
from ekodb_client import Client, field_decimal, get_value

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def main():
    client = Client.new(BASE_URL, API_KEY)
    print("=== KV Precision: Float vs Decimal ===\n")

    # =========================================================================
    # Test 1: Float Precision Loss
    # =========================================================================
    print("=== Test 1: Using Python Floats (LOSES PRECISION) ===")

    float_products = [
        {"key": "product:float:1", "value": {"name": "Widget A", "price": 29.99}},
        {"key": "product:float:2", "value": {"name": "Widget B", "price": 39.99}},
        {"key": "product:float:3", "value": {"name": "Widget C", "price": 49.99}},
    ]

    # Store with float values
    await client.kv_batch_set(float_products)
    print("✓ Stored products with float prices")

    # Retrieve and show the precision loss
    # Note: KV values come back with type wrappers like { "type": "Float", "value": 29.99 }
    float_keys = [p["key"] for p in float_products]
    float_results = await client.kv_batch_get(float_keys)
    print("\nRetrieved float prices:")
    for i, val in enumerate(float_results):
        expected_price = float_products[i]["value"]["price"]
        actual_price = get_value(val.get("price")) if val else None
        match = "✓" if expected_price == actual_price else "✗ PRECISION LOST"
        print(
            f"  {float_products[i]['value']['name']}: ${actual_price} (expected ${expected_price}) {match}"
        )

    # =========================================================================
    # Test 2: Decimal Precision Preserved
    # =========================================================================
    print("\n=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===")

    decimal_products = [
        {
            "key": "product:decimal:1",
            "value": {"name": "Widget A", "price": field_decimal("29.99")},
        },
        {
            "key": "product:decimal:2",
            "value": {"name": "Widget B", "price": field_decimal("39.99")},
        },
        {
            "key": "product:decimal:3",
            "value": {"name": "Widget C", "price": field_decimal("49.99")},
        },
    ]

    # Store with decimal values
    await client.kv_batch_set(decimal_products)
    print("✓ Stored products with decimal prices")

    # Retrieve and show precision is preserved
    # Decimal values come back as { "type": "Decimal", "value": "29.99" } - get_value extracts the string
    decimal_keys = [p["key"] for p in decimal_products]
    decimal_results = await client.kv_batch_get(decimal_keys)
    print("\nRetrieved decimal prices:")
    for i, val in enumerate(decimal_results):
        expected_price = decimal_products[i]["value"]["price"]["value"]
        actual_price = get_value(val.get("price")) if val else None
        print(
            f"  {decimal_products[i]['value']['name']}: ${actual_price} (expected ${expected_price}) ✓"
        )

    # =========================================================================
    # Test 3: Real-world calculation comparison
    # =========================================================================
    print("\n=== Test 3: Sum Calculation Comparison ===")

    # Sum up prices - floats will accumulate errors
    float_sum = sum(
        get_value(val.get("price")) or 0 if val else 0 for val in float_results
    )
    print(f"  Float sum: ${float_sum} (expected $119.97)")

    # For decimal, values come back as strings - need string-based arithmetic
    decimal_sum = 0.0
    for val in decimal_results:
        if val:
            price_str = get_value(val.get("price")) or "0"
            decimal_sum += float(price_str)
    print(f"  Decimal sum: ${decimal_sum:.2f} (expected $119.97)")

    # =========================================================================
    # Test 4: More extreme precision example
    # =========================================================================
    print("\n=== Test 4: Extreme Precision Example ===")

    # Store a value that floats can't represent exactly
    await client.kv_set("precision:float", {"amount": 0.1 + 0.2})
    await client.kv_set("precision:decimal", {"amount": field_decimal("0.30")})

    float_precision = await client.kv_get("precision:float")
    decimal_precision = await client.kv_get("precision:decimal")

    float_amount = get_value(float_precision.get("amount")) if float_precision else None
    print(f"  Float 0.1 + 0.2 = {float_amount} (should be 0.3)")

    decimal_amount = (
        get_value(decimal_precision.get("amount")) if decimal_precision else None
    )
    print(f'  Decimal "0.30" = {decimal_amount} (exact!)')

    # =========================================================================
    # Cleanup
    # =========================================================================
    print("\n=== Cleanup ===")
    all_keys = float_keys + decimal_keys + ["precision:float", "precision:decimal"]
    await client.kv_batch_delete(all_keys)
    print("✓ Cleaned up test keys")

    print("\n=== Summary ===")
    print("✅ Use field_decimal() for monetary values, percentages, and")
    print("   any case where floating-point errors are unacceptable.")
    print("✅ field_decimal() stores values as strings internally,")
    print("   preserving exact precision across all operations.")


if __name__ == "__main__":
    asyncio.run(main())
