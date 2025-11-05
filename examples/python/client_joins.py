"""
Join Operations Example

Demonstrates multi-collection joins:
- Single collection joins
- Multi-collection joins
- Using joins with queries
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv
from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)


async def main():
    # Create client
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
    client = Client.new(base_url, api_key)

    print("=== Join Operations Examples ===\n")

    # Use unique collection names
    users_collection = "joins_users_client_py"
    departments_collection = "joins_departments_client_py"
    orders_collection = "joins_orders_client_py"
    profiles_collection = "joins_profiles_client_py"

    # Cleanup any existing test collections
    try:
        await client.delete_collection(users_collection)
        await client.delete_collection(departments_collection)
        await client.delete_collection(orders_collection)
        await client.delete_collection(profiles_collection)
    except:
        pass  # Ignore errors if collections don't exist

    # Setup: Create sample data
    print("Setting up sample data...")

    # Create departments first (need their IDs for users)
    dept1 = await client.insert(
        departments_collection,
        {
            "name": "Engineering",
            "location": "Building A",
        },
    )

    dept2 = await client.insert(
        departments_collection,
        {
            "name": "Sales",
            "location": "Building B",
        },
    )

    # Create users with actual department IDs
    user1 = await client.insert(
        users_collection,
        {
            "name": "Alice Johnson",
            "email": "alice@example.com",
            "department_id": dept1.get("id"),
        },
    )

    user2 = await client.insert(
        users_collection,
        {
            "name": "Bob Smith",
            "email": "bob@example.com",
            "department_id": dept2.get("id"),
        },
    )

    # Create orders
    await client.insert(
        orders_collection,
        {
            "user_id": user1.get("id"),
            "product": "Laptop",
            "amount": 1200,
            "status": "completed",
        },
    )

    await client.insert(
        orders_collection,
        {
            "user_id": user1.get("id"),
            "product": "Mouse",
            "amount": 25,
            "status": "completed",
        },
    )

    await client.insert(
        orders_collection,
        {
            "user_id": user2.get("id"),
            "product": "Keyboard",
            "amount": 75,
            "status": "pending",
        },
    )

    # Create profiles
    await client.insert(
        profiles_collection,
        {
            "user_id": user1.get("id"),
            "bio": "Senior Software Engineer",
            "skills": ["JavaScript", "TypeScript", "React"],
        },
    )

    await client.insert(
        profiles_collection,
        {
            "user_id": user2.get("id"),
            "bio": "Sales Manager",
            "skills": ["Negotiation", "CRM", "Communication"],
        },
    )

    print("✅ Sample data created\n")

    # Example 1: Simple single collection join
    print("1. Single collection join (users with departments):")
    query1 = {
        "join": {
            "collections": [departments_collection],
            "local_field": "department_id",
            "foreign_field": "id",
            "as_field": "department",
        },
        "limit": 10,
    }

    results1 = await client.find(users_collection, query1)
    print(f"Found {len(results1)} users with department data:")
    for user in results1:
        user_name = (
            user.get("name", {}).get("value")
            if isinstance(user.get("name"), dict)
            else user.get("name", "Unknown")
        )
        # Join returns an array, get first element
        depts = user.get("department", [])
        dept = depts[0] if depts else {}
        dept_name = (
            dept.get("name", {}).get("value")
            if isinstance(dept.get("name"), dict)
            else dept.get("name", "No department")
        )
        print(f"  - {user_name}: {dept_name}")
    print()

    # Example 2: Join with filtering
    print("2. Join with filtering:")
    query2 = {
        "filter": {
            "type": "Condition",
            "content": {
                "field": "department_id",
                "operator": "Eq",
                "value": dept1.get("id"),
            },
        },
        "join": {
            "collections": [departments_collection],
            "local_field": "department_id",
            "foreign_field": "id",
            "as_field": "department",
        },
    }

    results2 = await client.find(users_collection, query2)
    print(f"Found {len(results2)} users in Engineering:")
    for user in results2:
        user_name = (
            user.get("name", {}).get("value")
            if isinstance(user.get("name"), dict)
            else user.get("name", "Unknown")
        )
        # Join returns an array, get first element
        depts = user.get("department", [])
        dept = depts[0] if depts else {}
        location = (
            dept.get("location", {}).get("value")
            if isinstance(dept.get("location"), dict)
            else dept.get("location", "Unknown")
        )
        print(f"  - {user_name}: {location}")
    print()

    # Example 3: Join with user profiles
    print("3. Join with user profiles:")
    query3 = {
        "join": {
            "collections": [profiles_collection],
            "local_field": "id",
            "foreign_field": "user_id",
            "as_field": "profile",
        },
        "limit": 10,
    }

    results3 = await client.find(users_collection, query3)
    print(f"Found {len(results3)} users with profile data:")
    for user in results3:
        user_name = (
            user.get("name", {}).get("value")
            if isinstance(user.get("name"), dict)
            else user.get("name", "Unknown")
        )
        # Join returns an array, get first element
        profiles = user.get("profile", [])
        profile = profiles[0] if profiles else {}
        bio = (
            profile.get("bio", {}).get("value")
            if isinstance(profile.get("bio"), dict)
            else profile.get("bio", "N/A")
        )
        print(f"  - {user_name}: {bio}")
    print()

    # Example 4: Join orders with user data
    print("4. Join orders with user data:")
    query4 = {
        "filter": {
            "type": "Condition",
            "content": {"field": "status", "operator": "Eq", "value": "completed"},
        },
        "join": {
            "collections": [users_collection],
            "local_field": "user_id",
            "foreign_field": "id",
            "as_field": "user",
        },
    }

    results4 = await client.find(orders_collection, query4)
    print(f"Found {len(results4)} completed orders:")
    for order in results4:
        product = (
            order.get("product", {}).get("value")
            if isinstance(order.get("product"), dict)
            else order.get("product", "Unknown")
        )
        amount = (
            order.get("amount", {}).get("value")
            if isinstance(order.get("amount"), dict)
            else order.get("amount", 0)
        )
        # Join returns an array, get first element
        users = order.get("user", [])
        user = users[0] if users else {}
        user_name = (
            user.get("name", {}).get("value")
            if isinstance(user.get("name"), dict)
            else user.get("name", "Unknown")
        )
        print(f"  - {product} (${amount}) by {user_name}")
    print()

    # Example 5: Complex join with multiple conditions
    print("5. Complex join with multiple conditions:")
    query5 = {
        "filter": {
            "type": "Condition",
            "content": {
                "field": "email",
                "operator": "Contains",
                "value": "@example.com",
            },
        },
        "join": {
            "collections": [departments_collection],
            "local_field": "department_id",
            "foreign_field": "id",
            "as_field": "department",
        },
        "sort": [{"field": "name", "ascending": True}],
    }

    results5 = await client.find(users_collection, query5)
    print(f"Found {len(results5)} users with example.com emails:")
    for user in results5:
        user_name = (
            user.get("name", {}).get("value")
            if isinstance(user.get("name"), dict)
            else user.get("name", "Unknown")
        )
        email = (
            user.get("email", {}).get("value")
            if isinstance(user.get("email"), dict)
            else user.get("email", "Unknown")
        )
        # Join returns an array, get first element
        depts = user.get("department", [])
        dept = depts[0] if depts else {}
        location = (
            dept.get("location", {}).get("value")
            if isinstance(dept.get("location"), dict)
            else dept.get("location", "N/A")
        )
        print(f"  - {user_name} ({email}): {location}")
    print()

    # Cleanup
    print("=== Cleanup ===")
    await client.delete_collection(users_collection)
    await client.delete_collection(departments_collection)
    await client.delete_collection(orders_collection)
    await client.delete_collection(profiles_collection)
    print("✅ Deleted test collections\n")

    print("✅ Join operations examples completed!")


if __name__ == "__main__":
    asyncio.run(main())
