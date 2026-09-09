# Client Library Parity Matrix

**Last Updated:** September 8, 2026

**Client version:** 0.26.2 (Rust, Python, TypeScript, Kotlin); Go is maintained
in the separate `ekodb-client-go` repository.

## Status: Core parity with typed Kotlin search

The core parity baseline below was verified at v0.21.0. Search API coverage was
updated for v0.26.2; language-specific differences are noted separately.

> The v0.21.0 parity pass closed a set of per-client method gaps. Newly brought
> to parity:
>
> - **`kv_clear`** — now on Rust, Python, TypeScript, Kotlin, and Go.
> - **`find_by_id_with_projection`** — added to Rust and Python (already in
>   TypeScript, Kotlin, Go).
> - **WebSocket `cancelChat`** — added to TypeScript, Kotlin, and Python
>   (already in Rust and Go).
> - **`list_user_collections`** — now on all five clients: Python, TypeScript,
>   Kotlin, and Go (`ListUserCollections`, passing `exclude_internal=true`);
>   already in Rust.
> - **Python WebSocket `ws_batch_update`** — added (WS find-all is already
>   provided by the existing `find_all`, so no `ws_find_all` alias was added).
> - **WebSocket `unsubscribe`** — added to Rust, Python (`ws_unsubscribe`), and
>   Kotlin (already in TypeScript and Go). All five clients now send the
>   server-side `Unsubscribe` frame (`{type, messageId, payload.collection}`) in
>   addition to tearing down the local subscription, so subscriptions are
>   stopped explicitly rather than only on receiver drop / connection close.
> - **WebSocket msgpack binary transport** — all five clients now perform the
>   additive `Hello`/`Welcome` handshake and transparently switch to binary
>   msgpack frames when the server welcomes it (else stay JSON text). Internal,
>   back-compatible, no public API change.
> - **`extract_record_id`** — added to Python (already in Rust, TypeScript, Go,
>   Kotlin), so all five resolve a record's id by alias → `id` → `_id`.
> - **WebSocket `close()`** — added to Python (already in Rust, TypeScript, Go,
>   Kotlin) for deterministic teardown.
> - **Schema cache** — now first-class in Python (`Client.new(schema_cache=…)`)
>   and Kotlin (`Builder.schemaCache(…)`), each auto-wiring the cache into the
>   WebSocket client; matches the enable-then-auto-wire ergonomics of Rust/Go.
> - **`refresh_token`** — added to Go as public `RefreshToken` (already in Rust,
>   TypeScript, Python, Kotlin), so all five expose an eager token refresh.
>
> Note: the query-builder `regex()` filter has been removed from all clients
> until server-side regex filtering is available (tracked internally). Use
> `contains` / `startsWith` / `endsWith` instead.

All core features are implemented across all client libraries (Rust, Python,
TypeScript, Go, Kotlin). This includes:

- Core CRUD, batch operations, transactions
- Search (text, vector, hybrid), KV store, document TTL
- Chat sessions, models, streaming, branching, merging
- User Functions, Scripts, Query Builder, Schema Builder
- Goals, Tasks, Agents (full lifecycle)
- Schedule management (CRUD + pause/resume)
- KV document linking
- WebSocket: full CRUD parity (14 methods), subscriptions, chat streaming
- SSE subscriptions (for clients behind reverse proxies that block WS)
- Schema cache (in-memory LRU with TTL, realtime invalidation via SchemaChanged)
- `extractRecordId()` / `ExtractRecordID()` with custom `primary_key_alias`
  support
- Utility functions, field builders, value extractors
- Crypto stages: HMAC sign/verify, AES-256-GCM encrypt/decrypt, UUID v4, TOTP
  generate/verify, Base64/Hex encode/decode, Slugify (v0.18.0)
- Concurrency stages: IdempotencyClaim (SETNX + TTL), RateLimit (fixed-window
  via atomic increment), LockAcquire/Release (token-fenced)

**Admin-only endpoints intentionally excluded from clients:** query index
management (`create_query_index`, `list_query_indexes`, `delete_query_index`,
`explain_query`) and the three search-index explain helpers
(`explain_text_search`, `explain_vector_search`, `explain_hybrid_search`). These
require `admin_filter` auth and live on the server-side admin surface. See
ekodb_client/CHANGELOG.md "Removed" section under v0.16.0 for the rationale.

**Atomic KV primitives intentionally not exposed as direct client methods:**
`kv_increment` and `kv_set_if_absent` exist on the server but are reachable only
through stored-function concurrency stages (`IdempotencyClaim`, `RateLimit`,
`LockAcquire`). The stage path bundles TTL, fence-token, and idempotent-retry
semantics that direct client access would lose. Revisit if a customer asks.

## Search API

- Text, vector, and hybrid search — Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅
  | JavaScript ✅ | Kotlin ✅
- Kotlin provides `SearchQuery`, `SearchQueryBuilder`,
  `SearchResult`/`SearchResponse`, and typed `search` overloads. The request
  model covers the Rust/TypeScript search fields, including metadata filters,
  named vector fields, projections, and custom hybrid weights.
- Kotlin raw JSON search and record-list helpers remain available. Typed results
  expose scores, matched fields, totals, and optional execution time.
- Search uses JSON HTTP transport in Rust, TypeScript, and Kotlin, including
  when Kotlin's experimental MessagePack format is selected.

