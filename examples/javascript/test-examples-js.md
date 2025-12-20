make test-examples-javascript 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 644ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:64616) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 10 Examples ===[0m [34m ===
Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:64619) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'hNPNIzSkA3dosq1VfDcFwQDk3abC1-8N9gR00fxls9JcjMUIKM-53-URbFo0X_4ZLemiMemQG2a6HO40W9MYBQ'
}

=== Find by ID === Found: { active: { type: 'Boolean', value: true }, name: {
value: 'Test Record', type: 'String' }, value: { type: 'Integer', value: 42 },
id:
'hNPNIzSkA3dosq1VfDcFwQDk3abC1-8N9gR00fxls9JcjMUIKM-53-URbFo0X_4ZLemiMemQG2a6HO40W9MYBQ'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { type: 'Integer', value: 100 }, id:
'hNPNIzSkA3dosq1VfDcFwQDk3abC1-8N9gR00fxls9JcjMUIKM-53-URbFo0X_4ZLemiMemQG2a6HO40W9MYBQ',
active: { value: true, type: 'Boolean' }, name: { value: 'Updated Record', type:
'String' } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:64631) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
h55LGL5gfIitB9FnEuNni-5AAMeQnY9oEbJRjPp16SM1o4GjNGmxOcG4osdtIoM-AUC5A05DlZgFw_aCRDHnLw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"h55LGL5gfIitB9FnEuNni-5AAMeQnY9oEbJRjPp16SM1o4GjNGmxOcG4osdtIoM-AUC5A05DlZgFw_aCRDHnLw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"-g3odd0twQrSsVSLVsISeu0mVnyzhoYUmFFRGNOwE63t_8qQwJM0iIFFDVgCcXfWKD0yRacu6KH6p4ml-i7KgQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"dTPCdp_g6cKAoqQLPCx-Y-rgKg5040O1X1Y4EBATWa3og1GDIm2dwdlm5-E8NZFc3KliH7RMsq4MNmmrvv0NMQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"BqL4lJ96z7t2MSMg5wG1jQZCCV83GzgtuZAhxdm5IqpxPXen3tQ-AFbQUd2RqqJMrJvIgqdSsuxR9whS4c3EnQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:64644) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:64647) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-20T08:37:59.904Z', userId:
123, username: 'john_doe' }

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

=== Create Collection (via insert) === (node:64650) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
8pl1Zl3kcC3XKUFVHaHD5PNCo0x76P9VupUQE8DbrmXG2ZhijxTCie3pu2MwZpRQ28a6_B7uS87qBduUt8QPPg

=== List Collections === Total collections: 23 Sample collections: [
'schema_employees_client_ts', 'scripts__ek0_bench', 'users', 'websocket_test',
'schema_documents_client_py' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:64666) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
KRXAl0sTEFgTJBEVwb4kazPDphi1TKfhOlrZEoZomk8_GPMH5Fmsb525jw73attfVuBuYkmmVy1JQtW6tNJDLA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:64669) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
ykYYNcVFf0AtwDhTEEkyW2B7XTTIMMBuVnKuJQqzWFAHYZt0-H4Bde7tT8tmuqAYdOYEpb-mqsv_7oQ_YCpkbg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
4YW-\_V7-glSzc3snaNlrDmuLUEnooVPqicFJeHhX_8g2QyrG3-APH5MlFvhEbX75pME-S0MKpIStC2dVN3kbAw
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
U7onT-GFtLxXRL_dnZETli-qrEmwCMx21696xrzOLC2FE1yjTPDT_Md3lDZQARKW7amWvL0i3MYviSJyITMGdQ
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
iDzmB1WHCpEZjRPzuhT2IKZKEmcNYIhAB7tlbthAWHT7yFadFnLdatufKl3Sv8gG43LuSMjPY8Y4YkXySzLoMQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"max_score":100,"avg_score":60,"count":10,"status":"active"}
{"status":"inactive","max_score":90,"avg_score":50,"count":10}

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
qyhKTuLeB8JysOzjwQmVJVtfovqQzgnrvq96Zl*c-jxYN0hy44tSzrSBDflcDKtF3jM4rUMdvVaJzkhAFB6i9A
Created Bob: $500 - ID:
sISkOk0szdD8SlGs9yvDfx-yo0SeczB1gflSubvSgAPVj8BcFR9*-rM0_UuBK-z5-jdchGfKv0F0-uVXNxXCGw

=== Example 1: Begin Transaction === Transaction ID:
b098620c-e8f2-4d41-bb5d-26c1fbac5818

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: 800 Bob: 700

=== Example 5: Rollback === New transaction:
8f6aac08-cbaa-40d0-af5d-6080c1ed9ee2 Updated Bob: $700 → $600 (in transaction) ✓
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
inSM-Xo2tnvT0NUKyF20CzT4m24LWKUBz5RafmJw8UA-Uytj9HUraYMf1qed_wyesNvSa636uRA1K3Lni_T71w

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Name: [object Object] 📋 Email:
{"value":"alice@example.com","type":"String"} 📋 Status: [object Object] 📋
Credits: [object Object]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
b3YFcCHr9IGnU_8CvoRCyiN8csIgOuCVUxhtXL5J9JXuPKLam4eN-xPlML0P-tqX4oduYW-uCcA1u8xUjEZR_Q

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
{"type":"String","value":"active"} 📋 Name: {"type":"String","value":"Alice
Smith"}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
urDR2k-beKAVbj1qC3Wh7-w4h1_ZWyxkZcADsAQXZi8beGRZR7yaKk8-8u9AZRz5dZJaJEDauvZUAVpRXAyGPQ

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
{"value":0,"type":"Integer"} 📋 Status: {"type":"String","value":"active"} 📋
Name: {"type":"String","value":"Alice Smith"}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
mX86OE3OZlC1My5OpMktjp31MXrCVhfJkuty-1ZVli\_\_vS5L4DdI_4JpFEQKWsRvEM7tAL9RruqgCBlSuKWyiQ

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
inSM-Xo2tnvT0NUKyF20... ✅ Deleted script: b3YFcCHr9IGnU_8CvoRC... ✅ Deleted
script: urDR2k-beKAVbj1qC3Wh... ✅ Deleted script: mX86OE3OZlC1My5OpMkt... ✅
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

up to date, audited 7 packages in 790ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.4.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 632ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'LGoFNZrgWiS_a1NsKLgP-7VpjDWlnn0IqWpik_3oLBpgWy2qWoKXmK1fVkmVw5OE0KuJs5DJew0nAvDSMhWOTQ'
}

