make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 7 packages in 2s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 1s

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'p2NFAHb6sUkU9omg_KCBc-tDwYZSZxO1lYIP56Ju2DoLH5jd-YNDdhfVRzaIlQHENjkVI3XP_LhlMnkbfI99oQ'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  id: 'p2NFAHb6sUkU9omg_KCBc-tDwYZSZxO1lYIP56Ju2DoLH5jd-YNDdhfVRzaIlQHENjkVI3XP_LhlMnkbfI99oQ',
  name: { type: 'String', value: 'Test Record' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-01-03T21:35:28.473Z' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  price: { value: 99.99, type: 'Float' },
  value: { value: 42, type: 'Integer' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03T21:35:28.473Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  id: 'p2NFAHb6sUkU9omg_KCBc-tDwYZSZxO1lYIP56Ju2DoLH5jd-YNDdhfVRzaIlQHENjkVI3XP_LhlMnkbfI99oQ',
  name: 'Test Record',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  created_at: '2026-01-03T21:35:28.473Z',
  data: 'aGVsbG8gd29ybGQ=',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  categories: [ 'electronics', 'computers' ],
  price: 99.99,
  value: 42,
  metadata: { nested: { deep: true }, key: 'value' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  id: 'p2NFAHb6sUkU9omg_KCBc-tDwYZSZxO1lYIP56Ju2DoLH5jd-YNDdhfVRzaIlQHENjkVI3XP_LhlMnkbfI99oQ',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  created_at: { type: 'DateTime', value: '2026-01-03T21:35:28.473Z' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  price: { type: 'Float', value: 99.99 },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { type: 'Boolean', value: true }
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

=== Create Collection (via insert) ===
Collection created with first record: mNny9H8UXqkPoFTz5Ds0U7rDDm2nvfvPiP5XJ3Ox3g0ssL80Aj5rkxlLH_x7z225NdmhhAkfo070NLy1ceF_Wg

=== List Collections ===
Total collections: 18
Sample collections: schema_employees_client_py,batch_users,test_collection,websocket_test,schema_employees_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: _5GSriCpmheUUMrk_498rljOF_NNewdZuZX8ar91DrJCeOiLMUycqiUjjjr-yzs2cYV4LLoP5imO4avEzShilg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: l-Ohldt4xV4GXyeO9w0g1h66uWraTfpfL35-7TWGdCYN-M1yUdidvpFlDdBM4MDFIvqngsiaxdYgGLkY7eIutg

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
✓ Inserted test record: nEPVK8c2fqk7n_Qwe-3NInTDLGza5V0YsnPzbHOPtLVYweL1c5dHeFHf-bF7ou-oX236JEdywgUHwi71vE_Jeg

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
✓ Inserted document with TTL: wRuJq3IiUJliZrxn7SKz9qMYqc73_0k8yivf_znywwSjzgXwD5omQd3eEwvMqlcK69T9bfEats78RwMHKUOHLw

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
✓ Created session: BxypNJVjxc3WKBhWWblz_0-0ffBRajRQqrhjGPNw5QECgj4J-WYmRleVT_yxrDX5BHS9YXLDqXxGSLf2KIY-8g

=== Sending Chat Message ===
Message ID: BqkYceA6JCuPI0u9fwBbdefScrPxCSZ8iEPtQoLmkdkIScvdG19XaIgqvB4ifapkxYnuTvQTV9oAsCs-GXdK6g

=== AI Response ===
Based on the provided information, the available products and their prices are as follows:

1. ekoDB Cloud: This is a fully managed cloud database service product, and it is priced at $499.

2. ekoDB: This is a high-performance database product with AI capabilities, and its price is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. It is available for $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    price: 499,
    id: '1RO0zIN-oIJ4Zy85TgAdKoOet9r_VUvgJydnq0-cjusqc7nSa7duuVRVUJlwHtO3Jg_fly2cVnHskIR9kQwOdQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'LB-RKigaVc4nWTkkULCsAGXCQqM0ZNmSk7RP0odUw7gFjB7gHHPV-rCKvQjMUdrPcv-QP_soPH3vrEcdeXCivw',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'xUM5aRS4RqI_4WpLbL4r1Hl77roMwZNDx3QDQauZdCXB4sdF0_rlSc3MZVXtIcrRi--oMKnTaS3CFqPk1EQnIw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2405ms

=== Token Usage ===
Prompt tokens: 611
Completion tokens: 89
Total tokens: 700

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 7s_x5pLxFo_-s7n_HkpigCdPTVgIwdlp0Qot2QZOzhaWXYfbhhzor7sR5D50sJjzulxgZKJvTDuljVBFkYSh_Q

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is "ekoDB". It is a high-performance database product. The price for this product is 99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: oU694hDApeyerLvTRK4bVjiGzF9-PsKbU6axv5hwLbJjt8SGRETTkLaH8T3spMhOmGeFUancRtUDqjVYtViSjg
  Parent: 7s_x5pLxFo_-s7n_HkpigCdPTVgIwdlp0Qot2QZOzhaWXYfbhhzor7sR5D50sJjzulxgZKJvTDuljVBFkYSh_Q

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: oU694hDApeyerLvTRK4bVjiGzF9-PsKbU6axv5hwLbJjt8SGRETTkLaH8T3spMhOmGeFUancRtUDqjVYtViSjg (Untitled)
  Session 2: 7s_x5pLxFo_-s7n_HkpigCdPTVgIwdlp0Qot2QZOzhaWXYfbhhzor7sR5D50sJjzulxgZKJvTDuljVBFkYSh_Q (Untitled)
  Session 3: BxypNJVjxc3WKBhWWblz_0-0ffBRajRQqrhjGPNw5QECgj4J-WYmRleVT_yxrDX5BHS9YXLDqXxGSLf2KIY-8g (Untitled)
  Session 4: bJbj73clvxGp4MYjUexkDsFYJZ78e9upaU8opyzkREOc5kTXrWSo5hoLF3ikXUAIGgYil_ZVf0xY0U0x0pVNIg (Untitled)
  Session 5: DTY5mWctSn5Krw3sgTQNMfEazH3V04OiZEt9zM6kSi8E4kav4Tg_jfUI8SxWcpcFxSKfrG0F1KY-_7sYjoQSGw (Untitled)
  Session 6: 2UHvU8UWM77TNkFkjMaTU754Z7MRFajZpf8CCNnHtWyx3ilR76h8zZSuGv-KlxJJSLPd6y2Tbh2TOhhNVOB7Ew (Untitled)
  Session 7: HvdlKRKII1VWid5cNQ0-HK17Qbk6_WXWU-tlxQ7Fjx7gnJwvMUCldZ21ikUg_fz2dfOn5hv90pnIG3uLW3Z3Rw (Untitled)
  Session 8: wg3xdpQyALUaf3raEAK_aaMirLmzVKNiRvDmUlFEchZL29zEYSS5OQTeoRJ_SUgMMVXZ0tYKAanLUknaT9L6UA (Untitled)
  Session 9: suW32bDnrdg3dzucV75MS8O41JMRsxVPZR4KXhybpNPRgnT6cVCZdvuc9_7tH0H_Nvr4g0bW5094uRafHh7o-Q (Untitled)
  Session 10: 0dBcE6CuGPe38twSLr6KwkhV4MWkOcyUILsxWuw1XuDS6lcHD9gKIUGow5S05ZNUmWnef4DZqYv4Z5Wg4QZaeg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: oU694hDApeyerLvTRK4bVjiGzF9-PsKbU6axv5hwLbJjt8SGRETTkLaH8T3spMhOmGeFUancRtUDqjVYtViSjg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Al_18l7xyIhHvw8MIrs7DRzVDOVtJsACVssEEeAOEZSX5Skr-uBs5LRalEEYGwnmVW4FMjOXwzMb1M2QYlPn6Q

=== Sending Initial Message ===
✓ Message sent
  Response: The available product based on the provided context is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: EZXhmtbp5YLQ1ovUR8cLdQh0oHylcQz2VAbg6N5S6yeiX7Ce7GtymeHRzxgsCZmRJC4OmSDd_sIs3TQXmXe74A
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
  1. Score: 26.400, Matched: title, title.value, bio, bio.value
  2. Score: 26.400, Matched: bio.value, title, title.value, bio
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: title.value, bio.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title, bio.value, bio, title.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: title, bio.value, bio, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.779, Matched: 
  2. Score: 0.764, Matched: 
  3. Score: 0.763, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.712, Matched: content.value, title, title.value, content
  2. Score: 1.506, Matched: content, title, content.value, title.value
  3. Score: 0.305, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, bio.value, skills.value

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

✅ Script saved: uuI8v2-vF0Tq3d0YV6-HLAXUk9lCCJqUd1K6p7vGAnF88Qg6JNNXZTabejXlN9v5M5veyqyB7JlBbt_tg9kt9g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: _Xm1VY-9S5qTJ4Ihb5KvWow6yTRITzJLFLVT9isPht-3wVgftNnC3KOzWNwzeEJl6fa_4DACmqTAu_3iNLm6Eg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: fJgg1usvWdV_Nmu7yPiexUrpIwxVMq7L3wb8J_umxD6QWpjmUp4mrzq4mZbIrXo2-XIwzmc-rK2YgSaYn_897A
📊 Statistics: 2 groups
   {"count":5,"status":"active","avg_score":60}
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

✅ Script saved: OUg8Vn_UlSfANKKEXBjQ_m_ySQGeuKcUMIrCuBuALu06udLSgZt4d6VKk9R7E1Ht8F2n7J41NJhrZucHiE9SBQ
📊 Found 2 product groups
   {"category":"Furniture","avg_price":474,"count":2}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: y5uUYeGSKLs__skRMuSjCl8e5xLvzO9VcRlhDWjLO3qiOVvAdgHchBr8ShT24PCoXv1Es-ZGmFFXBeOLlk4Tww
Created Bob: $500 - ID: skGK3kkpl0TFBt6qJ9PlCvH0X9_zEcdEJ__scBgkDQwrGbeamplVfhV-GPIN06jjxhFEXo7u0NgGLvq2vNaqXA

=== Example 1: Begin Transaction ===
Transaction ID: 006c88ee-9f38-46aa-94e2-0a8da95f7a46

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9c3f308c-5292-42f1-ade5-7fe1207c08fd
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
