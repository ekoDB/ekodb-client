make test-examples-typescript 📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.4.0 prepare npm run build

> @ekodb/ekodb-client@0.4.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.4.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 641ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'mNXDtq_O56G5JgxDURUKp_THNkyxEhoHZ6ZiX5svtDrULzaDK1mziJNMMR1siAp0uWO3Y5auf3OkSFE3ZdJS4g'
}

=== Find by ID === Found: { active: { value: true, type: 'Boolean' }, id:
'mNXDtq_O56G5JgxDURUKp_THNkyxEhoHZ6ZiX5svtDrULzaDK1mziJNMMR1siAp0uWO3Y5auf3OkSFE3ZdJS4g',
value: { value: 42, type: 'Integer' }, name: { type: 'String', value: 'Test
Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { type: 'Integer', value: 100 },
active: { value: true, type: 'Boolean' }, id:
'mNXDtq_O56G5JgxDURUKp_THNkyxEhoHZ6ZiX5svtDrULzaDK1mziJNMMR1siAp0uWO3Y5auf3OkSFE3ZdJS4g',
name: { type: 'String', value: 'Updated Record' } }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
FCfORvZic6GzUM2chbEgYKZoHETZchIiyOPvto2v0pFD07aJhPd-7DP57tlDtNrn_sWixABELEX_M1LUyJdc3A

=== List Collections === Total collections: 18 Sample collections:
scripts\_\_ek0_bench,websocket_test,schema_documents_client_py,schema_users_client_go,schema_employees_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
NMPRK-KpjcR1RipIx0-MKlqNAJAiW99WFB7MJ1gCYJxOAWcquYp7_EeVmB6\_\_WOI5TiP1XGNd5RNVWx5C8N3NA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
zxJ91zOWW76kXW7JjA9poUil-O_KbjvJFPcBOPt7YaPIZ0HozDnpwP38QR8_Od6V8ZHce4RMLQ927odziIABog

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
iYJgu7ge66cqydk_c8Hdiypi_iWSEy50Tob14RUl8685TnYKOcyIShjHVyRWlHfNOfiRf_8fpzhMnmEAeBleGA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
5Zk04fBZ7bIzjRlRdcNgxxA-as7obxyo8nWIe7zzFu0jq9K01A9FV5BTVYpORj1frcUcJBsMoZe_T0_e406Vzw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
SngZo3l4_KRCF_AHJMU2c6QrBn_b2A61Btx4CnhiPTKepSlxp95VdfMtrOQYBo7JAlO7NjqXbDeHkoHLKBIY-Q

=== Sending Chat Message === Message ID:
AlEmyvlJelEIBHQRPh2AarXZ0NIWayjcRcxmd60i2zdNrVTQuilAzLPQNFHwoSftwW_LYzgtMRH471XmhJpHXw

=== AI Response === I'm sorry, but the context provided does not include
specific information about the names or prices of any products. Could you please
provide more detailed information?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { name: { value: 'ekoDB Pro', type: 'String' },
id:
'ROJ7nld8cpEgVPjgwLNdW3LslcT7Rk5A7-L372j_zceb0JvgOtP6yVEtmr2Z-sxdTRUlGE8GDKGMvuY9-PYkKA',
price: { value: 299, type: 'Integer' }, description: { value: 'Enterprise
edition product with advanced features', type: 'String' } }, score:
0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: { collection:
'client_chat_basic_ts', record: { price: { type: 'Integer', value: 99 }, id:
'FLrLO6Ckk_b0uzOsdtDAD31AOArTSO3TQ3jBeWQBHFLCeL6Cr0dPxTNFx3lmo8ogMhbZu8ufQk_41NbBEA45Kw',
description: { type: 'String', value: 'A high-performance database product with
AI capabilities' }, name: { type: 'String', value: 'ekoDB' } }, score:
0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: { collection:
'client_chat_basic_ts', record: { description: { type: 'String', value: 'Fully
managed cloud database service product' }, id:
'\_enem1mL3KL1jmvF4tvFUyyThGG0X2oci4vsmvUiOFB3d_hi9fuZQY2Xxq2RguvcSd9KDRCwRLQNNHq34ahn2w',
price: { type: 'Integer', value: 499 }, name: { value: 'ekoDB Cloud', type:
'String' } }, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 1125ms

=== Token Usage === Prompt tokens: 312 Completion tokens: 30 Total tokens: 342

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
sgngjXb6_bp3zDAGpPWCq7z6WzaoKH9CC9eysgNqtmhdqnm1UOr7FVJAPkajpQpc4EldHKAoN-ZV1aKTy50mlg

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not include specific information about any products. Could you
please provide more details?

✓ Message 2 sent Response: As an assistant, I'm sorry but I cannot provide the
price as the context does not specify any product. Could you please provide more
details or specify the product you're referring to?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
TTGkBG2oCVvHaIu1FV9mR2s_dPjniPYvWF8_W_nQr4-LiilCNV5CZ9gD6ztByPueHnfxjB9oWxhKlYUjZ2iWeg
Parent:
sgngjXb6_bp3zDAGpPWCq7z6WzaoKH9CC9eysgNqtmhdqnm1UOr7FVJAPkajpQpc4EldHKAoN-ZV1aKTy50mlg

=== Listing Sessions === ✓ Found 10 sessions Session 1:
TTGkBG2oCVvHaIu1FV9mR2s_dPjniPYvWF8_W_nQr4-LiilCNV5CZ9gD6ztByPueHnfxjB9oWxhKlYUjZ2iWeg
(Untitled) Session 2:
sgngjXb6_bp3zDAGpPWCq7z6WzaoKH9CC9eysgNqtmhdqnm1UOr7FVJAPkajpQpc4EldHKAoN-ZV1aKTy50mlg
(Untitled) Session 3:
SngZo3l4_KRCF_AHJMU2c6QrBn_b2A61Btx4CnhiPTKepSlxp95VdfMtrOQYBo7JAlO7NjqXbDeHkoHLKBIY-Q
(Untitled) Session 4:
mwE_UXqsFXQSRhOFX3Nt4hIcexq-iFNBRqP5H0ZO8L9T_DRrXwPd7ew4cQqi6J3KXSiL_LHAuUz3c936tjUDPQ
(Untitled) Session 5:
k4j8sFpfurOcPhiKbDk0sPYzER1PhQ5lKRQOqxTQUqdIR8jGnrA8E6AXhZQQ4dvea02zgRAZYEf7cvpbnGslHg
(Untitled) Session 6:
l4ZyFBZ3lJRgkSZ4Yf_N1b9Rfvz-q_wNsegBtgwaPpi0efGToI0ktyU3xujoPSikQhQid-3f9P65Q_6az3ddpA
(Untitled) Session 7:
HJUs621HzLEuG-m3hdCO5FpoC-fowMhZgf8gzr4v6o8alHcz6CEqa3dEurUFur_j7nF6j77ahqyjSO2SnTo54A
(Untitled) Session 8:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg
(Untitled) Session 9:
jC9n-MLxQZb3--dNvpyK_CMNOi8WJDW2Mm_iNNCGVoKg8l-xArdqbsUhIQcVe-iVPo8jjolL5q5-41gIUXepAQ
(Untitled) Session 10:
fcQkL9x-SJ9VPI8YEYzJjcZh5CrpXeQAC8VL_ax38Gh3b2yMZsDgCkIYqONB9cTxXJiRFa5iA6AsALNAhrtwzw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
TTGkBG2oCVvHaIu1FV9mR2s_dPjniPYvWF8_W_nQr4-LiilCNV5CZ9gD6ztByPueHnfxjB9oWxhKlYUjZ2iWeg

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
5nAF76hbhd4_GP47_LsQ5OFFMV_5oxUVNAM8ZkQVhOJj95jVCxvvNEFGBwfRa9ydXjGkLL4mjUGbu_TXBJt0eg

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain information about any specific products. Could
you please provide more details?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Apologies, but the context provided does not include information about the price
of ekoDB. I recommend checking the product's official website or contacting
their customer service for accurate pricing details.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
4tm8p8PHqserwqg0kX_R7SbM_FrLwlMB3wkaw4zz-h2VoLtY65QidsxhWYR_FHbGtONOjUZS2i0V-1F_c53How
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! === Query Builder
Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 3 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740, Matched: name.value, name, email, email.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio, title, title.value, bio.value
1. Score: 26.400, Matched: title.value, title, bio, bio.value
1. Score: 26.400, Matched: title.value, bio.value, title, bio
1. Score: 26.400, Matched: title.value, title, bio, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title.value, bio, bio.value, title
1. Score: 39.600, Matched: title.value, bio.value, bio, title
1. Score: 39.600, Matched: bio.value, bio, title.value, title
1. Score: 39.600, Matched: title.value, title, bio, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.759, Matched:
1. Score: 0.755, Matched:
1. Score: 0.730, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.704, Matched: title, content, content.value, title.value
1. Score: 1.502, Matched: content.value, title.value, content, title
1. Score: 0.292, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: skills.value, bio, skills, bio.value

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: [ 'age', 'email', 'name',
   'status' ] Schema version: 1

5. Retrieving collection metadata: Collection has 4 fields

6. Creating employee schema with all constraint types: ✅ Employee schema with
   all constraints created

✅ Schema management examples completed! === Join Operations Examples ===

Setting up sample data... ✅ Sample data created

1. Single collection join (users with departments): Found 2 users with
   department data:

- Bob Smith: Sales
- Alice Johnson: Engineering

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

4. Join orders with user data: Found 2 completed orders:

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
lWAL6q3xJ01hpSZE0-EpdQxccTWR7uZ3fcgu1xQgp4fZJvmTVzl8vH4CwSGgX5tRVgrC-N0ZqHPMw6Et1E4r6A
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
yO6aO86yLtjKxit7Gv9NlhKXbarIF4ir9X4GuPZMnhVDpkaEsgQdezu6Hn1QYSUvSPWhpNxfqSYFN9lnadlL1A
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
lA9VOZiBP9eQct43aD3l1Y8PhYNYi4i4E1sfDiXigISAhN1Lyc4k8YpXBaNUvDB3EyQPkV7XcEz5TBQvnZefaw
📊 Statistics: 2 groups {"status":"active","count":5,"avg_score":60}
{"status":"inactive","avg_score":50,"count":5}

📝 Example 4: Script Management

📋 Total scripts: 18 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
MWdyl_vfgk0x8uS78OixHacMQtypIMScmGUAL6Io-prBqVItQ1IssV5G78Z54KVFFDkg4w0KxKU5eGUErMAPGQ
📊 Found 2 product groups {"count":2,"category":"Furniture","avg_price":474}
{"category":"Electronics","avg_price":575.6666666666666,"count":3} ⏱️ Execution
time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":3,"category":"Electronics"}
{"category":"Furniture","count":2} ⏱️ Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved 📊 Pipeline executed 3 stages ⏱️ Total execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system: ✅ FindAll operations ✅
Group aggregations (Count, Average) ✅ Multi-stage pipelines (FindAll → Group →
Count) ✅ Parameter definitions ✅ Script management (save, call, delete) ✅
[32mTypeScript client examples complete![0m
