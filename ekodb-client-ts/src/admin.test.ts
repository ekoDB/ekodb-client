import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { EkoDBAdminClient } from "./admin";

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
});
afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("admin wire contract", () => {
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
    auth();
    request.mockResolvedValueOnce(response(key));
    const body = {
      api_key: "target-secret",
      collection: "users",
      read: false,
      write: true,
    };
    expect(await admin().updateApiKeyCollectionPermissions(body)).toEqual(key);
    expect(request.mock.calls[1][0]).toBe(
      "https://example.test/api/auth/permissions",
    );
    expect(request.mock.calls[1][1]!.method).toBe("PUT");
    expect(JSON.parse(request.mock.calls[1][1]!.body as string)).toEqual(body);
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

  it("rejects unsupported field restriction updates before sending", async () => {
    await expect(
      admin().updateApiKeyCollectionPermissions({
        api_key: "target-secret",
        collection: "users",
        read: true,
        write: false,
        // @ts-expect-error The server ignores field restrictions on PUT.
        select_fields: ["name"],
      }),
    ).rejects.toThrow("does not support field restrictions");
    expect(request).not.toHaveBeenCalled();
  });
});
