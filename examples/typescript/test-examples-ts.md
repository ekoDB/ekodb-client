make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 850ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Fz-ivhDkn_ud2aIQFViWUGJO1yw7eoLI7T3gteJCrliUdcldkYmgdnTa1vsOug1U0rCady0ezEVA0G-Fb6IR1w'
}

=== Find by ID ===
Found: {
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: 'Fz-ivhDkn_ud2aIQFViWUGJO1yw7eoLI7T3gteJCrliUdcldkYmgdnTa1vsOug1U0rCady0ezEVA0G-Fb6IR1w',
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  active: { type: 'Boolean', value: true },
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  created_at: { type: 'DateTime', value: '2026-01-08T01:17:56.378Z' },
  price: { value: 99.99, type: 'Float' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-08T01:17:56.378Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: 'Fz-ivhDkn_ud2aIQFViWUGJO1yw7eoLI7T3gteJCrliUdcldkYmgdnTa1vsOug1U0rCady0ezEVA0G-Fb6IR1w',
  metadata: { key: 'value', nested: { deep: true } },
  active: true,
  value: 42,
  name: 'Test Record',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  created_at: '2026-01-08T01:17:56.378Z',
  price: 99.99,
  data: 'aGVsbG8gd29ybGQ=',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  categories: [ 'electronics', 'computers' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  name: { value: 'Updated Record', type: 'String' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  created_at: { type: 'DateTime', value: '2026-01-08T01:17:56.378Z' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  id: 'Fz-ivhDkn_ud2aIQFViWUGJO1yw7eoLI7T3gteJCrliUdcldkYmgdnTa1vsOug1U0rCady0ezEVA0G-Fb6IR1w',
  price: { type: 'Float', value: 99.99 },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: qtO-lrTr3khr5ltvVm5MqmneIv896oKMXv_Bsj_WERCd2CniZdyO7gpluUXE2YXc7U_DjaozJuwpR2mnC9j3nA

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
Collection created with first record: unpq-GtBPLSjnMU0dGTrOJkS8ky66ODpTW-FwelTjLL0k6hILudpUlicJqWLE4vwzJiMVNJYU3yWEyBQPbuwhQ

=== List Collections ===
Total collections: 21
Sample collections: websocket_test,chat_configurations__ek0_testing,test_accounts,test_collection,chat_messages__ek0_testing

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

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: N6p7_qRbRKEbfzOPeOV0zt2sEhzvfcwKjOOr7RJ-2P8t-_iIYEadT0h9mw-hZszXNkn8zDuVYfxGrCrDHeC8DA
Created Bob: $500 - ID: FZG1MydYth6W3RRnDZei_hzNU_CAbr2lkBkzP6BQPTm1AVhxWUlTi2xBa55659oiDWBQsl6u-eAzD8T9DXG2hQ

=== Example 1: Begin Transaction ===
Transaction ID: 5336ffc1-bacb-4062-abf3-3e0c6472fe5e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 551b707e-8748-4924-a119-f248ffaa5ec4
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
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: title.value, bio, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, title.value, bio
  2. Score: 39.600, Matched: title.value, bio, title, bio.value
  3. Score: 39.600, Matched: bio, title, title.value, bio.value
  4. Score: 39.600, Matched: title, bio, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.751, Matched: 
  2. Score: 0.740, Matched: 
  3. Score: 0.732, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700, Matched: title, content, content.value, title.value
  2. Score: 1.496, Matched: title.value, content, title, content.value
  3. Score: 0.293, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio, skills, bio.value

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
✓ Inserted document: GtmRl4hJgpKZLHnlxb2au7VosXB7waGea8X-9-RyCoOFG6ern7UGrItIu2WpxwgVXLffQregyauNRnjlZ6_K6g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: bF9SOxvg3PblXL4QRaOJzjH7JuONl6GK7bkUmoCOWXoObVPXfWUkGij3ig-UQDgtFQDSmWfya9_J3BFX_fwp1A

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
✓ Inserted document with TTL: qXpLyXJKggco_z2aR23z2pNnqrhTZiV3tZdWpGYcO1OfPnwn3LPF_DfeWyh6Qg9KghXZYjI1bQOANjLRjNN6gw

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
✓ Edge cache script created: 2Ed_eKa8EN3c2-lbU6HojY85QgxpI0oHRWv95ZfddQ9UGThCg-xAKW6NBiSwgZuQefMCL_KO-_UYrgcLUqCnlw

Call 1: Cache miss (fetches from API)
Response time: 5ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (1.7x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: AQZjbhpPxCXf0mQsavbhDHUutTFyrM1rl8e1XXOoV3q102cp3_7AIFM1S9uxQeAbGNPjX50i1DLQIW4JENrnGQ
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: UqJhkuYGKe1WmXA3WcrcC5Vli6r7aC1E8J0RQ8KxkivlUFyHrXDxhMiPT2crSXPpjuriXP3MIggTozILw2QYXw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: c2jm1Nmek3G8QpuKkXXshUE_qHfkAfVPpDgDaaalMGJyY8nWQp3WK3y3E0SpZeXmTNWHHdOHAzFaPCO1J61eKw
📊 Statistics: 3 groups
   {"avg_score":60,"count":5,"status":"null"}
   {"status":"inactive","avg_score":50,"count":5}
   {"count":5,"avg_score":60,"status":"active"}

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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 145ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 89ms
   📊 Records: 1
   🚀 Cache speedup: 1.6x faster!

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

✅ Script saved: pFDLTWmFTcHfqgC3A2iLrS67_qY5iZxTiWayMKw66O7YVPw6Zl2zkNIaQKHvCB5qVpFC1McBiqeZ9zdtR1WJ4Q
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"category":"Furniture","avg_price":474,"count":2}
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

✅ Inserted order: qVFRuTplvw1hOvZEMJziErefflq86QG8FgS5Pf8vuVmswl45_iRXcybftj-caEpqIt2sstG-xMXFBtFRLKXjfA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: OxWQcgUn_fEeX1yFM-6tD6qViWxsh6DN1aiCTff_-0pngiQspVR4e9ujNdnW0fStbnGwQRCjaBGsaUC1pi96ow
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: jrzLGayoFSu5HiK-5V1MVx7yt6ONzjXv2_iI8f7ziMRgbLgrOYTC9BQeEFyHCj392ElZmQ-RVUAC1YTJM1ECfQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: tR_543JgbfgXk05p8F70hNDXd9VwPaLciuzG1-u3jT6l8WplIFM16Z16mLhlLZ4mtDPek3uXQtszuGa8SBt3Pg
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
✓ Created SWR script: fetch_github_user (Hi2S9CrJHC8Ktb80FUn0w2drS7tXHfFWXpYa7aajG90vqIN_qFbeTPMxIZfPVBHvxajwM3ExrnMDbU1Ait7uWQ)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (6XR5bNV8Tj_61iAA3Ia4H_YThotOSaRyjQ30YVbG1YRR65CJPY0wZcPVn17LigutfrX8wL-1R4zYk9SjUdg7Gg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
   {"count":5,"avg_price":367,"category":"Electronics"}
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
   Vector databases offer several benefits such as high precision, scalability, and flexibility. They maintain the quality of data at any scale, allow complex geographic operations, and support a variety of data types. They also facilitate easy updates and modifications.
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
   {"count":3,"role":"admin","avg_score":20}
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
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":2,"category":"AI"}
   {"count":3,"category":"Database"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: OZ_T9jx1dAT8W36cWXYVS3t3jWv-dFVb9yE-5VMaU_Lc8pT3j8qiRUOgFqiQD7HZHCB--uMQ7mO0Pq3qkwO5rA

=== Sending Chat Message ===
Message ID: ZX9v6wGKHlUvNQv8pXFRmbsUEGyn2VgRRkf5BnD-VeJ40RVFiUM1vo539sVF9LVk_Y6x_POxCVDj_vbN2Bk3gA

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'D-SVzrWLsotjrp8pjsws9RZfDWjDLujsPw7PJeLZvin1NaaDWcYJNIjplVeOqVI52-Cf2_oXWjuJmYmY-YecgQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'Pla7uLIPdalpjzspEW7M3u3yLbr8Boq9AJZfCxqDDbJIjLjuCAwHQReobLmgRrNQJ5WCWRrxe90OttGQLo65Eg',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'Ao9Y2NNtMnIbrkSMNWtCYyoLPnctQcg9Xe564x5vYkNZTU5xwZnRqv5BorTek5nbR9AWn2HliAASNnNV61GMbg',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2292ms

=== Token Usage ===
Prompt tokens: 447
Completion tokens: 78
Total tokens: 525

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 90jhhFgK054clYXxQDplb3Q1SN8UiozsWh94pcQg66ULsDFdI6pdx2z_RpOcZFinceO787DzOeiKUaeC_C2IIg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one available product is ekoDB. It's a high-performance database product priced at $99.

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
✓ Created second session: OJ9Gi-vngn4KrARAppWTtZ7ti6k5KXV6-QKU4Ac8wIAMiVB4OvmEul0OPmrl80jf6Rb2M6A10R06MDIUuPdv7w
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
✓ Created session: kzV0MNU7tpfkIQcxs153-OpdN_qimgUG662sOfSOlDprVZd8I7AjzRXf-eiT386Jxxu1SDLwVtjUQjKx_hiqdA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 9cwkWRhnDnUWwMcravBbi-f-UDbVI_HJDiDV2K2M-KR7sg0DmvrdufO24_V_TqPcoTrcbLktcuO2soVZTR51UA
  Parent: kzV0MNU7tpfkIQcxs153-OpdN_qimgUG662sOfSOlDprVZd8I7AjzRXf-eiT386Jxxu1SDLwVtjUQjKx_hiqdA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 9cwkWRhnDnUWwMcravBbi-f-UDbVI_HJDiDV2K2M-KR7sg0DmvrdufO24_V_TqPcoTrcbLktcuO2soVZTR51UA (Untitled)
  Session 2: kzV0MNU7tpfkIQcxs153-OpdN_qimgUG662sOfSOlDprVZd8I7AjzRXf-eiT386Jxxu1SDLwVtjUQjKx_hiqdA (Untitled)
  Session 3: OJ9Gi-vngn4KrARAppWTtZ7ti6k5KXV6-QKU4Ac8wIAMiVB4OvmEul0OPmrl80jf6Rb2M6A10R06MDIUuPdv7w (Untitled)
  Session 4: OZ_T9jx1dAT8W36cWXYVS3t3jWv-dFVb9yE-5VMaU_Lc8pT3j8qiRUOgFqiQD7HZHCB--uMQ7mO0Pq3qkwO5rA (Untitled)
  Session 5: obmigcGQx2Qj83fWYWW49S8iF05biY_bfytJuESASiYJthxdjNJujJn5OuEjpD_Uqht3SV9bogTZN1Zu-HXTHg (Untitled)
  Session 6: l8mfhsis2iPfO0NlBWcX4WM_Pvyc0UaawYXDVAH4BZDg11Yd5hxY3iAx30wwnGhCxHUWxO6gdzuHI0g6scE3YQ (Untitled)
  Session 7: h6e74e5rz97_0LQS0bX5e6STHfC-5AUx81xH2YClP806irvsP_RuIbiX5-AwnTwRbNCOQqkRwx6A4-PeLBDrcg (Untitled)
  Session 8: aZdrcSeIjeyVGJsYPzGBaySnv7pp-AdcgCG4v4JHTyDaDani-qspk-Lg5xexSo6BRPtrryL-3UyGX1KowazhTg (Untitled)
  Session 9: Bg740pblowaCtxTkdGKFhaJLWzCwIMey-ah00qOlMj3rx5TlfQzPpITnoquy86ak3fOozB_lSks_gxC3cQQiVQ (Untitled)
  Session 10: oqMt7Ogs0ZHIcPwl2kJjGKykHiGfcrJ32htPbW4fiv8eN0ClrX1rcTKvGMK9f_I8n_GL_iZnOonVi7Vz15AGAg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 9cwkWRhnDnUWwMcravBbi-f-UDbVI_HJDiDV2K2M-KR7sg0DmvrdufO24_V_TqPcoTrcbLktcuO2soVZTR51UA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'GzKJz68IUOQwm3cF_1mntsCfZnJevt7UV5w1nqij6xjbQS4ZDTfiVnLVnhtj4coPvIxaUUt5Lveg4O1qojMnEQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'GzKJz68IUOQwm3cF_1mntsCfZnJevt7UV5w1nqij6xjbQS4ZDTfiVnLVnhtj4coPvIxaUUt5Lveg4O1qojMnEQ',
  active: { value: true, type: 'Boolean' }
}
✓ Second upsert (insert): {
  id: 'SYUqZ3QZiDie5eoPvZadX_qC8nsOrAahex5rx8kjDO6GJmqgh8RTtE0DnKCEfHq1XTkwgh9kehHZZkY6TB0ZUQ'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { value: 29, type: 'Integer' },
  name: { value: 'Alice Johnson', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'GzKJz68IUOQwm3cF_1mntsCfZnJevt7UV5w1nqij6xjbQS4ZDTfiVnLVnhtj4coPvIxaUUt5Lveg4O1qojMnEQ'
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
   Inserted with ripple: {"id":"qZTk3Fs8uTkubNUAFdpDMOGLedvzidfQoGTDWsmlD4FX7fDmAtV3q0Ynky8cyQp6k7qHifAweeLndPpxdvaY8g"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"nOwYtXhUy1hFBVVxof1MFvTR994fC4YCO1tX4Ul0SDg-TT7AOYm49TkxbFl1N0CIYaqI0PQmAucUN7kU_9gHyg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"qZTk3Fs8uTkubNUAFdpDMOGLedvzidfQoGTDWsmlD4FX7fDmAtV3q0Ynky8cyQp6k7qHifAweeLndPpxdvaY8g","name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"8fZ4fAbvxM2CX7Q0vHsfYa62RFPet0vOOQZ_8JavrAKo-Eh_TPOLeJTexINnrXbsE-qV8JalTZGckuonO4F1Cg"}

✅ All bypass_ripple operations completed successfully!
✅ [32mTypeScript client examples complete![0m
