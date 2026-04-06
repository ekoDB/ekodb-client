# ekodb-client v0.16.0: Transparent Transport Routing

**Status:** Planning — March 27, 2026

**Prerequisite:** v0.15.1 (WS CRUD parity, schema cache, SSE subscriptions)

---

## Goal

Make transport selection invisible to users. `client.insert()` automatically
uses the fastest available channel (WS > REST). `client.subscribe()`
automatically uses WS if connected, falls back to SSE. Users never need to call
`ws.insert()` vs `client.insert()` — the client picks the optimal path.

---

## Design

### Transport Priority

```
CRUD operations (insert, query, update, delete, batch*):
  1. WS (if connected) — zero HTTP overhead, reuses auth'd connection
  2. REST (fallback) — always works, MessagePack serialization

Subscriptions:
  1. WS (if connected) — bidirectional, real-time
  2. SSE (fallback) — works behind reverse proxies that block WS upgrades

Schema resolution:
  1. Schema cache (in-memory) — zero network
  2. GET /api/schemas/{collection}/version — cheap staleness check
  3. GET /api/schemas/{collection} — full schema fetch on cache miss
```

### Config

```rust
let client = EkoDBClient::builder()
    .base_url("https://api.ekodb.net")
    .api_key("key")
    .schema_cache(true)
    .auto_connect_ws(true)       // auto-establish WS on first call
    .transport_upgrade(true)      // route CRUD through WS when available
    .build()?;

// These all transparently use WS if connected:
client.insert("users", data).await?;
client.query("users", filter).await?;
let rx = client.subscribe("orders").await?;  // WS or SSE automatically
```

### Implementation Approach

Each `Client` method checks `self.ws_connected()` before choosing transport:

```rust
async fn insert(&self, collection: &str, data: Value) -> Result<Value> {
    if self.config.transport_upgrade && self.ws_connected() {
        return self.ws.insert(collection, data, None).await;
    }
    // Existing REST path
    self.execute_with_token_refresh(|token| {
        self.http.insert(collection, &data, &token)
    }).await
}
```

### WS Auto-Connect

When `auto_connect_ws` is enabled:

- First CRUD call triggers `connect_ws()` in the background
- First call goes via REST (don't block on WS handshake)
- Subsequent calls use WS once connected
- Auto-reconnect on disconnect with exponential backoff
- Grace period: don't attempt WS reconnect more than once per 5 seconds

### Subscription Fallback

```rust
async fn subscribe(&self, collection: &str, opts: SubscribeOptions) -> Result<Receiver<Mutation>> {
    // Try WS first
    if self.ws_connected() {
        return self.ws.subscribe(collection, opts).await;
    }
    // Fall back to SSE
    self.subscribe_sse(collection, opts.filter_field, opts.filter_value).await
}
```

### MessagePack over WS

v0.15.1 WS is JSON-only. v0.16.0 adds optional MessagePack:

- Negotiate format on WS connect via query param `?format=msgpack`
- Server checks `format` param, uses MessagePack for binary frames
- Falls back to JSON for text frames (backward compat)
- Estimated 30-40% bandwidth reduction for large record payloads

---

## Scope

### In v0.16.0

- Transparent CRUD routing (WS when connected, REST fallback)
- Transparent subscription routing (WS → SSE fallback)
- WS auto-connect with reconnect
- MessagePack over WS (server + all clients)
- KV operations over WS (server needs to add KV to WebSocketMessage enum)
- Transaction operations over WS (server needs to add Transaction support)

### NOT in v0.16.0

- Live queries (re-run query on mutation, push diffs) — needs server-side change
  detection
- WS multiplexing (multiple logical connections over one socket)
- QUIC/HTTP3 transport

---

## Build Sequence

```
1. Server: Add KV + Transaction WS message types
2. Server: MessagePack WS negotiation
3. Rust client: transparent routing + auto-connect + reconnect
4. Python bindings: transparent routing wrapper
5. Go client: transparent routing + auto-connect
6. TypeScript client: transparent routing + auto-connect
7. Kotlin client: transparent routing + auto-connect
```

---

## Migration Guide (v0.15.1 → v0.16.0)

No breaking changes. New config options are opt-in and default to `false`.

```rust
// v0.15.1 (still works):
let ws = client.connect_ws().await?;
let record = ws.insert("users", data).await?;

// v0.16.0 (new, recommended):
let client = EkoDBClient::builder()
    .transport_upgrade(true)
    .build()?;
let record = client.insert("users", data).await?;  // auto-routes
```
