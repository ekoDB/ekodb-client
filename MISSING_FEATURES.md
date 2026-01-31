# Feature Parity Comparison Matrix (COMPLETED)

All features below are now implemented across all client libraries (January
2026).

## Chat Models API

- `getChatModels()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ | JavaScript
  ✅ | Kotlin ✅
- `getChatModel(name)` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ |
  JavaScript ✅ | Kotlin ✅
- `getChatMessage(chatId, msgId)` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅
  | JavaScript ✅ | Kotlin ✅

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

## Collection Operations

- `collectionExists()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ |
  JavaScript ✅ | Kotlin ✅
- `countDocuments()` - Rust ✅ | Go ✅ | Python ✅ | TypeScript ✅ | JavaScript
  ✅ | Kotlin ✅

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

## Beforehand Findings (disregard for now)

## Missing Client Features

This document tracks ekoDB server features that are available via API but not
yet exposed in client libraries.

**Last Updated:** January 23, 2026

---

## 1. Query Index Management ❌

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

### Missing Client Methods

**All Languages Need:**

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

## 2. Search Index Management ❌

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

### Missing Client Methods

**All Languages Need:**

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

## 3. KV Document Linking ❌

### Description

Document linking allows KV store entries to reference collection documents,
creating relationships between KV cache data and persistent documents.

### Server Endpoints

```
GET  /api/kv/links/{key}                        - Get all linked documents for key
POST /api/kv/link                               - Create link between key and document
POST /api/kv/unlink                             - Remove link between key and document
```

### Missing Client Methods

**All Languages Need:**

```typescript
// TypeScript/JavaScript
kvGetLinks(key: string): Promise<DocumentLink[]>
kvLink(key: string, collection: string, documentId: string): Promise<void>
kvUnlink(key: string, collection: string, documentId: string): Promise<void>
```

```go
// Go
KVGetLinks(key string) ([]DocumentLink, error)
KVLink(key, collection, documentId string) error
KVUnlink(key, collection, documentId string) error
```

```rust
// Rust
pub async fn kv_get_links(&self, key: &str) -> Result<Vec<DocumentLink>>
pub async fn kv_link(&self, key: &str, collection: &str, document_id: &str) -> Result<()>
pub async fn kv_unlink(&self, key: &str, collection: &str, document_id: &str) -> Result<()>
```

```python
# Python
def kv_get_links(self, key: str) -> List[DocumentLink]
def kv_link(self, key: str, collection: str, document_id: str) -> None
def kv_unlink(self, key: str, collection: str, document_id: str) -> None
```

```kotlin
// Kotlin
suspend fun kvGetLinks(key: String): List<DocumentLink>
suspend fun kvLink(key: String, collection: String, documentId: String)
suspend fun kvUnlink(key: String, collection: String, documentId: String)
```

### Use Cases

- **Cache Relationships:** Link cached data to source documents
- **Invalidation:** Track which documents are referenced by cache keys
- **Consistency:** Maintain relationships between KV store and collections

### Implementation Priority

🟡 **MEDIUM** - Useful for advanced caching patterns

---

## 4. Schedule Management (Cron Jobs) ❌

### Description

Schedules allow running Functions or Scripts on a cron schedule. Full CRUD
operations available on server.

### Server Endpoints

```
POST   /api/schedules                           - Create schedule
GET    /api/schedules                           - List all schedules
GET    /api/schedules/{id}                      - Get schedule details
PUT    /api/schedules/{id}                      - Update schedule
DELETE /api/schedules/{id}                      - Delete schedule
POST   /api/schedules/{id}/pause                - Pause schedule
POST   /api/schedules/{id}/resume               - Resume schedule
```

### Missing Client Methods

**All Languages Need:**

- `createSchedule(config)` - Create cron job
- `listSchedules()` - List all schedules
- `getSchedule(id)` - Get schedule details
- `updateSchedule(id, config)` - Update schedule
- `deleteSchedule(id)` - Delete schedule
- `pauseSchedule(id)` - Pause execution
- `resumeSchedule(id)` - Resume execution

### Use Cases

- **Automated Tasks:** Run cleanup, aggregations, reports on schedule
- **Data Processing:** Periodic ETL jobs, batch processing
- **Maintenance:** Scheduled backups, cache warming

### Implementation Priority

🟡 **MEDIUM** - Important for automation workflows

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

## Implementation Roadmap

### Phase 1: Query & Search Index Management

**Target:** Q1 2026

- Implement query index methods (create, list, delete, explain)
- Implement search index methods (create, explain variants)
- Add comprehensive tests for all index operations
- Update examples showing index usage

### Phase 2: KV Document Linking

**Target:** Q2 2026

- Implement KV link methods (get, link, unlink)
- Add tests for link operations
- Create examples demonstrating cache-to-document relationships

### Phase 3: Schedules & Advanced Features

**Target:** Q2-Q3 2026

- Implement schedule CRUD methods
- Add schema constraint methods
- Add WAL operation methods (admin-focused)
- Comprehensive testing and examples

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
  cron: string;
  functionName?: string;
  scriptName?: string;
  paused: boolean;
  nextRun: string;
  lastRun?: string;
}

interface ScheduleConfig {
  name: string;
  cron: string;
  functionName?: string;
  scriptName?: string;
  paused?: boolean;
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

1. **Check server implementation** - Review `/ekodb/ekodb_server/src/main.rs`
   for endpoint details
2. **Follow existing patterns** - Match style of current client methods
3. **Add types first** - Define all TypeScript/Rust/etc types before
   implementation
4. **Test thoroughly** - Unit tests + integration tests + examples
5. **Update docs** - CLIENT_LIBRARY_GAPS.md and language-specific READMEs

---

## Questions?

- **Server API Documentation:** See `/ekodb/API.md`
- **Current Client Status:** See `/documentation/CLIENT_LIBRARY_GAPS.md`
- **Implementation Examples:** Review existing methods in client source files
