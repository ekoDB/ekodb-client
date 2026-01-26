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

added 1 package, removed 1 package, and audited 13 packages in 833ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '3D1ASBe9iDtTF8b0xj29HfL8vFMNFR8z9XO2yeGYQ7-xpEURZH2g_WLcnMbnq3WWeHRP8A9hPvt4Zfr5ih2l7g'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  created_at: '2026-01-26T22:55:56.565Z',
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  value: 42,
  id: '3D1ASBe9iDtTF8b0xj29HfL8vFMNFR8z9XO2yeGYQ7-xpEURZH2g_WLcnMbnq3WWeHRP8A9hPvt4Zfr5ih2l7g',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  price: 99.99
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T22:55:56.565Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  active: true,
  created_at: '2026-01-26T22:55:56.565Z',
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  value: 42,
  id: '3D1ASBe9iDtTF8b0xj29HfL8vFMNFR8z9XO2yeGYQ7-xpEURZH2g_WLcnMbnq3WWeHRP8A9hPvt4Zfr5ih2l7g',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  price: 99.99
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-01-26T22:55:56.565Z' },
  price: { type: 'Float', value: 99.99 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  id: '3D1ASBe9iDtTF8b0xj29HfL8vFMNFR8z9XO2yeGYQ7-xpEURZH2g_WLcnMbnq3WWeHRP8A9hPvt4Zfr5ih2l7g',
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: wsAkbbGRb7M6YbtmDqeTn0-26o64paYtx6nzJapJ9A7WONhwH3YIgBYbr9JvqIhS7Zf56MyeQfPCM-DDaTelAg

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
Collection created with first record: oSboI93WeC5jBbYl2j-wuskW4csNXTh4Fi5escu6TFVwBiaZkX_zx5vzPN26MyNou7yi4cnHNQH73tCQMKRbYQ

=== List Collections ===
Total collections: 20
Sample collections: products,schema_products_client_py,schema_documents_client_py,client_collection_management_ts,batch_users

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
  cache:product:2: { value: { price: 39.99, name: 'Product 2' } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

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
Created Alice: $1000 - ID: TTGrngM5sGtIKV6bO_fhTfqLuq_wOH7ettxwn4yAvrl98RlrdcvfAhXVFJl4g9tFqTuj6HW2EHR05aXW0cghEA
Created Bob: $500 - ID: IWi8-buWKXkjQNFrTWCmiGuYduq4izSBWbVaDVUHDHWfJLJngsonJN1SuDwKtBJuYdnsbFNsgnqDrIWHr7XPQw

=== Example 1: Begin Transaction ===
Transaction ID: 184925b9-8143-4633-a4e3-e8f000c0437e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6adabef6-394b-4104-a0c1-333effb08f6d
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
  1. Score: 25.740, Matched: name.value, name, email, email.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio, bio.value
  2. Score: 26.400, Matched: title, bio.value, title.value, bio
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: title, bio.value, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, bio, title.value, title
  2. Score: 39.600, Matched: title.value, bio.value, bio, title
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

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
  2. Score: 0.753, Matched: 
  3. Score: 0.745, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.714, Matched: content.value, title.value, title, content
  2. Score: 1.501, Matched: title, title.value, content.value, content
  3. Score: 0.298, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio.value, bio

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
✓ Inserted document: khr8KMzEmearwoU93GUPmvxIYfh47LBS61PmMder_ma4sTS75eugCEM7Shl4Wzo91qLbANcjdTbxUjvv9li4Dg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4WU2nV5kl_84t3If402UdUG1buwrP6zFMlMql6LB3fYoVpbSTBhlzwf18sH5SQGAlC74335jVrxSuyIdab1lIA

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
✓ Inserted document with TTL: RVVgp0zgqPEw84mBoNW99kfRox5GpeqVHZ9jptzEVi8cllJ7oljxD7FvhWcvcCjDfw8h3GyQpz3BWhvsA4LVzg

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
✓ Edge cache script created: GOY02TdRJeSes9ZwpKT92HSQUychHaMJsLd4UvsDh7L9GvQG_TCwleQl3CBRH8Kk6vcdxsRaEmw2BbnOJqdL-w

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

✅ Script saved: 30eCyMPsmLOD78dvGF1o4o2Bh_QL-XWnPS2LcROuLK7CUlowLBuCZ3Mf2dMNl2oZwzl0CyznhoAJ9zfTj7ZtEA
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: MAWH8wNC0Om_NQ_qOknPKsHFIJEk6UMyaFHNQRd8Y7QhvYLCQVHBUBdaUFjxEpolmFsSRcnkeT_ry7VEjowCEw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: OARZ7PvEy5uWhNRIPx_DVrYijX-pUEJ_xGg0NdON20mo8zuFV3O0xImB708coLFOocSjBNBIe_Vq-GKKjgW0Kw
📊 Statistics: 3 groups
   {"status":"active","count":5,"avg_score":60}
   {"count":5,"avg_score":50,"status":"inactive"}
   {"count":5,"avg_score":60,"status":"null"}

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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

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
    "name": "Leanne Graham",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "suite": "Apt. 5...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "suite": "Apt. 5...
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

✅ Script saved: fzX3JMZPkvhGjGKG08-E4Cho3LhhoupaBLQkf3QFhkPqNXHkCWmI3anLxgKWJHtSX7Qke96c-c_wTv2z6f-BhA
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"category":"Furniture","avg_price":474,"count":2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: ztsbzN8E3Ppg62cDnBU4ZWcJmhEwjmvlRTibtO3ItHtc7BBzdRqtpw7EDpTUCnpNKwh2aTnJ_f3pTnZv3SjsMw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 0DkrAJ75419bDmT9aFuRNsxFRO3m-dzj0BfKNuwFKDwBEcorsk3q2zLT7Kq5uGK0yFDtHL_shG8xtjmCn_zSFQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 8lpkPzzOL0kX25fVfJ2OVTPb6JPgaD5VxFDqFI23EQ_6lIaBbYe_aWvouDjfcjS9JnqXdXdHfgpNQaERpDQ7ew
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: fYDDGwwYjIsPbY_Or-_KQtGwHMSEixsi9RLwpPQp1qhqn46x2cx-XxG9BMH4K8qYkDhx070X1T9zi1bvqIChjg
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
✓ Created SWR script: fetch_api_user (nIUOspsJkKlYaqcSX3u5NCqXZhDI1EPJqalEFE6DHAPaO4nO4bDyISWZDNeWlkh3X0WZo2kLyfKyeZak2HQyXg)

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
Response time: 3ms (served from cache)
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
✓ Created enrichment script: fetch_product_with_reviews (sux78RJt8omJCSZO2OSZ54maQBzQbz6H1GCyJBUnEzupOOkt9O8UTqEh1PyFzkrVOc8qwIixGe4zpv1QDfpsYA)

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
   {"category":"Electronics","count":5,"avg_price":367}
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
   Vector databases provide benefits such as high scalability, spatial accuracy, and better representation of geographic features. They also allow for complex geographic operations like overlay and network analysis. Additionally, data in vector format can be easily edited and manipulated.
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
   {"avg_score":20,"count":3,"role":"admin"}
   {"avg_score":70,"count":7,"role":"user"}
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
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":3,"category":"Database"}
   {"category":"AI","count":2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: JToxVxwy9m-aQ0baFIrjo0PtqS51i8p9N88c_RHC0v4z17BNDGuTHGNEMWVwQ10UH_6RtpwtGrM63PdyqN75Aw

=== Sending Chat Message ===
Message ID: P8gOagkSr6vmh0s5wgXE1LQlYOChGC0usVsyyMNChVTbQTJy5c30XyyVkYCBV8g-3I9W3CXZUmg_CiVCOkG9dQ

=== AI Response ===
There are three products available:

1. The "ekoDB Pro" is an Enterprise edition product with advanced features. Its price is $299.
  
2. The "ekoDB Cloud" is a fully managed cloud database service product. Its price is $499.

3. The "ekoDB" is a high-performance database product with AI capabilities. Its price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'zcdPFVwO50mpp30oh82xCLCo-ei8VmFb3NFbZIaYwbw3C3k9siQomOf1nTycYzk728vwbmVN7zGKO1KWqiRO0A'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: 'TzAL9RyhYIy4smgKAkyj8U_tEdY_DmYtLzhOI7-OFjorq7Lh_kVIgsPLi0bbKGP9yErRgq_YliZJl2nPqv_fPg',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: 'q1hK0v9AvFl5Pa1NUmeWSYnyqq5vdl-jbPL_JumxLnV4yVdvbv2PQ2gkvJGFUTsV8qz6Oh7b5a1AhHAkbvw_ow',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3198ms

=== Token Usage ===
Prompt tokens: 448
Completion tokens: 76
Total tokens: 524

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: NMXAX1pvNsGTDlfXY1PxlrEVVjG_ZxPWRWsXvbWk58e8tg8ffxUUFXqLEBoM8wGUUjIV-ZrkToXFw759R5zqDw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, there is a product called "ekoDB". It is a high-performance database product and it is priced at $99.

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
✓ Created second session: bmtKFSsJIMgaxYxIJIgwDd7vqN1yertFomG__Mw_2ViZSrcjToYxhjK62OgIGfTnUdQA6Ihz7U--JY3HYEO97g
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
✓ Created session: 6HMYe8f2LnvnT7htO9InHcE5ZpmMWDJiCxpKSnhFsoCMT35VT1BjOWXE_mgY6rjk4N9upYD3JdAXu-XGceqrew

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product on offer is "ekoDB". It's a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1DzU8Sgq78Ske2VenfpTi07dGiRTV7Uk8HNoJ6xTwGGLhIrvmbK44IKzOQ0cJzVO1dnsXPcTow5d8Avtv5hgGg
  Parent: 6HMYe8f2LnvnT7htO9InHcE5ZpmMWDJiCxpKSnhFsoCMT35VT1BjOWXE_mgY6rjk4N9upYD3JdAXu-XGceqrew

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 1DzU8Sgq78Ske2VenfpTi07dGiRTV7Uk8HNoJ6xTwGGLhIrvmbK44IKzOQ0cJzVO1dnsXPcTow5d8Avtv5hgGg (Untitled)
  Session 2: 6HMYe8f2LnvnT7htO9InHcE5ZpmMWDJiCxpKSnhFsoCMT35VT1BjOWXE_mgY6rjk4N9upYD3JdAXu-XGceqrew (Untitled)
  Session 3: bmtKFSsJIMgaxYxIJIgwDd7vqN1yertFomG__Mw_2ViZSrcjToYxhjK62OgIGfTnUdQA6Ihz7U--JY3HYEO97g (Untitled)
  Session 4: JToxVxwy9m-aQ0baFIrjo0PtqS51i8p9N88c_RHC0v4z17BNDGuTHGNEMWVwQ10UH_6RtpwtGrM63PdyqN75Aw (Untitled)
  Session 5: vIkerozaAQbKTwFKq5b79WasmEUkBW8FnwuwBtvUPKVVtL8jry26uj9VJ7-R-lJO_VSYmKBgB-YAgoXUu4WYgA (Untitled)
  Session 6: jBVTTzZ69hYnYPb7KjJmc_mUrtTOlS86_zYWhi4yc9DZgyPeDLwFr5z4S0bA13twwEabL5YuAUIjrRDTGC0VDw (Untitled)
  Session 7: kH7-Vta-h2D_ObzJ5F7JY5yXmQLoSUq2DDjvWf0gxR5lfY1gIXOlDWMqEt2vV2XJkwmsTYJKlC6SM4Mnind7dw (Untitled)
  Session 8: kyC0Ud3IJFGfhFk2Y2madFyLgiMvCvPmsvGcY1ih02i2fp016-68MsK1BxLwHr8CiBTZH7_6rwrr4c85w1mskw (Untitled)
  Session 9: abgQ5GG0yWu3q7SuaGVLCcUVlpN9Lzr4K_Ny5-l2OTbo_TIKL8_i_0l6B09fmXoNwSeA7l-_BUQDVow0CLWaqQ (Untitled)
  Session 10: uFJ8rSkbpXH0q3zdp1u6zQl-s1cb-aPG7sUj-qT6fhtiUDubtV-rfrCcwCAbv0AHO4McRBQPiLBRyERPvdYQ5A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1DzU8Sgq78Ske2VenfpTi07dGiRTV7Uk8HNoJ6xTwGGLhIrvmbK44IKzOQ0cJzVO1dnsXPcTow5d8Avtv5hgGg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'Uf-waAU8P3cYcIbvfjeywow_mAtX8k-X8d6iY8U4hUrwgRuZywjXNUhrpxh3c0kKuZqJa3YtssGpT3o5L9rnOQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { value: 'Alice Johnson', type: 'String' },
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 },
  id: 'Uf-waAU8P3cYcIbvfjeywow_mAtX8k-X8d6iY8U4hUrwgRuZywjXNUhrpxh3c0kKuZqJa3YtssGpT3o5L9rnOQ',
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: 'HxEq40Q9GihzR6ihOqaID7bCb8B0dt1WPyVfV_tNv17FGtiMu8BLH2AcMuQncprUsz98AssiWTCDrCEtzNNeRg'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'Uf-waAU8P3cYcIbvfjeywow_mAtX8k-X8d6iY8U4hUrwgRuZywjXNUhrpxh3c0kKuZqJa3YtssGpT3o5L9rnOQ',
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
   Inserted with ripple: {"id":"SwaRpAnNc27S3ZP46dDThn8bDhVHcBAVdR4_3Rj5RE4Fkn3rv0D8t2_2LNaVrvaGzEg2TwwIvjCgNjFwRVsXTQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"Rsx952dN1J4VkrcJ-R-f2My4w1TNZDfABbxV47mGO0OFl3Rz1X87ZkLPyiuhR_VFccG6yBYPsDdUOUSjH7yijQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"id":"SwaRpAnNc27S3ZP46dDThn8bDhVHcBAVdR4_3Rj5RE4Fkn3rv0D8t2_2LNaVrvaGzEg2TwwIvjCgNjFwRVsXTQ","name":{"value":"Product 1","type":"String"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"reNqZkPcES5aFvWcmcduc2aFmQUJOsorhUGPoB2itxlM9b58ICdLb-7_M11RnvvggKfQ10NEIk_ArcAzBcP6JA"}

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
