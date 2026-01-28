make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.9.0 prepare
> npm run build


> @ekodb/ekodb-client@0.9.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.9.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 840ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'woVEKnrSNiHANAsmFzNKaMhxtswNWuaEBN3lk8PrByfELnMOWSplLgGUZYId3KJdkLQElipOGrN5aPg_jZ9vfA'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-01-27T17:14:22.409Z',
  metadata: { nested: { deep: true }, key: 'value' },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  active: true,
  price: 99.99,
  id: 'woVEKnrSNiHANAsmFzNKaMhxtswNWuaEBN3lk8PrByfELnMOWSplLgGUZYId3KJdkLQElipOGrN5aPg_jZ9vfA'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-27T17:14:22.409Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-01-27T17:14:22.409Z',
  metadata: { nested: { deep: true }, key: 'value' },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  active: true,
  price: 99.99,
  id: 'woVEKnrSNiHANAsmFzNKaMhxtswNWuaEBN3lk8PrByfELnMOWSplLgGUZYId3KJdkLQElipOGrN5aPg_jZ9vfA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { type: 'Boolean', value: true },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  id: 'woVEKnrSNiHANAsmFzNKaMhxtswNWuaEBN3lk8PrByfELnMOWSplLgGUZYId3KJdkLQElipOGrN5aPg_jZ9vfA',
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  created_at: { value: '2026-01-27T17:14:22.409Z', type: 'DateTime' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: rUYY3YbJh5DHYmGkAObAwF8EKY0f62NaTIEd7kiZ9T1b5sVarbH39_19qaerOeVpSU3lzO9Fwb7z0xAzHfKkKg

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
Collection created with first record: iLEsdH8y_67WF-uJ278dFKk7gCJxwSNatm-5KAncVMYHEtKVJ7PAu2oofUWsGnpbxTx7vV75sr1yDXKWdCdzMg

=== List Collections ===
Total collections: 20
Sample collections: chat_configurations__ek0_testing,test_accounts,schema_users_client_go,batch_users,products

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
  cache:product:1: { value: { name: 'Product 1', price: 29.99 } }
  cache:product:2: { value: { price: 39.99, name: 'Product 2' } }
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
Created Alice: $1000 - ID: M2tQ-SnLR2aBJDlu7PsF8f23SN4M8QWeW2PUueCn8geMqTCoVStECh01WqTyEJ8OWr7idU90k5S8YTgeEccnmQ
Created Bob: $500 - ID: YUFTilwUPCs2e2MHlQ0DGbGqZaHo-cs6vwYC1gtcndTyL9wmlnNFskszqa-5s-PQ5qBs5aURImYI-fjwhX9aww

=== Example 1: Begin Transaction ===
Transaction ID: ff79bb91-29e9-4f07-9c0f-7950e4285aa0

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 17455b77-6398-4fdb-90c7-bbb565b94ec2
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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title.value, bio.value, title
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: title, bio.value, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title.value, title
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.779, Matched: 
  2. Score: 0.777, Matched: 
  3. Score: 0.755, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: content.value, title.value, title, content
  2. Score: 1.511, Matched: title, content, title.value, content.value
  3. Score: 0.312, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio, bio.value, skills.value

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
✓ Inserted document: Op4AJ67IbhGH2dhoxdlRXrJ-RXeeRzsKkKsstWjNZZ74krNX2rXcJcql_r9Yq0IjOvi-ZIeSu9d8bobe5WVxfQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: eAGLNw826oyIDAKo276FxZFfv77DQsvbKxfITyuWxg0Je6CZRSidkT4kW1pA1M_ixGM9NpvcZPdwrSgflJEfcQ

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
✓ Inserted document with TTL: IckGKX1xz601nYteepG2t24Z7DdLRSNHRl62QAodfvqevvLTRh0cpUmAIbQFI0GjG4k4fjgbY8YD62C-EWxXMA

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
✓ Edge cache script created: Di0Y5_V3XctSrP-9XpVGhxVuGDLmtTikD8-d4xwpZ6jvU3eqRz-SdS6TU9eG2NE7NXSFi8BA2D5dbo5kmBBI0A

Call 1: Cache miss (fetches from API)
Response time: 74ms
Result: {
  "records": [
    {
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
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "username": "Bret",
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442"
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
Response time: 4ms (18.5x faster!)
Result: {
  "records": [
    {
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
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "username": "Bret",
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442"
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

✅ Script saved: EfVdItjG2Ofr76la6v9yFTklOzRWLSY_80IUUeIPosX2ES81BBukO3qJgnBCfFBD9DC41mEiyIzPQ_oCv1i0PQ
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: Qlz458qh93x0_u2MbKXrJLwPJz-p2szwjmUg5-_8l5pOyVbGyTSxSn8zfquFDIe0R7khJ6FStVzTlHmjhotfqw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: UN-TJZV_Tu379aiU03hIkEhP6ML0zWi0KL8tf2E_Fdtn31uSv92vZ3OCsjovpL4JWj2gKdqXE4HRLxwLnG3qZQ
📊 Statistics: 3 groups
   {"status":"active","count":5,"avg_score":60}
   {"status":"null","avg_score":60,"count":5}
   {"avg_score":50,"count":5,"status":"inactive"}

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
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

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
    "phone": "1-770-736-8031 x56442",
    "username": "Bret",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "company": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "username": "Bret",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "company": ...
   🚀 Cache speedup: 0.8x faster!

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

✅ Script saved: 7y8fDvCKMhIRuQFyKnbDPYgaM7-5y7NzhFtJA4672KjF1PxSOobrg0n0HeTzcqDPIpCjh5C5wLvMgzHSkXk0PA
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: kgUijrXH7884MwD4L50hqFieayppt9bGUYXOqIVcvM7Jdmsbuh9keDN6958W2fkXET-M-edn4elD_9tTXp9veQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: auiMbxQ1650iJYToZoebmjEj7JPCdf3o8KPimNdKSi1Gps7kK8YW98XVtO2nObQNqJgWXuuW24LXdS0x3j0EbQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: CXE9yWkhoWA4-n5bCtvOa4BEH01PSEwAStApIK_M1YurxRUOsvZLcwKCNzhBFh_CFDG_u47JIx3zFAzvLBugOQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: UOHnRF3aFpMAwhWe-NCKPJDX9FWMKPvy_1ZsWoWVsJo3LsZ9HrHTR5qRN9NoAd5hitLKZqahs66akAPu_V8Awg
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
✓ Created SWR script: fetch_api_user (Ox_anr0xpLBhMxcVZ35nkXs6zLJ3jXeEoiu_s6xUatactjZPx7aOZxNCC_84wTfYqKyOfyfJrCPQ1CQ7UjSoHA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
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
        "email": "Sincere@april.biz",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
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
Response time: 4ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
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
        "email": "Sincere@april.biz",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
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
✓ Created enrichment script: fetch_product_with_reviews (nI8HjRiasJ-UeCXslL_a1Fw2CdF41ZmIzLufMy_RDW8N4esPsnezqSHC8L1Z67f27TzFa0Izo64889BLEIaKNg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "brand": "Essence",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "warrantyInformation": "1 week warranty",
        "rating": 2.56,
        "minimumOrderQuantity": 48,
        "availabilityStatus": "In Stock",
        "shippingInformation": "Ships in 3-5 business days",
        "dimensions": {
          "width": 15.14,
          "depth": 22.99,
          "height": 13.08
        },
        "discountPercentage": 10.48,
        "returnPolicy": "No return policy",
        "price": 9.99,
        "meta": {
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "weight": 4,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "category": "beauty",
        "id": 1,
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "reviews": [
          {
            "rating": 3,
            "reviewerName": "Eleanor Collins",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "comment": "Very satisfied!"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 5,
            "comment": "Highly impressed!",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "title": "Essence Mascara Lash Princess"
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
   {"category":"Electronics","count":5,"avg_price":367}
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
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
   Vector databases offer several benefits such as:

1. High Precision: They provide accurate representations of data points, lines, and polygons.
2. Scalability: They allow data to be scaled up or down without loss of quality.
3. Advanced Analysis: They enable complex spatial and analytical operations.
4. Flexibility: Data can be easily manipulated and visualized in different ways.
5. Storage Efficiency: They store data in a more compact form compared to raster databases.
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
   {"avg_score":70,"count":7,"role":"user"}
   {"avg_score":20,"count":3,"role":"admin"}
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
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
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
✓ Created session: Rfcw3lOe2ZuFSzTrMCeubJP7IxjrSY6z_Dw9BiTfwgmTiOsSQNFOgu35bVOjFkpnyOWlF1TL_fX6LZLCFNyVbw

=== Sending Chat Message ===
Message ID: uZOsfry3k3K7SK10IkOFMpac2R5-bJcyCkpvO46VPX9cEnD4Aww5vrtTxdccwqk9zabsZ1H-hlbXoAcgen0KTQ

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is our Enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'zHWmzNdufOyDh-wO1qSvJZDeako5Rbi3DgcC3Z6D4R5Oy021STzrherDeUddVltdhIrL_jn2PBWFPjX5ogx2Tg',
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'COttlqVaOqQmC_PpuEK6DbN3wnfvLwyO8dCJImJGyKRaRJaHoASJUm0liVWVOi8ruH6SkOXmJfXjgU07ZN2U-A',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    id: 'd5NSi41_G6ICMHIXiMxk1M9TMoMKO0mcnyojs-QWQHB7ak5sGIWrhCa393Vt48sZlcw0L6TZSDwc3qM-kfVPqA',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2756ms

=== Token Usage ===
Prompt tokens: 453
Completion tokens: 84
Total tokens: 537

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: L8bRvTi_cddoqGud8HQstlneZuB84tn-UfOzL8Gq63cgvSsxGDp_LiCPtx3B_8DDTAONG5Qk8SOb5xw4MPLcYg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. This is a high-performance database product priced at $99.

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
✓ Created second session: viUGfP54D7GHkKPi8-y4oIu0sEsSzcUcy7IW8t0kM-CkKDtnbvxdblIVCqmWuoPT_hcFzoh6yh4VMXX97NNiTg
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
✓ Created session: 1LfAHPb4XlXtdOzun-Pt8V8ShLVy3wFUaQFnCwK6m_wc_1ggMQ-sf8rmo3QKUKyvLkjp98CZH7KUdjax_4Pesg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product available is "ekoDB". It is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 4JvCHg8ADJ-gSD3oSHH3n4-9LaO-2n6PS5rXcdnmCUInC381pVjsQqfhkTyijg6Y81Lh-Wm1O_UJb9SW_Ju0vA
  Parent: 1LfAHPb4XlXtdOzun-Pt8V8ShLVy3wFUaQFnCwK6m_wc_1ggMQ-sf8rmo3QKUKyvLkjp98CZH7KUdjax_4Pesg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 4JvCHg8ADJ-gSD3oSHH3n4-9LaO-2n6PS5rXcdnmCUInC381pVjsQqfhkTyijg6Y81Lh-Wm1O_UJb9SW_Ju0vA (Untitled)
  Session 2: 1LfAHPb4XlXtdOzun-Pt8V8ShLVy3wFUaQFnCwK6m_wc_1ggMQ-sf8rmo3QKUKyvLkjp98CZH7KUdjax_4Pesg (Untitled)
  Session 3: viUGfP54D7GHkKPi8-y4oIu0sEsSzcUcy7IW8t0kM-CkKDtnbvxdblIVCqmWuoPT_hcFzoh6yh4VMXX97NNiTg (Untitled)
  Session 4: Rfcw3lOe2ZuFSzTrMCeubJP7IxjrSY6z_Dw9BiTfwgmTiOsSQNFOgu35bVOjFkpnyOWlF1TL_fX6LZLCFNyVbw (Untitled)
  Session 5: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ (Untitled)
  Session 6: vCj_jR9APFVECAEupGE-eShauY1CU4zi6rs881YufAEuruBTM5UiMLTrGofiOuPSc5t7QN0jtSJMOoTnG5FXbw (Untitled)
  Session 7: H5TOVzzNxakRARI8q0N8aI6MXf9khSLO60NUFeIYlRi4rNsHQf9Ft3xqdUkG0HrNylE4YlWGD3w9dHAkqNo0Ig (Untitled)
  Session 8: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A (Untitled)
  Session 9: sAzMtmJ-PQw2AMEuPiO1cVavd-TPGz-NdoWzNaWrQwQC5YnvBxHARHZQzeL1qVMGkgyKyhSGl-4SFyQ4Yp8K6w (Untitled)
  Session 10: wI1DCXeEsJdeUO_kt02FRPdNDwDBsstg9B7FWw2CKRTfOPdsGlg_28kcDF5_qt7kKB0d1QzG3ChFwb0qYkflbw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 4JvCHg8ADJ-gSD3oSHH3n4-9LaO-2n6PS5rXcdnmCUInC381pVjsQqfhkTyijg6Y81Lh-Wm1O_UJb9SW_Ju0vA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'cmhbfTMrlm1AbSfzQEl8EA4TSFyMTLXnG29r8jc57rLktAJYpvNqJngeAL8d6uauF1CHEedrasc3KL5o3Uq65w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'cmhbfTMrlm1AbSfzQEl8EA4TSFyMTLXnG29r8jc57rLktAJYpvNqJngeAL8d6uauF1CHEedrasc3KL5o3Uq65w',
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: 'Zg3ICN59ev-FFZCBek9vZQBilugIJ7_ZTZTwXD3ULEwWrAhVATAl79mYcaha_F4F_VmxcV6qtdEJ21wVYRTsOA'
}

=== Find One Operation ===
✓ Found user by email: {
  id: 'cmhbfTMrlm1AbSfzQEl8EA4TSFyMTLXnG29r8jc57rLktAJYpvNqJngeAL8d6uauF1CHEedrasc3KL5o3Uq65w',
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' }
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
   Inserted with ripple: {"id":"mNNFGMf25izaeK_feKGNBeiKtoLjBgK1BiMcP9wnjRmWB_b4OJ0TdblMqdIulwM0I1HULQOOpr9bd0jylbQirg"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"f0KqCntAZVEppT4cjtFjZlA0pEfRkTyt_srk3AJfLv4jjHAG2R7R7gCeRoSMOyRE7gAdoAudLghpLYXzxdmdnA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"mNNFGMf25izaeK_feKGNBeiKtoLjBgK1BiMcP9wnjRmWB_b4OJ0TdblMqdIulwM0I1HULQOOpr9bd0jylbQirg","name":{"value":"Product 1","type":"String"},"price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"bm6fNq4Y4f7cdK2E212k-yXRT2Im0UIoerAPviQJKZtR_JpJFHy0qxFtNbzrhvCnvQJwxqze6trPNf2nOY1fvw"}

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
