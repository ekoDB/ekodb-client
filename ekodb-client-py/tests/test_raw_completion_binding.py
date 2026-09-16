"""Binding-surface tests for optional raw-completion streaming arguments."""

import pytest
from ekodb_client import Client


def _client():
    return Client.new(
        "http://localhost:8080", "test-api-key", should_retry=False, timeout_secs=1
    )


@pytest.mark.parametrize(
    ("method_name", "args"),
    [
        ("raw_completion_stream", ("system", "message")),
        ("raw_completion_stream_with_progress", ("system", "message", lambda _: None)),
    ],
)
def test_stream_methods_accept_only_required_arguments(method_name, args):
    method = getattr(_client(), method_name)
    with pytest.raises(TypeError):
        method(*args, not_a_real_kwarg=True)

    try:
        method(*args)
    except TypeError as exc:  # pragma: no cover - failure path
        pytest.fail(f"{method_name} rejected its documented optional arguments: {exc}")
    except Exception:
        pass


def test_raw_completion_stream_accepts_optional_keywords():
    try:
        _client().raw_completion_stream(
            "system", "message", provider="openai", model="gpt-4o-mini", max_tokens=8
        )
    except TypeError as exc:  # pragma: no cover - failure path
        pytest.fail(f"raw_completion_stream rejected optional keywords: {exc}")
    except Exception:
        pass
