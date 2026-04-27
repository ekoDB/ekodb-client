/**
 * Unit tests for the stored-function builder helpers (Stage + parameterRef).
 *
 * These tests cover the pure-data construction helpers and the structural
 * parameter placeholder. They don't hit a running ekoDB — server-side
 * behavior is covered by the Rust integration tests in
 * `ekodb/ekodb_server/tests/function_parameters_tests.rs`.
 */

import { describe, it, expect } from "vitest";
import { Stage, parameterRef, type FunctionStageConfig } from "./functions";

describe("parameterRef", () => {
  it("produces the structural placeholder shape ekoDB's resolver expects", () => {
    expect(parameterRef("record")).toEqual({
      type: "Parameter",
      name: "record",
    });
  });

  it("preserves an arbitrary parameter name verbatim", () => {
    expect(parameterRef("user_id").name).toBe("user_id");
  });
});

describe("Stage.param", () => {
  it("is an alias for parameterRef(name)", () => {
    expect(Stage.param("x")).toEqual(parameterRef("x"));
  });
});

describe("Stage.insert with a structural parameter placeholder", () => {
  it("embeds the whole-record placeholder into Insert.record", () => {
    const stage = Stage.insert("users", Stage.param("record")) as Extract<
      FunctionStageConfig,
      { type: "Insert" }
    >;

    expect(stage.type).toBe("Insert");
    expect(stage.collection).toBe("users");
    expect(stage.record).toEqual({ type: "Parameter", name: "record" });
  });

  it("also accepts per-field placeholders mixed with literal values", () => {
    const stage = Stage.insert("items", {
      label: "{{label}}",
      parent_id: Stage.param("parent_id"),
      kind: "item",
      tags: Stage.param("tags"),
    }) as Extract<FunctionStageConfig, { type: "Insert" }>;

    expect(stage.record.label).toBe("{{label}}");
    expect(stage.record.parent_id).toEqual({
      type: "Parameter",
      name: "parent_id",
    });
    expect(stage.record.kind).toBe("item");
    expect(stage.record.tags).toEqual({ type: "Parameter", name: "tags" });
  });
});

describe("Stage.updateById with a structural parameter placeholder", () => {
  it("embeds the whole-updates placeholder into UpdateById.updates", () => {
    const stage = Stage.updateById(
      "items",
      "{{id}}",
      Stage.param("updates"),
    ) as Extract<FunctionStageConfig, { type: "UpdateById" }>;

    expect(stage.type).toBe("UpdateById");
    expect(stage.collection).toBe("items");
    expect(stage.record_id).toBe("{{id}}");
    expect(stage.updates).toEqual({ type: "Parameter", name: "updates" });
  });

  it("accepts per-field placeholders for partial fine-grained updates", () => {
    const stage = Stage.updateById("items", "{{id}}", {
      label: "{{label}}",
      updated_at: "{{updated_at}}",
    }) as Extract<FunctionStageConfig, { type: "UpdateById" }>;

    expect(stage.updates.label).toBe("{{label}}");
    expect(stage.updates.updated_at).toBe("{{updated_at}}");
  });
});

describe("Stage.update (filter-based) with structural updates", () => {
  it("accepts a Parameter placeholder in both filter values and the updates body", () => {
    const stage = Stage.update(
      "items",
      {
        type: "Condition",
        content: {
          field: "id",
          operator: "Eq",
          value: Stage.param("id"),
        },
      },
      Stage.param("updates"),
    ) as Extract<FunctionStageConfig, { type: "Update" }>;

    expect(stage.type).toBe("Update");
    expect(stage.filter).toEqual({
      type: "Condition",
      content: {
        field: "id",
        operator: "Eq",
        value: { type: "Parameter", name: "id" },
      },
    });
    expect(stage.updates).toEqual({ type: "Parameter", name: "updates" });
  });
});

