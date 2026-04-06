# ekodb-client v0.15.1: Full WS CRUD Parity + Schema Cache + SSE Subscriptions

**Status:** Complete — March 27, 2026

---

## Goals

1. **Full WS CRUD Parity** — expose all 17 server-supported WS operations in
   every client
2. **Schema Cache** — opt-in in-memory LRU cache per client instance for
   schema/primary_key resolution
3. **SSE Subscriptions** — collection mutation subscriptions over SSE for
   clients behind reverse proxies
4. **Server Schema Push** — realtime schema/config change notifications to
   connected clients
5. **`extract_record_id`** — universal ID extractor that handles custom
   `primary_key_alias`

---

## Architecture: Schema Cache

### Problem

ekoDB's `primary_key_alias` is configurable per collection (default `"id"`, but
can be `"_id"`, `"user_id"`, `"{collection}_id"`, etc.). Clients that hardcode
`"id"` break when users customize this. Every ID extraction currently requires
either guessing or making a schema API call.

### Solution

```
EkoDBClient
├── SchemaCache (opt-in, per-instance)
│   ├── LRU<collection_name, CachedSchema>
│   │   ├── primary_key_alias: String
│   │   ├── fields: HashMap<name, FieldSchema>
│   │   ├── version: u64
│   │   └── cached_at: Instant
│   ├── max_entries: usize (default 100)
│   ├── ttl: Duration (default 5min, fallback if no realtime channel)
│   └── Invalidation channels:
│       ├── WS: SchemaChanged { collection, version } event (instant)
│       └── SSE: event: schema_changed (instant, for REST-only clients)
```

### Cache lifecycle

1. **Miss**: CRUD function needs schema → fetch from
   `GET /api/schema/{collection}` → store in LRU
2. **Hit**: return cached schema (O(1) HashMap lookup, no network)
3. **Invalidation (proactive)**: server pushes `SchemaChanged` event → evict
   that collection
4. **Invalidation (TTL)**: entry older than TTL → re-fetch on next access
5. **Manual**: `client.schema_cache().invalidate("users")` / `.invalidate_all()`

### Config

```rust
let client = EkoDBClient::builder()
    .base_url("https://api.ekodb.net")
    .api_key("key")
    .schema_cache(true)          // enable (default: false for backward compat)
    .schema_cache_ttl(300)       // seconds (default 300)
    .schema_cache_max(100)       // max collections (default 100)
    .build()?;
```

When disabled, behavior is identical to v0.15.0 — no overhead.

### Integration with CRUD

Every CRUD method that returns records uses the cached `primary_key_alias`:

```rust
// Internal flow (invisible to user):
async fn insert(&self, collection: &str, data: Value) -> Result<Value> {
    let result = self.http_insert(collection, data).await?;
    // ID field in response uses the collection's primary_key_alias
    // extract_record_id() uses cache to know the alias
    Ok(result)
}
```

The `extract_record_id()` function (added in this session) becomes cache-aware:

```rust
pub fn extract_record_id(&self, collection: &str, record: &Value) -> Option<String> {
    let alias = self.schema_cache()
        .map(|c| c.get_alias(collection))
        .flatten()
        .unwrap_or("id".to_string());
    ekodb_client::extract_record_id(record, &[&alias])
}
```

---

## Phase 0: Server Prerequisites (ekodb)

### 0A. MessageId Echo in WS Responses

**Why:** `serialize_ws_response()` doesn't include `messageId` in Success/Error
responses. The single-pending-request heuristic breaks with concurrent WS CRUD.

**File:** `ekodb/ekodb_server/src/handlers.rs`

- Modify `serialize_ws_response()`: add `message_id: Option<&str>` parameter
- In Success/Error branches: include `"messageId": message_id` when present
- In `handle_websocket()` (~line 797): extract `messageId` from raw JSON before
  parsing into `WebSocketMessage`, pass through to `serialize_ws_response()`

**Tests:**

- Unit: `serialize_ws_response` with/without messageId
- Integration: WS Insert with messageId → response includes same messageId

### 0B. SchemaChanged WS Event

**Why:** Clients need to know when schema/primary_key changes so their cache
stays fresh.

**File:** `ekodb/ekodb_server/src/connection.rs`

- Add `SchemaChanged` variant to outbound WS response enum:
  ```rust
  SchemaChanged {
      collection: String,
      version: u64,
      primary_key_alias: String,
  }
  ```
