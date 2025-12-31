make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 876ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 688ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'TprlvND5ZqYFy5G9T_5XBCZLX01OGtG80kgoS4AzS1GabrrSsG0xJl2wu4eiuxw2InwXpwel9ESBqmkbH7mqxw'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' },
  id: 'TprlvND5ZqYFy5G9T_5XBCZLX01OGtG80kgoS4AzS1GabrrSsG0xJl2wu4eiuxw2InwXpwel9ESBqmkbH7mqxw',
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  name: { value: 'Updated Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'TprlvND5ZqYFy5G9T_5XBCZLX01OGtG80kgoS4AzS1GabrrSsG0xJl2wu4eiuxw2InwXpwel9ESBqmkbH7mqxw'
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
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: Bfzo-jFFZJG82d8kY2JqSq4Cm-3LFZheRU5J7Wnlb6ARx-Oir4EMcUKPphx2w9CMMiZvbtlVQBXbc9kiWTy9kw

=== List Collections ===
Total collections: 18
Sample collections: schema_documents_client_go,websocket_test,ttl_cache,schema_employees_client_py,schema_products_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: RZGIcq2CiiIQVXXVrlM4xhuCKwsrStv_JqsZZfQ10pGXqQzTd1t6ZqWtjTWiI6WCzhRSZ49dexFCt_prqoW8-g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: U964hmaeJMfBMqgWcVT4Evuj3j_Hi6wO_anuVl_Pbfsj6VQ7h4WPrDl2Ru_-uQBlhh4vQfUNfp6iaOhUzkBNjg

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
✓ Inserted test record: 4fwDz8KeKlkIFUxWie6dHcS9W5G8HhUvFRmpZJR2kPlR_YnFGEWqVmakJCDyxEvOmS8YQfrbFkwj33UATFuWgQ

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
✓ Inserted document with TTL: aM3-h0D5FIKno-lC0VNBZu_idQCzRFaymbScHXGKVyLEjD93yDkEOrHPwIwzwot_5hSZskGZSAZy8BmSJD3nhQ

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
✓ Created session: tyjQSPLhUkrwuy7usi4J-ACs56k7JtISW1NKu4Mc01d4HdqAS5UtDhYgML4vZfENohr1kMxZe-cYyaKgyv-kqQ

=== Sending Chat Message ===
Message ID: Kl16o9mTM6QBl3fPeJTNMzxIQUQydgWwwKQry5I0PsKURJcQSi4jubnX2bdlaNTslGBwMisEJrrD4hCUCV_IHg

=== AI Response ===
The available products and their prices are:

1. ekoDB Cloud: This is a fully managed cloud database service product and it is priced at $499.
2. ekoDB: This is a high-performance database product with AI capabilities. It has a price tag of $99.
3. ekoDB Pro: This is an enterprise edition product with advanced features. It is available for $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'vebaNzUFFc9RWte8_6j8Y1X4u2sUAXIr0nqZTjjjgNXKbKwgMjd19uBksSOo15c6xEElr9C4QvwpmtW5RKgwQA',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'JFEfrStWoRTnAYIp_XHZaPxcFV7cBPX6wt0GfON17-xR9bD4Mj0Ld02ykGpAjtr04ecF2Fmkn7kzEUyywHsBEA',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    id: 'DlkCgptWj1sIj03dMvJZiIIHEiCC9aNJCowpV5bCB9-kY_p63TIfIhk60C-ZeuSqjFyUzOAgjB6vFEKQ4qrMiA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3300ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 82
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: TE0B1tmmDPi7xvN_2zBCap9AL__HdNuTlsKjDaz76LJRSRZpDD26mE2I6II1jdl38xe6PHv29rZ104cGauOIlA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is "ekoDB". It is a high-performance database product. The price for ekoDB is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5qDuAePdPKipmjsIsZ5ov0i7-dEuGLxWNONFXYeYtYqF2NjChjGGdj0BAUfyS4u6lYCwBscAKjCwnyosnbBmcA
  Parent: TE0B1tmmDPi7xvN_2zBCap9AL__HdNuTlsKjDaz76LJRSRZpDD26mE2I6II1jdl38xe6PHv29rZ104cGauOIlA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 5qDuAePdPKipmjsIsZ5ov0i7-dEuGLxWNONFXYeYtYqF2NjChjGGdj0BAUfyS4u6lYCwBscAKjCwnyosnbBmcA (Untitled)
  Session 2: TE0B1tmmDPi7xvN_2zBCap9AL__HdNuTlsKjDaz76LJRSRZpDD26mE2I6II1jdl38xe6PHv29rZ104cGauOIlA (Untitled)
  Session 3: tyjQSPLhUkrwuy7usi4J-ACs56k7JtISW1NKu4Mc01d4HdqAS5UtDhYgML4vZfENohr1kMxZe-cYyaKgyv-kqQ (Untitled)
  Session 4: EoxFsNGvDGoht1Q_yKy8wRxk6vja0w6JhYWhzZrain-YcVDqpDEU6v2mJoT9pNfcSopyQbQVaAcUDJECztvOhQ (Untitled)
  Session 5: CssK2C9NwDqB6gnxHjQmOEWnlPFpUh7XiAwg7Wvu-k6ZAZLAZe5IkReQusVCWlsU96NjKPvgu-n6XzcfI2kidw (Untitled)
  Session 6: BFktfJOMJ7YljgfQGEQDsMgJIZ-0uhKET-2tVlgR9fOhkzOHWvpdYvlxe2DZcxoZd1fmY8aweNcjmyR3ZbNI-A (Untitled)
  Session 7: w3uwW8-60AB64JP2kQHBpGnrWGOXfXp289TQ8dj3GhYDE9QI6-wPz9Ctj-OMc9v_ttISNovA-FnkX8mqAqCyCA (Untitled)
  Session 8: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw (Untitled)
  Session 9: JehJVzy0fGzjb5QdDQEKy5wozk9uYujxIkjq0V5YTjSRI0ad3j65tSRgHdY1djrSCKaYCyqxoZ0QuX97tozN6Q (Untitled)
  Session 10: FCexZoTUlOX6KTMmjH8vgzqs3v-JG_bq367FxD8kJTuYbPFS0SQ--AfP1uiO3tCjYL3IFjfW2XJv984i2JMbig (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 5qDuAePdPKipmjsIsZ5ov0i7-dEuGLxWNONFXYeYtYqF2NjChjGGdj0BAUfyS4u6lYCwBscAKjCwnyosnbBmcA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: h-c1tRfU3JxVU0Yr00lcyT10BDn4NYC3nCDIceXpqwWmpQuwiMlumXt2Og8gjZgeKkvtg9lUPx20V8rwX4pfRA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the available context, the product "ekoDB" is available. It is described as a high-performance database product and it is priced at $99.

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
✓ Created second session: SYGB6sQ1IYqqkigD9f7pPdYxL6NQcPSn80b89Q0DIvHyTL3FNly2qLQ1q31a4xZdPvPtNT473MpSqRhsIJxrfQ
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title.value, bio, title, bio.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title.value, bio, title, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, bio.value, title
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: bio.value, bio, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.731, Matched: 
  3. Score: 0.719, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.708, Matched: content.value, title, title.value, content
  2. Score: 1.488, Matched: title, content, content.value, title.value
  3. Score: 0.293, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, bio, skills, skills.value

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

✅ Script saved: 1iuG_Dy8qAo-JnGTJFLe8DPz_AQaNWuTWjKxWoNm3SQp_Duljm9GWv0taZdXDGKbq4QiaXgs2YAt1sRMY805cA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: E-vv9T6zVKPHGxZTkm80F1CRYarPIO7_eWjjVqXeQtHDcQY9YIh0HL4YtD7HHfUH0GttLvp3M-Q-owczbQweLw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: UYZ0v10iWx7fykOm-QRK3TKHu-aiLJvNIsKgUl54oGjXcNCm6SgA2O-FgxDpZKQgs7hSbfQs-bQjsmungi7F6Q
📊 Statistics: 2 groups
   {"avg_score":60,"count":5,"status":"active"}
   {"count":5,"avg_score":50,"status":"inactive"}

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

✅ Script saved: MNXERYth0xZn0P7Lsq3GWCNOFA3kBz5dlf_snzER25f2I91DSgYJ8Yjg0ePN-Ax3smsfyf2xiftck4V31VW3cw
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"count":2,"category":"Furniture","avg_price":474}
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
✅ [32mTypeScript client examples complete![0m