describe("Stage.batchInsert with structural placeholders inside each record", () => {
  it("lets callers template each record's record-body from params", () => {
    const stage = Stage.batchInsert("audit_log", [
      { actor: Stage.param("user_id"), at: "{{now}}", message: "created" },
      { actor: Stage.param("user_id"), at: "{{now}}", message: "initialized" },
    ]) as Extract<FunctionStageConfig, { type: "BatchInsert" }>;

    expect(stage.type).toBe("BatchInsert");
    expect(stage.records).toHaveLength(2);
    expect(stage.records[0].actor).toEqual({
      type: "Parameter",
      name: "user_id",
    });
    expect(stage.records[0].at).toBe("{{now}}");
    expect(stage.records[1].message).toBe("initialized");
  });
});

describe("JSON serialization round-trip (what actually goes on the wire)", () => {
  it("serializes a structural placeholder exactly as ekoDB expects", () => {
    const stage = Stage.insert("users", Stage.param("record"));
    const wire = JSON.parse(JSON.stringify(stage));
    expect((stage as any).ttl).toBeUndefined();
    expect("ttl" in wire).toBe(false);
    expect(wire).toEqual({
      type: "Insert",
      collection: "users",
      record: { type: "Parameter", name: "record" },
      bypass_ripple: false,
    });
  });
});

// ============================================================================
// Crypto primitives: BcryptHash, BcryptVerify, RandomToken (ekoDB >= 0.41.0)
// ============================================================================

describe("Stage.bcryptHash", () => {
  it("produces a BcryptHash stage with text placeholder + explicit cost", () => {
    const stage = Stage.bcryptHash(
      "{{password}}",
      "password_hash",
      12,
    ) as Extract<FunctionStageConfig, { type: "BcryptHash" }>;
    expect(stage.type).toBe("BcryptHash");
    expect(stage.plain).toBe("{{password}}");
    expect(stage.cost).toBe(12);
    expect(stage.output_field).toBe("password_hash");
  });

  it("leaves cost undefined when the caller omits it", () => {
    const stage = Stage.bcryptHash("{{password}}", "pw_hash") as Extract<
      FunctionStageConfig,
      { type: "BcryptHash" }
    >;
    expect(stage.cost).toBeUndefined();
  });
});

describe("Stage.bcryptVerify", () => {
  it("produces a BcryptVerify stage wiring hash_field and output_field", () => {
    const stage = Stage.bcryptVerify(
      "{{password}}",
      "password_hash",
      "valid",
    ) as Extract<FunctionStageConfig, { type: "BcryptVerify" }>;
    expect(stage.type).toBe("BcryptVerify");
    expect(stage.plain).toBe("{{password}}");
    expect(stage.hash_field).toBe("password_hash");
    expect(stage.output_field).toBe("valid");
  });
});

describe("Stage.randomToken", () => {
  it("produces a RandomToken stage with explicit encoding", () => {
    const stage = Stage.randomToken(32, "session_token", "hex") as Extract<
      FunctionStageConfig,
      { type: "RandomToken" }
    >;
    expect(stage.type).toBe("RandomToken");
    expect(stage.bytes).toBe(32);
    expect(stage.encoding).toBe("hex");
    expect(stage.output_field).toBe("session_token");
  });

  it("leaves encoding undefined by default (server treats as hex)", () => {
    const stage = Stage.randomToken(16, "token") as Extract<
      FunctionStageConfig,
      { type: "RandomToken" }
    >;
    expect(stage.encoding).toBeUndefined();
  });

  it("accepts base64 and base64url encodings", () => {
    const a = Stage.randomToken(16, "t", "base64") as Extract<
      FunctionStageConfig,
      { type: "RandomToken" }
    >;
    const b = Stage.randomToken(16, "t", "base64url") as Extract<
      FunctionStageConfig,
      { type: "RandomToken" }
    >;
    expect(a.encoding).toBe("base64");
    expect(b.encoding).toBe("base64url");
  });
});

