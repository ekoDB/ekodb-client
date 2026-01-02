make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.6.0 prepare
> npm run build


> @ekodb/ekodb-client@0.6.0 build
> tsc


up to date, audited 7 packages in 881ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 692ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'NZZ5HFFqFZi66TS0JOrkmTWk5qnrr78rhmo1eK1-UzM4JaPaXMdhxIlJiU4JWWQs9oMtl7ZBiw70Y99oXtibJw'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  price: { type: 'Float', value: 99.99 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { type: 'Integer', value: 42 },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  id: 'NZZ5HFFqFZi66TS0JOrkmTWk5qnrr78rhmo1eK1-UzM4JaPaXMdhxIlJiU4JWWQs9oMtl7ZBiw70Y99oXtibJw',
  created_at: { type: 'DateTime', value: '2026-01-02T05:55:02.313Z' },
  name: { value: 'Test Record', type: 'String' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-02T05:55:02.313Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  metadata: { nested: { deep: true }, key: 'value' },
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  value: 42,
  data: 'aGVsbG8gd29ybGQ=',
  id: 'NZZ5HFFqFZi66TS0JOrkmTWk5qnrr78rhmo1eK1-UzM4JaPaXMdhxIlJiU4JWWQs9oMtl7ZBiw70Y99oXtibJw',
  created_at: '2026-01-02T05:55:02.313Z',
  name: 'Test Record',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  id: 'NZZ5HFFqFZi66TS0JOrkmTWk5qnrr78rhmo1eK1-UzM4JaPaXMdhxIlJiU4JWWQs9oMtl7ZBiw70Y99oXtibJw',
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 100 },
  created_at: { value: '2026-01-02T05:55:02.313Z', type: 'DateTime' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: Y9SSVOzXiKKNf7RpzSModta3T6xO72yKirOnuVYg3nqp2-jmoplSktH3Hff2zdmutp_sNSDbYAb5URYf-LazPA

=== List Collections ===
Total collections: 18
Sample collections: batch_users,scripts__ek0_testing,client_collection_management_ts,schema_documents_client_go,schema_employees_client_py

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 9MRA5VRTudAQ-2xwq_QrrJ2nlP-zBcz4ivnafACmqcmYBBFo9JySGx61VYaEkZ_VKuXiAc9OrtHTFvCau8rKjQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: b6GkpaxX1HMQgkcJ3qBNPrb7sUIMfREyh8GfKHcjTCqXkq5IvsJKJp6mYuhce_kI6Exd5IGB0lD5VrmHE2KWEg

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

=== Inserting Test Data ===
✓ Inserted test record: 93KEZrE5NTiJkrefKlXFUrhU9Cr0IKWwyM99PXsch-JiavD_RRq21evIbaoxO0iix2FDBtJeTdjNc4m2bbFBQA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: zyfjobr0aru_O0bPOkjV1LTkVVA02B_cfKcjusbOL4HctRHxxjhkn8EmT49nhlxGskhUUCj9R8lOyUJpS75rzg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: vUOFm1gwk16f2Yy3fKRYFQPEY_LVz3C3gOuAtjTCgPLTkm5GdtvmabEOOTU05KJU3TAN6lsYQC-h_9NZWeM6tg

=== Sending Chat Message ===
Message ID: 7VFzxRI57xKH_gWvF6whM0b11iLB43Uio0XGC3_PxBujFEd8pKmAvlJbm2ns4g1rWFecdRcdT6xGWYX2v-s1qw

=== AI Response ===
The available products are:

1. ekoDB: It is a high-performance database product with AI capabilities. It's priced at $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this service is $499.

3. ekoDB Pro: This is the enterprise edition product with advanced features. It costs $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'V4zPmgg8oKYa0MZk9bY0thu0RERWtjTtVnwvGtzVpcrs66VJ40kd5TSOigEAWfQcsFPkZDdJ_CY6ExOI849Huw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '1wFL4t7ffZsj5rMUFEPkQc4ohUYmdC94SKm4Bz_0XiheOGsFJXbKn3aeeb7z_sPcVhuGh1_x1ENX_clXHzCAZw',
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: '1ipveGOc_TnGH2j8JytXdRdvJZDwyVNgDL52AAt6DP-oV7nmWt9bQn5omENcVN-bcGYxGB3t41bMrV02l_goFg'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2836ms

=== Token Usage ===
Prompt tokens: 607
Completion tokens: 77
Total tokens: 684

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: AUrWYP3AQ478NPx62UUEN2ZF9qNnXHhc0t1Q9mwuU1Fok7gFaaBxk3H24qrcB1N9TIlHN_DNipjbbZRv7Et_Fw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, one product that is available is ekoDB. It is a high-performance database product that is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ZmVXv4PiVPl2jcsJ0gDsHujkKcwQUD8pgRq12mNe6l82EIqmMAsuVEJFiIawVgCbd_CAEcvpC9zdbgZx3UxS4g
  Parent: AUrWYP3AQ478NPx62UUEN2ZF9qNnXHhc0t1Q9mwuU1Fok7gFaaBxk3H24qrcB1N9TIlHN_DNipjbbZRv7Et_Fw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: ZmVXv4PiVPl2jcsJ0gDsHujkKcwQUD8pgRq12mNe6l82EIqmMAsuVEJFiIawVgCbd_CAEcvpC9zdbgZx3UxS4g (Untitled)
  Session 2: AUrWYP3AQ478NPx62UUEN2ZF9qNnXHhc0t1Q9mwuU1Fok7gFaaBxk3H24qrcB1N9TIlHN_DNipjbbZRv7Et_Fw (Untitled)
  Session 3: vUOFm1gwk16f2Yy3fKRYFQPEY_LVz3C3gOuAtjTCgPLTkm5GdtvmabEOOTU05KJU3TAN6lsYQC-h_9NZWeM6tg (Untitled)
  Session 4: 1lwWsuF-SpuDCtoWk_T-U8D5vc25tw6GmsM1uUb7QWJq4olAo9gNU9okLB8-xHp3ROrd89v3xm9edkETQcAvDg (Untitled)
  Session 5: 9ooPTyK8bGLT90NGU7Lv89FzCXtNlq6M2ntR2f6tGRxIkRkbzSEKeEaLdt8xegkxdkttJFZPUiGBksyDItd1lw (Untitled)
  Session 6: v75EUAjj7PJMmulZcK-nv-L3uWabEVFw9NTNXlqSRC0zjIdx0xwdJNTBP_a22jrfVAYdObbxOvjsjnk2h_JUtQ (Untitled)
  Session 7: 8ij4JREUz3EKuqBCksCRA7svRIYYv-D5XIJB5gTr2mYoHY___fq_MEDed0_QHBNvUjkHoahbTBSyMnR4_TWC8Q (Untitled)
  Session 8: kGSJYCEbzW3WYaCWW7qVi3faqJ3rd7v582U6huy7pCWYY8f6ObSU-0M1hvqtXTDMoeMrmCByIiW54wUOC3a90A (Untitled)
  Session 9: DszLGnWdRc3qVTd1hrSCP54uqUp7i7BhSY2t8yWH1gNfoggn4Hap_8D6QTrKr9D_wYIYcLDRI8xQtzsAUY7HiQ (Untitled)
  Session 10: m0J43kGqILKtUrl5O0HN4ccU2Y0S8UshpcPMxHmfvLO8rxHA6k6TbxC3u0LD39-JDoyHbM80XLYomD3kVphUqQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ZmVXv4PiVPl2jcsJ0gDsHujkKcwQUD8pgRq12mNe6l82EIqmMAsuVEJFiIawVgCbd_CAEcvpC9zdbgZx3UxS4g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -OdWMRookXJTiyO0UvTrT5SVFMs49B1UVR0_Aqcas9aF82fMCF2iqeJN1ADc4gJ2vzolPmEeKjueyetaa5WR9w

=== Sending Initial Message ===
✓ Message sent
  Response: The product available is ekoDB. It is a high-performance database product and is priced at $99.

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
✓ Created second session: q0hu2aIumqVGuwDbjf448esCGD7kDNK2c3YQzwDUKuUVCsG99poJd4RtiLb6WEImzCMMRhaDfLz_1suaKKfBlw
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
  1. Score: 25.740, Matched: name, email.value, name.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: bio, bio.value, title.value, title
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: bio, title, title.value, bio.value
  3. Score: 39.600, Matched: bio, title, title.value, bio.value
  4. Score: 39.600, Matched: title, title.value, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.748, Matched: 
  2. Score: 0.744, Matched: 
  3. Score: 0.740, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.696, Matched: content, title, content.value, title.value
  2. Score: 1.498, Matched: content, title.value, content.value, title
  3. Score: 0.299, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 3lzvpjD5aAIXaQbqUZmmT276tRi1QwJDUyopBj4-682lK8Mjr1XK_iiK6sOTnMJjYY84R2jj7Ime6GvQO_lXRg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: EKoMlYIG1diPg8vmUB4EHLSgnAC6UL4arUd2V9IVth7w7tcc3cLbxVWrbA_GI3pl0xgebX77N8KQz0b_crtUCw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: K95BvURyU20dZyLoFyUg6dAk1dwFFMEEquK9LHQa5Ap3FAqKGhgrpYLQKMP16ukCQumX501tm-kxre0z6_S-Cg
📊 Statistics: 2 groups
   {"status":"inactive","count":5,"avg_score":50}
   {"avg_score":60,"count":5,"status":"active"}

📝 Example 4: Script Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: Pv-d4up0FP-4U9jQRdHAcnD-6X9b9NPZF3D7pxlGT51BH7OxD3c-HOJ1gllIzaA6pr61e8LqTVngjIP-bQI71Q
📊 Found 2 product groups
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
   {"count":2,"category":"Furniture","avg_price":474}
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
✅ [32mTypeScript client examples complete![0m
