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

added 1 package, removed 1 package, and audited 13 packages in 797ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'MvxyMuRnSODOCglgKyfT8ipnOZxaEihEZAHvQzfM0COQ5x2j2rb-7H9RBpBnSDbYiTa8jXIk86ad_kfmDDghAA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  created_at: { value: '2026-01-03T23:55:33.190Z', type: 'DateTime' },
  id: 'MvxyMuRnSODOCglgKyfT8ipnOZxaEihEZAHvQzfM0COQ5x2j2rb-7H9RBpBnSDbYiTa8jXIk86ad_kfmDDghAA',
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03T23:55:33.190Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  metadata: { nested: { deep: true }, key: 'value' },
  created_at: '2026-01-03T23:55:33.190Z',
  id: 'MvxyMuRnSODOCglgKyfT8ipnOZxaEihEZAHvQzfM0COQ5x2j2rb-7H9RBpBnSDbYiTa8jXIk86ad_kfmDDghAA',
  categories: [ 'electronics', 'computers' ],
  value: 42,
  active: true,
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'MvxyMuRnSODOCglgKyfT8ipnOZxaEihEZAHvQzfM0COQ5x2j2rb-7H9RBpBnSDbYiTa8jXIk86ad_kfmDDghAA',
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  created_at: { type: 'DateTime', value: '2026-01-03T23:55:33.190Z' },
  price: { type: 'Float', value: 99.99 },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { value: true, type: 'Boolean' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] }
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
Collection created with first record: rpAh7ElzSljzYzWHh9Yjxqp7dESPfEyO8GRF-lAETeoq9hcgdBeMqhTYq_1B0FT7ba6Wskv5mfalKU2aLYYhUw

=== List Collections ===
Total collections: 18
Sample collections: chat_configurations__ek0_testing,ttl_cache,schema_documents_client_py,schema_products_client_go,schema_users_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: yD-DlYdmV3clotFc4Ib4t8I9beXd3OGNKWSLSrlGP5_yUbFOL-85IbSYQn4ll07wg0V_UtO6KQQ59vSc9104FQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: UE80CJqooBI52iloRFunNzVlxy6nLrlNrnG_PO31a0luD8hc2qu-vRxSy8PdONQ7N7D7OonD5DbrXlLVhVYqNA

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
✓ Inserted test record: 3dK618MpMRBxvO9jB4NTrmfiu-zgudKZ1uOlQPdf_AQz5S05EVisdNwCQ8G5fNvzoWvGk3zPS-SGrN75h0dzPw

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
✓ Inserted document with TTL: zmZLVFVTWeYYa_nsiutYcobYLoiwIY0-3GlZzfFzxhjPils_Ql4BCODMV1Gs-VPDqMRQYRxvIKwnRzmAhAPG9Q

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
✓ Created session: KoYojp58i-Wk3pORhJffUTNtd0Ki68ZOkImoEKp6JE1zVBIyRy5o5psjVUzGT_ZnZGvdN_ihaq7n22yiVEE8Ng

=== Sending Chat Message ===
Message ID: UsrvSwpWqoi0RZx6RjosrIzLBRp4ivxWewaPJ87UFb134WzrbZP9yBZpaTnd3E6QV0Doud28EunihOBIvlDT1g

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Cloud: This is a fully managed cloud database service product priced at $499.
2. ekoDB: A high-performance database product with AI capabilities and it costs $99.
3. ekoDB Pro: This is an enterprise edition product with advanced features and it is priced at $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'ZHigRoalnzdfENV2s5stMLszWZEZz5AMKEKHsBIqkyAg_aIn_umQm-RglF3pIfpgK8zGwRhtp8z4rFNWk4noXw',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: 'u51-1uWMTccZD2inEEONLbbld0poR8TAtVHtbpjua0Blu_wqz5XGEe8pPAeL-XlxbzaMZiAIyfHItG1wbkIVvw',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: 'wRKQo9RqBVUWF5S88qWrL8_Xzi1mR52ld4vozfK2lR3TfqP-5aPYKISG44E3UiBRZtOuaEfmGs27JgconIFVGg',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1849ms

