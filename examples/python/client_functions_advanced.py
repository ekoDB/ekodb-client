"""
Advanced Functions Example - Query, Sort, Limit, Group

Demonstrates advanced query and aggregation operations using simple patterns
"""

import asyncio
import os
from pathlib import Path

from dotenv import load_dotenv

env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


def _is_already_exists_error(err):
    """Detect the server's 409 'function already exists' response."""
    msg = str(err)
    return "409" in msg or "already exists" in msg


def _is_not_found_error(err):
    msg = str(err).lower()
    return "404" in msg or "not found" in msg


def field_value(record, key):
    value = record.get(key)
    if isinstance(value, dict) and "value" in value:
        return value["value"]
    return value


async def save_or_update(client, script):
    """Save a function, falling back to an update if its label already exists."""
    label = script["label"]
    try:
        return await client.save_function(script)
    except Exception as e:
        if not _is_already_exists_error(e):
            raise
        await client.update_function(label, script)
        print(f"ℹ️  Function '{label}' already existed — updated instead")
        existing = await client.get_function(label)
        function_id = existing.get("id")
        if not function_id:
            raise RuntimeError(f"Updated function '{label}' did not include an id")
        return function_id


async def run_examples(client, script_ids):
    print("🚀 ekoDB Python Advanced Functions Example\n")

    # Setup test data
    print("📋 Setting up test data...")
    products = [
        {
            "name": "Laptop Pro",
            "category": "Electronics",
            "price": 1299,
            "stock": 15,
            "rating": 4.8,
        },
        {
            "name": "Wireless Mouse",
            "category": "Electronics",
            "price": 29,
            "stock": 45,
            "rating": 4.5,
        },
        {
            "name": "Desk Chair",
            "category": "Furniture",
            "price": 349,
            "stock": 8,
            "rating": 4.2,
        },
        {
            "name": "Standing Desk",
            "category": "Furniture",
            "price": 599,
            "stock": 12,
            "rating": 4.7,
        },
        {
            "name": "USB-C Cable",
            "category": "Electronics",
            "price": 19,
            "stock": 100,
            "rating": 4.3,
        },
        {
            "name": 'Monitor 27"',
            "category": "Electronics",
            "price": 399,
            "stock": 20,
            "rating": 4.6,
        },
        {
            "name": "Bookshelf",
            "category": "Furniture",
            "price": 149,
            "stock": 5,
            "rating": 4.1,
        },
        {
            "name": "Keyboard",
            "category": "Electronics",
            "price": 89,
            "stock": 30,
            "rating": 4.4,
        },
    ]

    inserted_ids = []
    for product in products:
        inserted = await client.insert("advanced_products_py", product)
        product_id = field_value(inserted, "id")
        if not product_id:
            raise AssertionError("product insert did not return an ID")
        inserted_ids.append(product_id)
    if len(set(inserted_ids)) != 8:
        raise AssertionError("product inserts did not return 8 unique IDs")
    print(f"✅ Created {len(products)} products\n")

    # Example 1: List All Products
    print("📝 Example 1: List All Products\n")
    script1 = {
        "label": "list_all_products_adv_py",
        "name": "List All Products",
        "version": "1.0",
        "parameters": {},
        "functions": [{"type": "FindAll", "collection": "advanced_products_py"}],
        "tags": ["products", "list"],
    }
    script_id1 = await save_or_update(client, script1)
    script_ids.append(script_id1)
    print("✅ Function saved")

    result1 = await client.call_function("list_all_products_adv_py", None)
    print(f"📊 Found {len(result1['records'])} products")
    if len(result1["records"]) != 8:
        raise AssertionError("list function did not return all 8 products")
    print(f"⏱️  Execution time: {result1['stats']['execution_time_ms']}ms\n")

    # Example 2: Group Products by Category
    print("📝 Example 2: Group Products by Category\n")
    script2 = {
        "label": "products_by_category_py",
        "name": "Products by Category",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": "advanced_products_py"},
            {
                "type": "Group",
                "by_fields": ["category"],
                "functions": [
                    {"output_field": "count", "operation": "Count"},
                    {
                        "output_field": "avg_price",
                        "operation": "Average",
                        "input_field": "price",
                    },
                ],
            },
        ],
        "tags": ["products", "analytics"],
    }
    script_id2 = await save_or_update(client, script2)
    script_ids.append(script_id2)
    print("✅ Function saved")

    result2 = await client.call_function("products_by_category_py", None)
    print("📊 Category breakdown:")
    category_stats = {}
    for record in result2["records"]:
        print(f"   {record}")
        category_stats[field_value(record, "category")] = (
            field_value(record, "count"),
            field_value(record, "avg_price"),
        )
    expected_stats = {
        "Electronics": (5, 367.0),
        "Furniture": (3, 1097 / 3),
    }
    if set(category_stats) != set(expected_stats):
        raise AssertionError(f"unexpected category groups: {category_stats!r}")
    for category, (expected_count, expected_average) in expected_stats.items():
        count, average = category_stats[category]
        if count != expected_count or not isinstance(average, (int, float)):
            raise AssertionError(
                f"unexpected {category} aggregate: {(count, average)!r}"
            )
        if abs(average - expected_average) > 1e-9:
            raise AssertionError(f"unexpected {category} average: {average!r}")
    print(f"⏱️  Execution time: {result2['stats']['execution_time_ms']}ms\n")

    # Example 3: Count Total
    print("📝 Example 3: Count Total Products\n")
    script3 = {
        "label": "count_products_py",
        "name": "Count Products",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": "advanced_products_py"},
            {"type": "Count", "output_field": "total"},
        ],
        "tags": ["products", "count"],
    }
    script_id3 = await save_or_update(client, script3)
    script_ids.append(script_id3)
    print("✅ Function saved")

    result3 = await client.call_function("count_products_py", None)
    print(f"📊 Total products: {result3['records']}")
    if len(result3["records"]) != 1 or field_value(result3["records"][0], "total") != 8:
        raise AssertionError(f"unexpected total count response: {result3['records']!r}")
    print(f"⏱️  Execution time: {result3['stats']['execution_time_ms']}ms\n")


async def main():
    from ekodb_client import Client

    client = Client.new(BASE_URL, API_KEY)
    collection = "advanced_products_py"
    script_ids = []
    try:
        await client.delete_collection(collection)
    except Exception as error:
        if not _is_not_found_error(error):
            raise

    operation_error = None
    try:
        await run_examples(client, script_ids)
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        operation_error = error

    print("🧹 Cleaning up...")
    cleanup_errors = []
    for script_id in reversed(script_ids):
        try:
            await client.delete_function(script_id)
        except Exception as error:  # noqa: BLE001 - attempt every cleanup
            cleanup_errors.append(f"function {script_id}: {error}")
    try:
        await client.delete_collection(collection)
    except Exception as error:  # noqa: BLE001 - preserve the operation error
        if not _is_not_found_error(error):
            cleanup_errors.append(f"collection {collection}: {error}")

    if operation_error is not None:
        if cleanup_errors:
            operation_error.add_note(
                "cleanup also failed: " + "; ".join(cleanup_errors)
            )
            print("⚠️  Cleanup errors: " + "; ".join(cleanup_errors))
        raise operation_error
    if cleanup_errors:
        raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))

    print("✅ Cleanup complete\n")
    print("✅ All advanced script examples finished!")


if __name__ == "__main__":
    asyncio.run(main())
