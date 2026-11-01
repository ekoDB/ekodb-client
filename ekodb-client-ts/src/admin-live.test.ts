import { randomUUID } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it, vi } from "vitest";
import {
  EkoDBAdminClient,
  type UpdateApiKeyCollectionPermissionsRequest,
} from "./admin";

// Opt-in only. The caller supplies the authorized disposable environment.
it.skipIf(process.env.EKODB_ADMIN_LIVE !== "1")(
  "admin disposable key lifecycle",
  async () => {
    const baseURL = process.env.API_BASE_URL;
    const apiKey = process.env.API_BASE_KEY;
    if (!baseURL || !apiKey)
      throw new Error("API_BASE_URL and API_BASE_KEY are required");
    await disposableLifecycle(baseURL, apiKey, `sdk-ts-admin-${randomUUID()}`);
  },
  60_000,
);

async function disposableLifecycle(
  baseURL: string,
  apiKey: string,
  label: string,
): Promise<void> {
  const admin = new EkoDBAdminClient({ baseURL, apiKey });
  try {
    const created = await admin.createApiKey({
      label,
      collection_permissions: { sdk_admin_probe: ["read"] },
    });
    const createdKey = created.key;
    // Only the key created in this lifecycle may be edited. Never dump secrets.
    expect(createdKey === apiKey).toBe(false);
    expect(created.is_admin).toBe(false);
    const matches = (await admin.listApiKeys()).filter(
      (key) => key.label === label,
    );
    expect(matches.length).toBe(1);
    expect(matches[0].key === createdKey).toBe(true);
    const scoped = new EkoDBAdminClient({ baseURL, apiKey: createdKey });
    const before = await scoped.getPermissions();
    expect(before.is_admin).toBe(false);
    expect(before.collection_permissions.sdk_admin_probe.read).toBe(true);
    expect(before.collection_permissions.sdk_admin_probe.write).toBe(false);
    await expect(scoped.listApiKeys()).rejects.toThrow();
    const modern = await admin.usesKeyPermissionRoutes();
    if (modern) {
      expect(
        typeof created.id === "string" && /^[0-9a-f]{32}$/.test(created.id),
      ).toBe(true);
      expect(matches[0].id === created.id).toBe(true);
    }
    const update = async (
      grant: Omit<UpdateApiKeyCollectionPermissionsRequest, "key_id">,
    ) => {
      if (modern)
        return admin.updateApiKeyCollectionPermissions({
          key_id: created.id!,
          ...grant,
        });
      return admin.legacyUpdateApiKeyCollectionPermissions({
        api_key: createdKey,
        ...grant,
      });
    };
    await update({
      collection: "sdk_admin_probe",
      read: true,
      write: true,
      select_fields: ["public", "editable"],
      exclude_fields: ["secret"],
      writable_fields: ["editable"],
    });
    let refreshed = new EkoDBAdminClient({ baseURL, apiKey: createdKey });
    let after = await refreshed.getPermissions();
    expect(after.collection_permissions.sdk_admin_probe).toMatchObject({
      read: true,
      write: true,
      select_fields: ["public", "editable"],
      exclude_fields: ["secret"],
      writable_fields: ["editable"],
    });
    await update({
      collection: "sdk_admin_probe",
      read: true,
      write: false,
      exclude_fields: null,
      writable_fields: [],
    });
    refreshed = new EkoDBAdminClient({ baseURL, apiKey: createdKey });
    after = await refreshed.getPermissions();
    expect(after.collection_permissions.sdk_admin_probe).toMatchObject({
      read: true,
      write: false,
      select_fields: ["public", "editable"],
      exclude_fields: null,
      writable_fields: [],
    });
    if (modern) {
      const id = created.id!;
      const names: { accepted: Array<{ name: string; value: string }> } =
        JSON.parse(
          readFileSync(
            resolve(
              __dirname,
              "../../test-fixtures/admin-permission-names.json",
            ),
            "utf8",
          ),
        );
      for (const { value } of names.accepted) {
        const collectionChange = await admin.putApiKeyCollectionPermissions({
          key_id: id,
          collection: value,
          read: true,
          write: false,
        });
        expect(collectionChange.after?.read).toBe(true);
        expect(
          (await admin.getApiKeyCollectionPermissions(id, value)).read,
        ).toBe(true);
        expect(
          (await admin.deleteApiKeyCollectionPermissions(id, value)).after,
        ).toBeNull();
        const kvChange = await admin.putApiKeyKvPermissions({
          key_id: id,
          prefix: value,
          read: true,
          write: false,
        });
        expect(kvChange.after?.read).toBe(true);
        expect((await admin.getApiKeyKvPermissions(id, value)).read).toBe(true);
        expect(
          (await admin.deleteApiKeyKvPermissions(id, value)).after,
        ).toBeNull();
      }
      const collection = "sdk probe+tenant:a/";
      const put = await admin.putApiKeyCollectionPermissions({
        key_id: id,
        collection,
        read: true,
        write: false,
        select_fields: ["name"],
      });
      expect(put.before).toBeNull();
      expect(put.after?.select_fields).toEqual(["name"]);
      expect(
        (await admin.getApiKeyCollectionPermissions(id, collection)).read,
      ).toBe(true);
      expect(
        (await admin.listApiKeyCollectionPermissions(id))[collection].read,
      ).toBe(true);
      const patched = await admin.patchApiKeyCollectionPermissions({
        key_id: id,
        collection,
        select_fields: null,
      });
      expect(patched.after?.select_fields).toBeNull();
      expect(
        (await admin.deleteApiKeyCollectionPermissions(id, collection)).after,
      ).toBeNull();
      const prefix = "sdk probe+tenant:a/";
      expect(
        (
          await admin.putApiKeyKvPermissions({
            key_id: id,
            prefix,
            read: true,
            write: false,
          })
        ).after?.read,
      ).toBe(true);
      expect((await admin.getApiKeyKvPermissions(id, prefix)).read).toBe(true);
      expect(
        (await admin.listApiKeyKvPermissions(id, prefix)).some(
          (grant) => grant.prefix === prefix,
        ),
      ).toBe(true);
      for (const filter of [".", ".."]) {
        expect(await admin.listApiKeyKvPermissions(id, filter)).toEqual([]);
      }
      expect(
        (await admin.listApiKeyKvPermissions(id, "")).some(
          (grant) => grant.prefix === prefix,
        ),
      ).toBe(true);
      expect(
        (
          await admin.patchApiKeyKvPermissions({
            key_id: id,
            prefix,
            read: false,
            write: false,
          })
        ).after,
      ).toBeNull();
      await admin.putApiKeyKvPermissions({
        key_id: id,
        prefix: "*",
        read: true,
        write: false,
      });
      expect((await admin.deleteApiKeyKvPermissions(id, "*")).after).toBeNull();
    }
  } finally {
    try {
      await cleanupDisposableKey(admin, label, apiKey);
    } catch {
      throw new Error(
        `Disposable key cleanup failed; revoke the key labeled ${label}`,
      );
    }
  }
}

