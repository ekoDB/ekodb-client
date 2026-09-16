"""
Example demonstrating bypass_ripple usage (Python)

The bypass_ripple parameter allows you to skip ripple propagation for specific operations,
which is useful in multi-node deployments when you want to prevent data from being
automatically replicated to other nodes.
"""

import asyncio
import os
from pathlib import Path

from dotenv import load_dotenv

from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)
COLLECTION = "bypass_ripple_products_py"


async def run_examples(client):
    print("=== Bypass Ripple Example ===\n")

    # Basic insert (no bypass)
    print("1. Basic insert (ripple enabled):")
    record1 = {"name": "Product 1", "price": 100}
    result1 = await client.insert(COLLECTION, record1)
    print(f"   Inserted with ripple: {result1}\n")

    # Insert with bypass_ripple
    print("2. Insert with bypass_ripple:")
    record2 = {"name": "Product 2", "price": 200}
    result2 = await client.insert(COLLECTION, record2, bypass_ripple=True)
    print(f"   Inserted with bypass_ripple: {result2}\n")

    # Update with bypass_ripple
    print("3. Update with bypass_ripple:")
    user_id = result1["id"]
    update_record = {"price": 150}
    result3 = await client.update(
        COLLECTION, user_id, update_record, bypass_ripple=True
    )
    print(f"   Updated with bypass_ripple: {result3}\n")

    # Delete with bypass_ripple (use result2's ID since result1 was just updated)
    print("4. Delete with bypass_ripple:")
    delete_id = result2["id"]
    await client.delete(COLLECTION, delete_id, bypass_ripple=True)
    print("   Deleted with bypass_ripple\n")

    # Batch operations with bypass_ripple
    print("5. Batch insert with bypass_ripple:")
    records = [
        {"name": "Batch 1", "price": 10},
        {"name": "Batch 2", "price": 20},
    ]
    batch_results = await client.batch_insert(COLLECTION, records, bypass_ripple=True)
    print(f"   Batch inserted with bypass_ripple: {len(batch_results)} records\n")

    # Upsert with bypass_ripple (use existing record ID from result1)
    print("6. Upsert with bypass_ripple:")
    upsert_record = {"name": "Upsert Product", "price": 500}
    upsert_result = await client.upsert(
        COLLECTION, user_id, upsert_record, bypass_ripple=True
    )
    print(f"   Upserted with bypass_ripple: {upsert_result}\n")


async def main():
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
    client = Client.new(base_url, api_key)
    try:
        await client.delete_collection(COLLECTION)
    except Exception as error:
        if "404" not in str(error) and "not found" not in str(error).lower():
            raise

    operation_error = None
    try:
        await run_examples(client)
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        operation_error = error
    try:
        await client.delete_collection(COLLECTION)
    except Exception as cleanup_error:
        if "404" in str(cleanup_error) or "not found" in str(cleanup_error).lower():
            pass
        elif operation_error is not None:
            operation_error.add_note(f"cleanup also failed: {cleanup_error}")
        else:
            raise
    if operation_error is not None:
        raise operation_error

    print("✅ All bypass_ripple operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
