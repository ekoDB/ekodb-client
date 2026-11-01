"""Both permission route generations through the public PyO3 HTTP boundary."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

import pytest
from ekodb_client.admin import AdminClient

CONTRACT = json.loads(
    (Path(__file__).parents[2] / "test-fixtures/admin-contract-0.80.0.json").read_text()
)


@pytest.fixture
def modern_endpoint():
    state = {"requests": [], "change": None, "probe_status": 200}

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def handle_request(self):
            raw = self.rfile.read(int(self.headers.get("Content-Length") or 0))
            body = json.loads(raw) if raw else None
            state["requests"].append((self.command, self.path, body))
            status = 200
            if self.path == "/api/auth/token":
                response = {"token": "jwt-synthetic"}
            elif self.path == "/api/auth/keys/me/permissions":
                response = CONTRACT["permissions"]
                status = state["probe_status"]
            elif self.path == "/api/auth/register":
                response = state.get("key_record", CONTRACT["key"])
            elif self.path == "/api/auth/keys":
                response = [state.get("key_record", CONTRACT["key"])]
            elif self.path == "/api/auth/permissions":
                response = {
                    "is_admin": False,
                    "collection_permissions": {},
                    "kv_permissions": None,
                }
            elif self.path in state.get("reads", {}):
                response = state["reads"][self.path]
            elif state["change"] is not None:
                response = state["change"]["response"]
            else:
                response = {}
                status = 404
            encoded = json.dumps(response).encode()
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(encoded)))
            self.end_headers()
            self.wfile.write(encoded)

        do_GET = do_POST = do_PUT = do_PATCH = do_DELETE = handle_request

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(
        target=lambda: server.serve_forever(poll_interval=0.01), daemon=True
    )
    thread.start()
    try:
        yield state, f"http://127.0.0.1:{server.server_address[1]}"
    finally:
        server.shutdown()
        server.server_close()
        thread.join()


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "case", CONTRACT["permission_changes"], ids=lambda x: x["name"]
)
async def test_modern_permission_changes_and_secret_free_routes(modern_endpoint, case):
    state, base = modern_endpoint
    state["change"] = case
    client = AdminClient(base, "synthetic-admin-key-0123456789-abcdef")
    assert await client.uses_key_permission_routes()
    assert await client.uses_key_permission_routes()
    facet = case["facet"]
    name = "collection" if facet == "collections" else "prefix"
    request = {
        **(case["body"] or {}),
        "key_id": CONTRACT["key_id"],
        name: case["name"],
    }
    method_name = {
        ("collections", "PUT"): "put_api_key_collection_permissions",
        ("collections", "PATCH"): "patch_api_key_collection_permissions",
        ("collections", "DELETE"): "delete_api_key_collection_permissions",
        ("kv", "PUT"): "put_api_key_kv_permissions",
        ("kv", "PATCH"): "patch_api_key_kv_permissions",
        ("kv", "DELETE"): "delete_api_key_kv_permissions",
    }[(facet, case["method"])]
    method = getattr(client, method_name)
    if case["method"] == "DELETE":
        result = await method(CONTRACT["key_id"], case["name"])
    else:
        result = await method(request)
    assert result == case["response"]
    assert state["requests"][-1] == (case["method"], case["path"], case["body"])
    assert (
        sum(path == "/api/auth/keys/me/permissions" for _, path, _ in state["requests"])
        == 1
    )
    for _, path, body in state["requests"]:
        if "/permissions" in path:
            assert CONTRACT["key"]["key"] not in path
            assert CONTRACT["key"]["key"] not in json.dumps(body)
            assert "api_key" not in json.dumps(body)


@pytest.mark.asyncio
async def test_ids_and_permission_metadata_survive_python_binding(modern_endpoint):
    state, base = modern_endpoint
    client = AdminClient(base, "synthetic-admin-key-0123456789-abcdef")
    created = await client.create_api_key()
    listed = await client.list_api_keys()
    assert created["id"] == CONTRACT["key_id"]
    assert listed[0]["id"] == CONTRACT["key_id"]
    permissions = await client.get_permissions()
    assert permissions["is_admin"] is False
    assert (
        permissions["collection_permissions"] == CONTRACT["permissions"]["collections"]
    )
    assert permissions["kv_permissions"] == CONTRACT["permissions"]["kv"]
    count = len(state["requests"])
    with pytest.raises(RuntimeError, match="key id or me"):
        await client.get_api_key_permissions(CONTRACT["key"]["key"])
    assert len(state["requests"]) == count
    with pytest.raises(RuntimeError, match="A KV grant"):
        await client.put_api_key_kv_permissions(
            {"key_id": "me", "prefix": "session:", "read": False, "write": False}
        )
    assert len(state["requests"]) == count
    with pytest.raises(RuntimeError):
        await client.legacy_update_api_key_collection_permissions(
            {
                "api_key": CONTRACT["key"]["key"],
                "collection": "users",
                "read": True,
                "write": False,
            }
        )


@pytest.mark.asyncio
@pytest.mark.parametrize("status", [403, 404, 405, 500, 503])
async def test_probe_fallback_is_narrow_and_failures_are_not_cached(
    modern_endpoint, status
):
    state, base = modern_endpoint
    state["probe_status"] = status
    client = AdminClient(base, "synthetic-admin-key-0123456789-abcdef")
    for _ in range(2):
        if status in (404, 405):
            assert (await client.get_permissions())["is_admin"] is False
        else:
            with pytest.raises(RuntimeError):
                await client.get_permissions()
    probes = sum(
        path == "/api/auth/keys/me/permissions" for _, path, _ in state["requests"]
    )
    assert probes == (1 if status in (404, 405) else 2)


@pytest.mark.asyncio
async def test_nullable_registration_kv_grants_are_preserved(modern_endpoint):
    state, base = modern_endpoint
    state["key_record"] = {**CONTRACT["key"], "kv_permissions": None}
    client = AdminClient(base, "synthetic-admin-key-0123456789-abcdef")
    assert (await client.create_api_key())["kv_permissions"] is None
    assert (await client.list_api_keys())[0]["kv_permissions"] is None


@pytest.mark.asyncio
async def test_facet_reads_and_update_alias_cross_python_http_boundary(modern_endpoint):
    state, base = modern_endpoint
    key_id = CONTRACT["key_id"]
    root = f"/api/auth/keys/{key_id}/permissions"
    grant = CONTRACT["permissions"]["collections"]["tenant:a/"]
    state["reads"] = {
        root + "/collections": {"tenant:a/": grant},
        root + "/collections/tenant%3Aa%2F": grant,
        root + "/kv?prefix=tenant%3Aa%2F%20%2B": CONTRACT["permissions"]["kv"],
        root + "/kv/session%3A": CONTRACT["permissions"]["kv"][0],
    }
    state["change"] = {
        "response": {"before": None, "after": grant, "future_revision": 3}
    }
    client = AdminClient(base, "synthetic-admin-key-0123456789-abcdef")
    assert (await client.list_api_key_collection_permissions(key_id))[
        "tenant:a/"
    ] == grant
    assert (
        await client.get_api_key_collection_permissions(key_id, "tenant:a/")
    ) == grant
    assert (
        await client.list_api_key_kv_permissions(key_id, "tenant:a/ +")
    ) == CONTRACT["permissions"]["kv"]
    assert (await client.get_api_key_kv_permissions(key_id, "session:")) == CONTRACT[
        "permissions"
    ]["kv"][0]
    result = await client.update_api_key_collection_permissions(
        {
            "key_id": key_id,
            "collection": "users",
            "read": True,
            "write": False,
            "select_fields": None,
        }
    )
    assert result["future_revision"] == 3
    assert state["requests"][-1] == (
        "PATCH",
        root + "/collections/users",
        {"read": True, "write": False, "select_fields": None},
    )


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "method,payload",
    [
        (
            "put_api_key_collection_permissions",
            {
                "collection": "users",
                "read": True,
                "write": False,
                "select_field": ["name"],
            },
        ),
        (
            "patch_api_key_collection_permissions",
            {"collection": "users", "select_field": ["name"]},
        ),
        (
            "put_api_key_kv_permissions",
            {"prefix": "session:", "read": True, "write": False, "unknown": True},
        ),
        ("patch_api_key_kv_permissions", {"prefix": "session:", "unknown": True}),
        ("patch_api_key_collection_permissions", {"collection": "users", "read": None}),
        ("patch_api_key_kv_permissions", {"prefix": "session:", "write": None}),
    ],
)
async def test_invalid_dict_fields_and_null_rights_fail_before_http(
    modern_endpoint, method, payload
):
    state, base = modern_endpoint
    client = AdminClient(base, "synthetic-admin-key-0123456789-abcdef")
    with pytest.raises(ValueError, match="Invalid permission request"):
        await getattr(client, method)({"key_id": CONTRACT["key_id"], **payload})
    assert state["requests"] == []


@pytest.mark.asyncio
@pytest.mark.parametrize("name", [".", ".."])
@pytest.mark.parametrize("facet", ["collection", "kv"])
@pytest.mark.parametrize("operation", ["get", "put", "patch", "delete"])
async def test_dot_segment_facets_fail_before_http(
    modern_endpoint, name, facet, operation
):
    state, base = modern_endpoint
    client = AdminClient(base, "synthetic-admin-key-0123456789-abcdef")
    method = getattr(client, f"{operation}_api_key_{facet}_permissions")
    with pytest.raises(RuntimeError, match="Invalid permission facet"):
        if operation in ("get", "delete"):
            await method("me", name)
        else:
            noun = "collection" if facet == "collection" else "prefix"
            await method({"key_id": "me", noun: name, "read": True, "write": False})
    assert state["requests"] == []
