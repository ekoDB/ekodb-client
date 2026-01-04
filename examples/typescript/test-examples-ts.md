make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 825ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'wbnlv7IPVfQLzd4Oo90Toe0WBX_Q9-wdqKNHGwymPD2dfWjNq4Jmkxglaesx0GvhTMr2YXr6eBQ8qT1Z-MkZLw'
}

=== Find by ID ===
Found: {
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  active: { type: 'Boolean', value: true },
  created_at: { value: '2026-01-04T22:31:11.159Z', type: 'DateTime' },
  price: { value: 99.99, type: 'Float' },
  id: 'wbnlv7IPVfQLzd4Oo90Toe0WBX_Q9-wdqKNHGwymPD2dfWjNq4Jmkxglaesx0GvhTMr2YXr6eBQ8qT1Z-MkZLw',
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04T22:31:11.159Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  active: true,
  created_at: '2026-01-04T22:31:11.159Z',
  price: 99.99,
  id: 'wbnlv7IPVfQLzd4Oo90Toe0WBX_Q9-wdqKNHGwymPD2dfWjNq4Jmkxglaesx0GvhTMr2YXr6eBQ8qT1Z-MkZLw',
  value: 42,
  name: 'Test Record',
  metadata: { nested: { deep: true }, key: 'value' },
  tags: [ 'tag1', 'tag2', 'tag3' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { type: 'Integer', value: 100 },
  created_at: { value: '2026-01-04T22:31:11.159Z', type: 'DateTime' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'wbnlv7IPVfQLzd4Oo90Toe0WBX_Q9-wdqKNHGwymPD2dfWjNq4Jmkxglaesx0GvhTMr2YXr6eBQ8qT1Z-MkZLw',
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
Collection created with first record: nk6M0DROdHaT11ffY374uQsLhqjMtp6KL5f0dhCzajVpEh3IGldy69-YY0vCQbJEyOKtVEaQjs2NRRBwtL9chA

=== List Collections ===
Total collections: 18
Sample collections: ttl_cache,websocket_test,chat_messages__ek0_testing,client_collection_management_ts,test_collection

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: HtNhBt4EA_Us2yXUZrSCAJQe7fFzFBxPDe0HgoQnP9OW3mtQHal5hnY2y9RdVmuDTLOys044bgzXFnY9Q3LBOA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Xet_sVTaUa4fDct0XyvXmLOQ1iYY2vI5ikpEmgEvMEYzvDwK8i-MldxaQl_Wc2HCIF1E5cYP1NuKtgXsDq9GHw

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
✓ Inserted test record: FmVg0_o_2Ct2pKSFBJb6pBB-1nZkSV2-nektxWPjVysHQJgBZVvJfhr-NObq5c2RkH1Dersc93XjcCNCvUQhig

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
✓ Inserted document with TTL: RZH9ayJlEtO1U2g9X8oexgkYnKXZ0B-qyLwl6oPWmGjISDC96-SAVN23ZerBr4r_bhHGPKgOjHwgZ33L-xv4BQ

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
✓ Created session: k7sW1byJTicQYj0VGGVxPgDYMIhyRWcHbXxcasr2-4ia6FiATCUPrkmgSr3sy9XBd0edmgaMTUOkgHhVPmarFg

=== Sending Chat Message ===
Message ID: 8zoJ9Bmpvd-5vFnYMEjocAqzfWh-lQz7kRFC4tjesIQ_VSWd9zsdNX0ZsjfF5moJrtV5wKW5suPb2YEk5Bt4NA

=== AI Response ===
We have three products available:

1. "ekoDB Pro" is an enterprise edition product with advanced features. Its price is $299.
2. "ekoDB Cloud" is a fully managed cloud database service product. Its price is $499.
3. "ekoDB" is a high-performance database product with AI capabilities. Its price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: 'xUSs7eSBeqWWhEsU10Ic_9ckdDyrSA5rbJrhAZXjowTtG_tXKq2sDm4T91S2Kd21upyxGFlY-FeVnSWsiyeeCw',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'PsDmy7UkJSGOKu4Uk8UENjBGDP49ecC-5lc-yCacxXBK6Wyh5_Q_p4ZgNS-ejBBeUknw2ak_HlEN6K5jXVlCNw',
    description: 'Fully managed cloud database service product',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '4KUxTbp22Q9jpHMAkNnzHarKJQRhh3YRAUHOSpuPWmwQZpG2dmL8EG7Drdq4-y85MD4W8XXjxdZKlJAPcJ7I-A',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2367ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 72
Total tokens: 682

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5Iga9LK7In9cz8yTSFeG0tRBNbckjkFWiXeq2Xv3uH0pgjnB21Z7tHAhaFfSsUs4P0PCzKq9whyY3jewFh-fjw

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One product available is ekoDB. It is a high-performance database product and it costs 99 units of your local currency.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: w9nJCzvongrZC33ecIgcoT70258Eh1UWGJHZBbdGPwvJGIsJ0CBeOG1-wRpek3t5SLuSjrpqgPSd0PUgj1bfqA
  Parent: 5Iga9LK7In9cz8yTSFeG0tRBNbckjkFWiXeq2Xv3uH0pgjnB21Z7tHAhaFfSsUs4P0PCzKq9whyY3jewFh-fjw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: w9nJCzvongrZC33ecIgcoT70258Eh1UWGJHZBbdGPwvJGIsJ0CBeOG1-wRpek3t5SLuSjrpqgPSd0PUgj1bfqA (Untitled)
  Session 2: 5Iga9LK7In9cz8yTSFeG0tRBNbckjkFWiXeq2Xv3uH0pgjnB21Z7tHAhaFfSsUs4P0PCzKq9whyY3jewFh-fjw (Untitled)
  Session 3: k7sW1byJTicQYj0VGGVxPgDYMIhyRWcHbXxcasr2-4ia6FiATCUPrkmgSr3sy9XBd0edmgaMTUOkgHhVPmarFg (Untitled)
  Session 4: NZ3y677q6jvHnfJfB7cTEZQef4i2DBvbBXefm61pJxn-HBf9p7So7p4zJUjILngY864c_I2FcDV6_j51na4qAA (Untitled)
  Session 5: Gu_xIGD5MefdoNTeJDkJ0I4d3RSsP9ER4PGpxn03S00E8P1XDAGDW9F6VOqwiNH5nVOvIOO25KjqjHwrAQDw4w (Untitled)
  Session 6: Qyvsy0PzVbiBeghb9hUiKkm9jkqwn0zqoSrh-sUl0FxvsU7IdcSFGYFMzXIP0bqVZJLQrKxGOYxDiL50740Fuw (Untitled)
  Session 7: Io01vXhsW8BkPYUCUUTLdDWRPT21aGynTNwl8dKLlJ6X24xSeN2mBCOkKES2IJnJ9F6h51vrnyjWWQLYmuyOMQ (Untitled)
  Session 8: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ (Untitled)
  Session 9: LJ5cZyqHoXs52cGWUxESsBowN9WD_4okFMLRbpD8-ggFUhYqKZPFaRlUDCmTo8J2LkXgnMPzPinMwwzCxLmR8w (Untitled)
  Session 10: zgZmlmVFLWGNLEdEB26uHagZbr2ZF7802mz_wjqfZqon0azZ3gofLs6k7o8B_odODTl89yzN_di8oFEaFf0y8g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: w9nJCzvongrZC33ecIgcoT70258Eh1UWGJHZBbdGPwvJGIsJ0CBeOG1-wRpek3t5SLuSjrpqgPSd0PUgj1bfqA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wepjtZ8JUYkWErNBcJsUilMOTat_9Mhm6JrvNrekcxl5_rEHSE9KBA-tNZytQB0-_YigHneVXXHAO3rPdRRxag

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, "ekoDB" is available. It is a high-performance database product and the price is $99.

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
✓ Created second session: V29yi8ko2by95UxL-GMdsvrCm7M-ixzOUXY4rfI59cL85dNf777_dbogc3M-f-C_TDnYWHKtfjL75z72NzwOwQ
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
  1. Score: 25.740, Matched: email.value, email, name.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio, title, bio.value, title.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.765, Matched: 
  2. Score: 0.752, Matched: 
  3. Score: 0.752, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.701, Matched: title, content, content.value, title.value
  2. Score: 1.506, Matched: content.value, content, title.value, title
  3. Score: 0.301, Matched: 

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
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 3-ZQD-GsVD71qHmxaBrHSzkI4Il2hrgmvCbGYV_MH7sSVpoNqY2Hu2FXPTsIuKp7qkZ14Ijaerwd336X1zdBZw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: CIXxeDBWYb0CH8XUcF3nB5yB2Bqz9eb-3nrzz8YaUa5UBOmA2JHvJMTyPFiiJKNLhHaySOiaNGGBs5nIw7Fq8g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: aV1PY6O9PCxQ1rdPnkzVHEixSL9nkXU_uXJ201YEs5usECCj4IWAYlJ1A-teKQDG78B6OTenUmInakuiba3S6g
📊 Statistics: 2 groups
   {"count":5,"avg_score":50,"status":"inactive"}
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

✅ Script saved: rdfpirVtpfbAfB4KgPMNyx4tiwjdnvNmzea2B5eZUxjnTf0Z8Pphd9OQuZo8R41yLrXpICnoyNpMLCPwTlcg0Q
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
Created Alice: $1000 - ID: I0Pz-f5IduXtsG2am_2uJCXAiG-EUKDTrdsVX-IEGPUE7f7NU013c-ihNcKEgxc5eTUgK892JLf7frJCdWs5Uw
Created Bob: $500 - ID: JuR5-QzLqokyOWCi-cPC3IIFdm9Fy4QcRpwTTVapLgik50oMYPMpZ8DYRuzcGPgVCy0ip9sgC7d-k5x7fs67_A

=== Example 1: Begin Transaction ===
Transaction ID: 115d991f-2c5f-40d3-9767-8556ab840b3a

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: d695ad83-3276-49d4-90c1-015531d704bd
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
