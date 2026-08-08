"""ekoDB Python Client - Goal Template CRUD Example

Demonstrates creating, listing, getting, updating, and deleting goal templates.
"""

import asyncio
import os

from ekodb_client import Client


async def main():
    print("=== ekoDB Goal Template CRUD Example (Python) ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY")
    if not api_key:
        raise ValueError("API_BASE_KEY environment variable is required")

    client = Client.new(base_url, api_key)

    # 1. Create a goal template
    print("--- Creating goal template ---")
    template = await client.goal_template_create(
        {
            "title": "Data Migration",
            "description": "Template for migrating data between schemas",
            "steps": [
                {"description": "Analyze source schema"},
                {"description": "Create target schema"},
                {"description": "Migrate records"},
                {"description": "Validate results"},
            ],
        }
    )
    template_id = template["id"]
    print(f"Created template: {template.get('title')} (id: {template_id})")

    # 2. List all templates
    print("\n--- Listing templates ---")
    templates = await client.goal_template_list()
    print(f"Templates: {templates}")

    # 3. Get template by ID
    print("\n--- Getting template ---")
    fetched = await client.goal_template_get(template_id)
    print(f"Fetched: {fetched.get('title')}")

    # 4. Update template
    print("\n--- Updating template ---")
    updated = await client.goal_template_update(
        template_id,
        {
            "description": "Updated: comprehensive data migration workflow",
        },
    )
    print(f"Updated description: {updated.get('description')}")

    # 5. Delete template
    print("\n--- Deleting template ---")
    await client.goal_template_delete(template_id)
    print("Template deleted successfully")

    print("\n✓ Goal template CRUD example completed")


if __name__ == "__main__":
    asyncio.run(main())
