from typing import Any, Dict, TypeVar, cast
from .types import TypedValue, ValueType

T = TypeVar('T')

def create_typed_value(value: Any) -> TypedValue[Any]:
    """
    Create a TypedValue instance by inferring the type from the Python value.
    
    Args:
        value: Any Python value to be converted to a TypedValue
        
    Returns:
        TypedValue: A properly typed value for the EkoDB API
        
    Examples:
        >>> create_typed_value("hello")
        TypedValue(type=ValueType.STRING, value="hello")
        >>> create_typed_value(42)
        TypedValue(type=ValueType.INTEGER, value=42)
    """
    if isinstance(value, str):
        return TypedValue(type=ValueType.STRING, value=value)
    elif isinstance(value, bool):
        return TypedValue(type=ValueType.BOOLEAN, value=value)
    elif isinstance(value, int):
        return TypedValue(type=ValueType.INTEGER, value=value)
    elif isinstance(value, float):
        return TypedValue(type=ValueType.FLOAT, value=value)
    elif value is None:
        raise ValueError("Cannot create TypedValue from None")
    else:
        raise ValueError(f"Unsupported type for value: {type(value)}")

def create_typed_record(data: Dict[str, Any]) -> Dict[str, TypedValue[Any]]:
    """
    Convert a dictionary of Python values to a dictionary of TypedValues.
    
    Args:
        data: Dictionary of field names to Python values
        
    Returns:
        Dict[str, TypedValue]: Dictionary of field names to TypedValues
        
    Examples:
        >>> create_typed_record({"name": "John", "age": 30})
        {
            "name": TypedValue(type=ValueType.STRING, value="John"),
            "age": TypedValue(type=ValueType.INTEGER, value=30)
        }
    """
    return {
        key: create_typed_value(value)
        for key, value in data.items()
        if value is not None
    }

def extract_value(typed_value: TypedValue[T]) -> T:
    """
    Extract the raw value from a TypedValue instance.
    
    Args:
        typed_value: TypedValue instance to extract from
        
    Returns:
        The raw value contained in the TypedValue
        
    Examples:
        >>> extract_value(TypedValue(type=ValueType.STRING, value="hello"))
        "hello"
    """
    return typed_value.value

def extract_record_values(record: Dict[str, TypedValue[Any]]) -> Dict[str, Any]:
    """
    Convert a dictionary of TypedValues to a dictionary of raw Python values.
    
    Args:
        record: Dictionary of field names to TypedValues
        
    Returns:
        Dict[str, Any]: Dictionary of field names to raw Python values
        
    Examples:
        >>> extract_record_values({
        ...     "name": TypedValue(type=ValueType.STRING, value="John"),
        ...     "age": TypedValue(type=ValueType.INTEGER, value=30)
        ... })
        {"name": "John", "age": 30}
    """
    return {
        key: extract_value(typed_value)
        for key, typed_value in record.items()
    }

def validate_collection_name(collection: str) -> None:
    """
    Validate that a collection name meets the required format.
    
    Args:
        collection: Name of the collection to validate
        
    Raises:
        ValueError: If the collection name is invalid
        
    Examples:
        >>> validate_collection_name("users")  # Valid
        >>> validate_collection_name("")  # Raises ValueError
        >>> validate_collection_name("users/invalid")  # Raises ValueError
    """
    if not collection:
        raise ValueError("Collection name cannot be empty")
    if not collection.isalnum() and not all(c in '-_' for c in collection if not c.isalnum()):
        raise ValueError(
            "Collection name can only contain alphanumeric characters, hyphens, and underscores"
        )
    if len(collection) > 64:
        raise ValueError("Collection name cannot be longer than 64 characters")

def validate_record_id(record_id: str) -> None:
    """
    Validate that a record ID meets the required format.
    
    Args:
        record_id: ID to validate
        
    Raises:
        ValueError: If the record ID is invalid
        
    Examples:
        >>> validate_record_id("user-123")  # Valid
        >>> validate_record_id("")  # Raises ValueError
        >>> validate_record_id("user/123")  # Raises ValueError
    """
    if not record_id:
        raise ValueError("Record ID cannot be empty")
    if not record_id.isalnum() and not all(c in '-_' for c in record_id if not c.isalnum()):
        raise ValueError(
            "Record ID can only contain alphanumeric characters, hyphens, and underscores"
        )
    if len(record_id) > 64:
        raise ValueError("Record ID cannot be longer than 64 characters")