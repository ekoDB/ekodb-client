#!/usr/bin/env python3
"""
WebSocket Chat Streaming Example - Real-time LLM responses

Demonstrates using the WebSocket client to send a chat message
and receive streaming responses, including tool calls.

Requires: ekoDB server running on localhost:8080
"""

import os
import asyncio
from dotenv import load_dotenv
from ekodb_client import Client

load_dotenv()

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
WS_URL = os.getenv("WS_BASE_URL", "ws://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def main():
    print("=== WebSocket Chat Streaming Example (Python) ===\n")

    # Create client
    client = Client.new(BASE_URL, API_KEY)

    # Create a chat session first (via HTTP)
    session = await client.create_chat_session(
        {
            "system_prompt": "You are a helpful assistant.",
        }
    )
    chat_id = session["chat_id"]
    print(f"Created chat session: {chat_id}")

    # Connect WebSocket
    ws = await client.websocket(WS_URL)

    # Send a chat message and stream the response
    print("\nSending message: 'What is the capital of France?'")
    receiver = await ws.chat_send(chat_id, "What is the capital of France?")

    # Receive streaming events
    full_response = ""
    while True:
        event = await receiver.recv()
        if event is None:
            break

        if event["type"] == "chunk":
            # Accumulate text chunks
            full_response += event["content"]
            print(event["content"], end="", flush=True)
        elif event["type"] == "end":
            print(f"\n\n--- Stream ended ---")
            print(f"Message ID: {event['message_id']}")
            print(f"Execution time: {event['execution_time_ms']}ms")
            if "token_usage" in event:
                print(f"Token usage: {event['token_usage']}")
            break
        elif event["type"] == "tool_call":
            print(f"\n[Tool Call] {event['tool_name']}({event['arguments']})")
            # Send tool result back
            await ws.send_tool_result(
                chat_id,
                event["call_id"],
                True,
                {"result": "Tool executed successfully"},
            )
        elif event["type"] == "error":
            print(f"\n[Error] {event['error']}")
            break

    print(f"\nFull response: {full_response[:200]}...")


async def chat_with_client_tools():
    """Example of registering client-side tools."""
    print("\n=== Chat with Client Tools ===\n")

    client = Client.new(BASE_URL, API_KEY)
    ws = await client.websocket(WS_URL)

    session = await client.create_chat_session(
        {
            "system_prompt": "You are a helpful assistant with access to tools.",
        }
    )
    chat_id = session["chat_id"]

    # Register client-side tools
    await ws.register_client_tools(
        chat_id,
        [
            (
                "get_weather",
                "Get the current weather for a location",
                {
                    "type": "object",
                    "properties": {
                        "location": {"type": "string", "description": "City name"},
                    },
                    "required": ["location"],
                },
            ),
        ],
    )
    print("Registered client tools")

    # Send message that might trigger tool use
    receiver = await ws.chat_send(chat_id, "What's the weather in Paris?")

    while True:
        event = await receiver.recv()
        if event is None:
            break

        if event["type"] == "chunk":
            print(event["content"], end="", flush=True)
        elif event["type"] == "tool_call":
            print(f"\n[Tool Call] {event['tool_name']}({event['arguments']})")
            # Execute the tool and return result
            await ws.send_tool_result(
                chat_id,
                event["call_id"],
                True,
                {"temperature": "22°C", "condition": "Sunny"},
            )
        elif event["type"] == "end":
            print(f"\n--- Done ({event['execution_time_ms']}ms) ---")
            break
        elif event["type"] == "error":
            print(f"\n[Error] {event['error']}")
            break


if __name__ == "__main__":
    asyncio.run(main())
    # asyncio.run(chat_with_client_tools())
