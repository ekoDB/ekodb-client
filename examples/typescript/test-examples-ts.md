make test-examples-typescript
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 886ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 790ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '7BuvcTK6tCPvtvHr8bI5zimkSnKoLY4DBXI_gEOuEMHvtSHu8DOg9dkYRCQWg7T4J4bw2o8TQhmCDShn7mEOEw'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: '7BuvcTK6tCPvtvHr8bI5zimkSnKoLY4DBXI_gEOuEMHvtSHu8DOg9dkYRCQWg7T4J4bw2o8TQhmCDShn7mEOEw',
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  id: '7BuvcTK6tCPvtvHr8bI5zimkSnKoLY4DBXI_gEOuEMHvtSHu8DOg9dkYRCQWg7T4J4bw2o8TQhmCDShn7mEOEw',
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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: TMx48sRDwHogTHdds2s9KRtFGqbu921DS6hlVe-0aDVUB8wvRxlSSMgxrUE07uRgh2uYHEnwphhUkvycTaAhPw

=== List Collections ===
Total collections: 18
Sample collections: schema_products_client_go,test_collection,schema_employees_client_go,chat_messages__ek0_testing,client_collection_management_ts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: HOKDoMjn4LYz57b81OmlVsE9Y6UbwG8ZHkXXipTMb5L0mrRpAQRzFmZZVR5SJZiw9MZQQorwLsC0lEdL5tqm5w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 6s_71rrFZsulS2nBA-9TjDXlQDHHwXEoQgJvQkVUyp_TIYNT8x_9YeIiq2Bj4BxybdocMqljOAZACE-_Wy-kgw

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
✓ Inserted test record: BT3xBqbQITDFtoNoNHUOfLPSIqOIab67k-nyP9aVSY9nmUJZeLBH56M1F6MK4NlCVCBVsrFel3PmWgMhfcVqnA

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
✓ Inserted document with TTL: mc2q7q-OfmTRCPfpG3eKeyhTLiOMnAjuJIEQNaEXrCU_xVgOBZ_rvZHCwA8dLz0Vsc8q4lWnXHA4YUhxudECCw

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
✓ Created session: EcfuSEeAPKm9boQ_qCcTXAPrdYySTZP5BFOF-ROwtw2VNENWetCHIf89czWfJfBY7_cu-fYnZvtXxSxp1mI2vw

