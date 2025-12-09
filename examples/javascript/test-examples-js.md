make test-examples-javascript 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 651ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:26319) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 9 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:26322) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'yXnIhKW4M_h0JtyJESZyji4e79a6wBTmsyl_V0T7SEjYWOcQhg4W5zpUtYKxJjEjYVI3BU0ZMm9FdUPp29YU3A'
}

=== Find by ID === Found: { name: { value: 'Test Record', type: 'String' },
value: { value: 42, type: 'Integer' }, active: { type: 'Boolean', value: true },
id:
'yXnIhKW4M_h0JtyJESZyji4e79a6wBTmsyl_V0T7SEjYWOcQhg4W5zpUtYKxJjEjYVI3BU0ZMm9FdUPp29YU3A'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
value: { type: 'Integer', value: 100 }, name: { value: 'Updated Record', type:
'String' }, id:
'yXnIhKW4M_h0JtyJESZyji4e79a6wBTmsyl_V0T7SEjYWOcQhg4W5zpUtYKxJjEjYVI3BU0ZMm9FdUPp29YU3A'
}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:26335) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
A1pbDsJbqPKfY4-SKfOeuctbX-rKBkMv1luw6vpzCdlq26DFb0MNtnJVrugvGVKbhpTLoXcTh58sJ_vU4QSS1g

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"7mvDvjLu-3eoQZwjlZon8ia6SymRSNNQo0VlDw_zXmr7rkofCdE4da0lfkWdQKmCUTscj8culXnsEWFRpq3cCw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"A1pbDsJbqPKfY4-SKfOeuctbX-rKBkMv1luw6vpzCdlq26DFb0MNtnJVrugvGVKbhpTLoXcTh58sJ_vU4QSS1g",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"Xmfr_Z2upSSFQAcp-owmbvissITLMYTXOLA0QCeJe1vZiQq6v5nK8U39FZycJMcGfHfeCzslbFfsriw4sfw7VQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"-StpvC0i8Gt6eyenL3QP9h3rW2r0I9qfrnNaba88DIxHpZPxfUglMnSlcwVMvT1ZWYpBf50k510Fiy7LaTQCaQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:26342) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:26345) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-09T05:40:14.187Z', userId:
123, username: 'john_doe' }

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

=== Create Collection (via insert) === (node:26352) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
u5VtRZhI7H1xR2G_uBbGoixV18NGoSC8_hebKgdhoUoziaYE3ZM2xDjX5RyqVqdxMSqoFZrzNUX1XvIPEro7dA

=== List Collections === Total collections: 23 Sample collections: [
'schema_products_client_ts', 'users', 'schema_documents_client_py',
'schema_documents_client_go', 'demo_collection' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:26355) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
fjFQ_EEs9KunXb3ssFc9MLnh-1x16A1VhcZyqjlmUYdCK3k3XWw0tlxuVvlXN61mxmWWrCbpYIV8VCML_75lkg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:26362) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
KQV9PawhitGmg3NXwL2_UgEUfs0F_WpQz0RMjhYM28CQIGEkmBkaTP-rNkNWpnQBZ41gUneN8Be3CZ9dxQq-yA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Script saved:
BcsESSowQ1HTy\_-T6kyT7x9AfiUDPJzhGuFjmUe8yKaTJ98tcxg-kyh7uyK3otWXKeXXu1YUPD3iowGsbflONw
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Script saved:
Oop9mbvL2-tbDgzHepw0Q1MiXIbP_pLlt6MvRm6cUyEGilh1Lnwx8TxiH0g6w606mVjZE7oOq-NWmpwBWKR5QA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Script saved:
EzNksXVaQ5dMKlPoDR_kLKuXViTTqgZRuYjfy3DAE3R_kZfgOqagxyYGmGfICkem-JBLiPxiyw6AdATFn2XqVg
📊 Statistics: 2 groups {"status":"inactive","count":10,"avg_score":50}
{"status":"active","avg_score":60,"count":10}

📝 Example 4: Function Management

