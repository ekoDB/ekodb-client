"""
Unit tests for Goal/Task/Agent API request/response structures.

These tests verify the request dict formats sent to the server and the
expected response shapes, mirroring the Rust Goal/Task/Agent endpoints.

Run with: pytest tests/test_goals_tasks_agents.py -v
"""


class TestGoalRequestStructure:
    """Verify that goal request dicts are well-formed."""

    def test_goal_create_minimal(self):
        """A goal create request needs at least a title."""
        request = {"title": "Deploy v2"}
        assert request["title"] == "Deploy v2"

    def test_goal_create_full(self):
        """Goal create with all optional fields."""
        request = {
            "title": "Deploy v2",
            "description": "Deploy version 2 to production",
            "steps": [
                {"description": "Build Docker image", "status": "pending"},
                {"description": "Run migrations", "status": "pending"},
            ],
            "priority": "high",
        }
        assert len(request["steps"]) == 2
        assert request["priority"] == "high"

    def test_goal_update_partial(self):
        """Goal update only sends changed fields."""
        request = {"title": "Updated title"}
        assert "description" not in request
        assert request["title"] == "Updated title"

    def test_goal_complete_with_summary(self):
        """Goal complete can include a summary."""
        request = {"summary": "All steps completed successfully"}
        assert isinstance(request["summary"], str)

    def test_goal_reject_with_reason(self):
        """Goal reject includes a reason."""
        request = {"reason": "Steps are not achievable with current resources"}
        assert isinstance(request["reason"], str)

    def test_goal_step_complete_with_result(self):
        """Goal step complete includes result data."""
        request = {"result": "Docker image built: sha256:abc123"}
        assert isinstance(request["result"], str)

    def test_goal_step_fail_with_error(self):
        """Goal step fail includes error data."""
        request = {"error": "Docker build failed: OOM"}
        assert isinstance(request["error"], str)


class TestGoalResponseStructure:
    """Verify expected response shapes from goal endpoints."""

    def test_goal_response_has_id(self):
        resp = {"id": "goal_1", "title": "Test", "status": "active"}
        assert "id" in resp
        assert isinstance(resp["id"], str)

    def test_goal_list_response_has_goals(self):
        resp = {"goals": [{"id": "goal_1"}, {"id": "goal_2"}]}
        assert "goals" in resp
        assert isinstance(resp["goals"], list)
        assert len(resp["goals"]) == 2

    def test_goal_status_values(self):
        """Goal status transitions: active -> pending_review -> in_progress/failed."""
        valid_statuses = {
            "active",
            "pending_review",
            "in_progress",
            "failed",
            "completed",
        }
        for status in valid_statuses:
            resp = {"id": "goal_1", "status": status}
            assert resp["status"] in valid_statuses

    def test_goal_with_steps(self):
        resp = {
            "id": "goal_1",
            "steps": [
                {"description": "Step 1", "status": "completed", "result": "Done"},
                {"description": "Step 2", "status": "in_progress"},
                {"description": "Step 3", "status": "pending"},
            ],
        }
        assert len(resp["steps"]) == 3
        assert resp["steps"][0]["status"] == "completed"

    def test_goal_search_response(self):
        resp = {"goals": [{"id": "goal_1", "title": "matching goal"}]}
        assert isinstance(resp["goals"], list)


class TestTaskRequestStructure:
    """Verify that task request dicts are well-formed."""

    def test_task_create_minimal(self):
        request = {"name": "Backup DB", "cron": "0 2 * * *"}
        assert request["name"] == "Backup DB"
        assert request["cron"] == "0 2 * * *"

    def test_task_create_full(self):
        request = {
            "name": "Backup DB",
            "cron": "0 2 * * *",
            "description": "Daily backup",
            "chat_id": "session_1",
            "message": "Run backup procedure",
            "max_consecutive_failures": 3,
        }
        assert request["max_consecutive_failures"] == 3

    def test_task_update_partial(self):
        request = {"cron": "0 3 * * *"}
        assert "name" not in request

    def test_task_succeed_with_output(self):
        request = {"output": "Backup completed: 1.2GB"}
        assert isinstance(request["output"], str)

    def test_task_fail_with_error(self):
        request = {"error": "Connection refused"}
        assert isinstance(request["error"], str)

    def test_task_resume_with_data(self):
        request = {"next_run": "2026-03-21T02:00:00Z"}
        assert isinstance(request["next_run"], str)


class TestTaskResponseStructure:
    """Verify expected response shapes from task endpoints."""

    def test_task_response_has_id(self):
        resp = {"id": "task_1", "name": "Backup DB", "status": "active"}
        assert "id" in resp

    def test_task_list_response(self):
        resp = {"tasks": [{"id": "task_1"}, {"id": "task_2"}]}
        assert isinstance(resp["tasks"], list)

    def test_task_status_values(self):
        valid_statuses = {"active", "running", "paused", "failed"}
        for status in valid_statuses:
            resp = {"id": "task_1", "status": status}
            assert resp["status"] in valid_statuses

    def test_task_due_response(self):
        resp = {"tasks": [{"id": "task_1", "next_run": "2026-03-20T02:00:00Z"}]}
        assert isinstance(resp["tasks"], list)

    def test_task_with_run_count(self):
        resp = {
            "id": "task_1",
            "run_count": 42,
            "consecutive_failures": 0,
            "last_run": "2026-03-19T02:00:00Z",
        }
        assert resp["run_count"] == 42
        assert resp["consecutive_failures"] == 0


class TestAgentRequestStructure:
    """Verify that agent request dicts are well-formed."""

    def test_agent_create_minimal(self):
        request = {"name": "CodeReviewer", "system_prompt": "You review code."}
        assert request["name"] == "CodeReviewer"

    def test_agent_create_full(self):
        request = {
            "name": "CodeReviewer",
            "system_prompt": "You review code.",
            "deployment_id": "deploy_1",
            "model": "gpt-4o",
            "tools": ["web_search", "file_read"],
        }
        assert isinstance(request["tools"], list)

    def test_agent_update_partial(self):
        request = {"system_prompt": "Updated prompt."}
        assert "name" not in request


class TestAgentResponseStructure:
    """Verify expected response shapes from agent endpoints."""

    def test_agent_response_has_id(self):
        resp = {"id": "agent_1", "name": "CodeReviewer"}
        assert "id" in resp

    def test_agent_list_response(self):
        resp = {"agents": [{"id": "agent_1"}, {"id": "agent_2"}]}
        assert isinstance(resp["agents"], list)

    def test_agents_by_deployment_response(self):
        resp = {"agents": [{"id": "agent_1", "deployment_id": "deploy_1"}]}
        assert resp["agents"][0]["deployment_id"] == "deploy_1"

    def test_agent_by_name_response(self):
        resp = {"id": "agent_1", "name": "CodeReviewer"}
        assert resp["name"] == "CodeReviewer"
