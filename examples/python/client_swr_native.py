#!/usr/bin/env python3
"""
ekoDB Python Client - Native SWR Function Example

Demonstrates the native SWR (Stale-While-Revalidate) function that
encapsulates the entire cache-aside pattern in a single operation.
"""

import os
import time
from ekodb_client import EkoDBClient, Stage
from dotenv import load_dotenv

load_dotenv()

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def example_basic_swr(client: EkoDBClient):
    """Example 1: Basic Native SWR - Cache Miss → Cache Hit"""
    print("\nExample 1: Basic Native SWR")
    print("─" * 80)
    print("Single function replaces KvGet → If → HttpRequest → KvSet pipeline")

    # Create script with native SWR function
    basic_swr_script = {
        "label": "github_user_native",
        "name": "GitHub User Lookup (Native SWR)",
        "description": "Fetches GitHub user data with automatic caching using native SWR",
        "parameters": {
            "username": {
                "type": "String",
                "description": "GitHub username to fetch",
                "required": True,
            }
        },
        "functions": [
            Stage.swr(
                cache_key="github:user:{{username}}",
                ttl="15m",
                url="https://api.github.com/users/{{username}}",
                method="GET",
                headers={"User-Agent": "ekoDB-Client"},
                output_field="user_data",
            )
        ],
        "tags": ["github", "swr", "native"],
    }

    script_id = await client.save_script(basic_swr_script)
    print(f"✓ Created native SWR script: github_user_native ({script_id})")

    # First call - cache miss
    print("\nFirst call (cache miss - will fetch from GitHub API):")
    start1 = time.time()
    result1 = await client.call_script("github_user_native", {"username": "torvalds"})
    duration1 = (time.time() - start1) * 1000
    print(f"  Response time: {duration1:.0f}ms")
    print(f"  Records returned: {len(result1.get('records', []))}")

    # Second call - cache hit
    print("\nSecond call (cache hit - instant from KV store):")
    start2 = time.time()
    result2 = await client.call_script("github_user_native", {"username": "torvalds"})
    duration2 = (time.time() - start2) * 1000
    speedup = duration1 / duration2 if duration2 > 0 else 0
    print(f"  Response time: {duration2:.0f}ms")
    print(f"  Speedup: {speedup:.1f}x faster 🚀")
    print(f"  Records returned: {len(result2.get('records', []))}\n")

    return script_id


async def example_audit_trail(client: EkoDBClient):
    """Example 2: SWR with Audit Trail"""
    print("\nExample 2: SWR with Built-in Audit Trail")
    print("─" * 80)
    print("Optional collection parameter for automatic request logging")

    # Create script with audit trail
    audit_swr_script = {
        "label": "product_swr_audit",
        "name": "Product API with Audit (Native SWR)",
        "description": "Caches product data and logs all requests automatically",
        "parameters": {
            "product_id": {
                "type": "String",
                "description": "Product ID to fetch",
                "required": True,
            }
        },
        "functions": [
            Stage.swr(
                cache_key="product:{{product_id}}",
                ttl="1h",
                url="https://fakestoreapi.com/products/{{product_id}}",
                method="GET",
                output_field="product",
                collection="swr_audit_trail",
            )
        ],
        "tags": ["products", "audit"],
    }

    audit_script_id = await client.save_script(audit_swr_script)
    print(
        f"✓ Created SWR script with audit trail: product_swr_audit ({audit_script_id})"
    )

    print("\nFetching product (will create audit trail entry):")
    product_result = await client.call_script("product_swr_audit", {"product_id": "1"})
    print("  ✓ Product fetched and cached")
    print("  ✓ Audit record created in 'swr_audit_trail' collection")
    print(f"  Records: {len(product_result.get('records', []))}\n")

    return audit_script_id


