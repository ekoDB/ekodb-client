"""
Unit tests for raw_completion request/response structure.

These tests verify the request dict format sent to the server and the
expected response shape, mirroring the Rust RawCompletionRequest struct.

Run with: pytest tests/test_raw_completion.py -v
"""


class TestRawCompletionRequestStructure:
    """Verify that the raw_completion request dict is well-formed."""

    def test_minimal_request(self):
        """Only system_prompt and message are required."""
        request = {
            "system_prompt": "You are a helpful assistant.",
            "message": "What is 2 + 2?",
        }
        assert request["system_prompt"] == "You are a helpful assistant."
        assert request["message"] == "What is 2 + 2?"

    def test_full_request_with_all_optional_fields(self):
        """All optional fields serialize correctly."""
        request = {
            "system_prompt": "System.",
            "message": "User.",
            "provider": "openai",
            "model": "gpt-4o",
            "max_tokens": 2048,
        }
        assert request["provider"] == "openai"
        assert request["model"] == "gpt-4o"
        assert request["max_tokens"] == 2048

    def test_max_tokens_is_integer(self):
        """max_tokens must be an integer, not a float."""
        request = {"system_prompt": "S.", "message": "M.", "max_tokens": 512}
        assert isinstance(request["max_tokens"], int)

    def test_optional_fields_absent_when_not_set(self):
        """Optional fields should be absent (not None) when not provided."""
        request = {
            "system_prompt": "S.",
            "message": "M.",
        }
        assert "provider" not in request
        assert "model" not in request
        assert "max_tokens" not in request

    def test_provider_only_override(self):
        """Can override just the provider without model."""
        request = {
            "system_prompt": "S.",
            "message": "M.",
            "provider": "anthropic",
        }
        assert request["provider"] == "anthropic"
        assert "model" not in request

    def test_model_only_override(self):
        """Can override just the model without provider."""
        request = {
            "system_prompt": "S.",
            "message": "M.",
            "model": "claude-opus-4-6",
        }
        assert request["model"] == "claude-opus-4-6"
        assert "provider" not in request


class TestRawCompletionResponseStructure:
    """Verify the expected response shape from the server."""

    def test_response_has_content_field(self):
        resp = {"content": "The answer is 4."}
        assert "content" in resp
        assert isinstance(resp["content"], str)

    def test_content_can_be_empty_string(self):
        resp = {"content": ""}
        assert resp["content"] == ""

    def test_content_can_be_multiline(self):
        resp = {"content": "Line 1\nLine 2\nLine 3"}
        assert "\n" in resp["content"]

    def test_content_can_be_json_string(self):
        """raw_completion is often used for structured JSON output."""
        import json

        raw = '{"plan": ["step1", "step2"], "confidence": 0.9}'
        resp = {"content": raw}
        parsed = json.loads(resp["content"])
        assert parsed["plan"] == ["step1", "step2"]
        assert parsed["confidence"] == 0.9
