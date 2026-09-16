#!/usr/bin/env python3
"""Transaction Examples - Direct HTTP API calls"""

import asyncio
import os
from pathlib import Path

import aiohttp
from dotenv import load_dotenv

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
COLLECTION = "transactions_accounts_py"


async def get_auth_token(session):
    """Get authentication token"""
    async with session.post(
        f"{BASE_URL}/api/auth/token",
        json={"api_key": API_KEY},
        headers={"Content-Type": "application/json"},
    ) as response:
        data = await response.json()
        return data["token"]


async def get_config(session, headers):
    """Get current database configuration"""
    async with session.get(
        f"{BASE_URL}/api/config",
        headers=headers,
    ) as response:
        return await response.json()


async def update_config(session, headers, config_update):
    """Update database configuration"""
    async with session.put(
        f"{BASE_URL}/api/config",
        headers=headers,
        json=config_update,
    ) as response:
        return await response.json()


async def delete_collection(session, headers):
    """Delete only this language's transaction fixture collection."""
    async with session.delete(
        f"{BASE_URL}/api/collections/{COLLECTION}", headers=headers
    ) as response:
        if response.status != 404:
            response.raise_for_status()


async def run_transaction_test(session, headers, test_name):
    """Run a single transaction test"""
    print(f"\n{'='*60}")
    print(f"🔥 {test_name}")
    print(f"{'='*60}\n")

    # Setup test data
    print("=== Setup: Creating Test Accounts ===")
    async with session.post(
        f"{BASE_URL}/api/insert/{COLLECTION}",
        headers=headers,
        json={"account_id": "ACC001", "name": "Alice", "balance": 1000},
    ) as response:
        alice = await response.json()
        alice_id = alice["id"]
        print(f"Created Alice: $1000 - ID: {alice_id}")

    async with session.post(
        f"{BASE_URL}/api/insert/{COLLECTION}",
        headers=headers,
        json={"account_id": "ACC002", "name": "Bob", "balance": 500},
    ) as response:
        bob = await response.json()
        bob_id = bob["id"]
        print(f"Created Bob: $500 - ID: {bob_id}\n")

    # Begin Transaction
    print("=== Example 1: Begin Transaction ===")
    async with session.post(
        f"{BASE_URL}/api/transactions",
        headers=headers,
        json={"isolation_level": "ReadCommitted"},
    ) as response:
        tx_data = await response.json()
        tx_id = tx_data["transaction_id"]
        print(f"Transaction ID: {tx_id}\n")

    # Perform operations in transaction
    print("=== Example 2: Operations with transaction_id ===")
    async with session.put(
        f"{BASE_URL}/api/batch/update/{COLLECTION}?transaction_id={tx_id}",
        headers=headers,
        json={
            "updates": [
                {"id": alice_id, "data": {"balance": 800}},
                {"id": bob_id, "data": {"balance": 700}},
            ]
        },
    ) as response:
        print("Updated Alice: $1000 → $800")
        print("Updated Bob: $500 → $700\n")

    # Check status
    print("=== Example 3: Transaction Status ===")
    async with session.get(
        f"{BASE_URL}/api/transactions/{tx_id}",
        headers=headers,
    ) as response:
        status = await response.json()
        print(f"Status: {status['state']}")
        print(f"Operations: {status['operations_count']}\n")

    # Commit
    print("=== Example 4: Commit Transaction ===")
    async with session.post(
        f"{BASE_URL}/api/transactions/{tx_id}/commit",
        headers=headers,
    ) as response:
        print("✓ Transaction committed\n")

    # Verify
    print("=== Verification ===")
    async with session.get(
        f"{BASE_URL}/api/find/{COLLECTION}/{alice_id}",
        headers=headers,
    ) as response:
        alice_final = await response.json()
        print(f"Alice: ${alice_final['balance']}")

    async with session.get(
        f"{BASE_URL}/api/find/{COLLECTION}/{bob_id}",
        headers=headers,
    ) as response:
        bob_final = await response.json()
        print(f"Bob: ${bob_final['balance']}\n")

    # Test Rollback
    print("=== Example 5: Rollback ===")
    async with session.post(
        f"{BASE_URL}/api/transactions",
        headers=headers,
        json={"isolation_level": "ReadCommitted"},
    ) as response:
        tx_data2 = await response.json()
        tx_id2 = tx_data2["transaction_id"]
        print(f"New transaction: {tx_id2}")

    async with session.put(
        f"{BASE_URL}/api/batch/update/{COLLECTION}?transaction_id={tx_id2}",
        headers=headers,
        json={"updates": [{"id": bob_id, "data": {"balance": 600}}]},
    ) as response:
        print("Updated Bob: $700 → $600 (in transaction)")

    async with session.post(
        f"{BASE_URL}/api/transactions/{tx_id2}/rollback",
        headers=headers,
    ) as response:
        print("✓ Transaction rolled back")

    async with session.get(
        f"{BASE_URL}/api/find/{COLLECTION}/{bob_id}",
        headers=headers,
    ) as response:
        bob_after = await response.json()
        print(f"Bob after rollback: ${bob_after['balance']}\n")

    # Cleanup
    print("=== Cleanup ===")
    await session.delete(
        f"{BASE_URL}/api/delete/{COLLECTION}/{alice_id}", headers=headers
    )
    await session.delete(
        f"{BASE_URL}/api/delete/{COLLECTION}/{bob_id}", headers=headers
    )

    print("✓ All transaction examples completed")


