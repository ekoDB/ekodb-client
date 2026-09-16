"""
AI Functions Example - Chat and Embed Operations

Demonstrates AI operations in scripts:
- Chat completions with context
- Embedding generation
- Simple AI workflows
"""

import asyncio
import os
from pathlib import Path

from dotenv import load_dotenv

from ekodb_client import ChatMessage, Client, Stage

env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
COLLECTION = "ai_articles_py"


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


async def run_examples(client, script_ids):
    print("🚀 ekoDB Python AI Functions Example\n")

    # Setup test data
    print("📋 Setting up test data...")

    articles = [
        {
            "title": "Getting Started with ekoDB",
            "content": "ekoDB is a high-performance database with AI capabilities...",
            "status": "published",
        },
        {
            "title": "Advanced Query Patterns",
            "content": "Learn how to build complex queries in ekoDB...",
            "status": "published",
        },
    ]

    for article in articles:
        await client.insert(COLLECTION, article)
    print(f"✅ Created {len(articles)} articles\n")

    # Example 1: Simple Chat Completion
    print("📝 Example 1: Simple Chat Completion\n")
    script1 = {
        "label": "ai_assistant_py",
        "name": "AI Chat Assistant",
        "description": "Simple AI chat completion",
        "version": "1.0",
        "parameters": {},
        "functions": [
            Stage.chat(
                [
                    ChatMessage.system(
                        "You are a helpful database assistant. Be concise."
                    ),
                    ChatMessage.user(
                        "What are the benefits of using vector databases?"
                    ),
                ],
                "gpt-4o-mini",
                0.7,
            )
        ],
        "tags": ["ai", "chat"],
    }
    script_id1 = await save_or_update(client, script1)
    script_ids.append(script_id1)
    print("✅ Chat script saved")

    result1 = await client.call_function("ai_assistant_py", None)
    print("🤖 AI Response:")
    if not result1.get("records"):
        raise RuntimeError("Chat function returned no records")
    response_data = result1["records"][0].get("response")
    if isinstance(response_data, dict):
        response_data = response_data.get("value")
    if not isinstance(response_data, str) or not response_data.strip():
        raise RuntimeError("Chat function returned no response text")
    print(f"   {response_data}")
    print(f"⏱️  Execution time: {result1['stats']['execution_time_ms']}ms\n")

    # Example 2: Embed Generation
    print("📝 Example 2: Generate Embeddings\n")
    script2 = {
        "label": "generate_embedding_py",
        "name": "Generate Embedding",
        "description": "Generate embedding for text",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": COLLECTION},
            Stage.embed("content", "embedding"),
        ],
        "tags": ["ai", "embed"],
    }
    script_id2 = await save_or_update(client, script2)
    script_ids.append(script_id2)
    print("✅ Embed script saved")

    result2 = await client.call_function("generate_embedding_py", None)
    if len(result2.get("records", [])) != len(articles):
        raise RuntimeError(
            f"Embedding function returned {len(result2.get('records', []))} records; "
            f"expected {len(articles)}"
        )
    dimensions = []
    for record in result2["records"]:
        embedding = record.get("embedding")
        if isinstance(embedding, dict):
            embedding = embedding.get("value")
        if not isinstance(embedding, list) or not embedding:
            raise RuntimeError("Embedding function returned an empty embedding vector")
        dimensions.append(len(embedding))
    print(f"📊 Generated {len(dimensions)} embeddings")
    print(f"   Dimensions: {dimensions[0]}")
    print(f"⏱️  Execution time: {result2['stats']['execution_time_ms']}ms\n")


async def main():
    client = Client.new(BASE_URL, API_KEY)
    script_ids = []
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
        await client.delete_collection(COLLECTION)
    except Exception as error:  # noqa: BLE001 - preserve the operation error
        cleanup_errors.append(f"collection {COLLECTION}: {error}")

    if operation_error is not None:
        if cleanup_errors:
            print("⚠️  Cleanup errors: " + "; ".join(cleanup_errors))
        raise operation_error
    if cleanup_errors:
        raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))
    print("✅ Cleanup complete\n")

    print("✅ All AI script examples finished!")
    print("\n💡 This example demonstrates:")
    print("   ✅ Chat completions with system/user messages")
    print("   ✅ Embedding generation for text")


if __name__ == "__main__":
    asyncio.run(main())
