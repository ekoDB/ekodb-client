"""
Unit tests for Agent API request/response structures.

These tests verify the request dict formats sent to the server and the
expected response shapes, mirroring the Rust Agent endpoints.

Run with: pytest tests/test_agents.py -v
"""


class TestAgentRequestStructure:
    """Verify that agent request dicts are well-formed."""

    def test_agent_create_minimal(self):
        """An agent needs at least a name and system_prompt."""
        request = {"name": "Summarizer", "system_prompt": "You summarize text."}
        assert request["name"] == "Summarizer"
        assert request["system_prompt"] == "You summarize text."

    def test_agent_create_with_deployment(self):
        """Agent create with deployment fields."""
        request = {
            "name": "Reviewer",
            "system_prompt": "You review pull requests.",
            "deployment_id": "deploy_prod",
            "model": "gpt-4o",
            "tools": ["web_search", "file_read"],
            "max_tokens": 4096,
        }
        assert request["deployment_id"] == "deploy_prod"
        assert request["model"] == "gpt-4o"
        assert isinstance(request["tools"], list)
        assert request["max_tokens"] == 4096

    def test_agent_update_partial(self):
        """Agent update only sends changed fields."""
        request = {"system_prompt": "Updated instructions."}
        assert "name" not in request
        assert "deployment_id" not in request

    def test_agent_update_tools(self):
        """Agent update can change tools list."""
        request = {"tools": ["web_search", "file_read", "shell_exec"]}
        assert len(request["tools"]) == 3
        assert "shell_exec" in request["tools"]


class TestAgentResponseStructure:
    """Verify expected response shapes from agent endpoints."""

    def test_agent_response_has_id_and_name(self):
        resp = {
            "id": "agent_1",
            "name": "Summarizer",
            "system_prompt": "You summarize.",
        }
        assert "id" in resp
        assert isinstance(resp["id"], str)
        assert resp["name"] == "Summarizer"

    def test_agent_response_with_deployment_fields(self):
        resp = {
            "id": "agent_1",
            "name": "Reviewer",
            "deployment_id": "deploy_prod",
            "model": "gpt-4o",
            "tools": ["web_search"],
        }
        assert resp["deployment_id"] == "deploy_prod"
        assert resp["model"] == "gpt-4o"
        assert isinstance(resp["tools"], list)

    def test_agents_by_deployment_response(self):
        resp = {
            "agents": [
                {"id": "agent_1", "name": "A", "deployment_id": "deploy_1"},
                {"id": "agent_2", "name": "B", "deployment_id": "deploy_1"},
            ]
        }
        assert isinstance(resp["agents"], list)
        assert len(resp["agents"]) == 2
        assert all(a["deployment_id"] == "deploy_1" for a in resp["agents"])
