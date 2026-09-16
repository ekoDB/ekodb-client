#!/usr/bin/env python3
"""
KV Store & Wrapped Types Example for ekoDB Python Client

Demonstrates: KV operations, wrapped type field builders
"""

import asyncio
import os
from datetime import datetime, timezone
from pathlib import Path

from dotenv import load_dotenv

from ekodb_client import (
    Client,
    field_boolean,
    field_datetime,
    field_decimal,
    field_duration,
    field_float,
    field_integer,
    field_number,
    field_object,
    field_set,
    field_uuid,
    field_vector,
)

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY")
ORDERS = "orders_example_py"
PRODUCTS = "products_example_py"
PROCESSED_ORDERS = "processed_orders_example_py"
SESSION_KEY = "kv_wrapped:py:user:session:123"
CACHE_KEY = "kv_wrapped:py:cache:product:456"
CONFIG_THEME = "kv_wrapped:py:config:app:theme"
CONFIG_LANGUAGE = "kv_wrapped:py:config:app:language"
CONFIG_NOTIFICATIONS = "kv_wrapped:py:config:app:notifications"
CONFIG_PREFERENCES = "kv_wrapped:py:config:user:preferences"
ORDER_KEY = "kv_wrapped:py:order:status:c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6"
COLLECTIONS = (ORDERS, PRODUCTS, PROCESSED_ORDERS)
KV_KEYS = (
    SESSION_KEY,
    CACHE_KEY,
    CONFIG_THEME,
    CONFIG_LANGUAGE,
    CONFIG_NOTIFICATIONS,
    CONFIG_PREFERENCES,
    ORDER_KEY,
)


# =============================================================================
# Wrapped Types Examples
# =============================================================================


async def wrapped_types_insert(client: Client) -> None:
    """Example 1: Inserting Records with Wrapped Types"""
    print("📝 Example 1: Inserting Records with Wrapped Types\n")

    # Insert an order with wrapped types for precise data handling
    order = {
        "order_id": field_uuid("550e8400-e29b-41d4-a716-446655440000"),
        "total": field_decimal("1234.56"),  # Precise decimal
        "created_at": field_datetime(datetime.now(timezone.utc).isoformat()),
        "processing_time": field_duration(3600000),  # 1 hour in ms
        "quantity": field_number(42),
        "tags": field_set(["priority", "express", "international"]),
        "embedding": field_vector([0.1, 0.2, 0.3, 0.4, 0.5]),
        "metadata": field_object({"source": "web", "campaign": "summer2024"}),
    }

    result = await client.insert(ORDERS, order)
    print(f"✅ Inserted order: {result.get('id')}")

    # Insert products with wrapped types
    products = [
        {
            "sku": field_uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"),
            "name": "Laptop Pro",
            "price": field_decimal("1299.99"),
            "stock": field_integer(15),
            "rating": field_float(4.8),
            "available": field_boolean(True),
            "categories": field_set(["electronics", "computers"]),
        },
        {
            "sku": field_uuid("b1ffcd00-0d1c-5fg9-cc7e-7cc0ce491b22"),
            "name": "Wireless Mouse",
            "price": field_decimal("29.99"),
            "stock": field_integer(150),
            "rating": field_float(4.5),
            "available": field_boolean(True),
            "categories": field_set(["electronics", "accessories"]),
        },
    ]

    for product in products:
        await client.insert(PRODUCTS, product)

    print(f"✅ Inserted {len(products)} products with wrapped types\n")


async def wrapped_types_query(client: Client) -> None:
    """Example 2: Querying Records with Wrapped Types"""
    print("📝 Example 2: Querying and Extracting Wrapped Types\n")

    # Query all products
    products = await client.find(PRODUCTS, limit=10)
    print(f"📊 Found {len(products)} products")

    # Demonstrate extracting wrapped type values
    for product in products:
        name = product.get("name", {})
        if isinstance(name, dict) and "value" in name:
            name = name["value"]
        print(f"   • {name}")

    print()


# =============================================================================
# KV Store Examples
# =============================================================================


async def kv_basic_operations(client: Client) -> None:
    """Example 3: Basic KV Store Operations"""
    print("📝 Example 3: Basic KV Store Operations\n")

    # Set a simple value
    await client.kv_set(SESSION_KEY, {"userId": "user_abc", "role": "admin"})
    print("✅ Set session data")

    # Get the value back
    session = await client.kv_get(SESSION_KEY)
    print(f"📊 Retrieved session: {session}")

    # Check if key exists
    exists = await client.kv_exists(SESSION_KEY)
    print(f"🔍 Key exists: {exists}")

    # Set with TTL (1 hour)
    await client.kv_set(
        CACHE_KEY,
        {"name": "Cached Product", "price": 99.99},
        ttl="1h",
    )
    print("✅ Set cached data with 1 hour TTL")

    # Delete a key
    await client.kv_delete(SESSION_KEY)
    print("🗑️  Deleted session\n")


