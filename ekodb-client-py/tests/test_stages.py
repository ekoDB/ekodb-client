"""Unit tests for the stored-function Stage helpers + parameter_ref.

Feature parity with the TS, Go, Rust, and Kotlin clients' equivalent
helpers. Server-side behavior for structural parameter placeholders is
covered by the Rust integration tests in
`ekodb/ekodb_server/tests/function_parameters_tests.rs`.
"""

from ekodb_client import Stage, parameter_ref


# ---------------------------------------------------------------------------
# parameter_ref()
# ---------------------------------------------------------------------------


def test_parameter_ref_shape():
    assert parameter_ref("record") == {"type": "Parameter", "name": "record"}


def test_parameter_ref_preserves_arbitrary_name():
    assert parameter_ref("user_id")["name"] == "user_id"
    assert parameter_ref("updates")["name"] == "updates"


def test_stage_param_alias():
    assert Stage.param("x") == parameter_ref("x")


# ---------------------------------------------------------------------------
# Stage.insert with structural placeholder
# ---------------------------------------------------------------------------


def test_insert_accepts_whole_record_parameter():
    stage = Stage.insert("users", Stage.param("record"))
    assert stage["type"] == "Insert"
    assert stage["collection"] == "users"
    assert stage["record"] == {"type": "Parameter", "name": "record"}


def test_insert_accepts_per_field_placeholders():
    stage = Stage.insert(
        "items",
        {
            "label": "{{label}}",
            "parent_id": Stage.param("parent_id"),
            "kind": "item",
            "tags": Stage.param("tags"),
        },
    )
    assert stage["record"]["label"] == "{{label}}"
    assert stage["record"]["kind"] == "item"
    assert stage["record"]["parent_id"] == {
        "type": "Parameter",
        "name": "parent_id",
    }
    assert stage["record"]["tags"] == {"type": "Parameter", "name": "tags"}


# ---------------------------------------------------------------------------
# Stage.update_by_id with structural placeholder
# ---------------------------------------------------------------------------


def test_update_by_id_accepts_whole_updates_parameter():
    # Python client uses snake_case update_by_id.
    stage = Stage.update_by_id("items", "{{id}}", Stage.param("updates"))
    assert stage["type"] == "UpdateById"
    assert stage["collection"] == "items"
    assert stage["record_id"] == "{{id}}"
    assert stage["updates"] == {"type": "Parameter", "name": "updates"}


# ---------------------------------------------------------------------------
# Stage.update (filter-based) with structural filter + updates
# ---------------------------------------------------------------------------


def test_update_with_structural_filter_and_updates():
    filter_expr = {
        "type": "Condition",
        "content": {
            "field": "id",
            "operator": "Eq",
            "value": Stage.param("id"),
        },
    }
    stage = Stage.update("items", filter_expr, Stage.param("updates"))
    assert stage["type"] == "Update"
    assert stage["filter"]["content"]["value"] == {
        "type": "Parameter",
        "name": "id",
    }
    assert stage["updates"] == {"type": "Parameter", "name": "updates"}


# ---------------------------------------------------------------------------
# Stage.batch_insert with per-record Parameter placeholders
# ---------------------------------------------------------------------------


def test_batch_insert_accepts_per_record_structural_parameters():
    stage = Stage.batch_insert(
        "audit_log",
        [
            {
                "actor": Stage.param("user_id"),
                "at": "{{now}}",
                "message": "created",
            },
            {
                "actor": Stage.param("user_id"),
                "at": "{{now}}",
                "message": "initialized",
            },
        ],
    )
    assert stage["type"] == "BatchInsert"
    assert len(stage["records"]) == 2
    assert stage["records"][0]["actor"] == {
        "type": "Parameter",
        "name": "user_id",
    }
    assert stage["records"][0]["at"] == "{{now}}"
    assert stage["records"][1]["message"] == "initialized"


# ---------------------------------------------------------------------------
# JSON serialization — what actually goes on the wire
# ---------------------------------------------------------------------------


def test_insert_json_serialization():
    import json

    stage = Stage.insert("users", Stage.param("record"))
    wire = json.loads(json.dumps(stage))
    assert wire["type"] == "Insert"
    assert wire["collection"] == "users"
    assert wire["record"] == {"type": "Parameter", "name": "record"}


def test_update_by_id_json_serialization():
    import json

    stage = Stage.update_by_id("items", "{{id}}", Stage.param("updates"))
    wire = json.loads(json.dumps(stage))
    assert wire["type"] == "UpdateById"
    assert wire["record_id"] == "{{id}}"
    assert wire["updates"] == {"type": "Parameter", "name": "updates"}


# ---------------------------------------------------------------------------
# Crypto primitives: BcryptHash, BcryptVerify, RandomToken (ekoDB >= 0.42.0)
# ---------------------------------------------------------------------------


def test_bcrypt_hash_with_explicit_cost():
    stage = Stage.bcrypt_hash("{{password}}", "password_hash", cost=12)
    assert stage["type"] == "BcryptHash"
    assert stage["plain"] == "{{password}}"
    assert stage["cost"] == 12
    assert stage["output_field"] == "password_hash"


def test_bcrypt_hash_omits_cost_when_none():
    stage = Stage.bcrypt_hash("{{password}}", "pw_hash")
    assert "cost" not in stage


def test_bcrypt_verify_wires_all_fields():
    stage = Stage.bcrypt_verify("{{password}}", "password_hash", "valid")
    assert stage["type"] == "BcryptVerify"
    assert stage["plain"] == "{{password}}"
    assert stage["hash_field"] == "password_hash"
    assert stage["output_field"] == "valid"


def test_random_token_with_explicit_encoding():
    stage = Stage.random_token(32, "session_token", encoding="hex")
    assert stage["type"] == "RandomToken"
    assert stage["bytes"] == 32
    assert stage["encoding"] == "hex"
    assert stage["output_field"] == "session_token"


def test_random_token_omits_encoding_when_none():
    stage = Stage.random_token(16, "token")
    assert "encoding" not in stage


def test_random_token_accepts_base64_variants():
    a = Stage.random_token(16, "t", encoding="base64")
    b = Stage.random_token(16, "t", encoding="base64url")
    assert a["encoding"] == "base64"
    assert b["encoding"] == "base64url"


def test_crypto_stages_json_round_trip():
    import json

    stages = [
        Stage.bcrypt_hash("{{password}}", "password_hash", cost=12),
        Stage.bcrypt_verify("{{password}}", "password_hash", "valid"),
        Stage.random_token(32, "token", encoding="base64url"),
    ]
    for stage in stages:
        wire = json.loads(json.dumps(stage))
        assert wire == stage, f"JSON round-trip changed {stage}"
        assert wire["type"] in ("BcryptHash", "BcryptVerify", "RandomToken")