- In `set_collection_schema()` and `set_primary_key_alias()`: broadcast
  `SchemaChanged` to all connected WS clients subscribed to that collection (or
  all clients).

**Tests:**

- Integration: change schema via REST → verify WS client receives
  `SchemaChanged` event
- Integration: change `primary_key_alias` → verify event includes new alias

### 0C. SSE Subscription Endpoint

**Endpoint:**
`GET /api/subscribe/{collection}?filter_field=...&filter_value=...`

**File:** `ekodb/ekodb_server/src/handlers.rs`

- Auth: JWT from `Authorization: Bearer` header
- Uses existing `SubscriptionManager.subscribe_with_filter()`
- SSE events: `event: subscribed` (ack), `event: mutation` (data),
  `event: schema_changed`, `event: error`
- Heartbeat via `warp::sse::keep_alive()`
- Cleanup: unsubscribe on stream drop
- Same `MutationEvent` format as WS notifications

**Tests:**

- Integration: SSE subscribe → insert record → verify SSE `mutation` event
  received
- Integration: SSE subscribe with filter → verify only matching mutations arrive
- Integration: SSE auth (401 without token)
- Integration: change schema → verify `schema_changed` SSE event

### 0D. Schema Version Endpoint

**Endpoint:** `GET /api/schema/{collection}/version`

**Returns:**
`{ "collection": "users", "version": 3, "primary_key_alias": "id" }`

Ultra-cheap check for "has my cached schema gone stale?" — single tiny GET vs
full schema fetch.

**Tests:**

- Unit: returns correct version after schema update
- Integration: create collection → get version → update schema → version
  increments

---

## Phase 1: Rust Client (ekodb_client)

### 1A. Schema Cache Module

**New file:** `ekodb_client/src/schema_cache.rs`

```rust
pub struct SchemaCache {
    entries: DashMap<String, CachedSchema>,
    max_entries: usize,
    ttl: Duration,
    enabled: bool,
}

pub struct CachedSchema {
    pub primary_key_alias: String,
    pub fields: HashMap<String, FieldSchema>,
    pub version: u64,
    pub cached_at: Instant,
}

impl SchemaCache {
    pub fn get_alias(&self, collection: &str) -> Option<String>;
    pub fn get(&self, collection: &str) -> Option<&CachedSchema>;
    pub fn insert(&self, collection: &str, schema: CachedSchema);
    pub fn invalidate(&self, collection: &str);
    pub fn invalidate_all(&self);
    pub fn handle_schema_changed(&self, collection: &str, version: u64, alias: &str);
}
```

**Tests (unit, 10+):**

- insert + get round-trip
- TTL expiry (entry becomes stale)
- invalidate single collection
- invalidate_all clears everything
- max_entries eviction (LRU)
- handle_schema_changed updates entry
- handle_schema_changed with newer version replaces
- handle_schema_changed with older version is ignored
- disabled cache returns None
- concurrent access (multi-threaded)

### 1B. WebSocketClient Full CRUD

**File:** `ekodb_client/src/websocket.rs`

Add `WebSocketRequest` variants and public methods for each missing operation.
Pattern: `ensure_connected()` → generate messageId → construct request →
`send_and_wait()` → parse response.

**New WS request variants (14):**

| Method                                                         | WS Message Type  | Returns                          |
| -------------------------------------------------------------- | ---------------- | -------------------------------- |
| `ws.insert(collection, record, bypass_ripple?)`                | Insert           | Record                           |
| `ws.query(collection, filter?, sort?, limit?, skip?)`          | Query            | Vec\<Record\>                    |
| `ws.find_by_id(collection, id)`                                | FindById         | Record (already exists — verify) |
| `ws.update(collection, id, record, bypass_ripple?)`            | Update           | Record                           |
| `ws.delete(collection, id, bypass_ripple?)`                    | Delete           | ()                               |
| `ws.batch_insert(collection, records, bypass_ripple?)`         | BatchInsert      | Vec\<Record\>                    |
| `ws.batch_update(collection, updates, bypass_ripple?)`         | BatchUpdate      | Vec\<Record\>                    |
| `ws.batch_delete(collection, ids, bypass_ripple?)`             | BatchDelete      | ()                               |
| `ws.text_search(collection, query, fields?, limit?)`           | TextSearch       | Vec\<SearchResult\>              |
| `ws.distinct_values(collection, field, filter?)`               | DistinctValues   | Vec\<Value\>                     |
| `ws.update_with_action(collection, id, action, field, value?)` | UpdateWithAction | Record                           |
| `ws.create_collection(name, schema?)`                          | CreateCollection | ()                               |
| `ws.list_collections()`                                        | GetCollections   | Vec\<String\>                    |
| `ws.delete_collection(name)`                                   | DeleteCollection | ()                               |

