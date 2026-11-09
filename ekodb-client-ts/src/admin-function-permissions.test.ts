import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import {
  EkoDBAdminClient,
  MAX_FUNCTION_PATTERN_BYTES,
  type FunctionPermission,
} from "./admin";
import { EkoDBClient } from "./client";
import type { Principal, UserFunction } from "./functions";

interface Change {
  name: string;
  method: "PUT" | "DELETE";
  pattern: string;
  path: string;
  body: unknown;
  status: number;
  response: Record<string, unknown>;
}
const fixture = JSON.parse(
  readFileSync(
    resolve(
      __dirname,
      "../../test-fixtures/admin-function-permissions-contract.json",
    ),
    "utf8",
  ),
) as {
  key_id: string;
  registration_request: Record<string, unknown>;
  registration_response: Record<string, unknown>;
  registration_response_without_function_permissions: Record<string, unknown>;
  expected_registered_patterns: string[];
  key_list_record: Record<string, unknown>;
  permissions: Record<string, unknown>;
  permissions_without_functions: Record<string, unknown>;
  function_grants: FunctionPermission[];
  function_grant_changes: Change[];
  rejected_patterns: Array<{ name: string; value?: string; length?: number }>;
  refusals: Array<{
    name: string;
    method: "PUT" | "DELETE";
    pattern: string;
    status: number;
  }>;
  function_call_refusal: { status: number; body: unknown };
  function_definition: UserFunction;
  function_definition_unrecorded: UserFunction;
  function_save_request: UserFunction;
  schedule_record: Record<string, unknown> & {
    id: string;
    created_by: Principal;
  };
  function_permissions_reply_values: {
    accepted: Array<{ name: string; value: unknown; patterns: string[] }>;
    rejected: Array<{ name: string; value: unknown }>;
  };
};
const request = vi.fn<typeof fetch>();
const keyId = fixture.key_id;
const base = "https://example.test";

