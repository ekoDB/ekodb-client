make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.13.0 prepare
> npm run build


> @ekodb/ekodb-client@0.13.0 build
> tsc


up to date, audited 53 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.13.0 build
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
  id: 'Cd8LG6vAMR3Za9nL7hJ6FOPdgYz3k88pggYeViEXDJjTfWjeLy2h6_25sGelBTzT4xlC6NfKYOVWwC8qb6HGmQ'
}

=== Find by ID ===
Found: {
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  id: 'Cd8LG6vAMR3Za9nL7hJ6FOPdgYz3k88pggYeViEXDJjTfWjeLy2h6_25sGelBTzT4xlC6NfKYOVWwC8qb6HGmQ',
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  name: { value: 'Test Record', type: 'String' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 },
  price: { type: 'Float', value: 99.99 },
  created_at: { value: '2026-03-22T16:47:29.190Z', type: 'DateTime' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-22T16:47:29.190Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  id: 'Cd8LG6vAMR3Za9nL7hJ6FOPdgYz3k88pggYeViEXDJjTfWjeLy2h6_25sGelBTzT4xlC6NfKYOVWwC8qb6HGmQ',
  categories: [ 'electronics', 'computers' ],
  metadata: { key: 'value', nested: { deep: true } },
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  active: true,
  value: 42,
  price: 99.99,
  created_at: '2026-03-22T16:47:29.190Z'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  id: 'Cd8LG6vAMR3Za9nL7hJ6FOPdgYz3k88pggYeViEXDJjTfWjeLy2h6_25sGelBTzT4xlC6NfKYOVWwC8qb6HGmQ',
  active: { value: true, type: 'Boolean' },
  created_at: { value: '2026-03-22T16:47:29.190Z', type: 'DateTime' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  value: { value: 100, type: 'Integer' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: ijqPY22hfzmP68Ji-i50-qWUh6d-7I6OZJwH8okIc30XGCvz2CGV1JktMLPutkdZhRBUx_6nHj6gtqYnVLQCIg

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
Collection created with first record: 8voLHWA99u49dzrjUQp4nXjpAU3TnJHsazN2JLmSDdRszL2vlLiPRpEJrFWfLQx-pZx1n7vZnzGU_oaFcLxwZA

=== List Collections ===
Total collections: 22
Sample collections: batch_users,schema_documents_client_py,users,schema_employees_client_py,schema_employees_client_go

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
Created Alice: $1000 - ID: nDs6tW9nGva4buP8LzFFTuYUQOph4yqyaGvWmxZtLIvs2cqq8g0C6qHCurmijOD8RbgzubpEcHLMrTCzWn_S-w
Created Bob: $500 - ID: n7_TuNZLmmoVtg01JwWJJcvFDHTkfdHCAdTDyuOx0rS4Fr8NfkjbDu66q2ompLY178cdwee6IQlszW-W9Q9eFA

=== Example 1: Begin Transaction ===
Transaction ID: 9b4d6d40-3f7a-4fba-89c1-66aba9ae5f36

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 93511b94-9b28-4fa5-bb25-56e5db51e079
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
  1. Score: 25.740, Matched: email, email.value, name, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, title, bio
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: bio, title.value, bio.value, title
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: bio, bio.value, title.value, title
  3. Score: 39.600, Matched: bio, title.value, bio.value, title
  4. Score: 39.600, Matched: title, bio.value, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.781, Matched: 
  2. Score: 0.768, Matched: 
  3. Score: 0.765, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707, Matched: title.value, content.value, title, content
  2. Score: 1.513, Matched: content, title, title.value, content.value
  3. Score: 0.306, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, bio.value, skills, skills.value

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
✓ Inserted document: FyUJNy7rgj0GizsSJz0pjCG-0DUcTyx8sjXrlklg9D3qyuUeU3FLECZs1u38PeFC8e-PSABvie4ZVvQAFVkSqw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: KZE4wuDMsWZO0tPRYSmVC1UCxeMEgj0hvy27a9QhjGk0vKo8FrDssGxgdrWPpbDcGndHZ0VMJX207zzv5MxgFQ

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
✓ Inserted document with TTL: Vg_cUTu5kgnLDCsRCzzzbEVSwcMa5eljtIsPuZmBK0l1Uo5B2UHJSABMxH0X68OweTjxiHp3QhNphb4zkk4E9g

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
✓ Edge cache script created: y6reeg3PYvBVVIS4eG6nJZ9NnEicoxBwEfZNdxhEGej4XVcDKKc_RkPx6Z9UsZIxy8Ox-Br14AVeiyag49ioNg

Call 1: Cache miss (fetches from API)
Response time: 70ms
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "street": "Kulas Light"
        },
        "id": 1,
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
Response time: 3ms (23.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "street": "Kulas Light"
        },
        "id": 1,
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

✅ Script saved: W9n4SKFv6JwKkGNd0oh9onbjwI-XrS_SrKatsoU5RGsB1cTrAjes1Wssc13NydXAXUr3Ij5VJqnVfqQLYdiAIA
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: Toynlwun17m3eiZfnFgULRrWpBhn9TneQQaNt1xuVpdorPiNSsTF3iHPYMSinnG4dbtkRnpfQ09sAwo0hxPYkw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: xivPr-kyTf0BMpaabzeUFd8dSanzpCPNXU3WdqaMfH2eO-2aQKxJ_AVcdSRf9K9dk-bGzfkO2kd1MSCCN7uXgg
📊 Statistics: 3 groups
   {"avg_score":60,"status":"active","count":5}
   {"avg_score":60,"status":"null","count":5}
   {"count":5,"status":"inactive","avg_score":50}

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
    "name": "Leanne Graham",
    "username": "Bret",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "geo": {
  ...

Second call (cache hit - from cache):
   ⏱️  Duration: 6ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "username": "Bret",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "geo": {
  ...
   🚀 Cache speedup: 0.5x faster!

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

✅ Script saved: rGQZHcEzyPXVcWnq8Y86Jgd7BKaxuzjgVzpEkdu2OnD245-rFfIN5x-5VzCZJu_Wl4k6w0-g091zztfvWvvf3Q
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
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

✅ Inserted order: fDcWNv7pm4-l_Li1lRU5BIdOZEooLLjZHILjShKZjEsK6Ks6oXGE3BHN8tnQd19NyNiMtDQcoESEjpSFxpVvGg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: u3-t3jj9X17Cmk2eDFiLioD1BLcvJbRxJIt5L--_5bwwThEuxrWa3pXvoZMJQxzsl6unGy9tuVpJ6-8XkNCZaw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: yshmJpZAm31UDuLTWpq6vVKxk6kG0EckYJ5LHrgmjZUNRAibdSUFiHwiPx3ui0q5bB1-hN5QO2E8A6zfTiz__A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: tSIef5nFjaLkzxqENZSBEbWItgmpuR3B0Aedx1iG_M9aZhDnenOofyYx4raV4F9uU9zrGDD3UBkDggsrV97STA
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
✓ Created SWR script: fetch_api_user (U0sidrgrOFXRF69UwlvURywADi7e_rjhfCEKnv1wtyAg1t4Zz6AHNqXNVzE5WSYKpPn8OGk7LazamhbSVd_3cg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "email": "Sincere@april.biz",
        "username": "Bret",
        "id": 1,
        "address": {
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874",
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "email": "Sincere@april.biz",
        "username": "Bret",
        "id": 1,
        "address": {
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874",
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (FU7-7LIqn_qOJ78AvHMCfv8vXPfsj4qlN35qdjt7qm5L_GblMnSfKi3L3kO3KrjpQnEBHL2AiVQxB3H5PW7-1g)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "stock": 99,
        "meta": {
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "id": 1,
        "warrantyInformation": "1 week warranty",
        "rating": 2.56,
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "sku": "BEA-ESS-ESS-001",
        "category": "beauty",
        "price": 9.99,
        "weight": 4,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "brand": "Essence",
        "returnPolicy": "No return policy",
        "dimensions": {
          "width": 15.14,
          "height": 13.08,
          "depth": 22.99
        },
        "title": "Essence Mascara Lash Princess",
        "discountPercentage": 10.48,
        "minimumOrderQuantity": 48,
        "tags": [
          "beauty",
          "mascara"
        ],
        "reviews": [
          {
            "comment": "Would not recommend!",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "reviewerName": "Lucas Gordon",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "comment": "Very satisfied!",
            "rating": 4
          },
          {
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5
          }
        ],
        "availabilityStatus": "In Stock",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "shippingInformation": "Ships in 3-5 business days"
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
✓ Created native SWR script: github_user_native (rExrtEF8UYOUL_ti-CbBgtLxQy1dPWE_ZTv07nyf7VIJkm3ZIF3onOE1DTrItd7yJLoF4ZpWCVs1T8OB9UzcHw)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 1.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (rGjzHuxt9muukTI4o1q6TuvVeAwueT-AfPTOZYp7w0xRrMp_4lg-3Wx0cuJQMOBQ5spMYIxGQFemCgPTX55vgQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (8QOsDlmazwuPh2paLXSL-BlsBqMLutj8-f7sVkCmoQRpFYlkW0CmU2Aqoppzpy6peRedNnlVuhgUOg3_zwc92g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (3OaEs8YFKLeRx0onAZtzmKeo3TQjEtE9GcZf1geS5V0nw_SdE7SHldhKVtcqIhUbUjQkG2WokVd-U0_CdPW36Q)
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
   {"avg_price":367,"count":5,"category":"Electronics"}
   {"count":3,"category":"Furniture","avg_price":365.6666666666667}
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
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in performing similarity searches for high-dimensional data, such as images, audio, and text, using vector representations.

2. **Scalability**: Vector databases can handle large datasets effectively, allowing for quick indexing and retrieval of vectors.

3. **Real-time Performance**: They provide fast querying capabilities, enabling real-time analysis and retrieval.

4. **Support for Complex Queries**: Vector databases support advanced querying techniques like nearest neighbor searches, which are valuable for various applications.

5. **Integration with AI**: They are well-suited for AI and machine learning tasks, facilitating the storage and retrieval of embeddings from models.

6. **Flexibility**: They can work with various data types and are adaptable to different use cases, including recommendation systems and natural language processing.

7. **High Dimensionality Handling**: Vector databases are specifically designed to handle the challenges of high-dimensional spaces, which can be cumbersome for traditional relational databases.

Overall, vector databases provide efficient solutions for modern applications that rely on complex data types and require rapid retrieval and analysis.
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
   {"count":7,"role":"user","avg_score":70}
   {"avg_score":20,"role":"admin","count":3}
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
   2. Vector Databases Explained (Database)
   3. Getting Started with ekoDB (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":3,"category":"Database"}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: p5RD3hp1iBkXTpUnJyvIRLM9ya3BgxrJnXngS3I3gnsqYIyhzLN8_CG0naNZ1ltBiQgDkP7DoFR6gLYqFQWxdg

=== Sending Chat Message ===
Message ID: G5a3LOMo6FNFQzZskhOonPUaGzQqBF3knGzKtuP0d2umr_QlixMMjSfwfA76x23fSM1Y7FnwhUVpPjHPbMSh5Q

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99,
    id: 'vEBCWK0hl7nVbgV6xION0ZYmMr8G0Oe5i78xshtppRYlS0JGek9kX_1vvGi7bzzwnDJ9qGd1jJBVh4MJxXdkXA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: 'r7VN74TM-UetMMGvbhME9ZJ1pQlNh7IHHnBzCVw17E9ArR_K-iC82EwgmETPtyFkGBokX0Y_CY4HHI5k6Nk0uw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'IG4s1ZI4r5hViJGjmQjTxQvqUb_oIeWVy5e1IgTLQLdoo-yVPDpqSPzUk2UgTwlGkN6ISQYVV6eOhJ2SkykoOw',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3826ms

=== Token Usage ===
Prompt tokens: 840
Completion tokens: 89
Total tokens: 929

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 3EO5YVO0oSCBFCP7A5Co4LwIOz7xpGH8I5RZ5utMnk-_UqkZ7xajEWpJKpwdv0MO3x8E2PXZSfAjeHXtIEG8ow

=== Sending Initial Message ===
✓ Message sent
  Response: The available product in the database is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or details about this product, please let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you need any more information or have other questions, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: bKuI4y_8m5uyFAaEfVS3PaOVxD7GjZ2RfSyAwirMSjpAY2aSjEvsK8V8bDcXP8XOEc-UXGCrUMSc1WAvIL7CKg
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
✓ Created session: iyFUX2cxcYpZjLC-aO8fe1MesTruIsYcWvaJcqe7grtxkIr7zf1qZv7m91QK4wHUelLrwZPQr1_A0N3fWKGVPw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
  - **Description:** A high-performance database product
  - **Price:** $99

If you need more information about this product or others, please let me know!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: rk9ULlCvRFrs5tkugweOfx8Qrze6eJfXQ5tlwccLA-kbrxe19PXshjSxjWIrVvjPgb8CZuw7nNJDdR2pXGcoBQ
  Parent: iyFUX2cxcYpZjLC-aO8fe1MesTruIsYcWvaJcqe7grtxkIr7zf1qZv7m91QK4wHUelLrwZPQr1_A0N3fWKGVPw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: rk9ULlCvRFrs5tkugweOfx8Qrze6eJfXQ5tlwccLA-kbrxe19PXshjSxjWIrVvjPgb8CZuw7nNJDdR2pXGcoBQ (Untitled)
  Session 2: iyFUX2cxcYpZjLC-aO8fe1MesTruIsYcWvaJcqe7grtxkIr7zf1qZv7m91QK4wHUelLrwZPQr1_A0N3fWKGVPw (Untitled)
  Session 3: bKuI4y_8m5uyFAaEfVS3PaOVxD7GjZ2RfSyAwirMSjpAY2aSjEvsK8V8bDcXP8XOEc-UXGCrUMSc1WAvIL7CKg (Untitled)
  Session 4: p5RD3hp1iBkXTpUnJyvIRLM9ya3BgxrJnXngS3I3gnsqYIyhzLN8_CG0naNZ1ltBiQgDkP7DoFR6gLYqFQWxdg (Untitled)
  Session 5: Yc_a7W4O9604Zk9lNVFl_NqbkG7vUB5Osr6abi8__5cRWYDqxRX6jYXgRIWOT8ISTdXTjI9IBKasVihl5pY8Hg (Untitled)
  Session 6: JWnsfRU1jdDeaYWO7MdIDo9H4YkYxvm1Z00Cjh_jbt69CPNJ8TRyt8zYTcitiNV3T7gtZp6rlOi9D-Rbny5VXw (Untitled)
  Session 7: OeeHcsFKYGiGZUjmxkrM2k2ObJVz-EvfRoRycmed4KJjbmLsZhBcBd-yUwujh-jicf84Ez3OdkehS9FFkbuYCw (Untitled)
  Session 8: lZ-dHfTQtOvK_Ambtc8tfQzEpmB8XwXEFdc7vVWFKvSkBZCItX0c-WY7CFvfFQQLHZtCvXNcMQWq6BcvIx4IJw (Untitled)
  Session 9: SJ__4_tAnJzZ6ZZlC8gl6-14igCopuku1oasc0pG2kbV-qkfDXM_WPbwt13pTfUavgCN6IERQv9JPoP9WLgPMA (Untitled)
  Session 10: _fo5hgR1vkLS2vlxwrtb4BPdLkZWZ8RcOzv2pNjf_ifz_JJ8rvT7bi9JmrohVuvII0KiZxR5Dbotgz57uXzZ-A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: rk9ULlCvRFrs5tkugweOfx8Qrze6eJfXQ5tlwccLA-kbrxe19PXshjSxjWIrVvjPgb8CZuw7nNJDdR2pXGcoBQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'USmFK67HZuhtq9u_VjxBTSxz2DIXnnqCPo6ffmSMXosWwH9HsAKruxNIpYuJo9NtGQ8QegsgPSGru-sSLte32Q'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  active: { type: 'Boolean', value: true },
  id: 'USmFK67HZuhtq9u_VjxBTSxz2DIXnnqCPo6ffmSMXosWwH9HsAKruxNIpYuJo9NtGQ8QegsgPSGru-sSLte32Q',
  name: { value: 'Alice Johnson', type: 'String' },
  email: { value: 'alice.j@newdomain.com', type: 'String' }
}
✓ Second upsert (insert): {
  id: 'cZXWHMQ-aOtthhwWD1TmtoFgTdKDlWz6E6__WTLtraY3aRApfbHREqhdCCfyEj3TZysLk0UfRsoCIKL2Jw_Mpw'
}

=== Find One Operation ===
✓ Found user by email: {
  id: 'USmFK67HZuhtq9u_VjxBTSxz2DIXnnqCPo6ffmSMXosWwH9HsAKruxNIpYuJo9NtGQ8QegsgPSGru-sSLte32Q',
  active: { type: 'Boolean', value: true },
  age: { value: 29, type: 'Integer' },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
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
   Inserted with ripple: {"id":"y_LGxuDvzmZL3Umf7dKQf7R8RtlZA_xdRdJ_pIR2odFkRshiqcGOD88-IzaZqMcQivrji2tOnUgZO4TOBW784w"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"H8m3iLSfsTjHyPNC7hIntjCUltGTvEIUVM1XecjAfJ7IJC4_W96_bbuUzI6cX7YSCz50JjrO9TljZyAxMrSGXA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"id":"y_LGxuDvzmZL3Umf7dKQf7R8RtlZA_xdRdJ_pIR2odFkRshiqcGOD88-IzaZqMcQivrji2tOnUgZO4TOBW784w","name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"price":{"type":"Integer","value":500},"id":"custom-id","name":{"value":"Upsert Product","type":"String"}}

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
  Fields returned: ["id","status","created_at","age","avatar_url","name","email","bio","user_role"]

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
    - Fields: ["api_key","avatar_url","email","id","created_at","password","age","bio","user_role","name","status","secret_token"]
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
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4-mini, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-4-1106-preview, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-4-0125-preview, gpt-4-turbo-preview, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: Pp2-aYZOoUpVd2lcrd9ewrw-HC0qJFrAXzLl0KCjyJV-bR4XP2QMW6SvpwNGvD9GU8O6XTiV5RGpYSPCM0wLTA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - get_active_users_ts: Get Active Users
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - cache_api_call: Cache External API Call
  - fetch_api_user: Fetch User with Cache
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - cache_api_call_py: Cache External API Call

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
