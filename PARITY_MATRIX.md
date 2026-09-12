# Client Library Parity Matrix

**Last Updated:** September 12, 2026

**Published client version:** 0.26.4. The changes under `[Unreleased]` are on
the current development branches. Go is maintained in the separate
`ekodb-client-go` repository.

> Renamed from `MISSING_FEATURES.md` (April 28, 2026) to reflect the current
> intent: a parity tracker, not a missing-features checklist. Inbound links from
> `AUDIT_*.md` files are historical snapshots and intentionally left pointing at
> the old path. The companion `documentation/CLIENT_LIBRARY_GAPS.md` was removed
> in the same pass (it duplicated this file at a stale revision).

## Status: Core and stored-function contract parity

The core parity baseline below was verified at v0.21.0. Search API coverage was
updated through v0.26.4; the current stored-function and example-parity work is
documented below. Language-specific differences are noted separately.

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
- Schedule management (CRUD + pause/resume + immediate trigger)
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

### Kotlin API Differences and Follow-ups

- HTTP search uses the full `SearchQuery` model. WebSocket text search and
  stored-function search stages have separate request contracts.
- Kotlin's search model stores `fields` and `weights` as strings; its builder
  accepts lists/maps and normalizes them. Alternate JSON representations can be
  sent through raw search. Typed decoding requires `matched_fields`, while
  execution time is optional.
- Chat operations, transaction status, and WebSocket responses still expose raw
  JSON in parts of the Kotlin API. Typed search does not imply identical
  response modeling across every client operation.
- The existing query builder uses `JsonElement` filters and dynamic inputs.
  Vector-index algorithm and metric options remain strings in Kotlin.
- Some unrelated Quick Start examples still need correction: `client.query`,
  `deleteWhere`, and `offset` are not current Kotlin APIs, and CRUD methods use
  `Record` rather than the maps shown in those examples. Use the compiled
  examples in `examples/kotlin/examples` as the reference for those operations.
  Tracked in #216.

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

### Stored-function wire contract

- The five SDK implementations preserve `transaction_config`; omitting its
  isolation level uses the server default.
- Rust, Python, TypeScript, and Kotlin share a generated 67-stage fixture that
  covers every modeled stage and field. It includes typed query-expression
  filters, field-comparison conditions, the complete group-operation set, and
  the current mutation/search/HTTP stage shapes.
- The Go client models the same server contract in its separate repository and
  its public examples compile against that checkout through a configurable
  worktree path.
- `scripts/check-example-parity.py` enforces the same client-example scenario
  set across Rust, Python, Go, TypeScript, and Kotlin. JavaScript executes the
  compiled TypeScript set because both languages use the same npm SDK.

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

