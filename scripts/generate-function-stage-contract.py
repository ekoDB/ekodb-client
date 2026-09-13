#!/usr/bin/env python3
"""Generate the public stored-function contract fixture from a Rust enum.

The source path is supplied explicitly so this utility is usable in any
checkout. The generated fixture is committed and consumed by every client
test suite; regenerate it whenever the authoritative enum changes.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


FILTER = {
    "type": "Condition",
    "content": {"field": "status", "operator": "Eq", "value": "active"},
}
NESTED_STAGE = {"type": "FindAll", "collection": "nested"}


def strip_comments(source: str) -> str:
    source = re.sub(r"/\*.*?\*/", "", source, flags=re.DOTALL)
    return re.sub(r"//.*", "", source)


def matching_brace(source: str, opening: int) -> int:
    depth = 0
    for index in range(opening, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
            if depth == 0:
                return index
    raise ValueError("unclosed enum or variant body")


def extract_contract(source: str) -> list[tuple[str, list[str]]]:
    clean = strip_comments(source)
    enum_match = re.search(r"pub\s+enum\s+Function\s*\{", clean)
    if enum_match is None:
        raise ValueError("could not find `pub enum Function`")
    enum_open = clean.index("{", enum_match.start())
    enum_close = matching_brace(clean, enum_open)
    body = clean[enum_open + 1 : enum_close]

    variants: list[tuple[str, list[str]]] = []
    cursor = 0
    while cursor < len(body):
        match = re.search(r"(?m)^\s{4}([A-Z][A-Za-z0-9_]*)\s*\{", body[cursor:])
        if match is None:
            break
        name = match.group(1)
        opening = cursor + match.end() - 1
        closing = matching_brace(body, opening)
        variant_body = re.sub(r"#\[[^\]]+\]", "", body[opening + 1 : closing])
        fields = re.findall(
            r"(?:^|,)\s*([a-z][A-Za-z0-9_]*)\s*:(?!:)",
            variant_body,
            flags=re.MULTILINE,
        )
        variants.append((name, fields))
        cursor = closing + 1

    if not variants:
        raise ValueError("Function enum extraction produced zero variants")
    return variants


def sample_value(variant: str, field: str) -> Any:
    if field == "filter":
        return FILTER
    if field == "sort":
        return [{"field": "created_at", "ascending": False}]
    if field == "query_vector":
        return [0.12345678901234568, 0.25]
    if field == "messages":
        return [{"role": "user", "content": "hello"}]
    if field in {
        "then_functions",
        "else_functions",
        "functions",
        "try_functions",
        "catch_functions",
        "on_error",
    }:
        if variant == "Group" and field == "functions":
            return [
                {"output_field": "total", "operation": "Sum", "input_field": "amount"}
            ]
        return [NESTED_STAGE]
    if field == "condition":
        return {"type": "HasRecords"}
    if field == "fields":
        if variant == "AddFields":
            return [
                {"field_name": "total", "expression": {"type": "Literal", "value": 1}}
            ]
        if variant == "Return":
            return {"ok": True, "result": "{{value}}"}
        return ["title", "status"]
    if field == "by_fields":
        return ["category"]
    if field == "records":
        return [{"name": "first"}, {"name": "second"}]
    if field == "record":
        return {"name": "sample"}
    if field == "updates":
        return {"status": "updated"}
    if field == "headers":
        return {"x-contract-test": "true"}
    if field == "body":
        return "message body" if variant == "EmailSend" else {"payload": True}
    if field == "params":
        return {"id": "sample-id"}
    if field == "claims":
        return {"sub": "sample-user"}
    if field == "schema":
        return {"type": "object", "required": ["name"]}
    if field == "record_ids":
        return ["id-1", "id-2"]
    if field in {
        "bypass_ripple",
        "include_expired",
        "exclude",
        "fuzzy",
        "html",
        "url_safe",
        "wait_for_all",
    }:
        return True
    if field in {
        "limit",
        "skip",
        "max_tokens",
        "timeout_seconds",
        "cost",
        "bytes",
        "expires_in_secs",
        "digits",
        "period",
        "skew",
        "ttl_secs",
        "window_secs",
        "status_code",
        "duration_ms",
    }:
        return {
            "skip": 2,
            "cost": 12,
            "bytes": 16,
            "digits": 6,
            "period": 30,
            "skew": 1,
            "status_code": 200,
        }.get(field, 7)
    if field == "threshold":
        return 0.75
    if field == "temperature":
        return 0.5
    if field == "ttl":
        return 60
    if field == "by":
        return 2
    if field == "value":
        return {"sample": True} if variant == "KvSet" else "sample-value"

    strings = {
        "collection": "items",
        "record_id": "sample-id",
        "key": "sample-key",
        "field": "sample_field",
        "field_name": "sample_field",
        "output_field": "result",
        "query_text": "sample query",
        "model": "sample-model",
        "url": "https://example.test/data",
        "method": "POST",
        "function_label": "nested_function",
        "name": "savepoint",
        "cache_key": "cache:sample",
        "pattern": "cache:.*",
        "plain": "{{password}}",
        "hash_field": "password_hash",
        "encoding": "hex",
        "secret": "{{env.CONTRACT_SECRET}}",
        "algorithm": "sha256" if variant not in {"JwtSign", "JwtVerify"} else "HS256",
        "token_field": "token",
        "to": "to@example.test",
        "subject": "subject",
        "from": "from@example.test",
        "reply_to": "reply@example.test",
        "api_key": "{{env.API_KEY}}",
        "provider": "sendgrid",
        "input": "sample input",
        "input_field": "source_text",
        "provided_mac": "deadbeef",
        "plaintext": "sample plaintext",
        "ciphertext_field": "ciphertext",
        "key_encoding": "hex",
        "code": "123456",
        "on_exceed": "fail",
        "token": "lock-token",
        "output_error_field": "error",
        "data_field": "input",
        "action": "increment",
    }
    if field in strings:
        return strings[field]
    raise ValueError(f"no sample value for {variant}.{field}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "source",
        type=Path,
        help="Rust source containing the authoritative Function enum",
    )
    parser.add_argument("output", type=Path, help="JSON fixture to write")
    args = parser.parse_args()

    contract = extract_contract(args.source.read_text())
    variants = []
    for name, fields in contract:
        stage = {"type": name}
        stage.update({field: sample_value(name, field) for field in fields})
        variants.append({"name": name, "fields": fields, "stage": stage})

    payload = {
        "schema_version": 1,
        "coverage_floor": 67,
        "variant_count": len(variants),
        "variants": variants,
    }
    if payload["variant_count"] < payload["coverage_floor"]:
        raise SystemExit(
            f"extracted {payload['variant_count']} variants; coverage floor is {payload['coverage_floor']}"
        )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(payload, indent=2) + "\n")


if __name__ == "__main__":
    main()
