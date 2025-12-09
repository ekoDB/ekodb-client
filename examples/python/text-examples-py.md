make test-examples-python 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'8INv_qFN0b9ssm5aOFdK3yObZMRkYEUVoFNLg7TKSyYJsijIeL31Wdz3VNiwZq-micn4LYHCXLhyEUkApdTj5w'}

=== Find by ID === Found: {'name': {'type': 'String', 'value': 'Test Record'},
'value': {'type': 'Integer', 'value': 42}, 'id':
'8INv_qFN0b9ssm5aOFdK3yObZMRkYEUVoFNLg7TKSyYJsijIeL31Wdz3VNiwZq-micn4LYHCXLhyEUkApdTj5w',
'active': {'type': 'Boolean', 'value': True}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'value': {'value': 100, 'type': 'Integer'},
'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String',
'value': 'Updated Record'}, 'id':
'8INv_qFN0b9ssm5aOFdK3yObZMRkYEUVoFNLg7TKSyYJsijIeL31Wdz3VNiwZq-micn4LYHCXLhyEUkApdTj5w'}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
gxyJOI5aO4nhazTz8ifgYCc10ASLp66ugU-6T1lqQ8eZiBqUWG9WA3Sc0Dkd8Pjpv60sdsWUbp8WImMAo_lVzw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"xHB7dOzhn-TURJ68KMxUhViza0AENh17KfL1vjmYJsMyjVX49kQaqRZqLD_AbCoe-BrfQx-JZokRamE6b0Vm3Q",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"gxyJOI5aO4nhazTz8ifgYCc10ASLp66ugU-6T1lqQ8eZiBqUWG9WA3Sc0Dkd8Pjpv60sdsWUbp8WImMAo_lVzw",
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
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
vKWB0GFhmLTBYQ3tc1dLUTPf3m5KyzEbY4z_FljKONWgrZLzzfIDGpRZ-mOH0NNDST938EyiTKKiC_W2amw3rQ

=== List Collections === Total collections: 11 Sample collections:
['demo_collection', 'websocket_test', 'test_collection', 'ttl_cache',
'chat_configurations__ek0_bench']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
ozvL2wxIchT35cCgC7VEgwyY3bf9KVo87EcaYEWw2bxsJmbVYD_cutc2vV3ZgfFg1qci_rijlvbwvIF4-gV9rw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
jLMS3Fv2fTKbPUARREeoJ0KT0Iw1VuYhWPx3q7kTFH8eSF1Nxu3sxgYI7T9CbbJ-7j4f36BS4OaMG5W-uDj0Sw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully 🚀 ekoDB Scripts Example
(Python/HTTP)

✓ Authentication successful

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
zakqnvMDekK3-aWCtyGN9ax4o6NHnACRddYnpsMGPhJKZiL8K2sW7Qob69BUa0qOcODg8QD0xEWm1Th-t2_03A
📊 Found 15 active users

📝 Example 2: Complex Filter with Multiple Conditions

✅ Script saved:
pYM_YF8_Zcx8X8OVKMtbctgTidhSObkXvM6HpFegPg4gYVv5yVRB5TaYA_bFmVWkdvRbTmrGNPG5VzjrcpuV0w
📊 Found 9 users (status=active, score>50, sorted by score)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
snD0R3fTdIhUNcqVp_r1CKe95dE-AyKGMI5s7tl7TLN2UE6o4zwUmLPnMsLSzchZxusYPQOFDiUkDp7Lmw3n8Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{'avg_score': 60.0, 'count': 15, 'max_score': 100, 'status': 'active'} {'count':
15, 'max_score': 90, 'avg_score': 50.0, 'status': 'inactive'}

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
mrrcUEjCT0xOzeGskzqKUKXr6k8JInKclrq-1nNsHV6utKR24MckIeh1yvzGO9OXObdRFrjTHLPV2K4U0tUL8w
Created Bob: $500 - ID:
ohIAPqosWsPE9xbYfWLl7Dvky8nY0CIlaDFL_8bS6vbicdrFj3lTipOPxgkeHnMTj-NpI30uZxTM95E5HaRrRg

=== Example 1: Begin Transaction === Transaction ID:
88fcd72d-f124-4ce8-b9bf-94d41cd04e25

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Example 5: Rollback === New transaction:
40645dd6-5c39-407c-bb6a-871fe51ef9a6 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
mjvLQjVVQKwoa7HgWcvlijp6POX8N1MeHMC5Mj-eKSVSS_akgdOGWllkTZpjj_PfsbS2PBRMiMjh3Lmnq9fGKw
Created Bob: $500 - ID:
I-8pTC0m9QxeteQWSymNHbl-BG3UXy9nXwqrAEvyT25pXWObB3qmtkmpMX6WuGXwAQR1lXkeu4NGjoV45pksog

