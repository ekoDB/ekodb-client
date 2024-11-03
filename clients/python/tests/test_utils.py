import pytest
from ekodb.utils import (
    create_typed_value,
    create_typed_record,
    extract_value,
    extract_record_values,
    validate_collection_name,
    validate_record_id
)
from ekodb.types import ValueType, TypedValue

def test_create_typed_value():
    assert create_typed_value("test").type == ValueType.STRING
    assert create_typed_value(42).type == ValueType.INTEGER
    assert create_typed_value(3.14).type == ValueType.FLOAT
    assert create_typed_value(True).type == ValueType.BOOLEAN
    
    with pytest.raises(ValueError):
        create_typed_value(None)

def test_create_typed_record():
    data = {
        "name": "John",
        "age": 30,
        "active": True,
        "score": 98.6
    }
    record = create_typed_record(data)
    
    assert record["name"].type == ValueType.STRING
    assert record["age"].type == ValueType.INTEGER
    assert record["active"].type == ValueType.BOOLEAN
    assert record["score"].type == ValueType.FLOAT

def test_extract_value():
    typed_value = TypedValue(type=ValueType.STRING, value="test")
    assert extract_value(typed_value) == "test"

def test_extract_record_values():
    record = {
        "name": TypedValue(type=ValueType.STRING, value="John"),
        "age": TypedValue(type=ValueType.INTEGER, value=30)
    }
    values = extract_record_values(record)
    
    assert values == {"name": "John", "age": 30}

def test_validate_collection_name():
    # Valid names
    validate_collection_name("users")
    validate_collection_name("test_collection")
    validate_collection_name("my-collection")
    
    # Invalid names
    with pytest.raises(ValueError):
        validate_collection_name("")
    with pytest.raises(ValueError):
        validate_collection_name("users/invalid")
    with pytest.raises(ValueError):
        validate_collection_name("a" * 65)

def test_validate_record_id():
    # Valid IDs
    validate_record_id("user-123")
    validate_record_id("test_record")
    validate_record_id("abc123")
    
    # Invalid IDs
    with pytest.raises(ValueError):
        validate_record_id("")
    with pytest.raises(ValueError):
        validate_record_id("user/123")
    with pytest.raises(ValueError):
        validate_record_id("a" * 65)

# tests/test_client.py (Updated version)
import pytest
from ekodb import EkoDBClient, EkoDBError
from ekodb.types import ValueType, TypedValue

def test_client_initialization():
    client = EkoDBClient({
        "base_url": "http://test.api",
        "api_key": "test-key"
    })
    assert client.base_url == "http://test.api"
    assert client.api_key == "test-key"
    assert client.token is None

@pytest.mark.asyncio
async def test_authentication_success(requests_mock):
    client = EkoDBClient({
        "base_url": "http://test.api",
        "api_key": "test-key"
    })
    
    requests_mock.post(
        "http://test.api/api/auth/token",
        json={"token": "test-token"}
    )
    
    await client.authenticate()
    assert client.token == "test-token"
    assert "Bearer test-token" in client.session.headers["Authorization"]

@pytest.mark.asyncio
async def test_authentication_failure(requests_mock):
    client = EkoDBClient({
        "base_url": "http://test.api",
        "api_key": "invalid-key"
    })
    
    requests_mock.post(
        "http://test.api/api/auth/token",
        status_code=401,
        json={"message": "Invalid API key"}
    )
    
    with pytest.raises(EkoDBError) as exc_info:
        await client.authenticate()
    assert exc_info.value.status_code == 401

@pytest.mark.asyncio
async def test_crud_operations(requests_mock):
    client = EkoDBClient({
        "base_url": "http://test.api",
        "api_key": "test-key"
    })
    
    # Mock authentication
    requests_mock.post(
        "http://test.api/api/auth/token",
        json={"token": "test-token"}
    )
    
    # Test insert
    requests_mock.post(
        "http://test.api/api/insert/users",
        json={"id": "test-id"}
    )
    
    record = {
        "name": TypedValue(type=ValueType.STRING, value="John"),
        "age": TypedValue(type=ValueType.INTEGER, value=30)
    }
    
    result = await client.insert("users", record)
    assert result == "test-id"
    
    # Test find (corrected serialization)
    requests_mock.get(
        "http://test.api/api/find/users",
        json=[{
            "name": {"type": "String", "value": "John"},
            "age": {"type": "Integer", "value": 30}
        }]
    )
    
    results = await client.find("users")
    assert len(results) == 1
    assert results[0]["name"].value == "John"
    assert results[0]["age"].value == 30

@pytest.mark.asyncio
async def test_batch_operations(requests_mock):
    client = EkoDBClient({
        "base_url": "http://test.api",
        "api_key": "test-key"
    })
    
    # Mock authentication
    requests_mock.post(
        "http://test.api/api/auth/token",
        json={"token": "test-token"}
    )
    
    # Test batch insert
    requests_mock.post(
        "http://test.api/api/batch/insert/users",
        json={"insertedIds": ["id-1", "id-2"], "success": True}
    )
    
    records = [
        {
            "name": TypedValue(type=ValueType.STRING, value="John"),
            "age": TypedValue(type=ValueType.INTEGER, value=30)
        },
        {
            "name": TypedValue(type=ValueType.STRING, value="Jane"),
            "age": TypedValue(type=ValueType.INTEGER, value=25)
        }
    ]
    
    result = await client.batch_insert("users", records)
    assert result.insertedIds == ["id-1", "id-2"]
    assert result.success is True