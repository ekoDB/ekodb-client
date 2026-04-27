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
# Crypto primitives: BcryptHash, BcryptVerify, RandomToken (ekoDB >= 0.41.0)
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


# ---------------------------------------------------------------------------
# JWT primitives: JwtSign, JwtVerify (ekoDB >= 0.43.0)
# ---------------------------------------------------------------------------


def test_jwt_sign_with_claims_expiry_and_algorithm():
    stage = Stage.jwt_sign(
        {"sub": "{{user_id}}", "role": "admin"},
        "{{env.JWT_SECRET}}",
        "token",
        expires_in_secs=3600,
        algorithm="HS256",
    )
    assert stage["type"] == "JwtSign"
    assert stage["claims"] == {"sub": "{{user_id}}", "role": "admin"}
    assert stage["secret"] == "{{env.JWT_SECRET}}"
    assert stage["expires_in_secs"] == 3600
    assert stage["algorithm"] == "HS256"
    assert stage["output_field"] == "token"


def test_jwt_sign_omits_optional_fields_when_none():
    stage = Stage.jwt_sign({"sub": "u"}, "{{env.JWT_SECRET}}", "t")
    assert "algorithm" not in stage
    assert "expires_in_secs" not in stage


def test_jwt_verify_wires_all_fields():
    stage = Stage.jwt_verify(
        "auth_token",
        "{{env.JWT_SECRET}}",
        "claims",
        algorithm="HS512",
    )
    assert stage["type"] == "JwtVerify"
    assert stage["token_field"] == "auth_token"
    assert stage["secret"] == "{{env.JWT_SECRET}}"
    assert stage["algorithm"] == "HS512"
    assert stage["output_field"] == "claims"


def test_jwt_stages_json_round_trip():
    import json

    stages = [
        Stage.jwt_sign(
            {"sub": "user-1"},
            "{{env.JWT_SECRET}}",
            "token",
            expires_in_secs=900,
            algorithm="HS256",
        ),
        Stage.jwt_verify(
            "token",
            "{{env.JWT_SECRET}}",
            "claims",
            algorithm="HS256",
        ),
    ]
    for stage in stages:
        wire = json.loads(json.dumps(stage))
        assert wire == stage, f"JSON round-trip changed {stage}"
        assert wire["type"] in ("JwtSign", "JwtVerify")


# ---------------------------------------------------------------------------
# EmailSend (ekoDB >= 0.43.0)
# ---------------------------------------------------------------------------


def test_email_send_with_full_payload():
    stage = Stage.email_send(
        to="alice@example.com",
        subject="Welcome",
        body="<p>Hi Alice</p>",
        from_="bot@example.com",
        api_key="{{env.SENDGRID_API_KEY}}",
        reply_to="support@example.com",
        provider="sendgrid",
        html=True,
        output_field="send_result",
    )
    assert stage["type"] == "EmailSend"
    assert stage["to"] == "alice@example.com"
    assert stage["subject"] == "Welcome"
    assert stage["from"] == "bot@example.com"
    assert stage["reply_to"] == "support@example.com"
    assert stage["api_key"] == "{{env.SENDGRID_API_KEY}}"
    assert stage["provider"] == "sendgrid"
    assert stage["html"] is True
    assert stage["output_field"] == "send_result"


def test_email_send_omits_optional_fields_when_none():
    stage = Stage.email_send(
        to="x@example.com",
        subject="s",
        body="b",
        from_="f@example.com",
        api_key="k",
    )
    for k in ("reply_to", "provider", "html", "output_field"):
        assert k not in stage, f"{k} must be omitted"


def test_email_send_json_round_trip():
    import json

    stage = Stage.email_send(
        to="u@example.com",
        subject="Hi",
        body="<p>Hi</p>",
        from_="f@example.com",
        api_key="{{env.SENDGRID_API_KEY}}",
        provider="sendgrid",
        html=True,
    )
    wire = json.loads(json.dumps(stage))
    assert wire == stage


