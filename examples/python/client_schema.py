"""
Schema Management Example

Demonstrates schema creation and management:
- Creating collections with schemas
- Field types and constraints
- Indexes (text, vector, btree, hash)
- Validation rules
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv
from ekodb_client import Client

COLLECTIONS = (
    "schema_users_client_py",
    "schema_products_client_py",
    "schema_documents_client_py",
    "schema_employees_client_py",
)

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)


async def run_example(client):
    print("=== Schema Management Examples ===\n")

    # Use unique collection names
    (
        users_collection,
        products_collection,
        documents_collection,
        employees_collection,
    ) = COLLECTIONS

    # Example 1: Create a simple user schema
    print("1. Creating user schema with basic fields:")
    user_schema = {
        "fields": {
            "name": {"field_type": "String", "required": True, "regex": "^[a-zA-Z ]+$"},
            "email": {
                "field_type": "String",
                "required": True,
                "unique": True,
                "regex": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
            },
            "age": {"field_type": "Integer", "min": 0, "max": 150},
            "status": {
                "field_type": "String",
                "default": "active",
                "enums": ["active", "inactive", "suspended"],
            },
        }
    }

    await client.create_collection(users_collection, user_schema)
    print("✅ User schema created\n")

    # Example 2: Schema with text index for full-text search
    print("2. Creating product schema with text index:")
    product_schema = {
        "fields": {
            "name": {
                "field_type": "String",
                "required": True,
                "index": {"type": "text", "language": "english"},
            },
            "description": {
                "field_type": "String",
                "index": {"type": "text", "language": "english"},
            },
            "price": {"field_type": "Float", "required": True, "min": 0},
            "category": {
                "field_type": "String",
                "required": True,
                "index": {"type": "btree"},
            },
            "sku": {
                "field_type": "String",
                "required": True,
                "unique": True,
                "index": {"type": "hash"},
            },
        }
    }

    await client.create_collection(products_collection, product_schema)
    print("✅ Product schema with indexes created\n")

    # Example 3: Schema with vector index for semantic search
    print("3. Creating document schema with vector index:")
    document_schema = {
        "fields": {
            "title": {
                "field_type": "String",
                "required": True,
                "index": {"type": "text", "language": "english"},
            },
            "content": {
                "field_type": "String",
                "required": True,
                "index": {"type": "text", "language": "english"},
            },
            "embedding": {
                "field_type": "Array",
                "index": {
                    "type": "vector",
                    "algorithm": "hnsw",
                    "metric": "cosine",
                    "m": 16,
                    "ef_construction": 200,
                },
            },
            "tags": {"field_type": "Array"},
            "created_at": {"field_type": "DateTime"},
        }
    }

    await client.create_collection(documents_collection, document_schema)
    print("✅ Document schema with vector index created\n")

    # Example 4: Get collection schema
    print("4. Retrieving collection schema:")
    schema = await client.get_schema(users_collection)
    print(f"Schema fields: {list(schema.get('fields', {}).keys())}")
    print(f"Schema version: {schema.get('version', 1)}")
    print()

    # Example 5: Get collection metadata
    print("5. Retrieving collection metadata:")
    metadata = await client.get_collection(users_collection)
    print(
        f"Collection has {len(metadata.get('collection', {}).get('fields', {}))} fields"
    )
    print()

    # Example 6: Complex schema with all constraint types
    print("6. Creating employee schema with all constraint types:")
    employee_schema = {
        "fields": {
            "employee_id": {
                "field_type": "String",
                "required": True,
                "unique": True,
                "regex": "^EMP[0-9]{6}$",
            },
            "full_name": {"field_type": "String", "required": True},
            "email": {
                "field_type": "String",
                "required": True,
                "unique": True,
                "index": {"type": "hash"},
            },
            "department": {
                "field_type": "String",
                "required": True,
                "enums": ["Engineering", "Sales", "Marketing", "HR", "Finance"],
                "index": {"type": "btree"},
            },
            "salary": {
                "field_type": "Float",
                "required": True,
                "min": 0,
                "max": 1000000,
            },
            "hire_date": {"field_type": "DateTime", "required": True},
            "is_active": {"field_type": "Boolean", "default": True},
            "bio": {
                "field_type": "String",
                "index": {"type": "text", "language": "english"},
            },
        },
        "bypass_ripple": False,
    }

    await client.create_collection(employees_collection, employee_schema)
    print("✅ Employee schema with all constraints created\n")


async def cleanup(client):
    errors = []
    for collection in COLLECTIONS:
        try:
            await client.delete_collection(collection)
        except Exception as error:  # noqa: BLE001 - attempt every owned cleanup
            if "404" not in str(error) and "not found" not in str(error).lower():
                errors.append(f"{collection}: {error}")
    if errors:
        raise RuntimeError("cleanup failed: " + "; ".join(errors))


async def main():
    client = Client.new(
        os.getenv("API_BASE_URL", "http://localhost:8080"),
        os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb"),
    )
    await cleanup(client)
    primary_error = None
    try:
        await run_example(client)
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        primary_error = error
    cleanup_error = None
    try:
        await cleanup(client)
    except Exception as error:  # noqa: BLE001 - preserve the primary failure
        cleanup_error = error
    if primary_error is not None:
        if cleanup_error is not None:
            primary_error.add_note(f"Cleanup also failed: {cleanup_error}")
            print(f"⚠️  Cleanup also failed: {cleanup_error}")
        raise primary_error
    if cleanup_error is not None:
        raise cleanup_error
    print("✅ Schema management examples completed!")


if __name__ == "__main__":
    asyncio.run(main())
