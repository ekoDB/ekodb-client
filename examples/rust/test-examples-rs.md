make test-examples-rust 🧪 [36mRunning Rust examples (direct
HTTP/WebSocket)...[0m Finished `dev` profile [unoptimized + debuginfo] target(s)
in 0.13s Running `target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("9hpDtjuAr6r_1ZJwIbbUVwXC0Js6E5l03hDTlejTBFm75bLYVrkM6aGyfxkijbN5HwQgndIdDChmC-tQ0w7Z8A")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("9hpDtjuAr6r_1ZJwIbbUVwXC0Js6E5l03hDTlejTBFm75bLYVrkM6aGyfxkijbN5HwQgndIdDChmC-tQ0w7Z8A"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("9hpDtjuAr6r_1ZJwIbbUVwXC0Js6E5l03hDTlejTBFm75bLYVrkM6aGyfxkijbN5HwQgndIdDChmC-tQ0w7Z8A"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("9hpDtjuAr6r_1ZJwIbbUVwXC0Js6E5l03hDTlejTBFm75bLYVrkM6aGyfxkijbN5HwQgndIdDChmC-tQ0w7Z8A"),
"name": Object {"type": String("String"), "value": String("Updated Record")},
"value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized
+ debuginfo] target(s) in 0.15s Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"xHB7dOzhn-TURJ68KMxUhViza0AENh17KfL1vjmYJsMyjVX49kQaqRZqLD_AbCoe-BrfQx-JZokRamE6b0Vm3Q"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"xHB7dOzhn-TURJ68KMxUhViza0AENh17KfL1vjmYJsMyjVX49kQaqRZqLD_AbCoe-BrfQx-JZokRamE6b0Vm3Q",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.10s Running `target/debug/examples/batch_operations` ✓
Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
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

✓ All KV operations completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.08s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"6pLz3KZlcc8xEqoVTwyIMmskgT-BowJwy1A9VvMzrOWG5mHmLQSom6NSnQrIf5CTodOWYixbSBNaACA3WG8tAA"

=== List Collections === Total collections: 4 Sample collections:
["demo_collection", "websocket_test", "test_collection", "batch_users"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"DuYw9RORUb64nW-fHnceeh1ccZrTTOxSxtwveqn6Un4iOD09pTMMDvTzfxcSPnE6sFPMXJaxSInNSI6MiWmtpg"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("XqwPlBLUsD98yk8OYTMaH77EV6gGm74aklYivnPMAQdYyo7RA_G5O4J4W74ZKuYo1ViyqkaYqoVwodjmqghcig")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.10s Running
`target/debug/examples/http_functions` 🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
E6QTwL66IK0gUE_e_YIvyUx5UcYehdSKQHYJ1z63f6sCDyhFOFPKQ2MaGTYHQfo3EuiiX_vFUNitaQnIb_ngZA
📊 Found 5 active users

📝 Example 2: Complex Filter with Multiple Conditions

✅ Script saved:
T-315lEXcqYM-c2OOM9z0zXtKr8xGqoLmf2lrInlzxk4eJBqSqXOql_ks2IXg5BB-Qjbi8MxB9Fx3lMQfyLzoQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
qeUYORRBXFTFMTwwnhYU1relS6Jzt-4oSO4aElOfW0P0Xgi0HoIOoOFFKX5d95wpFNoXoc0TJ6Dkx6p5Uafkzw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":60.0,"count":5,"max_score":100,"status":"active"}
{"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 3 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.09s Running `target/debug/examples/transactions` ✓ Authentication
successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
FN2T2VRD3r7q-88MvCk05_mbNNiL9IASdcOy5l18LWn6a6cFUX7UnlIgm3HJ2DvZHzQTeCfpwF6ZBAykc5jLJg
Created Bob: $500 - ID:
FfQA4GZZBZ9F5AyhAFs9H2btnxUguQq6ioBypE0k9sCA0DMfUcEvg2BhP9QvJcCRkt29LpH-rD9nzE4N1Nb95g

=== Example 1: Begin Transaction === Transaction ID:
f5763029-07e1-4adb-bf65-c95d0d829dea

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: "Active" Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"type":"Integer","value":700}

=== Example 5: Rollback === New transaction:
ce4dea53-0649-431d-ad6e-91ae6fc4843d Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed ✅ [32mRust direct examples complete![0m 🛠️
[36mBuilding client library...[0m cargo build -p ekodb_client Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.08s ✅ [32mClient build
complete![0m 🧪 [36mRunning Rust client library examples...[0m Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("VEYHUYoPB5zglTy32HMFRlZ3hxx7ZeAE3BNhE5ouJc_ALeN-RjYQwZjzyALZgAfbGqHimHjfEDFGw7Ytr_jAag")}
}

=== Find by ID === Found: Record { fields: {"value": Object({"type":
String("Integer"), "value": Integer(42)}), "id":
String("VEYHUYoPB5zglTy32HMFRlZ3hxx7ZeAE3BNhE5ouJc_ALeN-RjYQwZjzyALZgAfbGqHimHjfEDFGw7Ytr_jAag"),
"active": Object({"value": Boolean(true), "type": String("Boolean")}), "name":
Object({"value": String("Test Record"), "type": String("String")})} }

=== Find with Query === Found documents: [Record { fields: {"value":
Object({"type": String("Integer"), "value": Integer(42)}), "name":
Object({"type": String("String"), "value": String("Test Record")}), "active":
Object({"value": Boolean(true), "type": String("Boolean")}), "id":
String("VEYHUYoPB5zglTy32HMFRlZ3hxx7ZeAE3BNhE5ouJc_ALeN-RjYQwZjzyALZgAfbGqHimHjfEDFGw7Ytr_jAag")}
}]

=== Update Document === Updated: Record { fields: {"value": Object({"type":
String("Integer"), "value": Integer(100)}), "active": Object({"type":
String("Boolean"), "value": Boolean(true)}), "name": Object({"value":
String("Updated Record"), "type": String("String")}), "id":
String("VEYHUYoPB5zglTy32HMFRlZ3hxx7ZeAE3BNhE5ouJc_ALeN-RjYQwZjzyALZgAfbGqHimHjfEDFGw7Ytr_jAag")}
}

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized
+ debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
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

✓ All KV operations completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"am3VYRYLKCI1_NONWR13x1DrAoM94Nyi5cLxtVYteAPf7ECTIylaIj-8dyHqMvVhJBqTitRnHeTsVKk7_pKlNw"

=== List Collections === Total collections: 9 Sample collections:
["client_collection_management_rust", "websocket_test", "test_collection",
"ttl_cache", "batch_users"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"g67v9Au*kCuYzrr-UDVUwTsJTRNE44MTrl9hO*-kWlIR2IrF0MS9_1v5b1UkXLR23fGVL2R0i50lmmYSbzm58A"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("OKeDQhZeewb0YqbkMhrfaV3wiYkthtxO_NCzDPJO-a_73fFhCIxlmS7vnpImI4uwlKdpIZfzVhZMwlYAFEBQ0g"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
LpThyF1bQn-9VnFM8507EelrE22jsSMHYutVMPB2yq2HBWDS5oCm1fW36OIWhdhHNDH80JSyuNFcHeu1Do6P6Q

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("6rk1O8GKTdvKuajKQ59DhkDdJSaeeR7tirVtrXzOzSN6pcKL0FhwdRQJngyn_oagdEc4Zqr1Hdz6xIUZgMtJ9g"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/examples/client_query_builder` === ekoDB Query Builder
Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Eve")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Eve")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"type": String("String"), "value": String("Bob")}))
2. Some(Object({"value": String("David"), "type": String("String")}))
3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"type": String("String"), "value": String("Charlie")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator === ✓ Found 4 users NOT inactive

