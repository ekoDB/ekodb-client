make test-examples-python 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'8ZnDx-KsO-PWYgwIgKaoWy8L_F6OY8a0RUFFWHFXjod-iTgVjNtj2uWFGRtDDxgXzRyvMY9KculftjdRhzBdLA'}

=== Find by ID === Found: {'name': {'type': 'String', 'value': 'Test Record'},
'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type':
'Boolean'}, 'id':
'8ZnDx-KsO-PWYgwIgKaoWy8L_F6OY8a0RUFFWHFXjod-iTgVjNtj2uWFGRtDDxgXzRyvMY9KculftjdRhzBdLA'}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'value': {'type': 'Integer', 'value': 100},
'name': {'value': 'Updated Record', 'type': 'String'}, 'id':
'8ZnDx-KsO-PWYgwIgKaoWy8L_F6OY8a0RUFFWHFXjod-iTgVjNtj2uWFGRtDDxgXzRyvMY9KculftjdRhzBdLA',
'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
3w36KS3micZ5Sx6JR842Xfz3tMv0TGF8mEiZuZQug0PjIq9F8NIVLPIicjsMLRFBoh5oyoGjV\_-pFNM8Vo2rBw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"3w36KS3micZ5Sx6JR842Xfz3tMv0TGF8mEiZuZQug0PjIq9F8NIVLPIicjsMLRFBoh5oyoGjV_-pFNM8Vo2rBw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"6zgKLocRk8WYRfFz-J1dF0GPcsGdsemFXr7AyajGSwtUPG0vJlbWvqUCsZlMtjaTUx4KY3S3gw_ebP0Jy8A1kQ",
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

=== KV Get === Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
gDW6t3xDFQrKt4FfZ1aTvwIkhN7ozGrVtyl5WEQq-u_apzmM0R4tNUkapDBn-WwUmcSJfxUxMhBM7tb4SpwQ2w

