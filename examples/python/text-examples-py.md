make test-examples-python 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'qqyKpugYrSfk3bCQIC6NoDmbEMPxZfUJGdg26esnHx8Iud1cGIwxsvFZEXTHXz-MMyGiHj5WRIG7jg3J5JeRQg'}

=== Find by ID === Found: {'active': {'value': True, 'type': 'Boolean'},
'value': {'value': 42, 'type': 'Integer'}, 'id':
'qqyKpugYrSfk3bCQIC6NoDmbEMPxZfUJGdg26esnHx8Iud1cGIwxsvFZEXTHXz-MMyGiHj5WRIG7jg3J5JeRQg',
'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'value': True, 'type': 'Boolean'},
'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'value': 100,
'type': 'Integer'}, 'id':
'qqyKpugYrSfk3bCQIC6NoDmbEMPxZfUJGdg26esnHx8Iud1cGIwxsvFZEXTHXz-MMyGiHj5WRIG7jg3J5JeRQg'}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
bk4bCkONct5iiO7cGmRUEYih--5u3TSYBQBvnBf7oLbPFW8O9KekSQ_V6NNbM5JjduSNOA-TQs5W87l0V4qeMg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"bk4bCkONct5iiO7cGmRUEYih--5u3TSYBQBvnBf7oLbPFW8O9KekSQ_V6NNbM5JjduSNOA-TQs5W87l0V4qeMg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"ygX4zwfZD59Pc7Wlu1yNBjgmQZtkOpiXEpwp9VZMKq1H-3SZoGHnVaSVZv5jcMnvUabr6bpopfbc-Um7Ps9GQg",
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
Zi_xSFA8ZxQyYDtsZrCkXl6htx4ZtY_YgF0tVnb1VNepI46dcDG0oxAfqIlgJGuzI0ugFLPm4g6m3Rza6Zz0Eg

