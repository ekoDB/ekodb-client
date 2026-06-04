make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.8',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

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
  id: '8JiWRE-qqvrhOIeAgrlUOzHKmtzUD02KfAL7PNbhoRUzs6Flley9AucKgC-mYMYyIxkKiogOcxzqnsclzMghFQ'
}

=== Find by ID ===
Found: {
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  id: '8JiWRE-qqvrhOIeAgrlUOzHKmtzUD02KfAL7PNbhoRUzs6Flley9AucKgC-mYMYyIxkKiogOcxzqnsclzMghFQ',
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  active: { value: true, type: 'Boolean' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { type: 'String', value: 'Test Record' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  price: { type: 'Float', value: 99.99 },
  value: { type: 'Integer', value: 42 },
  created_at: { type: 'DateTime', value: '2026-06-04T02:06:36.239Z' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-04T02:06:36.239Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  metadata: { nested: { deep: true }, key: 'value' },
  id: '8JiWRE-qqvrhOIeAgrlUOzHKmtzUD02KfAL7PNbhoRUzs6Flley9AucKgC-mYMYyIxkKiogOcxzqnsclzMghFQ',
  categories: [ 'electronics', 'computers' ],
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  data: 'aGVsbG8gd29ybGQ=',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  price: 99.99,
  value: 42,
  created_at: '2026-06-04T02:06:36.239Z'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  id: '8JiWRE-qqvrhOIeAgrlUOzHKmtzUD02KfAL7PNbhoRUzs6Flley9AucKgC-mYMYyIxkKiogOcxzqnsclzMghFQ',
  name: { type: 'String', value: 'Updated Record' },
  created_at: { type: 'DateTime', value: '2026-06-04T02:06:36.239Z' },
  value: { value: 100, type: 'Integer' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: aZcJ2vk9HuP0I7za9McLVsDuH5V2BfHwkizPPqMqq_YH33pklbkSegpcuiSnK9BNcIpLDcR2_-AKFXMauWf0AQ

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
Collection created with first record: vTVg9dfWFtZKfCsZj45LvGgDTBJnG417yvqp_5PNibFHXx7I0qgL4v-NwgeRQEAXsoSIWrT3XPPtwKsEUtslhg

=== List Collections ===
Total collections: 19
Sample collections: test_collection,agent_function_versions__ek0_testing,batch_users,ws_ttl_test,chat_configurations__ek0_testing

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
  cache:product:2: { name: 'Product 2', price: 39.99 }
  cache:product:3: { price: 49.99, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 11

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
Created Alice: $1000 - ID: ZJR_bwT1GL_G--O0CHLhzXirNMjfT_1tKMkoDQnoVpsbAfuEv8KIyfAdYwN13dVrZ9wXdB_kz7MxjDJG_nNOkA
Created Bob: $500 - ID: 7MXv6jMosVinxSmyl0slhyx4nqSuF6B56MbbFQSLHbIvcXX-Gqr9bGEMowwZnhqrXNcvC1JnN_sduTCdfcPf7Q

=== Example 1: Begin Transaction ===
Transaction ID: 77a9f121-1711-4b11-ab0e-735b9fedf1fd

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b09af56d-243e-42cb-9e74-5bf576bdda26
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
  1. Score: 26.400, Matched: bio, bio.value, title, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: title, bio.value, bio, title.value
  4. Score: 26.400, Matched: title, bio, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, bio.value, title.value
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: bio.value, title.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.767, Matched: 
  2. Score: 0.761, Matched: 
  3. Score: 0.755, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: title.value, content, title, content.value
  2. Score: 1.504, Matched: content.value, title, content, title.value
  3. Score: 0.307, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills.value, skills, bio

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
✓ Inserted document: 6pUAJV8ErPWZ96rMKLByg6Lz_vhT84EfKqLbjgJQd2rMkv3W6BeigEIjzr8T58BnfdWGEdhPgBqZY4sm2tIj6A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 1bl0Ou0kU9uXhdKrgLxhPkYeteuyWEmBQsUShH78oOwxzFkXMPnqHE-Ivrxa74alPuwjr-GG0JZXx_J2Ywy1OQ

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
✓ Inserted document with TTL: hPYUxzDyDDmyeDk6sCFDvPKyBnV4KGZhygTEUK7H5CuOU6JMG9T-lhAt0oVfwRq0JjwCZwCYLRIJQYMFRHG_Ig

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
✓ Edge cache script created: EUIEmcs-6qsl43F4fqV6g0dj2sExaUDTnTZ-509YizuUhb800VIf7Z_gbtzleqb3TvaaYFU0552Ncmhp6Zm6OA

Call 1: Cache miss (fetches from API)
Response time: 132ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
          },
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442"
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

Call 2: Cache hit (served from ekoDB)
Response time: 41ms (3.2x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
          },
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442"
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

✅ Function saved: qNqSXWWc91gUhzP1c94Y1QN_4_digzVxcaErZwMXefP2xb8qQKsHJotFVgatFYdGwIa7NpwCgCOAXbonE_NL2Q
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: p9LRhfI0dqkX_wiDZHipbcU7EsOdpGfgop4jHcVcE8F2Jinhi9ISjKkJ4_9lro_y9e5wLAvFHEGLUuAhM0OiYQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: -Hw1yxx1KcXAA9vulo57uDs27W31x_B8g7oqYZPrLP2dvZYOxWaAEtBSbWlXM_rOhhdHUl6OrgbzeVppk_d3gw
📊 Statistics: 3 groups
   {"avg_score":{"value":50,"type":"Float"},"status":{"value":"inactive","type":"String"},"count":{"value":5,"type":"Integer"}}
   {"count":{"type":"Integer","value":5},"status":{"type":"String","value":"null"},"avg_score":{"type":"Float","value":60}}
   {"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"count":{"value":5,"type":"Integer"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 11
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
    "value": {
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "id": 1,
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "email": "Sincere@...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "id": 1,
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "email": "Sincere@...
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

✅ Function saved: tmaiP2mK3nTxeVNpw2RR3-AzlglSFLAhlC3ciM5q7JDiTOJn68ukvJ6rvYy7KeL1pXTmZ6obf8Y_wMeKTqzdpw
📊 Found 2 product groups
   {"count":{"type":"Integer","value":3},"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"type":"String","value":"Electronics"}}
   {"count":{"value":2,"type":"Integer"},"category":{"value":"Furniture","type":"String"},"avg_price":{"value":474,"type":"Float"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":2},"category":{"value":"Furniture","type":"String"}}
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
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

✅ Inserted order: ZIYRqjHxXfDxCsO16aonlYdbbpYteHEvc1K0mJLSuw7XPB9xobz-pRlcaG7QCALbpSY-6iCBaADc1J2F8ZBoaA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: Rr5_LuilbKkHH6Lrwc2xpM4YvKqi6sM1TZrZ_07b1RlQTwPAal3voeu9cmow8cz3ZtqInW_JrSVqXzzL7r5lVQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: fCWtFANmINT6J3yfqRZ8iBKIg_bAWJTA8NAwNoU50KorRZj7JbvtZf8P-npmW4nqjdIXJu-qfyeioQKWxsUjhw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: lmVgD9Eh9n97jFfGGb7RttMuZ3ey6CKLclvPSeWWLidt_fR2j3VM3KKS1TBqEu4VnuaQDiERBYsSfTuSy8QKiQ
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
✓ Created SWR script: fetch_api_user (LMpX1e9-oOfWiKJsR11U8uJOtYn60c5hC8K2Tvl7Qj7qsIdcY8n-KGNdV93zSi7xWwsilaxGl3kxhkTlZiDWtA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "address": {
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light"
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 46ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "address": {
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light"
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (2isbX97i4Py-EKe48RkWuXRYMPCVYoZAZH_5_weSUcg17Jq7QiKZFcK4PyFbLXM1W09tEHIrYW81852v2n9Glw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "reviews": [
            {
              "rating": 3,
              "comment": "Would not recommend!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            },
            {
              "comment": "Very satisfied!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "rating": 5,
              "comment": "Highly impressed!"
            }
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "availabilityStatus": "In Stock",
          "sku": "BEA-ESS-ESS-001",
          "discountPercentage": 10.48,
          "id": 1,
          "meta": {
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "createdAt": "2025-04-30T09:41:02.053Z"
          },
          "returnPolicy": "No return policy",
          "minimumOrderQuantity": 48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "warrantyInformation": "1 week warranty",
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "rating": 2.56,
          "brand": "Essence",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "tags": [
            "beauty",
            "mascara"
          ],
          "shippingInformation": "Ships in 3-5 business days",
          "title": "Essence Mascara Lash Princess",
          "price": 9.99,
          "category": "beauty",
          "stock": 99,
          "weight": 4
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
✓ Created native SWR script: github_user_native (KbU6q43gjxLywOFxyxYd9u1KPsaq9L-aBCEJin8Mit6aqfiDa7KLhT7PxrPQTVu0IztEc7gaSGL0yEfPIcTewA)

First call (cache miss - will fetch from GitHub API):
  Response time: 15ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 15ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (ModLQPbfae7xWWo4KBpbIy26rQKFYFEvTy1bM0EP3NwLbYIu7LIJrNOfRzeJY-V6Xe04waULKYJVTYtQnPm_Ag)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (y8zU3KtOwp5aRRcnrrrfOxXBRhyA7Yo-aX8mpjeef4C6lxQ1wIWnGD5c22MwesX2ebAEYql0HMKKSloO9rwVcQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (ovYwXm3iOvyEdTHEKELhqjj0jjhY23tDo9gUONDalReH9KZRUigPPyq_zFWxPkLPGJIs_u8LE6oCNr72JLQUAA)
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
   {"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":3}}
   {"avg_price":{"type":"Float","value":367},"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":5}}
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

1. **High-dimensional Data Handling**: Efficiently manage and query high-dimensional data, such as embeddings from machine learning models.

2. **Fast Similarity Search**: Enable quick retrieval of similar items based on vector similarity metrics (e.g., cosine similarity, Euclidean distance).

3. **Scalability**: Handle large datasets seamlessly, suitable for applications like recommendation systems and image retrieval.

4. **Real-time Processing**: Support real-time querying and updates, allowing dynamic data handling for applications like chatbots and personalized content.

5. **Integration with ML Models**: Easily integrate with machine learning workflows for tasks like feature extraction and semantic search.

6. **Rich Query Capabilities**: Allow for complex querying and filtering based on vector representations, improving search relevance.

7. **Multi-modal Data Support**: Incorporate various data types (text, images, audio) represented as vectors for unified processing and analysis.

These advantages make vector databases particularly suited for AI and machine learning applications.
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
   {"count":{"type":"Integer","value":3},"role":{"value":"admin","type":"String"},"avg_score":{"value":20,"type":"Float"}}
   {"avg_score":{"type":"Float","value":70},"role":{"value":"user","type":"String"},"count":{"value":7,"type":"Integer"}}
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
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"value":"Database","type":"String"},"count":{"type":"Integer","value":3}}
   {"count":{"type":"Integer","value":2},"category":{"value":"AI","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 3mAvNfe7dflrgRURThj9oM-4rGGuIYQXizP88R1pNXcac7V9lINGyY2r5cW7UHNaXE1dV1T9yAxK9LRDJ7bcRg

=== Sending Chat Message ===
Message ID: H9VF755xUbSP9_WK9ages5SYZHEiqiyCrTBVlzkXocNp73rwnWeq5ZMFBbK05r5kbK2ft5dMsmzKs-IJVwUsAw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**: $99
2. **ekoDB Cloud**: $499
3. **ekoDB Pro**: $299

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: '-wt8qrx_Y9lGjFf74vil054GBEpgzW3Cw1_ajHMjiR2LORKDanidQXJitjfMbK-3FxB1x7zZihpFSVJnP4sxaQ',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'JWZmGzOCiBAiOikcoQ3U-WPa6PXTiOF87tJ-oAeSLdzBrR4cckl85veyY1Af_7H5U-yqQQ_iXTNIAqCfd_fW6Q',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'wKPlW7_IKUXowFOfVh85a7llIhnP892C8PKVDSjHQv8TazxinA112DxRsgpOXABNRMSWo4mff5p2OQHMu-XDtg',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 7684ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 30
Total tokens: 3498

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: deTdVSOTN4c5Q--O1-7rfiockpFj5_lCbmTq5y4WHVvXe14qQFZE6NtKVKLvKxYuwHE6-AW_f6NImIJgDpKvUg

=== Sending Initial Message ===
✓ Message sent
  Response: It appears there are currently no products marked as available in the database. The only product mentioned is ekoDB, which has a description of "High-performance database product" and a price of $99, but it may not be listed as available.

Would you like to check for any other specific conditions or products?

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
✓ Created second session: HfGZcnyq8NwojPN6-v5d-nRR5Uubl-dFg9DDA8SBZ6NtIpfyb6HNmmJaP_nPGa7cHC8UlqQMS-STw0XyqcKYvA
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
✓ Created session: EOP1Z85mGEUbI6_yFqz-2_OXs_ohTczT_kfs_BL6sHDX94_U6Lbl-I75fESVvsElUZU38NIxEoMw3cL1IceNqA

=== Sending Messages ===
✓ Message 1 sent
  Response: It seems that there are currently no products marked as "available" in the database. The only product I have context for is ekoDB, which is a high-performance database product priced at $99. Would you like more information on this product or assistance with something else?

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: nWcl7Zxza3D7HZ1RPXdkdUe3Y_vUJND2Kyg76AYHVoOIS56RYMBylpiTdgW3ybBDPxEZEtEWGgGXmTDI02D4mQ
  Parent: EOP1Z85mGEUbI6_yFqz-2_OXs_ohTczT_kfs_BL6sHDX94_U6Lbl-I75fESVvsElUZU38NIxEoMw3cL1IceNqA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: nWcl7Zxza3D7HZ1RPXdkdUe3Y_vUJND2Kyg76AYHVoOIS56RYMBylpiTdgW3ybBDPxEZEtEWGgGXmTDI02D4mQ (Untitled)
  Session 2: EOP1Z85mGEUbI6_yFqz-2_OXs_ohTczT_kfs_BL6sHDX94_U6Lbl-I75fESVvsElUZU38NIxEoMw3cL1IceNqA (Untitled)
  Session 3: HfGZcnyq8NwojPN6-v5d-nRR5Uubl-dFg9DDA8SBZ6NtIpfyb6HNmmJaP_nPGa7cHC8UlqQMS-STw0XyqcKYvA (Untitled)
  Session 4: 3mAvNfe7dflrgRURThj9oM-4rGGuIYQXizP88R1pNXcac7V9lINGyY2r5cW7UHNaXE1dV1T9yAxK9LRDJ7bcRg (Untitled)
  Session 5: wVBUzKZaxSYdkFueTddSDuAi9jOUvwDF-WbYzAOZdxsSVxcdi1b4lsRMEIimpUh1u3f52j0a2NsV8Rb2HjvaRA (Untitled)
  Session 6: T5W80S-H_MX9c4WQB6y4fWR-aNghY0kz69aVp-dqjj5-PyiukQozkzsGVFvH00Y7hbOtMhd0ZJev16qgS6Cl5w (Untitled)
  Session 7: KpcIGJvJmt2UtuJVYJ08cviX2TCsr0DApW1HhaWbWtB2d-4g6TwfHAHbHct8g1rKPUwjvV4AF9sJPgK8fQPHaQ (Untitled)
  Session 8: ZnMU1BjciAw2rS9106kKBePPyJOfcaB3tFAYSjLGyO-pTDcJ_jZV06419p5D4ewxBa1M4sRxwr6PtzjftqWVCw (Untitled)
  Session 9: _eYhw9e3j1ObVxpx5rh0Fa1d2BXpRBp-NeIe4RR3aPOS7pga7HBIZNUie-xahYi5o1cgUpGTgZie-zQDQjIICw (Untitled)
  Session 10: hwh6992BOfXfWMM0XTcSlXVMKtkPmLW3JrRi3p5MTu4iz12WeWaK6zI4fd7OMrm33ryx_lXu2e9SOXY0uSw7Kg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: nWcl7Zxza3D7HZ1RPXdkdUe3Y_vUJND2Kyg76AYHVoOIS56RYMBylpiTdgW3ybBDPxEZEtEWGgGXmTDI02D4mQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'gqCOPBrHRDCeSy2b8ZMasIMWJfV1kjNqYQ3JTbHfNIuLIX4qGo9rB504CWjNZ318uSTVvSYmg5QWiXa37fyp6w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { value: 29, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'gqCOPBrHRDCeSy2b8ZMasIMWJfV1kjNqYQ3JTbHfNIuLIX4qGo9rB504CWjNZ318uSTVvSYmg5QWiXa37fyp6w'
}
✓ Second upsert (insert): {
  id: 'FV6-IcFgz19Iiy59ld0rSjPrrTZvtUIgRRGdb5lJG73zRn-MiL4U4jKX2o0YDvgXqlVVISmAlEe4WeMITcH8Mg'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'gqCOPBrHRDCeSy2b8ZMasIMWJfV1kjNqYQ3JTbHfNIuLIX4qGo9rB504CWjNZ318uSTVvSYmg5QWiXa37fyp6w',
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 }
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
   Inserted with ripple: {"id":"SM8b55hDzoCZQ6Js9MMJaaxqr3OzpgX_RcqEJEObFY-7UrUGkoBExN2QKBmYsaUTKuK_TSZuD05rz6EwqObMqA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"WydWMtoLNJWVoE1oq8Y6GkwaOS3ejhxJJL51x0FDFXrFz6KWq63Klk3LYw3XRFFxcV6PG_iVNybMQ4IcqnBjUQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"SM8b55hDzoCZQ6Js9MMJaaxqr3OzpgX_RcqEJEObFY-7UrUGkoBExN2QKBmYsaUTKuK_TSZuD05rz6EwqObMqA","name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"value":"Upsert Product","type":"String"},"price":{"type":"Integer","value":500},"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","email","name"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["email","status","avatar_url","bio","age","created_at","user_role","name","id"]

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
    - Fields: ["status","age","api_key","avatar_url","password","user_role","email","created_at","name","id","secret_token","bio"]
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
Created user function with ID: VTPemv_gGIvgHqFV9Flde-YDyfDzEWCe253k1VE5suapgyb4AUSG1F-VWxRRTiMa0nBoZ-WAzxOznSGXYL6dRw

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - fetch_slim_user: Validate and slim down user
  - fetch_api_user: Fetch User with Cache
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_ts: Get Active Users
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - get_users_by_status: Get Users By Status
  - cache_api_call: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_verified_user: Get verified and validated user

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
