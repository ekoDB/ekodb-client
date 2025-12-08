make test-examples-typescript 📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 751ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'x2Z8PgI80SgoHwn54yajeHF_V5ruANlXkmCGBfTp4xqyBq_oSg34T9qZunyfEK4z9QQ6uByGWPIJ-ih9qf4cJA'
}

=== Find by ID === Found: { value: { type: 'Integer', value: 42 }, active: {
value: true, type: 'Boolean' }, name: { type: 'String', value: 'Test Record' },
id:
'x2Z8PgI80SgoHwn54yajeHF_V5ruANlXkmCGBfTp4xqyBq_oSg34T9qZunyfEK4z9QQ6uByGWPIJ-ih9qf4cJA'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, value: { value: 100, type: 'Integer' }, id:
'x2Z8PgI80SgoHwn54yajeHF_V5ruANlXkmCGBfTp4xqyBq_oSg34T9qZunyfEK4z9QQ6uByGWPIJ-ih9qf4cJA',
active: { value: true, type: 'Boolean' } }

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
ekqU7HXcstC87C9Qsh0RT-2XNTsjsaFnt_IVIsdzqQRDBfQQIqABvkyFTaLxfNmtvH279i6Q4WxGvcvxLFRmfw

=== List Collections === Total collections: 17 Sample collections:
schema_employees_client_go,schema_products_client_go,websocket_test,chat_configurations\_\_ek0_bench,client_collection_management_ts

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
ocM66myNyECsdURGU2uABjgkDzjmb8kLY3uDNIbms0ico_hrUVW1LLxVSYDdY5CEGsb04SeE0Cq2stp4x86viA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
0kTg-PRAXyxJOqnzAnUfMsprnUJWaJr0J7776oLDJiqsWm1MI6VrPLTmkAyVBZYBfcaC3gv49uzLOex1adoJkA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
e897jECKOgmUz1Lzroo_PrY9uO4CL2NIdbOIKXxfK_q-1Bv93aMkarN7GPfH6LpNWNoE7dLXbbsqMuQN1KjWtg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
udKVSq-rQdzoV71r0BvjmuDDnPOc1Ag2Dt9yh8FJG1lLYgAxyDCUzkDw1kE_3bDWBBKJ91PACETuMyIBmhxSwg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
KYvxTV6hNa76U1VJgoyAYjISJeetCX_6XpUHPfGCwF54g6NImIEG0mBd_tocpLV0qHI2gsnIy_Dha9tFj-Ue0Q

=== Sending Chat Message === Message ID:
hfv11PM03khioh5J1boS-6LoxlDevIdMDDHM6ObFPnYhrJb4G62vPszTzsob690ulAovTGw7wXRzHhr82pu3OQ

=== AI Response === The available products are:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is
   priced at $499.
2. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.
3. ekoDB Pro: This is the enterprise edition product with advanced features. It
   is priced at $299.

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { description: { value: 'Fully managed cloud
database service product', type: 'String' }, id:
'Jqtridh7uBeQiLaUfwZjm5Z1aUPcSq4lrZu3LY_PDUAkWl79MSkEvBKAR4N0SBlxrRgqYDC7C8FlKgWbD3jWwQ',
name: { value: 'ekoDB Cloud', type: 'String' }, price: { type: 'Integer', value:
499 } }, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet
2: { collection: 'client_chat_basic_ts', record: { price: { value: 99, type:
'Integer' }, description: { type: 'String', value: 'A high-performance database
product with AI capabilities' }, name: { type: 'String', value: 'ekoDB' }, id:
'le5-hx0cbOUAeQxxxxeUO-X4R6vosxAm1205Ahxd0S8NNvxKVM9opKl-qnr9WnVW2crA8QUHwJzBcYYQCZtZAA'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { description: { value: 'Enterprise
edition product with advanced features', type: 'String' }, name: { value: 'ekoDB
Pro', type: 'String' }, price: { type: 'Integer', value: 299 }, id:
'JR2I6VP-O4YaH3_IbNX4iS18V7uSdwUQCmz4EP0DdnO3r5yKXs9ZDS_orcq5Lp3ZKaFTfVkrO2S1sEhoCm75Qg'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 2952ms

=== Token Usage === Prompt tokens: 435 Completion tokens: 77 Total tokens: 512

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
3uOMa-Hw2rvS4Ako5ErWaV_UeihOuZ4cLrc9fmDRfBOsQryBsWB1zSpDLW-X2N0r0LUGMLTLQN8t5054ZHW5Nw

=== Sending Messages === ✓ Message 1 sent Response: Based on the information
provided, the available product is a high-performance database product named
"ekoDB". It is priced at $99.

✓ Message 2 sent Response: Apologies, but the context provided doesn't include
information about a specific product's price. Could you please specify the
product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
jEifjOxmLKwnziqSInuUz63h-fa1WvSnLbrkhXr03VjyPe-dKXbB_X1lr18AeQZxnN4m7Ss3CU4ymg0igRel6w
Parent:
3uOMa-Hw2rvS4Ako5ErWaV_UeihOuZ4cLrc9fmDRfBOsQryBsWB1zSpDLW-X2N0r0LUGMLTLQN8t5054ZHW5Nw

=== Listing Sessions === ✓ Found 10 sessions Session 1:
jEifjOxmLKwnziqSInuUz63h-fa1WvSnLbrkhXr03VjyPe-dKXbB_X1lr18AeQZxnN4m7Ss3CU4ymg0igRel6w
(Untitled) Session 2:
3uOMa-Hw2rvS4Ako5ErWaV_UeihOuZ4cLrc9fmDRfBOsQryBsWB1zSpDLW-X2N0r0LUGMLTLQN8t5054ZHW5Nw
(Untitled) Session 3:
KYvxTV6hNa76U1VJgoyAYjISJeetCX_6XpUHPfGCwF54g6NImIEG0mBd_tocpLV0qHI2gsnIy_Dha9tFj-Ue0Q
(Untitled) Session 4:
jb5a8ZjKnd5P0u0yjq3ZJZ1JPu9YrAcfqXKC7OQW1B7bDSq69mH1k7NvTSkXRCiqbKBDw8D7CA3n02f_4sShJA
(Untitled) Session 5:
mi7hFPVOAsbqyVH3r6ItfGqqUuRJpQmk9TTtMq28cgDg1WOPfNt7xDphiYZ9nYl--xfnYvgKgExzfowr8g8WBw
(Untitled) Session 6:
XTIulD8-zJOCZDUG_73NrBoP2NbE8rQOTNMH9MRYPjnsmlZvR_AgohumDZpQ2-8RTGemZn1rJ_tV3eSyaHfzhA
(Untitled) Session 7:
xk1gRyFojsYWBB7kDMWNkw5y4h5JIcVoLN-zoBLR-Ws_UEc2ZU_DROpqO2lL92f4140PY5eFcPvVj8cpfOeyxg
(Untitled) Session 8:
nhwhhgOvM3Cs7T7pPSwLlWkqiCW86qVdGG2kHPlhvy_kUxnHKlipsiOXJ_S4NiIuccj1fcrME6jXHzyY1I9Aew
(Untitled) Session 9:
bEGYtiLgEbT1EbPG8_0FOTynTXtkYIvcZGBaPcKgHVDQ8pSe4ATrA0QDir6A-85pvJpCR4dZe5GYNJPqqZY12g
(Untitled) Session 10:
gCQ0KOMfyF62QuoyABvyWlQg0eTLqa-0V8wAplTvIRrRNnlIywMtHlRzS4i-GM4rSk61UZXenP7tTndJnc_Znw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
jEifjOxmLKwnziqSInuUz63h-fa1WvSnLbrkhXr03VjyPe-dKXbB_X1lr18AeQZxnN4m7Ss3CU4ymg0igRel6w

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
xCY1-APAodg2sFRlp0ja8It7H6X1luOw-q1fr3abPhiwBqfp6CfAp0Ew4Kbg9qDQKhtCBW_NbTkk1BjH6WsVPg

=== Sending Initial Message === ✓ Message sent Response: Answer: The available
product is a high-performance database product named ekoDB, priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided doesn't include information about
the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
agSc1qqA3fBcsXeoo2Pb9poYM6IYV_HLOHHVfS2WMc5NdB1teuT1SNwXk9-0w8oig_v69k2dQbKZRF4d7iVEeA
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
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio, bio.value, title.value
1. Score: 26.400, Matched: title.value, title, bio, bio.value
1. Score: 26.400, Matched: bio.value, title, title.value, bio
1. Score: 26.400, Matched: bio.value, title, bio, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, title.value, bio.value, title
1. Score: 39.600, Matched: title, bio.value, bio, title.value
1. Score: 39.600, Matched: title, bio.value, title.value, bio
1. Score: 39.600, Matched: title.value, title, bio, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.777, Matched:
1. Score: 0.758, Matched:
1. Score: 0.721, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.711, Matched: title.value, content, content.value, title
1. Score: 1.489, Matched: title, content.value, content, title.value
1. Score: 0.303, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: bio, skills, skills.value, bio.value

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
z_XyfmcSPicF9uqjxQsWJizMwm5j_hbkl9RhSxCvZbBTJ8ZRdgUSNoOfMbUpOCWjvdoO26GAMTDwpwI3s2f48g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
SSXXFfo9muLKRq6b10PZZPPmN1-7nNoIRZxHVYQKbnLx4SsspFgGvJUkqmMmPeYg_8NwAwh1ndaiy3X6D2Orzw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
FgeNnWeaeJxreww2L2gLD78z4lPETSP2heOKKoVXGEjisKNvDGSwA2080K21B0Aq9AEg1W90xv8JSwenzVEs0w
📊 Statistics: 2 groups {"count":5,"status":"active","avg_score":60}
{"avg_score":50,"count":5,"status":"inactive"}

📝 Example 4: Script Management

📋 Total scripts: 11 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
uRcxipplCyYi5xEdexlBSn04yH5iGrkBUsp6RUT6O0eJzLzfQP8_LbdMo6D4CUFI4Etyg5irMtAnOrG4mVfB4Q
📊 Found 2 product groups {"count":2,"avg_price":474,"category":"Furniture"}
{"count":3,"avg_price":575.6666666666666,"category":"Electronics"} ⏱️ Execution
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
