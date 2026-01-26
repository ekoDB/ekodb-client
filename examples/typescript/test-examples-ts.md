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

added 1 package, removed 1 package, and audited 13 packages in 829ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'HN48ECCuA7sbCTxMLrLMNGBsxDYq70UVxBqkBHEQnefLHv-0q8VTybIMrQYLaJV6MJXZHwKeN6Lu65sOpQdrXw'
}

=== Find by ID ===
Found: {
  active: true,
  value: 42,
  price: 99.99,
  created_at: '2026-01-26T15:06:48.609Z',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  categories: [ 'electronics', 'computers' ],
  id: 'HN48ECCuA7sbCTxMLrLMNGBsxDYq70UVxBqkBHEQnefLHv-0q8VTybIMrQYLaJV6MJXZHwKeN6Lu65sOpQdrXw',
  data: 'aGVsbG8gd29ybGQ=',
  metadata: { key: 'value', nested: { deep: true } }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T15:06:48.609Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  value: 42,
  price: 99.99,
  created_at: '2026-01-26T15:06:48.609Z',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  categories: [ 'electronics', 'computers' ],
  id: 'HN48ECCuA7sbCTxMLrLMNGBsxDYq70UVxBqkBHEQnefLHv-0q8VTybIMrQYLaJV6MJXZHwKeN6Lu65sOpQdrXw',
  data: 'aGVsbG8gd29ybGQ=',
  metadata: { key: 'value', nested: { deep: true } }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  id: 'HN48ECCuA7sbCTxMLrLMNGBsxDYq70UVxBqkBHEQnefLHv-0q8VTybIMrQYLaJV6MJXZHwKeN6Lu65sOpQdrXw',
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  name: { value: 'Updated Record', type: 'String' },
  created_at: { value: '2026-01-26T15:06:48.609Z', type: 'DateTime' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  value: { value: 100, type: 'Integer' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  active: { type: 'Boolean', value: true },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: pIUI5BZ1tHMlLLznh3sv482DLRsSlNd_2WPk7pXynKWf7GVMrQV3N_Yimqt5AnM9mza2QC9zbt9Gjk1aPs_1lw

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
Collection created with first record: KMFRxXDIZ6FSn3WtR5ozV4uxnqbLd6GmyYVMmAGfDw2YCMFMxvSffZFApD_n7va8q5m2JI9iATJ-AVCYtgnSfw

=== List Collections ===
Total collections: 21
Sample collections: batch_users,schema_employees_client_py,ttl_cache,chat_configurations__ek0_testing,schema_documents_client_go

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
  cache:product:1: { value: { price: 29.99, name: 'Product 1' } }
  cache:product:2: { value: { price: 39.99, name: 'Product 2' } }
  cache:product:3: { value: { price: 49.99, name: 'Product 3' } }

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
Created Alice: $1000 - ID: 3ta5r58mMawCvmdIxDpoEbXhkECmElExgU5669cHenqpKav3i1M3BXgcgDzcovRn92zntlgLdRsZAXAflU_y9Q
Created Bob: $500 - ID: h-sSpdYMS8c5vNSYVX-ybCOKzpymqfQUbdsBFBDhOPuXBTJxvkzuhG9fHUym-Y0wPFbdiAzh2P4qINLv4VDS2w

=== Example 1: Begin Transaction ===
Transaction ID: 384fc9c6-736f-46c8-a498-0585244ab8f6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: cf36828a-c692-4222-81c9-b7c0dbe7b00b
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: bio, title, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, title, bio
  2. Score: 39.600, Matched: bio.value, title.value, bio, title
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.784, Matched: 
  2. Score: 0.768, Matched: 
  3. Score: 0.730, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707, Matched: title.value, content, title, content.value
  2. Score: 1.513, Matched: title, title.value, content, content.value
  3. Score: 0.292, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, bio.value, skills.value

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
✓ Inserted document: -ie4yfC7o7J4E7a1SHPQkZmFupM2BkKptL2jdDg_BDzjGd98BADhyxMZ0ERf39pkZjWTZXur5ayTsKOE6UfNvA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: E75SlNiUUXjvCijCJFXMZTflzYe0Tz2h9DtvBvwqGN_0aNAAaZj8uZZMJ_gRe3N_sqK78H0tMBMJhqMcbB_hdw

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
✓ Inserted document with TTL: sp34cqXAOz8dl05PlKpFoySwqw4Lgklc1bb7jKy_qcdQja8O5ILitkZfAEq4PhfyxZxao1EKjcRpN-NVPsoj0A

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
✓ Edge cache script created: fvWduTPe_FJRwBFyk5XuNjeWkfB8A8pQmqeOgv3yHA_4OUVwdx1fVuF9Mw1imnNqv7AYBZ_sBaweQEpR7_Wrdw

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
Response time: 3ms (1.3x faster!)
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

✅ Script saved: rnGkg_RJzOBYRTA_KpODx33BYs0HQETZnYs_K56JIsT2HaZ1A9PeyNOaJm0TDYD9_4FuKJoi404_hOMDpS9Law
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: Dfqi3E4moH5qzdt_o5yxd692WgLmPYMVN6xmnbx-PHopxj4_2WnDijvd8wJhNaOdMef9i6t_5WabOdyPvAIjpQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Ls-N__gqKREtHjm2Up0-eLMC48YbDERJITe7z81xOKSI9apJRYxW_B1mL2lEa1q6xHzTg5tc1EluKfFhmKo4Mw
📊 Statistics: 3 groups
   {"status":"inactive","count":5,"avg_score":50}
   {"avg_score":60,"status":"active","count":5}
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
   ⏱️  Duration: 64ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 66ms
   📊 Records: 1
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

✅ Script saved: k_xTqAMJFC-NePbKL-VJCYawQt5rXNETrdncbJowAzF5e32GvjT0NqCekx5wFw-ECu6nMNNvs48r720YRIu3UQ
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
   {"category":"Furniture","count":2,"avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: hmVgVYd2Qe1IGd6apmpVlcWn-bGhwrmbpxB8bGRXUOEaWklZmagqQXfw_H7aMNKe-U7DnnZZIEa32v6sVBoGLw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 7PUBPI1SOKrBPFW6-5vfQCq-vDMhQhwbgrSEYfuE07mmLTV77GKkMTFkkSjcN6t6CzoD5lsd4-SfmGXgVnoxzw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: r2qsmaoOBIR8AJ3bHhObUtA7HF4Rv9dJTux8eVB7V-jIGRlRPrp44zxkOWONCUg9lJ1qMxHSFHlrzIKUpdr3rA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: dy0ImGB5LauJnEkHiAT_JIrVII3DEAaF0xUHbNAJXkx_g6slWRkud6tL7648yS3bFnHoW7sM6-Ihz64iA90Aww
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
✓ Created SWR script: fetch_api_user (nD_A4-yulxni75YUvbc6cyCrNqriQ05YQVJ0STulyjl-JIwfPf2Q0d54rHVN6frfipC5ldyFtjtIcBKX-gS7aQ)

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
Response time: 4ms (served from cache)
Result (cached): {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (msdnxsedBW__Xu1-N3248fuag3kNb4XBHM9SFOHY0FRyvkRGNIcoci3D28jB6wE8WMYuwq1tEaYXnSExvM0_kg)

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
   {"count":3,"avg_price":365.6666666666667,"category":"Furniture"}
   {"count":5,"avg_price":367,"category":"Electronics"}
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
   Vector databases offer benefits such as efficient data management, high precision in storing data, support for complex geographical features, capability to represent data at different scales, and ability to store both spatial and attribute data.
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
   {"role":"user","avg_score":70,"count":7}
   {"role":"admin","count":3,"avg_score":20}
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
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
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
✓ Created session: P9Gtt__FZAGXMtDCzI9MT2N1gzXCMhjEAa1-TF9ILjQ1ACRA4eq914qs0HtLB7mmAy3pjzxtcO4hRA1YTE2bOw

=== Sending Chat Message ===
Message ID: xwqAS1MpGDzhbAlvRCG9_5d6HIy8FoWqETyt1v8y26NFtEbUC9FvYw0nUJISrSZh4m-YoKLS8bH-kN2qm7uYFw

=== AI Response ===
Here are the products available along with their prices:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: 'qM0rzlLnOKuujmI4phRKAxptGTxv40n6-ROB6nz0UVPhgK_tsyx4FLs_YYzmofqdhhAFgmfzmktXb1oBoB6I3Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'wFXdYtcI1r6323Zdu_54Hp0LMojdOAiiSlfFSq4BIbVrVbUk9zlDqb9LqdFjDBNj7kb82NbRxuMtLOHEtFg2-w',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'j04UCAfSEVSBrBTK_edvulGxsRqCqfgIu4g9ARu_nMPYlkkfLIVLPFPCFbIrB_UfujgWiW9qEnyzl-JEYZx_Rg',
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2296ms

=== Token Usage ===
Prompt tokens: 445
Completion tokens: 88
Total tokens: 533

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: AyRQZC_46Ymm3EMuwmxQDt-w8FMa677Q1rDM9lWvb8Pgn25mv9dJAD2KtdnoeKfjbEyRi5H-TWdAog3FvwqsXA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is "ekoDB". This is a high-performance database product. It is priced at $99.

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
✓ Created second session: FB4-TYOgQSPy0uF6q52_r9rLImQOmn3HVKf6BvDduWMjeTC4gkL3_7fb-FEDBXCnat2j-KLUZw3ERScwlqxqZg
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
✓ Created session: XzoY0Q3plKT5z1-cw0LeEN9F4tZpkI3AZHbLfdgmKiuv84NhQsbRCNGQ32gP4u3I4UqWGHNh27Ve1pYiMjVjpw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product and its price is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3Kw7zYysTDZZP_heWQnlunkfy8kd4Opc8kS6zkxyJ6Eo7BREtu6Qm5I3dos0UQS4-Fpg2ekWsvJB2QlNawO69w
  Parent: XzoY0Q3plKT5z1-cw0LeEN9F4tZpkI3AZHbLfdgmKiuv84NhQsbRCNGQ32gP4u3I4UqWGHNh27Ve1pYiMjVjpw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 3Kw7zYysTDZZP_heWQnlunkfy8kd4Opc8kS6zkxyJ6Eo7BREtu6Qm5I3dos0UQS4-Fpg2ekWsvJB2QlNawO69w (Untitled)
  Session 2: XzoY0Q3plKT5z1-cw0LeEN9F4tZpkI3AZHbLfdgmKiuv84NhQsbRCNGQ32gP4u3I4UqWGHNh27Ve1pYiMjVjpw (Untitled)
  Session 3: FB4-TYOgQSPy0uF6q52_r9rLImQOmn3HVKf6BvDduWMjeTC4gkL3_7fb-FEDBXCnat2j-KLUZw3ERScwlqxqZg (Untitled)
  Session 4: P9Gtt__FZAGXMtDCzI9MT2N1gzXCMhjEAa1-TF9ILjQ1ACRA4eq914qs0HtLB7mmAy3pjzxtcO4hRA1YTE2bOw (Untitled)
  Session 5: XVLAxRIZ2pkRmHp9G4_7Fxu3Q3sWMtIfMsNr_xL7N20PJT9DHMvXAk9fn_d3yknl2b_WHniFz3mwSyofDnGjVQ (Untitled)
  Session 6: D4Z_YfTWI7OhM6qRxOYYxzbG94AXkSb6FBwL_qCLZ2a2Xy4gOwGLjEmD0XswFmozj2pgAXGv_Tmm9VUd1b--hQ (Untitled)
  Session 7: pk7aA3dAX2d3eAynAmikTPNZOCiz1gvd-t23welEmVxmgiKenDhkabrvAxJJCY4a187SCvG2PnwjZv7g2Sk8Ow (Untitled)
  Session 8: HFG9kxwFQSLLrPWzsffCsQoAqtrWGP867ZAGED-kiFKNCjBAroYOXSMr96SDrNcSy6IB22lwZ6pUe59bwrjPuA (Untitled)
  Session 9: O9hPELmjeoF5xcC-AXO5BYlAw51P13QCevQPKOms7urZi09TugPjOlCyJwNaWCAPQHtBPKSG5G77SG5izjXBdQ (Untitled)
  Session 10: WEGypk7qoA_kChE-4sfyjDdjSAsXDRjBIfhNexFlxd_H5EtOlhbH93hvkk0xguAi-36XXCmnH-riuzJDgvxy9A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3Kw7zYysTDZZP_heWQnlunkfy8kd4Opc8kS6zkxyJ6Eo7BREtu6Qm5I3dos0UQS4-Fpg2ekWsvJB2QlNawO69w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'jLkkXHZwLhV8mF3BJ6pOFF6L_gYGCBi2nG1GaNZRhlyVUP1AjsoeMLW4ZF8d83E0C_F060leZ4-Nac5wpdCsPA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'jLkkXHZwLhV8mF3BJ6pOFF6L_gYGCBi2nG1GaNZRhlyVUP1AjsoeMLW4ZF8d83E0C_F060leZ4-Nac5wpdCsPA',
  active: { type: 'Boolean', value: true },
  age: { value: 29, type: 'Integer' },
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: 'qoq_a-ZAcvuEFMLRv6LBDGyA6CnF_0eezqsXChvXSy26mUqsZ9qBqoD8fUO2dd9iXVahrwzWgU6heGQahOnbnQ'
}

=== Find One Operation ===
✓ Found user by email: {
  id: 'jLkkXHZwLhV8mF3BJ6pOFF6L_gYGCBi2nG1GaNZRhlyVUP1AjsoeMLW4ZF8d83E0C_F060leZ4-Nac5wpdCsPA',
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { value: 'Alice Johnson', type: 'String' }
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
   Inserted with ripple: {"id":"cC0BHZ6IAuMUVy5JbbtVDOYxIt1du4fYHx1qwSG3xpoA2pVyhThfrMkE6dm1k_8hBm-8HNGOVS7QMCYX1vp5aA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"yua9PZdLYBjL7vi2QTVwXfYTZTQ4wq_M8xx44R8hTK0t6XQANo8vDB6i97QmTpGbU5wdc1eGaZJMdOfzkagfow"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"cC0BHZ6IAuMUVy5JbbtVDOYxIt1du4fYHx1qwSG3xpoA2pVyhThfrMkE6dm1k_8hBm-8HNGOVS7QMCYX1vp5aA","name":{"type":"String","value":"Product 1"},"price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"rnsf-WfkH6BeiCYqoKA1ltfwatax9c4fXv7DnDhd6VtWgrpTh8x4NxzL63QaMFcopl44yUemlSw9FqqXKBiDLA"}

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
