make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.6.0 prepare
> npm run build


> @ekodb/ekodb-client@0.6.0 build
> tsc


up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 648ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'MKGAmFE3x59kRkQMOL4RF2h7ohxwaBtLQJjuIUjBh5ImXEDn1af2CdtkPCW7kRbBiPkUYkLksBKGtf75MNF3fA'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  id: 'MKGAmFE3x59kRkQMOL4RF2h7ohxwaBtLQJjuIUjBh5ImXEDn1af2CdtkPCW7kRbBiPkUYkLksBKGtf75MNF3fA',
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  id: 'MKGAmFE3x59kRkQMOL4RF2h7ohxwaBtLQJjuIUjBh5ImXEDn1af2CdtkPCW7kRbBiPkUYkLksBKGtf75MNF3fA',
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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: QehE7YjAw0wf_85NiYRDi33H2Wk45Vs1wSLHS84NK_AxfERh6WORwqREOc8ZmTTUiMF3HW-VigxZn_0y6reNZg

=== List Collections ===
Total collections: 18
Sample collections: chat_messages__ek0_testing,schema_employees_client_py,test_accounts,websocket_test,schema_products_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: KeVmFYriqakjM5i2lJF3WB7Lxp4hytDJbyFCmnC5EmR3ArqUsEDvPiJkMNdQ-ZHkrsFc8CDOA15SwJxva1vxPg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 5RuBjbqot9z4TF95HexHOLwQhHawaMJ17rlke9ED_NCsPbQ_sp5v8uGoFil4pABKVo9LpaUE_otxpYtoiore6g

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
✓ Inserted test record: hi2eEUJhhKVmGYgfTOb2DCsnDoCbyeQwTSBlzJQeEBd7P3hZwmmeVYSpuM4Z_1ZYf5UY7MPtNr1VinwBPG0z9A

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
✓ Inserted document with TTL: MP_KAG4rfZnRJz1lVnZWbolEBOpourswrT_pQvfFa_WVHd5GbLGc_YCZhDMERXPVU7tZBX3CxOidDEpeRtNP-A

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
✓ Created session: lK1vgZjE1NSCHPntMNQKzUOyqNMXqMje9tcMX_ktjRsIkzYqcm6kR4q7cc6YzRQFLr2E4hSBSScdWjU0O8bYVw

=== Sending Chat Message ===
Message ID: 16DiwEDp4PAXscGk6CM5t7cDrsa8-pbwRFKPM4821Kxr8DaeQUMwz6qweP7SUEloewQrTj_nL0Lu15pAOedlgw

=== AI Response ===
Based on the information provided, the following products are available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The cost of this product is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price of this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    id: 'Br-K4AH9V5W_8Z5PiBFVcrq9dbAXOZYo-QVu0GZ37b0m7Df_ZjmoUXzLV6XBBiZUmBcKhYncQCz5r7OZ-tdWtw',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'pV4--A4OM3EgHZdqSStzUnBsb74c72jav--56FL2lyqZl4lZ0Lbv3O5Q9qIVbRyioTTgadW-OuB3XFFQomlAXg'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'qOf1X_TgdwdULBDqPaLTCeIPemcnTo4_wgzzgHIyNmOi__n8etEomy1TTicCvnVjU75qOYAvaIrWYjsbuzC9Jw',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3301ms

