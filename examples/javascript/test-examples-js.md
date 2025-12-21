make test-examples-javascript 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 790ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:63994) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 10 Examples ===[0m [34m ===
Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:63997) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'FQopvvtve-kJBuBgvgoB-yYDjLyrJOqwQ1pGz7xvl_acFnquFebroKQSZXcRu2XQYrYf5dCKLW-Vafz2nOEk1Q'
}

=== Find by ID === Found: { value: { type: 'Integer', value: 42 }, name: {
value: 'Test Record', type: 'String' }, id:
'FQopvvtve-kJBuBgvgoB-yYDjLyrJOqwQ1pGz7xvl_acFnquFebroKQSZXcRu2XQYrYf5dCKLW-Vafz2nOEk1Q',
active: { type: 'Boolean', value: true } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'FQopvvtve-kJBuBgvgoB-yYDjLyrJOqwQ1pGz7xvl_acFnquFebroKQSZXcRu2XQYrYf5dCKLW-Vafz2nOEk1Q',
value: { type: 'Integer', value: 100 }, active: { type: 'Boolean', value: true
}, name: { type: 'String', value: 'Updated Record' } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:64000) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
JcFanAYz014N6pzJG3QUhCU3QaejJx9ZcgC7FIPzG67u0P5jgsCWecU6GF3PYE39p-6KAyb-zX24QL7w87F9rQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ZcYBdW4BqNkDzf5RbWUTc1jIg4MNTZSmLAIkKJV-qyVNSV9E7XHRzPgh3hNqd7aO-Wc1hgdTsZeNeSg32CNDwg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"qQFo7AuYWumBE-nyEtkv0vnDprHbBQs6MyvSoxLr6x1dPGKK912zpqx7rJPxDOhxF1hw2rZdIWEl5BK9pSW0TA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"UAcR9Yc_QZABIRubc0RoEp8eawAI-On9ZRl3kxmnfsYBDHLvLFzM9aWMaEkGLksmTlL4s2AxDw-B-UvQT1QkXQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"JcFanAYz014N6pzJG3QUhCU3QaejJx9ZcgC7FIPzG67u0P5jgsCWecU6GF3PYE39p-6KAyb-zX24QL7w87F9rQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:64013) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:64017) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-21T00:49:32.912Z', userId:
123, username: 'john_doe' }

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

=== Create Collection (via insert) === (node:64020) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
VVooCLD_QXAC1fkb-SUpcxixiTQyC-TxpGU62Yo6Fo1j0zrt6IkrGRNdJID7xWTTBt9DSm075iusSmmpvYj4wg

=== List Collections === Total collections: 23 Sample collections: [
'schema_documents_client_py', 'websocket_test', 'schema_employees_client_py',
'demo_collection', 'schema_documents_client_go' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:64033) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
0tfzsFT_XSLvsXPfj3JuN6A8CCP-lxnk9b7Uoenkm8sFHYztZAppPZ3g7K_zaKvK8xtsJns5WLpzu4rjvG9SzA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:64036) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
BWHtkjYcF7JQltQs7ig8y-Cu6Ta8zSsJNG7qltmZE1Nw7suC8Jz7eOq2nsRh8yHRi5dNbEsguFM_ICT3BTQTxg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
TwbA-YCUH9gM-BIcW9jqQqcOO6q5VVq5np58FYGlQiuhqz0cwhJaLNTR0lOGLEsR9okn7fLYQ5iPd_0MXvuxmA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
h4tFVxWW6F2WbJY6wBmZi3H2wH_oFtdkebcSV9NZck_Zb9ZsL273ewQ8IopmI4s5VZWTkSCKmwa2YM9XuoIJpw
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
DeoAs9IpaLhB5KLqPW6RPicijapUBRkHbkgPHxXLSoUSfVWM2Kom6u_PbskILIJG-4652YnYq_tm-tTcJ4aUZQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":60,"max_score":100,"count":10,"status":"active"}
{"status":"inactive","avg_score":50,"max_score":90,"count":10}

📝 Example 4: Function Management

