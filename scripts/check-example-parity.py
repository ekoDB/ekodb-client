#!/usr/bin/env python3
"""Enforce semantic example parity across the five SDK implementations."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def snake_case(name: str) -> str:
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    return name.lower()


def scenarios(directory: str, pattern: str, prefix: str, suffix: str) -> set[str]:
    result = set()
    for path in (ROOT / directory).glob(pattern):
        name = path.name.removeprefix(prefix).removesuffix(suffix)
        result.add(snake_case(name))
    return result


language_scenarios = {
    "rust": scenarios("examples/rust/examples", "client_*.rs", "client_", ".rs"),
    "python": scenarios("examples/python", "client_*.py", "client_", ".py"),
    "go": scenarios("examples/go", "client_*.go", "client_", ".go"),
    "typescript": scenarios("examples/typescript", "client_*.ts", "client_", ".ts"),
    "kotlin": scenarios("examples/kotlin/examples", "Client*.kt", "Client", ".kt"),
}

# Kotlin keeps these two historical filenames for the shared schema scenario
# and the auxiliary projection demo.
language_scenarios["kotlin"].discard("projection")
if "schema_management" in language_scenarios["kotlin"]:
    language_scenarios["kotlin"].remove("schema_management")
    language_scenarios["kotlin"].add("schema")

reference = language_scenarios["typescript"]
errors: list[str] = []
for language, actual in language_scenarios.items():
    missing = sorted(reference - actual)
    extra = sorted(actual - reference)
    if missing or extra:
        errors.append(
            f"{language}: missing={missing or '[]'} extra={extra or '[]'}"
        )

auxiliary = {
    "rust": ["examples/rust/examples/bypass_ripple_example.rs", "examples/rust/examples/projection_example.rs"],
    "python": ["examples/python/bypass_ripple_example.py", "examples/python/projection_example.py"],
    "go": ["examples/go/bypass_ripple_example.go", "examples/go/projection_example.go"],
    "typescript": ["examples/typescript/bypass_ripple_example.ts", "examples/typescript/projection_example.ts"],
    "kotlin": ["examples/kotlin/examples/BypassRippleExample.kt", "examples/kotlin/examples/ClientProjection.kt"],
}
for language, paths in auxiliary.items():
    absent = [path for path in paths if not (ROOT / path).is_file()]
    if absent:
        errors.append(f"{language}: missing auxiliary examples {absent}")

# These are public synthetic example defaults, not configured credentials. Keep
# the shared fallback registerable under the documented API-key contract.
default_key = re.compile(r"[\"'](a-test-api-key-from-ekodb[^\"']*)[\"']")
for directory, pattern in [
    ("examples/rust/examples", "*.rs"),
    ("examples/python", "*.py"),
    ("examples/go", "*.go"),
    ("examples/typescript", "*.ts"),
    ("examples/javascript", "*.js"),
    ("examples/kotlin/examples", "*.kt"),
]:
    for path in (ROOT / directory).glob(pattern):
        for key in default_key.findall(path.read_text()):
            if len(key) < 32 or len(set(key)) < 8:
                errors.append(f"{path.relative_to(ROOT)}: example default API key does not meet the registration minimum")

if errors:
    print("Client example parity check failed:", file=sys.stderr)
    for error in errors:
        print(f"  - {error}", file=sys.stderr)
    raise SystemExit(1)

print(
    "Client example parity OK: "
    f"{len(reference)} shared scenarios across Rust, Python, Go, TypeScript, and Kotlin; "
    "JavaScript executes the compiled TypeScript scenario set."
)
