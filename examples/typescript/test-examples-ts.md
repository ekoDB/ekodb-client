make test-examples-typescript 📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 679ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'YE6_I58BUE42oQ4vmKd2pCB2NKXUcNgRTh1Gwnnmp3IFl6Zsnzp_QwjMS7ZQscJuCEQyZxUAa39FmdpheBT5EQ'
}

=== Find by ID === Found: { value: { type: 'Integer', value: 42 }, name: { type:
'String', value: 'Test Record' }, id:
'YE6_I58BUE42oQ4vmKd2pCB2NKXUcNgRTh1Gwnnmp3IFl6Zsnzp_QwjMS7ZQscJuCEQyZxUAa39FmdpheBT5EQ',
active: { type: 'Boolean', value: true } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { type: 'Integer', value: 100 }, id:
'YE6_I58BUE42oQ4vmKd2pCB2NKXUcNgRTh1Gwnnmp3IFl6Zsnzp_QwjMS7ZQscJuCEQyZxUAa39FmdpheBT5EQ',
name: { type: 'String', value: 'Updated Record' }, active: { type: 'Boolean',
value: true } }

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

=== KV Get === Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
0A3L_O4MeQMMxcvml5O04sUF28N04FxnM-k3DkfrbDK7UHhpGzFjquAw6b4l3wDxPiSUy9A8p_PcFQHMugyGiw

=== List Collections === Total collections: 18 Sample collections:
schema_employees_client_py,schema_products_client_go,schema_documents_client_go,schema_users_client_py,websocket_test

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
p4XCtZs_WvGPw4VfMOUy092v83a7lSLProhjcQn-bWWNalLFboK8tQURSJc2fX-wzzhH7DYqXv3i9UXpoLHhzQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
AMQM0psojfPCdfw0dVod6MQLrrvlDJPIaaHVxuGBKVzzEth6_EaTeiGzqEH04ldnIYncc4Z9G6JMxbgWlRbx9A

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
owxI-T4iab654gqUHDsQPXEUaIru_56gkg0uKlBv_k7TMRZCJQsrMRPEFeK0zSUCVHQgNPSrrBDid2f_0-cHNw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
9gdpgUR3xQkmO0Z-zgp6psEKiCnOZYFlK_wszqsHSE6wBVy7dcWfol4nM6Ml_UvhGYmWBME7rSZLH0aM2cDyHw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
Kqz2qCe8a79AOhXHQ-31fwJp6BnA-HGZgPf-2-LrTdU_swMLcEkgZ5bGcNvjG4y2FAXpgGKnLH08BMYHsGUULg

=== Sending Chat Message === Message ID:
H9TN5YmGpdH7ZSmpxKOr2ALKYiiAv8RRPahSmSitfEt6O8jK8NNp-2UWa6VWuEMhniY2rqL9O8ky9-MVEtuRmA

