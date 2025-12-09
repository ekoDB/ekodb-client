make test-examples-typescript 📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 654ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'zu-bx-dsnN87SMFoBFtB9zxnOjkBdHIxMDC4Rf8CHjRvDE9b2sFlN-KS8F1lHABxvZ2r6-AhcOMne03NkDf_KA'
}

=== Find by ID === Found: { value: { value: 42, type: 'Integer' }, id:
'zu-bx-dsnN87SMFoBFtB9zxnOjkBdHIxMDC4Rf8CHjRvDE9b2sFlN-KS8F1lHABxvZ2r6-AhcOMne03NkDf_KA',
active: { value: true, type: 'Boolean' }, name: { type: 'String', value: 'Test
Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, id:
'zu-bx-dsnN87SMFoBFtB9zxnOjkBdHIxMDC4Rf8CHjRvDE9b2sFlN-KS8F1lHABxvZ2r6-AhcOMne03NkDf_KA',
active: { value: true, type: 'Boolean' }, value: { type: 'Integer', value: 100 }
}

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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
edVt267TB09S3atm3PVFoeGHdnAI3eoMlElQCxp5gBIz7bAWZYM-LUCwnF8bjrQEIVnf0py5KC0AkBcEsxpObg

=== List Collections === Total collections: 18 Sample collections:
schema_users_client_py,schema_employees_client_py,test_collection,chat_configurations**ek0_bench,chat_messages**ek0_bench

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
IyDZXQemCyEhN8GXGdJRQTIJX7OCikRuTftLU4GXhdVa4r02JF1ZLsls1B7SsYpJiE1L7h8fmynLx7cCIKXrIg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
V1e7PX33fcol4riEDY9_4dIQZPmcTx0JvmGtXyxJyP2KaikWP5uwrtWlI-lujq-oTvq7VKYaQ8I4WyJAyGr1ig

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
XgigwdqKDkzpbFre26Z9Dz7OEy0gUYsI-1eexgLUTfIY49aJQRHI1wGmx_ZnFRFI0mznQOovwUbKO8jbXRXMyw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
0koT1SAbEioOUcPC-YPHD0lFhT78JwNM6t9VoTU1VktIIOKtTkzfnniQIe2eo-dDc9MlDlRmGLx0r0_9jgk_9w

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
X9rqox4icxg1eHIMX3le_ALFb5WkGaoLzeXuDub5dQmtts3RjPkPogfYABVPsPWsCosoNwlJqt-5qcVHKG63vA

=== Sending Chat Message === Message ID:
MiSbqRf7Q9wM44qR4JfqIjFZCzvL7cfRNLdD9YCyudrXHlOpGb21KGKuN6xc1jQbC3NkXGE_vBMYnPHvomiFHw

=== AI Response === I'm sorry, but the provided context does not include
specific product names or their prices. Could you please provide more
information or a more specific query?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { price: { type: 'Integer', value: 99 }, id:
'2FLvbZUDCwXQk3BBjvA4e8WysdQOiLB_OdcD_ZfFcU7TvmopbcRj4G0n_6zVu6ynSWWo7rmCvwS-a4HtNxlcvQ',
description: { value: 'A high-performance database product with AI
capabilities', type: 'String' }, name: { type: 'String', value: 'ekoDB' } },
score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: {
collection: 'client_chat_basic_ts', record: { price: { type: 'Integer', value:
299 }, description: { type: 'String', value: 'Enterprise edition product with
advanced features' }, id:
'tbDiPHcHZk3aDesFih7mWEa1MAbHpjc60JJGq11SdXriTih_Swry2NnOBJ22ABifWHPyaZz-Y8TFKsVlZG17vw',
name: { value: 'ekoDB Pro', type: 'String' } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] } Snippet 3: { collection:
'client_chat_basic_ts', record: { name: { value: 'ekoDB Cloud', type: 'String'
}, id:
'Nzbz4jaEmakggY6HVRofgOjoaHsUUbh6vxjU5DJyjz4AjCs3zagBoqpGEsUyt_l6Tnrs-SeI2NCoJP_3uSZt2Q',
description: { value: 'Fully managed cloud database service product', type:
'String' }, price: { type: 'Integer', value: 499 } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] }

Execution Time: 1298ms

=== Token Usage === Prompt tokens: 315 Completion tokens: 30 Total tokens: 345

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
HuW1e8hufSqm7oAJcImllRsc-UCNwpEocV8vvtBEhsyOBC1t3rtGOxmi8L0f_m18yVEdyZiL6Fde73wHYGrgVA

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the provided
context does not contain any specific product information. Could you please
provide more details or a different context?

✓ Message 2 sent Response: Apologies, but the context provided doesn't include
information about the price of a product. Could you please provide more details
or specify the product you're inquiring about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
RAu4uYfELQ0ynJEfZVwizs7opJD4HWx2hRjCjKEqhL7ChHVLaefQ0Em1n8EOe9Oy8AoemZK3Rue763a8mNe0lQ
Parent:
HuW1e8hufSqm7oAJcImllRsc-UCNwpEocV8vvtBEhsyOBC1t3rtGOxmi8L0f_m18yVEdyZiL6Fde73wHYGrgVA

=== Listing Sessions === ✓ Found 10 sessions Session 1:
RAu4uYfELQ0ynJEfZVwizs7opJD4HWx2hRjCjKEqhL7ChHVLaefQ0Em1n8EOe9Oy8AoemZK3Rue763a8mNe0lQ
(Untitled) Session 2:
HuW1e8hufSqm7oAJcImllRsc-UCNwpEocV8vvtBEhsyOBC1t3rtGOxmi8L0f_m18yVEdyZiL6Fde73wHYGrgVA
(Untitled) Session 3:
X9rqox4icxg1eHIMX3le_ALFb5WkGaoLzeXuDub5dQmtts3RjPkPogfYABVPsPWsCosoNwlJqt-5qcVHKG63vA
(Untitled) Session 4:
lpIFDaYbl2PzBY3_7ZdF0F17T3jmH-N9SWDwgE-kGVnbK3WYJqXh90SMIs9iFktkkQx2yWwW29t6dOdULEkAVQ
(Untitled) Session 5:
BlUuNrxrPcg7DT7XH5RyCQlSIP904lBzfFMBc4clDioYQu4G-WqYuSz1vsRACAucqG9_n8ALXwofDGG2gtFfPQ
(Untitled) Session 6:
o5YWLghBe-uU6P90uarQF7o_A3m8Nfh1xrlX2qTK7f1VPTkTotbZc-1gxSPn36OqJiIh4jVLqfaPsgfU9S0cLg
(Untitled) Session 7:
Cba_JGR3M-Emd84AjyvP6hxbHaewbJeQAOh-i20QRBPQACBK8Irp8dY1A6TPc5I9I56o0Qa25EiDAL1kGr_vXQ
(Untitled) Session 8:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ
(Untitled) Session 9:
3A1UH696f5qnpJrqcP7cydw8RytBVZgW2EDAYDoXRrH67_UAfeowZ81CpQI1Yw53fBOly5lNPvxJdQCq9e4CWg
(Untitled) Session 10:
Y9oeEuI0Dmbz-mFP_WrCEsZDbKYoKGSpek0DIqaEDWl5ia3L5pUftCBVIKBw4dtX5sMlhoqqhoK5Dj-TltDRgg
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
RAu4uYfELQ0ynJEfZVwizs7opJD4HWx2hRjCjKEqhL7ChHVLaefQ0Em1n8EOe9Oy8AoemZK3Rue763a8mNe0lQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
YNYXc9UjaG2DyEHUK52J0UtomAq29sWZGDIi_dJgV8V9oGU5a5zND3FJnGYtrk11rZXdUrp3TAI2vJFiBGIISQ

=== Sending Initial Message === ✓ Message sent Response: Answer: The context
does not provide specific information about available products. Could you please
provide more details?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided doesn't include information about
the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
vJGopzlrJHB8R88ifgEYe4u4nCdsxLN-aOMSIq8krTjbnLqflnw7l7cSfxRVcvFc4W6t5yTf4ck9B3jW8kNcNw
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
1. Score: 25.740, Matched: name, email, email.value, name.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title.value, title, bio.value, bio
1. Score: 26.400, Matched: title.value, bio.value, title, bio
1. Score: 26.400, Matched: bio, title.value, title, bio.value
1. Score: 26.400, Matched: bio.value, title.value, bio, title

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, title, bio.value, title.value
1. Score: 39.600, Matched: bio.value, title.value, title, bio
1. Score: 39.600, Matched: title.value, title, bio.value, bio
1. Score: 39.600, Matched: title, bio, bio.value, title.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.760, Matched:
1. Score: 0.753, Matched:
1. Score: 0.748, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.701, Matched: title.value, title, content.value, content
1. Score: 1.504, Matched: title.value, title, content, content.value
1. Score: 0.299, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: skills.value, bio.value, bio, skills

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

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

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
oFjVOpckAMclxm*0dTQ-Tf1SnU-R98Vq4Eydy7pydxU*-Gzgz8rGQ48tDPmLl90DCwVJeGqJ1i_G6J3pzzxekg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
ktaZhVTgvm1wpQqTpsBvktGXkh522neYgePfA7ThKmfJdjIguZz_yKbsrnz9ir4OH2M_vPGBphEvWFknmLh1Cw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
Ul4g8MKXNSgjkyH2lvXbByYpzXp1sbQr0yCFZ-XZaqzG_A2ByQKPvq1sO4D0ZeviBnRk_ctvdzVF-BY_WVWH_A
📊 Statistics: 2 groups {"status":"inactive","avg_score":50,"count":5}
{"count":5,"avg_score":60,"status":"active"}

📝 Example 4: Script Management

📋 Total scripts: 17 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
mCCzbyBjwXJY2NLQvVIKgnbOhuxraunjMDpuD6i8XmGyQfcOj3W5EwoJJY90DZFjhdXwAlS47VQk9VRSMDeZbQ
📊 Found 2 product groups
{"count":3,"category":"Electronics","avg_price":575.6666666666666}
{"avg_price":474,"category":"Furniture","count":2} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":3,"category":"Electronics"}
{"count":2,"category":"Furniture"} ⏱️ Execution time: 0ms

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
