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
        # TEST 2: Multiple Documents with Different TTLs
        # ═══════════════════════════════════════════════════════════════════════
        print()
        print("═══════════════════════════════════════════════════════════")
        print("TEST 2: Multiple Documents with Different TTLs")
        print("═══════════════════════════════════════════════════════════")

        # Insert 3 documents: 3s TTL, 10s TTL, no TTL (with better margins)
        print("\n[Step 1] Insert 3 documents with different TTLs")

        doc_3s = await client.insert(
            collection, {"name": "3 second TTL", "expires": "soon"}, ttl="3s"
        )
        doc_10s = await client.insert(
            collection, {"name": "10 second TTL", "expires": "later"}, ttl="10s"
        )
        doc_perm = await client.insert(
            collection, {"name": "Permanent", "expires": "never"}
        )

        id_3s = doc_3s.get("id")
        id_10s = doc_10s.get("id")
        id_perm = doc_perm.get("id")

        print(
            f"  Inserted: 3s TTL={id_3s[:8]}, 10s TTL={id_10s[:8]}, Permanent={id_perm[:8]}"
        )
        print("  ✓ PASS: All documents inserted")

        # Wait 5 seconds - 3s doc should expire, 10s doc should remain
        print("\n[Step 2] Wait 5 seconds (3s doc should expire)")
        time.sleep(5)

        # Check 3s doc (should be gone)
        try:
            check_3s = await client.find_by_id(collection, id_3s)
        except:
            check_3s = None
        if check_3s is None:
            print("  ✓ PASS: 3s TTL document expired")
        else:
            raise Exception("❌ FAILED: 3s TTL document should have expired")

        # Check 10s doc (should still exist)
        try:
            check_10s = await client.find_by_id(collection, id_10s)
        except:
            check_10s = None
        if check_10s is not None:
            print("  ✓ PASS: 10s TTL document still exists")
        else:
            raise Exception("❌ FAILED: 10s TTL document expired too early")

        # Check permanent doc (should exist)
        try:
            check_perm = await client.find_by_id(collection, id_perm)
        except:
            check_perm = None
        if check_perm is not None:
            print("  ✓ PASS: Permanent document still exists")
        else:
            raise Exception("❌ FAILED: Permanent document should exist")

        # Wait 7 more seconds - 10s doc should now expire (12s total)
        print("\n[Step 3] Wait 7 more seconds (10s doc should expire)")
        time.sleep(7)

        try:
            check_10s_again = await client.find_by_id(collection, id_10s)
        except:
            check_10s_again = None
        if check_10s_again is None:
            print("  ✓ PASS: 10s TTL document expired")
        else:
            raise Exception("❌ FAILED: 10s TTL document should have expired")

        try:
            check_perm_again = await client.find_by_id(collection, id_perm)
        except:
            check_perm_again = None
        if check_perm_again is not None:
            print("  ✓ PASS: Permanent document still exists")
        else:
            raise Exception("❌ FAILED: Permanent document should still exist")

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