# ---------------------------------------------------------------------------
# Error Handling & Control Flow: TryCatch, Parallel, Sleep
# ---------------------------------------------------------------------------


def test_try_catch_basic():
    stage = Stage.try_catch(
        try_functions=[Stage.http_request("https://api.example.com")],
        catch_functions=[Stage.insert("errors", {"msg": "failed"})],
        output_error_field="api_error",
    )
    assert stage["type"] == "TryCatch"
    assert len(stage["try_functions"]) == 1
    assert len(stage["catch_functions"]) == 1
    assert stage["output_error_field"] == "api_error"


def test_try_catch_omits_output_error_field_when_none():
    stage = Stage.try_catch(
        try_functions=[Stage.find_all("users")],
        catch_functions=[Stage.insert("errors", {"msg": "fail"})],
    )
    assert "output_error_field" not in stage


def test_parallel_basic():
    stage = Stage.parallel(
        functions=[Stage.find_all("users"), Stage.find_all("orders")],
        wait_for_all=True,
    )
    assert stage["type"] == "Parallel"
    assert len(stage["functions"]) == 2
    assert stage["wait_for_all"] is True


def test_parallel_defaults_wait_for_all_true():
    stage = Stage.parallel(functions=[Stage.find_all("users")])
    assert stage["wait_for_all"] is True


def test_parallel_race_mode():
    stage = Stage.parallel(
        functions=[Stage.find_all("a"), Stage.find_all("b")],
        wait_for_all=False,
    )
    assert stage["wait_for_all"] is False


def test_sleep_numeric():
    stage = Stage.sleep(1000)
    assert stage["type"] == "Sleep"
    assert stage["duration_ms"] == 1000


def test_sleep_placeholder():
    stage = Stage.sleep("{{delay}}")
    assert stage["duration_ms"] == "{{delay}}"


# ---------------------------------------------------------------------------
# Response Formatting: Return
# ---------------------------------------------------------------------------


def test_return_response_with_status():
    stage = Stage.return_response(
        fields={"message": "ok", "user_id": "{{id}}"},
        status_code=201,
    )
    assert stage["type"] == "Return"
    assert stage["fields"]["message"] == "ok"
    assert stage["status_code"] == 201


def test_return_response_omits_status_code_when_none():
    stage = Stage.return_response(fields={"success": True})
    assert "status_code" not in stage


# ---------------------------------------------------------------------------
# Data Validation: Validate
# ---------------------------------------------------------------------------


def test_validate_basic():
    schema = {"type": "object", "required": ["name"]}
    stage = Stage.validate(
        schema=schema,
        data_field="{{input}}",
        on_error=[Stage.return_response({"error": "invalid"}, 400)],
    )
    assert stage["type"] == "Validate"
    assert stage["schema"] == schema
    assert stage["data_field"] == "{{input}}"
    assert len(stage["on_error"]) == 1


def test_validate_omits_on_error_when_none():
    stage = Stage.validate(schema={"type": "object"}, data_field="record")
    assert "on_error" not in stage


# ---------------------------------------------------------------------------
# New stages JSON round-trip
# ---------------------------------------------------------------------------


def test_new_stages_json_round_trip():
    import json

    stages = [
        Stage.try_catch([Stage.find_all("a")], [Stage.find_all("b")], "err"),
        Stage.parallel([Stage.find_all("a")], True),
        Stage.sleep(500),
        Stage.return_response({"ok": True}, 200),
        Stage.validate({"type": "object"}, "data"),
    ]
    for stage in stages:
        wire = json.loads(json.dumps(stage))
        assert wire == stage, f"JSON round-trip changed {stage}"
        assert wire["type"] in ("TryCatch", "Parallel", "Sleep", "Return", "Validate")


# ---------------------------------------------------------------------------
# Crypto + concurrency stages
# ---------------------------------------------------------------------------


