"""Python search binding accepts the optional HNSW beam width."""

import inspect
import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest
from ekodb_client import Client


def test_search_exposes_optional_ef_search():
    signature = inspect.signature(Client.search)
    parameter = signature.parameters["ef_search"]
    assert parameter.default is None
    assert parameter.kind is inspect.Parameter.POSITIONAL_OR_KEYWORD


@pytest.mark.asyncio
async def test_search_sends_ef_search_without_changing_default():
    requests = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_POST(self):
            body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            if self.path == "/api/auth/token":
                response = {"token": "mock_jwt_token"}
            elif self.path == "/api/search/items":
                requests.append(body)
                response = {"results": [], "total": 0, "execution_time_ms": 1}
            else:
                self.send_error(404)
                return
            encoded = json.dumps(response).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(encoded)))
            self.end_headers()
            self.wfile.write(encoded)

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        client = Client.new(
            f"http://127.0.0.1:{server.server_address[1]}",
            "test-api-key",
            should_retry=False,
        )
        await client.search("items", "", vector=[0.1, 0.2], ef_search=128)
        await client.search("items", "", vector=[0.1, 0.2])
        assert requests[0]["ef_search"] == 128
        assert "ef_search" not in requests[1]
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
