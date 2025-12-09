make test-examples-javascript 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 638ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:2706) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 10 Examples ===[0m [34m ===
Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:2709) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'MarFFkwYBtYZj9Knl4GErjJgd1K9HknX9EwTLsER7PXmhPVkkmNUk5zJIbsYbqTVwa8-XyKjR_acVFNR7VNOAQ'
}

=== Find by ID === Found: { id:
'MarFFkwYBtYZj9Knl4GErjJgd1K9HknX9EwTLsER7PXmhPVkkmNUk5zJIbsYbqTVwa8-XyKjR_acVFNR7VNOAQ',
value: { value: 42, type: 'Integer' }, active: { type: 'Boolean', value: true },
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
value: { value: 100, type: 'Integer' }, id:
'MarFFkwYBtYZj9Knl4GErjJgd1K9HknX9EwTLsER7PXmhPVkkmNUk5zJIbsYbqTVwa8-XyKjR_acVFNR7VNOAQ',
name: { value: 'Updated Record', type: 'String' } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:2716) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
4yx8jCkldYJ3cSCgMHvAGndl1C9n2sQflx-zuD3OFEyAU1RMURnPMjjzhkHV3UpH03NR9TifKjAABtwxqQVI5Q

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ChsE4hhtNg8BkHvjsfyGVK_RCbVHQx8IaoOx1Vo0vx5l_0yuETorylmHzekR4I61txQOtNqmD1ntnmwC4zO4Nw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"4yx8jCkldYJ3cSCgMHvAGndl1C9n2sQflx-zuD3OFEyAU1RMURnPMjjzhkHV3UpH03NR9TifKjAABtwxqQVI5Q",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"G2EZWyV1Cqti1eOKOJQtnPuxXRm0BHD5rChCO4pYktf6M8SCa1YvUpMpN3sRzJctGTa8GF9hzyPmiwJZfj0q_w",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"p0WGdKAIQtu4s5ajCoE5oCqbhiPonpADBDtzNzVo2gEYwqeaFyLY3JmHjuNrJ0ywk4YSqv-4LoaE9RFYLVD3ww",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:2720) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:2727) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-09T18:40:53.513Z', userId:
123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:2730) [DEP0040] DeprecationWarning:
The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record:
CJl22S2mK56Kf-IC8yoA-oqn-vaA5uhwMOFeXNrzhctEliroSH3Y7cIo1vsdFP4iyu2DANTCbzgyic9gja6juw

=== List Collections === Total collections: 23 Sample collections: [
'schema_users_client_py', 'schema_employees_client_py', 'test_collection',
'chat_configurations__ek0_bench', 'demo_collection' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:2737) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
jtzib-fbYAKvn3e7fzZiJIOEtTSYbChhqXoEdTf7b7bIxdoj46kV5PxSoCuvAv0SXDj8cdaPapBhKnuZnlEkNA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:2742) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
fn18xskx_cL3kMQCen4v_H4KlB3nqgiEC-5Kc93pBdWtb2mgDj2YD9FLEidkEnzTXs3SPHMqk_p_vhyQ8smhPQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
5Yn0rEKxTQE20J50PZe6CuDuX4WZEC28hvo8MTOpz6tLrHMaYiWC-dXephSxugBfnuaFDzYA4GMYOVD0y-C5Sg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
n9cPNC9uAbRY7ud_FIJheELjTlJhUs5mmVkJiR46CCdo1kQu1kgQNO5lppx5Ky0C-iGFeYreHxEXqZitHqpqJA
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
vR_P70Z_kvWwesv6DN5w_rVWti81iYfByiHj9Yz4-aFmiFfF-XtXsOMfqAjtvZp1deckLsujfSF73iJKGIgcXQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"count":10,"status":"inactive","avg_score":50,"max_score":90}
{"max_score":100,"count":10,"avg_score":60,"status":"active"}

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
B_D7s0bCsL8RXJzNuwDeWcX74TFTH2JyDxJqf8hnaYvcza_AHhJ-48TX1MtuZOve2ygVHTWkLwiiwkl2qojWXg
Created Bob: $500 - ID:
NN-6qZNFQoQzcjDDo5B4V9BLxKGsVfcT9H0IxnsYMRLqWHZny5ABXMHCoimWXWwkTn3A2vTx526B3irCj8wMdA

=== Example 1: Begin Transaction === Transaction ID:
839afbf9-09d8-4324-aa79-e373acf511a2

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"value":700,"type":"Integer"}

=== Example 5: Rollback === New transaction:
a05094b5-3218-4ef1-879b-ff1400b7b89c Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

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
\_6HT1SHkhAMYkBy4oBy6cwVXJJkCXae5CJH36pC-Yhg-GyE6dGUucx5O5T_PlOpqZr7aN7sLngaHEd4UqwfbvQ

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Name: [object Object] 📋 Email:
{"value":"alice@example.com","type":"String"} 📋 Status: [object Object] 📋
Credits: [object Object]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
cIGU2qG0yA-knbnRFZ5rFLIV0YHy_vLVXVTEHwBMYQMpEQ2aufLmLLNCNv_wCts5qSYgz-kHieLjNWSsMMT8mg

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
{"value":"pending","type":"String"} 📋 Name: {"value":"Alice
Smith","type":"String"}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
Yrh-NR96dElUrUqnhQXA6hxwCh-PrbB6Tfxn3bwInELbZMZXQxgVINewnz5vUZXIbRtP7gTAiqbA19IWubKbeg

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
{"value":0,"type":"Integer"} 📋 Status: {"value":"active","type":"String"} 📋
Name: {"type":"String","value":"Alice Smith"}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
0P767uZZOpbh9jK4czeMSDKerSP69vrCt6RXnU7PhsU1lLpSkXJHTmC7vnHIhDrIDOzEiZbDHVGYgDZuVplZ4A

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
\_6HT1SHkhAMYkBy4oBy6... ✅ Deleted script: cIGU2qG0yA-knbnRFZ5r... ✅ Deleted
script: Yrh-NR96dElUrUqnhQXA... ✅ Deleted script: 0P767uZZOpbh9jK4czeM... ✅
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

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 2s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 632ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'FNX8_AImcx6vh7s3UHa0V_GBATR1HCN0phXdIOS0iwDgNVKgeiob3WJbScnQi6NyDSA0jPwnuzEElDyNCla3QQ'
}

