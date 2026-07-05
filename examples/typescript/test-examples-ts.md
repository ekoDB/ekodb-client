make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.9',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.23.0 prepare
> npm run build


> @ekodb/ekodb-client@0.23.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.23.0 build
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
  id: 'p0zxpXxZUnWFAIyK-GtVoMi7xPY-8zvSJX0d44Dy2S8LguYG-bovINgIYe2V48_ApGPOAux6kOF-vxlklGRTHw'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  price: { type: 'Float', value: 99.99 },
  created_at: { value: '2026-07-05T03:59:48.555+00:00', type: 'DateTime' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { type: 'Integer', value: 42 },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  id: 'p0zxpXxZUnWFAIyK-GtVoMi7xPY-8zvSJX0d44Dy2S8LguYG-bovINgIYe2V48_ApGPOAux6kOF-vxlklGRTHw',
  name: { type: 'String', value: 'Test Record' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-07-05T03:59:48.555Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  price: 99.99,
  created_at: '2026-07-05T03:59:48.555+00:00',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: 'p0zxpXxZUnWFAIyK-GtVoMi7xPY-8zvSJX0d44Dy2S8LguYG-bovINgIYe2V48_ApGPOAux6kOF-vxlklGRTHw',
  name: 'Test Record',
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  id: 'p0zxpXxZUnWFAIyK-GtVoMi7xPY-8zvSJX0d44Dy2S8LguYG-bovINgIYe2V48_ApGPOAux6kOF-vxlklGRTHw',
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  value: { type: 'Integer', value: 100 },
  price: { type: 'Float', value: 99.99 },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  created_at: { type: 'DateTime', value: '2026-07-05T03:59:48.555+00:00' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 0Hy8FstRSGXtVq9TmYl4ezNimJe1MpzUgwE3nwynWsS0e-oK7ygA56zzwfFe11gYdm_yJ0YOwXFFX7SsHb8mmQ

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
Collection created with first record: ngQNY6n85n3gtcfmWW7UOeT71U73fCD62bWm47cxHawxjPFbbBFeaYGjd00QBG-DvvqfjQoWjP0sAYUJVYzr3Q

=== List Collections ===
Total collections: 34
Sample collections: module_user_notifications,ip_locations,chat_configurations__ek0_testing,cached_deployment_logs,zone_locations

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
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

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
Total keys in store: 8

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
Created Alice: $1000 - ID: CVTzw62WJDMppoco2aXBMiwoqssOZUgc4kxRHZ3ys3_Fe50WLK1lVshzBxP-PdIsvKS_JuhVCr3EYLDLwNuZLw
Created Bob: $500 - ID: NErBaUKe0vcQkCOscWmyUcJR0PuGiLPpgOrNQ5zTVTWbUTPbDFMl-0z2zOVSpmMo2xXZdxP1uoMICkZQCxXNBw

=== Example 1: Begin Transaction ===
Transaction ID: 84c8db7f-1567-462d-8735-1580170c4e41

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 33842313-a5ef-46bc-9e00-155d81048d35
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
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
  1. Score: 0.769, Matched: 
  2. Score: 0.768, Matched: 
  3. Score: 0.765, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.508, Matched: title, content
  2. Score: 0.906, Matched: title, content
  3. Score: 0.307, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

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
✓ Inserted document: nhoyGSo2P91Fsi5CIca2kqHmA_RFzPBiHIc6_Goz03a4rqyN2l81542RQGHW6SynRbXocSFYAj2Zfz0ZB-7WqA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: clrNT0u3ZyPauPd88ZC394Yc3MFUBuCsTdE-TqZOvOxwFy86n156Fhp0oJELDyzdStSsu1kDznN0hFMf27m6tQ

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
✓ Inserted document with TTL: PJlKLJbn5NtPzUKSDjJH9CmMLPz0JAbttoFb9p4dgVEjP8QzIlx2uX1bYJIOrT_kPR-QktJjDr5VEuTndSx-xg

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
✓ Edge cache script created: KXeAZrfVvxnmKGZ3dJ1NMRSUM5aoHtEZIXoDHQdgjM-sFeW8h1kF8YbY9hd47bD0C1ruMGOX4-MC3iiPQv8keQ

Call 1: Cache miss (fetches from API)
Response time: 142ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "username": "Bret",
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
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

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (47.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "username": "Bret",
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
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

✅ Function saved: aD0PfpP22yWk48JO7AzAH5dU7KlEwO4IR-UnBZdTQCa_TBGVmYGVWbDrCzhA4XwoQLehZLvCLp_pszyyL_9YlA
📊 Found 12 active users

📝 Example 2: Parameterized Function

✅ Function saved: XaUqmIrthA4Pa8bzUaAp7aE5v4d4TpZixGfLaXyMxsm6FuERH79ShOpxjyvObKUy1NrWPLGoyBVAl6twD5hmgg
📊 Found 12 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 00JVA_t8NQBaeQKmQEDRNDxMONUcSOElO4rfTom5BgdCwhdSvwP5DgoSToH2cNKUSmuG_e5n5bAuApb41ExHIQ
📊 Statistics: 3 groups
   {"status":{"value":"null","type":"String"},"count":{"value":2,"type":"Integer"},"avg_score":{"value":0,"type":"Float"}}
   {"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5}}
   {"status":{"value":"inactive","type":"String"},"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 4
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
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 56ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "username": "Bret",
      "website": "hildegard.org",
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
  ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "username": "Bret",
      "website": "hildegard.org",
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
  ...
   🚀 Cache speedup: 28.0x faster!

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

✅ Function saved: j7ojVzdwWhuHedDzxLPAvjpS5i-bDHS3kn53Gpq6aBBmWUZJJgQV1c-ungx2GzTR961w_rZDTaziSXf4rldbOg
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
   {"count":{"value":3,"type":"Integer"},"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"type":"String","value":"Electronics"}}
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

✅ Inserted order: kLFvmIwz14abEh0XRqL6O5f_WXR_nFNgSBNxxh_tnr9A4st0FwBc35RXf6DR_-6GGUXz4FyBK8wwSkShR9in9g
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: cWBqzFdq_X3gixV3LRIXrQzN3j0-CnCyh3tJwQFikfbZKvQ62wvCbb4-F6I-DooyQutGRE7_SV4ARN_5TzevrA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: NXuOu5RLDSYsZdqQtDdxbeL2kU5MJB54X5j6gQJBRDWDjsP0Bttb9LxKlAtUTnrKXDzjGUeXdYxejZZ4rNy9GA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: V0uQpqhGsm6_VIJ-PP8f45mPy-0zF4ws0cPL_MyEKYNRtPmI-dSWWEO8I0T9Fu9rzMjiN3bRRb7td5b3d401lg
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
✓ Created SWR script: fetch_api_user (CIR4fmizoO2RsXOIGAVCsBFoOM97ZAJCrJhWXxHaVYEm7rbEyGlciNEFSVS7gg8W0KSWGzxkz2NLKVlUVoT0oA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "id": 1,
          "website": "hildegard.org",
          "address": {
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
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
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "id": 1,
          "website": "hildegard.org",
          "address": {
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (XztXvv5ERuZk31XTCOfz4GhTiuvg2Fw_xanTIyvOq-3l8VgAPbx51mT5SB3HsF5qfP4-PJXddLK5laC3_vMUAQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "title": "Essence Mascara Lash Princess",
          "warrantyInformation": "1 week warranty",
          "meta": {
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "createdAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "price": 9.99,
          "shippingInformation": "Ships in 3-5 business days",
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
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Very satisfied!",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com"
            },
            {
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 5,
              "comment": "Highly impressed!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "discountPercentage": 10.48,
          "category": "beauty",
          "brand": "Essence",
          "availabilityStatus": "In Stock",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "sku": "BEA-ESS-ESS-001",
          "stock": 99,
          "returnPolicy": "No return policy",
          "tags": [
            "beauty",
            "mascara"
          ],
          "weight": 4,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "id": 1,
          "minimumOrderQuantity": 48,
          "rating": 2.56
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
✓ Created native SWR script: github_user_native (YXs3W07odzYtpWp_w7QPmVjHWnZC8YMaM3bTw5e900oMz0xBnyLbe6wgKr4DscoKxJMocoXM-Ey6nBESbz1FHQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 193ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 38.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (cFKkldPGAq-03zCDK7hexnL1othuDYveu5CZdE-Alfi2hZiNbG_qBqUt9gc8k0JsYyTb6z6D9vHbXq0PETRifA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (RK5S_o7aMTYyILYQzIcGysn6rCOGH4cWpPhIWD3Lu0D8boCpQpuTqI89YkfDAvO-GOeYHKNvKdSvlPpcPJFz_w)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (nwHXEAoQyWt-oWO5mRekvpb7xi2pFTxs0UqU18FW9J2BNAorTYD74t-APbFHSQbYHZr3vuNI9ONqMfQEHzEDtw)
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
   {"count":{"value":3,"type":"Integer"},"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"value":"Furniture","type":"String"}}
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":5},"avg_price":{"type":"Float","value":367}}
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

1. **Enhanced Performance**: They are optimized for high-dimensional data retrieval, enabling faster similarity searches and efficient querying.

2. **Scalability**: Vector databases can handle large datasets, making them suitable for applications involving big data.

3. **Flexibility**: They support unstructured data formats, such as images, text, and audio, allowing for diverse applications in AI and machine learning.

4. **Advanced Search Capabilities**: Users can perform complex searches based on vector embeddings, facilitating more accurate results in recommendation systems and natural language processing.

5. **Improved Accuracy**: By leveraging similarity metrics (e.g., cosine similarity), vector databases can enhance the precision of search results and recommendations.

6. **Integration with AI/ML**: They often integrate seamlessly with machine learning frameworks to support model training and inference.

7. **Real-time Data Processing**: Many vector databases are designed for real-time operations, improving the responsiveness of applications that depend on quick data access. 

8. **Reduced Complexity**: They simplify the process of managing and querying complex data relationships compared to traditional databases. 

Overall, vector databases are particularly advantageous in applications like recommendation engines, image retrieval, and natural language understanding.
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
   {"role":{"type":"String","value":"admin"},"count":{"value":3,"type":"Integer"},"avg_score":{"type":"Float","value":20}}
   {"count":{"type":"Integer","value":7},"role":{"type":"String","value":"user"},"avg_score":{"type":"Float","value":70}}
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
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Database"}}
   {"category":{"type":"String","value":"AI"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: zSXPnxx8hxhXiFVN982r7ezANlojKtSOjXmUofHEWr4eo_X8j5JPZ-lNRBYxIr2npEy-pUaX9karHJ85DT2jvQ

=== Sending Chat Message ===
Message ID: 0mULDMxrBELEqRLm2-BuyFeKgv64pTOeO3dXfVSkOVjO7-MYZOuk5by_d_PoGmTiO7NB4JsNIhGVZlVzve53ag

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product.
   
2. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features.
   
3. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities. 

Let me know if you need further information about any of these products!

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 'otWqNOF2p26UfbhNkoA6CWsHA__fFi0LAF-hQsuQ4_8-bT4HTQ504rl7ufdv6XMQQeRrmVE219hXncV695ZEsQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    id: 'EQaOjv8WuKdAXMn14fIPmrVZBL-iGbDDMw5TzKEkbEiZz7lVHtt2TVxneT6WIDVMm8f4gkGk1vWp3UYLQjLZaA',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'zZVDBHRbTVkAro6FotqcYlitg5ajso7UWUg0q0sVptboyc-g5uiSJcEEONmMjh-Drfci12GUQIVeVThytaZA9g'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4204ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 103
Total tokens: 3516

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: O-aGuxWS45XK-txp6WbQJzZ61W-GYcB61u35GVSPPt8dUHfCLz61i_T5EvMwjRLyxzlO_1m8lZOCYrqRjQLMWw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or additional products, just let me know!

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
✓ Created second session: H3lXLnzrutEO7IWVAn6TMp5kvvr6RI1Qx2--PWE_PNPjyKhlEbqFqlnxpcjrAHACPZyMMbZ3dbCMrHJ95Hd6oA
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
✓ Created session: gixhtuhaBBL8oTUHNkrq0frgGOzuH21YEZBeuCB1qEkMaJ12PjoLpfSzcQeyXQvm8EEOHspcB9BGEJDkVOCyuw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or assistance regarding this product, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: izdNtuLbfUIVVpoZf-rPsegJGkh8IFCHgpSUI-IBm7UYzLtfEfKeRwmhpntbqLWjqPkd8CefVvWn1GmUaEbB1Q
  Parent: gixhtuhaBBL8oTUHNkrq0frgGOzuH21YEZBeuCB1qEkMaJ12PjoLpfSzcQeyXQvm8EEOHspcB9BGEJDkVOCyuw

=== Listing Sessions ===
✓ Found 5 sessions
  Session 1: izdNtuLbfUIVVpoZf-rPsegJGkh8IFCHgpSUI-IBm7UYzLtfEfKeRwmhpntbqLWjqPkd8CefVvWn1GmUaEbB1Q (Untitled)
  Session 2: gixhtuhaBBL8oTUHNkrq0frgGOzuH21YEZBeuCB1qEkMaJ12PjoLpfSzcQeyXQvm8EEOHspcB9BGEJDkVOCyuw (Untitled)
  Session 3: H3lXLnzrutEO7IWVAn6TMp5kvvr6RI1Qx2--PWE_PNPjyKhlEbqFqlnxpcjrAHACPZyMMbZ3dbCMrHJ95Hd6oA (Untitled)
  Session 4: zSXPnxx8hxhXiFVN982r7ezANlojKtSOjXmUofHEWr4eo_X8j5JPZ-lNRBYxIr2npEy-pUaX9karHJ85DT2jvQ (Untitled)
  Session 5: 7Wl_JaekUtjsXcN0ozc1-8xBXxPdqDLpq9ObKzJBNtFyEjoCW86W0RrZenrKV823VnjIpAEFXy2ribled8D0mw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: izdNtuLbfUIVVpoZf-rPsegJGkh8IFCHgpSUI-IBm7UYzLtfEfKeRwmhpntbqLWjqPkd8CefVvWn1GmUaEbB1Q

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'HUs1_Rj7TxT7GuDKr66H0CzZxm3GMK2Kr0n-eUn6bk_A5jQNEFV8RwMu9KYO_e1vp2fkgpzmhjR2CFFcO6q8ew'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { type: 'String', value: 'Alice Johnson' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'HUs1_Rj7TxT7GuDKr66H0CzZxm3GMK2Kr0n-eUn6bk_A5jQNEFV8RwMu9KYO_e1vp2fkgpzmhjR2CFFcO6q8ew',
  age: { type: 'Integer', value: 29 },
  active: { type: 'Boolean', value: true }
}
✓ Second upsert (insert): {
  id: 'AK0ygo31ifsla3SmPm2l19xfTigv4-anEVeF1GU_4PGMVQtVE_fs3_0k_n8zBliKsDSuXcUb0sUAjw_PLCVDUg'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: 'HUs1_Rj7TxT7GuDKr66H0CzZxm3GMK2Kr0n-eUn6bk_A5jQNEFV8RwMu9KYO_e1vp2fkgpzmhjR2CFFcO6q8ew',
  age: { type: 'Integer', value: 29 },
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
   Inserted with ripple: {"id":"pbinA4AA5fwYdUSXyiNgE7rL0-YGBNDydw3oYez2L2FbfI3MOhv3RFnWfnS4KKXknUOMEOuruJWrhlXt8fdHdg"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"QgrcHQcyVEREV4pqng5EmMrR7Gn8H6syrE7xmCxaxqsmVR4RIFKp2z5wVku2QxAFUZgEemazpKgsP_Fves4B_w"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"pbinA4AA5fwYdUSXyiNgE7rL0-YGBNDydw3oYez2L2FbfI3MOhv3RFnWfnS4KKXknUOMEOuruJWrhlXt8fdHdg","name":{"value":"Product 1","type":"String"},"price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"ulK2LV06rVSKoLzLs1FuHKce_ut4cex1OLEcCLl-ra1S-_5GiIT5iEb4bmSgH1d1_RW9rbPf_Buw_38KqMgLlw"}

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
  Fields returned: ["id","avatar_url","age","name","status","created_at","email","bio","user_role"]

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
    - Fields: ["id","avatar_url","name","user_role","bio","age","created_at","secret_token","status","api_key","password","email"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","name","email"]
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
  Float 0.1 + 0.2 = undefined (should be 0.3)
  Decimal "0.30" = undefined (exact!)

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
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: kvDBWnd8Qttgpza7N-fdkvQ8EekrEYs-wPpuSiDETyMvD7gXurAStVA1iKJupfmBQxCGgHmYNQuhM-l1AliNIA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_active_users_updated: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_ts: Get Active Users
  - fetch_api_user: Fetch User with Cache
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - cache_api_call: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - get_users_by_status: Get Users By Status
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)

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
