make test-examples-python 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'g37V-LwIE-D39vDQNH7q_TksP6sDufBFxhi1ECioxyR88XpT6QWqNCw1IWVX85pSVlCVUlgUpPI3dnV157xT7w'}

=== Find by ID === Found: {'value': {'value': 42, 'type': 'Integer'}, 'name':
{'type': 'String', 'value': 'Test Record'}, 'id':
'g37V-LwIE-D39vDQNH7q_TksP6sDufBFxhi1ECioxyR88XpT6QWqNCw1IWVX85pSVlCVUlgUpPI3dnV157xT7w',
'active': {'type': 'Boolean', 'value': True}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'id':
'g37V-LwIE-D39vDQNH7q_TksP6sDufBFxhi1ECioxyR88XpT6QWqNCw1IWVX85pSVlCVUlgUpPI3dnV157xT7w',
'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record',
'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
p0WGdKAIQtu4s5ajCoE5oCqbhiPonpADBDtzNzVo2gEYwqeaFyLY3JmHjuNrJ0ywk4YSqv-4LoaE9RFYLVD3ww

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ChsE4hhtNg8BkHvjsfyGVK_RCbVHQx8IaoOx1Vo0vx5l_0yuETorylmHzekR4I61txQOtNqmD1ntnmwC4zO4Nw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"p0WGdKAIQtu4s5ajCoE5oCqbhiPonpADBDtzNzVo2gEYwqeaFyLY3JmHjuNrJ0ywk4YSqv-4LoaE9RFYLVD3ww",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 2
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 10 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="None"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
TsOFnLpHldLLbxKFjoygZUcqyhT8ltw_ZFuuzuq6A6NLb96OWd4p69StAXq7Z-bx15F1P4O2_tjcXSscTQJygA

=== List Collections === Total collections: 11 Sample collections:
['test_collection', 'chat_configurations__ek0_bench', 'demo_collection',
'chat_messages__ek0_bench', 'ws_ttl_test']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
6UIDPf2Z_63HksMFonfY3_sR8bN9fXpx-1NBbcgSg9fGSy-62xizTdFNP4jy3NbNKGvUqRInbdH0_eP_zGGKmA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
bnYTF9_MDGLwyhD3PuNL7uPnDXrEkhSmGsExG6rRjGPPowa7XRxQF9UGCIfbaxcz8LFay7IRED20_LFkI_zHCQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully 🚀 ekoDB Scripts Example
(Python/HTTP)

✓ Authentication successful

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
fWIApn-Dnv9HpF3G5VUZ6o1OxbrMHyHwA86wh0HGF4tD8fyPE_vWFhwJqnNZ6P67SuRatVMiX6L_9Z8Qcoa3jA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
SVJ2gSFfaIyFE5xeql2qqNrOwh4_iOuzelDTv2Ryfm_jvYwbzjrfqhOvLPU7hRW7wSNEkG7P6IoySBb61ItF1g
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
eURngPV3SI7qyakXhhH8du5dHzAJuctMRUbZyVQnCLYanuh7tK2_ZtI4mkGobaCV-GwrT5msizo2kO4MAzsHew
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups {'count':
15, 'max_score': 100, 'avg_score': 60.0, 'status': 'active'} {'max_score': 90,
'count': 15, 'status': 'inactive', 'avg_score': 50.0}

📝 Example 4: Function Management

📋 Total scripts: 7 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✓ Authentication successful

📋 Getting original configuration... Original durable_operations: False

============================================================ 🔥 TEST 1: Original
Config (durable=False)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
kGZ8ZwqW2pZR5qjVIJxN9yxEWVvrWoNwG2WEmtYzPMy06uNMCXvpH_2WflhpEqTzhKX0hJxKdqfy7mxbc8KvWg
Created Bob: $500 - ID:
DFqwBv9OYNLaZYClum3AK38UIWgL722Fw8gcy9-5SwSH_m_M-7bCESOeI5tXocW3KW23VQCcSNMKzdxtAUqYtA

=== Example 1: Begin Transaction === Transaction ID:
8cafb4a2-bbb5-4bc9-9502-6a213a1dc991

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'type':
'Integer', 'value': 700}

=== Example 5: Rollback === New transaction:
44d40a41-5298-40cc-9e90-5ad97b690de7 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
MQ8DUqux8MwUeeURelb0W2N76pZEWUwCHBSgYqIrceNHnFpBYB7S6RL-\_7z6KosunnMwH07j_mdNCyturRS36A
Created Bob: $500 - ID:
8rgHgxVtQRIKupq4xZsdPttNYBgLHokMFX7G6A20X0FILWpVigZshQBvswsMBBWg7vknpJITY5Chf-9WNb5Qww

