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
 * Requires ekoDB >= 0.42.0.
 */

import { EkoDBClient, Stage, type UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

function isNotFoundError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 404") || /not found/i.test(message);
}

/** True when a save failed because the function label already exists (HTTP 409). */
function isAlreadyExistsError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 409") || message.includes("already exists");
}

/** Idempotent save: create the function, or update it in place on 409. */
async function saveOrUpdateUserFunction(
  client: EkoDBClient,
  fn: UserFunction,
): Promise<void> {
  try {
    await client.saveUserFunction(fn);
  } catch (error) {
    if (!isAlreadyExistsError(error)) throw error;
    await client.updateUserFunction(fn.label, fn);
    console.log(`Function '${fn.label}' already existed — updated instead`);
  }
}

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const suffix = `${process.pid}_${Date.now()}`;
  const payLabel = `conc_demo_pay_ts_${suffix}`;
  const failLabel = `conc_demo_rl_fail_ts_${suffix}`;
  const skipLabel = `conc_demo_rl_skip_ts_${suffix}`;
  const lockLabel = `conc_demo_lock_ts_${suffix}`;
  const labels = [payLabel, failLabel, skipLabel, lockLabel];
  let primaryError: unknown;

  try {
    console.log("✓ Client created");

    // 1. Idempotent payment — claim → branch on replay → do work.
    const pay: UserFunction = {
      label: payLabel,
      name: "Idempotent payment",
      parameters: {
        idempotency_key: { required: true },
        amount: { required: true },
      },
      functions: [
        Stage.idempotencyClaim(
          `conc:pay_ts:${suffix}:{{idempotency_key}}`,
          86400,
          "claim",
        ),
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
            Stage.insert(`conc_charges_ts_${suffix}`, {
              amount: "{{amount}}",
              idempotency_key: "{{idempotency_key}}",
            }),
            Stage.returnResponse({ status: "charged" }, 201),
          ],
        ),
      ],
    };
    await saveOrUpdateUserFunction(client, pay);
    console.log("✓ conc_demo_pay saved");

    // 2. Rate-limited endpoint (fail mode — gate is authoritative).
    const rlFail: UserFunction = {
      label: failLabel,
      name: "Rate-limit (fail mode)",
      parameters: { user_id: { required: true } },
      functions: [
        Stage.rateLimit(`conc:rl_fail_ts:${suffix}:{{user_id}}`, 10, 60, "rl"),
        Stage.returnResponse({ ok: true }, 200),
      ],
    };
    await saveOrUpdateUserFunction(client, rlFail);
    console.log("✓ conc_demo_rl_fail saved");

    // 3. Rate-limited endpoint (skip mode — pipeline keeps running, branch on result).
    const rlSkip: UserFunction = {
      label: skipLabel,
      name: "Rate-limit (skip mode)",
      parameters: { user_id: { required: true } },
      functions: [
        Stage.rateLimit(
          `conc:rl_skip_ts:${suffix}:{{user_id}}`,
          10,
          60,
          "rl",
          "skip",
        ),
        Stage.if(
          { type: "FieldEquals", value: { field: "rl.allowed", value: false } },
          [Stage.returnResponse({ status: "rate_limited" }, 429)],
          [Stage.returnResponse({ status: "ok" }, 200)],
        ),
      ],
    };
    await saveOrUpdateUserFunction(client, rlSkip);
    console.log("✓ conc_demo_rl_skip saved");

    // 4. Distributed lock — acquire + critical section + release (token-fenced).
    const lock: UserFunction = {
      label: lockLabel,
      name: "Critical section under lock",
      parameters: { resource: { required: true } },
      functions: [
        Stage.lockAcquire(`conc:lock_ts:${suffix}:{{resource}}`, 30, "lock"),
        Stage.if(
          {
            type: "FieldEquals",
            value: { field: "lock.acquired", value: false },
          },
          [Stage.returnResponse({ status: "busy" }, 409)],
          [
            Stage.insert(
              `conc_lock_audit_ts_${suffix}`,
              { resource: "{{resource}}" },
              false,
              60,
            ),
            Stage.lockRelease(
              `conc:lock_ts:${suffix}:{{resource}}`,
              "{{lock.token}}",
              "release",
            ),
            Stage.returnResponse({ status: "done" }, 200),
          ],
        ),
      ],
    };
    await saveOrUpdateUserFunction(client, lock);
    console.log("✓ conc_demo_lock saved");

    console.log("\nInvoke them like:");
    console.log(
      `  POST /api/functions/${payLabel} { "idempotency_key": "...", "amount": 100 }`,
    );
    console.log(`  POST /api/functions/${failLabel} { "user_id": 42 }`);
    console.log(`  POST /api/functions/${skipLabel} { "user_id": 42 }`);
    console.log(
      `  POST /api/functions/${lockLabel} { "resource": "queue:drain" }`,
    );
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    const cleanupErrors: unknown[] = [];
    for (const label of labels.reverse()) {
      try {
        await client.deleteUserFunction(label);
      } catch (error) {
        if (!isNotFoundError(error)) cleanupErrors.push(error);
      }
    }
    if (cleanupErrors.length > 0) {
      throw new AggregateError(
        primaryError === undefined
          ? cleanupErrors
          : [primaryError, ...cleanupErrors],
        "Concurrency stages example or cleanup failed",
      );
    }
    console.log("\n✓ Cleaned up demo functions");
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
