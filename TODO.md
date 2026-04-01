# ekodb-client Transport Parity TODO

**Created:** April 1, 2026

## Current State

All 5 clients (Rust, TypeScript, Python, Go, Kotlin) support HTTP REST, WebSocket,
and SSE. But CRUD-over-WebSocket coverage is inconsistent.

## Transport Parity Matrix

### CRUD over WebSocket

| Operation       | Rust | TypeScript | Python | Go      | Kotlin |
|-----------------|------|------------|--------|---------|--------|
| Insert          | -    | WS         | -      | WS      | WS     |
| Find/Query      | WS   | WS         | WS     | WS      | WS     |
| Update          | -    | WS         | -      | WS      | WS     |
| Delete          | -    | WS         | -      | WS      | WS     |
| Batch Insert    | -    | WS         | -      | -       | WS     |
| Batch Update    | -    | WS         | -      | -       | WS     |
| Batch Delete    | -    | WS         | -      | -       | WS     |
| Text Search     | -    | WS         | -      | -       | -      |
| Distinct Values | -    | WS         | -      | -       | -      |

`-` = HTTP only, `WS` = WebSocket supported

### KV over WebSocket

No client supports KV operations over WebSocket. All KV is HTTP-only.

### Schema/Collection Management over WebSocket

Only Kotlin supports `createCollection`/`deleteCollection` over WS.
All others are HTTP-only.

## Tasks

### Phase 1: Rust WS CRUD (Python gets it for free)

- [ ] Add WS insert (single) to Rust client
- [ ] Add WS update (single) to Rust client
- [ ] Add WS delete (single) to Rust client
- [ ] Add WS batch insert to Rust client
- [ ] Add WS batch update to Rust client
- [ ] Add WS batch delete to Rust client
- [ ] Add WS text search to Rust client
- [ ] Add WS distinct values to Rust client
- [ ] Verify Python client inherits all WS CRUD from Rust

### Phase 2: Go WS Batch Operations

- [ ] Add WS batch insert to Go client
- [ ] Add WS batch update to Go client
- [ ] Add WS batch delete to Go client
- [ ] Add WS text search to Go client
- [ ] Add WS distinct values to Go client

### Phase 3: Kotlin Remaining WS Operations

- [ ] Add WS text search to Kotlin client
- [ ] Add WS distinct values to Kotlin client

### Phase 4: KV over WebSocket (all clients)

- [ ] Decide if KV over WS is needed (currently HTTP-only everywhere)
- [ ] If yes, implement in Rust → Python, then TypeScript, Go, Kotlin