=== Example 1: Begin Transaction === Transaction ID:
d77248bf-79ac-4a7e-a6fb-a26bbb32df52

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'type':
'Integer', 'value': 700}

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to NON-DURABLE mode... ✓ Config updated: durable_operations=false

============================================================ 🔥 TEST 2:
Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
wS5CMC4hZhyPjPyrIJcRV66TGvE74Aal0o_HErf8OckEKSdF5fJLvUpJ7B61PhyXH7i3ZUXo-jVR-g6KVM41fQ
Created Bob: $500 - ID:
mKnLbXH-1Fdq5RSCLvGK7cuq_RNzxL7cm4Wp-tyq7KT-wJEyPL2MThk9oZzYsR3lj5ZcaYJHagPWGuoUCCz0TA

=== Example 1: Begin Transaction === Transaction ID:
444194d3-1cbf-4b6e-bd1e-bd8c2beb14e9

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Example 5: Rollback === New transaction:
29725547-2659-4dfc-bcc3-c4c3c935820f Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
R7Lm8mE4Mtol54_kX3_FphjQLJSUK4yg51pvPWJO6Ubo472XiZbRr_H2lWESHgtjTSdZLTvvz-rDk_KPiGTj7w
Created Bob: $500 - ID:
GXQHGhuwe9hqR-bsWYpKL0RQ0DFj0SRkF2P8_etGuf2i5x8AjmYHOr6xIfA0bVQY5CXV6cXnNaRuSHgKYNOkbw

=== Example 1: Begin Transaction === Transaction ID:
67d29b0f-5a9a-43b9-bfde-d53cccec9fa6

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'type':
'Integer', 'value': 700}

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to DURABLE mode... ✓ Config updated: durable_operations=true

============================================================ 🔥 TEST 3: Durable
Mode (durable=true) ============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
cmC8f6gZKXDm2pwNw2jzy4LuuEfsfcGWo12mEGeOX-q6KAN6ZmWOR1d9W6lYwfpXSwzV29w9kPnMTbVesjQrpg
Created Bob: $500 - ID:
UOdhmHGU7dyVoLylfJOdRnd392rKfi-6gRrys2JxanszDTXbhxZnaulj8KicZ1uGqJr0V9I_NDA4psqCwVV5_Q

=== Example 1: Begin Transaction === Transaction ID:
fd4031b8-e673-4705-98e1-2df12d500ac6

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'type':
'Integer', 'value': 700}

=== Example 5: Rollback === New transaction:
2d72b69c-8da6-48f7-8d2b-64e6072e7c39 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
Wq9Ty-v6LTne-ieYAI2Ac7wLU3HS9GPUViYvPHGbm3B5lrC014f4eKYNb8peLmr3-l1Qo9nKs0u_xM62ypveBg
Created Bob: $500 - ID:
9RxB0J8ZvaGh3hV94y42gDhZuSs6YmEWYTMrUGWy6Bw0IRUOrej6W0Em_2MgfAjJZCM7Ef3MvJodwHwsRaI63g

