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
  id: 'cbIo81oOOqQoDNLMT07SSXpGguX4s43AjYCyRXtrb7Er6LX1L1v3LJf846dCwi2fIj9CUA44dhr1sjayx5OYqw'
}

=== Find by ID ===
Found: {
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  value: { type: 'Integer', value: 42 },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  name: { type: 'String', value: 'Test Record' },
  id: 'cbIo81oOOqQoDNLMT07SSXpGguX4s43AjYCyRXtrb7Er6LX1L1v3LJf846dCwi2fIj9CUA44dhr1sjayx5OYqw',
  created_at: { type: 'DateTime', value: '2026-03-20T05:08:57.095Z' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-20T05:08:57.095Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  active: true,
  price: 99.99,
  data: 'aGVsbG8gd29ybGQ=',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  value: 42,
  metadata: { nested: { deep: true }, key: 'value' },
  name: 'Test Record',
  id: 'cbIo81oOOqQoDNLMT07SSXpGguX4s43AjYCyRXtrb7Er6LX1L1v3LJf846dCwi2fIj9CUA44dhr1sjayx5OYqw',
  created_at: '2026-03-20T05:08:57.095Z'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  value: { value: 100, type: 'Integer' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  id: 'cbIo81oOOqQoDNLMT07SSXpGguX4s43AjYCyRXtrb7Er6LX1L1v3LJf846dCwi2fIj9CUA44dhr1sjayx5OYqw',
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  price: { value: 99.99, type: 'Float' },
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-03-20T05:08:57.095Z' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: V1bedK-YCcH1hLyJfxzTTPLo07kq6GNSkhyIktrP45CKoB4FRjFg4BHqIG6Xr7QkHv54RYf-iLZx4dHmb_q72g

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
Collection created with first record: Dq1mYEw2FW9WK89I5g_voLCWuNlHuoMXDMRGkE6FAonYeztmbsPK2FtK2xPA0UYiSMyqPF0dYlTcCybIgChPuA

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_go,schema_employees_client_go,test_collection,batch_users,client_collection_management_ts

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
Created Alice: $1000 - ID: qgBjUw7ADEeuSpzKWjbGZleJYZFYNTJjX4uq3lMn6-m3Kk2wOphLKfWbTM_r8smKl7PqNoWRM2m3RbKHppQvTQ
Created Bob: $500 - ID: TYkAe1yuK8EL9zsqGNCgJO1dxZ9YdyxCT40T9YerzVSPnt1fKQ36VPz8pUyzDyr-1vO230PTcrN6dFHVHjowCw

=== Example 1: Begin Transaction ===
Transaction ID: fd12d566-4a47-4911-88b5-d54791b76d62

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3072cd69-4888-4c65-869a-0544618754c7
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
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, bio.value, title.value, title
  2. Score: 26.400, Matched: bio.value, title, title.value, bio
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: title, bio, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, bio.value, title
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.743, Matched: 
  2. Score: 0.741, Matched: 
  3. Score: 0.734, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.697, Matched: content.value, title, title.value, content
  2. Score: 1.494, Matched: content.value, title, title.value, content
  3. Score: 0.296, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, bio, skills

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
✓ Inserted document: mFB9i-tw1dMgLNNemzJzUivAmHHWh8AbAIaRvQHj9lwEhqMydzRpMibEdG0IIvEKsAnjQ8AaGRXAIipR5_EHYg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: dxKcg3TeJ3nWGOac9VnxAzkTREioE0X8BeeZ0YY0rq87TXsL7pq1S1X3uWO-J2bJ3gr_EJr7P6nw1vHusiEAtg

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
✓ Inserted document with TTL: QV6S7iYZivTy1hX5HbMJG1p0120C-S-GQwxJvyFT_ZpyH9vMA3q59joLeSW8vaJ-Kym4tl-ypmesirYlRcGXgA

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
✓ Edge cache script created: hbiwGq_aoc7nv2ht_kDR45ASMbaUarfceKGBiZbF-gg9U_2UQqjwyvc7rJgB-Idef9ZEw5E-PC-Jv1m4nOLHJQ

Call 1: Cache miss (fetches from API)
Response time: 74ms
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "username": "Bret",
        "id": 1,
        "name": "Leanne Graham",
        "address": {
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "street": "Kulas Light",
          "suite": "Apt. 556"
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
Response time: 4ms (18.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "username": "Bret",
        "id": 1,
        "name": "Leanne Graham",
        "address": {
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "street": "Kulas Light",
          "suite": "Apt. 556"
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

✅ Script saved: agntUnX3IvX5lgedyHMLVGpM0dxDcktLGSg-ZIViRGV6erkI5mJ1_NsSJbi0bV3rx0kF_bDj0A78wTO4NQ1cpw
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: q153U_l1mjwDPjcr8KJMwO0RMZqbfHzRWqdnkenHCBAxq6bh97dnjg9lOG7a_yhG5eI7eBQ8C-vTxk_L1g4cHg
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: ydTt9HYJD7kS2GPn8-s1t4w0iBUFPFqt-g70SqQu9HFeI3g9hJT0cKBtYYdQ1QsZxnTxlaHrJBvVk0Kj2Chzcg
📊 Statistics: 3 groups
   {"count":5,"status":"inactive","avg_score":50}
   {"status":"null","avg_score":60,"count":5}
   {"status":"active","avg_score":60,"count":5}

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
    "website": "hildegard.org",
    "email": "Sincere@april.biz",
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romagu...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "email": "Sincere@april.biz",
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romagu...
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

✅ Script saved: cZMeN2AA7DVzOS_JPdHf6dvAmu6fvaZJDkQGgMB_sEXBRX_zZHxkOYvaC5ARH7vPgbXMtes4sU_1xMMHVADL1w
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
   {"category":"Furniture","count":2,"avg_price":474}
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

✅ Inserted order: cPEF8l--M9b5GzR-de1z79p-eHO8HE76xRj-gOJd5iebry62Zs-6u7nEy8jRVFT7dVZSIuQjF_EglLl98VBi8A
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: r0NEamoJmCPWKQ0DXRoounuuelRwmTztsbpAVoydDZBWEk16FMSKuMlZSfwtOtf1loYYx8sA583NCJbEuQq2lA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: JRKRSR7WdRfwvb4F4l1LlQG19HGkVx03jvkjZHfrKBNrH5JxJZOq4QzZsgdxYqDoLEt_HqQ0ugIVu0OnIcbjIg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: IFjiWk4SL5wBseDpv6H7HMcGLIY8btAuuognG0KSNys-If3GnHuT0wlAI4h2j-xCpKActO9L1y9YLUfM9bt6eQ
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
✓ Created SWR script: fetch_api_user (VreiwC_unEUNaEf5pFX010EuvilpmNpmyeqlNjoWCaDtPBkbKBSwQw0-KYrUqaD4khRmpdjUW_Jx5HtYWWdUDw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "address": {
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
        },
        "email": "Sincere@april.biz",
        "website": "hildegard.org",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
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
        "phone": "1-770-736-8031 x56442",
        "address": {
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
        },
        "email": "Sincere@april.biz",
        "website": "hildegard.org",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
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
✓ Created enrichment script: fetch_product_with_reviews (h_04y8Ssmo8zn2Wx-dG5Q0Dg9LI5eeUlWOn_EJodqGhpALAGiBDsccsN4wVsWxqGL6XHO81ozgekLERAz_wULg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "availabilityStatus": "In Stock",
        "tags": [
          "beauty",
          "mascara"
        ],
        "minimumOrderQuantity": 48,
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "category": "beauty",
        "stock": 99,
        "title": "Essence Mascara Lash Princess",
        "discountPercentage": 10.48,
        "weight": 4,
        "price": 9.99,
        "rating": 2.56,
        "dimensions": {
          "height": 13.08,
          "width": 15.14,
          "depth": 22.99
        },
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "comment": "Would not recommend!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          },
          {
            "rating": 4,
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon",
            "comment": "Very satisfied!"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "comment": "Highly impressed!",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "meta": {
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687"
        },
        "warrantyInformation": "1 week warranty",
        "brand": "Essence",
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
✓ Created native SWR script: github_user_native (LYuM5ICMrvTOl9keahxVgcgde6icG6uOihtwxg5GHHa9JsYeUJicQRcqcE_LImKPaHBymIxSb_5ZdDN8z8xSOQ)

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
✓ Created SWR script with audit trail: product_swr_audit (33Fx0LMDCPV8DmnOy2sfd4paRbCPn91HcTm2LAUd3oq1DQcAp6FEn3TTyX9JyHZu9gsYAw-Gz6D-PfXFZs3Idg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (-KkOECAZneduwjYz9BqQQHuurFnIP3b5H_CLIVe3u6q4v3Pvin5fUNR4--28iuYyAiHl6F654Pdihup74EC7GQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (u-HD5KQ_IJ3eZpExra20bQV6w60nDG8q3rymL5IWEpzUES-yY-_GDcoa9_QcLgnUlMtUP_4Ts1RLQpxm5q2aCQ)
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
   {"category":"Furniture","count":3,"avg_price":365.6666666666667}
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
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They enable rapid similarity searches using vector embeddings, making it easier to find similar items.

2. **Handling High-Dimensional Data**: They excel in managing and querying high-dimensional data, which is common in AI and machine learning applications.

3. **Scalability**: Vector databases can efficiently scale to accommodate large datasets, essential for big data applications.

4. **Real-Time Processing**: They support real-time querying, allowing instant retrieval of relevant results.

5. **Advanced Query Capabilities**: Users can perform complex queries including nearest neighbor searches and range searches.

6. **Integration with ML Models**: They facilitate the deployment of machine learning models by storing vector representations of data.

7. **Multi-modal Support**: Vector databases can handle various data types, including text, images, and audio, making them versatile for different applications.

8. **Performance Optimization**: They utilize advanced indexing techniques to optimize query performance. 

These benefits make vector databases particularly valuable in fields like natural language processing, image retrieval, and recommendation systems.
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
   {"avg_score":70,"role":"user","count":7}
   {"role":"admin","count":3,"avg_score":20}
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
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
   5. Database Design Principles (Database)
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
✓ Created session: cJ7KFC3ReqCiB715pcWAmK8VGiiCAoL7HsEQBUB3lyq_E5ftp6sYb3PGz7-SsEqSQ2Xrmtbhgnsj7Mk_1w7fww

=== Sending Chat Message ===
Message ID: kgO4j2MR7A7NnJx1QH-LLf8vaLAR-49lftKk3POHSFUj82MVoEYq4EceM_tDaYS2Q8-Dtv7PMn4B7fbN-4CuVA

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

2. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

3. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'JKMDLBQwtk7pZrmngEutqIMHuA-V7X7WSy_J6kO67sdF-cHgqtG1JKHvi115LU9-KHwrAoGYqrhNL_b6EV6Q4g',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'ERzlricvs26FZMqank2SSlIJnhwHPUYV1BnuawQelsyfebZDwbIWO_WF4dp2FxRSpzndylrkRaefLeOmQxtfFw',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'TLLdWcy5K8P0t_Z1K7vbnIpXsUb9oVLfMyFM3FRoSi5V0RG1Dyykl1z2qb8a6sljfkw1TqTX7haNcfseStruXQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4185ms

=== Token Usage ===
Prompt tokens: 695
Completion tokens: 91
Total tokens: 786

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fsPx_u8AukqYl6e1te8EmSPaL1nYvpRElKPQ3oJBUVDhcIGTSgjwYERjNnM3iQwSzRzypQLT6h9QSIRVmhGC9A

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99 

If you would like more information or additional products, please let me know!

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
✓ Created second session: Uz7UFkiZf7lX6l7txVycldEete_4ijtSoDBLZLSBlVy_E5BJQd_e1q23v8DhWe7dhkyGRgP-3vA9tBojVcUeSA
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
✓ Created session: Z6iT8PdRI7325tHWcZRmjKEEbOSXeIoG34CJu5wHNPpHa8dpRY5ybLoC77_ZkpVgZULKKZZ6eg3j4tGTNwtK7Q

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
  - **Description**: A high-performance database product
  - **Price**: $99

If you need more information or additional products, please let me know!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: BxnZaQ6AUz9wYpUC4RSm8HY-d2wiU27KlKKxwjLVvopoQn2bq9yFJzt5IprQPBy60b3wV2leWS9kdWl0yBhx7g
  Parent: Z6iT8PdRI7325tHWcZRmjKEEbOSXeIoG34CJu5wHNPpHa8dpRY5ybLoC77_ZkpVgZULKKZZ6eg3j4tGTNwtK7Q

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: BxnZaQ6AUz9wYpUC4RSm8HY-d2wiU27KlKKxwjLVvopoQn2bq9yFJzt5IprQPBy60b3wV2leWS9kdWl0yBhx7g (Untitled)
  Session 2: Z6iT8PdRI7325tHWcZRmjKEEbOSXeIoG34CJu5wHNPpHa8dpRY5ybLoC77_ZkpVgZULKKZZ6eg3j4tGTNwtK7Q (Untitled)
  Session 3: Uz7UFkiZf7lX6l7txVycldEete_4ijtSoDBLZLSBlVy_E5BJQd_e1q23v8DhWe7dhkyGRgP-3vA9tBojVcUeSA (Untitled)
  Session 4: cJ7KFC3ReqCiB715pcWAmK8VGiiCAoL7HsEQBUB3lyq_E5ftp6sYb3PGz7-SsEqSQ2Xrmtbhgnsj7Mk_1w7fww (Untitled)
  Session 5: -5sh5e53RHkzuNWTMIzkIDupGIb2ADBO_bDvRcRqrnhJe10g_-L7XKkcFg2nbnllj3j0ZoKVE1PSQ5uwkOIx0Q (Untitled)
  Session 6: 7dAJgIqPXPqtMwr6bScgNj5of5zkTnymUSqX109dbdd8LjyBLlNym5b98ESExHEvFdD6dE081U2rDnPpdV3-NA (Untitled)
  Session 7: mEiOcs6Z4Ri0oBGSTbUwcnwZPY7MfDnxHYCfFunBxqZ48nxDLi1TXnVOOEzfv_VqgFWI_91PJjtoGLD_uDBlXQ (Untitled)
  Session 8: CZufl80QZKGTZhBenZZ6Ezg7fg6m-lA5FRE9kJQ5rJW2xLoGlMVu5IZradKN5uTB9F3lcK5L0jPPyxsbXCoiAA (Untitled)
  Session 9: y-VwU57ehbRGol8_x25jWtrFY5sPw-SKwyUyRdSi9dCqAYx6YJ1FFrr9v52U24SV0_iICPskaQrpx2Weoo9rfA (Untitled)
  Session 10: qU2ekewe9QpQb7Pyszr-Ne5pJ7TkIO1VRKIX87AuyQ1gjTIjCOAKT5wGgKyOtIRtn9QknVzujzMwSQN8k_zqPQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: BxnZaQ6AUz9wYpUC4RSm8HY-d2wiU27KlKKxwjLVvopoQn2bq9yFJzt5IprQPBy60b3wV2leWS9kdWl0yBhx7g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'MpR9sRcnQwlZ5kQSDh9OaFFwvtSnCHQBOWy6kbrb2PxJBbC5Kttp69CZJnPNGNJNhxD1VT6PjatCY77nwx9vZQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { value: 29, type: 'Integer' },
  id: 'MpR9sRcnQwlZ5kQSDh9OaFFwvtSnCHQBOWy6kbrb2PxJBbC5Kttp69CZJnPNGNJNhxD1VT6PjatCY77nwx9vZQ',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: '62V3rtfhhZCW6Ft6DEksQdWKrsl9Qr-Qid5wQHbKRgirBJdif9J28a6zvbVaKfwNymZAjipBXo2hWnK0iq4Itg'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { value: 29, type: 'Integer' },
  id: 'MpR9sRcnQwlZ5kQSDh9OaFFwvtSnCHQBOWy6kbrb2PxJBbC5Kttp69CZJnPNGNJNhxD1VT6PjatCY77nwx9vZQ',
  email: { value: 'alice.j@newdomain.com', type: 'String' }
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
   Inserted with ripple: {"id":"iZSy-JTbulEyeyCEd-a3qf77UGDq7kDMJExbp44jf0mdS66cWwvIhd2SIDZs8hLrCiVTntQT_VbcSNu1AUQ_dQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"RrQL_Byl8zGBAt1Kqb89a4PKK8X8tA04vLb7d9coLt7i9JaiBYX3nxGTk2JeNju9jZFzv9KvWSrH0GVSEdO_2A"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"iZSy-JTbulEyeyCEd-a3qf77UGDq7kDMJExbp44jf0mdS66cWwvIhd2SIDZs8hLrCiVTntQT_VbcSNu1AUQ_dQ","name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150}}

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
  Fields returned: ["id","email","name"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["status","user_role","email","avatar_url","created_at","name","id","bio","age"]

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
    - Fields: ["status","id","created_at","password","user_role","age","api_key","name","email","secret_token","avatar_url","bio"]
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
Created user function with ID: k3bDqzZl7577G7AKt94Q3eKtvZsQyGxaeEUioAdHDzLTJI9Eyzj3ynsgguWE3dHQzbOoW1sHdZN6Gyg77FWxsg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - cache_api_call: Cache External API Call
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_api_user: Fetch User with Cache
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - get_active_users_ts: Get Active Users
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call_py: Cache External API Call
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user

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
