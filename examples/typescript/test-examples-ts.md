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

added 1 package, removed 1 package, and audited 13 packages in 872ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '1vJlsWrivWIStxd8xzx-LEzomHQHYhX4-eT5SNgsbZXV8gdR_csEZaGWQfTkNs2C_Hu6EEPIWUea-tO6x1GIvQ'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  price: { value: 99.99, type: 'Float' },
  value: { value: 42, type: 'Integer' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { value: '2026-01-08T02:41:32.317Z', type: 'DateTime' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  name: { type: 'String', value: 'Test Record' },
  id: '1vJlsWrivWIStxd8xzx-LEzomHQHYhX4-eT5SNgsbZXV8gdR_csEZaGWQfTkNs2C_Hu6EEPIWUea-tO6x1GIvQ',
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-08T02:41:32.317Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  price: 99.99,
  value: 42,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ=',
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-01-08T02:41:32.317Z',
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  id: '1vJlsWrivWIStxd8xzx-LEzomHQHYhX4-eT5SNgsbZXV8gdR_csEZaGWQfTkNs2C_Hu6EEPIWUea-tO6x1GIvQ',
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { value: true, type: 'Boolean' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  id: '1vJlsWrivWIStxd8xzx-LEzomHQHYhX4-eT5SNgsbZXV8gdR_csEZaGWQfTkNs2C_Hu6EEPIWUea-tO6x1GIvQ',
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  name: { value: 'Updated Record', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { value: '2026-01-08T02:41:32.317Z', type: 'DateTime' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: BBj7NBquiHWd0gA4ooHRPhnZNSLfsLjtJMAAhxRyEz_kZUjjXJcspEWiLxEZFkuC-NwH0NzreQsQZE2z829eOw

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
Collection created with first record: 3SpSxX7EkejJZe47N2HTVGwRc4i9WfUqjUa3HUltPMDgTfujoHXDJdsGLlhWk_muIWMNeQ_DAG7tXnYaga1kog

=== List Collections ===
Total collections: 21
Sample collections: schema_users_client_go,github_cache,websocket_test,test_accounts,schema_products_client_go

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
Created Alice: $1000 - ID: DKk8CckXapbk-AxwMr7IfeP4YRuWomPUZMP_LKs-W8UOchXKY8wV3u_E2fHGGsWFyGxajZNIaQv2fknb2x1Tmg
Created Bob: $500 - ID: NPR9I-KuLmAkvCqG3k59X37P4S1nQdAXUsyG_K87kwlxUmM-icowYjoDJQ7TswZPIHec2M_HoqzS1hjOArEdqg

=== Example 1: Begin Transaction ===
Transaction ID: b8587b5b-54f1-41e2-88ee-979e4bde372e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f55916c2-f7dc-4587-986b-93f841e5e2c8
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
  1. Score: 26.400, Matched: bio.value, title.value, bio, title
  2. Score: 26.400, Matched: bio, bio.value, title, title.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title.value, bio, title, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: title, bio.value, title.value, bio
  4. Score: 39.600, Matched: title, bio.value, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.773, Matched: 
  2. Score: 0.727, Matched: 
  3. Score: 0.721, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.709, Matched: title.value, content.value, content, title
  2. Score: 1.488, Matched: content, content.value, title.value, title
  3. Score: 0.291, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, skills, bio.value, bio

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
✓ Inserted document: 59Rj6t52nwc4CyLAJrQAxrvsytxNGejAEzusn3WcYx_Ufxw9SpcoHiTeEacjyFkUwnp-OMZ5UL0bx_KmMBeYxw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: _5-YnfTIZW7IdH2TqqkjpuhVIYk9jH0IVkPUHAWy-sTENGbr4g5vDWPcxqgi4y27-7JNELXxptdpP0TMQz1hlA

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
✓ Inserted document with TTL: KoigzFFZrnSBvtp2-brKpqVoAeZZD55CCQqtoY5d5Q9jONqyWNYkPRm6VAAkV74A--3e6s5BVR7aWZ0VjuUXPg

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
✓ Edge cache script created: FKnytWaFy2vhrFsSGj-NOygvnknLskUzS4ugOY1ffpO55JsZMxwmwEpuUvCq_xP7axgTjWk9OMhz6_vMLSJ69Q

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
Response time: 3ms (1.3x faster!)
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

✅ Script saved: -PPk2UgGZhDVBZRibSQhk4dWTqeyockvuFyd9GHhbEvsMpw5Oazrx35D1UXxU5YoDKjk4bhDW-Xfad_dWUvVbw
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: HLk2J9f9fZGKqJ76GSfCrr8L5_1lgF38bunESEq-4Zv73nBQyCtaF3nCEdt0kfNXsEWP8wqu3LV4MA4R0J3S6Q
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: xYfGaWM95MjRoNuL7IeIqblh63xN4cKL5upkMDvxN9_3etyUSkmMwfZSd_09VR1TKCUBT8thQpkc79nMe93voA
📊 Statistics: 3 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"avg_score":60,"count":5,"status":"active"}
   {"avg_score":60,"status":"null","count":5}

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
   ⏱️  Duration: 133ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 81ms
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

✅ Script saved: ENxYSWIl-2W1Uemw_gq9RyPikyRnIfUUaqsME7uiHw1qpSg5CsDZgLcjVyNTPtmjT_fIYytTRrEpuGDPeYfnmw
📊 Found 2 product groups
   {"count":2,"avg_price":474,"category":"Furniture"}
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
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

✅ Inserted order: VkF-UkO8cvu_Qv74GcwsFU2AZhe-ZRpkPIRuc-XISDk0B6H_As12qZSdECoLNKfmF-tl600zPmV5Sct7wLzBMg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 3g5bf33qLRiibZN_PtJEk7-7_tItvEF8HqQxV5k_HoigvQB4YIu18n6vui9AtZW0UWDeRzEwrHuZQ5Q1iPxZOg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: s4VM-yo4C8OxTVQD5FNmlZRIGhtLuQr9YLwDNaHyIaf7ntp286FCCGvc2wAyn1w6c3dF_QSOvMAPsXia4rIZ0w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: eMVsxaxTQuQnYBvvJ0OwGh_HPjHOIv0b0XE2I23HKKxeQ64Xz55GCoUrNlel0po5RRn9lS0OlWoZoCTfh4Fgww
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
✓ Created SWR script: fetch_github_user (ed8JKCKfPoqbVpxS1xxUSj2n-SUfaSseQniB8EtsNQXcMT40bB17giyp_NjfTcc6U-Jh2IRGby6iUKqbsRDF7Q)

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
✓ Created enrichment script: fetch_product_with_reviews (dIV90t_jyMF_AwUlipG8yJfRqq5oBfflcW728gsvpc8oKxZxDXHy3TcG8w1se0gFxsIYyMDC7Ksk8DkXHR2N9w)

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
   Vector databases offer several benefits including high accuracy in storing spatial data, efficient representation of data like lines and polygons, ability to store topology information, and capability for advanced spatial analysis. They also allow for easy modification and manipulation of data, and precise geographic positioning.
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
   {"role":"user","avg_score":70,"count":7}
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
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Database Design Principles (Database)
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
✓ Created session: lz2iSHAk36CpdWaoAO7vTyjyhl-FfwkFjQZFzMQkZBi-zOZMQ4h7d9XOmO13VdPhlcHPs5GaedO4RiS3Ddsc3w

=== Sending Chat Message ===
Message ID: rHTmgf5LFqPwL16CVUr0tU0RrCW6YXBR6CiW-NvT5KPI2CNU-lq5Yrd1tCkUxJHDunDQSf5W-bHFQDzSRK-jvA

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: 'kOBroG1EZQocXax53gJmEIud6cnu67wHqvzdqr9aiKhh3lzcH6_qxKyzHgRyToKWTSxkRnvKM9VLVsQUG8E29w',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'stcaDE_Umu6X-YB7CVuTqkPCTsJzobQKorsNF9hXYZpqp4xqkpfrLouT4VPV13iyOYYEfZBqMJ92Jmm34w6Z8g'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: '7WUOugWnRXuoSGSpv2zqTYab2zhngtaReVScy5ROhP-FgjkIR9FBoEknfMxCrBASCQZKRgjr7J7M9PW09HA7pg',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3787ms

=== Token Usage ===
Prompt tokens: 443
Completion tokens: 79
Total tokens: 522

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: UfNrFkKgamzTOi3dIf0Ljq8UUTDqiWyA4b_E4QPnAx_T4otU4sBFe3J-7tK9stWfeOA_2scTChBFWxHmiKjP_w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a high-performance database product available named "ekoDB". It is priced at $99.

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
✓ Created second session: yfnh7ZzS61Cp1Gl83RKHeLmZDgPVjEfr2kzSw87kAygTXYLPuOwk-HaYoB7mwiKnjOGxfVlHVdO7z6KHNDfTFg
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
✓ Created session: zxC62bVDmMB87FzrbkrzM5apjFZd63cSCh_3PMkA3ZOEeiPLpnSlTz5zFu-QhvDb6yoOFtzYPpP1XF2hReSFnw

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is "ekoDB", which is a high-performance database product. The price of ekoDB is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fvGmZJNG67mEW58BGUm8jXLzJoijmmN6T2x--rQA_3nWvQ5YGqOPTSlkfUsSxaHqFtgb90zFPe_hgQUP7MOXJw
  Parent: zxC62bVDmMB87FzrbkrzM5apjFZd63cSCh_3PMkA3ZOEeiPLpnSlTz5zFu-QhvDb6yoOFtzYPpP1XF2hReSFnw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fvGmZJNG67mEW58BGUm8jXLzJoijmmN6T2x--rQA_3nWvQ5YGqOPTSlkfUsSxaHqFtgb90zFPe_hgQUP7MOXJw (Untitled)
  Session 2: zxC62bVDmMB87FzrbkrzM5apjFZd63cSCh_3PMkA3ZOEeiPLpnSlTz5zFu-QhvDb6yoOFtzYPpP1XF2hReSFnw (Untitled)
  Session 3: yfnh7ZzS61Cp1Gl83RKHeLmZDgPVjEfr2kzSw87kAygTXYLPuOwk-HaYoB7mwiKnjOGxfVlHVdO7z6KHNDfTFg (Untitled)
  Session 4: lz2iSHAk36CpdWaoAO7vTyjyhl-FfwkFjQZFzMQkZBi-zOZMQ4h7d9XOmO13VdPhlcHPs5GaedO4RiS3Ddsc3w (Untitled)
  Session 5: 3cCe1l-PZtrwagRkwcVNerAu-2d3BsEGkHWi6LMbv9kmRQBh6hRQz0eP-ez6i_TDyDacU9Cumo9k1IP-m0MoTQ (Untitled)
  Session 6: nY1nketylBmANam_9ptNwEUzoV2FPMqnGhEmiVsJ1azyUsLeDn9rZqVTtiKpCHEz087myGQhlRW1vJrSj7Gd_g (Untitled)
  Session 7: JWwv8AH1Ej25LutJrccUBNhSF4_0ynyuRbITNEUozPddqCs-d6VKpvdxHRveT5YEtzMASZkt21X8AaEkrkfkHQ (Untitled)
  Session 8: DIGMAxv66exWItthxiQVC4SoPGLxkvXlUzFvHP20kylx7_FzOqzocFxd1lYW5DLF9u1HNjN0wz0M7g7gd6A7Yg (Untitled)
  Session 9: 57jBIahNPlAPPXE6bVy_VvwYy7ki92yCKM39SaZ_pcZ7IcxH-IWyi9f2UGzrJF2gEDNqLRqCP8q4hFV7rznaHw (Untitled)
  Session 10: 3NxXAggJhkJnSgTudToXG9vYml6UMoNakmstzAWOv8utXP8AT5SIXcBWeApEuFalEyudulfn0gvvqKudKjty-Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fvGmZJNG67mEW58BGUm8jXLzJoijmmN6T2x--rQA_3nWvQ5YGqOPTSlkfUsSxaHqFtgb90zFPe_hgQUP7MOXJw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'yvmX9ibtOaKAAFojBKIwmHw-644hGLTXdgqVaUOnXKXhh2EFznzplxfLMyvU3fkuWY6e2H41zp6V6gMn54AGKQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'yvmX9ibtOaKAAFojBKIwmHw-644hGLTXdgqVaUOnXKXhh2EFznzplxfLMyvU3fkuWY6e2H41zp6V6gMn54AGKQ',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { value: true, type: 'Boolean' }
}
✓ Second upsert (insert): {
  id: 'mEiclnEwahkZlDD9gQ-kO45LpTX5wWdN2f6kkSulRQlCrxBy0c5IcLQ3QPmrJlXpcvDyo8NaFoBs6wxHdIH0tQ'
}

=== Find One Operation ===
✓ Found user by email: {
  age: { type: 'Integer', value: 29 },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'yvmX9ibtOaKAAFojBKIwmHw-644hGLTXdgqVaUOnXKXhh2EFznzplxfLMyvU3fkuWY6e2H41zp6V6gMn54AGKQ',
  active: { value: true, type: 'Boolean' }
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
   Inserted with ripple: {"id":"0SB29hU3o5Pjk4DYQX8GUbEAiEooYDKn90Y6bATY-1lm2upSupp3-Z2FOOAD8KHTr2C0qOEde9LByuCOV8_NhA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"GVfHHTLbxNhULysh8Jo-tn96p1oKNFCWzMbo6KhyDT1CHKqRg82PHmoa0NRMAjGkfMak1DV50unXjMH0y4gjog"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"0SB29hU3o5Pjk4DYQX8GUbEAiEooYDKn90Y6bATY-1lm2upSupp3-Z2FOOAD8KHTr2C0qOEde9LByuCOV8_NhA","name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"NbnMipCD0TqtmP_xcrl2M7pl2C_YAVtt_A5RucEXlS-e4giUD64_OznL0XfjlxUfqKRYret-mO0HpZQECfW9JA"}

✅ All bypass_ripple operations completed successfully!
✅ [32mTypeScript client examples complete![0m
