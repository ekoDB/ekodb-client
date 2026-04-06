make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 1s

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '4QU7TCx44hovZHDtPMXmUwYY8M21i_rl4xyn4wtoyb3Sw8NNQgHeT3kZoNzjUDnJ0WpyieTibCwILMmba__0CQ'
}

=== Find by ID ===
Found: {
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { value: 42, type: 'Integer' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: '4QU7TCx44hovZHDtPMXmUwYY8M21i_rl4xyn4wtoyb3Sw8NNQgHeT3kZoNzjUDnJ0WpyieTibCwILMmba__0CQ',
  price: { value: 99.99, type: 'Float' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  created_at: { type: 'DateTime', value: '2026-04-06T03:52:46.178Z' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-06T03:52:46.178Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  data: 'aGVsbG8gd29ybGQ=',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  value: 42,
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  active: true,
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: '4QU7TCx44hovZHDtPMXmUwYY8M21i_rl4xyn4wtoyb3Sw8NNQgHeT3kZoNzjUDnJ0WpyieTibCwILMmba__0CQ',
  price: 99.99,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  created_at: '2026-04-06T03:52:46.178Z'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  active: { type: 'Boolean', value: true },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  price: { type: 'Float', value: 99.99 },
  value: { value: 100, type: 'Integer' },
  id: '4QU7TCx44hovZHDtPMXmUwYY8M21i_rl4xyn4wtoyb3Sw8NNQgHeT3kZoNzjUDnJ0WpyieTibCwILMmba__0CQ',
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-04-06T03:52:46.178Z' },
  name: { value: 'Updated Record', type: 'String' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 4tBqGs3Q9P0WnmIMFO3zbXDAUbvS1R-crIJHhpQzfIoPh0YRV4O7VxgUQfollWv9toJPNP45HAvq_9y3k28hBQ

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
Collection created with first record: rMhromTnb8yJuC_45LIvqVpEDt94EbLPyD4gQGjI-SXnXbGzhxnqY5uuMhnyyLgmwiUV3rzV8qaEdr71BP-1Ew

=== List Collections ===
Total collections: 47
Sample collections: audit__ek0_testing,cached_database_metrics,schema_products_client_go,enriched_users,schema_employees_client_py

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
  cache:product:1: { price: 29.99, name: 'Product 1' }
  cache:product:2: { price: 39.99, name: 'Product 2' }
  cache:product:3: { name: 'Product 3', price: 49.99 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 16

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
Created Alice: $1000 - ID: YbxBc26KgKObajyFaKpJ4lhse-iz0ymr6VR0AaT2FeYubDvkd0VvYcTlVSa0rCeRM9tZji-jFe11HlafEb-jxA
Created Bob: $500 - ID: 8j7NvhHUs5x_2e3j5GbP1NpDlz6AJyRlCEvZ8KzFB3whcNkSz5kJXHdkT75RgUfEXH-NGmKcZUtQhW0Wcd676w

=== Example 1: Begin Transaction ===
Transaction ID: 285681d6-a30b-4970-b2d0-a14ba58b8069

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f5d23281-9d21-4388-a759-24ab5db854b9
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
  1. Score: 25.740, Matched: email.value, email, name, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, title, bio
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: bio.value, title.value, bio, title
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.753, Matched: 
  2. Score: 0.749, Matched: 
  3. Score: 0.746, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: title, content, content.value, title.value
  2. Score: 1.501, Matched: content, title, title.value, content.value
  3. Score: 0.300, Matched: 

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
✓ Inserted document: 2T734UIfqgGql83KkbPuppfYc1ISat12ZDJexQlERg0DHnVtvlT4G6GGDSvBEjrBNwyKhGrNNUT4xmx_C4Ttsw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: EA2EWBKEGwJ839BN_JvW9wmG797UOh4vil6ZTM4mBa4YyJ-AAF4jd-StTQyFPWqT1k2s_5MsK2RyH6VYqnXulA

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
✓ Inserted document with TTL: nhOR0zcjhnJgCqfLrV85onDP8IH8j3cqmWt0w8K7-p3ozFhMkQd7lkTNAH1zFS27O35Y4ruk8Re-AZWdwdfS-w

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
✓ Edge cache script created: dRrjLP_W22J7hwvwcxjX3JR_1C7o52Mg216hOdtK9RzFh93zISsi9aRlRCWHMj8UXHb6hZdlS-K5YE1s7CKx0Q

Call 1: Cache miss (fetches from API)
Response time: 149ms
Result: {
  "records": [
    {
      "value": {
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "username": "Bret",
        "id": 1,
        "phone": "1-770-736-8031 x56442"
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
Response time: 42ms (3.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "username": "Bret",
        "id": 1,
        "phone": "1-770-736-8031 x56442"
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

✅ Function saved: dTbiz0gYeHSDVZbBP96Fa11TXD3t29j53ObozT_VBvbLR3LFxwHUkMdSiUW2rW_Itq1rUAaNbx9VAR4MKLozNA
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: EmQgv4LwyFTIdCFx-qkwa_X4I7sBpm6EYRy7sjNNtRL1cWq7m66qlxvm9r9Rd2mZIZIAF-vVNyR_ATPhqkt6oQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: LbTDpjPzvP3-lhuYrt5s2V5xB_dGiKfogSl91VqETWRB-05AoDT-O0qB0GEFXFcSTXr0T8orA8NBg26G-928Bw
📊 Statistics: 3 groups
   {"status":"active","count":5,"avg_score":60}
   {"status":"null","count":5,"avg_score":60}
   {"avg_score":50,"status":"inactive","count":5}

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
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "phone": "1-770-736-8031 x56442",
    "email": "Sincere@april.biz",
    "id": 1,
    "name": "Leanne Graham",
    "address": {
      "geo": {
       ...

Second call (cache hit - from cache):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "phone": "1-770-736-8031 x56442",
    "email": "Sincere@april.biz",
    "id": 1,
    "name": "Leanne Graham",
    "address": {
      "geo": {
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

✅ Function saved: 8PuhnZZwqaV9E5dQYBQRaNDUDWuBz3GG4FAg6lbd_qUUlB0RRtPEKttVYD4rIarSf2L1RWfZm2k8SrJoSzb0hA
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"category":"Furniture","count":2,"avg_price":474}
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

✅ Inserted order: 6EzTRMvpSMKDmPwWAvSyaS7RMMqyeSPO5kz8-zraAhv8UImMs9piBqee0vnPCivbIFCdL8uWXfsJ1WFulf7O3w
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: wlaUPvAxZftBqEEb-slwcBQDQZCbQ5LHlIS1PvCv1HdQtgJCQz_dA3bV2pZI3DLS4b5FYMTmcUIGB_b3LhQ2vw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 2HYdK_PpA1YdZBKsC0gN1lubTx0VsqBUod3HYFe4u2B9OfmAp2BNbAl0j7bJez8RA-oN4DFi3TfjPMa0XbJOtQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: nRCDRQyow8s8V-qhzi24mQ-yiHd4cvaKeI0kpUtwMnrlk2wPw2dF_SIfmZQ92uqg6l-EcC861x4GUHQRG-8J_Q
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
✓ Created SWR script: fetch_api_user (vWY53V5nfh8IqKgDXN52H5rPDC2VIgA8y2EraopflYyMMNNMjkvHDCEu1rK3zOHIIACk6K1CCMLYflHOt7gn4g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "username": "Bret",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
Response time: 43ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "username": "Bret",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
✓ Created enrichment script: fetch_product_with_reviews (knK0G8Qj3D9wxM2fHBeBQsFSknTTcf4HqjumQMYsQeqF07NCWge8R3mPlU3T3sCxdiJ3VawjsDc_1P7a4FMA2w)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "shippingInformation": "Ships in 3-5 business days",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "tags": [
          "beauty",
          "mascara"
        ],
        "weight": 4,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "stock": 99,
        "sku": "BEA-ESS-ESS-001",
        "discountPercentage": 10.48,
        "rating": 2.56,
        "id": 1,
        "brand": "Essence",
        "availabilityStatus": "In Stock",
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "rating": 3,
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "comment": "Very satisfied!",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "rating": 5,
            "date": "2025-04-30T09:41:02.053Z"
          }
        ],
        "returnPolicy": "No return policy",
        "meta": {
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687"
        },
        "minimumOrderQuantity": 48,
        "dimensions": {
          "height": 13.08,
          "width": 15.14,
          "depth": 22.99
        },
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "price": 9.99,
        "category": "beauty",
        "warrantyInformation": "1 week warranty"
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
✓ Created native SWR script: github_user_native (aHKJzT6T8qWK8a8g3Lcbo_G02RudXoOCoaAh1Rt36gnX6s1oF3OirWFoM4nQNFAnyv7l2A4qXoWlY5WOP_fIRQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 17ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 18ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (j5I8GhIOV4NoTt4AoQANXTFeAraT4BBmMQZwajFva1Ypk07JmDjLB4sIDd1u5rThxbQghJEba86T8kKXcN8d7w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (qXbso1pauPbjr4Gq27joLo-wsvA0S3KNc8lGhlrCt4EjiEsPshxqk8h7EHmG3BPFwQXYiM_8fLb23DXRodNAWA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (OEj1xqf6BE-MVmcWDm-QkEbRQFej7zYOKKoTxVN9nspDi2RhY4zrOuK0YXsu1XdIXO9qDnNovJ1obYJsBWNL3Q)
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
   {"avg_price":367,"count":5,"category":"Electronics"}
   {"count":3,"avg_price":365.6666666666667,"category":"Furniture"}
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

1. **Efficient Similarity Search**: They excel in querying high-dimensional data, allowing for fast nearest neighbor searches.

2. **Handling Unstructured Data**: Vector databases can store unstructured data like images, audio, and text in a format suitable for machine learning.

3. **Scalability**: They can efficiently manage large datasets and scale as data grows.

4. **Real-Time Querying**: Support for real-time analytics and updates, making them suitable for applications requiring immediate results.

5. **Support for Machine Learning Models**: They facilitate the integration of embeddings from machine learning models for better data representation.

6. **Enhanced Flexibility**: Ability to combine different data types and formats, enabling complex and multifaceted queries.

7. **Improved Performance**: Optimized for vector operations, providing faster performance than traditional databases for specific tasks like recommendation systems.

8. **Advanced Analytics**: Facilitate deep learning applications and other advanced analytical techniques.

These advantages make vector databases particularly useful in AI, machine learning, and data-driven applications.
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"count":7,"role":"user","avg_score":70}
   {"avg_score":20,"count":3,"role":"admin"}
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
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: NywLVYKnaHEA4WfOYsmM9dY1oavQ8hnEq8Dodp6wkPp3PUoHYtYQTghQjokz331UmUU3qkT2VrZ7j6Wtw6o1qg

=== Sending Chat Message ===
Message ID: Bbyu3xlsyt0VU1Wg-dtfsuusdG0oHBawP8p8K1UpNfSrtqtZndYjtuPLUXHAwfjbIsm0eUVak3f64MGu4y-6mw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'r0arQC9gqPhgLpp-XNfI3-YH8IY9jl6IHhn45WdOJ3Yz8ULeG7y2W4gDlrlVDB0LQNg4eoTC6-u8XyddAQ1teg',
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'mqpGv-mT2EN-mucQD1rJbs0PfjEQGTI2NswnWXjVITq1c289DsAvbbUikTdCT77Ea9X2Cprq0tmdb_yYzIgHig',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'jowBaOHxPzdCPWg4mQNTO078MkMIK-KHQ6beYpeDIy_FpX0CKTKKkVdYXaSNuK4ZXOyCcJuSTFWJEhuudYPHOA',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2116ms

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
✓ Created session: zma8OuCYPgYiDltVwdyLwErUr96wKzJZuD9Z_dbjXGX7EqEdXk-kaNT-B5CRlAEKhVGlOZ-VozoD4SU2CU4Fyw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or details about other products, just let me know!

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
✓ Created second session: p8s1qPSKPwGUe4GBfg6clOWiJdpIODgQAG5XkF0F0iSmDTfY_Omh2ocmr_ZcLYDggYpFsqMAy6LmFs5Qi-Zv3w
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
✓ Created session: FR9F_qSfuVyQ4TdxyeyUH62vek2rS32PNm4TEei_tYVSi2403Te85WeB2e8oMlCh0cZQe4SqPTeUlpiKvzJTvg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, just let me know!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: NJjql1Ff67C9ThiAtEMzrVEHfou-YaKjYZnEk7WhecrX3JvsDS-rHTOPYsnBClMjkkrvylHFrBrLZ4ZxLneivg
  Parent: FR9F_qSfuVyQ4TdxyeyUH62vek2rS32PNm4TEei_tYVSi2403Te85WeB2e8oMlCh0cZQe4SqPTeUlpiKvzJTvg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: NJjql1Ff67C9ThiAtEMzrVEHfou-YaKjYZnEk7WhecrX3JvsDS-rHTOPYsnBClMjkkrvylHFrBrLZ4ZxLneivg (Untitled)
  Session 2: FR9F_qSfuVyQ4TdxyeyUH62vek2rS32PNm4TEei_tYVSi2403Te85WeB2e8oMlCh0cZQe4SqPTeUlpiKvzJTvg (Untitled)
  Session 3: p8s1qPSKPwGUe4GBfg6clOWiJdpIODgQAG5XkF0F0iSmDTfY_Omh2ocmr_ZcLYDggYpFsqMAy6LmFs5Qi-Zv3w (Untitled)
  Session 4: NywLVYKnaHEA4WfOYsmM9dY1oavQ8hnEq8Dodp6wkPp3PUoHYtYQTghQjokz331UmUU3qkT2VrZ7j6Wtw6o1qg (Untitled)
  Session 5: NDeOBVKsMCqOpcQHsNmV96Nji7V4MJEHYbKVcf_YfpvNMO5mcJIO19Nb_3ljx4BrrIZG572g5PizSy-vllMSSA (Untitled)
  Session 6: _DgimDs70IFjtBMQiEuIgkb4BNyDwHu4ZzjUhOnlzAxAeCdSKVpiyLbaB8_hA-OsMxz_HPNaHeYBLDnawpQd2w (Untitled)
  Session 7: j35FSFi7T5MNfApTG_tS_0nktaY5bNDTaUm9_Oh1NZq0crz26C65MZINFB_tBRD96KcT1EvDsvfv9PqQglIKyg (Untitled)
  Session 8: Lgj7ao7hej-PLDxXRWbB-gJVVaO_BmS1m4YQMX35qgSJrEXyZNFjQFzsDIw6bWYPKPhfAxANp1-b36KjoJu-yQ (Untitled)
  Session 9: YP5qswPHkbIDIVGAfdU0RmffBHKwrpZl6ZFKCk3xKPLWYOAHHg_fQI5ac-GqelJrYv1Wo87jFj0o7a-KiObgVg (Untitled)
  Session 10: DxcKnNVdBVFO5renQijv8LByj3cDksxCjDxl7-r5_dvAptNWkD2LgUn-bgrVymKRhkRW2wL_vJ1x5VFVI6VPHw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: NJjql1Ff67C9ThiAtEMzrVEHfou-YaKjYZnEk7WhecrX3JvsDS-rHTOPYsnBClMjkkrvylHFrBrLZ4ZxLneivg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'pgWuRKsch94VuJSn02EZNHGwNGSFSF8E5aSqLCYu0e29X_BtdlDhb5cd3fIBMf_pcEaCpFuMsMpjV-T-RJ62Wg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'pgWuRKsch94VuJSn02EZNHGwNGSFSF8E5aSqLCYu0e29X_BtdlDhb5cd3fIBMf_pcEaCpFuMsMpjV-T-RJ62Wg',
  name: { value: 'Alice Johnson', type: 'String' }
}
✓ Second upsert (insert): {
  id: 'Jfra5ReiSc6gxLmfIT_v-oMf0Rkjsz4Vpt6Gx8JjPdlBU-WrqySi0puk3RuoFz2J3dpUcgb0CU9X1G4mVy2iqA'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  active: { type: 'Boolean', value: true },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'pgWuRKsch94VuJSn02EZNHGwNGSFSF8E5aSqLCYu0e29X_BtdlDhb5cd3fIBMf_pcEaCpFuMsMpjV-T-RJ62Wg'
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
   Inserted with ripple: {"id":"I4_OIe-7hh8nRnC7z8D_Am5pAiENpvFJmkSM9YjJD1C4vdCRpEZdmTefs-nbRds1VgyB8XrFqxpkPI-qeBVu_A"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"0lAK0_dnb9laQ8bFBhb7oqufQHNVjxMt3HczSuNiGzUWw8Rx-FB6i0YQQjzatxKqtUlrIaC1hS84_bcNTAsWxw"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"name":{"value":"Product 1","type":"String"},"id":"I4_OIe-7hh8nRnC7z8D_Am5pAiENpvFJmkSM9YjJD1C4vdCRpEZdmTefs-nbRds1VgyB8XrFqxpkPI-qeBVu_A"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id","name":{"value":"Upsert Product","type":"String"},"price":{"type":"Integer","value":500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name","email","id"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["bio","id","email","created_at","name","user_role","age","status","avatar_url"]

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
    - Fields: ["email","password","bio","status","api_key","user_role","secret_token","created_at","id","name","age","avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","id","email"]
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
Created user function with ID: JuUMUj1owtQUWe_2rFWZLZm1L6CPZbeHnyQQp838UDL3ffY0djPLEsbzfOrwtEaJHzstUmQmmBF70GLCT-dOOw

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_api_user: Fetch User with Cache
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call_py: Cache External API Call
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call: Cache External API Call
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_active_users_ts: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user

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
