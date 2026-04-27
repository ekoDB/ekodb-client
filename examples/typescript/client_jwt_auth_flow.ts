/**
 * JWT auth flow — registration + login as pure stored functions.
 *
 * Demonstrates the canonical password-auth pattern using ekoDB stages:
 *
 *   register flow:  BcryptHash → Insert
 *   login flow:     FindOne  → BcryptVerify → If(ok) { JwtSign + Return }
 *   verify flow:    JwtVerify → If(claims != null) { 200 } else { 401 }
 *
 * Operator-owned secrets (the JWT signing key) flow through
 * `{{env.JWT_SECRET}}` so they never enter the function definition or
 * the LLM context window.
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

  // 1. Register: bcrypt-hash, insert.
  const register: UserFunction = {
    label: "ts_users_register",
    name: "Register user",
    description: "Validate, bcrypt-hash, insert.",
    parameters: {
      email: { required: true },
      password: { required: true },
    },
    functions: [
      Stage.bcryptHash("{{password}}", "password_hash", 12),
      Stage.insert("ts_users", {
        email: "{{email}}",
        password_hash: "{{password_hash}}",
      }),
    ],
  };
  await client.saveUserFunction(register).catch(() => {});
  console.log("✓ ts_users_register saved");

  // 2. Login: find user, verify bcrypt, sign JWT on success.
  const login: UserFunction = {
    label: "ts_users_login",
    name: "Login user",
    description: "Verify password, mint JWT.",
    parameters: {
      email: { required: true },
      password: { required: true },
    },
    functions: [
      Stage.findOne("ts_users", "email", "{{email}}"),
      Stage.bcryptVerify("{{password}}", "password_hash", "password_ok"),
      Stage.if(
        { type: "FieldEquals", value: { field: "password_ok", value: true } },
        [
          Stage.jwtSign(
            { email: "{{email}}" },
            "{{env.JWT_SECRET}}",
            "token",
            3600,
            "HS256",
          ),
          Stage.returnResponse({ ok: true, token: "{{token}}" }, 200),
        ],
        [Stage.returnResponse({ ok: false }, 401)],
      ),
    ],
  };
  await client.saveUserFunction(login).catch(() => {});
  console.log("✓ ts_users_login saved");

  // 3. Verify a JWT — fail-closed when claims is null.
  const verify: UserFunction = {
    label: "ts_users_verify_token",
    name: "Verify JWT token",
    parameters: { token: { required: true } },
    functions: [
      // Synthetic record so JwtVerify has working_data[0] to read off.
      Stage.insert("_inflight_jwt_check", { token: "{{token}}" }, true, 60),
      Stage.jwtVerify("token", "{{env.JWT_SECRET}}", "claims", "HS256"),
      Stage.if(
        { type: "FieldEquals", value: { field: "claims", value: null } },
        [Stage.returnResponse({ ok: false }, 401)],
        [Stage.returnResponse({ ok: true }, 200)],
      ),
    ],
  };
  await client.saveUserFunction(verify).catch(() => {});
  console.log("✓ ts_users_verify_token saved");

  console.log("\n=== Auth flow defined as pure stored functions ===");
  console.log("Call them like:");
  console.log(
    '  POST /api/functions/ts_users_register { "email": "a@b.com", "password": "s3cret" }',
  );
  console.log(
    '  POST /api/functions/ts_users_login    { "email": "a@b.com", "password": "s3cret" }',
  );
  console.log(
    '  POST /api/functions/ts_users_verify_token { "token": "<jwt>" }',
  );
  console.log(
    "\nSet JWT_SECRET in ekoDB's environment_vars whitelist before invoking.",
  );

  for (const label of [
    "ts_users_register",
    "ts_users_login",
    "ts_users_verify_token",
  ]) {
    await client.deleteUserFunction(label).catch(() => {});
  }
  console.log("\n✓ Cleaned up demo functions");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
