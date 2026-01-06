"""
AI Scripts Example - Chat and Embed Operations

Demonstrates AI operations in scripts:
- Chat completions with context
- Embedding generation
- Simple AI workflows
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
    from ekodb_client import Client, Stage, ChatMessage

    client = Client.new(BASE_URL, API_KEY)

    print("🚀 ekoDB Python AI Scripts Example\n")

    # Setup test data
    print("📋 Setting up test data...")
    try:
        await client.delete_collection("ai_articles_py")
    except Exception:
        pass

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
        await client.insert("ai_articles_py", article)
    print(f"✅ Created {len(articles)} articles\n")

    script_ids = []

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
                "gpt-4",
                0.7,
            )
        ],
        "tags": ["ai", "chat"],
    }
    script_id1 = await client.save_script(script1)
    script_ids.append(script_id1)
    print("✅ Chat script saved")

    result1 = await client.call_script("ai_assistant_py", None)
    print("🤖 AI Response:")
    if result1.get("records"):
        response = result1["records"][0]
        response_data = response.get("response", "No response")
        if isinstance(response_data, dict) and "value" in response_data:
            print(f"   {response_data['value']}")
        else:
            print(f"   {response_data}")
    print(f"⏱️  Execution time: {result1['stats']['execution_time_ms']}ms\n")

    # Example 2: Embed Generation
    print("📝 Example 2: Generate Embeddings\n")
    script2 = {
        "label": "generate_embedding_py",
        "name": "Generate Embedding",
        "description": "Generate embedding for text",
        "version": "1.0",
        "parameters": {
            "text": {"required": True, "description": "Text to embed"},
        },
        "functions": [Stage.embed("text", "embedding")],
        "tags": ["ai", "embed"],
    }
    script_id2 = await client.save_script(script2)
    script_ids.append(script_id2)
    print("✅ Embed script saved")

    result2 = await client.call_script(
        "generate_embedding_py", {"text": "ekoDB is a powerful database"}
    )
    print("📊 Embedding generated")
    if result2.get("records"):
        embedding = result2["records"][0].get("embedding", [])
        if isinstance(embedding, dict):
            embedding = embedding.get("value", [])
        print(f"   Dimensions: {len(embedding) if embedding else 'N/A'}")
    print(f"⏱️  Execution time: {result2['stats']['execution_time_ms']}ms\n")

    # Cleanup
    print("🧹 Cleaning up...")
    for script_id in script_ids:
        try:
            await client.delete_script(script_id)
        except Exception:
            pass
    try:
        await client.delete_collection("ai_articles_py")
    except Exception:
        pass
    print("✅ Cleanup complete\n")

    print("✅ All AI script examples finished!")
    print("\n💡 This example demonstrates:")
    print("   ✅ Chat completions with system/user messages")
    print("   ✅ Embedding generation for text")


if __name__ == "__main__":
    asyncio.run(main())
