#!/usr/bin/env python3
"""JWT auth flow — registration + login as pure stored functions.

Demonstrates the canonical password-auth pattern using ekoDB stages:

    register flow:  BcryptHash → Insert
    login flow:     FindOne  → BcryptVerify → If(ok) { JwtSign + Return }
    verify flow:    JwtVerify → If(claims != null) { 200 } else { 401 }

Operator-owned secrets (the JWT signing key) flow through
``{{env.JWT_SECRET}}`` so they never enter the function definition or
the LLM context window.

Requires ekoDB >= 0.43.0.
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv

from ekodb_client import Client, Stage

env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def main() -> None:
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    # 1. Register: bcrypt-hash, insert.
    register = {
        "label": "py_users_register",
        "name": "Register user",
        "description": "Validate, bcrypt-hash, insert.",
        "parameters": {
            "email": {"required": True},
            "password": {"required": True},
        },
        "functions": [
            Stage.bcrypt_hash("{{password}}", "password_hash", cost=12),
            Stage.insert(
                "py_users",
                {"email": "{{email}}", "password_hash": "{{password_hash}}"},
            ),
        ],
    }
    try:
        await client.save_user_function(register)
        print("✓ py_users_register saved")
    except Exception as e:
        print(f"SaveUserFunction error: {e}")

    # 2. Login: find user, verify bcrypt, sign JWT on success.
    login = {
        "label": "py_users_login",
        "name": "Login user",
        "description": "Verify password, mint JWT.",
        "parameters": {
            "email": {"required": True},
            "password": {"required": True},
        },
        "functions": [
            {
                "type": "FindOne",
                "collection": "py_users",
                "key": "email",
                "value": "{{email}}",
            },
            Stage.bcrypt_verify("{{password}}", "password_hash", "password_ok"),
            Stage.if_condition(
                {
                    "type": "FieldEquals",
                    "value": {"field": "password_ok", "value": True},
                },
                then_functions=[
                    Stage.jwt_sign(
                        {"email": "{{email}}"},
                        "{{env.JWT_SECRET}}",
                        "token",
                        algorithm="HS256",
                        expires_in_secs=3600,
                    ),
                    Stage.return_response({"ok": True, "token": "{{token}}"}, 200),
                ],
                else_functions=[Stage.return_response({"ok": False}, 401)],
            ),
        ],
    }
    try:
        await client.save_user_function(login)
        print("✓ py_users_login saved")
    except Exception as e:
        print(f"SaveUserFunction error: {e}")

    # 3. Verify a JWT — fail-closed when claims is null.
    verify = {
        "label": "py_users_verify_token",
        "name": "Verify JWT token",
        "parameters": {"token": {"required": True}},
        "functions": [
            # Synthetic record so JwtVerify has working_data[0] to read off.
            Stage.insert(
                "_inflight_jwt_check",
                {"token": "{{token}}"},
                bypass_ripple=True,
                ttl=60,
            ),
            Stage.jwt_verify(
                "token", "{{env.JWT_SECRET}}", "claims", algorithm="HS256"
            ),
            Stage.if_condition(
                {"type": "FieldEquals", "value": {"field": "claims", "value": None}},
                then_functions=[Stage.return_response({"ok": False}, 401)],
                else_functions=[Stage.return_response({"ok": True}, 200)],
            ),
        ],
    }
    try:
        await client.save_user_function(verify)
        print("✓ py_users_verify_token saved")
    except Exception as e:
        print(f"SaveUserFunction error: {e}")

    print("\n=== Auth flow defined as pure stored functions ===")
    print("Call them like:")
    print(
        '  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }'
    )
    print(
        '  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }'
    )
    print('  POST /api/functions/py_users_verify_token { "token": "<jwt>" }')

    for label in ("py_users_register", "py_users_login", "py_users_verify_token"):
        try:
            await client.delete_user_function(label)
        except Exception:
            pass
    print("\n✓ Cleaned up demo functions")


if __name__ == "__main__":
    asyncio.run(main())
