make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 835ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Fd_-IqDKfFaTgbG36gCjr0ulLsl9bsJ782tNLYkT15pZjpwup2R3-WR5KIZVCo0tdM38EjUsrcFJ3iA_-UJI5w'
}

=== Find by ID ===
Found: {
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Test Record' },
  price: { type: 'Float', value: 99.99 },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-01-05T15:34:11.604Z' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { value: 42, type: 'Integer' },
  id: 'Fd_-IqDKfFaTgbG36gCjr0ulLsl9bsJ782tNLYkT15pZjpwup2R3-WR5KIZVCo0tdM38EjUsrcFJ3iA_-UJI5w'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-05T15:34:11.604Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { key: 'value', nested: { deep: true } },
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  created_at: '2026-01-05T15:34:11.604Z',
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  id: 'Fd_-IqDKfFaTgbG36gCjr0ulLsl9bsJ782tNLYkT15pZjpwup2R3-WR5KIZVCo0tdM38EjUsrcFJ3iA_-UJI5w'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'Fd_-IqDKfFaTgbG36gCjr0ulLsl9bsJ782tNLYkT15pZjpwup2R3-WR5KIZVCo0tdM38EjUsrcFJ3iA_-UJI5w',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { value: '2026-01-05T15:34:11.604Z', type: 'DateTime' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { value: 'Updated Record', type: 'String' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 3XimrUlLON2XL36IAsahkWJe_g9CatKNy4D2lSB9c8acUMO1NNWof8T8YLmmUqt5_owXxcHxnt-yyS5iMMKMbA

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
Collection created with first record: Im4Gt8daO3NU3OEywM0UYkxhiWq3_OeZ6vV5WhZsIjAXZYTmi21CmxDHKvKbgh3y4yNj1my3YocQ5qQ07tPSTA

=== List Collections ===
Total collections: 21
Sample collections: schema_employees_client_go,chat_configurations__ek0_testing,schema_documents_client_py,schema_users_client_py,edge_cache_py

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

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ZL4hDogryOakKoo44rFKvWenMwC_IPBbUVXL7EY27czp-Wal8xNdfG2gnfDradSI2WqfW4NgabyHqTsqQbq8aA
Created Bob: $500 - ID: gy4XSoNXGozJnqqp9QVPF8kiWxeJCqK0DXADr-wbFbiWzlo9tqxAQ61gosUidQymC6NucxvB8WfT2PFNkCyt3Q

=== Example 1: Begin Transaction ===
Transaction ID: c96bb7a1-a043-4205-8ae5-4c3d03a818d3

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: de5b0e16-28f0-4b35-9a64-4426a4ee2d69
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
  1. Score: 25.740, Matched: name, email, name.value, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title.value, bio, title
  3. Score: 26.400, Matched: title.value, title, bio, bio.value
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: title.value, bio, bio.value, title
  4. Score: 39.600, Matched: title.value, title, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.766, Matched: 
  3. Score: 0.747, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.708, Matched: content, title, content.value, title.value
  2. Score: 1.499, Matched: content, content.value, title, title.value
  3. Score: 0.307, Matched: 

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
✓ Inserted document: HQ2E2_G3VD7jYmfYVRd7N0EF-E-8RhnPlBabMjsFNL4_kFbHJXdcT9tIu21bGejTQHbL0HXT1I25S3Jq32P36A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: hKHU0Gl7fyZo_vjyBs1qVPc7zDp8aeeBXczvUsbm8L9xJUWRI76QPtdvtbBnpaGwF7lVeJICSXarqwQ6IKsrQg

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
✓ Inserted document with TTL: IudBCfUFS0_d2pe2xJP-Hmhn0Gofn8LtsMEdfLh7AOw-p0sdg4eQDr3SRLwVjNfUo886iauSQMpTK0mRhOsZNw

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
✓ Edge cache script created: br0kdSjFyik_P4CoL8ePXjs-WyJe6GMIms6FPkYd-VN9a2ci_mWFMOr4eNm0qZeLqi9WKhSgQnUib4XWLzqGhg

Call 1: Cache miss (fetches from API)
Response time: 92ms
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:34:17.014Z",
      "id": "vmDJHfdNhk89suVwI156x46lkct8KieLZeyO2v_kw9xVkx1DVX3Qy7DnSuWMHdLfOAlj6VyMR3MzHQVad_jOrQ",
      "data": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "website": "hildegard.org"
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
Response time: 62ms (1.5x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:34:17.014Z",
      "data": {
        "value": {
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "address": {
            "street": "Kulas Light",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "id": 1,
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "name": "Leanne Graham"
        },
        "type": "Object"
      },
      "id": "8Rl21izu4dG2pOh1XsyspEH2WRX4n9v_K23srfezi29LJeRT51HInLvbJyB1HcRLNO1lEx7v9L7NpXjpMjX6bg"
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

✅ Script saved: yF62WUpdHrEhE6QGuncoOcc9LP93I4eGifPDEDz3XmvXHVAIpWgBqJvetmxZRuH8UYdJqlqYmo-xmZNyUCxi9g
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: 212-VJFuy_BlrLgMW50Yld4buroZUYMWBayl5n0b94wXcnOtiXBSzjCY11X2LFgDO9oHqYpKloZGci8euv2xKw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: i4VlwsxGKDqce4eYISx0xAjvV-VLtQXN7hIn1P_H9Ek4M5hlyPgDV-41BEy7UVmJIMfQDJGlP4hbM3x1RGcFyw
📊 Statistics: 3 groups
   {"status":"active","avg_score":60,"count":5}
   {"avg_score":60,"status":"null","count":5}
   {"avg_score":50,"count":5,"status":"inactive"}

📝 Example 4: Script Management

📋 Total scripts: 42
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 84ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 90ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: XipX7IOFNr-DBjU-Ckb3xEdK7coSNEk9uuSBq06-OlNga7x7oDvHth75XG1Tt4kIWPrsmZ9YSWBlEyXNac8-dQ
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
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

✅ Inserted order: jtxWpDmM_MFxZQG-Zz5VSMC2Keji3omoRlPL_kRYGwPB3TsZEREcJvQXfV-QDPPXNZ9v9P9TCgZUviAgxNAjBg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: -lfwRxGP30zQXhhR7C74qELdgPSOHc30ALbKOKZBbD9ifbo-kxl9b2lGlwb2NhIrI3y2fc188GStGvN-3WoeTQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session: undefined

📝 Example 4: KV Operations in Scripts

✅ Script saved: jTcwmMuFAw-PERz9KocuyfxLy53bd97TIrsWJdyXhqsaRS5KL3A_lxJShzx7LO9Bdscrd9-5fBDhpYzyx14oCQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: vegu5rXZFgB5eFc2c4ROKNIIO9OrO70NnzIVaEDcM17aI7rSPs4cEtkgVBUGNj9QO-e0ME1CcrtaF7Ue4lqeBg
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
✓ Created SWR script: fetch_github_user (yhNU24BgGG5qtHkdUBZY-ImTNKB46ZugX6uSlfl3K4SXbF5Os18pXw30MORPJkfuNAlUa35FjZ7k5l_slvmORw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:34:19.448Z",
      "data": {
        "type": "Object",
        "value": {
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "site_admin": false,
          "name": "Linus Torvalds",
          "type": "User",
          "html_url": "https://github.com/torvalds",
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "updated_at": "2025-11-24T04:16:14Z",
          "followers": 270115,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "public_repos": 9,
          "company": "Linux Foundation",
          "location": "Portland, OR",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "following": 0,
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "created_at": "2011-09-03T15:26:22Z",
          "hireable": null,
          "blog": "",
          "login": "torvalds",
          "public_gists": 1,
          "user_view_type": "public",
          "bio": null,
          "email": null,
          "gravatar_id": "",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "id": 1024025,
          "twitter_username": null,
          "url": "https://api.github.com/users/torvalds",
          "followers_url": "https://api.github.com/users/torvalds/followers"
        }
      },
      "id": "CEk0N1QiSq7KzqWC3rboQkBs8vnArPSuJNg5n4anf06AqAzQZMAzUnixRKu_3qzS_kwIIXAJ5G-7Zk6EMt75hg"
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
Response time: 75ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (ZPIw2fzZe4lZG-314Dwau2UIY1GajyNdYakcomnvmq0_tzudUfhxzRATD4OjwPEiZg31UVj7xS9AWxgRFT-O-w)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2026-01-05T15:34:19.655Z",
      "id": "ia1rruUgxkLCcenvsNyDD5ulFS7xDCjzc-5XPEmvefvM3W91iU3G5i3o8uyk05bVDH71IjVMi9o1Q66gzOunEQ",
      "enriched_data": {
        "type": "Object",
        "value": {
          "rating": 2.56,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "warrantyInformation": "1 week warranty",
          "id": 1,
          "discountPercentage": 10.48,
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
              "comment": "Very satisfied!",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "rating": 4
            },
            {
              "rating": 5,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "weight": 4,
          "category": "beauty",
          "stock": 99,
          "availabilityStatus": "In Stock",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "meta": {
            "barcode": "5784719087687",
            "createdAt": "2025-04-30T09:41:02.053Z",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "shippingInformation": "Ships in 3-5 business days",
          "sku": "BEA-ESS-ESS-001",
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "title": "Essence Mascara Lash Princess",
          "minimumOrderQuantity": 48,
          "brand": "Essence",
          "returnPolicy": "No return policy",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"
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
   {"count":3,"category":"Furniture","avg_price":365.6666666666667}
   {"avg_price":367,"category":"Electronics","count":5}
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
   Vector databases offer several benefits including high data precision, representation of geographic features in their true form, efficient spatial analysis and processing, easy updating and editing of data, and support for various types of data such as points, lines, and polygons.
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
   {"role":"user","count":7,"avg_score":70}
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
   1. Database Design Principles (Database)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
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
✓ Created session: tTeTYVvoCpkCE8mwvywHqGTvhBESl6Y3-hK-sJYWZuTZ84GtFd9LpyY1FiSJkud0UbisY45OGTN7phcAe77HKQ

=== Sending Chat Message ===
Message ID: 0HOdjGwm2AYSrJu11uMBgzUiTbFZpGazxQj9N8q_qht94tccWihqidH4lG9p74cFCx4DIlS65wwFBOGnRbv17Q

=== AI Response ===
The available products are:

1. ekoDB Cloud: It is a fully managed cloud database service product. The price of ekoDB Cloud is $499.

2. ekoDB: This product is a high-performance database with AI capabilities. The price for ekoDB is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: '9F1OwvTEpdjdJK0hprmdKMCNKkuExNJO5xg9VJWWf-KnOHFVJo_qfYQoCXNsh84gZyL5J-CFKGPpb0X9ht9Wug',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99,
    id: 'InL9qWTrO0dnU2nDoOb4RhH78WMzyi-QfqiELSEaM5IU5IzPSb8WOP1lGeSt00GihQYwmc9BpO00Z0B_ST0_gw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    id: 'ATP8N8a7ju9CwlQV5TF6xhQ-VvffQyEmvoa4UlFUubysy6ONh5p-Z5tDPAU_fVp7h97YqcMzOCiPQDKEzeMPhw',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4666ms

=== Token Usage ===
Prompt tokens: 608
Completion tokens: 88
Total tokens: 696

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: _zHOReQbiG3YuA0rPmKK_LI6POxGdEy8QOAxunP8RvlHMFGaOZBoaJtOBcM7n2KwEtEX_1IC-INASNPftxXRdw

=== Sending Initial Message ===
✓ Message sent
  Response: The product available based on the provided context is "ekoDB". It's a high-performance database product and the price is $99.

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
✓ Created second session: Ln9JpayhkcQBVZmhSyUlBii4_1YO7Jv735ptsTk2UMjzkE_C8d2oCYSqlcBdq1P1D5N499ZseVzWUcLa-nEg2Q
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
✓ Created session: i__SpHRQbfx2YOk5wES38fvcZ8XvmLjLBCSPblkPuOJ6aHpl2886G9c4bWcV9IV1aEK-r5Be_60X8Ebcadde9w

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, we have a product named "ekoDB". It is a high-performance database product.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: HGfxv8QGSncmCtRj1or7UTVXbEk7-ikcOztK-LZuekfPWm4SJ9W7-9jbaYWV_Kk6sGqoQMtCe7NZ_DiPiOgw0Q
  Parent: i__SpHRQbfx2YOk5wES38fvcZ8XvmLjLBCSPblkPuOJ6aHpl2886G9c4bWcV9IV1aEK-r5Be_60X8Ebcadde9w

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: HGfxv8QGSncmCtRj1or7UTVXbEk7-ikcOztK-LZuekfPWm4SJ9W7-9jbaYWV_Kk6sGqoQMtCe7NZ_DiPiOgw0Q (Untitled)
  Session 2: i__SpHRQbfx2YOk5wES38fvcZ8XvmLjLBCSPblkPuOJ6aHpl2886G9c4bWcV9IV1aEK-r5Be_60X8Ebcadde9w (Untitled)
  Session 3: Ln9JpayhkcQBVZmhSyUlBii4_1YO7Jv735ptsTk2UMjzkE_C8d2oCYSqlcBdq1P1D5N499ZseVzWUcLa-nEg2Q (Untitled)
  Session 4: tTeTYVvoCpkCE8mwvywHqGTvhBESl6Y3-hK-sJYWZuTZ84GtFd9LpyY1FiSJkud0UbisY45OGTN7phcAe77HKQ (Untitled)
  Session 5: HlgLEuO5eVHTzp6emc9vrrKYyDWQ_IXDgKzI1_2UEdimEilMntvib7YtjlQgvo4ULsDrHzblIa1vlqaZnRedrA (Untitled)
  Session 6: 4XcmiBaLEOBsWSazHwzTs9wPONLFauBk7LZdGPM9-ARmVHIYgjyE9sNHDGnBZQ3o1ShlaUag_CEdZiYGkdgiCQ (Untitled)
  Session 7: aR-Adl9yzHwPf704PXKnJCZnaJlOPXDFagZ71UxUCWKDj7zBUGseVZJaZx2ziMfvsQ20kiytyD3UqhPjfCNiVQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: HGfxv8QGSncmCtRj1or7UTVXbEk7-ikcOztK-LZuekfPWm4SJ9W7-9jbaYWV_Kk6sGqoQMtCe7NZ_DiPiOgw0Q

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mTypeScript client examples complete![0m
