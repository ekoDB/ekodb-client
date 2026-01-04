make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
   Compiling ekodb_client v0.7.1 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.36s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("Ys2dmRZWtTMe0l2ReNH2FrULWzf90PYWNsK9dP9RqAmbNlxe05sNt_FoB13dWTM2LA6BAFH0mweUIzF3suXG2A")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Ys2dmRZWtTMe0l2ReNH2FrULWzf90PYWNsK9dP9RqAmbNlxe05sNt_FoB13dWTM2LA6BAFH0mweUIzF3suXG2A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Ys2dmRZWtTMe0l2ReNH2FrULWzf90PYWNsK9dP9RqAmbNlxe05sNt_FoB13dWTM2LA6BAFH0mweUIzF3suXG2A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Ys2dmRZWtTMe0l2ReNH2FrULWzf90PYWNsK9dP9RqAmbNlxe05sNt_FoB13dWTM2LA6BAFH0mweUIzF3suXG2A"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "LBF7Gequi0v04qbT1iANWTN-FRO0bYDKvukFbxdX9CO7wYKIchKcISe0P-UObFrpY2NNO2nXY-XcPKH9NW1CkA"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "LBF7Gequi0v04qbT1iANWTN-FRO0bYDKvukFbxdX9CO7wYKIchKcISe0P-UObFrpY2NNO2nXY-XcPKH9NW1CkA",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "type": "Success"
}
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/examples/batch_operations`
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/examples/kv_operations`
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"userId": Number(123), "username": String("john_doe")}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: Object {"name": String("Product 1"), "price": Number(29.99)}
cache:product:2: Object {"name": String("Product 2"), "price": Number(39.989999999999995)}
cache:product:3: Object {"name": String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "QyeHSH9UzS_oUE3aq2Cf9nxkJcyQxYLhwsxV9WMfp2j3xrn4HT1cBnjO44WidUKlb3VQYL2DSkNPWmfS-NVp7A"

=== List Collections ===
Total collections: 4
Sample collections: ["websocket_test", "test_collection", "batch_users", "demo_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "0IlTsVbutQW914kYBjmqSV_PL2yZvJqeQ0wOtmCbPvn_Qwe3nlreqfOWpVU-Ob3hDveKLb9ruNf98B1VvGSk7A"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "qNh0jqif6Q3gKokN56aMYB_NVAnFpguT25vXR_V358aRC3i-Gu634e8SL7bfM3IrHdS21AeeQAlafZOsI0ztow"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "U2NIV9wo83c-JtmjxbdrHeskmjqSgNwxnxk83qeCx0xvTy_B_DR8gKEEvSEYoeC6gdg1Cqmm15COg7w8XUwyhQ"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("h4M-Lj9i-Q9y8QviXLlQCAzf4myMsNLP2Yw_OSuuib6uoz_T3XtzlvTQt8voHUElEeETv-noUebYjXcUR6olRA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: _0e-Zh3ghrcTkLTAoHNV8jY4gt7fUcSVYw7IoSZDabH6W0f2N2OPbLr78-NW9M20EKOBwqGwKOgGxv2R5JK5JQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "rI70rCE1-wKosyn8u57cBU8eiShjLdfl3UTmkVi_bhAWoX68r9VNyhDt6E_5pMadgILyvyZfOsYscmIlFhqaog"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: cqXcxBJ3LYcaFkr85R53G3xiTVRkFSbOi2eZ6rO94WLeYEiveM4XsU5Jhvf2zk7tQeCXwgHdfq9DF1U_dC3ubQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: t0VeD_ZymAkrJ6eh1-rDfdpH6TUsxhuAVSweP7JapRU-xqxnyCzkRLPPaDYrHxufTgRtRTPzWSb5KfjRnB5vKQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: jH95uqiJO1wmOHfWE46STSQ22MT_b9_SxqXrPoiWhYbhfgoFlkn7Lw2wrfbuo-sRSJCeVbzfsklBDQp14773bA
Created Bob: $500 - ID: vkbtIKTlv7hQzEJIw3pzy16znDh7pmnhv29wxd1bKWRGPHSlTv6wx3r_rDHTL9u4_uyQWEdiXYyo6mS1mTMf6w

=== Example 1: Begin Transaction ===
Transaction ID: f8aeaa8d-c0f2-4497-98fa-0a93718ec5d0

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: ccc91aee-33b6-4748-a520-fb89c8e2565b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.87s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("LZnyRD8xOZpX5uu-H2z818vWEAEvjHnlmT7rkTWRWD2u86GTwjYLJML7_RMXtHEhHloFPsoiNWFUZYzfK8QlfQ")} }

