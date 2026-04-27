#!/usr/bin/env python3
"""Concurrency stages — IdempotencyClaim, RateLimit, LockAcquire/Release.

These three primitives wrap ekoDB's atomic KV layer with the right
semantics for common server-side patterns:

    IdempotencyClaim     replay protection with TTL
    RateLimit            fixed-window counter; "fail" or "skip" mode
    LockAcquire/Release  token-fenced distributed lock

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

    # 1. Idempotent payment — claim → branch on replay → do work.
    pay = {
        "label": "conc_demo_pay",
        "name": "Idempotent payment",
        "parameters": {
            "idempotency_key": {"required": True},
            "amount": {"required": True},
        },
        "functions": [
            Stage.idempotency_claim("{{idempotency_key}}", 86400, "claim"),
            Stage.if_condition(
                {
                    "type": "FieldEquals",
                    "value": {"field": "claim.claimed", "value": False},
                },
                then_functions=[
                    Stage.return_response(
                        {
                            "status": "replay",
                            "idempotency_key": "{{idempotency_key}}",
                        },
                        200,
                    )
                ],
                else_functions=[
                    Stage.insert(
                        "charges",
                        {
                            "amount": "{{amount}}",
                            "idempotency_key": "{{idempotency_key}}",
                        },
                    ),
                    Stage.return_response({"status": "charged"}, 201),
                ],
            ),
        ],
    }
    await _save(client, pay)

    # 2. Rate-limited endpoint (fail mode).
    rl_fail = {
        "label": "conc_demo_rl_fail",
        "name": "Rate-limit (fail mode)",
        "parameters": {"user_id": {"required": True}},
        "functions": [
            Stage.rate_limit("user-{{user_id}}", 10, 60, "rl"),
            Stage.return_response({"ok": True}, 200),
        ],
    }
    await _save(client, rl_fail)

    # 3. Rate-limited endpoint (skip mode).
    rl_skip = {
        "label": "conc_demo_rl_skip",
        "name": "Rate-limit (skip mode)",
        "parameters": {"user_id": {"required": True}},
        "functions": [
            Stage.rate_limit("user-{{user_id}}", 10, 60, "rl", on_exceed="skip"),
            Stage.if_condition(
                {
                    "type": "FieldEquals",
                    "value": {"field": "rl.allowed", "value": False},
                },
                then_functions=[Stage.return_response({"status": "rate_limited"}, 429)],
                else_functions=[Stage.return_response({"status": "ok"}, 200)],
            ),
        ],
    }
    await _save(client, rl_skip)

    # 4. Distributed lock — acquire + critical section + release (token-fenced).
    lock = {
        "label": "conc_demo_lock",
        "name": "Critical section under lock",
        "parameters": {"resource": {"required": True}},
        "functions": [
            Stage.lock_acquire("{{resource}}", 30, "lock"),
            Stage.if_condition(
                {
                    "type": "FieldEquals",
                    "value": {"field": "lock.acquired", "value": False},
                },
                then_functions=[Stage.return_response({"status": "busy"}, 409)],
                else_functions=[
                    Stage.insert(
                        "lock_demo_audit", {"resource": "{{resource}}"}, ttl=60
                    ),
                    Stage.lock_release("{{resource}}", "{{lock.token}}", "release"),
                    Stage.return_response({"status": "done"}, 200),
                ],
            ),
        ],
    }
    await _save(client, lock)

    print("\nInvoke them like:")
    print(
        '  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }'
    )
    print('  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }')
    print('  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }')
    print('  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }')

    for label in (
        "conc_demo_pay",
        "conc_demo_rl_fail",
        "conc_demo_rl_skip",
        "conc_demo_lock",
    ):
        try:
            await client.delete_user_function(label)
        except Exception:
            pass
    print("\n✓ Cleaned up demo functions")


async def _save(client, func):
    try:
        await client.save_user_function(func)
        print(f"✓ {func['label']} saved")
    except Exception as e:
        print(f"SaveUserFunction({func['label']}) error: {e}")


if __name__ == "__main__":
    asyncio.run(main())
