/** Read-only by default; EKODB_ADMIN_LIVE=1 creates and revokes one disposable key. */
import { EkoDBAdminClient } from "@ekodb/ekodb-client/admin";
import { randomUUID } from "node:crypto";
import * as dotenv from "dotenv";
dotenv.config();
async function main() {
  const baseURL = process.env.API_BASE_URL!;
  const owner = process.env.API_BASE_KEY!;
  if (!baseURL || !owner)
    throw new Error("API_BASE_URL and API_BASE_KEY are required");
  const admin = new EkoDBAdminClient({ baseURL, apiKey: owner });
  const current = await admin.getPermissions();
  console.log("Admin permissions read successfully");
  if (process.env.EKODB_ADMIN_LIVE !== "1") return;
  if (!current.is_admin) throw new Error("An admin key is required");
  const label = `sdk-ts-admin-example-${randomUUID()}`;
  console.log(`Disposable cleanup label: ${label}`);
  try {
    const created = await admin.createApiKey({
      label,
      collection_permissions: { sdk_admin_probe: ["read"] },
    });
    if (created.key === owner || created.is_admin)
      throw new Error("Unexpected disposable key");
    const scoped = new EkoDBAdminClient({ baseURL, apiKey: created.key });
    if ((await scoped.getPermissions()).is_admin)
      throw new Error("Unexpected admin grant");
    const modern = await admin.usesKeyPermissionRoutes();
    if (modern) {
      if (
        !created.id ||
        !(await admin.listApiKeys()).some(
          (key) => key.id === created.id && key.label === label,
        )
      )
        throw new Error("Registration/listing omitted key id");
      const prefix = `sdk-admin:${label}:`;
      const change = await admin.putApiKeyKvPermissions({
        key_id: created.id,
        prefix,
        read: true,
        write: false,
      });
      if (!change.after?.read || change.after.write)
        throw new Error("KV grant creation failed");
      await admin.patchApiKeyKvPermissions({
        key_id: created.id,
        prefix,
        write: true,
      });
      if (
        (await admin.deleteApiKeyKvPermissions(created.id, prefix)).after !==
        null
      )
        throw new Error("KV grant removal failed");
    }
    const edit = async (
      body: {
        collection: string;
        read: boolean;
        write: boolean;
        select_fields?: string[] | null;
        exclude_fields?: string[] | null;
        writable_fields?: string[] | null;
      },
      replace = false,
    ) => {
      if (modern) {
        const request = { ...body, key_id: created.id! };
        return replace
          ? admin.putApiKeyCollectionPermissions(request)
          : admin.updateApiKeyCollectionPermissions(request);
      }
      return admin.legacyUpdateApiKeyCollectionPermissions({
        ...body,
        api_key: created.key,
      });
    };
    await edit(
      {
        collection: "sdk_admin_probe",
        read: true,
        write: true,
        select_fields: ["public"],
        exclude_fields: ["secret"],
        writable_fields: ["public"],
      },
      true,
    );
    const initial = (
      await new EkoDBAdminClient({
        baseURL,
        apiKey: created.key,
      }).getPermissions()
    ).collection_permissions.sdk_admin_probe;
    if (
      !initial?.read ||
      !initial.write ||
      JSON.stringify(initial.select_fields) !== '["public"]' ||
      JSON.stringify(initial.exclude_fields) !== '["secret"]' ||
      JSON.stringify(initial.writable_fields) !== '["public"]'
    )
      throw new Error("Initial restriction readback failed");
    await edit({
      collection: "sdk_admin_probe",
      read: true,
      write: false,
      exclude_fields: null,
      writable_fields: [],
    });
    const grant = (
      await new EkoDBAdminClient({
        baseURL,
        apiKey: created.key,
      }).getPermissions()
    ).collection_permissions.sdk_admin_probe;
    if (
      !grant?.read ||
      grant.write ||
      JSON.stringify(grant.select_fields) !== '["public"]' ||
      grant.exclude_fields !== null ||
      JSON.stringify(grant.writable_fields) !== "[]"
    )
      throw new Error("Restriction readback failed");
  } finally {
    const matches = (await admin.listApiKeys()).filter(
      (key) => key.label === label,
    );
    if (matches.length > 1 || matches.some((key) => key.key === owner))
      throw new Error("Unexpected disposable cleanup match");
    for (const key of matches) await admin.revokeApiKey(key.id ?? key.key);
    if ((await admin.listApiKeys()).some((key) => key.label === label))
      throw new Error(`Disposable cleanup failed for label ${label}`);
  }
  console.log("Disposable admin lifecycle verified; no matching key remains");
}
main().catch(() => {
  console.error(
    "Admin example failed; use the disposable cleanup label printed above if needed",
  );
  process.exitCode = 1;
});
