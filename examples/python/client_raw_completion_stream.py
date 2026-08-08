"""
ekoDB Python Client - Raw Completion Stream (SSE) Example

Demonstrates using SSE streaming for raw LLM completions.
SSE streaming keeps the connection alive with heartbeat events, making it
ideal for deployed instances where reverse proxies may kill idle connections.
"""

import asyncio
import os
from ekodb_client import Client
from dotenv import load_dotenv

load_dotenv()


async def main():
    print("=== ekoDB Raw Completion Stream (SSE) Example ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "")
    client = Client.new(base_url, api_key)

    # --- 1. Basic streaming raw completion ---
    print("--- Basic SSE Raw Completion ---")
    response = await client.raw_completion_stream(
        system_prompt="You are a helpful assistant. Keep responses concise.",
        message="What are the three primary colors?",
        max_tokens=256,
    )
    print(f"Response: {response['content']}\n")

    # --- 2. Structured output via SSE ---
    print("--- Structured Output via SSE ---")
    response = await client.raw_completion_stream(
        system_prompt="Output only valid JSON. No other text.",
        message="Return a JSON array of 3 planets with name and diameter_km fields.",
        max_tokens=512,
    )
    print(f"JSON response: {response['content']}\n")

    # --- 3. Comparison: blocking HTTP vs SSE ---
    # On deployed instances behind reverse proxies, raw_completion() may timeout.
    # raw_completion_stream() keeps the connection alive with SSE heartbeats.
    print("--- Blocking HTTP (for comparison) ---")
    response = await client.raw_completion(
        system_prompt="You are a helpful assistant.",
        message="Say hello in one sentence.",
        max_tokens=64,
    )
    print(f"Blocking response: {response['content']}\n")

    print("=== Done ===")


if __name__ == "__main__":
    asyncio.run(main())
