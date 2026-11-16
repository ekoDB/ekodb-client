"""Exercise get_schedule_stats against a local HTTP server."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

from ekodb_client import ApiError, Client
from ekodb_client._ekodb_client import SerializationFormat


def serve(status, body):
    seen = []

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
            if length:
                self.rfile.read(length)
            if self.path == "/api/auth/token":
                self.reply(200, json.dumps({"token": "mock_jwt_token"}))
            else:
                self.reply(404, json.dumps({"error": "Not found"}))

        def do_GET(self):
            seen.append((self.path, self.headers.get("Authorization")))
            if self.path == "/api/schedules/stats":
                self.reply(status, body)
            else:
                self.reply(404, json.dumps({"error": "Not found"}))

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
    return server, thread, client, seen


def stop(server, thread):
    server.shutdown()
    server.server_close()
    thread.join()


@pytest.mark.asyncio
async def test_get_schedule_stats_reads_active_schedules():
    server, thread, client, seen = serve(200, json.dumps({"active_schedules": 3}))
    try:
        assert await client.get_schedule_stats() == {"active_schedules": 3}
        assert seen == [("/api/schedules/stats", "Bearer mock_jwt_token")]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_get_schedule_stats_forbidden_raises_api_error():
    server, thread, client, _ = serve(403, json.dumps({"error": "admin required"}))
    try:
        with pytest.raises(ApiError) as caught:
            await client.get_schedule_stats()
        assert caught.value.status == 403
    finally:
        stop(server, thread)
