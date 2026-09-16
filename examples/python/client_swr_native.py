#!/usr/bin/env python3
"""
ekoDB Python Client - Native SWR Function Example

Demonstrates the native SWR (Stale-While-Revalidate) function that
encapsulates the entire cache-aside pattern in a single operation.
"""

import os
import time

from dotenv import load_dotenv

from ekodb_client import Client, Stage

load_dotenv()


def _is_already_exists_error(err):
    """Detect the server's 409 'function already exists' response."""
    msg = str(err)
    return "409" in msg or "already exists" in msg


async def save_or_update(client, script):
    """Save a function, falling back to an update if its label already exists."""
    label = script["label"]
    try:
        return await client.save_function(script)
    except Exception as e:
        if not _is_already_exists_error(e):
            raise
        await client.update_function(label, script)
        print(f"ℹ️  Function '{label}' already existed — updated instead")
        existing = await client.get_function(label)
        function_id = existing.get("id")
        if not function_id:
            raise RuntimeError(f"Updated function '{label}' did not return an id")
        return function_id


BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
FUNCTION_LABELS = {
    "basic": "github_user_native_py",
    "audit": "product_swr_audit_py",
    "enrichment": "user_enrichment_pipeline_py",
    "dynamic": "flexible_cache_py",
}
COLLECTIONS = {
    "audit": "swr_audit_trail_py",
    "enrichment": "enriched_users_swr_py",
}
KV_KEYS = {
    "basic_template": "swr_native:py:github:user:{{username}}",
    "basic": "swr_native:py:github:user:torvalds",
    "audit_template": "swr_native:py:product:{{product_id}}",
    "audit": "swr_native:py:product:1",
    "enrichment_template": "swr_native:py:api:user:{{user_id}}",
    "enrichment": "swr_native:py:api:user:1",
    "dynamic_template": "swr_native:py:resource:{{resource_id}}",
    "dynamic": "swr_native:py:resource:test",
}


def _is_not_found_error(err):
    message = str(err).lower()
    return "404" in message or "not found" in message


async def example_basic_swr(client: Client):
    """Example 1: Basic Native SWR - Cache Miss → Cache Hit"""
    print("\nExample 1: Basic Native SWR")
    print("─" * 80)
    print("Single function replaces KvGet → If → HttpRequest → KvSet pipeline")

    # Create script with native SWR function
    basic_swr_script = {
        "label": FUNCTION_LABELS["basic"],
        "name": "GitHub User Lookup (Native SWR)",
        "description": "Fetches GitHub user data with automatic caching using native SWR",
        "parameters": {
            "username": {
                "description": "GitHub username to fetch",
                "required": True,
            }
        },
        "functions": [
            Stage.swr(
                cache_key=KV_KEYS["basic_template"],
                ttl="15m",
                url="https://api.github.com/users/{{username}}",
                method="GET",
                headers={"User-Agent": "ekoDB-Client"},
                output_field="user_data",
            )
        ],
        "tags": ["github", "swr", "native"],
    }

    script_id = await save_or_update(client, basic_swr_script)
    print(f"✓ Created native SWR script: {FUNCTION_LABELS['basic']} ({script_id})")

    # First call - cache miss
    print("\nFirst call (cache miss - will fetch from GitHub API):")
    start1 = time.time()
    result1 = await client.call_function(
        FUNCTION_LABELS["basic"], {"username": "torvalds"}
    )
    duration1 = (time.time() - start1) * 1000
    print(f"  Response time: {duration1:.0f}ms")
    print(f"  Records returned: {len(result1.get('records', []))}")

    # Second call - cache hit
    print("\nSecond call (cache hit - instant from KV store):")
    start2 = time.time()
    result2 = await client.call_function(
        FUNCTION_LABELS["basic"], {"username": "torvalds"}
    )
    duration2 = (time.time() - start2) * 1000
    speedup = duration1 / duration2 if duration2 > 0 else 0
    print(f"  Response time: {duration2:.0f}ms")
    print(f"  Speedup: {speedup:.1f}x faster 🚀")
    print(f"  Records returned: {len(result2.get('records', []))}\n")

    return script_id


async def example_audit_trail(client: Client):
    """Example 2: SWR with Audit Trail"""
    print("\nExample 2: SWR with Built-in Audit Trail")
    print("─" * 80)
    print("Optional collection parameter for automatic request logging")

    # Create script with audit trail
    audit_swr_script = {
        "label": FUNCTION_LABELS["audit"],
        "name": "Product API with Audit (Native SWR)",
        "description": "Caches product data and logs all requests automatically",
        "parameters": {
            "product_id": {
                "description": "Product ID to fetch",
                "required": True,
            }
        },
        "functions": [
            Stage.swr(
                cache_key=KV_KEYS["audit_template"],
                ttl="1h",
                url="https://fakestoreapi.com/products/{{product_id}}",
                method="GET",
                output_field="product",
                collection=COLLECTIONS["audit"],
            )
        ],
        "tags": ["products", "audit"],
    }

    audit_script_id = await save_or_update(client, audit_swr_script)
    print(
        f"✓ Created SWR script with audit trail: {FUNCTION_LABELS['audit']} ({audit_script_id})"
    )

    print("\nFetching product (will create audit trail entry):")
    product_result = await client.call_function(
        FUNCTION_LABELS["audit"], {"product_id": "1"}
    )
    print("  ✓ Product fetched and cached")
    print(f"  ✓ Audit record created in '{COLLECTIONS['audit']}' collection")
    print(f"  Records: {len(product_result.get('records', []))}\n")

    return audit_script_id


