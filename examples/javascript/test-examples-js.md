make test-examples-javascript 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 717ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:2947) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 7 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:2951) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'GsstunTtWWub3fFPbAUYdRZgdrtZUU7UFy5sQlXcgeqPNuQTlcK53HFIiPVlW_XU0Ed4nYSrprOk_EO5BN1rqA'
}

=== Find by ID === Found: { name: { value: 'Test Record', type: 'String' },
value: { type: 'Integer', value: 42 }, id:
'GsstunTtWWub3fFPbAUYdRZgdrtZUU7UFy5sQlXcgeqPNuQTlcK53HFIiPVlW_XU0Ed4nYSrprOk_EO5BN1rqA',
active: { type: 'Boolean', value: true } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'GsstunTtWWub3fFPbAUYdRZgdrtZUU7UFy5sQlXcgeqPNuQTlcK53HFIiPVlW_XU0Ed4nYSrprOk_EO5BN1rqA',
name: { type: 'String', value: 'Updated Record' }, value: { value: 100, type:
'Integer' }, active: { type: 'Boolean', value: true } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:2971) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
NEqmdjgxKpektKkOQ871HMEMcX7i6kTHpDFBK39sEnKikL1mbuJtsaczQGjNPKpPJTckiC1YJCWwlSun-42kgA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"bk4bCkONct5iiO7cGmRUEYih--5u3TSYBQBvnBf7oLbPFW8O9KekSQ_V6NNbM5JjduSNOA-TQs5W87l0V4qeMg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"ygX4zwfZD59Pc7Wlu1yNBjgmQZtkOpiXEpwp9VZMKq1H-3SZoGHnVaSVZv5jcMnvUabr6bpopfbc-Um7Ps9GQg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"NEqmdjgxKpektKkOQ871HMEMcX7i6kTHpDFBK39sEnKikL1mbuJtsaczQGjNPKpPJTckiC1YJCWwlSun-42kgA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"FcYv2T8043aRaJq2URI56f8dcaxvd3uHgbcAoZo0VouAL3Da5L5D5MCKSAf29hThOk0F5BSTyk10f8VG0EvsDA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:2978) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:2981) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { userId: 123, loginTime:
'2025-12-08T14:45:04.930Z', username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:2985) [DEP0040] DeprecationWarning:
The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record:
0kj7Gq37Vqw3MC9oqZ7IpfAMdm821OIyN2r_J_6VqD_pozU2nsKt6bbpQ55y78kOLiZxOeP3mYHeJz5D8yT8hA

=== List Collections === Total collections: 22 Sample collections: [
'schema_employees_client_go', 'schema_products_client_go', 'websocket_test',
'schema_documents_client_ts', 'schema_employees_client_ts' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:2992) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
v-T-MpVDfwKw6Z2NHouk770eduJSu2nBRmR_SmhUIEYo1KmcuQZNmc2zAzTZSQmA2-ldIeLNhxAO3C3H5mAVSg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:2995) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
S-8MANTOdmep1Isz98E4qvjtpA1YROEvxJIrlm-FWLJ44teAiu666tVObOB6yfh6TZvN7vKLlmvqusNUJwNimQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 886ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 672ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'NZfxrBi8xm--iVNcgyi_yaQXv_mPB3L5e3Nhb7-cYUd42wGuyAwAqoPmFNLEMebE4VsrPUqsbrxOLc5NMjlLyQ'
}

