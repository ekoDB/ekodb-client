"""ekoDB Python Client - Chat Message Stream (SSE) Example

Demonstrates streaming chat responses via HTTP SSE (Server-Sent Events).
This is a simpler alternative to WebSocket streaming.
"""

import asyncio
import os

from ekodb_client import Client


async def main():
    print("=== ekoDB Chat Message Stream (SSE) Example (Python) ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY")
    if not api_key:
        raise ValueError("API_BASE_KEY environment variable is required")

    client = Client.new(base_url, api_key)

    # Create a chat session
    session = await client.create_chat_session(
        collections=[],
        llm_provider="openai",
        llm_model="gpt-4o-mini",
        system_prompt="You are a helpful assistant.",
    )
    chat_id = session["chat_id"]
    print(f"Created session: {chat_id}")
    try:
        # Stream a chat message via SSE
        print("\nStreaming response for: 'What is ekoDB?'\n")
        stream = await client.chat_message_stream(chat_id, "What is ekoDB?")
        completed = False

        while True:
            event = await stream.recv()
            if event is None:
                break

            if event["type"] == "chunk":
                print(event["content"], end="", flush=True)
            elif event["type"] == "end":
                completed = True
                print("\n\n--- Stream complete ---")
                print(f"Message ID: {event.get('message_id')}")
                print(f"Execution time: {event.get('execution_time_ms')}ms")
                if event.get("context_window"):
                    print(f"Context window: {event['context_window']} tokens")
            elif event["type"] == "error":
                raise RuntimeError(f"Chat stream failed: {event.get('error')}")

        if not completed:
            raise RuntimeError("Chat stream ended before an end event")
    finally:
        await client.delete_chat_session(chat_id)

    print("\n✓ Chat message stream example completed")


if __name__ == "__main__":
    asyncio.run(main())
