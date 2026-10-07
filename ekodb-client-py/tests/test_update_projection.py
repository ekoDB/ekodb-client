"""Python update projection options reach the Rust HTTP transport."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, urlsplit

import pytest
from ekodb_client import Client
from ekodb_client._ekodb_client import SerializationFormat


@pytest.mark.asyncio
async def test_update_projection_and_cache_options_reach_server():
    requests = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def reply(self, value):
            body = json.dumps(value).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_POST(self):
            self.rfile.read(int(self.headers["Content-Length"]))
            assert self.path == "/api/auth/token"
            self.reply({"token": "mock_jwt_token"})

        def do_PUT(self):
            requests.append(parse_qs(urlsplit(self.path).query))
            self.rfile.read(int(self.headers["Content-Length"]))
            self.reply({"id": "id_1", "first name": "Alice"})

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        client = Client.new(
            f"http://127.0.0.1:{server.server_address[1]}",
            "test-api-key",
            should_retry=False,
            format=SerializationFormat.Json,
        )
        await client.update(
            "users",
            "id_1",
            {"first name": "Alice"},
            bypass_cache=False,
            select_fields=["first name", "email"],
            exclude_fields=["secret&token"],
        )
        await client.update("users", "id_1", {"first name": "Alice"})

        assert requests[0] == {
            "bypass_cache": ["false"],
            "select_fields": ["first name,email"],
            "exclude_fields": ["secret&token"],
        }
        assert requests[1] == {}
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
