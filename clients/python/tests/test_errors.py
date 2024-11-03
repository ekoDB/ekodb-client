import pytest
from ekodb import EkoDBError

def test_error_initialization():
    error = EkoDBError(
        message="Test error",
        status_code=400,
        response={"error": "Bad Request"}
    )
    
    assert error.message == "Test error"
    assert error.status_code == 400
    assert error.response == {"error": "Bad Request"}
    assert str(error) == "Test error (Status: 400)"

def test_error_without_status():
    error = EkoDBError("Test error")
    assert error.status_code is None
    assert error.response is None
    assert str(error) == "Test error"

def test_error_with_response_status():
    error = EkoDBError(
        message="Test error",
        response={"status": 404, "error": "Not Found"}
    )
    assert error.status_code == 404
    assert error.response == {"status": 404, "error": "Not Found"}

def test_error_priority_status():
    # Status code should take priority over response status
    error = EkoDBError(
        message="Test error",
        status_code=400,
        response={"status": 404}
    )
    assert error.status_code == 400

def test_error_with_non_dict_response():
    error = EkoDBError(
        message="Test error",
        response="Invalid response"
    )
    assert error.status_code is None
    assert error.response == "Invalid response"

def test_error_inheritance():
    error = EkoDBError("Test error")
    assert isinstance(error, Exception)
    assert error.name == "EkoDBError"