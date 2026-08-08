"""
Binding-surface tests for transaction_id on the batch write path.

The Python client wraps the Rust core (ekodb_client). batch_insert(),
batch_update(), and batch_delete() each accept an optional transaction_id
keyword arg; when supplied the wrapper routes to the *_with_options Rust call,
which forwards it as the `transaction_id` URL query param so the batch is
staged into an MVCC transaction instead of committed immediately. The HTTP wire
behavior (the query param actually being sent) is covered by the Rust client's
test_batch_*_with_transaction_id_sends_query_param tests.

These tests assert the PyO3 signature accepts the transaction_id kwarg so the
binding surface stays in parity with the TS/Kotlin/Go/Rust clients, and that
existing call sites (no transaction_id) still bind unchanged (additive).
"""

import pytest
from ekodb_client import Client


def _client():
    return Client.new(
        "http://localhost:8080", "test-api-key", should_retry=False, timeout_secs=30
    )


class TestBatchTransactionId:
    # These assert the PyO3 signature binding only — they call the method WITHOUT
    # awaiting, so no network I/O happens and the suite can't time out or flake on
    # a stray service. PyO3 raises TypeError for an unknown kwarg during argument
    # binding, before the async body runs; the real kwargs must get past that
    # binding (called outside an event loop, the async bridge then raises
    # RuntimeError("no running event loop") — never TypeError — and makes no
    # network call). A bogus-kwarg control proves the binding is actually strict,
    # so "no TypeError for the real kwargs" is meaningful and not vacuous.

    def test_batch_insert_accepts_transaction_id(self):
        client = _client()

        # Control: an unknown kwarg is rejected at signature binding.
        with pytest.raises(TypeError):
            client.batch_insert("users", [{"name": "A"}], not_a_real_kwarg=True)

        try:
            client.batch_insert("users", [{"name": "A"}], transaction_id="tx_123")
        except TypeError as exc:  # pragma: no cover - failure path
            pytest.fail(f"batch_insert() rejected transaction_id: {exc}")
        except Exception:
            pass  # non-TypeError (e.g. no running event loop) => kwargs bound

    def test_batch_update_accepts_transaction_id(self):
        client = _client()

        with pytest.raises(TypeError):
            client.batch_update(
                "users", [("id_1", {"name": "B"})], not_a_real_kwarg=True
            )

        try:
            client.batch_update(
                "users", [("id_1", {"name": "B"})], transaction_id="tx_123"
            )
        except TypeError as exc:  # pragma: no cover - failure path
            pytest.fail(f"batch_update() rejected transaction_id: {exc}")
        except Exception:
            pass  # non-TypeError (e.g. no running event loop) => kwargs bound

    def test_batch_delete_accepts_transaction_id(self):
        client = _client()

        with pytest.raises(TypeError):
            client.batch_delete("users", ["id_1"], not_a_real_kwarg=True)

        try:
            client.batch_delete("users", ["id_1"], transaction_id="tx_123")
        except TypeError as exc:  # pragma: no cover - failure path
            pytest.fail(f"batch_delete() rejected transaction_id: {exc}")
        except Exception:
            pass  # non-TypeError (e.g. no running event loop) => kwargs bound

    def test_batch_ops_still_bind_without_transaction_id(self):
        """Existing call sites (no transaction_id) must keep working — additive."""
        client = _client()
        for call in (
            lambda: client.batch_insert("users", [{"name": "A"}]),
            lambda: client.batch_update("users", [("id_1", {"name": "B"})]),
            lambda: client.batch_delete("users", ["id_1"]),
        ):
            try:
                call()
            except TypeError as exc:  # pragma: no cover - failure path
                pytest.fail(f"batch op rejected legacy call: {exc}")
            except Exception:
                pass  # non-TypeError => bound fine
