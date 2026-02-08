make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.10.0 prepare
> npm run build


> @ekodb/ekodb-client@0.10.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.10.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 819ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'YNKv8xkytnEU7deWDpKJCxBGxWhjVYOvgZLDlZIgP2kyTLZ86ql0Ii2mIIAAhkyrg3oR94JtZDUYrFXA5i4GJQ'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  id: 'YNKv8xkytnEU7deWDpKJCxBGxWhjVYOvgZLDlZIgP2kyTLZ86ql0Ii2mIIAAhkyrg3oR94JtZDUYrFXA5i4GJQ',
  value: 42,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  price: 99.99,
  created_at: '2026-02-08T19:53:29.501Z',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  metadata: { key: 'value', nested: { deep: true } },
  data: 'aGVsbG8gd29ybGQ='
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-02-08T19:53:29.501Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  id: 'YNKv8xkytnEU7deWDpKJCxBGxWhjVYOvgZLDlZIgP2kyTLZ86ql0Ii2mIIAAhkyrg3oR94JtZDUYrFXA5i4GJQ',
  value: 42,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  price: 99.99,
  created_at: '2026-02-08T19:53:29.501Z',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  metadata: { key: 'value', nested: { deep: true } },
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  id: 'YNKv8xkytnEU7deWDpKJCxBGxWhjVYOvgZLDlZIgP2kyTLZ86ql0Ii2mIIAAhkyrg3oR94JtZDUYrFXA5i4GJQ',
  created_at: { type: 'DateTime', value: '2026-02-08T19:53:29.501Z' },
  active: { type: 'Boolean', value: true },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { type: 'Integer', value: 100 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  name: { value: 'Updated Record', type: 'String' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  price: { value: 99.99, type: 'Float' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: hugcK1FYNpUEaVMVboZDPy51_uqW_n-ksd1nWur1xohJ9SDkA2XlX5Muo-nVSELtp5-yllsm3Rteh5g50AywCQ

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
Collection created with first record: ge1AVuLES4AZVRT2HqZ_U-C2xAxXQoCpWUCSOLalIBY9EHqYQk-angJcBcZzOCPcbhvTcx_-8dH24lqhgVxjmA

=== List Collections ===
Total collections: 21
Sample collections: chat_configurations__ek0_testing,schema_employees_client_go,schema_products_client_go,schema_documents_client_py,ttl_cache

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
Created Alice: $1000 - ID: Wx3tHUOQEnUKHq49UFlByGEp1-Pcut3UEmuNmBkGkjRS024E1dHtE6_tu-TD__doFqSl8W73TSz-8EY6lvVnVg
Created Bob: $500 - ID: 2_gZyXRN2krhFCvXarkWnojG7k0A0y6brsUgqb8z5rdOiA2E2ZA8lZqdtnvfS6wMiGLBwMlc-sl46rFLpTtS8Q

=== Example 1: Begin Transaction ===
Transaction ID: c9e41230-e72a-49ec-868b-3841d08a5423

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0788f3d9-2664-4a15-b17a-23ecf5520879
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
  1. Score: 26.400, Matched: title, bio.value, title.value, bio
  2. Score: 26.400, Matched: bio, bio.value, title.value, title
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: bio.value, bio, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, bio, title, title.value
  2. Score: 39.600, Matched: title, bio.value, title.value, bio
  3. Score: 39.600, Matched: bio, title, title.value, bio.value
  4. Score: 39.600, Matched: title, bio, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.771, Matched: 
  2. Score: 0.752, Matched: 
  3. Score: 0.740, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.709, Matched: title.value, title, content.value, content
  2. Score: 1.501, Matched: title, title.value, content, content.value
  3. Score: 0.296, Matched: 

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
✓ Inserted document: 5gP9B5BbHZJjz777er_fUeBk3oOb-EK8PWOvpeAr-vEOGovVd6AZOZYA59hH6fLlQ9LMF6oCdZ4oNCo6cH5Cyg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Vma2QS0ZKz6kOb1FtsSi3q-Vd_F7NyspqpbIW8nPG-S3nqJN2Xj0iYjKj3q8pQUQ5ljDmoZKwyH5OYF0aHbYeg

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
✓ Inserted document with TTL: XUAtCl5Ox_ht9O3OZwmM3c4Jac2c4N8R476agXQ2_AbUWeYk2Xiq5oLWxqtKKR_avltUDKwOB3Y2fwogQGlVZg

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
✓ Edge cache script created: Lz066zJ53tm0OTvSvWGJnxTRiokc3fYZVspLlngqVsDvdwzPiOZQ-G-iOpAPqMq8kx4GLTlSNQlVXJ2CEPO9Ag

Call 1: Cache miss (fetches from API)
Response time: 70ms
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "id": 1,
        "username": "Bret",
        "address": {
          "city": "Gwenborough",
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874"
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
Response time: 4ms (17.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "id": 1,
        "username": "Bret",
        "address": {
          "city": "Gwenborough",
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874"
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

🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: p68CL5ePxCPAwMthETNLLLRt8OJKLRC0Ek7bfatiozZKMSz3eN5R4YlzMDZOSJzom20g3p07UwLdUj7DBLCGvg
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: sMNjLQP4CDHGwKcn0dtVm-0_aYT-OYZ6pRcMLgq0wWl7DuUBQUQ2jvUA_yIZE_1CRBN3fQmcnH9bpOOJwGmq9A
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: I1RihZjAKB9xgbMKE8wKLGbvclspJdDO98FEmMZc4zqptu_EhP45kSiM-RF4wgD7YtI64Yy2C4zuLWmrZZ7SJA
📊 Statistics: 3 groups
   {"status":"active","count":5,"avg_score":60}
   {"count":5,"avg_score":50,"status":"inactive"}
   {"avg_score":60,"status":"null","count":5}

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
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "username": "Bret",
    "address": {
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "username": "Bret",
    "address": {
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        ...
   🚀 Cache speedup: 1.5x faster!

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

✅ Script saved: 7UNAXdiMWryJVCOdiAyukX4n22lY7R4NnLoOpNDBdqpa4E887zGbq4uh_WPjQXOio4hVGVqtbNJs5AWmMoDbYA
📊 Found 2 product groups
   {"count":2,"category":"Furniture","avg_price":474}
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: bGCkOWcHvogdOy4bb67lWwrwtBw-G_lbJfYUWUslSDFZhAvWH84DB8WmvaMzdrzKjV3g54b9LG5jdz_DO6TykA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: B7VXn5M6EZPmeQer8eZQVStDJPsqULnjWEPN5ikPIeO38zcYKjDGnscOwqtaArW6qd_oMycQQ7Z3qnmCX40Iqg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: NhhrTqUBfToOlS0xbiBBVbNnP0IORzrCuR8FzgBXhRg2MVhT8Am91y1InnSFduvrlyn-Bg0DUWkSdjcB1Oz1-A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: nfKIbMnqPxVCIayIZYopOP9rwIHjw2Jlab0BWnWSZo_Nm_kNhpjQ4pUBX6s0IkqxXhIfIgHkGuW7oZ3iCPD1Cw
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
✓ Created SWR script: fetch_api_user (3yVbND_OUAnIXJAlHNK1Rr3SdHtlgNkZy5H_dKXWiUeZ8lMzTmK_aJEkdpOPiR88BZR-sEqag2HN5pMf87hRuw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets"
        },
        "id": 1,
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
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
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets"
        },
        "id": 1,
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
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
✓ Created enrichment script: fetch_product_with_reviews (f1HZQKJcHsfd3cr9a91vW09t5fVdqsTcq1m4dA68VxXg8IwPTu9y1Vu0zPF-6nilM2rE0mCg__bmBYQVVqx5ow)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "minimumOrderQuantity": 48,
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "rating": 2.56,
        "availabilityStatus": "In Stock",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "rating": 4,
            "reviewerName": "Lucas Gordon"
          },
          {
            "rating": 5,
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "date": "2025-04-30T09:41:02.053Z"
          }
        ],
        "price": 9.99,
        "discountPercentage": 10.48,
        "id": 1,
        "stock": 99,
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "meta": {
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z"
        },
        "weight": 4,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "returnPolicy": "No return policy",
        "brand": "Essence",
        "dimensions": {
          "height": 13.08,
          "width": 15.14,
          "depth": 22.99
        },
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
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
✓ Created native SWR script: github_user_native (N_Amc4NLWY0QQMGivDAtlsJTaaoTvd17B5_AvxS3fDayQPkcvb_hYgv5RAcShWsiWFcWnBw69OhbxISoo-qciw)

First call (cache miss - will fetch from GitHub API):
  Response time: 6ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 2.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (36P7zSYlduTJx1GlFw-Tr5zRcz4fZ5QribVErVbYpIkmEG6cCBRssU04UIPzCXcT1GVx4cmhZjLG5rqrhOIxJA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (8IkLsH9xNPDSGP9JytIKLpO2aQ63FU9hYhzTw4WdL3RMTugea_DDUJlvvc1velm4CpsaOaLydyrIck-CmE5DQQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (qjO95Y7LJGYqQvK-8ZYl8SUOEh51CG-F-lLCu0q3Zn4-I7_PBSY1C95QovLWvxwT85VKq9mHkR0Lk204GewTMw)
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
   Vector databases offer several benefits including:

1. High Accuracy: They provide more precise and detailed data representation.
2. Scalability: They allow for easy scaling without losing data quality.
3. Flexibility: They support various types of spatial data such as points, lines, and polygons.
4. Data Integrity: They maintain data consistency and accuracy by using topological information.
5. Data Manipulation: They allow for advanced spatial operations and queries.
6. Efficient Use of Space: They only store data points that are part of the spatial feature, leading to efficient space usage.
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
   {"avg_score":20,"role":"admin","count":3}
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
   1. Natural Language Processing (AI)
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
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
✓ Created session: 711_Gv3IpwDbfh5xZsMHLSptjEFL6xManhhlyvfdqIkqDhkGWMHRLVo9ao6vy1vrExkmZz5g1gEvXwjzgxhckQ

=== Sending Chat Message ===
Message ID: j_CAHNqY5slTXv2TCxdbIu5sAX28mjnYLgZ3cDg9u-BCnIJ9RjkQJM_TQqk4lQyz1pWAQEiZPF4UHoSH1b1WPw

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.
2. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.
3. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'jnhjhIJd6wFOdULpaUru0-35XDp0x6JOrR-rP7nitVEUFwCLoJxazO61ukODxO3-QNWasxiL7OeYvV0_hV5Ylw',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499,
    id: 'MIe4ZAcybExQjkC2VgQdhiO-r1OrAm4KnL4EOlQW_4ZTRfu8kgbksXysN7WYQVrVNxv92ZzLF4MO7ii3dYRScQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: 'ThcsVilgCpSoLMcXibc2kWn3xY4G23wsRttpi5927hvK4d-P2KxK_d5k7Man0jyjRk4NhC3BELsJsWVMBC0bdA',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2454ms

=== Token Usage ===
Prompt tokens: 453
Completion tokens: 78
Total tokens: 531

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sO5xylNrD8zabJufG2kHkRhBG0xGj2p9jvOwV8mRdGEpbjdBEmDQzm-AKIpRxWuQlCR2rVo3e2cvvzVCk9JCBw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one product available is "ekoDB". It is a high-performance database product and is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of the ekoDB high-performance database product is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: TaWmlcFpgzAI2X32v1WlA_fKAn8pDB4nI6foqkoxkgZ5iylNxvrMmUED-JJ1-YD_FXA57C5Iy6b4EFiEj3T1xQ
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
✓ Created session: em9at9-kPduAicETLmP5mx5wtUeopGDOytk3VJme23uMR--8wJI4w3xiHrAMgMFvwRUVLCqk9mQ7Bi2kP4qgsA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the available information, we have a high-performance database product available named ekoDB. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: cTdAzoFmw5RlSIgrTpHqbk_MkmnYGlsngygYOuUAUQZCh6XJn3FhSXNrv4Lzd2daRK4QXWIxKl6rxRXktJOilg
  Parent: em9at9-kPduAicETLmP5mx5wtUeopGDOytk3VJme23uMR--8wJI4w3xiHrAMgMFvwRUVLCqk9mQ7Bi2kP4qgsA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: cTdAzoFmw5RlSIgrTpHqbk_MkmnYGlsngygYOuUAUQZCh6XJn3FhSXNrv4Lzd2daRK4QXWIxKl6rxRXktJOilg (Untitled)
  Session 2: em9at9-kPduAicETLmP5mx5wtUeopGDOytk3VJme23uMR--8wJI4w3xiHrAMgMFvwRUVLCqk9mQ7Bi2kP4qgsA (Untitled)
  Session 3: TaWmlcFpgzAI2X32v1WlA_fKAn8pDB4nI6foqkoxkgZ5iylNxvrMmUED-JJ1-YD_FXA57C5Iy6b4EFiEj3T1xQ (Untitled)
  Session 4: 711_Gv3IpwDbfh5xZsMHLSptjEFL6xManhhlyvfdqIkqDhkGWMHRLVo9ao6vy1vrExkmZz5g1gEvXwjzgxhckQ (Untitled)
  Session 5: 2bi4bFjVEYp3gopUAs7-9HnbZl0kK_0rq6DxH8tbBgtzS8kMZ8OAYlOV_SLjMRFySjB8mWJSUEudr0jhRnhLrQ (Untitled)
  Session 6: sOvC4faavhKMOcYDSecz0ZvSFF65vVLIJvz-JxyDDEAB5GD-TbTjh0CMly_fG3q7Q0sDEbTKsc5-pcRCTYYVSg (Untitled)
  Session 7: JrK4ENrsy2TE8XO7LDnChGcTVTN-u7HLwY8knOu90tmyPTWUh8hAFfvU2_t-r8iEqtS5ihNvkpWVwPHDiyKKFA (Untitled)
  Session 8: 0qk3_g1hxPAFkccFSrfSCLDSE7z80pic3ubP9GPWCn75i8ZNrYzr4lAaA2kaBbY1bqzXpLliomhK8bdRouRJ0g (Untitled)
  Session 9: lgdqx8KlD32BMm6A3L4du4oBOvHE9M7TIMS84d3vS5rIzsffq3obVvG6em3SYhkBFnKqcUqZ3-txSUWw_l12sw (Untitled)
  Session 10: DPptNhOxlb-eV3tn6FaBuJFaxV3rKcT8ZiF-xySQoQC9TyZmsDaVt0g_3X-wh-57yzcbng6raG_utbQQsa0yAg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: cTdAzoFmw5RlSIgrTpHqbk_MkmnYGlsngygYOuUAUQZCh6XJn3FhSXNrv4Lzd2daRK4QXWIxKl6rxRXktJOilg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'HJ6j_z-1wnvh76aXIQfogxbH1s3iwixiXthqPF5qu-Ud-aT3xQxXHWuMfqtZthNGi9TNvmCADN_fFpi4Lcgvfw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  age: { type: 'Integer', value: 29 },
  id: 'HJ6j_z-1wnvh76aXIQfogxbH1s3iwixiXthqPF5qu-Ud-aT3xQxXHWuMfqtZthNGi9TNvmCADN_fFpi4Lcgvfw',
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true }
}
✓ Second upsert (insert): {
  id: 'pQ68kZkEHK0tt9udnKCHWj4f0Z02STy4LyaSIOaXIlGVPHPJuf3oaCQAXrAKOeuqST7jFsG9jRX6phj7dVJ0wA'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  id: 'HJ6j_z-1wnvh76aXIQfogxbH1s3iwixiXthqPF5qu-Ud-aT3xQxXHWuMfqtZthNGi9TNvmCADN_fFpi4Lcgvfw',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { value: 'Alice Johnson', type: 'String' },
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
   Inserted with ripple: {"id":"9xn3PpKjl8So4SUCfRymK6vX2wrw0EPbGDinDIp5AbA-SBtlF4wHCgTx6PXxkBxPjrkPj6TboFRz7wUYJD_sug"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"K649QbjJckI9V48kvkqbkCq2KjxeeoQbVuaXK0iXFIyHxKPh2jf_G7UMkREr0Xudt6XkVO3x71gy1pRpu0WPPA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"id":"9xn3PpKjl8So4SUCfRymK6vX2wrw0EPbGDinDIp5AbA-SBtlF4wHCgTx6PXxkBxPjrkPj6TboFRz7wUYJD_sug","price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"value":"Upsert Product","type":"String"},"id":"custom-id","price":{"type":"Integer","value":500}}

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
  Fields returned: ["email","user_role","status","id","avatar_url","age","name","created_at","bio"]

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
    - Fields: ["name","email","avatar_url","age","created_at","secret_token","id","user_role","password","api_key","bio","status"]
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
    - gpt-5.2-codex
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
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
    - chatgpt-4o-latest
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
    - gpt-4o-search-preview-2025-03-11
    - gpt-4o-search-preview
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
    - codex-mini-latest
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
    - gpt-3.5-turbo-16k
    - tts-1
    - whisper-1
    - text-embedding-ada-002
  anthropic:
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
    - claude-3-7-sonnet-20250219
    - claude-3-5-haiku-20241022
    - claude-3-haiku-20240307
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-4-1106-preview, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-4-0125-preview, gpt-4-turbo-preview, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, chatgpt-4o-latest, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-search-preview-2025-03-11, gpt-4o-search-preview, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, codex-mini-latest, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-7-sonnet-20250219, claude-3-5-haiku-20241022, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: RnFdkD0Lhfxu1EoKzrELDY6KUo9M0LzqYoWz2A4dY7cjXzkQKKiOsw6aBfk62KN02WoFHVZ4mdyq-0gy76no8w

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_slim_user: Validate and slim down user
  - get_active_users_ts: Get Active Users
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_api_user: Fetch User with Cache
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - validate_user: Check if user exists

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
