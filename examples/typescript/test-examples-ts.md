make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.19.0 prepare
> npm run build


> @ekodb/ekodb-client@0.19.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.19.0 build
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
  id: 'PR5eiXWg_Mt7czn3bMEq5OYHgmqH5u-dRQy7BzVawImqWxeZXWTuwOX6CvXbR1HUvTvtDksq1hl_1Bo9lS6OeQ'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  id: 'PR5eiXWg_Mt7czn3bMEq5OYHgmqH5u-dRQy7BzVawImqWxeZXWTuwOX6CvXbR1HUvTvtDksq1hl_1Bo9lS6OeQ',
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  value: { type: 'Integer', value: 42 },
  created_at: { value: '2026-06-03T12:05:31.052Z', type: 'DateTime' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  active: { type: 'Boolean', value: true },
  price: { value: 99.99, type: 'Float' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-03T12:05:31.052Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'PR5eiXWg_Mt7czn3bMEq5OYHgmqH5u-dRQy7BzVawImqWxeZXWTuwOX6CvXbR1HUvTvtDksq1hl_1Bo9lS6OeQ',
  categories: [ 'electronics', 'computers' ],
  value: 42,
  created_at: '2026-06-03T12:05:31.052Z',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  active: true,
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  metadata: { key: 'value', nested: { deep: true } },
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { type: 'Integer', value: 100 },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  created_at: { value: '2026-06-03T12:05:31.052Z', type: 'DateTime' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: 'PR5eiXWg_Mt7czn3bMEq5OYHgmqH5u-dRQy7BzVawImqWxeZXWTuwOX6CvXbR1HUvTvtDksq1hl_1Bo9lS6OeQ'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: W5AB1ohCHqKT_AxIBUoyEBe200w8xPo49y2kgyVKMMilXczEU2n64Hed75oUGQxQkxbXjZNZXTrV_JEREnl3PQ

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
Collection created with first record: y04O2KFLPAzXf-J9rNoPO1IP06S7-d_U_-38sWJWwUUuDroFaQO1awLNxyLiW2V4Fqg02KBoKQOv5sVY-VRMiA

=== List Collections ===
Total collections: 23
Sample collections: batch_users,chat_messages__ek0_testing,test_accounts,client_collection_management_ts,schema_employees_client_go

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
  cache:product:1: { price: 29.99, name: 'Product 1' }
  cache:product:2: { price: 39.99, name: 'Product 2' }
  cache:product:3: { price: 49.99, name: 'Product 3' }

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
Created Alice: $1000 - ID: sVKeURfJ7SKKyXn-uM8T6fFrvgA6KUQoqEwiSXWpK30jmQnlbWfCbhkjFXOYxF9oFzt5HuTyIXWLmcokziDP5Q
Created Bob: $500 - ID: izFANuj_kgzBm08KG2oPfBLFdmeBlm9KxdnqAoG4Fo6lfIELTVy1ZuoEVWDASGZvKczF6MTGJmbUCrA1WD2BQw

=== Example 1: Begin Transaction ===
Transaction ID: 49d85220-cc24-4433-a7e8-d351d8f50179

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: e99c8d5a-bf7a-48d0-b5ae-fa12e41fe9f3
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
  1. Score: 25.740, Matched: name.value, name, email.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: bio, title.value, title, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, bio, title.value
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.789, Matched: 
  2. Score: 0.750, Matched: 
  3. Score: 0.734, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.716, Matched: content, content.value, title, title.value
  2. Score: 1.494, Matched: title.value, content, title, content.value
  3. Score: 0.300, Matched: 

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
✓ Inserted document: 2B_uWkZ0Rv7cxiMMjTQc_pwwZ1h2T1qTgZq-NCH_2SNHqdfHWs8nO8eqsmWR1SKqlri9jMOfw2BNfeTgSv6dLw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: FVbWvUGK5SZgLZkQheSejf2mO-fVyxurXQanYWoWBMSq6or4dQ6wJFX-SdbH-XkFueSsXGPQ1TvI-yWkdX5h_Q

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
✓ Inserted document with TTL: MgCj4RmqyzYl_yXw1N5OHl84N83QpOMTWfofsp0EPIsszCHAOcT2bVAEsPSew_l5nclOib0ngIb_tr-GxK-Y3g

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
✓ Edge cache script created: gyeKhM1BBwv0wsC8YdT7j-Ds2XGc2Wx5PPYaLI9bqkfGJfjk0elBRinsdWNQGMz_MHPhALXTGdX18UqOkNDsxQ

Call 1: Cache miss (fetches from API)
Response time: 196ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "address": {
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          }
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
Response time: 43ms (4.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "address": {
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          }
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

✅ Function saved: lKtPrALzqj8YUDHy6Xd5Q1Yjc3uVFGHgNpA1xN1VmYeKa7oQacLX7_yuonrKvZvVklkiFRKXi5JRZfefoifgjw
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: 4yOWKlzWGWm4URp1Gk07U5uPBdWyW9aJrhzLlSoZQjkGUEoYH9_gVar0mSFFSpMknaLijd7fAvR772-bKQoj_Q
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 21iZ2vw_IOxoFzPANxA60gQr1tdYE8nqp5Ux2T2YI7JwDvAkI_eEfvCFZydZelGWu1iiuaOn6xD1cQD_QNziQQ
📊 Statistics: 3 groups
   {"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5}}
   {"avg_score":{"type":"Float","value":60},"status":{"value":"null","type":"String"},"count":{"type":"Integer","value":5}}
   {"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60}}

📝 Example 4: UserFunction Management

📋 Total scripts: 12
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

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Ro...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Ro...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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

✅ Function saved: YGG1B7mUC-fx7oGvq0Ic7dHVYt7b87FsxNVhibU65s1hXpqhzMLsumxLbeNZfJokuKsTJGqbmy0MpSt-3BDkig
📊 Found 2 product groups
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666}}
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"value":474,"type":"Float"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
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

✅ Inserted order: _KuSFQH6gwY0em4tiSeRe4bz3FfAD36B2qtKpue-Ag69TxPK97HkbEfHzwh2X7Wb7QCmlZSbWf7EMyTzng67Kw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: VfVDl2O82XgZG77p-MvloLz_taWIwMwYewVJvrPAh8RG_5Ll8v7qnDmc9GARoqMh0kk-hG0JqbQMcH_MxssabQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Ki0OemOvGNDQJ6lTB6_zuidnS9ftkKexQmkIa4YlLK7szy6fhRU5EJzr_RLUcck3P-ZggYEi5FNWhCVUDdHxeA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: gUlAk1oykSzbQoxYHiLcBs4zrFaPF_e7T3053mK4TauBG681a-v0LiFlsBvyij_9IGluzJsrJjmCgw7EqVVpXA
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
✓ Created SWR script: fetch_api_user (2-n3V8PShpxiSktAyPHGbO_FFDyBsa6oGpOYuKudRi4ndbhg8bpxLftrYFB0I5z7nZfm-Dnl6J8L41cCR0uL2Q)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "id": 1
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 50ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "id": 1
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
✓ Created enrichment script: fetch_product_with_reviews (S5v4-U4Ux2XpSwfVge4bPft3X_qVcIaDSthRNvmLJxh0cUhkfPJNhN4bNMW6cvJoy3tqgbKo4JptnuFemrK-UA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "price": 9.99,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "minimumOrderQuantity": 48,
          "rating": 2.56,
          "shippingInformation": "Ships in 3-5 business days",
          "warrantyInformation": "1 week warranty",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "category": "beauty",
          "brand": "Essence",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "dimensions": {
            "height": 13.08,
            "width": 15.14,
            "depth": 22.99
          },
          "meta": {
            "barcode": "5784719087687",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "createdAt": "2025-04-30T09:41:02.053Z"
          },
          "returnPolicy": "No return policy",
          "availabilityStatus": "In Stock",
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "reviewerName": "Eleanor Collins"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon",
              "rating": 4,
              "comment": "Very satisfied!",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "reviewerName": "Eleanor Collins",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!"
            }
          ],
          "stock": 99,
          "discountPercentage": 10.48,
          "title": "Essence Mascara Lash Princess",
          "sku": "BEA-ESS-ESS-001",
          "tags": [
            "beauty",
            "mascara"
          ],
          "weight": 4,
          "id": 1
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
✓ Created native SWR script: github_user_native (xeeCCml5RjyDw2mBhutYJAybJEcSXotHi_Hy2oeW6o-xKL4i7-DtlfNrOGY6QMsUjMb9RDThEx2kqCuGQOTcmA)

First call (cache miss - will fetch from GitHub API):
  Response time: 40ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 18ms
  Speedup: 2.2x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (butte0bFQBa4NsaqvDsEiFR_hl7Clxhvj8nBQzvMxithuNIAKEz9_-KaPhInMl-mphtJ4MrmxHY45OSzo7NWRQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (TkKNUh3tc0qhmqFlZw83Exi1QsosBlaPtN9yhU6x5c890lKVO863zBiiDJDpeVl_xRlY0sDdPLifQkoLQ4Qitg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (80Iql-EWWMYTdNvxa4qoOg-bugB4SQ03vWf6O9MIuGOsz1Bu6HPlg-UqwNfUk17NsLJvFoxRrBUNnt2fyN_TuA)
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
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"value":365.6666666666667,"type":"Float"}}
   {"avg_price":{"type":"Float","value":367},"category":{"value":"Electronics","type":"String"},"count":{"value":5,"type":"Integer"}}
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
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel at nearest neighbor searches and can quickly find similar items based on high-dimensional vectors.

2. **Scalability**: Designed to handle large datasets and scale effectively, making them suitable for big data applications.

3. **Performance**: Optimized for handling unstructured data, providing faster query performance compared to traditional databases.

4. **Flexibility**: Support various data types, including images, audio, and text, by converting them into vector representations.

5. **Integration with AI**: Easily integrates into machine learning and AI workflows, enabling natural language processing and other intelligent applications.

6. **Real-time Analytics**: Capable of processing and retrieving data in real-time, which is essential for applications like recommendation systems.

7. **Dimensionality Reduction**: Often incorporate techniques for reducing the dimensionality of data while preserving relationships, improving overall performance. 

These benefits make vector databases particularly powerful for use cases involving AI, machine learning, and complex data retrieval tasks.
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
   {"role":{"value":"user","type":"String"},"avg_score":{"type":"Float","value":70},"count":{"value":7,"type":"Integer"}}
   {"avg_score":{"value":20,"type":"Float"},"role":{"value":"admin","type":"String"},"count":{"value":3,"type":"Integer"}}
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
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"AI"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Database","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: D7wAj80NgvEr9Yc0d-mLl5Skucba9uJrNch064lxMheD90lwkgeRsFS9r1jhx8md_K3UzyGWYFg_G_jQR7K29g

=== Sending Chat Message ===
Message ID: sag0QcWIg1AKC-QSPE5FUyQQ9wceK3BmNEoH26Z8IxED7o1UN2flXJnBDz0f8ViRZcwykQosTdqv1CZjg4agJw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product

2. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features

3. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'EcNSMxuAf59kQR4WHNO-OaR1tM_OrfUaYFjjq0anYrZ50xTDHgYhG7m1LD2aTwl4mlfjBD8vvriGRn-Fn2p23A',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    id: '1GWg7TGJdn5y8by3Ft57g0udwvpwzvcjNDzGpj1BRWiuiPxvGPBEJ2zXKNYJvEjith_8yeDvK8_GdQIBMni_2A'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: 'pc6lmX4i_KSLoRuHMs2mhoE5D4AXPIe3oDE-TjI8d9p9hwK_C36SS6CeiLzXuMnFsXxEQVngC8a9VDLBhRZ57w',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2445ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 82
Total tokens: 3550

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: zuAq6B56OC5DTKN5Sx6FphlfdptOsUTg1M2_LIpZzyKqdYEI0w-USP6I_YxrVx2wLpYBWPTzILpQDqjgP_qeYQ

=== Sending Initial Message ===
✓ Message sent
  Response: There are currently no available products in the database. If you have any specific queries or need further assistance, feel free to ask!

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
✓ Created second session: hw1YktkqKM6M8gKVwjWqYkkIYCLA6jPxYMi65XqHMVWNY_sMHxyMNQuhANhhRcrmdjNBaZwZlbRmg4s6NTqNYw
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
✓ Created session: Lsf8xYElZqDSv-MPOr4tSu4CTkOgnVfrcL5WXhD5AHA1q5a-aVGXzad1znA7g7zDT0Z5N_WZRul_s6oS26ViyA

=== Sending Messages ===
✓ Message 1 sent
  Response: It appears there are currently no products available in the database. If you need further assistance or want to explore something else, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: LzveWEMksOLoMxfxD127DJzTUnverosgx9I8lElv3wYfPJyTK0UqiVJhsgxTcA5ncoZRFjz5eVu-T9HGBZXA2A
  Parent: Lsf8xYElZqDSv-MPOr4tSu4CTkOgnVfrcL5WXhD5AHA1q5a-aVGXzad1znA7g7zDT0Z5N_WZRul_s6oS26ViyA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: LzveWEMksOLoMxfxD127DJzTUnverosgx9I8lElv3wYfPJyTK0UqiVJhsgxTcA5ncoZRFjz5eVu-T9HGBZXA2A (Untitled)
  Session 2: Lsf8xYElZqDSv-MPOr4tSu4CTkOgnVfrcL5WXhD5AHA1q5a-aVGXzad1znA7g7zDT0Z5N_WZRul_s6oS26ViyA (Untitled)
  Session 3: hw1YktkqKM6M8gKVwjWqYkkIYCLA6jPxYMi65XqHMVWNY_sMHxyMNQuhANhhRcrmdjNBaZwZlbRmg4s6NTqNYw (Untitled)
  Session 4: D7wAj80NgvEr9Yc0d-mLl5Skucba9uJrNch064lxMheD90lwkgeRsFS9r1jhx8md_K3UzyGWYFg_G_jQR7K29g (Untitled)
  Session 5: n_Aag-iezC8kDJvkKeosutVyUStsN3sDN3_ovArhjj10xZEKa4Ua8wr1_QKx02Vi8tRsY3sv8qIa3-RnqPl60A (Untitled)
  Session 6: w_72DfAp1ZSzXbQAedA_nPr27WvQ1no3ZuxJy-G52J1Wrxdqv1IWunC5BDjmu7daUXqM47AZ4VhqkgIEGeTCWA (Untitled)
  Session 7: 8lOgsFKTJ7YMWW61K9ANczXpSqKs60HCnKVdsVuYz6YWuAAP8V1vHEimiBeo4HFBFwkAF5g45Tfkxp-lmjSAUw (Untitled)
  Session 8: e1NS_GYOMBjlALR7qkoESKXcY-BTdqxCm3wyofu77RyD7pV_Wi4LxTG5m5Vt5zpztLo6DtJN9j6PqR8fCdhI8Q (Untitled)
  Session 9: qEYTMhOJ3BnVMernsVx3QgJTF1LKEhtCTXH2ZdCpGzrdmCWlV1aqUGI0mkCvWmB03YcGHlwD4Q3nQIScY8VIvA (Untitled)
  Session 10: j_XDtCmWdHnMpCXaTQqd2qIC8s6J7_UtisbJJEVwbFmRCIMhA4hPDyYNnlNp2y7ZPuI7akpbsMcOKY0Ll75CyA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: LzveWEMksOLoMxfxD127DJzTUnverosgx9I8lElv3wYfPJyTK0UqiVJhsgxTcA5ncoZRFjz5eVu-T9HGBZXA2A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: '-sZBwzwnmHgZpWz2PJyoOtdRRi5-cQ2QgEJi74A8Ct2WrzabVH80rrwQ94vJy2N8uRaUQ-q3rtoqdvnSNPyq5Q'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  id: '-sZBwzwnmHgZpWz2PJyoOtdRRi5-cQ2QgEJi74A8Ct2WrzabVH80rrwQ94vJy2N8uRaUQ-q3rtoqdvnSNPyq5Q',
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { value: true, type: 'Boolean' }
}
✓ Second upsert (insert): {
  id: '0aUthc4kMx_pMlwhjxAI9UMOn68inPAYy5AivTJy6SQPJZI7iTBCuDN3g4BiCO3yJ5-ZgRI-pAfvtN9f-v5Rhw'
}

=== Find One Operation ===
✓ Found user by email: {
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { value: 'Alice Johnson', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: '-sZBwzwnmHgZpWz2PJyoOtdRRi5-cQ2QgEJi74A8Ct2WrzabVH80rrwQ94vJy2N8uRaUQ-q3rtoqdvnSNPyq5Q'
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
   Inserted with ripple: {"id":"qqJqOufs-vD7DLGq9kWjHe19yxo-zp_H7xzxZtu3lSQRNFy9z9KCbW17VHL0ijpcu4oNcx8Z-EgkGrLgfEeLQQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"t0C4IYgyLREk-jG8XHHAmtjCuZRr8Zy3W93yOfipzyJQXgsaV5QhcV6lXUCnNlCI1ghS6K0nS5ezY8TiKsdWZg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"value":"Product 1","type":"String"},"id":"qqJqOufs-vD7DLGq9kWjHe19yxo-zp_H7xzxZtu3lSQRNFy9z9KCbW17VHL0ijpcu4oNcx8Z-EgkGrLgfEeLQQ","price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"type":"String","value":"Upsert Product"},"price":{"type":"Integer","value":500},"id":"custom-id"}

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
  Fields returned: ["name","user_role","avatar_url","id","age","status","bio","email","created_at"]

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
    - Fields: ["age","secret_token","status","api_key","user_role","password","bio","created_at","id","avatar_url","name","email"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","name","id"]
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
    - text-embedding-ada-002
    - whisper-1
    - gpt-3.5-turbo
    - tts-1
    - gpt-3.5-turbo-16k
    - gpt-4-0613
    - gpt-4
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
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
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - o1-2024-12-17
    - o1
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
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - gpt-5.4-mini
    - gpt-image-2
    - gpt-image-2-2026-04-21
    - gpt-5.5
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-5.5-pro-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-realtime-2
    - gpt-realtime-whisper
  anthropic:
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: HJ4BZX9Y1ro2X7rdiAJHjD223mflyhd4S9wLtP7ihSoxlkSxS4h4pshILp6FlLUPYJ1M5LEVNqoXaY1tuzJegg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_active_users_ts: Get Active Users
  - cache_api_call_py: Cache External API Call
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - fetch_api_user: Fetch User with Cache
  - cache_api_call: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_users_by_status: Get Users By Status

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
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/ts_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
✓ Client created
✓ ts_route_admin → GET /api/route/users/admin
✓ ts_route_user_by_id → GET /api/route/users/:id
✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ ts_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mTypeScript client examples complete![0m