describe("Crypto stages JSON wire format", () => {
  it("BcryptHash round-trips through JSON unchanged", () => {
    const stage = Stage.bcryptHash("{{password}}", "password_hash", 12);
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire).toEqual({
      type: "BcryptHash",
      plain: "{{password}}",
      cost: 12,
      output_field: "password_hash",
    });
  });

  it("BcryptVerify round-trips through JSON unchanged", () => {
    const stage = Stage.bcryptVerify("{{password}}", "password_hash", "valid");
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire).toEqual({
      type: "BcryptVerify",
      plain: "{{password}}",
      hash_field: "password_hash",
      output_field: "valid",
    });
  });

  it("RandomToken round-trips through JSON unchanged", () => {
    const stage = Stage.randomToken(32, "token", "hex");
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire).toEqual({
      type: "RandomToken",
      bytes: 32,
      encoding: "hex",
      output_field: "token",
    });
  });
});

// ============================================================================
// JWT primitives: JwtSign, JwtVerify (ekoDB >= 0.42.0)
// ============================================================================

describe("Stage.jwtSign", () => {
  it("produces a JwtSign stage with claims, expiry, and algorithm", () => {
    const stage = Stage.jwtSign(
      { sub: "{{user_id}}", role: "admin" },
      "{{env.JWT_SECRET}}",
      "token",
      3600,
      "HS256",
    ) as Extract<FunctionStageConfig, { type: "JwtSign" }>;
    expect(stage.type).toBe("JwtSign");
    expect(stage.claims).toEqual({ sub: "{{user_id}}", role: "admin" });
    expect(stage.secret).toBe("{{env.JWT_SECRET}}");
    expect(stage.expires_in_secs).toBe(3600);
    expect(stage.algorithm).toBe("HS256");
    expect(stage.output_field).toBe("token");
  });

  it("leaves algorithm and expires_in_secs undefined when omitted", () => {
    const stage = Stage.jwtSign(
      { sub: "u" },
      "{{env.JWT_SECRET}}",
      "t",
    ) as Extract<FunctionStageConfig, { type: "JwtSign" }>;
    expect(stage.algorithm).toBeUndefined();
    expect(stage.expires_in_secs).toBeUndefined();
  });
});

describe("Stage.jwtVerify", () => {
  it("produces a JwtVerify stage wiring token_field and output_field", () => {
    const stage = Stage.jwtVerify(
      "auth_token",
      "{{env.JWT_SECRET}}",
      "claims",
      "HS512",
    ) as Extract<FunctionStageConfig, { type: "JwtVerify" }>;
    expect(stage.type).toBe("JwtVerify");
    expect(stage.token_field).toBe("auth_token");
    expect(stage.secret).toBe("{{env.JWT_SECRET}}");
    expect(stage.algorithm).toBe("HS512");
    expect(stage.output_field).toBe("claims");
  });
});

describe("JWT stages JSON wire format", () => {
  it("JwtSign round-trips through JSON unchanged", () => {
    const stage = Stage.jwtSign(
      { sub: "user-1" },
      "{{env.JWT_SECRET}}",
      "token",
      3600,
      "HS256",
    );
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire).toEqual({
      type: "JwtSign",
      claims: { sub: "user-1" },
      secret: "{{env.JWT_SECRET}}",
      algorithm: "HS256",
      expires_in_secs: 3600,
      output_field: "token",
    });
  });

  it("JwtVerify round-trips through JSON unchanged", () => {
    const stage = Stage.jwtVerify(
      "token",
      "{{env.JWT_SECRET}}",
      "claims",
      "HS256",
    );
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire).toEqual({
      type: "JwtVerify",
      token_field: "token",
      secret: "{{env.JWT_SECRET}}",
      algorithm: "HS256",
      output_field: "claims",
    });
  });
});

// ============================================================================
// EmailSend (ekoDB >= 0.42.0)
// ============================================================================

