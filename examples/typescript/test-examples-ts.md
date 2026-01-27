make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 855ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'zm70CzAPFczwVhULfnx-12iciGfT6dBGoFMF8PiP2ytbCbkJjQED-ezYCoziyLEv4yyFfZC14IwL2HUJAOFArg'
}

=== Find by ID ===
Found: {
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  metadata: { key: 'value', nested: { deep: true } },
  value: 42,
  price: 99.99,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  id: 'zm70CzAPFczwVhULfnx-12iciGfT6dBGoFMF8PiP2ytbCbkJjQED-ezYCoziyLEv4yyFfZC14IwL2HUJAOFArg',
  active: true,
  name: 'Test Record',
  created_at: '2026-01-27T00:56:33.851Z',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-27T00:56:33.851Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  metadata: { key: 'value', nested: { deep: true } },
  value: 42,
  price: 99.99,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  id: 'zm70CzAPFczwVhULfnx-12iciGfT6dBGoFMF8PiP2ytbCbkJjQED-ezYCoziyLEv4yyFfZC14IwL2HUJAOFArg',
  active: true,
  name: 'Test Record',
  created_at: '2026-01-27T00:56:33.851Z',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'zm70CzAPFczwVhULfnx-12iciGfT6dBGoFMF8PiP2ytbCbkJjQED-ezYCoziyLEv4yyFfZC14IwL2HUJAOFArg',
  value: { value: 100, type: 'Integer' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { value: '2026-01-27T00:56:33.851Z', type: 'DateTime' },
  price: { value: 99.99, type: 'Float' },
  name: { type: 'String', value: 'Updated Record' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: fl_T3NeP_JKhsToTP3ughF8ra0o4-4URqNXw60o_y-2050rVCz7k3OV4dkMo2ep0gRbFYVz3fiTX_AS1b5VKnA

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
Collection created with first record: fbc64Xa00CAywrU0mYhXXFGzMgsqBkq40IPA6qNeW00yoLPNyqIaz_MkZ_pghtPYE1sqncSmu6fx4V_oZpjSog

=== List Collections ===
Total collections: 20
Sample collections: schema_employees_client_py,scripts__ek0_testing,schema_employees_client_go,ws_ttl_test,test_accounts

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
  cache:product:1: { value: { price: 29.99, name: 'Product 1' } }
  cache:product:2: { value: { name: 'Product 2', price: 39.99 } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

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
Created Alice: $1000 - ID: YRPItypastjnZJw404xIoiIrt20C7ZkAQ-Cc394rNi2k5bzuX1ZVn7PrASCAbFFKobXVZy-xwVDYqoKZAo-Sow
Created Bob: $500 - ID: ZiKvz39TCkmpFNRZl69QbXF6iLq_guYZpVNJt0dz40RLXhIBusz1vsyBONHllVsGJpw5NmP9tbhM9HWJCY6JYg

=== Example 1: Begin Transaction ===
Transaction ID: 898acf0f-e37b-4209-b124-a643a599b5fd

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 97a3e7f3-1481-4b36-b6d0-33a2f1e55a87
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
  1. Score: 25.740, Matched: name.value, email, name, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: bio, bio.value, title.value, title
  3. Score: 26.400, Matched: bio.value, title, bio, title.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: bio.value, title, bio, title.value
  3. Score: 39.600, Matched: bio.value, title.value, title, bio
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.766, Matched: 
  2. Score: 0.753, Matched: 
  3. Score: 0.747, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: title, content.value, content, title.value
  2. Score: 1.506, Matched: title.value, content, content.value, title
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills.value, skills, bio.value

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
✓ Inserted document: JLtMdDcRu7uV7VTMbpgWhExcXg0PRYq3r4b0AarFWgjPx5Yv5VWG3y3s1GuzJ6l1Irf2VoIBFJHXD5CGMbrjtA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: QyPtSs2IOYVTjEWyoc6Wy_5HarvAjzq4Cf-ER2yY8tiBLiUWZmwoc033P6485B71Z-NfayWVQkpwVE219nIo8A

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
✓ Inserted document with TTL: IJ7ROvkpnee3JtSI3sSpmECMMVByF4-ZmxJ5UFP_jbZQvrPGhU6nI0GzvM-zSD9NijYs8LASDFPkaxjdZ8naxQ

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
✓ Edge cache script created: U1tZC-7hrhEUP8tzMoOK1ZcYEfMtMbD_bN6z_ObebHqsWWEsu1punJ9wbJFaRoCdonvi-s510rfWDff_vtxeZw

Call 1: Cache miss (fetches from API)
Response time: 64ms
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "username": "Bret",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "city": "Gwenborough"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
Response time: 4ms (16x faster!)
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "username": "Bret",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "city": "Gwenborough"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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

🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 9cPOp0TR4yO0rDeAcImZ7KMVUCXezYZnA9e4c-4zQRTcHr0KGD-_zvoGcq3R8IGCMFkv1M5gDMPdKGXFLZ_YoA
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: bTQADpofRAZsKgo4-r6r9IhCY23feGyofKNmGO6eLpPFarX_2tJ9g80mWCTc0GJ-9F-gRAuYZXHZG4kx8GbuJQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: MOiD8_fcciGNsrKzKSGi692dd0PVHBOwDj4sJcpw9M9QVczc9tnDQ-A6ZkKQwjowjBKipDaIQt9C4CWEBaOW2A
📊 Statistics: 3 groups
   {"avg_score":60,"status":"active","count":5}
   {"status":"null","count":5,"avg_score":60}
   {"count":5,"avg_score":50,"status":"inactive"}

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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "username": "Bret",
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "geo":...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "username": "Bret",
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "geo":...
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
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 68dCdmFb5_X3ApEHKCcRpd6HGEgJCT5_JYXEKFrVYHEbZZYQFy1chf7rLIg7lNHQGgocBWqYnGf4NLy_5z5Sbg
📊 Found 2 product groups
   {"avg_price":474,"category":"Furniture","count":2}
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
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

✅ Inserted order: saw9tYSWr8lgh8F24VH7HQ1if5LgAKIIWQQHvi10zf5DCRXRqaejRy_pXp7NanpdhuET0QchloiVTAHHV48Mfg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: RoG3lrDmaaY2ugq7v08XU95Al2TsZxw45S4y7_a5iDPY6NU1CG0_5bR37bART8ysQtPffYQy0ivI5-OMVdPMXg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: XaLBRC9p1E-a76dnLhUnWo8ySxht3ntV04gTpffdgdnoyfyqHu5Avq22dl06V9K9HcAJJugpe4C2H9Sybo6xPg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: FLxvt3rqUJulae0sXa_p8y9F2BUGgcSQJns_fAnAca-j5FOpWxD-vf0okolFwIro-1y41sTGb4tUgkG-XBnlDQ
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
✓ Created SWR script: fetch_api_user (r7cDDBQyzmfPP2gr5HY7iNMWeXJxeneOGFPREcWgC-PK35iPMnQPIHGdNB7DPn8TX2fDeWSKplxSc6zA8eXwRA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "id": 1,
        "username": "Bret",
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "street": "Kulas Light"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "phone": "1-770-736-8031 x56442",
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "id": 1,
        "username": "Bret",
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "street": "Kulas Light"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "phone": "1-770-736-8031 x56442",
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (Ji1PPGoL6QXSXn8nHgmyRfgQSJ0lgWKv7lbaE7NfkyGkV2aWtdCP5HbgraC_2ck32yPdHKJMu41RwZUtVAkUbQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "availabilityStatus": "In Stock",
        "returnPolicy": "No return policy",
        "sku": "BEA-ESS-ESS-001",
        "dimensions": {
          "height": 13.08,
          "width": 15.14,
          "depth": 22.99
        },
        "meta": {
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "barcode": "5784719087687",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "reviewerName": "Lucas Gordon",
            "rating": 4,
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "lucas.gordon@x.dummyjson.com"
          },
          {
            "rating": 5,
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "stock": 99,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "category": "beauty",
        "discountPercentage": 10.48,
        "id": 1,
        "brand": "Essence",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "warrantyInformation": "1 week warranty",
        "price": 9.99,
        "weight": 4,
        "minimumOrderQuantity": 48,
        "rating": 2.56,
        "tags": [
          "beauty",
          "mascara"
        ],
        "title": "Essence Mascara Lash Princess",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula."
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
   {"count":5,"avg_price":367,"category":"Electronics"}
   {"category":"Furniture","count":3,"avg_price":365.6666666666667}
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
   Vector databases offer benefits such as efficient storage, precision in data representation, ability to perform complex spatial operations, and the ability to store topological information. They are particularly useful for applications that require detailed geographic representations like CAD, 3D modeling, and urban planning.
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {"role":"admin","count":3,"avg_score":20}
   {"role":"user","avg_score":70,"count":7}
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
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":2,"category":"AI"}
   {"category":"Database","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: NhhawogDR-W1uyrubf4Vy3LpqE-Mm_jmIIxwHzWJ2SHLplFr2q2vhzNbxdlZiEvsdOHBNp8QHDfD4gdK_rv-Jw

=== Sending Chat Message ===
Message ID: gyFZaZH7adVx1GuoH_T3rrpl2of6YZCnbDNyga_m2yI7ef-Jow3TunjL7gLEokD7EEUNeTacrQIz0uwTr6Tv8g

=== AI Response ===
There are three products available:

1. ekoDB - It is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Pro - This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB Cloud - This is a fully managed cloud database service product. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'aWCyNmdBaBEjM1yz-I1PCyRDafwLLlFlXx7tR4ZgTqmDdFyAq0lZl7mu_DbceTtxANo2HhTw1JMdYjqceCA-3Q',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
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
    description: 'Enterprise edition product with advanced features',
    id: 'UQojL2-9T6-Db5-0hoQtt_kldgObkSn9lso8WFvtkdbcx6LaZqRkhcPeEeHUEkaNm0M3DbBeZYq6OV_hgMefpQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    id: 'vcKn6dlmmyobEvJcdDjpcgo4neh49x_o8zAK6LsJpL-2jdVln5d1IUn7tbx1qTGbYl_vJ534_pkMQdRJyasu2A',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2575ms

=== Token Usage ===
Prompt tokens: 449
Completion tokens: 84
Total tokens: 533

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wLZ_6XQKq5Rgjh3Aea5l3lJOC_S_xeCt45XFM5854aQ721fGpbM8Ju3mzSQ4DrEsMDpep_BpM_y_jueizY8YSA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, there is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: MYsswK26dIhK0ipLj8nJFRTbG8IQiyF_4mKpg5mEtX4HXEnEH7Vew-lQtKJa9eHlLmxdL_cfNftqvm8d8SR6BA
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
✓ Created session: dpg_aOZtkhEvHo8iJpUKoqa55h9hgs3sBNiM71LBROeSc3an4F6XM8JhUYKx016FvWx96YXa51QLZQ0Po9a26Q

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product available is "ekoDB". It is a high-performance database product. The price of this product is 99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: bDNIt_1wlrgBY9HFzv-Ns8k6cT3Oo5xPsuW90Bfy8dBPvcTzfwZT2-xKP8Y5ge1w6_RboaoUpOm9RZXjn0et-Q
  Parent: dpg_aOZtkhEvHo8iJpUKoqa55h9hgs3sBNiM71LBROeSc3an4F6XM8JhUYKx016FvWx96YXa51QLZQ0Po9a26Q

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: bDNIt_1wlrgBY9HFzv-Ns8k6cT3Oo5xPsuW90Bfy8dBPvcTzfwZT2-xKP8Y5ge1w6_RboaoUpOm9RZXjn0et-Q (Untitled)
  Session 2: dpg_aOZtkhEvHo8iJpUKoqa55h9hgs3sBNiM71LBROeSc3an4F6XM8JhUYKx016FvWx96YXa51QLZQ0Po9a26Q (Untitled)
  Session 3: MYsswK26dIhK0ipLj8nJFRTbG8IQiyF_4mKpg5mEtX4HXEnEH7Vew-lQtKJa9eHlLmxdL_cfNftqvm8d8SR6BA (Untitled)
  Session 4: NhhawogDR-W1uyrubf4Vy3LpqE-Mm_jmIIxwHzWJ2SHLplFr2q2vhzNbxdlZiEvsdOHBNp8QHDfD4gdK_rv-Jw (Untitled)
  Session 5: iNZEpgS6r8Uqw0x8SEIEnhHg4JXAtjjC-KqEzM7n8XtjCj_svrdOBRIH8JMbHviLioiCVq76SLfW74PxEZma7Q (Untitled)
  Session 6: woXubsbUeD5ZkrfMPFhh4KzorIMwlqY44nzU-yATGRo-tEmr6MvBd3MketHD7BynjDK0wDfBu7EvT86fiOCBHg (Untitled)
  Session 7: hOirRCXEPeKhjl3cqR-FjGWJYz-CC5616S4qL_3_e5F9XF2l-2x_HP-5_OkQmN-iOwXBNvT7oE538Kvzwf8R4Q (Untitled)
  Session 8: SMtDTtUsPPGtex-Ojt2Tj_Ve8q93_4P8VE49G97Cqp1WuMODqN66buqX9WkHOJlHD56oTCxFfDjbDht_dAd4xw (Untitled)
  Session 9: vEMkVSZyHWnylKAfzuZhIfSJHFddMGWFbGS5usQHj1mOghiTFrJwKrtlOnm4q7P9cQjYzNv780LUL5XkrObmOg (Untitled)
  Session 10: DIyovkFIAzMyJ-y-TF-R3WXH-CnHmRyC3kFLhWSqFybg7QqLg6KzmNnjWLg4V4xbYSnZ3PbuGBPFJToeMYMbug (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: bDNIt_1wlrgBY9HFzv-Ns8k6cT3Oo5xPsuW90Bfy8dBPvcTzfwZT2-xKP8Y5ge1w6_RboaoUpOm9RZXjn0et-Q

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'nMBt5gnJOq02eT5GLCNU6jKNAuY1SPxgf4L4L92Nm3aDGSge0QWlVm7RGix4LZiSh8V6KUrQz7b2-qfc2Ia2uQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { type: 'Integer', value: 29 },
  id: 'nMBt5gnJOq02eT5GLCNU6jKNAuY1SPxgf4L4L92Nm3aDGSge0QWlVm7RGix4LZiSh8V6KUrQz7b2-qfc2Ia2uQ',
  active: { value: true, type: 'Boolean' }
}
✓ Second upsert (insert): {
  id: 'qyITbjqJK5aW3d08vQBqyUQDIQD546wzhNgrvGimPid-RT49GuDXAeOQHW9sb7PuEOOSzGPBmMhD54I6rmZ8JA'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { value: 'Alice Johnson', type: 'String' },
  active: { value: true, type: 'Boolean' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  age: { type: 'Integer', value: 29 },
  id: 'nMBt5gnJOq02eT5GLCNU6jKNAuY1SPxgf4L4L92Nm3aDGSge0QWlVm7RGix4LZiSh8V6KUrQz7b2-qfc2Ia2uQ'
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
   Inserted with ripple: {"id":"OoZ7czRO5r9aMcZ_tv3HBowDT8QpiVjVj0b_-p7EYU9bXNGXXDQzbneeHY0yacdjFyvEISTjZxws4cSeIiuFIw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"NEIhIQBt00k3p17G3gDtWlTPGeI6kLHCO0_RovlwTpqBQwLd6Z-EnO-fjfmUD5AGYsVve2vOFRWnKZRaHtNlQg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"id":"OoZ7czRO5r9aMcZ_tv3HBowDT8QpiVjVj0b_-p7EYU9bXNGXXDQzbneeHY0yacdjFyvEISTjZxws4cSeIiuFIw","name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"2Joo2k0hMnKDacj9fkp5SuwfhtJN6whwtFdy84xoZkDrIrju3dy40LDc33PTwpN20n9uN4lR8xvBGYyHxNkbqQ"}

✅ All bypass_ripple operations completed successfully!
✓ Client created

Setting up test data...
✓ Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: [object Object]

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

🧹 Cleaning up test data...
✅ Cleanup complete

✅ All projection examples completed successfully!
✅ [32mTypeScript client examples complete![0m
