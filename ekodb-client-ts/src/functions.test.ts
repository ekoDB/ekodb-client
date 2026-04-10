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
    expect(wire).toEqual({
      type: "Insert",
      collection: "users",
      record: { type: "Parameter", name: "record" },
      bypass_ripple: false,
      ttl: undefined,
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
