"""Generated stored-function contract coverage through the real PyO3 codec."""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

import pytest
from ekodb_client import Client

captured = {}


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def do_POST(self):
        length = int(self.headers.get("Content-Length") or 0)
        body = self.rfile.read(length)
        if self.path == "/api/auth/token":
            response = b'{"token":"mock_jwt_token"}'
        elif self.path == "/api/functions":
            captured["function"] = json.loads(body)
            response = b'{"status":"success","id":"contract-id"}'
        else:
            self.send_response(404)
            self.end_headers()
            return

        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(response)))
        self.end_headers()
        self.wfile.write(response)


@pytest.fixture(scope="module")
def server():
    httpd = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=httpd.serve_forever, daemon=True)
    thread.start()
    yield f"http://127.0.0.1:{httpd.server_address[1]}"
    httpd.shutdown()


def load_fixture():
    path = Path(__file__).parents[2] / "test-fixtures/function-stage-contract.json"
    return json.loads(path.read_text())


@pytest.mark.asyncio
async def test_every_generated_stage_round_trips_through_rust_codec(server):
    fixture = load_fixture()
    assert fixture["variant_count"] == len(fixture["variants"])
    assert len(fixture["variants"]) >= fixture["coverage_floor"]
    stages = [case["stage"] for case in fixture["variants"]]
    function = {
        "label": "contract_round_trip",
        "name": "Contract round trip",
        "parameters": {},
        "functions": stages,
        "tags": [],
        "transaction_config": {
            "enabled": True,
            "auto_rollback": True,
            "isolation_level": "Serializable",
        },
    }

    client = Client.new(server, "test-api-key", should_retry=False, timeout_secs=10)
    assert await client.save_user_function(function) == "contract-id"
    assert captured["function"] == function


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "stage",
    [
        {"type": "FindAll", "collection": "items", "future_field": True},
        {"type": "FutureStage", "future_value": True},
    ],
)
async def test_unknown_stage_data_fails_before_it_can_be_sent(server, stage):
    client = Client.new(server, "test-api-key", should_retry=False, timeout_secs=10)
    function = {
        "label": "unknown_contract",
        "name": "Unknown contract",
        "functions": [stage],
    }
    with pytest.raises(RuntimeError, match="Failed to parse user function"):
        await client.save_user_function(function)
