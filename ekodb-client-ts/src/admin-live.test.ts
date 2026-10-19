import { randomUUID } from "node:crypto";
import { expect, it, vi } from "vitest";
import { EkoDBAdminClient } from "./admin";

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
    // The SDK gate refuses every PUT until the fixed release is set and verified.
    await admin.updateApiKeyCollectionPermissions({
      api_key: createdKey,
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
    await admin.updateApiKeyCollectionPermissions({
      api_key: createdKey,
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
  for (const key of matches) await admin.revokeApiKey(key.key);
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

it("cleans up its disposable key when the unset version gate refuses the round-trip", async () => {
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
  request.mockResolvedValueOnce(new Response(JSON.stringify([record])));
  request.mockResolvedValueOnce(new Response(null, { status: 204 }));
  request.mockResolvedValueOnce(new Response("[]"));
  vi.stubGlobal("fetch", request);
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
    expect(request).toHaveBeenCalledTimes(9);
  } finally {
    vi.unstubAllGlobals();
  }
});
