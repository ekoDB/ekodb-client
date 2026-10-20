# Agent guide

This repository contains Rust, Python, TypeScript and Kotlin SDKs. Go lives in
the public `ekoDB/ekodb-client-go` repository. Python wraps Rust through PyO3: a
Rust API alone does not expose a Python API.

## Required SDK parity audit

For every added or changed API, request/response field, retry/error behavior,
dependency or example, inspect all five SDKs before implementation and before
reporting completion. Read and follow the canonical
[sdk-parity skill](.agents/skills/sdk-parity/SKILL.md); Codex, Claude and
Copilot all use that one source. Compare the actual exports, bindings and
transport paths, not just method names or test counts. Follow
[PARITY_MATRIX.md](PARITY_MATRIX.md#contributing).

Record per-language implementation, behavior-test, example and documentation
evidence in the tracking issue and draft PR. Verify suspected gaps in source,
search existing tickets, then ticket them on the affected repository board and
take them In Progress. Missing or unassessed SDKs block a full-parity completion
claim; an explicit user scope restriction remains a visible exclusion. Running
all language suites cannot substitute for implementing the feature in those
languages.

Preserve wire distinctions such as omission versus null versus empty arrays.
Keep admin operations separate from data CRUD and use the verified public
contract. Update each affected language's examples, dependencies and README,
plus the changelog and parity matrix. Never put private repository names,
tickets or paths in public code, docs or GitHub artifacts.

## Verification

Run from the checkout holding the branch: `make lint-fix && make fmt`, inspect
every changed file, then `make lint && make test && make build`. These targets
cover all four languages. Use `make example-parity-check` and
`make check-client-examples` for matching example scenarios and compilation.
Live writes require applicable explicit authorization; an audit grants none. Do
not bump versions or publish unless authorized.
