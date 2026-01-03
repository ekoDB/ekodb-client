"""
Utility functions for working with ekoDB records
"""

from typing import Any, Dict, List, Optional, TypeVar

T = TypeVar("T")


def get_value(field: Any) -> Any:
    """
    Extract the value from an ekoDB field object.
    ekoDB returns fields as {"type": "String", "value": "..."} objects.
    This helper safely extracts the value or returns the input if it's not a field object.

    Args:
        field: The field value from ekoDB (may be {"type", "value"} or a plain value)

    Returns:
        The extracted value

    Example:
        >>> user = await client.find_by_id("users", user_id)
        >>> email = get_value(user["email"])  # Extracts string from {"type": "String", "value": "user@example.com"}
        >>> age = get_value(user["age"])      # Extracts int from {"type": "Integer", "value": 25}
    """
    if isinstance(field, dict) and "value" in field:
        return field["value"]
    return field


def get_values(record: Dict[str, Any], fields: List[str]) -> Dict[str, Any]:
    """
    Extract values from multiple fields in a record.
    Useful for processing entire records returned from ekoDB.

    Args:
        record: The record object from ekoDB
        fields: List of field names to extract values from

    Returns:
        Dictionary with extracted values

    Example:
        >>> user = await client.find_by_id("users", user_id)
        >>> values = get_values(user, ["email", "first_name", "status"])
        >>> email = values["email"]
    """
    result = {}
    for field in fields:
        if field in record:
            result[field] = get_value(record[field])
    return result


def get_datetime_value(field: Any) -> Optional[Any]:
    """Extract a datetime value from an ekoDB DateTime field"""
    from datetime import datetime

    val = get_value(field)
    if isinstance(val, datetime):
        return val
    if isinstance(val, str):
        try:
            return datetime.fromisoformat(val.replace("Z", "+00:00"))
        except (ValueError, TypeError):
            return None
    return None


def get_uuid_value(field: Any) -> Optional[str]:
    """Extract a UUID string from an ekoDB UUID field"""
    val = get_value(field)
    return val if isinstance(val, str) else None


def get_decimal_value(field: Any) -> Optional[float]:
    """Extract a float from an ekoDB Decimal field"""
    val = get_value(field)
    if isinstance(val, (int, float)):
        return float(val)
    if isinstance(val, str):
        try:
            return float(val)
        except (ValueError, TypeError):
            return None
    return None


def get_duration_value(field: Any) -> Optional[float]:
    """Extract duration in seconds from an ekoDB Duration field"""
    val = get_value(field)
    if isinstance(val, (int, float)):
        return float(val)
    if isinstance(val, dict):
        secs = val.get("secs", 0)
        nanos = val.get("nanos", 0)
        return float(secs) + (float(nanos) / 1_000_000_000)
    return None


def get_bytes_value(field: Any) -> Optional[bytes]:
    """Extract bytes from an ekoDB Bytes field"""
    import base64

    val = get_value(field)
    if isinstance(val, bytes):
        return val
    if isinstance(val, list):
        return bytes(val)
    if isinstance(val, str):
        try:
            return base64.b64decode(val)
        except Exception:
            return None
    return None


def get_binary_value(field: Any) -> Optional[bytes]:
    """Extract bytes from an ekoDB Binary field"""
    return get_bytes_value(field)


def get_array_value(field: Any) -> Optional[List[Any]]:
    """Extract a list from an ekoDB Array field"""
    val = get_value(field)
    return val if isinstance(val, list) else None


def get_set_value(field: Any) -> Optional[List[Any]]:
    """Extract a list from an ekoDB Set field"""
    val = get_value(field)
    return val if isinstance(val, list) else None


def get_vector_value(field: Any) -> Optional[List[float]]:
    """Extract a list of floats from an ekoDB Vector field"""
    val = get_value(field)
    if isinstance(val, list):
        try:
            return [float(v) for v in val]
        except (ValueError, TypeError):
            return None
    return None


