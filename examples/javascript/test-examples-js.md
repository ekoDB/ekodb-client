make test-examples-javascript 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 657ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:17313) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 7 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:17315) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'GM2z3W4zBlm-4BGEUyRKDlahIFEnbQTdwGOXhh5Mk66ouFk1HPuF7AaEq1p96XJO29ijpChKviphq_J8rU0qrw'
}

=== Find by ID === Found: { id:
'GM2z3W4zBlm-4BGEUyRKDlahIFEnbQTdwGOXhh5Mk66ouFk1HPuF7AaEq1p96XJO29ijpChKviphq_J8rU0qrw',
active: { value: true, type: 'Boolean' }, value: { type: 'Integer', value: 42 },
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { value: 100, type: 'Integer' }, name:
{ value: 'Updated Record', type: 'String' }, active: { value: true, type:
'Boolean' }, id:
'GM2z3W4zBlm-4BGEUyRKDlahIFEnbQTdwGOXhh5Mk66ouFk1HPuF7AaEq1p96XJO29ijpChKviphq_J8rU0qrw'
}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:17318) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
hVpqSXhpvTz7V3vDIscxjh4qx4qCstQGEAHrHjcRrRk3w9pt92Kqnh17YE9tHveGctaPKBqhOduLbW9zMIFNRA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"aRE3XVwlbTj5n7jpL8YazH5Sle1ShUD8o1xXCtJz3n3OMfAvG6SWCG1CUYNC0BVcjv19UXFhDXmpbPhqQ2iYgA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"hVpqSXhpvTz7V3vDIscxjh4qx4qCstQGEAHrHjcRrRk3w9pt92Kqnh17YE9tHveGctaPKBqhOduLbW9zMIFNRA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"PhhdxEYbQszJEMgXIonEwzmyOwVtvGc1Yocd1cTDxLWhJrpoLdEWJOyKQ864_IEWJUlyIlVlbZH6TB-II9J5bQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"P7tzKk-WVv2ZyOr-TimEHkFoy3v_DOuYNWiVtY4ylJowBxvP5zi3OBl9U3QFj4kRZo5NQGmwIdpc3gvO_se9AQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:17331) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:17333) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { userId: 123, username: 'john_doe', loginTime:
'2025-11-10T04:25:17.228Z' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:17340) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
BCsO-rKuX8SsKq-eO102pDt9Zes5UAWP4I4dDaw42thov1rvW8inTyYoHChUE0gqeJX3em3h4VyiTeGRevnQ0w

=== List Collections === Total collections: 22 Sample collections: [
'ws_ttl_test', 'schema_documents_client_ts', 'schema_products_client_ts',
'schema_documents_client_py', 'batch_users' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:17347) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
CDlGnXTHuLgUya7RN8t5QLd4g3aJBhR5KA6xxarCDRbIj5RBpIvzzYyPhRoYiuDKSOym5ht7-10VgZAN6yMICw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:17349) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
N8rB-o_XdPtKYE84_97JVDou5OEwYbuKeLzq_52EYstxCaCCxnLDd22KxJ_5VSFLjtg8KUfRBHB09tHrsIyPfA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 870ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 744ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'-h6jiQJPYr5KcnJp8mjfRlGi10qMuh92P2z4wng4aM_oFOJAARtnRhSKVaJOeCIG9ZS57pJw6a2VZop9S0f7qw'
}

