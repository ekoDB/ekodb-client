"""Transaction call failures through the binding's real HTTP boundary (#406).

A transaction route answers a failure with
`{"error": "...", "transaction_id": "...", "retryable": <bool>}`. A failed
commit has already discarded its transaction, so the client never re-sends a
transaction call, even with retries enabled: the caller sees the server's own
message and flag, and `retryable` means begin a new transaction.
"""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

from ekodb_client import ApiError, Client
from ekodb_client._ekodb_client import SerializationFormat


def serve(status, body, content_type="application/json"):
    """Serve `status`/`body` for every transaction route; count those requests.

    The client has retries enabled, so a single recorded call proves the client
    chose not to re-send.

    Returns `(server, thread, client, calls)`, where `calls` is a list that
    gains one entry per transaction request the server received.
    """
    calls = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def reply(self, code, text, kind):
            payload = text.encode()
            self.send_response(code)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)

        def do_POST(self):
            length = int(self.headers.get("Content-Length") or 0)
            if length:
                self.rfile.read(length)
            if self.path == "/api/auth/token":
                self.reply(
                    200, json.dumps({"token": "mock_jwt_token"}), "application/json"
                )
            elif self.path.startswith("/api/transactions"):
                calls.append(self.path)
                self.reply(status, body, content_type)
            else:
                self.reply(404, json.dumps({"error": "Not found"}), "application/json")

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
    return server, thread, client, calls


def stop(server, thread):
    server.shutdown()
    server.server_close()
    thread.join()


@pytest.mark.asyncio
async def test_commit_conflict_raises_retryable_api_error_sent_once():
    body = json.dumps(
        {
            "error": "write conflict on users/1",
            "transaction_id": "tx-1",
            "retryable": True,
        }
    )
    server, thread, client, calls = serve(409, body)
    try:
        with pytest.raises(ApiError) as caught:
            await client.commit_transaction("tx-1")
        error = caught.value
        assert error.status == 409
        assert error.message == "write conflict on users/1"
        assert error.retryable is True
        # Retries are enabled, and the commit is still sent exactly once.
        assert calls == ["/api/transactions/tx-1/commit"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_commit_constraint_failure_is_not_retried():
    body = json.dumps(
        {
            "error": "unique constraint violated",
            "transaction_id": "tx-2",
            "retryable": False,
        }
    )
    server, thread, client, calls = serve(400, body)
    try:
        with pytest.raises(ApiError) as caught:
            await client.commit_transaction("tx-2")
        error = caught.value
        assert error.status == 400
        assert error.message == "unique constraint violated"
        assert error.retryable is False
        assert calls == ["/api/transactions/tx-2/commit"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_commit_non_json_error_body_falls_back_to_raw_text():
    server, thread, client, calls = serve(502, "upstream connect error", "text/plain")
    try:
        with pytest.raises(ApiError) as caught:
            await client.commit_transaction("tx-3")
        error = caught.value
        assert error.status == 502
        assert error.message == "upstream connect error"
        assert error.retryable is False
        assert calls == ["/api/transactions/tx-3/commit"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_rollback_of_a_missing_session_keeps_the_server_message():
    body = json.dumps(
        {
            "error": "transaction tx-9 not found",
            "transaction_id": "tx-9",
            "retryable": False,
        }
    )
    server, thread, client, calls = serve(404, body)
    try:
        with pytest.raises(ApiError) as caught:
            await client.rollback_transaction("tx-9")
        error = caught.value
        assert error.status == 404
        assert error.message == "transaction tx-9 not found"
        assert error.retryable is False
        assert str(error).startswith("Rollback transaction failed: ")
        assert calls == ["/api/transactions/tx-9/rollback"]
    finally:
        stop(server, thread)
