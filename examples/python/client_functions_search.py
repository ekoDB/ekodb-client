"""
Search Functions Example - Basic Search Operations

Demonstrates simple search and query operations using scripts
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

    print("🚀 ekoDB Python Search Functions Example\n")

    # Setup test data
    print("📋 Setting up test data...")
    try:
        await client.delete_collection("search_docs_py")
    except Exception:
        pass

    documents = [
        {
            "title": "Introduction to Machine Learning",
            "content": "Machine learning is a subset of artificial intelligence.",
            "category": "AI",
            "tags": ["ml", "ai", "data science"],
        },
        {
            "title": "Database Design Principles",
            "content": "Good database design involves normalization and indexing.",
            "category": "Database",
            "tags": ["database", "design", "sql"],
        },
        {
            "title": "Vector Databases Explained",
            "content": "Vector databases store data based on semantic similarity.",
            "category": "Database",
            "tags": ["vector", "database", "embeddings"],
        },
        {
            "title": "Natural Language Processing",
            "content": "NLP enables computers to understand human language.",
            "category": "AI",
            "tags": ["nlp", "ai", "text"],
        },
        {
            "title": "Getting Started with ekoDB",
            "content": "ekoDB is a high-performance database with AI capabilities.",
            "category": "Database",
            "tags": ["ekodb", "database", "tutorial"],
        },
    ]

    for doc in documents:
        await client.insert("search_docs_py", doc)
    print(f"✅ Inserted {len(documents)} documents\n")

    script_ids = []

    # Example 1: List All Documents
    print("📝 Example 1: List All Documents\n")
    script1 = {
        "label": "list_all_docs_py",
        "name": "List All Documents",
        "version": "1.0",
        "parameters": {},
        "functions": [{"type": "FindAll", "collection": "search_docs_py"}],
        "tags": ["search", "list"],
    }
    script_id1 = await client.save_function(script1)
    script_ids.append(script_id1)
    print("✅ Function saved")

    result1 = await client.call_function("list_all_docs_py", None)
    print(f"📊 Found {len(result1['records'])} documents")
    for i, doc in enumerate(result1["records"]):
        title = doc.get("title", {})
        if isinstance(title, dict):
            title = title.get("value", title)
        category = doc.get("category", {})
        if isinstance(category, dict):
            category = category.get("value", category)
        print(f"   {i + 1}. {title} ({category})")
    print(f"⏱️  Execution time: {result1['stats']['execution_time_ms']}ms\n")

    # Example 2: Count Documents by Category
    print("📝 Example 2: Count Documents by Category\n")
    script2 = {
        "label": "docs_by_category_py",
        "name": "Documents by Category",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": "search_docs_py"},
            {
                "type": "Group",
                "by_fields": ["category"],
                "functions": [{"output_field": "count", "operation": "Count"}],
            },
        ],
        "tags": ["search", "analytics"],
    }
    script_id2 = await client.save_function(script2)
    script_ids.append(script_id2)
    print("✅ Function saved")

    result2 = await client.call_function("docs_by_category_py", None)
    print("📊 Documents by category:")
    for group in result2["records"]:
        print(f"   {group}")
    print(f"⏱️  Execution time: {result2['stats']['execution_time_ms']}ms\n")

    # Cleanup
    print("🧹 Cleaning up...")
    for script_id in script_ids:
        try:
            await client.delete_function(script_id)
        except Exception:
            pass
    try:
        await client.delete_collection("search_docs_py")
    except Exception:
        pass
    print("✅ Cleanup complete\n")

    print("✅ All search script examples finished!")


if __name__ == "__main__":
    asyncio.run(main())