def get_object_value(field: Any) -> Optional[Dict[str, Any]]:
    """Extract a dict from an ekoDB Object field"""
    val = get_value(field)
    return val if isinstance(val, dict) else None


def extract_record(record: Dict[str, Any]) -> Dict[str, Any]:
    """
    Transform an entire record by extracting all field values.
    Preserves the 'id' field and extracts values from all other fields.

    Args:
        record: The record object from ekoDB

    Returns:
        Dictionary with all field values extracted

    Example:
        >>> user = await client.find_by_id("users", user_id)
        >>> plain_user = extract_record(user)
        >>> # plain_user is now {"id": "123", "email": "user@example.com", "first_name": "John", ...}
    """
    if not isinstance(record, dict):
        return record

    result = {}
    for key, value in record.items():
        if key == "id":
            result[key] = value
        else:
            result[key] = get_value(value)
    return result


# =============================================================================
# Wrapped Type Builders
# =============================================================================
# These functions create wrapped type objects for sending to ekoDB.
# Use these when inserting/updating records with special field types.
#
# Example:
#     await client.insert("orders", {
#         "id": field_uuid("550e8400-e29b-41d4-a716-446655440000"),
#         "total": field_decimal("99.99"),
#         "created_at": field_datetime(datetime.now()),
#         "tags": field_set(["sale", "featured"]),
#     })


def field_uuid(value: str) -> Dict[str, Any]:
    """Create a UUID field value"""
    return {"type": "UUID", "value": value}


def field_decimal(value: str) -> Dict[str, Any]:
    """Create a Decimal field value for precise numeric values"""
    return {"type": "Decimal", "value": value}


def field_datetime(value: Any) -> Dict[str, Any]:
    """Create a DateTime field value from datetime or RFC3339 string"""
    from datetime import datetime

    if isinstance(value, datetime):
        return {"type": "DateTime", "value": value.isoformat()}
    return {"type": "DateTime", "value": value}


def field_duration(milliseconds: int) -> Dict[str, Any]:
    """Create a Duration field value (in milliseconds)"""
    return {"type": "Duration", "value": milliseconds}


def field_number(value: Any) -> Dict[str, Any]:
    """Create a Number field value (flexible numeric type)"""
    return {"type": "Number", "value": value}


def field_set(values: List[Any]) -> Dict[str, Any]:
    """Create a Set field value (unique elements)"""
    return {"type": "Set", "value": values}


def field_vector(values: List[float]) -> Dict[str, Any]:
    """Create a Vector field value (for embeddings/similarity search)"""
    return {"type": "Vector", "value": values}


def field_binary(value: Any) -> Dict[str, Any]:
    """Create a Binary field value from bytes or base64 string"""
    import base64

    if isinstance(value, bytes):
        return {"type": "Binary", "value": base64.b64encode(value).decode("utf-8")}
    return {"type": "Binary", "value": value}


def field_bytes(value: Any) -> Dict[str, Any]:
    """Create a Bytes field value from bytes or base64 string"""
    import base64

    if isinstance(value, bytes):
        return {"type": "Bytes", "value": base64.b64encode(value).decode("utf-8")}
    return {"type": "Bytes", "value": value}


def field_array(values: List[Any]) -> Dict[str, Any]:
    """Create an Array field value"""
    return {"type": "Array", "value": values}


def field_object(value: Dict[str, Any]) -> Dict[str, Any]:
    """Create an Object field value"""
    return {"type": "Object", "value": value}


def field_string(value: str) -> Dict[str, Any]:
    """Create a String field value (explicit wrapping)"""
    return {"type": "String", "value": value}


def field_integer(value: int) -> Dict[str, Any]:
    """Create an Integer field value (explicit wrapping)"""
    return {"type": "Integer", "value": value}


def field_float(value: float) -> Dict[str, Any]:
    """Create a Float field value (explicit wrapping)"""
    return {"type": "Float", "value": value}


def field_boolean(value: bool) -> Dict[str, Any]:
    """Create a Boolean field value (explicit wrapping)"""
    return {"type": "Boolean", "value": value}
