#!/usr/bin/env python3
"""
Transaction Example using ekoDB Python Client

Demonstrates:
- Begin transaction with isolation level
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


async def main():
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created\n")

    # Setup test data
    print("=== Setup: Creating Test Accounts ===")
    alice = await client.insert(
        "test_accounts", {"account_id": "ACC001", "name": "Alice", "balance": 1000}
    )
    alice_id = alice["id"]
    print(f"Created Alice: $1000 - ID: {alice_id}")

    bob = await client.insert(
        "test_accounts", {"account_id": "ACC002", "name": "Bob", "balance": 500}
    )
    bob_id = bob["id"]
    print(f"Created Bob: $500 - ID: {bob_id}\n")

    # Example 1: Begin Transaction
    print("=== Example 1: Begin Transaction ===")
    tx_id = await client.begin_transaction("ReadCommitted")
    print(f"Transaction ID: {tx_id}\n")

    # Example 2: Operations within Transaction
    print("=== Example 2: Operations within Transaction ===")
    print("Updated Alice: $1000 → $800 (simulated)")
    print("Updated Bob: $500 → $700 (simulated)\n")

    # Example 3: Transaction Status
    print("=== Example 3: Transaction Status ===")
    status = await client.get_transaction_status(tx_id)
    print(f"Status: {status.get('state')}")
    print(f"Operations: {status.get('operations_count')}\n")

    # Example 4: Commit Transaction
    print("=== Example 4: Commit Transaction ===")
    await client.commit_transaction(tx_id)
    print("✓ Transaction committed\n")

    # Example 5: Rollback Demo
    print("=== Example 5: Rollback Demo ===")
    tx_id2 = await client.begin_transaction("ReadCommitted")
    print(f"New transaction: {tx_id2}")

    status2 = await client.get_transaction_status(tx_id2)
    print(f"Status before rollback: {status2.get('state')}")

    await client.rollback_transaction(tx_id2)
    print("✓ Transaction rolled back\n")

    # Cleanup
    print("=== Cleanup ===")
    await client.delete("test_accounts", alice_id)
    await client.delete("test_accounts", bob_id)
    print("✓ Deleted test accounts\n")

    print("✓ All client transaction examples completed")


if __name__ == "__main__":
    asyncio.run(main())
