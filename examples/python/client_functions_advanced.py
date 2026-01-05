"""
Advanced Scripts Example - Query, Sort, Limit, Group

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


async def main():
    from ekodb_client import Client, Stage

    client = Client.new(BASE_URL, API_KEY)

    print("🚀 ekoDB Python Advanced Scripts Example\n")

    # Setup test data
    print("📋 Setting up test data...")
    try:
        await client.delete_collection("advanced_products_py")
    except Exception:
        pass

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

    for product in products:
        await client.insert("advanced_products_py", product)
    print(f"✅ Created {len(products)} products\n")

    script_ids = []

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
    script_id1 = await client.save_script(script1)
    script_ids.append(script_id1)
    print("✅ Script saved")

    result1 = await client.call_script("list_all_products_adv_py", None)
    print(f"📊 Found {len(result1['records'])} products")
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
    script_id2 = await client.save_script(script2)
    script_ids.append(script_id2)
    print("✅ Script saved")

    result2 = await client.call_script("products_by_category_py", None)
    print(f"📊 Category breakdown:")
    for record in result2["records"]:
        print(f"   {record}")
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
    script_id3 = await client.save_script(script3)
    script_ids.append(script_id3)
    print("✅ Script saved")

    result3 = await client.call_script("count_products_py", None)
    print(f"📊 Total products: {result3['records']}")
    print(f"⏱️  Execution time: {result3['stats']['execution_time_ms']}ms\n")

    # Cleanup
    print("🧹 Cleaning up...")
    for script_id in script_ids:
        try:
            await client.delete_script(script_id)
        except Exception:
            pass
    try:
        await client.delete_collection("advanced_products_py")
    except Exception:
        pass
    print("✅ Cleanup complete\n")

    print("✅ All advanced script examples finished!")


if __name__ == "__main__":
    asyncio.run(main())
