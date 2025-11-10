"""
Complete Scripts Example - ekoDB Scripts
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
        await client.insert("complete_products", product)

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
            {"type": "FindAll", "collection": "complete_products"},
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

    script_id = await client.save_script(script)
    print(f"✅ Script saved: {script_id}")

    result = await client.call_script("product_stats", None)

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
        "functions": [
            {"type": "FindAll", "collection": "complete_products"},
        ],
        "tags": ["products", "list"],
    }

    script_id = await client.save_script(script)
    print("✅ Script saved")

    result = await client.call_script("list_all_products", None)

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
            {"type": "FindAll", "collection": "complete_products"},
            {
                "type": "Group",
                "by_fields": ["category"],
                "functions": [{"output_field": "count", "operation": "Count"}],
            },
        ],
        "tags": ["products", "analytics"],
    }

    script_id = await client.save_script(script)
    print("✅ Script saved")

    result = await client.call_script("count_by_category", None)

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
            {"type": "FindAll", "collection": "complete_products"},
        ],
        "tags": ["products", "quality"],
    }

    script_id = await client.save_script(script)
    print("✅ Script saved")

    result = await client.call_script("top_rated_products", None)

    print(f"📊 Found {len(result['records'])} products")
    print(f"⏱️  Execution time: {result['stats']['execution_time_ms']}ms\n")

    return script_id


async def script_with_parameter(client):
    """Example 5: Script with Parameter Definition"""
    print("📝 Example 5: Script with Parameter Definition\n")

    script = {
        "label": "list_with_limit",
        "name": "List Products with Limit",
        "version": "1.0",
        "parameters": {
            "max_items": {
                "param_type": "Integer",
                "required": False,
                "default": 5,
            },
        },
        "functions": [
            {"type": "FindAll", "collection": "complete_products"},
        ],
        "tags": ["products", "list"],
    }

    script_id = await client.save_script(script)
    print("✅ Script saved")

    result = await client.call_script("list_with_limit", {"max_items": 3})

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
            {"type": "FindAll", "collection": "complete_products"},
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

    script_id = await client.save_script(script)
    print("✅ Script saved")

    result = await client.call_script("product_summary", None)

    print(f"📊 Pipeline executed {result['stats']['stages_executed']} stages")
    print(f"⏱️  Total execution time: {result['stats']['execution_time_ms']}ms")
    print()

    return script_id


async def cleanup(client, script_ids):
    """Cleanup test data and scripts"""
    print("🧹 Cleaning up...")

    try:
        for script_id in script_ids:
            await client.delete_script(script_id)
        await client.delete_collection("complete_products")
        print("✅ Cleanup complete\n")
    except Exception as e:
        print(f"⚠️  Cleanup had some errors: {e}\n")


async def main():
    """Main function"""
    print("🚀 ekoDB Python Complete Scripts Example\n")
    print("📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines\n")

    # Import client (assuming it's available)
    try:
        from ekodb_client import Client
    except ImportError:
        print("❌ ekodb_client not found. Please install the Python client.")
        return

    client = Client.new(BASE_URL, API_KEY)

    try:
        await setup_test_data(client)
        script_ids = []
        script_ids.append(await advanced_query_function(client))
        script_ids.append(await list_products_script(client))
        script_ids.append(await category_count_script(client))
        script_ids.append(await top_rated_script(client))
        script_ids.append(await script_with_parameter(client))
        script_ids.append(await multi_stage_pipeline(client))
        await cleanup(client, script_ids)

        print("✅ All complete script examples finished!")
        print("\n💡 This example demonstrates ekoDB's Script system:")
        print("   ✅ FindAll operations")
        print("   ✅ Group aggregations (Count, Average)")
        print("   ✅ Multi-stage pipelines (FindAll → Group → Count)")
        print("   ✅ Parameter definitions")
        print("   ✅ Script management (save, call, delete)")
    except Exception as e:
        print(f"❌ Error: {e}")


if __name__ == "__main__":
    asyncio.run(main())