describe("Stage.emailSend", () => {
  it("produces a SendGrid EmailSend stage with full payload", () => {
    const stage = Stage.emailSend(
      "alice@example.com",
      "Welcome",
      "<p>Hi Alice</p>",
      "bot@example.com",
      "{{env.SENDGRID_API_KEY}}",
      {
        reply_to: "support@example.com",
        provider: "sendgrid",
        html: true,
        output_field: "send_result",
      },
    ) as Extract<FunctionStageConfig, { type: "EmailSend" }>;
    expect(stage.type).toBe("EmailSend");
    expect(stage.to).toBe("alice@example.com");
    expect(stage.subject).toBe("Welcome");
    expect(stage.body).toBe("<p>Hi Alice</p>");
    expect(stage.from).toBe("bot@example.com");
    expect(stage.reply_to).toBe("support@example.com");
    expect(stage.api_key).toBe("{{env.SENDGRID_API_KEY}}");
    expect(stage.provider).toBe("sendgrid");
    expect(stage.html).toBe(true);
    expect(stage.output_field).toBe("send_result");
  });

  it("leaves optional fields undefined when omitted", () => {
    const stage = Stage.emailSend(
      "x@example.com",
      "s",
      "b",
      "f@example.com",
      "k",
    ) as Extract<FunctionStageConfig, { type: "EmailSend" }>;
    expect(stage.reply_to).toBeUndefined();
    expect(stage.provider).toBeUndefined();
    expect(stage.html).toBeUndefined();
    expect(stage.output_field).toBeUndefined();
  });
});

// ============================================================================
// Error Handling & Control Flow: TryCatch, Parallel, Sleep (ekoDB >= 0.42.0)
// ============================================================================

describe("Stage.tryCatch", () => {
  it("produces a TryCatch stage with try/catch function lists", () => {
    const stage = Stage.tryCatch(
      [Stage.httpRequest("https://api.example.com/data")],
      [Stage.insert("fallback_log", { error: "{{error}}" })],
      "api_error",
    ) as Extract<FunctionStageConfig, { type: "TryCatch" }>;
    expect(stage.type).toBe("TryCatch");
    expect(stage.try_functions).toHaveLength(1);
    expect(stage.catch_functions).toHaveLength(1);
    expect(stage.output_error_field).toBe("api_error");
  });

  it("leaves output_error_field undefined when omitted", () => {
    const stage = Stage.tryCatch(
      [Stage.findAll("users")],
      [Stage.insert("errors", { msg: "failed" })],
    ) as Extract<FunctionStageConfig, { type: "TryCatch" }>;
    expect(stage.output_error_field).toBeUndefined();
  });
});

describe("Stage.parallel", () => {
  it("produces a Parallel stage with functions and wait_for_all", () => {
    const stage = Stage.parallel(
      [Stage.findAll("users"), Stage.findAll("orders")],
      true,
    ) as Extract<FunctionStageConfig, { type: "Parallel" }>;
    expect(stage.type).toBe("Parallel");
    expect(stage.functions).toHaveLength(2);
    expect(stage.wait_for_all).toBe(true);
  });

  it("defaults wait_for_all to true", () => {
    const stage = Stage.parallel([Stage.findAll("users")]) as Extract<
      FunctionStageConfig,
      { type: "Parallel" }
    >;
    expect(stage.wait_for_all).toBe(true);
  });

  it("accepts wait_for_all = false for race semantics", () => {
    const stage = Stage.parallel(
      [Stage.findAll("users"), Stage.findAll("cache")],
      false,
    ) as Extract<FunctionStageConfig, { type: "Parallel" }>;
    expect(stage.wait_for_all).toBe(false);
  });
});

describe("Stage.sleep", () => {
  it("produces a Sleep stage with numeric duration", () => {
    const stage = Stage.sleep(1000) as Extract<
      FunctionStageConfig,
      { type: "Sleep" }
    >;
    expect(stage.type).toBe("Sleep");
    expect(stage.duration_ms).toBe(1000);
  });

  it("accepts a text placeholder for parameter substitution", () => {
    const stage = Stage.sleep("{{delay}}") as Extract<
      FunctionStageConfig,
      { type: "Sleep" }
    >;
    expect(stage.duration_ms).toBe("{{delay}}");
  });
});

// ============================================================================
// Response Formatting: Return (ekoDB >= 0.42.0)
// ============================================================================