📋 Total scripts: 20 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.js completed successfully[0m
[34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js
===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
2dK6LvJlI2264a-ai1HCkDEGQuSudptuuiZ9jj4kIVT23-gqBOvqJW4EFSsTv46AYx9eD0_9YImaZf8yCElQuQ
Created Bob: $500 - ID:
QLkBPRRSWOqXYDCkiUM6Ql6V1FqSBBU43rQNFBbGPROrX_maunfIW3pWs13FMnwNsihgJButTXFZjTy8sTT2-w

=== Example 1: Begin Transaction === Transaction ID:
78bc9a67-af0c-4c56-9a89-493a59048b57

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: 800 Bob: 700

=== Example 5: Rollback === New transaction:
4faea1b4-5b3d-4b96-87ec-c84966fa178c Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: 700

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_scripts.js
===[0m 🚀 ekoDB Complete CRUD Scripts Example
============================================================ Demonstrates: •
Insert + Verify (using Query) • Query + Update Status + Verify • Query + Update
Credits + Verify • Query Before Delete + Delete + Verify Gone

# Each Script shows Functions chaining with proper verification

============================================================ 📝 Script 1:
Insert + Verify ============================================================

1️⃣ Saving Script... ✅ Script saved:
hxupRm0IXCvZcl2RrZBz7KlnkgRfCdXJeaeMskrwf96w0qRylncWWEuf-RzwysmtUXIHca2MfzPs6G-FwOxu5Q

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Name: [object Object] 📋 Email:
{"type":"String","value":"alice@example.com"} 📋 Status: [object Object] 📋
Credits: [object Object]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
\_YoPgd4rux1ReycSVLXwObCMhZPEfIhAK6GcjoW-i9ZB_yIfzSEPKrMPKBKvrtl4vaKQdFczSUCN6NrlKvXrgQ

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
{"type":"String","value":"active"} 📋 Name: {"type":"String","value":"Alice
Smith"}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
ILCY5aHf--hggnvnseSBsYxq0TI-bblZlda8fbrlq-9P1H96uaK7pB-1BlSJspv2E6nNHguW_oon8nKTEUyWyw

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
{"value":0,"type":"Integer"} 📋 Status: {"value":"active","type":"String"} 📋
Name: {"type":"String","value":"Alice Smith"}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
VGR7tb6S51WiMhb64sFD92bDLZmgz3GKF1IpFQkaV5FI49i-r23Ss7-6BRdNC-ywo0MAApQbZuvzP0PFvuUvvQ

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
hxupRm0IXCvZcl2RrZBz... ✅ Deleted script: \_YoPgd4rux1ReycSVLXw... ✅ Deleted
script: ILCY5aHf--hggnvnseSB... ✅ Deleted script: VGR7tb6S51WiMhb64sFD... ✅
Deleted collection: users

============================================================ ✅ Complete CRUD
Scripts Example Finished!
============================================================

💡 Key Takeaways: ✅ Scripts chain Functions together ✅ Each Script
demonstrates operation + verification ✅ Parameters make Scripts reusable ✅
Verification is built into the Script itself ✅ Complete CRUD lifecycle in 4
focused Scripts [32m✓ crud_scripts.js completed successfully[0m [34m
╔════════════════════════════════════════╗[0m [34m║ Test Summary ║[0m
[34m╚════════════════════════════════════════╝[0m [34mTotal: 10[0m [32mPassed:
10[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m 📦
[36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.4.0 prepare npm run build

> @ekodb/ekodb-client@0.4.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.4.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 783ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'wMY6lw0l2-EWlDcSgmG9SvQVqEgunCEwcon5lwLv0Hx3-1YVWIKcM3_2Y-eQisgCykg_atM3vptpHHFbqvV_UQ'
}

=== Find by ID === Found: { name: { value: 'Test Record', type: 'String' }, id:
'wMY6lw0l2-EWlDcSgmG9SvQVqEgunCEwcon5lwLv0Hx3-1YVWIKcM3_2Y-eQisgCykg_atM3vptpHHFbqvV_UQ',
value: { value: 42, type: 'Integer' }, active: { value: true, type: 'Boolean' }
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'wMY6lw0l2-EWlDcSgmG9SvQVqEgunCEwcon5lwLv0Hx3-1YVWIKcM3_2Y-eQisgCykg_atM3vptpHHFbqvV_UQ',
value: { value: 100, type: 'Integer' }, active: { value: true, type: 'Boolean'
}, name: { type: 'String', value: 'Updated Record' } }

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
OCvTBXqIjbEbRK1A518wiAm70cS9B85VYdWYY3C7VRiF4kFogk-qiMc7G9WmAVF_Xmno0Euw8xJ4VIyRRJaYlA

=== List Collections === Total collections: 22 Sample collections:
schema_documents_client_py,websocket_test,schema_employees_client_py,schema_documents_client_go,schema_products_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
ErCcw5gT0GL24Tc5rsfFQNBT3y8KFM7SFq5d2OPfuCnTtdbQK6W38vdKZJx_MKpHssczjum55ur4LrntYdlJ-A

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
SpfB16ISZuXEsl-eIQ6iZ8fIhxXyERGY-oTBEDCNAde5q_jNs1IZYLE-d1sm6jQO_m0pfGSIy34NmM-zmwvOkQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
pjxjDCEgEPJzabFvRinkTT3XEsuiKuSG12QPrRuzejYRr8HmHH6Sc5DmLTZw11YxxsJnbaBdEwDQFTf5AGDZ2g

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
h9-8xkV5uIIkr_YZHFeVk_rsUxVkqJ_xjdx6OJDvT2iOFCd9k5wU_6hovnsb6UmOSFOiAZWBOEhmVX-CcO_MtQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
OMVywZrcSgn6P3uk8V_v7Q0S2I-oMZBHnZAtIybu4Ub5GpEYh_KozJxCZx6kDPreenqjozsD-9V1WaK9pu9ztg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
anjLadVtxO7rOvcFGFBsdhs1VWzqUCwGZiIOqvbOPHYzkH0blqtc1bjjxvNC0yVzfc6z0b5CJE2_6kiMf24T2Q
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
Xx-5HnsisvzTtRk6kMRpdIRUVeeX8cocmYTfdB3pACdn3mb0-kQOAf0HQql4gNLj_Dy7YPF-zpTT3CVCyBxYPg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"max_score":100,"avg_score":60,"count":5,"status":"active"}
{"status":"inactive","count":5,"max_score":90,"avg_score":50}

📝 Example 4: Function Management

📋 Total scripts: 22 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
EBZVIH9OCJ3pLhPero5WIzcva0S1ziouWPY-Ho1hRIhN0J0hSOoGtvH7-4JOOsmjfkhywOq6vXfWsks0b2MBBg
📊 Found 20 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 20 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":10,"status":"inactive","avg_score":50}
{"count":10,"avg_score":60,"status":"active"} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 24 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 20 ⏱️ Execution time: 0ms

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

✅ Script saved 📊 Category analysis (2 categories): Furniture: Products: 4 |
Stock: 43 | Avg Rating: ⭐4.26 Electronics: Products: 6 | Stock: 232 | Avg
Rating: ⭐4.52

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Desk Chair - $349 (⭐4.2)
2.  Standing Desk - $599 (⭐4.7)
3.  Keyboard - $89 (⭐4.4) ⏱️ Execution time: 0ms

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

1.  User 6 (user6@example.com) - Score: 60
2.  User 4 (user4@example.com) - Score: 40
3.  User 8 (user8@example.com) - Score: 80
4.  User 2 (user2@example.com) - Score: 20
5.  User 1 (user1@example.com) - Score: 10 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): inactive undefineds: 3 users, Total
Score: 180 active undefineds: 7 users, Total Score: 370 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Getting Started with ekoDB (Database)
2.  Introduction to Machine Learning (AI)
3.  Natural Language Processing (AI)
4.  Database Design Principles (Database)
5.  Vector Databases Explained (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: Database: 3 documents AI: 2 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Getting Started with ekoDB
2.  Introduction to Machine Learning
3.  Natural Language Processing
4.  Database Design Principles
5.  Vector Databases Explained ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Getting Started with ekoDB
2.  Introduction to Machine Learning
3.  Natural Language Processing ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Getting Started with ekoDB (Database)
2.  Introduction to Machine Learning (AI) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Efficient Data Management: Vector databases allow efficient storage,
   management, and retrieval of spatial data.

2. Scalability: They can handle large amounts of data without compromising
   performance.

3. High Precision: Vector databases provide high precision due to the use of
   coordinates and points to represent data.

4. Superior Analysis Capabilities: They offer superior spatial analysis
   capabilities like network analysis, proximity analysis, overlay operations,
   etc.

5. Visual Appeal: Vector data models can represent data more realistically,
   creating visually appealing maps.

6. Data Integration: They allow seamless integration of different types of data
   (point, line, polygon).

7. Less Storage: Compared to raster databases, vector databases often require
   less storage space. ⏱️ Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Getting Started with ekoDB" - 1536D vector
2.  "Advanced Query Patterns" - 1536D vector
3.  "Draft Article" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m