=== Example 1: Begin Transaction === Transaction ID:
262c03ab-adcb-422a-817f-03a032327af9

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'type':
'Integer', 'value': 700}

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to NON-DURABLE mode... ✓ Config updated: durable_operations=false

============================================================ 🔥 TEST 2:
Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
Hra42avoznNrN-UGVn-vqe1Y9sYIy3fRCG_tib8tPLEWhlM4ZynruklV0JfxF_Q4WduItRnYNOp-INwGIvfSQQ
Created Bob: $500 - ID:
ZTJuH1SxOJsa8UeUCUfn9qf08GC2WKXl9Kvd8oSPEWTaiAAk0vb4B2UX1a8l_MTfM49oqfl0hc4-tF336wBt_g

=== Example 1: Begin Transaction === Transaction ID:
e8f197a7-27a1-412f-943a-67b6f8d526b3

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'type':
'Integer', 'value': 700}

=== Example 5: Rollback === New transaction:
3d3f0b3c-89ba-4558-ba7b-a308275177b9 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
71S2b_zBC0G1OfRs0trddCTjLBXJmwc8s_bdTVi7U2KEpHkKqJwj-23Jx78nkqyAOhhtQSvGZc4zD3yPa4lPnA
Created Bob: $500 - ID:
oakM3_iT-8AZqWdDScBV_omn8FKpGwgpnuloWJhXSUGfe-7Wb47yTeJNzU89zyVsisygB-5pAQPN9pbl4nruaQ

=== Example 1: Begin Transaction === Transaction ID:
28f36a07-7771-4e64-9b2b-085532ceb900

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'value':
700, 'type': 'Integer'}

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to DURABLE mode... ✓ Config updated: durable_operations=true

============================================================ 🔥 TEST 3: Durable
Mode (durable=true) ============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
522MI73dnvqvpaOJe7o7k6DgKNE6FIBrW_uss6v--XEPPSH4vmwedI9zjAadrh9jkT-1opbdRAS8WSZ-esGoQA
Created Bob: $500 - ID:
sg-Ekpcsh-q0Zv-Vtd3rhr6ngHYt4OsrBF45hNjpzuH2-BsB3yHXHbT2JIZeN9JQY5pBOo6_z_rb8s--lWIL-g

=== Example 1: Begin Transaction === Transaction ID:
ae0fa13b-f9da-4b6f-af3f-83a529893a09

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'value':
700, 'type': 'Integer'}

=== Example 5: Rollback === New transaction:
8ddff3e4-7828-404f-aa2c-8a778eba91c3 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
i4XTeakagkHtN5kGXGCeyiFpEPm5ytQmuFtvTcnoRscrgygGMbcGks5NkGxEMb9wqq51qY5qS5S10laswO4Ccw
Created Bob: $500 - ID:
BO6771wiS2GsDH1ixXdBvPbymFJ-nP3tn89LeJMCasjW61PcQ7wNK6sx-KCrhzdBicsUqz_YTHunUp6QemRTkA

=== Example 1: Begin Transaction === Transaction ID:
828d61eb-8e57-4935-9409-dec66adefb3c

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'value':
700, 'type': 'Integer'}

=== Cleanup === ✓ Deleted test accounts

🔄 Restoring original configuration... ✓ Config restored:
durable_operations=False

============================================================ ✅ ALL TESTS
PASSED - Transactions successful
============================================================

