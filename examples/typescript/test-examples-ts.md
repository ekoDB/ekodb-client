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

added 1 package, removed 1 package, and audited 13 packages in 832ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'NjAf6nqkHZWq5JtfrPGqZlDILtJ1Gk4g8WR-g-x4OiU3k51omF1tGf68PwP_0_X_lk93nA25BFPJrxAFjlmcUA'
}

=== Find by ID ===
Found: {
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  id: 'NjAf6nqkHZWq5JtfrPGqZlDILtJ1Gk4g8WR-g-x4OiU3k51omF1tGf68PwP_0_X_lk93nA25BFPJrxAFjlmcUA',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  created_at: '2026-01-26T08:01:56.487Z',
  name: 'Test Record',
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T08:01:56.487Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  id: 'NjAf6nqkHZWq5JtfrPGqZlDILtJ1Gk4g8WR-g-x4OiU3k51omF1tGf68PwP_0_X_lk93nA25BFPJrxAFjlmcUA',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  created_at: '2026-01-26T08:01:56.487Z',
  name: 'Test Record',
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 100 },
  price: { type: 'Float', value: 99.99 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: 'NjAf6nqkHZWq5JtfrPGqZlDILtJ1Gk4g8WR-g-x4OiU3k51omF1tGf68PwP_0_X_lk93nA25BFPJrxAFjlmcUA',
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-01-26T08:01:56.487Z' },
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: UdDi7uTNip0KsylYfoMECzf_lE5uh2gL-WzAerJGJYb1KnljUbTaIA_86TsrEPz2LeQf7-TYJ20pXEDvQaL2Sw

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
Collection created with first record: rp49mJuvGvFf3VwB7EJ8qM16LX2Bk8gwIm-22p0wLzkBTkKTkehto4W4pfpK-aM7RVsQf50teO5wmAHgpOQxXg

=== List Collections ===
Total collections: 21
Sample collections: users,schema_users_client_go,ttl_cache,test_collection,schema_documents_client_go

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
Retrieved value: { username: 'john_doe', userId: 123 }

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { value: { price: 29.99, name: 'Product 1' } }
  cache:product:2: { value: { price: 39.99, name: 'Product 2' } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

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
Created Alice: $1000 - ID: bQouoLAnlzFV6_pfXA-9c8mhgxOHIlzajECFFUpeHPjR3fDEQN7PsrGX6DeUu7BeZUa1Zj_bDD49smb-qf6AEA
Created Bob: $500 - ID: vwnpg2ku8zgTj9mkb0TITJSbl0SAcskov6swGGi2sbnBiK8pJaDxXOofT2iNE8Ck00yE0nd46fmJv8h15BPrJA

=== Example 1: Begin Transaction ===
Transaction ID: 02e7f5f9-90ec-4478-bb07-19f907f10c34

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f020a502-8c45-43fc-9056-8aa12855f426
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
  1. Score: 25.740, Matched: email.value, email, name.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, bio, title.value, title
  3. Score: 26.400, Matched: title, title.value, bio.value, bio
  4. Score: 26.400, Matched: bio.value, title, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title.value, bio.value, title, bio
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.751, Matched: 
  2. Score: 0.737, Matched: 
  3. Score: 0.722, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700, Matched: content.value, title, title.value, content
  2. Score: 1.495, Matched: content, title.value, title, content.value
  3. Score: 0.289, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio, skills.value, bio.value

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
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
✓ Inserted document: NQbzVy4Fdp4-y10ynl2qLiSFfh1c1RjGen8UBT-hAsRu0cL73Hd7Gg1OLNNYp_Z46QN3cv8mJhGrqSVZ5BXY0Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: NPKmBoTtxJDC5ZI_cMeO63Za7DLhP0qKX3b6Kqvv2AUDwTjVSrvb_JqGPSllInE8XZLeqcYuFoMOvcMrl7uKUA

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
✓ Inserted document with TTL: Fo-jKeDiux2ZqASkeKwJ9EuGHOgR3WHyRgGpRdUOJTDPCzzS2f3MPQgZZGPOv3Mfo3OHb10WdIq1EK3hyB-CnA

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
✓ Edge cache script created: IPIwEohoDboPQAZb3-uLwSq5s8XzE8aa5MH9AcjRmOX3T0sFi9Jafnar3F8Ic-_NY--n-5oSAgqNMDQe-omaoA

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 4ms (1x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: 1513PcOj3DXurFYK-9arPYLDAVdl26HMFDsiO9grKIgWqIrbrjF5VJYMtDOzUTwLkgbGFrqeFj3hJ8bLM8h1PA
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: 3XCPSw5x0gJ039D4Q8T8onM5G4-6wgupzfQNSSr7UTCflmmlaPW2NrX3bJm1j5kMfmoSJapYCrT0KvgfyQPZ1Q
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 3dvL3iK7a6828TuC7M2EmCcqwaQcPIlLpOl01Ud9sgo3uFqXad8aTHHQf2HfEZ4OuEZj7Xg5YE8zS5qQ15H63g
📊 Statistics: 3 groups
   {"avg_score":50,"status":"inactive","count":5}
   {"count":5,"status":"active","avg_score":60}
   {"status":"null","count":5,"avg_score":60}

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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 74ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 68ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

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

✅ Script saved: 6IWlwo-yxeTrGZk1HBOqD9tQIF4sxDJuz1pYx8BnFvaXZsO6S6FNKX5c95Fds-N34HOIgSGADDOA82wasLivtA
📊 Found 2 product groups
   {"avg_price":474,"category":"Furniture","count":2}
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: GFssfnQKkXlVXt4cvLN-FFsxEwrgeErqKqKMWTzX-VLJeznHVHw8fl5hUtitQq0o3-020Hf0JDOK6OqW9Urc6g
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: LWL2kBu2i0sVEkYbUNkEO1xY79u2bB2IR3UB40h_Uii0MZEbqiGTVuDgkDqkDRcSI5gFtSRPGJYhx4klQJNGKw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 90E72IbhS3j7A37BBB5GT_WuvTcTs-lGGZC7gntRyP1DJoJ3BpnUZ9uJ_F1FrJRaQ6OiDM7K37Hw4gTJW4jcrA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: sD88Ehw3LExCBsjmHPMg0p1WQ8S1mxJ6J4zNSrd1nxlcJWK_YfIgD-kKaBQEtUV8eTsYQVpFNNLromNX4qBUJA
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
✓ Created SWR script: fetch_api_user (nl0CvaPGGkdAd-ZujyoXx5fNmPj03lkpbbFcEWYyf90c70_It1pNawe9DS3YC1HXWUzVpyvPNYM59qcJRoZ_OA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (SK3yj6eVi02tbrtEaf88VYPo2ICeq0reQ_IUMbXcVSNEP32ElNEGMMUM7KusYLO7mtv2j4a2NEq6Pynylm2_mA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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
   {"count":5,"avg_price":367,"category":"Electronics"}
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
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
   Vector databases offer several benefits including high scalability, precise data representation, easy data manipulation such as scaling and rotating, and they require less storage space as they store data in mathematical formulas. They also support complex geographic operations like adjacency, connectivity, and proximity analysis.
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {"count":3,"role":"admin","avg_score":20}
   {"role":"user","avg_score":70,"count":7}
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
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":2,"category":"AI"}
   {"category":"Database","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: doaPGJAF7e3z4i0nEIk9aJm3ASjNJ51v1ObUWXku0_s4SqDqnEBotSJ12_4wIanyzkSEcPSqTXEm0jVd_xM1Uw

=== Sending Chat Message ===
Message ID: 9TwGV4VKsCHsR13Yf_griLlFiBY5NrYC4sXaDicgO79XtzflNUVRGq6qeStktIiTrND3Cs0ZeHfgPE3dw3ARxg

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'EayEgUv6MNsLFSDa_1VV6yHHgKBamQApj0wNuEn9C1u1GExaOii54-vpY4R8SuGQTeufxi37w1YyJ9Umyh_gmw',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'f7XBt6cydpFmLOai7WbfBrMC36hyno9w789GZano4tC172Jp7f04M1knEn9Ut_bx_UpWvx_HmuMIPHrXpuNKgA',
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '5zJasQZD8aOF-3_nXYJClA5bCSpy54_E6mcwiw1TPgDFl1cvG0ECsH0mubKO7jFI9-naRCehRuYiccF2eNbvLg',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2902ms

=== Token Usage ===
Prompt tokens: 446
Completion tokens: 84
Total tokens: 530

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: VTmAXQEgx1yETJler8SaHOomFHLfjYps11sisUIUdNOy25U0G-X2HmQwPUDMmTfXmvgsMhAGtMW_awXv27016Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product that is available is "ekoDB". It is a high-performance database product which is priced at $99.

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
✓ Created second session: fwouIl2cr-G6N6vNT0z14MolEjcyX2FSbskE7sSUhtpFrghvIfYLMZYeoj-7-HKa8zhavOf0bf0XN3kitgzqkQ
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
✓ Created session: adrkqaUlLO292DEXjF9g-V8TStYqHCmoKU2zeyvk7CFpO9XTqFR3qss8ZtZSXRoYXns5MrvgmZt6YarMyvxcIA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is "ekoDB". It is a high-performance database product, priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Hg0m-n5jbm5b1Sw5y7VymFn3_eWKJP6k22S8rme5rig4Xeiot00is67VXY6YXazscqnjtmoHEEHZ8rpSY-GdJQ
  Parent: adrkqaUlLO292DEXjF9g-V8TStYqHCmoKU2zeyvk7CFpO9XTqFR3qss8ZtZSXRoYXns5MrvgmZt6YarMyvxcIA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: Hg0m-n5jbm5b1Sw5y7VymFn3_eWKJP6k22S8rme5rig4Xeiot00is67VXY6YXazscqnjtmoHEEHZ8rpSY-GdJQ (Untitled)
  Session 2: adrkqaUlLO292DEXjF9g-V8TStYqHCmoKU2zeyvk7CFpO9XTqFR3qss8ZtZSXRoYXns5MrvgmZt6YarMyvxcIA (Untitled)
  Session 3: fwouIl2cr-G6N6vNT0z14MolEjcyX2FSbskE7sSUhtpFrghvIfYLMZYeoj-7-HKa8zhavOf0bf0XN3kitgzqkQ (Untitled)
  Session 4: doaPGJAF7e3z4i0nEIk9aJm3ASjNJ51v1ObUWXku0_s4SqDqnEBotSJ12_4wIanyzkSEcPSqTXEm0jVd_xM1Uw (Untitled)
  Session 5: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw (Untitled)
  Session 6: L2n3N8nWclhIqgCg_HvoXpytdb71VhqTt_bw1_KjLcMzxTom1LQ9TYXYlaiJBahUKc_Q6LPU6nAMWPHPP4g4Zw (Untitled)
  Session 7: cpHlk-ADD4R88a2saVoQSXnT-rPHLY-3TLG0Q-4D1KmO5jf_Yq15-g-qFDRzbar4P_XZYuWWMmUL3QOqQXwdeQ (Untitled)
  Session 8: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw (Untitled)
  Session 9: GtfMq8eRFBL5hDUmOLtkGd4bjUEosRy4nkpiK5_-XmqIIiKyyeNwaHW-_s-pEBkz5CEYEcuVDBgdrN47637TsQ (Untitled)
  Session 10: r3WEzajhiOln7bNynUKzsDgIZFkUdrh2pPsREnAIE12Hs9SDxUUNYqdLerZ05GZScJZq9i4hw9rGdStaEqZt-g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: Hg0m-n5jbm5b1Sw5y7VymFn3_eWKJP6k22S8rme5rig4Xeiot00is67VXY6YXazscqnjtmoHEEHZ8rpSY-GdJQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'S6DVbm6xA8EkWbgiH6tmRSOZ2MWlrxw_49Yfu7vNln60X4mQIAsFcpESIauHrnR4iu4fdkNCR1rh6pHIA_i7Cg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'S6DVbm6xA8EkWbgiH6tmRSOZ2MWlrxw_49Yfu7vNln60X4mQIAsFcpESIauHrnR4iu4fdkNCR1rh6pHIA_i7Cg'
}
✓ Second upsert (insert): {
  id: '3fW5RvBtQmwB1oe7Wicap8o29hlcssubUKCmWgw7fsrv5cDMdF2Wf5aHG2oo_bhORea1H7OvBNXaPaPl5wQdtA'
}

=== Find One Operation ===
✓ Found user by email: {
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { value: true, type: 'Boolean' },
  id: 'S6DVbm6xA8EkWbgiH6tmRSOZ2MWlrxw_49Yfu7vNln60X4mQIAsFcpESIauHrnR4iu4fdkNCR1rh6pHIA_i7Cg',
  email: { value: 'alice.j@newdomain.com', type: 'String' }
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
   Inserted with ripple: {"id":"YGqCXk4frebAfWXanNUYVSLO57HeC0Ktpm7TzT9SqC5fA_oe5CM3WqsQ0CPtSMHpYdydwDEH5Be8cUJ6qLPTuQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"t9oaBP79effRNED-Y_pQvtgYUJeohkJqMsM8p6pVLK2miTpWhB6vWEiukRdzxdJXridepT5piaGslZdr5QXogA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"YGqCXk4frebAfWXanNUYVSLO57HeC0Ktpm7TzT9SqC5fA_oe5CM3WqsQ0CPtSMHpYdydwDEH5Be8cUJ6qLPTuQ","name":{"type":"String","value":"Product 1"},"price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"eZ3gP1C7aGQpcUV-tVaxjB27kPxPZfblKgrUpNF6PQwe4CxMspr9poODHPj2ZmJp9tbXBTJ4ygdOscar7uywXQ"}

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
