"""Transaction call failures through the binding's real HTTP boundary (#406).

A call on an existing transaction (commit, rollback, or a savepoint call)
answers a failure with
`{"error": "...", "transaction_id": "...", "retryable": <bool>}`. A failed
commit has already discarded its transaction, so the client never re-sends
those calls, even with retries enabled: the caller sees the server's own
message and flag, and `retryable` means begin a new transaction. Begin and
status keep the normal retry policy and error types. A 401 is the one answer
sent again, once, after the token is refreshed.
"""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

from ekodb_client import ApiError, Client, RateLimitError
from ekodb_client._ekodb_client import SerializationFormat


def serve(status, body, content_type="application/json"):
    """Serve `status`/`body` for every transaction route; count those requests.

    The client has retries enabled, so a single recorded call proves the client
    chose not to re-send.

    Returns `(server, thread, client, calls)`, where `calls` is a list that
    gains one entry per transaction request the server received.
    """
    server, thread, client, calls, _ = serve_answers([(status, body, content_type, {})])
    return server, thread, client, calls


def serve_answers(answers):
    """Serve `answers` in order to transaction requests; the last one repeats.

    Each answer is `(status, body, content_type, headers)`. Returns
    `(server, thread, client, calls, token_calls)`: `calls` gains one entry per
    transaction request, and `token_calls` one per token exchange.
    """
    calls = []
    token_calls = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def reply(self, code, text, kind, headers):
            payload = text.encode()
            self.send_response(code)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(payload)))
            for name, value in headers.items():
                self.send_header(name, value)
            self.end_headers()
            self.wfile.write(payload)

        def do_POST(self):
            length = int(self.headers.get("Content-Length") or 0)
            if length:
                self.rfile.read(length)
            if self.path == "/api/auth/token":
                token_calls.append(self.path)
                self.reply(
                    200,
                    json.dumps({"token": "mock_jwt_token"}),
                    "application/json",
                    {},
                )
            elif self.path.startswith("/api/transactions"):
                calls.append(self.path)
                answer = answers[min(len(calls), len(answers)) - 1]
                self.reply(*answer)
            else:
                self.reply(
                    404, json.dumps({"error": "Not found"}), "application/json", {}
                )

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
    return server, thread, client, calls, token_calls


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


@pytest.mark.asyncio
async def test_commit_non_boolean_retryable_reads_as_false():
    body = json.dumps(
        {"error": "write conflict", "transaction_id": "tx-4", "retryable": "true"}
    )
    server, thread, client, calls = serve(409, body)
    try:
        with pytest.raises(ApiError) as caught:
            await client.commit_transaction("tx-4")
        error = caught.value
        assert error.status == 409
        assert error.message == "write conflict"
        assert error.retryable is False
        assert calls == ["/api/transactions/tx-4/commit"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_commit_429_is_sent_once_as_api_error():
    body = json.dumps(
        {"error": "too many requests", "transaction_id": "tx-5", "retryable": True}
    )
    server, thread, client, calls, _ = serve_answers(
        [(429, body, "application/json", {"Retry-After": "0"})]
    )
    try:
        with pytest.raises(ApiError) as caught:
            await client.commit_transaction("tx-5")
        error = caught.value
        assert error.status == 429
        assert error.message == "too many requests"
        assert error.retryable is True
        assert calls == ["/api/transactions/tx-5/commit"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_begin_429_is_retried_and_succeeds():
    server, thread, client, calls, _ = serve_answers(
        [
            (429, "", "text/plain", {"Retry-After": "0"}),
            (200, json.dumps({"transaction_id": "tx-new"}), "application/json", {}),
        ]
    )
    try:
        assert await client.begin_transaction() == "tx-new"
        # A refused begin started no transaction, so it is sent again.
        assert calls == ["/api/transactions", "/api/transactions"]
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_begin_429_after_retries_raises_rate_limit_error():
    server, thread, client, calls, _ = serve_answers(
        [(429, "", "text/plain", {"Retry-After": "0"})]
    )
    try:
        with pytest.raises(RateLimitError) as caught:
            await client.begin_transaction()
        assert caught.value.retry_after_secs == 0
        # The first send plus the client's three retries.
        assert calls == ["/api/transactions"] * 4
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_commit_401_refreshes_the_token_once_and_resends():
    server, thread, client, calls, token_calls = serve_answers(
        [
            (401, "", "text/plain", {}),
            (200, json.dumps({"status": "committed"}), "application/json", {}),
        ]
    )
    try:
        await client.commit_transaction("tx-6")
        assert calls == ["/api/transactions/tx-6/commit"] * 2
        # The first token exchange plus the one refresh.
        assert len(token_calls) == 2
    finally:
        stop(server, thread)


@pytest.mark.asyncio
async def test_commit_401_twice_raises_after_two_sends():
    server, thread, client, calls, token_calls = serve_answers(
        [(401, "", "text/plain", {})]
    )
    try:
        with pytest.raises(ApiError) as caught:
            await client.commit_transaction("tx-7")
        assert caught.value.status == 401
        assert calls == ["/api/transactions/tx-7/commit"] * 2
        assert len(token_calls) == 2
    finally:
        stop(server, thread)
