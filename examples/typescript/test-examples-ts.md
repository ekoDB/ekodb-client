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

added 1 package, removed 1 package, and audited 15 packages in 997ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'lpeDZswhO-AzCkDIz9gECm_w5X6acQEPZ--HtqaCESV3mpJN83MeuvEPND0Tm9lai-S7f2eMkhjezpKlhA-YRg'
}

=== Find by ID ===
Found: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  id: 'lpeDZswhO-AzCkDIz9gECm_w5X6acQEPZ--HtqaCESV3mpJN83MeuvEPND0Tm9lai-S7f2eMkhjezpKlhA-YRg',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  price: { value: 99.99, type: 'Float' },
  created_at: { value: '2026-04-05T05:20:23.451Z', type: 'DateTime' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  active: { type: 'Boolean', value: true },
  name: { value: 'Test Record', type: 'String' },
  value: { value: 42, type: 'Integer' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-05T05:20:23.451Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: 'lpeDZswhO-AzCkDIz9gECm_w5X6acQEPZ--HtqaCESV3mpJN83MeuvEPND0Tm9lai-S7f2eMkhjezpKlhA-YRg',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  metadata: { nested: { deep: true }, key: 'value' },
  price: 99.99,
  created_at: '2026-04-05T05:20:23.451Z',
  data: 'aGVsbG8gd29ybGQ=',
  active: true,
  name: 'Test Record',
  value: 42,
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  name: { type: 'String', value: 'Updated Record' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  created_at: { type: 'DateTime', value: '2026-04-05T05:20:23.451Z' },
  active: { type: 'Boolean', value: true },
  id: 'lpeDZswhO-AzCkDIz9gECm_w5X6acQEPZ--HtqaCESV3mpJN83MeuvEPND0Tm9lai-S7f2eMkhjezpKlhA-YRg',
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { value: 100, type: 'Integer' },
  price: { value: 99.99, type: 'Float' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: e-MOj5Vg_Eus1tx2BJBSGSmQtFQJ4b2SGjAYYB9J-a8ROZBZsUXKQ24AqKfjmG2eq_YsIU0KDzMTjdA0I2h3SA

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
Collection created with first record: 9I603G8N1e1-pB0MY8T0mayVbinypodgj1LS2Jv-xp2zewupLnDqW8lApuvf5sz6Fg6yjXch6hZ6hO4_XMir3w

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_go,ttl_cache,chat_messages__ek0_testing,client_collection_management_ts,schema_documents_client_py

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
Created Alice: $1000 - ID: umZeaA6whSgqUFUB9WKO5wixZgSYZ1zJp3ddqQjVrVMiGtLRJl4uaw5a-j1n8THF4mZJ-3etms8qi6TCb_O_tA
Created Bob: $500 - ID: o7Qp3YP6_qJWw-nXkA330hlHsfhDaOgtEm9b0w05Rb2ki5DhziAxRGBMI7SHkHIlfc1_wIXhBA8mUQ6ubIMFqg

=== Example 1: Begin Transaction ===
Transaction ID: 8e90d848-3eda-4774-9af2-4153bd9e9149

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a4e1e118-eb21-4181-8ca8-eee643ea68fa
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
  1. Score: 25.740, Matched: name.value, email.value, name, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, bio, title.value
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: bio, title, bio.value, title.value
  4. Score: 26.400, Matched: title.value, title, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: title, title.value, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.763, Matched: 
  2. Score: 0.751, Matched: 
  3. Score: 0.740, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.705, Matched: title.value, content.value, title, content
  2. Score: 1.496, Matched: content.value, title.value, content, title
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, bio.value, skills.value

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
✓ Inserted document: BnDUHwHruU8w-ILf5xx5PUvfouUeUKA6XHB9H3r-6kDx9aFYMe7elbhbjQschE9zUUsX6Lmzj5lG1YPw-uY1dQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: GGx_3tdZFSouvQQAosWgOJAsJ05HARNZA060GSVOu9WqeLu_dTtUtobKN0Hb521FagRaa_DkDOy8VjJ3HwBT6g

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
✓ Inserted document with TTL: EbZntllrotla2HL7PmMyGZEIe3MscqpMZ6G_uCPhyhLx_C2HnhMDUFzQ8m4VLVGmumXyDeJ7q5K1qcVzyvmc3w

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
✓ Edge cache script created: rFbvNCgKpS65ccX_ULXxFLs6NgazvPlxseTDbWLlcca0iTpkst1FIPblu0b03M0bUxVES8FyQnNYNPh2GmP56g

Call 1: Cache miss (fetches from API)
Response time: 147ms
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "website": "hildegard.org",
        "id": 1,
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442",
        "address": {
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "suite": "Apt. 556",
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
Response time: 40ms (3.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "website": "hildegard.org",
        "id": 1,
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442",
        "address": {
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "suite": "Apt. 556",
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

✅ Function saved: Ez46Vvogp38B05Mc7wNlmSXullhoTXQxwA9iA05TVTdV4ntGH8L5dXnBBQoRiScMo9rF0RtliF15nH0NaH2GBQ
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: vG1pB79VhtrMSRSWZYwwhy1QnpQm1z4u3g4kagCrTB6oIc-5F4GrndXyxcYXsC5LAv3H0BAxFpYYIeBea0mLWw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: Y2jDBiwJHQwjm04g73j8-iLGuP4MsFm-inAyCiKTlGv_2QSZN-uhCqxoHk9mL3eTWwXjhqM5Xqq5DsJWSzcs2g
📊 Statistics: 3 groups
   {"status":"inactive","avg_score":50,"count":5}
   {"count":5,"status":"null","avg_score":60}
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
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
      "geo": {
      ...

Second call (cache hit - from cache):
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
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

✅ Function saved: GGIkZe0Unfb90QfSTe_2DN8-HF_Q2WBnCN_5d5C3HWj4XXFfbFiEPOZrcRb6FvLR9GTs1CM25yqOzb4jomhOfA
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"avg_price":474,"category":"Furniture","count":2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"category":"Furniture","count":2}
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

✅ Inserted order: DgYVV4QlCF4hGIHHAj5lpCQbiQGahdLqUJ0JofXPP76ttRVFfbS6IgN3_0mgBgfO6Pzx8EOenUmXcZdbCOR6Pw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: kaZU6GEomdUgjRD3YRev0VUJb1CMCs6NirqCrHTbVBllcnvP7R29zq8v-EBa5WNdVQP67LQvSRr8bq1Vf72lrg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: -t9O7lQ1ZWqfwqsYYbLS2J3q1b60qPX4p7eRDvwLpUZqqvONWEDUqcPyQ8mcVMQyyvKOQE6TRqOfhpdYSiRVCw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: MGevczBNOfGOAeHDmitk-YJmJ6UySOXi9qPjt0DHlnMRR-VhlDEL7hc1CRqimlPKSNohJTlHuZL7qZO7Of258g
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
✓ Created SWR script: fetch_api_user (zAqcR8yvbFSdcfbsb5rmRDSexMoUPAHU6mWcbvnpjRgcdRZRkgKBI9u8utRl17fIovLSLGD7qYDzYtnn9G4TeQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "zipcode": "92998-3874",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "street": "Kulas Light",
          "suite": "Apt. 556"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "username": "Bret",
        "website": "hildegard.org",
        "name": "Leanne Graham",
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442",
        "id": 1
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
Response time: 42ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "address": {
          "zipcode": "92998-3874",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "street": "Kulas Light",
          "suite": "Apt. 556"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "username": "Bret",
        "website": "hildegard.org",
        "name": "Leanne Graham",
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442",
        "id": 1
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
✓ Created enrichment script: fetch_product_with_reviews (uMGn0leOvLqirFytgzz316gk0ByOJKLOEwoQBf7AdjHbW-ujnMi9TM5HfOQbcKJxHlPzMNytIP9uh9MHXQtfjQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "discountPercentage": 10.48,
        "rating": 2.56,
        "price": 9.99,
        "returnPolicy": "No return policy",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "title": "Essence Mascara Lash Princess",
        "weight": 4,
        "brand": "Essence",
        "tags": [
          "beauty",
          "mascara"
        ],
        "dimensions": {
          "depth": 22.99,
          "width": 15.14,
          "height": 13.08
        },
        "reviews": [
          {
            "comment": "Would not recommend!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "rating": 3,
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "comment": "Very satisfied!",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "date": "2025-04-30T09:41:02.053Z",
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
        "minimumOrderQuantity": 48,
        "warrantyInformation": "1 week warranty",
        "category": "beauty",
        "meta": {
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "availabilityStatus": "In Stock",
        "id": 1
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
✓ Created native SWR script: github_user_native (xxqO8f3mjE1I699glI73DqXAsXpZSCP0Wtlhi4j5lqRQm_Zs-dzFdrXNDDpGE6CXDBjpCuZWHKCKLBJlgeJNow)

First call (cache miss - will fetch from GitHub API):
  Response time: 17ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 0.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (hGfVSelKmABymFpciJsmjSmYSjmLAr5bnr7HaSRTIF_bJpq2SygC3Kll3vJOfyC1w3AI9d0vByhVH2huLrlGMg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (13cRkvvbGhkNMWK0URXC-PQnq2_aNQS7yewCESfw3UkLM6i18vwC7cEirxRBuu6og3kBO6-aChzTZOvnB0MEFQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (ZFBtrQWEMh87KPm_FhLLjJrORYg-ihYeS-kmMvEB2Ib3MtxwXxX2soUb7fC2YDswi1TZ92ZmRbL17Br-FHMWwA)
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
   {"category":"Electronics","count":5,"avg_price":367}
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

1. **Efficient Similarity Search**: They provide fast and efficient retrieval of similar items using vector embeddings, ideal for tasks like image and text similarity.

2. **Scalability**: Vector databases are designed to handle large-scale data and high-dimensional vectors, making them suitable for big data applications.

3. **Handling Unstructured Data**: They support unstructured data types by converting them into vector representations, enabling easier comparisons and searches.

4. **Improved Performance**: Optimized indexing and search algorithms (like approximate nearest neighbors) enhance performance for real-time analytics.

5. **Flexibility**: They can easily integrate with machine learning workflows, allowing seamless updates and embeddings generation.

6. **Rich Query Capabilities**: Support for complex queries combining vector similarities with traditional database queries.

7. **High-Dimensional Data Support**: Well-suited for data types with high dimensions, which is common in deep learning applications.

Overall, vector databases enhance data retrieval capabilities, especially for AI and machine learning applications.
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
   {"role":"admin","avg_score":20,"count":3}
   {"role":"user","avg_score":70,"count":7}
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
   2. Introduction to Machine Learning (AI)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
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
✓ Created session: ZzXwZ62zSy29eDryiiPTrlNcdexdA_lC-Fhi7IgYMecSrKKCC9TYiWVDZJqTCU4fw51UfAH6VwEbzSnC1cwVBg

=== Sending Chat Message ===
Message ID: k6D8mHnjFst3lBrHVwnxcU6-1YYrzP3HRcn36y5pYHeLX1KbmKkGyE9LQ5OqG9pVnZ1adTIs19GPlPco-mUJwg

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'nRV3N4oIzBLYCovOLBwylqfHsw9ZYSrhgMjtGYursEru2SmmOHxo8CRxsbRAGu-38cAI0VzieURd47r7x8du7A',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299,
    id: 'vJnfVH1CC0EPQloczZ-huTwCBEuusu2By1EZPXyW9x9DaGj26AeZ2mC4aeDit3j-3Tg80j6qOnKlesqtotyW1g'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'F35JBrQjjMkJADsEXOhWma50fdCmuag-T1_Z6LI5KlahiRZYvhurANsXBoTowqi8lxKrhaKQJcnpcoBzO3hv_Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2466ms

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
✓ Created session: VDEh2VIN9fapI2ZehbUBRgtGTbiueaJHAJ45r0pxGOS0S0R1_vMDIyz0xVTsCPOq-q2iQHMRSegIPkrbBVX8xQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have further questions, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need more details, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: YlC_6OxGmSQWIjGWb6cWzmAMweRr1aMPVgjWWn61Cz8oZ5Bnh5sEfznNjX9i2zMfsBW4DMxA1kqMemiPPQfRUw
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
✓ Created session: p0mgDCG_plaRA80Tf7sjvYBIdjXQKQNMCIEH75Q2PA2b6hSTTX7RirdluinPQvpVl8pr3Dq9wfqntEhfBnnTeQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, just let me know!

✓ Message 2 sent
  Response: The price of the ekoDB product is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: FPFThMK0OnZqr6zY94re4Ni3lSEjtR0LO3_PDbBQA3l2mQ79xw-hj2p7zFKqmR68fev8SAQNYtC-NGUN6CO0Nw
  Parent: p0mgDCG_plaRA80Tf7sjvYBIdjXQKQNMCIEH75Q2PA2b6hSTTX7RirdluinPQvpVl8pr3Dq9wfqntEhfBnnTeQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: FPFThMK0OnZqr6zY94re4Ni3lSEjtR0LO3_PDbBQA3l2mQ79xw-hj2p7zFKqmR68fev8SAQNYtC-NGUN6CO0Nw (Untitled)
  Session 2: p0mgDCG_plaRA80Tf7sjvYBIdjXQKQNMCIEH75Q2PA2b6hSTTX7RirdluinPQvpVl8pr3Dq9wfqntEhfBnnTeQ (Untitled)
  Session 3: YlC_6OxGmSQWIjGWb6cWzmAMweRr1aMPVgjWWn61Cz8oZ5Bnh5sEfznNjX9i2zMfsBW4DMxA1kqMemiPPQfRUw (Untitled)
  Session 4: ZzXwZ62zSy29eDryiiPTrlNcdexdA_lC-Fhi7IgYMecSrKKCC9TYiWVDZJqTCU4fw51UfAH6VwEbzSnC1cwVBg (Untitled)
  Session 5: L-i-wdaN13ZIEgsA4f8TODZpG1VETNbwlBSN6zJqZJVegkZFHF3AHSw9rB7i33x8SGrwhu7hpe_aLQIzhBkqvg (Untitled)
  Session 6: Tkc91CJkSRbOsOa29gqzOm---g9q2D8jvab9QJB1X4xAtBbuQPLwHauSVSX5RtAiwW03AI_6rkXizUzfnAlwsA (Untitled)
  Session 7: wDHGhDHQow440ENunyApPaYlggdEgVQBOK3TDrLOor-A8e6BtjXPOLXso8hn6qurUBaJWROUAZ5PGcQUdXcNCw (Untitled)
  Session 8: QHGbuY3c63URk_CEyxJlf9FFov1UFbUvfYRMXtPTYRJFyfp53Dl5QMnJZ0EbeO5j3-Kac3ZkyiwL41v6rl1q1Q (Untitled)
  Session 9: JqAsGGQYR6E_XIdp4OqFeNK0LW8NTvIgVoq90S1HsyDnDZ_-GRMk0qW9UGan26odpyK7yfl78jmp0zKdEurGYw (Untitled)
  Session 10: PpGkNdYNC-xnB_qboKh3_HG8uDOU5O227iZB5HnWpVP9lC2mN4foelxzmM9bDGDk0IcglQHjgCOjNpX_nKvFZg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: FPFThMK0OnZqr6zY94re4Ni3lSEjtR0LO3_PDbBQA3l2mQ79xw-hj2p7zFKqmR68fev8SAQNYtC-NGUN6CO0Nw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'Qif_k6qPlCVwtKCO7cDQx5-n2IVdrbcib8ub4f4ow1RJjYfb4Rfbfkk1GCAGrHXwst7cHPqJHHZ0t12qA1q3bg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { type: 'Boolean', value: true },
  id: 'Qif_k6qPlCVwtKCO7cDQx5-n2IVdrbcib8ub4f4ow1RJjYfb4Rfbfkk1GCAGrHXwst7cHPqJHHZ0t12qA1q3bg',
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'qwmCFJP-o2eCSxwZ_FH_toGFZCUnKFK0slYaaomvscqIgiYZiPJGChKFY5KhZoTJDBF-2XdJajbGJ_--o18SgA'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: 'Qif_k6qPlCVwtKCO7cDQx5-n2IVdrbcib8ub4f4ow1RJjYfb4Rfbfkk1GCAGrHXwst7cHPqJHHZ0t12qA1q3bg',
  name: { type: 'String', value: 'Alice Johnson' },
  age: { value: 29, type: 'Integer' },
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
   Inserted with ripple: {"id":"sUOmOlplxWrCmwMbxLCxr9gN22thoBC0DyhILTM6vPSy5KGdEJMjcoqYg8d8pKJ_kEsiw-wYr4G4whU5MJLeTA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"uIE9hQLa1f7o7ty_LXI1utCFubIaEzzmn0YfFYHZPXjJHMd8W4vVqS81cptmQDPoGJP6drC2VhzgRjFFRpJbAw"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"value":150,"type":"Integer"},"id":"sUOmOlplxWrCmwMbxLCxr9gN22thoBC0DyhILTM6vPSy5KGdEJMjcoqYg8d8pKJ_kEsiw-wYr4G4whU5MJLeTA"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id","price":{"value":500,"type":"Integer"},"name":{"type":"String","value":"Upsert Product"}}

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
  Fields returned: ["status","age","name","created_at","id","email","user_role","avatar_url","bio"]

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
    - Fields: ["id","password","created_at","name","age","api_key","avatar_url","secret_token","email","user_role","status","bio"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","id","name"]
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
Created user function with ID: UXe4BIfhBxe-SfXB-kLly--GB9uEkKq-rIS84ztu53u3SAzvMwYaUpd4fytAGq1kwuN2urbgWUWs40wdGxNFvg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call: Cache External API Call
  - cache_api_call_py: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - fetch_api_user: Fetch User with Cache
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_ts: Get Active Users
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - fetch_user: Fetch user by code

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