function response(body: unknown, status = 200) {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}
function client() {
  return new EkoDBAdminClient({
    baseURL: base,
    apiKey: "synthetic-admin-key-0123456789-abcdef",
  });
}
function modern() {
  request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
  request.mockResolvedValueOnce(
    response({ admin: true, collections: {}, kv: [], functions: [] }),
  );
}
function calls() {
  return request.mock.calls.map(([url, options]) => ({
    path: new URL(String(url)).pathname,
    method: options?.method,
    body: options?.body ? JSON.parse(String(options.body)) : undefined,
  }));
}
function rejectedPattern(entry: { value?: string; length?: number }) {
  return entry.value ?? "a".repeat(entry.length ?? 0);
}
beforeEach(() => {
  request.mockReset();
  vi.stubGlobal("fetch", request);
});
afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("function call grants", () => {
  it("sends registration grants and reads the echoed map", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(response(fixture.registration_response));
    const key = await client().createApiKey({
      label: "report-runner",
      function_permissions: {
        "report_*": ["call"],
        nightly_close: ["call"],
        audit_only: [],
      },
    });
    const register = calls().find((c) => c.path === "/api/auth/register");
    expect(register?.body).toEqual(fixture.registration_request);
    expect(key.function_permissions?.map((g) => g.pattern)).toEqual(
      fixture.expected_registered_patterns,
    );
  });

  it("keeps the old registration body and reads null without grants", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(
      response(fixture.registration_response_without_function_permissions),
    );
    const key = await client().createApiKey({ label: "report-runner" });
    const register = calls().find((c) => c.path === "/api/auth/register");
    expect(register?.body).not.toHaveProperty("function_permissions");
    expect(key.function_permissions).toBeNull();
  });

  it.each(["", "re*port", "report_**", "*report"])(
    "refuses registration pattern %j before HTTP",
    async (pattern) => {
      await expect(
        client().createApiKey({
          function_permissions: { [pattern]: ["call"] },
        }),
      ).rejects.toThrow("Invalid function pattern");
      expect(request).not.toHaveBeenCalled();
    },
  );

  it("refuses a sparse rights array, which JSON would send as [null]", async () => {
    await expect(
      client().createApiKey({
        function_permissions: {
          "report_*": new Array(1) as unknown as ["call"],
        },
      }),
    ).rejects.toThrow("Registration requires call action arrays");
    await expect(
      client().createApiKey({
        collection_permissions: {
          orders: new Array(1) as unknown as ["read"],
        },
      }),
    ).rejects.toThrow("Registration requires read/write action arrays");
    expect(request).not.toHaveBeenCalled();
  });

  it("refuses a right other than call", async () => {
    await expect(
      client().createApiKey({
        function_permissions: { "report_*": ["run" as "call"] },
      }),
    ).rejects.toThrow("Registration requires call action arrays");
    expect(request).not.toHaveBeenCalled();
  });

  it("carries grants on key listings and aggregate permission reads", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(response(fixture.permissions));
    request.mockResolvedValueOnce(response([fixture.key_list_record]));
    const admin = client();
    expect((await admin.getPermissions()).function_permissions).toEqual(
      fixture.function_grants,
    );
    expect((await admin.listApiKeys())[0].function_permissions).toEqual(
      fixture.function_grants,
    );
  });

  describe("reads function grants in the shapes every SDK accepts", () => {
    const cases = fixture.function_permissions_reply_values;
    function answer(value: unknown) {
      request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
      request.mockResolvedValueOnce(
        response([{ ...fixture.key_list_record, function_permissions: value }]),
      );
      request.mockResolvedValueOnce(
        response({ ...fixture.permissions, functions: value }),
      );
    }
    /** A server without key permission routes; its legacy read carries `value`. */
    function answerLegacy(value: unknown) {
      request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
      request.mockResolvedValueOnce(response({ error: "not found" }, 404));
      request.mockResolvedValueOnce(
        response({
          is_admin: false,
          collection_permissions: {},
          kv_permissions: [],
          function_permissions: value,
        }),
      );
    }
    /** The probe answered 404, then exactly one legacy read followed. */
    function expectLegacyRead() {
      expect(calls().slice(-2)).toEqual([
        {
          path: "/api/auth/keys/me/permissions",
          method: "GET",
          body: undefined,
        },
        { path: "/api/auth/permissions", method: "GET", body: undefined },
      ]);
    }
    for (const entry of cases.accepted) {
      it(`accepts ${entry.name}`, async () => {
        answer(entry.value);
        const admin = client();
        const keys = await admin.listApiKeys();
        expect(
          (keys[0].function_permissions ?? []).map((grant) => grant.pattern),
        ).toEqual(entry.patterns);
        const permissions = await admin.getPermissions();
        expect(
          permissions.function_permissions.map((grant) => grant.pattern),
        ).toEqual(entry.patterns);

        answerLegacy(entry.value);
        const legacy = client();
        expect(await legacy.usesKeyPermissionRoutes()).toBe(false);
        expect(
          (await legacy.getPermissions()).function_permissions.map(
            (grant) => grant.pattern,
          ),
        ).toEqual(entry.patterns);
        expectLegacyRead();
      });
    }
    for (const entry of cases.rejected) {
      it(`refuses ${entry.name}`, async () => {
        answer(entry.value);
        const admin = client();
        await expect(admin.listApiKeys()).rejects.toThrow(
          "Invalid admin response",
        );
        await expect(admin.getPermissions()).rejects.toThrow(
          "Invalid admin response",
        );

        answerLegacy(entry.value);
        const legacy = client();
        expect(await legacy.usesKeyPermissionRoutes()).toBe(false);
        await expect(legacy.getPermissions()).rejects.toThrow(
          "Invalid admin response",
        );
        expectLegacyRead();
      });
    }
  });

  it("reads no grants from a server that predates them", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(
      response(fixture.permissions_without_functions),
    );
    expect((await client().getPermissions()).function_permissions).toEqual([]);
  });

  it("lists and gets function grants", async () => {
    modern();
    request.mockResolvedValueOnce(response(fixture.function_grants));
    request.mockResolvedValueOnce(response({ pattern: "report_*" }));
    const admin = client();
    expect(await admin.listApiKeyFunctionPermissions(keyId)).toEqual(
      fixture.function_grants,
    );
    expect(await admin.getApiKeyFunctionPermissions(keyId, "report_*")).toEqual(
      {
        pattern: "report_*",
      },
    );
    expect(calls().slice(2)).toEqual([
      {
        path: `/api/auth/keys/${keyId}/permissions/functions`,
        method: "GET",
        body: undefined,
      },
      {
        path: `/api/auth/keys/${keyId}/permissions/functions/report_%2A`,
        method: "GET",
        body: undefined,
      },
    ]);
  });

  it.each(fixture.function_grant_changes)(
    "follows the recorded route, body and reply: $name",
    async (recorded) => {
      modern();
      request.mockResolvedValueOnce(
        response(recorded.response, recorded.status),
      );
      const admin = client();
      const result =
        recorded.method === "PUT"
          ? await admin.putApiKeyFunctionPermissions(keyId, recorded.pattern)
          : await admin.deleteApiKeyFunctionPermissions(
              keyId,
              recorded.pattern,
            );
      const [sent] = calls().slice(2);
      expect(sent.path).toBe(recorded.path);
      expect(sent.method).toBe(recorded.method);
      expect(sent.body ?? null).toEqual(recorded.body);
      expect(result).toEqual(recorded.response);
    },
  );

  it.each(fixture.refusals)(
    "surfaces a $status refusal without the body: $name",
    async (refusal) => {
      modern();
      request.mockResolvedValueOnce(
        response({ error: "synthetic-secret" }, refusal.status),
      );
      const admin = client();
      const error = await (
        refusal.method === "PUT"
          ? admin.putApiKeyFunctionPermissions(keyId, refusal.pattern)
          : admin.deleteApiKeyFunctionPermissions(keyId, refusal.pattern)
      ).catch((e: Error) => e);
      expect((error as Error).message).toBe(
        `Admin request failed (HTTP ${refusal.status})`,
      );
    },
  );

  it.each(fixture.rejected_patterns)(
    "rejects pattern $name before HTTP",
    async (entry) => {
      const pattern = rejectedPattern(entry);
      const admin = client();
      for (const run of [
        () => admin.getApiKeyFunctionPermissions(keyId, pattern),
        () => admin.putApiKeyFunctionPermissions(keyId, pattern),
        () => admin.deleteApiKeyFunctionPermissions(keyId, pattern),
      ])
        await expect(run()).rejects.toThrow("Invalid function pattern");
      expect(request).not.toHaveBeenCalled();
    },
  );

  it.each(["report_\ud800", "\udc00report"])(
    "rejects a lone surrogate in %j before HTTP",
    async (pattern) => {
      const admin = client();
      for (const run of [
        () => admin.getApiKeyFunctionPermissions(keyId, pattern),
        () => admin.putApiKeyFunctionPermissions(keyId, pattern),
        () => admin.deleteApiKeyFunctionPermissions(keyId, pattern),
      ])
        await expect(run()).rejects.toThrow("Invalid function pattern");
      expect(request).not.toHaveBeenCalled();
    },
  );

  it("never accepts a key value as the target", async () => {
    await expect(
      client().putApiKeyFunctionPermissions(
        "synthetic-target-key-0123456789-abcdef",
        "report_*",
      ),
    ).rejects.toThrow("A key id or me is required");
    expect(request).not.toHaveBeenCalled();
  });

  it("accepts the longest pattern", async () => {
    const pattern = `${"a".repeat(MAX_FUNCTION_PATTERN_BYTES - 1)}*`;
    modern();
    request.mockResolvedValueOnce(
      response({ before: null, after: { pattern } }),
    );
    const result = await client().putApiKeyFunctionPermissions(keyId, pattern);
    expect(result.after).toEqual({ pattern });
    expect(calls()[2].path.endsWith(`${"a".repeat(255)}%2A`)).toBe(true);
  });

  it("counts bytes, not characters, against the pattern limit", async () => {
    const pattern = "é".repeat(MAX_FUNCTION_PATTERN_BYTES / 2 + 1);
    await expect(
      client().putApiKeyFunctionPermissions(keyId, pattern),
    ).rejects.toThrow("Invalid function pattern");
    expect(request).not.toHaveBeenCalled();
  });

  it("needs key-addressed routes", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(response({}, 404));
    await expect(
      client().putApiKeyFunctionPermissions("me", "report_*"),
    ).rejects.toThrow("requires per-key permission routes");
  });
});