=== AI Response === I'm sorry, but the context provided does not contain
specific product names or their prices. Could you please provide more
information or a specific product you are interested in?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { id:
'H9ocFKmx1Xw42wwj5XcfOpmaZ8wK82GR3V0OlTYrgqFWSZZXFKrOmMfRtlQRZZmuoXKZqr9Gl53DZmSkJZkmhA',
name: { value: 'ekoDB Cloud', type: 'String' }, description: { type: 'String',
value: 'Fully managed cloud database service product' }, price: { value: 499,
type: 'Integer' } }, score: 0.1111111111111111, matched_fields: [ 'description'
] } Snippet 2: { collection: 'client_chat_basic_ts', record: { price: { value:
99, type: 'Integer' }, description: { type: 'String', value: 'A high-performance
database product with AI capabilities' }, name: { type: 'String', value: 'ekoDB'
}, id:
'K4bqcemEfud9qnSR2bDwS2AFl2ILzwXOQ09VyjdPsDtHOUjRPWxzwG4ncoY7dCNvooLpC3MLtqawWxp5mKvZcQ'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { description: { type: 'String',
value: 'Enterprise edition product with advanced features' }, name: { type:
'String', value: 'ekoDB Pro' }, id:
'Cb42y3dwe57HETIIBXAL1r7T3W3IgMtUZDHvnLVZ9QzjL23zqzSGhqXaFzV5SY3Yf0qgfiu31HQb74n1ReKw2A',
price: { type: 'Integer', value: 299 } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] }

Execution Time: 1609ms

=== Token Usage === Prompt tokens: 324 Completion tokens: 33 Total tokens: 357

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
sqvc2HxJsbU2fq5aXDrqt3Un7wySo9sTcZF4K2c5NzQDHbLBl8uYfqolBshThxYQmL5_sab5uhymTR7wZP-2Aw

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the provided
context does not contain any specific product information. Could you please
provide more details or context?

✓ Message 2 sent Response: Apologies, but there doesn't appear to be any
specific product mentioned in the context provided. Could you please provide
more information or specify the product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
RBVIDd3sAoVySsVCuZqcVlSuz0X6f_WOPoAgbj7-FLGOEcqbm42FKQleJv-tIijGFBmmx-MN3XcQKTwvZhoe6w
Parent:
sqvc2HxJsbU2fq5aXDrqt3Un7wySo9sTcZF4K2c5NzQDHbLBl8uYfqolBshThxYQmL5_sab5uhymTR7wZP-2Aw

=== Listing Sessions === ✓ Found 10 sessions Session 1:
RBVIDd3sAoVySsVCuZqcVlSuz0X6f_WOPoAgbj7-FLGOEcqbm42FKQleJv-tIijGFBmmx-MN3XcQKTwvZhoe6w
(Untitled) Session 2:
sqvc2HxJsbU2fq5aXDrqt3Un7wySo9sTcZF4K2c5NzQDHbLBl8uYfqolBshThxYQmL5_sab5uhymTR7wZP-2Aw
(Untitled) Session 3:
Kqz2qCe8a79AOhXHQ-31fwJp6BnA-HGZgPf-2-LrTdU_swMLcEkgZ5bGcNvjG4y2FAXpgGKnLH08BMYHsGUULg
(Untitled) Session 4:
-FJ4CVqdG5Gip24DhL4UpNnFgCiZxYnXepzawKad4RjbD6jL_BV5WwyqS4NVrBLHohr1LJ5o7OlQj4JlhDcCbQ
(Untitled) Session 5:
GRKGm3YelUoMrdfx0o_hpFj4N7PxezKTNrdcMsXTvFu1kSlg7otVecgeEkdSB-onud-TB671Yc6WuBsUS28daQ
(Untitled) Session 6:
oWpN5T2vfqfqTqW59MzTvosMCC_kV2m9afaABNUB4Fq93Hk_V1wOdqrQsV8wgRuWrKjyDl0A19tWZ_kFg0r6XQ
(Untitled) Session 7:
3dKLTjbKKdWiKWJMiZSl27k-QOuXrlUsrkYfN3u6MHsPZpxafROQqHN7VqKXYIA_cDw_ok8lLtRs1RHmTZtdpg
(Untitled) Session 8:
jl_qC01XuY7UlWB31pIZDlb5ZT28fbBT40JNtj-beGEb5IMZTAfAVmzBAP5t6TIvQfE-17axFs_WESUzv_q1lA
(Untitled) Session 9:
GcJoGq8p4I9IzQJSpC3mOI9-Knp0ck9RKI25PN7C5bKsw0x4b-iHoBJ16NxTCck-JY9EZKnSBXcCVt5Uqoo2Qw
(Untitled) Session 10:
DZrN-AQV9Nn4wOTelR2KA6397oce29THpcBtxbI_iT2QOQo1yaK92LZIYuZGB_sN1pUrtH1Cg8FTgApuh7BRoA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
RBVIDd3sAoVySsVCuZqcVlSuz0X6f_WOPoAgbj7-FLGOEcqbm42FKQleJv-tIijGFBmmx-MN3XcQKTwvZhoe6w

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
rIymRRJa-YuBm4lcaQmRBKNgMYhpPdUCgpnjG8D-17CfGfecy7mXNtsP6PcSioSwX7vuBUeJONqzdovazZEyvA

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain any specific information about available
products. Can you provide more details?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Apologies, but the context provided does not include information on the price of
ekoDB. I recommend checking the product's official website or contacting the
seller for the most accurate and current pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
6SohYOR4Ze4eEK_cCktreEuzA40r5l6m9U8aUqGKkdOQrTkYNpOMX-10tL1vWPEeKhiSUnxUsTHYP-c58BoF5w
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
1. Score: 25.740, Matched: email.value, email, name, name.value
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: bio.value, bio, title.value, title
1. Score: 26.400, Matched: bio.value, title, title.value, bio

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title.value, bio, bio.value, title
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: bio.value, title, title.value, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.749, Matched:
1. Score: 0.746, Matched:
1. Score: 0.743, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.699, Matched: title, content, title.value, content.value
1. Score: 1.498, Matched: title, content.value, content, title.value
1. Score: 0.297, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: bio, skills.value, skills, bio.value

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
8hpqrr-WV6c966VRPy9QCkHc6_qTXfLqlfJWU1p89po2plAYai8ztG8tPi0L81VOKcQCC4WirY8NHOg2lFJA0Q
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
mw7eqUL82RaQp3RN2WF99HhN7z7m_JawPyp4nk1wGs6RuYi2a6ivg1mzmM7enN5mReQbgTjtUuMWfcyipVYbyw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
apgovIXduWcTD44f0O3u7t1w4kXWnA07uqOE3Sv2ZZsqYTIAVNdFmy9Z2ngxeWk9Y-P-H9Dl95xBF2J8DyZgPA
📊 Statistics: 2 groups {"count":5,"avg_score":60,"status":"active"}
{"avg_score":50,"status":"inactive","count":5}

📝 Example 4: Script Management

📋 Total scripts: 17 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
QSSB8uOk1bYa6CitxfUhRnaLaRRLG1QlfnbW-gKncl99swMndeR0Psi7X8vef-qBuciiMXW8jp-2B8P0XmnjPA
📊 Found 2 product groups
{"count":3,"category":"Electronics","avg_price":575.6666666666666}
{"count":2,"avg_price":474,"category":"Furniture"} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":2,"category":"Furniture"}
{"count":3,"category":"Electronics"} ⏱️ Execution time: 0ms

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
