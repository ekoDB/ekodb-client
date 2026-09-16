"""ekoDB Python Client - Schedule Management Integration Example

Demonstrates the full schedule lifecycle:
  create -> list -> get -> update -> trigger -> pause -> resume -> delete

The example creates a temporary no-op function for the schedule to invoke.
"""

import asyncio
import os

from ekodb_client import Client, Stage


async def main():
    print("=== ekoDB Schedule Management Example (Python) ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY")
    if not api_key:
        raise ValueError("API_BASE_KEY environment variable is required")

    client = Client.new(base_url, api_key)

    function_label = f"schedule_noop_python_{os.getpid()}"
    function_id = await client.save_function(
        {
            "label": function_label,
            "name": "Schedule example no-op",
            "parameters": {},
            "functions": [Stage.return_response({"triggered": True}, 200)],
        }
    )

    schedule_id = None
    try:
        print("--- create_schedule ---")
        schedule = await client.create_schedule(
            {
                "name": "Nightly Report",
                "function_label": function_label,
                "cron_expression": "0 0 0 * * *",
                "description": "Generate and email nightly analytics report",
                "timezone": "UTC",
                "enabled": True,
            }
        )
        schedule_id = schedule["id"]
        print(f"Created schedule: {schedule.get('name')} (id: {schedule_id})")

        print("\n--- list_schedules ---")
        print(f"Schedules: {await client.list_schedules()}")

        print("\n--- get_schedule ---")
        fetched = await client.get_schedule(schedule_id)
        print(f"Fetched: {fetched.get('name')} cron={fetched.get('cron_expression')}")

        print("\n--- update_schedule ---")
        updated = await client.update_schedule(
            schedule_id,
            {
                "cron_expression": "0 30 1 * * *",
                "description": "Changed to 1:30 AM UTC",
            },
        )
        print(f"Updated cron: {updated.get('cron_expression')}")

        print("\n--- trigger_schedule ---")
        print(f"Triggered: {await client.trigger_schedule(schedule_id)}")

        print("\n--- pause_schedule ---")
        print(
            f"Paused: enabled={(await client.pause_schedule(schedule_id)).get('enabled')}"
        )

        print("\n--- resume_schedule ---")
        print(
            f"Resumed: enabled={(await client.resume_schedule(schedule_id)).get('enabled')}"
        )
    finally:
        cleanup_error = None
        if schedule_id is not None:
            try:
                await client.delete_schedule(schedule_id)
                print("Schedule deleted successfully")
            except Exception as exc:
                cleanup_error = exc
        try:
            await client.delete_function(function_id)
        except Exception as exc:
            if cleanup_error is None:
                cleanup_error = exc
        if cleanup_error is not None:
            raise cleanup_error

    print("\n=== Schedule management example completed ===")


if __name__ == "__main__":
    asyncio.run(main())
