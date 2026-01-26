make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 827ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '-cIttMLE919pUeHMl6b9Tc9eEB3K0wA-kB8P0p6yL0DJ7p01T7BaGFewa6tck1ZK8UIke5L1BZEbkS9kozJpzw'
}

=== Find by ID ===
Found: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  created_at: '2026-01-26T23:25:39.184Z',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: '-cIttMLE919pUeHMl6b9Tc9eEB3K0wA-kB8P0p6yL0DJ7p01T7BaGFewa6tck1ZK8UIke5L1BZEbkS9kozJpzw',
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T23:25:39.184Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  created_at: '2026-01-26T23:25:39.184Z',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: '-cIttMLE919pUeHMl6b9Tc9eEB3K0wA-kB8P0p6yL0DJ7p01T7BaGFewa6tck1ZK8UIke5L1BZEbkS9kozJpzw',
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  created_at: { type: 'DateTime', value: '2026-01-26T23:25:39.184Z' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  price: { type: 'Float', value: 99.99 },
  id: '-cIttMLE919pUeHMl6b9Tc9eEB3K0wA-kB8P0p6yL0DJ7p01T7BaGFewa6tck1ZK8UIke5L1BZEbkS9kozJpzw',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Updated Record' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: q_HCiqysoRdGo1MhOSbr6cnEk4siqz_aaiRN8b301SaGgA-VP41dZm5UlCJF2jWD_Jn8ZX2VaS10FGzp4f72JA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: nhWIaN_23P5bdKiQ0j6uLcMSeamn1XWOPgy5VFtmDh41GPF59jP9-QTXSsuKaUeevzKmLTd-goZ6asSJKZIKPQ

=== List Collections ===
Total collections: 20
Sample collections: ws_ttl_test,test_collection,chat_configurations__ek0_testing,schema_documents_client_go,schema_users_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { value: { name: 'Product 1', price: 29.99 } }
  cache:product:2: { value: { name: 'Product 2', price: 39.99 } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 10

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ISIUzgKLXWCWFpWr4n6XS00gAd6iF8a8EqQonZMc0bPnnDO95_XaZ0cnEE2DyanIj9gKGEh-9HfT2IfiJD771g
Created Bob: $500 - ID: VVpXxSC8qOQc24x1NO3A9O03R1tyEDmEv5fqUzZoqGJ5Pf6VuaRP-vrJM3Zk1SsaR9DdESFw6r0PWDgeAG7Wng

=== Example 1: Begin Transaction ===
Transaction ID: c84905e8-455a-4526-bfa0-025ee7d7934e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 806917a6-2e5a-45ad-b456-9360bc11998d
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
=== Query Builder Examples ===

Setting up test data...
✅ Test data created

1. Simple equality query:
Found 2 active users

2. Range query with sorting:
Found 3 users aged 18-65

3. String operations:
Found 2 users with @example.com emails

4. IN operator:
Found 2 privileged users

5. Complex query with multiple conditions:
Found 1 active US users over 21

6. Pagination:
Page 1: 2 users

7. NOT IN operator:
Found 3 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: bio, bio.value, title.value, title
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: bio.value, title.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: title.value, bio, title, bio.value
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: bio.value, title.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.768, Matched: 
  2. Score: 0.743, Matched: 
  3. Score: 0.735, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707, Matched: title.value, content.value, title, content
  2. Score: 1.497, Matched: content.value, title.value, content, title
  3. Score: 0.294, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio.value, bio, skills.value

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: [ 'age', 'email', 'name', 'status' ]
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders:
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 0LkbWcb-HQi2pTzlbMtvMNXNG9ooUi66BGErUkaQfx8SYTZQh3XqDyOL_jN2i15kn0CJsgKmTZ3IXXESqGUdAA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: nCnMhY6XH-Qtk_3KGnLisjKm_dlMqvur1kMvNTNHaWbCzAuhlm87i7QuKo4pD6pbGNdRhdjRFUyUDVX_lrFIgA

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: cjTvymrK4Qq-fAoRILgBz2NYAVBhSiQa3EMDFxeRfpo4LBQqq6SpA4IKskO2z8X5AByPED3ND3BjB2FhvWbpVw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: twdmrUEpeQOIDvNCxPz2rO4f9npzU4gdjWNdO_DWG5yUSKHg9NqETBeQKyi93v6HAqF3d5yy7caNgkiYf1myvQ

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {}
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (1.3x faster!)
Result: {
  "records": [
    {}
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: PYzCVjzBlGkjQ3627b5mgPv0z5genY_tFxCOXStPn-V9sA9XWuDrZW9xF7U0crRjbeLFV-VkNr1RbZM_vX7qsw
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: ZL-CYy66-qb8Een065nqpEVKHv4IGjvi3_OyZCZbEPyCVWUXlGQKH5taoEqvHj5CJG8PAekO6gE7rDhu2j-dEg
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 4dZAaz2ELD8x2ZBP9Qm64XN9MMBeYxC2_CdfrxaYrnntE_8_0jBkntyuOvQ6PE6NQmYp61dwqCvZMWVkFH9bKQ
📊 Statistics: 3 groups
   {"status":"inactive","count":5,"avg_score":50}
   {"status":"null","count":5,"avg_score":60}
   {"avg_score":60,"status":"active","count":5}

📝 Example 4: Script Management

📋 Total scripts: 41
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "id": 1,
    "addre...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "id": 1,
    "addre...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: HnI5ZPsVIN05jG0ivF4cFl_Q8-6eV7pvP3QCqegGL-R7p1k36cOLLkExKRweZHTnn-fvnWz0RPoAJyoqiwCo5A
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
   {"count":2,"avg_price":474,"category":"Furniture"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"category":"Electronics","count":3}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Script management (save, call, delete)
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: nUkWwdC9v9GsVOfODm94Uz_zOQtfwvZ-8yhtC5ht-FTK3JDOHe-_2bAoTNH6VbhRH5W-vvkR1dlQI82DX7jybA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: yhnHtRQiPl1yLI1MJft-UQ0AsUJD2p9bNlcAZORkDhcmmtgQ304HY10Gwl-O5OPilqHpcu3_5Jg2zsUbLcSuag
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: QOFfcVYHx-auQ7twZPbQHVVMoM_6ENMnih_Yl_jNzBSchcIW8144YvBS1tsqinmr6QEFcZGrZhKJelsw0keYTg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: ufzQaW3-nnJv39hwt2PMES3xvk2CxV57tUtigazLkc6HU4TmPLvz2qfYXUQwfSSbqqTjUCcDPhyzCCK5jhfmzA
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (uraAqvR2f0qd_AfN6i6SRuR2THa0mN2OQhA62ZUkCUJTk9qSKDcwPzpfWr5yrrCPe_TrS3qKqjG3g50W8qEBHg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {}
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
Result (cached): {
  "records": [
    {}
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (4qKkelZEziZSnQ0181BIxqBkMc6CyIqEDTiT7_hhgVcCtmMiga0SQz-U1xhsZbrMm72VTjKsib91Alo7neMZIg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {}
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

🚀 ekoDB TypeScript Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {"count":3,"category":"Furniture","avg_price":365.6666666666667}
   {"category":"Electronics","count":5,"avg_price":367}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer benefits such as high scalability, precise querying, and the ability to handle complex data structures. They also provide efficient storage and quick data retrieval due to their mathematical representation of data.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB TypeScript CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {"role":"user","avg_score":70,"count":7}
   {"avg_score":20,"role":"admin","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"AI","count":2}
   {"category":"Database","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: iROQ2JQ4r-HnxeynoBlz2KcWUCPcQWRtRj9k3KNE1uyiak-cJw_WhhGT4Q2EYUMf-Obas2TnD7CkUZCZzI8mcA

=== Sending Chat Message ===
Message ID: ATrjeedICxTYq4RvmJja5UXiPc3fEeHlBuBIuGRqW5N3r-Jrgj0rb0GfYSd-LgNq81CUHoLK_nMwemVu88APxA

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

3. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '2I1XX-yknuOcFfFOiqCBQMgHOQzn-x3cDRXQv4DOdi5K0e7eOJ-O9rbGcDzca33JVXWncOIGeWY_CO8SrEWGtQ',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'gPdkSViRCtVXStTMcku9Oa6VHXGuteTIyW3PFqqmedOzGEvEDfap8YG8VTttusHHdIEIVZZpp0-1P8x9u47TDQ',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: '85uqKW7S72ONCPslN32W4NOvaqK3TWV4GbTQH_OTm0cWOPJ4f9O1auSMtwlk9SQ6pORffuSKFPSegKYh2kYCrA',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2268ms

=== Token Usage ===
Prompt tokens: 446
Completion tokens: 78
Total tokens: 524

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: quVDSCQg1oVqo4_MQKXBJo56rwAcNl0MIW3YOZQYr8e8hRaEvfHnoZhZYgCDZXeVe74NkPVsvKbOjndyPe7TCg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is a high-performance database product named "ekoDB". It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: dKl084XOstr2VQgTKEYgeN8z-0xEMvJz0Nu1HCkrpzBbUfp6iTYdtIE79Z7GDnB6i99HlDZ5Qa8zegoGHyhtmQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 4

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 3

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: O9s_bNhs54ht7KpAhRJeLLCYKrHkl1tfsWMmIMGornTRtPrFV9uIQ35woUTAjqqFuPVgfpb7-pVhHjOXGhw9-A

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, there is a product available called ekoDB. It's a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Be4uMcJLefkFIwLaWGtR8ZfAuUOYv3h8C9OcfVnlkPJotPmBZjNzK2lsgTteH32R63C9a3fFJB6Q3-Jt_4f9pQ
  Parent: O9s_bNhs54ht7KpAhRJeLLCYKrHkl1tfsWMmIMGornTRtPrFV9uIQ35woUTAjqqFuPVgfpb7-pVhHjOXGhw9-A

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: Be4uMcJLefkFIwLaWGtR8ZfAuUOYv3h8C9OcfVnlkPJotPmBZjNzK2lsgTteH32R63C9a3fFJB6Q3-Jt_4f9pQ (Untitled)
  Session 2: O9s_bNhs54ht7KpAhRJeLLCYKrHkl1tfsWMmIMGornTRtPrFV9uIQ35woUTAjqqFuPVgfpb7-pVhHjOXGhw9-A (Untitled)
  Session 3: dKl084XOstr2VQgTKEYgeN8z-0xEMvJz0Nu1HCkrpzBbUfp6iTYdtIE79Z7GDnB6i99HlDZ5Qa8zegoGHyhtmQ (Untitled)
  Session 4: iROQ2JQ4r-HnxeynoBlz2KcWUCPcQWRtRj9k3KNE1uyiak-cJw_WhhGT4Q2EYUMf-Obas2TnD7CkUZCZzI8mcA (Untitled)
  Session 5: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w (Untitled)
  Session 6: LfxcAjSnqvQbHtrXI6GQFVFJ4iItWlvXOyC_AE-bc4lY_Jq9FZVUs9XeBwmZxf6v5e8mRkJiFUlwD8aaU50iqA (Untitled)
  Session 7: 5SXXYkac4oI8ZD6t96ysIzlR4gmK3V7-R1Iwtf2MJGfi98pnTNB6zu2F6x0ZDv_w28kkWW9OAnBPVRKv9l6Mqg (Untitled)
  Session 8: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg (Untitled)
  Session 9: MjMzthfgQkK_RCZ5U_aYC-J-lAwwz_GwyqBHLHRntDFGFW8elnqNQCAvkbPZ32JJA4sn7fvaUDNmK7HDcAzdAw (Untitled)
  Session 10: 4-V04W3imEKGCRZjDetm--x5I8nlgyhbglc0O0ne_DpXqOAoTm4N6TjYOBYebjqZSDug4o_S4hpEvTDgsa8NVg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: Be4uMcJLefkFIwLaWGtR8ZfAuUOYv3h8C9OcfVnlkPJotPmBZjNzK2lsgTteH32R63C9a3fFJB6Q3-Jt_4f9pQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'oecDCkJXm1QuPBWO3-CuoJJK_zP9SFYb1HQxUx3dee-HOk5ZhkpbMjEK4o5rCqah6a-LY6Rb0XDWiJHQe05Lfw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'oecDCkJXm1QuPBWO3-CuoJJK_zP9SFYb1HQxUx3dee-HOk5ZhkpbMjEK4o5rCqah6a-LY6Rb0XDWiJHQe05Lfw',
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'qkKIrB08GXCQhqV2wealOXpDXfa1AsjnVYoNTBobfo5aRp1mZqXBEEGoFC28Bzi0TyVfoJaVXibMnvSoBP-t6A'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'oecDCkJXm1QuPBWO3-CuoJJK_zP9SFYb1HQxUx3dee-HOk5ZhkpbMjEK4o5rCqah6a-LY6Rb0XDWiJHQe05Lfw'
}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"m96a7ULP7WJPTO2hoK1n_fj40EmhCWtK4SLYwPHnpe2aCoojYcYBKX_9pNlgzUfC1wQS6xYtgJVCOcGoz-nQGw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"yHHYR4PCN7BoMVR-0EeNLj5PejX6B33OYp_ySarYsNKbzvh9L1P0bxkTqLONjnZ6ug4tjAAVhPDOU51TmaZIwA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"m96a7ULP7WJPTO2hoK1n_fj40EmhCWtK4SLYwPHnpe2aCoojYcYBKX_9pNlgzUfC1wQS6xYtgJVCOcGoz-nQGw","name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"Gua6eNAR9Q4kcM-kf3H2xxWLfC-IKObW1LCSw73ErQRZdCSiH5ZdUqN0Ud2eADcGvi_5Iqh4zNmulxhcXxUoOQ"}

✅ All bypass_ripple operations completed successfully!
✓ Client created

Setting up test data...
✓ Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: [object Object]

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

🧹 Cleaning up test data...
✅ Cleanup complete

✅ All projection examples completed successfully!
✅ [32mTypeScript client examples complete![0m
