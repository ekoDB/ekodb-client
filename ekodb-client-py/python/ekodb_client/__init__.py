"""
ekoDB Python Client

A high-performance Python client for ekoDB, built with Rust for speed and safety.

Example:
    ```python
    import asyncio
    from ekodb_client import Client, RateLimitError

    async def main():
        # Create client with configuration
        client = Client.new(
            "http://localhost:8080",
            "your-api-key",
            should_retry=True,  # Enable automatic retries (default: True)
            max_retries=3,      # Maximum retry attempts (default: 3)
            timeout_secs=30     # Request timeout in seconds (default: 30)
        )
        
        try:
            # Insert a document
            record = await client.insert("users", {
                "name": "John Doe",
                "age": 30,
                "active": True
            })
            print(f"Inserted: {record}")
            
            # Find documents
            results = await client.find("users", limit=10)
            print(f"Found {len(results)} documents")
            
            # Update a document
            updated = await client.update("users", record["id"], {
                "age": 31
            })
            
            # Delete a document
            await client.delete("users", record["id"])
            
        except RateLimitError as e:
            print(f"Rate limited! Retry after {e.retry_after_secs} seconds")

    asyncio.run(main())
    ```
"""

from ._ekodb_client import Client, RateLimitInfo, RateLimitError

__version__ = "0.1.0"
__all__ = ["Client", "RateLimitInfo", "RateLimitError"]