=== Complex Chained Query === ✓ Found 3 users with all conditions

=== Cleanup === ✓ Deleted collection

✓ All query builder operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
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
   ["description.value", "description", "title", "title.value"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description.value", "description"]
3. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description.value", "description"]
4. Score: 1.0000 Title: Some(String("Database Design")) Matched: ["description",
   "description.value"]

=== Weighted Search === ✓ Found 2 results with field weights

1. Score: 23.1000 - Some(String("Python for Data Science"))
2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options === ✓ Found 1 results with stemming

1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit === ✓ Limited to 2 results (requested 2)

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup === ✓ Deleted collection

✓ All search operations completed successfully Execution time: 2ms Finished
`dev` profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("puigsNDOGrGUPoMMhVun08YWBTYD5fqJddBLRdiDKif2T0pnCuYxzRmj8wGZ_LfaNGEUf9APZw6ej4YzMigZTw"))
✓ Inserted user 2:
Some(String("M0aKwQgK1wzIl2v4Eiq4hfVk1W_bcBLBXB06FbE1m6mgRiEehRx-AAxsLrofcdZTOdWg4nzkFjGkh7kFOJIg4g"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- status: String
- title: String (required)
- age: Integer
- email: String (required)

=== Listing Collections === ✓ Total collections: 9 Sample: ["websocket_test",
"test_collection", "schema_client_rust", "ttl_cache", "batch_users"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_joins` === ekoDB Rust Client - Join Operations
Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Alice Johnson: Engineering
- Bob Smith: Sales

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
wFcIqUdFKFba7_NRR2SD1v1j68swGE6y6e1ZCGgA8_XfIMj_dJ_fXiQhjVuSR3TrnICo0OW2clU5xYAVpYChpw

=== Sending Chat Message === Message ID:
C0-xT-i8QsiGVRK35wxEqa74O6j2i1oKJwhRcNsHRfrvWpx3ZR9Cukxed1vJcDyZaSI8J10IDdO4vmqxRG3_9Q

=== AI Response === Response 1: I'm sorry, but the provided context does not
contain any specific information about ekoDB or its features. Could you please
provide more detailed context or specific information about ekoDB so I can
assist you better?

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["content", "category", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("140V9N3Adn8F3R4VAlAOKFifb7O6NjHAY4CTAbC_SS6Eo1c46skUFwkM1fPIXHsLjDFn9rJRISj07m6PTO7Bmg"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["title", "category", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("oZ3W7ZOv4-vXFWHsucoq4JKXH50LnwNwRjPhkNUvik9ZOj-KAzBhD-Fn-wsCseFgX1p_iikfWKCgey4sAHOHvw"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("k73_B1RmCsSizvNnU2tCgkf0vcqDIbWS3CsHIxpV-oJLxzVInxb_YferBigqsL32pso13bRLFJoiTdQVIQ2_qw"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Execution Time: 3316ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.08s Running `target/debug/examples/client_chat_sessions` ===
ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
ibO8oA4VdNKJx8MlQKtRtIuVB5WanPjnMLOrJSIIietcdTJk5n1aMoIgHOY00veFNThz80mq1tJMflxUHEmqEg
=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not contain specific information about any available products.
Could you please provide more details or clarify your request?

✓ Message 2 sent Response: Apologies, but there doesn't seem to be enough
context provided to answer your question about price. Could you please specify
the product you're referring to?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
uviWL2Srz8QZeCTNv4s0-R3MPBnpYgdoWQFHyavzhrFBmvuDWiKEsVlqFlOSrBL26sUMn1YFryESBo0SebA0xw
Parent:
ibO8oA4VdNKJx8MlQKtRtIuVB5WanPjnMLOrJSIIietcdTJk5n1aMoIgHOY00veFNThz80mq1tJMflxUHEmqEg

=== Listing Sessions === ✓ Found 3 sessions Session 1:
uviWL2Srz8QZeCTNv4s0-R3MPBnpYgdoWQFHyavzhrFBmvuDWiKEsVlqFlOSrBL26sUMn1YFryESBo0SebA0xw
(Untitled) Session 2:
ibO8oA4VdNKJx8MlQKtRtIuVB5WanPjnMLOrJSIIietcdTJk5n1aMoIgHOY00veFNThz80mq1tJMflxUHEmqEg
(Untitled) Session 3:
wFcIqUdFKFba7_NRR2SD1v1j68swGE6y6e1ZCGgA8_XfIMj_dJ_fXiQhjVuSR3TrnICo0OW2clU5xYAVpYChpw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
uviWL2Srz8QZeCTNv4s0-R3MPBnpYgdoWQFHyavzhrFBmvuDWiKEsVlqFlOSrBL26sUMn1YFryESBo0SebA0xw

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
8S0tYVoRFBTY0zxngkWkaKQL-lOSZ-PC01APteclqfDQsKXloEQit3Oi0eLt-Mm7C1Q4fenm5vZBuLlNraTilA

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain specific information about any available
products. Could you please provide more details or context?

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: Apologies, but the context provided does not include
information on the price of ekoDB. I recommend checking the official website or
contacting the seller for the most accurate and up-to-date price.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
DZrN-AQV9Nn4wOTelR2KA6397oce29THpcBtxbI_iT2QOQo1yaK92LZIYuZGB_sN1pUrtH1Cg8FTgApuh7BRoA
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_functions` === ekoDB Rust Client - Scripts Example
===

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
5YLeo1S6KjsVnMk4j8F-BkPIaQdwTUSODHQgJlo4YsqIJt-4xSUUs8ktUjbrFjGIcsupdXqHf8UXCGdgvAyBOQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
82PQ7OrjCrgaH6MP8SAi4R0GAyzBDdItHpCbmrxrYZgGHEusz05fvushTxLOdzd6dzJKKPrhTIxpIbAFCKX6lQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
aWGzIv6K1qdVtrsEN3hzWRu_IXQnD1LKnIzG6CQlmfXBxVfYOPvphIZr8reYAHqh5DJOgn1EtYDx8SOolM8JmA
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 5 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! ✅ [32mRust client examples complete![0m
