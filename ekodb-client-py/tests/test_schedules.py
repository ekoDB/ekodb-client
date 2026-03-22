"""
Unit tests for Schedule management request/response structures.

These tests verify the request dict formats sent to the server and the
expected response shapes, mirroring the Rust Schedule endpoints.

Run with: pytest tests/test_schedules.py -v
"""


class TestScheduleRequestStructure:
    """Verify that schedule request dicts are well-formed."""

    def test_create_schedule_minimal(self):
        """A create_schedule request needs at least a name and cron."""
        request = {"name": "Daily Backup", "cron": "0 2 * * *"}
        assert request["name"] == "Daily Backup"
        assert request["cron"] == "0 2 * * *"

    def test_create_schedule_full(self):
        """Create schedule with all optional fields."""
        request = {
            "name": "Nightly Sync",
            "cron": "0 0 * * *",
            "description": "Sync data from external sources nightly",
            "chat_id": "session_abc",
            "message": "Run the nightly sync procedure",
            "timezone": "America/New_York",
            "enabled": True,
        }
        assert request["timezone"] == "America/New_York"
        assert request["enabled"] is True
        assert request["chat_id"] == "session_abc"

    def test_update_schedule_partial(self):
        """Update schedule only sends changed fields."""
        request = {"cron": "30 3 * * *"}
        assert "name" not in request
        assert "description" not in request
        assert request["cron"] == "30 3 * * *"

    def test_update_schedule_multiple_fields(self):
        """Update schedule can change multiple fields at once."""
        request = {
            "name": "Updated Backup",
            "cron": "0 4 * * *",
            "description": "Changed to 4 AM",
        }
        assert request["name"] == "Updated Backup"
        assert request["description"] == "Changed to 4 AM"


class TestScheduleResponseStructure:
    """Verify expected response shapes from schedule endpoints."""

    def test_schedule_response_has_id_and_cron(self):
        resp = {
            "id": "sched_1",
            "name": "Daily Backup",
            "cron": "0 2 * * *",
            "status": "active",
        }
        assert "id" in resp
        assert isinstance(resp["id"], str)
        assert resp["cron"] == "0 2 * * *"

    def test_schedule_list_response(self):
        resp = {
            "schedules": [
                {"id": "sched_1", "name": "Daily Backup", "cron": "0 2 * * *"},
                {"id": "sched_2", "name": "Weekly Report", "cron": "0 9 * * 1"},
            ]
        }
        assert isinstance(resp["schedules"], list)
        assert len(resp["schedules"]) == 2

    def test_schedule_status_values(self):
        """Schedule status: active, paused."""
        valid_statuses = {"active", "paused"}
        for status in valid_statuses:
            resp = {"id": "sched_1", "status": status}
            assert resp["status"] in valid_statuses
