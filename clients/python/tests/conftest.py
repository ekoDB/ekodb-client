import pytest
from ekodb import EkoDBClient

@pytest.fixture
def client():
    """
    Create a test client with default configuration.
    """
    return EkoDBClient({
        "base_url": "http://localhost:8080",
        "api_key": "test-api-key"
    })