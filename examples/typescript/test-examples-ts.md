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
  id: 'BZmh1Ct60i1T_1Kk5Dof1_W6wj_br_Yq-c9bVHUMCdWG8b0-fNLOaT-Nu2ljYsiARVHvBsvk5gnuQ5qbLb-s3g'
}

=== Find by ID ===
Found: {
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-01-26T20:20:36.741Z',
  value: 42,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  id: 'BZmh1Ct60i1T_1Kk5Dof1_W6wj_br_Yq-c9bVHUMCdWG8b0-fNLOaT-Nu2ljYsiARVHvBsvk5gnuQ5qbLb-s3g',
  price: 99.99,
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T20:20:36.741Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-01-26T20:20:36.741Z',
  value: 42,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  id: 'BZmh1Ct60i1T_1Kk5Dof1_W6wj_br_Yq-c9bVHUMCdWG8b0-fNLOaT-Nu2ljYsiARVHvBsvk5gnuQ5qbLb-s3g',
  price: 99.99,
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'BZmh1Ct60i1T_1Kk5Dof1_W6wj_br_Yq-c9bVHUMCdWG8b0-fNLOaT-Nu2ljYsiARVHvBsvk5gnuQ5qbLb-s3g',
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  created_at: { type: 'DateTime', value: '2026-01-26T20:20:36.741Z' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { type: 'Boolean', value: true },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: kJcq7SstlnP9rBZRPGFrN67iJZjRoqDUNPej-MMkovD3XhKCn7i0oLKMFCTgWBgP3yqPjakoD5TzkSqdc8k6Fg

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
Collection created with first record: MDISO6krs6Wg6O1daXioL-kPXZ3w95XtegbwMpqCJa_21kXnjqAo4lkiWh3GbIMqAEIK0cmdwgPauTU-HyHn7A

=== List Collections ===
Total collections: 20
Sample collections: websocket_test,schema_employees_client_go,batch_users,schema_users_client_go,scripts__ek0_testing

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
Created Alice: $1000 - ID: WmW5si83DHc6E8IXwbJ4UbUvKUXwyho7JXiL2YQIB8Pu1xcUFWfPwxJXcTDGqFRCq7oXq7lPfxe3Ug8waEC5rw
Created Bob: $500 - ID: 3b5dJcZ8c_4bNYDzYBZAn0oruVHNGBptfed2mk_yHu0k4C7Z1T__UWitlYr2EDGYWH_nNiKQNKO8A6jvaYBt5w

=== Example 1: Begin Transaction ===
Transaction ID: 32679e99-182e-4a4a-9834-cbd9872e86fb

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b8acb908-e88e-4345-b808-27702117bb58
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
  1. Score: 25.740, Matched: name, email, name.value, email.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio.value, bio
  2. Score: 26.400, Matched: title.value, bio.value, title, bio
  3. Score: 26.400, Matched: title.value, bio.value, title, bio
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, title, bio.value
  2. Score: 39.600, Matched: bio, title, bio.value, title.value
  3. Score: 39.600, Matched: bio.value, title, bio, title.value
  4. Score: 39.600, Matched: bio, title, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.754, Matched: 
  2. Score: 0.746, Matched: 
  3. Score: 0.744, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698, Matched: title, title.value, content.value, content
  2. Score: 1.502, Matched: title.value, content.value, content, title
  3. Score: 0.298, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio, bio.value, skills

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
✓ Inserted document: NHlWoxNBsdKKjEmsunHZwcFB-dfbgpy3WSujkzBE1XBtP1_kCEpDX6Af6rxK4vEqS4gk9hS_3ejWu84Db71Rrg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: AQpfSrathsmueZKYl2OPVbYhetzfu6bgEv3S66hpf9bcwL8NrWfWpaX-mtu6Y8ze1rbEz-vBqI45PtwW97FyKA

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
✓ Inserted document with TTL: obqxIvJOKyVF9tHuz5uQZYUQ4vmHRNghsWpTne-UcpIX63UjSdxTb1cvYGm13CCrJExdbUiKdAwwmNlPq4f4pw

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
✓ Edge cache script created: ycjiZYzXOqM4IE2COg9dLkpYLKh0DtB716Y4VDrMcpny_cIB5FDR_AjarzQagy8Mk_MbwyYPHNYMBMZXE0F2cg

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

✅ Script saved: 14sSK7IrOsSNqx7LoMUtoFGDMarLyASmvdkUvdxG-5mTG0E6VenskeOgaXZHByRSWpxM-hJhEX1El1XjKiPEDw
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: Zsfi2AlfXNZF_VYVcpErqTF4zGCJoOmwIA_DSi2WqmIpCXBsc0fijSPkBUXK01bM470p52hMML_30jWX2TbrnA
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: ko_I-61ONf38Q3uNqkRfsS8UyuMbn3r1JXwIJGiTSGGBLQ6hN2ut8klKdKL6MS83X23X5DX8i_SOWqtU-sqgPw
📊 Statistics: 3 groups
   {"status":"null","count":5,"avg_score":60}
   {"avg_score":50,"count":5,"status":"inactive"}
   {"count":5,"status":"active","avg_score":60}

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
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 0
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

✅ Script saved: xactlDbRlsGBZ6-RaNdHmQbqM0a-kJa00awhSZelCdGqEQ5vypTQrm2yqIEqcD1ep6sZjUg31zKu1wAOx3Nh9A
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
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
   {"category":"Furniture","count":2}
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

✅ Inserted order: 8FE_VteBQ_Km5Fvo3lpS4GmVP6Tz4NeEX9_SYhetBLs-ChhuFAAfOHTAUfMqv2qyPsCbtqoZP7bVcECZy3-96A
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: eDGUQqlquqjeTjaLznDRm-kL86kdqWXX4GOw-H9FhwdrzbBv9V3U6_PydLvZGoEWOkyTTzglXjwqXU5bHcqJTg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: pPFiMI-oHLdtIPtvYCFmSwM2sDwj5iId2ZoGsCsx9DlGPRwr2pt1ri85VVgaiU7hhF2WSL00ezQUOzo383qr6w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: S0F9Sh8aARSOp7zaiHprkJ4Cwq4oypgbwQBRqpHokqB_JYidtWH3W1h_eglaSjMyIFeUfDn1-DEvQnC2bFNcRg
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
✓ Created SWR script: fetch_api_user (bZSzJTG0kV8-XSZkf5GdDbCdsZ2q4b5AlKo9-ns7F4IYtNfBllqj389FWET5kFDLww5YT-cxCPGxUX5wr9o4ng)

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
✓ Created enrichment script: fetch_product_with_reviews (GjYCunsskR653puZc5H-rH0Bvy7gPP0A7G4E6TCsQQnQ41yXSk9tJfnpnDiowowWu7VNm5m3MpzvW-y3lwPqXQ)

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
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
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
   Vector databases offer benefits such as precise data representation, scalability, ease of data manipulation, and support for complex data types. They allow for detailed analysis and modeling, as well as complex geometric and spatial operations. They are particularly useful in fields such as Geographic Information Systems (GIS) due to their ability to accurately represent geographical features.
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
   {"avg_score":20,"count":3,"role":"admin"}
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
   1. Vector Databases Explained (Database)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"category":"AI","count":2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: BJL5BGveOvrlTBWa7LTSiHSpT8iOJWuHoDQjzhX3oa_NErMdUJcxXMoJrTZ6VG8DTXGK2m5VAMTF_FZ_Jmzeug

=== Sending Chat Message ===
Message ID: glUoqnnixkCL6adiYCUfAPFFYpayH3XQJE_pXDLFacbNfmT75Xbj3qbgPwOx19nnaOnTCiPuOb6IVQhOBHwaXA

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    id: 'vUWFSwzVQiQ3FffabLsFqCMSbwSrBj3Behm7wuLEWPk3nkohKGYuz8A539HOP1JxfdM7MDNruy2eROcTzytMFQ',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    id: 'QYCi3_1Kf0gvEKxPu1XQWcsOTm75Rbq-rkgFojnrjXUkk7O8TZZNn8cgoOzERXyrMgHGQEV7Cf-UvOg-oBAgGQ',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: 'M1OEjPeFaSIZriWAGcoSdFbfxFSlXOtYTBBbcam1L_yvvWt2pIqnE28QEYQJKctCfQtjKriNlFeLdxDZml4sKA',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3111ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 78
Total tokens: 528

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: mYQaYK9pku6seRVBsDNU1d8vy3cUZPQGuvSJLMBroBtCFJQ_dNjaEtQK4qvQwmlMsgFZhcfbjHRmd_WhI5_dGA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product as per the context is ekoDB. It's a high-performance database product. The price of ekoDB is $99.

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
✓ Created second session: _SEGQUojBCVOpvf9Cq3wk5RyPyP4KUcV74lRMF3DDss6_4WUBBbFzXy0cn3-YbN3Pv-aVuck0zt2K9hb4BBCww
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
✓ Created session: VkWkyyKMKbPtF1BB-Vt5w0NGcX1A-DR1L5gdt6j4R3eu5BWYHJ5jDBRi6XMcBjEACiT_ebrrjsB_2dOhxKYQQA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It's a high-performance database product and the price is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Zzg6ARoRFfe9XPewupoAf-WgltvsyTWxEdYrYE-kl2Xzsp8ywQa-q3rM38lOPLBwMTpIAJIv-L0bz6sA8qZPTg
  Parent: VkWkyyKMKbPtF1BB-Vt5w0NGcX1A-DR1L5gdt6j4R3eu5BWYHJ5jDBRi6XMcBjEACiT_ebrrjsB_2dOhxKYQQA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: Zzg6ARoRFfe9XPewupoAf-WgltvsyTWxEdYrYE-kl2Xzsp8ywQa-q3rM38lOPLBwMTpIAJIv-L0bz6sA8qZPTg (Untitled)
  Session 2: VkWkyyKMKbPtF1BB-Vt5w0NGcX1A-DR1L5gdt6j4R3eu5BWYHJ5jDBRi6XMcBjEACiT_ebrrjsB_2dOhxKYQQA (Untitled)
  Session 3: _SEGQUojBCVOpvf9Cq3wk5RyPyP4KUcV74lRMF3DDss6_4WUBBbFzXy0cn3-YbN3Pv-aVuck0zt2K9hb4BBCww (Untitled)
  Session 4: BJL5BGveOvrlTBWa7LTSiHSpT8iOJWuHoDQjzhX3oa_NErMdUJcxXMoJrTZ6VG8DTXGK2m5VAMTF_FZ_Jmzeug (Untitled)
  Session 5: 7qELxPg2lrcRGy2spbCbBuOsp81Od503t0WkQ9fDmaf6UvOG_CMVqnir7wTo_-hhK4ukmBoUp94dva071u49FQ (Untitled)
  Session 6: 1S0ufLnEeEi2Yjuhd_ouj8oaoVh694rRPUWARagAFJABjlnRGDZR32ZnP9mdoPKNgiP8Eg-tgmRmUwEGcdZ6gw (Untitled)
  Session 7: ri8J9aosDC9gMwutTRft89s9bxluPtJ7ZTaTMX8IsGsL0M2ATyINvc74OjgMfIqVXTnnKGSKkbdYewbyGG1Nqg (Untitled)
  Session 8: K4Zl3YC8nD5iX6ndffmischiOraG9LEaYA26OQQC0MoUl9OpC9dq8LjD56bzWFfOvTHSry7KQ7fIVXNuqRK-Ng (Untitled)
  Session 9: IU5sdisPG6DUPq-ArVY1FAD_5uVNxM8FNtb3Mt3yyAzGdsVe3Ob_TyahjMtCqe5FF_gblxFglE6d7dzXhye3qw (Untitled)
  Session 10: 1QxWJqEU5iXvj8VlydtoGuU4bMr3VU6FwXWamrjsnxkrO5EcgEEzdzqt0TIbdGXppbLVvhddduxa-EooZD6DwA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: Zzg6ARoRFfe9XPewupoAf-WgltvsyTWxEdYrYE-kl2Xzsp8ywQa-q3rM38lOPLBwMTpIAJIv-L0bz6sA8qZPTg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'ooDDHqzoPAQZIf9reKxp-qVL4v7UohNsxtfAwYtorpCEbt6BUzi2fgjaGIqzIaF4x9k4AjZOuAQQNaGMS1tMgA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'ooDDHqzoPAQZIf9reKxp-qVL4v7UohNsxtfAwYtorpCEbt6BUzi2fgjaGIqzIaF4x9k4AjZOuAQQNaGMS1tMgA',
  age: { value: 29, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { value: 'Alice Johnson', type: 'String' }
}
✓ Second upsert (insert): {
  id: '_G3yzJu4oE-4lDuJarp_2ipDY97nWPmTdmd2Pjhf5GjyqLwrxOsvjHiyvVku7coiKl1gpLJxY5yGKrMY2naF4A'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 },
  id: 'ooDDHqzoPAQZIf9reKxp-qVL4v7UohNsxtfAwYtorpCEbt6BUzi2fgjaGIqzIaF4x9k4AjZOuAQQNaGMS1tMgA'
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
   Inserted with ripple: {"id":"xVO5ucraR6RkP1hJPjgEdOg77-QmqpZtBGjX34oYlgBnQ1zJql-2LL7XXphXstxiXQUwwNwyB3sYCmLshIVTMA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"f6pZFtmjfdruKUb9sjdYn7DudUGCPO5l71snWAG1q8CGj7JfbdOCdcT0AH_zp3vr4APc9g2YR6SWuskDL2_yXQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"id":"xVO5ucraR6RkP1hJPjgEdOg77-QmqpZtBGjX34oYlgBnQ1zJql-2LL7XXphXstxiXQUwwNwyB3sYCmLshIVTMA","name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"axjVRT7sI0arrQhdCn5I2z-wg1Mcty2Ed-q_qFzAVgNrRSBOFJ-tKwDCmjRJNjcgNqAImdYvx1diuX14hCd_wQ"}

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
