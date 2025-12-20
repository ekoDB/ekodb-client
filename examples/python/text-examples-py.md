make test-examples-python 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'0b2eEPAdIv7qIW0-GAmSc1mYz2D3oDCdAeKVOHxAamT-3DDDbIddV6V-g09iGSed5PQdOYRwkTeUInWh-6OkFg'}

=== Find by ID === Found: {'value': {'type': 'Integer', 'value': 42}, 'active':
{'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test
Record'}, 'id':
'0b2eEPAdIv7qIW0-GAmSc1mYz2D3oDCdAeKVOHxAamT-3DDDbIddV6V-g09iGSed5PQdOYRwkTeUInWh-6OkFg'}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'value': {'type': 'Integer', 'value': 100},
'active': {'type': 'Boolean', 'value': True}, 'id':
'0b2eEPAdIv7qIW0-GAmSc1mYz2D3oDCdAeKVOHxAamT-3DDDbIddV6V-g09iGSed5PQdOYRwkTeUInWh-6OkFg',
'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
dTPCdp_g6cKAoqQLPCx-Y-rgKg5040O1X1Y4EBATWa3og1GDIm2dwdlm5-E8NZFc3KliH7RMsq4MNmmrvv0NMQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"-g3odd0twQrSsVSLVsISeu0mVnyzhoYUmFFRGNOwE63t_8qQwJM0iIFFDVgCcXfWKD0yRacu6KH6p4ml-i7KgQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"dTPCdp_g6cKAoqQLPCx-Y-rgKg5040O1X1Y4EBATWa3og1GDIm2dwdlm5-E8NZFc3KliH7RMsq4MNmmrvv0NMQ",
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

=== KV Get === Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
7ZPXWPNsWpp9BZP4CS11VddoAS7-F221UQVm0ooqsm4FE7W1pdjzxNB-3SNZ7jMqnaEH-Xz7dcAQLEPzK0aMpw

=== List Collections === Total collections: 11 Sample collections:
['scripts__ek0_bench', 'users', 'websocket_test', 'test_collection',
'test_accounts']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
2fCysQdUJFn0uP-9i0XW6lYgZR4Q_ERcSVTLRd_Er4SMaehbgfzghf5MZDFI4bXmOGTcwjv83OFyaPx6aIuLUg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
9eq61ac2iCRMbYxMysrUM2_TBAnNRKQ2Q9_51KZQJj9hglzp1dvA4XrncYocbBvLjKXK5Yq2x1ptsOm6W6EBxw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully 🚀 ekoDB Scripts Example
(Python/HTTP)

✓ Authentication successful

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
vGns7ObnUR4Inrl0NJbcIKJPR31HJ1XVIGCRP1k\_-aVvyy5oXe3OhZ5Tn5y2LZZpltO3_VHeY_Zilf4eC-AR8w
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
lBIGFL-WwCZOlPefhXzp1VHBEG6TjF-\_B_aVZYg2KvnSq7M_9K66AQrcHEFDWQtSPh0GqKG7zfXiix4rkQr1_g
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
QYyd0nJaYutOdP8eP5O3EBdXlBgmjdT4luoXHgEGSWxtXVv_gpg5XcMQYRB5i-pF7JxY_BgOtlrmbKUXS3oWJg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups {'count':
15, 'status': 'inactive', 'avg_score': 50.0, 'max_score': 90} {'avg_score':
60.0, 'count': 15, 'max_score': 100, 'status': 'active'}

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
Cq5gvK8s6rA9DPlMJBP5G1oMAJNAIpzv5Iis8owE1-Zp-t2-cluEIdAdliv8BbJ_j7XASIv0o7VzHLpQRHZfKw
Created Bob: $500 - ID:
XU2qas4x9isSzJEB1VBcg2ZVT98m_1pu7CuU4HoTEj7ut8iLS4jvcDYXs0mK8yGgzhnVyJFE5RllbGN2jBX77g

=== Example 1: Begin Transaction === Transaction ID:
666ea42c-98b8-4fa3-9972-d7cac1e7ac84

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
f69690d1-b9d1-47bc-bc6c-d24dd73c2987 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
B65yf_FfotM9o7R--oTjvbWWU3kXXiKFNCFWcC_Pm936PkcDmvpO0LnWLxfY42nP-U9Cdz6DkGquGYUZPJN0Zw
Created Bob: $500 - ID:
qHxoz-sycuD78DBGkT3-AcN4fhBq3wLwzvqBDeLXbGuBx5dW0rplh-Z3kjnth0vj35Z81-lnRCx0wU_jiwAIgQ

=== Example 1: Begin Transaction === Transaction ID:
961be601-6b30-40eb-9a27-a8c8f72bff00

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
lirolGvIZxIUEaU3e8QB2_nKim8RTha8RGdXD9z6o_j-yC9zC6JC5v-yHG2_2c3wJSqMG7ONzWtZQTiBWbq3RQ
Created Bob: $500 - ID:
zHvExxWEusF21XwN20W7JvkOlVtntg2R7hyMzY9mOhBbvcQoaxCvCJDzTiFntgJ4qL64AH--ldQriMRstm40ig

=== Example 1: Begin Transaction === Transaction ID:
52af1949-f4fe-48e0-a874-5ddf04ead181

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
cbbd0e64-069a-48be-aed2-e32113d56e6d Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
f9yFv-7icuSkB2nxC5eojWlf2uuNKuztgA2j80KjlaQSnQCbtcakUDijEQKtMOTEsqWwtXKhBCLpPRL79ikDdg
Created Bob: $500 - ID:
NasvWFpz_nmGcBfEBAQX_fHWYMSS99NrF0vD0PWj6x5bjG3YGiBCuXqhoRnieT0GWECFaWKNWRzu0GwSOjzoSw

=== Example 1: Begin Transaction === Transaction ID:
3b1fb635-9f67-45db-b9f4-b889e79d872e

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to DURABLE mode... ✓ Config updated: durable_operations=true

============================================================ 🔥 TEST 3: Durable
Mode (durable=true) ============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
xvX_oLMEDAK9xX07wTSvijpMOoNzZJtJOG93zTVWWIDAhEp5VzGu7ELgziTPiltcNUhbch7Oowx3kDqiluKvvQ
Created Bob: $500 - ID:
22g3XVXEdJp1K_hxCnWkbCl65Cga0lzORAF-UnmbqKcnxvfe3VPUxn3EQyzfc5q7lA8uWevm9M3cndNr7XEgKw

=== Example 1: Begin Transaction === Transaction ID:
f77d79bc-8e46-4fdf-a623-b58bd9134c9c

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
7ca8aec7-8553-4c64-9501-83f809e06d63 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
-U4W2xdGETJe4wYV2guTkoGG9rRDgRWwsk4Wh-d-Fz0JXcR9Vyfn0OFfGIcogZwwDToei055G44X9emOetOOgg
Created Bob: $500 - ID:
OLDsoa6CcepY8iWzF3K2qx64slrMJ42nPpc2Rs6X7YAatjKoIOsuFsM7lYQLSb430Vt5mGr8PVLKpTFy0UgT4Q

=== Example 1: Begin Transaction === Transaction ID:
164a4e72-ef74-4b2c-9fe6-c0ce590beff1

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
admraV-x3hTt_IdYco_8frhrU0lE4WeJrO3kMPTojGxIrmvD7KCG8lFtVaKO99HZ8hx-Nm5bfksfmhbMT55mIw

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
U0xGwy8D_VpO9-z9AfIRWJv27T6T_xOUInPGtpinzuDpKCgqbYjcme7CH6Cw-NbWSyCFk83ckdZFAc5OdnytcQ
📋 Name: {'type': 'String', 'value': 'Alice Smith'} 📋 Email: {'type': 'String',
'value': 'alice@example.com'} 📋 Status: {'type': 'String', 'value': 'pending'}
📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
22O_tRh1bWye0Jklc-14DUgGur7xIDmZBnGG3g4V0R704gZDpBAAfP8zjOFwZ4QFm7D4MMc3rdQCe0phhZl9Ig

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to: {'value':
'active', 'type': 'String'} 📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
U99W7dp_QibtDjX8MBnjMKNQTbco1T-eKHjCFkNi76L5MM3Lm5hh2ZFlhYOS8z29wx7GgMVBFdd3iZtBNEMZTg

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to: {'value':
0, 'type': 'Integer'} 📋 Status: {'type': 'String', 'value': 'active'} 📋 Name:
{'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
Xoi5FvjMTsD5DcqC3oi9zJyvRym4pNzdsFC65WvlYKjqAFFg2afnumVcZaempB8aW8J1DCPd_U475eCiJCgkLw

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
admraV-x3hTt_IdYco_8... ✅ Deleted script: 22O_tRh1bWye0Jklc-14... ✅ Deleted
script: U99W7dp_QibtDjX8MBnj... ✅ Deleted script: Xoi5FvjMTsD5DcqC3oi9... ✅
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
default Finished `release` profile [optimized] target(s) in 0.13s 📦 Built wheel
for abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m Processing
./ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Successfully installed
ekodb-client-0.4.0 ✅ [32mPython client package built and installed![0m 🧪
[36mRunning Python client library examples...[0m ✓ Client created (token
exchange happens automatically)

=== Insert Document === Inserted: {'id':
'kq9-EKebeSyav56wOwfdXcvYf4RRuHH5g3GTb33CFabH2AZiiKgTQQJGWcmGxLcPGSOmS1nkFLzHHCI8zMjBYQ'}

=== Find by ID === Found: {'id':
'kq9-EKebeSyav56wOwfdXcvYf4RRuHH5g3GTb33CFabH2AZiiKgTQQJGWcmGxLcPGSOmS1nkFLzHHCI8zMjBYQ',
'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record',
'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'id':
'kq9-EKebeSyav56wOwfdXcvYf4RRuHH5g3GTb33CFabH2AZiiKgTQQJGWcmGxLcPGSOmS1nkFLzHHCI8zMjBYQ',
'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record',
'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

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
"DyzHSvf4H3h32qrWE5q3zHFfOfvIbByCtgPReew5YjRulc3oMDE0tDGF4tunCs3ZVQtKxMPy7KGwrqp9VcVepQ"

=== List Collections === Total collections: 10 Sample collections:
['scripts__ek0_bench', 'websocket_test', 'client_collection_management_python',
'test_collection', 'test_accounts']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
Cp47foUmRq3xplYGPWdRlWupssVh4loKGC85orhYjchcrxqp1wnA-wzk6-6qvvIEoPCbWxkZU8Vu8JmPxOSPPg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
eAWeemSz3153_uBCPiEQPFfCsWNzDorRdPNkIWw4Zr3DEohC4zstyzVwjte43jJv4PYdRc0JQtDosJxtHHB79A

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
khuNDoY2TIBByOsr2JmjM3xlvSFFOVVPz8p7saUkx4LYIyruUMpWu7gsjivuV9jnBEPi1JgaAMRqlDwebmaNqw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
QRZ_Q0EhMgAKVfAbMhzB-koQE48VkdwoWUR0Fh1lZY1AQEwzPUp6FyrpoJF7uC0PbAL8es25LqrhIUNQYgxMOQ

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
1. Score: 25.740, Matched: name.value, email.value, name, email
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title.value, bio.value, title, bio
1. Score: 26.400, Matched: title.value, bio, bio.value, title
1. Score: 26.400, Matched: title, title.value, bio.value, bio
1. Score: 26.400, Matched: bio, bio.value, title, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, bio, title.value, title
1. Score: 39.600, Matched: title.value, title, bio, bio.value
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: title.value, bio, bio.value, title

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.763
1. Score: 0.743
1. Score: 0.731

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.712, Matched: content.value, title.value, title, content
1. Score: 1.120, Matched: title, content.value, content, title.value
1. Score: 0.534, Matched:

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

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
jC9n-MLxQZb3--dNvpyK_CMNOi8WJDW2Mm_iNNCGVoKg8l-xArdqbsUhIQcVe-iVPo8jjolL5q5-41gIUXepAQ

=== Sending Chat Message === Message ID:
zH47drYScXP2CHGvijs8NhS6Txy6kopIuU2Y9c6CcnYXIk-Dcc0Z79h7lD_R-IRfTaJmvltz1hNmLKatMa6z3Q

=== AI Response === I'm sorry, but the provided context doesn't include specific
information about the names and prices of the products. Could you please provide
more details or a different context?

Execution Time: 1616ms

=== Token Usage === Prompt tokens: 316 Completion tokens: 33 Total tokens: 349

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided doesn't include specific information about any available products. Can
you please provide more details?

✓ Message 2 sent Response: Apologies, but I can't provide the information you're
looking for because the context doesn't specify any product or price. Could you
please provide more details?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
Fh9yMPjqKSHbTSfjflNvaXZGmC5XUwFaAmBxFJ7f7akUnp31EyAcxY9d--DsdTT9wjjsEZmNSebZuKVq_nOKWw
Parent:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg

=== Listing Sessions === ✓ Found 6 sessions Session 1:
Fh9yMPjqKSHbTSfjflNvaXZGmC5XUwFaAmBxFJ7f7akUnp31EyAcxY9d--DsdTT9wjjsEZmNSebZuKVq_nOKWw
(Untitled) Session 2:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg
(Untitled) Session 3:
jC9n-MLxQZb3--dNvpyK_CMNOi8WJDW2Mm_iNNCGVoKg8l-xArdqbsUhIQcVe-iVPo8jjolL5q5-41gIUXepAQ
(Untitled) Session 4:
fcQkL9x-SJ9VPI8YEYzJjcZh5CrpXeQAC8VL_ax38Gh3b2yMZsDgCkIYqONB9cTxXJiRFa5iA6AsALNAhrtwzw
(Untitled) Session 5:
O-C14UTsuD3FnQGVrtwgnk_yWQyQeK4v8dWxNrZVg2b8khWx1wOfk0BL6Wb_SIvHmEVaPQrg9QF7lQ2N9bD6Sw
(Untitled) Session 6:
tPrJggglSl_8sgRKtUhO7O74BjWJNwkAuJV493jQ-\_07K21Qrsf_Wtnqy-NM69uuKl475EErbtDdaRjEwITSqA
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
Fh9yMPjqKSHbTSfjflNvaXZGmC5XUwFaAmBxFJ7f7akUnp31EyAcxY9d--DsdTT9wjjsEZmNSebZuKVq_nOKWw

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
npi8nCqi4VoJXNetA3-XiriOJBVGGlTWhT79lXh-V4bR-\_BO3djsVLjXlPHnMYka41WqzI7K-1rSC_YAXaTmBQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain specific information about any available
products. Could you please provide more detailed information?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['id', 'role',
'token_usage', 'updated_at', 'content', 'created_at', 'chat_id']) Debug: First
message role: {'value': 'assistant', 'type': 'String'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: Answer: I'm sorry, but the
context provided does not have information on the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
HJUs621HzLEuG-m3hdCO5FpoC-fowMhZgf8gzr4v6o8alHcz6CEqa3dEurUFur_j7nF6j77ahqyjSO2SnTo54A
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
BFXx9qCk8JLY0YYkHj_bIEHjPWfsJruHhXjb3y8xZuhbCc98LEQ6fzfgdp9T16DKp762-8NepBHVX_BzeuHXTQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
6HEgY5dOU6*1GBFg_UMqky5hKTRqR9EOgVOt1AxLaZ9jTLobIDLN3Jia8cyBXmVYS*-Kyi-1gEl0h6bldVSFQA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
tGNLO_sIHTvRw7CJ2DaQ1uhhmEVs3CohSXcj20azKjdmH7gNFdgZzWIpbMMd7hS-5cz37kBzz6EHm6lb70eYNQ
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
9CFu1ShlhxCCNeUZhp6VNfbEvuT1QgyBIRhBkSE9mU8MuoRtwJkjH5_sPzyy31fKOO0jKgAMPyEQUkf62cF_JA
📊 Found 2 product groups {'avg_price': 575.6666666666666, 'category':
'Electronics', 'count': 3} {'avg_price': 474.0, 'category': 'Furniture',
'count': 2} ⏱️ Execution time: 0ms

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
