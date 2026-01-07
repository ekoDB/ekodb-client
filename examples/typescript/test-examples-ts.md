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

added 1 package, removed 1 package, and audited 13 packages in 866ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'e1zAewlzY9V_nuXKAvkhjrDvfr-Uh5qIZHe_GU6m0QJaiVjUze3UL3AuhGfdInuF-o_hjFwUgShwE7Nl_qtcRQ'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { value: true, type: 'Boolean' },
  name: { value: 'Test Record', type: 'String' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  created_at: { type: 'DateTime', value: '2026-01-07T16:50:50.684Z' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: 'e1zAewlzY9V_nuXKAvkhjrDvfr-Uh5qIZHe_GU6m0QJaiVjUze3UL3AuhGfdInuF-o_hjFwUgShwE7Nl_qtcRQ',
  price: { value: 99.99, type: 'Float' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-07T16:50:50.684Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  value: 42,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  active: true,
  name: 'Test Record',
  metadata: { nested: { deep: true }, key: 'value' },
  created_at: '2026-01-07T16:50:50.684Z',
  data: 'aGVsbG8gd29ybGQ=',
  id: 'e1zAewlzY9V_nuXKAvkhjrDvfr-Uh5qIZHe_GU6m0QJaiVjUze3UL3AuhGfdInuF-o_hjFwUgShwE7Nl_qtcRQ',
  price: 99.99
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { type: 'Boolean', value: true },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  created_at: { value: '2026-01-07T16:50:50.684Z', type: 'DateTime' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: 'e1zAewlzY9V_nuXKAvkhjrDvfr-Uh5qIZHe_GU6m0QJaiVjUze3UL3AuhGfdInuF-o_hjFwUgShwE7Nl_qtcRQ',
  name: { type: 'String', value: 'Updated Record' },
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
✓ Inserted test record: xezN1xJAfXc57wu9xLBnqkk6lPa1N2L8-MshhkZl9m8mIleN8zYha3MK9ZLV9JJc8QMN1-Ln-rjKnVap5saSxw

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
Collection created with first record: zd4A0mqyHUWAox4JMfbUx4PMMCBdcoAT8R9RusK7BHBLdgcIEkHodOlGn3_sRay0FeOXwqfSQgWyttiESN_Wug

=== List Collections ===
Total collections: 21
Sample collections: schema_documents_client_py,test_collection,schema_employees_client_go,schema_users_client_py,github_cache

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
Created Alice: $1000 - ID: uFC6WzykRb09ZE0TAdsKvPLKvW1o9ORaAmH4Q_ZVLy8-SQrlgf0oOiAN1MVEJcsZAc3-NJbZJDjBkDyaQAgieA
Created Bob: $500 - ID: nodFUEqQzRF2tnPx5sBiBl6_DRMpawSHdCg2vwHXonXE_EkmRQ02bWoYRwI4yl-K4pEc5JF70Ynl7ZhrtaU1xQ

=== Example 1: Begin Transaction ===
Transaction ID: f86b0814-315d-47f7-9c52-8046b43ba8af

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: abfcd393-65fc-4e4a-8b7a-0bc4998f4a01
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
  2. Score: 26.400, Matched: title.value, bio, title, bio.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio.value, bio
  2. Score: 39.600, Matched: bio.value, title, bio, title.value
  3. Score: 39.600, Matched: bio, title, title.value, bio.value
  4. Score: 39.600, Matched: title.value, bio.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.781, Matched: 
  2. Score: 0.773, Matched: 
  3. Score: 0.771, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.709, Matched: content.value, title.value, title, content
  2. Score: 1.512, Matched: title.value, content.value, content, title
  3. Score: 0.308, Matched: 

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
✓ Inserted document: I4eKWH4-f8tn8tjlfjQeqpz6WByVBxdw4rf9o5IEOy-8pW8GT6B6ztPi_IjpAeBr1Qvhd8M-3RKZ0cUI7C3mPA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: jRj0jZycREbvrYosrVfTLfbuYESXIPUT65CCCUx1JlR89XpVU3gEG6GHwLhM3zGJswGRnCs_Wn7SdBhsVCCSqQ

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
✓ Inserted document with TTL: cUDQm2OSCjglxpE_ymgKpA4xIlFgdRP42oIgCK3X-CjGFGWrwwZIAwLBCSu3beJ5_9o6fEmYQdM17tPwbp9YTA

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
✓ Edge cache script created: gpSv39ch9FZXmPZ8y21LbN7-O4ArlZh7lv1IAPip1E_zaBUa7vjyn6Zfmb-Sh73zpk9TgIRnfEJObyg88EaZ3Q

Call 1: Cache miss (fetches from API)
Response time: 4ms
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
Response time: 4ms (1x faster!)
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

✅ Script saved: b1dBoGkpizYdzVrDeHUM2h73GEVHFNdnloyjhsqJ7Z1ukzi4dipkikwStJePpHnO38KqGtvUMsSDosQLuXBiDA
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: isM3z8bvMyE2aX4tra2NziAMxFR57xB4lngL0WmpQtCAPFSv84MtpugkcUQxNCXfw9D4tZ-Vbg2rmQriCSU_SA
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: bfvm0SAibWrDeh4QHHQb0o-xVUSE7l7RrNpubT9Ba5MifGVxpSSZJthM5YKe1A0swX9pM9d03Teu6dRtcmckbA
📊 Statistics: 3 groups
   {"count":5,"avg_score":60,"status":"null"}
   {"status":"inactive","count":5,"avg_score":50}
   {"status":"active","count":5,"avg_score":60}

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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 159ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 102ms
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

✅ Script saved: 4ZUwOlN71NMLYnRdHW5bE7Qth9taDuzz-ukYOUPgu6fRzh-0__r18pBdhJDNLx__jWK9BvNaHYGtdOtYuu7dDg
📊 Found 2 product groups
   {"count":2,"avg_price":474,"category":"Furniture"}
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
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

✅ Inserted order: pnwaN2mgSvlSnGOXLgCB17AFYyHMyFlsemynNc8wVp0De_KHU53bYDTxux7xDaIfne8TjZ2OAly03wxV3zbSjw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 3kEsx40-ajxHLub0bFLOW6uTsW5JXVP3DAY97i0FRKFLzh06m8Nswag-1kQiWODDFhe1ay3kGM5VB3dbfaEqVw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: rPbifeUHqFy4NWsHmBQphR44UpAVBPdmthfbPkfg903iU0IFmp8FfRy3y4ce7wbs8f8YeAVTT_0lFW60cStGYQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: c8qhHpN_aTu7hEDdNhc1tEyJmr2DWKbuLIj3_I39tt-aFdUGtlHbI4lo88FJgrvS2ZQguJygkRXAJJ8-WTDUVQ
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
✓ Created SWR script: fetch_github_user (zNIpL05yBTiYZLgBkpZc9ze2BKCyz9KnlAg7iKTOta2KvafCrXimDEjYpYQvyZQSuTglq2lTSk4Q7DO2wLnFOQ)

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
Response time: 4ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (UwrTcnL7X3oS5A9FImlKdb6Z8OjgNtb84Sfduj-9OcwLEKstiOxI6NsoBGCwO0qOSn9pmx08b2wLIj0I4birDw)

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
   Vector databases offer several benefits: They provide more detailed and accurate data representation, allow for easy updates and modifications, support complex analysis like networking, and enable seamless scaling and zooming without loss of detail.
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
   {"role":"admin","count":3,"avg_score":20}
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
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
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
✓ Created session: J0IrKtT3wll5y-FEb7_C_gH3g-k2RGJIOXkUqSd7qabCPA1Ss_qSU2K9dwIFGw22f0TVyiJ3eyd4-ZEsTZ8X-w

=== Sending Chat Message ===
Message ID: _TfVRLGkbqYPduuHrB8ehj3fHmag6XzO88x29d6NAxSLxwiGOeKmgSw0SqnjU8ueECfBqQyEuyrhbnsXFvYpxw

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.
2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.
3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'OhE8BdX3jvG95k1tQxO6atZPnO8Oln6jM1xJFPBgSwKOidTDpknsbGvuEMEqTLwzGY2udhFsovhboQi-IfxHYg',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 'D2B-PVI5qjKBNOHVUs9_ZXYoRw4Bn5Z-Ea2QYMgXv5EgIkDqhlQW5CJqv2W9vb3zoj2GZ_z5_a8H0XQyExHp6Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'miyk3bey30A5R6tB85xDcDA73alFYwJ90Y9pcm49e0GUF18lqV2e5ZL5CUS9vlaTASr02yoCQjRUZru9AoJybQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3120ms

=== Token Usage ===
Prompt tokens: 455
Completion tokens: 84
Total tokens: 539

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wxgo2sd4y0ZztxvPsDMFR30rp-BMKI2X9ddvXewg4hrs1q96Mhre773uBNkf17ardBgIZp8lydsoCH5Ng8PRtA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is a high-performance database product called "ekoDB". It is priced at $99.

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
✓ Created second session: t0Mua8_dEvpfI51B2-s0OGkS6eT0UxpoMtrdUqUIHf31FqbeBOcy6zMVuxqha9vyZSsTkjgIwqTH3vIiyvro_g
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
✓ Created session: QxYFWCDEy4jGOjgzQgOorxKovil9I4j8byyc7AL3vJnfd1RfZ8NUXLCYI5-WqPaXh1VbTvrZ_2pIndyADTgycw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". This is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: YLpYrvXj1zeT8Pr59OIH3pTKNiUAp8lZZ6vdlm1V29wvbI5JqbsqC7sLc5QLnCRRktJY7haT-RN80MXjjf8XDA
  Parent: QxYFWCDEy4jGOjgzQgOorxKovil9I4j8byyc7AL3vJnfd1RfZ8NUXLCYI5-WqPaXh1VbTvrZ_2pIndyADTgycw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: YLpYrvXj1zeT8Pr59OIH3pTKNiUAp8lZZ6vdlm1V29wvbI5JqbsqC7sLc5QLnCRRktJY7haT-RN80MXjjf8XDA (Untitled)
  Session 2: QxYFWCDEy4jGOjgzQgOorxKovil9I4j8byyc7AL3vJnfd1RfZ8NUXLCYI5-WqPaXh1VbTvrZ_2pIndyADTgycw (Untitled)
  Session 3: t0Mua8_dEvpfI51B2-s0OGkS6eT0UxpoMtrdUqUIHf31FqbeBOcy6zMVuxqha9vyZSsTkjgIwqTH3vIiyvro_g (Untitled)
  Session 4: J0IrKtT3wll5y-FEb7_C_gH3g-k2RGJIOXkUqSd7qabCPA1Ss_qSU2K9dwIFGw22f0TVyiJ3eyd4-ZEsTZ8X-w (Untitled)
  Session 5: QgNaTKew4FukyFQpKPf4CDYmQLZpij-ZvQK1smny-h9u8n2GkvD5sxOPWGL5qBoaw1gpgODHHc8M8etF0zyo-A (Untitled)
  Session 6: 4ro2nsz3attRd93ZIQmS8CMbFnxwZnLU0AiIXFCx3wI71DKItAu8iu19Pv_gWkDSAkbXv578OyaCRNZlyKJN0w (Untitled)
  Session 7: VpNXcfJoGqkGtMBJo6_-992FNbN4k1fkjof5BM0K4YnCRY-6eIkZOJG7iE2pXivA2enUbqp-3-B0fo0rUMp1-A (Untitled)
  Session 8: Tz0-kccR_F1ZJIXKHGuZUUh-OSQHxZvgNILU04xBnpRdus8Nv00sSZTJBvkAZREjP8N4x2Lw5aDIblS6G4eZ2A (Untitled)
  Session 9: 2z0WtTj1U5IXwDuns-OLvc06ilIGp6zB7yZaf7wad7Ru6Sw29Paelo2Gjm5rlKDSxNiWGZEGHkxIYyjb80OtkA (Untitled)
  Session 10: -xcLjFXFHh-R-Bgx66ye6uW9RTmOYwXWVrLNxRA_zWqoqCYc3HDdw43VqJuBFw7ls_RJHgAgn19-gp-OvXZT5g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: YLpYrvXj1zeT8Pr59OIH3pTKNiUAp8lZZ6vdlm1V29wvbI5JqbsqC7sLc5QLnCRRktJY7haT-RN80MXjjf8XDA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'Gowy0_02HmTUm2YvzgVRw234pnno5dxbCy3xvLv45lumqIDHtuFdxpEdROFmBGk2bMcsmfdi9HHHVFNXoCNv4A'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: 'Gowy0_02HmTUm2YvzgVRw234pnno5dxbCy3xvLv45lumqIDHtuFdxpEdROFmBGk2bMcsmfdi9HHHVFNXoCNv4A',
  age: { value: 29, type: 'Integer' }
}
✓ Second upsert (insert): {
  id: 'hjVFOB5G-cCjhNjifjv6GpBcZe0DOCP-SmnTfkMDVzxzn2W6qVEQQ57qmLK-4FpcSFyuDmnPDZABCDON73hLrQ'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { type: 'Boolean', value: true },
  id: 'Gowy0_02HmTUm2YvzgVRw234pnno5dxbCy3xvLv45lumqIDHtuFdxpEdROFmBGk2bMcsmfdi9HHHVFNXoCNv4A',
  name: { type: 'String', value: 'Alice Johnson' },
  age: { type: 'Integer', value: 29 },
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
   Inserted with ripple: {"id":"fb9eLe8TDBD1_9OQ4JkqgtkJCSmTDR8bOULIdwgPF-CFopJ6KZz1tt3WM2JcFgj2udLvOfmbLyFP4lYXmnU6uw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"FIFjnuHGRmqGngfryIuxjN99xuv4oK0cY3fGwCjCTpXY7-O4LxeqFx0Px_g_s0ZFlLVE96zrT0oH6ZkiH9Yyhg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"},"id":"fb9eLe8TDBD1_9OQ4JkqgtkJCSmTDR8bOULIdwgPF-CFopJ6KZz1tt3WM2JcFgj2udLvOfmbLyFP4lYXmnU6uw"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"SQM4E0BoHVD6exDvBq0JokP9oPb2ecf0AhWsEehsiFCXAmje3E1S5HzSW9YUvDi-STSC6WVYPmMCWU8RfD2kJw"}

✅ All bypass_ripple operations completed successfully!
✅ [32mTypeScript client examples complete![0m
