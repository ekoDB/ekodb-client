make test-examples 🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

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

✅ All examples completed! ✅ [32mRust client examples complete![0m ✅ [32mAll
Rust integration tests complete![0m 🧪 [36mRunning Python examples (direct
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
[32mPython client examples complete![0m ✅ [32mAll Python integration tests
complete![0m 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m [34m
╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test Suite
║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m [32m✓ Server is ready[0m [33m === Running 9 Examples ===[0m
[34m === Running simple_crud.go ===[0m ✓ Authentication successful

=== Insert Document === Inserted:
map[id:E2DT9eZu4l_LVoKeii5DFgdo1VGA1p-1Srb7J4Xv-gxQWnJv9UVtYUU9-iKZKZze_lup4rUl8WGqlgNoxKGXiQ]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:E2DT9eZu4l_LVoKeii5DFgdo1VGA1p-1Srb7J4Xv-gxQWnJv9UVtYUU9-iKZKZze_lup4rUl8WGqlgNoxKGXiQ
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:E2DT9eZu4l_LVoKeii5DFgdo1VGA1p-1Srb7J4Xv-gxQWnJv9UVtYUU9-iKZKZze_lup4rUl8WGqlgNoxKGXiQ
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
tHnRpXS9iyrx1UuHjVIcjmAukoOpDqcb8Vibum5lJBrzq4lg2KxYmcaaf0nF2VPwRhNNDkl_gkP3b4Tr_E6DKw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"xHB7dOzhn-TURJ68KMxUhViza0AENh17KfL1vjmYJsMyjVX49kQaqRZqLD_AbCoe-BrfQx-JZokRamE6b0Vm3Q",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"gxyJOI5aO4nhazTz8ifgYCc10ASLp66ugU-6T1lqQ8eZiBqUWG9WA3Sc0Dkd8Pjpv60sdsWUbp8WImMAo_lVzw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"tHnRpXS9iyrx1UuHjVIcjmAukoOpDqcb8Vibum5lJBrzq4lg2KxYmcaaf0nF2VPwRhNNDkl_gkP3b4Tr_E6DKw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[l76PCfw-oG35i69Nh_byuVpzWXXoSvzt5UPUWuRIjCzaK3G8oo7Rfi22aHr5vM3wZWOY5zNugzNQy4if66CrfQ
XIb5xKGxOcMPi0gg6VXgJapYYGshAXlxaStE9rVz0HJKmK4w6mZwaIjhj0VdxAV21jqJh3I4tQ6QzDH9LU5AmQ
NSSfZ9EVVX44v4Rycj6G8utNfoyJsoG94eTlDlzqBOWYqFhTxvGoB6uCzguPRZZPqti_f83jRffq3YoqsS09ig
mvfzPrdd0ZVuLrLSXoBrLYkQLCmZ7LWkeUWWEe8jNIPszQs2YgNVnGnDBoxJWeTxneNwQxSYdyWryDoRHuMeCQ
l1XALdQunZuwHqDLYNUP-UbsX8r0oa-7MASqD1Phzwy4vTnSS7b5eBKmORzuN5mQKM7kbeU2Y1UuMN-k49wU8A]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[RBBepMDYKkgaKZTNFR6k8YrR4VmmArtxuzSKrxcCf4VBDvqEefpu9Nq_pONgDZbMYZRdc-5pnl5SC0PomKtgxg
Il0HhrPKKFZWpwMTC0GUva5cC0Jy57ilkWvvy1Mbqp4Ur31F8fc9MLwenDgOxS9L-qf6Po0yuD7FQtp2Pz0d4Q
-7w_mqSgAlCNo8QGwPWXjij4oM8b-dG9KB8nlo7thlprWb9ZFBcICwihHSLVyqNN5DoIpWhkyPpAH5Vk_ZOo6Q]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[RBBepMDYKkgaKZTNFR6k8YrR4VmmArtxuzSKrxcCf4VBDvqEefpu9Nq_pONgDZbMYZRdc-5pnl5SC0PomKtgxg
Il0HhrPKKFZWpwMTC0GUva5cC0Jy57ilkWvvy1Mbqp4Ur31F8fc9MLwenDgOxS9L-qf6Po0yuD7FQtp2Pz0d4Q
-7w_mqSgAlCNo8QGwPWXjij4oM8b-dG9KB8nlo7thlprWb9ZFBcICwihHSLVyqNN5DoIpWhkyPpAH5Vk_ZOo6Q]]
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
EeK8Avr_heKVqsFly_3LqpX-7j2iDlTd3rrDnPLop9VSwaNlJ44mQV8Rh-pAnQ5DKCE3kS0sYtsOH4u_VFeOlQ

=== List Collections === Total collections: 15 Sample collections:
[schema_employees_client_py demo_collection schema_users_client_py
websocket_test test_collection schema_documents_client_py ttl_cache
chat_configurations__ek0_bench batch_users test_accounts
schema_products_client_py users scripts__ek0_bench chat_messages__ek0_bench
ws_ttl_test]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
HBuof95rPU-RZFUD22hV9pTiBNhFKJVmVTp1tJkoJ\_\_MgaAw9g9k5y2czeY_dGok4W4O3TjZKXqB53kcBYzNvw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
xPXDMPwhplbRIiIEawZoom8x9ZFQoS7Wp09XQCz_Z1MabQ_ha0mhx7pcHHqxTkHjo8KIHAVjsZPVLJSU0QjsLA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m === Running http_functions.go ===[0m 🚀 ekoDB Scripts
Example (Go/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
tAw_KhuiiZFnma75AiqXoEdQ52lr7OZNwacq3Sku9cZvPvmRHfVwem1ZxPe0qapP1j42E1Xf4whcMe0YD91BUw
📊 Found 50 active users

📝 Example 2: Complex Filter with Multiple Conditions

✅ Script saved:
0NwCAqNnrjNo31Q-JJ1DYFZ97sGHxNTaiMPWxfaBu2rnVp71-3z6mv19_7NWRoY3rVpmldYdcGIcmot7sAQT8w
📊 Found 10 users (status=active, score>50, sorted by score)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
8HBZodO0GakL3GhLmyW0-qyYMh0GWEWK2K4vC_OFMO1H8PrVNRr_ZUxdzlBXQojvjrgISZEqqASDSlTuXzYjNw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":50,"count":25,"max_score":90,"status":"inactive"}
{"avg_score":60,"count":25,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 11 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.go completed successfully[0m
[34m === Running transactions.go ===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
lxV4qH6c8xTCOXzrKuAdJnFXpBDfR6VwZE4XWG05OK-PPhgAKoaGcpkHbUJXMbTdCL1y8I2uXyuP4qD8DHDlrw
Created Bob: $500 - ID:
2cUUWwA2Rkh9m_Uy5MaZBI67QSMe7MahtAjxdm2a5JZSK5Fp3Axr2bXGyiVLcFgLTK3_wB5wlN4DB1eIWVo8jA

=== Example 1: Begin Transaction === Transaction ID:
0c4e1f7b-a2bc-4055-a14e-0b0e61d85c84

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: map[type:Integer value:800] Bob: map[type:Integer
value:700]

=== Example 5: Rollback === New transaction:
10a6e703-e4de-4bcc-a595-7455fa98c5b0 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: map[type:Integer value:700]

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.go completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 9[0m
[32mPassed: 9[0m [32mFailed: 0[0m ✅ [32mGo direct examples complete![0m 🧪
[36mRunning Go client library examples...[0m ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted:
map[id:1atedczb3u31f46pk59IRljU_e7FQwU6ch6ERy8z83b6zGImru8AWJ7r0J6mDEFEwfqejf6EUrmFHRJaX8-ABA]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:1atedczb3u31f46pk59IRljU_e7FQwU6ch6ERy8z83b6zGImru8AWJ7r0J6mDEFEwfqejf6EUrmFHRJaX8-ABA
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:1atedczb3u31f46pk59IRljU_e7FQwU6ch6ERy8z83b6zGImru8AWJ7r0J6mDEFEwfqejf6EUrmFHRJaX8-ABA
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
exuRt59Y2dAfU7csMOy58KNKIDfx0GPxNTV-i7Hx05vGXUC6UVJZnILTwLvfO7yMflw_D1yYBu7orwIk3m7WKw

=== List Collections === Total collections: 15 Sample collections:
[schema_employees_client_py schema_users_client_py websocket_test
test_collection schema_documents_client_py]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
mcCNX0jkk8zWBL7bl5cCL-BJ5-GtlyFZ1TVif2df2iqQrsxXVF3Vui3y_1BcpYXWGaiSgL2Oid37mMgjAyGJgA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
nGxi9YqagjSJHZXAhQwlQvI0B2kGWC2t-IppSIfUUwqQ9it8E_DQseh6su18keT6SLhuRfNgyhK7Cem8rfiVig

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
0yPjCtOPNJ9WZ12wPBEMPaoN--\_6Lv_dh28cLbbiSh2v_W1MCz8xuUZS2qqbREbBZ4vi_T9OVKpESHL8p6dzBg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
UrcLm3AsyZAWVrQ4gXyPj3DzU95NFdJvxyWR6tRKo_K4d_wobUo2q88fIp7Fq_Miqxke3dykDNwLw2P6XJPYNg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
oWpN5T2vfqfqTqW59MzTvosMCC_kV2m9afaABNUB4Fq93Hk_V1wOdqrQsV8wgRuWrKjyDl0A19tWZ_kFg0r6XQ

=== Sending Chat Message === Message ID:
ltCbOFBxhAI1QwZEW_Ko752geKsWUYU6drukrqkRFoPN3wAWjPoe7Ww_JkLfztKCVE7lCcgMzxFZOF8vJObqcA

=== AI Response === I'm sorry, but the context provided does not include
specific product names or their prices. Could you provide more information or
specify what type of products you're interested in?

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:GCHzh04nLYJaGt25kdQdgIHr-T7ISKJJgpXy_6dL8Celogrhhje4Zdj1wI4SpbM5LCE81UPsdaYmI_ypsdwzLQ
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:UoqP-8bhFBz5GhaVPwzBnGktAYB07AbR97NivkjJOo3WcgqZVn4UsLZdCe97LH3PwWMqv2IlhviM7CtGGJ6mpQ
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:K5uIXkmmKeLjfgC_hwUb1OtIXuBDnBU7sLMPtWLj8OlFTZnjvUbvAtB\_\_2fV6aI4eWmQr-moUOWVqETseN6t7A
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111]

Execution Time: 2125ms

=== Token Usage === Prompt tokens: 321 Completion tokens: 34 Total tokens: 355

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
GRKGm3YelUoMrdfx0o_hpFj4N7PxezKTNrdcMsXTvFu1kSlg7otVecgeEkdSB-onud-TB671Yc6WuBsUS28daQ

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not give information about any specific products.

✓ Message 2 sent Response: Apologies, but there's no information provided in the
context about any specific product or its price. Could you please specify the
product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
W7FdHB52TWHr1YbgZdRpm-DyLX5loe7TP2G3hN3NFineUR8NhNqb0-wt4fMbuRjuJCOMgNXfw--Vv_cKdljcig
Parent:
GRKGm3YelUoMrdfx0o_hpFj4N7PxezKTNrdcMsXTvFu1kSlg7otVecgeEkdSB-onud-TB671Yc6WuBsUS28daQ

=== Listing Sessions === ✓ Found 9 sessions Session 1:
W7FdHB52TWHr1YbgZdRpm-DyLX5loe7TP2G3hN3NFineUR8NhNqb0-wt4fMbuRjuJCOMgNXfw--Vv_cKdljcig
(Untitled) Session 2:
GRKGm3YelUoMrdfx0o_hpFj4N7PxezKTNrdcMsXTvFu1kSlg7otVecgeEkdSB-onud-TB671Yc6WuBsUS28daQ
(Untitled) Session 3:
oWpN5T2vfqfqTqW59MzTvosMCC_kV2m9afaABNUB4Fq93Hk_V1wOdqrQsV8wgRuWrKjyDl0A19tWZ_kFg0r6XQ
(Untitled) Session 4:
3dKLTjbKKdWiKWJMiZSl27k-QOuXrlUsrkYfN3u6MHsPZpxafROQqHN7VqKXYIA_cDw_ok8lLtRs1RHmTZtdpg
(Untitled) Session 5:
jl_qC01XuY7UlWB31pIZDlb5ZT28fbBT40JNtj-beGEb5IMZTAfAVmzBAP5t6TIvQfE-17axFs_WESUzv_q1lA
(Untitled) Session 6:
GcJoGq8p4I9IzQJSpC3mOI9-Knp0ck9RKI25PN7C5bKsw0x4b-iHoBJ16NxTCck-JY9EZKnSBXcCVt5Uqoo2Qw
(Untitled) Session 7:
DZrN-AQV9Nn4wOTelR2KA6397oce29THpcBtxbI_iT2QOQo1yaK92LZIYuZGB_sN1pUrtH1Cg8FTgApuh7BRoA
(Untitled) Session 8:
ibO8oA4VdNKJx8MlQKtRtIuVB5WanPjnMLOrJSIIietcdTJk5n1aMoIgHOY00veFNThz80mq1tJMflxUHEmqEg
(Untitled) Session 9:
wFcIqUdFKFba7_NRR2SD1v1j68swGE6y6e1ZCGgA8_XfIMj_dJ_fXiQhjVuSR3TrnICo0OW2clU5xYAVpYChpw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
W7FdHB52TWHr1YbgZdRpm-DyLX5loe7TP2G3hN3NFineUR8NhNqb0-wt4fMbuRjuJCOMgNXfw--Vv_cKdljcig

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
PUQ4mLmnC_HL-at_Noi3NmO0d3uh9xLt6evxS7oSIy2Hmfyqy5dnWuuk2ci8J-FFh4zdgAfKPE8kXLssIYqsEQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not give specific information about any available
products. Could you please provide more details or a specific product category
you are interested in?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided doesn't contain information about
the price of ekoDB. Please provide more details or you may need to check the
official website or contact the supplier for the correct pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
-FJ4CVqdG5Gip24DhL4UpNnFgCiZxYnXepzawKad4RjbD6jL_BV5WwyqS4NVrBLHohr1LJ5o7OlQj4JlhDcCbQ
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
1. Score: 0.772
1. Score: 0.753
1. Score: 0.750

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.709
1. Score: 1.501
1. Score: 0.300

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
iTgzvugR5nqShziGkGfAkQjwHF8esGPNMXRmfE7ljoxd-tEkajYDxj38NiFB-okJLAxd0x_zlvhMcghaTbCdpw
📊 Found 60 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 60 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups map[avg_score:60 count:30 status:active]
map[avg_score:50 count:30 status:inactive] ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 13 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 60 ⏱️ Execution time: 0ms

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

added 1 package, removed 1 package, and audited 13 packages in 679ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'YE6_I58BUE42oQ4vmKd2pCB2NKXUcNgRTh1Gwnnmp3IFl6Zsnzp_QwjMS7ZQscJuCEQyZxUAa39FmdpheBT5EQ'
}

=== Find by ID === Found: { value: { type: 'Integer', value: 42 }, name: { type:
'String', value: 'Test Record' }, id:
'YE6_I58BUE42oQ4vmKd2pCB2NKXUcNgRTh1Gwnnmp3IFl6Zsnzp_QwjMS7ZQscJuCEQyZxUAa39FmdpheBT5EQ',
active: { type: 'Boolean', value: true } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { type: 'Integer', value: 100 }, id:
'YE6_I58BUE42oQ4vmKd2pCB2NKXUcNgRTh1Gwnnmp3IFl6Zsnzp_QwjMS7ZQscJuCEQyZxUAa39FmdpheBT5EQ',
name: { type: 'String', value: 'Updated Record' }, active: { type: 'Boolean',
value: true } }

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

=== KV Get === Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
0A3L_O4MeQMMxcvml5O04sUF28N04FxnM-k3DkfrbDK7UHhpGzFjquAw6b4l3wDxPiSUy9A8p_PcFQHMugyGiw

=== List Collections === Total collections: 18 Sample collections:
schema_employees_client_py,schema_products_client_go,schema_documents_client_go,schema_users_client_py,websocket_test

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
p4XCtZs_WvGPw4VfMOUy092v83a7lSLProhjcQn-bWWNalLFboK8tQURSJc2fX-wzzhH7DYqXv3i9UXpoLHhzQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
AMQM0psojfPCdfw0dVod6MQLrrvlDJPIaaHVxuGBKVzzEth6_EaTeiGzqEH04ldnIYncc4Z9G6JMxbgWlRbx9A

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
owxI-T4iab654gqUHDsQPXEUaIru_56gkg0uKlBv_k7TMRZCJQsrMRPEFeK0zSUCVHQgNPSrrBDid2f_0-cHNw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
9gdpgUR3xQkmO0Z-zgp6psEKiCnOZYFlK_wszqsHSE6wBVy7dcWfol4nM6Ml_UvhGYmWBME7rSZLH0aM2cDyHw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
Kqz2qCe8a79AOhXHQ-31fwJp6BnA-HGZgPf-2-LrTdU_swMLcEkgZ5bGcNvjG4y2FAXpgGKnLH08BMYHsGUULg

=== Sending Chat Message === Message ID:
H9TN5YmGpdH7ZSmpxKOr2ALKYiiAv8RRPahSmSitfEt6O8jK8NNp-2UWa6VWuEMhniY2rqL9O8ky9-MVEtuRmA

=== AI Response === I'm sorry, but the context provided does not contain
specific product names or their prices. Could you please provide more
information or a specific product you are interested in?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { id:
'H9ocFKmx1Xw42wwj5XcfOpmaZ8wK82GR3V0OlTYrgqFWSZZXFKrOmMfRtlQRZZmuoXKZqr9Gl53DZmSkJZkmhA',
name: { value: 'ekoDB Cloud', type: 'String' }, description: { type: 'String',
value: 'Fully managed cloud database service product' }, price: { value: 499,
type: 'Integer' } }, score: 0.1111111111111111, matched_fields: [ 'description'
] } Snippet 2: { collection: 'client_chat_basic_ts', record: { price: { value:
99, type: 'Integer' }, description: { type: 'String', value: 'A high-performance
database product with AI capabilities' }, name: { type: 'String', value: 'ekoDB'
}, id:
'K4bqcemEfud9qnSR2bDwS2AFl2ILzwXOQ09VyjdPsDtHOUjRPWxzwG4ncoY7dCNvooLpC3MLtqawWxp5mKvZcQ'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { description: { type: 'String',
value: 'Enterprise edition product with advanced features' }, name: { type:
'String', value: 'ekoDB Pro' }, id:
'Cb42y3dwe57HETIIBXAL1r7T3W3IgMtUZDHvnLVZ9QzjL23zqzSGhqXaFzV5SY3Yf0qgfiu31HQb74n1ReKw2A',
price: { type: 'Integer', value: 299 } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] }

Execution Time: 1609ms

=== Token Usage === Prompt tokens: 324 Completion tokens: 33 Total tokens: 357

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
sqvc2HxJsbU2fq5aXDrqt3Un7wySo9sTcZF4K2c5NzQDHbLBl8uYfqolBshThxYQmL5_sab5uhymTR7wZP-2Aw

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the provided
context does not contain any specific product information. Could you please
provide more details or context?

✓ Message 2 sent Response: Apologies, but there doesn't appear to be any
specific product mentioned in the context provided. Could you please provide
more information or specify the product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
RBVIDd3sAoVySsVCuZqcVlSuz0X6f_WOPoAgbj7-FLGOEcqbm42FKQleJv-tIijGFBmmx-MN3XcQKTwvZhoe6w
Parent:
sqvc2HxJsbU2fq5aXDrqt3Un7wySo9sTcZF4K2c5NzQDHbLBl8uYfqolBshThxYQmL5_sab5uhymTR7wZP-2Aw

=== Listing Sessions === ✓ Found 10 sessions Session 1:
RBVIDd3sAoVySsVCuZqcVlSuz0X6f_WOPoAgbj7-FLGOEcqbm42FKQleJv-tIijGFBmmx-MN3XcQKTwvZhoe6w
(Untitled) Session 2:
sqvc2HxJsbU2fq5aXDrqt3Un7wySo9sTcZF4K2c5NzQDHbLBl8uYfqolBshThxYQmL5_sab5uhymTR7wZP-2Aw
(Untitled) Session 3:
Kqz2qCe8a79AOhXHQ-31fwJp6BnA-HGZgPf-2-LrTdU_swMLcEkgZ5bGcNvjG4y2FAXpgGKnLH08BMYHsGUULg
(Untitled) Session 4:
-FJ4CVqdG5Gip24DhL4UpNnFgCiZxYnXepzawKad4RjbD6jL_BV5WwyqS4NVrBLHohr1LJ5o7OlQj4JlhDcCbQ
(Untitled) Session 5:
GRKGm3YelUoMrdfx0o_hpFj4N7PxezKTNrdcMsXTvFu1kSlg7otVecgeEkdSB-onud-TB671Yc6WuBsUS28daQ
(Untitled) Session 6:
oWpN5T2vfqfqTqW59MzTvosMCC_kV2m9afaABNUB4Fq93Hk_V1wOdqrQsV8wgRuWrKjyDl0A19tWZ_kFg0r6XQ
(Untitled) Session 7:
3dKLTjbKKdWiKWJMiZSl27k-QOuXrlUsrkYfN3u6MHsPZpxafROQqHN7VqKXYIA_cDw_ok8lLtRs1RHmTZtdpg
(Untitled) Session 8:
jl_qC01XuY7UlWB31pIZDlb5ZT28fbBT40JNtj-beGEb5IMZTAfAVmzBAP5t6TIvQfE-17axFs_WESUzv_q1lA
(Untitled) Session 9:
GcJoGq8p4I9IzQJSpC3mOI9-Knp0ck9RKI25PN7C5bKsw0x4b-iHoBJ16NxTCck-JY9EZKnSBXcCVt5Uqoo2Qw
(Untitled) Session 10:
DZrN-AQV9Nn4wOTelR2KA6397oce29THpcBtxbI_iT2QOQo1yaK92LZIYuZGB_sN1pUrtH1Cg8FTgApuh7BRoA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
RBVIDd3sAoVySsVCuZqcVlSuz0X6f_WOPoAgbj7-FLGOEcqbm42FKQleJv-tIijGFBmmx-MN3XcQKTwvZhoe6w

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
rIymRRJa-YuBm4lcaQmRBKNgMYhpPdUCgpnjG8D-17CfGfecy7mXNtsP6PcSioSwX7vuBUeJONqzdovazZEyvA

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain any specific information about available
products. Can you provide more details?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Apologies, but the context provided does not include information on the price of
ekoDB. I recommend checking the product's official website or contacting the
seller for the most accurate and current pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
6SohYOR4Ze4eEK_cCktreEuzA40r5l6m9U8aUqGKkdOQrTkYNpOMX-10tL1vWPEeKhiSUnxUsTHYP-c58BoF5w
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
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: bio.value, bio, title.value, title
1. Score: 26.400, Matched: bio.value, title, title.value, bio

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title.value, bio, bio.value, title
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: bio.value, title, title.value, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.749, Matched:
1. Score: 0.746, Matched:
1. Score: 0.743, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.699, Matched: title, content, title.value, content.value
1. Score: 1.498, Matched: title, content.value, content, title.value
1. Score: 0.297, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: bio, skills.value, skills, bio.value

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
8hpqrr-WV6c966VRPy9QCkHc6_qTXfLqlfJWU1p89po2plAYai8ztG8tPi0L81VOKcQCC4WirY8NHOg2lFJA0Q
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
mw7eqUL82RaQp3RN2WF99HhN7z7m_JawPyp4nk1wGs6RuYi2a6ivg1mzmM7enN5mReQbgTjtUuMWfcyipVYbyw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
apgovIXduWcTD44f0O3u7t1w4kXWnA07uqOE3Sv2ZZsqYTIAVNdFmy9Z2ngxeWk9Y-P-H9Dl95xBF2J8DyZgPA
📊 Statistics: 2 groups {"count":5,"avg_score":60,"status":"active"}
{"avg_score":50,"status":"inactive","count":5}

📝 Example 4: Script Management

📋 Total scripts: 17 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
QSSB8uOk1bYa6CitxfUhRnaLaRRLG1QlfnbW-gKncl99swMndeR0Psi7X8vef-qBuciiMXW8jp-2B8P0XmnjPA
📊 Found 2 product groups
{"count":3,"category":"Electronics","avg_price":575.6666666666666}
{"count":2,"avg_price":474,"category":"Furniture"} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":2,"category":"Furniture"}
{"count":3,"category":"Electronics"} ⏱️ Execution time: 0ms

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

added 1 package, removed 1 package, and audited 9 packages in 632ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:40725) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 9 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:40728) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'JtZ7g8fr8svPiESjLJyPhnUtjzAjNkabqDcIIlXKwG1tujth842KwiHp4wCCQW_y1_MkGMqDX6TgQoRBqnPNWQ'
}

=== Find by ID === Found: { active: { type: 'Boolean', value: true }, id:
'JtZ7g8fr8svPiESjLJyPhnUtjzAjNkabqDcIIlXKwG1tujth842KwiHp4wCCQW_y1_MkGMqDX6TgQoRBqnPNWQ',
value: { type: 'Integer', value: 42 }, name: { value: 'Test Record', type:
'String' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { type: 'Integer', value: 100 }, name:
{ value: 'Updated Record', type: 'String' }, id:
'JtZ7g8fr8svPiESjLJyPhnUtjzAjNkabqDcIIlXKwG1tujth842KwiHp4wCCQW_y1_MkGMqDX6TgQoRBqnPNWQ',
active: { type: 'Boolean', value: true } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:40732) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
jUSrcPm9ImFIqdf3jg2qKZQ8YfiwECjhL8f98mPXdBG4VZRoKZqO6zolEPIcBoe_qgBDOWJ2cYyRDRdcV2UHww

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"gxyJOI5aO4nhazTz8ifgYCc10ASLp66ugU-6T1lqQ8eZiBqUWG9WA3Sc0Dkd8Pjpv60sdsWUbp8WImMAo_lVzw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"tHnRpXS9iyrx1UuHjVIcjmAukoOpDqcb8Vibum5lJBrzq4lg2KxYmcaaf0nF2VPwRhNNDkl_gkP3b4Tr_E6DKw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"jUSrcPm9ImFIqdf3jg2qKZQ8YfiwECjhL8f98mPXdBG4VZRoKZqO6zolEPIcBoe_qgBDOWJ2cYyRDRdcV2UHww",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"xHB7dOzhn-TURJ68KMxUhViza0AENh17KfL1vjmYJsMyjVX49kQaqRZqLD_AbCoe-BrfQx-JZokRamE6b0Vm3Q",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:40738) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:40741) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-09T06:41:25.223Z',
username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:40744) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
wLoq2469adRvTzc3rAX0e7FY6_H_hIo3OH3E06gfyPbQ2pCmmivHXvuMO4oluwZeqRnsPZ4WM14sCJ14HQBMLA

=== List Collections === Total collections: 23 Sample collections: [
'schema_employees_client_py', 'demo_collection', 'schema_products_client_go',
'schema_documents_client_go', 'schema_users_client_py' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:40747) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
pAf6ipb7ujRsoX0B5GzSPZhTlwFtayR7QLnd0QS-hpBDNXoyQUPuPlzbjm_NQT2wDvr8fqTjROXOMOEcjbRX9g

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:40750) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
orOPJRbUqa7WM7lst27qgVUJ9-MNT07-ZcYyBuQqwDLjIZlItYOSgFMHTsTCsR8TjUUzt2UcrvzbCp52BKOBIQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
bWFHOO5NntHMBhbk0l-Xw7H-nxJp5iZF4MKpQBnWfWeb7KPkLgQgjaiFYRVHbi7HgetFxHN6yH_pFebxkflsMA
📊 Found 20 active users

📝 Example 2: Complex Filter with Multiple Conditions

✅ Script saved:
j9goPjtYKvYjdjML_6Giv51A049R_lhKqR6tk5btZI-3z79BqL2Q_uRuo2NGTtjU4cSlqHiT6yQ4UKsaDAoajA
📊 Found 6 users (status=active, score>50, sorted by score)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
4w_YNtgsjLXeJhqH3RRIqdnPj5UVkJyATFFKVmpQSLZ4yhG1q7gAcbQE7ReseWminCr3c7FdbPHmkQxg2ycZug
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":50,"max_score":90,"status":"inactive","count":10}
{"status":"active","count":10,"avg_score":60,"max_score":100}

📝 Example 4: Function Management

📋 Total scripts: 19 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.js completed successfully[0m
[34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js
===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
CM6ftPTv4VGx2R4AdS2V7zgZVvWBQOTU1exoGAfJAEu_QKDDXfLEyXWh6oyYmC6Ejc8r7KYePNC6-PTCnIRkEQ
Created Bob: $500 - ID:
\_X72N6rjdbmbOthsel8QOSdKXQ-5e2XZMBxVRLGlTfY8g057h2-m8qVraa-cquZdJd9SeiRuTnfTxzmsPc23RQ

=== Example 1: Begin Transaction === Transaction ID:
487b17da-c364-499b-93ba-2117b8763c6f

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"value":700,"type":"Integer"}

=== Example 5: Rollback === New transaction:
9e16d49e-4f05-4c99-bb20-d06f6b76c979 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 9[0m
[32mPassed: 9[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 837ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 647ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'FeMH7q9_KGxyHPTXbVIvI695kPaf16KtcvN-KsRie6LvNZKq3dPnae5YDzBVzEvciqFt-AVWTR1aDqeGrn07Tw'
}

=== Find by ID === Found: { id:
'FeMH7q9_KGxyHPTXbVIvI695kPaf16KtcvN-KsRie6LvNZKq3dPnae5YDzBVzEvciqFt-AVWTR1aDqeGrn07Tw',
value: { type: 'Integer', value: 42 }, name: { type: 'String', value: 'Test
Record' }, active: { value: true, type: 'Boolean' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { value: true, type: 'Boolean' },
name: { value: 'Updated Record', type: 'String' }, value: { type: 'Integer',
value: 100 }, id:
'FeMH7q9_KGxyHPTXbVIvI695kPaf16KtcvN-KsRie6LvNZKq3dPnae5YDzBVzEvciqFt-AVWTR1aDqeGrn07Tw'
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

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
zEIqyJJtGcPS6voPIdbnX9PNYhc1fnMQOvpcleotn8yj3zfvHfjaKbcMbtUr_8U6kStKy6g3vai1SO-4X3VkWg

=== List Collections === Total collections: 23 Sample collections:
schema_employees_client_py,schema_products_client_go,schema_documents_client_go,schema_users_client_py,websocket_test

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
9hVVM7tElARgkXegmPJveQ_wcdHfDiZa-t5-vPhfujI-ix91WNhQjZDzGhy_72Dqt7VNTqZpnz1-VbDkBSX0sg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
-wPdwTw4_YFkenVZLPoZx7hOhmHIR6rc_66UIVMAYP8C2f31EOArptLAbDikn_ns7zwzBALv6iAmarl2ejzpLg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
1naraKvXyg4FfbMBtMzEXB1afxNj5g7RFOKF0HcijS6iD5SKkltlqEtbKOaRM3-ALGB_PVIPCgz6CS3ue5j3LQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
wzMUuw8872R7vzQ0Fi1V_eS3-EBz6jmNQvelEodtGnvgabmWz6GkYlXh_Ju28DXspzdV3eU1BO8fAvVZe7Qtow

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
fUswEU3lAdFhqxiMBSHjfVxgLLXTBBZDjxZIHH2C0v3HlRoMg18ss-vBwGlaRl8fzpuf1GRWBfkXGskOJG03Hg
📊 Found 30 active users

📝 Example 2: Complex Filter with Multiple Conditions

✅ Script saved:
aRK3OQJD07eUFHT6VmjTILw5TGdnn21iB_0L6XkvupCsHHdeaisaVuwVHGpctItZGkkBiqZ528dh1G3bz0bXIw
📊 Found 9 users (status=active, score>50, sorted by score)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
NuQ8McDGqYz1cFQOqpfHZJLQg9ts57G9r_nSl_vrd2J3POK1O_mdAfhQ_5A1e-nV-wpJElV00M24ff2NfBAotw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"status":"active","max_score":100,"avg_score":60,"count":15}
{"status":"inactive","count":15,"max_score":90,"avg_score":50}

📝 Example 4: Function Management

📋 Total scripts: 21 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
iPyPp-EcKkJb1EjubfS2-DTGgUf4Nu0zTCbiIU5dul19-w3U9qvCdiSQWhAecM6SbvjYujU55PfjYn6n3Vdiow
📊 Found 40 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 40 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":20,"status":"inactive","avg_score":50}
{"status":"active","avg_score":60,"count":20} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 23 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 40 ⏱️ Execution time: 0ms

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

✅ Script saved 📊 Category analysis (2 categories): Furniture: Products: 4 |
Stock: 43 | Avg Rating: ⭐4.26 Electronics: Products: 6 | Stock: 232 | Avg
Rating: ⭐4.52

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Bookshelf - $149 (⭐4.1)
2.  Standing Desk - $599 (⭐4.7)
3.  Wireless Mouse - $29 (⭐4.5) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All advanced script examples completed! 🚀 ekoDB CRUD Scripts Example

📋 Setting up test data... ✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved 📊 Found 10 users ⏱️ Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved 📊 User counts by status: inactive: 3 users active: 7 users ⏱️
Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved 📊 Average scores by role: admin: 20.0 (3 users) user: 70.0 (7
users) ⏱️ Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved 📊 Users (showing first 5 of 10):

1.  User 7 (user7@example.com) - Score: 70
2.  User 2 (user2@example.com) - Score: 20
3.  User 5 (user5@example.com) - Score: 50
4.  User 9 (user9@example.com) - Score: 90
5.  User 3 (user3@example.com) - Score: 30 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Getting Started with ekoDB (Database)
2.  Vector Databases Explained (Database)
3.  Introduction to Machine Learning (AI)
4.  Database Design Principles (Database)
5.  Natural Language Processing (AI) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: Database: 3 documents AI: 2 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Getting Started with ekoDB
2.  Vector Databases Explained
3.  Introduction to Machine Learning
4.  Database Design Principles
5.  Natural Language Processing ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Getting Started with ekoDB
2.  Vector Databases Explained
3.  Introduction to Machine Learning ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Getting Started with ekoDB (Database)
2.  Vector Databases Explained (Database) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Scalability: Vector databases can handle large amounts of data and scale up
   efficiently.
2. Precision: They maintain high precision and quality of data, without any loss
   of details.
3. Flexibility: They allow for on-the-fly alterations and manipulation of data.
4. Compatibility: Vector databases are compatible with various data models and
   can integrate different types of spatial data.
5. Complex Analysis: They are suitable for complex spatial queries and analysis.
6. Memory Efficiency: They are more memory efficient for representing sparse
   data. ⏱️ Execution time: 0ms

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

BUILD SUCCESSFUL in 3s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
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
Record(fields={id=StringValue(value=wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A)})
User ID:
wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A

=== Read === ✓ Found user by ID:
Record(fields={name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
id=StringValue(value=wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)})})

=== Update === ✓ Updated user:
Record(fields={email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
city=ObjectValue(value={value=StringValue(value=San Francisco),
type=StringValue(value=String)}),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
id=StringValue(value=wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A)})

=== Query === ✓ Found 1 users matching query

- Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  city=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=San Francisco)}),
  name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  age=ObjectValue(value={value=IntegerValue(value=29),
  type=StringValue(value=Integer)}),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)}),
  id=StringValue(value=wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A)})

=== Delete === ✓ Deleted user with ID:
wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

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
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
oY4ckp466jvlgn3SdAhPOpHGNEv0a2zp2iUgo4voDC0aQQlt-RrYCwv7KPprLpy9datwSkIyzu4zHuFKTgFyww,
BWQsxHLvFLAfXijVh9d3-TCVXBjy_99MrWvIpQpwmMvraVuWpYYaZW4hpibtIdwvRhaVQaER4oW43QEteTBIFQ,
cXZvviK-gD3R1c8yji22cZHkVGo2O7zUa99BEaEbXLmni6jbTlomc_y5PTqQQbkzJoG5n34u8h2ilvjRh4c1sQ...

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

- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=95)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=92)})
- Score: ObjectValue(value={value=IntegerValue(value=88),
  type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Charlie)})
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
{"role":"admin","name":"Alice","email":"alice@example.com"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1765262530624,"user_id":"123"} (Will expire in 10 seconds)

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

=== List Collections === ✓ Found 21 collections

- schema_employees_client_py
- schema_products_client_go
- schema_documents_client_go
- schema_users_client_py
- websocket_test

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-09T06:42:14.850313Z","last_modified":"2025-12-09T06:42:14.850314Z","bypass_ripple":false,"primary_key_alias":"id"}

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
tNMU0os4OS8sZ1jvqwmROtFxGdTv63int8L-2qUCKE0uDlXLvmbahzYR8b-XZpqYPD4N_m_hpREIVZYQslfisQ

=== Verify Document Exists === ✓ Document found: [id, created_at, session_id,
user_id, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
\_XSWYI21welVEqFrBkfLTw80tfQ2V3syJKYmYfe4_mAgpkn-aSOHXSD7c0FJrYqPqzVzkPyrCkgVHYSt4WRB-A

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [id,
cache_key, ttl_duration, value]

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
{"payload":{"data":[{"id":"5nL1q-gMDRhlwSRsjzOBVKyu_lsCHdB7BnTcfBwCNYoPmwU-Wbk5wV-TrkuP0_AOnu3f8R3458GppwdVTGiQxQ","name":{"type":"String","value":"Test
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
StringValue(value=MS7WdKI6-WWZU9qWVxxF8lm82VLJUt2kp-IRJJUo1uv6kkCuGfTm3npeilXIQgbHTFjXEu8WrE_1x-iPOqVs0g)
✓ Inserted user 2:
StringValue(value=TpldEhD5GQeqzuBoIByQzs6fx-OKOniFzuC2nkibe28EOghpFQQlDxwaAGNPobV2zm0uNbPWgTPDU1J-6DAS2A)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_employees_client_py, kotlin_schema_example, schema_products_client_go,
schema_documents_client_go, schema_users_client_py]

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
{"results":[{"record":{"id":"IaGAXRfQJIm9Kpq0GJVTY37QqGsqIX1FHoEkV5KwX25mxCroFKtPXrzNOMWJvfjSMdQn7sdJQyWqF43dIsna2g","tags":"programming,rust,tutorial","views":252,"title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices."},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"description":"Build
modern web applications using JavaScript, React, and
Node.js.","tags":"programming,javascript,web","id":"nxlEb2faJO50NiYP8vDCSMz5W-jFhGhQUnISOy-5X2WGsugKzWDADWoxGaw2AjHq1LAlFCJalwPkQE9N0yrxKg","views":432,"title":"JavaScript
Web
Development"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"id":"BCMkr7V3HSMqtRyz0ADT5vrNWvHA3m_IwdtaiHKRBvoRaWHmI-3BVuAKTpykDL_emQ0btKRAR4yCztwPGviT1w","title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization.","tags":"programming,python,data-science","views":552},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"description":"Introduction to machine learning
algorithms and neural networks.","views":923,"title":"Machine Learning
Basics","tags":"ai,machine-learning,python","id":"4e7kiO4u2mPAzIu4G330ZacrfLa-jlzcTjZk3a82TwWGYPz9yPfLQHPL3rtdqsJ86X_4XvuF7VyxAHp7apAkHQ"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science","views":552,"id":"BCMkr7V3HSMqtRyz0ADT5vrNWvHA3m_IwdtaiHKRBvoRaWHmI-3BVuAKTpykDL_emQ0btKRAR4yCztwPGviT1w","tags":"programming,python,data-science"},"score":1.0,"matched_fields":["description"]},{"record":{"description":"Learn
database design principles, normalization, and query
optimization.","id":"3YnzEStGg6wb27G6LS5f6-bFHqgsGDPC2QNTobk9zgTXCOat3Mo7G1pIAyw5igh8_iC-yXgkiDk9UVd49p4yAw","title":"Database
Design","tags":"database,design,sql","views":510},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn
Rust programming language with hands-on examples and best
practices.","id":"IaGAXRfQJIm9Kpq0GJVTY37QqGsqIX1FHoEkV5KwX25mxCroFKtPXrzNOMWJvfjSMdQn7sdJQyWqF43dIsna2g","views":252,"title":"Rust
Programming","tags":"programming,rust,tutorial"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=2hwxOQ6emv8STEp5L7PnRxmlTC5i_KyLeOreUDqc7pU2zD5LyVH4lDnMlEXPS8xfiOZ6N1crHNxkiuTChHxUzg)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1765262555826},"id":"2hwxOQ6emv8STEp5L7PnRxmlTC5i_KyLeOreUDqc7pU2zD5LyVH4lDnMlEXPS8xfiOZ6N1crHNxkiuTChHxUzg","name":{"type":"String","value":"WebSocket
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

- Alice Johnson: Engineering
- Bob Smith: Sales

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

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
w6xFbh5G1TSMio4O_z1G-tTj0fHQMwFHsXj0ySMSnj1Bfys22ZTA0ufV0oBb62rmPN7CBBn9VYPIm76GsRj-Yg

=== Sending Chat Message === ✓ Chat response: Message ID:
"GHKk2Xk-r_OVpzxiALra-tQz3h_Afw5JGAfYy_jZQpchOY2Cnlv928JEXoun4gLO_8KZh5rfyz4Axha7QfjZcQ"
Responses: ["I'm sorry for the inconvenience, but the context provided does not
contain any specific information about ekoDB or its features. Could you please
provide more details or a different context? I'd be glad to assist you with the
right information."]

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
jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided does not include specific information about available products.
Could you please provide more details or context?"]

✓ Message 2 sent Responses: ["I'm sorry, but there's no information provided
about the product or its price in the context. Could you please provide more
details?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"1YDlwG54PeucCUd0L-vUlSmOOCg3E6OazJ2nzMmlfFU6eR1Qfq1F8MugKQlcj74xUzwqJb_yW3FgBLSnu3Cj5w","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T06:42:53.891381Z"},"id":"gtQS2zX7g0ZgV16E4mAp1M145F3kLCXuJ-oJ0_OYopijSmsq_S3pW8z1Me8G96CTMZSfnz_KjZnXYoH5tWSzKQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":158,"total_tokens":184}},"updated_at":{"type":"DateTime","value":"2025-12-09T06:42:53.891381Z"}},{"chat_id":{"type":"String","value":"jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg"},"content":{"type":"String","value":"I'm
sorry, but the context provided does not include specific information about
available products. Could you please provide more details or
context?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"1YDlwG54PeucCUd0L-vUlSmOOCg3E6OazJ2nzMmlfFU6eR1Qfq1F8MugKQlcj74xUzwqJb_yW3FgBLSnu3Cj5w","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T06:42:53.894886Z"},"id":"UzHq2L5Yk6Dr5ewZw1USXTog_uQw7t89FDewff2ojeh6gZO3UL55eHAEMK6n4HQCkLHG4BtQe869Qfmq0N6gow","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":158,"total_tokens":184}},"updated_at":{"type":"DateTime","value":"2025-12-09T06:42:53.894886Z"}},{"chat_id":{"type":"String","value":"jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-09T06:42:55.321462Z"},"id":"kbozPGUxKkaWngfoTTICWRjZexblR7-Bdg1g4YgtBkhwPyKpm1Pa301V_9Ovg6H3LC6lKrEQ19f0C7sXjW-YEw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":70,"total_tokens":97}},"updated_at":{"type":"DateTime","value":"2025-12-09T06:42:55.321462Z"}},{"chat_id":{"type":"String","value":"jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg"},"content":{"type":"String","value":"I'm
sorry, but there's no information provided about the product or its price in the
context. Could you please provide more
details?"},"created_at":{"type":"DateTime","value":"2025-12-09T06:42:55.324718Z"},"id":"3Q3aFWhjcETN6J3nVZT1tMgn3e5v7GtVr_HkWvcIAh0OK7aIiYVkSfDldYPlki0MC9LVa3vPV2hRUSu0ojyv3Q","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":70,"total_tokens":97}},"updated_at":{"type":"DateTime","value":"2025-12-09T06:42:55.324718Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
p43fU7mEirBQB9li20PnMZeERlttLSmoKYKA_3cw2rchXTKGEP95esGkTYwi0FUHUNnt2m2J4eEQjkeXE463tg

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 6s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
vfVFT2wW7LSP7omm3gIzyJty926EIGVyM-2CWm4D3xFb8l-eV5RoJTQ6Vs6K0XeHbh755zllWdQPykNR2JwnGw

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
context provided does not contain information about any specific products. Could
you please provide more details or a different context?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided does not include any information about available
products. Can you provide more details or try again?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
aX4zzkC5xiItRHpGT_ASx3o-fwJ-FOxjLGuJwqbxSarrppSehvPYiz7zXeKJEVv6FbDYn0Qb25pJTBgloHTh9g

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
6rMqCJMox4UUUAu3dhXPg5Ismaf_VpXwDTpHDv_nAQca-wT_9ogvJUmZE71YzBgmcFY6zF86VVHFoCJf3jqCFQ
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"avg_score":50.0,"status":"inactive","count":5}
{"status":"active","count":5,"avg_score":60.0} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 27 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
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
[32mKotlin client examples complete![0m 🟣 [33mKotlin Transactions...[0m
/bin/sh: kotlinc: command not found Kotlin example (requires kotlinc installed)
✅ [32mAll Kotlin integration tests complete![0m
