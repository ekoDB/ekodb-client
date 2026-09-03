"""
Binding-level tests for the chat stream's error events.

A stream error from the deployment carries the provider-failure
classification (`error_kind`, `provider`, `provider_status`,
`retry_after_secs`) when the failure was the LLM provider's answer; the
Python dict the binding hands back carries exactly those keys, and a plain
error carries none of them. A local SSE server drives the real binding.
"""

import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest
from ekodb_client import Client

CLASSIFIED = (
    b'event: token\ndata: {"token":"Hel"}\n\n'
    b'event: error\ndata: {"error":"OpenAI API error 429 Too Many Requests",'
    b'"error_kind":"provider_rate_limited","provider":"openai",'
    b'"provider_status":429,"retry_after_secs":7}\n\n'
)
PLAIN = b'data: {"error":"Model unavailable"}\n\n'
STRUCTURED = (
    b'event: error\ndata: {"error":{"code":"upstream_down"},'
    b'"error_kind":"provider_unavailable","provider":"gemini"}\n\n'
)


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
        bodies = {
            "/api/chat/classified/messages/stream": CLASSIFIED,
            "/api/chat/plain/messages/stream": PLAIN,
            "/api/chat/structured/messages/stream": STRUCTURED,
        }
        body = bodies.get(self.path)
        if body is None:
            self.send_response(404)
            self.end_headers()
            return
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
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


async def events_of(base_url, chat_id):
    client = Client.new(base_url, "test-api-key", should_retry=False, timeout_secs=10)
    stream = await client.chat_message_stream(chat_id, "hi")
    events = []
    while True:
        event = await stream.recv()
        if event is None:
            return events
        events.append(event)


@pytest.mark.asyncio
async def test_error_event_carries_the_provider_classification(server):
    events = await events_of(server, "classified")
    assert events[0] == {"type": "chunk", "content": "Hel"}
    assert events[1] == {
        "type": "error",
        "error": "OpenAI API error 429 Too Many Requests",
        "error_kind": "provider_rate_limited",
        "provider": "openai",
        "provider_status": 429,
        "retry_after_secs": 7,
    }
    assert len(events) == 2


@pytest.mark.asyncio
async def test_plain_error_event_carries_no_classification(server):
    events = await events_of(server, "plain")
    assert events == [{"type": "error", "error": "Model unavailable"}]


@pytest.mark.asyncio
async def test_structured_error_is_still_an_error_with_string_text(server):
    events = await events_of(server, "structured")
    assert events == [
        {
            "type": "error",
            "error": "Unknown error",
            "error_kind": "provider_unavailable",
            "provider": "gemini",
        }
    ]
