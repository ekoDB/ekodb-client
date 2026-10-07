"""Exercise the Python upsert binding against a stateful local HTTP server."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

from ekodb_client import Client
from ekodb_client._ekodb_client import SerializationFormat


@pytest.mark.asyncio
async def test_upsert_miss_preserves_requested_id_and_next_call_updates():
    records = {}
    writes = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def reply(self, status, value):
            body = json.dumps(value).encode()
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def body(self):
            return json.loads(self.rfile.read(int(self.headers["Content-Length"])))

        def do_GET(self):
            if self.path == "/api/find/users/requested" and "requested" in records:
                self.reply(200, records["requested"])
            else:
                self.reply(404, {"error": "Not found"})

        def do_POST(self):
            if self.path == "/api/auth/token":
                self.body()
                self.reply(200, {"token": "mock_jwt_token"})
            elif self.path == "/api/insert/users":
                record = self.body()
                writes.append(("insert", record.copy()))
                records[record["id"]] = record
                self.reply(200, record)
            else:
                self.reply(404, {"error": "Not found"})

        def do_PUT(self):
            if self.path == "/api/update/users/requested":
                update = self.body()
                writes.append(("update", update.copy()))
                records["requested"].update(update)
                self.reply(200, records["requested"])
            else:
                self.reply(404, {"error": "Not found"})

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        client = Client.new(
            f"http://127.0.0.1:{server.server_address[1]}",
            "test-api-key",
            should_retry=False,
            timeout_secs=10,
            format=SerializationFormat.Json,
        )
        first = await client.upsert(
            "users", "requested", {"id": "different", "name": "first"}
        )
        second = await client.upsert("users", "requested", {"name": "second"})

        assert first["id"] == second["id"] == "requested"
        assert records == {"requested": {"id": "requested", "name": "second"}}
        assert [method for method, _ in writes] == ["insert", "update"]
        assert writes[0][1]["id"] == "requested"
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
