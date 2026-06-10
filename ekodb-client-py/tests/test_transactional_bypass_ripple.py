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

import asyncio

import pytest
from ekodb_client import Client


def _client():
    return Client.new(
        "http://localhost:8080", "test-api-key", should_retry=False, timeout_secs=30
    )


class TestTransactionalBypassRipple:
    def test_find_accepts_bypass_ripple_and_transaction_id_together(self):
        """find() must accept both kwargs in one call without a TypeError on
        the PyO3 signature. The await fails on connection (no server), which
        proves the kwargs bound and routed past the signature check rather
        than being rejected as unexpected arguments."""

        async def run():
            client = _client()
            # If bypass_ripple/transaction_id were not accepted together, the
            # call would raise TypeError before any await. A connection error
            # (or any non-TypeError) means the binding accepted both kwargs.
            with pytest.raises(Exception) as exc_info:
                await client.find(
                    "users",
                    query=None,
                    bypass_ripple=True,
                    transaction_id="tx_123",
                )
            assert not isinstance(exc_info.value, TypeError)

        asyncio.run(run())

    def test_find_by_id_accepts_bypass_ripple_and_transaction_id_together(self):
        """find_by_id() must accept both kwargs in one call."""

        async def run():
            client = _client()
            with pytest.raises(Exception) as exc_info:
                await client.find_by_id(
                    "users",
                    "user_123",
                    bypass_ripple=True,
                    transaction_id="tx_123",
                )
            assert not isinstance(exc_info.value, TypeError)

        asyncio.run(run())

    def test_find_methods_exposed(self):
        client = _client()
        assert callable(client.find)
        assert callable(client.find_by_id)