async function cleanupDisposableKey(
  admin: EkoDBAdminClient,
  label: string,
  ownerKey: string,
): Promise<void> {
  const matches = (await admin.listApiKeys()).filter(
    (key) => key.label === label,
  );
  if (matches.length > 1 || matches.some((key) => key.key === ownerKey))
    throw new Error("Unexpected disposable key match");
  for (const key of matches) await admin.revokeApiKey(key.id ?? key.key);
  if ((await admin.listApiKeys()).some((key) => key.label === label))
    throw new Error("Disposable key remains");
}

it("recovers a uniquely labeled key after an ambiguous create response", async () => {
  const label = "sdk-ts-admin-cleanup-test";
  const record = {
    key: "new-secret",
    label,
    is_admin: false,
    collection_permissions: {},
  };
  const unrelated = { ...record, key: "unrelated-secret", label: "unrelated" };
  const request = vi.fn<typeof fetch>();
  request.mockResolvedValueOnce(
    new Response(JSON.stringify({ token: "jwt-secret" })),
  );
  request.mockResolvedValueOnce(
    new Response("malformed response after creating key"),
  );
  request.mockResolvedValueOnce(
    new Response(JSON.stringify([record, unrelated])),
  );
  request.mockResolvedValueOnce(new Response(null, { status: 204 }));
  request.mockResolvedValueOnce(new Response(JSON.stringify([unrelated])));
  vi.stubGlobal("fetch", request);
  try {
    const admin = new EkoDBAdminClient({
      baseURL: "https://example.test",
      apiKey: "admin-secret",
    });
    await expect(admin.createApiKey({ label })).rejects.toThrow(
      "Admin request failed",
    );
    await cleanupDisposableKey(admin, label, "admin-secret");
    expect(
      request.mock.calls
        .filter(([, options]) => options?.method === "DELETE")
        .map(([url]) => url),
    ).toEqual(["https://example.test/api/auth/keys/new-secret"]);
  } finally {
    vi.unstubAllGlobals();
  }
});

it("cleans up its disposable key when an older server version refuses the round-trip", async () => {
  const label = "sdk-ts-admin-gate-test";
  const record = {
    key: "new-secret",
    label,
    is_admin: false,
    collection_permissions: { sdk_admin_probe: { read: true, write: false } },
  };
  const request = vi.fn<typeof fetch>();
  for (const body of [
    { token: "admin-jwt" },
    record,
    [record],
    { token: "scoped-jwt" },
    record,
  ])
    request.mockResolvedValueOnce(new Response(JSON.stringify(body)));
  request.mockResolvedValueOnce(new Response("{}", { status: 403 }));
  request.mockResolvedValueOnce(
    new Response(JSON.stringify({ version: "0.79.1" })),
  );
  request.mockResolvedValueOnce(new Response(JSON.stringify([record])));
  request.mockResolvedValueOnce(new Response(null, { status: 204 }));
  request.mockResolvedValueOnce(new Response("[]"));
  vi.stubGlobal(
    "fetch",
    (input: Parameters<typeof fetch>[0], init?: RequestInit) =>
      String(input).endsWith("/api/auth/keys/me/permissions")
        ? Promise.resolve(new Response("{}", { status: 404 }))
        : request(input, init),
  );
  try {
    await expect(
      disposableLifecycle("https://example.test", "admin-secret", label),
    ).rejects.toThrow("earlier servers remove a grant's field restrictions");
    expect(
      request.mock.calls.some(([, options]) => options?.method === "PUT"),
    ).toBe(false);
    expect(
      request.mock.calls
        .filter(([, options]) => options?.method === "DELETE")
        .map(([url]) => url),
    ).toEqual(["https://example.test/api/auth/keys/new-secret"]);
    expect(request.mock.calls[request.mock.calls.length - 1]?.[0]).toBe(
      "https://example.test/api/auth/keys",
    );
    expect(request).toHaveBeenCalledTimes(10);
  } finally {
    vi.unstubAllGlobals();
  }
});
