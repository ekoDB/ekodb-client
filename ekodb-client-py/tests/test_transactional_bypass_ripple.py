"""
Binding-surface tests for bypass_ripple on the transactional read path.

The Python client wraps the Rust core (ekodb_client). Both find() and
find_by_id() accept bypass_ripple AND transaction_id keyword args; when both
are supplied the wrapper routes to the *_in_transaction Rust call and must
forward bypass_ripple (previously it was silently dropped). The HTTP wire
behavior (both query params sent together) is covered by the Rust client's
test_find_in_transaction_with_bypass_ripple /
test_find_by_id_in_transaction_with_bypass_ripple.

These tests assert the PyO3 signature accepts both kwargs together so the
binding surface stays in parity with the TS/Kotlin/Go clients.
"""

import pytest
from ekodb_client import Client


def _client():
    return Client.new(
        "http://localhost:8080", "test-api-key", should_retry=False, timeout_secs=30
    )


class TestTransactionalBypassRipple:
    # These assert the PyO3 signature binding only — they call the method WITHOUT
    # awaiting, so no network I/O happens and the suite can't time out or flake on
    # a stray service. PyO3 raises TypeError for an unknown kwarg during argument
    # binding, before the async body runs; the real kwargs must get past that
    # binding (called outside an event loop, the async bridge then raises
    # RuntimeError("no running event loop") — never TypeError — and makes no
    # network call). A bogus-kwarg control proves the binding is actually strict,
    # so "no TypeError for the real kwargs" is meaningful and not vacuous.

    def test_find_accepts_bypass_ripple_and_transaction_id_together(self):
        """find() must accept bypass_ripple and transaction_id together."""
        client = _client()

        # Control: an unknown kwarg is rejected at signature binding.
        with pytest.raises(TypeError):
            client.find("users", query=None, not_a_real_kwarg=True)

        # The real kwargs must bind (no TypeError); any other exception is fine.
        try:
            client.find(
                "users",
                query=None,
                bypass_ripple=True,
                transaction_id="tx_123",
            )
        except TypeError as exc:  # pragma: no cover - failure path
            pytest.fail(f"find() rejected bypass_ripple/transaction_id: {exc}")
        except Exception:
            pass  # non-TypeError (e.g. no running event loop) => kwargs bound

    def test_find_by_id_accepts_bypass_ripple_and_transaction_id_together(self):
        """find_by_id() must accept bypass_ripple and transaction_id together."""
        client = _client()

        with pytest.raises(TypeError):
            client.find_by_id("users", "user_123", not_a_real_kwarg=True)

        try:
            client.find_by_id(
                "users",
                "user_123",
                bypass_ripple=True,
                transaction_id="tx_123",
            )
        except TypeError as exc:  # pragma: no cover - failure path
            pytest.fail(f"find_by_id() rejected bypass_ripple/transaction_id: {exc}")
        except Exception:
            pass

    def test_find_methods_exposed(self):
        client = _client()
        assert callable(client.find)
        assert callable(client.find_by_id)
