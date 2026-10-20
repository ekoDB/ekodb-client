---
name: sdk-parity
description:
  Audit and complete ekoDB SDK contract parity across Rust, Python, TypeScript,
  Kotlin and Go when adding or changing client APIs, fields, behavior,
  dependencies or examples. Require source and behavior evidence, ticket
  verified gaps, and prevent partial implementations being reported as complete.
---

# SDK parity

Run this audit before implementation and again before reporting any ekoDB
client-contract change complete. It applies to Codex, Claude and Copilot. A
passing multi-language suite proves only the paths it exercises; it does not
prove an API exists in each SDK.

## Establish the contract and scope

Read the repository guide and existing fixtures, tests and examples first.
Identify the public methods, wire routes, request states, response types,
error/retry behavior, transports and minimum server release affected. Reuse
established fixtures and parity checkers. Do not infer a server fact or read
server source when the user has restricted it.

Check all five SDKs, even when the initial handoff describes one language. Rust,
Python (Rust/PyO3 bindings), TypeScript and Kotlin live in `ekodb-client`; Go
lives in `ekodb-client-go`. Explicit user scope restrictions take precedence:
record the remaining languages as unassessed or incomplete, and do not describe
the whole feature as complete. A narrow code request is not evidence of parity
elsewhere.

## Verify, ticket, then implement

Build an evidence table with one row per language and columns for
implementation, transport-boundary behavior, tests, examples, documentation, and
status. Open the actual source/export/binding for every row. Check the branch
being changed and the default branch separately; do not confuse an open draft
with shipped support. A name match or a successful build is not behavior
evidence.

For each suspected gap, reproduce or confirm it in source before ticketing.
Search existing issues first. Create or update a ticket in the affected
repository, add it to that repository's board, and take it In Progress before
changing code. Link an existing epic when appropriate. One issue may cover an
atomic multi-language implementation; keep independent workflow/process work
separate. Keep private repository names, paths and tickets out of public
artifacts.

Implement every required row using each language's established style. Expose the
Python binding as well as the Rust API. Reuse existing authentication and
transport without exposing unintended operations. Preserve omitted, null and
empty-array states, unknown metadata, safe failures and bounded retries. A
server capability unavailable in an authorized environment remains an explicit
failing/pending verification, never a passing test that asserts the defect.

Add contract fixtures and behavior tests that actually invoke each SDK's public
entry point through its HTTP/WS boundary. Pin negative cases and state
distinctions, not only method names. Update matching examples and their
dependency/build wiring, READMEs, changelog and parity matrix. Reuse existing
example-parity and contract checks; consult the canonical shared workflow
repository before adding CI or a cross-cutting guard. Do not vendor a local CI
mechanism.

## Completion gate

Run the repository's permitted formatting, lint/type checks, full tests and
build through its Makefile. Use the repository guide's sequence and scope.
Offline fixture tests and live verification are distinct evidence. Live
mutations require the user's applicable authorization; an audit or skill
invocation grants no additional write permission. Disposable tests must guard
ownership, register cleanup before creation can succeed ambiguously, revoke only
their own key/records, and verify absence.

Re-read the evidence table at the pushed immutable head. Every required language
needs implementation AND behavior evidence, examples and docs. If a language was
not inspected or a path was not exercised, mark it unverified. A ticket
documents work; it does not satisfy parity. Do not claim complete while a
required row is missing, failing or deferred. Explicit user exclusions must
remain visible in the handoff.

Follow the repository's draft-PR and independent review workflow. Put the
evidence table and exact verification commands/head in the issue summary and PR.
Move completed tested work to Ready for Review; do not close issues without
current authorization. Report remaining gaps and their ticket links. Never
promise that a skill alone makes future drift impossible.
