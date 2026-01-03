#!/usr/bin/env python3
"""
KV Store & Wrapped Types Example for ekoDB Python Client

Demonstrates: KV operations, wrapped type field builders
"""

import asyncio
import os
from datetime import datetime

from ekodb_client import (
    Client,
    field_uuid,
    field_decimal,
    field_datetime,
    field_duration,
    field_number,
    field_set,
    field_vector,
    field_object,
    field_integer,
    field_float,
    field_boolean,
)


BASE_URL = os.environ.get("API_BASE_URL", "http://localhost:8080")
API_KEY = os.environ.get("API_BASE_KEY", "a-test-api-key-from-ekodb")


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
        "created_at": field_datetime(datetime.now()),
        "processing_time": field_duration(3600000),  # 1 hour in ms
        "quantity": field_number(42),
        "tags": field_set(["priority", "express", "international"]),
        "embedding": field_vector([0.1, 0.2, 0.3, 0.4, 0.5]),
        "metadata": field_object({"source": "web", "campaign": "summer2024"}),
    }

    result = await client.insert("orders_example", order)
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
        await client.insert("products_example", product)

    print(f"✅ Inserted {len(products)} products with wrapped types\n")


async def wrapped_types_query(client: Client) -> None:
    """Example 2: Querying Records with Wrapped Types"""
    print("📝 Example 2: Querying and Extracting Wrapped Types\n")

    # Query all products
    products = await client.find("products_example", limit=10)
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
    await client.kv_set("user:session:123", {"userId": "user_abc", "role": "admin"})
    print("✅ Set session data")

    # Get the value back
    session = await client.kv_get("user:session:123")
    print(f"📊 Retrieved session: {session}")

    # Check if key exists
    exists = await client.kv_exists("user:session:123")
    print(f"🔍 Key exists: {exists}")

    # Set with TTL (1 hour)
    await client.kv_set(
        "cache:product:456",
        {"name": "Cached Product", "price": 99.99},
        ttl=3600,
    )
    print("✅ Set cached data with 1 hour TTL")

    # Delete a key
    deleted = await client.kv_delete("user:session:123")
    print(f"🗑️  Deleted session: {deleted}\n")


async def kv_pattern_query(client: Client) -> None:
    """Example 4: KV Pattern Query"""
    print("📝 Example 4: KV Pattern Query\n")

    # Set up multiple KV entries with a pattern
    await client.kv_set("config:app:theme", {"mode": "dark"})
    await client.kv_set("config:app:language", {"code": "en"})
    await client.kv_set("config:app:notifications", {"enabled": True})
    await client.kv_set("config:user:preferences", {"timezone": "UTC"})

    print("✅ Set 4 config entries")

    # Query all config:app:* keys
    app_configs = await client.kv_query(pattern="config:app:*")
    print(f"📊 Found {len(app_configs)} app config entries")

    # Query all config:* keys
    all_configs = await client.kv_query(pattern="config:*")
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
        "created_at": field_datetime(datetime.now()),
        "status": "processing",
    }

    result = await client.insert("processed_orders", order)
    print(f"✅ Inserted order: {result.get('id')}")

    # Cache order status in KV for quick lookups
    await client.kv_set(
        f"order:status:{order_id}",
        {
            "status": "processing",
            "updated_at": datetime.now().isoformat(),
        },
        ttl=86400,  # 24 hours
    )
    print("✅ Cached order status")

    # Quick status lookup via KV
    status = await client.kv_get(f"order:status:{order_id}")
    print(f"📊 Quick status lookup: {status}\n")


# =============================================================================
# Cleanup
# =============================================================================


async def cleanup(client: Client) -> None:
    """Clean up test data"""
    print("🧹 Cleaning up...")

    try:
        await client.delete_collection("orders_example")
        await client.delete_collection("products_example")
        await client.delete_collection("processed_orders")

        await client.kv_delete("cache:product:456")
        await client.kv_delete("config:app:theme")
        await client.kv_delete("config:app:language")
        await client.kv_delete("config:app:notifications")
        await client.kv_delete("config:user:preferences")
        await client.kv_delete("order:status:c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6")

        print("✅ Cleanup complete\n")
    except Exception:
        print("⚠️  Cleanup had some errors (may be expected)\n")


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

    try:
        # Wrapped Types Examples
        await wrapped_types_insert(client)
        await wrapped_types_query(client)

        # KV Store Examples
        await kv_basic_operations(client)
        await kv_pattern_query(client)

        # Combined Example
        await combined_example(client)

        # Cleanup
        await cleanup(client)

        print("✅ All KV & Wrapped Types examples completed!")
        print("\n💡 Key takeaways:")
        print("   ✅ Use field_* helpers for type-safe wrapped values")
        print("   ✅ field_decimal() preserves precision (no floating point errors)")
        print("   ✅ KV store is great for caching and quick lookups")
        print("   ✅ Combine KV caching with collection inserts for real workflows")

    except Exception as e:
        print(f"❌ Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(main())
