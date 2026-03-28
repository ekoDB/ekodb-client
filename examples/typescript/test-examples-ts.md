make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.1 prepare
> npm run build


> @ekodb/ekodb-client@0.15.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 956ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'u1sMXaINTOFp8k4yTW2z9UBlvadSKujT3iVmcyvidix2Kf6zdsoWUUQyou6F0ZUNMHCVWUiJNU2mhJauYuIMOQ'
}

=== Find by ID ===
Found: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  created_at: { type: 'DateTime', value: '2026-03-28T21:47:36.334Z' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  price: { type: 'Float', value: 99.99 },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  active: { type: 'Boolean', value: true },
  id: 'u1sMXaINTOFp8k4yTW2z9UBlvadSKujT3iVmcyvidix2Kf6zdsoWUUQyou6F0ZUNMHCVWUiJNU2mhJauYuIMOQ',
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-28T21:47:36.334Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-03-28T21:47:36.334Z',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  price: 99.99,
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  active: true,
  id: 'u1sMXaINTOFp8k4yTW2z9UBlvadSKujT3iVmcyvidix2Kf6zdsoWUUQyou6F0ZUNMHCVWUiJNU2mhJauYuIMOQ',
  metadata: { key: 'value', nested: { deep: true } }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  active: { value: true, type: 'Boolean' },
  created_at: { type: 'DateTime', value: '2026-03-28T21:47:36.334Z' },
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: 'u1sMXaINTOFp8k4yTW2z9UBlvadSKujT3iVmcyvidix2Kf6zdsoWUUQyou6F0ZUNMHCVWUiJNU2mhJauYuIMOQ',
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: V5SLQnzdVjChHCF0XHmAyMzMC_dBQN78uBLsziSfOF3kGUMBDs5LrPSilhXqveeT4USwy_ulhxBKbRFzAzA3Ww

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
Collection created with first record: CzhJ05GyE2f02XCYLHlYQA_-HvKKxTjaekfAoZLyoGL0-NsvEHw6OfxWcPShHt2n-yw9GyXfT67JeIvE2h9Dfw

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_go,ws_ttl_test,audit__ek0_testing,products,batch_users

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
Created Alice: $1000 - ID: q46NtavRFLqcsjDgMSPonGnxIn6_jFye9O5x6UB_DCsgobGFCA5h_YZ3EOMwjfAIuOgRypf_mUVQ31i9xFdVWQ
Created Bob: $500 - ID: eKObUvPpe0K5yEFCokPVCFib64L1aMqqfPYpEEXgBjqCG0H4fCEnZfjc7cfHJBP66fDBq5YtJAj9C1fWTGngnA

=== Example 1: Begin Transaction ===
Transaction ID: b76e8fc5-afd3-4e07-a689-07638652a16c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a27bf9b2-6a1a-4c15-b1ee-d1a5d3c1ab15
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, title, bio, title.value
  4. Score: 26.400, Matched: title.value, bio.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, bio, title
  2. Score: 39.600, Matched: title.value, title, bio, bio.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
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
  1. Score: 0.769, Matched: 
  2. Score: 0.750, Matched: 
  3. Score: 0.731, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.708, Matched: content.value, title, content, title.value
  2. Score: 1.493, Matched: content.value, content, title, title.value
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, bio, skills.value, skills

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
✓ Inserted document: fypTY-dQI353wWgIco4n_UTnbfmLSLLie40kIgTh4oqVXK3I47duMcQooFJ7DawiV3u0GuS0_I4bMn7sphxvvQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: lDS_uI4rFLBvZX2goA6zBPUwpBS9TPgWKBMN99v6Z1fFFVe8ANivb4y7ymwu8WEqgCHm8ToL9banGmMkNAbz7A

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
✓ Inserted document with TTL: pmLDaA5f_ykQ-jo2D2tUWqnvcwQLVYNBv1uFVmXxrOcIsoHobDGL8fYwM2JnqmG9204U_C8QyIK29G8TuoLXNw

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
✓ Edge cache script created: sc_EoIyfc-7-c3CpSVhzEyYW0AbRhD7LvYbRC3co3Y4G_1q-9dp21IK3EG2McxWXGw7SkeELUITlt3eVqI7wPA

Call 1: Cache miss (fetches from API)
Response time: 147ms
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "id": 1,
        "name": "Leanne Graham",
        "username": "Bret",
        "address": {
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "email": "Sincere@april.biz"
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
Response time: 41ms (3.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "id": 1,
        "name": "Leanne Graham",
        "username": "Bret",
        "address": {
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "email": "Sincere@april.biz"
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

✅ Script saved: Oe0JQr0idliWpDbumTkA1JgYgqA3Y-pUzxyWkqannFduZvmtzPa3xkyTBzqwS3ojopUjStjQQJL57B3ViH_Rxg
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: uyqRDUC9bImHkwvzoybWUTNt1nqtVfm8NXkFX_ADrB5hMeF2UIP5sSKQrcQRo2cqlHiM1Tu77IFCmw511iDjSQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Hcz8mRRU48nWKoE26-G0IBWPApkzE_kxewfiVfkBjFP0PDV2qQNvtEpJ8KKjggP3AiHQgBtyns-39mbM3MPWew
📊 Statistics: 3 groups
   {"count":5,"avg_score":60,"status":"null"}
   {"count":5,"avg_score":60,"status":"active"}
   {"avg_score":50,"count":5,"status":"inactive"}

📝 Example 4: Script Management

📋 Total scripts: 22
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
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...

Second call (cache hit - from cache):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...
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

✅ Script saved: WaWGEs-KmAuwED84v9KI2OUZVT-Uv651JcZJmX5V7Qzx_r3LmGLA6Rh2IiYxYr5f4xUc1S_NBpoj5TKGWWMe1g
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: g__UU1ijGxdFjgg5etAVNiUdvsVEvy6ShUtar_csMhw6YQbvFrRgrQy31nQRDX4J92iTNCUsQ-jK770R3Z1Buw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: g2GQTt-fZ46fIVgleD-1YT53WZeYE0hOhVKdW8deKXIVhVj_iueRWcxFOAFS72N6b3Uif17jHdEh8mhaG_ZxrQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: fGjh_GI94XGueUA2Kc216M04filfB9bTifgpzALWe3j9hHK2GfDyVsteJaV1_ilt4VTiRGnCY8puALET5lt8jA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: GsxIyimQLYKDVIUghOZ6HzCqrhzIXaDTzjz_B6pVKKM6ERVeWKymH-YAjE01PCuqg2uJO3kg9T9FNBed4lQyog
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
✓ Created SWR script: fetch_api_user (rH2a7sWu4lXbTuGICPYLgBTWC9IVVO5cZ6q1w2DrUXT20V8l30eFaIJPu2OHC-MQLw3r2MIwTTF1MXCf0n1c2A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "username": "Bret",
        "email": "Sincere@april.biz"
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
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "username": "Bret",
        "email": "Sincere@april.biz"
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
✓ Created enrichment script: fetch_product_with_reviews (MfddG2eZwXnT22UIVxKaCvVcUbH5pksm0_KJRlhxGjVOSkng_G6YTv2sCavaV0gsoRWi2LoxjEs4s6T69r4vhg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "rating": 3,
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          },
          {
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "rating": 4,
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerName": "Eleanor Collins",
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          }
        ],
        "sku": "BEA-ESS-ESS-001",
        "price": 9.99,
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "availabilityStatus": "In Stock",
        "meta": {
          "barcode": "5784719087687",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "warrantyInformation": "1 week warranty",
        "weight": 4,
        "rating": 2.56,
        "brand": "Essence",
        "discountPercentage": 10.48,
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "returnPolicy": "No return policy",
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "category": "beauty",
        "minimumOrderQuantity": 48
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
✓ Created native SWR script: github_user_native (g5eUjaQhYsz6jr6LCwDhoDF6xVp365e4KjsaRB3jslhtp0bSaYateWXNgKcVvdWpN9nNlF1DPMNPm6K8Z964yg)

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
✓ Created SWR script with audit trail: product_swr_audit (igPgyTjE8TcYJpzaByJV32ucmDpvmuv4_0WV50dkxO_vw1AT81_rdiR3NYOqJb8mHfioS8Y0h--EUUQ34-KAmg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (OV8Hq90QlAFyMVBcB7jlBPqx1n5KxNRX8-qp9mutkir6EXB_jTaMDUhsXryoZYf6yo22TvtYgNYXf1xAhSNX1g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (Sj7gljL-fStJ1VuMnPLt3PvJcZmOFQa8VaTOQpf3urKddXat1S_Dl5pzTnMAVEchAVx6z-GvkwQcgGecc2x9Bg)
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
   {"avg_price":367,"category":"Electronics","count":5}
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items based on vector embeddings, which is crucial for applications like recommendation systems and image search.

2. **Scalability**: Designed to handle large volumes of high-dimensional data effectively, making them suitable for big data applications.

3. **High Performance**: Their architecture allows for optimized storage and querying of vectors, improving performance in ML and AI applications.

4. **Flexibility**: They support various data types, such as text, images, and audio, enhancing the breadth of use cases.

5. **Integration with Machine Learning**: They easily integrate with machine learning workflows, facilitating the storage and querying of feature vectors generated from models.

6. **Advanced Indexing Techniques**: Use of techniques like Approximate Nearest Neighbor (ANN) to further speed up search times.

7. **Enhanced Retrieval Accuracy**: Provide better retrieval accuracy compared to traditional databases when dealing with complex, high-dimensional data. 

8. **Real-time Analysis**: Support real-time querying, which is beneficial for applications requiring immediate results. 

These advantages make vector databases particularly valuable for AI-driven applications.
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
   {"role":"user","count":7,"avg_score":70}
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
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
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
✓ Created session: i-BUzL5C2HrCbpnbd0AFuVafXbkVVZTFFUmBhONDsxEu6V3nsJBaxZIsxWxd1we4-zkOhR_yG4V4GWuPhJdl3Q

=== Sending Chat Message ===
Message ID: HQQwzwb5ggooNl41olApnqdvxTRPTE9FnFhdKyHpMYIq0lZKr_UM2uY0dXvCulI8jbRPmHmkrTHW1ZFJPPud2A

=== AI Response ===
Here are the available products along with their prices:

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
    description: 'Fully managed cloud database service product',
    price: 499,
    name: 'ekoDB Cloud',
    id: 'fSSZkG0sl_shw7Zg7-RYhZBbYBRtJV7ZbTbtwEqtwkxbr-9LdoxZkhVO4Ox8BFMPCA5bier2vqILNr1n-TQdSQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'V6cYON17TARNGR69rek7ccmBRyfq98SHkcQsJZphNWMn8wURGmcin0c-VOwi3KfyngCtVlTFauU3T8D6f7c8WQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'U8A2G8opXjZtCOJg7IfRCV2dvRjNhEimz6oD5grk3kWS1hnNXtszVJN15yIlLu09q3cHWZojKwD1w6tzfcHzNQ',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2032ms

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
✓ Created session: AFeU8FkylYyVbIyeg97wuoH5lA0wFLgv3hV5WRINRbv9kTFBCgA1smAndTMczYQyVgkG5ePxWRiLGvpj1hAJsw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or would like to see additional products, let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: cEDZEwnAiPqPjGQvqTYxwxrjJZyEqAy9ETnuVnZGbmvmU11zpr2pTDfnhAhbxgZn-OUc3iEiklbm49mxRRazVQ
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
✓ Created session: sx_rMnfqErh-mtyjHAyXFsabMqi00LrFPcyYpdKVjsVkC2qDvI96XR7o-5mkJ-n1g--B4q9yh2ZvVSS9th69DA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you would like more information or details about additional products, please let me know!

✓ Message 2 sent
  Response: The price of **ekoDB**, the high-performance database product, is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: X8TTrHDVeFd6iAWoUI1DDYB61iKXjpSGZhdm4-4FFlvvaC63RF1GE0deQucwiNxv1sPdmmAe1w5n_5oh9Dmf6g
  Parent: sx_rMnfqErh-mtyjHAyXFsabMqi00LrFPcyYpdKVjsVkC2qDvI96XR7o-5mkJ-n1g--B4q9yh2ZvVSS9th69DA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: X8TTrHDVeFd6iAWoUI1DDYB61iKXjpSGZhdm4-4FFlvvaC63RF1GE0deQucwiNxv1sPdmmAe1w5n_5oh9Dmf6g (Untitled)
  Session 2: sx_rMnfqErh-mtyjHAyXFsabMqi00LrFPcyYpdKVjsVkC2qDvI96XR7o-5mkJ-n1g--B4q9yh2ZvVSS9th69DA (Untitled)
  Session 3: cEDZEwnAiPqPjGQvqTYxwxrjJZyEqAy9ETnuVnZGbmvmU11zpr2pTDfnhAhbxgZn-OUc3iEiklbm49mxRRazVQ (Untitled)
  Session 4: i-BUzL5C2HrCbpnbd0AFuVafXbkVVZTFFUmBhONDsxEu6V3nsJBaxZIsxWxd1we4-zkOhR_yG4V4GWuPhJdl3Q (Untitled)
  Session 5: k1E8eIVqGqE-lO_7PkwnwtlQtLl2BUNR-B2feKtveL_8ak_ZAuzJeBKfLFkf_zI-KDxeX90IC5wEWVusJSscMA (Untitled)
  Session 6: 1gjmtoMHROg0KCa8IbrFbAQEV4A-em9sQzlFUQeQgdL-rCg0DPsL2p7KS9KY8zml-OlqB47tmELFAaSLOkddqw (Untitled)
  Session 7: pzOmm3GrdXAkvNrQvEDpLh7CvHMf-1VaM0kvPSika5LbfNwnJcNf5IQaEpCvYaancEPji9cden_C4jNJTfDDcQ (Untitled)
  Session 8: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ (Untitled)
  Session 9: jaaT3Vfrj3B45QAf8_r32GkVnIiUv6kshpVNj2mn7m7EavDP9Lsw_ae_nu16Y5DI3ZQ8BiHlV3Q4VbHiytX-NA (Untitled)
  Session 10: oSTbHI3HIfQPb12V5a71f57bJiy6odGOwleAbeB35d0aLyVVKHQDLIJaJDqfBEi770Qav9ZcpgPyr9k7AywD8A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: X8TTrHDVeFd6iAWoUI1DDYB61iKXjpSGZhdm4-4FFlvvaC63RF1GE0deQucwiNxv1sPdmmAe1w5n_5oh9Dmf6g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'lqKnXY38T-oEOgAHZEifKxRuP8KC6yvQaC1gZBd0_aaekoVgAPm3kaqdtTMvg1yTlTk1jCQGgT-lAg7PWnJSYQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { value: 29, type: 'Integer' },
  id: 'lqKnXY38T-oEOgAHZEifKxRuP8KC6yvQaC1gZBd0_aaekoVgAPm3kaqdtTMvg1yTlTk1jCQGgT-lAg7PWnJSYQ'
}
✓ Second upsert (insert): {
  id: 'IPLC8WvgenR0aFyVnTNyo9tY3eB5n4OFHD4t0dtMyJ6OTc9earE5Hb3yYvQUoDFJTO8MT8Y2TGAovmGSMZy-eg'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { value: 29, type: 'Integer' },
  id: 'lqKnXY38T-oEOgAHZEifKxRuP8KC6yvQaC1gZBd0_aaekoVgAPm3kaqdtTMvg1yTlTk1jCQGgT-lAg7PWnJSYQ',
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
   Inserted with ripple: {"id":"8SheOZd3PtHT1Q-wEle5MlgVo5tYv6g_Kddr5jhogVy4w7aeeNaWW76E9PpPrbL2yym0rVDj2SMe_d52u5lz5Q"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"GkgX9Cuixc3kB3-Bp_l4W1P5xyei_QfiPcyKPuXTZJ4unGMbeJ8B-m0MXa55FP3Nw19zZAmNXT1F_RO0UdjK_A"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"},"id":"8SheOZd3PtHT1Q-wEle5MlgVo5tYv6g_Kddr5jhogVy4w7aeeNaWW76E9PpPrbL2yym0rVDj2SMe_d52u5lz5Q"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"type":"String","value":"Upsert Product"},"id":"custom-id","price":{"value":500,"type":"Integer"}}

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
  Fields returned: ["avatar_url","email","status","bio","user_role","name","created_at","age","id"]

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
    - Fields: ["status","avatar_url","bio","age","secret_token","id","user_role","name","api_key","created_at","password","email"]
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
Created user function with ID: 2RD27jvfTYwuHWVsrHfkUCYLqV5l60_j54JtICKwJh1AvzUGDqxjE_WX38DOl00LZb07cqaUq51AoB1PPHo1xg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - cache_api_call_py: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_api_user: Fetch User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_ts: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call: Cache External API Call
  - get_verified_user: Get verified and validated user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
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
