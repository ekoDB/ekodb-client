# ekodb-client Transport TODO

**Created:** April 1, 2026 **Updated:** June 3, 2026 — CRUD-over-WS parity
verified complete in code; stale tasks removed.

## Current State

All 5 clients (Rust, TypeScript, Python, Go, Kotlin) support HTTP REST,
WebSocket, and SSE. **CRUD-over-WebSocket parity is complete** — every client
exposes insert/update/delete, batch insert/update/delete, find/query,
`text_search`, and `distinct_values` over WS (verified against source; see
`PARITY_MATRIX.md`). The earlier "inconsistent coverage" matrix was stale.

## Transport Parity Matrix

### CRUD over WebSocket — ✅ complete

| Operation                  | Rust | TypeScript | Python | Go  | Kotlin |
| -------------------------- | ---- | ---------- | ------ | --- | ------ |
| Insert / Update / Delete   | WS   | WS         | WS     | WS  | WS     |
| Find / Query               | WS   | WS         | WS     | WS  | WS     |
| Batch Insert/Update/Delete | WS   | WS         | WS     | WS  | WS     |
| Text Search                | WS   | WS         | WS     | WS  | WS     |
| Distinct Values            | WS   | WS         | WS     | WS  | WS     |

`WS` = WebSocket supported. Python exposes these as `ws_*` methods on its WS
wrapper.

### Collection management over WebSocket — ✅ complete

All five clients support `create` / `delete` / `list` collections over WS
(TypeScript via `sendCRUD` in its `WebSocketClient`).

### KV over WebSocket

No client supports KV over WS — all KV is HTTP-only. Whether to add a WS variant
is an open decision (lean: keep HTTP-only unless a real latency case appears).

## Open work

### Blob storage + imports API (all clients)

> Tracked: [#121](https://github.com/ekoDB/ekodb-client/issues/121) · Go client
> [ekodb-client-go#32](https://github.com/ekoDB/ekodb-client-go/issues/32)

Expose the server blob primitive + imports ingest with matching ergonomics
across Rust, Python, TypeScript, and Kotlin (clients do not extract text —
`text` is supplied by the caller). Pairs with server-side blob storage + ingest
endpoints tracked internally; **start once those endpoints are available**:

- [ ] `client.blob.*`: `put` / `get` / `get_stream` / `presign_url` / `delete` /
      `list` → `PUT/GET/DELETE /api/blob/{bucket}/{key}`
- [ ] `upload_file(...)` → `POST /api/uploads`, returns `upload_id` (body:
      `{bytes, text, filename, content_type, size}`)
- [ ] `get_upload(upload_id)` → `GET /api/uploads/{id}` (imports record)
- [ ] `get_upload_download_url(upload_id)` → `GET /api/uploads/{id}/download`
      (signed GET URL)
- [ ] Versions synced (`make sync-versions`); unit tests per language + a Rust
      integration round-trip

### KV over WebSocket — decision

> Tracked: [#112](https://github.com/ekoDB/ekodb-client/issues/112)

- [ ] Decide whether KV needs a WS variant (HTTP-only today). Lean: no.
