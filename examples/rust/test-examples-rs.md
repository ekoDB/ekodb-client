make test-examples-rust 🧪 [36mRunning Rust examples (direct
HTTP/WebSocket)...[0m Finished `dev` profile [unoptimized + debuginfo] target(s)
in 0.22s Running `target/debug/examples/simple_crud` ✓ Authentication successful

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

✅ All examples completed! ✅ [32mRust client examples complete![0m
