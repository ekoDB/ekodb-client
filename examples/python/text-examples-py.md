make test-examples-python 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'rfImyx0Q4L30WQ-kPr9UhtQu8jZxXLQpiSt233mQHm5P1hwFeTrh_MPZ7vqfkD9tbLZZ3sILPgTL2Wis9UYLtQ'}

=== Find by ID === Found: {'active': {'value': True, 'type': 'Boolean'}, 'id':
'rfImyx0Q4L30WQ-kPr9UhtQu8jZxXLQpiSt233mQHm5P1hwFeTrh_MPZ7vqfkD9tbLZZ3sILPgTL2Wis9UYLtQ',
'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42,
'type': 'Integer'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'name': {'type': 'String', 'value': 'Updated
Record'}, 'id':
'rfImyx0Q4L30WQ-kPr9UhtQu8jZxXLQpiSt233mQHm5P1hwFeTrh_MPZ7vqfkD9tbLZZ3sILPgTL2Wis9UYLtQ',
'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type':
'Boolean'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
7mvDvjLu-3eoQZwjlZon8ia6SymRSNNQo0VlDw_zXmr7rkofCdE4da0lfkWdQKmCUTscj8culXnsEWFRpq3cCw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"-StpvC0i8Gt6eyenL3QP9h3rW2r0I9qfrnNaba88DIxHpZPxfUglMnSlcwVMvT1ZWYpBf50k510Fiy7LaTQCaQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"7mvDvjLu-3eoQZwjlZon8ia6SymRSNNQo0VlDw_zXmr7rkofCdE4da0lfkWdQKmCUTscj8culXnsEWFRpq3cCw",
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
OC2B1SQkhxB_xDPI7p6I5H-ezEpLAdT9cTBS41M5_STySOdSz5DNCb1eL6pyP5vruBMLV4gsceH-gH9WXwDxsw

=== List Collections === Total collections: 11 Sample collections: ['users',
'demo_collection', 'test_collection', 'ws_ttl_test', 'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
cnEjeWZi6R6hvDC-H782GwtR5YK_nh0P8rjoPXsNSe3vLcFexb0fkzDF-crj-vRUc6DjK-t1Sggsz0LpcHGy4Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
CFNm2sX4w_8UkQ0i_HSJtOzEke-1BpY8B7yUW6s374NGFQJm9YDQVLtLCoxJhsSy36Z_QyzO703DMw5E64cBfw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ Authentication successful

📋 Getting original configuration... Original durable_operations: False

============================================================ 🔥 TEST 1: Original
Config (durable=False)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
b8cAtsZTYmGb3-TFcTkKANIt0ErZTC6HAsDNDzIov43yj07E33YCNHrJiTABuJcPzt2u2p2Ux74IsCoyDrEvOA
Created Bob: $500 - ID:
b2L7d7Nyp3TpZqk0vPUiJZJ4X40eu5IBJ22Wh5nreFnQqmKjQAU5yjUYK6Gy5cT806PveWPsHWeStS_k6enJ-A

=== Example 1: Begin Transaction === Transaction ID:
122b3d9a-645f-4eb7-9bdc-f095a4eca2a6

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'type':
'Integer', 'value': 700}

=== Example 5: Rollback === New transaction:
8e42790a-0da9-4e99-90da-d27366a5fdff Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
i14q4WzI6M1xSo1dPIXwIRhqOv2kt9GURwLKCrOAnBocuecACFtYUelgVM91YAUB2DMFZJEN9uY-FJL3DBBqWA
Created Bob: $500 - ID:
gTsBA9gtf3EfB3FSohw_dzMlRIgt8Zcuy_9Sn8Z-cjx7ouqD4nAzrLtdk1D8ZLBIVyk2DeZ2lGTcdBbL3BnKIg

=== Example 1: Begin Transaction === Transaction ID:
f91b19f2-a9eb-4a65-a608-1b3349c72282

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to NON-DURABLE mode... ✓ Config updated: durable_operations=false

============================================================ 🔥 TEST 2:
Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
ZkWU-CuJt69FzCUr7Cz277PyRTXN7ZIpRMhkeyzAfPEzi005V2r6olZN9YH3gp_x-Crr4kfyzxCGqG72LXTTUg
Created Bob: $500 - ID:
64PvEGrc-YSB8KNBDSA226TRJg2OEmwtqmmT7iIxpkpMfrPqyIBGJ2EyhtjVlLlI7RojRo1R6kZs_oIbVduvXw

=== Example 1: Begin Transaction === Transaction ID:
1e684921-985d-40a8-96e9-b073fdf20dd4

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Example 5: Rollback === New transaction:
4e65ffba-9915-4ccc-b6e1-bf83c4799b24 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
rFu44nbmAUEcTSM6cTzr1-30AdFOtSTHZOjRPnwdk0msTlPBEXFRrq0UrcIj30T_u8FT9gV3Sd65RFSVSZ45OQ
Created Bob: $500 - ID:
Nof5flxJ9tNedj1hVURTzt8H9OMRjeSS9yuhcAfBYEKCyXmFd7l7FYdZC2nuDFYjOuZMrEKrQ5pf2Jp-FST_jw

=== Example 1: Begin Transaction === Transaction ID:
8e527684-e4e0-40a9-8bf2-0e4683beb7a4

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
JfQqyQPxBTh4qnjABL*fvT6LdWZI7N6Ncmgfq0H2gOPO7KIH2zCgakEnyGGTGK8j9kl6n9f2NrzGb7lEL5JsaQ
Created Bob: $500 - ID:
qgqxZGc20yJLt3kt-\_rML6yA8jMBK8w8qMddvNBPw_TQ4tQyJa0*-vOoJsAKPMpa-70S3M5htl_edd-KKOzGgg

=== Example 1: Begin Transaction === Transaction ID:
71c80dad-0c47-4f81-ac9f-d9341687f536

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Example 5: Rollback === New transaction:
700c754e-e2eb-4683-b5f0-b2e898dbd849 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
XTrIsqzLWObvZLdyT4xaVDW13BTdQvaszAfTWAR-76LGjQFwUwJMqQ1Ca4jtW2wza5AAh0xTvQ-tDBK_ecno9A
Created Bob: $500 - ID:
14v1NOmvBZcSoIX4AeGLsP1NAekGfUoYyg9E-9uQziGRSgj3UMGMYqB6oyHK0Tk0A5CUcquMrTZSU8zMnmAdfQ

=== Example 1: Begin Transaction === Transaction ID:
4295f880-3549-43c8-8bb6-24c3b18d81db

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
8 Examples ===[0m [34m === Running simple_crud.py ===[0m [32m✓ simple_crud.py
completed successfully[0m [34m === Running simple_websocket.py ===[0m [32m✓
simple_websocket.py completed successfully[0m [34m === Running
batch_operations.py ===[0m [32m✓ batch_operations.py completed successfully[0m
[34m === Running kv_operations.py ===[0m [32m✓ kv_operations.py completed
successfully[0m [34m === Running collection_management.py ===[0m [32m✓
collection_management.py completed successfully[0m [34m === Running
document_ttl.py ===[0m [32m✓ document_ttl.py completed successfully[0m [34m ===
Running websocket_ttl.py ===[0m [32m✓ websocket_ttl.py completed successfully[0m
[34m === Running transactions.py ===[0m [32m✓ transactions.py completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 8[0m
[32mPassed: 8[0m [32mFailed: 0[0m ✅ [32mPython direct examples complete![0m 🐍
[36mBuilding Python client package...[0m 🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support 🐍 Not using a specific python
interpreter 💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by
default Finished `release` profile [optimized] target(s) in 0.08s 📦 Built wheel
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
'DuHXjjvqL-VtNeslR1ghBDgEaPHUQqADYHDJuI5a9agIwBVAUG-fk2f2NAQnz4dY7Ce5s93XMG3YBgxhpQi8zA'}

=== Find by ID === Found: {'id':
'DuHXjjvqL-VtNeslR1ghBDgEaPHUQqADYHDJuI5a9agIwBVAUG-fk2f2NAQnz4dY7Ce5s93XMG3YBgxhpQi8zA',
'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True,
'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'id':
'DuHXjjvqL-VtNeslR1ghBDgEaPHUQqADYHDJuI5a9agIwBVAUG-fk2f2NAQnz4dY7Ce5s93XMG3YBgxhpQi8zA',
'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type':
'Boolean'}, 'name': {'type': 'String', 'value': 'Updated Record'}}

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
"EkqwpueNc_UgUWKmPi3uNQR2Htjupj3yu7LGbxkVUm1rml2f0OpSxjZVCAzA0IHRqEuQe_FZdf71VAza5kWigA"

=== List Collections === Total collections: 11 Sample collections: ['users',
'test_collection', 'ws_ttl_test', 'ttl_cache', 'websocket_test']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
12KXBZYllJpfsRMGjw6esftpX453eOPZh-7Nf28DetBmgeHEckG7VOT9i8eFmwdThCopividxi0KaO9jNFpwWQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
9T9n_OEibzvPaaBLAZ1hx0LyTN1SKJavksbMT4CdXj8w5RZvfXJCJ-up1ceE976sXOEN3ZRlR1GAm0UeNMftpg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
8P1yBpYt_f0ek3k-7leIy8smD9kQ3NEMb3GStCRQRwfGp1Hh7gzL9VMcOJob8wFyP77JIt4O4HAeHjAENU0KxQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
pWroJEYL5-lwaPFE5PYG_Oiz5wot7bsOLrWpKmM8FEAVHFalMfoYJOKs_QZyZ627lRn4iEAGUwzW6yoYvSHlsA

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
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: bio, title.value, title, bio.value
1. Score: 26.400, Matched: title.value, bio.value, title, bio

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, title, title.value, bio
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: title, title.value, bio, bio.value
1. Score: 39.600, Matched: title.value, title, bio.value, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.764
1. Score: 0.764
1. Score: 0.745

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.722, Matched: title, title.value, content, content.value
1. Score: 1.135, Matched: content.value, title.value, content, title
1. Score: 0.535, Matched:

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

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

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
\_ZfZ-i61nrfaPM1Ql5QjKywbDAUwP0ApPNSm8aSneEByuytGCny8Dec49cx3-gRpg03AvoP_26-ID7_KtQfCLw

=== Sending Chat Message === Message ID:
IGBbxIDtjFOpl_BCM69QWCgqE2hqWF1lpMAcKOXa_44c59gVemZjDeYnU3DWPdk6Gzv0rbiFp2lFB8cp8KafQQ

=== AI Response === I'm sorry, but the provided context does not contain
specific information about available products or their prices. Could you please
provide additional details or context?

Execution Time: 2026ms

=== Token Usage === Prompt tokens: 317 Completion tokens: 29 Total tokens: 346

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the provided
context does not include specific information about any available products.
Could you please provide more details or a different context?

✓ Message 2 sent Response: Apologies, but the context provided does not include
information about the price of a product. Could you please specify which product
you're referring to so I can provide the accurate price?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
Vid2OjfaNug9XN5oIOmJyAO9yhSaIX_29Bhur4OjfwKD41BeqzhdkXGsLzioIBMlbfEVi8X5rx1Hqdm4aH3tiw
Parent:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA

=== Listing Sessions === ✓ Found 6 sessions Session 1:
Vid2OjfaNug9XN5oIOmJyAO9yhSaIX_29Bhur4OjfwKD41BeqzhdkXGsLzioIBMlbfEVi8X5rx1Hqdm4aH3tiw
(Untitled) Session 2:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA
(Untitled) Session 3:
\_ZfZ-i61nrfaPM1Ql5QjKywbDAUwP0ApPNSm8aSneEByuytGCny8Dec49cx3-gRpg03AvoP_26-ID7_KtQfCLw
(Untitled) Session 4:
B0IRmVduPyhJcej9WW3Kz3ykpu3orY3VUi_OYpThxcXTAAnvs_tOFH5m72d4njdGdiWzc0kZ-wkLVPly6-AHOg
(Untitled) Session 5:
7BDD-zJBVBWMsH1TZJ9mpV8ZjmliM9Il_h-MDHYIRwB09rF5oK14CqlOiemb2KwvAOhtymFAKWv5RYXVLn9LGg
(Untitled) Session 6:
YkiDVUICmI0KGpwjWBz6CkhCmH570quCFqsURqHvH-deVCrMTx6Hp0qv3b3MepmPdkUcfSUdnEOABuONpyYBKQ
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
Vid2OjfaNug9XN5oIOmJyAO9yhSaIX_29Bhur4OjfwKD41BeqzhdkXGsLzioIBMlbfEVi8X5rx1Hqdm4aH3tiw

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
WCg1KYLRUW1M91O-oLeLJtZan_OaKNtWXG_2g_jDnTZ1ja3C3d4Mo2jflrPF7BEWRt1yzuK_CVk5qredt3oFvQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not include any specific product information. Could you
please provide more details or context?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['id',
'created_at', 'content', 'role', 'token_usage', 'updated_at', 'chat_id']) Debug:
First message role: {'type': 'String', 'value': 'assistant'} === Feature 1:
Regenerate AI Response === ✓ Message regenerated New response: Answer: I'm
sorry, but the context provided does not include information on the price of
ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
48IeSr-b5OvgoPOo5UeudSGqoPBWM9t1iupVu29o83S7xz239syKzCaUsRhQY4gO-CBRPK3vUG5Fu8m694PnmA
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
tW1o_iiQQ-5cg9d0KX_a_vvsxJUWiWIH1Zjah5uRZTGaVU9j2xWkObIIGATHK4OtuNskUMuDHte-VvtFXyrE8Q
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
fNZKoLQgLgZfLC8t2quBrgRnibZlHStNWZYsMVscp7ILNVyWzYz3xyO-AAU0Hs6Mpeoea9FQmwN5Fjm2ZdRisw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
E1NARJ_03PorHZZco1JygYza-8f113bMd_12qtAIf2QB6mZIfFCMY07tk4EFaZ5HTNsg4zswU8fjI74M-LkjTQ
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 10, 'status': 'active'}

{'avg_score': 50.0, 'count': 10, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 5 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
vJb2ZQuyPEYr76krW27pULXPvySogU160hvFSiA1bkJSmQMivDun6zmNI0AB9oeR9dJdrzgLssqBxRuMiigU7g
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
