"""
ekoDB SWR (Stale-While-Revalidate) Pattern Example

Demonstrates using ekoDB as an edge cache with automatic invalidation.
"""

import asyncio
import json
import os
import time
from pathlib import Path
from dotenv import load_dotenv
from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)


async def main():
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

    client = Client.new(base_url, api_key)

    print("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n")

    # Step 1: Create SWR script for GitHub user caching
    print("Step 1: Create SWR function that acts as edge cache")

    swr_script = {
        "label": "fetch_github_user",
        "name": "Fetch GitHub User with Cache",
        "description": "SWR pattern: Check cache, fetch from GitHub API if stale, auto-update with TTL",
        "version": "1.0",
        "parameters": {
            "username": {"required": True, "description": "GitHub username to fetch"},
            "ttl": {
                "required": False,
                "default": 300,
                "description": "Cache TTL in seconds (default: 5 minutes)",
            },
        },
        "functions": [
            # 1. Check cache
            {
                "type": "FindById",
                "collection": "github_cache",
                "record_id": "{{username}}",
            },
            # 2. If cache exists, return it; else fetch from API
            {
                "type": "If",
                "condition": {"type": "HasRecords"},
                "then_functions": [
                    # Cache hit - return cached data
                    {
                        "type": "Project",
                        "fields": ["data", "cached_at"],
                        "exclude": False,
                    }
                ],
                "else_functions": [
                    # Cache miss - fetch external API and store
                    {
                        "type": "HttpRequest",
                        "url": "https://api.github.com/users/{{username}}",
                        "method": "GET",
                        "headers": {"User-Agent": "ekoDB-SWR-Example"},
                    },
                    {
                        "type": "Insert",
                        "collection": "github_cache",
                        "record": {
                            "cache_key": {"type": "String", "value": "{{username}}"},
                            "data": {"type": "Object", "value": "{{http_response}}"},
                            "cached_at": {
                                "type": "String",
                                "value": str(int(time.time())),
                            },
                        },
                        "bypass_ripple": False,
                    },
                ],
            },
        ],
        "tags": ["swr", "github", "cache"],
    }

    script_id = await client.save_script(swr_script)
    print(f"✓ Created SWR script: {swr_script['label']} ({script_id})\n")

    # Step 2: First call - Cache miss
    print("Step 2: First call - Cache miss, fetches from GitHub API")
    start1 = time.time()
    result1 = await client.call_script(
        "fetch_github_user", {"username": "torvalds", "ttl": 300}
    )
    duration1 = (time.time() - start1) * 1000
    print(f"Response time: {duration1:.0f}ms")
    print(f"Result: {json.dumps(result1['records'][:1], indent=2)}")
    print("✓ Data fetched from external API and cached\n")

    # Step 3: Second call - Cache hit
    print("Step 3: Second call - Cache hit, instant response from ekoDB")
    start2 = time.time()
    result2 = await client.call_script("fetch_github_user", {"username": "torvalds"})
    duration2 = (time.time() - start2) * 1000
    speedup = duration1 / duration2
    print(f"Response time: {duration2:.0f}ms ({speedup:.1f}x faster!)")
    print("✓ Lightning fast cache hit\n")

    # Advanced example: Data enrichment
    print("=== Advanced: SWR with Data Enrichment ===\n")
    print("Creating product enrichment function...")

    enrich_script = {
        "label": "fetch_product_enriched",
        "name": "Fetch Product with Enrichment",
        "description": "Demonstrates calling external API and enriching data",
        "version": "1.0",
        "parameters": {
            "product_id": {"required": True, "description": "Product ID"},
            "ttl": {
                "required": False,
                "default": 600,
                "description": "Cache TTL (10 minutes)",
            },
        },
        "functions": [
            {
                "type": "FindById",
                "collection": "product_cache",
                "record_id": "{{product_id}}",
            },
            {
                "type": "If",
                "condition": {"type": "HasRecords"},
                "then_functions": [
                    {"type": "Project", "fields": ["enriched_data"], "exclude": False}
                ],
                "else_functions": [
                    {
                        "type": "HttpRequest",
                        "url": "https://dummyjson.com/products/{{product_id}}",
                        "method": "GET",
                    },
                    {
                        "type": "Insert",
                        "collection": "product_cache",
                        "record": {
                            "cache_key": {"type": "String", "value": "{{product_id}}"},
                            "enriched_data": {
                                "type": "Object",
                                "value": "{{http_response}}",
                            },
                            "enriched_at": {
                                "type": "String",
                                "value": str(int(time.time())),
                            },
                        },
                        "bypass_ripple": False,
                    },
                ],
            },
        ],
        "tags": ["enrichment", "product", "cache"],
    }

    enrich_script_id = await client.save_script(enrich_script)
    print(
        f"✓ Created enrichment script: {enrich_script['label']} ({enrich_script_id})\n"
    )

    print(
        "Step 4: Call enrichment function - Fetches from API + stores enriched result"
    )
    enriched = await client.call_script(
        "fetch_product_enriched", {"product_id": "1", "ttl": 600}
    )
    print(f"Enriched data: {json.dumps(enriched['records'][:1], indent=2)}")
    print("✓ Data fetched, enriched, and cached atomically\n")

    # Print benefits
    print("=== Why This Is Powerful ===")
    print("✓ No separate cache layer (Redis, Memcached) needed")
    print("✓ No manual cache invalidation (TTL handles it)")
    print("✓ No separate edge infrastructure (ekoDB IS the edge)")
    print("✓ Atomic operations (function executes as transaction)")
    print("✓ With multi-node + ripples: Auto-sync across all nodes")
    print("✓ Sub-millisecond cache hits from internal storage")
    print("✓ One service instead of many (cache + API gateway + database)")

    print("\n=== Real-World Use Cases ===")
    print("1. API Gateway Pattern:")
    print(
        "   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache"
    )
    print("\n2. Database Federation:")
    print("   - Query multiple DBs (Postgres, MongoDB) + external APIs")
    print("   - Merge results in one function call")
    print("\n3. IoT Data Enrichment:")
    print("   - Sensor data + weather API + location API")
    print("   - Enrich and cache in one atomic operation")
    print("\n4. E-commerce Product Pages:")
    print("   - Product info + reviews + inventory + pricing")
    print("   - All from different sources, cached together")

    print("\n✓ Example complete - Your database IS your edge!\n")


if __name__ == "__main__":
    asyncio.run(main())