[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Python Examples
Test Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m ===
Checking Server Connection ===[0m [32m✓ Server is ready[0m [34m === Getting
Authentication Token ===[0m [32m✓ Authentication successful[0m [33m === Running
9 Examples ===[0m [34m === Running simple_crud.py ===[0m [32m✓ simple_crud.py
completed successfully[0m [34m === Running simple_websocket.py ===[0m [32m✓
simple_websocket.py completed successfully[0m [34m === Running
batch_operations.py ===[0m [32m✓ batch_operations.py completed successfully[0m
[34m === Running kv_operations.py ===[0m [32m✓ kv_operations.py completed
successfully[0m [34m === Running collection_management.py ===[0m [32m✓
collection_management.py completed successfully[0m [34m === Running
document_ttl.py ===[0m [32m✓ document_ttl.py completed successfully[0m [34m ===
Running websocket_ttl.py ===[0m [32m✓ websocket_ttl.py completed successfully[0m
[34m === Running http_functions.py ===[0m [32m✓ http_functions.py completed
successfully[0m [34m === Running transactions.py ===[0m [32m✓ transactions.py
completed successfully[0m [34m ╔════════════════════════════════════════╗[0m
[34m║ Test Summary ║[0m [34m╚════════════════════════════════════════╝[0m
[34mTotal: 9[0m [32mPassed: 9[0m [32mFailed: 0[0m ✅ [32mPython direct examples
complete![0m 🐍 [36mBuilding Python client package...[0m 🍹 Building a mixed
python/rust project 🔗 Found pyo3 bindings with abi3 support 🐍 Not using a
specific python interpreter 💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for
aarch64-apple-darwin by default Finished `release` profile [optimized] target(s)
in 0.08s 📦 Built wheel for abi3 Python ≥ 3.8 to
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
'xlGucQYSWGgINvoFHQQ62r5KgD5c9pFEI1oZW2Jsfm6qbqWehp\_-N_FOdCI378eCHrsJkNFOZOmgZt2xmscmzw'}

=== Find by ID === Found: {'value': {'type': 'Integer', 'value': 42}, 'active':
{'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test
Record'}, 'id':
'xlGucQYSWGgINvoFHQQ62r5KgD5c9pFEI1oZW2Jsfm6qbqWehp\_-N_FOdCI378eCHrsJkNFOZOmgZt2xmscmzw'}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'type': 'Boolean', 'value': True},
'id':
'xlGucQYSWGgINvoFHQQ62r5KgD5c9pFEI1oZW2Jsfm6qbqWehp\_-N_FOdCI378eCHrsJkNFOZOmgZt2xmscmzw',
'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'value': 100,
'type': 'Integer'}}

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
"cyptF36QktdLePjstpC24RwU7054jIq1RZ7KXErVrovWb4ssH6ix7a1KzgsIhpo8GtWgYHdeA6zzK27EFNcCLA"

=== List Collections === Total collections: 11 Sample collections:
['client_collection_management_python', 'websocket_test', 'test_collection',
'ttl_cache', 'chat_configurations__ek0_bench']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
YdbfSMV2JlR0KTR-1HhJHBCZyejkBy703tdRql3Pd2GfN_s5iDKby_GQ4Mr8RU0YHMjauA_8cN84mm-y1gOf_Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
4sqxDsR1WTTSXzAFVurbRA424tQL78AC3NWQPNCBWKGWYnj6R3Yoji64zKkLdXEY2vDqgBom3iRhX7uL87zvWA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
MnN2o-GMQKttEo_M290i5bjluf4K-7OirEuN9nXWK9LlhkbbJgdNZ2CH-iFy3a3AcQymEFam3DwwFvKNmcrQww

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
wfL-H_0CQAwqav5U_n4DLUq1xAayWtSu2zBJetsO7xlUX5snhOiQdjREKCTSICbc31VwAbfS7S4-hP5BR2b1Bg

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
1. Score: 25.740, Matched: email.value, email, name, name.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, title.value, bio, title
1. Score: 26.400, Matched: title.value, bio, title, bio.value
1. Score: 26.400, Matched: bio.value, title.value, bio, title
1. Score: 26.400, Matched: bio.value, title, title.value, bio

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: title.value, title, bio.value, bio
1. Score: 39.600, Matched: title, title.value, bio, bio.value
1. Score: 39.600, Matched: title, bio, title.value, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.762
1. Score: 0.747
1. Score: 0.747

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.723, Matched: title, title.value, content, content.value
1. Score: 1.134, Matched: content, title.value, content.value, title
1. Score: 0.523, Matched:

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
GcJoGq8p4I9IzQJSpC3mOI9-Knp0ck9RKI25PN7C5bKsw0x4b-iHoBJ16NxTCck-JY9EZKnSBXcCVt5Uqoo2Qw

=== Sending Chat Message === Message ID:
lW12GFTXFhWDaV72np-zf8qiBcjgoY1PWdEPEVThZoCDBHubCFWrlaC_igVc0KujfodyknZbBKElZXqGqkODOg

=== AI Response === I'm sorry, but the context provided doesn't give specific
information about the available products and their prices. Could you provide
more details or a different context?

Execution Time: 2079ms

=== Token Usage === Prompt tokens: 320 Completion tokens: 31 Total tokens: 351

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
jl_qC01XuY7UlWB31pIZDlb5ZT28fbBT40JNtj-beGEb5IMZTAfAVmzBAP5t6TIvQfE-17axFs_WESUzv_q1lA

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided doesn't contain any specific product information. Could you please
provide more details or context?

✓ Message 2 sent Response: Apologies, but there's no information provided about
a specific product's price in the context above. Could you please provide more
details or specify which product you're referring to?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
n9GQL3zQNX9gquVvf3FIOSjTOPPIQCpAVnnzqMFJ6_gNIValXIQ4aE8OKPdYoFdyN7jvGnGB6gpIWwa-3KUgCA
Parent:
jl_qC01XuY7UlWB31pIZDlb5ZT28fbBT40JNtj-beGEb5IMZTAfAVmzBAP5t6TIvQfE-17axFs_WESUzv_q1lA

=== Listing Sessions === ✓ Found 6 sessions Session 1:
n9GQL3zQNX9gquVvf3FIOSjTOPPIQCpAVnnzqMFJ6_gNIValXIQ4aE8OKPdYoFdyN7jvGnGB6gpIWwa-3KUgCA
(Untitled) Session 2:
jl_qC01XuY7UlWB31pIZDlb5ZT28fbBT40JNtj-beGEb5IMZTAfAVmzBAP5t6TIvQfE-17axFs_WESUzv_q1lA
(Untitled) Session 3:
GcJoGq8p4I9IzQJSpC3mOI9-Knp0ck9RKI25PN7C5bKsw0x4b-iHoBJ16NxTCck-JY9EZKnSBXcCVt5Uqoo2Qw
(Untitled) Session 4:
DZrN-AQV9Nn4wOTelR2KA6397oce29THpcBtxbI_iT2QOQo1yaK92LZIYuZGB_sN1pUrtH1Cg8FTgApuh7BRoA
(Untitled) Session 5:
ibO8oA4VdNKJx8MlQKtRtIuVB5WanPjnMLOrJSIIietcdTJk5n1aMoIgHOY00veFNThz80mq1tJMflxUHEmqEg
(Untitled) Session 6:
wFcIqUdFKFba7_NRR2SD1v1j68swGE6y6e1ZCGgA8_XfIMj_dJ_fXiQhjVuSR3TrnICo0OW2clU5xYAVpYChpw
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
n9GQL3zQNX9gquVvf3FIOSjTOPPIQCpAVnnzqMFJ6_gNIValXIQ4aE8OKPdYoFdyN7jvGnGB6gpIWwa-3KUgCA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
HJ1BMC6-Zmke1bkj_3EyQxRHJo6ZMR8QIQ3TP4_iqGJh75zyqRL5WbNomZX8AlYBuAvqbLViwuP-UGSOKyNgaw

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not give specific information about available products.
Could you please provide more details or clarify your query?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['content',
'created_at', 'id', 'role', 'updated_at', 'chat_id', 'token_usage']) Debug:
First message role: {'value': 'assistant', 'type': 'String'} === Feature 1:
Regenerate AI Response === ✓ Message regenerated New response: Apologies, but
the context provided does not include information on the price of ekoDB. I
recommend checking the official website or contacting the seller for the most
accurate and up-to-date price.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
3dKLTjbKKdWiKWJMiZSl27k-QOuXrlUsrkYfN3u6MHsPZpxafROQqHN7VqKXYIA_cDw_ok8lLtRs1RHmTZtdpg
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
H0DsA5rwBSSBpMpgXPNcJj8GZgRf8G-klwnfYqRyJfzkFi5f-2u3zF2um31HGlyS4lMER0GPqJo8HGnJ-N72Gg
📊 Found 40 active users

📝 Example 2: Parameterized Script

✅ Script saved:
pycIX-6PH5pFGe2LTKFTeDcGFfM9cGA4H2csEgA1YTCxGKCCeKfw8FX7Zp5kKhSmLJs7FXTGXT9rWl4HVd4MRw
📊 Found 40 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
zvtxKDbrGza97k4P5pCfHo2x0o8D_erylpA_Hsb9XaNrzYD00GLm0iNntXQ03fq4nCPUSD8UL27XbrxEabLKGA
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 20, 'status': 'active'}

{'avg_score': 50.0, 'count': 20, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 9 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
tWdOtGVvYJ3nDDkKSEtJVl9DlhQUoFIbeB3CEayoWZzSRs7SIJBITfIV2vQHGkryrYhhwoSZCerjRGVj-xRsXA
📊 Found 2 product groups {'avg_price': 575.6666666666666, 'category':
'Electronics', 'count': 3} {'avg_price': 474.0, 'category': 'Furniture',
'count': 2} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {'category': 'Furniture', 'count': 2}
{'category': 'Electronics', 'count': 3} ⏱️ Execution time: 0ms

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
