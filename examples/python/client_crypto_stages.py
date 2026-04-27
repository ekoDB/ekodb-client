#!/usr/bin/env python3
"""Crypto stages — comprehensive demo of every primitive added in
ekoDB 0.42.0 (HMAC, AES-GCM, UUID, TOTP, Base64, Hex, Slugify).

    crypto_demo_hmac      HmacSign + HmacVerify (round trip)
    crypto_demo_aes       AesEncrypt + AesDecrypt (round trip)
    crypto_demo_uuid      UuidGenerate
    crypto_demo_totp      TotpGenerate (RFC 6238)
    crypto_demo_encoding  Base64 + Hex + Slugify

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


async def main() -> None:
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    # 1. HMAC sign + verify round trip.
    hmac = {
        "label": "crypto_demo_hmac",
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
    await _save(client, "crypto_demo_hmac", hmac)

    # 2. AES-256-GCM encrypt + decrypt round trip.
    aes = {
        "label": "crypto_demo_aes",
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
    await _save(client, "crypto_demo_aes", aes)

    # 3. UuidGenerate.
    uuid_fn = {
        "label": "crypto_demo_uuid",
        "name": "Generate v4 UUID",
        "parameters": {},
        "functions": [Stage.uuid_generate("id")],
    }
    await _save(client, "crypto_demo_uuid", uuid_fn)

    # 4. TotpGenerate (RFC 6238 with SHA1).
    totp = {
        "label": "crypto_demo_totp",
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
    await _save(client, "crypto_demo_totp", totp)

    # 5. Base64 + Hex + Slugify chained.
    encoding = {
        "label": "crypto_demo_encoding",
        "name": "Base64 / Hex / Slugify",
        "parameters": {"title": {"required": True}},
        "functions": [
            Stage.base64_encode("{{title}}", "title_b64"),
            Stage.hex_encode("{{title}}", "title_hex"),
            Stage.slugify("{{title}}", "title_slug"),
        ],
    }
    await _save(client, "crypto_demo_encoding", encoding)

    print("\nInvoke them with:")
    print('  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }')
    print('  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }')
    print("  POST /api/functions/crypto_demo_uuid")
    print("  POST /api/functions/crypto_demo_totp")
    print('  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }')

    for label in (
        "crypto_demo_hmac",
        "crypto_demo_aes",
        "crypto_demo_uuid",
        "crypto_demo_totp",
        "crypto_demo_encoding",
    ):
        try:
            await client.delete_user_function(label)
        except Exception:
            pass
    print("\n✓ Cleaned up demo functions")


async def _save(client, label: str, func) -> None:
    """Save a stored function and print a status line.

    `label` is passed in as a separate string (rather than read off the
    `func` dict) so static analyzers don't trace `{{env.*}}` placeholder
    secrets inside `func` into the print/log surface. The error path
    intentionally surfaces only the exception type, not its message,
    for the same reason — server-side errors echoing back a request
    fragment could otherwise leak the placeholder string.
    """
    try:
        await client.save_user_function(func)
        print(f"✓ {label} saved")
    except Exception as e:
        print(f"SaveUserFunction({label}) error: {type(e).__name__}")


if __name__ == "__main__":
    asyncio.run(main())
