"""ekoDB Python Client - Schedule Management Integration Example

Demonstrates the full schedule lifecycle:
  create -> list -> get -> update -> pause -> resume -> delete
"""

import asyncio
import os

from ekodb_client import Client


async def main():
    print("=== ekoDB Schedule Management Example (Python) ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY")
    if not api_key:
        raise ValueError("API_BASE_KEY environment variable is required")

    client = Client.new(base_url, api_key)

    # 1. Create a schedule
    print("--- create_schedule ---")
    schedule = await client.create_schedule(
        {
            "name": "Nightly Report",
            "cron": "0 0 * * *",
            "description": "Generate and email nightly analytics report",
            "timezone": "UTC",
            "enabled": True,
        }
    )
    schedule_id = schedule["id"]
    print(f"Created schedule: {schedule.get('name')} (id: {schedule_id})")

    # 2. List all schedules
    print("\n--- list_schedules ---")
    schedules = await client.list_schedules()
    print(f"Schedules: {schedules}")

    # 3. Get schedule by ID
    print("\n--- get_schedule ---")
    fetched = await client.get_schedule(schedule_id)
    print(f"Fetched: {fetched.get('name')} cron={fetched.get('cron')}")

    # 4. Update schedule
    print("\n--- update_schedule ---")
    updated = await client.update_schedule(
        schedule_id,
        {
            "cron": "30 1 * * *",
            "description": "Changed to 1:30 AM UTC",
        },
    )
    print(f"Updated cron: {updated.get('cron')}")

    # 5. Pause schedule
    print("\n--- pause_schedule ---")
    paused = await client.pause_schedule(schedule_id)
    print(f"Paused: status={paused.get('status')}")

    # 6. Resume schedule
    print("\n--- resume_schedule ---")
    resumed = await client.resume_schedule(schedule_id)
    print(f"Resumed: status={resumed.get('status')}")

    # 7. Delete schedule
    print("\n--- delete_schedule ---")
    await client.delete_schedule(schedule_id)
    print("Schedule deleted successfully")

    print("\n=== Schedule management example completed ===")


if __name__ == "__main__":
    asyncio.run(main())
