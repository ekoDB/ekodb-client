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


async def main(resources):
    print("=== WebSocket Chat Streaming Example (Python) ===\n")

    # Create client
    client = Client.new(BASE_URL, API_KEY)
    resources["client"] = client

    # Create a chat session first (via HTTP)
    session = await client.create_chat_session(
        collections=[],
        llm_provider="openai",
        llm_model="gpt-4o-mini",
        system_prompt="You are a helpful assistant.",
    )
    chat_id = session["chat_id"]
    resources["chat_ids"].append(chat_id)
    print(f"Created chat session: {chat_id}")

    # Connect WebSocket
    ws = await client.websocket(WS_URL)
    resources["websockets"].append(ws)

    # Send a chat message and stream the response
    print("\nSending message: 'What is the capital of France?'")
    receiver = await ws.chat_send(chat_id, "What is the capital of France?")

    # Receive streaming events
    full_response = ""
    completed = False
    while True:
        event = await receiver.recv()
        if event is None:
            break

        if event["type"] == "chunk":
            # Accumulate text chunks
            full_response += event["content"]
            print(event["content"], end="", flush=True)
        elif event["type"] == "end":
            completed = True
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
            raise RuntimeError(f"WebSocket chat failed: {event['error']}")

    if not completed:
        raise RuntimeError("WebSocket chat ended before an end event")

    print(f"\nFull response: {full_response[:200]}...")


async def _chat_with_client_tools(resources):
    """Example of registering client-side tools."""
    print("\n=== Chat with Client Tools ===\n")

    client = Client.new(BASE_URL, API_KEY)
    resources["client"] = client
    ws = await client.websocket(WS_URL)
    resources["websockets"].append(ws)

    session = await client.create_chat_session(
        collections=[],
        llm_provider="openai",
        llm_model="gpt-4o-mini",
        system_prompt="You are a helpful assistant with access to tools.",
    )
    chat_id = session["chat_id"]
    resources["chat_ids"].append(chat_id)

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

    completed = False
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
            completed = True
            print(f"\n--- Done ({event['execution_time_ms']}ms) ---")
            break
        elif event["type"] == "error":
            raise RuntimeError(f"WebSocket chat failed: {event['error']}")

    if not completed:
        raise RuntimeError("WebSocket chat ended before an end event")


async def chat_with_client_tools():
    """Run the client-tools flow and always remove its chat session."""
    resources = {"client": None, "chat_ids": [], "websockets": []}
    try:
        await _chat_with_client_tools(resources)
    finally:
        cleanup_errors = []
        for ws in resources["websockets"]:
            try:
                await ws.close()
            except Exception as error:
                cleanup_errors.append(f"websocket: {error}")
        if resources["client"]:
            for chat_id in resources["chat_ids"]:
                try:
                    await resources["client"].delete_chat_session(chat_id)
                except Exception as error:
                    cleanup_errors.append(f"session {chat_id}: {error}")
        if cleanup_errors:
            raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))


if __name__ == "__main__":

    async def run():
        resources = {"client": None, "chat_ids": [], "websockets": []}
        primary_error = None
        try:
            await main(resources)
        except BaseException as error:
            primary_error = error

        cleanup_errors = []
        for ws in resources["websockets"]:
            try:
                await ws.close()
            except Exception as error:
                cleanup_errors.append(f"websocket: {error}")
        if resources["client"]:
            for chat_id in resources["chat_ids"]:
                try:
                    await resources["client"].delete_chat_session(chat_id)
                except Exception as error:
                    cleanup_errors.append(f"session {chat_id}: {error}")
        if primary_error:
            if cleanup_errors:
                print("Cleanup also failed: " + "; ".join(cleanup_errors))
            raise primary_error
        if cleanup_errors:
            raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))

    asyncio.run(run())
    # asyncio.run(chat_with_client_tools())
