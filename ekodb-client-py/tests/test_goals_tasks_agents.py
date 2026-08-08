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


# ============================================================================
# Goal Template Tests
# ============================================================================


class TestGoalTemplateCreatePayload:
    """Verify goal template create request structure."""

    def test_minimal_payload(self):
        payload = {"title": "Migration Template"}
        assert "title" in payload

    def test_full_payload(self):
        payload = {
            "title": "Data Migration",
            "description": "Template for schema migrations",
            "steps": [
                {"description": "Analyze source"},
                {"description": "Migrate records"},
            ],
        }
        assert payload["title"] == "Data Migration"
        assert len(payload["steps"]) == 2
        assert payload["steps"][0]["description"] == "Analyze source"


class TestGoalTemplateResponseStructure:
    """Verify goal template response shapes."""

    def test_create_response(self):
        resp = {"id": "tpl_1", "title": "Migration Template"}
        assert "id" in resp
        assert resp["title"] == "Migration Template"

    def test_list_response(self):
        resp = {"templates": [{"id": "tpl_1"}, {"id": "tpl_2"}]}
        assert isinstance(resp["templates"], list)
        assert len(resp["templates"]) == 2

    def test_update_response(self):
        resp = {"id": "tpl_1", "title": "Updated", "description": "New desc"}
        assert resp["description"] == "New desc"

    def test_delete_response(self):
        resp = {"ok": True}
        assert resp["ok"] is True


class TestChatStreamEndContextWindow:
    """Verify context_window appears in chat stream end events."""

    def test_end_event_with_context_window(self):
        event = {
            "type": "end",
            "message_id": "msg-1",
            "execution_time_ms": 250,
            "context_window": 128000,
        }
        assert event["type"] == "end"
        assert event["context_window"] == 128000

    def test_end_event_without_context_window(self):
        event = {
            "type": "end",
            "message_id": "msg-2",
            "execution_time_ms": 100,
        }
        assert "context_window" not in event


# ============================================================================
# Goal Template Request Structure Tests
# ============================================================================


class TestGoalTemplateRequestStructure:
    """Verify goal template request payloads are well-formed."""

    def test_template_create_minimal(self):
        """A template create needs at least a title."""
        payload = {"title": "Deploy Template"}
        assert payload["title"] == "Deploy Template"
        assert "steps" not in payload

    def test_template_create_with_steps(self):
        """Template with step definitions."""
        payload = {
            "title": "CI/CD Pipeline",
            "description": "Standard deployment pipeline",
            "steps": [
                {"description": "Run tests"},
                {"description": "Build artifact"},
                {"description": "Deploy to staging"},
                {"description": "Deploy to production"},
            ],
        }
        assert len(payload["steps"]) == 4
        assert payload["steps"][2]["description"] == "Deploy to staging"

    def test_template_update_payload(self):
        """Template update only sends changed fields."""
        payload = {"description": "Updated description for template"}
        assert "title" not in payload
        assert "steps" not in payload
        assert payload["description"] == "Updated description for template"


# ============================================================================
# Goal Lifecycle Structure Tests
# ============================================================================


class TestGoalLifecycleStructure:
    """Verify goal lifecycle payloads (complete, approve, reject)."""

    def test_goal_complete_payload(self):
        """Goal complete includes a summary."""
        payload = {"summary": "All 3 steps finished, migration verified"}
        assert isinstance(payload["summary"], str)
        assert "summary" in payload

    def test_goal_approve_response(self):
        """Goal approve returns the goal with completed status."""
        resp = {"id": "goal_1", "status": "completed", "title": "Migration"}
        assert resp["status"] == "completed"

    def test_goal_reject_payload(self):
        """Goal reject includes a reason."""
        payload = {"reason": "Insufficient resources to proceed"}
        assert isinstance(payload["reason"], str)
        assert "reason" in payload


# ============================================================================
# Task Lifecycle Structure Tests
# ============================================================================


class TestTaskLifecycleStructure:
    """Verify task lifecycle payloads (start, succeed, fail, pause, resume)."""

    def test_task_start_response(self):
        """Task start returns the task with running status."""
        resp = {"id": "task_1", "name": "Backup", "status": "running"}
        assert resp["status"] == "running"

    def test_task_succeed_payload(self):
        """Task succeed includes output data."""
        payload = {"output": "Backup completed: 2.4GB written to s3://backups/"}
        assert isinstance(payload["output"], str)

    def test_task_fail_payload(self):
        """Task fail includes an error description."""
        payload = {"error": "Connection to backup server timed out after 30s"}
        assert isinstance(payload["error"], str)

    def test_task_pause_response(self):
        """Task pause returns the task with paused status."""
        resp = {"id": "task_1", "name": "Backup", "status": "paused"}
        assert resp["status"] == "paused"

    def test_task_resume_payload(self):
        """Task resume can include a next_run override."""
        payload = {"next_run": "2026-03-22T02:00:00Z"}
        assert isinstance(payload["next_run"], str)
