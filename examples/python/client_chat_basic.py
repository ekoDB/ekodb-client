"""
ekoDB Python Client - Basic Chat Example

This example demonstrates basic chat functionality with ekoDB.
"""

import asyncio
import os
from ekodb_client import Client
from dotenv import load_dotenv

load_dotenv()


async def main():
    print("=== ekoDB Chat Basic Example ===\n")

    # Create client
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "")
    client = Client.new(base_url, api_key)

    collection = "client_chat_basic_python"

    # Step 1: Insert sample data
    print("=== Inserting Sample Data ===")
    await client.insert(
        collection,
        {
            "name": "ekoDB",
            "description": "A high-performance database product with AI capabilities",
            "price": 99,
        },
    )
    await client.insert(
        collection,
        {
            "name": "ekoDB Pro",
            "description": "Enterprise edition product with advanced features",
            "price": 299,
        },
    )
    await client.insert(
        collection,
        {
            "name": "ekoDB Cloud",
            "description": "Fully managed cloud database service product",
            "price": 499,
        },
    )
    print("✓ Inserted 3 sample documents\n")

    # Step 2: Create a chat session
    print("=== Creating Chat Session ===")
    session = await client.create_chat_session(
        collections=[(collection, [])],
        llm_provider="openai",
        llm_model="gpt-4",
        system_prompt="You are a helpful assistant for product information.",
    )
    chat_id = session["chat_id"]
    print(f"✓ Created session: {chat_id}\n")

    # Step 3: Send a chat message
    print("=== Sending Chat Message ===")
    response = await client.chat_message(
        chat_id, "What products are available and what are their prices?"
    )

    print(f"Message ID: {response['message_id']}")
    print("\n=== AI Response ===")
    for r in response["responses"]:
        print(r)
    print(f"\nExecution Time: {response['execution_time_ms']}ms")

    if "token_usage" in response:
        print("\n=== Token Usage ===")
        print(f"Prompt tokens: {response['token_usage']['prompt_tokens']}")
        print(f"Completion tokens: {response['token_usage']['completion_tokens']}")
        print(f"Total tokens: {response['token_usage']['total_tokens']}")

    # Cleanup: Delete the collection (chat session is managed by server)
    print("\n=== Cleanup ===")
    await client.delete_collection(collection)
    print("✓ Deleted collection")

    print("\n✓ Chat completed successfully")


if __name__ == "__main__":
    asyncio.run(main())