=== List Collections === Total collections: 10 Sample collections:
['websocket_test', 'chat_configurations__ek0_bench', 'ttl_cache', 'batch_users',
'chat_messages__ek0_bench']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
iJpkkGe3PEy76gWoeQzqMm1THXbdodnWaGnsCDw1dJ7phmwluag8IySsgu0bdq8WpIC0yWduCC-Pcv5tNcT58A

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
SxXkJJLQz40KBpxDmigz20S-uLloPxwmJH_NhYq1qFkLWlHYOl7HOT13UUEXyp_rBIcr2FK4kUtbATDGwYzNdg

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
Compiling proc-macro2 v1.0.103 Compiling quote v1.0.41 Compiling unicode-ident
v1.0.22 Compiling libc v0.2.177 Compiling cfg-if v1.0.4 Compiling smallvec
v1.15.1 Compiling pin-project-lite v0.2.16 Compiling bytes v1.10.1 Compiling
parking_lot_core v0.9.12 Compiling find-msvc-tools v0.1.4 Compiling shlex v1.3.0
Compiling futures-core v0.3.31 Compiling memchr v2.7.6 Compiling
stable_deref_trait v1.2.1 Compiling scopeguard v1.2.0 Compiling cc v1.2.44
Compiling futures-sink v0.3.31 Compiling zeroize v1.8.2 Compiling lock_api
v0.4.14 Compiling serde_core v1.0.228 Compiling target-lexicon v0.12.16
Compiling rustls-pki-types v1.13.0 Compiling itoa v1.0.15 Compiling autocfg
v1.5.0 Compiling futures-channel v0.3.31 Compiling pin-utils v0.1.0 Compiling
serde v1.0.228 Compiling untrusted v0.9.0 Compiling writeable v0.6.2 Compiling
fnv v1.0.7 Compiling litemap v0.8.1 Compiling futures-io v0.3.31 Compiling
futures-task v0.3.31 Compiling slab v0.4.11 Compiling http v1.3.1 Compiling
version_check v0.9.5 Compiling once_cell v1.21.3 Compiling icu_properties_data
v2.1.1 Compiling typenum v1.19.0 Compiling ring v0.17.14 Compiling
icu_normalizer_data v2.1.1 Compiling syn v2.0.108 Compiling getrandom v0.2.16
Compiling parking_lot v0.12.5 Compiling signal-hook-registry v1.4.6 Compiling
socket2 v0.6.1 Compiling mio v1.1.0 Compiling pyo3-build-config v0.20.3
Compiling generic-array v0.14.9 Compiling subtle v2.6.1 Compiling httparse
v1.10.1 Compiling percent-encoding v2.3.2 Compiling zerocopy v0.8.27 Compiling
http-body v1.0.1 Compiling num-traits v0.2.19 Compiling crc32fast v1.5.0
Compiling form_urlencoded v1.2.2 Compiling rustls v0.22.4 Compiling
tower-service v0.3.3 Compiling rustls v0.23.35 Compiling try-lock v0.2.5
Compiling utf8_iter v1.0.4 Compiling log v0.4.28 Compiling simd-adler32 v0.3.7
Compiling adler2 v2.0.1 Compiling crypto-common v0.1.6 Compiling miniz_oxide
v0.8.9 Compiling block-buffer v0.10.4 Compiling want v0.3.1 Compiling pyo3-ffi
v0.20.3 Compiling tracing-core v0.1.34 Compiling rand_core v0.6.4 Compiling
synstructure v0.13.2 Compiling webpki-roots v1.0.4 Compiling atomic-waker v1.1.2
Compiling paste v1.0.15 Compiling thiserror v1.0.69 Compiling byteorder v1.5.0
Compiling flate2 v1.1.5 Compiling tracing v0.1.41 Compiling digest v0.10.7
Compiling cpufeatures v0.2.17 Compiling memoffset v0.9.1 Compiling sync_wrapper
v1.0.2 Compiling ryu v1.0.20 Compiling compression-core v0.4.29 Compiling base64
v0.22.1 Compiling getrandom v0.3.4 Compiling portable-atomic v1.11.1 Compiling
ipnet v2.11.0 Compiling ppv-lite86 v0.2.21 Compiling serde_json v1.0.145
Compiling heck v0.4.1 Compiling tower-layer v0.3.3 Compiling rand_chacha v0.3.1
Compiling pyo3-macros-backend v0.20.3 Compiling compression-codecs v0.4.31
Compiling zerofrom-derive v0.1.6 Compiling yoke-derive v0.8.1 Compiling
zerovec-derive v0.11.2 Compiling tokio-macros v2.6.0 Compiling displaydoc v0.2.5
Compiling futures-macro v0.3.31 Compiling serde_derive v1.0.228 Compiling
thiserror-impl v1.0.69 Compiling tokio v1.48.0 Compiling rand v0.8.5 Compiling
zerofrom v0.1.6 Compiling sha1 v0.10.6 Compiling futures-util v0.3.31 Compiling
pyo3 v0.20.3 Compiling yoke v0.8.1 Compiling aho-corasick v1.1.4 Compiling utf-8
v0.7.6 Compiling regex-syntax v0.8.8 Compiling zerovec v0.11.5 Compiling
zerotrie v0.2.3 Compiling rustls-webpki v0.102.8 Compiling rustls-webpki
v0.103.8 Compiling tinystr v0.8.2 Compiling icu_locale_core v2.1.1 Compiling
potential_utf v0.1.4 Compiling icu_collections v2.1.1 Compiling icu_provider
v2.1.1 Compiling data-encoding v2.9.0 Compiling icu_normalizer v2.1.1 Compiling
icu_properties v2.1.1 Compiling hyper v1.7.0 Compiling idna_adapter v1.2.1
Compiling idna v1.1.0 Compiling hyper-util v0.1.17 Compiling tower v0.5.2
Compiling url v2.5.7 Compiling rust_decimal v1.39.0 Compiling tokio-rustls
v0.26.4 Compiling bitflags v2.10.0 Compiling iri-string v0.7.9 Compiling
core-foundation-sys v0.8.7 Compiling iana-time-zone v0.1.64 Compiling
hyper-rustls v0.27.7 Compiling tungstenite v0.21.0 Compiling tower-http v0.6.6
Compiling tokio-rustls v0.25.0 Compiling async-compression v0.4.32 Compiling
tokio-util v0.7.17 Compiling serde_urlencoded v0.7.1 Compiling futures-executor
v0.3.31 Compiling regex-automata v0.4.13 Compiling pyo3-macros v0.20.3 Compiling
rmp v0.8.14 Compiling webpki-roots v0.26.11 Compiling http-body-util v0.1.3
Compiling indoc v2.0.7 Compiling arrayvec v0.7.6 Compiling unindent v0.2.4
Compiling rmp-serde v1.3.0 Compiling reqwest v0.12.24 Compiling
tokio-tungstenite v0.21.0 Compiling uuid v1.18.1 Compiling regex v1.12.2
Compiling futures v0.3.31 Compiling chrono v0.4.42 Compiling pyo3-asyncio
v0.20.0 Compiling ekodb_client v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb_client) Compiling
ekodb-client-py v0.3.0
(/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py) Finished `release`
profile [optimized] target(s) in 28.13s 📦 Built wheel for abi3 Python ≥ 3.8 to
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
'LV9aFiwor4rEeXuRyhHSXF2a7LgbUW5Xu5e8uZp16xd-tr1tZwVmNhuO9w_d4rL33cNOItmxzvyNc5-HS0bYYw'}

