# Kotlin search quality/parity audit

**IMPLEMENTED ON CURRENT MAIN**, in a local branch based on main. These additions
are not claimed to be available in any published package.

## Provenance and boundaries

- Fresh standalone clone: `/tmp/ekodb-parity-9YrmB8/ekodb-client`.
- Clone source and unchanged origin: `https://github.com/ekoDB/ekodb-client.git`.
- Original branch: `main`; clean before changes; tags fetched.
- `EKODB_BASELINE=4a656993ced11d8f46a3c47c2d8391bbacd43d9d`.
- Local branch: `kotlin-client-parity-audit`. No push, PR, remote changes, or force
  operations. No intent-match checkout was opened or used as source/build input.
- Latest GitHub release checked: [v0.26.0](https://github.com/ekoDB/ekodb-client/releases/tag/v0.26.0),
  September 4, 2026; release commit `864935bdf58ed5c44c78f7b529c5fc0a23849518`.
- Main package versions: Rust `ekodb_client` **0.26.1**, TypeScript
  `@ekodb/ekodb-client` **0.26.1**, Python Cargo/pyproject **0.26.1**, Kotlin
  Gradle **0.26.1**. No version changes in this implementation.
- Go is a separate repository; this repository's examples use a sibling-path
  replacement and a placeholder pseudo-version, so they do not establish Go's
  current main version. Go's [latest GitHub release](https://github.com/ekoDB/ekodb-client-go/releases/tag/v0.26.0)
  was independently checked as **v0.26.0**. No external Go checkout was used.
- Registry publication of main's 0.26.1 was not verified. GitHub release status
  and a source version do not prove Maven Central/npm/crates.io availability.
- No public/hosted or local ekoDB server was contacted. Evidence is client
  source, exact emitted wire shapes, mock transport tests, and compilation.

## Repository policy and reference hierarchy

README claims complete feature parity. PARITY_MATRIX describes core capability
parity verified through 0.21.0, not identical type systems; its later historical
"missing" sections are explicitly historical. TODO reports CRUD-over-WebSocket
parity and deferred blob/import work. COMMANDS plus Makefile establish Kotlin
`ktlintCheck`, `test`, and `build`. No standalone Kotlin parity command was found
in the Makefile, scripts, workflow, or module. Live example targets are not a
unit parity gate and were not run.

Rust is the canonical model reference: `ekodb_client/src/search.rs`, the public
methods in `src/client.rs`, and actual `.json(&search_query)`/JSON response path
in `src/http.rs`. TypeScript `src/search.ts` and `src/client.ts` provide the
second reference, with `src/client.test.ts` and TypeScript/JavaScript
`client_search` examples showing real public construction. Python
`ekodb-client-py/src/lib.rs` maps every search keyword argument into Rust's
SearchQuery and serializes Rust's response back into a dictionary.

Relevant changelog evidence includes metadata prefilters forwarded by Kotlin's
raw search, 0.15.2 score injection, 0.26.0 additive typed chat models with unknown
key tolerance, and 0.26.1 chat-field wire-name correction. Source was checked
rather than treating stale documentation as the protocol.

## Complete HTTP search inventory

Before this pass, every listed request field was **WIRE CAPABLE** through
`search(collection, JsonObject)`. None was an established missing server
capability. The richer surface was **TYPED SURFACE INCOMPLETE**.

| Wire field / result | Kotlin before | Kotlin after |
| --- | --- | --- |
| `query` | String argument in text/hybrid helpers; raw general search | String in SearchQuery and builder/DSL |
| `language` | Raw only | Nullable String |
| `case_sensitive` | Raw only | Nullable Boolean |
| `fuzzy` | Raw only | Nullable Boolean |
| `min_score` | Raw only | Nullable Double |
| `fields` | Raw only for HTTP search | Nullable canonical String; builder also accepts List of String |
| `weights` | Raw only | Nullable canonical String; builder also accepts Map of String to Double |
| `enable_stemming` | Raw only | Nullable Boolean |
| `boost_exact` | Raw only | Nullable Boolean |
| `max_edit_distance` | Raw only | Nullable Int |
| `bypass_ripple`, `bypass_cache` | Raw only | Nullable Boolean each |
| `limit` | Int in legacy helpers; raw general search | Nullable Int in complete request |
| `vector` | List of Double in hybrid helper; raw pure-vector search | Nullable List of Double, including pure-vector search |
| `vector_field` | Raw only | Nullable String |
| `vector_metric` | Raw only | Nullable DistanceMetric: cosine/euclidean/dotproduct |
| `vector_k` | Raw only | Nullable Int |
| `vector_threshold` | Raw only | Nullable Double |
| `text_weight`, `vector_weight` | Raw only | Nullable Double each |
| `select_fields`, `exclude_fields` | Raw only | Nullable List of String each |
| `filters` | Raw search could forward existing QueryBuilder expression | Same canonical Query.filter JsonElement; builder `filters { ... }` |
| Result record | Raw JSON; helper returns records | First-class SearchResult.record, dynamic JsonObject |
| Score | Raw JSON or helper-injected `_score` | Required Double score, legacy `_score` unchanged |
| Matched fields | Raw only; dropped by legacy helper projection | Required List of String matchedFields |
| Total | Raw only; dropped by legacy helpers | Required Long total |
| Execution time | Raw only; dropped by legacy helpers | Nullable Long executionTimeMs |
| SearchQuery / SearchQueryBuilder | Absent | Serializable data class / fluent builder and DSL |
| SearchResult / SearchResponse | Absent | Serializable data classes |

No separate HTTP vector convenience helper is needed: `search(collection) {
vector(...); ... }` and `SearchQuery(vector = ...)` cover it without a parallel
options type. Existing text/hybrid helpers remain unchanged. A complete typed
query is the main interface for richer options, including custom hybrid weights.

## Exact wire behavior and sibling differences

Rust's SearchQuery serializes these exact keys (all snake_case):

```text
query, language, case_sensitive, fuzzy, min_score, fields, weights,
enable_stemming, boost_exact, max_edit_distance, bypass_ripple, bypass_cache,
limit, vector, vector_field, vector_metric, vector_k, vector_threshold,
text_weight, vector_weight, select_fields, exclude_fields, filters
```

`query` is always a string. Rust's Default gives it `""`; deserializing Rust
SearchQuery still requires the key. All other fields are Option values. All
unset options are omitted EXCEPT `bypass_ripple`, `bypass_cache`, and `limit`,
which Rust serializes as null. TypeScript and new Kotlin omit all unset options.
Kotlin always emits `query`, including `""`, via EncodeDefault. The tests do not
assert server equivalence of omission and null.

Rust stores `fields`/`weights` as strings, with deserialization accepting arrays
and objects and normalizing them. TypeScript's interface additionally accepts
arrays/objects directly; its builder normalizes to comma-separated strings.
Kotlin models the canonical string form and gives callers List/Map builder
methods. Alternate arrays/objects remain possible through raw search; this is
an alternate representation, not an inaccessible search capability. Kotlin does
not promise to decode every alternate TypeScript request representation into its
request data class. Integral builder weights become `2`, not `2.0`.

Filters are the same expression as ordinary find, e.g.:

```json
{"type":"Condition","content":{"field":"category","operator":"Eq","value":"ml"}}
```

They are placed under search's plural `filters`, not find's singular `filter`,
and not wrapped in a whole Query. Kotlin has no canonical typed QueryExpression
hierarchy: Query.filter is JsonElement, and QueryBuilder produces that shape.
Reusing it avoids incompatible duplicate filters. Common filters require no
handwritten JSON via the DSL; the underlying existing filter model is not a
fully statically checked AST. Tests compare simple and nested logical filters
with the actual find builder.

All clients allow a named vector field; omission leaves selection to the server
(sibling docs describe `embedding`). Explicit metric/k/threshold and independent
text/vector weights are forwarded. Rust's hybrid helper explicitly sets 0.5/0.5;
Python delegates to it. TypeScript and Kotlin hybrid helpers omit both weights.
Their omission is retained; no conclusion about server default weights follows.
Rust/Python/Kotlin text helpers return record lists with `_score`; TypeScript's
text helper accepts Partial<SearchQuery> and returns SearchResponse. Hybrid
helpers inject `_score`. New Kotlin general search retains complete metadata.

Rust SearchResult requires a dynamic record, f64 score, and Vec of matched field
names. SearchResponse requires results, usize total, and u64 execution time.
TypeScript makes execution time optional. Kotlin uses JsonObject records,
Double scores, Long total, and nullable Long timing: integer timing follows Rust
while missing timing follows TypeScript. Unknown document fields survive. Typed
envelopes ignore unknown keys consistent with the module's Json configuration;
raw search preserves arbitrary envelope/hit metadata. No new protocol fields
or server score/ranking guarantees were invented.

Rust and TypeScript force JSON for search. Kotlin previously chose a format
header from the client configuration, leaving search exposed to its experimental
MessagePack path even though the body/decoder were JSON-oriented. Search now
explicitly requests JSON in either mode and retains the same authentication,
path encoding, and retry machinery. Mock tests assert the actual HTTP method,
encoded collection path, authentication, Accept, Content-Type, and body.

## Bounded additional surface review

| Area | Finding and disposition |
| --- | --- |
| Vector indexes | Kotlin already sends `type=vector`, algorithm, metric, m, ef_construction; defaults flat/cosine/16/200 match Rust/TS. Flat/HNSW available; IVF is listed but Rust explicitly calls it future. No schema changes or index behavior claims. Kotlin strings remain less constrained than sibling schema enums. |
| Transactions | All three use string isolation levels. Kotlin returns raw status while TS names state/operations_count; request capability exists. Leave metadata modeling to a separate change. |
| Find/query | Typed Query and fluent canonical filter builder already exist. Raw JsonElement filter/sort/join and builder Any inputs are existing limitations; no filter-system rewrite. |
| WebSocket search | Kotlin/Rust/TS expose TextSearch with query, optional fields, limit; no complete HTTP SearchQuery over WS is established. Kotlin returns raw JsonObject, Rust/TS expose dynamic lists. Leave frame/result redesign separate. |
| SSE | Existing Flow/typed stream-event surfaces, including provider errors; no evidenced search gap requiring a rewrite. |
| Functions | Serializable stage hierarchy includes typed VectorSearch, TextSearch, HybridSearch and other stages. Stage contracts differ from HTTP SearchQuery and must not be conflated. No speculative fields added. |
| Chat | Numerous models already exist but create/session/message methods still accept/return JsonObject; typed transport integration remains a separate ergonomic task. One exact serialization bug fixed: FieldSearchOptions.field now emits field_name, matching Rust src/chat.rs and the existing changelog's server requirement. Kotlin source property is unchanged. |
| Embeddings | Kotlin embed/embedBatch return typed numeric vectors like Rust/TS; EmbedRequest/EmbedResponse types also exist. Public convenience helpers omit response model/dimensions metadata, broadly like siblings. No expansion. |
| Health/rate limits | Kotlin already has HealthStatus/HealthState and RateLimitInfo; no high-confidence missing typed capability found. |

Admin query/search-index explanation endpoints remain intentionally excluded by
repository policy. They are not Kotlin defects. No server capability limitation
was demonstrated by this audit. Ranking normalization, threshold direction by
metric, omitted defaults, ANN recall, index selection, and execution semantics
remain server questions; client parity tests cannot certify them.

## Documentation discrepancies

Confirmed against both baseline main and the v0.26.0 tag's Kotlin source/docs:

- QUICKSTART showed `search(collection, String)` and a search builder lambda,
  but only the raw JsonObject overload existed. The new overload/DSL makes those
  shapes valid on this local implementation; updated docs explicitly mark the
  release boundary.
- README used `results.size` on the raw response object to count articles. It
  counted envelope keys. Replaced with typed `total` and `results`, and raw total
  access in the escape-hatch example.
- README's HTTP search description mentioned only full text despite raw
  vector/hybrid capability. Updated it to distinguish wire and typed APIs.
- Existing QUICKSTART also contains unrelated APIs such as `client.query`,
  `deleteWhere`, and builder `offset` that are not present in the inspected
  Kotlin surface, plus Map-based CRUD examples where current methods use
  Record. These pre-existing non-search discrepancies are recorded, not fixed
  as part of this bounded search pass.

README, QUICKSTART, ClientSearch.kt, PARITY_MATRIX, and changelog now describe the
new surface. Examples demonstrate typed text/vector search, metadata-filtered
vectors, custom hybrid weights, result metadata, and the raw escape hatch.
Toy vectors are labeled illustrative, without claims about semantic quality or
exact/approximate indexing. All configured Kotlin examples compile; none ran.

## Validation and self-review

Baseline Kotlin: **420 tests passed** before edits. Initial default Gradle runs
failed first on JDK 17 auto-provisioning, then an incorrect Nix JDK root/shared
daemon diagnostic. The successful baseline and subsequent runs used a separate
Gradle home, no reusable daemon/filesystem watching, in-process Kotlin compiler,
and the actual JDK 17 `lib/openjdk` home. No other project's checkout/build
outputs were used. Rust's existing **19 search unit tests passed** before edits.

Final validation:

- Kotlin `ktlintFormat`, `ktlintCheck`, `test`, `build`: passed.
- Kotlin **437 tests**, no failures/skips: **16 new search tests** covering all
  20 requested categories in individual/combined/looped cases, plus **1 new
  nested chat-field wire-name test**. Includes omitted options and retained false values,
  all three metrics and invalid metric rejection, complete manual JSON
  equivalence, canonical filters, scores/matched fields/timing, raw/legacy
  compatibility, actual headers/body, 401 retry, and HTTP failure exposure.
- TypeScript `npm run lint`: passed. Relevant client and new fixture tests:
  **243 passed**, including five new wire/default tests.
- Rust new fixture tests: **2 passed**, exercising all four cases and the null
  distinction. No Rust/TS implementation code changed.
- Kotlin `examples/kotlin` `compileKotlin`: passed. Existing warnings remain;
  examples were not executed against a service.
- Four shared golden requests are constructed independently by each language;
  comparisons keep all keys and values visible, not snapshots with erased
  differences. This establishes those request shapes, not all-client parity.
- `git diff --check`: passed. Final diff manually reviewed for wire names,
  defaults, compatibility, field completeness, scope, and documentation.

Exact maintainer commands from the repository root:

```sh
make lint-kotlin
make test-kotlin
make build-kotlin-client
(cd ekodb-client-kt && ./gradlew test --tests io.ekodb.client.SearchTest --tests io.ekodb.client.FieldSearchOptionsTest)
cargo test -p ekodb_client search --lib
cargo test -p ekodb_client --test search_wire
(cd ekodb-client-ts && npm ci && npm run lint && npm test -- src/search-wire.test.ts src/client.test.ts)
(cd examples/kotlin && ./gradlew compileKotlin)
git diff --check
```

The equivalent Kotlin Gradle task invocation actually used here (replace the
machine-specific JDK/Gradle-home paths on another machine) was:

```sh
./ekodb-client-kt/gradlew -p ekodb-client-kt \
  -g /tmp/ekodb-parity-9YrmB8/gradle-home --no-daemon --no-watch-fs \
  -Pkotlin.compiler.execution.strategy=in-process \
  -Porg.gradle.java.installations.paths=/nix/store/90jn2n91br5x3fx9h9d311y46r8v23w2-openjdk-headless-17.0.18+8/lib/openjdk \
  ktlintFormat ktlintCheck test build --console=plain
```

Rust commands ran inside `nix-shell -p cargo rustc` because Cargo was not on the
original PATH. Logs are outside the clone in `/tmp/ekodb-parity-9YrmB8/`.

## Recommendation and compatibility

1. **Is Kotlin materially behind Rust/TS for HTTP search quality now? No** for
   the material text/vector/hybrid request and result surface audited. It can
   express every modeled field without hand-building a request, while reusing
   the canonical filter builder. This is not live-server certification or a
   claim of identical language APIs.
2. Remaining binding-quality work includes unrelated raw chat/transaction/WS
   responses, existing generic filters, and schema strings. Admin exclusions
   are intentional. No remaining search gap was shown to be a server
   limitation. Unclear ranking/default/index behavior remains server semantics,
   not a reason to omit client fields.
3. **Recommend an upstream PR: yes.** The change is additive, narrowly scoped,
   and backed by sibling wire fixtures and mock transport tests.

Proposed title: **feat(kotlin): add typed search API with cross-client wire tests**

Proposed description: Kotlin can forward full search requests but requires
manual JSON for most options and response metadata. Add serializable search
models, a fluent builder/DSL reusing existing filters, and typed search overloads
covering text/vector/hybrid options and ranked results. Preserve raw search and
legacy helper behavior; force JSON search negotiation to match Rust/TypeScript.
Also align Kotlin chat retrieval's field_name serialization with Rust. Validate
with Kotlin lint/build/tests, compiled examples, and four shared wire fixtures
constructed independently in Rust/TypeScript/Kotlin.

Migration: no required source changes for existing search callers. Existing raw
search and text/hybrid helpers retain signatures, results, and defaults. New
callers may use SearchQuery or the DSL; no separate vector/hybrid options types.
Search under experimental MessagePack intentionally negotiates JSON now.
FieldSearchOptions keeps the source property `field` but corrects its serialized
key; locally persisted old `field` JSON needs conversion to `field_name`.
New server fields or alternate request representations can still use raw search;
no known material HTTP search option requires that escape hatch. Unknown result
envelope metadata still requires raw access by design. Legacy helper defaults
and Rust's null serialization remain the documented meaningful differences.