async def kv_pattern_query(client: Client) -> None:
    """Example 4: KV Pattern Query"""
    print("📝 Example 4: KV Pattern Query\n")

    # Set up multiple KV entries with a pattern
    await client.kv_set(CONFIG_THEME, {"mode": "dark"})
    await client.kv_set(CONFIG_LANGUAGE, {"code": "en"})
    await client.kv_set(CONFIG_NOTIFICATIONS, {"enabled": True})
    await client.kv_set(CONFIG_PREFERENCES, {"timezone": "UTC"})

    print("✅ Set 4 config entries")

    # Query all config:app:* keys
    app_configs = await client.kv_query(pattern="kv_wrapped:py:config:app:*")
    print(f"📊 Found {len(app_configs)} app config entries")

    # Query all config:* keys
    all_configs = await client.kv_query(pattern="kv_wrapped:py:config:*")
    print(f"📊 Found {len(all_configs)} total config entries\n")


# =============================================================================
# Combined Example
# =============================================================================


async def combined_example(client: Client) -> None:
    """Example 5: Combined Wrapped Types + KV Usage"""
    print("📝 Example 5: Combined Wrapped Types + KV Usage\n")

    # Create an order with wrapped types
    order_id = "c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6"
    order = {
        "order_id": field_uuid(order_id),
        "total": field_decimal("299.99"),
        "created_at": field_datetime(datetime.now(timezone.utc).isoformat()),
        "status": "processing",
    }

    result = await client.insert(PROCESSED_ORDERS, order)
    print(f"✅ Inserted order: {result.get('id')}")

    # Cache order status in KV for quick lookups
    await client.kv_set(
        ORDER_KEY,
        {
            "status": "processing",
            "updated_at": datetime.now(timezone.utc).isoformat(),
        },
        ttl="24h",  # 24 hours
    )
    print("✅ Cached order status")

    # Quick status lookup via KV
    status = await client.kv_get(ORDER_KEY)
    print(f"📊 Quick status lookup: {status}\n")


# =============================================================================
# Cleanup
# =============================================================================


async def cleanup(client: Client, verbose=True) -> None:
    """Clean up test data"""
    if verbose:
        print("🧹 Cleaning up...")

    errors = []
    for collection in COLLECTIONS:
        try:
            await client.delete_collection(collection)
        except Exception as error:
            if "404" not in str(error) and "not found" not in str(error).lower():
                errors.append(f"collection {collection}: {error}")
    for key in KV_KEYS:
        try:
            if await client.kv_exists(key):
                await client.kv_delete(key)
        except Exception as error:
            errors.append(f"KV key {key}: {error}")
    if errors:
        if verbose:
            print("⚠️  Cleanup had some errors (may be expected)\n")
        raise RuntimeError("; ".join(errors))
    if verbose:
        print("✅ Cleanup complete\n")


# =============================================================================
# Main
# =============================================================================


async def main():
    print("🚀 ekoDB Python KV Store & Wrapped Types Example\n")
    print("📋 Demonstrates:")
    print("   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)")
    print("   • KV store operations (get, set, delete, exists, query)")
    print("   • Combined wrapped types + KV workflows\n")

    client = Client.new(BASE_URL, API_KEY)

    await cleanup(client, verbose=False)
    operation_error = None
    try:
        # Wrapped Types Examples
        await wrapped_types_insert(client)
        await wrapped_types_query(client)

        # KV Store Examples
        await kv_basic_operations(client)
        await kv_pattern_query(client)

        # Combined Example
        await combined_example(client)

    except Exception as e:
        print(f"❌ Error: {e}")
        operation_error = e

    try:
        await cleanup(client)
    except Exception as cleanup_error:
        if operation_error is not None:
            operation_error.add_note(f"cleanup also failed: {cleanup_error}")
        else:
            raise
    if operation_error is not None:
        raise operation_error

    print("✅ All KV & Wrapped Types examples completed!")
    print("\n💡 Key takeaways:")
    print("   ✅ Use field_* helpers for type-safe wrapped values")
    print("   ✅ field_decimal() preserves precision (no floating point errors)")
    print("   ✅ KV store is great for caching and quick lookups")
    print("   ✅ Combine KV caching with collection inserts for real workflows")


if __name__ == "__main__":
    asyncio.run(main())