describe("Stage.returnResponse", () => {
  it("produces a Return stage with fields and status_code", () => {
    const stage = Stage.returnResponse(
      { message: "ok", user_id: "{{id}}" },
      201,
    ) as Extract<FunctionStageConfig, { type: "Return" }>;
    expect(stage.type).toBe("Return");
    expect(stage.fields).toEqual({ message: "ok", user_id: "{{id}}" });
    expect(stage.status_code).toBe(201);
  });

  it("leaves status_code undefined when omitted (server defaults to 200)", () => {
    const stage = Stage.returnResponse({ success: true }) as Extract<
      FunctionStageConfig,
      { type: "Return" }
    >;
    expect(stage.status_code).toBeUndefined();
  });
});

// ============================================================================
// Data Validation: Validate (ekoDB >= 0.42.0)
// ============================================================================

describe("Stage.validate", () => {
  it("produces a Validate stage with schema, data_field, and on_error", () => {
    const schema = {
      type: "object",
      required: ["name", "email"],
      properties: {
        name: { type: "string" },
        email: { type: "string", format: "email" },
      },
    };
    const stage = Stage.validate(schema, "{{input}}", [
      Stage.returnResponse({ error: "validation failed" }, 400),
    ]) as Extract<FunctionStageConfig, { type: "Validate" }>;
    expect(stage.type).toBe("Validate");
    expect(stage.schema).toEqual(schema);
    expect(stage.data_field).toBe("{{input}}");
    expect(stage.on_error).toHaveLength(1);
  });

  it("leaves on_error undefined when omitted", () => {
    const stage = Stage.validate({ type: "object" }, "record") as Extract<
      FunctionStageConfig,
      { type: "Validate" }
    >;
    expect(stage.on_error).toBeUndefined();
  });
});

// ============================================================================
// New stages JSON wire format
// ============================================================================

describe("New stages JSON wire format", () => {
  it("TryCatch round-trips through JSON unchanged", () => {
    const stage = Stage.tryCatch(
      [Stage.findAll("users")],
      [Stage.insert("errors", { msg: "failed" })],
      "err",
    );
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire.type).toBe("TryCatch");
    expect(wire.try_functions).toHaveLength(1);
    expect(wire.catch_functions).toHaveLength(1);
    expect(wire.output_error_field).toBe("err");
  });

  it("Parallel round-trips through JSON unchanged", () => {
    const stage = Stage.parallel(
      [Stage.findAll("a"), Stage.findAll("b")],
      false,
    );
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire).toEqual({
      type: "Parallel",
      functions: [
        { type: "FindAll", collection: "a" },
        { type: "FindAll", collection: "b" },
      ],
      wait_for_all: false,
    });
  });

  it("Sleep round-trips through JSON unchanged", () => {
    const wire = JSON.parse(JSON.stringify(Stage.sleep(500)));
    expect(wire).toEqual({ type: "Sleep", duration_ms: 500 });
  });

  it("Return round-trips through JSON unchanged", () => {
    const stage = Stage.returnResponse({ ok: true }, 201);
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire).toEqual({
      type: "Return",
      fields: { ok: true },
      status_code: 201,
    });
  });

  it("Validate round-trips through JSON unchanged", () => {
    const stage = Stage.validate({ type: "object" }, "data");
    const wire = JSON.parse(JSON.stringify(stage));
    expect(wire).toEqual({
      type: "Validate",
      schema: { type: "object" },
      data_field: "data",
    });
  });
});