describe("function definitions and refusals", () => {
  function plain() {
    return new EkoDBClient({
      baseURL: base,
      apiKey: "synthetic-admin-key-0123456789-abcdef",
      shouldRetry: false,
    });
  }

  it("reads run_as and created_by from the server", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(response(fixture.function_definition));
    request.mockResolvedValueOnce(
      response(fixture.function_definition_unrecorded),
    );
    const sdk = plain();
    const read = await sdk.getUserFunction("nightly_close");
    expect(read.run_as).toBe("author");
    expect(read.created_by).toEqual(fixture.function_definition.created_by);
    const unrecorded = await sdk.getUserFunction("legacy_close");
    expect(unrecorded.run_as).toBeNull();
    expect(unrecorded.created_by).toBeNull();
  });

  it("rejects a run_as other than caller, author or null", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(
      response({ ...fixture.function_definition, run_as: "root" }),
    );
    request.mockResolvedValueOnce(
      response([{ ...fixture.function_definition, run_as: "root" }]),
    );
    const sdk = plain();
    await expect(sdk.getUserFunction("nightly_close")).rejects.toThrow(
      'run_as must be "caller" or "author"',
    );
    await expect(sdk.listUserFunctions()).rejects.toThrow("run_as");
    await expect(
      sdk.saveUserFunction({
        ...fixture.function_save_request,
        run_as: "root" as "caller",
      }),
    ).rejects.toThrow("run_as");
  });

  it("sends run_as when saving a function", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(response({ id: keyId }));
    await plain().saveUserFunction(fixture.function_save_request);
    const save = calls().find((c) => c.path === "/api/functions");
    expect(save?.body.run_as).toBe("caller");
    expect(save?.body).not.toHaveProperty("created_by");
  });

  it("surfaces a refused function call as the client's normal error", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(
      response(
        fixture.function_call_refusal.body,
        fixture.function_call_refusal.status,
      ),
    );
    await expect(plain().callFunction("nightly_close")).rejects.toThrow(
      /^Request failed with status 403: /,
    );
  });

  it("returns the creator the server recorded on a schedule", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(response(fixture.schedule_record));
    const schedule = await plain().getSchedule(fixture.schedule_record.id);
    expect(schedule.created_by).toEqual(fixture.schedule_record.created_by);
  });
});