=== Find by ID ===
Found: Record { fields: {"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "id": String("LZnyRD8xOZpX5uu-H2z818vWEAEvjHnlmT7rkTWRWD2u86GTwjYLJML7_RMXtHEhHloFPsoiNWFUZYzfK8QlfQ"), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-04T19:36:10.709063Z")})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-04T19:36:10.709063Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "id": String("LZnyRD8xOZpX5uu-H2z818vWEAEvjHnlmT7rkTWRWD2u86GTwjYLJML7_RMXtHEhHloFPsoiNWFUZYzfK8QlfQ"), "price": Object({"value": Float(99.99), "type": String("Float")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "name": Object({"type": String("String"), "value": String("Test Record")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"value": String("2026-01-04T19:36:10.709063Z"), "type": String("DateTime")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})} }]

=== Update Document ===
Updated: Record { fields: {"value": Object({"value": Integer(100), "type": String("Integer")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-04T19:36:10.709063Z")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("LZnyRD8xOZpX5uu-H2z818vWEAEvjHnlmT7rkTWRWD2u86GTwjYLJML7_RMXtHEhHloFPsoiNWFUZYzfK8QlfQ"), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "price": Object({"type": String("Float"), "value": Float(99.99)})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/client_batch_operations`
✓ Client created

=== Batch Insert (via multiple inserts) ===
✓ Inserted 5 records
✓ Verified: Found 5 total records in collection

=== Update Records ===
✓ Updated 3 records

=== Delete Records ===
✓ Deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"userId": Number(123), "username": String("john_doe")}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: Object {"name": String("Product 1"), "price": Number(29.99)}
cache:product:2: Object {"name": String("Product 2"), "price": Number(39.989999999999995)}
cache:product:3: Object {"name": String("Product 3"), "price": Number(49.989999999999995)}

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "aIhRp6TNQV24TZ4dlmn5qyW7O9pPmSe9HsSS4BrnSJF4T-E9hAPQGaX-AeQmieNdzziAMid3USNAfExZKa3PCg"

=== List Collections ===
Total collections: 9
Sample collections: ["ttl_cache", "test_accounts", "client_collection_management_rust", "ws_ttl_test", "websocket_test"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "G_GtuB2ZZH2Km8oiXNU7F3JK8Q7nkMvU8R1jfN_hylftF47s7CXp9huRCiHvwkfAvjf4ej_j685MBnqMl-xHGg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("ve0ouGMDcP3q1eNaicy5iOvUkGksr4owtb9nn9koyr915FR8wIh0YHIpF8yk_J-322rT080AM85nrH4P7-Zv8A"))

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: SFKzHIyqodJgsRk2GKWHB0FnNKXYi5qZFEMtEKdykfL6i4G7Fk_YySdDECWSrxo-N651-5aUexKAkw8UEdFX0g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("KUHwSwBgCTieKvBM03RtRkq0DF1G287yvYMuoj1FPo7jKW71cLJoH3ngB9yk6zyJq-3CZTAulSUqv4SB3rj15g"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(String("Rust Programming"))
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))
  3. Score: 6.6000 - Some(String("Python for Data Science"))

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(String("Rust Programming"))
  2. Score: 0.5500 - Some(String("JavaScript Web Development"))
  3. Score: 0.5500 - Some(String("Python for Data Science"))

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(String("Machine Learning Basics"))
     Matched: ["title", "title.value", "description", "description.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description", "description.value"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(String("Python for Data Science"))
  2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 19.8000 - Some(String("Rust Programming"))
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("LNt7u0uPnzI93ZfgxhIJMWOe-sUx0ko68tdOnlZwbNIliQa9UWD66pcI77bJp3x6OW5yaFDAdJK3Wes9ooO6VA"))
✓ Inserted user 2: Some(String("sQYuhEF0q058HLkVzDTGWkpOzetHmJ07QxrDp1jry6QgPKXWlLOHjkBrlaOSm8Haa3dGNxsgopkI6OCdl0ewng"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - age: Integer
  - title: String
    (required)
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["ttl_cache", "schema_client_rust", "test_accounts", "ws_ttl_test", "websocket_test"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: o1-Gof096ND-RPEft85EWug_1lfr_pJ9PK5P-0h1_mparol8lKHJxwGPXtTCJTuOEkqpagS5eun4RH9azwSU_A

=== Sending Chat Message ===
Message ID: 8wnmJowTa_bjqtLo47D5MpMBaMFM3bRT8PNGVGOFQlPDHTxghUPO6QHf4O1hFiaIe5RxjjjT7KUO9mbXETEuHA

=== AI Response ===
Response 1: ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. It has unique features like the AI Chat Integration, which allows you to query your database using natural language and receive AI-powered responses with relevant context. Moreover, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval. These features make it a powerful tool for managing and accessing data.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("9qdfl__IluO3CimRIBkHeD6JEbAz02CinAA_tkvF7j9LjFboKUzYlMEkaypgQtkAVpwpTp4v98TQM89NpE4nsg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("b7ONrauBCrkWDLkiSrJRGDSmIP8GQ-36Qk9jFWdzQT1upHlOb8WM-MYDk9gSajN5zUMbcK63C4OwMi_wDgGGxA"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("l2e10LdqupB-Iok474Fwix3n5eLImBrFWhFDtPLZ6LXKrBu7HRXfBxGjDB9lAhvjqF9F62hZmZd3o6dVIKpdCA"), "title": String("Search Features")}

Execution Time: 3161ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Z5AKjIPI2yAcnUe0vBJQ876BWLJQ6XJWcYAMhqakwaXAEkhgRaePkXsgJTBf8TLtVPGSn7RWgBvBAq_zwKXWwA
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is "ekoDB". This is a high-performance database product that comes with AI capabilities. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: qo9HrHE07OyydXN_IfuL6vKok27B2Yfq-8pm4u856Eu1ZRJCx8UGAuJcX3_2gh3-Vv8mEwaPnD_D_7mOMu-T6Q
  Parent: Z5AKjIPI2yAcnUe0vBJQ876BWLJQ6XJWcYAMhqakwaXAEkhgRaePkXsgJTBf8TLtVPGSn7RWgBvBAq_zwKXWwA

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: qo9HrHE07OyydXN_IfuL6vKok27B2Yfq-8pm4u856Eu1ZRJCx8UGAuJcX3_2gh3-Vv8mEwaPnD_D_7mOMu-T6Q (Untitled)
  Session 2: Z5AKjIPI2yAcnUe0vBJQ876BWLJQ6XJWcYAMhqakwaXAEkhgRaePkXsgJTBf8TLtVPGSn7RWgBvBAq_zwKXWwA (Untitled)
  Session 3: o1-Gof096ND-RPEft85EWug_1lfr_pJ9PK5P-0h1_mparol8lKHJxwGPXtTCJTuOEkqpagS5eun4RH9azwSU_A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: qo9HrHE07OyydXN_IfuL6vKok27B2Yfq-8pm4u856Eu1ZRJCx8UGAuJcX3_2gh3-Vv8mEwaPnD_D_7mOMu-T6Q

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.81s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: bc0vuXekplg3ieID4vl4Ol5DvV54J3mDT6Ll0aJ-_6nPbe-XTqNQpfwXboU0x7qtPvMTp0ndcY0O52bOX5dUqw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, ekoDB is available. It is a high-performance database product offered at a price of $99.

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: QRhmpERBtntusi1QZ2IH-CYZz8YqvGsUxP5rSLCZFUJV6k9Fh9IzpoKcyUkay5qYqXNuTe4_QsJe4LM3bnFRfg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 4

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 3

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: gZOoCxboe7L_hApKaSbK0dG9SoiKm4-qCNwaWnc43edtdDK-ZlXcu5qIHrsiMaer_tMWMuxc1TOlYWrPMHUFmg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: puuc2DUxVyAHFAtVibePfsZF1aZLjjgThldT4h1BorPn7TgaefuyBZZWERipdFXXzV_jsjVh-rV7Sgf0Z2a3SA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: yWmdELsrdrtAmjBrY-ky7UwBmhE1_z8jHnmq5D2rVIonXMSAizskByu5aZMqNDS9qp4Spl2jbN-_svLDzMdgdg
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: BoUxuRzduKftS7b71IVoZwLCm_pnTiBIfrQerSlZuBaEU19hAo2Texk7Zy-1nfXkQFjELiNlF-8D4DfK9-Pvrg
Created Bob: $500 - ID: 2GRy-mwRT1PorKe9A7dRxX0CHZHaedychr-Ve3o7OGs2sEHjm0HAs-52TBBmcT05iXWjkcxJ_wCU2ma3xpFpZA

=== Example 1: Begin Transaction ===
Transaction ID: c0e9839b-2d30-4082-9d9f-4913c20c0c28

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: cdfa6307-fc4d-4c8d-8216-18172f0d4911
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mRust client examples complete![0m
