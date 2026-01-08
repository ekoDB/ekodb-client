"""
SWR (Stale-While-Revalidate) Pattern Example

Demonstrates using ekoDB Functions as an SWR cache:
- Check cache first, return if exists
- On cache miss, fetch from external API
- Store result with TTL for auto-expiration
"""

import asyncio
import json
import os
import time
from datetime import datetime
from pathlib import Path

from dotenv import load_dotenv

env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def main():
    from ekodb_client import Client, Stage

    client = Client.new(BASE_URL, API_KEY)

    # Cleanup any stale collections from previous runs
    try:
        await client.delete_collection("github_cache_py")
    except Exception:
        pass

    # Create collection without schema to allow any data structure
    await client.create_collection("github_cache_py", None)

    print("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n")

    print("Step 1: Create SWR function that acts as edge cache")

    swr_script = {
        "label": "fetch_github_user_py",
        "name": "Fetch GitHub User with Cache",
        "description": "SWR pattern: Check cache, fetch from GitHub API if stale",
        "parameters": {
            "username": {"required": True, "description": "GitHub username to fetch"},
            "ttl": {
                "required": False,
                "default": 300,
                "description": "Cache TTL in seconds",
            },
            "cached_at": {
                "required": True,
                "description": "UTC timestamp when cached",
            },
        },
        "version": "1.0",
        "tags": ["swr", "github", "cache"],
        "functions": [
            # Check KV cache for user data
            Stage.kv_get("github:user:{{username}}"),
            Stage.if_condition(
                {"type": "HasRecords"},
                # Cache hit - return cached data
                [Stage.project(["data"], False)],
                # Cache miss - fetch from API and cache
                [
                    Stage.http_request(
                        "https://api.github.com/users/{{username}}",
                        "GET",
                        {"User-Agent": "ekoDB-SWR-Example"},
                    ),
                    # Store in KV with 5 minute TTL
                    Stage.kv_set(
                        "github:user:{{username}}",
                        "{{http_response}}",
                        300,
                    ),
                    # Retrieve the cached data to return
                    Stage.kv_get("github:user:{{username}}"),
                    Stage.project(["data"], False),
                ],
            ),
        ],
    }

    script_id = await client.save_script(swr_script)
    print(f"✓ Created SWR script: {swr_script['label']} ({script_id})\n")

    print("Step 2: First call - Cache miss, fetches from GitHub API")
    result1 = await client.call_script(
        "fetch_github_user_py",
        {
            "username": "torvalds",
            "ttl": 300,
            "cached_at": datetime.now().isoformat(),
        },
    )
    print(f"Result: {json.dumps(result1, indent=2)}")
    print("✓ Data fetched from external API and cached\n")

    print("Step 3: Second call - Cache hit, instant response from ekoDB")
    start = time.time()
    result2 = await client.call_script(
        "fetch_github_user_py",
        {"username": "torvalds", "cached_at": datetime.now().isoformat()},
    )
    duration = (time.time() - start) * 1000
    print(f"Response time: {duration:.0f}ms (served from cache)")
    print("✓ Lightning fast cache hit\n")

    # Cleanup
    print("🧹 Cleaning up...")
    try:
        await client.delete_script(script_id)
        await client.delete_collection("github_cache_py")
    except Exception:
        pass
    print("✓ Cleanup complete\n")

    print("=== SWR Pattern Summary ===")
    print("✅ Cache miss → Fetch from API → Store in ekoDB")
    print("✅ Cache hit → Instant response from ekoDB")
    print("✅ TTL handles automatic cache invalidation")


if __name__ == "__main__":
    asyncio.run(main())
