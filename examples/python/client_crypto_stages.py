#!/usr/bin/env python3
"""Crypto stages — comprehensive demo of every primitive added in
ekoDB 0.42.0 (HMAC, AES-GCM, UUID, TOTP, Base64, Hex, Slugify).

    crypto_demo_hmac_py      HmacSign + HmacVerify (round trip)
    crypto_demo_aes_py       AesEncrypt + AesDecrypt (round trip)
    crypto_demo_uuid_py      UuidGenerate
    crypto_demo_totp_py      TotpGenerate (RFC 6238)
    crypto_demo_encoding_py  Base64 + Hex + Slugify

Operator-owned secrets flow through ``{{env.NAME}}`` so they never
land in the function definition stored on disk.

Requires ekoDB >= 0.42.0.
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
FUNCTION_LABELS = (
    "crypto_demo_hmac_py",
    "crypto_demo_aes_py",
    "crypto_demo_uuid_py",
    "crypto_demo_totp_py",
    "crypto_demo_encoding_py",
)


async def main() -> None:
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")
    primary_error = None
    try:
        await _run(client)
    except Exception as error:
        primary_error = error

    cleanup_errors = []
    for label in FUNCTION_LABELS:
        try:
            await client.delete_user_function(label)
        except Exception as error:
            if not _is_not_found_error(error):
                cleanup_errors.append(RuntimeError(f"{label}: {error}"))

    if primary_error is not None:
        if cleanup_errors:
            cleanup_summary = "; ".join(str(error) for error in cleanup_errors)
            raise RuntimeError(
                f"{primary_error}; cleanup also failed: {cleanup_summary}"
            ) from primary_error
        raise primary_error
    if cleanup_errors:
        cleanup_summary = "; ".join(str(error) for error in cleanup_errors)
        raise RuntimeError(f"Crypto stages cleanup failed: {cleanup_summary}")
    print("\n✓ Cleaned up demo functions")


async def _run(client) -> None:

    # 1. HMAC sign + verify round trip.
    hmac = {
        "label": FUNCTION_LABELS[0],
        "name": "HMAC sign + verify",
        "parameters": {"payload": {"required": True}},
        "functions": [
            Stage.hmac_sign(
                "{{payload}}",
                "{{env.HMAC_KEY}}",
                "mac",
                algorithm="sha256",
                encoding="hex",
            ),
            Stage.hmac_verify(
                "{{payload}}",
                "{{mac}}",
                "{{env.HMAC_KEY}}",
                "verified",
                algorithm="sha256",
                encoding="hex",
            ),
        ],
    }
    await _save(client, FUNCTION_LABELS[0], hmac)

    # 2. AES-256-GCM encrypt + decrypt round trip.
    aes = {
        "label": FUNCTION_LABELS[1],
        "name": "AES encrypt + decrypt",
        "parameters": {"plaintext": {"required": True}},
        "functions": [
            Stage.aes_encrypt(
                "{{plaintext}}", "{{env.DATA_KEY}}", "envelope", key_encoding="hex"
            ),
            Stage.aes_decrypt(
                "envelope", "{{env.DATA_KEY}}", "recovered", key_encoding="hex"
            ),
        ],
    }
    await _save(client, FUNCTION_LABELS[1], aes)

    # 3. UuidGenerate.
    uuid_fn = {
        "label": FUNCTION_LABELS[2],
        "name": "Generate v4 UUID",
        "parameters": {},
        "functions": [Stage.uuid_generate("id")],
    }
    await _save(client, FUNCTION_LABELS[2], uuid_fn)

    # 4. TotpGenerate (RFC 6238 with SHA1).
    totp = {
        "label": FUNCTION_LABELS[3],
        "name": "Generate TOTP code",
        "parameters": {},
        "functions": [
            Stage.totp_generate(
                "{{env.TOTP_SECRET}}",
                "code",
                digits=6,
                period=30,
                algorithm="sha1",
            )
        ],
    }
    await _save(client, FUNCTION_LABELS[3], totp)

    # 5. Base64 + Hex + Slugify chained.
    encoding = {
        "label": FUNCTION_LABELS[4],
        "name": "Base64 / Hex / Slugify",
        "parameters": {"title": {"required": True}},
        "functions": [
            Stage.base64_encode("{{title}}", "title_b64"),
            Stage.hex_encode("{{title}}", "title_hex"),
            Stage.slugify("{{title}}", "title_slug"),
        ],
    }
    await _save(client, FUNCTION_LABELS[4], encoding)

    print("\nInvoke them with:")
    print(f'  POST /api/functions/{FUNCTION_LABELS[0]} {{ "payload": "hi" }}')
    print(f'  POST /api/functions/{FUNCTION_LABELS[1]} {{ "plaintext": "secret" }}')
    print(f"  POST /api/functions/{FUNCTION_LABELS[2]}")
    print(f"  POST /api/functions/{FUNCTION_LABELS[3]}")
    print(f'  POST /api/functions/{FUNCTION_LABELS[4]} {{ "title": "Héllo World" }}')


def _is_already_exists_error(err):
    """Detect the server's 409 'function already exists' response."""
    msg = str(err)
    return "409" in msg or "already exists" in msg


def _is_not_found_error(err):
    """Accept both HTTP 404 and the server's HTTP 400 missing-label response."""
    msg = str(err).lower()
    return "status 404" in msg or "not found" in msg


async def _save(client, label: str, func) -> None:
    """Save a stored function and print a status line.

    `label` is passed in as a separate string (rather than read off the
    `func` dict) so static analyzers don't trace `{{env.*}}` placeholder
    secrets inside `func` into the print/log surface. The error path
    intentionally surfaces only the exception type, not its message,
    for the same reason — server-side errors echoing back a request
    fragment could otherwise leak the placeholder string.

    On HTTP 409 (label already exists) the definition is updated via PUT
    so the example is idempotent across runs.
    """
    try:
        await client.save_user_function(func)
        print(f"✓ {label} saved")
    except Exception as e:
        if not _is_already_exists_error(e):
            raise RuntimeError(
                f"SaveUserFunction({label}) failed: {type(e).__name__}"
            ) from e
        await client.update_user_function(label, func)
        print(f"ℹ️  Function '{label}' already existed — updated instead")


if __name__ == "__main__":
    asyncio.run(main())
