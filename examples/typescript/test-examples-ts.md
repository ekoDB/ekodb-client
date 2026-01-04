make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 820ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'qp0GAShJIxFYG-OdZIqRASYVr2cyO7-GbmmGuNJNGlXTsNHhZfPfMUh95Mq9TGZ3aFeej2tKp3GbTNm_N0QElw'
}

=== Find by ID ===
Found: {
  price: { type: 'Float', value: 99.99 },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  value: { value: 42, type: 'Integer' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { value: true, type: 'Boolean' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  name: { type: 'String', value: 'Test Record' },
  id: 'qp0GAShJIxFYG-OdZIqRASYVr2cyO7-GbmmGuNJNGlXTsNHhZfPfMUh95Mq9TGZ3aFeej2tKp3GbTNm_N0QElw',
  created_at: { type: 'DateTime', value: '2026-01-04T00:31:11.367Z' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04T00:31:11.367Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  price: 99.99,
  data: 'aGVsbG8gd29ybGQ=',
  categories: [ 'electronics', 'computers' ],
  metadata: { key: 'value', nested: { deep: true } },
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test Record',
  id: 'qp0GAShJIxFYG-OdZIqRASYVr2cyO7-GbmmGuNJNGlXTsNHhZfPfMUh95Mq9TGZ3aFeej2tKp3GbTNm_N0QElw',
  created_at: '2026-01-04T00:31:11.367Z'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { type: 'Integer', value: 100 },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  id: 'qp0GAShJIxFYG-OdZIqRASYVr2cyO7-GbmmGuNJNGlXTsNHhZfPfMUh95Mq9TGZ3aFeej2tKp3GbTNm_N0QElw',
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-01-04T00:31:11.367Z' },
  active: { type: 'Boolean', value: true },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  price: { value: 99.99, type: 'Float' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
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
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
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

=== Create Collection (via insert) ===
Collection created with first record: pA4l9oHEL6VnWTgxHeSxYnasZcXGpjqzKW-VTwkfiQWJbeJtK2h1WOXfsiVsZMttnDoiOeJv4Z_ErWiwsQqIXg

=== List Collections ===
Total collections: 18
Sample collections: ttl_cache,schema_products_client_py,schema_documents_client_go,batch_users,test_collection

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: zlZDcp7wMsg9jR-MPtkl9ulVX0QPZDOU9YmhX2ounbH0uDyiItGG-aOZa0zoRPTBrp08n5XDPO0CNlQDoRlbMw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: AWHZMW0X1qQxRf7S4svGMdTd6_uTieRVeZrVBSWBDNZKv4t8KWuXdlURY5wu8Q-jMOaW0T15K6yDYKsEZzZoKg

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
✓ Inserted test record: WnSDzPzNjDGgZKLoS9B737KYeh0lAR1dUFbuXbvsSsnUjbVO2wxAUaCNXr0KMT9Wf_PoOxzGZ4mAxlUFM4Bsog

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
✓ Inserted document with TTL: c1C4QgDfodf7uYNYercrYIMPeljiAdEmNplGEnJcN0rGC0NfBl4h7qP4zlqXCDFsurXm6iDDmGlCsVUJx86K-w

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
✓ Created session: TX_DOulhqROP3QYBNpg9VSl2sqDE2_GxFX--WbOcwpWJCNRFqcjG4R2LST5CoLsHQ_ekD-UX7x18CSrmPA6NnQ

=== Sending Chat Message ===
Message ID: 2cN3IuLOelcKrOktcAzlBs1IwXUkmYXYKF4LCk1sWYt9E7i2XJU_xxNo1nPojj6sU6XE1M6oSaUOoRHmKx4sMQ

=== AI Response ===
The products available are:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    id: '8ljXMK-53dBfCFJuGhHJSFHyiM75Okug1swQHe_D0_U4YE9SuzE0jeL_jVEsivvz094oKsukC2ojTIblwynCvg',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '7E4uOaSRK43g8U9zFpxBrgv1LF640hGDRhbciF22fz6j3eo06NWDnaHMQtvyHuhxTxMAbjC6kFCJJCjuOUKFmQ',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'ctRcKwPHDjKi0q71QhXixuYr_gvhMQBEnya1PQDzPD6clVgmyoy72vPacIxv3pGW4ehMJzDYNfyhjMsLpuPFOg'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3411ms

=== Token Usage ===
Prompt tokens: 599
Completion tokens: 83
Total tokens: 682

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: bWGPxRS0ic4XsrV8RsXsr1MmTxtNBKlG1v2LOTwmYGS7gJaxEzmp-X41KsSNLhmg-Z7KpUcs0umjPCWdxq-K7g

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information I have, one product that is available is "ekoDB". It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fleuzb-jXI-jRyE2LP90r8Cfvgz1w-4zWbh0tr4wqgtmorVIYgjj9a0o2KMrZCw_t6bel8uCb7l6Wts0Sr5LIQ
  Parent: bWGPxRS0ic4XsrV8RsXsr1MmTxtNBKlG1v2LOTwmYGS7gJaxEzmp-X41KsSNLhmg-Z7KpUcs0umjPCWdxq-K7g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fleuzb-jXI-jRyE2LP90r8Cfvgz1w-4zWbh0tr4wqgtmorVIYgjj9a0o2KMrZCw_t6bel8uCb7l6Wts0Sr5LIQ (Untitled)
  Session 2: bWGPxRS0ic4XsrV8RsXsr1MmTxtNBKlG1v2LOTwmYGS7gJaxEzmp-X41KsSNLhmg-Z7KpUcs0umjPCWdxq-K7g (Untitled)
  Session 3: TX_DOulhqROP3QYBNpg9VSl2sqDE2_GxFX--WbOcwpWJCNRFqcjG4R2LST5CoLsHQ_ekD-UX7x18CSrmPA6NnQ (Untitled)
  Session 4: VRHWIC7kF05GxbyDMiUw3FVNimhEZRsjE5BzXOgppbkEvI0gPniZaKTNKA-Ta9tI1IjBFEdeIT2myg1F_JvQ4g (Untitled)
  Session 5: PJ8nggyQAzxxbTPNehaA9ppN_05e5b5tbmQFUBTc1fBMGrb9aGE7lYK6i9vyy0vBfrOLFoD_REB7ukGOYzECWQ (Untitled)
  Session 6: O0e5OhjkxTgEk947WT4uU8b5O7jaV8ukCVC1jT4HoUVUxWYNjIQW59gljpW6ScYuz8eBNtjDRB7XPM1v_2XhTQ (Untitled)
  Session 7: 6V4Pa6UrFZn93O2rj4Xl2oPWQBRhE5NdBj-mVQPTiEg9jLGGJBmAYi4aMsrNyEos-WdgVlcHz9orGRnSM0okww (Untitled)
  Session 8: otL5_twubZAn9rGrc4bEFnd5xkY4aS2V7VeuBxE3pivjxf5HDsQBPB-gbnqrtwHNe0x-K4V_bsQIlymnQFDwSw (Untitled)
  Session 9: 9_Jjc959t0oHyhGBza3R3NQbjHYLRTpswpfQR5l9OKH7tCF_LGFPkB1rbqnKFjS45RdDuv67_0nCrE_yC0eGJw (Untitled)
  Session 10: nG-we3gQtp1gSDq6fq6xMPlJ-4gSKMIesqqHvUqGbz21aNYku8Q_W2jLmV6iL1vTODEep_b5rEryHUaH2yHB-g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fleuzb-jXI-jRyE2LP90r8Cfvgz1w-4zWbh0tr4wqgtmorVIYgjj9a0o2KMrZCw_t6bel8uCb7l6Wts0Sr5LIQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 4PvMPZSvcexbluA75ghWVxfayg-0oWMhNELHe3l2DsRQ2RGoV0B4MfoLAhh_axFYwVuucRG4I9GBCee0npvlPA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one available product is "ekoDB". It is a high-performance database product. The price for ekoDB is $99.

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
✓ Created second session: 5SEaJb-mU5v5VYrvgH9Opdx9l83D41Dcrwc0CwBJnAzNkUVTLgKL0APa0JhyfgWb046Oa_g2c3gy_kB1XKQF4Q
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
  1. Score: 25.740, Matched: email.value, name, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio.value, title, bio
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: bio, title, bio.value, title.value
  4. Score: 26.400, Matched: title.value, title, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: bio.value, bio, title.value, title
  4. Score: 39.600, Matched: title, bio.value, bio, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.755, Matched: 
  2. Score: 0.747, Matched: 
  3. Score: 0.726, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: title, content, title.value, content.value
  2. Score: 1.490, Matched: title, content.value, title.value, content
  3. Score: 0.302, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, skills, bio, bio.value

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
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: WWOPHFPcHSP305a7jOVlTwmETrcXxXkXLw8UbEuUYIqi-w7uBHuu7TSVzsJ-lQbavV2Kf4m5vHgWqhvgOjgU4A
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: EJh5PISuPssIxK5UWcYy6O1qx2t2SXEzLiFN4Q8HLpjUXgHDGi79TNCco3JPTJyrJwA0CExGd0wBv6dgVEdOng
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 3XxGjPHODWPPQXE7LP1rHa4Vdf42Ih7Jpxr4ymmTp7FLtL_3S5O1zwUmzuNPlKTr7lRPxvzUUSgoTso4wrwfrA
📊 Statistics: 2 groups
   {"count":5,"avg_score":60,"status":"active"}
   {"status":"inactive","count":5,"avg_score":50}

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

✅ Script saved: emLR9mHyFO_aINAGEjjcQvl-psL8wRTz5n2xHDZePNQdsaF3ow5jXZ52cXXC8X2Bb1B8L9Tghoao-kUsmwkOVA
📊 Found 2 product groups
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
   {"avg_price":474,"category":"Furniture","count":2}
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: WmzWIlkvYxnq8PHwIZElr5xofEYnV3dyu9HraXxGFn2ETneGFSiEqTQgGkpa_B8kB2ddyXsmXfN5UC_YCk80sQ
Created Bob: $500 - ID: vrKAqLZN_v96yCm2TlCLPweZ5WXBZmp00mD0pL2E0nF5mJRO7dljAdyhLI8KHCIHW6b1-sssw1Bqv7pXPrc33Q

=== Example 1: Begin Transaction ===
Transaction ID: 406f762b-d1a4-47f0-a728-5325ef27f7ae

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6cf5a515-2468-4ab5-a655-f8f7674c037e
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