**Concurrent request support:**

- Each request gets a unique `messageId` (UUID)
- Response dispatcher matches `messageId` to pending request channel
- Replace single-pending heuristic with `HashMap<messageId, oneshot::Sender>`

**Tests (unit, 14 + integration, 5):**

- Unit: each method constructs correct JSON payload
- Unit: messageId is included in all requests
- Integration: WS insert → WS find_by_id → WS update → WS delete round-trip
- Integration: WS batch_insert → WS query → verify count
- Integration: WS text_search returns ranked results
- Integration: WS concurrent requests (3 parallel inserts, all succeed)
- Integration: WS create_collection → list_collections → delete_collection

### 1C. SSE Subscribe

**File:** `ekodb_client/src/http.rs` + `ekodb_client/src/client.rs`

```rust
pub async fn subscribe_sse(
    &self,
    collection: &str,
    filter_field: Option<&str>,
    filter_value: Option<&str>,
) -> Result<mpsc::Receiver<MutationNotificationPayload>>
```

- Spawns background task that connects to `GET /api/subscribe/{collection}`
- Parses SSE `data:` lines into `MutationNotificationPayload`
- Also handles `event: schema_changed` → invalidates schema cache
- Returns `mpsc::Receiver` for the caller to consume

**Tests (integration, 3):**

- SSE subscribe → REST insert → verify event on channel
- SSE subscribe with filter → verify only matching events
- SSE schema_changed event → verify cache invalidated

### 1D. Schema-Aware `extract_record_id`

**Already done** (this session).
`ekodb_client::extract_record_id(record, extra_candidates)` in `utils.rs`.

Extend with cache-aware instance method on `Client`:

```rust
impl Client {
    pub fn extract_id(&self, collection: &str, record: &Value) -> Option<String> {
        let extra = self.schema_cache()
            .and_then(|c| c.get_alias(collection))
            .map(|a| vec![a])
            .unwrap_or_default();
        let refs: Vec<&str> = extra.iter().map(|s| s.as_str()).collect();
        extract_record_id(record, &refs)
    }
}
```

### 1E. Client Convenience

**File:** `ekodb_client/src/client.rs`

```rust
pub async fn connect_ws(&self) -> Result<WebSocketClient> {
    let ws_url = self.base_url.replace("http", "ws") + "/ws";
    WebSocketClient::connect(&ws_url, &self.token).await
}
```

**Tests:**

- Unit: URL conversion (http→ws, https→wss)

---

## Phase 2: Python Bindings (ekodb-client-py)

**File:** `ekodb-client-py/src/lib.rs`

Add PyO3 `#[pymethods]` for each new WS method and `subscribe_sse()`. Follow
existing `find_all` pattern.

**New methods:** `ws_insert`, `ws_query`, `ws_update`, `ws_delete`,
`ws_batch_insert`, `ws_batch_update`, `ws_batch_delete`, `ws_text_search`,
`ws_distinct_values`, `ws_update_with_action`, `ws_create_collection`,
`ws_list_collections`, `ws_delete_collection`, `subscribe_sse`.

**Schema cache:** exposed as `client.enable_schema_cache(ttl=300, max=100)` and
`client.invalidate_schema_cache(collection=None)`.

**Tests (6):**

- test_ws_insert_query_round_trip
- test_ws_batch_operations
- test_sse_subscribe
- test_schema_cache_basic
- test_extract_record_id
- test_extract_record_id_custom_alias

---

## Phase 3: Go Client (ekodb-client-go)

### 3A. WebSocketClient CRUD

**File:** `ekodb-client-go/websocket.go`

Add methods following `FindAll` pattern (~line 545): construct
`map[string]interface{}` request → `sendRequest()` → parse response. Same 14
operations as Rust.

### 3B. SSE Subscribe

**File:** `ekodb-client-go/sse.go` (new)

```go
func (c *Client) SubscribeSSE(collection string, opts ...SubscribeOption) (<-chan MutationNotification, error)
```

Uses `streamClient` (no timeout) with `Accept: text/event-stream`. Parses SSE
with `bufio.Scanner`.

### 3C. Schema Cache

**File:** `ekodb-client-go/schema_cache.go` (new)

