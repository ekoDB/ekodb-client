"""
Complete Functions Example - ekoDB Functions
Demonstrates: Query (consolidated), Update, Delete, Batch Ops, TextSearch, Group, Count
"""

import asyncio
import os
import random
from pathlib import Path
from dotenv import load_dotenv

# Load environment variables first
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

# Note: Python client uses Rust bindings, so functions are defined using dictionaries

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
COLLECTION = "complete_products_py"
FUNCTION_LABELS = {
    "product_stats",
    "list_all_products",
    "count_by_category",
    "top_rated_products",
    "list_with_limit",
    "product_summary",
}


def _is_already_exists_error(err):
    """Detect the server's 409 'function already exists' response."""
    msg = str(err)
    return "409" in msg or "already exists" in msg


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


def generate_mock_embedding(size: int) -> list:
    """Generate mock embedding vector"""
    return [random.random() * 0.1 + (i % 100) / 100.0 for i in range(size)]


async def setup_test_data(client):
    """Setup test data for all examples"""
    print("📋 Setting up complete test data...")

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
            "name": 'Monitor 27"',
            "category": "Electronics",
            "price": 399,
            "stock": 20,
            "rating": 4.6,
        },
    ]

    for product in products:
        await client.insert(COLLECTION, product)

    print(f"✅ Created {len(products)} products\n")


async def advanced_query_function(client):
    """Example 1: FindAll + Group (Simple Aggregation)"""
    print("📝 Example 1: FindAll + Group (Simple Aggregation)\n")

    script = {
        "label": "product_stats",
        "name": "Product Statistics",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": COLLECTION},
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

    script_id = await save_or_update(client, script)
    print(f"✅ Function saved: {script_id}")

    result = await client.call_function("product_stats", None)

    print(f"📊 Found {len(result['records'])} product groups")
    for record in result["records"]:
        print(f"   {record}")
    print(f"⏱️  Execution time: {result['stats']['execution_time_ms']}ms\n")

    return script_id


async def list_products_script(client):
    """Example 2: Simple Product Listing"""
    print("📝 Example 2: Simple Product Listing\n")

    script = {
        "label": "list_all_products",
        "name": "List All Products",
        "version": "1.0",
        "parameters": {},
        "functions": [{"type": "FindAll", "collection": COLLECTION}],
        "tags": ["products", "list"],
    }

    script_id = await save_or_update(client, script)
    print("✅ Function saved")

    result = await client.call_function("list_all_products", None)

    print(f"📊 Found {len(result['records'])} products")
    print(f"⏱️  Execution time: {result['stats']['execution_time_ms']}ms\n")

    return script_id


async def category_count_script(client):
    """Example 3: Count by Category"""
    print("📝 Example 3: Count by Category\n")

    script = {
        "label": "count_by_category",
        "name": "Count Products by Category",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": COLLECTION},
            {
                "type": "Group",
                "by_fields": ["category"],
                "functions": [{"output_field": "count", "operation": "Count"}],
            },
        ],
        "tags": ["products", "analytics"],
    }

    script_id = await save_or_update(client, script)
    print("✅ Function saved")

    result = await client.call_function("count_by_category", None)

    print(f"📊 Found {len(result['records'])} categories")
    for record in result["records"]:
        print(f"   {record}")
    print(f"⏱️  Execution time: {result['stats']['execution_time_ms']}ms\n")

    return script_id


async def top_rated_script(client):
    """Example 4: High Rating Products"""
    print("📝 Example 4: High Rating Products\n")

    script = {
        "label": "top_rated_products",
        "name": "Top Rated Products",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": COLLECTION,
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "rating",
                        "operator": "Gte",
                        "value": 4.6,
                    },
                },
            }
        ],
        "tags": ["products", "quality"],
    }

    script_id = await save_or_update(client, script)
    print("✅ Function saved")

    result = await client.call_function("top_rated_products", None)
    if len(result["records"]) != 3:
        raise RuntimeError(
            f"Top-rated query returned {len(result['records'])} products; expected 3"
        )

    print(f"📊 Found {len(result['records'])} products")
    print(f"⏱️  Execution time: {result['stats']['execution_time_ms']}ms\n")

    return script_id