async def example_pipeline_enrichment(client: Client):
    """Example 3: SWR in Multi-Function Pipeline"""
    print("\nExample 3: SWR in Multi-Function Pipeline")
    print("─" * 80)
    print("Fetch external data → Process → Store in collection")

    # Create enrichment pipeline
    pipeline_script = {
        "label": FUNCTION_LABELS["enrichment"],
        "name": "User Data Enrichment Pipeline",
        "description": "Fetches external API data and stores enriched results",
        "parameters": {
            "user_id": {
                "description": "User ID to enrich",
                "required": True,
            }
        },
        "functions": [
            # Step 1: Fetch from external API with caching (30 min TTL)
            Stage.swr(
                cache_key=KV_KEYS["enrichment_template"],
                ttl="30m",
                url="https://jsonplaceholder.typicode.com/users/{{user_id}}",
                method="GET",
                output_field="user_data",
            ),
            # Step 2: Store enriched data in collection (24 hour TTL)
            Stage.insert(
                collection=COLLECTIONS["enrichment"],
                record={
                    "user_id": {"type": "String", "value": "{{user_id}}"},
                    "source_data": {"type": "Object", "value": "{{user_data}}"},
                },
                ttl="24h",
            ),
        ],
        "tags": ["enrichment", "pipeline"],
    }

    pipeline_script_id = await save_or_update(client, pipeline_script)
    print(
        f"✓ Created enrichment pipeline: {FUNCTION_LABELS['enrichment']} ({pipeline_script_id})"
    )

    print("\nRunning pipeline:")
    enrich_result = await client.call_function(
        FUNCTION_LABELS["enrichment"], {"user_id": "1"}
    )
    print("  ✓ Data fetched from API (cached 30m)")
    print(f"  ✓ Enriched data stored in '{COLLECTIONS['enrichment']}' (TTL 24h)")
    print(f"  Pipeline returned {len(enrich_result.get('records', []))} records\n")

    return pipeline_script_id


async def example_dynamic_ttl(client: Client):
    """Example 4: Parameterized TTL"""
    print("\nExample 4: Dynamic TTL Configuration")
    print("─" * 80)
    print("TTL as parameter - supports duration strings, integers, ISO timestamps")

    # Create script with dynamic TTL
    dynamic_ttl_script = {
        "label": FUNCTION_LABELS["dynamic"],
        "name": "Flexible Cache TTL (Native SWR)",
        "description": "Demonstrates parameterized TTL values",
        "parameters": {
            "resource_id": {
                "description": "Resource to fetch",
                "required": True,
            },
            "ttl": {
                "description": "Cache duration (e.g., '5m', '1h', '30s')",
                "required": True,
            },
        },
        "functions": [
            Stage.swr(
                cache_key=KV_KEYS["dynamic_template"],
                ttl="{{ttl}}",
                url="https://jsonplaceholder.typicode.com/posts/{{resource_id}}",
                method="GET",
            )
        ],
        "tags": ["dynamic"],
    }

    dynamic_script_id = await save_or_update(client, dynamic_ttl_script)
    print(
        f"✓ Created dynamic TTL script: {FUNCTION_LABELS['dynamic']} ({dynamic_script_id})"
    )

    # Test with different TTLs
    ttl_tests = [("5m", "5 minutes"), ("1h", "1 hour"), ("30s", "30 seconds")]

    for ttl_value, description in ttl_tests:
        await client.call_function(
            FUNCTION_LABELS["dynamic"], {"resource_id": "test", "ttl": ttl_value}
        )
        print(f"  ✓ Cached with TTL: {ttl_value} ({description})")

    return dynamic_script_id


async def cleanup(client: Client, script_ids: list):
    """Clean up only the functions, KV keys, and collections owned by this example."""
    print("\n🧹 Cleaning up...")
    errors = []
    cleanup_ids = set(script_ids)
    try:
        functions = await client.list_functions(None)
        cleanup_ids.update(
            function["id"]
            for function in functions
            if function.get("label") in FUNCTION_LABELS.values() and function.get("id")
        )
    except Exception as exc:
        errors.append(("discover functions", exc))
    deleted = 0
    for script_id in cleanup_ids:
        try:
            await client.delete_function(script_id)
            deleted += 1
        except Exception as exc:
            errors.append((script_id, exc))
    for key in (
        KV_KEYS["basic"],
        KV_KEYS["audit"],
        KV_KEYS["enrichment"],
        KV_KEYS["dynamic"],
    ):
        try:
            await client.kv_delete(key)
        except Exception as exc:
            if not _is_not_found_error(exc):
                errors.append((f"KV key {key}", exc))
    for collection in COLLECTIONS.values():
        try:
            await client.delete_collection(collection)
        except Exception as exc:
            if not _is_not_found_error(exc):
                errors.append((f"collection {collection}", exc))
    if errors:
        details = "; ".join(f"{resource}: {error}" for resource, error in errors)
        print(f"⚠ Cleanup failed: {details}")
        raise RuntimeError(
            f"Failed to clean up {len(errors)} owned resource(s): {details}"
        )
    print(f"✓ Deleted {deleted} test scripts and owned SWR resources")


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

    client = Client.new(BASE_URL, API_KEY)

    script_ids = []

    try:
        await cleanup(client, script_ids)
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
        raise

    finally:
        await cleanup(client, script_ids)

    print("\n✅ All examples completed!")


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())
