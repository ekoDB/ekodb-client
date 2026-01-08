"""
Unit tests for ekoDB Python client utility functions.

Run with: pytest tests/ -v
"""

from datetime import datetime
from ekodb_client.utils import (
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


# ============================================================================
# get_value Tests
# ============================================================================


class TestGetValue:
    """Tests for get_value function"""

    def test_get_value_from_wrapped_string(self):
        """Extract string from wrapped field"""
        field = {"type": "String", "value": "hello world"}
        assert get_value(field) == "hello world"

    def test_get_value_from_wrapped_integer(self):
        """Extract integer from wrapped field"""
        field = {"type": "Integer", "value": 42}
        assert get_value(field) == 42

    def test_get_value_from_wrapped_float(self):
        """Extract float from wrapped field"""
        field = {"type": "Float", "value": 3.14}
        assert get_value(field) == 3.14

    def test_get_value_from_wrapped_boolean(self):
        """Extract boolean from wrapped field"""
        field = {"type": "Boolean", "value": True}
        assert get_value(field) is True

    def test_get_value_from_wrapped_null(self):
        """Extract null from wrapped field"""
        field = {"type": "Null", "value": None}
        assert get_value(field) is None

    def test_get_value_from_plain_string(self):
        """Return plain string as-is"""
        assert get_value("plain string") == "plain string"

    def test_get_value_from_plain_int(self):
        """Return plain int as-is"""
        assert get_value(123) == 123

    def test_get_value_from_plain_float(self):
        """Return plain float as-is"""
        assert get_value(3.14159) == 3.14159

    def test_get_value_from_plain_bool(self):
        """Return plain bool as-is"""
        assert get_value(True) is True
        assert get_value(False) is False

    def test_get_value_from_plain_none(self):
        """Return None as-is"""
        assert get_value(None) is None

    def test_get_value_from_plain_list(self):
        """Return plain list as-is"""
        lst = [1, 2, 3]
        assert get_value(lst) == [1, 2, 3]

    def test_get_value_from_dict_without_value_key(self):
        """Return dict without 'value' key as-is"""
        d = {"name": "test", "count": 5}
        assert get_value(d) == {"name": "test", "count": 5}


# ============================================================================
# get_values Tests
# ============================================================================


class TestGetValues:
    """Tests for get_values function"""

    def test_get_values_multiple_fields(self):
        """Extract multiple field values from record"""
        record = {
            "name": {"type": "String", "value": "John"},
            "age": {"type": "Integer", "value": 30},
            "active": {"type": "Boolean", "value": True},
        }
        result = get_values(record, ["name", "age", "active"])
        assert result == {"name": "John", "age": 30, "active": True}

    def test_get_values_partial_fields(self):
        """Extract only requested fields"""
        record = {
            "name": {"type": "String", "value": "John"},
            "age": {"type": "Integer", "value": 30},
            "email": {"type": "String", "value": "john@example.com"},
        }
        result = get_values(record, ["name", "email"])
        assert result == {"name": "John", "email": "john@example.com"}
        assert "age" not in result

    def test_get_values_missing_fields(self):
        """Handle missing fields gracefully"""
        record = {
            "name": {"type": "String", "value": "John"},
        }
        result = get_values(record, ["name", "missing_field"])
        assert result == {"name": "John"}
        assert "missing_field" not in result

    def test_get_values_empty_record(self):
        """Handle empty record"""
        result = get_values({}, ["name", "age"])
        assert result == {}

    def test_get_values_empty_fields_list(self):
        """Handle empty fields list"""
        record = {"name": {"type": "String", "value": "John"}}
        result = get_values(record, [])
        assert result == {}

    def test_get_values_mixed_wrapped_and_plain(self):
        """Handle mixed wrapped and plain values"""
        record = {
            "wrapped": {"type": "String", "value": "wrapped_value"},
            "plain": "plain_value",
        }
        result = get_values(record, ["wrapped", "plain"])
        assert result == {"wrapped": "wrapped_value", "plain": "plain_value"}


# ============================================================================
# Specialized Value Extractors Tests
# ============================================================================


class TestGetDatetimeValue:
    """Tests for get_datetime_value function"""

    def test_datetime_from_iso_string(self):
        """Parse ISO format datetime string"""
        field = {"type": "DateTime", "value": "2024-01-15T10:30:00Z"}
        result = get_datetime_value(field)
        assert result is not None
        assert result.year == 2024
        assert result.month == 1
        assert result.day == 15

    def test_datetime_from_iso_with_offset(self):
        """Parse ISO format with timezone offset"""
        field = {"type": "DateTime", "value": "2024-01-15T10:30:00+00:00"}
        result = get_datetime_value(field)
        assert result is not None
        assert result.year == 2024

    def test_datetime_from_datetime_object(self):
        """Return datetime object as-is"""
        dt = datetime(2024, 1, 15, 10, 30, 0)
        field = {"type": "DateTime", "value": dt}
        result = get_datetime_value(field)
        assert result == dt

    def test_datetime_invalid_string(self):
        """Return None for invalid datetime string"""
        field = {"type": "DateTime", "value": "not-a-datetime"}
        result = get_datetime_value(field)
        assert result is None

    def test_datetime_none_value(self):
        """Return None for None value"""
        result = get_datetime_value(None)
        assert result is None


class TestGetUuidValue:
    """Tests for get_uuid_value function"""

    def test_uuid_from_wrapped(self):
        """Extract UUID from wrapped field"""
        field = {"type": "UUID", "value": "550e8400-e29b-41d4-a716-446655440000"}
        result = get_uuid_value(field)
        assert result == "550e8400-e29b-41d4-a716-446655440000"

    def test_uuid_from_plain_string(self):
        """Extract UUID from plain string"""
        result = get_uuid_value("550e8400-e29b-41d4-a716-446655440000")
        assert result == "550e8400-e29b-41d4-a716-446655440000"

    def test_uuid_non_string(self):
        """Return None for non-string"""
        result = get_uuid_value(12345)
        assert result is None


class TestGetDecimalValue:
    """Tests for get_decimal_value function"""

    def test_decimal_from_int(self):
        """Extract decimal from integer"""
        field = {"type": "Decimal", "value": 42}
        result = get_decimal_value(field)
        assert result == 42.0

    def test_decimal_from_float(self):
        """Extract decimal from float"""
        field = {"type": "Decimal", "value": 3.14159}
        result = get_decimal_value(field)
        assert abs(result - 3.14159) < 0.00001

    def test_decimal_from_string(self):
        """Extract decimal from string representation"""
        field = {"type": "Decimal", "value": "123.456"}
        result = get_decimal_value(field)
        assert abs(result - 123.456) < 0.001

    def test_decimal_invalid_string(self):
        """Return None for invalid decimal string"""
        field = {"type": "Decimal", "value": "not-a-number"}
        result = get_decimal_value(field)
        assert result is None


class TestGetDurationValue:
    """Tests for get_duration_value function"""

    def test_duration_from_seconds(self):
        """Extract duration from seconds"""
        field = {"type": "Duration", "value": 3600}
        result = get_duration_value(field)
        assert result == 3600.0

    def test_duration_from_dict(self):
        """Extract duration from secs/nanos dict"""
        field = {"type": "Duration", "value": {"secs": 10, "nanos": 500000000}}
        result = get_duration_value(field)
        assert abs(result - 10.5) < 0.001

    def test_duration_from_float(self):
        """Extract duration from float"""
        result = get_duration_value(10.5)
        assert result == 10.5


class TestGetBytesValue:
    """Tests for get_bytes_value function"""

    def test_bytes_from_bytes(self):
        """Extract bytes from bytes object"""
        field = {"type": "Bytes", "value": b"hello"}
        result = get_bytes_value(field)
        assert result == b"hello"

    def test_bytes_from_list(self):
        """Extract bytes from list of integers"""
        field = {"type": "Bytes", "value": [104, 101, 108, 108, 111]}  # "hello"
        result = get_bytes_value(field)
        assert result == b"hello"

    def test_bytes_from_base64(self):
        """Extract bytes from base64 string"""
        import base64

        encoded = base64.b64encode(b"hello").decode()
        field = {"type": "Bytes", "value": encoded}
        result = get_bytes_value(field)
        assert result == b"hello"

    def test_bytes_invalid(self):
        """Return None for invalid bytes"""
        result = get_bytes_value(12345)
        assert result is None


class TestGetBinaryValue:
    """Tests for get_binary_value function (alias for get_bytes_value)"""

    def test_binary_from_bytes(self):
        """Extract binary from bytes object"""
        field = {"type": "Binary", "value": b"data"}
        result = get_binary_value(field)
        assert result == b"data"


class TestGetArrayValue:
    """Tests for get_array_value function"""

    def test_array_from_list(self):
        """Extract array from list"""
        field = {"type": "Array", "value": [1, 2, 3, 4, 5]}
        result = get_array_value(field)
        assert result == [1, 2, 3, 4, 5]

    def test_array_from_plain_list(self):
        """Extract array from plain list"""
        result = get_array_value([1, 2, 3])
        assert result == [1, 2, 3]

    def test_array_non_list(self):
        """Return None for non-list"""
        result = get_array_value("not a list")
        assert result is None


class TestGetSetValue:
    """Tests for get_set_value function"""

    def test_set_from_list(self):
        """Extract set from list"""
        field = {"type": "Set", "value": ["a", "b", "c"]}
        result = get_set_value(field)
        assert result == ["a", "b", "c"]


class TestGetVectorValue:
    """Tests for get_vector_value function"""

    def test_vector_from_list(self):
        """Extract vector from list of floats"""
        field = {"type": "Vector", "value": [0.1, 0.2, 0.3, 0.4]}
        result = get_vector_value(field)
        assert result == [0.1, 0.2, 0.3, 0.4]

    def test_vector_from_int_list(self):
        """Extract vector from list of integers (converted to floats)"""
        field = {"type": "Vector", "value": [1, 2, 3]}
        result = get_vector_value(field)
        assert result == [1.0, 2.0, 3.0]

    def test_vector_invalid(self):
        """Return None for invalid vector"""
        field = {"type": "Vector", "value": ["not", "numbers"]}
        result = get_vector_value(field)
        assert result is None


class TestGetObjectValue:
    """Tests for get_object_value function"""

    def test_object_from_dict(self):
        """Extract object from dict"""
        field = {"type": "Object", "value": {"key": "value", "count": 5}}
        result = get_object_value(field)
        assert result == {"key": "value", "count": 5}

    def test_object_non_dict(self):
        """Return None for non-dict"""
        result = get_object_value([1, 2, 3])
        assert result is None


# ============================================================================
# Field Builder Tests
# ============================================================================


class TestFieldBuilders:
    """Tests for field builder functions"""

    def test_field_string(self):
        """Build string field"""
        result = field_string("hello")
        assert result == {"type": "String", "value": "hello"}

    def test_field_integer(self):
        """Build integer field"""
        result = field_integer(42)
        assert result == {"type": "Integer", "value": 42}

    def test_field_float(self):
        """Build float field"""
        result = field_float(3.14)
        assert result == {"type": "Float", "value": 3.14}

    def test_field_boolean(self):
        """Build boolean field"""
        result = field_boolean(True)
        assert result == {"type": "Boolean", "value": True}
        result = field_boolean(False)
        assert result == {"type": "Boolean", "value": False}

    def test_field_uuid(self):
        """Build UUID field"""
        uuid_str = "550e8400-e29b-41d4-a716-446655440000"
        result = field_uuid(uuid_str)
        assert result == {"type": "UUID", "value": uuid_str}

    def test_field_decimal(self):
        """Build decimal field"""
        result = field_decimal(123.456)
        assert result == {"type": "Decimal", "value": 123.456}

    def test_field_datetime(self):
        """Build datetime field"""
        dt = datetime(2024, 1, 15, 10, 30, 0)
        result = field_datetime(dt)
        assert result["type"] == "DateTime"
        assert "value" in result

    def test_field_duration(self):
        """Build duration field"""
        result = field_duration(3600.5)
        assert result == {"type": "Duration", "value": 3600.5}

    def test_field_number(self):
        """Build number field (auto-detects int vs float)"""
        result_int = field_number(42)
        assert result_int["type"] in ["Integer", "Number"]

        result_float = field_number(3.14)
        assert result_float["type"] in ["Float", "Number"]

    def test_field_array(self):
        """Build array field"""
        result = field_array([1, 2, 3])
        assert result == {"type": "Array", "value": [1, 2, 3]}

    def test_field_set(self):
        """Build set field"""
        result = field_set(["a", "b", "c"])
        assert result == {"type": "Set", "value": ["a", "b", "c"]}

    def test_field_vector(self):
        """Build vector field"""
        result = field_vector([0.1, 0.2, 0.3])
        assert result == {"type": "Vector", "value": [0.1, 0.2, 0.3]}

    def test_field_object(self):
        """Build object field"""
        result = field_object({"key": "value"})
        assert result == {"type": "Object", "value": {"key": "value"}}

    def test_field_bytes(self):
        """Build bytes field"""
        result = field_bytes(b"hello")
        assert result["type"] == "Bytes"
        assert "value" in result

    def test_field_binary(self):
        """Build binary field"""
        result = field_binary(b"data")
        assert result["type"] == "Binary"
        assert "value" in result


# ============================================================================
# extract_record Tests
# ============================================================================


class TestExtractRecord:
    """Tests for extract_record function"""

    def test_extract_record_all_fields(self):
        """Extract all fields from a record"""
        record = {
            "id": "user_123",
            "name": {"type": "String", "value": "John Doe"},
            "age": {"type": "Integer", "value": 30},
            "active": {"type": "Boolean", "value": True},
        }
        result = extract_record(record)
        assert result["id"] == "user_123"
        assert result["name"] == "John Doe"
        assert result["age"] == 30
        assert result["active"] is True

    def test_extract_record_with_nested_objects(self):
        """Extract record with nested objects"""
        record = {
            "user": {"type": "Object", "value": {"name": "John", "role": "admin"}},
            "tags": {"type": "Array", "value": ["python", "rust"]},
        }
        result = extract_record(record)
        assert result["user"] == {"name": "John", "role": "admin"}
        assert result["tags"] == ["python", "rust"]

    def test_extract_record_empty(self):
        """Extract empty record"""
        result = extract_record({})
        assert result == {}

    def test_extract_record_with_none_values(self):
        """Extract record with None values"""
        record = {
            "name": {"type": "String", "value": "John"},
            "optional": {"type": "Null", "value": None},
        }
        result = extract_record(record)
        assert result["name"] == "John"
        assert result["optional"] is None


# ============================================================================
# SWR Function Tests
# ============================================================================

from ekodb_client import Stage


class TestSWRStage:
    """Tests for SWR stage creation"""

    def test_swr_stage_basic(self):
        """Test basic SWR stage creation"""
        stage = Stage.swr(
            cache_key="user:{{user_id}}",
            ttl="15m",
            url="https://api.example.com/users/{{user_id}}",
            method="GET",
        )

        assert stage["type"] == "SWR"
        assert stage["cache_key"] == "user:{{user_id}}"
        assert stage["ttl"] == "15m"
        assert stage["url"] == "https://api.example.com/users/{{user_id}}"
        assert stage["method"] == "GET"

    def test_swr_stage_with_headers(self):
        """Test SWR stage with custom headers"""
        headers = {"User-Agent": "ekoDB-Client", "Authorization": "Bearer {{token}}"}
        stage = Stage.swr(
            cache_key="api:resource",
            ttl="1h",
            url="https://api.example.com/resource",
            method="GET",
            headers=headers,
        )

        assert stage["headers"] == headers

    def test_swr_stage_with_post_body(self):
        """Test SWR stage with POST body"""
        body = {"query": "{{search_term}}", "limit": 10}
        stage = Stage.swr(
            cache_key="search:{{search_term}}",
            ttl=900,
            url="https://api.example.com/search",
            method="POST",
            body=body,
        )

        assert stage["method"] == "POST"
        assert stage["body"] == body
        assert stage["ttl"] == 900

    def test_swr_stage_with_audit_collection(self):
        """Test SWR stage with audit trail collection"""
        stage = Stage.swr(
            cache_key="product:{{id}}",
            ttl="1h",
            url="https://api.example.com/products/{{id}}",
            method="GET",
            output_field="product",
            collection="swr_audit_trail",
        )

        assert stage["output_field"] == "product"
        assert stage["collection"] == "swr_audit_trail"

    def test_swr_stage_with_timeout(self):
        """Test SWR stage with custom timeout"""
        stage = Stage.swr(
            cache_key="slow:api",
            ttl="5m",
            url="https://slow-api.example.com/data",
            method="GET",
            timeout_seconds=120,
        )

        assert stage["timeout_seconds"] == 120

    def test_swr_stage_ttl_formats(self):
        """Test SWR stage supports various TTL formats"""
        # Duration string
        stage1 = Stage.swr(
            cache_key="test", ttl="30m", url="https://example.com", method="GET"
        )
        assert stage1["ttl"] == "30m"

        # Integer seconds
        stage2 = Stage.swr(
            cache_key="test", ttl=1800, url="https://example.com", method="GET"
        )
        assert stage2["ttl"] == 1800

    def test_swr_stage_optional_fields_omitted(self):
        """Test that optional fields are not included when None"""
        stage = Stage.swr(
            cache_key="minimal", ttl="15m", url="https://example.com", method="GET"
        )

        # Optional fields should not be present when None
        assert "headers" not in stage
        assert "body" not in stage
        assert "timeout_seconds" not in stage
        assert "output_field" not in stage
        assert "collection" not in stage
