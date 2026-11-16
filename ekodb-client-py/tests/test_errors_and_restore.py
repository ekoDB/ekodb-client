"""Exercise the Python error mapping and restore counts against a local HTTP server.

Every case goes through the binding's real HTTP boundary: the server answers a
status, and the test asserts the exception (or result) a caller sees.
"""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

from ekodb_client import ApiError, Client, RateLimitError
from ekodb_client._ekodb_client import SerializationFormat


def serve(answer):
    """Start a server whose `POST /api/trash/<collection>` replies `answer()`.

    `answer` returns `(status, headers, body_text)`.
    """

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def reply(self, status, headers, text):
            body = text.encode()
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            for name, value in headers.items():
                self.send_header(name, value)
            self.end_headers()
            self.wfile.write(body)

        def do_POST(self):
            length = int(self.headers.get("Content-Length") or 0)
            if length:
                self.rfile.read(length)
            if self.path == "/api/auth/token":
                self.reply(200, {}, json.dumps({"token": "mock_jwt_token"}))
            elif self.path == "/api/trash/users":
                self.reply(*answer())
            else:
                self.reply(404, {}, json.dumps({"error": "Not found"}))

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    client = Client.new(
        f"http://127.0.0.1:{server.server_address[1]}",
        "test-api-key",
        should_retry=False,
        timeout_secs=10,
        format=SerializationFormat.Json,
    )
    return server, thread, client


def stop(server, thread):
    server.shutdown()
    server.server_close()
    thread.join()


@pytest.mark.asyncio
async def test_rate_limit_error_has_retry_after_secs():
    server, thread, client = serve(
        lambda: (429, {"Retry-After": "42"}, json.dumps({"error": "slow down"}))
    )
    try:
        with pytest.raises(RateLimitError) as caught:
            await client.restore_collection("users")
        error = caught.value
        # The documented attribute and the original args[0] carry the same wait.
        assert error.retry_after_secs == 42
        assert error.args[0] == 42
        assert error.status == 429
        # A rate limit stays its own type, not an ApiError.
        assert not isinstance(error, ApiError)
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_module_docstring_rate_limit_example_runs_as_written():
    server, thread, client = serve(
        lambda: (429, {"Retry-After": "7"}, json.dumps({"error": "slow down"}))
    )
    try:
        printed = []
        try:
            await client.restore_collection("users")
        except RateLimitError as e:
            printed.append(f"Rate limited! Retry after {e.retry_after_secs} seconds")
        assert printed == ["Rate limited! Retry after 7 seconds"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "status,body",
    [
        (404, '{"error":"Collection not found"}'),
        (409, '{"error":"Conflict: unique constraint"}'),
        (500, '{"error":"internal"}'),
    ],
)
async def test_http_error_raises_api_error_with_status(status, body):
    server, thread, client = serve(lambda: (status, {}, body))
    try:
        with pytest.raises(ApiError) as caught:
            await client.restore_collection("users")
        error = caught.value
        assert error.status == status
        assert isinstance(error.message, str) and error.message
        # Code that caught RuntimeError before still catches it.
        assert isinstance(error, RuntimeError)
        assert str(error).startswith("Restore collection failed: ")
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_restore_collection_counts_reports_both_counts():
    server, thread, client = serve(
        lambda: (
            200,
            {},
            json.dumps(
                {"status": "success", "cleared_count": 5, "records_restored": 3}
            ),
        )
    )
    try:
        assert await client.restore_collection_counts("users") == {
            "cleared_count": 5,
            "records_restored": 3,
        }
        # The plain method keeps returning the cleared count.
        assert await client.restore_collection("users") == 5
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_restore_collection_counts_absent_records_restored_is_none():
    server, thread, client = serve(
        lambda: (200, {}, json.dumps({"status": "success", "cleared_count": 5}))
    )
    try:
        assert await client.restore_collection_counts("users") == {
            "cleared_count": 5,
            "records_restored": None,
        }
    finally:
        stop(server, thread)
