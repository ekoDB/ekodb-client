# API-key administration

Use the separate admin entry point from a trusted server or owner tool. Returned
key records contain secrets; never log them.

```ts
import { EkoDBAdminClient } from "@ekodb/ekodb-client/admin";

const admin = new EkoDBAdminClient({
  baseURL: process.env.API_BASE_URL!,
  apiKey: process.env.API_BASE_KEY!,
  timeout: 30_000,
});
const key = await admin.createApiKey({
  label: "worker-replacement",
  collection_permissions: { jobs: ["read", "write"] },
});
const permissions = await admin.getPermissions();
```

Registration defaults to a non-admin key and an empty collection grant map. The
server generates the new key. Registration and listing expose `id` when the
server supplies one; older responses may omit it. Check `ripple_warning` before
installing a new key: it indicates incomplete propagation after local success.

## Permission route generations

Version 0.80.0 moves permission routes under `/api/auth/keys/{id}/permissions`.
The target is the server-generated key id or `me` for the calling key. New
permission methods reject key values as targets. Collection names and KV
prefixes are encoded as path segments, including `/`, spaces and literal `+`
characters. Dot-only facet names (`.` and `..`) are rejected before HTTP.

Each client probes `GET /api/auth/keys/me/permissions` and caches the successful
generation decision. A 200 selects the new routes; only 404 or 405 selects the
legacy routes. Authentication and other failures propagate without selecting a
generation. The authenticated transport can exchange a fresh token once after
401, including when an old token needs replacement after a server upgrade.

The route capability is authoritative: new routes work on `0.80.0-rc.N` and on
pre-cap servers that still report `0.79.2`. The new API's documented release
minimum is `MIN_SERVER_VERSION_FOR_PERMISSION_EDITS = "0.80.0"`. Legacy edits
retain `MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS = "0.79.2"`; their cached
health check rejects earlier, malformed, unreadable and prerelease legacy
versions because older implementations erase field restrictions.

`getPermissions()` reads the caller's grants through either generation. Its
public result preserves `is_admin` and `collection_permissions`, and exposes
`kv_permissions`. Modern `{admin, collections, kv}` responses are normalized to
those names. Field limits, empty arrays, null limits and additional metadata are
retained. On modern servers, `getApiKeyPermissions(id)` reads a key's aggregate
resource. Any valid token may read its own key; other-key reads and all edits
require an admin token.

## Modern collection and KV grants

```ts
if (await admin.usesKeyPermissionRoutes()) {
  if (!key.id) throw new Error("Registration omitted the key id");
  await admin.putApiKeyCollectionPermissions({
    key_id: key.id,
    collection: "jobs",
    read: true,
    write: true,
    writable_fields: ["status"],
  });
  const change = await admin.patchApiKeyCollectionPermissions({
    key_id: key.id,
    collection: "jobs",
    exclude_fields: null,
  });
  // change.before and change.after contain grants, or null.
  await admin.putApiKeyKvPermissions({
    key_id: key.id,
    prefix: "session:",
    read: true,
    write: false,
  });
  await admin.deleteApiKeyKvPermissions(key.id, "session:");
}
```

Collection and KV methods expose `put`, `patch` and `delete` operations. The
aggregate, list and single-facet read methods are `getApiKeyPermissions`,
`listApiKeyCollectionPermissions`, `getApiKeyCollectionPermissions`,
`listApiKeyKvPermissions` and `getApiKeyKvPermissions`. KV listing accepts an
optional prefix filter. The `*` prefix denotes the entire KV namespace.

PUT replaces a grant: collection read/write flags are required, and an omitted
field limit means unlimited. PATCH preserves omitted flags and limits, clears a
limit sent as null, and sets a supplied array exactly, including an empty array.
`updateApiKeyCollectionPermissions` is a PATCH convenience method with required
read/write flags. DELETE removes a grant. All modern edits return
`{before, after}` without a token. KV PUT requires at least one of read/write; a
KV PATCH that removes both rights removes the grant.

A changed grant applies to the key's next token. Already minted tokens retain
their previous grants until expiry; create a fresh scoped client for readback.

## Explicit legacy compatibility

```ts
if (!(await admin.usesKeyPermissionRoutes())) {
  await admin.legacyUpdateApiKeyCollectionPermissions({
    api_key: key.key,
    collection: "jobs",
    read: true,
    write: false,
    exclude_fields: null,
  });
}
```

The explicit legacy method retains the old secret-targeted body and key-record
response only for the detected old generation. It refuses to send that body to a
modern server. Modern facet methods fail explicitly on old servers; the old API
cannot represent the same per-facet verbs and change responses. No legacy
KV-edit payload is inferred from the new API.

`revokeApiKey(key.id ?? key.key)` prefers an id and supports older servers that
only expose key values. The SDK sends no revoke request body. Install and verify
a replacement before revoking the old key.

## Transport and verification

The facade reuses token caching, proactive refresh, JSON transport and request
timeouts. It permits one refresh after a definite 401, rejects redirects and
does not retry ambiguous registration or edit failures. Errors omit response
bodies, request URLs and exception causes. After ambiguous registration, look up
the unique disposable label before retrying. No unrestricted request method is
exposed.

Legacy tests retain the historical contract fixtures. The synthetic
`test-fixtures/admin-contract-0.80.0.json` fixture specifies modern path
encoding, PATCH states and change results; it is not a captured server response.
Rust, TypeScript, Kotlin and Python test public HTTP entry points against both
route generations; Go has matching tests in its own repository.

Use the repository Make targets for unit tests, compilation and example parity.
The matching `client_admin` example reads permissions by default;
`EKODB_ADMIN_LIVE=1` enables a uniquely labeled disposable-key lifecycle,
registration/list id checks and modern KV grant edits. Cleanup revokes only the
owned key and checks absence. Live validation of each generation requires a real
server serving that generation; a version label alone is insufficient.

Keep rotation coordination, labels, installation and application-specific
permission policy in the application. See the
[SDK parity matrix](../PARITY_MATRIX.md#api-key-administration).
