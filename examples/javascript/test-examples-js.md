make test-examples-javascript 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 632ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:40725) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 9 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:40728) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'JtZ7g8fr8svPiESjLJyPhnUtjzAjNkabqDcIIlXKwG1tujth842KwiHp4wCCQW_y1_MkGMqDX6TgQoRBqnPNWQ'
}

=== Find by ID === Found: { active: { type: 'Boolean', value: true }, id:
'JtZ7g8fr8svPiESjLJyPhnUtjzAjNkabqDcIIlXKwG1tujth842KwiHp4wCCQW_y1_MkGMqDX6TgQoRBqnPNWQ',
value: { type: 'Integer', value: 42 }, name: { value: 'Test Record', type:
'String' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { type: 'Integer', value: 100 }, name:
{ value: 'Updated Record', type: 'String' }, id:
'JtZ7g8fr8svPiESjLJyPhnUtjzAjNkabqDcIIlXKwG1tujth842KwiHp4wCCQW_y1_MkGMqDX6TgQoRBqnPNWQ',
active: { type: 'Boolean', value: true } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:40732) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
jUSrcPm9ImFIqdf3jg2qKZQ8YfiwECjhL8f98mPXdBG4VZRoKZqO6zolEPIcBoe_qgBDOWJ2cYyRDRdcV2UHww

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"gxyJOI5aO4nhazTz8ifgYCc10ASLp66ugU-6T1lqQ8eZiBqUWG9WA3Sc0Dkd8Pjpv60sdsWUbp8WImMAo_lVzw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"tHnRpXS9iyrx1UuHjVIcjmAukoOpDqcb8Vibum5lJBrzq4lg2KxYmcaaf0nF2VPwRhNNDkl_gkP3b4Tr_E6DKw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"jUSrcPm9ImFIqdf3jg2qKZQ8YfiwECjhL8f98mPXdBG4VZRoKZqO6zolEPIcBoe_qgBDOWJ2cYyRDRdcV2UHww",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"xHB7dOzhn-TURJ68KMxUhViza0AENh17KfL1vjmYJsMyjVX49kQaqRZqLD_AbCoe-BrfQx-JZokRamE6b0Vm3Q",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:40738) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Batch
inserted 5 records ✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully [32m✓ batch_operations.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js
===[0m ✓ Authentication successful

=== KV Set === (node:40741) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-09T06:41:25.223Z',
username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:40744) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
wLoq2469adRvTzc3rAX0e7FY6_H_hIo3OH3E06gfyPbQ2pCmmivHXvuMO4oluwZeqRnsPZ4WM14sCJ14HQBMLA

=== List Collections === Total collections: 23 Sample collections: [
'schema_employees_client_py', 'demo_collection', 'schema_products_client_go',
'schema_documents_client_go', 'schema_users_client_py' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:40747) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
pAf6ipb7ujRsoX0B5GzSPZhTlwFtayR7QLnd0QS-hpBDNXoyQUPuPlzbjm_NQT2wDvr8fqTjROXOMOEcjbRX9g

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:40750) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
orOPJRbUqa7WM7lst27qgVUJ9-MNT07-ZcYyBuQqwDLjIZlItYOSgFMHTsTCsR8TjUUzt2UcrvzbCp52BKOBIQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
bWFHOO5NntHMBhbk0l-Xw7H-nxJp5iZF4MKpQBnWfWeb7KPkLgQgjaiFYRVHbi7HgetFxHN6yH_pFebxkflsMA
📊 Found 20 active users

📝 Example 2: Complex Filter with Multiple Conditions

✅ Script saved:
j9goPjtYKvYjdjML_6Giv51A049R_lhKqR6tk5btZI-3z79BqL2Q_uRuo2NGTtjU4cSlqHiT6yQ4UKsaDAoajA
📊 Found 6 users (status=active, score>50, sorted by score)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
4w_YNtgsjLXeJhqH3RRIqdnPj5UVkJyATFFKVmpQSLZ4yhG1q7gAcbQE7ReseWminCr3c7FdbPHmkQxg2ycZug
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":50,"max_score":90,"status":"inactive","count":10}
{"status":"active","count":10,"avg_score":60,"max_score":100}

📝 Example 4: Function Management