=== Sending Chat Message ===
Message ID: j1zV82txU5K7vRFkEefuKHV4fOlgBqn_bxcQr1-vOmfO3_bIUJZZPAft5o6nZ2iLoAIZ7UNAwMTJrcj6fn1Y3w

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price is 299 units.
2. ekoDB: This is a high-performance database product with AI capabilities. The price is 99 units.
3. ekoDB Cloud: This is a fully managed cloud database service product. The price is 499 units.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'tssPWTB_S0BSkhAjZkIX2CbH1s2-dpg5FmS-AX8UK49shS5QTxiTIVKCyQz6axS92WTwAhZlG0XIxXF5jDW-ZA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'p3N6fUUKc8WDIW7kTNX36ZE2Vwo_5kT4sa41OvCpaq58XR6yefiAQI3lh8D3ilExZ-bq1bTZnnp4pOvR-bOgtQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: 'De1cwxeYjbHw-AQ9YD5Piv4xRUz1VIaRCJRFTwPC4tot4LvMgiNqH3q6wrHC5DhxSLekT_wk6g-GkNVqWVb8Mw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2970ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 78
Total tokens: 687

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: f3cqw3oxlUv6OBEomautzumrrpf5HNMYsFmODEtlquVqrBk5NfYi1DVUm8muA_djECAEFz2HZoxC-hI1PMZfBw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is ekoDB. It is described as a high-performance database product. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: EBz573b8e0AAiWzHaAYZZ9PP-L2EUv4ZzS6vCZpGcxuGG182MRu80mJd0FYEj_uRBFthIBNjv3I7zoNOMwW7uA
  Parent: f3cqw3oxlUv6OBEomautzumrrpf5HNMYsFmODEtlquVqrBk5NfYi1DVUm8muA_djECAEFz2HZoxC-hI1PMZfBw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: EBz573b8e0AAiWzHaAYZZ9PP-L2EUv4ZzS6vCZpGcxuGG182MRu80mJd0FYEj_uRBFthIBNjv3I7zoNOMwW7uA (Untitled)
  Session 2: f3cqw3oxlUv6OBEomautzumrrpf5HNMYsFmODEtlquVqrBk5NfYi1DVUm8muA_djECAEFz2HZoxC-hI1PMZfBw (Untitled)
  Session 3: EcfuSEeAPKm9boQ_qCcTXAPrdYySTZP5BFOF-ROwtw2VNENWetCHIf89czWfJfBY7_cu-fYnZvtXxSxp1mI2vw (Untitled)
  Session 4: EB4Z9QpAgykYsvdkcb07eqv0-Q3ZRYPpC_ElC-i-1fKANvdGOuiJjA7D77TFhWuN5tRHAGcJcQiYD8Bv8xSudg (Untitled)
  Session 5: odv6MLCuEtIj3NUV-uSW6TTFYvnBemkIBtIuafULJVlO8vS_Zg-LszGuoGQcik32p3_p1cfQ2b2uCpQwXpfVIQ (Untitled)
  Session 6: 821H3iSPIbH7vew7_PwAtAC9ISMwvMRCdEty66oC-5tcpXoP4JTJmI0I9vxK_eBCrT28f9HnLWB3bWMIGxHKrg (Untitled)
  Session 7: rPOuQWVT2njcPDiEPMECJJCqn_dUz-waRLpPt-MeiwbHxylvroUkBcOh8x-E9Voy0mCLjYHyGVIQIQGvw2Fp1g (Untitled)
  Session 8: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw (Untitled)
  Session 9: aoe6MEuL3E1EmuO5qnfz0zmj9UCxpmR5TGic-qgUuC5Tr0b6ynRYl1hobYCeqXxu4wWURBibPcRgbY8-SCF3RA (Untitled)
  Session 10: Y9IlcOV2FYlrt19uHBT3nw_RiNqIdVs6MtG_7rOFYFLhEOfn0b8eJ3leVDMrFWKw2U_9gaJhiyPp1eiGoMGI4w (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: EBz573b8e0AAiWzHaAYZZ9PP-L2EUv4ZzS6vCZpGcxuGG182MRu80mJd0FYEj_uRBFthIBNjv3I7zoNOMwW7uA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dpeT26YlSy6FwuHGUz5lLHXvESg1rJHejtiGT8QwrLy7ILa5TJbwJFcu6XwpHw7lplMbJoqldD4tmVhVt_THyA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one of the available products is "ekoDB". It is a high-performance database product and is priced at $99.

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
✓ Created second session: mLTVcPRJ94iVpBTu-9gVST_Sjqr_WugLWph97L9WTgQpABTg92TuZLeACZDL7W_NYYNEdcUs7iUNd0qWMTJbaA
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
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: bio.value, title, title.value, bio
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: title, bio, bio.value, title.value
  3. Score: 39.600, Matched: title, bio.value, bio, title.value
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.772, Matched: 
  2. Score: 0.769, Matched: 
  3. Score: 0.760, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.709, Matched: title, content.value, title.value, content
  2. Score: 1.504, Matched: title, content.value, content, title.value
  3. Score: 0.308, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, skills.value, bio

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

✅ Script saved: W3vyzCiZzdC4oyvALJNk7JpDBXlQHI7F_uKW3l7w4JToQWfVevTYu62fu5v__6gGGd_hJOMQzOVFIIMsm8m33Q
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: zPn0dKIkooLp1gy7ko9tX5xVbxjb16hwpzUus9FE1aIVJiDeclkoFSQa-BvollaqgyMF6S_C9rWzwLizv-ctuQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: N39QKyGKRJ6-qvtyXwj0woM17Fxm75MPNXTraD0NH0h5B0MXIACfNmoelYitUROh9LlFZI0bRMamek5CpaC-4g
📊 Statistics: 2 groups
   {"status":"inactive","avg_score":50,"count":5}
   {"avg_score":60,"status":"active","count":5}

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

✅ Script saved: M9uyYUdOAq3xhfxhiTbKjGug9BTY59GPXcculM8UiF6P_2dPSggvt8KJ533MDbUMrUTjrML5ndeZuYmwqNu9kA
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
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