=== Find by ID === Found: { value: { value: 42, type: 'Integer' }, active: {
value: true, type: 'Boolean' }, id:
'NZfxrBi8xm--iVNcgyi_yaQXv_mPB3L5e3Nhb7-cYUd42wGuyAwAqoPmFNLEMebE4VsrPUqsbrxOLc5NMjlLyQ',
name: { value: 'Test Record', type: 'String' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { value: 100, type: 'Integer' }, name:
{ value: 'Updated Record', type: 'String' }, active: { value: true, type:
'Boolean' }, id:
'NZfxrBi8xm--iVNcgyi_yaQXv_mPB3L5e3Nhb7-cYUd42wGuyAwAqoPmFNLEMebE4VsrPUqsbrxOLc5NMjlLyQ'
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
4vjf72_iCvvJtxUVSvLFVsluFi8yNyUkQXbmCw-ji5tEaUjDj5khDu7YIbBn78amoXndyN1RbH12bydGgd20kA

=== List Collections === Total collections: 22 Sample collections:
schema_employees_client_go,schema_products_client_go,websocket_test,schema_documents_client_ts,schema_employees_client_ts

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
mAvJP-DB4-do4GEE-\_uh7CVaiua2wd15aDuTuvlgKdG5VZ_0N6N-NSST7WO-pJMfCS525DpVc9earadddhdxdg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
PrmFxT0ZeKiQHF8TiE2pQHt6d8ck3ZdBPPyk8pEFCkvK3vpkHOhVPAMYTTZvpWuv2BxH51oX8XffiAQMKMW2PA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
BbgPVhfNw-v9RBL3Sm6zWsDBJ-9Qo6881JeC4TtDQoaOlgz5quqBOa05HvfUgI5h7YtOd8XBn1y20SNb1x8WPA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
VbDgbQtUQdDgeiv7cuMgkidVV_b6J_eQgK03hSGNFgN6ikGaxmH-2ry8NnIIel49zNlsPMpQqKKFZKEGI9--4A

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Script saved:
4o8u0kGMDA7QB*m2U5IV1636n8lMTWg1px-kQ1Ce3Djaib*-7PCYH8-wDedIdWsyIwioaTD1ObnA43v-GKdWjw
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Script saved:
B1viRt2itrLG5iwpiQP7AnH9_YAPFCvWr2vae0ZZaagSj_iuvn8ihC2K2RAFB6qP1NiPmniJbB8GXP_S3iMYXA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Script saved:
xH-PIcXArh2dfv9qLQLY3M5rasVM1m9yK68u60QiRr-c_iwRJZ7e8A4G0_IphmNQ2Yl3a1Zy717nvDEDSLINhw
📊 Statistics: 2 groups {"count":10,"avg_score":60,"status":"active"}
{"status":"inactive","count":10,"avg_score":50}

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
Kwu0CMhcp20564TEK5c_IjTIBn0yqEzLqJM-3UWZq2UJ6iqm_JvlGvMjuwgwUSedy8h_JJqjRrpxJl91M3Wfdg
📊 Found 30 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 30 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"avg_score":60,"status":"active","count":15}
{"count":15,"avg_score":50,"status":"inactive"} ⏱️ Execution time: 0ms

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

✅ Script saved 📊 Category analysis (2 categories): Electronics: Products: 6 |
Stock: 232 | Avg Rating: ⭐4.52 Furniture: Products: 4 | Stock: 43 | Avg Rating:
⭐4.26

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Wireless Mouse - $29 (⭐4.5)
2.  Standing Desk - $599 (⭐4.7)
3.  Laptop Pro - $1299 (⭐4.8) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All advanced script examples completed! 🚀 ekoDB CRUD Scripts Example

📋 Setting up test data... ✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved 📊 Found 10 users ⏱️ Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved 📊 User counts by status: active: 7 users inactive: 3 users ⏱️
Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved 📊 Average scores by role: user: 70.0 (7 users) admin: 20.0 (3
users) ⏱️ Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved 📊 Users (showing first 5 of 10):

1.  User 8 (user8@example.com) - Score: 80
2.  User 5 (user5@example.com) - Score: 50
3.  User 1 (user1@example.com) - Score: 10
4.  User 2 (user2@example.com) - Score: 20
5.  User 3 (user3@example.com) - Score: 30 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Vector Databases Explained (Database)
2.  Database Design Principles (Database)
3.  Getting Started with ekoDB (Database)
4.  Natural Language Processing (AI)
5.  Introduction to Machine Learning (AI) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Vector Databases Explained
2.  Database Design Principles
3.  Getting Started with ekoDB
4.  Natural Language Processing
5.  Introduction to Machine Learning ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Vector Databases Explained
2.  Database Design Principles
3.  Getting Started with ekoDB ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Vector Databases Explained (Database)
2.  Database Design Principles (Database) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Scalability: Vector databases allow for efficient scaling as the amount of
   data increases.
2. Precision: They provide a high level of detail and accuracy in representing
   data.
3. Dynamic: Vector databases allow for dynamic queries and analysis.
4. Supports Complex Geometries: They are capable of representing complex
   geometrical features like points, lines, and polygons.
5. Less Distortion: Unlike raster data, vector data doesn't lose resolution when
   zoomed in or out. ⏱️ Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Advanced Query Patterns" - 1536D vector
2.  "Draft Article" - 1536D vector
3.  "Getting Started with ekoDB" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m
