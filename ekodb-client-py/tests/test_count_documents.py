"""`count_documents` through the binding's real HTTP boundary (#405).

`count_documents` sends `POST /api/count/{collection}` with `{}` (count
everything) or `{"filter": <query expression>}` and returns the server's
`count`, so a collection larger than one page of records is counted exactly.
The route is new in ekoDB server 0.84.0: an older server answers 404 for the
unknown path, which raises `ApiError` naming the required version, and the
client never falls back to counting a page of records.
"""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

from ekodb_client import ApiError, Client, QueryBuilder
from ekodb_client._ekodb_client import SerializationFormat

UNSUPPORTED = (
    "count requires ekoDB server 0.84.0 or later "
    "(POST /api/count is not available on this server)"
)


def serve_count(status, body):
    """Answer every count request with `status`/`body`; record each request.

    Returns `(server, thread, client, requests)`, where `requests` gains one
    `(path, json_body, accept)` entry per request other than the token
    exchange, so a find sent after a count shows up there too. Retries are
    enabled, so a single recorded call proves the client did not re-send.
    """
    requests = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def reply(self, code, text):
            payload = text.encode()
            self.send_response(code)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)

        def do_POST(self):
            length = int(self.headers.get("Content-Length") or 0)
            raw = self.rfile.read(length) if length else b""
            if self.path == "/api/auth/token":
                self.reply(200, json.dumps({"token": "mock_jwt_token"}))
                return
            requests.append(
                (self.path, json.loads(raw) if raw else None, self.headers["Accept"])
            )
            if self.path.startswith("/api/count/"):
                self.reply(status, body)
            else:
                self.reply(404, json.dumps({"error": "Not found"}))

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    client = Client.new(
        f"http://127.0.0.1:{server.server_address[1]}",
        "test-api-key",
        should_retry=True,
        max_retries=3,
        timeout_secs=10,
        format=SerializationFormat.Json,
    )
    return server, thread, client, requests


def stop(server, thread):
    server.shutdown()
    server.server_close()
    thread.join()


@pytest.mark.asyncio
async def test_count_above_one_page_is_exact():
    server, thread, client, requests = serve_count(200, json.dumps({"count": 2029}))
    try:
        assert await client.count_documents("users") == 2029
        assert requests == [("/api/count/users", {}, "application/json")]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_filtered_count_sends_the_filter():
    server, thread, client, requests = serve_count(200, json.dumps({"count": 1500}))
    condition = QueryBuilder.condition("status", "Eq", "active")
    try:
        assert await client.count_documents("users", filter=condition) == 1500
        assert requests == [
            ("/api/count/users", {"filter": condition}, "application/json")
        ]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_server_without_the_route_raises_the_version_error():
    server, thread, client, requests = serve_count(404, "Not Found")
    try:
        with pytest.raises(ApiError) as caught:
            await client.count_documents("users")
        error = caught.value
        assert error.status == 404
        assert error.message == UNSUPPORTED
        assert error.retryable is False
        # One count request, and no find after it.
        assert [path for path, _, _ in requests] == ["/api/count/users"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_forbidden_filter_raises_api_error_with_the_server_text():
    body = json.dumps({"error": "No read permission for field salary"})
    server, thread, client, requests = serve_count(403, body)
    try:
        with pytest.raises(ApiError) as caught:
            await client.count_documents(
                "users", filter=QueryBuilder.condition("salary", "Gt", 100)
            )
        error = caught.value
        assert error.status == 403
        assert error.message == "No read permission for field salary"
        assert error.retryable is False
        assert [path for path, _, _ in requests] == ["/api/count/users"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_a_bad_request_passes_through_as_itself():
    body = json.dumps({"error": "Invalid filter structure in body"})
    server, thread, client, requests = serve_count(400, body)
    try:
        with pytest.raises(ApiError) as caught:
            await client.count_documents(
                "users", filter=QueryBuilder.condition("salary", "Gt", 100)
            )
        error = caught.value
        assert error.status == 400
        assert error.message == "Invalid filter structure in body"
        assert "0.84.0" not in error.message
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_collection_name_is_path_escaped():
    server, thread, client, requests = serve_count(200, json.dumps({"count": 3}))
    try:
        assert await client.count_documents("team/a b") == 3
        assert [path for path, _, _ in requests] == ["/api/count/team%2Fa%20b"]
    finally:
        stop(server, thread)
