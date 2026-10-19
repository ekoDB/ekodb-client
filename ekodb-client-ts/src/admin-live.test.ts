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
    const admin = new EkoDBAdminClient({ baseURL, apiKey });
    const label = `sdk-ts-admin-${randomUUID()}`;
    try {
      const created = await admin.createApiKey({
        label,
        collection_permissions: { sdk_admin_probe: ["read"] },
      });
      const createdKey = created.key;
      // Assert booleans rather than dumping secret-bearing records on failure.
      expect(created.is_admin).toBe(false);
      expect(created.collection_permissions).toEqual({
        sdk_admin_probe: { read: true, write: false },
      });
      expect(
        (await admin.listApiKeys()).filter((key) => key.label === label).length,
      ).toBe(1);
      const scoped = new EkoDBAdminClient({ baseURL, apiKey: createdKey });
      const before = await scoped.getPermissions();
      expect(before.is_admin).toBe(false);
      expect(before.collection_permissions.sdk_admin_probe.read).toBe(true);
      expect(before.collection_permissions.sdk_admin_probe.write).toBe(false);
      await expect(scoped.listApiKeys()).rejects.toThrow();
      await admin.updateApiKeyCollectionPermissions({
        api_key: createdKey,
        collection: "sdk_admin_probe",
        read: true,
        write: true,
      });
      const refreshed = new EkoDBAdminClient({ baseURL, apiKey: createdKey });
      const after = await refreshed.getPermissions();
      expect(after.collection_permissions.sdk_admin_probe.write).toBe(true);
    } finally {
      try {
        await cleanupDisposableKey(admin, label, apiKey);
      } catch {
        throw new Error(
          `Disposable key cleanup failed; revoke the key labeled ${label}`,
        );
      }
    }
  },
  60_000,
);

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
