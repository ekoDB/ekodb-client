make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.16.0 prepare
> npm run build


> @ekodb/ekodb-client@0.16.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.16.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 974ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'hcXPpSrz8zAYC0TACmcnuJsmtmOzBpQgq-qCKkewP1SpYUljKPa_8JrpQAj33VBrAtOO1UACSp3fj5sxk6zI5A'
}

=== Find by ID ===
Found: {
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { type: 'String', value: 'Test Record' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  active: { value: true, type: 'Boolean' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  price: { type: 'Float', value: 99.99 },
  value: { type: 'Integer', value: 42 },
  id: 'hcXPpSrz8zAYC0TACmcnuJsmtmOzBpQgq-qCKkewP1SpYUljKPa_8JrpQAj33VBrAtOO1UACSp3fj5sxk6zI5A',
  created_at: { type: 'DateTime', value: '2026-04-12T16:44:40.587Z' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-12T16:44:40.587Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  data: 'aGVsbG8gd29ybGQ=',
  categories: [ 'electronics', 'computers' ],
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  price: 99.99,
  value: 42,
  id: 'hcXPpSrz8zAYC0TACmcnuJsmtmOzBpQgq-qCKkewP1SpYUljKPa_8JrpQAj33VBrAtOO1UACSp3fj5sxk6zI5A',
  created_at: '2026-04-12T16:44:40.587Z',
  metadata: { nested: { deep: true }, key: 'value' },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  id: 'hcXPpSrz8zAYC0TACmcnuJsmtmOzBpQgq-qCKkewP1SpYUljKPa_8JrpQAj33VBrAtOO1UACSp3fj5sxk6zI5A',
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-04-12T16:44:40.587Z' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: yJyFQ7qbk9hTGNG-w94oys-XK1qzCv4-EZpRCZrDELnCL-CHzWl0mAFgJW2lMRAoqvft2EFSRsPMzE07SL3pyQ

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
Collection created with first record: o05m13fwXNTvXX33bSFLJY0bAhCW-9JHkyxq7qQEjhSlbU9PdJgEi3QG05MjApU39w5a9NQmKfrApWE5YPeWJQ

=== List Collections ===
Total collections: 22
Sample collections: schema_users_client_py,test_collection,users,client_collection_management_ts,schema_users_client_go

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
Created Alice: $1000 - ID: HRAgqO6tuWSsiUIKkgYLs1VuhqejXN9fQYqtCcEb2jng2kUzehCJusvAjaNqQzKZ6ReafbhXvdyDPRa2O_iiRw
Created Bob: $500 - ID: MQEwBig-qkgTPj9NYANlkY8jP42yMPoFc5lgYxOQ5wEy4myQiTH-NWch8S1lOk4cva3FapI_CeZUxsDlPPEKjQ

=== Example 1: Begin Transaction ===
Transaction ID: 120f201e-92cb-4283-b60b-bde442b54081

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b420b001-179c-4a91-8a5b-ab50cca0bfe0
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
  1. Score: 25.740, Matched: name.value, email, email.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio.value, bio
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: title.value, bio.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title.value, title
  3. Score: 39.600, Matched: bio.value, bio, title, title.value
  4. Score: 39.600, Matched: title.value, bio, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.740, Matched: 
  3. Score: 0.733, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.696, Matched: title.value, content.value, title, content
  2. Score: 1.493, Matched: content.value, title, title.value, content
  3. Score: 0.308, Matched: 

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
✓ Inserted document: f0wGGRsrbKwpdrfNnlnfEIrbqGWyadlMMH0nXTiep9iG_j5k9nOUgw35y6028X4c9u2ZZApepv-0torADPf9ig

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: CWvhsfA-YcflG8O7Y3osz_nqihPH8ULarwTtPHXJh_sTOtet1GiPDNVE3gT3UMh0Q-IrA-dPF_IwSCMSqM_j3A

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
✓ Inserted document with TTL: RLc1sDOWevb0CKTprg8mItvMA1OYrpUHM-vN6rnX1NinlnVMrSPL0S4JtorLnafpQqmoQEEn3QZkF6v_k7YZ5A

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
✓ Edge cache script created: 9puGBOOBFhJk9D9xxSr0LL76ongM7dVWnZrr0wxid912iUDxGQRV_5dvXW8VcrfWv2-AIUFslLOo2uBrgzbW0Q

Call 1: Cache miss (fetches from API)
Response time: 144ms
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "name": "Leanne Graham",
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
Response time: 41ms (3.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "name": "Leanne Graham",
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

🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 9J5cZixeHep7dkFdou0xXsoR1xgicqbD3ukrEaiGhdjqVAp7CSa6LdZt3go89sflAtePS0Qnmx4Dn47pQxMrhQ
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: 7fBZUyOjI3Grex4rst1PtJTaicqCTS6Wr6cGL_vBUXixTW2wqPdr4VYOKXg5DpBFfFIyypGyyT4v00bimEj36g
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: dbeRJxIZyznFQEiezXQ-ygRy0NsA_dRthdRB6J9tNIE3FZnLuU2Aydyeao7MLSHXd1pzDNCZO8fNS3u_G83Ifg
📊 Statistics: 3 groups
   {"count":5,"avg_score":60,"status":"active"}
   {"status":"null","avg_score":60,"count":5}
   {"status":"inactive","count":5,"avg_score":50}

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
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "email": "Sincere@a...

Second call (cache hit - from cache):
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "email": "Sincere@a...
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

✅ Function saved: LykUNrgfwFKss-TQ1Nz2K7-xShYSwjlu951HckfLYUHJCjdvaOCjzcEHFP5ZAmSILYOig83BT7cipC_pzfGDYw
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"category":"Electronics","count":3}
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

✅ Inserted order: hZXYlkgZmeluv0_TqKJEbwcoZ8ty94f_8vyv1nqYkHjw3oOz9vWI1s9DtFX967PXhSlwuTJzZ4guXdyvRl06bg
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: 6sR6AkgkRRmayrYMH0jT7QKsnf-GuUuMpE75EvLMdTsYzEsE87_iqRTDKRsTnVrOFMMVjkRUY-juVXT9s7g68w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: CIEi4OytfrDKRsimoyyEpCsLouEOPaNUVqFvIQAxxArdQv_NFQX8WvvPrCsO12EPmLXr7kcPdkna2vaV5WAI-g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: hZQqBtmdYAf0wZX5AAHmZw6KheC2kKf5z2j5YJdAyxynoiU1Zsxg8hvSpdyAdtCfDbB4xHgX7kRyAjaGFOMYpQ
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
✓ Created SWR script: fetch_api_user (MqI874Dyx-d2rGRcMojFERR4k83OFH90HWPVO1ObyFmlGSljAS3kSuH2onei99bt1QkSwmGX8wQokFoMZMZ4gg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "username": "Bret",
        "website": "hildegard.org"
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
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "username": "Bret",
        "website": "hildegard.org"
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
✓ Created enrichment script: fetch_product_with_reviews (4LvuR4H773aHt_R31BUbhliN41SUvzKnRFRjG1_K6Jv4fnSqyWzOTEJmuIBbrWxxiNnXT39p5F2Us26oayTa_Q)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "discountPercentage": 10.48,
        "returnPolicy": "No return policy",
        "price": 9.99,
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "comment": "Would not recommend!",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon",
            "rating": 4
          },
          {
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "rating": 5,
            "date": "2025-04-30T09:41:02.053Z"
          }
        ],
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "brand": "Essence",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "createdAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "stock": 99,
        "availabilityStatus": "In Stock",
        "rating": 2.56,
        "weight": 4,
        "minimumOrderQuantity": 48,
        "title": "Essence Mascara Lash Princess",
        "tags": [
          "beauty",
          "mascara"
        ],
        "id": 1,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "warrantyInformation": "1 week warranty",
        "sku": "BEA-ESS-ESS-001",
        "shippingInformation": "Ships in 3-5 business days",
        "category": "beauty"
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
✓ Created native SWR script: github_user_native (AlxOpgbnopFdHtFUZONi8uxwCglcfk49VSN6LxTvjDceTFWVVK2pjgoDoqRuhrUP2Q1DEq3LzOFmR5Q0h4fMSw)

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
✓ Created SWR script with audit trail: product_swr_audit (j_J0e49UYABM1xme2CB5AuSGRQrUg-9gLSftPQ7BjY7ZgC9YdUcxFIF9ITuK1EDYOdN7PhzkBjalx1qJKiLaFg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (R2mz7UFJQVGIZY8wG6WCbbD5QZ3k8FEV-k01_2J5p5Pt3afYwzrlwq1VpzNd9Lj9TLUq2DNH4LMaVk7wbChjOw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (YTbRragPSDBAkE2xFE828wcrasZQbBp6TYTmRmpQqxnI9FcDQP99vXtYAYY0qsXpniaqZusDFa4XASPncfsN5w)
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
   {"avg_price":365.6666666666667,"count":3,"category":"Furniture"}
   {"avg_price":367,"category":"Electronics","count":5}
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

1. **Efficient Similarity Search**: They excel in searching high-dimensional data for similar items, useful for applications like recommendation systems and image retrieval.

2. **Handling Unstructured Data**: Vector databases effectively manage unstructured data such as text, images, and videos by converting them into vector representations.

3. **Scalability**: Many vector databases are designed to scale horizontally, accommodating large datasets and high query loads.

4. **Performance Optimization**: They use specialized indexing techniques (like HNSW or Annoy) for faster retrieval times, even in high-dimensional spaces.

5. **Integration with Machine Learning**: Vector databases support seamless integration with ML models, enabling real-time analytics and inference.

6. **Flexibility**: They can store and query multiple data types, making them suitable for various applications across industries.

7. **Advanced Query Capabilities**: Besides nearest neighbor searches, they allow complex queries involving multiple vector operations. 

These benefits make vector databases particularly well-suited for use cases in AI and data-driven applications.
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
   {"role":"admin","count":3,"avg_score":20}
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
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"category":"AI","count":2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Wq0FBjdjyZFL5dvEiXXh9oSzxml2MaulqyBAArknQ-YjhUBicp2wm_7t2j_4Jo6SfUpQdqr9EH0zvA7Vi4vvWA

=== Sending Chat Message ===
Message ID: IpwOQ-f3VKX_tVBRDq9R2yW6zXLkLdaWt8fJonDeWPHxfrDaf_KSkAIKltWh76rHqHziGogaLB6FAlC8c6-V1A

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'PuRK4oJGdIo7Odq-mSRoFwMeZ-eE5XClfylVPOV3wM74zlh2JBphzc5eU_Z3NE2Z1WwTQlF33eJuE3Uum2ZIiw',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'NADVBOkx3SUKMM3sizICfZRntt2bA2Wt1l5ilHNHyh-uDNDh6dpez8rynaTGh4L_DGzUS5Lp_x7UknJladcOZw',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    price: 99,
    id: 'S49oIm8diksFMoXfLwsLVJG2RoqIxcYtBma_fC3eVV_8MhwOyassKz6mBVnsi0IC2JkBebqcYc_4UMfdOOutKQ',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 8618ms

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
✓ Created session: 4H6V51Ufkzt0771BZSMO_VMEETFCLHsOAk4D55ccRESOra05RGofboOpw-0xMKL0ZEz_q3kN-VQcY3Y3Hxq_hg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you'd like to know more or have any other questions, feel free to ask!

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
✓ Created second session: sB-siCOBQMoeDbbZGWMpxBRbjV56-5IaFPUO_KWVohEzM2ZhO_fe7kEzGbHZSSkKCob-yIJYdQBhnRMJkRT54w
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
✓ Created session: oXyNlcmlgi1EJFwRbqJFi4_amcde50qpGVi13cHaHKVRLpr3h-YuqugcpR8JUdfSnb_FhzTvnWquljdcqIlFRA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB  
  **Description:** A high-performance database product  
  **Price:** $99  

If you have any more questions or need additional information, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: _C8xuhNOfyRDQ7Blx0FCOMUz_PMSO2wpLrCfgWtyu-jZNPo3Ne0g4FqO97tGQPCtvrkYpHTBRoabH8iEZMJNTw
  Parent: oXyNlcmlgi1EJFwRbqJFi4_amcde50qpGVi13cHaHKVRLpr3h-YuqugcpR8JUdfSnb_FhzTvnWquljdcqIlFRA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: _C8xuhNOfyRDQ7Blx0FCOMUz_PMSO2wpLrCfgWtyu-jZNPo3Ne0g4FqO97tGQPCtvrkYpHTBRoabH8iEZMJNTw (Untitled)
  Session 2: oXyNlcmlgi1EJFwRbqJFi4_amcde50qpGVi13cHaHKVRLpr3h-YuqugcpR8JUdfSnb_FhzTvnWquljdcqIlFRA (Untitled)
  Session 3: sB-siCOBQMoeDbbZGWMpxBRbjV56-5IaFPUO_KWVohEzM2ZhO_fe7kEzGbHZSSkKCob-yIJYdQBhnRMJkRT54w (Untitled)
  Session 4: Wq0FBjdjyZFL5dvEiXXh9oSzxml2MaulqyBAArknQ-YjhUBicp2wm_7t2j_4Jo6SfUpQdqr9EH0zvA7Vi4vvWA (Untitled)
  Session 5: Znp6mUA8YF5IZMlJ_Gi5RO5IK4Csu0uLWqmdO2yqmd_ANoxYa250sKiTx-bmvvu2SIgeuzVldlKiXpUYweOW_g (Untitled)
  Session 6: Be84Ao5KLBzjK4zjM0uyi7WHiFGkgnGiBz913zjhkQrGrZn6A3j7uDi1wNEsih7W6bTQ2aTuD5OcL0gfaQQrvQ (Untitled)
  Session 7: 32PFek8QtgY3SxhW6Q7KVWSbd7uQimZbD3OOUBIeM8mjW6NtmYSx4TqnDEaMk5EP30UDXloORbnCr2umJk9wrw (Untitled)
  Session 8: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg (Untitled)
  Session 9: 5qaltAlAg_c0xW2xSTM4Las-hGRQUIRoLBisgrFpRUkeQd8WdOmlaAueAj-04BaV2RHQ5oroV5f16673NJTdjQ (Untitled)
  Session 10: hqgZ7u1n4z1PjYSwN3mGKkYDjbJgw-JhqI7U6wQhwAaSmQMFswL4loRxhLaW_4sKaj0SpDoJPx-rNtOdQ8yMWw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: _C8xuhNOfyRDQ7Blx0FCOMUz_PMSO2wpLrCfgWtyu-jZNPo3Ne0g4FqO97tGQPCtvrkYpHTBRoabH8iEZMJNTw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'wJmcMI1raCyUNzf1MaYjswusSEAD0wuT1yaft2a9e1evvI4BrESvMXU19EWQIwjPvwuZo1728WkjspV35K0C5w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'wJmcMI1raCyUNzf1MaYjswusSEAD0wuT1yaft2a9e1evvI4BrESvMXU19EWQIwjPvwuZo1728WkjspV35K0C5w',
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: 'jQRp8Xy6FFc0gT5QCCYFKfUi3XSJtR0AJgnhcGCAJi64AUpq4CC9edM-1TEylXpVDxYsQ7IO6YTXlRE-9uOWag'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'wJmcMI1raCyUNzf1MaYjswusSEAD0wuT1yaft2a9e1evvI4BrESvMXU19EWQIwjPvwuZo1728WkjspV35K0C5w',
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
   Inserted with ripple: {"id":"Oo0_UgKl-rlf4kpD233u-oV5d9nrnytOyzPSCU471mkPz0QfNOmYYAKNUrMVxOE1DkRl1mkfAStcRNgXtAHU2Q"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"--EKiUw91yvsN-rqlMEKgQqZ4VeVQj7r9Hvb5yZPN6QUdtgvEouAQspU3bgoajL8b3fnKGuq5nBqnoYmwaieyg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150},"id":"Oo0_UgKl-rlf4kpD233u-oV5d9nrnytOyzPSCU471mkPz0QfNOmYYAKNUrMVxOE1DkRl1mkfAStcRNgXtAHU2Q"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"type":"String","value":"Upsert Product"},"id":"custom-id","price":{"type":"Integer","value":500}}

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
  Fields returned: ["name","bio","id","user_role","created_at","status","age","email","avatar_url"]

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
    - Fields: ["email","api_key","secret_token","id","age","status","password","created_at","user_role","bio","name","avatar_url"]
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
Created user function with ID: kHNsjlCO9VmzZupqjIiRV3OvOYXFOyNpY8BE74q2kRKpibCUvXlD5VUkAqYMcev6Um7BBPruX1g2KUsKKHwWRg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_ts: Get Active Users
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - cache_api_call_py: Cache External API Call
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_api_user: Fetch User with Cache
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
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
✅ [32mTypeScript client examples complete![0m
