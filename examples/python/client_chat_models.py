#!/usr/bin/env python3
"""Chat Models API Example - Using ekodb_client Python library

This example demonstrates the Chat Models API for retrieving available LLM providers and models.
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv

from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def chat_models_examples():
    """Run Chat Models API examples"""
    print("✓ Client created")

    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)

    # Example 1: Get all chat models from all providers
    print("\n=== Get All Chat Models ===")
    try:
        models = await client.get_chat_models()
        print(f"OpenAI models: {models.get('openai', [])}")
        print(f"Anthropic models: {models.get('anthropic', [])}")
        print(f"Perplexity models: {models.get('perplexity', [])}")
    except Exception as e:
        print(f"GetChatModels error (expected if no LLM configured): {e}")

    # Example 2: Get models for a specific provider
    print("\n=== Get OpenAI Models ===")
    try:
        openai_models = await client.get_chat_model("openai")
        print(f"OpenAI models: {openai_models}")
    except Exception as e:
        print(f"GetChatModel error (expected if no LLM configured): {e}")

    # Example 3: Get Anthropic models
    print("\n=== Get Anthropic Models ===")
    try:
        anthropic_models = await client.get_chat_model("anthropic")
        print(f"Anthropic models: {anthropic_models}")
    except Exception as e:
        print(f"GetChatModel error (expected if no LLM configured): {e}")

    print("\n✓ Chat Models API example complete")


if __name__ == "__main__":
    asyncio.run(chat_models_examples())
