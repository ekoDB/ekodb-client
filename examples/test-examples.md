make test-examples 🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("KiO57o5edaXc6u956l3dmhuHXM3lO5Yhhiez7jGQAVg6Pkzv5KlMf16SEQBkWDXwKz53SLXhJTa4yrJXPq5Kkw")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("KiO57o5edaXc6u956l3dmhuHXM3lO5Yhhiez7jGQAVg6Pkzv5KlMf16SEQBkWDXwKz53SLXhJTa4yrJXPq5Kkw"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("KiO57o5edaXc6u956l3dmhuHXM3lO5Yhhiez7jGQAVg6Pkzv5KlMf16SEQBkWDXwKz53SLXhJTa4yrJXPq5Kkw"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("KiO57o5edaXc6u956l3dmhuHXM3lO5Yhhiez7jGQAVg6Pkzv5KlMf16SEQBkWDXwKz53SLXhJTa4yrJXPq5Kkw"),
"name": Object {"type": String("String"), "value": String("Updated Record")},
"value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 1.07s Running
`target/debug/examples/simple_websocket` ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"ygX4zwfZD59Pc7Wlu1yNBjgmQZtkOpiXEpwp9VZMKq1H-3SZoGHnVaSVZv5jcMnvUabr6bpopfbc-Um7Ps9GQg"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ygX4zwfZD59Pc7Wlu1yNBjgmQZtkOpiXEpwp9VZMKq1H-3SZoGHnVaSVZv5jcMnvUabr6bpopfbc-Um7Ps9GQg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.52s Running
`target/debug/examples/batch_operations` ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.50s Running
`target/debug/examples/kv_operations` ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: Object {"userId": Number(123), "username":
String("john_doe")}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: Object {"name": String("Product 1"),
"price": Number(29.99)} cache:product:2: Object {"name": String("Product 2"),
"price": Number(39.989999999999995)} cache:product:3: Object {"name":
String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.70s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"ULhmOeMGfhiduBFS-VHNGxEyDk5VcR0DhinHewZwJdPN5islyr2uqi_ow12AUb-tcYJPam5Ppe-zlovjmnFY0A"

=== List Collections === Total collections: 4 Sample collections:
["websocket_test", "batch_users", "demo_collection", "test_collection"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"8fly9LjhwotYdfS5HkEygpGkfRT5vgirQo22E4deOFjyEiwtwNYvMfeuKH5H0K_pznwf_lTYsn_NvqDaKic02w"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.70s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("sVvAhusFlRNHqaAaMniaET1XaB9r7OvrKJfj10_8U_PX3OXT4m5BSqh99EX5DxjSqnLAA5dkvGVknjQDu6Z-zQ")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✅ [32mRust direct examples
complete![0m 🛠️ [36mBuilding client library...[0m cargo build -p ekodb_client
Compiling cfg-if v1.0.4 Compiling smallvec v1.15.1 Compiling stable_deref_trait
v1.2.1 Compiling bytes v1.10.1 Compiling pin-project-lite v0.2.16 Compiling libc
v0.2.177 Compiling scopeguard v1.2.0 Compiling syn v2.0.108 Compiling zeroize
v1.8.2 Compiling futures-core v0.3.31 Compiling itoa v1.0.15 Compiling lock_api
v0.4.14 Compiling serde_core v1.0.228 Compiling untrusted v0.9.0 Compiling
rustls-pki-types v1.13.0 Compiling fnv v1.0.7 Compiling writeable v0.6.2
Compiling litemap v0.8.1 Compiling pin-utils v0.1.0 Compiling futures-sink
v0.3.31 Compiling percent-encoding v2.3.2 Compiling subtle v2.6.1 Compiling
icu_properties_data v2.1.1 Compiling icu_normalizer_data v2.1.1 Compiling
typenum v1.19.0 Compiling getrandom v0.2.16 Compiling parking_lot_core v0.9.12
Compiling socket2 v0.6.1 Compiling signal-hook-registry v1.4.6 Compiling mio
v1.1.0 Compiling ring v0.17.14 Compiling http v1.3.1 Compiling once_cell v1.21.3
Compiling parking_lot v0.12.5 Compiling slab v0.4.11 Compiling futures-task
v0.3.31 Compiling memchr v2.7.6 Compiling generic-array v0.14.9 Compiling
httparse v1.10.1 Compiling zerocopy v0.8.27 Compiling form_urlencoded v1.2.2
Compiling http-body v1.0.1 Compiling adler2 v2.0.1 Compiling tower-service
v0.3.3 Compiling simd-adler32 v0.3.7 Compiling log v0.4.28 Compiling utf8_iter
v1.0.4 Compiling try-lock v0.2.5 Compiling crypto-common v0.1.6 Compiling
miniz_oxide v0.8.9 Compiling want v0.3.1 Compiling block-buffer v0.10.4
Compiling num-traits v0.2.19 Compiling crc32fast v1.5.0 Compiling tracing-core
v0.1.34 Compiling rustls-webpki v0.103.8 Compiling rustls-webpki v0.102.8
Compiling rand_core v0.6.4 Compiling webpki-roots v1.0.4 Compiling
futures-channel v0.3.31 Compiling byteorder v1.5.0 Compiling atomic-waker v1.1.2
Compiling rustls v0.23.35 Compiling rustls v0.22.4 Compiling tracing v0.1.41
Compiling flate2 v1.1.5 Compiling digest v0.10.7 Compiling ppv-lite86 v0.2.21
Compiling cpufeatures v0.2.17 Compiling sync_wrapper v1.0.2 Compiling
synstructure v0.13.2 Compiling ryu v1.0.20 Compiling rand_chacha v0.3.1
Compiling compression-core v0.4.29 Compiling base64 v0.22.1 Compiling
tower-layer v0.3.3 Compiling ipnet v2.11.0 Compiling getrandom v0.3.4 Compiling
rand v0.8.5 Compiling compression-codecs v0.4.31 Compiling sha1 v0.10.6
Compiling aho-corasick v1.1.4 Compiling regex-syntax v0.8.8 Compiling utf-8
v0.7.6 Compiling core-foundation-sys v0.8.7 Compiling bitflags v2.10.0 Compiling
iri-string v0.7.9 Compiling data-encoding v2.9.0 Compiling iana-time-zone
v0.1.64 Compiling rmp v0.8.14 Compiling serde_json v1.0.145 Compiling
webpki-roots v0.26.11 Compiling http-body-util v0.1.3 Compiling arrayvec v0.7.6
Compiling zerofrom-derive v0.1.6 Compiling yoke-derive v0.8.1 Compiling
zerovec-derive v0.11.2 Compiling displaydoc v0.2.5 Compiling tokio-macros v2.6.0
Compiling serde_derive v1.0.228 Compiling futures-macro v0.3.31 Compiling
thiserror-impl v1.0.69 Compiling regex-automata v0.4.13 Compiling tokio v1.48.0
Compiling zerofrom v0.1.6 Compiling futures-util v0.3.31 Compiling yoke v0.8.1
Compiling thiserror v1.0.69 Compiling serde v1.0.228 Compiling regex v1.12.2
Compiling zerovec v0.11.5 Compiling zerotrie v0.2.3 Compiling serde_urlencoded
v0.7.1 Compiling uuid v1.18.1 Compiling chrono v0.4.42 Compiling rust_decimal
v1.39.0 Compiling rmp-serde v1.3.0 Compiling tinystr v0.8.2 Compiling
potential_utf v0.1.4 Compiling icu_collections v2.1.1 Compiling icu_locale_core
v2.1.1 Compiling icu_provider v2.1.1 Compiling hyper v1.7.0 Compiling
tokio-rustls v0.26.4 Compiling tower v0.5.2 Compiling async-compression v0.4.32
Compiling tokio-util v0.7.17 Compiling tokio-rustls v0.25.0 Compiling
icu_normalizer v2.1.1 Compiling icu_properties v2.1.1 Compiling tower-http
v0.6.6 Compiling hyper-util v0.1.17 Compiling idna_adapter v1.2.1 Compiling idna
v1.1.0 Compiling url v2.5.7 Compiling hyper-rustls v0.27.7 Compiling tungstenite
v0.21.0 Compiling reqwest v0.12.24 Compiling tokio-tungstenite v0.21.0 Compiling
ekodb_client v0.3.0 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.16s ✅
[32mClient build complete![0m 🧪 [36mRunning Rust client library examples...[0m
Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 1.12s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("xOwZIl6W0UP70u0yp1ajUZ-jPH9wJI-QQH1_caaYl7cl6TM_d0ZaCg1xNu6I71BwodnAMe_5ZxNyfCIyjoMqeg")}
}

=== Find by ID === Found: Record { fields: {"name": Object({"value":
String("Test Record"), "type": String("String")}), "id":
String("xOwZIl6W0UP70u0yp1ajUZ-jPH9wJI-QQH1_caaYl7cl6TM_d0ZaCg1xNu6I71BwodnAMe_5ZxNyfCIyjoMqeg"),
"value": Object({"value": Integer(42), "type": String("Integer")}), "active":
Object({"type": String("Boolean"), "value": Boolean(true)})} }

=== Find with Query === Found documents: [Record { fields: {"name":
Object({"type": String("String"), "value": String("Test Record")}), "active":
Object({"value": Boolean(true), "type": String("Boolean")}), "id":
String("xOwZIl6W0UP70u0yp1ajUZ-jPH9wJI-QQH1_caaYl7cl6TM_d0ZaCg1xNu6I71BwodnAMe_5ZxNyfCIyjoMqeg"),
"value": Object({"type": String("Integer"), "value": Integer(42)})} }]

=== Update Document === Updated: Record { fields: {"id":
String("xOwZIl6W0UP70u0yp1ajUZ-jPH9wJI-QQH1_caaYl7cl6TM_d0ZaCg1xNu6I71BwodnAMe_5ZxNyfCIyjoMqeg"),
"value": Object({"type": String("Integer"), "value": Integer(100)}), "active":
Object({"value": Boolean(true), "type": String("Boolean")}), "name":
Object({"value": String("Updated Record"), "type": String("String")})} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.88s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.63s Running
`target/debug/examples/client_kv_operations` ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: Object {"userId": Number(123), "username":
String("john_doe")}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: Object {"name": String("Product 1"),
"price": Number(29.99)} cache:product:2: Object {"name": String("Product 2"),
"price": Number(39.989999999999995)} cache:product:3: Object {"name":
String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.86s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"ZjdLdN6sFeSeviBUQAeQtfRe36cRnIbC1Idqo6sI3nz6snuUO2PjAGUXi67k6MNVhypuynUaLHfKUvCpr_kJzA"

=== List Collections === Total collections: 6 Sample collections:
["client_collection_management_rust", "websocket_test", "ttl_cache",
"batch_users", "ws_ttl_test"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.88s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"ypzIsJDWaUNjiL82kuGzaNkry18_pOwuVmtnS6fRPwajryq7p2tcTF2juKTwS_IYThnFA3Uy281a3cOrYnLvbg"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("QoFEZUrGvBLVzSDtHGGy9Cex4OyhB_njYBCd0lt1WCGsCiZh56o6uFLMdEFfuFSksTE3Zq6pCLQHTNy14pJHaA"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 1.22s Running
`target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
Uprs5Xq_UU_fUfB9UOR8EIYVAfN8-IH3GL4MCl2FR8jof5Oh9B3Qa3Cohn5po241LXUrik0E92TM1AiiUTFM9A

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 1.21s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("t6ntqgvNbg9a9Z4SzPxmOGPmKgoiFSuH_5pOWJ2U8WvgHn5frQjVDwLvtY810D7bhpKGRySRfsoZ9-6g6eiaeQ"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.91s Running
`target/debug/examples/client_query_builder` === ekoDB Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"type": String("String"), "value": String("Eve")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("David")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"value": String("Eve"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"type": String("String"), "value": String("Bob")}))
2. Some(Object({"value": String("David"), "type": String("String")}))
3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator === ✓ Found 4 users NOT inactive

=== Complex Chained Query === ✓ Found 3 users with all conditions

=== Cleanup === ✓ Deleted collection

✓ All query builder operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.89s Running
`target/debug/examples/client_search` === ekoDB Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Found 3 results for 'programming'

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("Python for Data Science"))
3. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Fuzzy Search === ✓ Found 3 results for 'progamming' (typo)

1. Score: 1.6500 - Some(String("Rust Programming"))
2. Score: 0.5500 - Some(String("Python for Data Science"))
3. Score: 0.5500 - Some(String("JavaScript Web Development"))

=== Field-Specific Search === ✓ Found 4 results in title/description

1. Score: 4.0000 Title: Some(String("Machine Learning Basics")) Matched:
   ["title", "description.value", "title.value", "description"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description.value", "description"]
3. Score: 1.0000 Title: Some(String("Database Design")) Matched:
   ["description.value", "description"]
4. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description", "description.value"]

=== Weighted Search === ✓ Found 2 results with field weights

1. Score: 23.1000 - Some(String("Python for Data Science"))
2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options === ✓ Found 1 results with stemming

1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit === ✓ Limited to 2 results (requested 2)

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup === ✓ Deleted collection

✓ All search operations completed successfully Execution time: 4ms Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("w-tZKkQf7VVD4JI-gr6dKlGrBMBDU1y8K9oUbBRbyHgcf4jO4o19RzQzAdBqY_mDWlcwfYXA31HUq7AIdNhYJA"))
✓ Inserted user 2:
Some(String("\_iDRkYiVUBkCMEkn_hNyn5abwY09lMmVaD9QmXMwnzlVYOo7QE1TWQEYV4eD0ObjLzL1PGmOF28GllIeBasWoA"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- email: String (required)
- age: Integer
- status: String
- title: String (required)

=== Listing Collections === ✓ Total collections: 6 Sample:
["schema_client_rust", "websocket_test", "ttl_cache", "batch_users",
"ws_ttl_test"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s Running
`target/debug/examples/client_joins` === ekoDB Rust Client - Join Operations
Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Bob Smith: Sales
- Alice Johnson: Engineering

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.87s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
6vuPox7U6AzsVb7dniAaHDwJXJG7M4Vq9wwTyOecbl5pUvuf6mZ-Rk5P4p-2VHYZLB9ynnGPi8Cl0-bU_vq3NQ

=== Sending Chat Message === Message ID:
D3BLojPV10nj8sahcAS2dGd2GYcgRe7ia5slETYIQvw\_\_-buIuSQGVXmtDUHDVfOJu4LpGIAXZD1OPISMjPwtQ

=== AI Response === Response 1: ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration. It offers
several unique features. For instance, it supports full-text search, vector
search, and hybrid search with automatic context retrieval. Another exciting
feature is the AI Chat Integration, which allows you to query your database
using natural language and get AI-powered responses with relevant context.

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["category", "title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("4IjUY4xF8K7iDiCOQv6IWohhwCnA7lDBHQ2ijorEA-tLYPoyP00Bn-WCyJAP0BzxNQgJysa2XYR1Ike_6qL_XQ"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title", "category"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("cWWNCByKq6I_o4nJCTcRjcLX0WS2EbqY-hHv439m9UfLJ49s11twLz8sPpzVgIuWdvVIGS-1ox36BK9Bf3dVOw"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("MVF68-uDgdMWcLFdZcKj4B1sMZBiTwwuFl9878Rg7l9Er6oyo_lP0X-0brFSGj45EyjeLa2CdsrEKJFyJ7rPrg"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Execution Time: 3595ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 1.01s Running
`target/debug/examples/client_chat_sessions` === ekoDB Chat Session Management
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
qG-MclQB4tcPo_IDrpnwKXSccRvWgHAUzM6rY63jpPZp2LRHxq5iaq4Kgn-gNcd_sQydUflDkM8gGmnoenCzhw
=== Sending Messages === ✓ Message 1 sent Response: Answer: The product
available is ekoDB, a high-performance database product with AI capabilities. It
is priced at 99 units.

✓ Message 2 sent Response: Apologies, but the context provided doesn't contain
information about the price of a product. Could you please provide more details
or specify the product you are interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
jl37SpT18yLDpKeErye1CFzDVselVTjZSv_RVN1fA1yHsgycjGD1-AAqD9YdJ1DaLvvy-NXfhuyarofIUs8c5Q
Parent:
qG-MclQB4tcPo_IDrpnwKXSccRvWgHAUzM6rY63jpPZp2LRHxq5iaq4Kgn-gNcd_sQydUflDkM8gGmnoenCzhw

=== Listing Sessions === ✓ Found 3 sessions Session 1:
jl37SpT18yLDpKeErye1CFzDVselVTjZSv_RVN1fA1yHsgycjGD1-AAqD9YdJ1DaLvvy-NXfhuyarofIUs8c5Q
(Untitled) Session 2:
qG-MclQB4tcPo_IDrpnwKXSccRvWgHAUzM6rY63jpPZp2LRHxq5iaq4Kgn-gNcd_sQydUflDkM8gGmnoenCzhw
(Untitled) Session 3:
6vuPox7U6AzsVb7dniAaHDwJXJG7M4Vq9wwTyOecbl5pUvuf6mZ-Rk5P4p-2VHYZLB9ynnGPi8Cl0-bU_vq3NQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
jl37SpT18yLDpKeErye1CFzDVselVTjZSv_RVN1fA1yHsgycjGD1-AAqD9YdJ1DaLvvy-NXfhuyarofIUs8c5Q

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.00s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
K2pfYMQH805byyK1m-ttcqUIUpsgjl0ATkFCimQe3MfOyGcT8PSEhin6Gj05lbI5AlA3GPh1mKdY9hTOnTBRjA

=== Sending Initial Message === ✓ Message sent Response: Answer: The available
product is a high-performance database product named "ekoDB". It is priced at
$99.

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: I'm sorry, but the context provided doesn't include
information on the price of ekoDB. I would recommend checking the official
website or the platform where ekoDB is sold for the most accurate and up-to-date
pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
gCQ0KOMfyF62QuoyABvyWlQg0eTLqa-0V8wAplTvIRrRNnlIywMtHlRzS4i-GM4rSk61UZXenP7tTndJnc_Znw
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 1.18s Running
`target/debug/examples/client_functions` === ekoDB Rust Client - Scripts Example
===

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
HoUIv7FtcxrL26QaO8fgBuZof7Kh8bUOYBGPSqu--VcVd9u4MVJ9faPsweWI8UmX_ySj2YVzL7vLv6wsL87_Sw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
WgfNofy5e7h11LaThxw2MAt3herLtB2t-WigHJQY7ysYiiIPtKs81ydB5uYtcRNQ3bVrOSIhrVKIuE39eSMrJA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
TjHVP_qpAYIciRC0O1O5OlL8u9KvGNqEsZpPUEFjf646yFf0y1xhCCSEzhXsegs9IAZZ561-N2x0A3oBL7LMzw
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 3 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! ✅ [32mRust client examples complete![0m ✅ [32mAll
Rust integration tests complete![0m 🧪 [36mRunning Python examples (direct
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
[32mPython client examples complete![0m ✅ [32mAll Python integration tests
complete![0m 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m [34m
╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test Suite
║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m [32m✓ Server is ready[0m [33m === Running 7 Examples ===[0m
[34m === Running simple_crud.go ===[0m ✓ Authentication successful

=== Insert Document === Inserted:
map[id:DajANNNCnjO8I4l-vHEreQ2aiRLo0jY3CbTbIZmJlBd8XWJ-BBw4_9-m5ADWvqce8klESpN56oqUaEjVJflIig]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:DajANNNCnjO8I4l-vHEreQ2aiRLo0jY3CbTbIZmJlBd8XWJ-BBw4_9-m5ADWvqce8klESpN56oqUaEjVJflIig
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:DajANNNCnjO8I4l-vHEreQ2aiRLo0jY3CbTbIZmJlBd8XWJ-BBw4_9-m5ADWvqce8klESpN56oqUaEjVJflIig
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
FcYv2T8043aRaJq2URI56f8dcaxvd3uHgbcAoZo0VouAL3Da5L5D5MCKSAf29hThOk0F5BSTyk10f8VG0EvsDA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"bk4bCkONct5iiO7cGmRUEYih--5u3TSYBQBvnBf7oLbPFW8O9KekSQ_V6NNbM5JjduSNOA-TQs5W87l0V4qeMg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"ygX4zwfZD59Pc7Wlu1yNBjgmQZtkOpiXEpwp9VZMKq1H-3SZoGHnVaSVZv5jcMnvUabr6bpopfbc-Um7Ps9GQg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"FcYv2T8043aRaJq2URI56f8dcaxvd3uHgbcAoZo0VouAL3Da5L5D5MCKSAf29hThOk0F5BSTyk10f8VG0EvsDA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[OXl4cyZBM-OWvORJO7pJ0Fq0I7e5iAUdzUAMj910m1kMDYSbVkjIRIkfrWa3yfDKm3wdULfF03hozCVstt9xRA
dGXqKsn0hXFMHdlklGf_bp0_9zZpvZ4KEjJbKoHrn0se6RsbCP7UuCll5SENpPptVK-YOFAfoAVwEGsGLkMPhQ
j2q7BZ6i9egQ79OWzuAP56iNNb7o1HX_4xZFSo0sabgtuM7frLtNuwor4iWk-z237lDaUnoGXXUcVKXXARpCcg
WfnSftDsNlWBryZm_eDa_2roV6Z_msG6G8JsZlhw_AKYjNr4C4CgiRH_SvzQsun04JWx3lcaNKSt493u3_dQrg
bD1x9NTiqdr7fsX-DJqudy30h2LENaGFIbwSWYsj80mH2qCQPZ_4Vyok3WAB5h1IInmli0uHr_1_CFrMfB2Hzg]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[1XSvz3_HMzqJno9BgTsT5_5W5U817wazBsM2jEBuXQwUGZevuZYBI7dFEt00KfHrsymkTblRhRlwkV3LWf-9Sg
eKh4x6Gcy4i2nYk4weMB2wPFXCT_FhLU6LRI_d4B0KkgHjkOtmN5t4jLveIKLN3buD5f7AipuqmkWw7fkTmvhw
zKt5g_5SpZBCTDaqNa1kiD30_Wr6_L3Jk9dvfqyM9neLkOr9eH3gvjQeq-dvJBfEad1tNlxri57Yuc5h7Y7awQ]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[1XSvz3_HMzqJno9BgTsT5_5W5U817wazBsM2jEBuXQwUGZevuZYBI7dFEt00KfHrsymkTblRhRlwkV3LWf-9Sg
eKh4x6Gcy4i2nYk4weMB2wPFXCT_FhLU6LRI_d4B0KkgHjkOtmN5t4jLveIKLN3buD5f7AipuqmkWw7fkTmvhw
zKt5g_5SpZBCTDaqNa1kiD30_Wr6_L3Jk9dvfqyM9neLkOr9eH3gvjQeq-dvJBfEad1tNlxri57Yuc5h7Y7awQ]]
✓ Batch delete completed

✓ All batch operations completed successfully [32m✓ batch_operations.go
completed successfully[0m [34m === Running kv_operations.go ===[0m ✓
Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995] cache:product:3:
map[name:Product 3 price:49.989999999999995]

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.go completed
successfully[0m [34m === Running collection_management.go ===[0m ✓
Authentication successful

=== Create Collection (via insert) === Collection created with first record:
P4I3wwWuwvAPPEx-928ZvZclX1NdbOGap9uWTW0zBPUMWNBUM-FPyOP-VyRX-zzZ_6PXyNl3zVXUr6PzdCbuMA

=== List Collections === Total collections: 14 Sample collections:
[websocket_test chat_configurations__ek0_bench schema_users_client_py ttl_cache
batch_users schema_employees_client_py schema_documents_client_py
chat_messages__ek0_bench demo_collection schema_products_client_py
scripts__ek0_bench ws_ttl_test users test_collection]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
KobX3rdvW1L2ttWnENC4VHZmYtXJ_uaa97RuzxLt-EG5W6AvJ-6eUAyL1I6O6VDDJ3faVoHo616x912lGCh6YA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
7LGjfBIsokCWhXPjQskSv3AaNBYf7FjDhqdAcBNyNN0IWhMpYASiXzVBPlpBJ9Avgr-FgHE8uf1r1s-nESvBIA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mGo direct examples complete![0m 🧪
[36mRunning Go client library examples...[0m ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted:
map[id:s5wrNZrE0NuevWjmjuicqEJ2Ulweli5K2abm9aXaEkL8-7fOeWdD8zYugHz3SYuB9-Jewaj_zuAY2RQrPRMklA]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:s5wrNZrE0NuevWjmjuicqEJ2Ulweli5K2abm9aXaEkL8-7fOeWdD8zYugHz3SYuB9-Jewaj_zuAY2RQrPRMklA
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:s5wrNZrE0NuevWjmjuicqEJ2Ulweli5K2abm9aXaEkL8-7fOeWdD8zYugHz3SYuB9-Jewaj_zuAY2RQrPRMklA
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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

=== KV Get === Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995] cache:product:3:
map[name:Product 3 price:49.989999999999995]

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
opU53uDBvQri5mNY6E6WYC2stN8x9ezdOtOy4V35DAn6BSQWrQsUV_lQZe_8CVeb5WZ78VfmuOFH2-I9_KBMyw

=== List Collections === Total collections: 14 Sample collections:
[websocket_test client_collection_management_go chat_configurations__ek0_bench
schema_users_client_py ttl_cache]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
9T6TaBdeLGLshiiR2mxQ9ZjTqGkSDW9J3pZnNmazyy6MoP61uGJcqe8c53CaJb5tzQWZePPR-aDEd_X-YJ2HwA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
JbnNZYyQMIVnsr2nKR-UOe6FZruvJ7bG3VAxnbCs8FGeQR5j3myzdTmVEmf--yDmxqryRlo4lTd4HugVkWD8zQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
6XSFGU2_mPi7VEF3fYLNYPYwt1ouiMa4FiBOyuq2tStVF_meeiU8mQGORx6D8_Qa2cXG9Lh6Gt5PBhddIozV8w

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
rLWOWDo8Md0D2wqwwQBgr6osaYKq9-iiDXKqKa5KxNGHpYiM0LxQwOjLKpi1tqXkbFdq0o3Cb-L0OEFTc3Q8kQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
XTIulD8-zJOCZDUG_73NrBoP2NbE8rQOTNMH9MRYPjnsmlZvR_AgohumDZpQ2-8RTGemZn1rJ_tV3eSyaHfzhA

=== Sending Chat Message === Message ID:
BvljaZn2l2_rWhC1sVkqp8pjfdaoks9_a1hO6itXUtxHCuKDbBzYDCYR8nVbQQjivHHOlj3VmE8aNFNhUMIsVg

=== AI Response === There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. It is
   priced at $499.

3. ekoDB Pro: This is an Enterprise edition product with advanced features. It
   is priced at $299.

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:Bh_LTw6q2VxsT89PZYbD9kMW0gfRjSH97rslsyiHbganDsVOLwqblOgm6pGs_ljniJyKpjfD49k5CBjWi-3FQw
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:sllbhFoaHzA1IQ6kqhq4gc6LapJ1wgqCqT84RB8CxflXS81DjxUAn50HTCsxjglxO4ATLRo--XQCw88ZaldCSg
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:uHDRQ4pZV0mc_0sIuTtAPJZ9tx5K1eONUaEu-0xxoPUURMQ-WFk-RZSnkhN3lVs_S_XF6j8u4xqN18ZX7qzu3g
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111]

Execution Time: 3684ms

=== Token Usage === Prompt tokens: 430 Completion tokens: 78 Total tokens: 508

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
mi7hFPVOAsbqyVH3r6ItfGqqUuRJpQmk9TTtMq28cgDg1WOPfNt7xDphiYZ9nYl--xfnYvgKgExzfowr8g8WBw

=== Sending Messages === ✓ Message 1 sent Response: Based on the context
provided, the product available is "ekoDB", which is a high-performance database
product. It is priced at 99.

✓ Message 2 sent Response: Apologies, but the context provided doesn't include
information about a specific product or its price. Could you please provide more
details?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
kL3hYwS_H9WDoMohVlOUfgc8SiqeyjTeX0wc-7KNA30QC8PA7nnUi05VpnpMtrIfK0JWHomXyHwSFAMAv0W7LQ
Parent:
mi7hFPVOAsbqyVH3r6ItfGqqUuRJpQmk9TTtMq28cgDg1WOPfNt7xDphiYZ9nYl--xfnYvgKgExzfowr8g8WBw

=== Listing Sessions === ✓ Found 9 sessions Session 1:
kL3hYwS_H9WDoMohVlOUfgc8SiqeyjTeX0wc-7KNA30QC8PA7nnUi05VpnpMtrIfK0JWHomXyHwSFAMAv0W7LQ
(Untitled) Session 2:
mi7hFPVOAsbqyVH3r6ItfGqqUuRJpQmk9TTtMq28cgDg1WOPfNt7xDphiYZ9nYl--xfnYvgKgExzfowr8g8WBw
(Untitled) Session 3:
XTIulD8-zJOCZDUG_73NrBoP2NbE8rQOTNMH9MRYPjnsmlZvR_AgohumDZpQ2-8RTGemZn1rJ_tV3eSyaHfzhA
(Untitled) Session 4:
xk1gRyFojsYWBB7kDMWNkw5y4h5JIcVoLN-zoBLR-Ws_UEc2ZU_DROpqO2lL92f4140PY5eFcPvVj8cpfOeyxg
(Untitled) Session 5:
nhwhhgOvM3Cs7T7pPSwLlWkqiCW86qVdGG2kHPlhvy_kUxnHKlipsiOXJ_S4NiIuccj1fcrME6jXHzyY1I9Aew
(Untitled) Session 6:
bEGYtiLgEbT1EbPG8_0FOTynTXtkYIvcZGBaPcKgHVDQ8pSe4ATrA0QDir6A-85pvJpCR4dZe5GYNJPqqZY12g
(Untitled) Session 7:
gCQ0KOMfyF62QuoyABvyWlQg0eTLqa-0V8wAplTvIRrRNnlIywMtHlRzS4i-GM4rSk61UZXenP7tTndJnc_Znw
(Untitled) Session 8:
qG-MclQB4tcPo_IDrpnwKXSccRvWgHAUzM6rY63jpPZp2LRHxq5iaq4Kgn-gNcd_sQydUflDkM8gGmnoenCzhw
(Untitled) Session 9:
6vuPox7U6AzsVb7dniAaHDwJXJG7M4Vq9wwTyOecbl5pUvuf6mZ-Rk5P4p-2VHYZLB9ynnGPi8Cl0-bU_vq3NQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
kL3hYwS_H9WDoMohVlOUfgc8SiqeyjTeX0wc-7KNA30QC8PA7nnUi05VpnpMtrIfK0JWHomXyHwSFAMAv0W7LQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
943tbxFY6hHKBG3HXk19X-nHPEi6BGSLoNlPmAgBExCX-eDisj1KHNh4l72p9kB*-PO7gseJRiBqRv4vx*-GYw

=== Sending Initial Message === ✓ Message sent Response: Answer: The available
product is ekoDB, a high-performance database product. It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the provided context does not include information about
the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
jb5a8ZjKnd5P0u0yjq3ZJZ1JPu9YrAcfqXKC7OQW1B7bDSq69mH1k7NvTSkXRCiqbKBDw8D7CA3n02f_4sShJA
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! === Query Builder
Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 3 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740
1. Score: 12.540

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400
1. Score: 26.400
1. Score: 26.400
1. Score: 26.400

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600
1. Score: 39.600
1. Score: 39.600
1. Score: 39.600

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200
1. Score: 13.200

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.777
1. Score: 0.762
1. Score: 0.740

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.711
1. Score: 1.496
1. Score: 0.305

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: 4 fields Schema version: 1

5. Retrieving collection metadata: Collection has 4 fields

6. Creating employee schema with all constraint types: ✅ Employee schema with
   all constraints created

✅ Schema management examples completed! === Join Operations Examples ===

Setting up sample data... ✅ Sample data created

1. Single collection join (users with departments): Found 2 users with
   department data

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

4. Join orders with user data: Found 2 completed orders

- Mouse ($0) by Alice Johnson
- Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
OC8-pHM3kINrzorQPj7fOOyS7kC1iRDLtcTObzkJatT5XdfWOvPfQjw4JwwaMNX7zxdWZQlNtFnyCzchI8-MTw
📊 Found 30 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 30 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups map[avg_score:60 count:15 status:active]
map[avg_score:50 count:15 status:inactive] ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 7 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 30 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling ✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m 📦 [36mBuilding TypeScript client
library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 751ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'x2Z8PgI80SgoHwn54yajeHF_V5ruANlXkmCGBfTp4xqyBq_oSg34T9qZunyfEK4z9QQ6uByGWPIJ-ih9qf4cJA'
}

=== Find by ID === Found: { value: { type: 'Integer', value: 42 }, active: {
value: true, type: 'Boolean' }, name: { type: 'String', value: 'Test Record' },
id:
'x2Z8PgI80SgoHwn54yajeHF_V5ruANlXkmCGBfTp4xqyBq_oSg34T9qZunyfEK4z9QQ6uByGWPIJ-ih9qf4cJA'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, value: { value: 100, type: 'Integer' }, id:
'x2Z8PgI80SgoHwn54yajeHF_V5ruANlXkmCGBfTp4xqyBq_oSg34T9qZunyfEK4z9QQ6uByGWPIJ-ih9qf4cJA',
active: { value: true, type: 'Boolean' } }

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

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
ekqU7HXcstC87C9Qsh0RT-2XNTsjsaFnt_IVIsdzqQRDBfQQIqABvkyFTaLxfNmtvH279i6Q4WxGvcvxLFRmfw

=== List Collections === Total collections: 17 Sample collections:
schema_employees_client_go,schema_products_client_go,websocket_test,chat_configurations\_\_ek0_bench,client_collection_management_ts

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
ocM66myNyECsdURGU2uABjgkDzjmb8kLY3uDNIbms0ico_hrUVW1LLxVSYDdY5CEGsb04SeE0Cq2stp4x86viA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
0kTg-PRAXyxJOqnzAnUfMsprnUJWaJr0J7776oLDJiqsWm1MI6VrPLTmkAyVBZYBfcaC3gv49uzLOex1adoJkA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
e897jECKOgmUz1Lzroo_PrY9uO4CL2NIdbOIKXxfK_q-1Bv93aMkarN7GPfH6LpNWNoE7dLXbbsqMuQN1KjWtg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
udKVSq-rQdzoV71r0BvjmuDDnPOc1Ag2Dt9yh8FJG1lLYgAxyDCUzkDw1kE_3bDWBBKJ91PACETuMyIBmhxSwg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
KYvxTV6hNa76U1VJgoyAYjISJeetCX_6XpUHPfGCwF54g6NImIEG0mBd_tocpLV0qHI2gsnIy_Dha9tFj-Ue0Q

=== Sending Chat Message === Message ID:
hfv11PM03khioh5J1boS-6LoxlDevIdMDDHM6ObFPnYhrJb4G62vPszTzsob690ulAovTGw7wXRzHhr82pu3OQ

=== AI Response === The available products are:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is
   priced at $499.
2. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.
3. ekoDB Pro: This is the enterprise edition product with advanced features. It
   is priced at $299.

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { description: { value: 'Fully managed cloud
database service product', type: 'String' }, id:
'Jqtridh7uBeQiLaUfwZjm5Z1aUPcSq4lrZu3LY_PDUAkWl79MSkEvBKAR4N0SBlxrRgqYDC7C8FlKgWbD3jWwQ',
name: { value: 'ekoDB Cloud', type: 'String' }, price: { type: 'Integer', value:
499 } }, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet
2: { collection: 'client_chat_basic_ts', record: { price: { value: 99, type:
'Integer' }, description: { type: 'String', value: 'A high-performance database
product with AI capabilities' }, name: { type: 'String', value: 'ekoDB' }, id:
'le5-hx0cbOUAeQxxxxeUO-X4R6vosxAm1205Ahxd0S8NNvxKVM9opKl-qnr9WnVW2crA8QUHwJzBcYYQCZtZAA'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { description: { value: 'Enterprise
edition product with advanced features', type: 'String' }, name: { value: 'ekoDB
Pro', type: 'String' }, price: { type: 'Integer', value: 299 }, id:
'JR2I6VP-O4YaH3_IbNX4iS18V7uSdwUQCmz4EP0DdnO3r5yKXs9ZDS_orcq5Lp3ZKaFTfVkrO2S1sEhoCm75Qg'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 2952ms

=== Token Usage === Prompt tokens: 435 Completion tokens: 77 Total tokens: 512

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
3uOMa-Hw2rvS4Ako5ErWaV_UeihOuZ4cLrc9fmDRfBOsQryBsWB1zSpDLW-X2N0r0LUGMLTLQN8t5054ZHW5Nw

=== Sending Messages === ✓ Message 1 sent Response: Based on the information
provided, the available product is a high-performance database product named
"ekoDB". It is priced at $99.

✓ Message 2 sent Response: Apologies, but the context provided doesn't include
information about a specific product's price. Could you please specify the
product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
jEifjOxmLKwnziqSInuUz63h-fa1WvSnLbrkhXr03VjyPe-dKXbB_X1lr18AeQZxnN4m7Ss3CU4ymg0igRel6w
Parent:
3uOMa-Hw2rvS4Ako5ErWaV_UeihOuZ4cLrc9fmDRfBOsQryBsWB1zSpDLW-X2N0r0LUGMLTLQN8t5054ZHW5Nw

=== Listing Sessions === ✓ Found 10 sessions Session 1:
jEifjOxmLKwnziqSInuUz63h-fa1WvSnLbrkhXr03VjyPe-dKXbB_X1lr18AeQZxnN4m7Ss3CU4ymg0igRel6w
(Untitled) Session 2:
3uOMa-Hw2rvS4Ako5ErWaV_UeihOuZ4cLrc9fmDRfBOsQryBsWB1zSpDLW-X2N0r0LUGMLTLQN8t5054ZHW5Nw
(Untitled) Session 3:
KYvxTV6hNa76U1VJgoyAYjISJeetCX_6XpUHPfGCwF54g6NImIEG0mBd_tocpLV0qHI2gsnIy_Dha9tFj-Ue0Q
(Untitled) Session 4:
jb5a8ZjKnd5P0u0yjq3ZJZ1JPu9YrAcfqXKC7OQW1B7bDSq69mH1k7NvTSkXRCiqbKBDw8D7CA3n02f_4sShJA
(Untitled) Session 5:
mi7hFPVOAsbqyVH3r6ItfGqqUuRJpQmk9TTtMq28cgDg1WOPfNt7xDphiYZ9nYl--xfnYvgKgExzfowr8g8WBw
(Untitled) Session 6:
XTIulD8-zJOCZDUG_73NrBoP2NbE8rQOTNMH9MRYPjnsmlZvR_AgohumDZpQ2-8RTGemZn1rJ_tV3eSyaHfzhA
(Untitled) Session 7:
xk1gRyFojsYWBB7kDMWNkw5y4h5JIcVoLN-zoBLR-Ws_UEc2ZU_DROpqO2lL92f4140PY5eFcPvVj8cpfOeyxg
(Untitled) Session 8:
nhwhhgOvM3Cs7T7pPSwLlWkqiCW86qVdGG2kHPlhvy_kUxnHKlipsiOXJ_S4NiIuccj1fcrME6jXHzyY1I9Aew
(Untitled) Session 9:
bEGYtiLgEbT1EbPG8_0FOTynTXtkYIvcZGBaPcKgHVDQ8pSe4ATrA0QDir6A-85pvJpCR4dZe5GYNJPqqZY12g
(Untitled) Session 10:
gCQ0KOMfyF62QuoyABvyWlQg0eTLqa-0V8wAplTvIRrRNnlIywMtHlRzS4i-GM4rSk61UZXenP7tTndJnc_Znw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
jEifjOxmLKwnziqSInuUz63h-fa1WvSnLbrkhXr03VjyPe-dKXbB_X1lr18AeQZxnN4m7Ss3CU4ymg0igRel6w

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
xCY1-APAodg2sFRlp0ja8It7H6X1luOw-q1fr3abPhiwBqfp6CfAp0Ew4Kbg9qDQKhtCBW_NbTkk1BjH6WsVPg

=== Sending Initial Message === ✓ Message sent Response: Answer: The available
product is a high-performance database product named ekoDB, priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided doesn't include information about
the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
agSc1qqA3fBcsXeoo2Pb9poYM6IYV_HLOHHVfS2WMc5NdB1teuT1SNwXk9-0w8oig_v69k2dQbKZRF4d7iVEeA
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! === Query Builder
Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 3 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740, Matched: email.value, email, name, name.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio, bio.value, title.value
1. Score: 26.400, Matched: title.value, title, bio, bio.value
1. Score: 26.400, Matched: bio.value, title, title.value, bio
1. Score: 26.400, Matched: bio.value, title, bio, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, title.value, bio.value, title
1. Score: 39.600, Matched: title, bio.value, bio, title.value
1. Score: 39.600, Matched: title, bio.value, title.value, bio
1. Score: 39.600, Matched: title.value, title, bio, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.777, Matched:
1. Score: 0.758, Matched:
1. Score: 0.721, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.711, Matched: title.value, content, content.value, title
1. Score: 1.489, Matched: title, content.value, content, title.value
1. Score: 0.303, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: bio, skills, skills.value, bio.value

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: [ 'age', 'email', 'name',
   'status' ] Schema version: 1

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

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
z_XyfmcSPicF9uqjxQsWJizMwm5j_hbkl9RhSxCvZbBTJ8ZRdgUSNoOfMbUpOCWjvdoO26GAMTDwpwI3s2f48g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
SSXXFfo9muLKRq6b10PZZPPmN1-7nNoIRZxHVYQKbnLx4SsspFgGvJUkqmMmPeYg_8NwAwh1ndaiy3X6D2Orzw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
FgeNnWeaeJxreww2L2gLD78z4lPETSP2heOKKoVXGEjisKNvDGSwA2080K21B0Aq9AEg1W90xv8JSwenzVEs0w
📊 Statistics: 2 groups {"count":5,"status":"active","avg_score":60}
{"avg_score":50,"count":5,"status":"inactive"}

📝 Example 4: Script Management

📋 Total scripts: 11 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
uRcxipplCyYi5xEdexlBSn04yH5iGrkBUsp6RUT6O0eJzLzfQP8_LbdMo6D4CUFI4Etyg5irMtAnOrG4mVfB4Q
📊 Found 2 product groups {"count":2,"avg_price":474,"category":"Furniture"}
{"count":3,"avg_price":575.6666666666666,"category":"Electronics"} ⏱️ Execution
time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":3,"category":"Electronics"}
{"category":"Furniture","count":2} ⏱️ Execution time: 0ms

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
[32mTypeScript client examples complete![0m ✅ [32mAll TypeScript integration
tests complete![0m 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 717ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:2947) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 7 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:2951) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'GsstunTtWWub3fFPbAUYdRZgdrtZUU7UFy5sQlXcgeqPNuQTlcK53HFIiPVlW_XU0Ed4nYSrprOk_EO5BN1rqA'
}

=== Find by ID === Found: { name: { value: 'Test Record', type: 'String' },
value: { type: 'Integer', value: 42 }, id:
'GsstunTtWWub3fFPbAUYdRZgdrtZUU7UFy5sQlXcgeqPNuQTlcK53HFIiPVlW_XU0Ed4nYSrprOk_EO5BN1rqA',
active: { type: 'Boolean', value: true } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'GsstunTtWWub3fFPbAUYdRZgdrtZUU7UFy5sQlXcgeqPNuQTlcK53HFIiPVlW_XU0Ed4nYSrprOk_EO5BN1rqA',
name: { type: 'String', value: 'Updated Record' }, value: { value: 100, type:
'Integer' }, active: { type: 'Boolean', value: true } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:2971) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
NEqmdjgxKpektKkOQ871HMEMcX7i6kTHpDFBK39sEnKikL1mbuJtsaczQGjNPKpPJTckiC1YJCWwlSun-42kgA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"bk4bCkONct5iiO7cGmRUEYih--5u3TSYBQBvnBf7oLbPFW8O9KekSQ_V6NNbM5JjduSNOA-TQs5W87l0V4qeMg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"ygX4zwfZD59Pc7Wlu1yNBjgmQZtkOpiXEpwp9VZMKq1H-3SZoGHnVaSVZv5jcMnvUabr6bpopfbc-Um7Ps9GQg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"NEqmdjgxKpektKkOQ871HMEMcX7i6kTHpDFBK39sEnKikL1mbuJtsaczQGjNPKpPJTckiC1YJCWwlSun-42kgA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"FcYv2T8043aRaJq2URI56f8dcaxvd3uHgbcAoZo0VouAL3Da5L5D5MCKSAf29hThOk0F5BSTyk10f8VG0EvsDA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:2978) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:2981) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { userId: 123, loginTime:
'2025-12-08T14:45:04.930Z', username: 'john_doe' }

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

=== Create Collection (via insert) === (node:2985) [DEP0040] DeprecationWarning:
The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record:
0kj7Gq37Vqw3MC9oqZ7IpfAMdm821OIyN2r_J_6VqD_pozU2nsKt6bbpQ55y78kOLiZxOeP3mYHeJz5D8yT8hA

=== List Collections === Total collections: 22 Sample collections: [
'schema_employees_client_go', 'schema_products_client_go', 'websocket_test',
'schema_documents_client_ts', 'schema_employees_client_ts' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:2992) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
v-T-MpVDfwKw6Z2NHouk770eduJSu2nBRmR_SmhUIEYo1KmcuQZNmc2zAzTZSQmA2-ldIeLNhxAO3C3H5mAVSg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:2995) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
S-8MANTOdmep1Isz98E4qvjtpA1YROEvxJIrlm-FWLJ44teAiu666tVObOB6yfh6TZvN7vKLlmvqusNUJwNimQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 886ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 672ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'NZfxrBi8xm--iVNcgyi_yaQXv_mPB3L5e3Nhb7-cYUd42wGuyAwAqoPmFNLEMebE4VsrPUqsbrxOLc5NMjlLyQ'
}

=== Find by ID === Found: { value: { value: 42, type: 'Integer' }, active: {
value: true, type: 'Boolean' }, id:
'NZfxrBi8xm--iVNcgyi_yaQXv_mPB3L5e3Nhb7-cYUd42wGuyAwAqoPmFNLEMebE4VsrPUqsbrxOLc5NMjlLyQ',
name: { value: 'Test Record', type: 'String' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { value: 100, type: 'Integer' }, name:
{ value: 'Updated Record', type: 'String' }, active: { value: true, type:
'Boolean' }, id:
'NZfxrBi8xm--iVNcgyi_yaQXv_mPB3L5e3Nhb7-cYUd42wGuyAwAqoPmFNLEMebE4VsrPUqsbrxOLc5NMjlLyQ'
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

=== KV Get === Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
4vjf72_iCvvJtxUVSvLFVsluFi8yNyUkQXbmCw-ji5tEaUjDj5khDu7YIbBn78amoXndyN1RbH12bydGgd20kA

=== List Collections === Total collections: 22 Sample collections:
schema_employees_client_go,schema_products_client_go,websocket_test,schema_documents_client_ts,schema_employees_client_ts

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
mAvJP-DB4-do4GEE-\_uh7CVaiua2wd15aDuTuvlgKdG5VZ_0N6N-NSST7WO-pJMfCS525DpVc9earadddhdxdg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
PrmFxT0ZeKiQHF8TiE2pQHt6d8ck3ZdBPPyk8pEFCkvK3vpkHOhVPAMYTTZvpWuv2BxH51oX8XffiAQMKMW2PA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
BbgPVhfNw-v9RBL3Sm6zWsDBJ-9Qo6881JeC4TtDQoaOlgz5quqBOa05HvfUgI5h7YtOd8XBn1y20SNb1x8WPA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
VbDgbQtUQdDgeiv7cuMgkidVV_b6J_eQgK03hSGNFgN6ikGaxmH-2ry8NnIIel49zNlsPMpQqKKFZKEGI9--4A

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Script saved:
4o8u0kGMDA7QB*m2U5IV1636n8lMTWg1px-kQ1Ce3Djaib*-7PCYH8-wDedIdWsyIwioaTD1ObnA43v-GKdWjw
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Script saved:
B1viRt2itrLG5iwpiQP7AnH9_YAPFCvWr2vae0ZZaagSj_iuvn8ihC2K2RAFB6qP1NiPmniJbB8GXP_S3iMYXA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Script saved:
xH-PIcXArh2dfv9qLQLY3M5rasVM1m9yK68u60QiRr-c_iwRJZ7e8A4G0_IphmNQ2Yl3a1Zy717nvDEDSLINhw
📊 Statistics: 2 groups {"count":10,"avg_score":60,"status":"active"}
{"status":"inactive","count":10,"avg_score":50}

📝 Example 4: Function Management

📋 Total scripts: 13 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
Kwu0CMhcp20564TEK5c_IjTIBn0yqEzLqJM-3UWZq2UJ6iqm_JvlGvMjuwgwUSedy8h_JJqjRrpxJl91M3Wfdg
📊 Found 30 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 30 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"avg_score":60,"status":"active","count":15}
{"count":15,"avg_score":50,"status":"inactive"} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 15 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 30 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • ChatMessage helpers • Cleaner, more maintainable code •
Built-in error handling 🚀 ekoDB Advanced Scripts Example

📋 Setting up test data... ✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved 📊 Found 10 products ⏱️ Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved 📊 Found 2 categories Electronics: 6 items (avg $325.67)
Furniture: 4 items (avg $294.00) ⏱️ Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved 📊 Total products: 10 ⏱️ Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved 📊 Category analysis (2 categories): Electronics: Products: 6 |
Stock: 232 | Avg Rating: ⭐4.52 Furniture: Products: 4 | Stock: 43 | Avg Rating:
⭐4.26

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Wireless Mouse - $29 (⭐4.5)
2.  Standing Desk - $599 (⭐4.7)
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

✅ Script saved 📊 Users (showing first 5 of 10):

1.  User 8 (user8@example.com) - Score: 80
2.  User 5 (user5@example.com) - Score: 50
3.  User 1 (user1@example.com) - Score: 10
4.  User 2 (user2@example.com) - Score: 20
5.  User 3 (user3@example.com) - Score: 30 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Vector Databases Explained (Database)
2.  Database Design Principles (Database)
3.  Getting Started with ekoDB (Database)
4.  Natural Language Processing (AI)
5.  Introduction to Machine Learning (AI) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Vector Databases Explained
2.  Database Design Principles
3.  Getting Started with ekoDB
4.  Natural Language Processing
5.  Introduction to Machine Learning ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Vector Databases Explained
2.  Database Design Principles
3.  Getting Started with ekoDB ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Vector Databases Explained (Database)
2.  Database Design Principles (Database) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Scalability: Vector databases allow for efficient scaling as the amount of
   data increases.
2. Precision: They provide a high level of detail and accuracy in representing
   data.
3. Dynamic: Vector databases allow for dynamic queries and analysis.
4. Supports Complex Geometries: They are capable of representing complex
   geometrical features like points, lines, and polygons.
5. Less Distortion: Unlike raster data, vector data doesn't lose resolution when
   zoomed in or out. ⏱️ Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Advanced Query Patterns" - 1536D vector
2.  "Draft Article" - 1536D vector
3.  "Getting Started with ekoDB" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m ✅
[32mAll JavaScript integration tests complete![0m 🟣 [36mBuilding Kotlin client
library...[0m To honour the JVM settings for this build a single-use Daemon
process will be forked. For more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE Task :jar UP-TO-DATE Task :assemble UP-TO-DATE Task
> :compileTestKotlin NO-SOURCE Task :compileTestJava NO-SOURCE Task
> :processTestResources NO-SOURCE Task :testClasses UP-TO-DATE Task :test
> NO-SOURCE Task :check UP-TO-DATE Task :build UP-TO-DATE

BUILD SUCCESSFUL in 5s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
built![0m 🧪 [36mRunning Kotlin client library examples...[0m To honour the JVM
settings for this build a single-use Daemon process will be forked. For more on
this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple CRUD Example ===

=== Create === ✓ Inserted user:
Record(fields={id=StringValue(value=ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg)})
User ID:
ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg

=== Read === ✓ Found user by ID:
Record(fields={id=StringValue(value=ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)})})

=== Update === ✓ Updated user:
Record(fields={age=ObjectValue(value={value=IntegerValue(value=29),
type=StringValue(value=Integer)}),
city=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=San Francisco)}),
id=StringValue(value=ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  id=StringValue(value=ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg),
  active=ObjectValue(value={value=BooleanValue(value=true),
  type=StringValue(value=Boolean)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  city=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=San Francisco)}),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)})})

=== Delete === ✓ Deleted user with ID:
ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
TPsLMr3ZYbv0K3WviH0anRieCUAULPpsat9sFnh9RUX6qJoi_JeNgnr20aZOXVaSlRn6G8VZXMZL7tqvRMTrfQ,
h_y5bZy61YiEUQjcw8Jy3Whgv_7sC7n_42fbsR2shnYQVVWJzkvE4lnufGjty-rZFDtWaBRCyKHLtB_9opFHdw,
3C_SwnqmYeFCv_hazL4NlG1dRtWNaIbYIuaozXXDPxKBC3IQU03nrPjpzovsbL0zLQ26x7-egA7VyeNbmTOCqw...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records

=== Cleanup === ✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') === ✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) === ✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) === ✓ Top 3 scores:

- Score: ObjectValue(value={value=IntegerValue(value=95),
  type=StringValue(value=Integer)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=92)})
- Score: ObjectValue(value={value=IntegerValue(value=88),
  type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={value=StringValue(value=Charlie),
  type=StringValue(value=String)})
- ObjectValue(value={value=StringValue(value=Diana),
  type=StringValue(value=String)})

=== Query 7: Contains (name contains 'a') === ✓ Found 2 names containing 'a'

=== Cleanup === ✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1765205151442,"user_id":"123"} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 20 collections

- schema_employees_client_go
- schema_products_client_go
- websocket_test
- schema_documents_client_ts
- schema_employees_client_ts

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-08T14:45:55.685161Z","last_modified":"2025-12-08T14:45:55.685161Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
t0F0AwqIxASJJSvasumhM3c2R1gXrbd58h-ZBhW_ALylwH61lsj55J9Y5E1MWGn_peK1sG3NhK6QfdmN3zjxUA

=== Verify Document Exists === ✓ Document found: [session_id, created_at,
ttl_duration, user_id, id]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
zIoC5SVPkbGdGqSjRoMK29fhLEQGo6w05C3b4IwTLROU8yiZL08q_tHCr6RMWGtpyQ3s6fUFRuh-DACQPzQU5g

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [id,
ttl_duration, value, cache_key]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"a5nrYJVRJe4tXqN63nBOl97Q9WI8IiB-V-Sa6hBxycETXh5EtnibLlIFUt09BPPm5ZKrBdWZTTItzKPDxbOFlg","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=4kpGYHC0QCu0iOi0Pc3ouy8Q0LqdeZi3MaVieFV99G_yn\_\_ppcuNRwlmiEadg6EgwGtLTBCFBS5N9f1KZzjwhg)
✓ Inserted user 2:
StringValue(value=8DhQ6gvhKXs-jEhM29W-Wagwghv0IAmzxeFsEJM9wYXTDuvvSI14x4NqPVWjrAOeN9zFooSFBmeOVQEo9Jsxpg)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 21 Sample:
[schema_employees_client_go, schema_products_client_go, websocket_test,
schema_documents_client_ts, schema_employees_client_ts]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"title":"Rust
Programming","tags":"programming,rust,tutorial","description":"Learn Rust
programming language with hands-on examples and best
practices.","id":"DuATXOIyK0oxoyj5cKRXfbnNIj05cNmATRWpggSVYqY1I6zrY1uqXyT5CmBAFxxdrqiFH3FkUv7_LCfZemmJSA","views":834},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"description":"Build
modern web applications using JavaScript, React, and
Node.js.","tags":"programming,javascript,web","title":"JavaScript Web
Development","id":"AXfkiYY6By2xFWVT479vT93hff8GmceUMdJ9Eueodv2qOSkAtFYOukD4cVQfMeSn9Q8NClFK5UR81pfQ6e0eIQ","views":611},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science","views":220,"id":"TvoA-\_uMOPMFI4cJKlt7pt_VfRSd3QHBJ-XHFrO1tGjup0J_uW6Qu5jGFHKba9VnANpjZA1yNEzP-sps8eswNg"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"description":"Introduction to machine learning
algorithms and neural
networks.","id":"ohUO20FnYKr6_MDDdGWB9NkUSZJYVzOwISzu05-TEfHlRcwG359q67he9RpiRNeqcXvTnT6BkQrOkHsMEb8JBg","tags":"ai,machine-learning,python","title":"Machine
Learning
Basics","views":903},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"id":"TvoA-\_uMOPMFI4cJKlt7pt_VfRSd3QHBJ-XHFrO1tGjup0J_uW6Qu5jGFHKba9VnANpjZA1yNEzP-sps8eswNg","tags":"programming,python,data-science","title":"Python
for Data Science","views":220,"description":"Master Python for data analysis,
machine learning, and
visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"views":535,"description":"Learn
database design principles, normalization, and query
optimization.","title":"Database
Design","tags":"database,design,sql","id":"JMxTXwW-adM3O1n-z62bnmMvo6l0MQWpSDL0Eg1XRTuKVDeHaOWAPCDAMv7xxlNkckxrupy7_cWH5j-XQr731A"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"DuATXOIyK0oxoyj5cKRXfbnNIj05cNmATRWpggSVYqY1I6zrY1uqXyT5CmBAFxxdrqiFH3FkUv7_LCfZemmJSA","tags":"programming,rust,tutorial","title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices.","views":834},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=ckUiQ5UlYoyYepl9B1rZJgQNFb0Y1UBYWDmyf4o2tSiMbGH7f8ZsrtO1DBhid-KYoqypjUguYriIlD1qqR-rYQ)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1765205177035},"id":"ckUiQ5UlYoyYepl9B1rZJgQNFb0Y1UBYWDmyf4o2tSiMbGH7f8ZsrtO1DBhid-KYoqypjUguYriIlD1qqR-rYQ","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Join Operations Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Bob Smith: Sales
- Alice Johnson: Engineering

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
cr3r6_e0Eq2S0dEYv9SZ2LmOmF9lOHFaE65z8s41mW8nhpWhV73Y91tqlG1PK-wI3AiCJ2Np5AGKnTykzlkJUg

=== Sending Chat Message === ✓ Chat response: Message ID:
"w7LC-s6mANDQ0JcMqUpGfiYcdtpNNMCMU7mvcjEpBL-02biIONKd2KLCRsYYQzD2P1p4Xoe9nqXRUAXR9UTXyQ"
Responses: ["ekoDB is a high-performance database with intelligent caching,
real-time capabilities, and AI integration. It offers several features including
full-text search, vector search, and hybrid search with automatic context
retrieval. Additionally, ekoDB integrates an AI Chat feature that allows you to
query your database using natural language and receive AI-powered responses with
relevant context."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA

=== Sending Messages === ✓ Message 1 sent Responses: ["Answer: The available
product is a high-performance database product named \"ekoDB\". It has AI
capabilities and is priced at $99."]

✓ Message 2 sent Responses: ["Apologies, but there seems to be no context
provided regarding any product to provide its price. Could you please specify
the product you're asking about?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"x7qLfE_RozXT5AUCzv8GKC_cMhOgiEGviLLAP0Lix78ZELl0eRkIgtyTFDLBy6Q_88yBGatmT8XuxMEIIm8hlA","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-08T14:46:35.850886Z"},"id":"pp4djELwPAkNTIx-n7kdxhTpIrJRe2k1eYfMBSsmpsIpw6Vy57LYWspyCtniv3Vojt_8UOZ0UqSJLTBXXzm-Dg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":192,"total_tokens":219}},"updated_at":{"type":"DateTime","value":"2025-12-08T14:46:35.850886Z"}},{"chat_id":{"type":"String","value":"aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA"},"content":{"type":"String","value":"Answer:
The available product is a high-performance database product named \"ekoDB\". It
has AI capabilities and is priced at
$99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"x7qLfE_RozXT5AUCzv8GKC_cMhOgiEGviLLAP0Lix78ZELl0eRkIgtyTFDLBy6Q_88yBGatmT8XuxMEIIm8hlA","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-08T14:46:35.856596Z"},"id":"d_P2sNBrnppYfyk97aBI1SXj4ElJ8bmaFVihnRhJ5yI7rgBUmzJeOZzHZo3wd9JsnwZwa4VvlflJMlRII9Eosw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":192,"total_tokens":219}},"updated_at":{"type":"DateTime","value":"2025-12-08T14:46:35.856596Z"}},{"chat_id":{"type":"String","value":"aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-08T14:46:37.567655Z"},"id":"6uMsqaw0mkeC-QTJCUNCNLl5AuotlPd6FfeZOQErk_dc244M3JWpemf7-E1w-YaYt_fgsVIMrVcUWBEtAPjOnQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":70,"total_tokens":100}},"updated_at":{"type":"DateTime","value":"2025-12-08T14:46:37.567655Z"}},{"chat_id":{"type":"String","value":"aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA"},"content":{"type":"String","value":"Apologies,
but there seems to be no context provided regarding any product to provide its
price. Could you please specify the product you're asking
about?"},"created_at":{"type":"DateTime","value":"2025-12-08T14:46:37.572281Z"},"id":"ZnRXE82jXMrt6cQzafoiZN9RhaIp0snRInEvAXI0X0TBW7SV3EwGJi85dxSPOEhddbi0-qYrMz1uKSoxBQRNlw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":70,"total_tokens":100}},"updated_at":{"type":"DateTime","value":"2025-12-08T14:46:37.572281Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
aRa0cP1Q4h7Hvs8w65Q37E0gq5_I0vN-41p971hK6o5NqNkZFlyZ0Wj0DTwYsDgInfeK-QqnMrlDETft0oKOUg

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
GKGpGxOHujGesIm35zDMrJCVkPUaoGXT2H046T\_-YTcsqsUk854viG7T3fNCKwzf2_QeJOZcuWkoVa4UkLBlZQ

=== Sending Initial Message === ✓ Message sent Responses: ["Answer: The
available product is a high-performance database product named ekoDB. It is
priced at $99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses:
["Answer: The available product is a high-performance database product named
ekoDB. It is priced at $99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
v691\_\_hnyk1qa5Bk1D2NcgM5Bt7RHfn9o4cOay_epjmGpdSIgNefrxDH8SHq07I4xRh29jWqcEvXQgW5vK_rwA

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 9s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run 🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to no-operation
(NOP) logger implementation SLF4J(W): See
https://www.slf4j.org/codes.html#noProviders for further details. ✅ Client
initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
kECVBlWXLjNoQS9DK5REJYmjiIaNBk91dE5YLUBmRWJdGV5kxTc4Gs8rk1IV0kyOhLJHm8noGSQv0rD9WMMKKQ
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"status":"inactive","avg_score":50.0,"count":5}
{"avg_score":60.0,"status":"active","count":5} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 19 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 10 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m ✅ [32mAll Kotlin integration tests
complete![0m
