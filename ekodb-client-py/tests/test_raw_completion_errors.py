"""A failure the server reports inside a streamed completion, through the Python binding."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

from ekodb_client import ApiError, Client
from ekodb_client._ekodb_client import SerializationFormat

STREAM = b'data: {"error": "provider unavailable"}\n\n'


def serve():
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_POST(self):
            length = int(self.headers.get("Content-Length") or 0)
            if length:
                self.rfile.read(length)
            if self.path == "/api/auth/token":
                body = json.dumps({"token": "mock_jwt_token"}).encode()
                content_type = "application/json"
            elif self.path == "/api/chat/complete/stream":
                # A 200 stream whose only event is the server's error.
                body = STREAM
                content_type = "text/event-stream"
            else:
                self.send_response(404)
                self.end_headers()
                return
            self.send_response(200)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

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


@pytest.mark.asyncio
@pytest.mark.parametrize("with_progress", [False, True])
async def test_stream_error_event_raises_api_error_500(with_progress):
    server, thread, client = serve()
    try:
        with pytest.raises(ApiError) as caught:
            if with_progress:
                await client.raw_completion_stream_with_progress(
                    "s", "m", lambda _token: None
                )
            else:
                await client.raw_completion_stream("s", "m")
        # The failure carries no status of its own, so it is reported as 500
        # with the server's message.
        assert caught.value.status == 500
        assert caught.value.message == "provider unavailable"
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