**Known API differences:** Rust emits unset cache flags and limit as null;
TypeScript and Kotlin omit them. Kotlin and TypeScript hybrid helpers leave
weights unset, while Rust's helper sets 0.5/0.5. Kotlin reuses the existing
`QueryBuilder` filter representation. Dynamic record fields are preserved;
unknown response-envelope fields require Kotlin's raw search overload.

Shared fixtures in `test-fixtures/search-requests.json` verify four complete
Rust/TypeScript/Kotlin request shapes. The Kotlin search example also passed a
live integration smoke test during
[#208](https://github.com/ekoDB/ekodb-client/pull/208#issuecomment-5580501554);
ranking, index selection, and ANN recall are not covered by those checks. See
[Testing Commands](COMMANDS.md#search-and-schema-compatibility).

## Chat Models API

- `getChatModels()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ | JavaScript
  ✅ | Kotlin ✅ (incl. `gemini` and the per-provider `providers` status map;
  Kotlin's typed form is `chatModels()`, beside the raw `getChatModels()`)
- `getChatModel(name)` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ |
  JavaScript ✅ | Kotlin ✅
- `getChatMessage(chatId, msgId)` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅
  | JavaScript ✅ | Kotlin ✅

Kotlin's `FieldSearchOptions.field` serializes as `field_name`, matching the
server contract. The Kotlin property name is unchanged.

## User Functions API

- `saveUserFunction()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ |
  JavaScript ✅ | Kotlin ✅
- `getUserFunction()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ | JavaScript
  ✅ | Kotlin ✅
- `listUserFunctions()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ |
  JavaScript ✅ | Kotlin ✅
- `updateUserFunction()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ |
  JavaScript ✅ | Kotlin ✅
- `deleteUserFunction()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ |
  JavaScript ✅ | Kotlin ✅

## Collection Operations

- `collectionExists()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ |
  JavaScript ✅ | Kotlin ✅
- `countDocuments()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ | JavaScript
  ✅ | Kotlin ✅

### Schema Compatibility

Schema field types are case-sensitive. Server 0.72.2 accepts both `Vector` and
`Array` with a vector index; lowercase `vector`, `string`, `integer`, and
`boolean` are rejected. Index configuration uses lowercase `type: "vector"`.
Python, Go, and TypeScript schema examples use `Array`; Kotlin's schema example
uses canonical scalar types.

**Kotlin (unreleased):** `FieldTypeSchemaBuilder` normalizes known type names to
server casing, retaining canonical inputs and unknown types. Regression tests
cover the accepted schema types. Python's builder still passes type strings
through, so callers must use canonical names despite its lowercase documentation
examples.

**Kotlin HTTP errors (unreleased):** `EkoDBHttpException` exposes terminal
status codes and response bodies, including exhausted server errors. See the
[Kotlin error-handling guide](ekodb-client-kt/README.md#error-handling).

---

## Completed Implementations - January 2026

**Go Client:**

- GetChatModels(), GetChatModel(name) - Chat Models API
- CollectionExists(collection), CountDocuments(collection) - Collection
  utilities
- Unit tests: `client_test.go`
- Integration examples: `examples/go/client_chat_models.go`,
  `examples/go/client_collection_utils.go`

**Python Client:**

- get_chat_models(), get_chat_model(name), get_chat_message() - Chat Models API
- save_user_function(), get_user_function(), list_user_functions(),
  update_user_function(), delete_user_function() - User Functions API
- collection_exists(), count_documents() - Collection utilities
- Unit tests: `tests/test_chat_user_functions.py`
- Integration examples: `examples/python/client_chat_models.py`,
  `examples/python/client_user_functions.py`,
  `examples/python/client_collection_utils.py`

**TypeScript/JavaScript Client:**

- getChatModels(), getChatModel(name), getChatMessage() - Chat Models API
- saveUserFunction(), getUserFunction(), listUserFunctions(),
  updateUserFunction(), deleteUserFunction() - User Functions API
- collectionExists(), countDocuments() - Collection utilities
- Unit tests: `src/client.test.ts` (17 new tests)
- Integration examples: `examples/typescript/client_chat_models.ts`,
  `examples/typescript/client_user_functions.ts`,
  `examples/typescript/client_collection_utils.ts`

**Kotlin Client:**

- saveUserFunction(), getUserFunction(), listUserFunctions(),
  updateUserFunction(), deleteUserFunction() - User Functions API
- Unit tests: `EkoDBClientTest.kt` (6 new tests)

---

## Testing Requirements

Each new feature must include:

1. **Unit Tests**
   - Success scenarios
   - Error handling (404, 401, 400)
   - Edge cases (empty results, invalid input)

2. **Integration Tests**
   - End-to-end workflows
   - Cross-feature interactions
   - Performance under load

3. **Examples**
   - Basic usage example
   - Advanced usage with all options
   - Real-world use case demonstration

4. **Documentation**
   - Method signatures
   - Parameter descriptions
   - Return value documentation
   - Usage examples in README

---

## Contributing

When implementing these features:

1. **Check server API docs** - Review the ekoDB server API documentation for
   endpoint details
2. **Follow existing patterns** - Match style of current client methods
3. **Add types first** - Define all TypeScript/Rust/etc types before
   implementation
4. **Test thoroughly** - Unit tests + integration tests + examples
5. **Update docs** - language-specific READMEs

---

## Questions?

- **Server API Documentation:** See the ekoDB server documentation
- **Current Client Status:** See this parity matrix
- **Implementation Examples:** Review existing methods in client source files
