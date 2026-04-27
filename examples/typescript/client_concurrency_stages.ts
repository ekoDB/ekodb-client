/**
 * Concurrency stages — IdempotencyClaim, RateLimit, LockAcquire/Release.
 *
 * These three primitives wrap ekoDB's atomic KV layer with the right
 * semantics for common server-side patterns:
 *
 *   - IdempotencyClaim — replay protection with TTL.
 *   - RateLimit         — fixed-window counter; "fail" or "skip" mode.
 *   - LockAcquire/Release — token-fenced distributed lock.
 *
 * Requires ekoDB >= 0.43.0.
 */

import { EkoDBClient, Stage, type UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created");

  // 1. Idempotent payment — claim → branch on replay → do work.
  const pay: UserFunction = {
    label: "conc_demo_pay",
    name: "Idempotent payment",
    parameters: {
      idempotency_key: { required: true },
      amount: { required: true },
    },
    functions: [
      Stage.idempotencyClaim("{{idempotency_key}}", 86400, "claim"),
      Stage.if(
        {
          type: "FieldEquals",
          value: { field: "claim.claimed", value: false },
        },
        [
          Stage.returnResponse(
            { status: "replay", idempotency_key: "{{idempotency_key}}" },
            200,
          ),
        ],
        [
          Stage.insert("charges", {
            amount: "{{amount}}",
            idempotency_key: "{{idempotency_key}}",
          }),
          Stage.returnResponse({ status: "charged" }, 201),
        ],
      ),
    ],
  };
  await client.saveUserFunction(pay).catch(() => {});
  console.log("✓ conc_demo_pay saved");

  // 2. Rate-limited endpoint (fail mode — gate is authoritative).
  const rlFail: UserFunction = {
    label: "conc_demo_rl_fail",
    name: "Rate-limit (fail mode)",
    parameters: { user_id: { required: true } },
    functions: [
      Stage.rateLimit("user-{{user_id}}", 10, 60, "rl"),
      Stage.returnResponse({ ok: true }, 200),
    ],
  };
  await client.saveUserFunction(rlFail).catch(() => {});
  console.log("✓ conc_demo_rl_fail saved");

  // 3. Rate-limited endpoint (skip mode — pipeline keeps running, branch on result).
  const rlSkip: UserFunction = {
    label: "conc_demo_rl_skip",
    name: "Rate-limit (skip mode)",
    parameters: { user_id: { required: true } },
    functions: [
      Stage.rateLimit("user-{{user_id}}", 10, 60, "rl", "skip"),
      Stage.if(
        { type: "FieldEquals", value: { field: "rl.allowed", value: false } },
        [Stage.returnResponse({ status: "rate_limited" }, 429)],
        [Stage.returnResponse({ status: "ok" }, 200)],
      ),
    ],
  };
  await client.saveUserFunction(rlSkip).catch(() => {});
  console.log("✓ conc_demo_rl_skip saved");

  // 4. Distributed lock — acquire + critical section + release (token-fenced).
  const lock: UserFunction = {
    label: "conc_demo_lock",
    name: "Critical section under lock",
    parameters: { resource: { required: true } },
    functions: [
      Stage.lockAcquire("{{resource}}", 30, "lock"),
      Stage.if(
        {
          type: "FieldEquals",
          value: { field: "lock.acquired", value: false },
        },
        [Stage.returnResponse({ status: "busy" }, 409)],
        [
          Stage.insert(
            "lock_demo_audit",
            { resource: "{{resource}}" },
            false,
            60,
          ),
          Stage.lockRelease("{{resource}}", "{{lock.token}}", "release"),
          Stage.returnResponse({ status: "done" }, 200),
        ],
      ),
    ],
  };
  await client.saveUserFunction(lock).catch(() => {});
  console.log("✓ conc_demo_lock saved");

  console.log("\nInvoke them like:");
  console.log(
    '  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }',
  );
  console.log('  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }');
  console.log('  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }');
  console.log(
    '  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }',
  );

  for (const label of [
    "conc_demo_pay",
    "conc_demo_rl_fail",
    "conc_demo_rl_skip",
    "conc_demo_lock",
  ]) {
    await client.deleteUserFunction(label).catch(() => {});
  }
  console.log("\n✓ Cleaned up demo functions");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