```go
type SchemaCache struct {
    entries sync.Map
    ttl     time.Duration
    enabled bool
}

func (c *Client) EnableSchemaCache(ttl time.Duration, maxEntries int)
func (c *Client) ExtractRecordID(collection string, record map[string]interface{}) string
```

### 3D. Connect Helper

```go
func (c *Client) ConnectWS(wsURL string) (*WebSocketClient, error)
```

**Tests (12):**

- Unit: each WS method constructs correct JSON
- Unit: schema cache insert/get/invalidate/TTL
- Unit: ExtractRecordID with default, \_id, custom alias
- Integration: WS CRUD round-trip
- Integration: SSE subscribe + insert → verify event

---

## Phase 4: TypeScript Client (ekodb-client-ts)

### 4A. WebSocketClient CRUD

**File:** `ekodb-client-ts/src/client.ts` (WebSocketClient class ~line 3029)

Add methods following `findAll()` pattern. Raw JSON construction with messageId.

### 4B. SSE Subscribe

```typescript
subscribeSSE(collection: string, options?: SubscribeOptions): EventStream<MutationNotification>
```

Uses `fetch()` with streaming response body. Returns async iterable.

### 4C. Schema Cache

```typescript
class SchemaCache {
  private entries: Map<string, CachedSchema>;
  get(collection: string): CachedSchema | undefined;
  invalidate(collection: string): void;
  handleSchemaChanged(event: SchemaChangedEvent): void;
}

// Config
const client = new EkoDBClient({ schemaCache: true, schemaCacheTTL: 300 });
client.extractRecordId(collection, record); // cache-aware
```

**Tests (vitest, 10):**

- Unit: each WS method
- Unit: schema cache lifecycle
- Unit: extractRecordId with aliases
- Integration: SSE subscribe

---

## Phase 5: Kotlin Client (ekodb-client-kt)

### 5A. WebSocketClient CRUD

**File:** `ekodb-client-kt/.../WebSocketClient.kt`

Methods following `findAll()` pattern. Uses `buildJsonObject`.

### 5B. SSE Subscribe

```kotlin
fun subscribeSSE(collection: String, filterField: String?, filterValue: String?): Flow<MutationNotification>
```

Uses Ktor HTTP streaming.

### 5C. Schema Cache

```kotlin
class SchemaCache(val ttl: Duration, val maxEntries: Int) {
    fun getAlias(collection: String): String?
    fun invalidate(collection: String)
    fun handleSchemaChanged(event: SchemaChangedEvent)
}
```

**Tests (JUnit, 6):**

- Unit: WS methods
- Unit: schema cache
- Unit: extractRecordId

---

## Build Sequence

```
Phase 0: Server (messageId echo + SchemaChanged event + SSE endpoint + version endpoint)
    ↓ prerequisite for all clients
Phase 1: Rust client (schema cache + WS CRUD + SSE subscribe)
    ↓ prerequisite for Python
Phase 2: Python bindings (PyO3 wrappers)
Phase 3: Go client      ← independent, can parallel with 4+5
Phase 4: TypeScript client ← independent
Phase 5: Kotlin client     ← independent
```

Phases 3-5 can be fully parallelized.

---

## Error Mapping (WS → Client Error)

Standardize across all languages:

| WS Error Code | Client Error Type       |
| ------------- | ----------------------- |
| 404           | `NotFound`              |
| 401           | `TokenExpired`          |
| 429           | `RateLimit`             |
| 400           | `Validation(message)`   |
| 409           | `Conflict(message)`     |
| 500+          | `Api { code, message }` |

---

## Verification Matrix

### Server

| Test                                      | Type        | Validates            |
| ----------------------------------------- | ----------- | -------------------- |
| serialize_ws_response with messageId      | Unit        | MessageId echo       |
| serialize_ws_response without messageId   | Unit        | Backward compat      |
| WS Insert with messageId → response match | Integration | End-to-end messageId |
| WS concurrent requests (3 parallel)       | Integration | No response mixup    |
| SSE subscribe → insert → event received   | Integration | SSE mutations        |
| SSE subscribe with filter → filtering     | Integration | SSE filter           |
| SSE auth 401 without token                | Integration | SSE auth             |
| Schema change → WS SchemaChanged event    | Integration | Schema push (WS)     |
| Schema change → SSE schema_changed event  | Integration | Schema push (SSE)    |
| GET /api/schema/{coll}/version            | Integration | Version endpoint     |

### Rust Client

