# Changelog

All notable changes to ekodb_client will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- **Fix rustls-webpki CRL vulnerability (3 Dependabot alerts)** — Updated
  `rustls-webpki` from 0.103.7/0.103.9 to 0.103.10 in `ekodb-client-py` and
  `examples/rust` lock files. Fixes faulty CRL Distribution Point matching logic
  that could cause CRLs to not be considered authoritative.

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

### Fixed

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