=== Find by ID === Found: {'id':
'LV9aFiwor4rEeXuRyhHSXF2a7LgbUW5Xu5e8uZp16xd-tr1tZwVmNhuO9w_d4rL33cNOItmxzvyNc5-HS0bYYw',
'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value':
'Test Record'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'id':
'LV9aFiwor4rEeXuRyhHSXF2a7LgbUW5Xu5e8uZp16xd-tr1tZwVmNhuO9w_d4rL33cNOItmxzvyNc5-HS0bYYw',
'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'value': 100,
'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}

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
"m2JylB87mvU-PV37HiPc9bndNqptjKU_tfzFh_LEL00e7QEE8BDbyOonC1QITMOVOpj2dEvg7oZg0mXqw7knrw"

=== List Collections === Total collections: 10 Sample collections:
['websocket_test', 'chat_configurations__ek0_bench', 'ttl_cache',
'client_collection_management_python', 'batch_users']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
pSwS6dfXq9PLNZfnxgeQwd4ZlWi66eEQQZDaVLxktzV2f7ZaIe6TygSxHifejSG4O4sNbLtIpTfAH5dmM37d6w

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
xB6sGkKQKhMhyZDl1nHWyoePrlyiRjqBH6qubLnc-Oaut-ulOvw10UM_0Um8zkJiru-aYVz2POW9R_kDwFcgUA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
c1ZdPS009dNF4qquqMYerzFc3ZWM_TTZmzZ9EnqLvyHJy564_7q79C3l3KmnkkyYltsCNeEDcAtD3XTYj4_6ig

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
-OOLjNhwmmWP4OZta24z73fiDSw8LHi95GOyyfBlxzY01WmVQBlUWcHCE8v_ZjqmeO01WDy0jTtTR1fSPLDfEw

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
1. Score: 25.740, Matched: name, name.value, email, email.value
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio.value, bio, title.value
1. Score: 26.400, Matched: title.value, bio, bio.value, title
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: title.value, bio.value, title, bio

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title, bio, bio.value, title.value
1. Score: 39.600, Matched: bio.value, title, bio, title.value
1. Score: 39.600, Matched: bio, title.value, bio.value, title
1. Score: 39.600, Matched: title.value, bio.value, title, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.782
1. Score: 0.769
1. Score: 0.727

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.738, Matched: title, title.value, content.value, content
1. Score: 1.109, Matched: title.value, content, content.value, title
1. Score: 0.547, Matched:

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
bEGYtiLgEbT1EbPG8_0FOTynTXtkYIvcZGBaPcKgHVDQ8pSe4ATrA0QDir6A-85pvJpCR4dZe5GYNJPqqZY12g

=== Sending Chat Message === Message ID:
5jtTeoE94Jz3tr71GzohKx1dDln9s0YY3gf1CUs_ob4CJ3WYxXNonnM2DvTUeiWWvCsWReof4kKzyeIVGhLwHw

=== AI Response === The available products and their respective prices are as
follows:

1. ekoDB Cloud - This is a fully managed cloud database service product, priced
   at $499.
2. ekoDB Pro - This is an enterprise edition product with advanced features,
   priced at $299.
3. ekoDB - This is a high-performance database product with AI capabilities,
   priced at $99.

Execution Time: 4443ms

=== Token Usage === Prompt tokens: 434 Completion tokens: 77 Total tokens: 511

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
nhwhhgOvM3Cs7T7pPSwLlWkqiCW86qVdGG2kHPlhvy_kUxnHKlipsiOXJ_S4NiIuccj1fcrME6jXHzyY1I9Aew

=== Sending Messages === ✓ Message 1 sent Response: Answer: The available
product is a high-performance database product named ekoDB. It is priced at $99.

✓ Message 2 sent Response: I'm sorry, but there's no information provided about
the product's price in the context above. Could you please specify which product
you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
cyqwVmB5ZTE4I_wACb1qRJqzzeacVjC4038hv9_YfpXMm0_TgA_YoTvFdfYu5tCu9JyZmkjkIOxkkj9L-OQq6w
Parent:
nhwhhgOvM3Cs7T7pPSwLlWkqiCW86qVdGG2kHPlhvy_kUxnHKlipsiOXJ_S4NiIuccj1fcrME6jXHzyY1I9Aew

=== Listing Sessions === ✓ Found 6 sessions Session 1:
cyqwVmB5ZTE4I_wACb1qRJqzzeacVjC4038hv9_YfpXMm0_TgA_YoTvFdfYu5tCu9JyZmkjkIOxkkj9L-OQq6w
(Untitled) Session 2:
nhwhhgOvM3Cs7T7pPSwLlWkqiCW86qVdGG2kHPlhvy_kUxnHKlipsiOXJ_S4NiIuccj1fcrME6jXHzyY1I9Aew
(Untitled) Session 3:
bEGYtiLgEbT1EbPG8_0FOTynTXtkYIvcZGBaPcKgHVDQ8pSe4ATrA0QDir6A-85pvJpCR4dZe5GYNJPqqZY12g
(Untitled) Session 4:
gCQ0KOMfyF62QuoyABvyWlQg0eTLqa-0V8wAplTvIRrRNnlIywMtHlRzS4i-GM4rSk61UZXenP7tTndJnc_Znw
(Untitled) Session 5:
qG-MclQB4tcPo_IDrpnwKXSccRvWgHAUzM6rY63jpPZp2LRHxq5iaq4Kgn-gNcd_sQydUflDkM8gGmnoenCzhw
(Untitled) Session 6:
6vuPox7U6AzsVb7dniAaHDwJXJG7M4Vq9wwTyOecbl5pUvuf6mZ-Rk5P4p-2VHYZLB9ynnGPi8Cl0-bU_vq3NQ
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
cyqwVmB5ZTE4I_wACb1qRJqzzeacVjC4038hv9_YfpXMm0_TgA_YoTvFdfYu5tCu9JyZmkjkIOxkkj9L-OQq6w

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
23l5Fepook9qcgnU5kTCt1eDgdUKwgq1VZIXyy_BNzGNYfA4dOl3F4HnntTku7TwnLva0PjIZsemQ-VBsMXlPg

=== Sending Initial Message === ✓ Message sent Response: Answer: The available
product is a high-performance database product named "ekoDB". It is priced
at 99.

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['token_usage',
'updated_at', 'content', 'created_at', 'chat_id', 'id', 'role']) Debug: First
message role: {'type': 'String', 'value': 'assistant'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: Answer: I'm sorry, but the
context provided does not include information about the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
xk1gRyFojsYWBB7kDMWNkw5y4h5JIcVoLN-zoBLR-Ws_UEc2ZU_DROpqO2lL92f4140PY5eFcPvVj8cpfOeyxg
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
93ss5w4FF8cfTLoq8_lcUaiOct5L6yuTMqQ-YIW0iKchyHi4NX8hW67F7TJOXQyWh5KBaY5zrAS6GqAeUT_gXA
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
2i4cGgl0FJyJBEzBegiJp9jtZ7oZV3r8NEOPVSiymnSsLWU5Yg73Kh5-ym_0-bpd4glEfCVD67Fcu-N9ZeVdDA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
\_vQdqdcDtiJ77PAWiiYIo4NrjPofLh8s4yp-B48fgPJvuPEqoUYIFj6BbOCxnBnSFVSZK-VYohyes7BDorOW7w
📊 Statistics: 2 groups {'avg_score': 50.0, 'count': 10, 'status': 'inactive'}

{'avg_score': 60.0, 'count': 10, 'status': 'active'}

📝 Example 4: Script Management

📋 Total scripts: 5 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
DYBaP8By8CdidJwTVC7na8HllQCRHJHuaLgApeZ5LeKrAZJHZTxmEYM9bwRzQs14isQ4A1wz_O-ckCpUWztPFw
📊 Found 2 product groups {'avg_price': 474.0, 'category': 'Furniture', 'count':
2} {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3} ⏱️
Execution time: 0ms

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