| Test                                        | Type        | Validates         |
| ------------------------------------------- | ----------- | ----------------- |
| Schema cache insert/get/TTL/eviction        | Unit (10)   | Cache correctness |
| Schema cache concurrent access              | Unit        | Thread safety     |
| SchemaChanged handler updates cache         | Unit        | Invalidation      |
| Each WS method JSON payload                 | Unit (14)   | Request format    |
| messageId included in all WS requests       | Unit        | Correlation       |
| extract_record_id with default/custom/typed | Unit (7)    | ID extraction     |
| WS insert → find_by_id → update → delete    | Integration | CRUD round-trip   |
| WS batch_insert → query → count             | Integration | Batch + query     |
| WS 3 parallel inserts                       | Integration | Concurrency       |
| WS create/list/delete collection            | Integration | Collection mgmt   |
| SSE subscribe + insert → event              | Integration | SSE               |
| SSE schema_changed → cache invalidated      | Integration | Schema push       |
| REST insert → extract_id with custom alias  | Integration | End-to-end alias  |

### Go Client

| Test                     | Type        | Validates         |
| ------------------------ | ----------- | ----------------- |
| Each WS method JSON      | Unit (14)   | Request format    |
| Schema cache lifecycle   | Unit (5)    | Cache correctness |
| ExtractRecordID variants | Unit (4)    | ID extraction     |
| WS CRUD round-trip       | Integration | End-to-end        |
| SSE subscribe + insert   | Integration | SSE               |

### TypeScript Client

| Test                     | Type        | Validates         |
| ------------------------ | ----------- | ----------------- |
| Each WS method           | Unit (14)   | Request format    |
| Schema cache lifecycle   | Unit (5)    | Cache correctness |
| extractRecordId variants | Unit (4)    | ID extraction     |
| SSE subscribe            | Integration | SSE               |

### Kotlin Client

| Test                     | Type      | Validates         |
| ------------------------ | --------- | ----------------- |
| Each WS method           | Unit (14) | Request format    |
| Schema cache lifecycle   | Unit (5)  | Cache correctness |
| extractRecordId variants | Unit (4)  | ID extraction     |

### Benchmarks (ekodb/benchmarks/)

| Benchmark                                   | Location         | Validates           |
| ------------------------------------------- | ---------------- | ------------------- |
| WS vs REST insert throughput (1K records)   | benchmarks/api/  | WS overhead         |
| WS vs REST query latency (p50/p95/p99)      | benchmarks/api/  | Latency comparison  |
| WS concurrent CRUD (10 parallel clients)    | benchmarks/api/  | Concurrency         |
| Schema cache hit/miss ratio under load      | benchmarks/api/  | Cache effectiveness |
| SSE subscription latency (mutation → event) | benchmarks/api/  | SSE latency         |
| YCSB Workload A (50/50 R/W) over WS         | benchmarks/ycsb/ | Standard benchmark  |
| YCSB Workload B (95/5 R/W) over WS          | benchmarks/ycsb/ | Read-heavy          |

---

## Scope Boundaries (NOT in v0.15.1)

- Transparent routing (auto WS/REST selection) — deferred to v0.16
- KV operations over WS — server doesn't support them yet
- Transaction operations over WS — server doesn't support them
- Live queries (re-run query on mutation, push diffs) — new server feature
  needed
- MessagePack over WS — server change needed
- Schema migration tooling (auto-apply schema changes) — separate feature

---

## Key Serialization Note

WS is JSON-only; REST supports MessagePack. This is acceptable — the persistent
WS connection saves TLS handshake + HTTP header overhead per request, which more
than compensates for JSON vs MessagePack on the wire. MessagePack over WS can be
added in v0.16.0 if needed.

---

## Migration Guide (v0.15.0 → v0.15.1)

### No breaking changes

All existing APIs work unchanged. New features are additive:

```rust
// Before (still works):
let id = record["id"].as_str();

// After (recommended):
let id = client.extract_id("users", &record);
// or without client instance:
let id = ekodb_client::extract_record_id(&record, &[]);

// Schema cache (opt-in):
let client = EkoDBClient::builder()
    .schema_cache(true)
    .build()?;

// WS CRUD (new):
let ws = client.connect_ws().await?;
let record = ws.insert("users", data).await?;
let results = ws.query("users", Some(filter)).await?;

// SSE subscriptions (new):
let rx = client.subscribe_sse("orders", None, None).await?;
while let Some(event) = rx.recv().await {
    println!("mutation: {:?}", event);
}
```
