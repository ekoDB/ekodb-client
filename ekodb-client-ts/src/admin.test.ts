import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  EkoDBAdminClient,
  MIN_SERVER_VERSION_FOR_PERMISSION_EDITS,
} from "./admin";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { ApiKey, UpdateApiKeyCollectionPermissionsRequest } from "./admin";

const specified = JSON.parse(
  readFileSync(
    resolve(
      __dirname,
      "../../test-fixtures/admin-permission-edits-specified.json",
    ),
    "utf8",
  ),
) as {
  permission_edits: {
    name: string;
    request: UpdateApiKeyCollectionPermissionsRequest;
    response: ApiKey;
  }[];
};

const versionGate = vi.hoisted(() => ({
  minimum: undefined as string | undefined,
}));
// Synthetic release values are used only by mock tests; production remains unset.
vi.mock("./admin-version", () => ({
  get MIN_SERVER_VERSION_FOR_PERMISSION_EDITS() {
    return versionGate.minimum;
  },
}));
import { RateLimitError } from "./client";

const request = vi.fn<typeof fetch>();
const key = {
  key: "new-secret",
  label: "disposable",
  description: null,
  is_admin: false,
  collection_permissions: {
    users: {
      read: true,
      write: false,
      select_fields: ["name"],
      exclude_fields: ["secret"],
      writable_fields: [],
    },
  },
  kv_permissions: [],
  created_at: 1,
  expires_at: null,
  ripple_warning: "Propagation incomplete",
};
function response(body: unknown, status = 200) {
  return new Response(status === 204 ? null : JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}
function admin(timeout?: number) {
  return new EkoDBAdminClient({
    baseURL: "https://example.test/",
    apiKey: "admin-secret",
    timeout,
  });
}
function auth(token = "jwt-secret") {
  request.mockResolvedValueOnce(response({ token }));
}

beforeEach(() => {
  vi.stubGlobal("fetch", request);
  request.mockReset();
  versionGate.minimum = undefined;
});
afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("admin wire contract", () => {
  it.each([
    Object.assign(new Error("Admin request failed (HTTP 500)"), {
      cause: "admin-secret",
      diagnostic: "target-secret",
    }),
    Object.assign(new RateLimitError(1, "jwt-secret"), {
      cause: "admin-secret",
      diagnostic: "target-secret",
    }),
  ])(
    "reconstructs safe exceptions from wrapped fetch failures",
    async (original) => {
      auth();
      request.mockRejectedValueOnce(original);
      let failure: unknown;
      try {
        await admin().revokeApiKey("target-secret");
      } catch (error) {
        failure = error;
      }
      expect(failure).toBeInstanceOf(Error);
      expect(failure).not.toBe(original);
      expect(String(failure)).not.toContain("secret");
      expect((failure as { cause?: unknown }).cause).toBeUndefined();
      expect((failure as { diagnostic?: unknown }).diagnostic).toBeUndefined();
      expect(request).toHaveBeenCalledTimes(2);
    },
  );
  it("retries a definite 401 with a fresh token successfully", async () => {
    auth("old-secret");
    request.mockResolvedValueOnce(response({ error: "old-secret" }, 401));
    auth();
    request.mockResolvedValueOnce(response(key));
    await expect(admin().getPermissions()).resolves.toEqual(key);
    expect(request).toHaveBeenCalledTimes(4);
  });

  it("requests redirect rejection for both authentication and admin calls", async () => {
    auth();
    request.mockResolvedValueOnce(response({ error: "new-secret" }, 307));
    await expect(admin().createApiKey()).rejects.toThrow("HTTP 307");
    expect(
      request.mock.calls.every(([, options]) => options?.redirect === "error"),
    ).toBe(true);
  });
  it("registers a non-admin key with exact JSON and both credentials", async () => {
    auth();
    request.mockResolvedValueOnce(
      response({ ...key, collection_permissions: { users: ["read"] } }),
    );
    expect(
      await admin().createApiKey({
        label: "disposable",
        collection_permissions: { users: ["read"] },
      }),
    ).toEqual({
      ...key,
      collection_permissions: { users: { read: true, write: false } },
    });
    expect(request.mock.calls[0][0]).toBe(
      "https://example.test/api/auth/token",
    );
    expect(JSON.parse(request.mock.calls[0][1]!.body as string)).toEqual({
      api_key: "admin-secret",
    });
    const [url, options] = request.mock.calls[1];
    expect(url).toBe("https://example.test/api/auth/register");
    expect(options).toMatchObject({
      method: "POST",
      headers: {
        Authorization: "Bearer jwt-secret",
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      redirect: "error",
    });
    expect(JSON.parse(options!.body as string)).toEqual({
      api_key: "admin-secret",
      label: "disposable",
      is_admin: false,
      collection_permissions: { users: ["read"] },
    });
    expect(options!.signal).toBeInstanceOf(AbortSignal);
  });

  it.each([200, 201])(
    "supports creation status %s and explicit admin",
    async (status) => {
      auth();
      request.mockResolvedValueOnce(
        response({ ...key, is_admin: true }, status),
      );
      expect((await admin().createApiKey({ is_admin: true })).is_admin).toBe(
        true,
      );
      expect(JSON.parse(request.mock.calls[1][1]!.body as string)).toEqual({
        api_key: "admin-secret",
        is_admin: true,
        collection_permissions: {},
      });
    },
  );

  it("preserves full key metadata and field restrictions on list and permission reads", async () => {
    auth();
    request.mockResolvedValueOnce(response([key]));
    request.mockResolvedValueOnce(response(key));
    const client = admin();
    expect(await client.listApiKeys()).toEqual([key]);
    expect(await client.getPermissions()).toEqual(key);
    expect(
      request.mock.calls
        .slice(1)
        .map(([url, options]) => [url, options!.method, options!.body]),
    ).toEqual([
      ["https://example.test/api/auth/keys", "GET", undefined],
      ["https://example.test/api/auth/permissions", "GET", undefined],
    ]);
  });

  it("updates exactly one collection by the target secret", async () => {
    versionGate.minimum = "9.8.7";
    auth();
    request.mockResolvedValueOnce(response({ version: "9.8.7" }));
    request.mockResolvedValueOnce(response(key));
    const body = {
      api_key: "target-secret",
      collection: "users",
      read: false,
      write: true,
    };
    expect(await admin().updateApiKeyCollectionPermissions(body)).toEqual(key);
    expect(request.mock.calls[2][0]).toBe(
      "https://example.test/api/auth/permissions",
    );
    expect(request.mock.calls[2][1]!.method).toBe("PUT");
    expect(JSON.parse(request.mock.calls[2][1]!.body as string)).toEqual(body);
  });

  it.each([200, 204])(
    "encodes the whole revocation segment and accepts status %s",
    async (status) => {
      auth();
      request.mockResolvedValueOnce(response({ status: "success" }, status));
      await expect(
        admin().revokeApiKey("secret/ ?#%+&=雪"),
      ).resolves.toBeUndefined();
      expect(request.mock.calls[1][0]).toBe(
        "https://example.test/api/auth/keys/secret%2F%20%3F%23%25%2B%26%3D%E9%9B%AA",
      );
      expect(request.mock.calls[1][1]!.method).toBe("DELETE");
    },
  );

  it("refreshes after one definite 401, but never loops", async () => {
    auth("first-secret");
    request.mockResolvedValueOnce(response({ error: "first-secret" }, 401));
    auth("second-secret");
    request.mockResolvedValueOnce(response({ error: "second-secret" }, 401));
    await expect(admin().listApiKeys()).rejects.toThrow("HTTP 401");
    expect(request).toHaveBeenCalledTimes(4);
    expect(
      (request.mock.calls[3][1]!.headers as { [key: string]: string })
        .Authorization,
    ).toBe("Bearer second-secret");
  });

  it("refreshes an expired cached token before the next admin request", async () => {
    auth(
      `header.${Buffer.from(JSON.stringify({ exp: 1 })).toString("base64url")}.signature`,
    );
    request.mockResolvedValueOnce(response([]));
    auth();
    request.mockResolvedValueOnce(response([]));
    const client = admin();
    await client.listApiKeys();
    await client.listApiKeys();
    expect(request.mock.calls.map(([url]) => url)).toEqual([
      "https://example.test/api/auth/token",
      "https://example.test/api/auth/keys",
      "https://example.test/api/auth/token",
      "https://example.test/api/auth/keys",
    ]);
  });

  it.each([403, 429, 500, 503])(
    "redacts echoed secrets on HTTP %s and never retries creation",
    async (status) => {
      auth();
      request.mockResolvedValueOnce(
        response(
          { error: "admin-secret jwt-secret new-secret Invalid token" },
          status,
        ),
      );
      const spy = vi.spyOn(console, "log").mockImplementation(() => {});
      let error: unknown;
      try {
        await admin().createApiKey();
      } catch (value) {
        error = value;
      }
      expect(String(error)).not.toMatch(/admin-secret|jwt-secret|new-secret/);
      expect(error).toBeInstanceOf(Error);
      expect(request).toHaveBeenCalledTimes(2);
      expect(spy.mock.calls.flat().join(" ")).not.toContain("secret");
    },
  );

  it.each([
    new TypeError("new-secret"),
    new DOMException("admin-secret", "TimeoutError"),
    new SyntaxError("jwt-secret"),
  ])("redacts ambiguous failures without retry", async (error) => {
    auth();
    request.mockRejectedValueOnce(error);
    await expect(admin().createApiKey()).rejects.toThrow(
      /^Admin request (failed|timed out)$/,
    );
    expect(request).toHaveBeenCalledTimes(2);
  });

  it("bounds the actual HTTP request timeout", async () => {
    auth();
    request.mockImplementationOnce(
      (_url, options) =>
        new Promise((_resolve, reject) => {
          options!.signal!.addEventListener("abort", () =>
            reject(options!.signal!.reason),
          );
        }),
    );
    await expect(admin(10).createApiKey()).rejects.toThrow("timed out");
    expect(request).toHaveBeenCalledTimes(2);
  });

  it("redacts JSON parser errors containing response text", async () => {
    auth();
    request.mockResolvedValueOnce(
      new Response("new-secret is not JSON", { status: 200 }),
    );
    await expect(admin().listApiKeys()).rejects.toThrow(
      /^Admin request failed$/,
    );
    expect(request).toHaveBeenCalledTimes(2);
  });

  it("redacts secret-bearing revocation transport errors", async () => {
    auth();
    request.mockRejectedValueOnce(
      new TypeError("https://example.test/api/auth/keys/new-secret"),
    );
    await expect(admin().revokeApiKey("new-secret")).rejects.toThrow(
      /^Admin request failed$/,
    );
  });

  it.each([{ error: "admin-secret" }, { token: "" }, { token: 123 }])(
    "redacts authentication failures and malformed tokens",
    async (body) => {
      request.mockResolvedValueOnce(
        response(body, "error" in body ? 403 : 200),
      );
      await expect(admin().getPermissions()).rejects.toThrow(
        "Admin authentication failed",
      );
      expect(request).toHaveBeenCalledTimes(1);
    },
  );

  it.each([
    null,
    {},
    { ...key, is_admin: "false" },
    { ...key, collection_permissions: { users: { read: true } } },
    {
      ...key,
      collection_permissions: {
        users: { read: true, write: true, writable_fields: "new-secret" },
      },
    },
    { ...key, collection_permissions: { users: ["admin-secret"] } },
  ])("rejects malformed responses without exposing content", async (body) => {
    auth();
    request.mockResolvedValueOnce(response(body));
    await expect(admin().getPermissions()).rejects.toThrow(
      "Invalid admin response",
    );
  });

  it("rejects a non-list and a malformed record", async () => {
    const client = admin();
    auth();
    request.mockResolvedValueOnce(response({ key: "new-secret" }));
    await expect(client.listApiKeys()).rejects.toThrow(
      "Invalid admin response",
    );
    request.mockResolvedValueOnce(response([{ ...key, key: null }]));
    await expect(client.listApiKeys()).rejects.toThrow(
      "Invalid admin response",
    );
  });

  it("rejects registration object grants rather than losing restrictions", async () => {
    await expect(
      admin().createApiKey({
        collection_permissions: {
          // @ts-expect-error Registration takes action arrays only.
          users: { read: true, select_fields: ["name"] },
        },
      }),
    ).rejects.toThrow("action arrays");
    expect(request).not.toHaveBeenCalled();
  });

  it.each(specified.permission_edits)(
    "preserves exact three-state body: $name",
    async (contract) => {
      versionGate.minimum = "9.8.7";
      auth();
      request.mockResolvedValueOnce(response({ version: "9.8.7" }));
      request.mockResolvedValueOnce(response(contract.response));
      expect(
        await admin().updateApiKeyCollectionPermissions(contract.request),
      ).toEqual(contract.response);
      const [url, options] = request.mock.calls[2];
      expect(url).toBe("https://example.test/api/auth/permissions");
      expect(options).toMatchObject({
        method: "PUT",
        headers: { Authorization: "Bearer jwt-secret" },
      });
      expect(JSON.parse(options!.body as string)).toStrictEqual(
        contract.request,
      );
    },
  );

  it("never exchanges explicit null and omitted/undefined fields", async () => {
    versionGate.minimum = "9.8.7";
    auth();
    request.mockResolvedValueOnce(response({ version: "9.8.7" }));
    request.mockResolvedValueOnce(response(key));
    await admin().updateApiKeyCollectionPermissions({
      api_key: "target-secret",
      collection: "users",
      read: true,
      write: false,
      select_fields: null,
      exclude_fields: undefined,
    });
    const body = JSON.parse(request.mock.calls[2][1]!.body as string);
    expect(Object.prototype.hasOwnProperty.call(body, "select_fields")).toBe(
      true,
    );
    expect(body.select_fields).toBeNull();
    expect(Object.prototype.hasOwnProperty.call(body, "exclude_fields")).toBe(
      false,
    );
    expect(Object.prototype.hasOwnProperty.call(body, "writable_fields")).toBe(
      false,
    );
  });

  it.each(["select_fields", "exclude_fields", "writable_fields"] as const)(
    "omits explicit undefined for %s",
    async (field) => {
      versionGate.minimum = "9.8.7";
      auth();
      request.mockResolvedValueOnce(response({ version: "9.8.7" }));
      request.mockResolvedValueOnce(response(key));
      await admin().updateApiKeyCollectionPermissions({
        api_key: "target-secret",
        collection: "users",
        read: true,
        write: false,
        [field]: undefined,
      });
      expect(
        JSON.parse(request.mock.calls[2][1]!.body as string),
      ).toStrictEqual({
        api_key: "target-secret",
        collection: "users",
        read: true,
        write: false,
      });
    },
  );

  it("fails closed without any request while the release constant is unset", async () => {
    expect(MIN_SERVER_VERSION_FOR_PERMISSION_EDITS).toBeUndefined();
    await expect(
      admin().updateApiKeyCollectionPermissions({
        api_key: "target-secret",
        collection: "users",
        read: true,
        write: true,
      }),
    ).rejects.toThrow("earlier servers remove a grant's field restrictions");
    expect(request).not.toHaveBeenCalled();
  });

  it.each([
    {},
    null,
    { version: 9 },
    { version: "9.8" },
    { version: "09.8.7" },
    { version: "999999999999999999999.8.7" },
    { version: "9.8.7-rc.1" },
    { version: "0.79.0" },
    { version: "9.8.6" },
    { version: "9.7.99" },
    { version: "8.99.99" },
  ])(
    "refuses unreadable, prerelease or older server versions: %j",
    async (health) => {
      versionGate.minimum = "9.8.7";
      auth();
      request.mockResolvedValueOnce(response(health));
      const client = admin();
      const edit = {
        api_key: "target-secret",
        collection: "users",
        read: true,
        write: true,
      };
      await expect(
        client.updateApiKeyCollectionPermissions(edit),
      ).rejects.toThrow("newer ekoDB server");
      await expect(
        client.updateApiKeyCollectionPermissions(edit),
      ).rejects.toThrow("newer ekoDB server");
      expect(request.mock.calls.map(([url]) => url)).toEqual([
        "https://example.test/api/auth/token",
        "https://example.test/api/health",
      ]);
      expect(request.mock.calls[1][1]).toMatchObject({
        method: "GET",
        headers: { Authorization: "Bearer jwt-secret" },
      });
    },
  );

  it.each(["9.8.7", "9.8.7+build.1", "9.8.8", "9.9.0", "10.0.0"])(
    "accepts sufficient release %s and caches its health read per instance",
    async (version) => {
      versionGate.minimum = "9.8.7";
      auth();
      request.mockResolvedValueOnce(response({ version }));
      request.mockResolvedValueOnce(response(key));
      request.mockResolvedValueOnce(response(key));
      const client = admin();
      const edit = {
        api_key: "target-secret",
        collection: "users",
        read: true,
        write: true,
      };
      await client.updateApiKeyCollectionPermissions(edit);
      await client.updateApiKeyCollectionPermissions(edit);
      auth();
      request.mockResolvedValueOnce(response({ version }));
      request.mockResolvedValueOnce(response(key));
      await admin().updateApiKeyCollectionPermissions(edit);
      expect(
        request.mock.calls.map(([url, options]) => [url, options!.method]),
      ).toEqual([
        ["https://example.test/api/auth/token", "POST"],
        ["https://example.test/api/health", "GET"],
        ["https://example.test/api/auth/permissions", "PUT"],
        ["https://example.test/api/auth/permissions", "PUT"],
        ["https://example.test/api/auth/token", "POST"],
        ["https://example.test/api/health", "GET"],
        ["https://example.test/api/auth/permissions", "PUT"],
      ]);
    },
  );

  it.each([
    new TypeError("admin-secret"),
    response({ error: "jwt-secret" }, 403),
    new Response("secret invalid JSON"),
  ])("sanitizes and caches an unreadable health response", async (failure) => {
    versionGate.minimum = "9.8.7";
    auth();
    if (failure instanceof Error) request.mockRejectedValueOnce(failure);
    else request.mockResolvedValueOnce(failure);
    const client = admin();
    const edit = {
      api_key: "target-secret",
      collection: "users",
      read: true,
      write: true,
    };
    for (let n = 0; n < 2; n++)
      await expect(
        client.updateApiKeyCollectionPermissions(edit),
      ).rejects.toThrow(
        /^Permission updates need a newer ekoDB server because earlier servers remove a grant's field restrictions when it is updated$/,
      );
    expect(request).toHaveBeenCalledTimes(2);
  });

  it("ignores inherited restriction fields and unrecognized properties", async () => {
    versionGate.minimum = "9.8.7";
    auth();
    request.mockResolvedValueOnce(response({ version: "9.8.7" }));
    request.mockResolvedValueOnce(response(key));
    const edit = Object.assign(Object.create({ select_fields: null }), {
      api_key: "target-secret",
      collection: "users",
      read: false,
      write: true,
      extra: "not sent",
    });
    await admin().updateApiKeyCollectionPermissions(edit);
    expect(JSON.parse(request.mock.calls[2][1]!.body as string)).toStrictEqual({
      api_key: "target-secret",
      collection: "users",
      read: false,
      write: true,
    });
  });

  it.each([null, undefined, []])(
    "rejects non-object update %j",
    async (edit) => {
      await expect(
        admin().updateApiKeyCollectionPermissions(edit as never),
      ).rejects.toThrow("Invalid collection permission update");
      expect(request).not.toHaveBeenCalled();
    },
  );

  it("shares the pending health read across concurrent updates", async () => {
    versionGate.minimum = "9.8.7";
    auth();
    request.mockResolvedValueOnce(response({ version: "9.8.7" }));
    request.mockResolvedValueOnce(response(key));
    request.mockResolvedValueOnce(response(key));
    const client = admin();
    const edit = {
      api_key: "target-secret",
      collection: "users",
      read: true,
      write: true,
    };
    await Promise.all([
      client.updateApiKeyCollectionPermissions(edit),
      client.updateApiKeyCollectionPermissions(edit),
    ]);
    expect(
      request.mock.calls.filter(
        ([url]) => url === "https://example.test/api/health",
      ),
    ).toHaveLength(1);
    expect(
      request.mock.calls.filter(([, options]) => options!.method === "PUT"),
    ).toHaveLength(2);
  });

  it.each([
    { api_key: "" },
    { api_key: "  " },
    { api_key: 4 },
    { collection: "" },
    { collection: "  " },
    { collection: false },
    { read: 1 },
    { write: "false" },
    ...["select_fields", "exclude_fields", "writable_fields"].flatMap((field) =>
      [false, {}, "name", [""], ["  "], [null], [4], new Array(1)].map(
        (value) => ({ [field]: value }),
      ),
    ),
  ])(
    "rejects invalid edits before authentication or health: %j",
    async (invalid) => {
      await expect(
        admin().updateApiKeyCollectionPermissions({
          api_key: "target-secret",
          collection: "users",
          read: true,
          write: false,
          ...invalid,
        } as never),
      ).rejects.toThrow("Invalid collection permission update");
      expect(request).not.toHaveBeenCalled();
    },
  );
});
