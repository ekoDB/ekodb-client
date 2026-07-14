"""Health-contract parity for the Python client (mirrors ekodb-client-go)."""

import ekodb_client


def test_client_exposes_health_status_and_check():
    assert hasattr(ekodb_client.Client, "health_status")
    assert hasattr(ekodb_client.Client, "health")


def test_parse_health_status_degraded_is_reachable():
    hs = ekodb_client.parse_health_status({"status": "degraded", "integrity_ok": False})
    assert hs.reachable is True
    assert hs.status == "degraded"
    assert hs.integrity_ok is False


def test_parse_health_status_reads_admin_nested_integrity():
    hs = ekodb_client.parse_health_status(
        {"status": "ok", "integrity": {"healthy": True, "manifest_load_failed": []}}
    )
    assert hs.status == "ok"
    assert hs.integrity_ok is True


def test_parse_health_status_missing_status_fails_safe_to_degraded():
    hs = ekodb_client.parse_health_status({"integrity_ok": True})
    assert hs.reachable is True
    assert hs.status == "degraded"


def test_parse_health_status_preserves_raw_status():
    # Matches the Go client: a non-empty string status is kept verbatim.
    hs = ekodb_client.parse_health_status({"status": "healthy"})
    assert hs.status == "healthy"


def test_to_dict_excludes_detail_but_detail_is_accessible():
    hs = ekodb_client.parse_health_status(
        {
            "status": "degraded",
            "integrity": {"manifest_load_failed": ["secret_collection"]},
        }
    )
    summary = hs.to_dict()
    assert summary == {"reachable": True, "status": "degraded", "integrity_ok": False}
    assert "detail" not in summary
    # detail is still readable in-process (the full body)
    assert hs.detail is not None
    assert "integrity" in hs.detail
