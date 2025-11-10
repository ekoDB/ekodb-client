make test-examples-python 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'zT8v5xzyqPEJBqaCpao4u37UTs1Y4ENEswI238pmsZivhUtCm9HnFRzYA1BWmNd-3prKmSfRLuVrwNfIYqgEhw'}

=== Find by ID === Found: {'value': {'type': 'Integer', 'value': 42}, 'active':
{'value': True, 'type': 'Boolean'}, 'name': {'value': 'Test Record', 'type':
'String'}, 'id':
'zT8v5xzyqPEJBqaCpao4u37UTs1Y4ENEswI238pmsZivhUtCm9HnFRzYA1BWmNd-3prKmSfRLuVrwNfIYqgEhw'}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'name': {'value': 'Updated Record', 'type':
'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'id':
'zT8v5xzyqPEJBqaCpao4u37UTs1Y4ENEswI238pmsZivhUtCm9HnFRzYA1BWmNd-3prKmSfRLuVrwNfIYqgEhw',
'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
PhhdxEYbQszJEMgXIonEwzmyOwVtvGc1Yocd1cTDxLWhJrpoLdEWJOyKQ864_IEWJUlyIlVlbZH6TB-II9J5bQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"aRE3XVwlbTj5n7jpL8YazH5Sle1ShUD8o1xXCtJz3n3OMfAvG6SWCG1CUYNC0BVcjv19UXFhDXmpbPhqQ2iYgA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"PhhdxEYbQszJEMgXIonEwzmyOwVtvGc1Yocd1cTDxLWhJrpoLdEWJOyKQ864_IEWJUlyIlVlbZH6TB-II9J5bQ",
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

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
rdzlEhcDkXNzF97wnbKkEI4B3vG5hh8cc*NedXO*-p12DzBHA8MjA1f0JnPK1XI9P2jJ_QLYaZ_b1UYahhjQhA

=== List Collections === Total collections: 10 Sample collections:
['ws_ttl_test', 'batch_users', 'test_collection', 'chat_messages__ek0_bench',
'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
ULyxLR-y2KBjhZy0oiBgEKit42qQoMiNwX9tAHVApPLUw_36R6pX3fQZe68hqYWyITjTS5QJPwl7pe18ptRNcQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
TlpRywEVJnBddQrJpYG8FaYA-YYrlXyQl72b_ERunbjxBLB5yhTKiXs0eul3PiiFOfm42Bp2z4CVn6itBkM9yQ

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
Finished `release` profile [optimized] target(s) in 0.15s 📦 Built wheel for
abi3 Python ≥ 3.8 to
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
'Hk93chYxPBAlSk6FPX8TWs9zgUcfqpUWzQh9KA9H6_dlmM9oFeg34qLdD0PWeOww00YHm7eTYsBcu7V_WA0QJw'}

=== Find by ID === Found: {'value': {'value': 42, 'type': 'Integer'}, 'name':
{'type': 'String', 'value': 'Test Record'}, 'id':
'Hk93chYxPBAlSk6FPX8TWs9zgUcfqpUWzQh9KA9H6_dlmM9oFeg34qLdD0PWeOww00YHm7eTYsBcu7V_WA0QJw',
'active': {'type': 'Boolean', 'value': True}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'value': True, 'type': 'Boolean'},
'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record',
'type': 'String'}, 'id':
'Hk93chYxPBAlSk6FPX8TWs9zgUcfqpUWzQh9KA9H6_dlmM9oFeg34qLdD0PWeOww00YHm7eTYsBcu7V_WA0QJw'}

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
"AGYBzxThU_p_TWg3ZE3GSPPWIUtAQ8ws8RwGV4Fh9XMRN3KiaF75v6WkpQ43CGe-iCIhTUvTLqo7auU25ROgOw"

