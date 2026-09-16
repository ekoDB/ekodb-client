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
    print("🚀 ekoDB Python Search Functions Example\n")

    # Setup test data
    print("📋 Setting up test data...")
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

    inserted_ids = []
    for doc in documents:
        inserted = await client.insert("search_docs_py", doc)
        document_id = field_value(inserted, "id")
        if not document_id:
            raise AssertionError("document insert did not return an ID")
        inserted_ids.append(document_id)
    if len(set(inserted_ids)) != 5:
        raise AssertionError("document inserts did not return 5 unique IDs")
    print(f"✅ Inserted {len(documents)} documents\n")

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
    script_id1 = await save_or_update(client, script1)
    script_ids.append(script_id1)
    print("✅ Function saved")

    result1 = await client.call_function("list_all_docs_py", None)
    print(f"📊 Found {len(result1['records'])} documents")
    if len(result1["records"]) != 5:
        raise AssertionError("list function did not return all 5 documents")
    returned_titles = set()
    for i, doc in enumerate(result1["records"]):
        title = field_value(doc, "title")
        category = field_value(doc, "category")
        if not isinstance(title, str) or category not in {"AI", "Database"}:
            raise AssertionError(f"malformed search record: {doc!r}")
        returned_titles.add(title)
        print(f"   {i + 1}. {title} ({category})")
    if returned_titles != {doc["title"] for doc in documents}:
        raise AssertionError(f"unexpected document titles: {returned_titles!r}")
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
    script_id2 = await save_or_update(client, script2)
    script_ids.append(script_id2)
    print("✅ Function saved")

    result2 = await client.call_function("docs_by_category_py", None)
    print("📊 Documents by category:")
    category_counts = {}
    for group in result2["records"]:
        print(f"   {group}")
        category_counts[field_value(group, "category")] = field_value(group, "count")
    if category_counts != {"AI": 2, "Database": 3}:
        raise AssertionError(f"unexpected category groups: {category_counts!r}")
    print(f"⏱️  Execution time: {result2['stats']['execution_time_ms']}ms\n")


async def main():
    from ekodb_client import Client

    client = Client.new(BASE_URL, API_KEY)
    collection = "search_docs_py"
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
    print("✅ All search script examples finished!")


if __name__ == "__main__":
    asyncio.run(main())
