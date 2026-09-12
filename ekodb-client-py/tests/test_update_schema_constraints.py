"""
Binding-level test for update_schema_constraints.

The Python client wraps the Rust core (ekodb_client), which sends
`PUT /api/schemas/{collection}` with a body of `{"constraints": {...}}` — a
per-field, per-attribute partial update where an attribute left off a
field's dict is omitted from the request entirely (mirrors the Rust
`SchemaConstraintUpdate`'s `skip_serializing_if = "Option::is_none"` on every
attribute). A local HTTP server captures the real outgoing PUT body so this
test drives the real binding rather than reimplementing the serialization by
hand.
"""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest
from ekodb_client import Client

captured = {}


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *_):  # keep pytest output clean
        pass

    def do_POST(self):
        length = int(self.headers.get("Content-Length") or 0)
        self.rfile.read(length)
        if self.path == "/api/auth/token":
            body = b'{"token":"mock_jwt_token"}'
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
            return
        self.send_response(404)
        self.end_headers()

    def do_PUT(self):
        length = int(self.headers.get("Content-Length") or 0)
        raw = self.rfile.read(length)
        captured["path"] = self.path
        captured["body"] = json.loads(raw)
        body = b"{}"
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)


@pytest.fixture(scope="module")
def server():
    httpd = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=httpd.serve_forever, daemon=True)
    thread.start()
    yield f"http://127.0.0.1:{httpd.server_address[1]}"
    httpd.shutdown()


@pytest.mark.asyncio
async def test_update_schema_constraints_sends_partial_update(server):
    client = Client.new(server, "test-api-key", should_retry=False, timeout_secs=10)
    result = await client.update_schema_constraints(
        "users",
        {
            "email": {"required": True},
            "age": {"min": 0, "max": 150},
        },
    )

    assert result is None
    assert captured["path"] == "/api/schemas/users"
    assert captured["body"] == {
        "constraints": {
            "email": {"required": True},
            "age": {"min": 0, "max": 150},
        }
    }
    # Partial update: an attribute left off a field's dict must not appear
    # on the wire at all — not present, not null.
    email_constraint = captured["body"]["constraints"]["email"]
    for omitted in ("field_type", "default", "unique", "enums", "max", "min", "regex"):
        assert omitted not in email_constraint

    age_constraint = captured["body"]["constraints"]["age"]
    for omitted in ("field_type", "default", "unique", "required", "enums", "regex"):
        assert omitted not in age_constraint
