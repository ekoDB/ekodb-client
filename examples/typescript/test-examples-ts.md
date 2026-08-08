make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.25.0 prepare
> npm run build


> @ekodb/ekodb-client@0.25.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.25.0 build
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
  id: 'RO6ZYZxnZdHY5w6LkO-fHsl0ghoq877Qu0W5SB_01w6vpYS9YG4OWay9z5Zpbo6qdRD5iyc5V8xaPOqFEiaw8Q'
}

=== Find by ID ===
Found: {
  id: 'RO6ZYZxnZdHY5w6LkO-fHsl0ghoq877Qu0W5SB_01w6vpYS9YG4OWay9z5Zpbo6qdRD5iyc5V8xaPOqFEiaw8Q',
  value: { value: 42, type: 'Integer' },
  price: { type: 'Float', value: 99.99 },
  name: { type: 'String', value: 'Test Record' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { type: 'DateTime', value: '2026-07-14T13:50:18.375+00:00' },
  active: { value: true, type: 'Boolean' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-07-14T13:50:18.375Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  id: 'RO6ZYZxnZdHY5w6LkO-fHsl0ghoq877Qu0W5SB_01w6vpYS9YG4OWay9z5Zpbo6qdRD5iyc5V8xaPOqFEiaw8Q',
  value: 42,
  price: 99.99,
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  metadata: { nested: { deep: true }, key: 'value' },
  data: 'aGVsbG8gd29ybGQ=',
  created_at: '2026-07-14T13:50:18.375+00:00',
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  categories: [ 'electronics', 'computers' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'RO6ZYZxnZdHY5w6LkO-fHsl0ghoq877Qu0W5SB_01w6vpYS9YG4OWay9z5Zpbo6qdRD5iyc5V8xaPOqFEiaw8Q',
  price: { type: 'Float', value: 99.99 },
  name: { value: 'Updated Record', type: 'String' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { value: '2026-07-14T13:50:18.375+00:00', type: 'DateTime' },
  active: { value: true, type: 'Boolean' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 100 },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 47t_FDSyQqKespuqoJ8amy-dQMl9tyit5XQcjB86bWq6yGUeuW8rYtn2di8jXwETG3kB4B10dw49Oj9jmX_Taw

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
Collection created with first record: aEDqecVDrrlsDryEoVoMlJUW2d7SVfV-BP7CRnkKuLcAU3BP8YEH6Faosv8IEy9SkzY6HWL9CTpkPcO3reVJVQ

=== List Collections ===
Total collections: 23
Sample collections: audit__ek0_testing,products,functions__ek0_testing,schema_users_client_go,users

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
Created Alice: $1000 - ID: JI5jLoFO2OFhwpn6rQZ9gTJEjPsmT9W2cKN4oLsFPwpDZkpaYWTq27koYmiqFm5sfiJLWwuPlMy1X9bitrNkZw
Created Bob: $500 - ID: cg7HgJLO7J1U-OVTTz8nv_aWAUyraySATq4SpU5eOc93hfXxveijSrE9bhuzbpNDSZ0P4Is5xMuA-SIg4QsDPw

=== Example 1: Begin Transaction ===
Transaction ID: 9e9d9927-a371-4d99-a6a2-a66523622ca6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3beedf42-3b43-41c0-90fb-080d8bbf5494
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
  4. Score: 13.200, Matched: title, bio

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
  1. Score: 0.752, Matched: 
  2. Score: 0.748, Matched: 
  3. Score: 0.744, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.498, Matched: content, title
  2. Score: 0.901, Matched: content, title
  3. Score: 0.299, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

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
✓ Inserted document: 9Y2BAri4aNS6dWZuyNqzld1tInStfBJe4RJ8MYXhINJfK1fXUVaeNk6Jvy2vrw_mKFO6Hs9bWcM8BUd-SYpnug

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ZIEm3K3keZGoYajrsNhZqmdc8vocj0jDOqzDaSGZOAmqEs8dlRjKvdekz_fMYvEdP-U5WoHHR_AhZU2vByITuw

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
✓ Inserted document with TTL: c2WKxm1hNrLClgaCVdqWgTkLdmvHp2VB0Y9rJcT-REw15bOJp-pJbQH9hZICnqntC4Trraq_-dL6DcT0NYnzLg

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
✓ Edge cache script created: KQi-8scDkISboSAuAb70dh_kYJEFw5w3Ubxx3QQG3e_ENo30-L0Qu0sB_ww5pbgQhkmMJ_TcknOlBAlzJYr5VQ

Call 1: Cache miss (fetches from API)
Response time: 67ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442"
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

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (22.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442"
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

✅ Function saved: vsqr4wahZ2UYu4x18M9ix_FNbEKpWJ3WYBpP9UmV0IX_4tWOa7lDdATil4sdKbT4YUfqI8I3QQnyNkbnCtH9Bg
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: kNGk5xK6hmOwy0fNFR-QOzaHtxiYO0lUxba-mJMkErR0eqFYlF4j_UayfXop58jPfW9uLOInNVhdDWTwiuo1QQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: TVY_nyX25qyuNHOyZSs8GEC8MxXlHVeiwBiqkF-c63czrZRWXxvYWkOb7RwVF6rH4ojXTBmfHtjG6zDgMO3Qyg
📊 Statistics: 3 groups
   {"status":{"type":"String","value":"null"},"avg_score":{"type":"Float","value":60},"count":{"value":5,"type":"Integer"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":50,"type":"Float"},"status":{"value":"inactive","type":"String"}}
   {"status":{"value":"active","type":"String"},"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"}}

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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Ro...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona"
      }...
   🚀 Cache speedup: 0.8x faster!

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

✅ Function saved: X7X1Kcvca6CEFPldh_3OERIm9WJcO1SIPXiQtgjWYioYt5CvmwtFrQMRAuiQZPIctOn6qdtxeP5PPjNLalfxRA
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"},"count":{"value":3,"type":"Integer"}}
   {"avg_price":{"value":474,"type":"Float"},"count":{"type":"Integer","value":2},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
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

✅ Inserted order: PQUw_rua-MnjJt0ZNSPlK1IfdQVEkp0D8er0c0O5H9RU27upehm3ZODxT0y79HaK5n1LNy3zEJuhdrkymCVogA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: BMaLHv-YuHcfxJfdjbzwq6OFNuaiPWYkKHAOS1EmQrfCXnykR3UpUZCWqcngb9oO-C6JpHhTWtPW1mGvVUPoUw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: r3spL713dkVBCn77PNwY4c_0XYxUF0uCSXjcsWL3sWuwOSSzv6MNxsiVfICx1rmxIxaV8kAfh0SnsQ0gHR6GUg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: heKX8Uo43MjcAse0N6nTFgihsexXNyW5xIW3lG5EpLntaSN2s8VzRCWLY2EJuYUnTXNDdqrxDdHpb0Bk6aHQnQ
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
✓ Created SWR script: fetch_api_user (f1iEG7NeZk-Q_sa0F0_UIemC4FAr5SARNTcntG0sJw_D1rjUM6Np3sRkQfKU27uBz53sq1LIQuLPA4c2lVEPXg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret"
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
Response time: 2ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret"
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
✓ Created enrichment script: fetch_product_with_reviews (JNd4ytf_-kF6ArDeNwVHYpHY0yLZIxC25JxJjhcuRkbRewstiZpgdQvPaKUbIAuQrnEVgn349yCLhf-775AnqA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "brand": "Essence",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "shippingInformation": "Ships in 3-5 business days",
          "title": "Essence Mascara Lash Princess",
          "returnPolicy": "No return policy",
          "id": 1,
          "discountPercentage": 10.48,
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "availabilityStatus": "In Stock",
          "reviews": [
            {
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "rating": 3,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "comment": "Very satisfied!",
              "rating": 4,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon"
            },
            {
              "comment": "Highly impressed!",
              "rating": 5,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "minimumOrderQuantity": 48,
          "rating": 2.56,
          "warrantyInformation": "1 week warranty",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "stock": 99,
          "weight": 4,
          "category": "beauty",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "price": 9.99,
          "meta": {
            "barcode": "5784719087687",
            "createdAt": "2025-04-30T09:41:02.053Z",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "sku": "BEA-ESS-ESS-001"
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
✓ Created native SWR script: github_user_native (qgR4HRkbvNagJW2iVGF1mDDPWeYGhDcMjSKOEnSQ4dH_KVcz5fzCusS_m67dEiKP2pKY9jGiNM0369g_g78Dpw)

First call (cache miss - will fetch from GitHub API):
  Response time: 2ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 0.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (PQrb1O7Xwp1lSOgpmTN9J3YK3qh3qX_XyDaSKeLDe3k65ZYVZtuw4iBIXKXjx8VAZfB7MGbkLNLE3A0BCvKHHQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (D7NSMfnN7-DXBhlnP5WyqO6pAlylzr2EfytZqEe7SwmUBD5EyhBzt5olyss5cVz9xAdHVe0eVa2NbINpLN081A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (zw5zM5G_d4Ihh6gVdgzCksUa2qejmKru8Geuom4S3FoTSOVaSfgnMmtHXMvBBDByhZW0JGJRAXbwZTZyPFw3NA)
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
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":5},"avg_price":{"value":367,"type":"Float"}}
   {"count":{"type":"Integer","value":3},"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"value":"Furniture","type":"String"}}
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

1. **Efficient Similarity Search**: They support fast nearest neighbor searches in high-dimensional spaces, making it easier to find similar items quickly.

2. **Scalability**: Designed to handle large volumes of data, vector databases can scale horizontally to manage increasing datasets.

3. **Support for Unstructured Data**: They effectively manage unstructured data types like text, images, and audio by representing them as vectors.

4. **Enhanced Machine Learning**: By providing tools for vector operations, they facilitate advanced machine learning tasks such as clustering, classification, and recommendation systems.

5. **Real-Time Analytics**: Vector databases enable real-time processing and querying of data, which is crucial for applications like search engines and recommendation systems.

6. **Customizable Distance Metrics**: Users can define and utilize various distance measures (e.g., cosine similarity, Euclidean distance) to tailor search and retrieval processes.

7. **Integration with AI/ML Workflows**: They integrate seamlessly with AI and machine learning frameworks, allowing for easy deployment of models.

8. **Improved Performance**: Optimized for vector data, they often outperform traditional databases in specific tasks related to vectorized data.

Overall, vector databases are ideal for applications requiring high-performance similarity searches and handling complex data types efficiently.
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
   {"role":{"type":"String","value":"user"},"avg_score":{"type":"Float","value":70},"count":{"value":7,"type":"Integer"}}
   {"count":{"type":"Integer","value":3},"role":{"type":"String","value":"admin"},"avg_score":{"value":20,"type":"Float"}}
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
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"AI"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Database","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: y13STnGRlV8yVtIOyoLZ1umXLkWfiFqDhf0HA_WGnHbpkwAk53NEgjkgJ3VT2i2YMmgrKfTc3_PH3NUNs4sd3w

=== Sending Chat Message ===
Message ID: duMOs6bdPVXUQ9ZeK6m0TMMRnnItd1QC_XSM1RjwCOn7tlVWZRxbn1q0QTSpFjIo_NscLfMNDG08rbW2qBMkTA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: '93vQgM92kC3RpMYk4AedtVEY0iCV6SrFtHZGl02o313FVswLx6DIsSeJlFQF8gkyjIjjMfm9XCmsSU6BGnOfGw',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'ZvlvCBVGO6-5vn94IoiTUjsGUHOscbHinEjLwOteXNgkhLUC1tp94DNxsIARzBNKJiUYrqsjD3xnwNfKf30NLw',
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'PeNHZ_5B8xNrwi_CsuXZ08s4hxt3NRJ2ChmVIAbROJujpHHTX7bAI7fG2LjiJaj-rqn0uKyp3DSD-eYS-17iRA',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2137ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: V5wWwQxsBR-jQiuplF9Me5WQWs8ANlYUoBpaGPBTgaX6Kxc7NReyxEwdHJRS4yOv4Mu54flnZCZPzWriEo8OZQ

=== Sending Initial Message ===
✓ Message sent
  Response: It appears there are no available products in the database. Currently, only the ekoDB product is available at a price of $99. If you need information on any other products or additional details, please let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any further questions or need assistance with anything else, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: kh63L_Y5cAQ8KiVa8b7WjW98FN4ggbXSH_Kk-FUGlEqa6zM6YHI2GKnDi0pS7MR2hzRVGAMg36eRndnAO3pTrw
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
✓ Created session: DyNzgkvtjaAEoImVA8qNctSkz8BbiwUXfGUJMDK4mTzHUOfuIQUdw216S1dx6wZcldM2PCrYlhsRidPdyZwOsw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you have any further questions or need more information, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: DeCFB5gVG70zEvQmDqpbZz0urvijHB5PzDb1DtO6iGcyJeqwDXChtLgF-o1vqPC3XKFFuJJGQLQfEqezbIIxdA
  Parent: DyNzgkvtjaAEoImVA8qNctSkz8BbiwUXfGUJMDK4mTzHUOfuIQUdw216S1dx6wZcldM2PCrYlhsRidPdyZwOsw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: DeCFB5gVG70zEvQmDqpbZz0urvijHB5PzDb1DtO6iGcyJeqwDXChtLgF-o1vqPC3XKFFuJJGQLQfEqezbIIxdA (Untitled)
  Session 2: DyNzgkvtjaAEoImVA8qNctSkz8BbiwUXfGUJMDK4mTzHUOfuIQUdw216S1dx6wZcldM2PCrYlhsRidPdyZwOsw (Untitled)
  Session 3: kh63L_Y5cAQ8KiVa8b7WjW98FN4ggbXSH_Kk-FUGlEqa6zM6YHI2GKnDi0pS7MR2hzRVGAMg36eRndnAO3pTrw (Untitled)
  Session 4: y13STnGRlV8yVtIOyoLZ1umXLkWfiFqDhf0HA_WGnHbpkwAk53NEgjkgJ3VT2i2YMmgrKfTc3_PH3NUNs4sd3w (Untitled)
  Session 5: JZXsdYZRowdSNnZC032ydC1IzfTYLbRWpHFsE88nJurrWEPU0PI3q-JLNCAcfF2ZKHALHQZU1v4rN6hakdJY_g (Untitled)
  Session 6: 0dx8aDH0e8RiG18HdyqkVaDYNAEO2fEpN4pWEznplWmNPHbZAEilObvPbVGmetQq8kRVsjnVTP7e5vKlDZvgSQ (Untitled)
  Session 7: G_8w79TiVJKFrYmE8bAkQud9vdaHl8IbnT9tGq9o3CVOjU90bk2JgrCBki15lCiU_ij8pEeIxRf4-tKWXu7l2w (Untitled)
  Session 8: r7WPdqwlcHC_G8bUVExMEIQxl49agy-fuwLtSQFNbE_xvN04r96fw4vSL_t44KICXCfkJ1if-ziB7sa59TQDNQ (Untitled)
  Session 9: 6_YNwhItnPEY95NBWHOKtlC0SifEp_7tEs0ifMM9MU75wlOF12GsqvLKNp7d5jQh_QE9el7fET13yIaX301_4A (Untitled)
  Session 10: r_wc02Y4Gj4Oe95sscpaoNiwXFY2xq5ICG5_mlCQyko4WnJsHw8mO5PhlM5zAuyeabpTT2-ZvI46ge7ETicnbQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: DeCFB5gVG70zEvQmDqpbZz0urvijHB5PzDb1DtO6iGcyJeqwDXChtLgF-o1vqPC3XKFFuJJGQLQfEqezbIIxdA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'L4Ak-R_ySRTPF2lVFZ5JkzigloEFm90lImfRNCgl-UgU5oDnBsLh3hCtCHHG3xzmZcHuJftmt-WofERiJJvCFg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { value: 29, type: 'Integer' },
  id: 'L4Ak-R_ySRTPF2lVFZ5JkzigloEFm90lImfRNCgl-UgU5oDnBsLh3hCtCHHG3xzmZcHuJftmt-WofERiJJvCFg',
  name: { value: 'Alice Johnson', type: 'String' },
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' }
}
✓ Second upsert (insert): {
  id: '0rCjjvulPdPOonbX1-ypPZqcH-w37HQP7BQgw-ZOJNDYCrDR52YL2j7YhLPonYfGAyLnwdzknZ4vLs6HMm3DOQ'
}

=== Find One Operation ===
✓ Found user by email: {
  id: 'L4Ak-R_ySRTPF2lVFZ5JkzigloEFm90lImfRNCgl-UgU5oDnBsLh3hCtCHHG3xzmZcHuJftmt-WofERiJJvCFg',
  active: { value: true, type: 'Boolean' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' }
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
   Inserted with ripple: {"id":"2hP47lCQGq7l6nq37qLIP4Mk4NudRPgpvbWwAjG1aRPBpVhkRy8OvG9Jc4XOWFfv-fyYySfn-2abBjuVjIGwzw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"vX6JrHMQMeFRnwDxWLBOWjJaBQQAidzFIozY1n0TWmpw9ykx25BvxgFRJc1DoTstY00W4N5I1jtDxB3gdzb7Wg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"value":"Product 1","type":"String"},"price":{"value":150,"type":"Integer"},"id":"2hP47lCQGq7l6nq37qLIP4Mk4NudRPgpvbWwAjG1aRPBpVhkRy8OvG9Jc4XOWFfv-fyYySfn-2abBjuVjIGwzw"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"price":{"value":500,"type":"Integer"},"name":{"type":"String","value":"Upsert Product"},"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email","id","name"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age","user_role","email","avatar_url","status","created_at","name","bio","id"]

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
    - Fields: ["age","user_role","name","api_key","created_at","status","bio","password","id","avatar_url","secret_token","email"]
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
    - gpt-5.6-sol
    - gpt-5.6-terra
    - gpt-5.6-luna
    - gpt-realtime-2.1
    - gpt-realtime-2.1-mini
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
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: BRHBoSDmxHjO_Q_flbGSmdgyTvyji-S6DpMwW131YZtrwMb3nhCx67ohwcnzf_cocOFVSohPxIy7JpfTz4facg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_active_users_updated: Get Active Users (Updated)
  - get_verified_user: Get verified and validated user
  - get_active_users_ts: Get Active Users
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call: Cache External API Call
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - cache_api_call_py: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_api_user: Fetch User with Cache
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