=== List Collections === Total collections: 10 Sample collections:
['ws_ttl_test', 'batch_users', 'test_collection', 'chat_messages__ek0_bench',
'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
oVbjT-AHPNNvtjh7WxQcGEEWGS2dIkETNwHRfC3XdzLjJoRTqBwJBMR24GJtBTbqqzkbaSXhw8zWpHucYRKaJA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
8ltuVMIAKJZ8NTsZhdiaH15iBB10NrpKnZsoIic-27qQkp8XWAYwNzl7Ho-p9hvnCF3LFOMFy5XTdlrDhiwKwg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
NPtMovMMrWKB7mzaulDV-EB0Sjtp9Hyl4OmAmjX33BeRzN9OgGYfQC0uORNqBRe0I_uwxKJa1WNB6RQ-M1cjfA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
OulqWOsU3C-6YkyaYWX026qLx6nZLBGcy5pICRvWFv-\_h3mPUNmH2aCEc7GME9vUkoq2I98TY_8CRUMJgq1hyw

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
1. Score: 25.740, Matched: name.value, email, name, email.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: title, bio.value, bio, title.value
1. Score: 26.400, Matched: bio.value, bio, title.value, title

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: title, title.value, bio.value, bio
1. Score: 39.600, Matched: title.value, title, bio, bio.value
1. Score: 39.600, Matched: bio, bio.value, title.value, title

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 0 semantically similar documents

1. Hybrid search (text + vector): Found 2 results using hybrid search (text +
   vector)
1. Score: 1.200, Matched: content, title, title.value, content.value
1. Score: 0.600, Matched: title.value, title, content.value, content

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

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
06JVJeje00VfyFekc9GCgSS8QpyxR8sK66prVhP6gAv2K7ckAfjRtW-bACzxAnBwbgqh6MK9wxo7TFbN7sHeMw

=== Sending Chat Message === Message ID:
1MQbeyZkxuyX5OgPwc_wqhIKpZviBAammVlIgSMuIVmX1f9l5UDb_K71MTpl2BDepG_uookCzkhqvdWsYTxKAg

=== AI Response === There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is
   priced at $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. It
   is priced at $299.

3. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.

Execution Time: 3241ms

=== Token Usage === Prompt tokens: 369 Completion tokens: 78 Total tokens: 447

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
sfm7SuCU_7I60z0VFUIdufcM4PTTusaCwHVTUjlfO03ayFDkrlNa0-3aQx99iAqXzQotv1ywUpfN073Gq_6RvA

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Message 2 sent Response: As an assistant, I'm sorry, but there's no context
provided above regarding a specific product. Could you please provide more
information or specify the product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
NeTq7n0Y-Lqk3Eb-eWuffPPciZ8AdXGSNPbe0HbEqj7hy1D-PM_kpFyW3bIEtZr6IzmT21yIcvpNX0lORLHofg
Parent:
sfm7SuCU_7I60z0VFUIdufcM4PTTusaCwHVTUjlfO03ayFDkrlNa0-3aQx99iAqXzQotv1ywUpfN073Gq_6RvA

=== Listing Sessions === ✓ Found 6 sessions Session 1:
NeTq7n0Y-Lqk3Eb-eWuffPPciZ8AdXGSNPbe0HbEqj7hy1D-PM_kpFyW3bIEtZr6IzmT21yIcvpNX0lORLHofg
(Untitled) Session 2:
sfm7SuCU_7I60z0VFUIdufcM4PTTusaCwHVTUjlfO03ayFDkrlNa0-3aQx99iAqXzQotv1ywUpfN073Gq_6RvA
(Untitled) Session 3:
06JVJeje00VfyFekc9GCgSS8QpyxR8sK66prVhP6gAv2K7ckAfjRtW-bACzxAnBwbgqh6MK9wxo7TFbN7sHeMw
(Untitled) Session 4:
SCAx_XbQF_b-0GA8DV0bvD7IiWmVtRNnI8iySeWi8uy956NxdqKfiXXD9TjJ40k2E0xG4N60M_gjtLGOZHDN3Q
(Untitled) Session 5:
mwJq0JqojQdSve6Yo19pdIZeOdkM4CH0MRXJ04YUrrjq0XCKgvyYyXWU598PsuBif8g5mBc5HttM7WOBlMxywA
(Untitled) Session 6:
5UAdpVm_csoCxOJQxiqdq5hVSVqlgdZq5q3paht_qgWqQJ6DX7lDB_nEP0ehvl1DBU5MEeStrS_FL7mGVCCI_w
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
NeTq7n0Y-Lqk3Eb-eWuffPPciZ8AdXGSNPbe0HbEqj7hy1D-PM_kpFyW3bIEtZr6IzmT21yIcvpNX0lORLHofg

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
8X7U3Wq7XO40iFIvLATr5G6JuLPsDM18cjKI8vzpslW3BSTk-uTFyc9uJJmo8ToaMa7qeM7BPAwzzQBFns27Kw

=== Sending Initial Message === ✓ Message sent Response: Based on the context
provided, the available product is a high-performance database product named
ekoDB. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['content',
'chat_id', 'id', 'token_usage', 'updated_at', 'role', 'created_at']) Debug:
First message role: {'value': 'assistant', 'type': 'String'} === Feature 1:
Regenerate AI Response === ✓ Message regenerated New response: I'm sorry, but
the context provided does not include information about the price of ekoDB. I
recommend checking the product's official website or contacting the seller for
the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
JTaC0beEkTbQdlCZePnWpSgtzxgkeSFO6MBawhoKUreXdifOnFbcRmCZ7m-9shlCBFXP2NkUL1SMmviUhEGXKA
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
KqZE8baWe3sXES47JbDn7fBbQHj3CUm7U7wIWu_BZYTn-\_jmXu0rNatsElppto2CKOibRr-y_octMd0GHWVfGQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
pqv-QCbvTr-AOJhNCKF8HjS3DTMCNigfl12K59fFdHxYMe3FCzfuQQYTSmuJafa_cT5KzXKlHBSThJzdtWidEw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
fxN8APrp5_oao3fTHcC_tUJe6pwq7v5N1FRQo_OmPosdGwpdbJATCzoHv2xcUVrbnCcD5jHWSuiOF819a_xB5w
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
ZUCRvZp4hpgRsCrEpLSCmR3z1YgXsIQQ4ZeOLnSg2obHiG_7Bki-9xSCIffOC1-1DckZUVGEsVeSK8KwWMndBg
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