📋 Total scripts: 13 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.js completed successfully[0m
[34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js
===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
Bwweft8xib18fG1fZPRfV8n_gc0FfcKDWOZllgSeU9y2rLkw99EDgB66KlLbDRBAzAwHLJd3SdOt1NBP3xnROQ
Created Bob: $500 - ID:
5z1am5csUiYq0_NQ6dijpV0aAH5rBw-n98aj59BkS3tGT6k_CFk0bhuQFiMhshgkN272RDpIIftlhnprQD7nDQ

=== Example 1: Begin Transaction === Transaction ID:
f5335fe4-d9c2-48fb-85df-39178b611847

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"type":"Integer","value":700}

=== Example 5: Rollback === New transaction:
7af84d93-e71c-40a5-9c72-e88e4f78caaf Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 9[0m
[32mPassed: 9[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 740ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'Z35cPTqTpyzwuAXBHTG80wSTbQi4uI6b-MItF4ZzQMDmGEg9ykMT5GZvTUwfu1C_qEzzUu8sK7VWnQwSeOtZUA'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' }, id:
'Z35cPTqTpyzwuAXBHTG80wSTbQi4uI6b-MItF4ZzQMDmGEg9ykMT5GZvTUwfu1C_qEzzUu8sK7VWnQwSeOtZUA',
value: { value: 42, type: 'Integer' }, active: { type: 'Boolean', value: true }
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
value: { type: 'Integer', value: 100 }, name: { type: 'String', value: 'Updated
Record' }, id:
'Z35cPTqTpyzwuAXBHTG80wSTbQi4uI6b-MItF4ZzQMDmGEg9ykMT5GZvTUwfu1C_qEzzUu8sK7VWnQwSeOtZUA'
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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
4pvlDp5r6DxEo4Sprica6aWqAaoxUrLvtSb9irj6P5lK6TIVWR2Mi17DJ4nNikjoxEkUDI6udtvQvsS_R0oUbA

=== List Collections === Total collections: 23 Sample collections:
schema_products_client_ts,users,schema_documents_client_py,schema_documents_client_go,schema_users_client_ts

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
BgqvCu9lVggY8LDhgug1cYKWpKvIh2CnmMAHcZIyMBkSsG4PilLNOFby1iXXNS62FV9skyLQuxPDVt5cshfXvw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
t4AShukRaFzk4pOKwrImk0Es9fV5HE1VCOaAIzcc1u4ZCR8UMNLYtkpfAZzU6H2QKnNufDBFPptgwnBEjNLKoA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
amYwdF8YzC-a9JyaZ5ddHm79tOlUZMMgrXndzMDp*ktkBvHc3aW76*-Dfr5mk8b7zAF2WNnmpyZ7eHwNJxjCXA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
l7xt9-MtDJRIZkK3vPbCE92B94P3F35eWSniO4FLgfbfFdRSGCdJgkWoJ3fy6MK6CiG-1lpzSuCSp6YvC7uCnQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Script saved:
c7BdNiK5_AnbNuc7KoTg2a51OEuoi6UWXBS_nCLyRyvG4XArXNEBClzilgLxRfIR5jhv7xf4VVV6lM1kggFdFQ
📊 Found 30 active users

📝 Example 2: Parameterized Function

✅ Script saved:
qlYrzyVNxYA7kklGdcpDGZA1JQwmiSlgN1iQZb3OmLeP8TixbAUKz0WxZfqO65Ptj-NBccmi1SGKjRxQ4cGaIw
📊 Found 30 users (limited)

📝 Example 3: Aggregation Function

✅ Script saved:
fcb4A2jNJN_P9M49PeYP_1xjBPChMTAmlWDTYiLbp1Ed8QYQ6bU6FohuyaT_cFuNC6tNnO8I4d6cWt5AwSUVjQ
📊 Statistics: 2 groups {"count":15,"avg_score":50,"status":"inactive"}
{"avg_score":60,"count":15,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 15 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
\_VvQsHpCuw075Mow0zJE33mrtjW0Vs05EXtl2v001oyl-J8-AXCrK2QnEzuIYy2zQHQKdhVXPgLfKe67aSRXYw
📊 Found 40 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 40 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"avg_score":50,"count":20,"status":"inactive"}
{"avg_score":60,"count":20,"status":"active"} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 17 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
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

✅ Script saved 📊 Found 2 categories Furniture: 4 items (avg $294.00)
Electronics: 6 items (avg $325.67) ⏱️ Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved 📊 Total products: 10 ⏱️ Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved 📊 Category analysis (2 categories): Electronics: Products: 6 |
Stock: 232 | Avg Rating: ⭐4.52 Furniture: Products: 4 | Stock: 43 | Avg Rating:
⭐4.26

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Desk Chair - $349 (⭐4.2)
2.  Monitor 27" - $399 (⭐4.6)
3.  Standing Desk - $599 (⭐4.7) ⏱️ Execution time: 0ms

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

1.  User 5 (user5@example.com) - Score: 50
2.  User 6 (user6@example.com) - Score: 60
3.  User 8 (user8@example.com) - Score: 80
4.  User 1 (user1@example.com) - Score: 10
5.  User 2 (user2@example.com) - Score: 20 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Vector Databases Explained (Database)
2.  Introduction to Machine Learning (AI)
3.  Database Design Principles (Database)
4.  Natural Language Processing (AI)
5.  Getting Started with ekoDB (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Vector Databases Explained
2.  Introduction to Machine Learning
3.  Database Design Principles
4.  Natural Language Processing
5.  Getting Started with ekoDB ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Vector Databases Explained
2.  Introduction to Machine Learning
3.  Database Design Principles ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Vector Databases Explained (Database)
2.  Introduction to Machine Learning (AI) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Scalability: Vector databases can efficiently handle large datasets due to
   their spatial indexing capabilities.

2. High Precision: They provide high accuracy and detail, as vector data is
   represented in points, lines, and polygons, which maintains the preciseness
   of the data.

3. Less Space: Vector databases require less storage space compared to raster
   databases.

4. Analytical Capabilities: They support complex spatial analyses, such as
   network analysis and proximity analysis.

5. Visualization: Vector data is easier to visualize and interpret, providing
   clear representations of geographical features. ⏱️ Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Draft Article" - 1536D vector
2.  "Getting Started with ekoDB" - 1536D vector
3.  "Advanced Query Patterns" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m