=== Token Usage ===
Prompt tokens: 604
Completion tokens: 90
Total tokens: 694

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 7zocUzCu9NmjbBgTfAuj3jLptl5_dudHfn0ehExuAT4iTWJdWxvhbdx-0myhkvLR5xYmckZcY_Zi6-486Yt1Qw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is "ekoDB". It's a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: lA3iDtH089AIQ2tmQvHDr0PjW4oIQw7cDv6M2bTI51je4KABzzuFk2INvr0tYoohAWTl1kRFkHflNH1T54Xfyg
  Parent: 7zocUzCu9NmjbBgTfAuj3jLptl5_dudHfn0ehExuAT4iTWJdWxvhbdx-0myhkvLR5xYmckZcY_Zi6-486Yt1Qw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: lA3iDtH089AIQ2tmQvHDr0PjW4oIQw7cDv6M2bTI51je4KABzzuFk2INvr0tYoohAWTl1kRFkHflNH1T54Xfyg (Untitled)
  Session 2: 7zocUzCu9NmjbBgTfAuj3jLptl5_dudHfn0ehExuAT4iTWJdWxvhbdx-0myhkvLR5xYmckZcY_Zi6-486Yt1Qw (Untitled)
  Session 3: lK1vgZjE1NSCHPntMNQKzUOyqNMXqMje9tcMX_ktjRsIkzYqcm6kR4q7cc6YzRQFLr2E4hSBSScdWjU0O8bYVw (Untitled)
  Session 4: 8Smsmom8pjVYdUP71Gk8rl1718m-bz1nU6XQAhrym_9JEzS93wLb8F4WJj2lZNmy2ODc5AOevjWB7nsXfC1AzQ (Untitled)
  Session 5: uG0aZNV03fdA8F-6QPZsbDlooeIPE9qSmbi36dJenCU-ZuXi8kB8TgCG-o2HqgNdifFgXJrkVQs1qHzfM8H01Q (Untitled)
  Session 6: z_7GlQKFQxUqZgud8O5V6t2cg3Hy7zRwtLJScBJKIHkYz_5kO1_kr2g2VlrUXZxXY57NZ8lIMjiyZjlVUh6ViQ (Untitled)
  Session 7: IldEnBWbEmtuzluqWP2jIEkU5fEl-tkgvYm_Ws_4Txx-zvPQ4p6OrzVY6M-ejEo1hk1UOA5yZ1ceJxqIT73qgw (Untitled)
  Session 8: 098hQxXsSWrfeco-Rh28tx65AW2CPFqfirZRpCKkdrdI-eH8o4gZ9XROWCgCRUEiFNQphpO4aCi05xZJ_pgJMQ (Untitled)
  Session 9: KQOalY4bPJPqNB915OmVLg_bRHONknf6mt2pN34QtbgXg1de6yTko9Mp822in1Pivy0H5-NxZjwyBR7wcoMXTA (Untitled)
  Session 10: OUSEtgQ_-7kKWGMt4AmVYrdXPPGik3263KCnNt9ueJAKP1k4kK_PUO1YW4JlGYF-TG2Tmcwoh-qhMRAInCKHkA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: lA3iDtH089AIQ2tmQvHDr0PjW4oIQw7cDv6M2bTI51je4KABzzuFk2INvr0tYoohAWTl1kRFkHflNH1T54Xfyg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: X56FKkeqkgUyGqRBx6i65aIcu074V6AvDtCfCa-PtTMkmaFmWnCkMbq-cd_3HZq1XlzioIj1aO0qYvntPHNXfQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is ekoDB. It's a high-performance database product, priced at $99.

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
✓ Created second session: jUjUpG75oMAxFkIaW6l-cCwBZyd6tRaDY7xykxZNZavOOFftUgyjbRzNrf-Q5ahgpYYHZ-vbO5NqVlMBW_OFQA
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
  1. Score: 25.740, Matched: name.value, email, name, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio, title.value, title, bio.value
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: bio, title.value, title, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: title.value, title, bio, bio.value
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: title.value, title, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.767, Matched: 
  2. Score: 0.756, Matched: 
  3. Score: 0.745, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: content, content.value, title, title.value
  2. Score: 1.498, Matched: content.value, content, title, title.value
  3. Score: 0.307, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills.value, skills, bio.value

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

✅ Script saved: 8ivg78waXdYcmv111YLfOhj-6nt8TJ_fOXOnBEybNPQV3YyGBn0losspdXgKUE1Jmbi3CzQcwI78gmzqssePvw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: kqwGOuc5b0JmpKieq6F1DUVmrbys57tvxka915Bd02Kp84zkbsvgLYq1_CRevMigzXLWuURkOftJjXczCV-3mA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: SS3CgNsOl8NkserzkEfBEmmQB41bELwRKhiqu_akPVWhGBAFct-gr4E0fKjcwQACFUbycM92uTV3QlrtnIlJHQ
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

✅ Script saved: VI_FiepLiTEw0OEknCUmD3o5Hh3Rz8VKv-6ssSxueOK6mNlEOmRvT28BivyBeRkO3GwTZCR4N_Mwx21rJaCJRg
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
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