async def run_single_transaction_flow(session, headers):
    # Setup test data
    print("=== Setup: Creating Test Accounts ===")
    async with session.post(
        f"{BASE_URL}/api/insert/{COLLECTION}",
        headers=headers,
        json={"account_id": "ACC001", "name": "Alice", "balance": 1000},
    ) as response:
        alice = await response.json()
        alice_id = alice["id"]
        print(f"Created Alice: $1000 - ID: {alice_id}")

    async with session.post(
        f"{BASE_URL}/api/insert/{COLLECTION}",
        headers=headers,
        json={"account_id": "ACC002", "name": "Bob", "balance": 500},
    ) as response:
        bob = await response.json()
        bob_id = bob["id"]
        print(f"Created Bob: $500 - ID: {bob_id}\n")

    # Begin Transaction
    print("=== Example 1: Begin Transaction ===")
    async with session.post(
        f"{BASE_URL}/api/transactions",
        headers=headers,
        json={"isolation_level": "ReadCommitted"},
    ) as response:
        tx_data = await response.json()
        tx_id = tx_data["transaction_id"]
        print(f"Transaction ID: {tx_id}\n")

    # Perform operations in transaction
    print("=== Example 2: Operations with transaction_id ===")
    async with session.put(
        f"{BASE_URL}/api/batch/update/{COLLECTION}?transaction_id={tx_id}",
        headers=headers,
        json={
            "updates": [
                {"id": alice_id, "data": {"balance": 800}},
                {"id": bob_id, "data": {"balance": 700}},
            ]
        },
    ) as response:
        print("Updated Alice: $1000 → $800")
        print("Updated Bob: $500 → $700\n")

    # Commit
    print("=== Example 4: Commit Transaction ===")
    async with session.post(
        f"{BASE_URL}/api/transactions/{tx_id}/commit",
        headers=headers,
    ) as response:
        print("✓ Transaction committed\n")

    # Verify
    print("=== Verification ===")
    async with session.get(
        f"{BASE_URL}/api/find/{COLLECTION}/{alice_id}",
        headers=headers,
    ) as response:
        alice_final = await response.json()
        print(f"Alice: ${alice_final['balance']}")

    async with session.get(
        f"{BASE_URL}/api/find/{COLLECTION}/{bob_id}",
        headers=headers,
    ) as response:
        bob_final = await response.json()
        print(f"Bob: ${bob_final['balance']}\n")

    # Cleanup
    print("=== Cleanup ===")
    await session.delete(
        f"{BASE_URL}/api/delete/{COLLECTION}/{alice_id}", headers=headers
    )
    await session.delete(
        f"{BASE_URL}/api/delete/{COLLECTION}/{bob_id}", headers=headers
    )
    print("✓ Deleted test accounts\n")


if __name__ == "__main__":

    async def transaction_examples():
        """Run transaction examples with different durability configs"""
        async with aiohttp.ClientSession() as session:
            token = await get_auth_token(session)
            headers = {
                "Authorization": f"Bearer {token}",
                "Content-Type": "application/json",
            }
            print("✓ Authentication successful\n")
            await delete_collection(session, headers)

            # Get original config
            print("📋 Getting original configuration...")
            original_config = await get_config(session, headers)
            original_durable = original_config.get("durable_operations")
            print(f"   Original durable_operations: {original_durable}\n")

            operation_error = None
            try:
                # Test 1: With original config
                await run_transaction_test(
                    session,
                    headers,
                    f"TEST 1: Original Config (durable={original_durable})",
                )
                await run_single_transaction_flow(session, headers)

                # Test 2: Switch to non-durable
                print("\n🔄 Switching to NON-DURABLE mode...")
                await update_config(session, headers, {"durable_operations": False})
                print("   ✓ Config updated: durable_operations=false\n")

                await run_transaction_test(
                    session, headers, "TEST 2: Non-Durable Mode (durable=false)"
                )
                await run_single_transaction_flow(session, headers)

                # Test 3: Switch to durable
                print("\n🔄 Switching to DURABLE mode...")
                await update_config(session, headers, {"durable_operations": True})
                print("   ✓ Config updated: durable_operations=true\n")

                await run_transaction_test(
                    session, headers, "TEST 3: Durable Mode (durable=true)"
                )
                await run_single_transaction_flow(session, headers)

            except BaseException as error:  # noqa: BLE001 - cleanup must always run
                operation_error = error

            cleanup_errors = []
            try:
                # Restore original config.
                print("\n🔄 Restoring original configuration...")
                if original_durable is not None:
                    await update_config(
                        session, headers, {"durable_operations": original_durable}
                    )
                print(f"   ✓ Config restored: durable_operations={original_durable}\n")
            except Exception as error:  # noqa: BLE001 - attempt collection cleanup too
                cleanup_errors.append(f"restore configuration: {error}")

            try:
                await delete_collection(session, headers)
            except Exception as error:  # noqa: BLE001 - preserve the primary error
                cleanup_errors.append(f"collection {COLLECTION}: {error}")

            if operation_error is not None:
                if cleanup_errors:
                    operation_error.add_note(
                        "cleanup also failed: " + "; ".join(cleanup_errors)
                    )
                    print("⚠️  Cleanup errors: " + "; ".join(cleanup_errors))
                raise operation_error
            if cleanup_errors:
                raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))

            print("\n" + "=" * 60)
            print("✅ ALL TESTS PASSED - Transactions successful")
            print("=" * 60 + "\n")

    asyncio.run(transaction_examples())