=== Find by ID === Found: { active: { type: 'Boolean', value: true }, value: {
value: 42, type: 'Integer' }, id:
'FNX8_AImcx6vh7s3UHa0V_GBATR1HCN0phXdIOS0iwDgNVKgeiob3WJbScnQi6NyDSA0jPwnuzEElDyNCla3QQ',
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, id:
'FNX8_AImcx6vh7s3UHa0V_GBATR1HCN0phXdIOS0iwDgNVKgeiob3WJbScnQi6NyDSA0jPwnuzEElDyNCla3QQ',
active: { type: 'Boolean', value: true }, value: { type: 'Integer', value: 100 }
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

=== KV Get === Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
1HgNJJ_KETfBECFaIJ3q8bczaqfc9AtoJnP9i8Ym4ASeXK0d3KEso8eKda3WsCU-jsmdNgvI5rxGEJQDXouY7Q

=== List Collections === Total collections: 22 Sample collections:
schema_users_client_py,schema_employees_client_py,test_collection,chat_configurations**ek0_bench,chat_messages**ek0_bench

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
arO3JyCQJuMb_CFS87TDLYa1foZP4SdAJmDgZHJOeA4Uxnkt0jJhhSZw6m5krrkSy5OdgPNw_5waz-iQ5h5AlA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
JfChkOlXjz6zU3H60JIesTeDN0_MdXRJ57S-4CCMG6B69myLCyCp8JdhXfc-PEqbMc8xLb7NX2D0D5x8z5w_AQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
YbZxqUCuk_AymEM0iPH_IgM5h7f3BN4UX2deTKufp7qhSkGDLxi8Al-ikxPZircGk95kbExAhE_9MSFDQ7SFWg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
2wyhy76s94P5BSmsgZ9UfN-B_grbfSBkNyBMm1A6Kz_b9Sdc6MEWOewPL1X7rGghMzyvHJRo0seUpsN6f67dzw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
uATiYQnploA4ykurX-Akng8m1mgP3FL_Q_4faeBjenTlWt2M5vE_vuHrsCo8Ve-e6gymwovkbhR2b7MFozV_Cw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
xK0tbUJ8ZCy3bMtUaCofSCl_bUtFt4CkyNx8e3kwC59yhl8F5fJcu0ZWA_EJM_FMbcoo1qVjctctMqRcqjGZCw
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
ozKAxzwwsWKdQmt_RYD7s9EvNz7h7oOhNYI09wVk8CrxNJIZjON0DHrs54stsBPwSlRliFosnvoNGaUiFhMrAQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"max_score":100,"status":"active","count":5,"avg_score":60}
{"max_score":90,"status":"inactive","avg_score":50,"count":5}

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
0c4EoSXXWu5f302EWkwRKPYCT7fEYP9m_90TsLAFhhUV9mX15ZzZbhTaSSTWdpmqap-68Bj_T0kmCUv4R08HdQ
📊 Found 20 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 20 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":10,"avg_score":50,"status":"inactive"}
{"status":"active","count":10,"avg_score":60} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 23 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
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

1.  Office Lamp - $79 (⭐4.05)
2.  Wireless Mouse - $29 (⭐4.5)
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

✅ Script saved 📊 Users (showing first 5 of 10):

1.  User 5 (user5@example.com) - Score: 50
2.  User 4 (user4@example.com) - Score: 40
3.  User 2 (user2@example.com) - Score: 20
4.  User 8 (user8@example.com) - Score: 80
5.  User 10 (user10@example.com) - Score: 100 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Natural Language Processing (AI)
2.  Introduction to Machine Learning (AI)
3.  Vector Databases Explained (Database)
4.  Getting Started with ekoDB (Database)
5.  Database Design Principles (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Natural Language Processing
2.  Introduction to Machine Learning
3.  Vector Databases Explained
4.  Getting Started with ekoDB
5.  Database Design Principles ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Natural Language Processing
2.  Introduction to Machine Learning
3.  Vector Databases Explained ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Natural Language Processing (AI)
2.  Introduction to Machine Learning (AI) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Scalability: Vector databases can handle massive amounts of data and scale up
   easily.
2. Performance: They offer high-speed data processing and efficient querying
   capabilities.
3. Flexibility: They can store and manage different types of data including
   spatial, temporal, and multimedia data.
4. Precision: They maintain the highest level of detail and precision, without
   data degradation.
5. Interactivity: They facilitate interactive operations like point-in-polygon
   or network analysis.
6. Integration: They can be easily integrated with other systems or data types.
7. Security: They provide robust security features to protect sensitive data. ⏱️
   Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Draft Article" - 1536D vector
2.  "Getting Started with ekoDB" - 1536D vector
3.  "Advanced Query Patterns" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m
