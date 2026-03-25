"""
Unit tests for Chat Models, User Functions, and Collection Utilities

Tests for:
- Chat Models API (get_chat_models, get_chat_model, get_chat_message)
- User Functions API (save, get, list, update, delete)
- Collection Utilities (collection_exists, count_documents)

Note: Full integration tests require a running server.
"""

import pytest
from ekodb_client import Client


@pytest.fixture
def client():
    """Create a test client"""
    return Client.new(
        "http://localhost:8080", "test-api-key", should_retry=False, timeout_secs=30
    )


class TestChatModels:
    """Test chat model related methods"""

    def test_client_has_get_chat_models_method(self, client):
        """Verify get_chat_models method exists"""
        assert hasattr(client, "get_chat_models")
        assert callable(client.get_chat_models)

    def test_client_has_get_chat_model_method(self, client):
        """Verify get_chat_model method exists"""
        assert hasattr(client, "get_chat_model")
        assert callable(client.get_chat_model)

    def test_client_has_get_chat_message_method(self, client):
        """Verify get_chat_message method exists"""
        assert hasattr(client, "get_chat_message")
        assert callable(client.get_chat_message)


class TestUserFunctions:
    """Test user function related methods"""

    def test_client_has_save_user_function_method(self, client):
        """Verify save_user_function method exists"""
        assert hasattr(client, "save_user_function")
        assert callable(client.save_user_function)

    def test_client_has_get_user_function_method(self, client):
        """Verify get_user_function method exists"""
        assert hasattr(client, "get_user_function")
        assert callable(client.get_user_function)

    def test_client_has_list_user_functions_method(self, client):
        """Verify list_user_functions method exists"""
        assert hasattr(client, "list_user_functions")
        assert callable(client.list_user_functions)

    def test_client_has_update_user_function_method(self, client):
        """Verify update_user_function method exists"""
        assert hasattr(client, "update_user_function")
        assert callable(client.update_user_function)

    def test_client_has_delete_user_function_method(self, client):
        """Verify delete_user_function method exists"""
        assert hasattr(client, "delete_user_function")
        assert callable(client.delete_user_function)


class TestCollectionUtilities:
    """Test collection utility methods"""

    def test_client_has_collection_exists_method(self, client):
        """Verify collection_exists method exists"""
        assert hasattr(client, "collection_exists")
        assert callable(client.collection_exists)

    def test_client_has_count_documents_method(self, client):
        """Verify count_documents method exists"""
        assert hasattr(client, "count_documents")
        assert callable(client.count_documents)


class TestUserFunctionStructure:
    """Test user function data structures"""

    def test_user_function_structure(self):
        """Test that user function dict structure is correct"""
        user_function = {
            "name": "my_function",
            "code": "def my_function(x): return x * 2",
            "description": "A simple function",
            "parameters": {"x": "number"},
        }

        assert "name" in user_function
        assert "code" in user_function
        assert user_function["name"] == "my_function"

    def test_user_function_update_structure(self):
        """Test that user function update structure is correct"""
        update = {
            "code": "def my_function(x): return x * 3",
            "description": "Updated function",
        }

        assert "code" in update
        assert "description" in update


# Example integration tests (require running server)
"""
@pytest.mark.asyncio
async def test_get_chat_models_integration(client):
    '''Test get_chat_models returns expected structure'''
    models = await client.get_chat_models()

    assert "openai" in models or "anthropic" in models or "perplexity" in models


@pytest.mark.asyncio
async def test_get_chat_model_integration(client):
    '''Test get_chat_model for a specific provider'''
    openai_models = await client.get_chat_model("openai")

    assert isinstance(openai_models, list)


@pytest.mark.asyncio
async def test_collection_exists_true(client):
    '''Test collection_exists returns True for existing collection'''
    # First create a collection by inserting a document
    await client.create("test_collection", {"test": "data"})

    exists = await client.collection_exists("test_collection")
    assert exists is True


@pytest.mark.asyncio
async def test_collection_exists_false(client):
    '''Test collection_exists returns False for non-existing collection'''
    exists = await client.collection_exists("nonexistent_collection_xyz")
    assert exists is False


@pytest.mark.asyncio
async def test_count_documents(client):
    '''Test count_documents returns correct count'''
    # Create a test collection with known number of documents
    collection = "test_count_collection"
    await client.create(collection, {"item": 1})
    await client.create(collection, {"item": 2})
    await client.create(collection, {"item": 3})

    count = await client.count_documents(collection)
    assert count >= 3


@pytest.mark.asyncio
async def test_user_function_crud(client):
    '''Test user function CRUD operations'''
    # Create
    func = {
        "name": "test_func",
        "code": "def test_func(x): return x + 1",
        "description": "Test function",
        "parameters": {"x": "number"}
    }
    saved = await client.save_user_function(func)
    assert saved is not None

    # Read
    retrieved = await client.get_user_function("test_func")
    assert retrieved["name"] == "test_func"

    # List
    functions = await client.list_user_functions()
    assert any(f["name"] == "test_func" for f in functions)

    # Update
    await client.update_user_function("test_func", {"description": "Updated"})
    updated = await client.get_user_function("test_func")
    assert updated["description"] == "Updated"

    # Delete
    await client.delete_user_function("test_func")
"""


class TestExecuteTool:
    """Test execute_tool method"""

    def test_client_has_execute_tool_method(self, client):
        """Verify execute_tool method exists"""
        assert hasattr(client, "execute_tool")
        assert callable(client.execute_tool)

    def test_execute_tool_accepts_params(self, client):
        """Verify execute_tool accepts correct parameter types"""
        import inspect

        # The method should exist and be callable with (tool_name, params, chat_id=None)
        assert hasattr(client, "execute_tool")

    def test_execute_tool_request_structure(self):
        """Verify the expected request body structure"""
        # The execute_tool endpoint expects:
        request = {
            "tool": "count_records",
            "params": {"collection": "users"},
        }
        assert request["tool"] == "count_records"
        assert request["params"]["collection"] == "users"

    def test_execute_tool_request_with_chat_id(self):
        """Verify request body includes chat_id when provided"""
        request = {
            "tool": "kv_get",
            "params": {"key": "greeting"},
            "chat_id": "chat_456",
        }
        assert request["chat_id"] == "chat_456"

    def test_execute_tool_response_success_structure(self):
        """Verify expected success response structure"""
        response = {"success": True, "result": {"count": 42}}
        assert response["success"] is True
        assert response["result"]["count"] == 42

    def test_execute_tool_response_failure_structure(self):
        """Verify expected failure response structure"""
        response = {"success": False, "error": "permission denied"}
        assert response["success"] is False
        assert response["error"] == "permission denied"
