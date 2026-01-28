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

added 1 package, removed 1 package, and audited 13 packages in 834ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '0ezsXc_PQcUlCwzUd1-5bRIYcu4KdxeF7ENRbUDx3xF-_ViR0wRBl4k19nFmUq1rRTpaAgJLk9pgcPwncgra8Q'
}

=== Find by ID ===
Found: {
  metadata: { key: 'value', nested: { deep: true } },
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  active: true,
  price: 99.99,
  data: 'aGVsbG8gd29ybGQ=',
  id: '0ezsXc_PQcUlCwzUd1-5bRIYcu4KdxeF7ENRbUDx3xF-_ViR0wRBl4k19nFmUq1rRTpaAgJLk9pgcPwncgra8Q',
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-01-28T00:13:09.653Z',
  name: 'Test Record'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-28T00:13:09.653Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { key: 'value', nested: { deep: true } },
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  active: true,
  price: 99.99,
  data: 'aGVsbG8gd29ybGQ=',
  id: '0ezsXc_PQcUlCwzUd1-5bRIYcu4KdxeF7ENRbUDx3xF-_ViR0wRBl4k19nFmUq1rRTpaAgJLk9pgcPwncgra8Q',
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-01-28T00:13:09.653Z',
  name: 'Test Record'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  price: { type: 'Float', value: 99.99 },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { value: 100, type: 'Integer' },
  id: '0ezsXc_PQcUlCwzUd1-5bRIYcu4KdxeF7ENRbUDx3xF-_ViR0wRBl4k19nFmUq1rRTpaAgJLk9pgcPwncgra8Q',
  created_at: { type: 'DateTime', value: '2026-01-28T00:13:09.653Z' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 3LfXCq69fOv0E9tEfCxn0in7Zu2eNfIZKKrGzIqbHFHysq4aCfD8tePwvk4z1Jur_5q_Zycym3x6ynV3XTp6hw

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
Collection created with first record: YPltC5hKLLBik-dTrKWlYUedTF03AufMsRi4gznb3O4r_hdt9TJJwedHwD3vd7-MfT8BEpBkKlEue1YaKUN-nQ

=== List Collections ===
Total collections: 21
Sample collections: schema_employees_client_go,test_accounts,schema_documents_client_py,products,schema_products_client_py

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
Created Alice: $1000 - ID: Z8Tdo6f0xpw1jL6hz7LrJxcQlGUaaxT2PEj0gt5keWJ95Q_uh3GeF9C1NQRRJjNfjm1CqsDWKGp3-Su8vQnxKA
Created Bob: $500 - ID: NPcdL8cJn7DVXeKK1eWst8S1quI1Prn1LLNM7xxIP1Bm6M4kS0r6qAukHsnSZNbh_TtmsviHAU6uZP6jpF2XNA

=== Example 1: Begin Transaction ===
Transaction ID: 9804c1f5-868c-44b7-87a2-b730f10c3dee

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 20adc88a-233c-4b2e-8aa5-4b8b30ac801a
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
  1. Score: 25.740, Matched: name.value, name, email, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio, bio.value
  2. Score: 26.400, Matched: bio.value, title.value, bio, title
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: title, bio.value, title.value, bio
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: title.value, bio, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.780, Matched: 
  2. Score: 0.766, Matched: 
  3. Score: 0.746, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698, Matched: title, title.value, content.value, content
  2. Score: 1.507, Matched: title, title.value, content.value, content
  3. Score: 0.312, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio.value, skills.value, bio

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
✓ Inserted document: wNC_UZ87x-OPQrtmUcRjBqVjni93pcvQDRXZU_Q1HLAsKBQaGLQW-yHSBViD8kPfRpiQ5lAK897EXl32chXpSA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: yn7c6889_pZ2npT7uoRUwZZ7boVjtw_rfv63DvVBYLoq4RMInRaMzUPYYLRncm1fhygQvHHLtNqYLpwBBQe-bg

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
✓ Inserted document with TTL: -EBZFSOQ9N6l06uSu6UBsUM94y2WNhVbJCTBJyOuIoj2OwqqnUfkFaopheQMs3ZML7VF0wETLz_nBYBq46S6hA

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
✓ Edge cache script created: TL7CBus4tu2F-vcYkRlQmZJtCbHkAtTWRnDBuE1A-oa2f53PFDABnMsOZqFpxMzeJMAjtup5Yd0vUq3TvDe34A

Call 1: Cache miss (fetches from API)
Response time: 73ms
Result: {
  "records": [
    {
      "value": {
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "username": "Bret",
        "website": "hildegard.org",
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
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
Response time: 4ms (18.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "username": "Bret",
        "website": "hildegard.org",
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
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

✅ Script saved: zibO53amUc27NCbFtoQ06qDzdcSoQNZXh_0f3EjdTGqx2Wkg5FR6_rKG4DxSbCQbbXY-n2FFZndLNDJgaKwudw
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: 1uirWcDDV6viBPPeSs1mVLXkqyTCmgiVuIVquZVCKPqu4E-AdEPVR4dMuWc7NDjid3JXHvw9o7L_zOj0gn0o0Q
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: LwC8OMnIb_O-uX6SmVqBk9htvpN_6sOkl1aA6n6CZGqrGEhGae95LeUFamZAzMthL3qK1JFtpnJfwKWQZwAYiQ
📊 Statistics: 3 groups
   {"status":"null","avg_score":60,"count":5}
   {"status":"inactive","count":5,"avg_score":50}
   {"count":5,"status":"active","avg_score":60}

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
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harnes...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harnes...
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

✅ Script saved: wiPjTiZAKKsazJ7yKC8FTQOA_PruxJUyfHFEHwMoavX7_xKUV8TkUqWcONedOL8tL0k0n7RVAxSutkCJk9EZEA
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
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

✅ Inserted order: mUxW9fMPfd1kboPwWn9XPDtP7Eh8U7UvtcVWVud8Zr96Nzz0WU9Vz3BDBHREEjx0SvSfqVFWR1niBDpNbFERZA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 0eJOHTfY-Zc9nWLdKcrCO3tWJ-8dpWMYbdUnumKmMA5epK30iH40iOCwz8n0qVeqw30f-hqlbIO-X2ew8PR23Q
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: LKEc_lIruVV59_Qs4ZapuqakOBoGkzxHLolGAEhDAIJh-SLs46humPligsObnlvIwCAx7UMYqs1DysjyMQ0a4w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: JUOb-8SNMwBx-uEZgcWizRVAkLEwr8kgZgJU7JsEoqANi_Li-InpLrygOHIyudAGU2V3er3K82hH24gWfBOKvQ
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
✓ Created SWR script: fetch_api_user (MrZHUYxYUYGb8F9ChE8L2UMT3wdWMnEKKYw_Ids2epKN3_LAove2XVsd4ldiraZkXWAMFFjrNn0cC_1vRxXrKg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "street": "Kulas Light"
        },
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "website": "hildegard.org",
        "id": 1,
        "name": "Leanne Graham",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "address": {
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "street": "Kulas Light"
        },
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "website": "hildegard.org",
        "id": 1,
        "name": "Leanne Graham",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (JDWESt9HXATr_BTLPWIITesHUi6qWeIVkwoDCF7lIxi3ZnNCvqPLE5nHk01dYHupH_p3maEbvo0VD75MUFC8sQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "id": 1,
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "returnPolicy": "No return policy",
        "availabilityStatus": "In Stock",
        "price": 9.99,
        "sku": "BEA-ESS-ESS-001",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "stock": 99,
        "minimumOrderQuantity": 48,
        "shippingInformation": "Ships in 3-5 business days",
        "rating": 2.56,
        "category": "beauty",
        "dimensions": {
          "depth": 22.99,
          "width": 15.14,
          "height": 13.08
        },
        "reviews": [
          {
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 3,
            "comment": "Would not recommend!",
            "reviewerName": "Eleanor Collins",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon",
            "comment": "Very satisfied!"
          },
          {
            "comment": "Highly impressed!",
            "reviewerName": "Eleanor Collins",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          }
        ],
        "tags": [
          "beauty",
          "mascara"
        ],
        "meta": {
          "createdAt": "2025-04-30T09:41:02.053Z",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "barcode": "5784719087687"
        },
        "title": "Essence Mascara Lash Princess",
        "brand": "Essence",
        "warrantyInformation": "1 week warranty",
        "weight": 4,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "discountPercentage": 10.48
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
✓ Created native SWR script: github_user_native (qrMvcFH277XSbx7ZYfrXGUXWxetTLENXNCubVcZPlxSKfqA0h6_HpCEt3pMf2ErfOFOj13rrpKJxC2c2YXSoSw)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 4ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (svI2fHpbGlwq20FcbGweuSh2_oULbSCbJQrt86w3mJhrfd2Tub4eQzMMF3TTLjz_gvUVpRKAzaFOe5KCwlp_YQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (uyBfhkpCkwY7p6nlOB9eJMlbVHZuDNIza3D1O0tGz58RBBfhqqXKbnt9c-Pc0xZ1S40DTBUEwAfE5qZae48H2Q)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (KpwpDAQ7xzgphoxPMlAM1H1vDTtCQRZi9AhHQizZHQcWnz9WBqvlzEpaxBO4HYbbnc4NhgQ-2zeliGHzGItziQ)
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
   {"count":5,"category":"Electronics","avg_price":367}
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
   Vector databases offer several benefits including: precise representation of data points, lines, and polygons; high quality spatial analysis; efficient storage for complex data; and easy data manipulation and scalability. They also maintain topological information, allowing for advanced spatial queries and analysis.
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
   {"count":3,"role":"admin","avg_score":20}
   {"count":7,"role":"user","avg_score":70}
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
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 2WR0InFC-qeXX9VK4maTnsaT0VQ9MZMJjG-GYITX1ZZy6xBtyDrI785FLoDXPUOMUL872PUCs5UDnbBZ0Z3uNQ

=== Sending Chat Message ===
Message ID: TzlEb9fwBZbI0LQsbioXydgc4rnHLGJ6aDJL7QuNNDyyZG5qFYG2f-pxox9selsoAiAwFx0H8SBg9OFJBd2rdg

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price of this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price of this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'FROsWzTi6-pJNkNrVdfost-4K9T94v89rLVKKKmSIDwxMJQY4erEP_xawa_nZ72yC6hBcxwhSEmSI2S6aN9XoQ',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'DXxEOb6pg_spGtknHsVKFn8v_K1FGtqh-cmYKXOWOh5HtSiYOMZM_UPl2XpwhpGeOwDfzau_S-8XfWXLd7w0MA',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    id: '9AaNz7LQSUrfq4XQZmD_bHrdGHIe2XC4Te9R6rTHlt7y2h77RCLjMXcqS1s-bN1Wk5jSdt7otzqVKJX1jqZ0ng',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3648ms

=== Token Usage ===
Prompt tokens: 452
Completion tokens: 84
Total tokens: 536

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: I-fREsRBetcgvFwrqM0fD5km0dq9OhO_ucVn1JRWFon32cd3m89zNbMag_-FJi_uXKum3ZO15VpC4yvs83bw9g

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product available is ekoDB. It is a high-performance database product.

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
✓ Created second session: PrDZapYRBDXb0krYAwkAKnXfZUOSgqcgU7tqRVf9dNiJcoo_7VaT41GdHKL3je6SgvKQEitPVy93IY0fiY03cg
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
✓ Created session: P_aaJN68XhRIbhqfgQ_jSrjyRJVoyUUykicgdR0eXORz9SYRtDEcPdyMkgNNqEGMPJ6Z-2-jUFnlI589d2aHmQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is "ekoDB". It is a high-performance database product that costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: DoE3FlKKRhpQGsW4Z5qjIvZesfVMEU3WrZL-gsAsBQOq8QW7rsAXU590xk_vR-bI6NIycPAwZSmgduuh2qUTTQ
  Parent: P_aaJN68XhRIbhqfgQ_jSrjyRJVoyUUykicgdR0eXORz9SYRtDEcPdyMkgNNqEGMPJ6Z-2-jUFnlI589d2aHmQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: DoE3FlKKRhpQGsW4Z5qjIvZesfVMEU3WrZL-gsAsBQOq8QW7rsAXU590xk_vR-bI6NIycPAwZSmgduuh2qUTTQ (Untitled)
  Session 2: P_aaJN68XhRIbhqfgQ_jSrjyRJVoyUUykicgdR0eXORz9SYRtDEcPdyMkgNNqEGMPJ6Z-2-jUFnlI589d2aHmQ (Untitled)
  Session 3: PrDZapYRBDXb0krYAwkAKnXfZUOSgqcgU7tqRVf9dNiJcoo_7VaT41GdHKL3je6SgvKQEitPVy93IY0fiY03cg (Untitled)
  Session 4: 2WR0InFC-qeXX9VK4maTnsaT0VQ9MZMJjG-GYITX1ZZy6xBtyDrI785FLoDXPUOMUL872PUCs5UDnbBZ0Z3uNQ (Untitled)
  Session 5: GWN7n_LCJc5sI8y74tHllA5-oBLAoo9mdo1YYk47xcv89lSXIaEmVmie9ZzHja53uGONrfdAsiQZICuwn5wOhw (Untitled)
  Session 6: _XwItNsgyui8eFwQdJ6vlZ3UIvBQDyuKd_-KbvxG5BgvgtF-S9jyqIDFa40sGXoYzxX4EjtHa9ev55NghF-r5g (Untitled)
  Session 7: 5HlYZzepTIGvK8nKfiHDDimKGkZe_gVOvDe1xEdj-yE_eDBZBijc4ne1Q4SstWoYaWF52WMpcUodRD87Ef9tLw (Untitled)
  Session 8: 6AhoM-pLbXz0EkVBNngDDFBYloDHpeJWLOckv2klHQDr0Vo2qoDXVPNFCm0hoI-6JepnpIg60RTqwWMYpwc9dg (Untitled)
  Session 9: vBYgCKA7tCIFVCp4q8I18qr8arv5t1VHkvH8epnh6z8K-qtc4ACIAQ8D9tijYnjG5uegaQ52hu-xo1IcjJHWTw (Untitled)
  Session 10: VofpQp9wru7iR1LwyfXa3iLZcXzr9oQBFOB34zHu_xa9TrylGOYBgMtqMcdLtGSAcC0VP6elo5_67K8BE79w7g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: DoE3FlKKRhpQGsW4Z5qjIvZesfVMEU3WrZL-gsAsBQOq8QW7rsAXU590xk_vR-bI6NIycPAwZSmgduuh2qUTTQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'q6zsZtGwsq380FfHAwWBgfhqlMj-QRnn9yAVyGoK15oBKTs62_j0nOIUVlZyEc_3_NWT_pEbvEPpCMLk7ZB9lQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  id: 'q6zsZtGwsq380FfHAwWBgfhqlMj-QRnn9yAVyGoK15oBKTs62_j0nOIUVlZyEc_3_NWT_pEbvEPpCMLk7ZB9lQ',
  email: { value: 'alice.j@newdomain.com', type: 'String' }
}
✓ Second upsert (insert): {
  id: 'YhRPaHyCxltsBaJZGZLHibiuU7UOto1QhpAw8mOkFEX7lH4R8GieLotqtRZ65pekcUqeSme5XJI6kv057h66DA'
}

=== Find One Operation ===
✓ Found user by email: {
  age: { value: 29, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'q6zsZtGwsq380FfHAwWBgfhqlMj-QRnn9yAVyGoK15oBKTs62_j0nOIUVlZyEc_3_NWT_pEbvEPpCMLk7ZB9lQ',
  name: { type: 'String', value: 'Alice Johnson' },
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
   Inserted with ripple: {"id":"K14_E3hH7fKLRv3C5AOGt3K91G99mKmkSjuvo_ig0q3Ehu8FjT4NKJHlkvwmQlvXtuKma0nlzldqbVZ_ba5_TA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"_8yPyiPsSoIG6ZTO35xW37ryDPsLsDM2T1SPJc1NKWJq1wFaQO1R812kI45NYElQjw16fxN2F-AiuN1uePEF0Q"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"id":"K14_E3hH7fKLRv3C5AOGt3K91G99mKmkSjuvo_ig0q3Ehu8FjT4NKJHlkvwmQlvXtuKma0nlzldqbVZ_ba5_TA","name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"TQXZ-J7jpySwdKOP53kG5uzVXWcVBnB-GjOC21kp3MeRZ6eOu-D9isGXY4DTNKckLXgska7MwPTGOcfBltJfhA"}

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
✅ [32mTypeScript client examples complete![0m
