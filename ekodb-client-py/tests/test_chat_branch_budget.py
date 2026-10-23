"""Completion budgets cross the public Python/PyO3/Rust HTTP boundary."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest
from ekodb_client import Client


@pytest.fixture
def endpoint():
    requests = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_POST(self):
            body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            if self.path == "/api/auth/token":
                response = {"token": "mock_jwt_token"}
            elif self.path == "/api/chat/branch":
                requests.append(body)
                response = {
                    "chat_id": "branched-chat",
                    "message_id": "message",
                    "responses": [],
                    "context_snippets": [],
                    "execution_time_ms": 0,
                }
            else:
                self.send_error(404)
                return
            data = json.dumps(response).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        yield f"http://127.0.0.1:{server.server_port}", requests
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=2)


@pytest.mark.asyncio
@pytest.mark.parametrize("options", [{}, {"max_tokens": None}, {"max_tokens": 1024}])
async def test_branch_completion_budget_and_legacy_arguments(endpoint, options):
    url, requests = endpoint
    client = Client.new(url, "test-key", should_retry=False)
    result = await client.branch_chat_session(
        "parent-chat", 0, [("products", ["name"])], "openai", "gpt-6-luna", **options
    )
    assert result["chat_id"] == "branched-chat"
    assert len(requests) == 1
    body = requests[0]
    assert body["parent_id"] == "parent-chat"
    assert body["branch_point_idx"] == 0
    assert body["llm_model"] == "gpt-6-luna"
    assert body["collections"][0]["fields"][0]["field_name"] == "name"
    if options.get("max_tokens") is None:
        assert "max_tokens" not in body
    else:
        assert body["max_tokens"] == 1024
    assert "temperature" not in body
