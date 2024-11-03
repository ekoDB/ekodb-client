import asyncio
from ekodb import EkoDBClient, TypedValue, ValueType

async def main():
    # Initialize client
    client = EkoDBClient({
        "base_url": "http://localhost:8080",
        "api_key": "test-api-key"
    })

    # Create a test record
    record = {
        "name": TypedValue(type=ValueType.STRING, value="Test User"),
        "age": TypedValue(type=ValueType.INTEGER, value=25)
    }

    try:
        # Insert record
        record_id = await client.insert("users", record)
        print(f"Inserted record with ID: {record_id}")

        # Retrieve record
        result = await client.find_by_id("users", record_id)
        print(f"Retrieved record: {result}")

    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    asyncio.run(main())