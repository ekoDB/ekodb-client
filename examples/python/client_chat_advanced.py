"""
ekoDB Python Client - Advanced Chat Features Example

This example demonstrates advanced chat features:
- Message regeneration
- Message editing
- Message deletion
- Session merging
- Forgotten messages (exclude from context)
"""

import asyncio
import os
from ekodb_client import Client
from dotenv import load_dotenv

load_dotenv()


async def main():
    print("=== ekoDB Advanced Chat Features Example ===\n")

    # Create client
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "")
    client = Client.new(base_url, api_key)

    collection = "client_chat_advanced_python"

    # Insert sample data
    print("=== Inserting Sample Data ===")
    await client.insert(
        collection,
        {
            "name": "ekoDB",
            "description": "High-performance database product",
            "price": 99,
        },
    )
    print("✓ Inserted sample product\n")

    # Create a chat session
    print("=== Creating Chat Session ===")
    session = await client.create_chat_session(
        collections=[(collection, [])],
        llm_provider="openai",
        llm_model="gpt-4",
        system_prompt="You are a helpful product assistant.",
    )
    chat_id = session["chat_id"]
    print(f"✓ Created session: {chat_id}\n")

    # Send initial message
    print("=== Sending Initial Message ===")
    msg1 = await client.chat_message(chat_id, "What products are available?")
    print("✓ Message sent")
    if msg1["responses"]:
        print(f"  Response: {msg1['responses'][0]}\n")

    # Send a second message
    await client.chat_message(chat_id, "What is the price of ekoDB?")
    print("✓ Second message sent\n")

    # Get messages to find user and assistant message IDs
    messages = await client.get_chat_session_messages(chat_id, limit=10, sort="desc")

    # Debug: print message structure
    if messages["messages"]:
        print(f"Debug: Found {len(messages['messages'])} messages")
        print(f"Debug: First message keys: {messages['messages'][0].keys()}")
        print(f"Debug: First message role: {messages['messages'][0].get('role')}")

    # Find assistant message
    assistant_message = next(
        (
            msg
            for msg in messages["messages"]
            if (
                isinstance(msg.get("role"), dict)
                and msg.get("role", {}).get("value") == "assistant"
            )
            or msg.get("role") == "assistant"
        ),
        None,
    )
    if not assistant_message:
        raise Exception(
            f'Could not find assistant message. Total messages: {len(messages["messages"])}'
        )
    assistant_message_id = assistant_message["id"]

    # Find user message
    user_message = next(
        (
            msg
            for msg in messages["messages"]
            if (
                isinstance(msg.get("role"), dict)
                and msg.get("role", {}).get("value") == "user"
            )
            or msg.get("role") == "user"
        ),
        None,
    )
    if not user_message:
        raise Exception("Could not find user message")
    user_message_id = user_message["id"]

    # Feature 1: Regenerate Message
    print("=== Feature 1: Regenerate AI Response ===")
    regenerated = await client.regenerate_chat_message(chat_id, assistant_message_id)
    print("✓ Message regenerated")
    if regenerated["responses"]:
        print(f"  New response: {regenerated['responses'][0]}\n")

    # Feature 2: Update Message Content
    print("=== Feature 2: Edit Message ===")
    await client.update_chat_message(
        chat_id, user_message_id, "What is the updated price of ekoDB?"
    )
    print("✓ Message content updated\n")

    # Feature 3: Toggle Forgotten Status
    print("=== Feature 3: Mark Message as Forgotten ===")
    await client.toggle_forgotten_message(chat_id, user_message_id, True)
    print("✓ Message marked as forgotten (excluded from LLM context)\n")

    # Unmark as forgotten
    await client.toggle_forgotten_message(chat_id, user_message_id, False)
    print("✓ Message unmarked as forgotten\n")

    # Feature 4: Session Merging
    print("=== Feature 4: Merge Chat Sessions ===")

    # Create a second session
    session2 = await client.create_chat_session(
        collections=[("products", [])],
        llm_provider="openai",
        llm_model="gpt-4",
    )
    chat_id2 = session2["chat_id"]
    print(f"✓ Created second session: {chat_id2}")

    # Send a message in the second session
    await client.chat_message(chat_id2, "Tell me more about the features")
    print("✓ Sent message in second session")

    # Merge sessions
    merge_result = await client.merge_chat_sessions(
        source_chat_ids=[chat_id2],
        target_chat_id=chat_id,
        merge_strategy="Chronological",
    )
    print("✓ Sessions merged successfully")
    print(f"  Total messages in merged session: {merge_result['message_count']}\n")

    # Feature 5: Delete Message
    print("=== Feature 5: Delete Message ===")
    await client.delete_chat_message(chat_id, user_message_id)
    print("✓ Message deleted\n")

    # Verify message count after deletion
    session_details = await client.get_chat_session(chat_id)
    print(f"✓ Messages remaining: {session_details['message_count']}\n")

    # Cleanup
    print("=== Cleanup ===")
    await client.delete_chat_session(chat_id)
    print("✓ Deleted session")
    await client.delete_collection(collection)
    print("✓ Deleted collection\n")

    print("✓ All advanced chat features demonstrated successfully!")


if __name__ == "__main__":
    asyncio.run(main())