=== Token Usage ===
Prompt tokens: 602
Completion tokens: 75
Total tokens: 677

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: IdJ7XroAEsW30zhOxwJTaoxysS1ptJV8WRE7EqkoXMcHsfpjbvhigBhR0ulgYhAOTif7rq_sHpLUrtDIuQmleQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available according to the provided context is "ekoDB", which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: smpGicIdxDseC3WtZbVuXszqXXnw74pxWOJ8YKSyE2aJlEWbqkdo3t2cpOQFC-38zg-pu8GgC18HyudbqtqH8w
  Parent: IdJ7XroAEsW30zhOxwJTaoxysS1ptJV8WRE7EqkoXMcHsfpjbvhigBhR0ulgYhAOTif7rq_sHpLUrtDIuQmleQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: smpGicIdxDseC3WtZbVuXszqXXnw74pxWOJ8YKSyE2aJlEWbqkdo3t2cpOQFC-38zg-pu8GgC18HyudbqtqH8w (Untitled)
  Session 2: IdJ7XroAEsW30zhOxwJTaoxysS1ptJV8WRE7EqkoXMcHsfpjbvhigBhR0ulgYhAOTif7rq_sHpLUrtDIuQmleQ (Untitled)
  Session 3: KoYojp58i-Wk3pORhJffUTNtd0Ki68ZOkImoEKp6JE1zVBIyRy5o5psjVUzGT_ZnZGvdN_ihaq7n22yiVEE8Ng (Untitled)
  Session 4: o_BggN1qIUogt0S1WPe31EPA0f2Y0eNckWn74WiDxXaSXeoZx_KvkCI0z7b3iqRZc9GJPCmI5xLqZFA-U-BXkg (Untitled)
  Session 5: KnySy_WEfsp_X7M9HFbWUQFUF0_e0-g83hQ4ruClauV_v3Hx3qsidknLRmcrBq9N9LjEYo3Qf3E-Zf9vp1BW0A (Untitled)
  Session 6: Dogxv1Bte8IojZZSfjozm5Q7Bh1PRfDRm7Kts0TLSDNhwVoTkZU9B7Zyryl4unpGYGP_brfhd1cvzWqBLSZoxw (Untitled)
  Session 7: pJAkPJ50ceg34xRRKItswW6PzC6DxtbaJCb0cxXJ-vky-Pz_nkfgehDNTp6ms__wEKw2RaPUOav2-ltIrOmPqg (Untitled)
  Session 8: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA (Untitled)
  Session 9: l4g7zNZrVeaCo5zuQPZks0Yqia4jcciEDUMpkyHfX0jkRBqSmIH-phsZn7hXgLEJV4kU7DCyyt3ahLVBx7oCHQ (Untitled)
  Session 10: Z2pocQ1N977hiiEZksfO2WyAC5l-VC1h9TJzhg0S1xbXfB2ONM-Qk_Zqb6aWaumPAg1PXd6Wrerc4mYNkdmKdw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: smpGicIdxDseC3WtZbVuXszqXXnw74pxWOJ8YKSyE2aJlEWbqkdo3t2cpOQFC-38zg-pu8GgC18HyudbqtqH8w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sN8lTnbo0eczBZScGPE0q8sUUXc0QVUpCLtHoARXhheKGupZri6GvXgJxZMsOMZWeGInTURM__ZGCQZ83O_asA

=== Sending Initial Message ===
✓ Message sent
  Response: The product available is ekoDB, a high-performance database product. It is priced at $99.

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
✓ Created second session: IxBgwdbCT9z9Mtjl5_Kv-MoF93OLRm9xaxJZAwE0yrJ-C4joZ_2CyNIegYiej98z3vBnvoYFledtEVJ4sgBXHw
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
  1. Score: 25.740, Matched: name.value, email, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.755, Matched: 
  3. Score: 0.750, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: content, content.value, title, title.value
  2. Score: 1.508, Matched: content, title, content.value, title.value
  3. Score: 0.300, Matched: 

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
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: lbEfbEVqj3ejQnJeVcv5emBlwxmi6o-jzNZYPmdSWgG_fUKaVQhKdX0K3kK6Bwt9LPPeLuLIUtv3y02BaWBD3g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 7XEZe-vrtE8hq4GPixPEtgYV96113yiDN2JEqTWTbUo8Z48KTI8SJoJeWqvTKBXlF_7m-LfYT-F8O6w2P9KX7w
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: mcAW1pqmlSTW7Pj4zDhIiD_c77pRXfC8peTmDFUmy-vB3L_exRwF5NjVDZ8Fvt1ybUtzmyDDOl4flABmddCFBg
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":5}
   {"status":"active","avg_score":60,"count":5}

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

✅ Script saved: XK_1kYWMtaBPQj7GtDlKp_ICd6juKxZCvFrWr2WTH6_HOG304ieKch9UubjoBPCsczzCjngp-8CU9ITw8WWA7g
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: DTtj-IFwRN_JdxgyGk3jUoODWWuydl-p-A3CZAHeHVJouY6sVC8Tx9BPE643TR8hsyVyvjDSmMZGxUH7vUkkLQ
Created Bob: $500 - ID: db4gZt12fnbsFaCzHz2gjJJlRXpVKghPmyjQEyPHWFAlna8oyNsHSMKtvUoypNgBHMN_AenNCNWT3R4QFvtCqw

=== Example 1: Begin Transaction ===
Transaction ID: 170de70a-773e-43c5-a586-2bc6c7f44156

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6f3f4253-a414-44ea-a914-ff193e024364
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