def test_hmac_sign_and_verify_build_correctly():
    sign = Stage.hmac_sign(
        "{{p}}", "{{env.K}}", "mac", algorithm="sha256", encoding="hex"
    )
    assert sign == {
        "type": "HmacSign",
        "input": "{{p}}",
        "secret": "{{env.K}}",
        "output_field": "mac",
        "algorithm": "sha256",
        "encoding": "hex",
    }
    verify = Stage.hmac_verify("{{p}}", "{{m}}", "{{env.K}}", "ok")
    assert verify["type"] == "HmacVerify"
    assert "algorithm" not in verify
    assert "encoding" not in verify


def test_aes_uuid_totp_stages_build():
    enc = Stage.aes_encrypt("{{plain}}", "{{env.K}}", "envelope", key_encoding="hex")
    assert enc["type"] == "AesEncrypt"
    assert enc["key_encoding"] == "hex"

    dec = Stage.aes_decrypt("envelope", "{{env.K}}", "plain")
    assert dec["type"] == "AesDecrypt"
    assert "key_encoding" not in dec

    assert Stage.uuid_generate("id") == {"type": "UuidGenerate", "output_field": "id"}

    totp = Stage.totp_generate(
        "{{env.T}}", "code", digits=6, period=30, algorithm="sha1"
    )
    assert totp["type"] == "TotpGenerate"
    assert totp["digits"] == 6
    assert totp["period"] == 30


def test_base64_hex_slugify_stages_build():
    assert Stage.base64_encode("{{x}}", "b", url_safe=True) == {
        "type": "Base64Encode",
        "input": "{{x}}",
        "output_field": "b",
        "url_safe": True,
    }
    assert Stage.base64_decode("{{b}}", "x") == {
        "type": "Base64Decode",
        "input": "{{b}}",
        "output_field": "x",
    }
    assert Stage.hex_encode("{{x}}", "h") == {
        "type": "HexEncode",
        "input": "{{x}}",
        "output_field": "h",
    }
    assert Stage.hex_decode("{{h}}", "x") == {
        "type": "HexDecode",
        "input": "{{h}}",
        "output_field": "x",
    }
    assert Stage.slugify("{{title}}", "slug") == {
        "type": "Slugify",
        "input": "{{title}}",
        "output_field": "slug",
    }


def test_concurrency_stages_build():
    assert Stage.idempotency_claim("{{ikey}}", 60, "claim") == {
        "type": "IdempotencyClaim",
        "key": "{{ikey}}",
        "ttl_secs": 60,
        "output_field": "claim",
    }
    rl = Stage.rate_limit("{{user}}", 100, 60, "rl", on_exceed="skip")
    assert rl["type"] == "RateLimit"
    assert rl["on_exceed"] == "skip"
    assert Stage.lock_acquire("{{r}}", 30, "lock") == {
        "type": "LockAcquire",
        "key": "{{r}}",
        "ttl_secs": 30,
        "output_field": "lock",
    }
    assert Stage.lock_release("{{r}}", "{{lock.token}}", "rel") == {
        "type": "LockRelease",
        "key": "{{r}}",
        "token": "{{lock.token}}",
        "output_field": "rel",
    }


def test_crypto_concurrency_stages_json_round_trip():
    import json

    stages = [
        Stage.hmac_sign("a", "k", "m", algorithm="sha256"),
        Stage.aes_encrypt("p", "k", "e", key_encoding="hex"),
        Stage.uuid_generate("id"),
        Stage.totp_generate("s", "c", digits=6),
        Stage.base64_encode("x", "b"),
        Stage.idempotency_claim("k", 60, "c"),
        Stage.rate_limit("k", 5, 60, "r"),
        Stage.lock_acquire("r", 60, "l"),
    ]
    for stage in stages:
        wire = json.loads(json.dumps(stage))
        assert wire == stage
