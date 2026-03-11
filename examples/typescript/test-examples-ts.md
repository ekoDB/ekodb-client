make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.11.0 prepare
> npm run build


> @ekodb/ekodb-client@0.11.0 build
> tsc


up to date, audited 48 packages in 1s

14 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.11.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 816ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'SHcIf_naC25ePk5aA3GBnHLO7GhkkpJHhgdqpA3fdsJSNfLweqRrqwZOllk1Nq2pkNvC1LhY7OE7fRGibzoN1Q'
}

=== Find by ID ===
Found: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  id: 'SHcIf_naC25ePk5aA3GBnHLO7GhkkpJHhgdqpA3fdsJSNfLweqRrqwZOllk1Nq2pkNvC1LhY7OE7fRGibzoN1Q',
  value: { type: 'Integer', value: 42 },
  price: { value: 99.99, type: 'Float' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  created_at: { value: '2026-03-11T05:06:58.538Z', type: 'DateTime' },
  active: { type: 'Boolean', value: true },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Test Record' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-11T05:06:58.538Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'SHcIf_naC25ePk5aA3GBnHLO7GhkkpJHhgdqpA3fdsJSNfLweqRrqwZOllk1Nq2pkNvC1LhY7OE7fRGibzoN1Q',
  value: 42,
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  metadata: { nested: { deep: true }, key: 'value' },
  created_at: '2026-03-11T05:06:58.538Z',
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'SHcIf_naC25ePk5aA3GBnHLO7GhkkpJHhgdqpA3fdsJSNfLweqRrqwZOllk1Nq2pkNvC1LhY7OE7fRGibzoN1Q',
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  name: { value: 'Updated Record', type: 'String' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  value: { type: 'Integer', value: 100 },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  active: { type: 'Boolean', value: true },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  created_at: { type: 'DateTime', value: '2026-03-11T05:06:58.538Z' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Cc3CdMRxV5pAHM3ZqWfz1XXlcOuNJFp9B-Y-qqlB-uOueOckvKTaARCArkrJh3d_nhvwWUQrBRxBVYmIZB9YNw

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
Collection created with first record: gbh5ht00S-Y3WsEiJZie2nDn4jExsu52IiyoEHxP8B_o-QbK1pLvGRjY_uPU2AMem3bcDKJb0ixuqoFm_rKoiQ

=== List Collections ===
Total collections: 37
Sample collections: schema_documents_client_go,ws_subscribe_example_go,schema_employees_client_js,schema_products_client_js,chat_configurations__ek0_testing

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
  cache:product:1: { name: 'Product 1', price: 29.99 }
  cache:product:2: { price: 39.99, name: 'Product 2' }
  cache:product:3: { name: 'Product 3', price: 49.99 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 15

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
Created Alice: $1000 - ID: 4gRmjmAKxDQrRLbtUQphqQWbQKs-BoA2jxhtmSET_i8OZ0ali-fDxTWTo54s95CYfrzKemhIcbpkzO2ea5ElFQ
Created Bob: $500 - ID: oxlLfLTWAOeZlAR96oRGc7logiueXYSGF3iT-6k4FO6ZSN2ELbjLyUA013ZhElLaEYtgN0-6lcZOdcQV0IGNoA

=== Example 1: Begin Transaction ===
Transaction ID: a3e6c3fd-860a-4a79-af48-ff8a25a99f3d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: bcbf06be-6a2e-4c13-9667-82d4eef8e70d
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
  1. Score: 25.740, Matched: email.value, name.value, name, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio, bio.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: title.value, title, bio, bio.value
  4. Score: 26.400, Matched: title.value, bio.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, bio, title
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: title.value, bio.value, bio, title
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.756, Matched: 
  2. Score: 0.753, Matched: 
  3. Score: 0.728, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.701, Matched: title, title.value, content, content.value
  2. Score: 1.502, Matched: content, content.value, title, title.value
  3. Score: 0.291, Matched: 

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
✓ Inserted document: myYzOifl0l36XINFAmW8EXshacc22ovzqhHI4qvGrT2mzHrtxlT785UMph8tU6tkjff-B3dGrgNRGwvg2cuEXQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 5sguiglKA4J-7HmBhqTgPRIS-oPHGTVblece6hCLsllSU-kfsYuNOyGiFuONeB-W9e716NmoDFVla2YUZ-_Phw

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
✓ Inserted document with TTL: keGRywRyM5J5DW9mMNjqPctqS97jN0Njy0utrXjRvHmz2x5Gd59imMs_vQkfVKf3nXHB0gaB3v9T7_cFieeRqw

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
✓ Edge cache script created: EShjE_-7GaUqR3rKFQ59IC33y8wiS-V88Rl3tmWfK2cIGpOAQ966glI4KnT9XaIvqy5rGsX2gZLtMika8Gt_9g

Call 1: Cache miss (fetches from API)
Response time: 80ms
Result: {
  "records": [
    {
      "value": {
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret"
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
Response time: 6ms (13.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret"
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

✅ Script saved: tlTiyrfVqivJu8iV_k8DEoah2lLdL_MFVSStfSIcBzE_lfAkkjCwbPEBccyi6d3Uxqu4fOd80MgSP6I7pv5JuQ
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: Fik5wC9cz3uQzad64n1umxYrlydFZ-2kjt05FkUN26SPEseZIqg7HENvCwjvWWTu9GUPfxeUzbdEyHjS_ExGaw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: rVMTpjE0gDSppQ6NUvQ8m1RX1xDN33ksp8uk-dD1hm-bsk16KWJsKIfJIch4bh_HQucAEzHFqbbs8DDj9QT5ew
📊 Statistics: 3 groups
   {"status":"active","count":5,"avg_score":60}
   {"avg_score":60,"status":"null","count":5}
   {"count":5,"avg_score":50,"status":"inactive"}

📝 Example 4: Script Management

📋 Total scripts: 88
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
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "city": "Gwenborough",
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "city": "Gwenborough",
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
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: GPDoinSLx0ZjwKbIVQJGD7Pjz8-YZYX3tDpsepbZNLtVhWW9hiyjuW9I6iMhVTjgSB8c8TxGfrRVyhT7Jgr_bA
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"count":2,"avg_price":474,"category":"Furniture"}
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

✅ Inserted order: qAA18Pr266O72A-bLpmAPGGYOtHa0PTCjE1RMpDM_8dZI9Sxb_XjuAmOV4XeKBrbncCJnOIuaXHE4o9ievFL4Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: ibQL-T1x30ppKJa-PXmexGgmGPXlJxSA_erLd25p-c_S7KIxHkUcBzRqrruf_hkGX6Ne82YVdh_n24V_-j6xGA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: qkNhbt_N8sHNquvQkjOpCmICfuAjUxCehgqdDFXjeLFXNgtaCXNZhq8j1r4FhERv6fho5we_T3VzCCTWcv0RbA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: CRFfG5muNsS0MYRiRlg_02z5IsD97TzG5j4XS2yycYY42TDLkPWB0hCphOTwgfniNSeMJMkAxgdeZj_duqr5bQ
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
✓ Created SWR script: fetch_api_user (CabOT_fPSFIRe1eGd-JjIRyycZb0ndqw9y3RhBwdPVRGkIaps60suOPNBS7906VVTZmi6Fh46VVTLPmP0QteEg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "username": "Bret",
        "email": "Sincere@april.biz",
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
Response time: 5ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "username": "Bret",
        "email": "Sincere@april.biz",
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
✓ Created enrichment script: fetch_product_with_reviews (pY8QW4z6sozoQvfu0-F8TJX2V-PHVR2HvAVOiYz5lj0o2uqrAMLswYcZRlIz7B4qSf1KGCDXFQG9kMw5POHlRA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "discountPercentage": 10.48,
        "rating": 2.56,
        "dimensions": {
          "height": 13.08,
          "depth": 22.99,
          "width": 15.14
        },
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "category": "beauty",
        "weight": 4,
        "shippingInformation": "Ships in 3-5 business days",
        "returnPolicy": "No return policy",
        "warrantyInformation": "1 week warranty",
        "brand": "Essence",
        "sku": "BEA-ESS-ESS-001",
        "title": "Essence Mascara Lash Princess",
        "meta": {
          "barcode": "5784719087687",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "price": 9.99,
        "id": 1,
        "availabilityStatus": "In Stock",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "rating": 5
          }
        ],
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "stock": 99,
        "minimumOrderQuantity": 48,
        "tags": [
          "beauty",
          "mascara"
        ]
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
✓ Created native SWR script: github_user_native (1lNwEPDaDcQIj5Q1I6xygFBf0ZlqzYljzGMMWsR-RBKhY_FHpzucFQvWkx8kIF36k4QS0le8Ke161eJS8MGJ0w)

First call (cache miss - will fetch from GitHub API):
  Response time: 8ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 7ms
  Speedup: 1.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (f0CYrZNyd_f-NywpxywIGWL-UhqMCJb7Iyj8p1HZ-xo9aMFoCYQwR7IUFI15CldJiwDi77unCh4XIc1s4JhNAg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (_vhkixInLwTPZgQyUsPMVJ5kUsCG_FbzBFXG8rEBjLO8ylnmw3iOlWiBqMais0UgHIQ0AUIppi2_YDh40c54NA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (HClyHy8nqUvrSHQMcF2sS4T6eO8pRPRG1Iv4I95DX_w-BefUhNOTvjGvS3yPp_zH9OISG2ENhmJqPwK8g6zBGQ)
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
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
   {"category":"Electronics","avg_price":367,"count":5}
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
   1. High Accuracy: Vector databases store data as points, lines, and polygons, preserving original details.
2. Scalability: Vector data maintains detail and quality when zoomed in or out and can be scaled up or down easily.
3. Smaller File Size: Vector databases often use less storage space as data is stored as coordinates, not pixel values.
4. Ability to Overlay: Multiple vector data layers can be overlaid for advanced spatial analysis.
5. Easy to Edit: Individual features can be added, removed or altered easily. 
6. Attribute Storage: Vector data allows for storing attributes for each individual point, line, or polygon.
7. Suitable for representing discrete data: Such as roads, buildings, and administrative boundaries.
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {"count":3,"avg_score":20,"role":"admin"}
   {"avg_score":70,"count":7,"role":"user"}
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"AI","count":2}
   {"count":3,"category":"Database"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 7WzWaO-_k65ftagvlYbGTUUc2IPgFZtqzaQN-3OdyiQvO5ihTm739mmGE9OaWVBA14UuxWP_HCCgVjksuecArA

=== Sending Chat Message ===
Message ID: sTZPP1kjxRNpBXCNIRG2PUJckz8d5GfjSx_4rDAMj3wlAwGD-ce9eKIsYtD4iLLe4IUk7ydFj7jxYp52VhxOkg

=== AI Response ===
The available products are:

1. ekoDB Cloud: This is a fully managed cloud database service product. Its price is $499.
2. ekoDB: This is a high-performance database product with AI capabilities. Its price is $99.
3. ekoDB Pro: This is the enterprise edition product with advanced features. Its price is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499,
    id: 'xVpvttM-hXpQv4xrYQV8W4Oa8Ydbng_OssO1UrsDlWkg0Wrssjk9YqXJawGMS6mjhBJboAuXGMCH0-tJO6M4Mw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: 'KD-kSBBh6qlgPGoZfR5Jv8DWx8eN3cDOv9-W79b3PCZtKMtnabnaBKhDDN-ngOBen04Fhhtjol19Sf5g0AhF1g',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    id: 'AvJ47Ap5DM4IlDWjrTQoicr-yyKQ8yN-6mg_Or5KRujpO9znP5282PYnWnmeyLUOBJ-qXANxjGXM1ZnKTKIInQ',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3003ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 74
Total tokens: 524

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: nUWTzHWZ-dX-dtJ3DC5MFRHmhlUJ9OApijwtEfG_Ocd6Y8VRwMss-PYeZThSUlOdfdaFoTOYJ55FHjfe6v4Nww

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is ekoDB, a high-performance database product. It is priced at $99.

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
✓ Created second session: syUDDGcZanFnWH2riQW4JI3JS9dnexrjHs1WEZhl-c7SMR4Orsuvdb0mRBuYipEQ2tdS_14sQ7ULTPjT1QWMfQ
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
✓ Created session: KGBDqDGylzhGPBadcvcxKjwKm1c0HIQuj7KCZBiJsqJ0FvUe30Koi_my84mDI6vhvjE8tWdYtzfO2VbTA8JlZQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, "ekoDB" is the available product. This is a high-performance database product and its price is $99.

✓ Message 2 sent
  Response: The price of the "ekoDB" product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: lvwwHUDqBnvq4LAsxso7Y0KJT1qc4DjZHJQ-lTJOvjem-RLlLeGsAXbET-CMok_fUKtXdyje6I6QqaHQPOoZFQ
  Parent: KGBDqDGylzhGPBadcvcxKjwKm1c0HIQuj7KCZBiJsqJ0FvUe30Koi_my84mDI6vhvjE8tWdYtzfO2VbTA8JlZQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: lvwwHUDqBnvq4LAsxso7Y0KJT1qc4DjZHJQ-lTJOvjem-RLlLeGsAXbET-CMok_fUKtXdyje6I6QqaHQPOoZFQ (Untitled)
  Session 2: KGBDqDGylzhGPBadcvcxKjwKm1c0HIQuj7KCZBiJsqJ0FvUe30Koi_my84mDI6vhvjE8tWdYtzfO2VbTA8JlZQ (Untitled)
  Session 3: syUDDGcZanFnWH2riQW4JI3JS9dnexrjHs1WEZhl-c7SMR4Orsuvdb0mRBuYipEQ2tdS_14sQ7ULTPjT1QWMfQ (Untitled)
  Session 4: 7WzWaO-_k65ftagvlYbGTUUc2IPgFZtqzaQN-3OdyiQvO5ihTm739mmGE9OaWVBA14UuxWP_HCCgVjksuecArA (Untitled)
  Session 5: DRavkpC7grjiyHyWQvY0pvKVm_ThJKy8IfyMiBUcirGFFBVw_JFsljqhMbqIm0n9yBrzSDuAhqi334Cx-dGsWw (Untitled)
  Session 6: c78LckyBlkvvU8eQYK4J2nLlh7fIdaD4jFfCR3rEp5KVSc9YldexJ16rwzaA_4I85FDxhbt-21k3NxhU5T1GRg (Untitled)
  Session 7: zsfTJPjo8BJW27P0qEtZ8_a5UhoJvv0n3b2HbNwaOCwU7Nov_kWwz2BRBkGtPwbeYUV9TGNROtHpCsO1L338oQ (Untitled)
  Session 8: 5-koefiH3sbQO341ilYlYK_OY5toxJC5d_pgjhiuQHQyMpKeT4QtjsFbEfFRnw2VRaFadDcquDugbBqfOniolw (Untitled)
  Session 9: hM5YYVmZtmGzOVtaRJcmpTAHVPGSKoEQ0lYcIGxoPQQpnnvNmLZAFXS2K29vmSF-cdd2B4o7O4YePM6Jf9u6IQ (Untitled)
  Session 10: 5xAXUajg47q6V-E_32OCJiQ01hyd3PNCnRneYDdTLcd5NM_tc5u9PSWf7j6c3nTmXxKr6ioRgg7Rrap-9Te0TQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: lvwwHUDqBnvq4LAsxso7Y0KJT1qc4DjZHJQ-lTJOvjem-RLlLeGsAXbET-CMok_fUKtXdyje6I6QqaHQPOoZFQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'svi9YsY1-4FUec4eSUMcHbEC0LHefIWRLDk14JdCaZ39mER4X2jJUznQ-3-Ai7xfwILmc_oYi_ixeG2UX442Sw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'svi9YsY1-4FUec4eSUMcHbEC0LHefIWRLDk14JdCaZ39mER4X2jJUznQ-3-Ai7xfwILmc_oYi_ixeG2UX442Sw',
  age: { value: 29, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: 'XBASR2N3wNpUr69hDV87omQHxylXCSGW5emMX-9s6Pow7aw5MS1cfb9FGlPZKPZpi0K3B_SxtV3skMcsdENZbA'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { type: 'String', value: 'Alice Johnson' },
  active: { value: true, type: 'Boolean' },
  id: 'svi9YsY1-4FUec4eSUMcHbEC0LHefIWRLDk14JdCaZ39mER4X2jJUznQ-3-Ai7xfwILmc_oYi_ixeG2UX442Sw',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { value: 29, type: 'Integer' }
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
   Inserted with ripple: {"id":"7EgpK6zvQEr12INg3ujRqj_RjRnv7-tBA2t6wdtpNwkl870W6-c09XNrjGVX6QGM-Kx9GovM57Mt_-RS8iDWDA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"3GCjnKU1ZQCZj0KLQSmZC5Z_zJHmIEQSigxHyQ9nWXiY7KHv0BxsPGhYYjDd7d8ZSdQnyll1Dz5vHKO6seCsIA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"value":"Product 1","type":"String"},"id":"7EgpK6zvQEr12INg3ujRqj_RjRnv7-tBA2t6wdtpNwkl870W6-c09XNrjGVX6QGM-Kx9GovM57Mt_-RS8iDWDA","price":{"type":"Integer","value":150}}

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
  Fields returned: ["email","name","id"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role","age","name","id","avatar_url","bio","created_at","status","email"]

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
    - Fields: ["age","status","name","password","user_role","created_at","email","bio","avatar_url","id","secret_token","api_key"]
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
    - gpt-5.4
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - dall-e-3
    - dall-e-2
    - gpt-4-1106-preview
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-4-0125-preview
    - gpt-4-turbo-preview
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
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-4-1106-preview, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-4-0125-preview, gpt-4-turbo-preview, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: 1CFXt-1YOw6--d8kcCUDXGQ8w-AN1WUr5EqcHnrFfOAd8cTPH45f_yTBOR_Dz_LAeAVqdlg20qAs0gToggtorA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 97 user functions:
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_api_user: Fetch User with Cache
  - get_active_users_ts: Get Active Users
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - cache_api_call: Cache External API Call
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - cache_api_call: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - fetch_product_enriched: Fetch Product with Enrichment
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - fetch_api_user: Fetch User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call_js: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_product_enriched: Fetch Product with Enrichment
  - fetch_slim_user: Validate and slim down user
  - fetch_product_enriched: Fetch Product with Enrichment
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - fetch_github_user: Fetch GitHub User with Cache
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_product_enriched: Fetch Product with Enrichment
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_api_user: Fetch User with Cache
  - validate_user: Check if user exists
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
✅ [32mTypeScript client examples complete![0m