=== Find by ID === Found: { id:
'-h6jiQJPYr5KcnJp8mjfRlGi10qMuh92P2z4wng4aM_oFOJAARtnRhSKVaJOeCIG9ZS57pJw6a2VZop9S0f7qw',
value: { type: 'Integer', value: 42 }, name: { type: 'String', value: 'Test
Record' }, active: { type: 'Boolean', value: true } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
name: { value: 'Updated Record', type: 'String' }, id:
'-h6jiQJPYr5KcnJp8mjfRlGi10qMuh92P2z4wng4aM_oFOJAARtnRhSKVaJOeCIG9ZS57pJw6a2VZop9S0f7qw',
value: { value: 100, type: 'Integer' } }

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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
Vxl2x5nmzyv50YMJD7DQVtJHrRfaFaDawoSXx3x7lKhCqtaLKYYFlvZkMWL2sdGottqSKOXO8bGTLgqXn9Of1g

=== List Collections === Total collections: 22 Sample collections:
ws_ttl_test,schema_documents_client_ts,schema_products_client_ts,schema_documents_client_py,batch_users

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
y6luYg11pmmpDEcT-8KYqigIA1934WHUNJ51xhDCdkFR_HPQXSHiQ-UeffauXQIBQEiDuXdJXXREHbGfXqh2xg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
eitRi_ZZTHQQqEh2K1hDtPZMSHMDjdJmsPopAdo37LMG-9B2J1A_2AJPf5OG3vWhGvp4G5JGAhme8vXR9HgM8A

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
brqmMdBqrJaejqgCyonw4Nl-iLKzOZdW0KHAJoBYfyEaT9QiSrSUkIbeCVGuAja_MH4LLy6eeu7mXozeBmWC9Q

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
34lr6nNUwcrJCbr6kkCp7US3ocXNQ5pfJHIPKmIav68xhWWyn4GAtnkfJUka7_GcifJSf8pYvLESdOsr0h-JhQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Script saved:
dQA8PAbgHy1JtrTScJpWdDloJWCT2W5KGJbRH61E56D1feoXpOOmYWbCPW-85H7oT6LMeOHEEhNKFcUd324AMw
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Script saved:
i0hPOH_5KXS37AnIJMU0mowfKk_P07FCAj057iJy63V2nEAnUJTxoLKXdc0_qGzYp_lgzifjR5REXvnO9iPg9g
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Script saved:
SStmYK6UbYkWuMzujuHB7ezPmCgJl8xit1JGWTCTs-ETW5Z8UTD-ngFbl6piRiNj41rCk_ylOIZnzKDh2eRwlQ
📊 Statistics: 2 groups {"avg_score":50,"count":10,"status":"inactive"}
{"avg_score":60,"status":"active","count":10}

📝 Example 4: Function Management

📋 Total scripts: 13 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
KpNpC_kP7h3OameA8aqm7TQpq8ID0OSBuZwH--gN79-sLxs3dUp16M2jerRzfTJtCvb3-4FFSnr9E5sW2P-mtg
📊 Found 30 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 30 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":15,"status":"active","avg_score":60}
{"avg_score":50,"status":"inactive","count":15} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 15 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 30 ⏱️ Execution time: 0ms

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

1.  Keyboard - $89 (⭐4.4)
2.  Laptop Pro - $1299 (⭐4.8)
3.  Monitor 27" - $399 (⭐4.6) ⏱️ Execution time: 0ms

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

✅ Script saved 📊 Users (showing first 5 of 0): ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Introduction to Machine Learning (AI)
2.  Natural Language Processing (AI)
3.  Vector Databases Explained (Database)
4.  Database Design Principles (Database)
5.  Getting Started with ekoDB (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Introduction to Machine Learning
2.  Natural Language Processing
3.  Vector Databases Explained
4.  Database Design Principles
5.  Getting Started with ekoDB ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Introduction to Machine Learning
2.  Natural Language Processing
3.  Vector Databases Explained ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Introduction to Machine Learning (AI)
2.  Natural Language Processing (AI) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Efficiency: Vector databases allow for efficient querying and retrieval of
   spatial data.
2. Precision: They provide higher precision for geographic data, ensuring
   accurate representations.
3. Flexibility: Vector data can be scaled up or down without losing resolution
   or detail.
4. Complexity: They can represent complex geographic features such as points,
   lines, and polygons.
5. Attributes: Vector databases allow for easy attachment of attributes to
   geographic features.
6. Analysis: They support various spatial analyses like network analysis,
   proximity analysis, and overlay analysis. ⏱️ Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Draft Article" - 1536D vector
2.  "Getting Started with ekoDB" - 1536D vector
3.  "Advanced Query Patterns" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m