describe("Crypto and concurrency stages", () => {
  it("hmacSign builds a stage with explicit algorithm and encoding", () => {
    const s = Stage.hmacSign("{{payload}}", "{{env.KEY}}", "mac", {
      algorithm: "sha256",
      encoding: "hex",
    });
    expect(s).toEqual({
      type: "HmacSign",
      input: "{{payload}}",
      secret: "{{env.KEY}}",
      algorithm: "sha256",
      output_field: "mac",
      encoding: "hex",
    });
  });

  it("hmacVerify wires all fields", () => {
    const s = Stage.hmacVerify("{{p}}", "{{m}}", "{{env.K}}", "ok");
    expect(s).toMatchObject({
      type: "HmacVerify",
      input: "{{p}}",
      provided_mac: "{{m}}",
      secret: "{{env.K}}",
      output_field: "ok",
    });
  });

  it("aesEncrypt and aesDecrypt build matching envelope contracts", () => {
    const enc = Stage.aesEncrypt(
      "{{plain}}",
      "{{env.DATA_KEY}}",
      "envelope",
      "hex",
    );
    expect(enc).toMatchObject({
      type: "AesEncrypt",
      plaintext: "{{plain}}",
      key: "{{env.DATA_KEY}}",
      key_encoding: "hex",
      output_field: "envelope",
    });
    const dec = Stage.aesDecrypt(
      "envelope",
      "{{env.DATA_KEY}}",
      "plain",
      "hex",
    );
    expect(dec).toMatchObject({
      type: "AesDecrypt",
      ciphertext_field: "envelope",
      output_field: "plain",
    });
  });

  it("uuidGenerate is a single-field stage", () => {
    expect(Stage.uuidGenerate("id")).toEqual({
      type: "UuidGenerate",
      output_field: "id",
    });
  });

  it("totpGenerate and totpVerify build with all options", () => {
    const gen = Stage.totpGenerate("{{env.TOTP}}", "code", {
      digits: 6,
      period: 30,
      algorithm: "sha1",
    });
    expect(gen.type).toBe("TotpGenerate");
    const ver = Stage.totpVerify("{{user_code}}", "{{env.TOTP}}", "ok", {
      skew: 1,
    });
    expect(ver.type).toBe("TotpVerify");
  });

  it("base64Encode/Decode and hexEncode/Decode build correctly", () => {
    expect(Stage.base64Encode("{{x}}", "b", true)).toMatchObject({
      type: "Base64Encode",
      url_safe: true,
    });
    expect(Stage.base64Decode("{{b}}", "x")).toMatchObject({
      type: "Base64Decode",
    });
    expect(Stage.hexEncode("{{x}}", "h")).toEqual({
      type: "HexEncode",
      input: "{{x}}",
      output_field: "h",
    });
    expect(Stage.hexDecode("{{h}}", "x")).toEqual({
      type: "HexDecode",
      input: "{{h}}",
      output_field: "x",
    });
  });

  it("slugify builds a stage", () => {
    expect(Stage.slugify("{{title}}", "slug")).toEqual({
      type: "Slugify",
      input: "{{title}}",
      output_field: "slug",
    });
  });

  it("idempotencyClaim, rateLimit, lockAcquire, lockRelease build correctly", () => {
    expect(Stage.idempotencyClaim("{{ikey}}", 3600, "claim")).toEqual({
      type: "IdempotencyClaim",
      key: "{{ikey}}",
      ttl_secs: 3600,
      output_field: "claim",
    });
    expect(Stage.rateLimit("{{user}}", 100, 60, "rl", "skip")).toMatchObject({
      type: "RateLimit",
      limit: 100,
      window_secs: 60,
      on_exceed: "skip",
    });
    expect(Stage.lockAcquire("{{r}}", 30, "lock")).toEqual({
      type: "LockAcquire",
      key: "{{r}}",
      ttl_secs: 30,
      output_field: "lock",
    });
    expect(Stage.lockRelease("{{r}}", "{{lock.token}}", "rel")).toEqual({
      type: "LockRelease",
      key: "{{r}}",
      token: "{{lock.token}}",
      output_field: "rel",
    });
  });

  it("crypto + concurrency stages round-trip through JSON unchanged", () => {
    const stages = [
      Stage.hmacSign("a", "k", "m", { algorithm: "sha256", encoding: "hex" }),
      Stage.aesEncrypt("p", "k", "e", "hex"),
      Stage.uuidGenerate("id"),
      Stage.totpGenerate("s", "c", {
        digits: 6,
        period: 30,
        algorithm: "sha1",
      }),
      Stage.idempotencyClaim("k", 60, "f"),
      Stage.rateLimit("k", 5, 60, "rl"),
      Stage.lockAcquire("r", 60, "l"),
    ];
    for (const s of stages) {
      const wire = JSON.parse(JSON.stringify(s));
      expect(wire.type).toBe(s.type);
    }
  });
});
