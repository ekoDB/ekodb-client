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

added 1 package, removed 1 package, and audited 13 packages in 849ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'qLesd_k_z1VhCq7Fq3ttcTJu7Yy8mT0jQCZD1kWaq0zov9tW2Sdhlf49oolE7OKuYgdaFg-jhIMUT5e55lFjIw'
}

=== Find by ID ===
Found: {
  metadata: { key: 'value', nested: { deep: true } },
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'qLesd_k_z1VhCq7Fq3ttcTJu7Yy8mT0jQCZD1kWaq0zov9tW2Sdhlf49oolE7OKuYgdaFg-jhIMUT5e55lFjIw',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  price: 99.99,
  name: 'Test Record',
  value: 42,
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  created_at: '2026-01-23T17:28:26.655Z',
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-23T17:28:26.655Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { key: 'value', nested: { deep: true } },
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'qLesd_k_z1VhCq7Fq3ttcTJu7Yy8mT0jQCZD1kWaq0zov9tW2Sdhlf49oolE7OKuYgdaFg-jhIMUT5e55lFjIw',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  price: 99.99,
  name: 'Test Record',
  value: 42,
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  created_at: '2026-01-23T17:28:26.655Z',
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  price: { value: 99.99, type: 'Float' },
  active: { value: true, type: 'Boolean' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  created_at: { type: 'DateTime', value: '2026-01-23T17:28:26.655Z' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  id: 'qLesd_k_z1VhCq7Fq3ttcTJu7Yy8mT0jQCZD1kWaq0zov9tW2Sdhlf49oolE7OKuYgdaFg-jhIMUT5e55lFjIw'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: wxlj5ADdj183wgWjK3sdeW2AiUIXy7GDfaSfw89xtzWA6KxJHqi-dABZVUTzlZpv2W3Ga_ut1nb-HI2HD5VIwg

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
Collection created with first record: mJ5pkgrPPzdPWnqAwIJ4tj5hurs_yfK2mC0wbtO2BipEcW1W-1oXKuXhBuGt4l1Yt_g71u3iPZ08gS1DxGPIGA

=== List Collections ===
Total collections: 21
Sample collections: schema_documents_client_go,schema_products_client_py,schema_users_client_go,chat_messages__ek0_testing,github_cache

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
  cache:product:1: { value: { price: 29.99, name: 'Product 1' } }
  cache:product:2: { value: { price: 39.99, name: 'Product 2' } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

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
Created Alice: $1000 - ID: GtGgIEXnlL0Kpxp4fYVQ-F2pWxioLCyaf8VTETuOTulMEvCuU-mWUPeSQUWWKK4EOJmfvsQLDWwvt2wVebA-FQ
Created Bob: $500 - ID: 76lIXOSsM9ros-jG25WpH5QOgrWQVl-c0P8iLS4ncsXY46KwBHwfSjWVGwGnaw9JTD0rTNH952_Wy6BMevEePA

=== Example 1: Begin Transaction ===
Transaction ID: 6dfd2d87-a85b-4745-8113-325faed49e6e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 36357267-f0ce-4ed6-b870-b8716d472a53
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
  1. Score: 25.740, Matched: email, name, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title, bio.value, bio, title.value
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

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
  2. Score: 0.746, Matched: 
  3. Score: 0.725, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.708, Matched: content.value, title.value, title, content
  2. Score: 1.498, Matched: title.value, content, title, content.value
  3. Score: 0.290, Matched: 

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
✓ Inserted document: eKFgihhIUoSZ9cEhqb8QeG3t-tqqFd6QuTUyb9YHKSBCzLbeuJDd60VoTFyAUVrvsnDd7rVfHNNYNbbl9nPb6g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: rNMasSX0wzjRkgKv_tvOVVbvLe7rECdh_jIBIlkTJrP-Fcj9QC5UWX-HZeS3-HO3S2lwegFgRlsgrht61lillA

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
✓ Inserted document with TTL: pW7ezbVYUSY2tx00VsBji8jKVy4-0rkFdSM745ssG-sFuUmy1-8rMT3zFRJ2QOw5TGn4dMU305FyTTDhgtPavA

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
✓ Edge cache script created: kLWiXEjuFhHJOyncprTu9vC3eMFF1HXlsKnRgB_apiqh40_YYKawnC1_3b-RhnHM5cqcAqz8Wp55JjgWCzQofA

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

✅ Script saved: zCcj7Yh_u28JPKUBYyZBZd_f77jdwr-4Z9-p8msveWeX8cV8SsAVDEERhQYFI8_kUE-sjmXif7IEZBoC7VjrHw
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: izB1IlvuqNNpdIgQCEGyoPbpx10ednXa_CbKehPv_lpARheNpO54LD3Mirz6lgcp5ygvuh1HkP7rIzezG0fIWg
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: TbvJ6xJ9jwmj4dWYjcd9EV1gH5tYa-SqyhDb-RBdYP9FSWpiXxdSF0VGLReUogu30smkm0hm4KtbLu9uqtG4YA
📊 Statistics: 3 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"avg_score":60,"status":"active","count":5}
   {"avg_score":60,"count":5,"status":"null"}

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
   ⏱️  Duration: 111ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 66ms
   📊 Records: 1
   🚀 Cache speedup: 1.7x faster!

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

✅ Script saved: VTxNX8t3U0OGzhuPrXp2LY5jyWkaiYX9eQ3JzL8NVKPhH6CZcjQ3thZLJ98WwhaQ_ZqoqPefNe7jqxadjpoh2w
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
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

✅ Inserted order: f5rPeQZICCMzIuh0gMg7SDTfgG9gw76Odx5bZQ2S1wnH60mBnjjoWbaQ9tcd459dvIe0covkbIhFwqdLAzsmUw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: Ux4Ovv63FKgBkzV214qI8GausUCOYBuZErQNnDOXknHLHCWd7DELal23sSNQAJtmnEcg4Zjd76dhbpTddJlS4w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: apCzuWMNt-vSq7soGoJKjh4ADWdi985vd1OKJ6baHnbC2SM86YEALHKfjOQo-hmG6eGkGinNDhwfpTBX3sgaLA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: weBWujmb4Bcqh6mJGn9Kx5nk7DWvwMYUSxEOB3IHwOoHJN27GOv4ocYi2foJ9KPFaYfnk4Vru8ab3MFzIE6myA
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
✓ Created SWR script: fetch_github_user (xz2Ryb-OTYJ5zuU3Dk10EjNgJMQDU4Q3qhZ-Vkk1sAXOzoPm_g7r7zvJ03H6BfhWS6tqvDTyxaOPIxt_PAU71w)

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
✓ Created enrichment script: fetch_product_with_reviews (qKZubPZ0_RasWjqh21EZD6X542q-U9sojg8M1uirege3GViuoWGyzacJVQmHYcUguKBh2vGRLbQzWIprhDqXHw)

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
   {"category":"Furniture","count":3,"avg_price":365.6666666666667}
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
   Vector databases offer benefits such as efficient storage, accurate representation of data, high quality graphics, and the ability to perform complex spatial analysis. They allow for easy modification and manipulation of data, and they preserve detailed geographic information.
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
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
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
✓ Created session: pOGOOxmPEbbRsJQtqdm6xQIKSSnyBzYzNv2ua9eCwXmKq3v8FBzOS04-JYqwjkZtfjTvWdttPNU8rzG222inrw

=== Sending Chat Message ===
Message ID: ew9ZyQ2M1AVQcem6Unf-25OVOaHmh0KIJD3dzTL14D22PZyIiKw3jcyvI7eKH35TgwmhHU06sbNIuYKllszDvw

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Cloud: This is a fully managed cloud database service product, and it is priced at $499.
2. ekoDB Pro: This is an enterprise edition product with advanced features, and it is priced at $299.
3. ekoDB: This is a high-performance database product with AI capabilities, and it is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'm7AORkjyNAi3lZZxu7TkW3nowUrpDzczScSffhssul5C3fyy6N7dH1bR5rld-0F7KhNknO7MqiTWLIHsAx8CxA',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'MF8qWQw_L1JXsP3H_PejnLqwIcpl02glJDs1R1oM3cKlBohfmjbk21EvoU7ByTEhpwz9Io2a06guAIIsLVdlCQ',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    price: 99,
    id: 'u3GqLVum5hAdEWq78pXa_mgiJXZU0XG2-sY48JJaYXSTErvAgoW8P4BFSGJpShggU7DKn2trEKGY-WWw9aPyhw',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2297ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 85
Total tokens: 539

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ktqC05OF4cf6AtcyXHkFgjdF6QDeiV-gPhXCX22I8qK-TZOqOB4elWQqc_CGQm3v95nDec8ZSo7wpceqtNhOpA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: 99Pn4ItmLj9yNXdJ9MHXaR_S6DJOS752ZPS7m0yTb1fLPcBFhVyB3y20uMRIHqLljr2_IjUyTEO3fa5MBjqN5w
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
✓ Created session: N7x7pp55YXw3bqdV-U7D_qqytgmhA6oUr6eEbffzzFO5Fpem9r1yPKwYwlDbOSWNFwpwNJ6GxPfwL_VjF7Y-7A

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is "ekoDB". It's a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: sZYkMaEjhIGpW3bngpaLj13oVv8s6D4hCducfEDhglLfhL7vK3fWAgwpLolyIqRzSJ6HZT7LPDo4XFrpMW2rxw
  Parent: N7x7pp55YXw3bqdV-U7D_qqytgmhA6oUr6eEbffzzFO5Fpem9r1yPKwYwlDbOSWNFwpwNJ6GxPfwL_VjF7Y-7A

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: sZYkMaEjhIGpW3bngpaLj13oVv8s6D4hCducfEDhglLfhL7vK3fWAgwpLolyIqRzSJ6HZT7LPDo4XFrpMW2rxw (Untitled)
  Session 2: N7x7pp55YXw3bqdV-U7D_qqytgmhA6oUr6eEbffzzFO5Fpem9r1yPKwYwlDbOSWNFwpwNJ6GxPfwL_VjF7Y-7A (Untitled)
  Session 3: 99Pn4ItmLj9yNXdJ9MHXaR_S6DJOS752ZPS7m0yTb1fLPcBFhVyB3y20uMRIHqLljr2_IjUyTEO3fa5MBjqN5w (Untitled)
  Session 4: pOGOOxmPEbbRsJQtqdm6xQIKSSnyBzYzNv2ua9eCwXmKq3v8FBzOS04-JYqwjkZtfjTvWdttPNU8rzG222inrw (Untitled)
  Session 5: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ (Untitled)
  Session 6: 8djIhno3iGEPxBoj-qxsGY1Zpx_yFB-AEO3e3JulWsFnGJE73TCbQ-BN95kEtrpkrz4vmqG1UpPQSqVXz7qukA (Untitled)
  Session 7: GFi9nITIlmmDiWZfKTsqe6vfG12LC7fx6h3DUO-A8GnXij598gQSbfMBnLLFF371mGI3CoYy-ndoWqrvEIJSvA (Untitled)
  Session 8: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg (Untitled)
  Session 9: zOR29wOGIutIB8YS2Abnb1ahQ69bxObd1qN0vAehTVcUeLY3FKlJVFqey_muiTzCTeh6hkk8hdQVGYFPMuvPoA (Untitled)
  Session 10: deBMeaW6ZyOXZgSVS4I7CjcdHEUe5mNs3ujERxw6PbgqG52yubBVJplrqnvQgRwQHBfnGxO5nWYgAy5o7IQrMw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: sZYkMaEjhIGpW3bngpaLj13oVv8s6D4hCducfEDhglLfhL7vK3fWAgwpLolyIqRzSJ6HZT7LPDo4XFrpMW2rxw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'vWyyTVb8qswLl22JQDFynl_o_yqExzqe-SrNWDLJKhpZt1kBEbXLaPFy3L8rogYgDgdzPv5LAsC0lxRxnPJ0bg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'vWyyTVb8qswLl22JQDFynl_o_yqExzqe-SrNWDLJKhpZt1kBEbXLaPFy3L8rogYgDgdzPv5LAsC0lxRxnPJ0bg',
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 }
}
✓ Second upsert (insert): {
  id: 'mqcvGgoVi4xJHwMxjlWSKj_QWcunLF4i4lBVJtCwpxsKW0z-g6coLlD6eq_rT4iEsZUmAfzUFEmPnuD_b_xEtw'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
  id: 'vWyyTVb8qswLl22JQDFynl_o_yqExzqe-SrNWDLJKhpZt1kBEbXLaPFy3L8rogYgDgdzPv5LAsC0lxRxnPJ0bg'
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
   Inserted with ripple: {"id":"WmzOIXqMs1kPwttTT5b7TNixVrKPsl0qQNKWzuBu4lriSS9hH_6pesaabqNYNCMVE_5S2TuntLYcg4j0CXcgJA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"XdhS5gWK0GOMlqC8uaTjW-h2ctaIm1HB3IFSxdQVu9pw4904IZmXSlklugz5aW5-qzV8lXTsLEP0cHw0kF53AA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"value":"Product 1","type":"String"},"id":"WmzOIXqMs1kPwttTT5b7TNixVrKPsl0qQNKWzuBu4lriSS9hH_6pesaabqNYNCMVE_5S2TuntLYcg4j0CXcgJA","price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"BMLy6fJByX4wbIaqpmfs-zC8V6KO1-PXIFz-ZxSVrTGz1aglzxvK7PfJAb2MNQyR8xliC1m5RAeZO7liwZ7Ptw"}

✅ All bypass_ripple operations completed successfully!
✅ [32mTypeScript client examples complete![0m
