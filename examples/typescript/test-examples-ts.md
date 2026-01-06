make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 846ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'NkOTwK2mvpfZxGQAoqd_hWqa0yiUhKEwu0EN4aAFrNedIwm2g1dMYc6juZ2pUwiJkpoApRja-WysZ_mbTpmxcQ'
}

=== Find by ID ===
Found: {
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  id: 'NkOTwK2mvpfZxGQAoqd_hWqa0yiUhKEwu0EN4aAFrNedIwm2g1dMYc6juZ2pUwiJkpoApRja-WysZ_mbTpmxcQ',
  name: { value: 'Test Record', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  active: { value: true, type: 'Boolean' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-01-06T16:11:46.507Z' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { type: 'Integer', value: 42 },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-06T16:11:46.507Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  metadata: { key: 'value', nested: { deep: true } },
  id: 'NkOTwK2mvpfZxGQAoqd_hWqa0yiUhKEwu0EN4aAFrNedIwm2g1dMYc6juZ2pUwiJkpoApRja-WysZ_mbTpmxcQ',
  name: 'Test Record',
  price: 99.99,
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-01-06T16:11:46.507Z',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  categories: [ 'electronics', 'computers' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  price: { value: 99.99, type: 'Float' },
  value: { type: 'Integer', value: 100 },
  id: 'NkOTwK2mvpfZxGQAoqd_hWqa0yiUhKEwu0EN4aAFrNedIwm2g1dMYc6juZ2pUwiJkpoApRja-WysZ_mbTpmxcQ',
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  created_at: { value: '2026-01-06T16:11:46.507Z', type: 'DateTime' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: BjA_o7c7bxFRX8luTpTzZoP-uVOCNarFDg63x5tgulG5F2t5I_2FMcHz12gMZXZokH_ipPOilm1MREkt0spWcQ

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
Collection created with first record: 8XkrVe2X7tEAuzDpHbCCdbaiQOsN41M15mzK5CwPua-Lp-7XIiEmCtGLjKez9FQAMGzrbWF_tPO4LsYfQsQOtQ

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_py,client_collection_management_ts,batch_users,ws_ttl_test,schema_users_client_go

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

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Jphp2U5n6aDtHHZDrtuO1XQMJi9Zdv138p19siK65KfDxJzrWqMAXB-yF-uwPOdK2a-I3D8pTwWfXeYhbxHt6Q
Created Bob: $500 - ID: H5D7vGHHH_a872fGpHI03S-FzOBqXC8bSSB3PuYBKPvqvzuoPyDJe0-p3pd6V7MMRvhb3dE1X8uaF17yOQGmQA

=== Example 1: Begin Transaction ===
Transaction ID: 49c2ad8e-ef80-4523-804b-a31a33baee08

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: af7e0a6e-b026-4b7b-ae84-c5bf57072ba2
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
  1. Score: 26.400, Matched: title, title.value, bio.value, bio
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: title, bio.value, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, bio.value, title.value
  2. Score: 39.600, Matched: title, bio, bio.value, title.value
  3. Score: 39.600, Matched: bio.value, bio, title, title.value
  4. Score: 39.600, Matched: bio, title, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.769, Matched: 
  2. Score: 0.764, Matched: 
  3. Score: 0.762, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706, Matched: title, title.value, content, content.value
  2. Score: 1.505, Matched: content.value, content, title.value, title
  3. Score: 0.308, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, skills, bio

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
✓ Inserted document: lmEHQ1wiMOqa7dc5EA1o0h8pzCvEKrcspbD6EjEr06Yf0HhJeH5USIRMs4AqY-TQ_hXvO2rqr03BQoitzMgXHw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Vj5Rq3xHf72piBSA6AQ4eTGhtJ9vcQkvgAVSLOT6Mb01_zBI-2EbCiMitoIj0N6twSNwrT_BeIbg2y7nBTkRwA

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
✓ Inserted document with TTL: d9zW7p-z6gvuILHuMcPA36CnhfQKnP8FpjQxQ0RnbDJpZkkzA-imkYxDXdWUYLHzed6R0JayzuPFvhuADcsY4w

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
✓ Edge cache script created: FQ07VMy6_2GZvmkOI3fP1a5bauP2I2XlMqxq2OEvFLXSnb9RmbsDBvYfGjyk8G1RSk3V6Qb6ryDQB_2oCAwEvA

Call 1: Cache miss (fetches from API)
Response time: 85ms
Result: {
  "records": [
    {
      "id": "cgbg1_3ca3qP14OjTTOlQOMJSTLgiQkp4l5JJ7NNMdkcJz7uoIsHdLJJnHve0P_-iLQd6rFeRy3LYgegq1RpSA",
      "cached_at": "2026-01-06T16:11:51.980Z",
      "data": {
        "value": {
          "username": "Bret",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "id": 1,
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham"
        },
        "type": "Object"
      }
    }
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
Response time: 62ms (1.4x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T16:11:51.980Z",
      "data": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "id": 1,
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "website": "hildegard.org",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          }
        }
      },
      "id": "D3kCYiP6RdCwQccHbl86nY26Fx_4VOi5g22Ehdf9POf1UR07FMQem5vp7QN6O6y8udDd6KAb87Uh-yXbC2uZlA"
    }
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

✅ Script saved: Td_ihLoFjFlRTD24nR5tDXqlNdq2N4XJK2ZaE8-CcibdKB8EXT1ySOv5XXF6G-AJy6I2fVrGeTZnMHkBPUCc3Q
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: CtqLp-0SspWSvbmWHQE5WXc_KhTpt1plsS1Cq8mBR0wLRaVrNLhwcfc-6XShAN89a71wSvfmWDo5kq9NPDhAZw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: KKOvQE7FQRC4fAKcgfX9Enic6VwAzuRJ0lkL-GSEzTZBCTBXepOCs3yZRQwFUldtKrxXiNXUPGdAkSTB9CSw1w
📊 Statistics: 3 groups
   {"count":5,"avg_score":60,"status":"null"}
   {"avg_score":50,"status":"inactive","count":5}
   {"status":"active","avg_score":60,"count":5}

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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 143ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82ms
   📊 Records: 1
   🚀 Cache speedup: 1.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: Efex87mRKT3hyc3lfTS7smLOs6AxuhJYZIb5WMXVlEYfOZifdxASp1nbHdot9jCnBB3ySSudS-MKeMu3PJi2aw
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
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

✅ Inserted order: 51ANrvHZIR5mzg-g9JUG0msry_j132qFpgk51-xJJCIf8pC599_Woo12eg5WybqF9kf47kPsIR_vA4JxraUdpg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: P_kP3vgLvn1wFEmepzuhoe_tKDpFy60SaP6JMqAymQJt_5eft4MsdihGW_eLBuTvKjwESlq5uAHfpWo74NsOog
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: bv9pM8WsYgrGhSdlOVsBeeFkd5tgeHCgc8LW9NYpXc6IePFRjz2zufJMif7YAvBMcAJyioubWQqQAWgyj3BgFw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 3kSNAgvLTNSLsfW8wwpa6z7XDxI5_ZaoArPsoftjg-xhYMcQUGeGbnIL9Wauhjt3qcr6bumJpuNZUq_Ta7ohCg
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
✓ Created SWR script: fetch_github_user (WNtHJ6kKcLy6_hOsLGO6fERKT_5e1TKbnmSmgXwVOq7Gc1Z8jUiSbZd869ZGBuz9wiKUxZKfsh_qCtiHYjlHjA)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "id": "ZJEYqgbXQF55FpeNz8mHjnVhUPPiz4LG8nPcUHpK8k_ZcRuMiYzZLlC4CeA53DmQzyXw2GSe9qPrSroRlsiIfQ",
      "data": {
        "type": "Object",
        "value": {
          "company": "Linux Foundation",
          "created_at": "2011-09-03T15:26:22Z",
          "twitter_username": null,
          "public_repos": 9,
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "id": 1024025,
          "name": "Linus Torvalds",
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "bio": null,
          "followers": 270299,
          "type": "User",
          "url": "https://api.github.com/users/torvalds",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "site_admin": false,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "email": null,
          "html_url": "https://github.com/torvalds",
          "hireable": null,
          "blog": "",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "gravatar_id": "",
          "location": "Portland, OR",
          "login": "torvalds",
          "following": 0,
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "user_view_type": "public",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "public_gists": 1,
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "updated_at": "2025-11-24T04:16:14Z"
        }
      },
      "cached_at": "2026-01-06T16:11:54.462Z"
    }
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
Response time: 76ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (MVWoOkLEmUN9XeKwaGIDx2A1cS8lnrqHawEw1unsUN0OvdLy7Z5WyEXU-QdcNvVc7HDDK54GEDS8F6SaEMLRtA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2026-01-06T16:11:54.675Z",
      "enriched_data": {
        "value": {
          "meta": {
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "createdAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687"
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "rating": 2.56,
          "returnPolicy": "No return policy",
          "warrantyInformation": "1 week warranty",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "dimensions": {
            "height": 13.08,
            "width": 15.14,
            "depth": 22.99
          },
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 3,
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            },
            {
              "rating": 4,
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com"
            },
            {
              "rating": 5,
              "comment": "Highly impressed!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            }
          ],
          "id": 1,
          "brand": "Essence",
          "discountPercentage": 10.48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "price": 9.99,
          "shippingInformation": "Ships in 3-5 business days",
          "availabilityStatus": "In Stock",
          "sku": "BEA-ESS-ESS-001",
          "stock": 99,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "minimumOrderQuantity": 48,
          "title": "Essence Mascara Lash Princess",
          "category": "beauty",
          "weight": 4
        },
        "type": "Object"
      },
      "id": "hnnkprsSa_6ADATIICz9g-OVIXQQVW2dLmEEIQEG8XMUNbAwOi48AnBOP6fE4aHU0R1hAXI1AFWk0-jJE5rJzQ"
    }
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
   {"category":"Electronics","avg_price":367,"count":5}
   {"category":"Furniture","avg_price":365.6666666666667,"count":3}
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
   Vector databases offer several benefits such as accurate representation of geographic data, ability to scale without losing quality, efficient storage and manipulation of complex spatial data, and ability to support various types of geometric operations. They also allow for easy editing and modification of data.
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
   {"role":"user","count":7,"avg_score":70}
   {"count":3,"role":"admin","avg_score":20}
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
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":2,"category":"AI"}
   {"count":3,"category":"Database"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 0OAikv6tJLqRDgUuWi9Pnl56j2x8xE7bzmwiLTgU9RMca2eAvrP-r175vrCL8NMFU-sg7NLECYq90SvuxK--_Q

=== Sending Chat Message ===
Message ID: AWow5_xsd4ujaD1AS0dy4ob5NHcgwTi8aQymSkOBU-i55hGyw_aJJPGYfXTHE1mwCrQdMs_nnS_24PeJK4PWXg

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. The price for ekoDB Pro is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'cSvFdR6v6AQ5ZwGpdlXD0uq0XHrI1h6t9Zhh2oATeVVkWsQiAQR_OUY7CdhcNcFcx2YXTKD39JHF9N9tlubfWA',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    id: '3iFPjMrT1zzUEUVIPWFhQ0DGC1794z9LaUerFeQrnai2sdTG7rwmGQH49mW6sQiK4hn-ehbjXJmPSs4p2qvxsg',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'S_ynNVqNmr2ZVUdVlc5mDRnyloersYTe89wg8vyheS6YoVZgMrhzMeiifcH6Ap0ckl8b7kNMHGfDw-EUeRT5sQ',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 7699ms

=== Token Usage ===
Prompt tokens: 453
Completion tokens: 93
Total tokens: 546

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Y2q-D-6ez9NzP7rIljtrPcF7qIgkxoYnDz9nl7syRY2cOA0QbaUEMai47E5HKZgyFcs45cbZkiSttELNuPsE-w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a high-performance database product available named ekoDB. It is priced at $99.

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
✓ Created second session: 9lzCblLUBkxRFffds-OVpG9c64OgZjfh7QP7knhbj8J8Zy-tt4xi24dUxNlQJojSy0TRk1HH_cqAGuZ5sFlChQ
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
✓ Created session: mokgXfm_deB8_nvB7m2cxF9e2-pijcG8V-souy39OJNjzlaLN_fLpWv_EUrNc5TCBYqoJB7Gako8ntad_ThuOQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 0dcPElXXqozuaR7GyPxnkvHWjPHq0m6it4SdEu5eI40L8MQ_ysBD1UM09wsiOge7y8V1CzbK7slx7tkQ1flTQQ
  Parent: mokgXfm_deB8_nvB7m2cxF9e2-pijcG8V-souy39OJNjzlaLN_fLpWv_EUrNc5TCBYqoJB7Gako8ntad_ThuOQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 0dcPElXXqozuaR7GyPxnkvHWjPHq0m6it4SdEu5eI40L8MQ_ysBD1UM09wsiOge7y8V1CzbK7slx7tkQ1flTQQ (Untitled)
  Session 2: mokgXfm_deB8_nvB7m2cxF9e2-pijcG8V-souy39OJNjzlaLN_fLpWv_EUrNc5TCBYqoJB7Gako8ntad_ThuOQ (Untitled)
  Session 3: 9lzCblLUBkxRFffds-OVpG9c64OgZjfh7QP7knhbj8J8Zy-tt4xi24dUxNlQJojSy0TRk1HH_cqAGuZ5sFlChQ (Untitled)
  Session 4: 0OAikv6tJLqRDgUuWi9Pnl56j2x8xE7bzmwiLTgU9RMca2eAvrP-r175vrCL8NMFU-sg7NLECYq90SvuxK--_Q (Untitled)
  Session 5: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg (Untitled)
  Session 6: EB3jEuzG-LxkynD65O1rXeJuvg8QIZkvVwPEHVz2k56v6PiRdnSL7oN61GeamNEYGSrhJJXdcEuHNcMobxBfPg (Untitled)
  Session 7: X9t5cttsehSZsRMNaQJaFNuP86HLbljgFO9jHItG45p3NzgjeeiOKF4tY97d975r8UT0y_xwUsC38ihAeBwYgg (Untitled)
  Session 8: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw (Untitled)
  Session 9: Vwv_dS-QtANBp_pTXuP31-g0Z56KtYJkKqzGlcNeqgBv68RCtjOiUv2gnGQKhO03ZeTUkAZZVX49FONaJsaS-w (Untitled)
  Session 10: ld9rVKp4aH4JOkUnFVYGKj4j8r2wHCUzMhFDp6pPQ_uKiRHIPwF3xlBjj_jGZnNHXzBvJCFoL7eFsx1FGxpNYw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 0dcPElXXqozuaR7GyPxnkvHWjPHq0m6it4SdEu5eI40L8MQ_ysBD1UM09wsiOge7y8V1CzbK7slx7tkQ1flTQQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'Ehk_CVjoj7BNideUXs3VgakYwDC7rsEEcUFUMr7QnZ5mrwFEY-0pbkXvvTtjRFDQTnWKMl-Ie5MkhBHfru0wkg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { value: 'Alice Johnson', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'Ehk_CVjoj7BNideUXs3VgakYwDC7rsEEcUFUMr7QnZ5mrwFEY-0pbkXvvTtjRFDQTnWKMl-Ie5MkhBHfru0wkg'
}
✓ Second upsert (insert): {
  id: '5qMgRQGFC3WLB87k-kyjWEaNuJFNLrzbG-DR5DbpcPSxw1F3OGF-twJ2NKFa2N1r2z5rsRXCGRPCnB67YMZbyw'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { type: 'Boolean', value: true },
  id: 'Ehk_CVjoj7BNideUXs3VgakYwDC7rsEEcUFUMr7QnZ5mrwFEY-0pbkXvvTtjRFDQTnWKMl-Ie5MkhBHfru0wkg',
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
   Inserted with ripple: {"id":"aNvVLsPmGEtG0T9miQl4NLiI5uqJEnjx7r5Ms-bHU8qS1xpM-oc1piwh2qviQ_dgrXgLAnL4KP3z1qhJWn1faA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"jPwtgFlNwpBdUDG3zuApWRR6WhWutzQYlj1jMjnRB_yatesA7Gmgct_ht1jp4HJEvYBGLsvJQnUz8tS2tDXHig"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"aNvVLsPmGEtG0T9miQl4NLiI5uqJEnjx7r5Ms-bHU8qS1xpM-oc1piwh2qviQ_dgrXgLAnL4KP3z1qhJWn1faA","price":{"type":"Integer","value":150},"name":{"value":"Product 1","type":"String"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"kxnE6nSdXfBtBP_3niB279pEJyzRDM1pVxUuDDePUwUsHfj37Vd0fTcCPXSB7aWY9iUdwxF8AMcEl0YIYNmgUg"}

✅ All bypass_ripple operations completed successfully!
✅ [32mTypeScript client examples complete![0m
