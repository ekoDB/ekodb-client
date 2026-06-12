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


up to date, audited 52 packages in 2s

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
  id: 'K8-uLi0zTnPWCJxIwQfGGOZ_UatPHhlDLVOJTbU4Y9_Z8CqnU1_hzBs2BLVZrxiL4n_eePv5qk9-SszS2IHHGw'
}

=== Find by ID ===
Found: {
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  active: { type: 'Boolean', value: true },
  price: { type: 'Float', value: 99.99 },
  value: { value: 42, type: 'Integer' },
  id: 'K8-uLi0zTnPWCJxIwQfGGOZ_UatPHhlDLVOJTbU4Y9_Z8CqnU1_hzBs2BLVZrxiL4n_eePv5qk9-SszS2IHHGw',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  name: { value: 'Test Record', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-06-12T19:58:34.653Z' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-12T19:58:34.653Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  data: 'aGVsbG8gd29ybGQ=',
  active: true,
  price: 99.99,
  value: 42,
  id: 'K8-uLi0zTnPWCJxIwQfGGOZ_UatPHhlDLVOJTbU4Y9_Z8CqnU1_hzBs2BLVZrxiL4n_eePv5qk9-SszS2IHHGw',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  metadata: { nested: { deep: true }, key: 'value' },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  created_at: '2026-06-12T19:58:34.653Z'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  value: { value: 100, type: 'Integer' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  name: { value: 'Updated Record', type: 'String' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  price: { type: 'Float', value: 99.99 },
  created_at: { type: 'DateTime', value: '2026-06-12T19:58:34.653Z' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { value: true, type: 'Boolean' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  id: 'K8-uLi0zTnPWCJxIwQfGGOZ_UatPHhlDLVOJTbU4Y9_Z8CqnU1_hzBs2BLVZrxiL4n_eePv5qk9-SszS2IHHGw'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: vWvAXRrNGGxj3zGCzornZpNP_m3iPDi5r724X-L6s_UH0QgmsnEWG4MFCMwtPbQBjEusYx6hC7FdcpoEI8qGHA

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
Collection created with first record: P4VWTU9ar2Ie5w3s9FL9FSCuQUDSbDKY3YKk05R1-7rGoOsgKZ-fN-zFY2tLbc4amXfPcMf5B5OFo9IU9x4GMA

=== List Collections ===
Total collections: 23
Sample collections: schema_users_client_go,chat_configurations__ek0_testing,schema_users_client_py,schema_documents_client_py,ttl_cache

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
Created Alice: $1000 - ID: lr75gzx3IRxEKM1wFrk1ySYh-zMfw7zlMG1T-rVcUMZdcaOPzbicGbKCcL7m5qA6t69RoCFUiVNR20IHA3OtTw
Created Bob: $500 - ID: Zn1bAxZRZlsToqNAKNTM01wY0QDNLZTkNKvd5DiAqn4LSeMUCyhykwVxf2Ez09bJ4Hmwf6bvko4U7SbaggTdJA

=== Example 1: Begin Transaction ===
Transaction ID: 96f92f8d-fffe-438d-9160-2efe41a88ad9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ae010b2c-e00c-45b0-9940-53a4fc0f33a9
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
  1. Score: 25.740, Matched: name, email.value, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, title.value, bio
  2. Score: 26.400, Matched: title, bio.value, title.value, bio
  3. Score: 26.400, Matched: bio.value, title, bio, title.value
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, bio, title.value, title
  2. Score: 39.600, Matched: title, bio.value, title.value, bio
  3. Score: 39.600, Matched: bio, title, title.value, bio.value
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.776, Matched: 
  2. Score: 0.775, Matched: 
  3. Score: 0.762, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.705, Matched: title, content, title.value, content.value
  2. Score: 1.510, Matched: title.value, content.value, title, content
  3. Score: 0.310, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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
✓ Inserted document: GDXAX_-zmHJ7F5JrFqxydGe2WFKEqMxRbxdagCW7gFV4Lsxd5MbdpZkrbGzyu2TQzMbTUyE9B_AEU5wltHVVYA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: HpRGg-j3NSldGbDIk7GC2dRRtqHBEhJIxfOULVg13yLaNTD9vkyGsc5I-VxwZFCBBiMQGtHd9N7FiG8As3qmBQ

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
✓ Inserted document with TTL: Nb7ful6YDdbK-X8AGNTFfFpUMmum9x6dh-6H9YhN-4GJNzhKWzXIlyRvT2jZaYe8mMH0dL5TldNv3l2EntxzMQ

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
✓ Edge cache script created: retYhfmwXD018MS5YC1g_CkEpaizbJUNSe0IXEMO9mCx4tCT7ho8tmP45p4CPutiTdxMsaq28bg6aD4_SkTWPA

Call 1: Cache miss (fetches from API)
Response time: 247ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "name": "Leanne Graham",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
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

Call 2: Cache hit (served from ekoDB)
Response time: 42ms (5.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "name": "Leanne Graham",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
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

✅ Function saved: JW6A3w2dzKcpzOWsLjWTSafYl30JvX7oZ9LiumPhb338jpCzvNjaUklQ-h0ZHd_rxrs8Ph3ABEeJ5w5LOyPswg
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: Zr9VF67iTVCUEtsxlulP_UmSDk7Wyrnn5aygtr7ikJk9e5OeHxQfx-4wvMqUUStO_iwglR-91b4DfuphnmrN2Q
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: jo_aFBLoMVOuOZUe0RTEKl4j4YpTHeE8uKkGoEPun5Oa4fG9MGLlX1ojBTql4n9CFtNe9ieYnloJPH6qo7ZfOw
📊 Statistics: 3 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5},"status":{"type":"String","value":"null"}}
   {"status":{"value":"active","type":"String"},"count":{"type":"Integer","value":5},"avg_score":{"value":60,"type":"Float"}}

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
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-server neural-net"
      }...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-server neural-net"
      }...
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

✅ Function saved: ir-rWO5lo_EoUp0uTl9CBXczz-B4eMpI6CSKsPpJrz5xZGbDMZRej50gO8gQIwXApUoIU40LlpgDdsaYfwt3nw
📊 Found 2 product groups
   {"count":{"value":3,"type":"Integer"},"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"}}
   {"avg_price":{"value":474,"type":"Float"},"count":{"type":"Integer","value":2},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
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

✅ Inserted order: YfZljtPjI2mG6IGCESbU1F7tT9u5kaL5fi6poLnnccrjmnHxEbdF_bRBLWAbfymOpnV8LnFIOBNa3ncRIN9nuw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: sEIEPmvTTpaazKP94PiMuLeLX4L6ReaoW8UJu1KrdQsdv4NfDqUIC3iVcUvhHJGfPZmjxRTTthvFkxVrUxQ5Rg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: POGPEOxsnuafBV4YTu6lSMiSXBDwhPwhdZYHsFWdoTRsLNt9PlizNfXerB3KUik4HA9fxXrWjTkZJl2-lMebYQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: qGzT7BPid9owDxhkeoX_4COcfP6BNZlydM1g7ucG7XfVpopgNgWSy0krF9GNFJtFzGA3wfpUVqDrUxJuujv5Wg
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
✓ Created SWR script: fetch_api_user (-io0fo24fphVrggPa0LFxAf6oBT7UeJX6R7ztph2Ukobjmhz8JvgGK6JiFN-3A4NGrptqsO7Fk5FUz0rpVuL1w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "name": "Leanne Graham",
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
Response time: 46ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "name": "Leanne Graham",
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
✓ Created enrichment script: fetch_product_with_reviews (IQQX8O2D1Q0ffiWxKQhFpigz5-hL7RYsrSC-G8afc2pUdz2zwA7T9JlsDaEP3x1owkOfBRUHZVC88aYVTH-mFg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "rating": 2.56,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "weight": 4,
          "stock": 99,
          "availabilityStatus": "In Stock",
          "title": "Essence Mascara Lash Princess",
          "shippingInformation": "Ships in 3-5 business days",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "brand": "Essence",
          "sku": "BEA-ESS-ESS-001",
          "category": "beauty",
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "reviews": [
            {
              "comment": "Would not recommend!",
              "rating": 3,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "rating": 4,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon",
              "comment": "Very satisfied!"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "meta": {
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "createdAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687"
          },
          "minimumOrderQuantity": 48,
          "price": 9.99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "id": 1,
          "returnPolicy": "No return policy",
          "warrantyInformation": "1 week warranty",
          "discountPercentage": 10.48
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
✓ Created native SWR script: github_user_native (AHpj17whCU9E2_WVwby3XsT6AI5g-kg1eQPvmMN6Zq7UwW-JnbkegKuflK2MZIb8QKWscKthvykE3sqXWGl8yw)

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
✓ Created SWR script with audit trail: product_swr_audit (N1TwaP53YUGkE378AWTFm_ZMy2CQ_eD7DDnYktzP-BiX_mKu3Jx2drDzCEw_dVEkI7M2y7vxSPGmsyNBxuDP0Q)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (RcjqSfJ4bPzau9qHzyIhrKT6igsPY4Uh659mLFGxLMQGvwyp1YHHSTSZr19pmAU-pmcqwVDSevQ_R4ccd2qF8A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (8cP2r4jdKNqZW7pzhbbHE_v1wnTftIqng3MQRoNjLVY2ZZCqUJaXALyQRZIW1qJhAox-aeZvzpqE1GAlHzV8hQ)
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
   {"avg_price":{"value":367,"type":"Float"},"count":{"value":5,"type":"Integer"},"category":{"value":"Electronics","type":"String"}}
   {"avg_price":{"type":"Float","value":365.6666666666667},"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
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

1. **Efficient Similarity Search**: They are optimized for finding similar items quickly, ideal for applications like recommendation systems and image retrieval.

2. **Handling High-Dimensional Data**: They effectively manage and query high-dimensional vectors, which are common in machine learning models.

3. **Scalability**: Designed to handle large volumes of data, making them suitable for big data applications.

4. **Real-time Processing**: Many vector databases support real-time data ingestion and querying, enabling fast insights.

5. **Integration with ML Models**: They seamlessly integrate with machine learning workflows, facilitating the storage and retrieval of embeddings.

6. **Reduced Latency**: Offer low-latency responses for time-sensitive applications.

7. **Flexibility**: Support various distance metrics (e.g., Euclidean, cosine similarity) for diverse use cases.

8. **Advanced Indexing Techniques**: Utilize specialized indexing techniques like HNSW (Hierarchical Navigable Small World) graphs for optimized performance. 

Overall, vector databases are essential for applications involving complex data representations and require similarity searches.
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
   {"count":{"type":"Integer","value":3},"avg_score":{"value":20,"type":"Float"},"role":{"type":"String","value":"admin"}}
   {"role":{"type":"String","value":"user"},"avg_score":{"value":70,"type":"Float"},"count":{"type":"Integer","value":7}}
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
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
   {"count":{"type":"Integer","value":2},"category":{"value":"AI","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: yvdXBXWCDsDbED0u2_D1Vki6219cUKhYsehO29osPEEhsxdsFPhrNvi3-mX6PL144_s-gnVkKJqTk79-_nwUtg

=== Sending Chat Message ===
Message ID: Ty1TxrJPtk1jkrrb8gFVWHIDGyOWmCA7prJGSICAVbe0F4XJD6_lAOIToFqywsxzY9zk5fWAGihOn7siAaXuoA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '0xTfS5Je1-SQQnzaAjHp4hPHBqtokd_z9OByQi9d1o98B1g4LlqwqGZ-nfblWbAbO7o-os5Jibc2n7N1jMq9NQ',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 'VePntCXY3UKCpZwVseXlLqheOJV9KNUvOljjVZnC0jIBUQSyqbqoEz1GWmirCZ62585aQTOwyoHrgAkCjX77mA',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    id: 'JG4HYb_SdmiEdVBXtvPVH-Yzdrf_5_rFWG9MgVm0S-RFZsWxHAgvAnMzowABEBQdOudZKB9-rscxJT-LXBCfUQ',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 6673ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 90
Total tokens: 3558

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Wn3RaszzeLspYujvQCFXVBsXDO-Ypb1fOA2pSEkcsfLAlKtAyI9yA3stDv-wHLwrHqFKt25sJs2AoUIlgYBsPA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or any other products, let me know!

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
✓ Created second session: bwSUkxYKHOYj3rNGm12xSKBhyXRE4Rtbq2d9bKW-m9PCqf6bPhMLgIVBk69_CKbjUSIpT8ujAseOH_5FBqIChg
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
✓ Created session: iW4sWA01lAMEGKTy7W-Iluozh-OZOk4yAjBqqe16541Ha4uR-GhFRotJHxAv7pkIR9eYPjx0FqiZwwTfWl_ZRA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: CM_Vh-Z8UzZLVw1wu3i_WDvlTBGcs8Il9RrDmf-od_jRlvjpwSzprotKfdOXw053cligsn51BhAUjIggfb1L4A
  Parent: iW4sWA01lAMEGKTy7W-Iluozh-OZOk4yAjBqqe16541Ha4uR-GhFRotJHxAv7pkIR9eYPjx0FqiZwwTfWl_ZRA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: CM_Vh-Z8UzZLVw1wu3i_WDvlTBGcs8Il9RrDmf-od_jRlvjpwSzprotKfdOXw053cligsn51BhAUjIggfb1L4A (Untitled)
  Session 2: iW4sWA01lAMEGKTy7W-Iluozh-OZOk4yAjBqqe16541Ha4uR-GhFRotJHxAv7pkIR9eYPjx0FqiZwwTfWl_ZRA (Untitled)
  Session 3: bwSUkxYKHOYj3rNGm12xSKBhyXRE4Rtbq2d9bKW-m9PCqf6bPhMLgIVBk69_CKbjUSIpT8ujAseOH_5FBqIChg (Untitled)
  Session 4: yvdXBXWCDsDbED0u2_D1Vki6219cUKhYsehO29osPEEhsxdsFPhrNvi3-mX6PL144_s-gnVkKJqTk79-_nwUtg (Untitled)
  Session 5: AamKgNzxJS2q6Gz46wtBnhpZjvOhy-BBj3ZLVzJm_GJfYimPAUIAqZJ9scJ88lG4JvJ6xmfey22QgnydrzCvZw (Untitled)
  Session 6: 5xZ93JgeSif7bmVc8N4s0KrnEGobUJvBYvx3qWFNtBNdf2PVmGtHAAY4tX2L2F6KEe_5Ci20Q0xEGtDaRfY5Gg (Untitled)
  Session 7: -CHGypZHQioO4FD5R17EMmh9ALEBZqRo9lWZnYJb3FqXkGaLB6yWE7QSVyMjHqrDRr_IwdIiP687MKwrwQoK1Q (Untitled)
  Session 8: tNuyfZ_alWXiFZx_AiOLMEiHugstnCYuZZLssI0VOkulWAdipZhLwzQRR_KrinK3avBTQ-z6o117GSMaeyn7RQ (Untitled)
  Session 9: 11d6MtdtDeEse3qogStB_REOE_-tW7MZPVZ-4ZOd69PJAk0fVLIv5eBJnX5s3Jw-eyRGqlqa0L2ZIGV8d6F11Q (Untitled)
  Session 10: qVxdnHRIOw0uqyTWQk6OVNOPWyJ3ImRFfFsWYq6bb5Rf2Uznt6alHpIguUH3TyIV0Cs9Ko_Fqwq6qBgQ8s7cMQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: CM_Vh-Z8UzZLVw1wu3i_WDvlTBGcs8Il9RrDmf-od_jRlvjpwSzprotKfdOXw053cligsn51BhAUjIggfb1L4A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'A2nqtZb9yT8rGHlhhEKIWJUFXMs0kC28yc7laoWocXggvZSESYGKODTa1vVYcYXZS_tnsZSCXeIYjdJY6mIQGw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  id: 'A2nqtZb9yT8rGHlhhEKIWJUFXMs0kC28yc7laoWocXggvZSESYGKODTa1vVYcYXZS_tnsZSCXeIYjdJY6mIQGw',
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: 'RD1vH7ZT-beQnfzSsZvh1pqycBB_PZF2SfJOpSQJMpk6zwoJLAL3K7b1qKrZrc8p4DcTbqys8b01XLYUAQloGQ'
}

=== Find One Operation ===
✓ Found user by email: {
  age: { value: 29, type: 'Integer' },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'A2nqtZb9yT8rGHlhhEKIWJUFXMs0kC28yc7laoWocXggvZSESYGKODTa1vVYcYXZS_tnsZSCXeIYjdJY6mIQGw',
  active: { value: true, type: 'Boolean' }
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
   Inserted with ripple: {"id":"2zFRxxaWCuvHp0bDU2Dwd9MPEC7U7kpe748rVEElE9pCen8ItfywCnWmLm_1J9G7TLFs7_Pv1e6HTM9lMTVLaA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"WRKJfqYaay1EbXaKZh8f9BN_bwgqZM_8a3bzQRIik20UPi8GwgrS6xomnxAPc9co3hNvJlz95nNMUfiOplU8Pg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"id":"2zFRxxaWCuvHp0bDU2Dwd9MPEC7U7kpe748rVEElE9pCen8ItfywCnWmLm_1J9G7TLFs7_Pv1e6HTM9lMTVLaA","name":{"value":"Product 1","type":"String"}}

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
  Fields returned: ["bio","email","name","created_at","avatar_url","user_role","age","status","id"]

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
    - Fields: ["api_key","avatar_url","id","status","created_at","user_role","email","age","bio","name","password","secret_token"]
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
Created user function with ID: 76THmvO-q-wlbCP1VVOYl7_rbD2UbgDHqZOhnwLwPVKldhcDrMQbvd32v5oZpcPcqgYITa3tFWqUq2MECzZ-jQ

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - get_active_users_ts: Get Active Users
  - cache_api_call_py: Cache External API Call
  - fetch_api_user: Fetch User with Cache
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - get_verified_user: Get verified and validated user
  - cache_api_call: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - swr_user: SWR pattern for user data (KV-based)
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