=== Example 1: Begin Transaction === Transaction ID:
4725a319-8905-4fff-a86b-2e297475a731

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

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
SMO6yZ-GGUsb9VaJQJOkgLG9JesyyDAv3vYphWXDE1jXt1iWUshlRLEGECLao8Nf37U48QvjGeDE19Qsj1n6jg

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
lzdksKZsnQdMQcsGeOxEmtLC4du25MKLv-pB3Lyvto2xfWUXqMlHTbjNY59jswWWZ7UH4VmAq1VJVQI5dGbfKg
📋 Name: {'type': 'String', 'value': 'Alice Smith'} 📋 Email: {'value':
'alice@example.com', 'type': 'String'} 📋 Status: {'value': 'pending', 'type':
'String'} 📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
HRN6ZsjKiJ2kcUwojlgaEfzd5AzD-v-x6iRVABbmaiTh0Wm5Fyjem4_ITAfsQ9zQxjL4h4E1dywQ3SrqY3gYjQ

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to: {'type':
'String', 'value': 'active'} 📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
7JHHSmPvEsLPvGNtLVfbiQY4T-EewxV6_2ryTngZ_9mOycZHabpE-aCbrm1U9iKFHxO6UyyAexC0buVg5wqssg

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to: {'value':
0, 'type': 'Integer'} 📋 Status: {'type': 'String', 'value': 'active'} 📋 Name:
{'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
8vUuVAzws5SA5m6bsaoYyaujFiRh5bXgbl4S-YW4xAGmMBsgRZzW6MBS5AflrqNLZh_ZosPjvUNwoXlGtvaQQg

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
SMO6yZ-GGUsb9VaJQJOk... ✅ Deleted script: HRN6ZsjKiJ2kcUwojlga... ✅ Deleted
script: 7JHHSmPvEsLPvGNtLVfb... ✅ Deleted script: 8vUuVAzws5SA5m6bsaoY... ✅
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
default Finished `release` profile [optimized] target(s) in 0.09s 📦 Built wheel
for abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.3.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m Processing
./ekodb-client-py/target/wheels/ekodb_client-0.3.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Attempting uninstall: ekodb-client
Found existing installation: ekodb_client 0.3.0 Uninstalling ekodb_client-0.3.0:
Successfully uninstalled ekodb_client-0.3.0 Successfully installed
ekodb-client-0.3.0 ✅ [32mPython client package built and installed![0m 🧪
[36mRunning Python client library examples...[0m ✓ Client created (token
exchange happens automatically)

=== Insert Document === Inserted: {'id':
'0mJiOObBA7_QjVOJ3oqjV6odAmEfMIsDuL1c1Iezrvlccx7MD1XRqK8SDABIs9ZJLoRzKy3fkB9sz_s8pWIOFg'}

=== Find by ID === Found: {'active': {'type': 'Boolean', 'value': True}, 'id':
'0mJiOObBA7_QjVOJ3oqjV6odAmEfMIsDuL1c1Iezrvlccx7MD1XRqK8SDABIs9ZJLoRzKy3fkB9sz_s8pWIOFg',
'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer',
'value': 42}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'name': {'type': 'String', 'value': 'Updated
Record'}, 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True,
'type': 'Boolean'}, 'id':
'0mJiOObBA7_QjVOJ3oqjV6odAmEfMIsDuL1c1Iezrvlccx7MD1XRqK8SDABIs9ZJLoRzKy3fkB9sz_s8pWIOFg'}

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
"PlrAHMapjVapOkUY8aVp6Kwhg1oaWwvj3yFyKdwrj6cs4xyMngmzirqLrJ-WMrnjrYRq0DoyWPmMivEGagLrqg"

=== List Collections === Total collections: 10 Sample collections:
['test_collection', 'chat_configurations__ek0_bench',
'chat_messages__ek0_bench', 'ws_ttl_test', 'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
6x45LnlGQ3IpwXHVGOuCpFEd5eegrSYKH02Cc-4EqqAqdZcy-N2hwGO4HnBChuwVExys6uRyGcQRy2ijR9zs4g

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
CNQtHMvJZjrYYlipqsH299sRuaKh-xSvYPjwoafKnPldlQzNZ1BWBOUZIs-Di1mTZ5gJMuFfer9Q88-Xf-HC7g

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
n1xNz7rB8SCMaamaCMM0xPG6JD9A9fwBApVEocGVLUpMn3dNlj1lr8AiSaRbTiZTzunIs8MseQBscCMqvetfPQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
vAAOMt0-K99WTtgGmhNf9sAlPxt8sJKMQVsWS9-uTkM6HuvVZkUEoXbzJeXyzC4DlpQqZxfaYLDrL2v6b40fAQ

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
1. Score: 25.740, Matched: email, name, email.value, name.value
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title.value, bio, title, bio.value
1. Score: 26.400, Matched: title.value, bio, title, bio.value
1. Score: 26.400, Matched: bio, bio.value, title, title.value
1. Score: 26.400, Matched: bio, bio.value, title, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title, bio, bio.value, title.value
1. Score: 39.600, Matched: bio.value, bio, title, title.value
1. Score: 39.600, Matched: bio, title, bio.value, title.value
1. Score: 39.600, Matched: bio.value, title.value, title, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.761
1. Score: 0.744
1. Score: 0.727

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.709, Matched: content, title, title.value, content.value
1. Score: 1.121, Matched: content, content.value, title.value, title
1. Score: 0.533, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: title.value, title

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

- Bob Smith: Sales
- Alice Johnson: Engineering

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

4. Join orders with user data: Found 2 completed orders:

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
3A1UH696f5qnpJrqcP7cydw8RytBVZgW2EDAYDoXRrH67_UAfeowZ81CpQI1Yw53fBOly5lNPvxJdQCq9e4CWg

=== Sending Chat Message === Message ID:
fkIvaqkYU5XWnJoa-Wfj2z0jvHfpgqmzVN7ED5FfRPUVVPfxjmfo-No1xoOQ0OMS04y4uJ-v_SRRbDZlkoWxeQ

=== AI Response === I'm sorry, but the context provided does not include
specific product names or their prices. Could you please provide more
information or a different context?

Execution Time: 1396ms

=== Token Usage === Prompt tokens: 314 Completion tokens: 29 Total tokens: 343

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not include specific information about any available products. Can
you provide more details or clarify your request?

✓ Message 2 sent Response: Apologies, but I can't provide the information you're
looking for because the context does not mention any specific product or its
price. Could you please specify the product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
L7MaATzIiWydZp3Z4ltujVgD_qKK2hWjOmFdZGj8kwXRTtRWTWOKARsoRcTsCkAAnT1JqYCZ-x8Z26xl5kOwuw
Parent:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ

=== Listing Sessions === ✓ Found 6 sessions Session 1:
L7MaATzIiWydZp3Z4ltujVgD_qKK2hWjOmFdZGj8kwXRTtRWTWOKARsoRcTsCkAAnT1JqYCZ-x8Z26xl5kOwuw
(Untitled) Session 2:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ
(Untitled) Session 3:
3A1UH696f5qnpJrqcP7cydw8RytBVZgW2EDAYDoXRrH67_UAfeowZ81CpQI1Yw53fBOly5lNPvxJdQCq9e4CWg
(Untitled) Session 4:
Y9oeEuI0Dmbz-mFP_WrCEsZDbKYoKGSpek0DIqaEDWl5ia3L5pUftCBVIKBw4dtX5sMlhoqqhoK5Dj-TltDRgg
(Untitled) Session 5:
Ols-WO2aHQOC7MeahkVVK-T9Tc-ADE9GzB9ZhrbriEtpGVorP3vLjzQyHBQ_eURfu6VZqSeNJB5cO1Ge2zKbGg
(Untitled) Session 6:
pya1NJ3sd_plhrdNkxjV2UR64G7fwYPWoMMYOLD9nKmXIYUpiHjhWAWx9vDsNgnjfiUhdknf53UZWjg0-GZM8Q
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
L7MaATzIiWydZp3Z4ltujVgD_qKK2hWjOmFdZGj8kwXRTtRWTWOKARsoRcTsCkAAnT1JqYCZ-x8Z26xl5kOwuw

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
3EDVfbbWHgsPJN_XjedWrLfWpxxvu_ERmBn6mGon1k2HGx5KKYdtyyyjtiI-J45RZLOLTioqYwwpVGEqgnaONg

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't include specific information about any available
products. Could you please provide more details or specific criteria you're
looking for in a product?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['role',
'token_usage', 'id', 'chat_id', 'updated_at', 'created_at', 'content']) Debug:
First message role: {'value': 'assistant', 'type': 'String'} === Feature 1:
Regenerate AI Response === ✓ Message regenerated New response: Answer: I'm
sorry, but the context provided does not include information on the price of
ekoDB. I recommend checking the product's official website or contacting their
customer service for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
Cba_JGR3M-Emd84AjyvP6hxbHaewbJeQAOh-i20QRBPQACBK8Irp8dY1A6TPc5I9I56o0Qa25EiDAL1kGr_vXQ
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
jU5BZeOaQRYJACIQ_5EXwytkFCU_6jEIjMxoza6egUOyKbFyxgOMF0mypdzZ1_foOI_RJ6h4lA96N1FXHFjMmw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
xXddHwoNkWJu5VfOsLl4W8-DSBRyzTQQKM4pkP-gdhFxxKwTQWXhkjJC0dDl1sCRpp0S2uISe3GaqaAZ-L7Eyg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
qT6LvSgG9Tbc8bPCrN3svW-9U1IvPok5AW0IKvSN2pbGGge2C6eKld0b0PVkIlhycwOKCx3EQd0Qvti3Vx-EWg
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 5, 'status': 'active'}

{'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 9 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
zVgS42drtKzMvKQ0dejWfQ7pMfzd0o1_6PbE_pCfVDdRZwFuxPc1mSXUn73qfkW97F-vBS5WU15mMyF5osZrqw
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
