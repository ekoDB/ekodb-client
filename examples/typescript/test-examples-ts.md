make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.6.0 prepare
> npm run build


> @ekodb/ekodb-client@0.6.0 build
> tsc


up to date, audited 7 packages in 853ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 669ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '8D80NQOBRwNCMfeNeg9-yEkAwiSP8rZq5JqZZBL7r1DxWjRwL3KiSkA37QlsFSUiS59tNFad_5wWxJatCOKJhQ'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  name: { type: 'String', value: 'Test Record' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-01-02T02:17:32.003Z' },
  id: '8D80NQOBRwNCMfeNeg9-yEkAwiSP8rZq5JqZZBL7r1DxWjRwL3KiSkA37QlsFSUiS59tNFad_5wWxJatCOKJhQ',
  price: { value: 99.99, type: 'Float' },
  value: { value: 42, type: 'Integer' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-02T02:17:32.003Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  data: 'aGVsbG8gd29ybGQ=',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  name: 'Test Record',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  created_at: '2026-01-02T02:17:32.003Z',
  id: '8D80NQOBRwNCMfeNeg9-yEkAwiSP8rZq5JqZZBL7r1DxWjRwL3KiSkA37QlsFSUiS59tNFad_5wWxJatCOKJhQ',
  price: 99.99,
  value: 42,
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  metadata: { nested: { deep: true }, key: 'value' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  name: { type: 'String', value: 'Updated Record' },
  id: '8D80NQOBRwNCMfeNeg9-yEkAwiSP8rZq5JqZZBL7r1DxWjRwL3KiSkA37QlsFSUiS59tNFad_5wWxJatCOKJhQ',
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  created_at: { value: '2026-01-02T02:17:32.003Z', type: 'DateTime' },
  value: { value: 100, type: 'Integer' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  price: { value: 99.99, type: 'Float' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  active: { value: true, type: 'Boolean' }
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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: NBpdssn7f4DQ81dlK-s0zb5SHODmBexwTMDpkloeuti7JM303iOEeTenlCb4KISHCKKgMNorasEKwVfu9LFUuw

=== List Collections ===
Total collections: 18
Sample collections: chat_configurations__ek0_testing,schema_employees_client_py,test_accounts,schema_products_client_py,test_collection

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: hqMyCIIDWBN5GAZ97TZVe99KdRpH7OZRZMpLz9L6DN87IFZtS-ovhVhDd9bTGtGKAWKWedBVyqFLwaHVEzPD-g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: qvyNauL1UzEU6YhxVRliSBTK-Wuby7iVk9FUn0iZt6WOeHh78OQeI6CxXXHsfOhLIjyUfK1PFxE0Yq4CRKtbsg

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
✓ Inserted test record: itE5C8QpLPDXpg5SG5tD4_FI1ZtKfM0AjeyvXBP55NnaQmFMgK9OIPEmg9gOQNBAem1pd6zdydQh1FqXZllCuA

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
✓ Inserted document with TTL: 0hq_WEPYQ6Ix_QOcBrGyK-iWymOGSX-DFFOD75KS62zcVe_-qBb6cywVKsCZ1kEvUlBZzrXcRIoMDvuY5_w_SQ

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
✓ Created session: dK0NRUA0xwpy_ivOfHCm8s-5WqbOZExeP1OfCFdBE0ppg2FqW-JHQSigM9Rpbfpflxxz_who0vnNkxqCSkAqDw

=== Sending Chat Message ===
Message ID: AIFRv2kaCb3GmghRrGc4hJQA8zbPxnQSpu-2B-8VvU2L61qu_wgj2P06B2RI8EFb51X_bIzeHdGBlUYeynPx8g

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. Its price is $499.

3. ekoDB Pro: This is our enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'ijip-P5RYNTX35qIDnynTl5KL9kpDxtwJkZBkVkKz8uVb2EAdxt0eOCbgDMmDHJ6LTB8PH5QwMUdywhyArJuNw',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    id: 'sMIKC5gmd0o-U-LXOsGLTcIaFaAXxFU5DQid47RIhaILdqCZMrUNzM1yGuTQtIzuQrMNKkwZPw0NO01VWJhtCw',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    id: 'v1tLldslfvWMh09SND7Hi5Bu89FVxn6b6nDViMz2RRfvMJRKJGAugrpg1c4fTUBUGpZM0QGkA5SR60ohtuGqEQ',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2411ms

=== Token Usage ===
Prompt tokens: 603
Completion tokens: 79
Total tokens: 682

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5l_HWgoxkaBzwmKHcXjc_osi-AlzwmevrZSEosXKN3iIju-TjecRA6alzUTDCO-adCB5qaWgUpYcQyWZExvPGw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, we have a product called "ekoDB". This is a high-performance database product. The price for this product is 99 (currency not specified).

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Dbp_EzQ64iQLs9dcsRriyNAJZe5vWoWEV7zqY3vb79vmG6Oo6Cm8C6D-gwio6YIHZdzBwyj2-iAYpyjKHTepXg
  Parent: 5l_HWgoxkaBzwmKHcXjc_osi-AlzwmevrZSEosXKN3iIju-TjecRA6alzUTDCO-adCB5qaWgUpYcQyWZExvPGw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: Dbp_EzQ64iQLs9dcsRriyNAJZe5vWoWEV7zqY3vb79vmG6Oo6Cm8C6D-gwio6YIHZdzBwyj2-iAYpyjKHTepXg (Untitled)
  Session 2: 5l_HWgoxkaBzwmKHcXjc_osi-AlzwmevrZSEosXKN3iIju-TjecRA6alzUTDCO-adCB5qaWgUpYcQyWZExvPGw (Untitled)
  Session 3: dK0NRUA0xwpy_ivOfHCm8s-5WqbOZExeP1OfCFdBE0ppg2FqW-JHQSigM9Rpbfpflxxz_who0vnNkxqCSkAqDw (Untitled)
  Session 4: 5vwqgmmAnv3Y-yS838JrSTCyWsphUC6GJk5-CZGtvDfoJJi465Tv8LranAB4M2-uz91CF6McbzYnfEJnizDGDA (Untitled)
  Session 5: c0bJRkvhyl0UhgpqjcpLeUXm6CMmPku6setuwlybdrediS41ssgqp78SPrtXuwizD3T-K6F7HG9nP2HqRrjGag (Untitled)
  Session 6: qoDX-6X-EBfv0Do0vMORjFhGHVQSswgRqwdUUCMAN2x5yVQgmQOYNWyMtCGoHUqEe4PlLZ-SEg9I12QvmH0d6Q (Untitled)
  Session 7: IrEDW5RVJSGcKFA6HYlzC3Jo45l1VQTq3iAf0nUXBShA7QxYasnCKR91wfvsZWK5sehGUT0fSMyCS8_ck9yeIQ (Untitled)
  Session 8: Qbq7z6JqiL85ZJejuPI55MMKxTTGgr7gDxB3N_FcD6e2OvrOabfKIMAFkSLjuM60ApEL40ENOiyaThC9cTF_uA (Untitled)
  Session 9: rtWqQ91ZdgT7AU06vsKV7rcjVfvC8s_aekKGnhDsMuSXt3hPn3DbR2FlL6POaqP5km1zzY-bcFb7-qL6A2CfNw (Untitled)
  Session 10: M5jfgFMLXAGf8KQ-Kc3oeMIk1LG1uVKrEIKZP_wh993kb2mGcllzluKQ5mV-o6qZU1qofEi4eQOSEA6BACYP8g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: Dbp_EzQ64iQLs9dcsRriyNAJZe5vWoWEV7zqY3vb79vmG6Oo6Cm8C6D-gwio6YIHZdzBwyj2-iAYpyjKHTepXg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 9U7sbvyy6pixbUyQAImyl73s2OMc1b3H9yjeB-IS8YPWuwY6hlJtycU0asV-gMdfs40M_vIV05zMlx871SQjhQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have a product called "ekoDB". It's a high-performance database product. The price of this product is $99.

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
✓ Created second session: 3-2qbejLZk46z8WJrPuDsmqYS9hCK-ZImG0OPF1uJfgPQdGo3jLAefh8EXfsTDw33muCYacK6Eyy6ARzWJo2DQ
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
  1. Score: 25.740, Matched: email, email.value, name, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, bio, title
  2. Score: 26.400, Matched: title, bio, bio.value, title.value
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, bio, title
  2. Score: 39.600, Matched: bio.value, title.value, bio, title
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.752, Matched: 
  2. Score: 0.743, Matched: 
  3. Score: 0.741, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.701, Matched: title, content.value, title.value, content
  2. Score: 1.496, Matched: title, title.value, content, content.value
  3. Score: 0.297, Matched: 

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

✅ Script saved: Z1fjbFE4dcDRShA97-6axg_rs1nJJFMiXqaFowRHDaRZkJXHZsiqiD88m_27s3aRfbaJwMxOTBPDGn53pW35fw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: oLfHlMQu-M9EfMpML-etEStX666c-KUiInq_ShuI7k8x-0VQ2eCOLmqzKyGDZvieawgAWVwmKiMkUUKyWkL1-g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 4bzOyF7YsuezRW2-qiTRu2MVXvGM8kSQA7Yag0XsFrDFUUAGtt34XCVzRN7zbZfy24MQlyJsLftXO24ahK7uCQ
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

✅ Script saved: nUnr5rVsKZ_JbQ9dcg8pAIDcujL_F1iGSPvly9LOTYXgLZmsagmw_mX3SBT63FQXhEYlcXD6hGwfd4yjgf68Ig
📊 Found 2 product groups
   {"avg_price":474,"category":"Furniture","count":2}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
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
✅ [32mTypeScript client examples complete![0m
