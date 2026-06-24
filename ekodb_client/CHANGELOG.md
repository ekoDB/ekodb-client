# Changelog

All notable changes to ekodb_client will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Client-tool keepalive for in-flight SSE chats, across all clients
  (ekoDB#530).** Sends a liveness keepalive (not a result) for a pending client
  tool on an active SSE chat stream, so a slow human confirmation or a
  long-running client tool resets the server's per-tool wait deadline
  (`client_tool_timeout_secs`, default 60s) instead of letting the turn time out
  mid-response. Call periodically (well under the timeout) while a confirmation
  prompt is shown or a tool is executing. Added with full parity across the
  language clients (mirrors each one's `submit_chat_tool_result`):
  - Rust: `Client::submit_chat_tool_keepalive(chat_id, call_id)`
  - Python: `Client.submit_chat_tool_keepalive(chat_id, call_id)`
  - TypeScript: `submitChatToolKeepalive(chatId, callId)`
  - Kotlin: `submitChatToolKeepalive(chatId, callId)`
  - (Go client is versioned separately — see ekodb-client-go
    `SubmitChatToolKeepalive`.)
- **Metadata pre-filter for text, vector, and hybrid search (#475).** Search now
  carries an optional `filters` field holding a canonical `QueryExpression` (the
  same AST as `find`); only records matching the filter are considered as
  candidates before ranking. Exposed as `.filters(...)` on the search query
  builder in the Rust, TypeScript, and Go clients, a `filters=` keyword argument
  on the Python client's `search`, and forwarded transparently by the Kotlin
  client's JSON-object `search`. Applies uniformly to text, vector, and hybrid
  search (including the indexed HNSW path). Each client's `examples/` search
  example now demonstrates a filtered search.

## [0.21.0] - 2026-06-09

### Fixed — examples test harness

- **Python example requirements no longer pin an impossible version.**
  `examples/python/requirements.txt` and `requirements-dev.txt` pinned
  `python-dotenv>=2.8.0`, but `python-dotenv`'s latest release is `1.2.2` (there
  is no 2.x — the `2.x` line is the unrelated Node `dotenv` package), so
  `pip install -r requirements.txt` failed to resolve. Corrected to
  `python-dotenv>=1.0.0` (installs the real latest, `1.2.2`) and aligned
  `websockets>=14.0` to match the client's requirement and the sibling
  `examples/requirements.txt`.

- **Dependency refresh.** Kotlin client + its examples: ktor `3.4.1 → 3.5.0`,
  kotlinx-coroutines `1.10.2 → 1.11.0`, kotlinx-serialization (json + cbor)
  `1.10.0 → 1.11.0` (Kotlin compiler is 2.3.0, comfortably compatible; full
  Kotlin test suite green). TypeScript examples: `@types/node` and `tsx` updated
  within their existing caret ranges (tsc clean).

- **Python direct examples no longer fail with `ModuleNotFoundError`.** The
  example test runner (`examples/python/test_runner.py`) launched each example
  with the hardcoded system `python3` instead of the interpreter running the
  runner. Since `make test-examples` runs it via the project `.venv` (which has
  `requests` / `aiohttp` / `python-dotenv` from `examples/requirements.txt`),
  the system interpreter lacked those deps and all 10 Python direct examples
  failed even though the venv was set up correctly. It now uses
  `sys.executable`. (`examples/test-examples.md` still shows the old failures
  until `make test-examples` is re-run against a server.)

### Added

- **Client parity fills (Python).** Closed the gaps where Python lagged the
  other clients:
  - The WebSocket client now exposes `close()` for deterministic teardown
    (matching every other WS client).
  - A module-level `extract_record_id(record, extra_candidates=None)` resolves a
    record's id trying custom aliases first, then `id`, then `_id` (handling
    typed-wrapper id fields) — backed by the canonical
    `ekodb_client::extract_record_id`, so Python callers never hardcode
    `id`/`_id`.
  - The **schema cache** is now reachable from Python: `Client.new(...)` accepts
    `schema_cache` / `schema_cache_ttl_secs` / `schema_cache_max`, and a
    WebSocket client created from a cache-enabled client automatically shares
    that cache (so WS CRUD is alias-aware and `SchemaChanged` events invalidate
    it). (#152)
  - Fixed a malformed `__all__` (`([...],)` — a tuple wrapping the list) that
    broke `from ekodb_client import *`; it is now a flat list of names.

- **Schema cache is now first-class in the Kotlin client.**
  `EkoDBClient.Builder` gained `schemaCache(enabled)` / `schemaCacheTtlMs(ms)` /
  `schemaCacheMax(max)`, and `EkoDBClient.websocket(url)` auto-wires the
  client's cache into the WebSocket client (so WS CRUD is alias-aware and
  `SchemaChanged` events invalidate it) — matching the enable-then-auto-wire
  ergonomics of the Rust, Python, and Go clients (previously a Kotlin user had
  to assign `ws.schemaCache` by hand).

- **WebSocket msgpack negotiation across all clients (transparent binary
  transport).** On every WS (re)connect each client now performs an additive
  `Hello`/`Welcome` handshake: it offers msgpack and, if the server welcomes it,
  transparently switches that connection to binary msgpack frames for both
  requests and responses; otherwise it stays on JSON text. The negotiation is
  internal — no public API changes — so callers (including ekodb-claw, which
  rides the Rust client) are unaffected. Fully back-compatible: a server that
  does not welcome msgpack (or an older server that never answers) leaves the
  connection on JSON. Incoming binary frames decode value-identically to JSON
  (binary fields stay number arrays, not base64), so decoded data is the same
  regardless of negotiated transport. Implemented in the Rust client (and thus
  the Python binding), TypeScript, and Kotlin; the Kotlin client gains a
  `msgpack-core` dependency since CBOR is not wire-compatible with the server's
  msgpack. (The Go client carries the same change in its own repository.)

- **Buffered-transaction support across all clients (read-your-writes +
  savepoints).** ekoDB transactions are now enforced and buffered server-side:
  statements carrying a `transaction_id` are staged and applied atomically at
  statements carrying a `transaction_id` are staged and applied atomically at
  commit (invisible to others until then). The clients now expose the full
  lifecycle and, critically, let **reads** join a transaction so they see the
  transaction's own staged writes (read-your-writes):
  - Reads accept a transaction id: Rust `find_by_id_in_transaction` /
    `find_in_transaction`; TypeScript `findById(c, id, { transactionId })` and
    `find(c, q, { transactionId })`; Python `find_by_id(…, transaction_id=…)` /
    `find(…, transaction_id=…)`; Kotlin `findById(c, id, transactionId=…)` /
    `find(c, q, transactionId=…)`.
  - Savepoints on every client: `create_savepoint` / `rollback_to_savepoint` /
    `release_savepoint` (camelCase per language).
  - Transactional **delete** now works in Rust/Python (added
    `delete_with_options`); the Python `delete(transaction_id=…)` previously
    accepted the argument but silently discarded it — fixed.
  - `commit` may surface a retryable conflict (HTTP 409) when a record the
    transaction read or wrote was changed by another committed transaction;
    documented on each client's begin/commit.

- **`kv_clear` on all clients (#148).** Clears the entire KV store via
  `DELETE /api/kv/clear`. Added to Rust (`kv_clear`), Python (`kv_clear`),
  TypeScript (`kvClear`), and Kotlin (`kvClear`); pairs with the Go client's
  `KVClear`.
- **`find_by_id_with_projection` on Rust and Python (#145).** Field-projection
  find-by-id (`select_fields` / `exclude_fields`) was already in TS/Kotlin/Go;
  added the Rust (`find_by_id_with_projection`) and Python
  (`find_by_id_with_projection`) equivalents for parity.
- **Python WebSocket `ws_batch_update` (#146).** The Python `WebSocketClient`
  could not batch-update over WS. (No `ws_find_all` was added: WS find-all is
  already provided by the existing `find_all` method, so a second name would
  duplicate it.)
- **WebSocket `cancelChat` on TypeScript, Kotlin, Python (#144).** Aborting an
  in-flight streaming chat over WSS was only possible from Rust and Go; the
  three remaining clients now send the same `CancelChat` frame.
- **`list_user_collections` on Python, TypeScript, Kotlin (#147).** Lists
  collections excluding internal chat/system collections
  (`GET /api/collections?exclude_internal=true`); previously Rust-only.
- **WebSocket `unsubscribe` now on all clients (#149).** Previously only the
  TypeScript (and Go) client sent a server-side `Unsubscribe` frame; the Rust,
  Python, and Kotlin WebSocket clients now expose `unsubscribe` and send the
  same best-effort `Unsubscribe` frame (with a unique messageId so the ack is
  dropped, not misrouted), so a client can tell the server to stop streaming a
  collection's mutations instead of relying on connection teardown. Full
  WebSocket unsubscribe parity across Rust / Python / TypeScript / Kotlin / Go.
  Frame-shape tests added per client.

### Fixed

- **All clients now percent-encode URL path segments (#153).** Request paths
  that interpolated a caller-supplied segment (collection, id, KV key, function
  label, chat model name, agent name, …) did NOT encode reserved characters — so
  a KV key like `session/abc` or a model name like `anthropic/claude-3` (both
  perfectly normal) produced a malformed URL the server 404'd. This was a
  **cross-client correctness + parity bug**: each client only encoded _some_
  paths (Rust none, TypeScript only KV, Kotlin only a few), none complete. Now
  every caller path segment is encoded in all of them — **Rust** routes all 59
  sites through the `api_path_url` helper; **TypeScript** wraps every path
  segment in `encodeURIComponent`; **Kotlin** uses ktor's `encodeURLPathPart()`;
  the **Python** binding inherits the Rust fix. (The standalone **Go** client is
  fixed in its own repo.) Behavior is unchanged for segments without reserved
  characters. Each client adds reserved-char encoding tests (`/`→`%2F`,
  space→`%20`, `#`, `?`).

- **Python `ekodb_client.__version__` is no longer stale.** It was hardcoded to
  `"0.1.0"` while the package was `0.21.0` (nothing synced it). It now derives
  from the installed distribution metadata (set by maturin from
  pyproject/Cargo), so it always matches the real package version. Covered by
  `tests/test_version.py`.

- **Kotlin `executeWithRetry` no longer wastes work — or hides the real status —
  on the final attempt.** A 401 (token refresh) or 429 (Retry-After sleep) on
  the last attempt would `return@repeat`, fall out of the loop without retrying,
  and then throw a generic "Request failed after N attempts" with no underlying
  cause — wasting a token refresh / sleeping the caller for nothing. The 401/429
  branches now guard on `attempt < maxRetries - 1` (matching the 5xx branch), so
  the final attempt falls through to the `4xx` handler and surfaces the real
  status + body. Covered by `401`/`429`-on-final-attempt tests plus a non-final
  401 refresh-and-retry test.

- **WebSocket `unsubscribe` tells the server to stop streaming (#149).** It sent
  only a local teardown, so the server kept streaming mutations for the
  collection until the connection dropped; it now also sends the best-effort
  `Unsubscribe` frame (see Added above for the cross-client parity work).
- **Kotlin: retry delays are now bounded (#142).** A server `Retry-After` is
  clamped to `MAX_RETRY_AFTER_SECONDS = 60` and 5xx/exception backoff to
  `MAX_BACKOFF_MS = 30_000`. The generic exception-retry path previously used an
  uncapped exponential delay; it now shares the same `clampBackoffMs` helper, so
  every retry path is bounded.
- **Rust `find_by_id_with_projection` now URL-encodes projection fields.** The
  `select_fields` / `exclude_fields` query was built by raw string
  concatenation, so a field name containing a reserved character (`,`, `&`, `=`,
  spaces, …) produced an ambiguous URL and the server received the wrong
  projection. It now builds the query with `Url::query_pairs_mut()` for proper
  percent-encoding, matching the `URLSearchParams` encoding the TypeScript
  client uses (the Python binding inherits the fix). Covered by
  `test_find_by_id_with_projection_encodes_reserved_chars`.

### Security

- **Transaction API path segments are now percent-encoded.** `transaction_id`
  and the user-supplied savepoint `name` were interpolated raw into request
  paths, so a name containing a reserved character (`/`, space, `?`, `#`, …)
  produced a malformed or wrong path (a `/` could escape its segment). Rust now
  builds these paths via `Url::path_segments_mut()` and Kotlin via
  `encodeURLPathPart()` (the Go client uses `url.PathEscape`), encoding space as
  `%20` and `/` as `%2F`, across create / rollback-to / release savepoint and
  the commit / rollback / status endpoints. The Python binding inherits the Rust
  fix. Tests added per client.
- **Rust `delete_with_options` now percent-encodes its query.** It built the
  `transaction_id` / `bypass_ripple` query by raw string concatenation; it now
  uses `Url::query_pairs_mut()`, matching the find methods.
- **WebSocket `cancelChat` now carries a `messageId` (TypeScript, Kotlin).** The
  `CancelChat` frame had no correlation id, so a server ack could be misrouted
  to an unrelated pending request by the dispatcher's single-pending fallback.
  It now includes a unique messageId (same pattern as `unsubscribe`), so the ack
  is safely correlatable and ignorable. Tests added.

### Documentation

- **Refreshed `PARITY_MATRIX.md` (#143)** — replaced the blanket "full parity"
  claim with the actual state and recorded the v0.21.0 parity additions,
  including WebSocket `unsubscribe` now present on all five clients (the one
  previously known remaining difference, now resolved).

## [0.20.0] - 2026-06-04

### Added

- **Kotlin: query builder gains `startsWith` / `endsWith`** (#129). The Kotlin
  `QueryBuilder` only had `contains`, even though the README already showed an
  `.endsWith(...)` example and the server exposes `StartsWith`/`EndsWith` filter
  operators. The two methods are now implemented (matching the Rust, TypeScript,
  Python, and Go builders), so the regex-removal guidance ("use
  contains/startsWith/endsWith") is actually actionable in Kotlin. README API
  reference updated; tests added.
- **Kotlin: query builder gains logical operators, `page`, and `rawFilter`.**
  The Kotlin `QueryBuilder` was missing the `and` / `or` / `not` logical
  combinators, the `page(page, pageSize)` pagination helper, and the `rawFilter`
  escape hatch that the Rust, TypeScript, Python, and Go builders all expose —
  so a Kotlin caller could not express an OR query (or any nested logical group)
  through the builder at all and could only AND conditions implicitly. Added all
  of them, plus a `QueryBuilder.condition(field, operator, value)` companion
  factory (mirroring Python's `condition`) for constructing the standalone
  operands that `and` / `or` / `not` take. README API reference updated; tests
  added.

### Removed

- **Removed the query-builder `regex()` filter** from all clients — the server
  has no regex filter operator, so it 400'd (or, in Rust, silently fell back to
  substring `Contains`). Removed until server-side regex filtering is available
  (tracked internally). Breaking: callers using `regex()` should switch to
  `contains` / `startsWith` / `endsWith`. Also removed the unreachable
  `QueryOperator.Regex` variant from the Kotlin `types/Query.kt` sealed class so
  no client type advertises a server operator that does not exist.

### Fixed

- **Python WebSocket examples updated for `websockets` >= 14.** The raw
  `websockets` example scripts (`examples/python/client_websocket_subscribe.py`
  and `simple_websocket.py`) passed `extra_headers=` to `websockets.connect`,
  which was removed in `websockets` 14 (renamed to `additional_headers`), so
  they raised `TypeError: ... unexpected keyword argument 'extra_headers'` on
  the installed `websockets` 16. Switched to `additional_headers` and bumped the
  examples' `requirements.txt` floor to `websockets>=14.0` so the pin matches
  the code. (Examples only — the Python client library is a Rust/pyo3 binding
  and does not use the `websockets` package.)
- **TypeScript: base URLs with a trailing slash no longer break requests.** The
  client appended paths by plain string concatenation (`${baseURL}${path}` for
  REST, `wsURL + "/api/ws"` for WebSocket), so a base URL passed with a trailing
  slash (e.g. `wss://host/`) produced a double-slash path (`//api/ws`). The
  server matches WS on the exact `api / ws` route, so the empty leading segment
  caused the connection to fail (and some proxies reject `//api/...` for REST
  too). Both the `EkoDBClient` and `WebSocketClient` constructors now strip
  trailing slashes from the supplied URL via a shared linear-scan helper (rather
  than a `/\/+$/` regex, which CodeQL flags as polynomial-time backtracking on
  caller-supplied input). Tests assert the request path is `/api/ws` (and the
  REST auth URL has no `//api`) for trailing-slash inputs.
- **TypeScript: WebSocket subscriptions now auto-reconnect and requests no
  longer hang forever** (#127). The WS client previously snapshotted the auth
  token once at construction, never reconnected after a transient socket drop,
  and registered request/response promises with no timeout — a response that
  never arrived left the promise pending indefinitely. Now: (1) the token is
  re-evaluated on every (re)connect via the client's existing
  `getToken()`/`refreshToken()` path, so a reconnect after a token rotation uses
  the current token instead of a stale one; (2) an unexpected close/error
  (anything other than an explicit `close()`/`unsubscribe()`) triggers an
  automatic reconnect with capped exponential backoff plus jitter (200ms → ~5s)
  that re-sends the `Subscribe` frames so the same `EventStream` keeps
  delivering mutations; and (3) every request/response call has a configurable
  per-request timeout (default 30s) that rejects with a clear error, and all
  in-flight requests are rejected on disconnect so callers fail fast rather than
  hang. Reconnect/timeout tunables are configurable via the new
  `WebSocketClientOptions` argument to `client.websocket(url, options)`.

- **Rust: all network ops now route through token auto-refresh** (#131). A 401
  refreshes the token and retries the operation once instead of surfacing
  `TokenExpired` to the caller — previously only 18 of the high-level `Client`
  methods did this and the rest leaked the error. In the same fix,
  `should_retry(false)` is now honored by every op: the low-level `HttpClient`
  request methods called the retry policy directly, ignoring the gate, so
  retries always ran; they now go through `execute_with_retry`, which disables
  retries when `should_retry` is false.

- **Python: complex `FieldType` values now read and write as native Python
  types** (#122). On read, the binding Debug-stringified every variant beyond
  String/Integer/Float/Boolean/Array/Object, returning garbage like
  `"Binary([255, 216, ...])"`. It now returns faithful native types:
  `Binary`/`Bytes` → `bytes`, `DateTime` → timezone-aware `datetime.datetime`
  (UTC), `UUID` → `uuid.UUID`, `Decimal`/`Number(Decimal)` → `decimal.Decimal`
  (precision-preserving), `Duration` → `datetime.timedelta`,
  `Number(Integer/Float)` → `int`/`float`, `Set` → `set` (falling back to a
  `list` when an item is unhashable), and `Vector` → `list`. On write,
  `py_to_field_type` now accepts `bytes`/`bytearray` → `Binary`,
  `datetime.datetime` → `DateTime`, `uuid.UUID` → `UUID`, `decimal.Decimal` →
  `Decimal`, and `set`/`frozenset` → `Set`, in addition to the existing
  scalar/list/dict handling. Covered by Rust round-trip tests over a live
  interpreter.
- **Python: rate-limited (HTTP 429) operations now raise `RateLimitError`**
  (#123). The custom `RateLimitError` exception was exported but never thrown —
  every network op wrapped errors in a generic `RuntimeError`, so callers could
  not distinguish a 429 or read the retry-after. A new `map_client_err` helper
  maps `Error::RateLimit { retry_after_secs }` to `RateLimitError(retry_after)`
  (the retry-after is the first exception arg) and all other errors to
  `RuntimeError`, and it is wired into every network call site
  (insert/find/update/delete/batch/paginate/exists/upsert/search/chat/WS/etc.).
  Covered by unit tests asserting the exception type and retry-after.
- **TypeScript: `chatMessageStream` streams incrementally** (#125). It buffered
  the entire response via `await response.text()` before emitting, so `chunk`
  events only fired once the whole reply had arrived. It now reads
  `response.body` and emits each SSE event as it arrives (reassembling lines
  split across chunk boundaries), with a `text()` fallback for environments
  without a readable body stream. Regression test added.
- **TypeScript: retries use exponential backoff with jitter** (#126). The 503
  and network-error retries used fixed 10s / 3s delays; they now use a capped
  exponential schedule (0.2s → 5s) with full jitter, and the 429 `Retry-After`
  is capped at 60s. Covered by a backoff-bounds test.
- **`getValue` no longer corrupts user objects** (#134). Across the Rust,
  TypeScript, Python, and Kotlin clients, `getValue` unwrapped any object that
  had a `value` key; a legitimate object like `{ value: 1, currency: "USD" }`
  lost its sibling fields on read. In the Kotlin client this affected the raw
  `Map` path in `getValue` (the `FieldType.ObjectValue` path already guarded on
  both keys). The TypeScript `extractRecordId` helper had the same unguarded
  unwrap in a separate code path (it pulled `.value` off any object when
  resolving a record's id/alias) and is now held to the same `type`+`value`
  rule, matching how the Rust and Kotlin id extractors already delegate to the
  guarded `get_value`/`getValue`. All now only unwrap a genuine typed wrapper
  (both `type` and `value` present) and pass every other object through
  untouched. Regression tests added per language. (The Go client had the same
  `getValue` bug, fixed separately; Python and Go have no separate id-extractor
  helper.)
- **Rust: WebSocket request IDs are now collision-free** (#132).
  `gen_message_id` used a nanosecond wall-clock value, so two requests in the
  same tick could collide and hang/mis-route a caller. It now uses a
  process-wide monotonic atomic counter.
- **Rust: retry backoff is capped and overflow-safe** (#133). The exponential
  backoff could overflow (`2^attempt`) and an unbounded server `Retry-After` was
  honored verbatim. Backoff now uses a bounded exponent + saturating math
  clamped to 30s, and `Retry-After` is capped at 60s (`TestRetryPolicy` backoff
  test). Also replaced a network-path `.unwrap()` in `health_check` with error
  propagation and removed an `_`-prefixed binding.
- **TypeScript: `chatMessageStream` now awaits `getToken()`** (#124). The token
  was read without `await`, so the SSE chat request sent
  `Authorization: Bearer [object Promise]` and every streamed chat 401'd (the
  `if (!token)` refresh branch was also dead, since a Promise is always truthy).
  Added a regression test asserting the request carries the resolved Bearer
  token.
- **Kotlin: 401 retry now uses the refreshed token** (#128). Request methods
  captured the token once before `executeWithRetry`, so a 401 that triggered
  `refreshToken()` still retried with the stale token and never recovered.
  `executeWithRetry` now fetches the token fresh on every attempt and passes it
  into the request block, so the retried attempt carries the refreshed token.
  The same path also no longer swallows `CancellationException` (it is rethrown
  so structured coroutine cancellation keeps working), and the `Logging` plugin
  redacts the `Authorization` header as defense-in-depth (INFO level already
  omits headers and bodies). Regression tests added (401→refresh→retry asserts
  the new bearer token; cancellation rethrow asserts no retry).
- **Kotlin: `getRecordId` resolves `primary_key_alias` / `_id`** (#130). It
  hardcoded `record["id"]`, so collections with a custom primary-key alias
  returned `null`. A new overload accepts alias candidates and tries them in
  order, then `id`, then `_id`; the no-argument form now also falls back to
  `_id`. Unit tests added for alias, `_id`, wrapped-value, and plain-`id`
  resolution.

### Changed

- **Python client now requires Python >= 3.9** (was `>= 3.8`). Python 3.8
  reached end-of-life in October 2024, and the modern `websockets` library used
  by the examples dropped 3.8 in v14. Bumped `requires-python` and removed the
  `Python :: 3.8` classifier in `ekodb-client-py/pyproject.toml` so the
  published support matrix is accurate. The pyo3 binding itself is unaffected;
  this is a metadata/support-policy change.
- **Repository metadata, license, and hygiene** (#135): pointed the
  `ekodb_client` crate `repository` field at the public `ekoDB/ekodb-client`;
  corrected the declared license to `MIT` in the Rust and Python manifests (only
  a MIT `LICENSE` file ships, matching the TypeScript and Kotlin clients);
  removed two committed compiled Go example binaries and added ignore rules for
  them; removed internal server-path references from test comments; refreshed
  `PARITY_MATRIX.md` to the current 0.19.0 line (parity re-verification for
  0.19.0 noted as in progress).

## [0.19.0] - 2026-06-02

### Fixed — Function examples now save-or-update on `409 Conflict`

The ekoDB server now makes `POST /api/functions` create-only, returning **409
Conflict** when a `label` already exists (use `PUT /api/functions/:label` to
update). Examples that save a fixed-label stored function therefore failed on a
re-run against a server that already held the label
(`A function with label 'get_active_users' already exists ...`). Every affected
example across all six languages now does **save-or-update**: it `POST`s the
function, and on a 409 it `PUT`s the same definition by label and continues,
exercising both the create and update paths. Where the example later needs the
function id (for by-id management or cleanup), it resolves the id by label via
`GET /api/functions/:label` on the 409 path, so the existing end-of-example
cleanup is unchanged. This was applied to **every** function-saving example (95
files across `examples/{rust,python,typescript,javascript,go,kotlin}` — the
`client_functions*`, `http_functions`, `client_user_functions`,
`client_function_composition`,
`client_functions_{advanced,ai,complete,crud,kv_wrapped,search}`,
`client_{concurrency,crypto}_stages`, `client_jwt_auth_flow`,
`client_path_routed_function`, `client_edge_cache`, `client_swr_native`,
`client_swr_pattern`, and `swr_pattern` variants). Each per-language build /
typecheck passes (`cargo build --examples`, `tsc --noEmit`, `go build`,
`node --check`, `py_compile`, `gradlew compileKotlin`). No client-library code
changed; each example carries a small local `save_or_update` helper. A few
TypeScript examples that previously swallowed save errors with
`.catch(() => {})` now surface non-409 failures properly.

### Security — Bump `ws` to `^8.20.1` (TypeScript client + example projects)

Bumped the `ws` floor to `^8.20.1` (resolves to `ws` 8.21.0) in the TypeScript
client and both bundled example projects (`examples/typescript` was `^8.18.0`,
`examples/javascript` was `^8.5.0`), patching GHSA-58qx-3vcg-4xpx — a moderate
uninitialized-memory-disclosure issue affecting `ws` 8.0.0–8.20.0. `npm audit`
reports zero vulnerabilities across all three; the client build and full vitest
suite (381 tests) pass.

### Added — On-demand chat compaction: `compact_chat` (ekodb #43)

All clients gained a method to compact a chat session's history on demand via
the new server endpoint `POST /api/chat/{id}/compact`: it folds the older
messages into a summary message and marks the originals forgotten, reclaiming
context-window budget. Returns
`{folded, kept_recent, summary_chars, summary_message_id, already_compact}`.

- **Rust** (`ekodb_client`):
  `client.compact_chat(chat_id, keep_recent: Option<usize>)` →
  `CompactChatResponse` (new `CompactChatRequest` / `CompactChatResponse` types,
  exported from the crate root).
- **TypeScript**: `client.compactChat(chatId, keepRecent?)` →
  `CompactChatResponse`.
- **Python** (PyO3 binding): `client.compact_chat(chat_id, keep_recent=None)` →
  dict.
- **Kotlin**: `client.compactChat(chatId, keepRecent?)` → `CompactChatResponse`.

`keep_recent` defaults server-side to the session's `max_context_messages` (or
50); `0` compacts the entire history. Each client mirrors its existing
chat-method conventions (auth, retry, serialization) and ships with unit tests
for the new method.

### Fixed — Rust SWR `flexible_cache` example pointed at a non-resolving host (ekodb-client)

The Rust `client_swr_native` example's `flexible_cache` (dynamic-TTL) function
set its server-side SWR fetch URL to `https://api.ekodb.net/api/health`, a host
that does not resolve. The server's fetch failed with
`400 HTTP request failed: error sending request`, and because the example uses
`?` on the call it **aborted the entire Rust example run**
(`make test-examples-rust-client` exited non-zero), which is why a single bad
line cascaded into "a lot of fails". Every other language's `flexible_cache`
(Go, Python, TypeScript, JavaScript, Kotlin) already used
`https://jsonplaceholder.typicode.com/posts/{{resource_id}}`; the Rust example
(`examples/rust/examples/client_swr_native.rs:227`) now matches, restoring
cross-language parity and unblocking the suite. (Net change versus 0.18.2: the
URL went from `https://app.ekodb.io/api/health` → the broken `api.ekodb.net`
host → the reachable jsonplaceholder endpoint used everywhere else.)

Note: saved functions are keyed by `label` and resolved by first match, so the
stale Rust definition could win label resolution for later same-server runs,
surfacing the same `api.ekodb.net` error in the Python/TypeScript example logs.
Aligning the URL removes that cross-run poisoning.

### Fixed — Doc/test base URLs aligned to `.ekodb.net` (ekodb-app#232)

Two remaining `.ekodb.io` examples that referenced deployed-database URLs
(rather than external/marketing surfaces) were updated to `.ekodb.net`, matching
the convention used everywhere else in the client docs and the ekodb-app
deployment UI.

- `ekodb-client-kt/src/test/kotlin/io/ekodb/client/KVBatchOperationsTest.kt:17`
  and `EkoDBClientTest.kt:23` — test fixture `testBaseUrl` is now
  `https://test.ekodb.net`.

## [0.18.2] - 2026-05-26

### Security — Dependabot sweep (ekodb-client#118)

Lockfile bumps across all three workspaces (`Cargo.lock`,
`ekodb-client-py/Cargo.lock`, `examples/rust/Cargo.lock`) to clear 14 open
Dependabot alerts (7 HIGH severity). No source changes; transport / TLS stacks
pull patched versions transitively.

- **openssl 0.10.73 / 0.10.78 → 0.10.80** (with `openssl-sys` 0.9.114 / 0.9.109
  → 0.9.116). Resolves GHSA-xv59-967r-8726 (heap buffer overflow in AES
  key-wrap-with-padding, MEDIUM), GHSA-xp3w-r5p5-63rr (undefined behavior in
  `X509Ref::ocsp_responders` for non-UTF-8 OCSP URLs, HIGH), GHSA-pqf5-4pqq-29f5
  (`Deriver::derive` / `PkeyCtxRef::derive` buffer overflow on OpenSSL 1.1.1,
  HIGH), GHSA-xmgf-hq76-4vx2 (out-of-bounds read in PEM password callback, LOW),
  GHSA-8c75-8mhr-p7r9 (incorrect bounds assertion in AES key wrap, HIGH),
  GHSA-hppc-g8h3-xhp3 (PSK/cookie trampoline length leaks adjacent memory to
  peer, HIGH), GHSA-ghm9-cr32-g9qj (`MdCtxRef::digest_final` writes past caller
  buffer, HIGH).
- **rustls-webpki 0.103.10 → 0.103.13** in `examples/rust/Cargo.lock`. Resolves
  GHSA-82j2-j2ch-gfr8 (denial of service via panic on malformed CRL
  `BIT STRING`, HIGH).
- **rand 0.8.5 → 0.8.6** and **rand 0.9.2 → 0.9.4** in
  `examples/rust/Cargo.lock`. Resolves GHSA-cq8v-f236-94qc (unsound behavior
  with a custom logger using `rand::rng()`, LOW) on both major lines.

All bumps are transitive — pulled via `cargo update -p <pkg>` with no manifest
edits. `cargo check`, `cargo clippy -p ekodb_client -- -D warnings`, and
`make test-rust` (222 + 3 + 8 + 145 unit/integration + 28 doctests, all passing)
verified clean.

### Tests — un-ignored stale `#[ignore]` on `test_client_builder`

`tests/integration_test.rs::test_client_builder` was tagged
`#[ignore] // Ignore by default since it requires a running server` but the test
only exercises `Client::builder().base_url(...).api_key(...).build()` —
pure-local builder validation, no network I/O, no actual connection. The header
comment that justified the ignore covered a different test
(`test_insert_and_find`) that genuinely does need a running ekoDB server. The
stale ignore is removed so a regression in builder validation fails CI
immediately. The `test_insert_and_find` ignore stays — converting it requires
mocking the entire ekoDB wire protocol (auth, ID encryption, schema), which is
real scope.

## [0.18.1] - 2026-04-29

### Added

- **No-op semantics**: cancelling a `chat_id` with no in-flight stream is safe
  and does not error.
- **`WebSocketClient::cancel_chat()` auto-reconnects via `ensure_connected()`**
  so callers don't have to keep the original streaming WS handle alive just to
  send a cancel — matches the connection contract every other public WS-RPC
  method on `WebSocketClient` already follows.
- **Wire-format tests for `WebSocketRequest::CancelChat`** —
  `cancel_chat_request_serializes_with_expected_shape` pins the
  `type: "CancelChat"` tag and `payload.chat_id` field name (both load-bearing
  on the server), and `cancel_chat_payload_deserializes_from_wire` exercises the
  inverse path. Catches accidental rename / shape drift across version
  boundaries.

### Dependencies

- Lockfile + version-string sweep across `Cargo.lock`,
  `ekodb-client-py/Cargo.lock`, `ekodb-client-ts/package-lock.json`,
  `ekodb-client-kt/build.gradle.kts`, and the `examples/` lockfiles to pin to
  0.18.1.

## [0.18.0] - 2026-04-29

### Added — Server-side chat cancel

- **`WebSocketClient::cancel_chat(chat_id)`** new method that sends
  `WebSocketRequest::CancelChat { CancelChatPayload }` over the WS. The server
  fires the matching `CancellationToken` and the in-flight LLM call inside
  `chat_message_streaming` aborts via `tokio::select!` — the assistant message
  is NOT persisted. Pre-fix, dropping the receiver only halted client-side chunk
  delivery; the LLM kept generating server-side and the "cancelled" turn still
  landed in `/history`.
- New `CancelChatPayload { chat_id: String }` and `WebSocketRequest::CancelChat`
  variant. Server matches a process-wide token registry keyed by `chat_id`, so
  the cancel works regardless of which WS connection sends it (claw's typical
  pattern uses a fresh `connect_ws()` for cancel).

## [0.17.0] - 2026-04-12

### Added (2026-04-26 — crypto + concurrency stage variants)

- **11 new crypto stage variants** across Rust, TypeScript, Python, and Kotlin
  clients: `HmacSign`, `HmacVerify`, `AesEncrypt`, `AesDecrypt`, `UuidGenerate`,
  `TotpGenerate`, `TotpVerify`, `Base64Encode`, `Base64Decode`, `HexEncode`,
  `HexDecode`, `Slugify`. Each gained a builder helper:
  - **Rust**: `Function::HmacSign { ... }` etc. enum variants.
  - **TypeScript**: `Stage.hmacSign(...)`, `Stage.aesEncrypt(...)`,
    `Stage.uuidGenerate(...)`, `Stage.totpGenerate(...)`,
    `Stage.base64Encode(...)`, `Stage.hexEncode(...)`, `Stage.slugify(...)`,
    plus the matching verify/decode/decrypt builders.
  - **Python**: `Stage.hmac_sign(...)`, `Stage.aes_encrypt(...)`,
    `Stage.uuid_generate(...)`, `Stage.totp_generate(...)`,
    `Stage.base64_encode(...)`, `Stage.hex_encode(...)`, `Stage.slugify(...)`,
    etc.
  - **Kotlin**: `FunctionStageConfig.HmacSign` etc. sealed subclasses with
    `@SerialName`-tagged kotlinx.serialization encoding.
- **4 new concurrency stage variants** for idempotency, rate limiting, and
  distributed locks: `IdempotencyClaim`, `RateLimit`, `LockAcquire`,
  `LockRelease`. Builders mirror the crypto naming pattern across all four
  clients.
- **Tests** — 4 new test functions per client (~30 assertions total per client)
  cover happy-path serialization, optional-field omission, and JSON round-trip
  equivalence.

**Requires ekoDB >= 0.42.0** for any of these stages to execute on the server.
Older servers will reject the function definition with "unknown variant" at
insertion time.

### Added (2026-04-26 — JWT + EmailSend + path-routed function fields)

- **`Function::JwtSign` / `Function::JwtVerify` variants** — bindings for
  ekoDB's HMAC JWT stages (HS256 / HS384 / HS512). `JwtSign` takes a claims
  object, secret, optional `expires_in_secs` (auto-stamps `iat` + `exp`), and
  writes the token to `output_field`. `JwtVerify` writes either decoded claims
  or `null` to `output_field` so callers can branch with `If { FieldEquals }`.
- **`Function::EmailSend` variant** — binding for the SendGrid v3 `mail/send`
  integration stage. Carries `to`, `subject`, `body`, `from`, optional
  `reply_to`, `api_key` (use `{{env.SENDGRID_API_KEY}}`), optional `provider`
  (defaults to `"sendgrid"`), `html` flag, and `output_field`.
- **`http_method` + `http_path` on `UserFunction`** plus a
  `with_http_route(method, path)` builder. Lets a stored function answer to a
  REST-style URL (`GET /api/route/users/:id`) via the ekoDB server's path-routed
  dispatcher. Both fields are optional and serialize with
  `skip_serializing_if = "Option::is_none"`, preserving wire compatibility for
  existing definitions.
- **Tests** — 4 JWT round-trip / reject cases and 3 EmailSend shape cases.

### Added (2026-04-25)

- **`Attachment` struct + `attachments: Option<Vec<Attachment>>` on
  `ChatMessageRequest`** — pairs with ekoDB's Anthropic / OpenAI / Gemini File
  API support. Pure addition; existing callers unaffected (default `None`).
- **`ChatStreamEvent::ToolCall` SSE variant** — recognized when the server emits
  `event: tool_call` mid-stream (set by ekoDB's `TOOL_PROGRESS_TX` task-local).
  Lets streaming consumers display in-flight tool calls before the assistant
  token stream resumes.

### Added

- **Five new function stage variants across Rust, TypeScript, Python, and Kotlin
  clients** — `TryCatch`, `Parallel`, `Sleep`, `Return`, and `Validate` stages
  that the server already supported were missing from all client-side `Function`
  enums / stage builders. This caused `list_functions` to fail with
  `unknown variant 'Return'` when any stored function used these stages, because
  the client couldn't deserialize the server response.
  - **Rust**: `Function::TryCatch`, `Function::Parallel`, `Function::Sleep`,
    `Function::Return`, `Function::Validate` variants with full serde
    round-trip. 8 new unit tests.
  - **TypeScript**: `FunctionStageConfig` union members + `Stage.tryCatch()`,
    `Stage.parallel()`, `Stage.sleep()`, `Stage.returnResponse()`,
    `Stage.validate()` builders. 16 new tests.
  - **Python**: `Stage.try_catch()`, `Stage.parallel()`, `Stage.sleep()`,
    `Stage.return_response()`, `Stage.validate()` helpers. 12 new tests.
  - **Kotlin**: `FunctionStageConfig.TryCatch`, `.Parallel`, `.Sleep`,
    `.Return`, `.Validate` sealed subclasses with kotlinx.serialization. 10 new
    tests.

- **Improved `ParameterRef` type safety (TypeScript)** — `parameterRef()` and
  `Stage.param()` now return a `ParameterRef` interface
  (`{ type: "Parameter"; name: string }`) instead of `Record<string, string>`,
  catching misuse at compile time. Stage methods that accept records/updates now
  accept `Record<string, any> | ParameterRef`.

### Fixed

- **Wire-format test for `ttl: undefined` (TypeScript)** — The JSON
  serialization round-trip test asserted `ttl: undefined` in the wire object,
  but `JSON.parse(JSON.stringify(...))` drops `undefined` keys entirely.
  Replaced with explicit `stage.ttl` undefined check + `"ttl" in wire` absence
  check.

- **Three new crypto-primitive function stages across every language client** —
  Rust, TypeScript, Python, and Kotlin now expose typed bindings
  - builder helpers for ekoDB 0.41.0's new `BcryptHash`, `BcryptVerify`, and
    `RandomToken` stored-function stages. Lets callers build a pure
    stored-function auth flow
    (`users_register = Validate + BcryptHash + Insert`,
    `users_login = FindOne + BcryptVerify + If`) and have ekoDB do the password
    hashing, verification, and session-token minting instead of hosting those
    primitives in every app layer. All four helpers produce identical JSON on
    the wire. Helper API by language:
  * **Rust** (`ekodb_client`): new
    `Function::BcryptHash { plain, cost, output_field }`,
    `Function::BcryptVerify { plain, hash_field, output_field }`, and
    `Function::RandomToken { bytes, encoding, output_field }` variants. 6 new
    tests in `ekodb_client/tests/unit_tests.rs`
    (`test_bcrypt_hash_stage_serializes_with_text_placeholder`,
    `test_bcrypt_hash_stage_omits_cost_when_none`,
    `test_bcrypt_verify_stage_serializes`,
    `test_random_token_stage_serializes_with_hex_default`,
    `test_random_token_stage_omits_encoding_when_none`,
    `test_crypto_stages_roundtrip_through_serde`).
  * **TypeScript** (`@ekodb/ekodb-client`): new
    `Stage.bcryptHash(plain, output_field, cost?)`,
    `Stage.bcryptVerify(plain, hash_field, output_field)`, and
    `Stage.randomToken(bytes, output_field, encoding?)` builders plus the
    matching `FunctionStageConfig` union members. 9 new tests in
    `ekodb-client-ts/src/functions.test.ts` — full TS suite: 347/347 passing.
  * **Python** (`ekodb_client`): new
    `Stage.bcrypt_hash(plain, output_field, cost=None)`,
    `Stage.bcrypt_verify(plain, hash_field, output_field)`, and
    `Stage.random_token(bytes, output_field, encoding=None)` methods on
    `ekodb_client.stages.Stage`. 7 new tests in
    `ekodb-client-py/tests/test_stages.py`.
  * **Kotlin** (`io.ekodb.client.functions`): new
    `FunctionStageConfig.BcryptHash`, `FunctionStageConfig.BcryptVerify`, and
    `FunctionStageConfig.RandomToken` `@SerialName`'d data classes. 6 new tests
    in
    `ekodb-client-kt/src/test/kotlin/io/ekodb/client/functions/FunctionStagesTest.kt`
    — full suite: 13/13 passing.

  **Requires ekoDB >= 0.41.0.** Sensitive runtime inputs (the password the user
  just typed) flow through the existing text-level `"{{password}}"` placeholder
  substituted by `substitute_parameters` at stage-execution time. Operator-owned
  secrets (peppers, data keys) continue to use `"{{env.NAME}}"` sourced from
  ekoDB's `environment_vars` config whitelist.

- **Structural parameter placeholder helper added to every language client** —
  Rust, TypeScript, Python, and Kotlin now expose a helper that builds the
  `{"type": "Parameter", "name": <name>}` shape ekoDB's
  `resolve_json_parameters` recognizes inside `Insert.record`, `Update.updates`,
  `UpdateById.updates`, `FindOneAndUpdate.updates`, `BatchInsert.records`, and
  any `QueryExpression` filter value. This is the structural alternative to the
  text-level `"{{name}}"` placeholder form — use it when the parameter is a
  whole-object record or a value whose type would otherwise be lost on a
  raw-JSON round-trip (Binary, DateTime, UUID, Decimal, Duration, Number, Set,
  Vector). Requires ekoDB >= 0.41.0 for the mutation-stage parameter-resolution
  consistency fix. Helper API by language:
  - **Rust** (`ekodb_client`):
    `parameter_ref(name: impl Into<String>) -> serde_json::Value`, re-exported
    from the crate root. Covered by 7 new tests in
    `ekodb_client/tests/unit_tests.rs` (`test_parameter_ref_shape`,
    `test_parameter_ref_arbitrary_name`,
    `test_insert_function_accepts_structural_parameter`,
    `test_insert_function_accepts_per_field_placeholders`,
    `test_update_by_id_accepts_structural_parameter`,
    `test_update_with_structural_filter_and_updates`,
    `test_batch_insert_accepts_per_record_structural_parameters`), plus a
    doctest on the function itself.
  - **TypeScript** (`@ekodb/ekodb-client`): top-level `parameterRef(name)` and
    `Stage.param(name)` shorthand. Covered by 11 new tests in
    `ekodb-client-ts/src/functions.test.ts` spanning Insert, UpdateById, Update
    (filter-based), BatchInsert, and JSON wire-format round-trip.
  - **Python** (`ekodb_client`): top-level `parameter_ref(name)` and
    `Stage.param(name)` shorthand in `ekodb_client.stages`. Covered by 9 tests
    in `ekodb-client-py/tests/test_stages.py` mirroring the TS coverage.
  - **Kotlin** (`io.ekodb.client.functions`): top-level
    `parameterRef(name): JsonObject` in `FunctionStages.kt`. Covered by 7 tests
    in
    `ekodb-client-kt/src/test/kotlin/io/ekodb/client/functions/FunctionStagesTest.kt`.

  All four helpers produce identical JSON on the wire and are interoperable with
  any of the ekoDB client libraries — an app can define its stored functions in
  one language and call them from another.

## [0.16.0] - 2026-04-01

### Added

- **`agent_id` field on chat sessions** — `CreateChatSessionRequest` and
  `ChatSession` now include an optional `agent_id` field to associate a session
  with a named agent. Rust client includes a `.agent_id()` builder method. Added
  across all clients (Rust, TypeScript, Go, Kotlin, Python).

- **Client tool fields on `ChatMessageRequest`** — Added `client_tools`,
  `confirm_tools`, and `exclude_tools` optional fields plus `ClientToolDef`
  struct for HTTP/SSE chat message requests. When provided, ekoDB merges client
  tool definitions with built-in tools and routes calls back via
  `__client_tool_call` SSE events. Includes builder methods in Rust. Added
  across all clients (Rust, TypeScript, Go, Kotlin, Python).

- **`submit_chat_tool_result()`** — New HTTP method to submit a client tool
  result for an in-flight SSE chat stream (`POST /api/chat/:id/tool-result`),
  unblocking ekoDB's tool loop. Added across all clients.

- **`subscribe_sse()`** — Subscribe to collection mutations via SSE (Server-Sent
  Events). Returns a channel/stream of `MutationNotification` events. Use when
  WebSocket connections aren't available (e.g. behind reverse proxies). Supports
  `filter_field`/`filter_value` query params. Added across all clients (Rust,
  TypeScript, Go, Kotlin, Python).

- **`list_user_collections()`** — New method that lists collections while
  filtering out internal chat/system collections via `exclude_internal=true`
  query parameter. Rust client only.

### Changed

- **Dependency management** — `make deps-update` and `make deps-update-rust` now
  use `cargo upgrade` (from `cargo-edit`) to bump `Cargo.toml` constraints
  before running `cargo update`, instead of only updating within existing
  constraints.

- **Updated dependencies** — Bumped dependencies across Rust, Python bindings,
  and TypeScript client packages.

## [0.15.2] - 2026-03-28

### Fixed

- **HTTP status check sweep** — Added `json_body<T>` helper that checks HTTP
  status before deserializing. All chat, KV, search, collection, and transaction
  methods now return proper typed errors (`NotFound`, `TokenExpired`,
  `RateLimit`, `Api`) instead of attempting to deserialize error response bodies
  as success types. This fixes the "missing field `session`" crash when
  restoring a deleted chat, and `kv_exists` returning errors instead of `false`
  after `kv_delete`.

- **`ChatSessionResponse` backward compatibility** — Added `#[serde(default)]`
  to `session` and `message_count` fields as defense-in-depth for edge cases
  where the server response shape is unexpected.

- **Search score injection** — `hybrid_search()` and `text_search()` helper
  methods now inject `_score` into returned records. Previously the score was
  stripped when mapping `SearchResult` to `Record`, causing all scores to read
  as 0.000 in RAG examples. Fixed across all clients (Rust, TypeScript, Go,
  Kotlin; Python inherits from Rust).

- **Python SWR example** — Removed `create_collection("user_cache_py", None)`
  call that failed with "Schema must contain at least one field". ekoDB
  auto-creates collections on first insert.

## [0.15.1] - 2026-03-27

### Added

- **Full WebSocket CRUD parity** — 14 new methods on `WebSocketClient`:
  `insert`, `query`, `find_by_id`, `update`, `delete`, `batch_insert`,
  `batch_update`, `batch_delete`, `text_search`, `distinct_values`,
  `update_with_action`, `create_collection`, `list_collections`,
  `delete_collection`. All use `messageId` for concurrent request correlation.

- **Schema cache** — Opt-in in-memory LRU cache for collection schema metadata
  (`primary_key_alias`, version). Configurable TTL (default 5min), max entries
  (default 100). Auto-invalidated via WS `SchemaChanged` events. Enable with
  `Client::builder().schema_cache(true)`.

- **SSE subscriptions** —
  `client.subscribe_sse(collection, filter_field?, filter_value?)` for mutation
  events over Server-Sent Events. Works behind reverse proxies that block
  WebSocket. Auto-invalidates schema cache on `schema_changed` SSE events.

- **`extract_record_id()`** — Universal ID extractor in `utils.rs`. Handles
  custom `primary_key_alias` by trying extra candidates first, then `"id"`, then
  `"_id"`. Also handles typed wrapper format
  `{"type": "String", "value": "..."}`.

- **`client.extract_id(collection, record)`** — Cache-aware instance method that
  uses the schema cache's `primary_key_alias` for the collection.

- **`client.connect_ws()`** — Convenience method to create a `WebSocketClient`
  connected to the same instance. Derives WS URL from base URL (http→ws,
  https→wss), passes current auth token, and attaches schema cache for
  auto-invalidation.

- **`SchemaChanged` WS event handling** — Dispatcher automatically invalidates
  the schema cache when the server pushes a `SchemaChanged` event.

- **Concurrent WS dispatch** — `Success` and `Error` responses now carry
  top-level `messageId` for proper concurrent request correlation.
  Backward-compatible single-pending fallback preserved.

### Changed

- **`WebSocketResponse::Error` now includes `message_id`** — Enables per-request
  error routing instead of broadcast to any pending request.

## [0.15.0] - 2026-03-25

### Security

- **Fix rustls-webpki CRL vulnerability (3 Dependabot alerts)** — Updated
  `rustls-webpki` from 0.103.7/0.103.9 to 0.103.10 in the root `Cargo.lock`,
  `ekodb-client-py/Cargo.lock`, and `examples/rust/Cargo.lock`. Fixes faulty CRL
  Distribution Point matching logic that could cause CRLs to not be considered
  authoritative.

### Added

- **Per-message LLM model override** — `ChatMessageRequest` now supports
  `.llm_model("model-name")` to override the session's default model for a
  single message. Enables routing simple tool-calling steps through a faster
  model while keeping the primary model for interactive chat.

- **JWT expiry-based token caching in Kotlin client** — `getToken()` now decodes
  the JWT `exp` claim and proactively refreshes 60 seconds before expiry,
  matching the Rust client's `AuthManager`. Added `extractJWTExpiry()`
  (internal) for Base64-decoding the JWT payload. `clearTokenCache()` now also
  resets the expiry. Includes 8 new unit tests covering expiry extraction, cache
  reuse, proactive refresh, and cache clearing.

- **Proactive token refresh in `AuthManager`** — `get_token()` now decodes the
  JWT `exp` claim and refreshes 60 seconds before expiry. All callers get a
  valid token automatically without needing `execute_with_token_refresh`
  wrappers. Eliminates cascading "Token expired" failures across all client
  methods.

- **Server-side `execute_tool()`** —
  `Client::execute_tool(tool_name, params, chat_id)` now delegates to ekoDB's
  `POST /api/chat/tools/execute` endpoint instead of dispatching tools
  client-side. All collection filtering, permission enforcement, and internal
  collection blocking happen server-side. Accepts optional `chat_id` for
  memory-tool context. Returns `None` on 404/405 so callers can fall back to
  chat/LLM routing.

- **`execute_tool_remote()` HTTP method** — New
  `HttpClient::execute_tool_remote()` POSTs to `/api/chat/tools/execute` with
  `{tool, params, chat_id?}` and returns the raw server response.

- **`executeTool` in TypeScript client** —
  `EkoDBClient.executeTool(toolName, params, chatId?)` calls the server-side
  tool execute endpoint. Returns `null` on 404/405 for graceful fallback.

- **`executeTool` in Kotlin client** —
  `EkoDBClient.executeTool(toolName, params, chatId?)` suspend function with
  `executeWithRetry`. Returns `null` on 404/405.

- **`ExecuteTool` in Go client** —
  `Client.ExecuteTool(toolName, params, chatID)` with
  `ExecuteToolRequest`/`ExecuteToolResult` types. Returns `nil, nil` on 404/405.

- **`execute_tool` in Python client** — PyO3 binding delegates to
  `client.execute_tool()`. Returns `None` on 404/405.

### Changed

- **Kotlin dependency updates** — Ktor 3.3.3 → 3.4.1, kotlinx-serialization
  1.9.0 → 1.10.0. Python bindings: borsh 1.6.0 → 1.6.1, zerocopy 0.8.42 →
  0.8.47, winnow 0.7.15 → 1.0.0 (transitive). Kotlin stays at 2.3.0 (2.3.20
  breaks CodeQL java-kotlin analysis).

- **New `Error::ToolExecution` variant** — Tool execution failures now use a
  dedicated error variant instead of fabricating `Error::Api { code: 400 }`,
  which was misleading for downstream error handling.

### Fixed

- **`execute_tool()` 404/405 fallback was broken (all languages)** — The Rust
  implementation matched `Error::Api { code: 404 }` but `handle_response()`
  returns `Error::NotFound` for 404s, so the `None` fallback never triggered.
  Now correctly matches both `Error::NotFound` and `Error::Api { code: 405 }`.
  TypeScript and Kotlin used fragile `err.message.includes("404")` string
  matching which could false-positive on unrelated error bodies; now parses the
  status code from the known error message prefix. Go used
  `strings.Contains(err.Error(), "404")` instead of type-asserting `*HTTPError`;
  now uses `errors.As` with `StatusCode` check.

- **`execute_tool()` bypassed token refresh** — Calls went directly through
  `get_token()` without `execute_with_token_refresh`, so 401 responses weren't
  retried with a fresh token. Now wrapped in `execute_with_token_refresh` like
  all other client methods.

- **`execute_tool_remote()` missing `Accept` header** — Added
  `Accept: application/json` for consistency with other chat HTTP methods.

- **Token refresh on core CRUD methods** — `find_by_id`, `update`, `delete`,
  `batch_insert`, `batch_update`, `batch_delete`, `list_collections`,
  `delete_collection`, `get_schema`, `create_collection`, `update_with_action`
  now use `execute_with_token_refresh` (was only on `insert`, `find`,
  `distinct_values`).

- **`count_documents` no longer fetches all records** — Uses
  `select_fields: ["_id"]` to minimize data transfer. Still no dedicated server
  count endpoint, but network payload is now ~100x smaller for large
  collections.

- **`find()` respects `should_retry` flag** — Was calling
  `retry_policy.execute()` directly, bypassing the `no_retry()` configuration.
  Now uses `execute_with_retry()` like all other methods.

### Removed

- **`Error::Auth` variant** — Duplicate of `Error::Authentication`. All usages
  migrated to `Error::Authentication`.

## [0.14.0] - 2026-03-21

### Added

- **TypeScript unit tests: schedules, KV links, text/hybrid search** — Added
  three new `describe` blocks to `client.test.ts`: `EkoDBClient schedules` (7
  tests: create, list, get, update, delete, pause, resume),
  `EkoDBClient kv links` (3 tests: kvGetLinks, kvLink, kvUnlink), and
  `EkoDBClient text and hybrid search` (2 tests: textSearch, hybridSearch).

- **TypeScript integration example: Goals, Tasks & Agents** — New
  `client_goals_tasks_agents.ts` exercising the full lifecycle of goals, tasks,
  and agents matching the Rust example.

- **TypeScript integration example: Schedules** — New `client_schedules.ts`
  covering create, list, get, update, pause, resume, and delete schedule.

- **TypeScript integration example: KV Links** — New `client_kv_links.ts`
  demonstrating kvSet, kvLink, kvGetLinks, kvUnlink with document cleanup.

- **TypeScript integration example: Advanced CRUD** — New
  `client_advanced_crud.ts` covering updateWithAction (increment, push, clear),
  updateWithActionSequence, restoreRecord, and restoreCollection.

- **Integration example: Goals, Tasks & Agents** — New
  `client_goals_tasks_agents.rs` example exercising the full lifecycle of goals
  (create, list, get, update, search, complete, approve, reject, step
  start/complete/fail, delete), tasks (create, list, get, start, succeed, pause,
  resume, fail, due, delete), and agents (create, list, get, get_by_name,
  update, agents_by_deployment, delete).

- **Integration example: Schedules** — New `client_schedules.rs` example
  covering create_schedule, list_schedules, get_schedule, update_schedule,
  pause_schedule, resume_schedule, and delete_schedule.

- **Integration example: KV Links** — New `client_kv_links.rs` example
  demonstrating kv_set, kv_link, kv_get_links, kv_unlink with document
  insert/cleanup.

- **Integration example: Advanced CRUD** — New `client_advanced_crud.rs` example
  covering update_with_action (increment, decrement, multiply, push, append,
  pop, remove), update_with_action_sequence, restore_deleted, and
  restore_collection.

- **Kotlin `countDocuments` method** — Added `countDocuments(collection)` to the
  Kotlin client, returning the number of documents in a collection. Calls
  `GET /api/{collection}/count`. All 5 clients now have this method.

- **`rawCompletionStreamWithProgress` parity** — Added to Kotlin client (with
  `onToken` callback lambda) and Python client (with `on_token` callable). Both
  match the existing Go and TypeScript implementations. Enables real-time
  per-token progress display during stateless LLM completions.

- **Goal template CRUD methods** — New client methods for managing goal
  templates: `goal_template_create`, `goal_template_list`, `goal_template_get`,
  `goal_template_update`, `goal_template_delete`. Calls
  `/api/chat/goal-templates` endpoints on the ekoDB server. Wired through Python
  PyO3 bindings as async methods on the `EkoDbClient` class.

- **SSE chat message streaming** — New `chatMessageStream()` method in
  TypeScript and Kotlin clients (Rust: `chat_message_stream()`). Streams chat
  responses via Server-Sent Events over HTTP. TypeScript returns an
  `EventStream<ChatStreamEvent>`, Kotlin returns `Flow<ChatStreamEvent>`. Calls
  `POST /api/chat/{id}/messages/stream`. Works behind reverse proxies that don't
  support WebSocket upgrades. Python PyO3 binding added as
  `chat_message_stream()` on the `Client` class, returning a
  `ChatStreamReceiver` (same receiver type as WebSocket `chat_send`).

- **`context_window` field on `ChatStreamEvent::End`** — The end event now
  includes the model's context window size in tokens. Supported in all clients:
  Rust (`Option<u32>`), Python (dict key), TypeScript
  (`contextWindow?: number`), Kotlin (`contextWindow: Int?`), Go
  (`ContextWindow uint32`). Allows clients to display context usage and warn
  when approaching limits.

- **WebSocket unit tests for `context_window`** — Added deserialization and
  round-trip tests in Rust (6 tests), Go (2 tests), TypeScript (1 test), and
  Kotlin (2 tests).

### Removed

- **Query index management methods (all languages)** — Removed
  `create_query_index`, `list_query_indexes`, `delete_query_index`,
  `explain_query` from Rust, Python, TypeScript, and Kotlin clients. These
  endpoints require admin auth (`admin_filter`) and do not belong in the client
  library.

- **Search index management methods (all languages)** — Removed
  `create_search_index`, `explain_text_search`, `explain_vector_search`,
  `explain_hybrid_search` from Rust, Python, TypeScript, and Kotlin clients.
  These endpoints require admin auth (`admin_filter`) and do not belong in the
  client library.

### Added

- **Streaming raw completion with progress** — New
  `raw_completion_stream_with_progress()` (Rust),
  `rawCompletionStreamWithProgress()` (TypeScript) sends each LLM token through
  a channel/callback as it arrives via SSE, enabling real-time progress display
  during long-running calls (e.g., goal plan generation).

- **Search index management methods (Rust, Python, TypeScript, Kotlin)** —
  `create_search_index`, `explain_text_search`, `explain_vector_search`,
  `explain_hybrid_search` for creating search indexes and explaining search
  query execution plans. Added Rust HTTP/client layer and Python PyO3 bindings.

- **KV document linking methods (Rust, Python, TypeScript, Kotlin)** —
  `kv_get_links`, `kv_link`, `kv_unlink` for linking and unlinking documents to
  KV keys. Added Rust HTTP/client layer and Python PyO3 bindings.

- **Schedule management methods (Rust, Python, TypeScript, Kotlin)** —
  `create_schedule`, `list_schedules`, `get_schedule`, `update_schedule`,
  `delete_schedule`, `pause_schedule`, `resume_schedule` for full CRUD and
  lifecycle management of scheduled tasks. Added Rust HTTP/client layer and
  Python PyO3 bindings.

- **Goal/Task/Agent REST client methods (all languages)** — Full coverage of
  `/api/chat/goals`, `/api/chat/tasks`, and `/api/chat/agents` endpoints
  including CRUD, lifecycle transitions, goal step lifecycle, search, and
  deployment queries. Added to Rust, Python, TypeScript, Kotlin, and Go clients.

- **WebSocket raw completion (Kotlin & Python)** — Added `rawCompletion()` to
  the Kotlin WebSocket client and `raw_completion()` to the Python WebSocket
  client (PyO3 binding). Both send a `RawComplete` message over the persistent
  WSS connection, matching the existing Rust implementation.

### Fixed

- **HTTP client timeout no longer kills SSE streams** — Changed from full
  request timeout to connect-only timeout across Rust (`connect_timeout`),
  Kotlin (`connectTimeoutMillis` only), and Go (`net.Dialer.Timeout`).
  TypeScript uses `fetch()` which doesn't have this issue. Python inherits the
  Rust fix via PyO3 bindings.

## [0.13.0] - 2026-03-18

### Added

- **SSE streaming raw completion** — New `raw_completion_stream()` method across
  all client libraries (Rust, Python, TypeScript `rawCompletionStream()`, Go
  `RawCompletionStream()`, Kotlin `rawCompletionStream()`). Calls the new
  `POST /api/chat/complete/stream` SSE endpoint. Keeps the connection alive with
  heartbeat events, preventing reverse proxy timeouts on deployed instances.

- **WebSocket raw completion** — New `WebSocketClient::raw_completion()` in the
  Rust client. Sends a `RawComplete` message over an existing WSS connection,
  avoiding HTTP entirely. Preferred transport when a WebSocket connection is
  already established.

### Fixed

- **Go client auth in RawCompletionStream** — `RawCompletionStream()` was using
  the raw API key directly instead of exchanging it for a JWT token via
  `getToken()`/`refreshToken()`. This would have caused authentication failures
  on any real deployment.

- **TypeScript client token init in rawCompletionStream** —
  `rawCompletionStream()` now calls `refreshToken()` if no cached token exists,
  matching the behavior of all other methods that go through `makeRequest()`.

- **Atomic field actions** — New `update_with_action()` and
  `update_with_action_sequence()` methods across all client libraries (Rust,
  TypeScript, Python, Kotlin). These call the server's atomic field action
  endpoints (`PUT /api/update/{collection}/{id}/action/{action}` and
  `PUT /api/update/sequence/{collection}/{id}`) for safe concurrent
  modifications: increment/decrement counters, push/pop/shift/unshift arrays,
  multiply/divide/modulo arithmetic, append strings, remove array items, and
  clear fields. Sequence variant applies multiple actions atomically in a single
  request. 6 new Rust unit tests, 5 new TypeScript tests.

- **Full WebSocket dispatcher for TypeScript** — Rewrote `WebSocketClient` with
  a proper message dispatcher that routes incoming frames by type. New methods:
  `subscribe()` (returns `EventStream<MutationNotification>` for real-time
  collection change notifications), `chatSend()` (returns streaming
  `EventStream<ChatStreamEvent>` with chunk/end/toolCall/error events),
  `registerClientTools()` (registers client-side tool definitions for a chat
  session), and `sendToolResult()` (returns tool execution results to the
  server). New types: `MutationNotification`, `ChatStreamEvent`,
  `ClientToolDefinition`, `ChatSendOptions`, `SubscribeOptions`, `EventStream`.
  15 new unit tests with mock WebSocket server.

- **Full WebSocket dispatcher for Kotlin** — Expanded `WebSocketClient` with
  coroutine-based dispatcher using `Flow` for streaming. New methods:
  `subscribe()` (returns `Flow<MutationNotification>`), `chatSend()` (returns
  `Flow<ChatStreamEvent>`), `registerClientTools()`, and `sendToolResult()`. New
  sealed class `ChatStreamEvent` with `Chunk`, `End`, `ToolCall`, `Error`
  variants. New data classes: `MutationNotification`, `ClientToolDefinition`,
  `ChatSendOptions`, `SubscribeOptions`. 18 new unit tests.

- **WebSocket subscribe/chat/tools bindings for Python (PyO3)** — Added
  `subscribe()`, `chat_send()`, `register_client_tools()`, and
  `send_tool_result()` to the Python `WebSocketClient`. New wrapper classes
  `SubscriptionReceiver` and `ChatStreamReceiver` with async `recv()` methods
  that return dicts. 13 new API shape tests.

- **Token management for Python and TypeScript** — Python: added
  `refresh_token()` and `clear_token_cache()` async methods to `Client`.
  TypeScript: made `refreshToken()` public and added `clearTokenCache()`. 5 new
  tests.

- **`SchemaBuilder` and `JoinConfig` for Kotlin** — New `FieldTypeSchemaBuilder`
  and `SchemaBuilder` classes for fluent collection schema construction with
  constraints, indexes (text, vector, btree, hash), and validation. New
  `JoinConfig` data class with `single()` and `multi()` companion factories plus
  `toJsonObject()` and `toMap()` converters. 23 new tests.

- **`QueryBuilder`, `SchemaBuilder`, and `JoinConfig` for Python** — Pure Python
  fluent builder classes matching the Rust reference implementation. 52 new
  tests covering all operators, logical combinations, sorting, pagination,
  joins, field projection, and schema construction.

- **`RateLimitInfo` for Kotlin** — Data class with `isNearLimit()`,
  `isExceeded()`, and `remainingPercentage()` methods. Automatically parsed from
  `X-RateLimit-*` response headers in `executeWithRetry()`. Accessible via
  `getRateLimitInfo()` and `isNearRateLimit()`. 13 new tests.

- **`findByIdWithProjection()` for TypeScript and Kotlin** — Query a single
  record by ID with field selection/exclusion. TypeScript uses URL query
  parameters, Kotlin builds a filtered query with projection. 3 new TS tests.

- **Chat streaming integration examples** — New `client_websocket_chat_stream`
  examples for Python, TypeScript, Go, and Kotlin demonstrating streaming chat
  responses with tool calling via WebSocket.

- **`get_chat_tools()` method (all clients)** — Calls `GET /api/chat/tools` and
  returns the full list of built-in ekoDB server-side chat tool definitions.
  Returns `Vec<serde_json::Value>` (Rust), `object[]` (TypeScript), `list[dict]`
  (Python), `JsonArray` (Kotlin). Used by planning agents to dynamically
  discover all tools available in Chat steps.

- **`Query::select_fields()` and `Query::exclude_fields()` builder methods** —
  The `Query` struct already had `select_fields` and `exclude_fields` fields but
  no fluent builder API. These new methods allow callers to construct
  field-projection queries without manually instantiating the struct. Useful for
  reducing token usage when querying collections with large schemas: use
  `select_fields` to return only named fields, or `exclude_fields` to drop
  specific fields (e.g. large blobs) while returning everything else.

- **`RawCompletionRequest.max_tokens` and `raw_completion()` method** —
  Stateless raw LLM completion via `POST /api/chat/complete` added to all
  clients (Rust, TypeScript `rawCompletion`, Python `raw_completion`, Kotlin
  `rawCompletion`). `RawCompletionRequest` carries `system_prompt`, `message`,
  and optional `provider`, `model`, `max_tokens` fields. Useful for
  structured-output planning tasks that must be parsed programmatically without
  session/history overhead.

- **`distinct_values()` method** — New method for retrieving all unique values
  for a specific field across records in a collection. Supports optional filter
  expressions, `bypass_ripple`, and `bypass_cache` flags. Returns a
  `DistinctValuesResponse` with `collection`, `field`, `values` (sorted), and
  `count`. Available in Rust, TypeScript (`distinctValues`), Python
  (`distinct_values`), Go (`DistinctValues`), and Kotlin (`distinctValues`).

- **`DistinctValuesQuery` and `DistinctValuesResponse` types** — New types for
  the distinct values API, exported from all client libraries.

- **Integration examples: `client_distinct_values`** — Demonstrates distinct
  values queries with and without filters, added for Rust, Python, TypeScript,
  and Go.

## [0.12.0] - 2026-03-11

### Added

- **Rust example: `client_collection_utils`** — Demonstrates collection utility
  methods (`collection_exists`, `count_documents`, `list_collections`,
  `delete_collection`). Translated from the Python example.

- **Rust example: `client_kv_precision`** — Demonstrates float vs decimal
  precision in KV operations using `FieldType::decimal()` and `get_value`/
  `get_string_value` utilities. Translated from the Python example.

- **`POST /api/embed` direct endpoint support** — `embed()` now calls the
  server's `/api/embed` endpoint directly instead of creating temporary
  collections and scripts. Much faster and cleaner.

- **`embed_batch()` method** — New method for generating embeddings for multiple
  texts in a single request. Available in Rust, TypeScript (`embedBatch`),
  Python (`embed_batch`), Go (`EmbedBatch`), and Kotlin (`embedBatch`).

- **`EmbedRequest` and `EmbedResponse` types** — New types for the embed API,
  exported from all client libraries.

- **`ToolConfig` and `ToolChoice` types** — New types for configuring tool
  calling in chat sessions. `ToolConfig` controls enabled tools, allowed
  collections, max iterations, write permissions, and tool choice strategy.
  Exported from `ekodb_client` for use by all client languages.

- **`tool_config` on `CreateChatSessionRequest`** — Configure tool calling when
  creating a new chat session.

- **`max_tokens` and `temperature` on `CreateChatSessionRequest`** — Control LLM
  generation parameters per session, matching the server API.

- **`tool_config` on `ChatMessageRequest`** — Override session tool config on a
  per-message basis.

- **`max_context_messages` and `bypass_ripple` on `UpdateSessionRequest`** —
  Allow updating context window size and ripple sync settings on existing
  sessions.

- **`Interleaved` merge strategy** — Added `MergeStrategy::Interleaved` for
  round-robin message merging from source sessions.

- **`bypass_ripple` on `MergeSessionsRequest`** — Control ripple replication
  during session merge operations.

- **Python bindings: expanded parameters** — `create_chat_session` now accepts
  `max_context_messages`, `bypass_ripple`, `max_tokens`, `temperature`.
  `chat_message` now accepts `bypass_ripple`, `force_summarize`,
  `max_iterations`. `update_chat_session` now accepts `title`,
  `max_context_messages`, `bypass_ripple`. `merge_chat_sessions` now accepts
  `bypass_ripple` and `Interleaved` strategy.

- **TypeScript: `ToolConfig`, `ToolChoice` interfaces** — Added interfaces and
  `max_iterations`, `tool_config` to `ChatMessageRequest`; `bypass_ripple`,
  `title`, `memory` to `UpdateSessionRequest`; `Interleaved` to `MergeStrategy`;
  `bypass_ripple` to `MergeSessionsRequest`.

- **`memory` field on `UpdateSessionRequest`** — Allows updating the chat-scoped
  memory object when updating a session. Enables clients to directly set/modify
  the session memory without going through LLM tool calls.

- **Public token access** — New `get_token()` and `clear_token_cache()` methods
  on `Client` to expose JWT retrieval and cache invalidation for downstream
  consumers (e.g., ekodb-claw WebSocket auth).

- **WebSocket chat streaming** — New `chat_send()` method on `WebSocketClient`
  sends a chat message over WSS and returns a receiver of `ChatStreamEvent`
  items (`Chunk`, `End`, `Error`) for real-time token streaming from the LLM.

- **Filtered WebSocket subscriptions** — New `subscribe()` method with optional
  `filter_field` / `filter_value` parameters to narrow mutation notifications to
  records matching a specific field value (e.g., `chat_id = X`).

- **New WebSocket response types** — `WebSocketResponse` now includes
  `ChatStreamChunk`, `ChatStreamEnd`, `ChatStreamError`, and
  `MutationNotification` variants for typed handling of server push messages.

- **Typed wrapper value extraction** — New `as_string()` and `as_bool()` methods
  on `FieldType` that transparently unwrap both direct values (`String("x")`)
  and ekoDB's typed wrapper format (`Object({"type": "String", "value": "x"})`).
  New `get_string()` and `get_bool()` convenience methods on `Record` combine
  field lookup with extraction in one call.

### Fixed

- **Kotlin `embed()` used legacy temp collection/script approach** — Updated to
  call `/api/embed` directly, matching all other client languages.

- **Rust RAG example `N/A` field extraction** — `rag_conversation_system.rs`
  used manual `FieldType::String` pattern matching which failed with typed
  wrappers. Replaced with `record.get_string()` which handles all variants.

- **Python `kv_get` precision example returned `None`** — `kv_get()` returns
  `{"value": "<json_string>"}`, but the example called `get_value()` directly on
  the wrapper dict. Fixed to parse the JSON string first.

- **Python `kv_delete` printed `None`** — `kv_delete()` returns `None` by
  design. Fixed example to not print the return value.

- **Kotlin chat merge printed `null` for chat_id** — The merge response contains
  `message_count`, not `chat_id`. Fixed to print `message_count`.

- **Kotlin client `executeWithRetry` silently returned 4xx error responses** —
  HTTP 4xx responses (e.g. 404 Not Found) were returned as successful results
  instead of throwing exceptions. This caused `upsert()` to return error records
  like `{error: "Record not found"}` instead of falling through to insert, and
  `getChatModel()` to fail with JSON parsing errors on 405 responses. Added 4xx
  status check to throw with the error body.

- **Rust `client_convenience_methods` upsert overwrote Alice's record** — The
  example upserted Bob's data into Alice's ID, then `find_one` for Alice's email
  correctly returned nothing. Fixed to use a separate ID for Bob's upsert.

- **Kotlin `ClientConvenienceMethods` used `toString()` for ID extraction** —
  `inserted["id"]?.toString()` produces `StringValue(value=...)` instead of the
  raw ID string. Fixed to use `getRecordId(inserted)` utility function.

- **WebSocket close handshake** — `close()` now sends a proper WS close frame
  via `SinkExt::close()` before dropping the connection, eliminating "Connection
  reset without closing handshake" warnings on the server side. Method signature
  changed from `&mut self` to `&self` for compatibility with shared references.

- **WebSocket deadlock during client tool calls** — `WebSocketClient` used a
  single `Mutex<Option<(WsWrite, WsRead)>>` for the connection. The spawned
  reader task held the lock while waiting for frames (`read.next().await`),
  blocking `send_tool_result()` from acquiring the lock to write. This caused
  24+ second delays on every client tool round-trip. Split into separate
  `writer` and `reader` mutexes so writes never block on reads. The reader task
  now drops the lock after each `read.next()` call before processing the
  message.

- **WebSocket reader contention between subscribe/chat tasks** — Multiple
  spawned reader tasks (subscriptions and chat streams) competed for the shared
  reader mutex, causing messages to be consumed by the wrong task. Replaced with
  a single dispatcher loop that demultiplexes incoming frames by message_id,
  chat_id, or subscription type to the correct channel.

- **WebSocket `find_all()` didn't match `message_id`** — Returned on the first
  `Success` response without verifying it matched the request's `message_id`.
  Now uses `send_and_wait()` with message_id correlation.

- **WebSocket `ensure_connected()` only checked writer half** — If the reader
  was cleared after a receive error but writer was still set, the client
  wouldn't reconnect. Now checks both writer presence and a `connected` flag
  maintained by the dispatcher.

- **WebSocket dispatcher held lock during async channel sends** — The dispatcher
  held the `DispatchState` mutex while awaiting `tx.send().await` on chat stream
  channels. If a channel was full, this blocked the entire dispatcher. Now
  clones/extracts senders while holding the lock, drops it, then performs async
  sends outside the critical section.

- **WebSocket subscription notifications broadcast to all subscribers** —
  `MutationNotification` events were sent to every subscription receiver
  regardless of collection. Multiple subscriptions to different collections
  received unrelated mutations. Now stores `(collection, sender)` tuples and
  only routes notifications to matching subscribers.

- **WebSocket `register_client_tools` used `pending_requests` without
  `messageId`** — The WS protocol for `RegisterClientTools` has no `messageId`
  field, so the ack was routed via the "single pending request" fallback and
  could misroute if another request was in-flight. Now uses a dedicated
  `register_tools_ack` oneshot channel in the dispatcher.

- **`embed()` HTTP status not checked before deserialization** — Non-2xx
  responses were deserialized as `EmbedResponse`, producing confusing
  serialization errors. Now checks status first and returns `Error::Api` with
  the status code and response body.

## [0.11.0] - 2026-02-08

### Added

- **Chat Models API** — Query available AI models across providers:
  `get_chat_models()` retrieves all available chat models;
  `get_chat_model(provider)` retrieves models for a specific provider. Available
  in all client languages.

- **User Functions API** — Reusable function sequences with lifecycle
  management: `save_user_function()`, `get_user_function()`,
  `list_user_functions()`, `update_user_function()`, `delete_user_function()`.
  Available in all client languages.

- **Collection utilities** — `collection_exists()` and `count_documents()` added
  to all client libraries.

- **`bypass_cache` on request parameters** — New option to bypass the read cache
  on find and query requests, forcing a fresh read from storage.

## [0.10.0] - 2026-01-27

### Added

- **TypeScript direct value return** — TypeScript client now returns field
  values directly instead of in a typed wrapper, matching server semantics.

- **KV precision examples** — New `client_kv_precision` examples demonstrating
  float vs decimal precision in KV operations across all languages.

- **Array response helpers** — New helper utilities for extracting array values
  from ekoDB responses.

### Changed

- **Breaking**: `kv_get()` return value semantics updated — Returns the value
  directly (`{"value": <data>}`) instead of a wrapped format. The `output_field`
  parameter has been removed.

## [0.9.0] - 2026-01-27

### Added

- **Field Projection** — `select_fields` and `exclude_fields` on query builder
  and find methods across all client languages. Control which fields are
  returned in query results.

- **KV Batch Operations** — `kv_batch_get()`, `kv_batch_set()`,
  `kv_batch_delete()` for efficient multi-key access in single requests.

- **StageSWR (Stale-While-Revalidate)** — New function stage for external API
  caching with automatic KV cache check → HTTP request → KV cache set workflow.
  Supports parameter substitution and configurable TTL.

- **ScriptCondition types** — Recursive condition system for function control
  flow: `ConditionHasRecords`, `ConditionFieldExists`, `ConditionFieldEquals`,
  `ConditionCountEquals`, `ConditionCountGreaterThan`, `ConditionCountLessThan`,
  `ConditionAnd`, `ConditionOr`, `ConditionNot`.

- **Query serialization improvements** — Enhanced query builder serialization
  for more reliable cross-language compatibility.

### Changed

- **Breaking**: `ScriptCondition` JSON serialization now uses adjacently-tagged
  format to match the Rust server's serde format.

### Fixed

- Collection, KV, and HTTP request data parameter consistency across all client
  languages.
- SWR client pattern now correctly uses `FieldExists` instead of `HasRecords`.
- WebSocket: use `CallFunction` parameter filter; add wait to check message.

## [0.8.0] - 2026-01-06

### Added

#### Core Features

- **Convenience methods** across all client libraries (Rust, TypeScript,
  JavaScript, Python, Kotlin, Go):
  - `upsert()` - Insert or update with automatic fallback (atomic
    insert-or-update semantics)
  - `findOne()` - Find single record by field value
  - `exists()` - Efficient existence check without fetching full record
  - `paginate()` - Simplified pagination with page/pageSize parameters
  - `textSearch()` - Full-text search helper (where supported)

#### API Improvements

- **Options structs** for cleaner method signatures:
  - `InsertOptions` - ttl, bypass_ripple, transaction_id, bypass_cache
  - `UpdateOptions` - bypass_ripple, transaction_id, bypass_cache
  - `UpsertOptions` - bypass_ripple, transaction_id
  - `DeleteOptions` - bypass_ripple, transaction_id
  - `FindOptions` - bypass_cache, transaction_id
- Builder pattern for all options structs

#### Examples

- **New bypass_ripple examples** demonstrating ripple control in multi-node
  deployments (all languages)
- **New convenience methods examples** showcasing ergonomic API helpers (all
  languages)
- **Enhanced schema examples** with proper error handling for existing schemas

#### Testing & Quality

- Comprehensive unit tests for all convenience methods (227+ new tests)
- Test coverage for options builders and edge cases
- Integration test suite improvements across all languages

### Fixed

- **Schema example error handling** in TypeScript/JavaScript:
  - Previously crashed with exit code 1 when schema already exists
  - Now logs warning and continues execution (matching Go behavior)
  - Prevents test suite failures on repeated runs
- Error handling consistency across all client implementations

### Changed

- Updated examples to use new convenience methods where appropriate
- Improved error messages and documentation
- Enhanced type safety with stricter option types

## [0.7.1] - 2026-01-03

### Fixed

- Kotlin client: JsonElement type handling for JsonObject, JsonArray,
  JsonPrimitive, and JsonNull
- Kotlin client: Record and Query serialization improvements
- Error handling messaging improvements
- Gradle wrapper and CI configuration issues
- Rust toolchain action naming
- Naming conventions standardization
- Kotlin version mismatch resolution

### Changed

- Updated dependencies:
  - tokio-tungstenite: 0.21.0 → 0.28.0
  - log: 0.4.28 → 0.4.29
  - rmp-serde: 1.3.0 → 1.3.1
  - mockito: 1.7.0 → 1.7.1
  - @msgpack/msgpack (TypeScript)
  - serde_json: 1.0.145 → 1.0.148
  - kotlinx-coroutines-core and kotlinx-coroutines-test
  - golang.org/x/net
  - ktor-client-core
- Re-ran test examples with updated configurations

## [0.7.0] - 2026-01-03

### Added

- Dependabot configuration for automated dependency management

### Fixed

- Utility function doc-tests
- Client transaction examples
- Document TTL examples

## [0.6.1] - 2026-01-02

### Added

- Type-specific getValue helpers for all client libraries
- getStringValue, getIntValue, getBoolValue, etc. utility functions

### Changed

- Python client: PyO3 upgrade from 0.20 to 0.27
- Re-ran examples with updated type utilities

### Fixed

- Code scanning workflow permissions

## [0.6.0] - 2025-12-31

### Added

- Functions and Scripts support across all clients
- Function tags and versioning (now optional)
- Chat completions examples
- Show function example

### Changed

- Re-ran all examples with Functions support
- Updated examples to demonstrate new functionality

### Fixed

- Python publish scripts

## [0.5.0] - 2025-12-21

### Added

- Self-improving RAG (Retrieval-Augmented Generation) examples
- Enhanced tag examples and documentation

### Changed

- Updated example execution locations and paths
- Improved example list validation

## [0.4.0] - 2025-12-10

### Added

- **Functions and Scripts** - Complete implementation across all client
  libraries
- Transaction support with examples
- Dynamic examples output
- Restore deleted records functionality
- Multiple function and script examples:
  - Basic function operations
  - Function composition
  - Complete CRUD functions
  - Advanced function patterns

### Changed

- FieldType JSON serialization (now uses fromValue)

## [0.3.0] - 2025-11-05

### Added

- **MessagePack serialization** support for binary data transfer
- **Gzip compression** for reduced bandwidth usage
- **Kotlin client library** with full feature parity
  - Maven publishing support
  - Comprehensive examples
  - Coroutine-based async API

### Changed

- Updated all clients to support MessagePack + Gzip
- Enhanced test examples with compression benchmarks
- Updated dependencies across all packages

### Fixed

- Kotlin publishing scripts

## [0.2.1] - 2025-10-14

### Changed

- Updated README with correct package locations
- Documentation improvements

## [0.2.0] - 2025-10-14

### Fixed

- Updated links to Go repository
- Repository reference corrections

## [0.1.8] - 2025-10-14

### Fixed

- Version bumping script now correctly updates ekodb_client dependency in Python
  client
- Dependency synchronization between Rust and Python packages

## [0.1.7] - 2025-10-14

### Added

- Initial multi-language client library release
- Rust client (ekodb_client)
- TypeScript/JavaScript client (@ekodb/ekodb-client)
- Python client (ekodb-client-py with PyO3 bindings)
- Go client (ekodb-client-go)
- Core CRUD operations: insert, find, update, delete, batch operations
- Query builder with filtering, sorting, pagination
- WebSocket support for real-time updates
- Schema management
- Collection management
- Full-text search capabilities
- Vector search support
- Authentication and token management
- Automatic retry with exponential backoff
- Comprehensive examples for all operations
- Type-safe APIs with strong typing
- Error handling with detailed error types