**Vector record values:** Rust and Kotlin `FieldType.vector`, TypeScript
`Field.vector`, and Python `field_vector` emit the server-accepted type/value
envelope, distinct from ordinary arrays. The Rust golden test exercises its
vector helper directly. The Kotlin live contract verifies insertion, named-field
cosine search, one Boolean filter, upsert visibility, and cleanup; it does not
establish broad prefilter or ANN semantics. Earlier schema-only validation did
not test record insertion. See
[contract commands and evidence](COMMANDS.md#search-and-schema-compatibility).

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

## Historical Notes

The sections below are preserved for reference. All features listed were
implemented across all clients as of March 2026 (v0.14.0). The "admin-only"
designations for Query Index and Search Index management remain correct — these
are server administration endpoints, not client library features.

---

## 1. Query Index Management ⛔ (Admin-only — not for client library)

### Description

Query indexes optimize database queries by pre-building indexes on specific
fields. The server supports creating, listing, and deleting query indexes, plus
query execution plan analysis.

### Server Endpoints

```
POST   /api/query/{collection}/explain          - Explain query execution plan
POST   /api/indexes/query/{collection}          - Create query index on field
GET    /api/indexes/query/{collection}          - List all query indexes
DELETE /api/indexes/query/{collection}/{field}  - Delete specific index
```

### Client Methods

All clients expose:

```typescript
// TypeScript/JavaScript
createQueryIndex(collection: string, field: string, options?: IndexOptions): Promise<void>
listQueryIndexes(collection: string): Promise<QueryIndex[]>
deleteQueryIndex(collection: string, field: string): Promise<void>
explainQuery(collection: string, query: Query): Promise<QueryPlan>
```

```go
// Go
CreateQueryIndex(collection, field string, options *IndexOptions) error
ListQueryIndexes(collection string) ([]QueryIndex, error)
DeleteQueryIndex(collection, field string) error
ExplainQuery(collection string, query interface{}) (*QueryPlan, error)
```

```rust
// Rust
pub async fn create_query_index(&self, collection: &str, field: &str, options: Option<IndexOptions>) -> Result<()>
pub async fn list_query_indexes(&self, collection: &str) -> Result<Vec<QueryIndex>>
pub async fn delete_query_index(&self, collection: &str, field: &str) -> Result<()>
pub async fn explain_query(&self, collection: &str, query: Query) -> Result<QueryPlan>
```

```python
# Python
def create_query_index(self, collection: str, field: str, options: Optional[IndexOptions] = None) -> None
def list_query_indexes(self, collection: str) -> List[QueryIndex]
def delete_query_index(self, collection: str, field: str) -> None
def explain_query(self, collection: str, query: dict) -> QueryPlan
```

```kotlin
// Kotlin
suspend fun createQueryIndex(collection: String, field: String, options: IndexOptions? = null)
suspend fun listQueryIndexes(collection: String): List<QueryIndex>
suspend fun deleteQueryIndex(collection: String, field: String)
suspend fun explainQuery(collection: String, query: Query): QueryPlan
```

### Use Cases

- **Performance Optimization:** Create indexes on frequently queried fields
- **Query Analysis:** Understand query execution plans before running expensive
  queries
- **Index Management:** List and remove unused indexes to free resources

### Implementation Priority

🔴 **HIGH** - Critical for production performance optimization

---

## 2. Search Index Management ⛔ (Admin-only — not for client library)

### Description

Search indexes enable full-text search and vector search capabilities. The
server supports creating text/vector indexes and explaining search execution
plans.

### Server Endpoints

```
POST /api/indexes/search/{collection}           - Create search index (text or vector)
POST /api/search/text/{collection}/explain      - Explain text search execution
POST /api/search/vector/{collection}/explain    - Explain vector search execution
POST /api/search/hybrid/{collection}/explain    - Explain hybrid search execution
```

### Client Methods

All languages expose:

```typescript
// TypeScript/JavaScript
createSearchIndex(collection: string, config: SearchIndexConfig): Promise<void>
explainTextSearch(collection: string, query: string, options?: SearchOptions): Promise<SearchPlan>
explainVectorSearch(collection: string, vector: number[], options?: SearchOptions): Promise<SearchPlan>
explainHybridSearch(collection: string, query: HybridQuery): Promise<SearchPlan>
```

```go
// Go
CreateSearchIndex(collection string, config SearchIndexConfig) error
ExplainTextSearch(collection, query string, options *SearchOptions) (*SearchPlan, error)
ExplainVectorSearch(collection string, vector []float64, options *SearchOptions) (*SearchPlan, error)
ExplainHybridSearch(collection string, query HybridQuery) (*SearchPlan, error)
```

```rust
// Rust
pub async fn create_search_index(&self, collection: &str, config: SearchIndexConfig) -> Result<()>
pub async fn explain_text_search(&self, collection: &str, query: &str, options: Option<SearchOptions>) -> Result<SearchPlan>
pub async fn explain_vector_search(&self, collection: &str, vector: Vec<f64>, options: Option<SearchOptions>) -> Result<SearchPlan>
pub async fn explain_hybrid_search(&self, collection: &str, query: HybridQuery) -> Result<SearchPlan>
```

```python
# Python
def create_search_index(self, collection: str, config: SearchIndexConfig) -> None
def explain_text_search(self, collection: str, query: str, options: Optional[SearchOptions] = None) -> SearchPlan
def explain_vector_search(self, collection: str, vector: List[float], options: Optional[SearchOptions] = None) -> SearchPlan
def explain_hybrid_search(self, collection: str, query: HybridQuery) -> SearchPlan
```

```kotlin
// Kotlin
suspend fun createSearchIndex(collection: String, config: SearchIndexConfig)
suspend fun explainTextSearch(collection: String, query: String, options: SearchOptions? = null): SearchPlan
suspend fun explainVectorSearch(collection: String, vector: List<Double>, options: SearchOptions? = null): SearchPlan
suspend fun explainHybridSearch(collection: String, query: HybridQuery): SearchPlan
```

### Use Cases

- **Search Setup:** Dynamically create text/vector indexes for collections
- **Performance Tuning:** Analyze search query execution before running
- **Index Configuration:** Configure HNSW parameters, BM25 settings, etc.

### Implementation Priority

🔴 **HIGH** - Essential for search-heavy applications

---

## 3. KV Document Linking ✅ (Implemented March 2026 — uses regular auth)

### Description

Document linking allows KV store entries to reference collection documents,
creating relationships between KV cache data and persistent documents.

### Server Endpoints

```
GET    /api/kv/{key}/links                                  - Get all linked documents for key
POST   /api/kv/{key}/links/{collection}/{document_id}      - Create link. A body MUST be sent; its fields (keys/field_path/metadata) are all optional, so `{}` is valid
DELETE /api/kv/{key}/links/{collection}/{document_id}      - Remove link
```

### Missing Client Methods

**All Languages Need:**

```typescript
// TypeScript/JavaScript
kvGetLinks(key: string): Promise<Record>
kvLink(key: string, collection: string, documentId: string, linkData?: LinkData): Promise<Record>
kvUnlink(key: string, collection: string, documentId: string): Promise<Record>
```

```go
// Go
KVGetLinks(key string) ([]map[string]interface{}, error)
KVLink(key, collection, documentId string) (map[string]interface{}, error)
KVUnlink(key, collection, documentId string) (map[string]interface{}, error)
```

```rust
// Rust
pub async fn kv_get_links(&self, key: &str) -> Result<serde_json::Value>
pub async fn kv_link(&self, key: &str, collection: &str, document_id: &str) -> Result<serde_json::Value>
pub async fn kv_unlink(&self, key: &str, collection: &str, document_id: &str) -> Result<serde_json::Value>
```

```python
# Python
await client.kv_get_links(key)
await client.kv_link(key, collection, document_id)
await client.kv_unlink(key, collection, document_id)
```

```kotlin
// Kotlin
suspend fun kvGetLinks(key: String): JsonArray
suspend fun kvLink(key: String, collection: String, documentId: String, linkData: JsonObject = buildJsonObject {}): JsonObject
suspend fun kvUnlink(key: String, collection: String, documentId: String): JsonObject
```

### Use Cases

- **Cache Relationships:** Link cached data to source documents
- **Invalidation:** Track which documents are referenced by cache keys
- **Consistency:** Maintain relationships between KV store and collections

---

## 4. Schedule Management (Cron Jobs) ✅

### Description

Schedules run saved functions on a cron schedule. Full CRUD and immediate
triggering are available on the server; these routes require admin
authentication.

### Server Endpoints

```
POST   /api/schedules                           - Create schedule
GET    /api/schedules                           - List all schedules
GET    /api/schedules/{id}                      - Get schedule details
PUT    /api/schedules/{id}                      - Update schedule
DELETE /api/schedules/{id}                      - Delete schedule
POST   /api/schedules/{id}/trigger              - Run a schedule now
GET    /api/schedules/stats                     - Schedule execution stats

NOTE: there is no /pause or /resume endpoint. Pause and resume are a partial
update - PUT /api/schedules/{id} with {"enabled": false|true} - which is what
the client pauseSchedule/resumeSchedule helpers do. This file previously listed
POST .../pause and .../resume, and every client implemented them literally; all
of them 404'd. Keep this table matched to the server's actual route table.
```

### Client Methods

All languages expose:

- `createSchedule(config)` - Create cron job
- `listSchedules()` - List all schedules
- `getSchedule(id)` - Get schedule details
- `updateSchedule(id, config)` - Update schedule
- `deleteSchedule(id)` - Delete schedule
- `pauseSchedule(id)` - Pause execution
- `resumeSchedule(id)` - Resume execution
- `triggerSchedule(id)` - Run immediately

### Use Cases

- **Automated Tasks:** Run cleanup, aggregations, reports on schedule
- **Data Processing:** Periodic ETL jobs, batch processing
- **Maintenance:** Scheduled backups, cache warming

---

## 5. Advanced Schema Features ❌

### Description

Advanced schema validation and constraint management beyond basic schema
operations.

### Server Endpoints

```
PUT /api/schema/{collection}/constraints        - Update schema constraints
GET /api/schema/{collection}/validate           - Validate records against schema
```

### Missing Client Methods

**All Languages Need:**

- `updateSchemaConstraints(collection, constraints)` - Set validation rules
- `validateRecords(collection, records)` - Pre-validate before insert

### Use Cases

- **Data Quality:** Enforce strict validation rules
- **Migration Safety:** Validate data before schema changes
- **Compliance:** Ensure data meets regulatory requirements

### Implementation Priority

🟢 **LOW** - Specialized use cases

---

## 6. WAL (Write-Ahead Log) Operations ❌

### Description

Direct access to Write-Ahead Log for replication gap filling and advanced
debugging.

### Server Endpoints

```
GET  /api/wal/health                            - Get WAL health status
POST /api/wal/rotate                            - Force WAL rotation
GET  /api/wal/entries                           - Get WAL entries in time range
POST /api/replication/wal                       - Receive WAL shipment from peer
```

### Missing Client Methods

**All Languages Need:**

- `getWALHealth()` - Check WAL status
- `rotateWAL()` - Force log rotation
- `getWALEntries(startTime, endTime)` - Retrieve historical operations

### Use Cases

- **Replication Debugging:** Inspect WAL for replication issues
- **Audit Logging:** Track all database operations
- **Disaster Recovery:** Replay WAL entries

### Implementation Priority

🟢 **LOW** - Admin/debugging tool, not typical application use

---

## Remaining Roadmap

### Query & Search Index Management

**Target:** Q1 2026

- Implement query index methods (create, list, delete, explain)
- Implement search index methods (create, explain variants)
- Add comprehensive tests for all index operations
- Update examples showing index usage

KV document linking, schedule management (including trigger), and schema
constraint updates are implemented. Remaining work in this historical roadmap is
limited to deliberately admin-focused index/WAL surfaces and any future server
contract additions.

---

## Type Definitions Needed

### Query Index Types

```typescript
interface IndexOptions {
  unique?: boolean;
  sparse?: boolean;
}

interface QueryIndex {
  field: string;
  type: string;
  unique: boolean;
  sparse: boolean;
}

interface QueryPlan {
  collection: string;
  indexUsed?: string;
  estimatedCost: number;
  stages: PlanStage[];
}
```

### Search Index Types

```typescript
interface SearchIndexConfig {
  type: "text" | "vector";
  field: string;
  textConfig?: {
    language?: string;
    stopWords?: string[];
  };
  vectorConfig?: {
    dimensions: number;
    metric?: "cosine" | "euclidean" | "dot";
    hnsw?: {
      m?: number;
      efConstruction?: number;
    };
  };
}

interface SearchPlan {
  indexUsed: string;
  queryType: "text" | "vector" | "hybrid";
  estimatedResults: number;
  executionSteps: string[];
}
```

### KV Link Types

```typescript
interface DocumentLink {
  collection: string;
  documentId: string;
  createdAt: string;
}

interface LinkData {
  key: string;
  collection: string;
  documentId: string;
}
```

### Schedule Types

```typescript
interface Schedule {
  id: string;
  name: string;
  cron_expression: string;
  function_label: string;
  parameters?: Record<string, unknown>;
  enabled: boolean;
  timezone?: string;
  next_execution?: string;
  last_execution?: string;
}

interface ScheduleConfig {
  name: string;
  cron_expression: string;
  function_label: string;
  parameters?: Record<string, unknown>;
  enabled?: boolean;
  timezone?: string;
}
```

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
