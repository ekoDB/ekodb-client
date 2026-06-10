make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.8',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.21.0 prepare
> npm run build


> @ekodb/ekodb-client@0.21.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.21.0 build
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
  id: 'lB0IWsbUI736phwZy8dspr3vKyUTqRwZHXML8f3vyTnPFpSFRqkwCHSb-igbWKDtxFbpRuJfQvtDTR4eJdvjuQ'
}

=== Find by ID ===
Found: {
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'lB0IWsbUI736phwZy8dspr3vKyUTqRwZHXML8f3vyTnPFpSFRqkwCHSb-igbWKDtxFbpRuJfQvtDTR4eJdvjuQ',
  created_at: { value: '2026-06-10T01:38:18.906Z', type: 'DateTime' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-10T01:38:18.906Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  value: 42,
  name: 'Test Record',
  active: true,
  id: 'lB0IWsbUI736phwZy8dspr3vKyUTqRwZHXML8f3vyTnPFpSFRqkwCHSb-igbWKDtxFbpRuJfQvtDTR4eJdvjuQ',
  created_at: '2026-06-10T01:38:18.906Z',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ=',
  metadata: { key: 'value', nested: { deep: true } },
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  id: 'lB0IWsbUI736phwZy8dspr3vKyUTqRwZHXML8f3vyTnPFpSFRqkwCHSb-igbWKDtxFbpRuJfQvtDTR4eJdvjuQ',
  active: { value: true, type: 'Boolean' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { value: '2026-06-10T01:38:18.906Z', type: 'DateTime' },
  price: { value: 99.99, type: 'Float' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  name: { value: 'Updated Record', type: 'String' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: VJddsELseApKE-kgtFMz7YzlDtTtwEQAy37Kbyn_ghcifYLglEXWIplPOLq9KWuqWoil_61c-WuUKN-SBvL0aA

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
Collection created with first record: 0PaPNX9TnM2wyRetVTWT69IJaXYxaL6xl7i7LT54pKBTUQWQJzLjYncZKRdkAG0rkqLyWOVIfbMQG-87ICmhvg

=== List Collections ===
Total collections: 23
Sample collections: chat_messages__ek0_testing,chat_configurations__ek0_testing,enriched_users,schema_users_client_go,batch_users

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
Created Alice: $1000 - ID: rzo-DCGiH2BMs2j7eWRwcga7VrpyUbLKcc-GcP7GQoblJvil6WKo82ioFKVRg_53NaX_CX9KPKRGd2qs1U2hFA
Created Bob: $500 - ID: 8FDrSh2H7hXWuFyFVTG05BYw1G3uF_NEHbuGW38Svzi9anDGDv7qbCk6xL-HbnhD5OYVJS45iJHZ8U0-Bhbh3g

=== Example 1: Begin Transaction ===
Transaction ID: 0335989a-83a0-4e36-a477-58e5d7f09037

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 18bf3840-4eea-412c-b7f8-fabf811cec33
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
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio, bio.value
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: bio, bio.value, title.value, title
  4. Score: 26.400, Matched: bio, title.value, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, bio, title
  2. Score: 39.600, Matched: title.value, title, bio.value, bio
  3. Score: 39.600, Matched: bio.value, title.value, bio, title
  4. Score: 39.600, Matched: bio.value, title, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.735, Matched: 
  2. Score: 0.735, Matched: 
  3. Score: 0.721, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.688, Matched: title.value, content, title, content.value
  2. Score: 1.494, Matched: title.value, title, content.value, content
  3. Score: 0.294, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills.value, bio.value, skills

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
✓ Inserted document: 2c84dRcB9AJe9w0BNY3CHNs25s3xMn2MkOkbLJTCWFsswQMQhjKTtaq9b3Q5JW4RB1EmvDz96JFucDGxyCR9Cg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: cdiMCcd9GDY4EHifBb3ChKvZ_Rq6pZa9YDtEoDHp_jNYyp2i0R6xh-cy9n3ikcyUYWdFX1cJ3BSi8XR37TETYg

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
✓ Inserted document with TTL: DsFkCzig84ncKrazZW20Ty4wzhaPmHOa0VT8XJqz5smzgBfmsZSC1L26VNGzxVq5Al7GX-SKXGq52BtHFdpKQg

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
✓ Edge cache script created: 4yheEeFL8mfNtySWXSURO53Kg8ombYJ2uEuJb-Zbu1OuKyZgQnFl_Y5xnLAyKky0En2-IWiknD5RHv5p9CUtJA

Call 1: Cache miss (fetches from API)
Response time: 160ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "name": "Leanne Graham",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
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
Response time: 42ms (3.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "name": "Leanne Graham",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
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

✅ Function saved: dQ1aI0bPCLDEE5bfX8WCFhp-nHjJoH6YksLWWd_0KAmdCypNLYxfMxfl_Hp5IZjWahPC7CzzW_i2Mje6ISSPUQ
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: Da_XLUV7yR0WgW77JrCmxsQ6KLs9amm4gmY-1d9utKNr7sKN0UiZyVlI_CAXKGdcjZSVRRv6mvysyCtlWbt80g
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: iLtF5fbLjcEiBYDC_99nCIlkNuXb5K_hNlnLOFbOEKq96kEfjx6NT3lWQpI8Ok6VlqbajrKtmiLVecym0R93HQ
📊 Statistics: 3 groups
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5},"status":{"type":"String","value":"inactive"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"null"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"active"}}

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
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "harness real-time ...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "harness real-time e-markets",
        "n...
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

✅ Function saved: bybCx768JHZO-vi_KK-rUcCO3AVtdRcbtBgv8q5MJvp4fGrlLernKBhXLj-c0AxYRIepiyjMsAFNA01GtEUfdQ
📊 Found 2 product groups
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"},"avg_price":{"type":"Float","value":474}}
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":575.6666666666666}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
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

✅ Inserted order: LHWJkaHHR6huu5Azbzuog_qWLjG8bmMkKO9mv_VaHDIUTf1Duv3D-hbOcguPQpAgBDFXLviP1_pc53IdmcT-_g
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: S7wyBTWqoaCCJ0q9mbfWWuveV4zx0ZqIrvoPiD1w_wDa-yN3JA29F3rDo3kI-boHLvFdfe_5w8rTXcxd88zV3w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: FVfweXPq-oLq6gIzV42COYjed51BVe5mr0CtnHRHseudhJWb7-FlOg4zIGc_-XKtJBt39QG0nwh1GEbuaXsmbQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: lsSh_YyRzFlvDi9Rggj27xt1tP22Ab9MkO80rjV_OwSUFLIvzfvnLdfu631q7CFd2furNhZSE3jhQvsdyPKu_w
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
✓ Created SWR script: fetch_api_user (dUAXJZFgqzhKn0tcMC_hAbRFaPTGznfQVpGp40sC8e1Xl6-MyxQ7EHX4STqGIp4k_SZvp1eCmtFSD2tBS9oh2A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "name": "Leanne Graham",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556"
          },
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz"
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
Response time: 41ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556"
          },
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz"
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
✓ Created enrichment script: fetch_product_with_reviews (EMXQfpv7YaxCLgC2wtQsEsrp8zGQipulELZ0OybLZtDgHDGgyrb0ArqiISXbqj25HOXOuFyQS9Eoon6kXYS-Ag)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "rating": 2.56,
          "discountPercentage": 10.48,
          "title": "Essence Mascara Lash Princess",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "warrantyInformation": "1 week warranty",
          "minimumOrderQuantity": 48,
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!",
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon"
            },
            {
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "id": 1,
          "stock": 99,
          "brand": "Essence",
          "sku": "BEA-ESS-ESS-001",
          "weight": 4,
          "returnPolicy": "No return policy",
          "category": "beauty",
          "availabilityStatus": "In Stock",
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "meta": {
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "createdAt": "2025-04-30T09:41:02.053Z"
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "shippingInformation": "Ships in 3-5 business days"
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
✓ Created native SWR script: github_user_native (ARpJ05PjcWqAlHhaUkPro9gFnzdNJTPxvhFCNHZEj8uZlzdBEI_u9EBhO_0m3B9onLrgTnRiQQ8CvcUjsoEolQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 16ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (2fSppb5aszjd3sX8SZN6eTZHQUeySMFCE_tVn1R8jbeGG1L0vbPITsrvG1qFSQLls7goJ7Dzz-D9P-kFmwVSWA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (cDlvHv8lrrAhWNB1YzsUiGoDRX5xUvdy4lWbEyvzu8YN3thWsCRqBAMWkJuELhALdqxz91Gk_AKyJIubL76erQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (ish_eBdn8IUc529iUF4iwUvTlnclCRzGtRYSa8xCZ-A7BFaqlagOXl2InbRup3ne_FAz_g4G4rpKPl4KNzmgSQ)
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
   {"count":{"type":"Integer","value":5},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":367}}
   {"count":{"value":3,"type":"Integer"},"avg_price":{"type":"Float","value":365.6666666666667},"category":{"value":"Furniture","type":"String"}}
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

1. **Efficient Similarity Search**: They enable fast retrieval of similar items using vector embeddings, which is essential for applications like recommendation systems and image search.

2. **Scalability**: Vector databases can handle large volumes of high-dimensional data, making them suitable for big data applications.

3. **Support for Machine Learning**: They integrate well with machine learning workflows, allowing storage and querying of complex data types like text, images, and audio.

4. **Semantic Search**: Vector databases provide enhanced search capabilities by understanding contextual similarities rather than relying solely on keyword matches.

5. **Real-time Processing**: They facilitate real-time analytics and decision-making by enabling quick access to relevant data.

6. **Flexibility**: They can work seamlessly with various data types and structures, allowing for diverse use cases in different domains.

7. **Dimensionality Reduction**: Many vector databases support techniques to reduce the dimensionality of data, improving efficiency without sacrificing information. 

Overall, they enhance the performance of applications that require advanced data retrieval and analysis.
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
   {"role":{"type":"String","value":"admin"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
   {"role":{"value":"user","type":"String"},"count":{"value":7,"type":"Integer"},"avg_score":{"type":"Float","value":70}}
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
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":2},"category":{"value":"AI","type":"String"}}
   {"count":{"value":3,"type":"Integer"},"category":{"value":"Database","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Pn8r_isvnjZVuq5VU-lwW6dtY8yitGZAlms9dpC2ETllmSDz9lhBe9hrs36T9feYNukoliHZb83qTuJNKD1h_w

=== Sending Chat Message ===
Message ID: Suu7PMlhwfmqSB8CC0zCg_JwT3rW_n47FnwTsRzs2F2YmPP8FfHewgRo-Z9TI3m4h_iX-aw9vWtNz4MiTuysyQ

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'ke46ZQjIrHt-6V6J7uZpf1m1W0a-f-xIszcbl_YXuSDtLxDdLCZahOWffsmiSI41bFH-O_msjNgmVYESIYOqiQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'LfKjScPlQ543Y_C-HxJLjNKLricOxCfljkHVUJlMTJTbrMKCG08WxleSX6OCuOVhi9f2C1KFIFW2Ll3_Y3_f7Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'V7hy5iTBWF7NMV_hfEzlc4EsUm7QUQrsWupPKlApSJMEgRGbhTIHWsawANHBNkvJU96xWPg7fV1WgZB1MuRDXQ',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 7207ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 91
Total tokens: 3559

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0HBA6NFJ5dNypv6Xuxr0s1mkWXLxP6-cOTsglK-EaybKKc90DZ3u_yPtkvj-EIjKCca7CcQXbIWPZl-twnUfBA

=== Sending Initial Message ===
✓ Message sent
  Response: It appears that there are currently no products available in the database. The specific query I executed did not return any records. 

If you'd like me to check for different criteria or provide more information on available products, please let me know!

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
✓ Created second session: sZ_pFtv-SgjN5qRqTCaVXKCAC8pSGDLdZfvZn0vhQYogZ3WN1oAJaiQ2QDYSwfSHBYwGasbWpOF0TVZpseAlLg
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
✓ Created session: kG2_CcjINKxJBETgxlXwzU5mswVn09wcpbRMt40aZYsilk5nYPkcaNZhtTzRBmIFBDdVP-Bv2ezLSAqU5IkPVA

=== Sending Messages ===
✓ Message 1 sent
  Response: The only product currently available is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any specific queries regarding this product, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: PJey8-qgnlAP4mK98B8uVp8A1psw-J747Y_sq578Sit7dNmLM_WAMgOAkMLO_K3c-YuTYrcxlygqOflS750T7w
  Parent: kG2_CcjINKxJBETgxlXwzU5mswVn09wcpbRMt40aZYsilk5nYPkcaNZhtTzRBmIFBDdVP-Bv2ezLSAqU5IkPVA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: PJey8-qgnlAP4mK98B8uVp8A1psw-J747Y_sq578Sit7dNmLM_WAMgOAkMLO_K3c-YuTYrcxlygqOflS750T7w (Untitled)
  Session 2: kG2_CcjINKxJBETgxlXwzU5mswVn09wcpbRMt40aZYsilk5nYPkcaNZhtTzRBmIFBDdVP-Bv2ezLSAqU5IkPVA (Untitled)
  Session 3: sZ_pFtv-SgjN5qRqTCaVXKCAC8pSGDLdZfvZn0vhQYogZ3WN1oAJaiQ2QDYSwfSHBYwGasbWpOF0TVZpseAlLg (Untitled)
  Session 4: Pn8r_isvnjZVuq5VU-lwW6dtY8yitGZAlms9dpC2ETllmSDz9lhBe9hrs36T9feYNukoliHZb83qTuJNKD1h_w (Untitled)
  Session 5: zQ9sPZrEwJouHWdIPzV9BxVUsjLgEVWPvg3BrCKx8qpphMyfN-7PLO8sytR8_4cyYxwTn6ULgBUTvM1x5lGCPQ (Untitled)
  Session 6: 02WYH5fvFCE8jEaFL6l-ZbPq2sEWjD9Le24i-ON89iHVdcsM7uDa9qzgXiCclIs5t99xOqTR6XzzdlGuMjzC8A (Untitled)
  Session 7: XMcm9EI1r0XL4MrLSKCmrJUS-QsNU0ih0n75U0Gmv1MumkPwb3ie_6eM7UNzTVyEipHHZKNJ0NDjQed-w7ci4Q (Untitled)
  Session 8: arDUIy0JvWFMhNR6jVL2eXTmmA8sAWo7vbbVrTChdH5D5YqwuC8lHzluHFaE8OIBJfBDRPAPZbyxjVy30ZKX7A (Untitled)
  Session 9: IMk5O_Jl94E_vRIrr-4t-mn7L9CpF2IyC5Z0Ef83_ZaCDyZSCJ89KDGpGXX2_R1t3gfRiIBRPdi5_MZmmySl_w (Untitled)
  Session 10: _xiMGVjJpu7r4vWBMXRFRHvuAvR2EBTWY8KdSC_AcOwC4PwMb97zKEra0DwnBDRctEAN6RSpdCdwj7-4hPnZTA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: PJey8-qgnlAP4mK98B8uVp8A1psw-J747Y_sq578Sit7dNmLM_WAMgOAkMLO_K3c-YuTYrcxlygqOflS750T7w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'BehXSoObhovCo1uLSD6QnnP0L22FVUW_UxoATb1CjTq3ex6AJj5qRmhzr19KJkBiKHI5F2SH7H8a-ZwFmegQtQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  active: { value: true, type: 'Boolean' },
  age: { value: 29, type: 'Integer' },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'BehXSoObhovCo1uLSD6QnnP0L22FVUW_UxoATb1CjTq3ex6AJj5qRmhzr19KJkBiKHI5F2SH7H8a-ZwFmegQtQ',
  email: { value: 'alice.j@newdomain.com', type: 'String' }
}
✓ Second upsert (insert): {
  id: 'I42AhZLcRO8MyhzPkFVlHVa5kfFH8xEm9xl1pz0iu5yaTPPPqpcZG18Njdzaph3xBRUjuUgLMbrjARWxeAP-jw'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { value: 'Alice Johnson', type: 'String' },
  active: { value: true, type: 'Boolean' },
  age: { value: 29, type: 'Integer' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: 'BehXSoObhovCo1uLSD6QnnP0L22FVUW_UxoATb1CjTq3ex6AJj5qRmhzr19KJkBiKHI5F2SH7H8a-ZwFmegQtQ'
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
   Inserted with ripple: {"id":"QCFG8fJXQYixVN6Ah8E0bev7vz2LZX3ojzblixKQkpAMrhn9qsjqL3IeDWDCrsrizBCxOTA2aTdKml2T5ZHGdQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"UzH3Rb4qh7ZSdsB9DUYOpWqHJOyWK62LMbcltao9fVIiKLQb2hDyzUUAuaS7z_1IxR7WrsS48syeUgfpgz164A"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"value":150,"type":"Integer"},"id":"QCFG8fJXQYixVN6Ah8E0bev7vz2LZX3ojzblixKQkpAMrhn9qsjqL3IeDWDCrsrizBCxOTA2aTdKml2T5ZHGdQ"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id","name":{"type":"String","value":"Upsert Product"},"price":{"value":500,"type":"Integer"}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","name","email"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age","avatar_url","created_at","bio","email","name","id","status","user_role"]

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
    - Fields: ["api_key","status","email","password","bio","user_role","avatar_url","age","id","name","created_at","secret_token"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","email","name"]
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
    - claude-fable-5
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
Anthropic models: claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: 9i9K4pZBZw-gw9eFNNTarww5T3LwJDq7-oZp-Z5obov4EvNKqD0usfB_llBKy0XR6x-7MglIGOfM_tDno5fbzA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_api_user: Fetch User with Cache
  - get_active_users_ts: Get Active Users
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - get_active_users_updated: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call: Cache External API Call

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
