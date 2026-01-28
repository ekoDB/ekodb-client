"""
ekoDB as Edge Cache Example

Demonstrates using ekoDB Functions as an edge cache:
- Create a cache function that checks cache first
- On cache miss, fetch from external API and store
- On cache hit, return cached data instantly
"""

import asyncio
import json
import os
import time
from datetime import datetime
from pathlib import Path

from dotenv import load_dotenv

from ekodb_client import Client, Stage

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def edge_cache_example():
    client = Client.new(BASE_URL, API_KEY)

    print("=== ekoDB as Edge Cache - Simple Example ===\n")

    print("Creating edge cache function...")

    # Simple passthrough cache pattern
    cache_script = {
        "label": "cache_api_call_py",
        "name": "Cache External API Call",
        "description": "Database as edge: Check cache → Call API if miss → Store result → Return",
        "parameters": {
            "cache_key": {
                "required": True,
                "description": "Unique cache key",
            },
            "api_url": {
                "required": True,
                "description": "External API URL to call",
            },
            "ttl_seconds": {
                "required": False,
                "default": 300,
                "description": "Time to live in seconds",
            },
        },
        "version": "1.0",
        "tags": ["cache", "edge"],
        "functions": [
            # 1. Check KV cache
            Stage.kv_get("{{cache_key}}"),
            # 2. If cache exists, return it; else fetch from API
            Stage.if_condition(
                # KvGet returns {value: ...} on hit, {value: null} on miss
                # So we check if "value" is not null to detect cache hit
                {
                    "type": "Not",
                    "value": {
                        "condition": {
                            "type": "FieldEquals",
                            "value": {"field": "value", "value": None},
                        }
                    },
                },
                # Cache hit - return cached data
                [Stage.project(["value"], False)],
                # Cache miss - fetch external API and store in KV
                [
                    Stage.http_request(
                        "{{api_url}}", "GET", {"User-Agent": "ekoDB-Edge-Cache"}
                    ),
                    # Store in KV with 5 minute TTL
                    Stage.kv_set(
                        "{{cache_key}}",
                        "{{http_response}}",
                        300,
                    ),
                    # Retrieve the cached data to return
                    Stage.kv_get("{{cache_key}}"),
                    Stage.project(["value"], False),
                ],
            ),
        ],
    }

    script_id = await client.save_script(cache_script)
    print(f"✓ Edge cache script created: {script_id}\n")

    # Test it - First call hits API
    print("Call 1: Cache miss (fetches from API)")
    start1 = time.time()
    result1 = await client.call_script(
        "cache_api_call_py",
        {
            "cache_key": "weather_nyc",
            "api_url": "https://api.open-meteo.com/v1/forecast?latitude=40.7128&longitude=-74.0060&current=temperature_2m",
            "ttl_seconds": 300,
        },
    )
    duration1 = (time.time() - start1) * 1000
    print(f"Response time: {duration1:.0f}ms")
    print(f"Result: {json.dumps(result1, indent=2)}")

    # Test it again - Second call hits cache
    print("\nCall 2: Cache hit (served from ekoDB)")
    start2 = time.time()
    result2 = await client.call_script(
        "cache_api_call_py",
        {
            "cache_key": "weather_nyc",
            "api_url": "https://api.open-meteo.com/v1/forecast?latitude=40.7128&longitude=-74.0060&current=temperature_2m",
        },
    )
    duration2 = (time.time() - start2) * 1000
    speedup = duration1 / duration2 if duration2 > 0 else 0
    print(f"Response time: {duration2:.0f}ms ({speedup:.1f}x faster!)")
    print(f"Result: {json.dumps(result2, indent=2)}")

    print("\n=== The Magic ===")
    print("- Your DATABASE is your EDGE")
    print("- No Redis needed")
    print("- No CDN needed")
    print("- No cache invalidation logic needed (TTL handles it)")
    print("- With ripples: All nodes auto-sync cache")
    print("- One service: Database + Cache + Edge Functions")

    print("\n✓ Example complete!\n")


if __name__ == "__main__":
    asyncio.run(edge_cache_example())
