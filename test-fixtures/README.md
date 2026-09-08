# Search wire cases

`search-requests.json` is shared by Rust, TypeScript, and Kotlin tests. Each
client constructs the four requests through its public API and compares the
complete JSON value against the same fixture. Object key order is irrelevant;
fields, nulls, arrays, numbers, and strings are not removed or normalized by the
tests. The cases cover filtered vectors, custom hybrid weights, a named vector
field, and full-text fields/weights/fuzziness.

The fixtures explicitly set `bypass_ripple`, `bypass_cache`, and `limit` because
Rust serializes those three unset options as null, while TypeScript and Kotlin
omit them. Separate tests lock down that difference. This is not a claim that
null and omission are equivalent on every server. Rust and TypeScript builders
emit strings for `fields`/`weights`; Kotlin follows that representation and also
accepts field lists and numeric maps through its builder.

Run from the repository root:

```sh
cargo test -p ekodb_client --test search_wire
(cd ekodb-client-ts && npm ci && npm test -- src/search-wire.test.ts)
(cd ekodb-client-kt && ./gradlew test --tests io.ekodb.client.SearchTest)
```

These tests establish client wire agreement, not live-server ranking, index
selection, prefilter execution, or recall guarantees.
