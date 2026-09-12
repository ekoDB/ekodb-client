#!/usr/bin/env python3
"""Fail when a public client stage model drifts from the generated fixture."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FIXTURE = ROOT / "test-fixtures/function-stage-contract.json"


def strip_comments(source: str) -> str:
    source = re.sub(r"/\*.*?\*/", "", source, flags=re.DOTALL)
    return re.sub(r"//.*", "", source)


def matching(source: str, opening: int, left: str, right: str) -> int:
    depth = 0
    quote: str | None = None
    escaped = False
    for index in range(opening, len(source)):
        char = source[index]
        if quote:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == quote:
                quote = None
            continue
        if char in {'"', "'"}:
            quote = char
        elif char == left:
            depth += 1
        elif char == right:
            depth -= 1
            if depth == 0:
                return index
    raise ValueError(f"unclosed {left}{right} block")


def rust_contract(path: Path) -> dict[str, set[str]]:
    source = strip_comments(path.read_text())
    match = re.search(r"pub\s+enum\s+Function\s*\{", source)
    if match is None:
        raise ValueError(f"Function enum not found in {path}")
    opening = source.index("{", match.start())
    body = source[opening + 1 : matching(source, opening, "{", "}")]
    result: dict[str, set[str]] = {}
    cursor = 0
    while True:
        variant = re.search(r"(?m)^\s{4}([A-Z][A-Za-z0-9_]*)\s*\{", body[cursor:])
        if variant is None:
            return result
        name = variant.group(1)
        variant_open = cursor + variant.end() - 1
        variant_close = matching(body, variant_open, "{", "}")
        variant_body = re.sub(
            r"#\[[^\]]+\]", "", body[variant_open + 1 : variant_close]
        )
        result[name] = set(
            re.findall(
                r"(?:^|,)\s*([a-z][A-Za-z0-9_]*)\s*:(?!:)",
                variant_body,
                flags=re.MULTILINE,
            )
        )
        cursor = variant_close + 1


def top_level_properties(body: str) -> set[str]:
    properties: set[str] = set()
    for match in re.finditer(r"\b([a-z][A-Za-z0-9_]*)\??\s*:", body):
        prefix = body[: match.start()]
        if prefix.count("{") == prefix.count("}"):
            properties.add(match.group(1))
    return properties


def typescript_contract(path: Path) -> dict[str, set[str]]:
    source = strip_comments(path.read_text())
    start = source.index("export type FunctionStageConfig =")
    end = source.index("export interface ChatMessage", start)
    union = source[start:end]
    result: dict[str, set[str]] = {}
    cursor = 0
    while True:
        opening = union.find("{", cursor)
        if opening < 0:
            return result
        closing = matching(union, opening, "{", "}")
        body = union[opening + 1 : closing]
        variant = re.search(r'\btype\s*:\s*"([A-Za-z0-9_]+)"', body)
        if variant:
            fields = top_level_properties(body)
            fields.discard("type")
            result[variant.group(1)] = fields
        cursor = closing + 1


def kotlin_contract(path: Path) -> dict[str, set[str]]:
    source = strip_comments(path.read_text())
    result: dict[str, set[str]] = {}
    pattern = re.compile(r'@SerialName\("([A-Za-z0-9_]+)"\)\s*data class\s+\w+\s*\(')
    for variant in pattern.finditer(source):
        opening = variant.end() - 1
        closing = matching(source, opening, "(", ")")
        body = source[opening + 1 : closing]
        result[variant.group(1)] = set(
            re.findall(r"\bval\s+([a-z][A-Za-z0-9_]*)\s*:", body)
        )
    return result


def compare(
    name: str, actual: dict[str, set[str]], expected: dict[str, set[str]], floor: int
) -> None:
    if len(actual) < floor:
        raise SystemExit(
            f"{name}: extracted {len(actual)} variants, below coverage floor {floor}"
        )
    if actual == expected:
        return

    missing = sorted(expected.keys() - actual.keys())
    invented = sorted(actual.keys() - expected.keys())
    mismatched = {
        variant: {
            "missing": sorted(expected[variant] - actual[variant]),
            "invented": sorted(actual[variant] - expected[variant]),
        }
        for variant in sorted(expected.keys() & actual.keys())
        if expected[variant] != actual[variant]
    }
    raise SystemExit(
        f"{name} function-stage parity failed:\n"
        f"  missing variants: {missing}\n"
        f"  invented variants: {invented}\n"
        f"  field mismatches: {json.dumps(mismatched, sort_keys=True)}"
    )


def check_query_expression_docs() -> None:
    checks = {
        ROOT / "ekodb_client/README.md": (
            r'ws\.query\([\s\S]*?"type"\s*:\s*"Condition"[\s\S]*?"content"\s*:',
            r'ws\.query\([^;\n]*Some\(json!\(\{\s*"field"\s*:',
        ),
        ROOT / "ekodb-client-py/README.md": (
            r'ws_query\([\s\S]*?filter\s*=\s*\{[\s\S]*?"type"\s*:\s*"Condition"[\s\S]*?"content"\s*:',
            r'ws_query\([^\n]*filter\s*=\s*\{\s*"field"\s*:',
        ),
    }
    for path, (required, rejected) in checks.items():
        source = path.read_text()
        if not re.search(required, source):
            raise SystemExit(f"{path}: tagged WebSocket QueryExpression example missing")
        if re.search(rejected, source):
            raise SystemExit(f"{path}: flat WebSocket filter example is invalid")


def main() -> None:
    fixture = json.loads(FIXTURE.read_text())
    floor = fixture["coverage_floor"]
    variants = fixture["variants"]
    if fixture["variant_count"] != len(variants) or len(variants) < floor:
        raise SystemExit(
            "function-stage fixture is empty, truncated, or below its coverage floor"
        )
    expected = {case["name"]: set(case["fields"]) for case in variants}

    compare(
        "Rust/Python",
        rust_contract(ROOT / "ekodb_client/src/functions.rs"),
        expected,
        floor,
    )
    compare(
        "TypeScript",
        typescript_contract(ROOT / "ekodb-client-ts/src/functions.ts"),
        expected,
        floor,
    )
    compare(
        "Kotlin",
        kotlin_contract(
            ROOT
            / "ekodb-client-kt/src/main/kotlin/io/ekodb/client/functions/FunctionStages.kt"
        ),
        expected,
        floor,
    )
    check_query_expression_docs()
    print(f"function-stage parity OK: {len(expected)} variants")


if __name__ == "__main__":
    main()
