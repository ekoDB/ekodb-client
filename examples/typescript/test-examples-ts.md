make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.6.1 prepare
> npm run build


> @ekodb/ekodb-client@0.6.1 build
> tsc


up to date, audited 7 packages in 826ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 686ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'kNF5PWkWtRlI30ab-QzdURd0kdDXD6DJD1Tr1kEaPx9gHFcSWqCdDlswoEC-0GVL3VYV7_lzcpVb9xfcAlzZ3g'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  value: { value: 42, type: 'Integer' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  created_at: { value: '2026-01-03T05:27:29.300Z', type: 'DateTime' },
  active: { type: 'Boolean', value: true },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  id: 'kNF5PWkWtRlI30ab-QzdURd0kdDXD6DJD1Tr1kEaPx9gHFcSWqCdDlswoEC-0GVL3VYV7_lzcpVb9xfcAlzZ3g',
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03T05:27:29.300Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  value: 42,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  created_at: '2026-01-03T05:27:29.300Z',
  active: true,
  data: 'aGVsbG8gd29ybGQ=',
  price: 99.99,
  metadata: { key: 'value', nested: { deep: true } },
  id: 'kNF5PWkWtRlI30ab-QzdURd0kdDXD6DJD1Tr1kEaPx9gHFcSWqCdDlswoEC-0GVL3VYV7_lzcpVb9xfcAlzZ3g',
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { value: '2026-01-03T05:27:29.300Z', type: 'DateTime' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  active: { value: true, type: 'Boolean' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' },
  price: { value: 99.99, type: 'Float' },
  id: 'kNF5PWkWtRlI30ab-QzdURd0kdDXD6DJD1Tr1kEaPx9gHFcSWqCdDlswoEC-0GVL3VYV7_lzcpVb9xfcAlzZ3g'
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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
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
Collection created with first record: HYV6-vvf-PlKh0BaoMWDyPU1Occ7nxqX5FYMYFRLgnn7NAQqBxXLFiuHAU6Bf3puUgBF7H27oncfrHG-KGprGQ

=== List Collections ===
Total collections: 18
Sample collections: ws_ttl_test,schema_users_client_go,client_collection_management_ts,schema_employees_client_py,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: e9adUdmjXFfHTDEshjUxK0A6v4kv-zYosky9P40JYq0eeP0bPXvE78PLdp6MlTuYmQHTstMz-Uiz7Ww2-sWMMw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: OGag60L9kxayK7Dz6xhZZC8BwA5ZZeh6z8hBiKZdm4kyQ86n6j5TKymGYt-SYs-TF1GZuaJVmRhYOF5I1jts0Q

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
✓ Inserted test record: v3lPsyX_odFY19uUOxrzHhPMXJBl9vzdh4sni-k9W69DhGGOAM51Sn-Qz_TxBE77dntbiB7WJIn37UT9Ec14Qg

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
✓ Inserted document with TTL: pUmZFoSP8grdCFvWnxjyiKzY3OfRkmaLvWZmqwT7izEZFjJaArzI7F2NN2S6prAdBlRA7mKxCSE9w6uBO-6u9A

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
✓ Created session: pdDITrnGlc2AA1r8vHiVWC4H4q8ee66esJLZiV_VwSS3wIb0pp09Ic4KLwP5LLUqvCGsxanygbvgG_kazNGnkQ

=== Sending Chat Message ===
Message ID: N_3G59vbqkLEZUmM0bOWXQyl5LzpuJHMRTbsI15Qit1JSSKUvfmecrp2CrF3T5-Kd2v67jP0nNHu1IoYTnv3yw

=== AI Response ===
Based on the information provided, the following products are available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'lpIjIjGRWXBmSSjxKsFng8KdLYexJ_Y0ModikJ_WwKZjNdRk4pmTsR11KO_uzxh6R6-hLBccZ1aXPnTu9U1ukA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: '9VW9rD1klIwzX7cnQPx-rxzbN5hXiw9f-UlHkZY4HK5BV7rpxUaNEoazOSCttI4EvfG0uZt_tASm2UhuNXYHIQ',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'aeE0pTP2QBcqp6wWtfiITyrakZsbdmqUck3F4W-7WrmP1nnpkRQaDKgzpb07MnpFdICPwrIj6rPGPOLzmfmNAQ',
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3035ms

=== Token Usage ===
Prompt tokens: 608
Completion tokens: 90
Total tokens: 698

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: UTay6yQdkYzsPM8WHQGqRzSlfngi3O3cgb0hnIy6CPgLxIw2ZhMlcRM203q4pMW_wiqqfCew5MqAGln-UrWqRw

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available based on the provided information is "ekoDB", a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: nEzN0QjAyzE69XO3uYBiH0P79hfdMzlPgcDwVy1tbRCGqbAZgWjd-MRWeGcpm96u0DYKL13gEzaISyNbih1bJQ
  Parent: UTay6yQdkYzsPM8WHQGqRzSlfngi3O3cgb0hnIy6CPgLxIw2ZhMlcRM203q4pMW_wiqqfCew5MqAGln-UrWqRw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: nEzN0QjAyzE69XO3uYBiH0P79hfdMzlPgcDwVy1tbRCGqbAZgWjd-MRWeGcpm96u0DYKL13gEzaISyNbih1bJQ (Untitled)
  Session 2: UTay6yQdkYzsPM8WHQGqRzSlfngi3O3cgb0hnIy6CPgLxIw2ZhMlcRM203q4pMW_wiqqfCew5MqAGln-UrWqRw (Untitled)
  Session 3: pdDITrnGlc2AA1r8vHiVWC4H4q8ee66esJLZiV_VwSS3wIb0pp09Ic4KLwP5LLUqvCGsxanygbvgG_kazNGnkQ (Untitled)
  Session 4: ElVzQrLNy8abDUqTzInc62FK1DQNQ5oWxkOaO6_uS9oZlfd78IvPRsDC01kg-5fKbgtzqgyw2gruwFjn6PhEBg (Untitled)
  Session 5: eLr7gqxll0XkMo0fngGviRakNgisqU8DCS85DaBsZs8CS6vga5l2NFW3o8tiPi6Hqdv_I52Fb4aGQ7L50MET_A (Untitled)
  Session 6: SBvNH3R38sKAL9Bc6P62j4zXLJyCdfX2OuFUr3FbjkmewR9MVcxDSE7U2FSqLYy7ramHAlvT9fpWecCMl0PwJQ (Untitled)
  Session 7: GcI_-JS9SW66hdDUtOx8VFC7p1LwmVDKuARk01Ave2Iv5cQ4tA5LKv4tnVsbWwxppCW-Vc4RbAEteKlXSvJ3aQ (Untitled)
  Session 8: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ (Untitled)
  Session 9: zzMnYpnbOdEiHo60NW7v27gF22aW9C9vI17DJPbTOj_dVJ8W5PrS1iXVv3DLxXuW_gp_SHv8N7h3_9yZz7UGpg (Untitled)
  Session 10: ufrTyD4PrTBEQptwzw4r9KilyotTBDWqwmk4QImGnlZfBf6Qx-Xc8uVTAkyswCKMpDmpE7KDxy8Iez2OfoOILQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: nEzN0QjAyzE69XO3uYBiH0P79hfdMzlPgcDwVy1tbRCGqbAZgWjd-MRWeGcpm96u0DYKL13gEzaISyNbih1bJQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: K2_5noSZyktyNQRGr6b_MZwl31wPER2Y2tyizviMDLmUwzF4bNewTGra1_pn3tL53oej8kzObSGzXIwlEArwIA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product in our database is the ekoDB. It's a high-performance database product. The price for this product is $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is 99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: lwsk8JG9Jgvz32hv-MNts3fZoTHIWFNy1Ba_Eo8yN89kIjrgxLNrSJsCMMPc-x530s9Gw4KximQIfz1mOWmrUA
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
  1. Score: 25.740, Matched: email.value, name.value, name, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title, bio.value, title.value, bio
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title.value, bio.value, bio, title
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.769, Matched: 
  2. Score: 0.751, Matched: 
  3. Score: 0.737, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707, Matched: content, content.value, title.value, title
  2. Score: 1.495, Matched: title, title.value, content, content.value
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, bio, skills.value, skills

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

✅ Script saved: XKPgbYfkU3cWsgLqekD_wocyjNs-SrEFp7k1R9KWTLRJ4wg9nxuh9Y62OF6dTidpJhl8WBNCRZaAe4bq2bYS9g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: __mey_4527pndd8Ku0xr5t0sA7y8En5gXUNHsoTZwpOKN1ovUvYmTuCs70sbqbQ_684ZHg2U50GIijwQnQ9eBg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 9vc5IbRA_DlrlC5yzkiDJQYY0jDZUReUd4d4YM3dZkC5dXbZcTAh4u4ZKWRt54iF3AEOlgANjeewA0nxrGMWDw
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":5}
   {"status":"active","count":5,"avg_score":60}

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

✅ Script saved: 5oHYFHBAv9_TJyWpUxYA2D4RVvir1jpN1UW_6TQM3ezDdBGl2GQhhJA740GHFDZ8dI_yFD5MxK0J17FJko5IAA
📊 Found 2 product groups
   {"count":2,"category":"Furniture","avg_price":474}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"category":"Electronics","count":3}
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
Created Alice: $1000 - ID: aUGaDpOLLl93omzAOin4CVIgDZV5-TP1-AykbZ7jH0VLCxGkLhIUQYcJywgYV4O518dfCL_bEDbHTLtWLxl6RQ
Created Bob: $500 - ID: 7yYg_OVntPnSZy1tSPsV99ojVzYKJz9Ogbf6cmgjKI21X7nDa9atOJvw_hQxzt29f_HqNzd2bpukuNZg4cLA5Q

=== Example 1: Begin Transaction ===
Transaction ID: 26b6f681-c850-4c58-acb0-3162f3445e2c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 56831a97-e9ed-4cea-ad8f-9f5ee4b091ac
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
