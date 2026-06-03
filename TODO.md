# ekodb-client Transport Parity TODO

**Created:** April 1, 2026

## Current State

All 5 clients (Rust, TypeScript, Python, Go, Kotlin) support HTTP REST,
WebSocket, and SSE. But CRUD-over-WebSocket coverage is inconsistent.

## Transport Parity Matrix

### CRUD over WebSocket

| Operation       | Rust | TypeScript | Python | Go  | Kotlin |
| --------------- | ---- | ---------- | ------ | --- | ------ |
| Insert          | -    | WS         | -      | WS  | WS     |
| Find/Query      | WS   | WS         | WS     | WS  | WS     |
| Update          | -    | WS         | -      | WS  | WS     |
| Delete          | -    | WS         | -      | WS  | WS     |
| Batch Insert    | -    | WS         | -      | -   | WS     |
| Batch Update    | -    | WS         | -      | -   | WS     |
| Batch Delete    | -    | WS         | -      | -   | WS     |
| Text Search     | -    | WS         | -      | -   | -      |
| Distinct Values | -    | WS         | -      | -   | -      |

`-` = HTTP only, `WS` = WebSocket supported

### KV over WebSocket

No client supports KV operations over WebSocket. All KV is HTTP-only.

### Schema/Collection Management over WebSocket

Only Kotlin supports `createCollection`/`deleteCollection` over WS. All others
are HTTP-only.

## Tasks

### Blob storage + imports API (all 4 clients)

> Tracked: [#121](https://github.com/ekoDB/ekodb-client/issues/121) · Go client
> [ekodb-client-go#32](https://github.com/ekoDB/ekodb-client-go/issues/32)

Expose the server blob primitive + imports ingest with matching ergonomics
across Rust, Python, TypeScript, and Kotlin (clients do not extract text —
`text` is supplied by the caller). Pairs with server-side blob storage + ingest
endpoints tracked internally; start once those endpoints are available:

- [ ] `client.blob.*`: `put` / `get` / `get_stream` / `presign_url` / `delete` /
      `list` → `PUT/GET/DELETE /api/blob/{bucket}/{key}`
- [ ] `upload_file(...)` → `POST /api/uploads`, returns `upload_id` (body:
      `{bytes, text, filename, content_type, size}`)
- [ ] `get_upload(upload_id)` → `GET /api/uploads/{id}` (imports record)
- [ ] `get_upload_download_url(upload_id)` → `GET /api/uploads/{id}/download`
      (signed GET URL)
- [ ] Versions synced (`make sync-versions`); unit tests per language + a Rust
      integration round-trip

### Rust WS CRUD (Python gets it for free)

- [ ] Add WS insert (single) to Rust client
- [ ] Add WS update (single) to Rust client
- [ ] Add WS delete (single) to Rust client
- [ ] Add WS batch insert to Rust client
- [ ] Add WS batch update to Rust client
- [ ] Add WS batch delete to Rust client
- [ ] Add WS text search to Rust client
- [ ] Add WS distinct values to Rust client
- [ ] Verify Python client inherits all WS CRUD from Rust

### Go WS Batch Operations

- [ ] Add WS batch insert to Go client
- [ ] Add WS batch update to Go client
- [ ] Add WS batch delete to Go client
- [ ] Add WS text search to Go client
- [ ] Add WS distinct values to Go client

### Kotlin Remaining WS Operations

- [ ] Add WS text search to Kotlin client
- [ ] Add WS distinct values to Kotlin client

### KV over WebSocket (all clients)

- [ ] Decide if KV over WS is needed (currently HTTP-only everywhere)
- [ ] If yes, implement in Rust → Python, then TypeScript, Go, Kotlin
