"""Public Python facade through its real PyO3/Rust HTTP boundary."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

import pytest
from ekodb_client.admin import AdminClient, MIN_SERVER_VERSION_FOR_PERMISSION_EDITS

CONTRACT = json.loads(
    (Path(__file__).parents[2] / "test-fixtures/admin-contract-0.79.2.json").read_text()
)


@pytest.fixture
def endpoint():
    state = {
        "version": "0.79.2",
        "requests": [],
        "status": 200,
        "token": "jwt-secret",
        "record": CONTRACT["permission_edits"][0]["response"],
        "legacy": False,
        "bad_json": False,
    }

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def handle_request(self):
            body = self.rfile.read(int(self.headers.get("Content-Length") or 0))
            body = json.loads(body) if body else None
            state["requests"].append(
                (self.command, self.path, body, self.headers.get("Authorization"))
            )
            status = 200
            if self.path == "/api/auth/token":
                result = {"token": state["token"]}
            elif self.path == "/api/health":
                result = {"version": state["version"]}
            elif self.command == "DELETE":
                result = None
                status = 204
            else:
                status = state["status"]
                record = state["record"]
                if state["legacy"]:
                    record = {
                        "key": "new-secret",
                        "is_admin": False,
                        "collection_permissions": {"users": ["read"]},
                        "ripple_warning": "Incomplete propagation",
                        "created_at": 1,
                        "future_policy": {"flag": True},
                    }
                result = [record] if self.path == "/api/auth/keys" else record
            encoded = (
                b"target-secret invalid JSON"
                if state["bad_json"] and self.path != "/api/auth/token"
                else json.dumps(result).encode()
            )
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header(
                "Content-Length", str(len(encoded) if status != 204 else 0)
            )
            if status == 307:
                self.send_header("Location", "/redirect-destination")
            self.end_headers()
            if status != 204:
                self.wfile.write(encoded)

        do_GET = do_POST = do_PUT = do_DELETE = handle_request

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(
        target=lambda: server.serve_forever(poll_interval=0.01), daemon=True
    )
    thread.start()
    yield AdminClient(
        f"http://127.0.0.1:{server.server_address[1]}", "admin-secret"
    ), state
    server.shutdown()
    server.server_close()
    thread.join()


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "case", CONTRACT["permission_edits"], ids=lambda case: case["name"]
)
async def test_recorded_permission_edits(endpoint, case):
    client, state = endpoint
    state["record"] = case["response"]
    for _ in range(2):
        result = await client.update_api_key_collection_permissions(case["request"])
        assert result["collection_permissions"]["users"] == case["fresh_token_grant"]
    assert [request[2] for request in state["requests"] if request[0] == "PUT"] == [
        case["request"],
        case["request"],
    ]
    assert sum(request[1] == "/api/health" for request in state["requests"]) == 1
    assert all(
        request[3] == "Bearer jwt-secret"
        for request in state["requests"]
        if request[1] != "/api/auth/token"
    )


@pytest.mark.asyncio
async def test_operations_nonadmin_default_legacy_metadata_and_encoding(endpoint):
    client, state = endpoint
    state["legacy"] = True
    key = await client.create_api_key()
    assert key["is_admin"] is False
    assert key["collection_permissions"]["users"]["read"] is True
    assert key["collection_permissions"]["users"]["write"] is False
    assert key["ripple_warning"] == "Incomplete propagation"
    assert key["future_policy"] == {"flag": True}
    assert key["created_at"] == 1
    assert len(await client.list_api_keys()) == 1
    assert (await client.get_permissions())["is_admin"] is False
    await client.revoke_api_key("secret/ ?#%+&=雪")
    create = next(
        request for request in state["requests"] if request[1] == "/api/auth/register"
    )
    assert create[2] == {
        "api_key": "admin-secret",
        "is_admin": False,
        "collection_permissions": {},
    }
    assert state["requests"][-1][:2] == (
        "DELETE",
        "/api/auth/keys/secret%2F%20%3F%23%25%2B%26%3D%E9%9B%AA",
    )
    assert not hasattr(client, "insert")


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "version",
    [
        None,
        9,
        "0.79.0",
        "0.79.1",
        "0.79.2-rc.1",
        "09.79.2",
        "0.79.2+",
        "0.79.2",
        "0.79.3",
        "1.0.0",
    ],
)
async def test_version_floor(endpoint, version):
    client, state = endpoint
    assert MIN_SERVER_VERSION_FOR_PERMISSION_EDITS == CONTRACT["minimum_server_version"]
    state["version"] = version
    request = {
        "api_key": "target-secret",
        "collection": "users",
        "read": True,
        "write": False,
    }
    for _ in range(2):
        if version in ("0.79.2", "0.79.3", "1.0.0"):
            await client.update_api_key_collection_permissions(request)
        else:
            with pytest.raises(RuntimeError, match="earlier servers remove"):
                await client.update_api_key_collection_permissions(request)
    assert sum(item[1] == "/api/health" for item in state["requests"]) == 1
    assert sum(item[0] == "PUT" for item in state["requests"]) == (
        2 if version in ("0.79.2", "0.79.3", "1.0.0") else 0
    )


@pytest.mark.asyncio
@pytest.mark.parametrize("status", [400, 401, 403, 429, 500, 503, 307])
async def test_safe_errors_and_bounded_retry(endpoint, status):
    client, state = endpoint
    state["status"] = status
    with pytest.raises(RuntimeError) as failure:
        await client.create_api_key()
    assert "secret" not in str(failure.value)
    assert str(status) in str(failure.value)
    assert len(state["requests"]) == (4 if status == 401 else 2)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "invalid",
    [
        {"read": 1},
        {"write": None},
        {"select_fields": False},
        {"select_fields": object()},
        {"exclude_fields": [""]},
        {"writable_fields": [None]},
        {"api_key": " "},
        {"collection": ""},
    ],
)
async def test_validation_precedes_http(endpoint, invalid):
    client, state = endpoint
    request = {
        "api_key": "target-secret",
        "collection": "users",
        "read": True,
        "write": False,
        **invalid,
    }
    with pytest.raises((ValueError, RuntimeError)):
        await client.update_api_key_collection_permissions(request)
    assert state["requests"] == []


@pytest.mark.asyncio
async def test_malformed_payload_is_redacted(endpoint):
    client, state = endpoint
    state["bad_json"] = True
    with pytest.raises(RuntimeError) as failure:
        await client.list_api_keys()
    assert "secret" not in str(failure.value)