📋 Total scripts: 19 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.js completed successfully[0m
[34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js
===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
CM6ftPTv4VGx2R4AdS2V7zgZVvWBQOTU1exoGAfJAEu_QKDDXfLEyXWh6oyYmC6Ejc8r7KYePNC6-PTCnIRkEQ
Created Bob: $500 - ID:
\_X72N6rjdbmbOthsel8QOSdKXQ-5e2XZMBxVRLGlTfY8g057h2-m8qVraa-cquZdJd9SeiRuTnfTxzmsPc23RQ

=== Example 1: Begin Transaction === Transaction ID:
487b17da-c364-499b-93ba-2117b8763c6f

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"value":700,"type":"Integer"}

=== Example 5: Rollback === New transaction:
9e16d49e-4f05-4c99-bb20-d06f6b76c979 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 9[0m
[32mPassed: 9[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 837ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 647ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'FeMH7q9_KGxyHPTXbVIvI695kPaf16KtcvN-KsRie6LvNZKq3dPnae5YDzBVzEvciqFt-AVWTR1aDqeGrn07Tw'
}

=== Find by ID === Found: { id:
'FeMH7q9_KGxyHPTXbVIvI695kPaf16KtcvN-KsRie6LvNZKq3dPnae5YDzBVzEvciqFt-AVWTR1aDqeGrn07Tw',
value: { type: 'Integer', value: 42 }, name: { type: 'String', value: 'Test
Record' }, active: { value: true, type: 'Boolean' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { value: true, type: 'Boolean' },
name: { value: 'Updated Record', type: 'String' }, value: { type: 'Integer',
value: 100 }, id:
'FeMH7q9_KGxyHPTXbVIvI695kPaf16KtcvN-KsRie6LvNZKq3dPnae5YDzBVzEvciqFt-AVWTR1aDqeGrn07Tw'
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

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
zEIqyJJtGcPS6voPIdbnX9PNYhc1fnMQOvpcleotn8yj3zfvHfjaKbcMbtUr_8U6kStKy6g3vai1SO-4X3VkWg

=== List Collections === Total collections: 23 Sample collections:
schema_employees_client_py,schema_products_client_go,schema_documents_client_go,schema_users_client_py,websocket_test

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
9hVVM7tElARgkXegmPJveQ_wcdHfDiZa-t5-vPhfujI-ix91WNhQjZDzGhy_72Dqt7VNTqZpnz1-VbDkBSX0sg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
-wPdwTw4_YFkenVZLPoZx7hOhmHIR6rc_66UIVMAYP8C2f31EOArptLAbDikn_ns7zwzBALv6iAmarl2ejzpLg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
1naraKvXyg4FfbMBtMzEXB1afxNj5g7RFOKF0HcijS6iD5SKkltlqEtbKOaRM3-ALGB_PVIPCgz6CS3ue5j3LQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
wzMUuw8872R7vzQ0Fi1V_eS3-EBz6jmNQvelEodtGnvgabmWz6GkYlXh_Ju28DXspzdV3eU1BO8fAvVZe7Qtow

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
fUswEU3lAdFhqxiMBSHjfVxgLLXTBBZDjxZIHH2C0v3HlRoMg18ss-vBwGlaRl8fzpuf1GRWBfkXGskOJG03Hg
📊 Found 30 active users

📝 Example 2: Complex Filter with Multiple Conditions

✅ Script saved:
aRK3OQJD07eUFHT6VmjTILw5TGdnn21iB_0L6XkvupCsHHdeaisaVuwVHGpctItZGkkBiqZ528dh1G3bz0bXIw
📊 Found 9 users (status=active, score>50, sorted by score)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
NuQ8McDGqYz1cFQOqpfHZJLQg9ts57G9r_nSl_vrd2J3POK1O_mdAfhQ_5A1e-nV-wpJElV00M24ff2NfBAotw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"status":"active","max_score":100,"avg_score":60,"count":15}
{"status":"inactive","count":15,"max_score":90,"avg_score":50}

📝 Example 4: Function Management

📋 Total scripts: 21 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
iPyPp-EcKkJb1EjubfS2-DTGgUf4Nu0zTCbiIU5dul19-w3U9qvCdiSQWhAecM6SbvjYujU55PfjYn6n3Vdiow
📊 Found 40 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 40 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":20,"status":"inactive","avg_score":50}
{"status":"active","avg_score":60,"count":20} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 23 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 40 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • ChatMessage helpers • Cleaner, more maintainable code •
Built-in error handling 🚀 ekoDB Advanced Scripts Example

📋 Setting up test data... ✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved 📊 Found 10 products ⏱️ Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved 📊 Found 2 categories Electronics: 6 items (avg $325.67)
Furniture: 4 items (avg $294.00) ⏱️ Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved 📊 Total products: 10 ⏱️ Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved 📊 Category analysis (2 categories): Furniture: Products: 4 |
Stock: 43 | Avg Rating: ⭐4.26 Electronics: Products: 6 | Stock: 232 | Avg
Rating: ⭐4.52

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Bookshelf - $149 (⭐4.1)
2.  Standing Desk - $599 (⭐4.7)
3.  Wireless Mouse - $29 (⭐4.5) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All advanced script examples completed! 🚀 ekoDB CRUD Scripts Example

📋 Setting up test data... ✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved 📊 Found 10 users ⏱️ Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved 📊 User counts by status: inactive: 3 users active: 7 users ⏱️
Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved 📊 Average scores by role: admin: 20.0 (3 users) user: 70.0 (7
users) ⏱️ Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved 📊 Users (showing first 5 of 10):

1.  User 7 (user7@example.com) - Score: 70
2.  User 2 (user2@example.com) - Score: 20
3.  User 5 (user5@example.com) - Score: 50
4.  User 9 (user9@example.com) - Score: 90
5.  User 3 (user3@example.com) - Score: 30 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Getting Started with ekoDB (Database)
2.  Vector Databases Explained (Database)
3.  Introduction to Machine Learning (AI)
4.  Database Design Principles (Database)
5.  Natural Language Processing (AI) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: Database: 3 documents AI: 2 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Getting Started with ekoDB
2.  Vector Databases Explained
3.  Introduction to Machine Learning
4.  Database Design Principles
5.  Natural Language Processing ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Getting Started with ekoDB
2.  Vector Databases Explained
3.  Introduction to Machine Learning ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Getting Started with ekoDB (Database)
2.  Vector Databases Explained (Database) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Scalability: Vector databases can handle large amounts of data and scale up
   efficiently.
2. Precision: They maintain high precision and quality of data, without any loss
   of details.
3. Flexibility: They allow for on-the-fly alterations and manipulation of data.
4. Compatibility: Vector databases are compatible with various data models and
   can integrate different types of spatial data.
5. Complex Analysis: They are suitable for complex spatial queries and analysis.
6. Memory Efficiency: They are more memory efficient for representing sparse
   data. ⏱️ Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Advanced Query Patterns" - 1536D vector
2.  "Draft Article" - 1536D vector
3.  "Getting Started with ekoDB" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m