=== List Collections === Total collections: 10 Sample collections:
['functions__ek0_bench', 'websocket_test', 'chat_configurations__ek0_bench',
'users', 'test_collection']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
IzNoC2nGghI6MDpw2lr2mIyFYVRphLt3FHpqxX5lTAeFhfbNCz1911XqgnasCanwe1Lm5R1rBphsGR2POvX_8w

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
WuF1pblBV917OTzOD0gpSqYrBpX8rBI7BN8kHVsikIr_demWC5RxxFf8_ZCwnNOBN8tkl8msLHIOWMpo-nABWw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [34m
╔════════════════════════════════════════╗[0m [34m║ ekoDB Python Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [34m === Getting
Authentication Token ===[0m [32m✓ Authentication successful[0m [33m === Running
7 Examples ===[0m [34m === Running simple_crud.py ===[0m [32m✓ simple_crud.py
completed successfully[0m [34m === Running simple_websocket.py ===[0m [32m✓
simple_websocket.py completed successfully[0m [34m === Running
batch_operations.py ===[0m [32m✓ batch_operations.py completed successfully[0m
[34m === Running kv_operations.py ===[0m [32m✓ kv_operations.py completed
successfully[0m [34m === Running collection_management.py ===[0m [32m✓
collection_management.py completed successfully[0m [34m === Running
document_ttl.py ===[0m [32m✓ document_ttl.py completed successfully[0m [34m ===
Running websocket_ttl.py ===[0m [32m✓ websocket_ttl.py completed successfully[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ Test Summary ║[0m
[34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m [32mPassed:
7[0m [32mFailed: 0[0m ✅ [32mPython direct examples complete![0m 🐍 [36mBuilding
Python client package...[0m 🍹 Building a mixed python/rust project 🔗 Found
pyo3 bindings with abi3 support 🐍 Not using a specific python interpreter 💻
Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
Compiling ekodb_client v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb_client) Compiling
ekodb-client-py v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py) Finished `release`
profile [optimized] target(s) in 10.76s 📦 Built wheel for abi3 Python ≥ 3.8 to
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
'mrFw0bquztGAo65nqe9fUNaaFPwPYNrP1nXP8V_sZhjTNNmNqhSgXWRhy39HC60DEnpHTCx6L95rU2M9ZIiKgQ'}

=== Find by ID === Found: {'id':
'mrFw0bquztGAo65nqe9fUNaaFPwPYNrP1nXP8V_sZhjTNNmNqhSgXWRhy39HC60DEnpHTCx6L95rU2M9ZIiKgQ',
'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record',
'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'type': 'Boolean', 'value': True},
'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'type':
'Integer', 'value': 100}, 'id':
'mrFw0bquztGAo65nqe9fUNaaFPwPYNrP1nXP8V_sZhjTNNmNqhSgXWRhy39HC60DEnpHTCx6L95rU2M9ZIiKgQ'}

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
"f1knd7NMYsGKKcfjjFGT_Vcn_zDWmYCslR5Vnh05nlQuGpSa_Ye1-jkzPCvg2-\_qkT2FajC_yvmWIJQ90ySN8A"

=== List Collections === Total collections: 10 Sample collections:
['functions__ek0_bench', 'websocket_test', 'chat_configurations__ek0_bench',
'users', 'test_collection']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
-0uadrn_Cphvq6_Tcm_v8f1HfVPr0VwaqNMD-ive02RLh94M-f1bGAb4UpuNl16dSSwppmI8d-9Woh0QFxapXw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
4ep9d9NK1JvIchU4khaFbFMKQZwuYyai0LDG-oXQxd4VbTVRWzzaThX1zQluI9UHVVMJhLF5vyZwJfe1H7YGuw

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
Ne-kt8KSXEwOaw-TbKeTvP9XU9taDMgsYmZLmfA0k-aVAAaRs6oEwbrcgdLBD7ptWH1ymEvCJY4MKmc5sa2YLw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Utga3rM8lMBZLxX52bFZoymlLs_z-08jQ0o87zNCBpAqjcB9qkBrIT2xC406peUIf-dWcY_LLo6Dr6vJIcY6qg

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
1. Score: 25.740, Matched: email.value, name, name.value, email
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, title.value, title, bio
1. Score: 26.400, Matched: bio, title.value, title, bio.value
1. Score: 26.400, Matched: bio.value, title.value, bio, title
1. Score: 26.400, Matched: title, title.value, bio, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title.value, bio.value, bio, title
1. Score: 39.600, Matched: bio, title, bio.value, title.value
1. Score: 39.600, Matched: bio, title.value, title, bio.value
1. Score: 39.600, Matched: title, bio.value, bio, title.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 0 semantically similar documents

1. Hybrid search (text + vector): Found 2 results using hybrid search (text +
   vector)
1. Score: 1.200, Matched: title.value, content, content.value, title
1. Score: 0.600, Matched: title, title.value, content.value, content

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
f-eT1zQ7ClDPwYDBZ992i0813Uo2z174-H8iquiQ8whjYhSChArtxUjmeRrWF8OvsZ378-nfgD74zHcId6rURA

=== Sending Chat Message === Message ID:
lD3TB9y5CpY1ujupQjzRPoEPldO-9xe2Wof6SwcSGdberSEt3FF0US9cfSBQ_U-9Z4MvmK8J6KmxMsBz_SN9qA

=== AI Response === There are three products available:

1. "ekoDB Cloud" is a fully managed cloud database service product, priced at
   $499.
2. "ekoDB Pro" is an enterprise edition product with advanced features, priced
   at $299.
3. "ekoDB" is a high-performance database product with AI capabilities, priced
   at $99.

Execution Time: 2067ms

=== Token Usage === Prompt tokens: 378 Completion tokens: 69 Total tokens: 447

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is ekoDB, a high-performance database product. It
is priced at $99.

✓ Message 2 sent Response: As the assistant, I'm sorry, but the context provided
does not include information about a specific product, its details, or its
price. Could you please provide more details or specify the product you are
interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
69YSQpc9vu7j9QPQfYTJE-ELGE4ZykbzuChBtvXR2GQiMhD2sCImhTBR0CV0KuGxBf3xT1UV9FonJh3UBlKH-g
Parent:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew

=== Listing Sessions === ✓ Found 6 sessions Session 1:
69YSQpc9vu7j9QPQfYTJE-ELGE4ZykbzuChBtvXR2GQiMhD2sCImhTBR0CV0KuGxBf3xT1UV9FonJh3UBlKH-g
(Untitled) Session 2:
21OnPz3iUFCAewjLBksLm7apObROKYm2hbandIHd4apRXOgAqWHWU4LltkLp14mGsGQ6y_YKWiy9CVPC6LCNew
(Untitled) Session 3:
f-eT1zQ7ClDPwYDBZ992i0813Uo2z174-H8iquiQ8whjYhSChArtxUjmeRrWF8OvsZ378-nfgD74zHcId6rURA
(Untitled) Session 4:
6YIRrX0Nw2WBJcMCmpa--QKGb8NaTSFlXRB270pQqNBrlt02ojr7tXiJzdLdk7ZEnlBZo8f2JPHTeDJF5GkUnw
(Untitled) Session 5:
IMktZywTlmF0uIb-kYLw2l5LJ8xdJAS4dx4xY-xhsemWgut5OpK7B58NbOzVhALjnXVTKhRhBdBFn8P3nJkgUw
(Untitled) Session 6:
loG91Ays_vwuIn8t-PAJR1kUYrlHQrvC6otIhAo144cYL7njJImlr4otR0lH0zbYX88T-9HBdOARxHSBjujdyQ
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
69YSQpc9vu7j9QPQfYTJE-ELGE4ZykbzuChBtvXR2GQiMhD2sCImhTBR0CV0KuGxBf3xT1UV9FonJh3UBlKH-g

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
KSvmdtn5PuNPM0TaUxo8GXliG02J_fMCVSEjH6w77zCB1hfHncpKvGbMaJzTfQ6zVSONIRTik20VV5OHuVe_iA

=== Sending Initial Message === ✓ Message sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['id', 'content',
'role', 'chat_id', 'updated_at', 'token_usage', 'created_at']) Debug: First
message role: {'value': 'assistant', 'type': 'String'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: I'm sorry, but the context
provided does not include information about the price of ekoDB. You may need to
check the official website or contact the sellers directly for the most accurate
and current pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
lAHOXEH0Vn_UaU7jgvyRgTYJAwZEfq-86R88IureusiszLg6fj3OIgdsy6hU1Z29liPpbFoRXma4WbfCp7IkDg
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! 🚀 ekoDB Saved Functions
Example (Python)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
sb-J3xcIkp558vZb6qcuViuyUBn-8os2Rd4hlf2RSdDQ2zmSulQuEInAkOnBtNJwS8t9rEVpSYmyfDVozj5r4w
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved:
z41iL79nO0ju0AHh1EudWU8bHXqO736fcdfpsjvbd372yBh7vlBwroI3my6Bw_Nl-I-\_yb-3C1JvnhZEKj-jZw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
qWOOdRpdFZ8ahC_62yJojaHyj7zjOAMcuZyg_UWjNyK7cZ0M2PoINeCrOAUHcFw0tQUsuMZ55Ji4ngw2ThXUGQ
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 10, 'status': 'active'}

{'avg_score': 50.0, 'count': 10, 'status': 'inactive'}

📝 Example 4: Function Management

📋 Total functions: 5 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✅ [32mPython client examples complete![0m
