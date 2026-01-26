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

added 1 package, removed 1 package, and audited 13 packages in 823ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'WckLvHDOQyGa4b7tn4Kz8iSML-6_TPlH04k4F33Tc9blPuCyRz2-YnQelbaOGevORxzG46n8W9BIHpErzSxDkw'
}

=== Find by ID ===
Found: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  created_at: '2026-01-26T05:58:27.729Z',
  name: 'Test Record',
  value: 42,
  metadata: { key: 'value', nested: { deep: true } },
  id: 'WckLvHDOQyGa4b7tn4Kz8iSML-6_TPlH04k4F33Tc9blPuCyRz2-YnQelbaOGevORxzG46n8W9BIHpErzSxDkw',
  price: 99.99,
  categories: [ 'electronics', 'computers' ]
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T05:58:27.729Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  created_at: '2026-01-26T05:58:27.729Z',
  name: 'Test Record',
  value: 42,
  metadata: { key: 'value', nested: { deep: true } },
  id: 'WckLvHDOQyGa4b7tn4Kz8iSML-6_TPlH04k4F33Tc9blPuCyRz2-YnQelbaOGevORxzG46n8W9BIHpErzSxDkw',
  price: 99.99,
  categories: [ 'electronics', 'computers' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: 'WckLvHDOQyGa4b7tn4Kz8iSML-6_TPlH04k4F33Tc9blPuCyRz2-YnQelbaOGevORxzG46n8W9BIHpErzSxDkw',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  active: { value: true, type: 'Boolean' },
  created_at: { type: 'DateTime', value: '2026-01-26T05:58:27.729Z' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  price: { type: 'Float', value: 99.99 },
  value: { value: 100, type: 'Integer' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: fHl_mTxPFmbcRP-ayDGwL0PA5lTPvxzBa-d93R8X5NGBohwnzNtWy2swxdZnLgn1nE07aTVI63VWDmDWuloLHA

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
Collection created with first record: 9ir_VwtvPhy2a10aKc7sV5F7ifj5oOwNm4V2YjGtyxtvwNQaaa1gB6_4ojqDBInlZ7CAReOMU4VQKi-SC4zPZA

=== List Collections ===
Total collections: 21
Sample collections: chat_configurations__ek0_testing,chat_messages__ek0_testing,client_collection_management_ts,scripts__ek0_testing,schema_products_client_go

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
  cache:product:1: { value: { name: 'Product 1', price: 29.99 } }
  cache:product:2: { value: { name: 'Product 2', price: 39.99 } }
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
Created Alice: $1000 - ID: ZZ6A08NhQC86sq4dUVhzG-GZKNjzaJ7w8n37AgnGHf24NUWPZpiP6W8KbXeppqO60BfLV8lpYOxvAWmLS3Mi-A
Created Bob: $500 - ID: WG6qs8rTzbVVBwJYcMPgMFelxDSPrDNxOjhcxv8HxRXQR9LouF6mt6qhpDOrcjCYDEakCpId7YRBF5WXJhKA-w

=== Example 1: Begin Transaction ===
Transaction ID: 43b2fe63-1ac0-4bab-8e9b-9eb0416e94eb

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6c91efa3-dc5b-4bd9-8a10-4fa07caabc84
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
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title.value, bio.value, title
  2. Score: 26.400, Matched: title, bio.value, bio, title.value
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title.value, title, bio, bio.value
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.768, Matched: 
  2. Score: 0.764, Matched: 
  3. Score: 0.748, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706, Matched: title, title.value, content.value, content
  2. Score: 1.499, Matched: content.value, title, title.value, content
  3. Score: 0.307, Matched: 

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
✓ Inserted document: maNto13JmSTc1IQLWcyR9qSdogQV7VunN6KRGhxT5h67URfwiLk2s4P3rN_uxOvzdlB840C-zRx9_J6qF3EYKQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: RFKWKr634oN7rJAdHR-49Uk6f1s3F5jpcnbblBAgisn9_gghICZj0d7-idSXK26vTLGHO_-3bDycA0x10cAf2Q

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
✓ Inserted document with TTL: m3AioByz4SBjEzEI4L8XVJ6TKoc2lSo7TRFMWKrBJab675GIQZ05t_8Oeb5RFtOzjbIKr4rPUe_IpM1T3A_zEQ

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
✓ Edge cache script created: t8cVRhzELa-H-mXx_qFFYe_UHB0M_oGSHhrAGDNchTxfkWyU_MLQcltUQvNp5uNWDjItQXLhH6CnACayLuyOvA

Call 1: Cache miss (fetches from API)
Response time: 3ms
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
Response time: 4ms (0.8x faster!)
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

✅ Script saved: RpK02iRQXhcwX1lf1ra9Q0MOrrZCQpvJRbFdPZJXDVcNOxV636bELj9FDwswc7v_WUp_5RbRnjRSuKbyLkMu6w
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: DvGOVRyFPnDD5QSVWfXA6Qjnn_yM3HH8cKJUXHpw81U_EXRinkVrHzuwnfedcjo8-ojuzxwtTAsd0Q0r5yoj8Q
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: cOe3FoH2qIhIrUEnn8nlI1Xy6cf-bXKsNd7SufBdNsGHmsJ8wplw7FTNULrm9m3ry1TeGi_jbt8wzPTHaJztzQ
📊 Statistics: 3 groups
   {"avg_score":60,"count":5,"status":"null"}
   {"count":5,"avg_score":60,"status":"active"}
   {"count":5,"status":"inactive","avg_score":50}

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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 69ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 73ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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

✅ Script saved: K9eb4IoX35bqq3dngg4odrGx4STfYyMK5d8Yjf2s_MURtIwDwW5csrJm6hANJbVrF0XeFTmtmjOr-mPyutqbEA
📊 Found 2 product groups
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
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

✅ Inserted order: SflxRp20_a4nYFXktDmg04UU8U-k4-v-MPBzTHHeFOvd_5Svg8kG1rncrxr5rvzg7vH9BsTVv0W2xGI8bXL0jw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: FNGczFMPqV1nOUwUbMz706IdeGc-bGonX7VB-0_MqfI3wrcGml1r07LEX_DrLsIiWjeYF8fHboJBcudlYOaoLQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 5F-trZmmQNFDKKHY_uVzUfhcx5YibIFHnWIbF14qltQfRAiuG8Mk-SNn_y4Jt6BsWM3FcZoSgiKkubscAvEe6w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: -N2n0NujOy0kQEw5lhJDB15wLflJ2S17f__GQPNpWOBukNt1yr_uaOzViWudtgouS8G8USltZPUjYx9-RijvTw
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
✓ Created SWR script: fetch_api_user (4Jc6Ifajdlz3EQjXUipVagoee5eVDJ_Hq1PGqEXT3Lmx04R-Qe_8ApS0D47ZDlnnEY9OGn_oF-rd-drTyO4a7w)

Step 2: First call - Cache miss, fetches from API
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
✓ Created enrichment script: fetch_product_with_reviews (jlndLWMiCeDY-Q7n-0zv8FR614Ko9q8Os3q_6e622YOsDkmxeEHQTF-z1Bq2C8UirkqJsm5AkY-CfhnghTywIw)

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
   {"count":5,"avg_price":367,"category":"Electronics"}
   {"avg_price":365.6666666666667,"count":3,"category":"Furniture"}
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
   Vector databases have several benefits such as:

1. High Resolution: They provide high-quality data representation, preserving the precise location and shape of geographical features.

2. Scalability: Vector data can be easily scaled up or down without losing quality.

3. Less Space: They can often use less storage space than raster data for large, less detailed areas.

4. Flexibility: They can represent different types of data including points, lines, and polygons.

5. Attribute Storage: Vector databases easily store attribute information, providing more descriptive capabilities.
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
   {"avg_score":70,"role":"user","count":7}
   {"role":"admin","avg_score":20,"count":3}
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
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
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
✓ Created session: cCTbi43y62OYHyTbqEf8g4kL6ATnISdC8ofkyh4kcPbUARjm81LRW8sfyFGLX7T7xvbZLD_08Uci_raZrIgmVw

=== Sending Chat Message ===
Message ID: 3teCjjXtIB0GexpRs5V1BxoUdJTMiQW3747t0JLXhO3PqMalyTXq26_q9I_1rTmkoEOXjdLKNPN0KLzKl8QTRA

=== AI Response ===
We have three products available:

1. "ekoDB Pro" is an enterprise edition product with advanced features. The price for this product is $299.

2. "ekoDB Cloud" is a fully managed cloud database service product. The price for this product is $499.

3. "ekoDB" is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: 'tboN-YAXg4HWj0qPPqUpBoY0cDyRFSjBXLS95Ert81SYKYgZZ-4jp0f3w7EeOMTqeVltH3zyTFcodMkf_nTEgQ',
    name: 'ekoDB Pro'
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
    id: 'FmhpLJWbr43xksidhHJTDkrWDTF_g9HsUI2_QaYukkmV6YLCqHxMhYyKdAs2Tesp6yxlqoyrv0DhdpNbDKkkDQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'RcyNU6yhcOSv6H7ZjBWwcH6jkbz6AEjdK0fc_CtmNZHBdy8IYZjctFOKtAbTUm4rwVeqJ3sLkr9xfy3QFZO_Wg',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2258ms

=== Token Usage ===
Prompt tokens: 448
Completion tokens: 81
Total tokens: 529

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: oQCVs41ExdTpWbFgvFvZarwqvwTib3gX4vP_COaZjkxg6TTxykbH9NyEbmT5MFDLHiGanx52WsZucIeh60iing

=== Sending Initial Message ===
✓ Message sent
  Response: The product available is called ekoDB. It's a high-performance database product priced at $99.

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
✓ Created second session: zLj1i9P9gFeDCksqf0ll8FGRv6gHp6Afp1qLA45CyG3Iorxhlt0XLEsoDaei5TNm6rWTg2ArXM9kdbmY06GxVA
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
✓ Created session: zI2BbtcsCgihdjDA9HAtYTbM1DUUPzL-XQruo1fAU8qhPzCDzdL5MAxLnDSwC7AL6V4BmzZjU6K2Nl7YGC1eNQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the provided context is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fX7F5zuk46Iga53FFWBe26rcSPvY6AmjHrBdsmKxQGWgsw09BgQTaxHCUN62Mt9xXkOdbMUJFlcGNpnrDSg20g
  Parent: zI2BbtcsCgihdjDA9HAtYTbM1DUUPzL-XQruo1fAU8qhPzCDzdL5MAxLnDSwC7AL6V4BmzZjU6K2Nl7YGC1eNQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fX7F5zuk46Iga53FFWBe26rcSPvY6AmjHrBdsmKxQGWgsw09BgQTaxHCUN62Mt9xXkOdbMUJFlcGNpnrDSg20g (Untitled)
  Session 2: zI2BbtcsCgihdjDA9HAtYTbM1DUUPzL-XQruo1fAU8qhPzCDzdL5MAxLnDSwC7AL6V4BmzZjU6K2Nl7YGC1eNQ (Untitled)
  Session 3: zLj1i9P9gFeDCksqf0ll8FGRv6gHp6Afp1qLA45CyG3Iorxhlt0XLEsoDaei5TNm6rWTg2ArXM9kdbmY06GxVA (Untitled)
  Session 4: cCTbi43y62OYHyTbqEf8g4kL6ATnISdC8ofkyh4kcPbUARjm81LRW8sfyFGLX7T7xvbZLD_08Uci_raZrIgmVw (Untitled)
  Session 5: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ (Untitled)
  Session 6: ltoCE9Mp-YHRJtsl5hQz6NbEq3eQLlHVQko1hD7fAv4kWIRFPEQ8lYFYcwbEKxKjiqcO4ZH5mj5uSn5vyDHDEw (Untitled)
  Session 7: iOoS8YtheulgTtAeIuuE8D3bhrehIrH7-gMf9kcjqZ1WhiK4u5IFgblmUK0mnvBgQq4b_DIusk1CVO0wAc81Ew (Untitled)
  Session 8: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg (Untitled)
  Session 9: hfFbxSDUNbfSKyxC79AJeZd7GCrQRPZm1gkmWSev-oxoy7vEStTatUY4EFqw212ot2-7w95yye9aE4-_D1Ioaw (Untitled)
  Session 10: qO-Z-fYTrhxTnTF_RGGmi6IGq6Cw3n5Wtt4IzSVHy-EGRp8ZpGeXPf6yW9lO2L5vAbnhl3E9xRUrLPDM6uHP9g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fX7F5zuk46Iga53FFWBe26rcSPvY6AmjHrBdsmKxQGWgsw09BgQTaxHCUN62Mt9xXkOdbMUJFlcGNpnrDSg20g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'Nvw-1P28ZkOrEYUsN_vDsc-RmA2WHcfVp4n8MSZBVzH3bU8pkdfoEpCssILbY1Lgh4_ZpuxDfbH9w-xJ9sByNg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'Nvw-1P28ZkOrEYUsN_vDsc-RmA2WHcfVp4n8MSZBVzH3bU8pkdfoEpCssILbY1Lgh4_ZpuxDfbH9w-xJ9sByNg',
  age: { type: 'Integer', value: 29 },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'Rign7R0oxx2gG9rlWM4IEup-nl4gIxvz8ReDSlHsTqYr30oSDfUiL6NqwzByOEoUi_qIp9X8r6-UfowzW9qltQ'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  age: { value: 29, type: 'Integer' },
  id: 'Nvw-1P28ZkOrEYUsN_vDsc-RmA2WHcfVp4n8MSZBVzH3bU8pkdfoEpCssILbY1Lgh4_ZpuxDfbH9w-xJ9sByNg',
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
   Inserted with ripple: {"id":"-DUoKnBmQ28ImH8caMTjWSQ2mBbqsGRQinIbZBbDnCyrbtSiMzow2IVBYrYHQC1gv6U9PXVfcDUE71sIFBcVYQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"DONR9Mk0xF07IVp72TIf2LBe0eK59W9K7sfjsv6LC0FvOhlhYVAImky04nueOAk8-eaXRxU3sRYUcIyNsi0H5g"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150},"id":"-DUoKnBmQ28ImH8caMTjWSQ2mBbqsGRQinIbZBbDnCyrbtSiMzow2IVBYrYHQC1gv6U9PXVfcDUE71sIFBcVYQ"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"ZydUgTtlF11M7KIbOxOxyGl5WUOJdtoY7higeq7t7uRfv9dYqmWKVGt1ncBapzQv7hQhOTIjx6fzxgRIxBCupA"}

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
✅ [32mTypeScript client examples complete![0m
