# API-key administration

Use the existing npm package's separate entry point from a trusted server or
owner tool:

```ts
import { EkoDBAdminClient } from "@ekodb/ekodb-client/admin";

const baseURL = process.env.API_BASE_URL!;
const admin = new EkoDBAdminClient({
  baseURL,
  apiKey: process.env.API_BASE_KEY!,
  timeout: 30_000,
});
const key = await admin.createApiKey({
  label: "worker-replacement",
  collection_permissions: { jobs: ["read", "write"] },
});

// Returned key records contain secrets. Never log them.
const scoped = new EkoDBAdminClient({ baseURL, apiKey: key.key });
const permissions = await scoped.getPermissions();
if (permissions.is_admin || !permissions.collection_permissions.jobs?.read) {
  throw new Error("Unexpected worker permissions");
}
// Install and verify the replacement in your application before revoking an old key.
// await admin.revokeApiKey(oldKey);
```

`createApiKey` defaults to `is_admin: false` and an empty grant map. An admin
key requires explicit `is_admin: true`. `listApiKeys()` returns all
secret-bearing key records. `revokeApiKey(secret)` identifies a key by its
actual secret value, which the SDK encodes as one path segment.

Check `key.ripple_warning` before installing a new key: a warning means
registration succeeded locally but replication to peers failed.

`getPermissions()` reads the calling token's grants, including `select_fields`,
`exclude_fields` and `writable_fields`. Array-form registration responses are
converted to read/write objects; object-form responses preserve field
restrictions, including null and empty arrays, and additional server metadata.

```ts
await admin.updateApiKeyCollectionPermissions({
  api_key: key.key,
  collection: "jobs",
  read: true,
  write: false,
});
```

Each update replaces `read` and `write` for that collection. Each restriction
has three edit states:

| Request value                   | Effect                             |
| ------------------------------- | ---------------------------------- |
| Omitted or `undefined`          | Keep the current restriction       |
| `null`                          | Remove the restriction             |
| A string array (including `[]`) | Replace it with exactly that array |

```ts
await admin.updateApiKeyCollectionPermissions({
  api_key: key.key,
  collection: "jobs",
  read: true,
  write: false,
  // select_fields omitted: keep the current selection.
  exclude_fields: null, // Clear this restriction.
  writable_fields: ["status"], // Replace this restriction.
});
```

Earlier servers remove a grant's field restrictions on this call, so the method
refuses to run against them. The server fix ships in a later release. The
minimum release is **set by the owner**;
`MIN_SERVER_VERSION_FOR_PERMISSION_EDITS` is currently unset, so all permission
updates are blocked before any HTTP request. There is no caller override. Once
the minimum is set, the client reads the authenticated `/api/health` payload's
top-level `version` and caches that read per client instance. Unreadable, older,
malformed or prerelease versions fail closed. Create a new client after a server
upgrade to refresh that cached reading.

Restrictions in requests must be null or arrays of non-empty strings; undefined
is omitted. Response restrictions retain their exact arrays, empty arrays and
nulls. This package does not wrap the endpoint for editing the caller's own key.

The facade reuses the data SDK's token cache, proactive refresh, JSON transport,
and request timeout. It refreshes once after a definite 401. It does not retry
network failures, timeouts, rate limits or server failures, and rejects
redirects. After an ambiguous registration failure, inspect the uniquely labeled
key list before manually retrying. Errors omit server bodies, request URLs, and
exception causes because those can contain credentials. No unrestricted request
method is exposed by the admin facade. CommonJS `require` and ESM `import` both
support the root and `/admin` entry points.

Keep rotation locks, labels, installation and replacement verification in the
application. This API provides no delegation, tenant isolation, inherited
quotas, permission ceilings or cascading revocation. Collection grants do not
constrain stored-function internals; see the
[function permission model](https://docs.ekodb.io/reference/functions-architecture#permission-model).

## Contract verification

Registration takes action arrays only, for example
`collection_permissions: { jobs: ["read", "write"] }`. An object-form
registration body returns 400. Array-form registration responses remain
supported. Both an admin Bearer JWT and the admin key in the registration body
are sent for deployed-server compatibility.

The
[specified permission-edit fixture](https://github.com/ekoDB/ekodb-client/blob/main/test-fixtures/admin-permission-edits-specified.json)
is hand-written and marked "specified, to be re-recorded against the fixed
server". The
[historical 0.79.0 observations](https://github.com/ekoDB/ekodb-client/blob/main/test-fixtures/admin-contract-0.79.0.json)
record restriction erasure and stale token grants; tests never treat those
update responses as the accepted contract. The server fix ships in a later
release. There is no format-probing retry during SDK registration.

Offline tests include installing an actual npm tarball in an isolated consumer:

```sh
make test-typescript
```

For an authorized disposable environment only, use the repository's example
configuration and run the opt-in lifecycle test. It creates one uniquely labeled
key, exercises all five operations, and always revokes that key in a `finally`
block with matching-label absence verification. Its restriction round-trip is
refused until the same version gate passes; no existing key is updated:

```sh
source scripts/load-root-env.sh
EKODB_ADMIN_LIVE=1 make test-typescript
```
