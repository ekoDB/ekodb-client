make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.9',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.22.0 prepare
> npm run build


> @ekodb/ekodb-client@0.22.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.22.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'b7gVER08TjNxKhJ7e6XJII7nsAWUVayZJSqaRpSepa4QBHI9ShONaUd9VXnr6iDxe2Lmo7dptkLuPrkWIqV3Tg'
}

=== Find by ID ===
Found: {
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  id: 'b7gVER08TjNxKhJ7e6XJII7nsAWUVayZJSqaRpSepa4QBHI9ShONaUd9VXnr6iDxe2Lmo7dptkLuPrkWIqV3Tg',
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  price: { value: 99.99, type: 'Float' },
  active: { value: true, type: 'Boolean' },
  created_at: { type: 'DateTime', value: '2026-06-26T16:55:28.504+00:00' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-26T16:55:28.504Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  name: 'Test Record',
  value: 42,
  id: 'b7gVER08TjNxKhJ7e6XJII7nsAWUVayZJSqaRpSepa4QBHI9ShONaUd9VXnr6iDxe2Lmo7dptkLuPrkWIqV3Tg',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  price: 99.99,
  active: true,
  created_at: '2026-06-26T16:55:28.504+00:00'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  created_at: { type: 'DateTime', value: '2026-06-26T16:55:28.504+00:00' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  id: 'b7gVER08TjNxKhJ7e6XJII7nsAWUVayZJSqaRpSepa4QBHI9ShONaUd9VXnr6iDxe2Lmo7dptkLuPrkWIqV3Tg'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: usAtQmo5ZNMY7c_Pe9EE8Hstau8zvfup7tprfICkw2hCbV8WYKDrK8wIRCHCl2ktQ4YsYS6jGc0XQQQgnxUtkg

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
Collection created with first record: p11izS8mMAuMnu6QlcvB0jCuXiEkTqCPNUKDtgpbHJX4-gXhhkN-u0n11Ba5bwIYHfxXDEl5Uf-z74XMKYfKxA

=== List Collections ===
Total collections: 23
Sample collections: test_collection,functions__ek0_testing,client_collection_management_ts,ttl_cache,chat_messages__ek0_testing

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
Created Alice: $1000 - ID: JGkwG3aGEN9Z217DTX3gbZKLan-E_MNq3hvmFfEKaVnHtjiqjcxVhUhI-tgLYfiHv_IzUITNX5oamoeTmmzt6A
Created Bob: $500 - ID: yQF4-WO00cnIX4qXlcCeFARKnCwXEs98Nq2-iYJCKDXZV0FdIQlZMthHTDtIrdz2si9OYA6DbhyjMGu1uzss5g

=== Example 1: Begin Transaction ===
Transaction ID: 45c7a58b-6a6c-4906-8c59-4bedd4fe2fb0

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 107ddf23-1c4c-4858-bc30-f334d0c237b8
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.757, Matched: 
  2. Score: 0.753, Matched: 
  3. Score: 0.752, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501, Matched: title, content
  2. Score: 0.901, Matched: content, title
  3. Score: 0.303, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

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
✓ Inserted document: erZ02cuxpbrIkpCyhhn6vxxFmnCJP8fgf1zUKFaeBiastnDquxSJ__7wXiIjAzBvXETXriHpRt4l-3fYPcJEnw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: wloENxxZliAvwe4PFgoHHsL3RfQbiK3Lj32upwjvGDPUI17iR5j2o9_W6J5mloFAmgtsJbP1ODOnGl7JXCMP-Q

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
✓ Inserted document with TTL: QLgmPxhEuRgVsjzgFSdg7vRlFMYeHmwYs9eP0rvkedFWMZmMMQTjOYcqk1R7hku6PkqC-cuKbW_40qwfB4JhkA

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
✓ Edge cache script created: Aid05EbUbHJDLcXhdrmXadip3yLUjstBqR9YqkmNxyhsCNvfpbTOygmkQZxgBb_YoHwgZwyfdQjzoRyOV-Wg2g

Call 1: Cache miss (fetches from API)
Response time: 66ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "address": {
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "username": "Bret"
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
Response time: 3ms (22x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "address": {
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "username": "Bret"
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

✅ Function saved: jn8MQkps18LnEmlrpow61xD-HMJLY762QuX2angZX8w9dxW6b0dmal5HNctqVA2vkVCsnzbbt_dcOqY2F0vQuw
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: qQman-_9D5Cte9LRunKUV1YqFb5BHcj74e9lbedGr6OKYa_h0TkTRLVFfrlkf8nhvo2QJHhOZIQewSL7Ef_cJQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: CTg5q-xaA2LdZNMoh5NeEfvYFqELf3f6qXimvWtsRNJZp6jbH8oCrwoxYzD-uiM3AQlZ86bI32fBiALvWFu2_w
📊 Statistics: 3 groups
   {"avg_score":{"type":"Float","value":60},"status":{"value":"null","type":"String"},"count":{"type":"Integer","value":5}}
   {"count":{"type":"Integer","value":5},"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50}}
   {"status":{"value":"active","type":"String"},"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"}}

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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets"
      }...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": "harness real-t...
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

✅ Function saved: oUlRYYOL_ORP2g4hyeAlRru-__1ibU6jshsQu_SN3RWLyOuDpTkTtTgzWKXPe56WB5_dLM_jzJiL4-7FVPgkFg
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"avg_price":{"value":474,"type":"Float"},"category":{"type":"String","value":"Furniture"}}
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"value":575.6666666666666,"type":"Float"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
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

✅ Inserted order: y91fFLU0D8GTdMdAfOSOq8E_y4t3ReuMVlDrvqziav0nA7wbAX24gehq5wVUcaErgdPvVI95_2ervw0GTN9pgQ
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: emxo33ZHBgIcvD9NDI8Hl5Jy2IxJ5Q1Gs_KXEA6KKSuj_P37zI4TV8pJcITJll5tqxqg7cifgmhhUu-oxAf6iw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: j4ut2Frp2Zkh0jRyILvPhyf68cI4SDmL-fkCIJYLhrrmMnjEtG3Fwf-TVRKB3MeHCuyEjECp3gbPvduCakjFXA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: eV7-fR14aghUsxKxlk7AZRRUDXRWWH8Qd8oTWOs5PHQs4dHVXL_AHD1eKnZ8QRLmYyZWa2JSzbqXMBpmm5nvpg
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
✓ Created SWR script: fetch_api_user (5e-THDgBamGiP7gRnezIqWY8J9FSf9vR1zGkOhl7lKHLZPfZpRHMV5pXIZ3TurbK6APrlSiE431hn9kTqpmsyw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "username": "Bret",
          "id": 1,
          "website": "hildegard.org"
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
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "username": "Bret",
          "id": 1,
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (ffhtp7lIxeI3XLrqi0ff-V4rmzC-3ciVs4eLVP-MoobgnD77XeZsFMQIST4KXqCb4gK-Gb_YhqtI-zrod-46IA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "shippingInformation": "Ships in 3-5 business days",
          "availabilityStatus": "In Stock",
          "category": "beauty",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "title": "Essence Mascara Lash Princess",
          "discountPercentage": 10.48,
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "weight": 4,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "reviewerName": "Eleanor Collins"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!"
            }
          ],
          "returnPolicy": "No return policy",
          "minimumOrderQuantity": 48,
          "meta": {
            "createdAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2025-04-30T09:41:02.053Z"
          },
          "brand": "Essence",
          "sku": "BEA-ESS-ESS-001",
          "rating": 2.56,
          "price": 9.99,
          "stock": 99,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "id": 1,
          "warrantyInformation": "1 week warranty"
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
✓ Created native SWR script: github_user_native (SIofGWKfI_vhRZgRbKDtRnrM3w2nCIk_5-VVj1AV2ZKndumP9_9QcMpgHA1PIaPlHQqj1xnEK21MNpxAzlZXFg)

First call (cache miss - will fetch from GitHub API):
  Response time: 3ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 1.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (HSQpegjMlSKvKg2d61WRMPGFXSfz40o9ZFIjLhJkieKctxWW2wFfKzX4zztMnRCA4gQUlUBcAEUHoTX8yxvTmg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (7EwvZxMwHgMedq-AgTrmHIjBvbonO3ThQepzN8FoYxva9r7bcNqjdbauRWuCeW5ed-GGPVOADPz6exuUNM6_mw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (dYZ5acohyO1atVgAN58MKllHAz-6Ch-RBjKd3TVwYuFHqaHgAboMBbXPCHw_MzzZTh431D3lbDjEdaunDVS6IA)
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
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":365.6666666666667}}
   {"avg_price":{"type":"Float","value":367},"category":{"type":"String","value":"Electronics"},"count":{"value":5,"type":"Integer"}}
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

1. **High-Dimensional Data Handling**: Efficiently manage and query high-dimensional data (like embeddings from machine learning models).

2. **Fast Similarity Search**: Enable rapid nearest neighbor searches, crucial for recommendation systems and search operations.

3. **Scalability**: Handle large datasets effectively, making them suitable for big data applications.

4. **Flexibility**: Support various data types and formats, accommodating unstructured data like images and texts.

5. **Real-time Processing**: Allow for real-time data retrieval and updates, enhancing performance in dynamic environments.

6. **Improved Performance**: Optimize storage and retrieval processes for complex queries compared to traditional relational databases.

7. **Enhanced Machine Learning Integration**: Seamlessly integrate with machine learning workflows, facilitating model deployment and real-time inference.

8. **Better Similarity Metrics**: Provide advanced distance metrics for evaluating similarity beyond traditional methods.
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
   {"role":{"value":"user","type":"String"},"avg_score":{"value":70,"type":"Float"},"count":{"type":"Integer","value":7}}
   {"count":{"value":3,"type":"Integer"},"avg_score":{"value":20,"type":"Float"},"role":{"type":"String","value":"admin"}}
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
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"value":"Database","type":"String"},"count":{"value":3,"type":"Integer"}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 6Wh9bCQ13lb_HCio7nnE5Hr8aCUA5gv-29SxdpvsvDtlSjZoNat3ZAXh1nqwBi-9AsG-HN5SFab1tYsSpenM2A

=== Sending Chat Message ===
Message ID: 4PIw86o6KyJrBx9xvN0OAwpVXwVRs8fSUK3Y0aim1CQehSoLBtOFmDoYbbCWpSnc4kRJg1SChMrhRiIP96Zkxw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

2. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

3. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'bLLCSyD4AAlGJG6DA9xy9pizC9GM__DcGL7IY6U90ig0WuPPANmHgLC4p47oQed6E_5TYQwXMAbvVPPngWJm8g',
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: 'NzdPHKMIIfEFo2-O0388epCmo44qK7dRznx8Q3a_sx8T_UPBs1KcVxF1Z5kr1RIbN8uVNAEL9okDNQP3RdZbeg',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'Ms1CyhPSdItsQ-6UKF2YXdeiVSJHR8FfiSo-Yw6cJn5jtwoWd0CdfrwO5I5AW5aivHcUaqziRGepMxWfpwkCjQ',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 10686ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: n2MeP84jyoIr06DLVd2uA36DjefoG7fbHZ3XoR3r3GuQW6jHmxJ_to9ImOHXJ822GBDGjmTgulzZ5LKumZ1_1w

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no products available in the database. The only product mentioned in the context is "ekoDB," which has a description of "High-performance database product" and a price of $99, but it doesn't seem to be listed as available. Would you like to know more details about "ekoDB" or explore something else?

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
✓ Created second session: 3DvYW-NyW3TeumsTUdqRKqI8_b6YRbC3vM0oRo8oPIOgoQG6XEkf0aLUyfOsJs3gzkw5P8vrdtWSr5pyFr3xIw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: tl5AgUXw9dsU27owPdl6mLfXHoIZp4TqFqUCIgszFovnGiHlT09d4XUbaEfpWaQXcPP9aneMgArcJcnrJ6QKUQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product.
- **Price:** $99

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: pk1lu9HdT4uShyBbVX5m9rvJVM3zuzL3rGL0b0TR2z7vKG-wyQOu5KrF24FZGrJ0RBLB66b2qR0m2chD-MVTvQ
  Parent: tl5AgUXw9dsU27owPdl6mLfXHoIZp4TqFqUCIgszFovnGiHlT09d4XUbaEfpWaQXcPP9aneMgArcJcnrJ6QKUQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: pk1lu9HdT4uShyBbVX5m9rvJVM3zuzL3rGL0b0TR2z7vKG-wyQOu5KrF24FZGrJ0RBLB66b2qR0m2chD-MVTvQ (Untitled)
  Session 2: tl5AgUXw9dsU27owPdl6mLfXHoIZp4TqFqUCIgszFovnGiHlT09d4XUbaEfpWaQXcPP9aneMgArcJcnrJ6QKUQ (Untitled)
  Session 3: 3DvYW-NyW3TeumsTUdqRKqI8_b6YRbC3vM0oRo8oPIOgoQG6XEkf0aLUyfOsJs3gzkw5P8vrdtWSr5pyFr3xIw (Untitled)
  Session 4: 6Wh9bCQ13lb_HCio7nnE5Hr8aCUA5gv-29SxdpvsvDtlSjZoNat3ZAXh1nqwBi-9AsG-HN5SFab1tYsSpenM2A (Untitled)
  Session 5: X8y4BQWBBNZKMk4RpdETvhBcErKX00VRLLkDmYJCw_loP0dqoXD5MaZ3rwYVtdDUG-uzHCR8Nd5SvG9SjI4_Aw (Untitled)
  Session 6: mqEaN0jW1PYv4-F-nE5E0Y9pe_1D-NBvlwKr9Hqp7Gl520k7RrXGqQpjE3cinyv438pzqdTSH71F20QHVQGxDw (Untitled)
  Session 7: dKGks_vWoqc7xBDZc623Z7LgNSCJJ5QFe-FA_xx6sKDPg8-MHLIDLF0rc3NBksrUQ-nv8W3py5ltjB6qOfhwsw (Untitled)
  Session 8: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w (Untitled)
  Session 9: ukiK-AXE26Q3Zjg_6vlCTPlIKs6tPt_cZ1FJZmS2k3GLm7E9jvnL_X7O14_DOGfmoVXa1X1BaT06zCM8dy9weQ (Untitled)
  Session 10: dNGb9c_qlepdYXRMjHPAGJy-MDa4ulQLp2WDlIqDZw6P0oMFTVsKtL3Kbm7QmxX2GWwRZn8COanmCIfYs3kCBA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: pk1lu9HdT4uShyBbVX5m9rvJVM3zuzL3rGL0b0TR2z7vKG-wyQOu5KrF24FZGrJ0RBLB66b2qR0m2chD-MVTvQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'c4yBR4uNna_v8EDPiVXYQ2Q17JSPS1Cq23GLB0loEsWFfiZfVeRDouVhrAsJHIL5efJo8ZdtsZuBEHtPHVRX_w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  id: 'c4yBR4uNna_v8EDPiVXYQ2Q17JSPS1Cq23GLB0loEsWFfiZfVeRDouVhrAsJHIL5efJo8ZdtsZuBEHtPHVRX_w'
}
✓ Second upsert (insert): {
  id: 'iF-q3QyBkS8COfkoWCXQwZpN8HDrgsZT5su6keY2bh-1k713wVtu8t5-m6OBkpAk7V_rr0m0JKi94MaftJ7UPw'
}

=== Find One Operation ===
✓ Found user by email: {
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { value: true, type: 'Boolean' },
  id: 'c4yBR4uNna_v8EDPiVXYQ2Q17JSPS1Cq23GLB0loEsWFfiZfVeRDouVhrAsJHIL5efJo8ZdtsZuBEHtPHVRX_w'
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
   Inserted with ripple: {"id":"4kLH3VsKBkSPY6ETCPnY9uoSyhOqdoa8d72G_7MPd-_zNm97rAwV6a-k6HVJC8nDtivbwi26hg4K6C3vEMCGXQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"jpmaQUiGl1OTJpDOyo7inz1cPTlLLLKD1KsirwTCU8bSwBZ5dd3RCKnAph6Lsg5OXl6IEBX59qugAQiOEJo03g"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"4kLH3VsKBkSPY6ETCPnY9uoSyhOqdoa8d72G_7MPd-_zNm97rAwV6a-k6HVJC8nDtivbwi26hg4K6C3vEMCGXQ","name":{"value":"Product 1","type":"String"},"price":{"type":"Integer","value":150}}

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
  Fields returned: ["id","name","email"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age","email","name","status","id","bio","avatar_url","created_at","user_role"]

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
    - Fields: ["password","name","email","secret_token","avatar_url","api_key","created_at","status","id","age","bio","user_role"]
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
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: 2s-xtl8xvcownEbU6PW8JSRpqAVtenaEoEX8ETOfr6JZJWBwq5WhGinqhuxyd_eBGFEbY6ppCchF0XcIoRsLNg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - fetch_api_user: Fetch User with Cache
  - cache_api_call: Cache External API Call
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - cache_api_call_py: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_ts: Get Active Users

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
