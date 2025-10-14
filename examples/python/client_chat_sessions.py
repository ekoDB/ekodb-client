"""
ekoDB Python Client - Chat Session Management Example

This example demonstrates advanced chat session management including:
- Creating sessions
- Sending messages
- Retrieving messages
- Updating sessions
- Branching sessions
- Listing sessions
- Deleting sessions
"""

import asyncio
import os
from ekodb_client import Client
from dotenv import load_dotenv

load_dotenv()


async def main():
    print("=== ekoDB Chat Session Management Example ===\n")

    # Create client
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "")
    client = Client.new(base_url, api_key)

    collection = "client_chat_sessions_python"

    # Step 1: Insert sample data
    print("=== Inserting Sample Data ===")
    await client.insert(
        collection,
        {
            "name": "ekoDB",
            "description": "A high-performance database product",
            "price": 99,
        },
    )
    print("✓ Inserted sample product\n")

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

    # Step 3: Send messages in the session
    print("=== Sending Messages ===")
    msg1 = await client.chat_message(chat_id, "What products are available?")
    print("✓ Message 1 sent")
    print(
        f"  Response: {msg1['responses'][0] if msg1['responses'] else 'No response'}\n"
    )

    msg2 = await client.chat_message(chat_id, "What is the price?")
    print("✓ Message 2 sent")
    print(
        f"  Response: {msg2['responses'][0] if msg2['responses'] else 'No response'}\n"
    )

    # Step 4: Get session messages
    print("=== Retrieving Session Messages ===")
    messages_response = await client.get_chat_session_messages(
        chat_id,
        limit=10,
        skip=None,
        sort="asc",
    )
    print(f"✓ Retrieved {len(messages_response['messages'])} messages\n")

    # Step 5: Update session
    print("=== Updating Session ===")
    await client.update_chat_session(
        chat_id,
        system_prompt="You are an expert product consultant.",
        llm_model=None,
    )
    print("✓ Session updated\n")

    # Step 6: Branch the session
    print("=== Branching Session ===")
    branched = await client.branch_chat_session(
        parent_id=chat_id,
        branch_point_idx=0,
        collections=[("products", [])],
        llm_provider="openai",
        llm_model="gpt-4",
    )
    branch_id = branched["chat_id"]
    print(f"✓ Created branch: {branch_id}")
    print(f"  Parent: {chat_id}\n")

    # Step 7: List all sessions
    print("=== Listing Sessions ===")
    sessions_list = await client.list_chat_sessions(
        limit=10,
        skip=None,
        sort="desc",
    )
    print(f"✓ Found {len(sessions_list['sessions'])} sessions")
    for i, s in enumerate(sessions_list["sessions"]):
        title = s.get("title", "Untitled")
        print(f"  Session {i + 1}: {s['chat_id']} ({title})")
    print()

    # Step 8: Delete branch session
    print("=== Deleting Branch Session ===")
    await client.delete_chat_session(branch_id)
    print(f"✓ Deleted branch session: {branch_id}\n")

    # Cleanup: Delete the collection (chat sessions are managed by server)
    print("=== Cleanup ===")
    await client.delete_collection(collection)
    print("✓ Deleted collection\n")

    print("✓ All session management operations completed successfully")


if __name__ == "__main__":
    asyncio.run(main())
