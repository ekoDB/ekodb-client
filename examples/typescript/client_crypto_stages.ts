/**
 * Crypto stages — comprehensive demo of every primitive added in
 * ekoDB 0.42.0 (HMAC, AES-GCM, UUID, TOTP, Base64, Hex, Slugify).
 *
 * Each stored function exercises one primitive (or a sign/verify
 * pair) so callers can copy whichever pattern they need.
 *
 *   - crypto_demo_hmac      HmacSign + HmacVerify (round trip)
 *   - crypto_demo_aes       AesEncrypt + AesDecrypt (round trip)
 *   - crypto_demo_uuid      UuidGenerate
 *   - crypto_demo_totp      TotpGenerate (RFC 6238 secret format)
 *   - crypto_demo_encoding  Base64Encode + HexEncode + Slugify
 *
 * Operator-owned secrets flow through `{{env.NAME}}` so they never
 * land in the function definition stored on disk.
 *
 * Requires ekoDB >= 0.42.0.
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

  // 1. HMAC sign + verify in a single pipeline.
  const hmac: UserFunction = {
    label: "crypto_demo_hmac",
    name: "HMAC sign + verify",
    parameters: { payload: { required: true } },
    functions: [
      Stage.hmacSign("{{payload}}", "{{env.HMAC_KEY}}", "mac", {
        algorithm: "sha256",
        encoding: "hex",
      }),
      Stage.hmacVerify(
        "{{payload}}",
        "{{mac}}",
        "{{env.HMAC_KEY}}",
        "verified",
        {
          algorithm: "sha256",
          encoding: "hex",
        },
      ),
    ],
  };
  await client.saveUserFunction(hmac).catch(() => {});
  console.log("✓ crypto_demo_hmac saved");

  // 2. AES-256-GCM encrypt + decrypt.
  const aes: UserFunction = {
    label: "crypto_demo_aes",
    name: "AES encrypt + decrypt",
    parameters: { plaintext: { required: true } },
    functions: [
      Stage.aesEncrypt("{{plaintext}}", "{{env.DATA_KEY}}", "envelope", "hex"),
      Stage.aesDecrypt("envelope", "{{env.DATA_KEY}}", "recovered", "hex"),
    ],
  };
  await client.saveUserFunction(aes).catch(() => {});
  console.log("✓ crypto_demo_aes saved");

  // 3. UuidGenerate — fresh ID every call.
  const uuidFn: UserFunction = {
    label: "crypto_demo_uuid",
    name: "Generate v4 UUID",
    parameters: {},
    functions: [Stage.uuidGenerate("id")],
  };
  await client.saveUserFunction(uuidFn).catch(() => {});
  console.log("✓ crypto_demo_uuid saved");

  // 4. TotpGenerate — RFC 6238 with SHA1 (most authenticator apps).
  const totp: UserFunction = {
    label: "crypto_demo_totp",
    name: "Generate TOTP code",
    parameters: {},
    functions: [
      Stage.totpGenerate("{{env.TOTP_SECRET}}", "code", {
        digits: 6,
        period: 30,
        algorithm: "sha1",
      }),
    ],
  };
  await client.saveUserFunction(totp).catch(() => {});
  console.log("✓ crypto_demo_totp saved");

  // 5. Base64 + Hex + Slugify chained on one input.
  const encoding: UserFunction = {
    label: "crypto_demo_encoding",
    name: "Base64 / Hex / Slugify",
    parameters: { title: { required: true } },
    functions: [
      Stage.base64Encode("{{title}}", "title_b64"),
      Stage.hexEncode("{{title}}", "title_hex"),
      Stage.slugify("{{title}}", "title_slug"),
    ],
  };
  await client.saveUserFunction(encoding).catch(() => {});
  console.log("✓ crypto_demo_encoding saved");

  console.log("\nInvoke them with:");
  console.log('  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }');
  console.log(
    '  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }',
  );
  console.log("  POST /api/functions/crypto_demo_uuid");
  console.log("  POST /api/functions/crypto_demo_totp");
  console.log(
    '  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }',
  );

  for (const label of [
    "crypto_demo_hmac",
    "crypto_demo_aes",
    "crypto_demo_uuid",
    "crypto_demo_totp",
    "crypto_demo_encoding",
  ]) {
    await client.deleteUserFunction(label).catch(() => {});
  }
  console.log("\n✓ Cleaned up demo functions");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
