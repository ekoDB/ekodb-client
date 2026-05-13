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

### Phase 5: Hardware function variants (IoT / robotics)

Pairs with `ekodb/documentation/TODO.md → Hardware / Machine I/O`.
Once the server lands `HardwareList`, `GpioWrite`, `SerialOpen/Write/
Read`, `I2cTransaction`, `PwmSet`, etc., clients DO NOT need new
transport plumbing — these go through the existing `callFunction`
path and ride on whichever transport the client already supports.
But the clients DO need ergonomic helpers and examples so users
don't have to hand-build the parameter wrappers (especially the
`Binary` wrapping that CLAUDE.md warns about).

- [ ] Typed parameter-builders per language for the hardware function
      variants. Rust: a `hardware` module under `ekodb_client/src/`
      with `HardwareList::call(client)`, `GpioWrite::new(pin, value)
      .call(client)`, etc. TS/Python/Go/Kotlin: matching helpers.
      Each helper handles the `{type:"Binary", value:"<base64>"}`
      wrapping for byte parameters so callers can pass raw `Vec<u8>`
      / `Uint8Array` / `[]byte` and not think about it.
- [ ] `SchemaCache`-style cache for the `hardware_list` output, with
      a configurable TTL (hardware hot-plugs but rarely; minutes is
      fine). Re-use the existing cache shape from
      `schema_cache.rs` so we get LRU + TTL for free.
- [ ] Examples under `ekodb-client/examples/<lang>/`:
  - [ ] `hardware_blink.{rs,ts,py,go,kt}` — list hardware, find a
        usable GPIO, save a `hw_blink` function, call it N times.
        The "hello world" of the platform.
  - [ ] `hardware_pump.{rs,ts,py,go,kt}` — compound function:
        `hw_pump_run_for(seconds)` composing `PwmSet` + `Sleep` +
        `PwmSet 0`. Shows the `CallFunction` stage pattern.
  - [ ] `hardware_serial_arduino.{rs,ts,py,go,kt}` — open a serial
        port, send `{"cmd":"motor","speed":128}`, read response.
        Demonstrates wrapping `bytes` parameters correctly.
- [ ] Integration tests gated on a `HARDWARE_LOOPBACK` env (a
      software loopback the server exposes when `hardware.enabled
      = false` but `hardware.test_loopback = true` — returns
      fabricated catalog + echoes writes back as reads). Keeps CI
      green without a physical Pi in the loop.
- [ ] Document the hardware permission model in each client's
      README — which scopes to set on the API key, what error
      shape to expect on scope failure (`missing_hardware_scope`),
      and the audit-log-everything contract.
