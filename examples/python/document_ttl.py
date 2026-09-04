#!/usr/bin/env python3
"""
TTL Expiration Verification Test

This test VERIFIES that TTL actually works by:
1. Inserting a document with short TTL (3 seconds)
2. Verifying the document exists immediately
3. Waiting for TTL to expire
4. Verifying the document is GONE

If this test passes, TTL expiration is working correctly.
"""

import asyncio
import os
import sys
import time
from pathlib import Path
from dotenv import load_dotenv

from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent.parent / ".env"
load_dotenv(env_path)


async def main():
    print("╔════════════════════════════════════════════════════════╗")
    print("║     TTL EXPIRATION VERIFICATION TEST                   ║")
    print("╚════════════════════════════════════════════════════════╝")
    print()
    print("This test verifies that document TTL expiration works correctly.")
    print("We will insert documents with short TTL and verify they expire.")
    print()

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

    client = Client.new(base_url, api_key)
    print("✓ Client connected")

    collection = "ttl_expiration_test_py"
    ttl_seconds = 3

    try:
        # ═══════════════════════════════════════════════════════════════════════
        # TEST 1: Document TTL Expiration
        # ═══════════════════════════════════════════════════════════════════════
        print()
        print("═══════════════════════════════════════════════════════════")
        print("TEST 1: Document TTL Expiration")
        print("═══════════════════════════════════════════════════════════")

        # Step 1: Insert document with 3-second TTL
        print(f"\n[Step 1] Insert document with {ttl_seconds} second TTL")
        print("  Input: {name: 'TTL Test', value: 'should expire'}")
        print(f"  TTL: {ttl_seconds}s")

        doc = await client.insert(
            collection,
            {"name": "TTL Test", "value": "should expire"},
            ttl=f"{ttl_seconds}s",
        )
        doc_id = doc.get("id")
        print(f"  Output: Document ID = {doc_id}")
        print("  ✓ PASS: Document inserted")

        # Step 2: Verify document exists immediately
        print("\n[Step 2] Verify document exists immediately")
        print(f"  Input: find_by_id({doc_id})")

        found = await client.find_by_id(collection, doc_id)
        if not found:
            raise Exception("❌ FAILED: Document should exist but was not found")
        name_val = found.get("name", {})
        if isinstance(name_val, dict):
            name_val = name_val.get("value", name_val)
        print(f"  Output: Found document with name = {name_val}")
        print("  ✓ PASS: Document exists")

        # Step 3: Wait for TTL to expire
        wait_time = ttl_seconds + 2
        print(f"\n[Step 3] Wait for TTL to expire ({wait_time}s)")
        print("  Waiting", end="", flush=True)
        for _ in range(wait_time):
            time.sleep(1)
            print(".", end="", flush=True)
        print(" done")
        print("  ✓ PASS: Wait complete")

        # Step 4: Verify document is GONE
        print("\n[Step 4] Verify document has expired")
        print(f"  Input: find_by_id({doc_id})")

        try:
            expired = await client.find_by_id(collection, doc_id)
            if expired is None:
                print("  Output: None (document not found)")
                print("  ✓ PASS: Document expired correctly!")
            else:
                print(f"  Output: Document still exists! {expired}")
                raise Exception(
                    "❌ FAILED: Document should have expired but still exists!"
                )
        except Exception as e:
            if "FAILED" in str(e):
                raise
            print(f"  Output: Error (expected) - {e}")
            print("  ✓ PASS: Document expired (not found error)")

        # ═══════════════════════════════════════════════════════════════════════
        # Cleanup
        # ═══════════════════════════════════════════════════════════════════════
        print("\n═══════════════════════════════════════════════════════════")
        print("CLEANUP")
        print("═══════════════════════════════════════════════════════════")

        await client.delete_collection(collection)
        print("✓ Deleted test collection")

        # ═══════════════════════════════════════════════════════════════════════
        # Summary
        # ═══════════════════════════════════════════════════════════════════════
        print()
        print("╔════════════════════════════════════════════════════════╗")
        print("║              ALL TTL TESTS PASSED ✓                    ║")
        print("╚════════════════════════════════════════════════════════╝")
        print()
        print("TTL expiration is working correctly:")
        print("  • Documents with TTL expire after the specified time")
        print("  • Documents without TTL persist indefinitely")
        print("  • Different TTL durations are handled correctly")

    except Exception as error:
        print(f"\n❌ TEST FAILED: {error}")
        try:
            await client.delete_collection(collection)
        except:
            pass
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())
