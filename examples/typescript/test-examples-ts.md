make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 981ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '1dJ8XeHjfDxZjp7DxeSN1ClymwTta71wZbPADZHVDyrpBKTB2oZ474mTYDrl-d0PV-sl9D4uq9hJ2QscRxZNdg'
}

=== Find by ID ===
Found: {
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  created_at: { type: 'DateTime', value: '2026-04-06T06:00:00.104Z' },
  active: { value: true, type: 'Boolean' },
  id: '1dJ8XeHjfDxZjp7DxeSN1ClymwTta71wZbPADZHVDyrpBKTB2oZ474mTYDrl-d0PV-sl9D4uq9hJ2QscRxZNdg',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  name: { type: 'String', value: 'Test Record' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  value: { type: 'Integer', value: 42 }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-06T06:00:00.104Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { key: 'value', nested: { deep: true } },
  created_at: '2026-04-06T06:00:00.104Z',
  active: true,
  id: '1dJ8XeHjfDxZjp7DxeSN1ClymwTta71wZbPADZHVDyrpBKTB2oZ474mTYDrl-d0PV-sl9D4uq9hJ2QscRxZNdg',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  price: 99.99,
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  value: 42
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  price: { value: 99.99, type: 'Float' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  created_at: { value: '2026-04-06T06:00:00.104Z', type: 'DateTime' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: '1dJ8XeHjfDxZjp7DxeSN1ClymwTta71wZbPADZHVDyrpBKTB2oZ474mTYDrl-d0PV-sl9D4uq9hJ2QscRxZNdg',
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: pnRsyj8hbCYq2jmH2bUOjBIGWjc-z6fIz7ebgxDqvx_q1_2VJmkuFN3ElKch0pdSOd2XjrvWOU1V4lz7hj5cXg

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
Collection created with first record: aP2eMSOss_DAGJgyr2HuF3AIHooJ5ORZc-dEmnoYXkOF2Sa0tpQ_mVSuq2UV8UhlSIaolk8k8bMvyM8E-3n3WA

=== List Collections ===
Total collections: 22
Sample collections: schema_employees_client_py,test_collection,products,websocket_test,schema_documents_client_go

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
  cache:product:1: { name: 'Product 1', price: 29.99 }
  cache:product:2: { price: 39.99, name: 'Product 2' }
  cache:product:3: { name: 'Product 3', price: 49.99 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

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
Created Alice: $1000 - ID: 19QqnNTFb6YdeTA22c4KYlSuVkr8vPfUkJjH5i8ABjnXSNTfYF6p2xEwoRAVhSI1oovu953gMCN9ItJrWdnlyw
Created Bob: $500 - ID: BuYdf4ZLwmwFP3FuROyK6SHJhVX7rBKOCyAES5m6ei_NgIA1eIOM2yK2ADrfrpgpkTKgPOZY4rMUNwZl_kMNWQ

=== Example 1: Begin Transaction ===
Transaction ID: 1a1e82d7-4a5f-4274-8f9e-0abf3a4dd459

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b4fba415-4deb-4525-af2e-d9c0bbcd4500
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
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title, bio.value, title.value, bio
  3. Score: 26.400, Matched: bio, bio.value, title, title.value
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: title.value, title, bio.value, bio
  3. Score: 39.600, Matched: bio, title, title.value, bio.value
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.744, Matched: 
  2. Score: 0.735, Matched: 
  3. Score: 0.722, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.697, Matched: content.value, title, title.value, content
  2. Score: 1.489, Matched: title, title.value, content, content.value
  3. Score: 0.294, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, skills.value, bio.value

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
✓ Inserted document: Ehh6I7iiCKCaRix8G3Ao8sR6ukv0fZ33UA8sFYgxaW2pZW8aMH22aSDuc6Snm4MZAiZxOePCbZyxZYXucSObWA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: G5nLKfjc3kby7I6vCiKVNUJcJza-h_3pwi3Zus41azkS0fGMx5RWKmJRVC_W00sLU9ZGvBSL0S5XPmqMA6U8vA

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
✓ Inserted document with TTL: RobTHiCMrAH88e8oP33hNcA2v-4EoklGOXKcK36nHKgCDw8QJ6PBkDyftOzP5hltaxMUaY8Io-Z-7BxAk1cSsQ

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
✓ Edge cache script created: XGqig4djMoFKMRQGaQYV-R6aKDKJgXfs-yU8ftDci2ASZ1tPm0AW7K2_FvdG4_aD-ohrb93o0bUUc-9bLuJy5A

Call 1: Cache miss (fetches from API)
Response time: 140ms
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "id": 1,
        "address": {
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "street": "Kulas Light",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          }
        },
        "name": "Leanne Graham"
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
Response time: 41ms (3.4x faster!)
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "id": 1,
        "address": {
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "street": "Kulas Light",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          }
        },
        "name": "Leanne Graham"
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: f31NmWgOggBH6AoXgE2yBnZvRtSK3Reym0-0XTCKMqHme8ykWRTjHnDb6Zp6gFTyhftprxZnaVzRpOPDFI8nIA
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: NoJbA0cSYgpjBY2ebTvx398OLq5yeHz09-3LAXIdX5rgDS9yNTy8mKFjLAAuyrJJeWfUZZapny4IpeFj6vWpcw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: _hA5jSI9KHmHC2SH9NzQc6z2wYm4A73J4xZfyZLz3ZVo1yYmnHSUFtemNloJ9s1woMI0W-sBQ_De_rl2vbYbLw
📊 Statistics: 3 groups
   {"count":5,"status":"inactive","avg_score":50}
   {"status":"null","avg_score":60,"count":5}
   {"avg_score":60,"count":5,"status":"active"}

📝 Example 4: UserFunction Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

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
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "city": "Gwenborough",
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        ...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "city": "Gwenborough",
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        ...
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: -b2KaCfOE68FjpHgXDyB4Bjt4DuDOPMWkw4q_-wrlRNmxx5xr5P0_Q37jRbCMB08tlb4_Rd7nMYBpNqgQtYraA
📊 Found 2 product groups
   {"count":2,"category":"Furniture","avg_price":474}
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"count":3,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: tVlS4eKAYbRoseoOP6wsKRLoIQAl0rWxNoFgCk0XT0xwUiukOzLr8dF9KQsbWgFesvibyQ1umkGCUuVEjnOh_A
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: 9HgrWsrQCdfbPBFmJ3FZhABuLWXj7q3nb7Ad4WqsSyIrvaVLXOkC4vXVT3jNbD2Tom89I2xzL-1Cb1sCoYwUQQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Lw7-UEd5hovALXKLam9Qqk1YJ97aa4_OZR7FBsTRMUHi-zVMy8-bmmzYsxPEoarprWxLvqAR8-CourIuXsmT2A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: PZAb2VWwXDFh2e0bGG8dBLLZlxtUWw5SnEmD1QZtoujJg3Rvd_2o7CMv0S6OjF5C4gH6ZJydJLEKWDTHJBCZpQ
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
✓ Created SWR script: fetch_api_user (4xgbCPmVnMCVkvukFGqHbUvZnjVvEQMfUSHD8IcoP5HWMgXKQ1DIiKxUgCEDAXZ9ADABaVLVWUYfCPv2boRFMQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "address": {
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "city": "Gwenborough"
        }
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 41ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "username": "Bret",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "address": {
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "city": "Gwenborough"
        }
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (NegEwI2V9ezlWP72NgRxt8tl5v0Fo-GECwNZWiotPbJ6w0cZ26PfMzNa0dJ20szn8uJxhQp9eX0iQPajJZ4Mvg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "availabilityStatus": "In Stock",
        "tags": [
          "beauty",
          "mascara"
        ],
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "minimumOrderQuantity": 48,
        "sku": "BEA-ESS-ESS-001",
        "category": "beauty",
        "reviews": [
          {
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Eleanor Collins",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "comment": "Would not recommend!",
            "rating": 3
          },
          {
            "comment": "Very satisfied!",
            "rating": 4,
            "reviewerName": "Lucas Gordon",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "lucas.gordon@x.dummyjson.com"
          },
          {
            "comment": "Highly impressed!",
            "reviewerName": "Eleanor Collins",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 5,
            "date": "2025-04-30T09:41:02.053Z"
          }
        ],
        "brand": "Essence",
        "dimensions": {
          "width": 15.14,
          "height": 13.08,
          "depth": 22.99
        },
        "discountPercentage": 10.48,
        "price": 9.99,
        "id": 1,
        "returnPolicy": "No return policy",
        "shippingInformation": "Ships in 3-5 business days",
        "stock": 99,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "meta": {
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687"
        },
        "rating": 2.56,
        "weight": 4
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

🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (ytHtJzp9KE6uyG_FoiVlE3ODzQakem539iL4ljFdxPic3Eb-Agw0UrcW7lTc-BlmLEI4b_MdLSWri9VNipFLKw)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (cpICA6y0kPmp_edA2Ja510Qes-ADtiP8FB-IGv3rJTZcIhDHp7mRadKKuXs4IkULILJIShCUgVCbaOEzHxdByg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (gpGpfDM3HsxlnXO5uh-07SrQjslD71iuSELv_REgNHtBOm-2LbGFn1UcmvncGHWhquLAPb1SA7zW8apQGmq_Kg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (hRgBjvE-DyF7JagV5DVPklewfXkqP5Zloz8XygM76U1egw1Jlc82uugooA4Qmy5L_-JQslY9t5PZ_7l8t2QMrw)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"count":5,"avg_price":367,"category":"Electronics"}
   {"avg_price":365.6666666666667,"count":3,"category":"Furniture"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They are optimized for retrieving similar items quickly using vector embeddings.
2. **Handling High-Dimensional Data**: Capable of managing high-dimensional vectors, which are common in AI applications.
3. **Scalability**: Can store and process large datasets, making them suitable for big data applications.
4. **Improved Performance**: Fast query times for vector similarity searches compared to traditional databases.
5. **Supports Various Data Types**: Ideal for unstructured data such as text, images, and audio converted into vector formats.
6. **Integration with Machine Learning**: Seamless integration with ML workflows, facilitating model deployment and inference.
7. **Real-Time Analytics**: Enables real-time feature extraction and analytics based on vector representations.
8. **Flexibility**: Allows developers to use various algorithms for indexing and search, accommodating different use cases.

These benefits make vector databases particularly useful in applications like recommendation systems, image retrieval, and natural language processing.
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
🚀 ekoDB TypeScript CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"count":3,"role":"admin","avg_score":20}
   {"count":7,"role":"user","avg_score":70}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
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
✓ Created session: g22PDETf2BZ5WIw_LqSlp9gnj_QGDSemITI-T1uCPOTIIGdgmZw56o1VRzKNQ2Qkcpgiw9n9EGd7Av72tuRBfQ

=== Sending Chat Message ===
Message ID: tr_TB91RCEYgrRz89iyLpE2w4auL5IAXKrKzMGWUm7zy0Us2SNil08BJhfDHtiNxl-8CBNCIG64v_UpnL0Lo_w

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'D55NMDKPDG1UBJxQZnaIH5fh_lAU5fVXr_iQ31ar9Eh8CsTsPOHP0FjxR-28pwdSWQ1sJEiyZcyts1URfn11AA',
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    id: '1ldj_JEyEll3prrRZ0hj1w_WC_iluyQpVT9MPa_SM5epY2ZLuHKmTJ3JpNupjZOShpYAadogMbg-FPLg_H_wuQ',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    id: '0ntc1OIar6n7qfFjR-k62IxEIQPM4LeWyuJqIJVhIGQUgIRV-olanDBgdJGW7QgsS0zI7xgq6qi612bNNYJeww',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 8785ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Ft92VBXuhIGfXW1XEtsYL7IB4ek7aNBP6vGen5Zf0b39zdZT7EN-xJhd1aBTKjQfk0Uhnz83j8Py9saKTBXRyg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you'd like to know more about other products or details, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: NcKy-_Uqx7KLOJl7eO9DHRmvf8mYRjOouyZWIz47G0-z83h9alj7KARdZjohQHAXz9PQI4Izg9uyrdYUXpIekQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 6

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 5

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hjv2vJ9ySHPM3JtRXgCRBxX2wCK5JRXeBOAbppTyYN2JQmYuXYVodT9BOlF_vVpqisOK7SlUT6SnflIhr12jwA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Price:** $99

If you need more details or have further questions, feel free to ask!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: rEpkYTZhtbhaeW-bwuNvEI8YXm6jSdOT7xJhQgZ2QtFYrwkHNqGWi1H4Ir0Wg4gFWD0QxDHwlnEBa-oLeLoAMA
  Parent: hjv2vJ9ySHPM3JtRXgCRBxX2wCK5JRXeBOAbppTyYN2JQmYuXYVodT9BOlF_vVpqisOK7SlUT6SnflIhr12jwA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: rEpkYTZhtbhaeW-bwuNvEI8YXm6jSdOT7xJhQgZ2QtFYrwkHNqGWi1H4Ir0Wg4gFWD0QxDHwlnEBa-oLeLoAMA (Untitled)
  Session 2: hjv2vJ9ySHPM3JtRXgCRBxX2wCK5JRXeBOAbppTyYN2JQmYuXYVodT9BOlF_vVpqisOK7SlUT6SnflIhr12jwA (Untitled)
  Session 3: NcKy-_Uqx7KLOJl7eO9DHRmvf8mYRjOouyZWIz47G0-z83h9alj7KARdZjohQHAXz9PQI4Izg9uyrdYUXpIekQ (Untitled)
  Session 4: g22PDETf2BZ5WIw_LqSlp9gnj_QGDSemITI-T1uCPOTIIGdgmZw56o1VRzKNQ2Qkcpgiw9n9EGd7Av72tuRBfQ (Untitled)
  Session 5: BRDQKPJhMQwbLExNC6cMPew3vnZGKnvPN9rVdxVMJv05Ub77NYPhR7cu400lI63ENpbFWpM6uXtsaJB2HOsylw (Untitled)
  Session 6: Mcj0ZDVNpmAiD_2ZVd1SYnOUyXJ8RUETAc5gHzjEQRV-t2Ymi8HhmT-2Dc4vU6hWs2Wrph9U6ZUUbepRBwby4A (Untitled)
  Session 7: SjEdAoC_Va1ggnLgMzZwEjIYtdkaslNGe94NH8jfoEQpkonVZWMoHaZUudJDWMa9cHYOfEbHclul1aZBzsu1wQ (Untitled)
  Session 8: mTCGfeh03CagCcNbWtbZuJ06U9VhaEvJ4cKQBYUiCjwZA5fTRD9jvGrreLB4SazgeKhUkofiI7cVALOqdWqq-A (Untitled)
  Session 9: _TqQUDru657FYIO91SiEMHfnut6HjfeL9twRb2M98HEWZSZRdWO8h9xS489XTUW1T74A59gtqftDW8GRxjPhWQ (Untitled)
  Session 10: reBocC0dkKKQT30ZXAe3Ad-0U_GUTjJE4CZZSEk9uDEvdg0lMMRdMGYBBHiUelD0Ms3X_RiwtAF15H2GzFu9Bg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: rEpkYTZhtbhaeW-bwuNvEI8YXm6jSdOT7xJhQgZ2QtFYrwkHNqGWi1H4Ir0Wg4gFWD0QxDHwlnEBa-oLeLoAMA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'Sf6B7ib4IjRsXvU4v1750lGU4njXTOo9JVbZf7k4pKciFbvH1zZt46VZNdONi8IrJ_DxyyspMKgBcSuS4sv60w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { value: 29, type: 'Integer' },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { type: 'Boolean', value: true },
  id: 'Sf6B7ib4IjRsXvU4v1750lGU4njXTOo9JVbZf7k4pKciFbvH1zZt46VZNdONi8IrJ_DxyyspMKgBcSuS4sv60w'
}
✓ Second upsert (insert): {
  id: 'SNdNNKHK6yaOR3mNP8Nw0LWH3YrC6G8GthIYJGbxIPpJxT-y2RPuvhg_FCMwe1OQFXB3-LNQPEJUx4GQYjzlqQ'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  id: 'Sf6B7ib4IjRsXvU4v1750lGU4njXTOo9JVbZf7k4pKciFbvH1zZt46VZNdONi8IrJ_DxyyspMKgBcSuS4sv60w',
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
   Inserted with ripple: {"id":"hjbC_nOQH0bTLLi81A6gslf3a_T9gf_G9qTFmPcXjnKDJm910YXzFeQWdgY64JEV7WA5fVICSs5mrVZrDAv9xQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"WRipbAymDv6kcwIIpLBeTbyu2KHvq41l0SQ3bjudlymL4VJJ2WrplivBlHp4Vk9WUOzH_IouLFkwHMAv5zT4eA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"hjbC_nOQH0bTLLi81A6gslf3a_T9gf_G9qTFmPcXjnKDJm910YXzFeQWdgY64JEV7WA5fVICSs5mrVZrDAv9xQ","price":{"type":"Integer","value":150},"name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id","name":{"value":"Upsert Product","type":"String"},"price":{"value":500,"type":"Integer"}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email","name","id"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age","id","email","name","status","user_role","bio","avatar_url","created_at"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - [object Object]: [object Object]

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["status","id","secret_token","bio","name","age","api_key","created_at","user_role","avatar_url","password","email"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","email","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-4-0613
    - gpt-4
    - gpt-3.5-turbo
    - gpt-5.4-mini
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - dall-e-3
    - dall-e-2
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-3.5-turbo-0125
    - gpt-4-turbo
    - gpt-4-turbo-2024-04-09
    - gpt-4o
    - gpt-4o-2024-05-13
    - gpt-4o-mini-2024-07-18
    - gpt-4o-mini
    - gpt-4o-2024-08-06
    - gpt-4o-audio-preview
    - gpt-4o-realtime-preview
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - gpt-4o-realtime-preview-2024-12-17
    - gpt-4o-audio-preview-2024-12-17
    - gpt-4o-mini-realtime-preview-2024-12-17
    - gpt-4o-mini-audio-preview-2024-12-17
    - o1-2024-12-17
    - o1
    - gpt-4o-mini-realtime-preview
    - gpt-4o-mini-audio-preview
    - o3-mini
    - o3-mini-2025-01-31
    - gpt-4o-2024-11-20
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-search-preview
    - gpt-4o-transcribe
    - gpt-4o-mini-transcribe
    - o1-pro-2025-03-19
    - o1-pro
    - gpt-4o-mini-tts
    - o3-2025-04-16
    - o4-mini-2025-04-16
    - o3
    - o4-mini
    - gpt-4.1-2025-04-14
    - gpt-4.1
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-mini
    - gpt-4.1-nano-2025-04-14
    - gpt-4.1-nano
    - gpt-image-1
    - gpt-4o-realtime-preview-2025-06-03
    - gpt-4o-audio-preview-2025-06-03
    - o4-mini-deep-research
    - gpt-4o-transcribe-diarize
    - o4-mini-deep-research-2025-06-26
    - gpt-5-chat-latest
    - gpt-5-2025-08-07
    - gpt-5
    - gpt-5-mini-2025-08-07
    - gpt-5-mini
    - gpt-5-nano-2025-08-07
    - gpt-5-nano
    - gpt-audio-2025-08-28
    - gpt-realtime
    - gpt-realtime-2025-08-28
    - gpt-audio
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-5-pro-2025-10-06
    - gpt-5-pro
    - gpt-audio-mini
    - gpt-audio-mini-2025-10-06
    - gpt-5-search-api
    - gpt-realtime-mini
    - gpt-realtime-mini-2025-10-06
    - sora-2
    - sora-2-pro
    - gpt-5-search-api-2025-10-14
    - gpt-5.1-chat-latest
    - gpt-5.1-2025-11-13
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-5.1-codex-mini
    - gpt-5.1-codex-max
    - gpt-image-1.5
    - gpt-5.2-2025-12-11
    - gpt-5.2
    - gpt-5.2-pro-2025-12-11
    - gpt-5.2-pro
    - gpt-5.2-chat-latest
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4o-mini-tts-2025-03-20
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
    - gpt-5.2-codex
    - gpt-5.3-codex
    - gpt-realtime-1.5
    - gpt-audio-1.5
    - gpt-4o-search-preview
    - gpt-4o-search-preview-2025-03-11
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - gpt-3.5-turbo-16k
    - tts-1
    - whisper-1
    - text-embedding-ada-002
  anthropic:
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
    - claude-3-haiku-20240307
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4-mini, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: vKk6BCPe4azCMTK_mbdc32hjdwAijTeh8FThykuar1hpVZkcrZ3hUr8ftKnKofuN2eLXfpmtVkdx7_Vvsau2cw

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - get_active_users_ts: Get Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_api_user: Fetch User with Cache
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - cache_api_call: Cache External API Call
  - fetch_and_store_user: Fetch user from API and cache in KV

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
✅ [32mTypeScript client examples complete![0m