=== Find by ID === Found: { active: { type: 'Boolean', value: true }, value: {
type: 'Integer', value: 42 }, name: { type: 'String', value: 'Test Record' },
id:
'LGoFNZrgWiS_a1NsKLgP-7VpjDWlnn0IqWpik_3oLBpgWy2qWoKXmK1fVkmVw5OE0KuJs5DJew0nAvDSMhWOTQ'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true }, id:
'LGoFNZrgWiS_a1NsKLgP-7VpjDWlnn0IqWpik_3oLBpgWy2qWoKXmK1fVkmVw5OE0KuJs5DJew0nAvDSMhWOTQ',
value: { value: 100, type: 'Integer' }, name: { type: 'String', value: 'Updated
Record' } }

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
Fp47CCeJ_dRIB1F2epTs6zOxaccHXisLBsYgZKcWceKqy31t3X5dJSJFEWDO0n9UI_pwGBoAwMPEQLNGZZ-Nzw

=== List Collections === Total collections: 22 Sample collections:
schema_employees_client_ts,scripts\_\_ek0_bench,websocket_test,schema_documents_client_py,schema_users_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
FTCz-L5hIUnmerkP8Yln8yf7Y5EuX7vmL9pmBc92YQwCBrrSEUmaTDyrUcc7HgKiEXEG-AhoUczIEJwdgiJxNw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Mke5N5SEfYvj9kavBTlgu0zvZmJKbGVRfRnnM1r3DtrARgQprdfstYTp5FsbwHzV_8g8Mcojqqw9xfg0IHQDbg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
2IErvAw4WfDmWVKT_FkQIOI6xEce9fn3gC1E0qdHHq-JC_uw0o3RHAThwXW6v_SGjCCBXWh-\_Ap_3r4quKE0LA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Poq1QSvPETIqTYC_vNW1IixNHHvAyH28YpruybbDRHQDam21hjfEbAbHwrVUNxpQafPXU3Mth6_YCiMeWttq2w

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
j1fQOXk3WHwpA2vOmuCWY2QsBjbDwca0wvaFq9ainR9kjyBNaXOovPxgKEBDtmD5hoQP9XHiX2yK6EKw2-Jb-A
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
9uiRsalCyekSSHYMJAwQEwlTZyXkBKPelqcdIY_o1bGUWns5PDj7WT8pA20rJl32eT2yQSe27vsnWazoLE9VMw
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
cp_d--rZWzOtyKPxobl6HVE3nCjMqiCNJL-cpx2CPUa8IIrZc0fQJRudxyB8R9PjyA8KaQmRQYHVw7mNjzb_Qg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"count":5,"max_score":90,"avg_score":50,"status":"inactive"}
{"avg_score":60,"max_score":100,"count":5,"status":"active"}

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
3sz1K9ABi3KB2qDlJbXBIhiyT0u2vQTjUK1f3epYosODL9Ntt4dvcSouvFg_jFSELaO4H4sIpE5QBD2ZGUbkFQ
📊 Found 20 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 20 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"status":"active","count":10,"avg_score":60}
{"status":"inactive","avg_score":50,"count":10} ⏱️ Execution time: 0ms

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

✅ Script saved 📊 Category analysis (2 categories): Electronics: Products: 6 |
Stock: 232 | Avg Rating: ⭐4.52 Furniture: Products: 4 | Stock: 43 | Avg Rating:
⭐4.26

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Webcam HD - $119 (⭐4.5)
2.  USB-C Cable - $19 (⭐4.3)
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

✅ Script saved 📊 Users (showing first 5 of 0): ⏱️ Execution time: 0ms

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
3.  Natural Language Processing (AI)
4.  Database Design Principles (Database)
5.  Getting Started with ekoDB (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Vector Databases Explained
2.  Introduction to Machine Learning
3.  Natural Language Processing
4.  Database Design Principles
5.  Getting Started with ekoDB ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Vector Databases Explained
2.  Introduction to Machine Learning
3.  Natural Language Processing ⏱️ Execution time: 0ms

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

1. Efficient Querying: Vector databases allow spatial indexing, which makes
   querying for certain data faster.
2. High Precision: They offer superior accuracy and precision as they store data
   as points, lines, and polygons.
3. Scalability: Vector databases can easily handle large datasets.
4. Flexibility: They allow various types of queries and spatial operations.
5. Better Visualization: Vector data offer better visualization of data as they
   can represent complex features accurately.
6. Supports Topological Analysis: Vector databases can identify spatial
   relationships between different entities.
7. Compact Data Storage: Vector models allow compact storage of complex data. ⏱️
   Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Advanced Query Patterns" - 1536D vector
2.  "Draft Article" - 1536D vector
3.  "Getting Started with ekoDB" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m
