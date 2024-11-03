import pytest
from ekodb import EkoDBClient, EkoDBError, TypedValue, ValueType

@pytest.mark.asyncio
async def test_client_initialization(client):
    assert client.base_url == "http://localhost:8080"
    assert client.api_key == "test-api-key"
    assert client.token is None

@pytest.mark.asyncio
async def test_authentication_success(requests_mock, client):
    requests_mock.post(
        "http://localhost:8080/api/auth/token",
        json={"token": "test-token"}
    )
    
    await client.authenticate()
    assert client.token == "test-token"
    assert client.session.headers["Authorization"] == "Bearer test-token"

@pytest.mark.asyncio
async def test_authentication_failure(requests_mock, client):
    error_response = {
        "error": "Unauthorized",
        "message": "Invalid API key",
        "status": 401
    }
    
    requests_mock.post(
        "http://localhost:8080/api/auth/token",
        status_code=401,
        json=error_response
    )
    
    with pytest.raises(EkoDBError) as exc_info:
        await client.authenticate()
    assert exc_info.value.status_code == 401
    assert exc_info.value.response == error_response

@pytest.mark.asyncio
async def test_crud_operations(requests_mock, client):
    # Mock authentication
    requests_mock.post(
        "http://localhost:8080/api/auth/token",
        json={"token": "test-token"}
    )
    
    record = {
        "name": TypedValue(type=ValueType.STRING, value="John"),
        "age": TypedValue(type=ValueType.INTEGER, value=30)
    }
    
    # Test insert
    requests_mock.post(
        "http://localhost:8080/api/insert/users",
        json={"id": "test-id"}
    )
    
    result = await client.insert("users", record)
    assert result == "test-id"
    
    # Test find
    requests_mock.get(
        "http://localhost:8080/api/find/users",
        json=[{
            "name": {"type": "String", "value": "John"},
            "age": {"type": "Integer", "value": 30}
        }]
    )
    
    results = await client.find("users")
    assert len(results) == 1
    assert results[0]["name"].value == "John"
    assert results[0]["name"].type == ValueType.STRING
    assert results[0]["age"].value == 30
    assert results[0]["age"].type == ValueType.INTEGER
    
    # Test update
    requests_mock.put(
        "http://localhost:8080/api/update/users/test-id",
        json={"success": True}
    )
    
    update_result = await client.update("users", "test-id", {
        "age": TypedValue(type=ValueType.INTEGER, value=31)
    })
    assert update_result is True
    
    # Test delete
    requests_mock.delete(
        "http://localhost:8080/api/delete/users/test-id",
        json={"success": True}
    )
    
    delete_result = await client.delete("users", "test-id")
    assert delete_result is True

@pytest.mark.asyncio
async def test_batch_operations(requests_mock, client):
    # Mock authentication
    requests_mock.post(
        "http://localhost:8080/api/auth/token",
        json={"token": "test-token"}
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
    
    # Test batch insert
    requests_mock.post(
        "http://localhost:8080/api/batch/insert/users",
        json={"insertedIds": ["id-1", "id-2"], "success": True}
    )
    
    result = await client.batch_insert("users", records)
    assert result.inserted_ids == ["id-1", "id-2"]
    assert result.success is True
    
    # Test batch update
    updates = {
        "id-1": {"age": TypedValue(type=ValueType.INTEGER, value=31)},
        "id-2": {"age": TypedValue(type=ValueType.INTEGER, value=26)}
    }
    
    requests_mock.put(
        "http://localhost:8080/api/batch/update/users",
        json={"updatedCount": 2, "success": True}
    )
    
    update_result = await client.batch_update("users", updates)
    assert update_result.updated_count == 2
    assert update_result.success is True
    
    # Test batch delete
    requests_mock.delete(
        "http://localhost:8080/api/batch/delete/users",
        json={"deletedCount": 2, "success": True}
    )
    
    delete_result = await client.batch_delete("users", ["id-1", "id-2"])
    assert delete_result.deleted_count == 2
    assert delete_result.success is True

@pytest.mark.asyncio
async def test_find_by_id(requests_mock, client):
    # Mock authentication
    requests_mock.post(
        "http://localhost:8080/api/auth/token",
        json={"token": "test-token"}
    )
    
    # Test successful find
    requests_mock.get(
        "http://localhost:8080/api/find/users/test-id",
        json={
            "name": {"type": "String", "value": "John"},
            "age": {"type": "Integer", "value": 30}
        }
    )
    
    result = await client.find_by_id("users", "test-id")
    assert result is not None
    assert result["name"].value == "John"
    assert result["age"].value == 30
    
    # Test not found
    requests_mock.get(
        "http://localhost:8080/api/find/users/non-existent",
        status_code=404
    )
    
    result = await client.find_by_id("users", "non-existent")
    assert result is None

@pytest.mark.asyncio
async def test_get_schema(requests_mock, client):
    # Mock authentication
    requests_mock.post(
        "http://localhost:8080/api/auth/token",
        json={"token": "test-token"}
    )
    
    schema = {
        "fields": {
            "name": {"type": "String", "required": True},
            "age": {"type": "Integer", "required": False}
        }
    }
    
    requests_mock.get(
        "http://localhost:8080/api/schemas/users",
        json=schema
    )
    
    result = await client.get_schema("users")
    assert result.fields["name"].type == ValueType.STRING
    assert result.fields["name"].required is True
    assert result.fields["age"].type == ValueType.INTEGER
    assert result.fields["age"].required is False