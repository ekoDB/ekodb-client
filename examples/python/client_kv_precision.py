#!/usr/bin/env python3
"""
KV Precision Example - Float vs Decimal Comparison

This example demonstrates the difference between using Python floats
(which lose precision) vs ekoDB's Decimal type (which preserves precision).

Key takeaway: Use field_decimal() for monetary values and other cases
where precision matters.
"""

import asyncio
import json
import os
from decimal import Decimal
from pathlib import Path

from dotenv import load_dotenv

from ekodb_client import Client, field_decimal, get_value

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
FLOAT_KEYS = [f"kv_precision:py:product:float:{index}" for index in range(1, 4)]
DECIMAL_KEYS = [f"kv_precision:py:product:decimal:{index}" for index in range(1, 4)]
FLOAT_PRECISION_KEY = "kv_precision:py:precision:float"
DECIMAL_PRECISION_KEY = "kv_precision:py:precision:decimal"
ALL_KEYS = [*FLOAT_KEYS, *DECIMAL_KEYS, FLOAT_PRECISION_KEY, DECIMAL_PRECISION_KEY]


def kv_object(result, label):
    if not isinstance(result, dict):
        raise TypeError(f"Unexpected {label} KV response: {result!r}")

    payload = result.get("value", result)
    if isinstance(payload, str):
        payload = json.loads(payload)
    payload = get_value(payload)
    if not isinstance(payload, dict):
        raise TypeError(f"Unexpected {label} KV payload: {result!r}")
    return payload


async def run_example(client):
    print("=== KV Precision: Float vs Decimal ===\n")

    # =========================================================================
    # Test 1: Float Precision Loss
    # =========================================================================
    print("=== Test 1: Using Python Floats (LOSES PRECISION) ===")

    float_products = [
        {"key": FLOAT_KEYS[0], "value": {"name": "Widget A", "price": 29.99}},
        {"key": FLOAT_KEYS[1], "value": {"name": "Widget B", "price": 39.99}},
        {"key": FLOAT_KEYS[2], "value": {"name": "Widget C", "price": 49.99}},
    ]

    # Store with float values
    await client.kv_batch_set(float_products)
    print("✓ Stored products with float prices")

    # Retrieve and show the precision loss
    # Note: KV values come back with type wrappers like { "type": "Float", "value": 29.99 }
    float_keys = [p["key"] for p in float_products]
    float_results = await client.kv_batch_get(float_keys)
    if len(float_results) != len(float_products):
        raise RuntimeError(
            f"Expected {len(float_products)} float products, got {len(float_results)}"
        )
    print("\nRetrieved float prices:")
    for i, val in enumerate(float_results):
        expected_price = float_products[i]["value"]["price"]
        actual_price = get_value(val.get("price")) if val else None
        match = "✓" if expected_price == actual_price else "✗ PRECISION LOST"
        print(
            f"  {float_products[i]['value']['name']}: ${actual_price} (expected ${expected_price}) {match}"
        )
        if actual_price != expected_price:
            raise RuntimeError(
                f"Unexpected float price for {float_products[i]['value']['name']}: "
                f"{actual_price!r}"
            )

    # =========================================================================
    # Test 2: Decimal Precision Preserved
    # =========================================================================
    print("\n=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===")

    decimal_products = [
        {
            "key": DECIMAL_KEYS[0],
            "value": {"name": "Widget A", "price": field_decimal("29.99")},
        },
        {
            "key": DECIMAL_KEYS[1],
            "value": {"name": "Widget B", "price": field_decimal("39.99")},
        },
        {
            "key": DECIMAL_KEYS[2],
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
    if len(decimal_results) != len(decimal_products):
        raise RuntimeError(
            f"Expected {len(decimal_products)} decimal products, got {len(decimal_results)}"
        )
    print("\nRetrieved decimal prices:")
    for i, val in enumerate(decimal_results):
        expected_price = decimal_products[i]["value"]["price"]["value"]
        actual_price = get_value(val.get("price")) if val else None
        print(
            f"  {decimal_products[i]['value']['name']}: ${actual_price} (expected ${expected_price}) ✓"
        )
        if actual_price != expected_price:
            raise RuntimeError(
                f"Unexpected decimal price for {decimal_products[i]['value']['name']}: "
                f"{actual_price!r}"
            )

    # =========================================================================
    # Test 3: Real-world calculation comparison
    # =========================================================================
    print("\n=== Test 3: Sum Calculation Comparison ===")

    # Sum up prices - floats will accumulate errors
    float_sum = sum(get_value(val.get("price")) for val in float_results)
    print(f"  Float sum: ${float_sum} (expected $119.97)")
    if abs(float_sum - 119.97) > 1e-9:
        raise RuntimeError(f"Unexpected float sum: {float_sum!r}")

    # For decimal, values come back as strings - need string-based arithmetic
    decimal_sum = Decimal(0)
    for val in decimal_results:
        price_str = get_value(val.get("price"))
        if not isinstance(price_str, str):
            raise TypeError(f"Unexpected decimal price in sum: {price_str!r}")
        decimal_sum += Decimal(price_str)
    print(f"  Decimal sum: ${decimal_sum:.2f} (expected $119.97)")
    if decimal_sum != Decimal("119.97"):
        raise RuntimeError(f"Unexpected decimal sum: {decimal_sum!r}")

    # =========================================================================
    # Test 4: More extreme precision example
    # =========================================================================
    print("\n=== Test 4: Extreme Precision Example ===")

    # Store a value that floats can't represent exactly
    await client.kv_set(FLOAT_PRECISION_KEY, {"amount": 0.1 + 0.2})
    await client.kv_set(DECIMAL_PRECISION_KEY, {"amount": field_decimal("0.30")})

    float_precision = await client.kv_get(FLOAT_PRECISION_KEY)
    decimal_precision = await client.kv_get(DECIMAL_PRECISION_KEY)

    # kv_get returns {"value": "<json_string>"}; decode and unwrap the object.
    float_parsed = kv_object(float_precision, "float precision")
    float_amount = get_value(float_parsed.get("amount"))
    if float_amount != 0.1 + 0.2:
        raise RuntimeError(f"Unexpected float precision value: {float_amount!r}")
    print(f"  Float 0.1 + 0.2 = {float_amount} (should be 0.3)")

    decimal_parsed = kv_object(decimal_precision, "decimal precision")
    decimal_amount = get_value(decimal_parsed.get("amount"))
    if decimal_amount != "0.30":
        raise RuntimeError(f"Unexpected decimal precision value: {decimal_amount!r}")
    print(f'  Decimal "0.30" = {decimal_amount} (exact!)')


async def main():
    client = Client.new(BASE_URL, API_KEY)
    await client.kv_batch_delete(ALL_KEYS)
    primary_error = None
    try:
        await run_example(client)
    except BaseException as error:
        primary_error = error
        raise
    finally:
        # =====================================================================
        # Cleanup
        # =====================================================================
        print("\n=== Cleanup ===")
        try:
            await client.kv_batch_delete(ALL_KEYS)
            print("✓ Cleaned up test keys")
        except BaseException as cleanup_error:
            if primary_error is None:
                raise
            print(f"Cleanup error after primary failure: {cleanup_error!r}")
            primary_error.add_note(f"Cleanup also failed: {cleanup_error}")

    print("\n=== Summary ===")
    print("✅ Use field_decimal() for monetary values, percentages, and")
    print("   any case where floating-point errors are unacceptable.")
    print("✅ field_decimal() stores values as strings internally,")
    print("   preserving exact precision across all operations.")


if __name__ == "__main__":
    asyncio.run(main())
