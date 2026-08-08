"""Regression tests for the package __version__."""

from importlib.metadata import version

import ekodb_client


def test_version_matches_distribution_metadata():
    # __version__ must derive from the installed distribution metadata (set by
    # maturin from pyproject.toml / Cargo.toml), not a hand-maintained literal
    # that drifts. Previously it was hardcoded to "0.1.0" while the package was
    # 0.21.0.
    assert ekodb_client.__version__ == version("ekodb_client")


def test_version_is_not_the_stale_placeholder():
    assert ekodb_client.__version__ != "0.1.0"