async def example_pipeline_enrichment(client: EkoDBClient):
    """Example 3: SWR in Multi-Function Pipeline"""
    print("\nExample 3: SWR in Multi-Function Pipeline")
    print("─" * 80)
    print("Fetch external data → Process → Store in collection")

    # Create enrichment pipeline
    pipeline_script = {
        "label": "user_enrichment_pipeline",
        "name": "User Data Enrichment Pipeline",
        "description": "Fetches external API data and stores enriched results",
        "parameters": {
            "user_id": {
                "type": "String",
                "description": "User ID to enrich",
                "required": True,
            }
        },
        "functions": [
            # Step 1: Fetch from external API with caching (30 min TTL)
            Stage.swr(
                cache_key="api:user:{{user_id}}",
                ttl="30m",
                url="https://jsonplaceholder.typicode.com/users/{{user_id}}",
                method="GET",
                output_field="user_data",
            ),
            # Step 2: Store enriched data in collection (24 hour TTL)
            Stage.insert(
                collection="enriched_users",
                record={
                    "user_id": {"type": "String", "value": "{{user_id}}"},
                    "source_data": {"type": "Object", "value": "{{user_data}}"},
                },
                ttl="24h",
            ),
        ],
        "tags": ["enrichment", "pipeline"],
    }

    pipeline_script_id = await client.save_script(pipeline_script)
    print(
        f"✓ Created enrichment pipeline: user_enrichment_pipeline ({pipeline_script_id})"
    )

    print("\nRunning pipeline:")
    enrich_result = await client.call_script(
        "user_enrichment_pipeline", {"user_id": "1"}
    )
    print("  ✓ Data fetched from API (cached 30m)")
    print("  ✓ Enriched data stored in 'enriched_users' (TTL 24h)")
    print(f"  Pipeline returned {len(enrich_result.get('records', []))} records\n")

    return pipeline_script_id


async def example_dynamic_ttl(client: EkoDBClient):
    """Example 4: Parameterized TTL"""
    print("\nExample 4: Dynamic TTL Configuration")
    print("─" * 80)
    print("TTL as parameter - supports duration strings, integers, ISO timestamps")

    # Create script with dynamic TTL
    dynamic_ttl_script = {
        "label": "flexible_cache",
        "name": "Flexible Cache TTL (Native SWR)",
        "description": "Demonstrates parameterized TTL values",
        "parameters": {
            "resource_id": {
                "type": "String",
                "description": "Resource to fetch",
                "required": True,
            },
            "ttl": {
                "type": "String",
                "description": "Cache duration (e.g., '5m', '1h', '30s')",
                "required": True,
            },
        },
        "functions": [
            Stage.swr(
                cache_key="resource:{{resource_id}}",
                ttl="{{ttl}}",
                url="https://jsonplaceholder.typicode.com/posts/{{resource_id}}",
                method="GET",
            )
        ],
        "tags": ["dynamic"],
    }

    dynamic_script_id = await client.save_script(dynamic_ttl_script)
    print(f"✓ Created dynamic TTL script: flexible_cache ({dynamic_script_id})")

    # Test with different TTLs
    ttl_tests = [("5m", "5 minutes"), ("1h", "1 hour"), ("30s", "30 seconds")]

    for ttl_value, description in ttl_tests:
        await client.call_script(
            "flexible_cache", {"resource_id": "test", "ttl": ttl_value}
        )
        print(f"  ✓ Cached with TTL: {ttl_value} ({description})")

    return dynamic_script_id


async def cleanup(client: EkoDBClient, script_ids: list):
    """Clean up test scripts"""
    print("\n🧹 Cleaning up...")
    try:
        for script_id in script_ids:
            await client.delete_script(script_id)
        print(f"✓ Deleted {len(script_ids)} test scripts")
    except Exception as e:
        print(f"⚠ Cleanup error (non-critical): {e}")


async def main():
    """Main execution"""
    print("🚀 ekoDB Python Client - Native SWR Function Examples\n")
    print("📋 Demonstrates:")
    print("   • Single-function SWR pattern (replaces 4-step pipeline)")
    print("   • Automatic cache checking, HTTP fetching, and cache setting")
    print("   • Built-in audit trail support")
    print("   • Duration string TTLs ('15m', '1h', '30s')")
    print("   • Multi-function pipeline integration")
    print("   • Dynamic TTL configuration\n")

    client = EkoDBClient(BASE_URL, API_KEY)
    await client.init()

    script_ids = []

    try:
        # Run examples
        script_ids.append(await example_basic_swr(client))
        script_ids.append(await example_audit_trail(client))
        script_ids.append(await example_pipeline_enrichment(client))
        script_ids.append(await example_dynamic_ttl(client))

        # Summary
        print("\n" + "=" * 80)
        print("✅ Key Benefits of Native SWR:")
        print("✅ Single function: Replaces 4-function cache-aside pattern")
        print(
            "✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds"
        )
        print("✅ Built-in audit: Optional collection parameter for automatic logging")
        print(
            "✅ Auto-enrichment: output_field populates params for downstream functions"
        )
        print(
            "✅ Transactional: Works correctly in both transactional and non-transactional contexts"
        )
        print("✅ KV-optimized: Uses native KV store with proper TTL handling")

        print("\n=== Performance Comparison ===")
        print("Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)")
        print("Native SWR:     SWR → Insert (2 functions)")
        print("Result:         60% fewer functions, cleaner code, same behavior 🎯")

    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback

        traceback.print_exc()

    finally:
        await cleanup(client, script_ids)

    print("\n✅ All examples completed!")


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())
