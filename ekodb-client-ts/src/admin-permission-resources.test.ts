import { beforeEach, afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import {
  EkoDBAdminClient,
  MIN_SERVER_VERSION_FOR_PERMISSION_EDITS,
  MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS,
  type ApiKey,
  type CollectionPermissions,
  type KvPermission,
  type KvGrant,
  type PermissionChange,
  type PutApiKeyCollectionPermissionsRequest,
  type PutApiKeyKvPermissionsRequest,
} from "./admin";

const fixture: {
  key_id: string;
  key: ApiKey;
  permissions: {
    admin: boolean;
    collections: Record<string, CollectionPermissions>;
    kv: KvPermission[];
  };
  permission_changes: Array<{
    name: string;
    method: "PUT" | "PATCH" | "DELETE";
    facet: "collections" | "kv";
    path: string;
    body: Partial<CollectionPermissions> | null;
    response: PermissionChange<CollectionPermissions | KvGrant>;
  }>;
} = JSON.parse(
  readFileSync(
    resolve(__dirname, "../../test-fixtures/admin-contract-0.80.0.json"),
    "utf8",
  ),
);
const request = vi.fn<typeof fetch>();
const permissionNames: {
  accepted: Array<{ name: string; value: string }>;
  rejected: Array<{ name: string; value: string }>;
} = JSON.parse(
  readFileSync(
    resolve(__dirname, "../../test-fixtures/admin-permission-names.json"),
    "utf8",
  ),
);
const namedOperations = [
  {
    facet: "collections",
    method: "GET",
    run: (admin: EkoDBAdminClient, name: string) =>
      admin.getApiKeyCollectionPermissions("me", name),
  },
  {
    facet: "collections",
    method: "PUT",
    run: (admin: EkoDBAdminClient, name: string) =>
      admin.putApiKeyCollectionPermissions({
        key_id: "me",
        collection: name,
        read: true,
        write: false,
      }),
  },
  {
    facet: "collections",
    method: "PATCH",
    run: (admin: EkoDBAdminClient, name: string) =>
      admin.patchApiKeyCollectionPermissions({
        key_id: "me",
        collection: name,
        read: true,
      }),
  },
  {
    facet: "collections",
    method: "DELETE",
    run: (admin: EkoDBAdminClient, name: string) =>
      admin.deleteApiKeyCollectionPermissions("me", name),
  },
  {
    facet: "kv",
    method: "GET",
    run: (admin: EkoDBAdminClient, name: string) =>
      admin.getApiKeyKvPermissions("me", name),
  },
  {
    facet: "kv",
    method: "PUT",
    run: (admin: EkoDBAdminClient, name: string) =>
      admin.putApiKeyKvPermissions({
        key_id: "me",
        prefix: name,
        read: true,
        write: false,
      }),
  },
  {
    facet: "kv",
    method: "PATCH",
    run: (admin: EkoDBAdminClient, name: string) =>
      admin.patchApiKeyKvPermissions({
        key_id: "me",
        prefix: name,
        read: true,
      }),
  },
  {
    facet: "kv",
    method: "DELETE",
    run: (admin: EkoDBAdminClient, name: string) =>
      admin.deleteApiKeyKvPermissions("me", name),
  },
];
function response(body: unknown, status = 200) {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}
function client() {
  return new EkoDBAdminClient({
    baseURL: "https://example.test",
    apiKey: "synthetic-admin-secret-0123456789",
  });
}
function modern() {
  request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
  request.mockResolvedValueOnce(response(fixture.permissions));
}
beforeEach(() => {
  request.mockReset();
  vi.stubGlobal("fetch", request);
});
afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("0.80 permission resources", () => {
  it.each(permissionNames.accepted)(
    "accepts shared facet name $name",
    async ({ value }) => {
      for (const operation of namedOperations) {
        request.mockReset();
        modern();
        const grant = { read: true, write: false };
        request.mockResolvedValueOnce(
          response(
            operation.method === "GET"
              ? grant
              : {
                  before: grant,
                  after: operation.method === "DELETE" ? null : grant,
                },
          ),
        );
        await operation.run(client(), value);
        const [url, options] =
          request.mock.calls[request.mock.calls.length - 1];
        expect(options?.method).toBe(operation.method);
        const path = new URL(String(url)).pathname;
        expect(path).toContain(`/permissions/${operation.facet}/`);
        expect(decodeURIComponent(path.split("/").pop()!)).toBe(value);
      }
    },
  );
  it.each(permissionNames.rejected)(
    "rejects shared facet name $name before HTTP",
    async ({ value }) => {
      for (const operation of namedOperations) {
        request.mockReset();
        await expect(operation.run(client(), value)).rejects.toThrow(
          /^Invalid (permission resource name|collection permission update|KV permission update)$/,
        );
        expect(request).not.toHaveBeenCalled();
      }
    },
  );
  it.each([undefined, ""])(
    "rejects a missing or empty aggregate KV prefix",
    async (prefix) => {
      request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
      request.mockResolvedValueOnce(
        response({
          ...fixture.permissions,
          kv: [{ prefix, read: true, write: false }],
        }),
      );
      await expect(client().getPermissions()).rejects.toThrow(
        "Invalid admin response",
      );
    },
  );
  it("exports both release boundaries", () => {
    expect(MIN_SERVER_VERSION_FOR_PERMISSION_EDITS).toBe("0.80.0");
    expect(MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS).toBe("0.79.2");
  });
  it("normalizes aggregate grants without dropping limits, KV, or future metadata", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(
      response({ ...fixture.permissions, future: { enabled: true } }),
    );
    const admin = client();
    const result = await admin.getPermissions();
    expect(result).toMatchObject({
      is_admin: false,
      collection_permissions: fixture.permissions.collections,
      kv_permissions: fixture.permissions.kv,
      future: { enabled: true },
    });
    expect(request.mock.calls.map(([url]) => url)).toEqual([
      "https://example.test/api/auth/token",
      "https://example.test/api/auth/keys/me/permissions",
    ]);
    request.mockResolvedValueOnce(
      response({ ...fixture.permissions, admin: true }),
    );
    expect((await admin.getPermissions()).is_admin).toBe(true);
    expect(request).toHaveBeenCalledTimes(3);
  });
  it("decodes independently supplied IDs from registration and listing", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(response(fixture.key));
    request.mockResolvedValueOnce(response([fixture.key]));
    const admin = client();
    expect((await admin.createApiKey()).id).toBe(fixture.key_id);
    expect((await admin.listApiKeys())[0].id).toBe(fixture.key_id);
    expect(fixture.key_id).not.toBe(fixture.key.key);
  });
  it("preserves a nullable KV field in registration metadata", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(
      response({ ...fixture.key, kv_permissions: null }),
    );
    expect((await client().createApiKey()).kv_permissions).toBeNull();
  });
  it.each([404, 405])(
    "falls back only after a %s probe and caches the generation",
    async (status) => {
      request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
      request.mockResolvedValueOnce(response({}, status));
      request.mockResolvedValueOnce(response(fixture.key));
      request.mockResolvedValueOnce(response(fixture.key));
      const admin = client();
      await admin.getPermissions();
      await admin.getPermissions();
      expect(request.mock.calls.map(([url]) => url)).toEqual([
        "https://example.test/api/auth/token",
        "https://example.test/api/auth/keys/me/permissions",
        "https://example.test/api/auth/permissions",
        "https://example.test/api/auth/permissions",
      ]);
    },
  );
  it.each([403, 500, 503])(
    "propagates %s without fallback or a poisoned detection cache",
    async (status) => {
      request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
      request.mockResolvedValueOnce(
        response({ secret: fixture.key.key }, status),
      );
      request.mockResolvedValueOnce(response(fixture.permissions));
      const admin = client();
      await expect(admin.usesKeyPermissionRoutes()).rejects.toThrow(
        `HTTP ${status}`,
      );
      expect(await admin.usesKeyPermissionRoutes()).toBe(true);
      expect(
        request.mock.calls.some(([url]) =>
          String(url).endsWith("/api/auth/permissions"),
        ),
      ).toBe(false);
    },
  );
  it("does not classify malformed success as a legacy server", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(
      response({ admin: false, collections: {}, kv: "secret" }),
    );
    const admin = client();
    await expect(admin.getPermissions()).rejects.toThrow(
      "Invalid admin response",
    );
    expect(request).toHaveBeenCalledTimes(2);
  });
  it("shares one capability probe across concurrent operations", async () => {
    modern();
    const admin = client();
    expect(
      await Promise.all([
        admin.usesKeyPermissionRoutes(),
        admin.usesKeyPermissionRoutes(),
      ]),
    ).toEqual([true, true]);
    expect(request).toHaveBeenCalledTimes(2);
  });
  it("uses actual routes without consulting a potentially older prerelease health label", async () => {
    modern();
    request.mockResolvedValueOnce(
      response(fixture.permission_changes[0].response),
    );
    await client().putApiKeyCollectionPermissions({
      key_id: fixture.key_id,
      collection: "tenant:a/",
      read: true,
      write: false,
      select_fields: ["name"],
    });
    expect(
      request.mock.calls.some(([url]) => String(url).endsWith("/api/health")),
    ).toBe(false);
  });
  it.each(fixture.permission_changes)(
    "matches the synthetic $method $facet $name contract",
    async (contract) => {
      modern();
      request.mockResolvedValueOnce(response(contract.response));
      const admin = client();
      const collection = {
        key_id: fixture.key_id,
        collection: contract.name,
        ...contract.body,
      } as PutApiKeyCollectionPermissionsRequest;
      const kv = {
        key_id: fixture.key_id,
        prefix: contract.name,
        ...contract.body,
      } as PutApiKeyKvPermissionsRequest;
      let result;
      if (contract.facet === "collections") {
        result =
          contract.method === "PUT"
            ? await admin.putApiKeyCollectionPermissions(collection)
            : contract.method === "PATCH"
              ? await admin.patchApiKeyCollectionPermissions(collection)
              : await admin.deleteApiKeyCollectionPermissions(
                  fixture.key_id,
                  contract.name,
                );
      } else {
        result =
          contract.method === "PUT"
            ? await admin.putApiKeyKvPermissions(kv)
            : contract.method === "PATCH"
              ? await admin.patchApiKeyKvPermissions(kv)
              : await admin.deleteApiKeyKvPermissions(
                  fixture.key_id,
                  contract.name,
                );
      }
      expect(result).toStrictEqual(contract.response);
      const [url, options] = request.mock.calls[request.mock.calls.length - 1]!;
      expect(url).toBe(`https://example.test${contract.path}`);
      expect(options?.method).toBe(contract.method);
      expect(
        options?.body === undefined ? null : JSON.parse(options.body as string),
      ).toStrictEqual(contract.body);
      for (const [
        permissionUrl,
        permissionOptions,
      ] of request.mock.calls.filter(([url]) =>
        String(url).includes("/permissions"),
      )) {
        expect(String(permissionUrl)).not.toContain(fixture.key.key);
        expect(String(permissionUrl)).not.toContain("synthetic-admin-secret");
        expect(String(permissionOptions?.body)).not.toContain(fixture.key.key);
        expect(String(permissionOptions?.body)).not.toContain(
          "synthetic-admin-secret",
        );
      }
    },
  );
  it("uses PATCH for the compatibility update name and preserves undefined/null/empty distinctions", async () => {
    modern();
    request.mockResolvedValueOnce(
      response(fixture.permission_changes[1].response),
    );
    await client().updateApiKeyCollectionPermissions({
      key_id: "me",
      collection: "a space+",
      read: true,
      write: false,
      select_fields: undefined,
      exclude_fields: null,
      writable_fields: [],
    });
    const [url, options] = request.mock.calls[request.mock.calls.length - 1]!;
    expect(url).toBe(
      "https://example.test/api/auth/keys/me/permissions/collections/a%20space%2B",
    );
    expect(options?.method).toBe("PATCH");
    expect(JSON.parse(options?.body as string)).toStrictEqual({
      read: true,
      write: false,
      exclude_fields: null,
      writable_fields: [],
    });
  });
  it("reads every facet and percent-encodes the prefix filter", async () => {
    modern();
    const grants = fixture.permissions.collections;
    request.mockResolvedValueOnce(response(fixture.permissions));
    request.mockResolvedValueOnce(response(grants));
    request.mockResolvedValueOnce(response(grants["tenant:a/"]));
    request.mockResolvedValueOnce(response(fixture.permissions.kv));
    request.mockResolvedValueOnce(response(fixture.permissions.kv[0]));
    const admin = client();
    expect(
      (await admin.getApiKeyPermissions(fixture.key_id)).kv_permissions,
    ).toEqual(fixture.permissions.kv);
    expect(await admin.listApiKeyCollectionPermissions("me")).toEqual(grants);
    expect(
      await admin.getApiKeyCollectionPermissions("me", "tenant:a/"),
    ).toEqual(grants["tenant:a/"]);
    expect(await admin.listApiKeyKvPermissions("me", "a space+")).toEqual(
      fixture.permissions.kv,
    );
    expect(await admin.getApiKeyKvPermissions("me", "tenant:a/")).toEqual(
      fixture.permissions.kv[0],
    );
    expect(request.mock.calls[request.mock.calls.length - 2]?.[0]).toBe(
      "https://example.test/api/auth/keys/me/permissions/kv?prefix=a%20space%2B",
    );
    expect(request.mock.calls[request.mock.calls.length - 1]?.[0]).toBe(
      "https://example.test/api/auth/keys/me/permissions/kv/tenant%3Aa%2F",
    );
  });
  it("refreshes one obsolete JWT and still parses a DELETE before/after response", async () => {
    modern();
    request.mockResolvedValueOnce(response({}, 401));
    request.mockResolvedValueOnce(response({ token: "replacement-jwt" }));
    request.mockResolvedValueOnce(
      response(fixture.permission_changes[2].response),
    );
    const result = await client().deleteApiKeyCollectionPermissions(
      fixture.key_id,
      "tenant:a/",
    );
    expect(result).toEqual(fixture.permission_changes[2].response);
    expect(
      request.mock.calls[request.mock.calls.length - 1]?.[1]?.headers,
    ).toMatchObject({
      Authorization: "Bearer replacement-jwt",
    });
    expect(
      request.mock.calls.filter(([, options]) => options?.method === "DELETE"),
    ).toHaveLength(2);
  });
  it.each([
    [".", "."],
    ["..", ".."],
    ["", ""],
    ["a space+", "a%20space%2B"],
    ["a/?&=%*", "a%2F%3F%26%3D%25%2A"],
  ])(
    "encodes KV prefix filters independently of path segments",
    async (prefix, encoded) => {
      modern();
      request.mockResolvedValueOnce(response([]));
      await expect(
        client().listApiKeyKvPermissions("me", prefix),
      ).resolves.toEqual([]);
      expect(request.mock.calls[request.mock.calls.length - 1]?.[0]).toBe(
        `https://example.test/api/auth/keys/me/permissions/kv?prefix=${encoded}`,
      );
    },
  );
  it.each([42, "\ud800"])(
    "rejects malformed KV prefix filters before authentication",
    async (prefix) => {
      await expect(
        client().listApiKeyKvPermissions("me", prefix as string),
      ).rejects.toThrow("Invalid permission resource name");
      expect(request).not.toHaveBeenCalled();
    },
  );
  it.each([fixture.key.key, "", "0123456789ABCDEF0123456789ABCDEF", "../me"])(
    "rejects a secret/invalid identifier before any network request",
    async (keyId) => {
      await expect(
        client().putApiKeyKvPermissions({
          key_id: keyId,
          prefix: "session:",
          read: true,
          write: false,
        }),
      ).rejects.toThrow("key id");
      expect(request).not.toHaveBeenCalled();
    },
  );
  it.each([".", "..", "x\u0000", "x\u200b", "\ud800"])(
    "rejects malformed resource names without leaking input",
    async (name) => {
      await expect(
        client().deleteApiKeyKvPermissions("me", name),
      ).rejects.toThrow("Invalid permission resource name");
      expect(request).not.toHaveBeenCalled();
    },
  );
  it("rejects unknown fields and invalid PUT rights before authentication", async () => {
    await expect(
      client().putApiKeyCollectionPermissions({
        key_id: "me",
        collection: "users",
        read: true,
        write: false,
        api_key: fixture.key.key,
      } as never),
    ).rejects.toThrow("Invalid collection permission update");
    await expect(
      client().putApiKeyKvPermissions({
        key_id: "me",
        prefix: "*",
        read: false,
        write: false,
      }),
    ).rejects.toThrow("Invalid KV permission update");
    expect(request).not.toHaveBeenCalled();
  });
  it("refuses modern operations on old routes and legacy secret operations on modern routes", async () => {
    request.mockResolvedValueOnce(response({ token: "synthetic-jwt" }));
    request.mockResolvedValueOnce(response({}, 404));
    await expect(
      client().putApiKeyKvPermissions({
        key_id: "me",
        prefix: "*",
        read: true,
        write: false,
      }),
    ).rejects.toThrow("per-key permission routes");
    modern();
    await expect(
      client().legacyUpdateApiKeyCollectionPermissions({
        api_key: fixture.key.key,
        collection: "users",
        read: true,
        write: false,
      }),
    ).rejects.toThrow("key-id permission methods");
    expect(
      request.mock.calls.some(([, options]) => options?.method === "PUT"),
    ).toBe(false);
  });
  it.each([
    { before: null },
    { before: null, after: {} },
    { before: null, after: { read: "yes", write: false } },
  ])("rejects malformed change envelopes", async (body) => {
    modern();
    request.mockResolvedValueOnce(response(body));
    await expect(
      client().putApiKeyKvPermissions({
        key_id: "me",
        prefix: "*",
        read: true,
        write: false,
      }),
    ).rejects.toThrow("Invalid admin response");
  });
});
