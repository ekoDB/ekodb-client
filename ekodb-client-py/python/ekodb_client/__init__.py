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

from ._ekodb_client import (
    Client,
    RateLimitInfo,
    RateLimitError,
    WebSocketClient,
    SubscriptionReceiver,
    ChatStreamReceiver,
    HealthStatus,
    extract_record_id,
    parse_health_status,
)

from .stages import Stage, ChatMessage, parameter_ref
from .query_builder import QueryBuilder
from .schema_builder import SchemaBuilder, FieldTypeSchemaBuilder
from .join import JoinConfig
from .utils import (
    get_value,
    get_values,
    extract_record,
    get_datetime_value,
    get_uuid_value,
    get_decimal_value,
    get_duration_value,
    get_bytes_value,
    get_binary_value,
    get_array_value,
    get_set_value,
    get_vector_value,
    get_object_value,
    # Field builders for wrapped types
    field_uuid,
    field_decimal,
    field_datetime,
    field_duration,
    field_number,
    field_set,
    field_vector,
    field_binary,
    field_bytes,
    field_array,
    field_object,
    field_string,
    field_integer,
    field_float,
    field_boolean,
)

# Derive the version from the installed distribution metadata (maturin sets it
# from pyproject.toml / Cargo.toml) so __version__ never drifts from the real
# package version. Falls back to a literal only when running from source without
# an installed distribution.
from importlib.metadata import version as _dist_version, PackageNotFoundError

try:
    __version__ = _dist_version("ekodb_client")
except PackageNotFoundError:
    __version__ = "0.21.0"

# Canonical HealthStatus.status values (status is a plain string; an
# off-contract value reported by the server is preserved verbatim).
HEALTH_OK = "ok"
HEALTH_DEGRADED = "degraded"
HEALTH_UNKNOWN = "unknown"

__all__ = [
    "Client",
    "WebSocketClient",
    "SubscriptionReceiver",
    "ChatStreamReceiver",
    "RateLimitInfo",
    "RateLimitError",
    "HealthStatus",
    "parse_health_status",
    "HEALTH_OK",
    "HEALTH_DEGRADED",
    "HEALTH_UNKNOWN",
    # Stage helpers for script functions
    "Stage",
    "ChatMessage",
    "parameter_ref",
    # Query, Schema, and Join builders
    "QueryBuilder",
    "SchemaBuilder",
    "FieldTypeSchemaBuilder",
    "JoinConfig",
    # Value extractors
    "get_value",
    "get_values",
    "extract_record",
    "extract_record_id",
    "get_datetime_value",
    "get_uuid_value",
    "get_decimal_value",
    "get_duration_value",
    "get_bytes_value",
    "get_binary_value",
    "get_array_value",
    "get_set_value",
    "get_vector_value",
    "get_object_value",
    # Field builders
    "field_uuid",
    "field_decimal",
    "field_datetime",
    "field_duration",
    "field_number",
    "field_set",
    "field_vector",
    "field_binary",
    "field_bytes",
    "field_array",
    "field_object",
    "field_string",
    "field_integer",
    "field_float",
    "field_boolean",
]
