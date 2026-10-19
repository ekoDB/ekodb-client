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

Each update replaces one collection grant. Server 0.79.0 (`2af52800`) accepts
only read/write flags on this endpoint and resets field restrictions to null.
This SDK does not accept field-restriction writes; JavaScript callers receive an
error if they supply those fields. Read existing grants before replacing a
restricted grant. The current server leaves its in-memory key cache stale after
a PUT, so newly authenticated tokens still carry the previous grant. The opt-in
live test fails here. Server correction and a passing rerun are required before
using permission updates for rotation or access control.

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

The live disposable tests on server 0.79.0 confirmed array registration,
permission readback, non-admin denial and cleanup, but failed because the
updated grant did not reach a newly authenticated token. The protocol probe also
confirmed object registration returns 400 and field restrictions sent to PUT are
ignored. See
[redacted evidence](https://github.com/ekoDB/ekodb-client/blob/main/test-fixtures/admin-contract-0.79.0.json)
in the repository. Both an admin Bearer JWT and the admin key in the
registration body are sent for deployed-server compatibility; the current
handler authenticates the body key. There is no format-probing retry during SDK
registration.

Offline tests include installing an actual npm tarball in an isolated consumer:

```sh
cd ekodb-client-ts
npm test -- src/admin.test.ts src/admin-package.test.ts
```

For an authorized disposable environment only, use the repository's example
configuration and run the opt-in lifecycle test. It creates one uniquely labeled
key, exercises all five operations, and revokes that key in cleanup:

```sh
source scripts/load-root-env.sh
export EKODB_ADMIN_LIVE=1
(cd ekodb-client-ts && npm test -- src/admin-live.test.ts)
```
