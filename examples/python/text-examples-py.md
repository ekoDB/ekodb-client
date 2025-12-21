make test-examples-python 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'Sgu5HrD4eF-wLJ1nZ3ggvsPNUCewPkWIFbAYh9gySVTtsjYRtTGeHLvP3jKJJ1JiSnJ4TNhq8nE3dC_poeu1Yg'}

=== Find by ID === Found: {'id':
'Sgu5HrD4eF-wLJ1nZ3ggvsPNUCewPkWIFbAYh9gySVTtsjYRtTGeHLvP3jKJJ1JiSnJ4TNhq8nE3dC_poeu1Yg',
'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record',
'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'value': {'type': 'Integer', 'value': 100},
'id':
'Sgu5HrD4eF-wLJ1nZ3ggvsPNUCewPkWIFbAYh9gySVTtsjYRtTGeHLvP3jKJJ1JiSnJ4TNhq8nE3dC_poeu1Yg',
'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String',
'value': 'Updated Record'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
qQFo7AuYWumBE-nyEtkv0vnDprHbBQs6MyvSoxLr6x1dPGKK912zpqx7rJPxDOhxF1hw2rZdIWEl5BK9pSW0TA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ZcYBdW4BqNkDzf5RbWUTc1jIg4MNTZSmLAIkKJV-qyVNSV9E7XHRzPgh3hNqd7aO-Wc1hgdTsZeNeSg32CNDwg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"qQFo7AuYWumBE-nyEtkv0vnDprHbBQs6MyvSoxLr6x1dPGKK912zpqx7rJPxDOhxF1hw2rZdIWEl5BK9pSW0TA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 2
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 10 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="{'type': 'String', 'value': 'active'}"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
bng5vlD9qS3H3bVMplnxDynTte9IK4swlyOE-eWkCxT4fsvip3JNKt7xONMAg2f5BUvqahGFvaaVp8LxbtoTjg

=== List Collections === Total collections: 11 Sample collections:
['websocket_test', 'demo_collection', 'test_accounts', 'scripts__ek0_bench',
'test_collection']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
GkJwQWqmN-qt4jQuEDmPlLid01HMX8MY4rORU9JeuNcKincTgyaQNvtf7zJ7GjtRZAOc5w6XnQwUaAyNPETX4Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
wykJw6OoGbwdGsE6_BGVpep7fITbUArm4FNPXSbllHkZbe1YltufxNtNOXtrXIo03dcY_xrxnqqApm3lxhGXkg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully 🚀 ekoDB Scripts Example
(Python/HTTP)

✓ Authentication successful

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
KCUaKvDGQAM1a5S3buVFfvzulkJ6scx8cKVRqTVUVquEDHy7r4d8Hs9_LT1eSh2iI2N1Z4uq_OxJPDriJK6zPQ
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
\_7tms6mo7-cd5pW3gUBPgZ4_dccXirPs4qSXPExnFRh0WrRGYe9gv2isFX6QV28SV5lsnJZoENZez5a4RhUmEA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
-NAQowjzjOcComOWvfpQjIdHjexJ1rxlHo24aIALnLHb3bJm_DfTpT61nxDu7pHXVGRl-DXmjd4CB7YjDN6ovg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{'avg_score': 50.0, 'count': 15, 'max_score': 90, 'status': 'inactive'}
{'count': 15, 'status': 'active', 'avg_score': 60.0, 'max_score': 100}

📝 Example 4: Function Management

📋 Total scripts: 8 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✓ Authentication successful

📋 Getting original configuration... Original durable_operations: False

============================================================ 🔥 TEST 1: Original
Config (durable=False)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
oW3c1V0M4eb3a7zJTAYYONhsighdduLEsdT0FAACkDZQZogPieNMVP3-ZBj8-aEsxlNcy_ln6qra6H7PzghpuA
Created Bob: $500 - ID:
yob07jtjPF9kEQlPiXv7EpwHw4ESluC2tpHQPcSaEr5zh2VSzZ6iP2lXFKGXfieXxC1g3Qf_YmObCnaVqetT3g

=== Example 1: Begin Transaction === Transaction ID:
62485035-eda9-470d-8197-231ccd964204

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
508efd9b-340f-4fe3-b538-f46063aab94c Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
L_o19ky7yJ9VHJONlP_xEJ18Qpgcuzgc8LM0nz9eR12KEYq4YfrgYSTm7XAh_5ghMZSMT4Xp-eWUkxwIkrAhyg
Created Bob: $500 - ID:
gBVrfMzeiRmSkM4NL4JhJMXpMTT6CizFapz1n0lJiPuy8sxVMcCBFpgHdaRWI1h9veRmhABFEsAP0MzJlp8mTQ

=== Example 1: Begin Transaction === Transaction ID:
e9215966-7258-4d83-83e3-f37d3bdc0d8e

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to NON-DURABLE mode... ✓ Config updated: durable_operations=false

============================================================ 🔥 TEST 2:
Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
9hCrWPJ0wNXxa-0ot2G_fwRoa16o-vfxUqViRB4bA4agmbqNvu-IWW9iiczRIt1n6i4HeqrguZ1y56Rgkctcyw
Created Bob: $500 - ID:
KgYSi0Y_dmuq4_55ueF7pVlBz3Aeo3ahEk4hoxJZ7yfXdh2FJLsL0du3Z2gxGKb9XxcjWT0JK4uyCM6fXrjLHw

=== Example 1: Begin Transaction === Transaction ID:
741ce6a5-9e2d-45da-a23e-b9f565bdd977

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
e019ae88-fe84-4f6b-9672-9b37d0551978 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
2H5fj8f4utQS4HCbQmqC7oFWlCZDJn-Q06352hzNlKJWS8jta_xhIP-UNNdi5caf1WLEp2v5i7JmNj2A38X3Lg
Created Bob: $500 - ID:
XM0RfBZIeSFhOTsG-LIyGz_S4tvPOUEByHKyXH-IdY1VDy7X_3zFTUoqkhvhWakANIm4kQiogguEXOYdlblUsw

=== Example 1: Begin Transaction === Transaction ID:
fc93b75c-8153-4f59-910e-b0504d6e5132

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to DURABLE mode... ✓ Config updated: durable_operations=true

============================================================ 🔥 TEST 3: Durable
Mode (durable=true) ============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
\_OyCtQm3VvLCGVKrGX4E0nnLVCvJ6edOcSx5TtT98YUbVSegolYcfCTquuBp7CegUt7ljSMjEIdk1bqQRPmE9g
Created Bob: $500 - ID:
D5DM_iqXmF96XRSi-vLcfA-5-7ApxfUgoFSlg9V1Ncbffy-wEsOFNvJwUA6_tHza-MHvTsfTdrQ9XKE1FnOMIg

=== Example 1: Begin Transaction === Transaction ID:
561b37af-7d67-49da-b2c7-64884fa8338a

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
e2f056d8-a2be-4692-b70c-ab4e44112441 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
zE2cAhfLoYzKC7xXYllGur6VD-5do4jhDYXLQ6C60q1Yl8b1qjDzPXofh0TPGPNMFi3CPQEoWKDh3aC8Z6vN1g
Created Bob: $500 - ID:
e0rlnPhOg2hHZd5PJlSAoOlP7HtNqqh-fKeCpi6Qw5AD9O-hi5QGZSgzUQbDmn7Y4Z4RvCyDz1ce8oT3r9vzyg

=== Example 1: Begin Transaction === Transaction ID:
fcd8ee4d-8c43-4164-b3d5-b76a4bcf2092

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Cleanup === ✓ Deleted test accounts

🔄 Restoring original configuration... ✓ Config restored:
durable_operations=False

============================================================ ✅ ALL TESTS
PASSED - Transactions successful
============================================================

# 🚀 ekoDB Complete CRUD Scripts Example

Demonstrates: • Insert + Verify (using Query) • Query + Update Status + Verify •
Query + Update Credits + Verify • Query Before Delete + Delete + Verify Gone

# Each Script shows Functions chaining with proper verification

============================================================ 📝 Script 1:
Insert + Verify ============================================================

1️⃣ Saving Script... ✅ Script saved:
LLA9inGkXisNwy3XpzoeuOaF6vHB7Z5aNQADm1eF1Gb3aM10qEFaDs6QJgaAOAJXLXj5LWaO09bRzZC80RaOlA

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
bVxqldxn_UhA7eXlltdsPw8T57ruRw3cMG6GyDr0WNUWW6W4OAsmyxwN8fUP4PS8hpNE94urn_ZQ0pBmEsSCFw
📋 Name: {'value': 'Alice Smith', 'type': 'String'} 📋 Email: {'value':
'alice@example.com', 'type': 'String'} 📋 Status: {'value': 'pending', 'type':
'String'} 📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
FmiAgxuOUY-Qi1gclZjEyMU5XfCBkRC_gb0-tTAAu6be1N1ImoIe1GG\_\_KdftXkAFKdWoO24fM32AySOXRgmLg

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to: {'value':
'active', 'type': 'String'} 📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
Ri4wlBGH1SkE4Tat3FLLZOQaY1NP_xnItMjm19g_R5fIMQZUpHYy9MYbZrwBV1wMDuQ_J20gobUXmLTuJd7V5g

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to: {'value':
0, 'type': 'Integer'} 📋 Status: {'value': 'active', 'type': 'String'} 📋 Name:
{'value': 'Alice Smith', 'type': 'String'}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
nqSQToD8hm3Njg6d3fUcdeTUklagpi1GMcKuuR2ZVJcwj3aO6gD92mSiJiHU8H_hzqCfiyNnr61PvnYdGcAf2w

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
LLA9inGkXisNwy3Xpzoe... ✅ Deleted script: FmiAgxuOUY-Qi1gclZjE... ✅ Deleted
script: Ri4wlBGH1SkE4Tat3FLL... ✅ Deleted script: nqSQToD8hm3Njg6d3fUc... ✅
Deleted collection: users

============================================================ ✅ Complete CRUD
Scripts Example Finished!
============================================================

💡 Key Takeaways: ✅ Scripts chain Functions together ✅ Each Script
demonstrates operation + verification ✅ Parameters make Scripts reusable ✅
Verification is built into the Script itself ✅ Complete CRUD lifecycle in 4
focused Scripts [34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB
Python Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m [32m✓ Server is ready[0m [34m === Getting Authentication Token
===[0m [32m✓ Authentication successful[0m [33m === Running 10 Examples ===[0m
[34m === Running simple_crud.py ===[0m [32m✓ simple_crud.py completed
successfully[0m [34m === Running simple_websocket.py ===[0m [32m✓
simple_websocket.py completed successfully[0m [34m === Running
batch_operations.py ===[0m [32m✓ batch_operations.py completed successfully[0m
[34m === Running kv_operations.py ===[0m [32m✓ kv_operations.py completed
successfully[0m [34m === Running collection_management.py ===[0m [32m✓
collection_management.py completed successfully[0m [34m === Running
document_ttl.py ===[0m [32m✓ document_ttl.py completed successfully[0m [34m ===
Running websocket_ttl.py ===[0m [32m✓ websocket_ttl.py completed successfully[0m
[34m === Running http_functions.py ===[0m [32m✓ http_functions.py completed
successfully[0m [34m === Running transactions.py ===[0m [32m✓ transactions.py
completed successfully[0m [34m === Running crud_scripts.py ===[0m [32m✓
crud_scripts.py completed successfully[0m [34m
╔════════════════════════════════════════╗[0m [34m║ Test Summary ║[0m
[34m╚════════════════════════════════════════╝[0m [34mTotal: 10[0m [32mPassed:
10[0m [32mFailed: 0[0m ✅ [32mPython direct examples complete![0m 🐍
[36mBuilding Python client package...[0m 🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support 🐍 Not using a specific python
interpreter 💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by
default Finished `release` profile [optimized] target(s) in 0.16s 📦 Built wheel
for abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m Processing
./ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Attempting uninstall: ekodb-client
Found existing installation: ekodb_client 0.4.0 Uninstalling ekodb_client-0.4.0:
Successfully uninstalled ekodb_client-0.4.0 Successfully installed
ekodb-client-0.4.0 ✅ [32mPython client package built and installed![0m 🧪
[36mRunning Python client library examples...[0m ✓ Client created (token
exchange happens automatically)

=== Insert Document === Inserted: {'id':
'OIpbl5lfka9L5F-8D6yh93oscIEP323DXVLlcjGv6jcsbLT6yroILgbb2AsiXYfkGorfpAuNYBOkwCETaUh33g'}

=== Find by ID === Found: {'value': {'type': 'Integer', 'value': 42}, 'name':
{'type': 'String', 'value': 'Test Record'}, 'id':
'OIpbl5lfka9L5F-8D6yh93oscIEP323DXVLlcjGv6jcsbLT6yroILgbb2AsiXYfkGorfpAuNYBOkwCETaUh33g',
'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'value': True, 'type': 'Boolean'},
'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record',
'type': 'String'}, 'id':
'OIpbl5lfka9L5F-8D6yh93oscIEP323DXVLlcjGv6jcsbLT6yroILgbb2AsiXYfkGorfpAuNYBOkwCETaUh33g'}

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

=== KV Get === Retrieved value: {'value':
'{"userId":123,"username":"john_doe"}'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'value': '{"name":"Product
1","price":29.99}'} cache:product:2: {'value': '{"name":"Product
2","price":39.989999999999995}'} cache:product:3: {'value': '{"name":"Product
3","price":49.989999999999995}'}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"ZNuL0gE-Ju38JEYHeD1vzgU-F2Qcn39i8q5v345727YGtjYCAPYF8aWhANOLkdI4bGupMBMi9YAJhXfx3N4VdA"

=== List Collections === Total collections: 10 Sample collections:
['websocket_test', 'test_accounts', 'scripts__ek0_bench', 'test_collection',
'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
jCkXFplMxGQPc1zQ_TNr04QLZVbS-UhtZwYFyo7okqB5R6NKac6wvK8hzIBGqCAt_7tZsVNXJ0ALKgJihmSxWQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
cBuYWY9TiAsRSkC7OzR84Ch4dJ9i-ppGozhJJLpbUpDsVA0dlZ-JSQq40WjGIrLTbNzNvx9692Y-7EgIq6V9HQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
JQllzjvfNYe6uH9LAueZ_5--RS4piMwmYbXTR82gRupWHxpXF1i6hlI1LD_q53I5E3keqqiIfyjCUPgmHreHZw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
DZ5HH2PrUA6L_Q2cVxwNlGAsZAboV5VxryKDZ22ijEMZC4nOLXY2argr5FDLs-J-su0jzyrO3SZRx50b0hqCQQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === Query Builder Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 2 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740, Matched: name, name.value, email.value, email
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, title, bio, title.value
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: title.value, bio, bio.value, title
1. Score: 26.400, Matched: bio, title.value, title, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title, bio, title.value, bio.value
1. Score: 39.600, Matched: bio.value, title.value, bio, title
1. Score: 39.600, Matched: bio.value, bio, title, title.value
1. Score: 39.600, Matched: title.value, title, bio, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.746
1. Score: 0.732
1. Score: 0.710

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.697, Matched: title, content, title.value, content.value
1. Score: 1.122, Matched: content.value, title.value, content, title
1. Score: 0.513, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: title, title.value

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: ['age', 'email', 'name',
   'status'] Schema version: 1

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

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
LCHZxuXi5of0QvhmSm0wm2f0WqfUiFUL6umnxwqTLFma71kgbt2sXS9UnwTggcBS-gSdQZqtf466qGxqUAT_hg

=== Sending Chat Message === Message ID:
mnpB0g-rJvXOy2kWuPYghDlgaWWBn1suxLD_owkIdaXU6HpQsPLfDcdg2CksAguScjVB2sI30LspYHhI1LGFqQ

=== AI Response === I'm sorry, but the context provided does not include
specific information about the names of the products or their prices. Could you
please provide more information or clarify your question?

Execution Time: 1226ms

=== Token Usage === Prompt tokens: 324 Completion tokens: 34 Total tokens: 358

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not contain specific information about any available products.
Could you please provide more details?

✓ Message 2 sent Response: Apologies, but the context provided doesn't contain
information about the price of any product. Could you please specify the product
you're inquiring about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
CQjUNeaSdiKP1ShvAlRIwPDt1I4W8xk4gcaoHLC0dJjsF2OlERdFi7HLpWj7w9WLGUzkb9IbIrnDN4xUWhGtug
Parent:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg

=== Listing Sessions === ✓ Found 6 sessions Session 1:
CQjUNeaSdiKP1ShvAlRIwPDt1I4W8xk4gcaoHLC0dJjsF2OlERdFi7HLpWj7w9WLGUzkb9IbIrnDN4xUWhGtug
(Untitled) Session 2:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg
(Untitled) Session 3:
LCHZxuXi5of0QvhmSm0wm2f0WqfUiFUL6umnxwqTLFma71kgbt2sXS9UnwTggcBS-gSdQZqtf466qGxqUAT_hg
(Untitled) Session 4:
pC_Hvla-W6X4dKvY4WkJu4fHKzUoGMR1u2f-5FM1qJvi3iON0PEl6jCJjAEIWgpAThKQIjskKUCCOt0Ad51qMQ
(Untitled) Session 5:
LxDb9FFuDweBcT_fv86mFw6VBNhoehESDjYzen5_WpbOU7Dk8scFwalnDWwRWmY4Jd35oW7KoM_EF8KG739cig
(Untitled) Session 6:
W9Ku4NfJAWIGjBvCJsMP2kNZAFEEvTkH8RenBjwisN141_0i-7ueIMCLe8CpfAgLZfO24z6cAVpzbYybitCyHA
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
CQjUNeaSdiKP1ShvAlRIwPDt1I4W8xk4gcaoHLC0dJjsF2OlERdFi7HLpWj7w9WLGUzkb9IbIrnDN4xUWhGtug

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
mfdanfZKSENFyTjCNKYTs8qxBOYrCLy9NhV27zEp-CJokU7s1eZZoZvtjLD_PX4qEwr1Fua3Nqb_cqvxzDsU2g

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't contain specific information about any products. Could
you please provide more details?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['updated_at',
'id', 'token_usage', 'created_at', 'role', 'content', 'chat_id']) Debug: First
message role: {'type': 'String', 'value': 'assistant'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: Answer: I'm sorry, but the
context provided does not include information on the price of ekoDB. You might
want to check the official website or contact the sellers for this information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
j0ZNc8BrbxhRUDgb2Fwn4wHDCMGkwyehQjCVIE_5utey3QYCVumADwRrDHTrdEJPdutXXUHL_TIEvjy9tas1ig
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! 🚀 ekoDB Scripts Example
(Python)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
qA6PohHSLGP5gU0p5jW0qxuO8HhjI-0uwezEZ9tJIK-9p55zM_b3vsr9GnJfjn1617Pp7r44We8uZwpDAGRaAQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
lXH9rYIVT7_nRuHuw6LIIr_AvyHgm_ZmKW-F6XIwgrOd7wPVKJM1kmJhUNxHLa0PyrmfRECrbQSxNokCDMUZdw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
SD4wF7xWiUBl0oV5oUPa6RyjtkbulsUGvc5qQ9mcBmpI9TBld5ttbHLxlqaegozamc9AFzZkwF6_X-YqX4ecFw
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 5, 'status': 'active'}

{'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 10 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
JLkVkUHAeu5iT4SG2mX46Birk_xQxw4FVjjlyGT0biy2GsYrWY3Fl7SQ6xkGSqT_WVDQMghA19iTyiw49_skUg
📊 Found 2 product groups {'avg_price': 474.0, 'category': 'Furniture', 'count':
2} {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3} ⏱️
Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {'category': 'Electronics', 'count': 3}
{'category': 'Furniture', 'count': 2} ⏱️ Execution time: 0ms

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
[32mPython client examples complete![0m
