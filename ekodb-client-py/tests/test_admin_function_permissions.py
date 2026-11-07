"""Stored-function call grants through the public PyO3 HTTP boundary."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

import pytest
from ekodb_client import Client
from ekodb_client.admin import MAX_FUNCTION_PATTERN_BYTES, AdminClient

CONTRACT = json.loads(
    (
        Path(__file__).parents[2]
        / "test-fixtures/admin-function-permissions-contract.json"
    ).read_text()
)
KEY_ID = CONTRACT["key_id"]
ADMIN_KEY = "synthetic-admin-key-0123456789-abcdef"


@pytest.fixture
def endpoint():
    """A local server answering from `state["routes"]` keyed by (method, path)."""
    state = {"requests": [], "routes": {}}

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def handle_request(self):
            raw = self.rfile.read(int(self.headers.get("Content-Length") or 0))
            body = json.loads(raw) if raw else None
            state["requests"].append((self.command, self.path, body))
            status, response = 404, {"error": "synthetic-secret"}
            if self.path == "/api/auth/token":
                status, response = 200, {"token": "jwt-synthetic"}
            elif self.path == "/api/auth/keys/me/permissions":
                status, response = state["routes"].get(
                    (self.command, self.path),
                    (
                        200,
                        {"admin": True, "collections": {}, "kv": [], "functions": []},
                    ),
                )
            elif (self.command, self.path) in state["routes"]:
                status, response = state["routes"][(self.command, self.path)]
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


def grant_path(pattern_segment):
    return f"/api/auth/keys/{KEY_ID}/permissions/functions/{pattern_segment}"


def non_auth(state):
    return [
        r
        for r in state["requests"]
        if r[1] not in ("/api/auth/token", "/api/auth/keys/me/permissions")
    ]


@pytest.mark.asyncio
async def test_registration_sends_function_permissions_and_reads_the_echoed_map(
    endpoint,
):
    state, base = endpoint
    state["routes"][("POST", "/api/auth/register")] = (
        200,
        CONTRACT["registration_response"],
    )
    client = AdminClient(base, ADMIN_KEY)
    request = {
        "label": "report-runner",
        "function_permissions": {
            "report_*": ["call"],
            "nightly_close": ["call"],
            "audit_only": [],
        },
    }
    key = await client.create_api_key(request)
    sent = non_auth(state)[0]
    assert sent[2] == CONTRACT["registration_request"]
    assert key["function_permissions"] == [
        {"pattern": pattern} for pattern in CONTRACT["expected_registered_patterns"]
    ]


@pytest.mark.asyncio
async def test_registration_without_grants_keeps_the_old_body_and_reads_none(
    endpoint,
):
    state, base = endpoint
    state["routes"][("POST", "/api/auth/register")] = (
        200,
        CONTRACT["registration_response_without_function_permissions"],
    )
    client = AdminClient(base, ADMIN_KEY)
    key = await client.create_api_key({"label": "report-runner"})
    body = non_auth(state)[0][2]
    assert "function_permissions" not in body
    assert key["function_permissions"] is None


@pytest.mark.asyncio
@pytest.mark.parametrize("pattern", ["", "re*port", "report_**", "*report"])
async def test_registration_refuses_an_invalid_pattern_before_http(endpoint, pattern):
    state, base = endpoint
    client = AdminClient(base, ADMIN_KEY)
    with pytest.raises(RuntimeError, match="Invalid function pattern"):
        await client.create_api_key({"function_permissions": {pattern: ["call"]}})
    assert state["requests"] == []


@pytest.mark.asyncio
async def test_registration_refuses_a_right_other_than_call(endpoint):
    state, base = endpoint
    client = AdminClient(base, ADMIN_KEY)
    with pytest.raises(ValueError):
        await client.create_api_key({"function_permissions": {"report_*": ["run"]}})
    assert state["requests"] == []


@pytest.mark.asyncio
async def test_key_listing_and_permission_reads_carry_function_grants(endpoint):
    state, base = endpoint
    state["routes"][("GET", "/api/auth/keys")] = (200, [CONTRACT["key_list_record"]])
    state["routes"][("GET", "/api/auth/keys/me/permissions")] = (
        200,
        CONTRACT["permissions"],
    )
    client = AdminClient(base, ADMIN_KEY)
    keys = await client.list_api_keys()
    assert keys[0]["function_permissions"] == CONTRACT["function_grants"]
    permissions = await client.get_permissions()
    assert permissions["function_permissions"] == CONTRACT["function_grants"]

    state["routes"][("GET", "/api/auth/keys/me/permissions")] = (
        200,
        CONTRACT["permissions_without_functions"],
    )
    older = AdminClient(base, ADMIN_KEY)
    assert (await older.get_permissions())["function_permissions"] == []


def answer_listing_and_aggregate(state, value):
    """Key listing and aggregate read whose function grants are `value`."""
    record = {**CONTRACT["key_list_record"], "function_permissions": value}
    permissions = {**CONTRACT["permissions"], "functions": value}
    state["routes"][("GET", "/api/auth/keys")] = (200, [record])
    state["routes"][("GET", "/api/auth/keys/me/permissions")] = (200, permissions)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "case",
    CONTRACT["function_permissions_reply_values"]["accepted"],
    ids=lambda case: case["name"],
)
async def test_listing_and_aggregate_read_accept_the_shared_shapes(endpoint, case):
    state, base = endpoint
    answer_listing_and_aggregate(state, case["value"])
    client = AdminClient(base, ADMIN_KEY)
    keys = await client.list_api_keys()
    listed = keys[0]["function_permissions"] or []
    assert [grant["pattern"] for grant in listed] == case["patterns"]
    permissions = await client.get_permissions()
    aggregate = permissions["function_permissions"]
    assert [grant["pattern"] for grant in aggregate] == case["patterns"]


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "case",
    CONTRACT["function_permissions_reply_values"]["rejected"],
    ids=lambda case: case["name"],
)
async def test_listing_and_aggregate_read_refuse_the_shared_shapes(endpoint, case):
    state, base = endpoint
    answer_listing_and_aggregate(state, case["value"])
    client = AdminClient(base, ADMIN_KEY)
    with pytest.raises(RuntimeError):
        await client.list_api_keys()
    with pytest.raises(RuntimeError):
        await client.get_permissions()


@pytest.mark.asyncio
async def test_list_and_get_read_the_function_grant_routes(endpoint):
    state, base = endpoint
    state["routes"][("GET", grant_path("").rstrip("/"))] = (
        200,
        CONTRACT["function_grants"],
    )
    state["routes"][("GET", grant_path("report_%2A"))] = (200, {"pattern": "report_*"})
    client = AdminClient(base, ADMIN_KEY)
    assert (
        await client.list_api_key_function_permissions(KEY_ID)
        == CONTRACT["function_grants"]
    )
    assert await client.get_api_key_function_permissions(KEY_ID, "report_*") == {
        "pattern": "report_*"
    }
    with pytest.raises(RuntimeError, match="404") as error:
        await client.get_api_key_function_permissions(KEY_ID, "nope")
    assert "synthetic-secret" not in str(error.value)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "case", CONTRACT["function_grant_changes"], ids=lambda case: case["name"]
)
async def test_grant_changes_follow_the_recorded_routes_bodies_and_replies(
    endpoint, case
):
    state, base = endpoint
    state["routes"][(case["method"], case["path"])] = (
        case["status"],
        case["response"],
    )
    client = AdminClient(base, ADMIN_KEY)
    verb = "put" if case["method"] == "PUT" else "delete"
    change = await getattr(client, f"{verb}_api_key_function_permissions")(
        KEY_ID, case["pattern"]
    )
    assert non_auth(state) == [(case["method"], case["path"], case["body"])]
    assert change == case["response"]


@pytest.mark.asyncio
@pytest.mark.parametrize("case", CONTRACT["refusals"], ids=lambda case: case["name"])
async def test_refused_grant_changes_surface_the_status_without_the_body(
    endpoint, case
):
    state, base = endpoint
    state["routes"][(case["method"], grant_path("report_%2A"))] = (
        case["status"],
        {"error": "synthetic-secret"},
    )
    client = AdminClient(base, ADMIN_KEY)
    verb = "put" if case["method"] == "PUT" else "delete"
    with pytest.raises(RuntimeError, match=str(case["status"])) as error:
        await getattr(client, f"{verb}_api_key_function_permissions")(
            KEY_ID, "report_*"
        )
    assert "synthetic-secret" not in str(error.value)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "case", CONTRACT["rejected_patterns"], ids=lambda case: case["name"]
)
@pytest.mark.parametrize("verb", ["get", "put", "delete"])
async def test_invalid_patterns_fail_before_http(endpoint, case, verb):
    state, base = endpoint
    pattern = case.get("value", "a" * case.get("length", 0))
    client = AdminClient(base, ADMIN_KEY)
    with pytest.raises(RuntimeError, match="Invalid function pattern"):
        await getattr(client, f"{verb}_api_key_function_permissions")(KEY_ID, pattern)
    assert state["requests"] == []


@pytest.mark.asyncio
async def test_a_key_value_is_never_a_target(endpoint):
    state, base = endpoint
    client = AdminClient(base, ADMIN_KEY)
    with pytest.raises(RuntimeError, match="key id or me"):
        await client.put_api_key_function_permissions(
            "synthetic-target-key-0123456789-abcdef", "report_*"
        )
    assert state["requests"] == []


@pytest.mark.asyncio
async def test_the_longest_pattern_is_accepted(endpoint):
    state, base = endpoint
    assert MAX_FUNCTION_PATTERN_BYTES == CONTRACT["accepted_pattern_length"]
    pattern = "a" * (MAX_FUNCTION_PATTERN_BYTES - 1) + "*"
    path = grant_path("a" * (MAX_FUNCTION_PATTERN_BYTES - 1) + "%2A")
    state["routes"][("PUT", path)] = (
        200,
        {"before": None, "after": {"pattern": pattern}},
    )
    client = AdminClient(base, ADMIN_KEY)
    change = await client.put_api_key_function_permissions(KEY_ID, pattern)
    assert change["after"] == {"pattern": pattern}


@pytest.mark.asyncio
async def test_function_grant_routes_need_key_addressed_servers(endpoint):
    state, base = endpoint
    state["routes"][("GET", "/api/auth/keys/me/permissions")] = (404, {})
    client = AdminClient(base, ADMIN_KEY)
    with pytest.raises(RuntimeError, match="modern permission routes"):
        await client.put_api_key_function_permissions("me", "report_*")


@pytest.mark.asyncio
async def test_function_definitions_carry_run_as_and_created_by(endpoint):
    state, base = endpoint
    definition = CONTRACT["function_definition"]
    state["routes"][("GET", "/api/functions/nightly_close")] = (200, definition)
    state["routes"][("GET", "/api/functions/legacy_close")] = (
        200,
        CONTRACT["function_definition_unrecorded"],
    )
    state["routes"][("POST", "/api/functions")] = (
        200,
        {"status": "success", "id": definition["id"]},
    )
    client = Client.new(base, ADMIN_KEY, should_retry=False, timeout_secs=30)

    read = await client.get_user_function("nightly_close")
    assert read["run_as"] == "author"
    assert read["created_by"] == definition["created_by"]
    unrecorded = await client.get_user_function("legacy_close")
    # Unrecorded fields are omitted from the returned dict, never invented.
    assert unrecorded.get("run_as") is None
    assert unrecorded.get("created_by") is None

    await client.save_user_function(CONTRACT["function_save_request"])
    saved = [r for r in state["requests"] if r[:2] == ("POST", "/api/functions")][0]
    assert saved[2]["run_as"] == "caller"
    assert "created_by" not in saved[2]

    with pytest.raises(Exception, match="expected `caller` or `author`"):
        await client.save_user_function(
            {**CONTRACT["function_save_request"], "run_as": "root"}
        )


@pytest.mark.asyncio
async def test_a_refused_function_call_is_a_normal_error(endpoint):
    state, base = endpoint
    refusal = CONTRACT["function_call_refusal"]
    state["routes"][("POST", "/api/functions/nightly_close")] = (
        refusal["status"],
        refusal["body"],
    )
    client = Client.new(base, ADMIN_KEY, should_retry=False, timeout_secs=30)
    with pytest.raises(Exception, match="403"):
        await client.call_function("nightly_close", {})


@pytest.mark.asyncio
async def test_schedules_return_the_creator_the_server_recorded(endpoint):
    state, base = endpoint
    record = CONTRACT["schedule_record"]
    state["routes"][("GET", f"/api/schedules/{record['id']}")] = (200, record)
    client = Client.new(base, ADMIN_KEY, should_retry=False, timeout_secs=30)
    schedule = await client.get_schedule(record["id"])
    assert schedule["created_by"] == record["created_by"]
