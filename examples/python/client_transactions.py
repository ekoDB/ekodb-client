#!/usr/bin/env python3
"""
Transaction Example using ekoDB Python Client

Demonstrates:
- Begin transaction with the server default or an explicit isolation level
- Operations within a transaction
- Transaction status checking
- Commit transaction
- Rollback transaction
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
COLLECTION = "client_transactions_accounts_py"


def field_value(field):
    """Unwrap ekoDB's typed field envelope while accepting raw JSON values."""
    if isinstance(field, dict) and "value" in field:
        return field["value"]
    return field


def require_equal(actual, expected, context):
    """Keep live contract checks active even when Python runs with -O."""
    if actual != expected:
        raise RuntimeError(f"{context}: got {actual!r}, expected {expected!r}")


async def main():
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created\n")
    active_transactions = []
    operation_error = None

    try:
        # Setup test data
        print("=== Setup: Creating Test Accounts ===")
        alice = await client.insert(
            COLLECTION,
            {"account_id": "ACC001", "name": "Alice", "balance": 1000},
        )
        alice_id = alice["id"]
        print(f"Created Alice: $1000 - ID: {alice_id}")

        bob = await client.insert(
            COLLECTION,
            {"account_id": "ACC002", "name": "Bob", "balance": 500},
        )
        bob_id = bob["id"]
        print(f"Created Bob: $500 - ID: {bob_id}\n")

        # Example 1: Begin Transaction
        print("=== Example 1: Begin Transaction ===")
        tx_id = await client.begin_transaction()
        active_transactions.append(tx_id)
        print(f"Transaction ID (server-default isolation): {tx_id}\n")

        # Example 2: Operations within Transaction
        print("=== Example 2: Operations within Transaction ===")
        await client.update(
            COLLECTION, alice_id, {"balance": 800}, transaction_id=tx_id
        )
        await client.update(COLLECTION, bob_id, {"balance": 700}, transaction_id=tx_id)
        alice_in_tx = await client.find_by_id(
            COLLECTION, alice_id, transaction_id=tx_id
        )
        bob_in_tx = await client.find_by_id(COLLECTION, bob_id, transaction_id=tx_id)
        print(f"Alice in transaction: {alice_in_tx}")
        print(f"Bob in transaction: {bob_in_tx}")
        require_equal(
            field_value(alice_in_tx["balance"]), 800, "Alice transactional balance"
        )
        require_equal(
            field_value(bob_in_tx["balance"]), 700, "Bob transactional balance"
        )
        print("Updated Alice: $1000 → $800")
        print("Updated Bob: $500 → $700\n")

        # Example 3: Transaction Status
        print("=== Example 3: Transaction Status ===")
        status = await client.get_transaction_status(tx_id)
        print(f"Status: {status.get('state')}")
        print(f"Operations: {status.get('operations_count')}\n")
        require_equal(status.get("state"), "Active", "transaction state")
        require_equal(status.get("operations_count"), 2, "transaction operation count")

        # Example 4: Commit Transaction
        print("=== Example 4: Commit Transaction ===")
        await client.commit_transaction(tx_id)
        active_transactions.remove(tx_id)
        print("✓ Transaction committed\n")

        committed_alice = await client.find_by_id(COLLECTION, alice_id)
        committed_bob = await client.find_by_id(COLLECTION, bob_id)
        require_equal(
            field_value(committed_alice["balance"]), 800, "committed Alice balance"
        )
        require_equal(
            field_value(committed_bob["balance"]), 700, "committed Bob balance"
        )
        print("✓ Verified committed balances: Alice=$800, Bob=$700\n")

        # Example 5: Rollback Demo
        print("=== Example 5: Rollback Demo ===")
        tx_id2 = await client.begin_transaction("ReadCommitted")
        active_transactions.append(tx_id2)
        print(f"New transaction: {tx_id2}")

        await client.update(COLLECTION, bob_id, {"balance": 600}, transaction_id=tx_id2)
        bob_in_rollback_tx = await client.find_by_id(
            COLLECTION, bob_id, transaction_id=tx_id2
        )
        require_equal(
            field_value(bob_in_rollback_tx["balance"]),
            600,
            "Bob balance before rollback",
        )
        print("Updated Bob: $700 → $600 (in transaction)")

        status2 = await client.get_transaction_status(tx_id2)
        print(f"Status before rollback: {status2.get('state')}")
        require_equal(status2.get("state"), "Active", "rollback transaction state")
        require_equal(
            status2.get("operations_count"), 1, "rollback transaction operation count"
        )

        await client.rollback_transaction(tx_id2)
        active_transactions.remove(tx_id2)
        print("✓ Transaction rolled back\n")

        bob_after_rollback = await client.find_by_id(COLLECTION, bob_id)
        require_equal(
            field_value(bob_after_rollback["balance"]),
            700,
            "Bob balance after rollback",
        )
        print("✓ Verified Bob remains $700 after rollback\n")
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        operation_error = error

    # Cleanup
    print("=== Cleanup ===")
    cleanup_errors = []
    for transaction_id in reversed(active_transactions):
        try:
            await client.rollback_transaction(transaction_id)
        except Exception as error:  # noqa: BLE001 - attempt every cleanup
            cleanup_errors.append(f"transaction {transaction_id}: {error}")
    try:
        await client.delete_collection(COLLECTION)
    except Exception as error:  # noqa: BLE001 - preserve the operation error
        cleanup_errors.append(f"collection {COLLECTION}: {error}")

    if operation_error is not None:
        if cleanup_errors:
            print("⚠️  Cleanup errors: " + "; ".join(cleanup_errors))
        raise operation_error
    if cleanup_errors:
        raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))

    print("✓ Deleted test account collection\n")

    print("✓ All client transaction examples completed")


if __name__ == "__main__":
    asyncio.run(main())