async def script_with_parameter(client):
    """Example 5: function with Parameter Definition"""
    print("📝 Example 5: function with Parameter Definition\n")

    script = {
        "label": "list_with_limit",
        "name": "List Products with Limit",
        "version": "1.0",
        "parameters": {
            "max_items": {
                "required": False,
                "default": 5,
            },
        },
        "functions": [
            {
                "type": "Query",
                "collection": COLLECTION,
                "limit": "{{max_items}}",
            },
        ],
        "tags": ["products", "list"],
    }

    script_id = await save_or_update(client, script)
    print("✅ Function saved")

    result = await client.call_function("list_with_limit", {"max_items": 3})
    if len(result["records"]) != 3:
        raise RuntimeError(
            f"Parameterized limit returned {len(result['records'])} products; expected 3"
        )

    print(f"📊 Found {len(result['records'])} products")
    print(f"⏱️  Execution time: {result['stats']['execution_time_ms']}ms\n")

    return script_id


async def multi_stage_pipeline(client):
    """Example 6: Multi-Stage Pipeline"""
    print("📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)\n")

    script = {
        "label": "product_summary",
        "name": "Product Summary Report",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": COLLECTION},
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
            {"type": "Count", "output_field": "total_categories"},
        ],
        "tags": ["products", "analytics"],
    }

    script_id = await save_or_update(client, script)
    print("✅ Function saved")

    result = await client.call_function("product_summary", None)

    print(f"📊 Pipeline executed {result['stats']['stages_executed']} stages")
    print(f"⏱️  Total execution time: {result['stats']['execution_time_ms']}ms")
    print()

    return script_id


async def cleanup(client, script_ids):
    """Cleanup test data and scripts"""
    print("🧹 Cleaning up...")

    errors = []
    ids = set(script_ids)
    try:
        for function in await client.list_functions(None):
            if function.get("label") in FUNCTION_LABELS and function.get("id"):
                ids.add(function["id"])
    except Exception as error:
        errors.append(f"list functions for cleanup: {error}")
    for script_id in ids:
        try:
            await client.delete_function(script_id)
        except Exception as error:
            errors.append(f"function {script_id}: {error}")
    try:
        await client.delete_collection(COLLECTION)
    except Exception as error:
        errors.append(f"collection {COLLECTION}: {error}")
    if errors:
        raise RuntimeError("Cleanup failed: " + "; ".join(errors))
    print("✅ Cleanup complete\n")


async def main():
    """Main function"""
    print("🚀 ekoDB Python Complete Functions Example\n")
    print("📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines\n")

    # Import client (assuming it's available)
    try:
        from ekodb_client import Client
    except ImportError as error:
        raise RuntimeError(
            "ekodb_client not found. Please install the Python client."
        ) from error

    client = Client.new(BASE_URL, API_KEY)

    script_ids = []
    try:
        try:
            await client.delete_collection(COLLECTION)
        except Exception:
            pass
        await setup_test_data(client)
        script_ids.append(await advanced_query_function(client))
        script_ids.append(await list_products_script(client))
        script_ids.append(await category_count_script(client))
        script_ids.append(await top_rated_script(client))
        script_ids.append(await script_with_parameter(client))
        script_ids.append(await multi_stage_pipeline(client))
        print("✅ All complete script examples finished!")
        print("\n💡 This example demonstrates ekoDB's function system:")
        print("   ✅ FindAll operations")
        print("   ✅ Group aggregations (Count, Average)")
        print("   ✅ Multi-stage pipelines (FindAll → Group → Count)")
        print("   ✅ Parameter definitions")
        print("   ✅ Function management (save, call, delete)")
    finally:
        await cleanup(client, script_ids)


if __name__ == "__main__":
    asyncio.run(main())
