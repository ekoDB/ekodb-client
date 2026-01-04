make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'Qw7V8C3puicAhL859VODekhDfuGDe4iYaagz5690hIgi35MHrGkC_sIT4slUbkCreI4a3UZMSbedxxj1oj5HhQ'}

=== Find by ID ===
Found: {'id': 'Qw7V8C3puicAhL859VODekhDfuGDe4iYaagz5690hIgi35MHrGkC_sIT4slUbkCreI4a3UZMSbedxxj1oj5HhQ', 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'Qw7V8C3puicAhL859VODekhDfuGDe4iYaagz5690hIgi35MHrGkC_sIT4slUbkCreI4a3UZMSbedxxj1oj5HhQ', 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 9jhiJD-vX2ICyYsmVzgVmtlyo9yTqcOWZTdsd5jfBvDaYIiuAKy4_bD3ASkcLX_E2gV2whXZrWIPWIfwLrXQAQ

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
        "id": "9jhiJD-vX2ICyYsmVzgVmtlyo9yTqcOWZTdsd5jfBvDaYIiuAKy4_bD3ASkcLX_E2gV2whXZrWIPWIfwLrXQAQ",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
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
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="{'value': 'active', 'type': 'String'}"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: jlpU3PK9S-LrSSpzG2p70P9NipbkyxQCQnWg0UtNnVP5DqmSC_h4htCyHQxkvxecdEeLMmAB8Y9yj1UlCOjOlg

=== List Collections ===
Total collections: 11
Sample collections: ['ttl_cache', 'test_accounts', 'ws_ttl_test', 'websocket_test', 'chat_messages__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: yRYLo9XNwSTAsFA-NASApGBGuTjfZw9wpFFE_KaVHwC7PO3UCVq6h-iVVcF7l7Kr_PQ8Gy7R2k-JvyWEwB1TaQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 2istEHazK4dUehzRj8Gq-VOsA65e9U2xmuzuGAZPkPnAoic0RheKhjByKmktu77c1eOw749-aj19zekLPPRohQ

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: kWbmlTMV66UXAgSqP3-Danmit2RBVG_b_XGVEceodiqAq5SI5CjUmMJhaM9XZmadtiIXf_xfEon7DkMjVObXCg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: -c23XrEgV1mhlw-xDD95pagbMK6zqKaj0xgsLrV__eNjPiEhX_c_9Gnn0E0m0K59mKGH12cXTny6NKWqLgI9fg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: s50egmjXQFhPOHbdSV4Myk0uwGc7WiSUF3h7FA7-2UOoDNC6-86MVVJWR1V7DicFa42WaznAX8VmiOnNklHjzg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: MYnYh925i_zsVDMPHHoAP0SBdyXt1-Oc1UAQNKcFz7f7pwwn9hEOqmIhT7lZpIkpnUVnAaPVYy9cEODLtk8MMA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'status': 'active', 'max_score': 100, 'avg_score': 60.0}
   {'avg_score': 50.0, 'status': 'inactive', 'count': 15, 'max_score': 90}

📝 Example 4: Function Management

📋 Total scripts: 8
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: False


============================================================
🔥 TEST 1: Original Config (durable=False)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: kK1rT2OFs_wzJf2jvZbzD7C1HO3hPPHD3FsVm5aR8spqyA_gYPvx8KhWmRVJJxzo-kO4bcoBJrgLva68WzCcWA
Created Bob: $500 - ID: Xjkz4YfczIq9ZtjlstzcX6ArRXJpfqZZQfzhSU-UrmS2shKnC-h9QNGBaqfI_NQB7LPVi3SoaQ8vz3SPkuAzHg

=== Example 1: Begin Transaction ===
Transaction ID: b9f91497-34bd-49de-a067-61a3a6e7aeae

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 4381001d-6d82-4609-bb4b-d5766696b41d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: HOT1LKb-cl6cSbVU2d5ABR6FPGJwKC_MXcDsxdnpwgschi9DTP08-2c_LKOn_oZsEWOkLZvxNRcCWo6WBhDeww
Created Bob: $500 - ID: 68tfarcjTK2QfBwc9sPISxRFCL6mZ5yivQ22yMzhsMW7T9o71ee39rngBSRTkIAtYGTX4VK6WIUQFhMrf3F6zg

=== Example 1: Begin Transaction ===
Transaction ID: 903f9431-01f5-4e37-832e-2755dccaba5b

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: _7bOTSXf5BJSlSvCNvJSVm_YtljFgZa6VQqfpCzJct65eb5ZcZvAJpU98EZl9D3g1EjS9Jtc8yWGxWXiBu4nWA
Created Bob: $500 - ID: vowbgXCWfqpMg77sBFuHl8qUB37SaO6mwyP_9gdoe8cp-WKfuHcHIL3sEw6ICCeOtvk29BtnQAEDaNWhLGIiWQ

=== Example 1: Begin Transaction ===
Transaction ID: 726339ca-0687-4e77-9ca2-99dc1710a898

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 8d1ef96e-5b13-4018-9588-3cf4eb1fad8b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: g0YDEHGG-c5xeCfeykui502rTfolqjjmVZcvTlE3wWtuo5DuCQB0QPTzSpa3HvfRvrne7MCCxd_cJYfl_S9YLw
Created Bob: $500 - ID: tD3l4mglYCsfYENhkfuigVlhnP4bKiWTIo_qpncWc8MpuvTMuMPlhEowr_GJxtDZWfgyKZ-kC2yhRaG4NqtLqQ

=== Example 1: Begin Transaction ===
Transaction ID: cb1bcce2-fd6d-4759-8a4f-c03aa22d3a0c

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: MsXTP7EQWW9QpGmRGaRJi-B5wAJbjVyaXkuWpzSu7SYtAN7qiqEHaTRJN_07vCl-a89y2dmt8RVC6b8TilOw8g
Created Bob: $500 - ID: 2KgL3heHxXZc6xBR-A-GGDKc18gMaLmrycDJTl31dCT1ZXd1KL67oyt5tYeWJx9BmBdBODVQBgGDM7wNf3m1xQ

=== Example 1: Begin Transaction ===
Transaction ID: 14a7d149-2b36-4e1f-ab1e-cc1877fdb6ac

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: e1d5a47d-8ebe-462c-b91e-ed428ec10dc9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: RmXhO8z-yeqd6uNeoA8jVj1LhrenA69HZrSibuvlasV8Sog0aVDvTtfgfhVhcEzrEjN0n_BZfCBEp53SgI7Dng
Created Bob: $500 - ID: cOOAsE11NWlelZFMOJ7B6vJPckMTrd6UFUGO1UyfA1fFXJQQ53TrDStb1bvoojWoGCc0g2J7SoClm-Coz60iVw

=== Example 1: Begin Transaction ===
Transaction ID: 3c863c5d-1173-459c-865f-63ac90282dd7

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=False


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

🚀 ekoDB Complete CRUD Scripts Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each Script shows Functions chaining with proper verification
============================================================
============================================================
📝 Script 1: Insert + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: RtMk-LB4lTL-dBnaPdVNXWGH5z4av1ufyMfzuQ06ccS4PlIDqN_8bk5MxEC5_PpzA82WNLSAEDPXcKfIuc2bWg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: vsQkKmEe4csJWmmcZgV2SkUmm8c7UGOEaVlLnyI0RLB8fHvf5LUzR48dz13XiP7s2YE_cRr_3jZf8DeW5o8ocQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: PLr_jj23SzhriG_t4194s0YyBaNE9V8oNtwGSaVJjPcxaRFWkKrQ-TllaT7rq3JIKMWXjlJNLLGL7AbaYA6InQ

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Z4kICiKSWZk1keJNdfJfOCHQFgyZZ5B_fjily-_7X-GjVC0wPoAlwkNGHZhmOF3JuKReK9YDjWemx_kxS7xxbw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: kzaA9ouBUpXp1faMi3vvuZpctajwuhbteUMiGkcXWSsruXI7ly1JVQrhhn6nITsulmFrctxL8u1RB0VYCBxxcw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: RtMk-LB4lTL-dBnaPdVN...
   ✅ Deleted script: PLr_jj23SzhriG_t4194...
   ✅ Deleted script: Z4kICiKSWZk1keJNdfJf...
   ✅ Deleted script: kzaA9ouBUpXp1faMi3vv...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Scripts Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Scripts chain Functions together
   ✅ Each Script demonstrates operation + verification
   ✅ Parameters make Scripts reusable
   ✅ Verification is built into the Script itself
   ✅ Complete CRUD lifecycle in 4 focused Scripts
[34m
╔════════════════════════════════════════╗[0m
[34m║   ekoDB Python Examples Test Suite    ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running simple_crud.py ===[0m
[32m✓ simple_crud.py completed successfully[0m
[34m
=== Running simple_websocket.py ===[0m
[32m✓ simple_websocket.py completed successfully[0m
[34m
=== Running batch_operations.py ===[0m
[32m✓ batch_operations.py completed successfully[0m
[34m
=== Running kv_operations.py ===[0m
[32m✓ kv_operations.py completed successfully[0m
[34m
=== Running collection_management.py ===[0m
[32m✓ collection_management.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
[34m
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
[34m
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_scripts.py ===[0m
[32m✓ crud_scripts.py completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mPython direct examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.13s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.1
    Uninstalling ekodb_client-0.7.1:
      Successfully uninstalled ekodb_client-0.7.1
Successfully installed ekodb-client-0.7.1
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '22dYwAeXMG1qIyY_qBOXiyTtDUHLKWzdQntaORsamupMAZYxcpG_2z9WWk0SXp8ERJctaLFOY2BYqGH7SNcewA'}

=== Find by ID ===
Found: {'id': '22dYwAeXMG1qIyY_qBOXiyTtDUHLKWzdQntaORsamupMAZYxcpG_2z9WWk0SXp8ERJctaLFOY2BYqGH7SNcewA', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'value': '2026-01-04T14:37:11.642256', 'type': 'String'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'type': 'Integer', 'value': 42}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'type': 'Float', 'value': 99.99}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04 14:37:11.642256
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'id': '22dYwAeXMG1qIyY_qBOXiyTtDUHLKWzdQntaORsamupMAZYxcpG_2z9WWk0SXp8ERJctaLFOY2BYqGH7SNcewA', 'data': 'aGVsbG8gd29ybGQ=', 'tags': ['tag1', 'tag2', 'tag3'], 'created_at': '2026-01-04T14:37:11.642256', 'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'active': True, 'categories': ['electronics', 'computers'], 'value': 42, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'price': 99.99}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'price': {'type': 'Float', 'value': 99.99}, 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'active': {'value': True, 'type': 'Boolean'}, 'id': '22dYwAeXMG1qIyY_qBOXiyTtDUHLKWzdQntaORsamupMAZYxcpG_2z9WWk0SXp8ERJctaLFOY2BYqGH7SNcewA', 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'created_at': {'type': 'String', 'value': '2026-01-04T14:37:11.642256'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"userId":123,"username":"john_doe"}'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'value': '{"name":"Product 1","price":29.99}'}
cache:product:2: {'value': '{"name":"Product 2","price":39.989999999999995}'}
cache:product:3: {'value': '{"name":"Product 3","price":49.989999999999995}'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "9ow_P5LGIOhszGkNPbsHi3kIKdSFitAJTgu5X9rhV0gy69of6WuVY_AJUIl5KZX_dzsSW9qtWGvtXvq0Es2uCA"

=== List Collections ===
Total collections: 10
Sample collections: ['ttl_cache', 'test_accounts', 'ws_ttl_test', 'websocket_test', 'chat_messages__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: raY9gHm5AROuIiSV6fjREopQASc35wH5Tc4-gN4tVUqJybKfHjd_4bq-eYp4vVfL_70h0ZijxuWk_95Ylw3OJw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: CzDYbtmkV6cfnSuSt4o4W9Ekd9Y77XV_YC06fsFzN6tfIswPXtvs3AvBtGEJKi0Q4xFgJSfSezg6OuAHXEXFxw

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
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: UOE0vMi-b_2ZhktT_IyDRMUqcra5M_YHDmTDVrY5YyFa_-k9V5sMs4dMeeYOZQv6KYhlwDwqqGVqgr2V4UjLIw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: FSngzkMdaKufGQyF3owRO2xB5aadZI84s52Flh9t_UWZgAwgBO0AAWrqDe-2kmK5iO_gA_GUJLCkIpwqnlRKwA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Query Builder Examples ===

Setting up test data...
✅ Test data created

1. Simple equality query:
Found 2 active users

2. Range query with sorting:
Found 3 users aged 18-65

3. String operations:
Found 2 users with @example.com emails

4. IN operator:
Found 2 privileged users

5. Complex query with multiple conditions:
Found 1 active US users over 21

6. Pagination:
Page 1: 2 users

7. NOT IN operator:
Found 2 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 25.740, Matched: email.value, name.value, email, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, title.value, bio.value
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title.value, bio, title, bio.value
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.755
  2. Score: 0.751
  3. Score: 0.737

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.726, Matched: content.value, content, title, title.value
  2. Score: 1.116, Matched: content, title, content.value, title.value
  3. Score: 0.529, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title, title.value

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: ['age', 'email', 'name', 'status']
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders:
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 75BwxYE7jLzRVr74SLD1UtuvPgDxlgpFIhcnXtSRADrscb60rK4_7_n7mx6Ip0Q5uiS2ZECQcBR6Z8uuZumVcQ

=== Sending Chat Message ===
Message ID: EZFyBh5_S9qnyecVa-LSmKpQXnVSXnEjDTJc_BFtWGSZHQHOT7IebgSsBOg9Wq08DUG2G8rLwvMUKzwXyWlEgQ

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

Execution Time: 2883ms

=== Token Usage ===
Prompt tokens: 611
Completion tokens: 89
Total tokens: 700

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is "ekoDB", which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB database product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ENZUc6J-ov9VDsYSxvtvbEWPu8mGL6CGnt8kN7HHlPpfaEyCLyttskpA-G9opIYiEBF5s_0NpuOKOCnQH7ZyhQ
  Parent: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: ENZUc6J-ov9VDsYSxvtvbEWPu8mGL6CGnt8kN7HHlPpfaEyCLyttskpA-G9opIYiEBF5s_0NpuOKOCnQH7ZyhQ (Untitled)
  Session 2: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw (Untitled)
  Session 3: 75BwxYE7jLzRVr74SLD1UtuvPgDxlgpFIhcnXtSRADrscb60rK4_7_n7mx6Ip0Q5uiS2ZECQcBR6Z8uuZumVcQ (Untitled)
  Session 4: QRhmpERBtntusi1QZ2IH-CYZz8YqvGsUxP5rSLCZFUJV6k9Fh9IzpoKcyUkay5qYqXNuTe4_QsJe4LM3bnFRfg (Untitled)
  Session 5: Z5AKjIPI2yAcnUe0vBJQ876BWLJQ6XJWcYAMhqakwaXAEkhgRaePkXsgJTBf8TLtVPGSn7RWgBvBAq_zwKXWwA (Untitled)
  Session 6: o1-Gof096ND-RPEft85EWug_1lfr_pJ9PK5P-0h1_mparol8lKHJxwGPXtTCJTuOEkqpagS5eun4RH9azwSU_A (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: ENZUc6J-ov9VDsYSxvtvbEWPu8mGL6CGnt8kN7HHlPpfaEyCLyttskpA-G9opIYiEBF5s_0NpuOKOCnQH7ZyhQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: C-m2kF2980TOCKFVBOFH0lgC6QY68KRQGCi1VYZ23KvaDWkMHcHaZetsZi9MdLek2pWYdzoi4VCeytz7yw8diA

=== Sending Initial Message ===
✓ Message sent
  Response: The product available based on the provided context is ekoDB. It is a high-performance database product and is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'role', 'content', 'created_at', 'chat_id', 'token_usage', 'context_snippets', 'updated_at'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: jmpddhHFZ8sTVNF70g7Qmqj7SExneQLGhcthObJr-pu8acdXHAXCRIuxUGP_rkBZ9_M_cZt0hyqCSWJLO55HBw
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
🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: lh5BUEM6DGKHZMuhANyoiGjQ76kmvqeQNQsvQInBY_Tk-G_V5iQtYG5rMIbzSg9Qx0SIXl2ODOPziwCdKBD5Kw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: W4GWgp3yKxkGoy1ZeIjUbHuhnRz5coy7NhTFh0R3YCNL3X6cqXuhRhPCxEeAyHqkuaNyY_LXvBOzIklwUfg1CA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Ma2AR1vDOW3NHQ4S6SR_LDyKzH89PfV_hrlj_QwG4A-uEsJ-eO5Lxp5FPVhHKvYE65Hbrj9lJrVS7ekDZpVGsQ
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

📝 Example 4: Script Management

📋 Total scripts: 10
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: YvJ2FI5_Fz083TiPQvhQaYaltm-h084hQI9jRQSS8z4BW4ZzNeXwhxV2Zi5bZN8IP1W9MEtsWPHqPpQfqR2vZw
📊 Found 2 product groups
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {'category': 'Furniture', 'count': 2}
   {'category': 'Electronics', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Script management (save, call, delete)
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: PaJGCKsyYxTEfpwA-52JJO3HTzC8Hu9My78wVjUeZQv2wedrQZvMsgc_V1kv858RzxULf7HLhVMaMywHG_CtEw
Created Bob: $500 - ID: lwZFsAQvm9mHqGB41fAjLPi8gCHIT1M4w-VcN7kMjGsfCWfsoiMi9Q5w6aMsRsSC0NgtWGmw6dFxVSOVH2NC-Q

=== Example 1: Begin Transaction ===
Transaction ID: 84ad6284-2080-44aa-bc2d-23d9bef43abf

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a5007ba8-7fb6-4b16-8c70-188fe95c2e19
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
✅ [32mAll Python integration tests complete![0m
🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║     ekoDB Go Examples Test Suite      ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
[32m✓ Server is ready[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running simple_crud.go ===[0m
✓ Authentication successful

=== Insert Document ===
Inserted: map[id:vMZbsOa4wgdIL6bjVA95Je54RKOaHfSzqopzS3rX-NLBUm8xIaof4I9mHNLBTkpKaeXzp_ImSTX4-mEw0mtM6A]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:vMZbsOa4wgdIL6bjVA95Je54RKOaHfSzqopzS3rX-NLBUm8xIaof4I9mHNLBTkpKaeXzp_ImSTX4-mEw0mtM6A name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:vMZbsOa4wgdIL6bjVA95Je54RKOaHfSzqopzS3rX-NLBUm8xIaof4I9mHNLBTkpKaeXzp_ImSTX4-mEw0mtM6A name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: y9NoyejrqnC0jG01aRCiN8LAwyxDTytOuFaianoVIj5ki1UHMIIB0_cmUCxZarYjkBfyk_34ya5TMs876CQIVw

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
        "id": "y9NoyejrqnC0jG01aRCiN8LAwyxDTytOuFaianoVIj5ki1UHMIIB0_cmUCxZarYjkBfyk_34ya5TMs876CQIVw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "9jhiJD-vX2ICyYsmVzgVmtlyo9yTqcOWZTdsd5jfBvDaYIiuAKy4_bD3ASkcLX_E2gV2whXZrWIPWIfwLrXQAQ",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
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
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.go completed successfully[0m
[34m
=== Running batch_operations.go ===[0m
✓ Authentication successful

=== Batch Insert ===
Batch insert response: map[failed:[] successful:[AOU4yto2RvqewJ2te7ppQ5POxNfmlmPA4F7RHP6pjIm3iDmzQy02r4xYgr6HG_dzKuSgKbzzTLIG5S90mbn3uQ VeC730SxMVTbVl_vGnwHqjSQsuyL7c8Mvm5KSQcqlrWtUflcn0OWWewrHyYXI7R9C2wuNQi_glCSrqkbvM5lQA 5UkLeHigdELGdOd7mmWWqd2w-UeD8fmZSzHWvtbQGXk99moszY0y8elImJmsau9I7W9zvrTlY8LUyWy0OpJlQQ 9ajghvCQcV0VZI0Vwnyex5yZ5F6Hp4JAGhM_wxD77ql72M-qzJ6xfFY9G5QyLm6o1mmKtJDwVrgSmH280kbY5A X6bIuFzKW3eV-MngaiNFgjLl3m9GeucYLHDrzQcTKzrLOWX0NfKI674DlrJaeKAlE62EuGkT7eNJOkRNp3fe_w]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[E8NIP4KS9zqwo6zKIa0ScOYACJkR82vu47gmF8mq9mfaafgU2xbomxwg6ozhKddu2IKf4Xp-4S4GyNM4qOJk6g U9hbftrUMvvNGJgdmyv5E-d_JDOAed3T7L5TkIho0QVAK352cuT0NjXUI9g3kWvc_0FkERcgGvHcGtOP75XQHw lmciiJvtPyteIjZGd3ocA6Fb17hUJRVwrYeRYq40PQdY97-Oj95Sz6MHkI45F-qkkWUeMXdM2Nuix-4RRjgOng]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[E8NIP4KS9zqwo6zKIa0ScOYACJkR82vu47gmF8mq9mfaafgU2xbomxwg6ozhKddu2IKf4Xp-4S4GyNM4qOJk6g U9hbftrUMvvNGJgdmyv5E-d_JDOAed3T7L5TkIho0QVAK352cuT0NjXUI9g3kWvc_0FkERcgGvHcGtOP75XQHw lmciiJvtPyteIjZGd3ocA6Fb17hUJRVwrYeRYq40PQdY97-Oj95Sz6MHkI45F-qkkWUeMXdM2Nuix-4RRjgOng]]
✓ Batch delete completed

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995]
cache:product:3: map[name:Product 3 price:49.989999999999995]

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.go completed successfully[0m
[34m
=== Running collection_management.go ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: ptddHEPSQEaJB-fBdFuXup2nTQfYjiB43-9jWw-G8Fqat1_qSVwbkf0UJMdTDW7h5lp1LLU7v-XALrXsVANGTw

=== List Collections ===
Total collections: 15
Sample collections: [ttl_cache schema_products_client_py test_accounts ws_ttl_test websocket_test schema_documents_client_py chat_messages__ek0_testing test_collection batch_users scripts__ek0_testing schema_employees_client_py chat_configurations__ek0_testing demo_collection schema_users_client_py users]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 93EUsAFTkEZid37cYhiq6DL-kf0RJ0bzLj3umKpYLumvn7FxYnjl3lwDL8IXmuulOtnOC9BjiL_iVRTW0Q7T2g

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: IMuIAGkLu3H5-m5R0dm7z-zYd7SfeKswVNOK58B5GWl6v61rEfWIp3jgpfm4gUjl3x59p7esAWcLVpMgCWNKuQ

=== Query Documents ===
✓ Found 6 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: mndcJS_9e9rw4DFBzHHar1STmp2Qjaxz0ZSJp5lf-tbAHoLD6PenjQZ_AdXUVbg3ZETlCm_QOcqkKurOTnJfKQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: voGIlYCtRq6Z-7Mlmt5OhVFcQthorYs5F61oQKcIvdZjinXFpuY_nQw82mL1RTTzSbXeNZwSlEXG-xhvvB2lyQ
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: yXoUypW4ag_ITjSTKrIoVXBr6QVUmBsv1AZqWK-edPRVnE3fbfK4jBxvAAxYJSw1m9_yrTAc77eMp0rv-NU0gw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 90f_d45fplSIo_ag-S-qknPZffuMbjbfNri7ns1ywx2jhi0mj2hY5XtM6EE7PlSC3ldxVlT5N9GthghX0F0hhA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 12
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.go completed successfully[0m
[34m
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: UCP6bkhJFRSHznVl9W7GIWz2wslTfRhMAVa-ZlFTRlOG-BzLWmwy_4Hb_p78axWOyz7mlJgVQwmWpMaYRHkKQQ
Created Bob: $500 - ID: HyL-iDekqT50foEfJ_gdmntwGMxaeOWhzCQlk_D8krG7xfigPFRALaFB83wxBrpWaUV5SXL6rV2aI27t3T58pw

=== Example 1: Begin Transaction ===
Transaction ID: 2a50b320-e77c-492f-9c3b-5ea900f18b89

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: 1221b17c-d2a0-4208-a3c4-f4d22402593b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_scripts.go ===[0m
🚀 ekoDB Complete CRUD Scripts Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each Script shows Functions chaining with proper verification
============================================================

============================================================
📝 Script 1: Insert + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: V1B5EkaDpzdZ6wfrI-2HvpK46n3OF_d6ftg2VGxYjg0qsddV4PbUmIcqa9_Qcl_n0y0cws5BeG9eXd22L5vlHg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: mkA1fnH9tKXCpwLiOTS_BkgGz_u1hmQ2X4xKRzKxnlexyZyUEhMdgM_jCisDeGsDvBBwfdUp5gXZTBCkKnAcAQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: W98TGpVnde8q-GEvpVM60VXi0uu_RR5IC-vJCfX2fUE1a9vLhjqR1oLGSytJlyY9rojRzqTyVoH-aseYeXnU1w

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 8zG0KGg18VAEU2CeY3bTdtjIknQfKN2JxEGd0ZmLMBslbvxESNPGmpoyyqJYkNQqUGkkNBeUfKADpMCMyCTsvQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: E3mpUECOwCaCQ1grRgDDyoKyDjpAaj78iQwVCl3xTqmbjLf9xvIuDeJBk2Time_VQFP5bYaHilDGnzU0ERdCgA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: V1B5EkaDpzdZ6wfrI-2H...
   ✅ Deleted script: W98TGpVnde8q-GEvpVM6...
   ✅ Deleted script: 8zG0KGg18VAEU2CeY3bT...
   ✅ Deleted script: E3mpUECOwCaCQ1grRgDD...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Scripts Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Scripts chain Functions together
   ✅ Each Script demonstrates operation + verification
   ✅ Parameters make Scripts reusable
   ✅ Verification is built into the Script itself
   ✅ Complete CRUD lifecycle in 4 focused Scripts
[32m✓ crud_scripts.go completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mGo direct examples complete![0m
🧪 [36mRunning Go client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:mXgclJsIydEPhyyDAAzGgpBCPQfaJpmOLi82-0fszHDLPcW3nBlMK0HGOqOPH0KAgu-xBNhg-WER-oc6sHp8Wg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T19:37:46Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:mXgclJsIydEPhyyDAAzGgpBCPQfaJpmOLi82-0fszHDLPcW3nBlMK0HGOqOPH0KAgu-xBNhg-WER-oc6sHp8Wg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-04 19:37:46 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-04T19:37:46Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:mXgclJsIydEPhyyDAAzGgpBCPQfaJpmOLi82-0fszHDLPcW3nBlMK0HGOqOPH0KAgu-xBNhg-WER-oc6sHp8Wg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T19:37:46Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:mXgclJsIydEPhyyDAAzGgpBCPQfaJpmOLi82-0fszHDLPcW3nBlMK0HGOqOPH0KAgu-xBNhg-WER-oc6sHp8Wg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995]
cache:product:3: map[name:Product 3 price:49.989999999999995]

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
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: LgzxqPtmMxmTRLSCEt6S_O4Vyvd6Uu3zhwMuYhO0cVRKJVBy7r9Lm2cfMUlnS8HFEUs51pv1LBi-gdFcX2s_jQ

=== List Collections ===
Total collections: 14
Sample collections: [ttl_cache schema_products_client_py test_accounts ws_ttl_test websocket_test]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: I5NlmwPKjQq8OJrR-_220eBY5R2MrvSjjzkeYfjHEhetvR4tgUAZEhdWED6QK83NOmQM3RiOKUTKJopgdSVaLw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Kzn4B_TE1OyIplWfuG7i5rWPfJfQVaH-4fyjPCJOcTov2_6C9u9_yeE9DB77-c8oQNALuXuhV-WCd2Ch1eZixw

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
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 9EBEsdCevxiIXptvK30YVkTmMV__gILck8LTvxwF543VUubQtYlAYyXcL3jEYjhbdV8DRXsp18QiQIf6Hp49VQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: rkl1K6tMPJssGR-Tt6j03YxA3GnjFE-5Fq3GQ3wvei2MJkaX3qcdr_UVdYof3JU9Wq2zaUYb55b3t5-t8vZpjw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 6c6I4tbKn4s5HXGqxA8u3X9GUS68Q-_arUUTv8flyLDZAgHVoQH7Il1ZjmGO5l1Wcc0-GL1_hEctqKojwhRU-A

=== Sending Chat Message ===
Message ID: g5FOe2S68Akz6LAimmdpkipAA4KsjzhULm86_ttidQ8LgxwbbOd7w1S_lhHqAoDvFkfrZrbRjGz1T6N30isG_A

=== AI Response ===
Based on the provided information, the available products and their prices are as follows:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.
2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.
3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:DQ0JWQzNPOoktqQTBnfh9YmhdY6AhMn8JLP-Supfi1GNn8E-63kJ70BibIv63VGM3n03nEl_gyqWrEXw8l0qsQ name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:7SM-XcP2TbhQQmNA41a2NFoP2fQLSBLfUvybp6tiOqXJY_GkHz3JcFC6Dk7HY3mmNNJ5lVGrJd2y5KuV2VuMdA name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:L-RrDH3t_0mBJ_Y2u2m3f_hBPUq1vpyHpC8Cb-w7S3dIFHcyX04ukW0wwiSNaWJzEeds6iquYjf2WhbKqbi94A name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2576ms

=== Token Usage ===
Prompt tokens: 612
Completion tokens: 94
Total tokens: 706

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Us_Q6dEoGIzoWbZSxPo30f5vvCQWxznglrknYiDn6iY2PXbYFDs2NCf4h1Mm5xImmYJBDFwmEMjlqVTkY0-A_A

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One product available is the ekoDB, which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: V5Qa3qamU75cR45Qkp4oec3EqnSXNUcSYqhl366Tl2t9DpM5EToFhEaAtpRhnX4YwQUzW7ih3TIAEX3x9nz8KQ
  Parent: Us_Q6dEoGIzoWbZSxPo30f5vvCQWxznglrknYiDn6iY2PXbYFDs2NCf4h1Mm5xImmYJBDFwmEMjlqVTkY0-A_A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: V5Qa3qamU75cR45Qkp4oec3EqnSXNUcSYqhl366Tl2t9DpM5EToFhEaAtpRhnX4YwQUzW7ih3TIAEX3x9nz8KQ (Untitled)
  Session 2: Us_Q6dEoGIzoWbZSxPo30f5vvCQWxznglrknYiDn6iY2PXbYFDs2NCf4h1Mm5xImmYJBDFwmEMjlqVTkY0-A_A (Untitled)
  Session 3: 6c6I4tbKn4s5HXGqxA8u3X9GUS68Q-_arUUTv8flyLDZAgHVoQH7Il1ZjmGO5l1Wcc0-GL1_hEctqKojwhRU-A (Untitled)
  Session 4: jmpddhHFZ8sTVNF70g7Qmqj7SExneQLGhcthObJr-pu8acdXHAXCRIuxUGP_rkBZ9_M_cZt0hyqCSWJLO55HBw (Untitled)
  Session 5: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw (Untitled)
  Session 6: 75BwxYE7jLzRVr74SLD1UtuvPgDxlgpFIhcnXtSRADrscb60rK4_7_n7mx6Ip0Q5uiS2ZECQcBR6Z8uuZumVcQ (Untitled)
  Session 7: QRhmpERBtntusi1QZ2IH-CYZz8YqvGsUxP5rSLCZFUJV6k9Fh9IzpoKcyUkay5qYqXNuTe4_QsJe4LM3bnFRfg (Untitled)
  Session 8: Z5AKjIPI2yAcnUe0vBJQ876BWLJQ6XJWcYAMhqakwaXAEkhgRaePkXsgJTBf8TLtVPGSn7RWgBvBAq_zwKXWwA (Untitled)
  Session 9: o1-Gof096ND-RPEft85EWug_1lfr_pJ9PK5P-0h1_mparol8lKHJxwGPXtTCJTuOEkqpagS5eun4RH9azwSU_A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: V5Qa3qamU75cR45Qkp4oec3EqnSXNUcSYqhl366Tl2t9DpM5EToFhEaAtpRhnX4YwQUzW7ih3TIAEX3x9nz8KQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: m0MPTKcmRmXi-6apnKaIbw5TD0tgD-Od21R_cvK-FXQQaNtlQ8Tv3-_A2d-wDBzFUjEcCyqRoSUe1JRrw70Ewg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: _NL6OYBqQLoEupcizbL9l5texJ70hYuEDsy6eTjOgiLN5oGcWPOY8SXyDAtM8qpA8loeptso8XXiGosWevuMEQ
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
=== Query Builder Examples ===

Setting up test data...
✅ Test data created

1. Simple equality query:
Found 2 active users

2. Range query with sorting:
Found 3 users aged 18-65

3. String operations:
Found 2 users with @example.com emails

4. IN operator:
Found 2 privileged users

5. Complex query with multiple conditions:
Found 1 active US users over 21

6. Pagination:
Page 1: 2 users

7. NOT IN operator:
Found 3 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 25.740
  2. Score: 12.540

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600
  2. Score: 39.600
  3. Score: 39.600
  4. Score: 39.600

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200
  2. Score: 13.200

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.739
  2. Score: 0.736
  3. Score: 0.716

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.695
  2. Score: 1.486
  3. Score: 0.296

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: 4 fields
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders
  - Laptop ($0) by Alice Johnson
  - Mouse ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: Q6zYUKQNbe9lKb7me9Zlp8tYSx6OfQ7NyI7caJRcPv4rsXBtE9_HzJ898A_tFPvwyisv0SOHM3lseZbgG-113Q
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:50 count:5 status:inactive]
   map[avg_score:60 count:5 status:active]
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 14
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: sjtcLjCz7JKna6vBlXKEzkZMAD6__1nsxpGQyyAWL3H2HdXkA4k0JypBR4nr0mFW9_c5kBT9SzTz2kPNCGj-gA
Created Bob: $500 - ID: ZOfPc_vnUFZgjWT0yBg7-tAvzwNfsqlBWbz-u9lbbgaxxidX6QUhSp3TfhMB6WJFyuvHEhuGM5mMGafWmaVZAA

=== Example 1: Begin Transaction ===
Transaction ID: af9c9b9f-3bed-452a-bada-ca16a5e71dc6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 307b2735-e02f-4c15-9c87-95ffc36f7d01
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 831ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 's5gozbcrRYw0OKLDLni8yoZJlBVgA_sySoTeQs7Kbt1uD6kPzuZzSndiikmQ6z2jzAbijQ4rv5WGJ2QWalnR6g'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { value: 'Test Record', type: 'String' },
  id: 's5gozbcrRYw0OKLDLni8yoZJlBVgA_sySoTeQs7Kbt1uD6kPzuZzSndiikmQ6z2jzAbijQ4rv5WGJ2QWalnR6g',
  price: { value: 99.99, type: 'Float' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-01-04T19:38:07.497Z' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04T19:38:07.497Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  id: 's5gozbcrRYw0OKLDLni8yoZJlBVgA_sySoTeQs7Kbt1uD6kPzuZzSndiikmQ6z2jzAbijQ4rv5WGJ2QWalnR6g',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  created_at: '2026-01-04T19:38:07.497Z',
  metadata: { key: 'value', nested: { deep: true } },
  data: 'aGVsbG8gd29ybGQ=',
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: 's5gozbcrRYw0OKLDLni8yoZJlBVgA_sySoTeQs7Kbt1uD6kPzuZzSndiikmQ6z2jzAbijQ4rv5WGJ2QWalnR6g',
  created_at: { type: 'DateTime', value: '2026-01-04T19:38:07.497Z' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  price: { type: 'Float', value: 99.99 },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: yQ-WmpQPON4JkTZGH0lenjIiZcKVesjQan-G1QPyAWU8iDW7sxNdEmTLxgvSdKm-RzjEuDF9VAB8NzP2mAtUUQ

=== List Collections ===
Total collections: 18
Sample collections: ttl_cache,client_collection_management_ts,schema_products_client_py,test_accounts,schema_users_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Kn3Avmw4m1Jh8os38iNPm5uJ_eAFAGw8Ft2DIIeLgCWk3i7ekjKJnyCA2F9w04qnNFoKwiYi3-2BdQVFKabsFw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ahFWEXg2GX9H9PhwsAF2OKJtAIkq3ssUnk9bzTZNlbOkbxHe7ThbJ5hxS1qHnwNyHR13zO5MbswDeUIpgcftSw

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
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: hXhTWMwO1ty8AboUtt4sO13vD5gZhnxfg3_0iyP1IJqa11kiRIGRoyXrJG85opdFjR0QBp08ghCVkL9gITr4LQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: p6QHKA8uzgD8Tjj6k_eBOzxFV3lxeNWWLAjRPQkNsDeEAXAwDvTgFo7avBXBpTU37kDzCTRqUoIhTwOm34IdEg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: hSk2g2QahlOh5JK78hvpCH0Ibw4bL1vCzP4Xmzfooqre7n4bb5OBIsJoS2T-c9clWa-X5x3r3c6M2JG_6Skt_g

=== Sending Chat Message ===
Message ID: 78RzaPWRS6Wdgs_FpIqvbAvagNax0bbdgnIh8_MjX6xCdnOCFsqqUMIZe0bMPgFLMPRNsDKnJUmsXbZOAgpKFg

=== AI Response ===
Based on the information available, there are three products:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: 'x9LUVmKBTptoYHerHD6WQTVcfsbde2ymqD07KiyyTk11YdLw5AUIS4K10sh4vpro5Qz_jgiXqdRfZ_3jMeMN9w',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'fa5WhcLB0TIZTZYTlJu06wXfJjpR1G8Al24DPpNhu86F5msX0tyDIvbwoRPZopESYndysyAtAc9Lpkkk5aMTMw',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: 'hc_3oHelXpYVuy6ZNMpadMuPxpkmDZoRUrFaLWkizGqEOgWZrlfCeevbiWG4YVE1gW7miB2yFrd3IjzwO3h4Ng',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3139ms

=== Token Usage ===
Prompt tokens: 603
Completion tokens: 89
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 8R8msFD8d66a8y7SauAyRzK9AoLBPfris-0M9ycc_ZNKBSZ1C24L2D-g7IX0BDi9Ra7_i2RtubB_-a1aO7Uz4Q

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the product available is "ekoDB", which is a high-performance database product. It is priced at 99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fEbUrzD93oaFZSCHpXH9BSCQGd5STK1xkmmiAXpK3B4Zm7xZMwnrpwgxIXgphDoz0uBmjdZVUMaKss58TbWA6A
  Parent: 8R8msFD8d66a8y7SauAyRzK9AoLBPfris-0M9ycc_ZNKBSZ1C24L2D-g7IX0BDi9Ra7_i2RtubB_-a1aO7Uz4Q

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fEbUrzD93oaFZSCHpXH9BSCQGd5STK1xkmmiAXpK3B4Zm7xZMwnrpwgxIXgphDoz0uBmjdZVUMaKss58TbWA6A (Untitled)
  Session 2: 8R8msFD8d66a8y7SauAyRzK9AoLBPfris-0M9ycc_ZNKBSZ1C24L2D-g7IX0BDi9Ra7_i2RtubB_-a1aO7Uz4Q (Untitled)
  Session 3: hSk2g2QahlOh5JK78hvpCH0Ibw4bL1vCzP4Xmzfooqre7n4bb5OBIsJoS2T-c9clWa-X5x3r3c6M2JG_6Skt_g (Untitled)
  Session 4: _NL6OYBqQLoEupcizbL9l5texJ70hYuEDsy6eTjOgiLN5oGcWPOY8SXyDAtM8qpA8loeptso8XXiGosWevuMEQ (Untitled)
  Session 5: Us_Q6dEoGIzoWbZSxPo30f5vvCQWxznglrknYiDn6iY2PXbYFDs2NCf4h1Mm5xImmYJBDFwmEMjlqVTkY0-A_A (Untitled)
  Session 6: 6c6I4tbKn4s5HXGqxA8u3X9GUS68Q-_arUUTv8flyLDZAgHVoQH7Il1ZjmGO5l1Wcc0-GL1_hEctqKojwhRU-A (Untitled)
  Session 7: jmpddhHFZ8sTVNF70g7Qmqj7SExneQLGhcthObJr-pu8acdXHAXCRIuxUGP_rkBZ9_M_cZt0hyqCSWJLO55HBw (Untitled)
  Session 8: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw (Untitled)
  Session 9: 75BwxYE7jLzRVr74SLD1UtuvPgDxlgpFIhcnXtSRADrscb60rK4_7_n7mx6Ip0Q5uiS2ZECQcBR6Z8uuZumVcQ (Untitled)
  Session 10: QRhmpERBtntusi1QZ2IH-CYZz8YqvGsUxP5rSLCZFUJV6k9Fh9IzpoKcyUkay5qYqXNuTe4_QsJe4LM3bnFRfg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fEbUrzD93oaFZSCHpXH9BSCQGd5STK1xkmmiAXpK3B4Zm7xZMwnrpwgxIXgphDoz0uBmjdZVUMaKss58TbWA6A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: DEwJpm7_VLd-G5G8GionxiGwXzGCjNRsrn4xLV4j8mt4BGRpu17rTpLTSGqpWWnxG135FDFD7r8yMNJ-B1vaow

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is ekoDB, a high-performance database product. It is priced at $99.

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
✓ Created second session: TXfM7lb2Ru0Uui3UoPSjYMPWbezndbeekYaKgHumU34_OQN-eEHe7s0HjV1ST93szUaIXFNTF9ls9VKDNS80ug
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
=== Query Builder Examples ===

Setting up test data...
✅ Test data created

1. Simple equality query:
Found 2 active users

2. Range query with sorting:
Found 3 users aged 18-65

3. String operations:
Found 2 users with @example.com emails

4. IN operator:
Found 2 privileged users

5. Complex query with multiple conditions:
Found 1 active US users over 21

6. Pagination:
Page 1: 2 users

7. NOT IN operator:
Found 3 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 25.740, Matched: email.value, name.value, email, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio.value, bio, title
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: bio.value, title, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: bio, bio.value, title.value, title
  4. Score: 39.600, Matched: title, bio.value, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.742, Matched: 
  2. Score: 0.724, Matched: 
  3. Score: 0.723, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.690, Matched: title.value, content.value, title, content
  2. Score: 1.489, Matched: content, content.value, title.value, title
  3. Score: 0.297, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, skills.value, bio.value

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: [ 'age', 'email', 'name', 'status' ]
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders:
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: kD0yrik_G3T_YjLMp3CC1j4naRIKBpYMTsN9_ldenQ6Q0SkzDKet-xbPSGItQbtwsk8knE4wCgQKDUj3Rxekyg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: dPJlZTzxJxfSQ0l1c-ito20QAgwhrI2erJpL0mqD60hlY5qIKoVB0zGPcs8KbIcABaTJsVWN6UJID5Dm6UuLFA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Yz6nEzuKy4NumCLlU_qalBrLi40FRQv3h0qynelRvFF6HIuhqLa7F8ouO-BGcozm5Pv25p5-uVOdWundR4x31w
📊 Statistics: 2 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"status":"active","count":5,"avg_score":60}

📝 Example 4: Script Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: gJToRnWRE1pKO0bnfu4-QrvgZR8Q_FCDZOwVw4qaIj3VLWlePvKpgGS8J9moI82WV4O2XxdEUt7WKynFeq7tlg
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
   {"category":"Furniture","count":2,"avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
   {"count":2,"category":"Furniture"}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Script management (save, call, delete)
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: voTZlnu9o_WKTVRYqi7JrLgMdDoQwZZzUzELRX1_ITAasIZVbHWONxfI9W43-m7zOeoCesvUUQCrawhN-yWDhw
Created Bob: $500 - ID: 4LHS4F79-584nfaqqV3unkxWJf3Ho7eVxZzzQIcaIvTMSAUx8NMFENH11M0lxM8FbBgLdQz03MWhPvTThSjx0w

=== Example 1: Begin Transaction ===
Transaction ID: 7c377337-cda0-4be7-a82d-ac6b7fdd24be

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0778703b-547f-45e7-b505-9cc1932ad0a0
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 823ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:9604) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===[0m
✓ Authentication successful

=== Insert Document ===
(node:9607) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'j_lszmgYJr2XR5KrjSf16UgKygNjJ4-S_BlcYFIO1QWWEP3DFF5PnSfFNw1z8Gmo-WxfyzNCTl1DEvsDufhSuQ'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 },
  id: 'j_lszmgYJr2XR5KrjSf16UgKygNjJ4-S_BlcYFIO1QWWEP3DFF5PnSfFNw1z8Gmo-WxfyzNCTl1DEvsDufhSuQ',
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'j_lszmgYJr2XR5KrjSf16UgKygNjJ4-S_BlcYFIO1QWWEP3DFF5PnSfFNw1z8Gmo-WxfyzNCTl1DEvsDufhSuQ',
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:9614) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: UcoB-gn-YjVytHZmK4ByYPmAMtzVBkcSZIxPsU9aBGCZCFqRLuPuKA_wRgFJTs11NvbM9D8D1cGbAw1Qc9PbnA

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "UcoB-gn-YjVytHZmK4ByYPmAMtzVBkcSZIxPsU9aBGCZCFqRLuPuKA_wRgFJTs11NvbM9D8D1cGbAw1Qc9PbnA",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "9jhiJD-vX2ICyYsmVzgVmtlyo9yTqcOWZTdsd5jfBvDaYIiuAKy4_bD3ASkcLX_E2gV2whXZrWIPWIfwLrXQAQ",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "y9NoyejrqnC0jG01aRCiN8LAwyxDTytOuFaianoVIj5ki1UHMIIB0_cmUCxZarYjkBfyk_34ya5TMs876CQIVw",
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
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js ===[0m
✓ Authentication successful

=== Batch Insert ===
(node:9631) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Batch inserted 5 records
✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
[32m✓ batch_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js ===[0m
✓ Authentication successful

=== KV Set ===
(node:9634) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2026-01-04T19:38:31.904Z',
  userId: 123,
  username: 'john_doe'
}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:9642) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: GRs9nzidzRrtkANeTH6ZDo_M2Ftk7QcU1wg0wX45RCiW6cbeeD6Pk4clW0y5SH5VSv-EETr6Y7xlP6ATzcRwhA

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_employees_client_ts',
  'ttl_cache',
  'schema_products_client_py',
  'test_accounts',
  'schema_users_client_go'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
(node:9656) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: 1XDKD4r71t-s5lIHrAq1sxa0Yj_1RrOU-NlRtnXqlJF4EiQDWSpARzs_-Snhuw-uYmsCAaoyzjKxG63RFmswaQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: vus_514-GMJ3USHYNdXHu-P7gaLKlweokkrsivZ1ehtKGKzXbGzH5xoUsP_YcmRzUqY2x46nPPYgEAYCgWfPdQ

=== Query Documents ===
✓ Found 7 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:9663) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: 1BTa2ZhUezRidHXMOj2ZLw9zTg90e0p6sd4xyti89kzz-3apfpQ5BHourwqzn4DAHcLKscMQnfONIAhD7byfsA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: dsMVETt7-S5VSyc7Rr8ODHILnSuZpKqfHUBRWPWzPS5ArS07SGHMUWadCcWVhNND-NcovsGbzwHADyDrMevvyQ
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: WD_qzh8Jb-lhW3jb11GkPFvuyw7IMin0tUp-RU95VRj1si30PHs-ZhopZZfq8v2VzXvjTB5aVktEgW7gXfPn7w
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: rGqD4YJ-UDZl3NZCdD5mKGyGlWf7U0DrrtAyq6K0XGN-TxyLi_T_pfNyWaPE1sZv-Q76B0s58XlceYRYvRwAsA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"max_score":90,"count":10,"status":"inactive"}
   {"status":"active","count":10,"avg_score":60,"max_score":100}

📝 Example 4: Function Management

📋 Total scripts: 20
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KN2wxYgi4XEhrLRhs8iofsFxS3a5nHTnUqQ5ZNE4wmPPZzFBiaRAgzgXaXudyDSfJIPIHBuspsz_vFl3cqDXyw
Created Bob: $500 - ID: ls52avK0YFx_gUnjXEhfZVWFGIl_OiEUXleASmP424u73gbkTq9CeA5CD5au3w0jTrQCzpBaRHnxIEH6M7nC0g

=== Example 1: Begin Transaction ===
Transaction ID: 02e5ed3d-165b-4bef-b6fc-77642664e936

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: 6cb5f52f-6ece-4098-bc58-aeb0f5fa82fc
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_scripts.js ===[0m
🚀 ekoDB Complete CRUD Scripts Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each Script shows Functions chaining with proper verification
============================================================

============================================================
📝 Script 1: Insert + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: U1scSkBobIZyHQipcfzyLEdoA2q51r-anlNhrydoH5a2XQA99jkWOGaRBR3SAl2bG7yTxAVz1SOCP_bVLsV78g

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: tJG-pdX8zZlPK03-DyaCoE7yrtulmylampLEh-t1YTi044QqtR2oVYTh3Xk10c05mVnyRrQjqzDo6VMzZXRNKA

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: DkNobJRGFfBkpG7Uq3Lisdm7039nQx75j6u967mfGoPwx2My5Ox1pGqabSFVoLq84NS4p3pLPi9v762AopybeA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: hBsl1M4h-NHu1eLupRwtJTzzRx8lOvEUy9JZkmRNikwVEl5EzG_jy5TdcVgTjD26qnmKiRQa6JsF1Y2Zl-K5yA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: U1scSkBobIZyHQipcfzy...
   ✅ Deleted script: tJG-pdX8zZlPK03-DyaC...
   ✅ Deleted script: DkNobJRGFfBkpG7Uq3Li...
   ✅ Deleted script: hBsl1M4h-NHu1eLupRwt...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Scripts Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Scripts chain Functions together
   ✅ Each Script demonstrates operation + verification
   ✅ Parameters make Scripts reusable
   ✅ Verification is built into the Script itself
   ✅ Complete CRUD lifecycle in 4 focused Scripts
[32m✓ crud_scripts.js completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 995ms

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 817ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'HAIVMBgDQ75mLNLwWB4f7ZTiH17uQNFid1sdPhrBOoowX9orjrB-7e2rxE1qvc1i2yRhnAEMuGqtRJ5SLOLOtw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  id: 'HAIVMBgDQ75mLNLwWB4f7ZTiH17uQNFid1sdPhrBOoowX9orjrB-7e2rxE1qvc1i2yRhnAEMuGqtRJ5SLOLOtw',
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'HAIVMBgDQ75mLNLwWB4f7ZTiH17uQNFid1sdPhrBOoowX9orjrB-7e2rxE1qvc1i2yRhnAEMuGqtRJ5SLOLOtw'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: yHKciOmSIS6NoqVrXkRYpF9rmojGG0olFA2xkehMjy31_Uab3_8enSpz5ja6J8fdnAth_HfoBzPxDTspV_stfw

=== List Collections ===
Total collections: 22
Sample collections: schema_employees_client_ts,ttl_cache,client_collection_management_js,schema_products_client_py,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: RxZyppwubrdz2zb-_KMvR35liNCh-nfah9wivYZWZ7v9uNFz9hzpE51GMemqaMz-j9YIe08a5D7iNXNG7BxtRA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: MG5mfiv8TbVZeR8lWuUKkziZWAJiCxfj5AgTjflR4NTKP0OhWT9x-JpvqmBDBoqPUjJL6CeYdlZUYVkFjhXqwQ

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
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: zcflI3Sdk3FNl2v56D_xbz7OnulTsQjGaYVGmVdLR1Rlfp_h9ZZ0ohNxo1RnxYHDAs5xFstf8r2xW6DD9_S2-g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: a0R2SIqvRW1ip8KGriqWyaBbRQCzaspy5zs1YOg7h2AJ3BA-DjKoMCvWrZQ4xUXnW_Dp7jKtBIJ9qz3uqG6buw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: e8-zWTVrfFLUjv60AParDjsDnBRaTWhTTGKE3riT5-uFNkFicYaqgD7rV0-7pfnnXuXalmXwW1uEGR1slpcjyw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: pm9AZezphZkpps2QwJvtwULg79BhKsme2Vonfn-w5m5Jc3NV0dnrLH3eM6C-EYbmqtGHbDI3UCBrKcHUgHHn7Q
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: tx_p4ch5Qr_BR21qqyB8ZQe3L7gk-TAwPR_Vpu7o0WGH0B8JyYrGhMk_mDzERi3vL_qz-eGnmNfirMIZ-esF9g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","count":5,"avg_score":50,"max_score":90}
   {"avg_score":60,"status":"active","count":5,"max_score":100}

📝 Example 4: Function Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: A6ZKG5TNneGxclex-2R92_etfDJTbCdedgnT5N020EyhWWGNZLfexK-_850RO5J0z54XUj4SEc7BqCuxpQVgig
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","avg_score":60,"count":10}
   {"status":"inactive","count":10,"avg_score":50}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 24
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 20
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
🚀 ekoDB Advanced Scripts Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Found 2 categories
   Furniture: 4 items (avg $294.00)
   Electronics: 6 items (avg $325.67)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved
📊 Category analysis (2 categories):
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Monitor 27" - $399 (⭐4.6)
   2. Desk Chair - $349 (⭐4.2)
   3. Office Lamp - $79 (⭐4.05)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
🚀 ekoDB CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average scores by role:
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 10):
   1. User 4 (user4@example.com) - Score: 40
   2. User 6 (user6@example.com) - Score: 60
   3. User 3 (user3@example.com) - Score: 30
   4. User 2 (user2@example.com) - Score: 20
   5. User 1 (user1@example.com) - Score: 10
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples completed!
🚀 ekoDB Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Natural Language Processing
   2. Introduction to Machine Learning
   3. Database Design Principles
   4. Vector Databases Explained
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Introduction to Machine Learning
   3. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. Spatial Accuracy: They provide better spatial accuracy as they use points, lines, and polygons to represent spatial data.

2. Scalability: They maintain the same visual quality when zoomed in or out, unlike raster databases.

3. Attribute Storage: They can store associated attribute data directly, making them useful for complex data analysis.

4. Smaller Size: They often require less storage space than raster databases.

5. Flexibility: They allow for more complex operations such as network analysis and topology.

6. Editability: Vector data is easy to edit, making it ideal for applications where data needs to be updated regularly.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Draft Article" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 7Be9bZ46UVxZOt-gD8LkietCrfunFXn6MLxpxUEpQchmJO-xgLLs6BOJMMcn1KB6CJJ7eWgx17pN1I6D4wwnrA
Created Bob: $500 - ID: YCaDEdYT_KOhrr9Hn6-ghod__vK0xcYxX7LH5qlARhBfOYcwnwwC0UhmMj6v3KkYGcK3wdL7EBXS7KeQEHijTQ

=== Example 1: Begin Transaction ===
Transaction ID: c39b45d8-cd93-4cf1-8ea1-ed29d0955c8e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 4ec6af3f-e560-4c3f-993a-5200cb0c0d49
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mJavaScript client examples complete![0m
✅ [32mAll JavaScript integration tests complete![0m
🟣 [36mBuilding Kotlin client library...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE
> Task :jar UP-TO-DATE
> Task :assemble UP-TO-DATE
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test UP-TO-DATE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s
4 actionable tasks: 4 up-to-date
✅ [32mKotlin client built![0m
🧪 [36mRunning Kotlin client library examples...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: Record(fields={id=StringValue(value=fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA)})
  User ID: fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA

=== Read ===
✓ Found user by ID: Record(fields={embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), id=StringValue(value=fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Jan 04 14:39:10 EST 2026)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Jan 04 14:39:10 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [embedding, data, active, metadata, user_id, categories, id, age, created_at, tags, email, price, name]

=== Update ===
✓ Updated user: Record(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 14:39:10 EST 2026), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 14:39:10 EST 2026), type=StringValue(value=String)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), id=StringValue(value=fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})

=== Delete ===
✓ Deleted user with ID: fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: NI5-q-JWYZEcgBt3usBylsRJJQa9-UHKc-q1XtsHyxktYcL3j153De4Bh4RrZV8B6BdRwFgiV-szTkJ6Mvb2Fw, 5-0DBd6FrZtskj9rrWTAc33QSAzZEJtxrO5cP-yaGikG8huweFHes9x2leBdlTy-O4SEbjcoRl3KiWP79geSWw, w5IyqyYk3IftqZ8QwEVEuQzx3wp38H3cXpxSk_bG49yYApjHDF-aQx07IY3mK4DZvS7EpE46b5bk9UNF-gT4WA...

=== Batch Update ===
✓ Updated 3 records

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') ===
✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) ===
✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) ===
✓ Top 3 scores:
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=95)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Charlie)})
  - ObjectValue(value={value=StringValue(value=Diana), type=StringValue(value=String)})

=== Query 7: Contains (name contains 'a') ===
✓ Found 2 names containing 'a'

=== Cleanup ===
✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"role":"admin","email":"alice@example.com","name":"Alice"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1767555564170}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 5

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

=== Cleanup ===
✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 21 collections
  - schema_employees_client_ts
  - ttl_cache
  - schema_products_client_py
  - test_accounts
  - schema_users_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-04T19:39:28.465115Z","last_modified":"2026-01-04T19:39:28.465115Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: STOIja0m3pBmA8ueNF-ttDyQHW_sZODcHFqGKWbQSGQ_9ttDSGDg2biZqUqg57hNVHZSQoBnWaYo_Bqi2N24lg

=== Verify Document Exists ===
✓ Document found: [session_id, created_at, user_id, id, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: RxK2Wv0J3GW7mllXuwz9reshsIX5L4OMA2AZshW_AmLByoLoogDQ1IG9c2EIkim0PGfx1r2DI3KQpbRxCglDUA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [value, id, cache_key, ttl]

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"a5XPfPtETyg2z3nS7gfVpNaVrXPRkpAZC2IZ3I3u7dosAucC3zTCVtqzbkfKGv1aLJhbwNNNgpJlJikKcJqzRQ","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=6AachGyt_8dA-_ghnfusBNdnDutC_eDj_0G0ka_uHv03zS_l78BdAganX-S9tinxvPVNIotMvdnDsCtwZo4G2Q)
✓ Inserted user 2: StringValue(value=38ZxG1FY8yiNviJeeKHWVk9x8-MgCVQxfS9fM4KR6MIe2C7o5qars3lWjTzos_Ywk8hAhZlsSV0cBzrumDfzFQ)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_employees_client_ts, ttl_cache, schema_products_client_py, test_accounts, schema_users_client_go]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"tags":"programming,rust,tutorial","title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","views":762,"id":"tmT80J3B0Ee_ijb_OyzfU-pt8cRQRZZ-qgYygm5rnaCS8j5GFJ01tSQENBgE1D9wK0tMWXI-wBze6GuVS9TVgA"},"score":19.8,"matched_fields":["title","description","tags"]},{"record":{"description":"Master Python for data analysis, machine learning, and visualization.","views":398,"tags":"programming,python,data-science","id":"1WDOLOTbZgRZbjs2mJNdpJhHNfMmUnJWMx-stDBge3FQ-BKFa03odlQ6ZMcsWWr0FgTKYAea-fHZzZTBSxFMkA","title":"Python for Data Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","id":"mPnAMR-iLKpRahGXNVzLt0KPUWszpt-XSpqQZCJ29E1HFLZHvNOwelJ3XopSm3nEmbF7pUup1V-BcmiGNAlZXw","tags":"programming,javascript,web","title":"JavaScript Web Development","views":759},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":"ai,machine-learning,python","views":853,"title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","id":"CahleZ-sZxB-8LUlHcFBC5I5dJ0Kf8tfuilUkmD9fISifjmoyh_uxW6TbatMRJH5zIH2MmlWHCKdEbHWmGPGMg"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","views":398,"id":"1WDOLOTbZgRZbjs2mJNdpJhHNfMmUnJWMx-stDBge3FQ-BKFa03odlQ6ZMcsWWr0FgTKYAea-fHZzZTBSxFMkA","tags":"programming,python,data-science"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"l_e3rTmfJVfzAmLfoXPmFFnYEG1xOieASooZNUxnNgYvMpx6BwF9ps2PcBPX4L_J_9SmVCX2Y7fYb5QpOQoSQw","title":"Database Design","views":805,"tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization."},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial","views":762,"id":"tmT80J3B0Ee_ijb_OyzfU-pt8cRQRZZ-qgYygm5rnaCS8j5GFJ01tSQENBgE1D9wK0tMWXI-wBze6GuVS9TVgA","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=QAHuUBCO4FK_aSITRshWon95aFLswnGmM8rFneBfA7TDn92FGtULzAR0HKrVNAJ5Z7yukKme1Sj1wxrVwshvAw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767555589870},"id":"QAHuUBCO4FK_aSITRshWon95aFLswnGmM8rFneBfA7TDn92FGtULzAR0HKrVNAJ5Z7yukKme1Sj1wxrVwshvAw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-04T20:39:49.011800Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Join Operations Example ===

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

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: BupP1oojiZ-6M5vOKcpzBsuDrwlLYn7GYJ21z_O5OuuyolTje7eVwv2T23z0DM1ss8DC1NT2WEkbNFglVfEspg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "RMtmkTNycMNaaf83_P9cm0lZWG3FzuvHYln_-F36DT3jZn6JBCgp65DqaDm_ogxI4T14Gq6DUNc2aDmNq989uA"
  Responses: ["ekoDB is a high-performance database with intelligent caching and real-time capabilities. It has integrated AI features that contribute to its robust functionality.\n\nOne of its key features is the AI Chat Integration. This feature allows you to query your database using natural language and get AI-powered responses with relevant context, enhancing the user's interaction with the database.\n\nekoDB also supports various types of searches including full-text search, vector search, and hybrid search. These capabilities, combined with automatic context retrieval, make it a powerful tool for finding and managing data.\n\nThese are just some of the features of ekoDB, and more details can be found in the rest of the documentation."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 7s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Answer: The product available is ekoDB, a high-performance database product with AI capabilities. The price of this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB, a high-performance database product with AI capabilities, is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"QNwXRJb6uu7HJUNKqhw5bFVl6EH7CrkOIn-vZIpy8IkfNezXcJlGRqybeQE2KRBdQ0hGO1hdNzSQAHz6oGGwYQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T19:40:08.548206Z"},"id":"RSrJRxnPdhC04bW7IResK47om1BS5RN28uaGcddclmnovYjI69inR-hADcCgBmb6KDd0x5ss5Fvtpq2SiSyL0A","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":28,"prompt_tokens":242,"total_tokens":270}},"updated_at":{"type":"DateTime","value":"2026-01-04T19:40:08.548206Z"}},{"chat_id":{"type":"String","value":"px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA"},"content":{"type":"String","value":"Answer: The product available is ekoDB, a high-performance database product with AI capabilities. The price of this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"QNwXRJb6uu7HJUNKqhw5bFVl6EH7CrkOIn-vZIpy8IkfNezXcJlGRqybeQE2KRBdQ0hGO1hdNzSQAHz6oGGwYQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T19:40:08.553113Z"},"id":"ezzP5pbW0pJIzv3RSF5teDxG4oCeRwwy6KSVW4qnHEXfAAOBwQ0Ypyw751PveK-NZPK_6PjqItNOyHXfD7XyIQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":28,"prompt_tokens":242,"total_tokens":270}},"updated_at":{"type":"DateTime","value":"2026-01-04T19:40:08.553113Z"}},{"chat_id":{"type":"String","value":"px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"QNwXRJb6uu7HJUNKqhw5bFVl6EH7CrkOIn-vZIpy8IkfNezXcJlGRqybeQE2KRBdQ0hGO1hdNzSQAHz6oGGwYQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T19:40:09.471904Z"},"id":"eI6xLJnyjNNvyLFKnre8PDj03UP2XxDOXm7UIoLeLP1VcHmnElwfYyFogpFTJgUFk3M4wbx9higqF5POxz7zIQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":242,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-04T19:40:09.471904Z"}},{"chat_id":{"type":"String","value":"px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA"},"content":{"type":"String","value":"The price of the ekoDB, a high-performance database product with AI capabilities, is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"QNwXRJb6uu7HJUNKqhw5bFVl6EH7CrkOIn-vZIpy8IkfNezXcJlGRqybeQE2KRBdQ0hGO1hdNzSQAHz6oGGwYQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T19:40:09.476876Z"},"id":"BjhQtYPbFz_BpfUPlGk2aPWH5WSIzP_H464n1GyHmtu-q4bemTbn1rwmBqOHfILd0_nyP8GlKOlK6bLTQ7gKVQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":242,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-04T19:40:09.476876Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: b-s3tzB_uaG3KEe2a-nC7BENoeubDh97wg_QdryhpYt4twP5aU690aV16s6gtsz95SnsweLPuwQ6GeF4--7jvg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6IKFI8iGciI54PKtHMilQ3RPlJNStPlbN0HCWe8emU7rsZMmFoa_QXoQol4F0YJ-RLv4AeuK_hH-G760sNrCLg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, one product that is available is ekoDB. It is a high-performance database product and it costs $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information available, we have a high-performance database product named \"ekoDB\". It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: gs9xkjzev8zBeINr5Ejlcb7ZZ3KmpZjJ__9DJavWK8I-kOmgjd9C8Fwzgwch1l1UG1PWrq30YN10D1pNtpnRPw

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 7s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: BWINm5TAKq9RmjslMExUsBWnMXc0JzXXrEXh8rD4magbzOnC9sK8yQFeCuNhmldQcCMHSPQzIBTtlQ53W3xzww
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"inactive","count":5,"avg_score":50.0}
   {"count":5,"status":"active","avg_score":60.0}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Script listing skipped (some scripts contain parameter placeholders)
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
⚠️  Script cleanup skipped (some scripts contain parameter placeholders)

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: bePc7rbwK-_Ibw8ELCXXUDt3ctitVysEA9uY8YBWclkEiDHbGLkKp5mTTKow1wBe6bbF22NXvWm64EQVjdkAoQ
Created Bob: $500 - ID: 2ywLNSmu5625eNJurWpW2nVHlz4MKoXWuLCXC7-voFt1sgeXukeXWZQrR8AULc_O8Jp5Q8sLgrQj8Fcg-kFawQ

=== Example 1: Begin Transaction ===
Transaction ID: 055d36a6-2b9c-4121-aadc-b8f23810abfc

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b516b8c9-f34c-4c2f-9921-5519aef3d0e5
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mAll Kotlin integration tests complete![0m

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m
✓ Rust client built

[36mBuilding Python client bindings...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
✓ Python client built and installed

[36mBuilding TypeScript client library...[0m
✓ TypeScript client built

[36mInstalling TypeScript client in examples...[0m
✓ TypeScript client installed

[36mBuilding TypeScript example...[0m
✓ TypeScript example built

[36mBuilding Go client library...[0m
✓ Go client built

[36mBuilding Go RAG example...[0m
✓ Go example built

[36mBuilding Kotlin client library...[0m
✓ Kotlin client built

[36mBuilding Kotlin RAG example...[0m
✓ Kotlin example built

[36mRunning Rust RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 340.199ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.085955458s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 270.495ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 831.032167ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 284.691125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 960.270416ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 296.677875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 361.374375ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 317.596375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.064772833s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.159537208s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 538.265375ms
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 551.580959ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 107.505084ms

✓ Found 5 related messages across all conversations:
  1. From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. From conv_database_design
     What is database normalization?

  3. From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. From conv_database_design
     When should I use NoSQL over SQL?

  5. From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe high-performance database code involves several strategies. Here are a few common practices:

1. **Optimize Your Queries**: This includes using indexes properly, minimizing the amount of data that needs to be read, and avoiding unnecessary queries. Indexes can greatly speed up data retrieval, but they slow down data modification. 

2. **Normalize Your Database**: As mentioned in Context 2 and 3, database normalization can help reduce redundancy and improve data integrity. This generally improves performance and makes your database structure more efficient.

3. **Use Appropriate Database Type**: Depending on the requirements, SQL or NoSQL might be more suitable. SQL databases are good for structured data and complex queries, and offer ACID transactions. NoSQL databases are often used for their flexibility, horizontal scaling, high write throughput and when dealing with unstructured data (Context 1 and 4).

4. **Manage Database Connections**: Database connections are expensive to establish and maintain. Use a connection pool to reuse existing connections, rather than creating a new connection every time you need to query the database. 

5. **Memory Management**: Use good coding practices to manage memory, such as disposing objects when they are no longer needed, using using statements in C# or try-with-resources in Java to automatically close resources, and avoiding memory leaks.

6. **Concurrent Processing**: Use asynchronous programming models where possible to avoid blocking the main thread, which can improve performance.

7. **Use Caching**: Caching frequently accessed data can reduce the load on the database and improve performance. Be careful managing the cache to avoid stale data.

8. **Batch Operations**: Where possible, batch database operations together. This can be more efficient than performing many individual operations.

9. **Use Prepared Statements**: Prepared statements not only increase security by preventing SQL injection, but also often improve performance as they can be compiled once and executed multiple times. 

10. **Use Profiling Tools**: Use database profiling tools to identify bottlenecks in your database queries. They can provide detailed information about how your queries are performing and where improvements can be made.

Remember, every database and application is different, so what works well in one situation might not work as well in another. Always test your changes to ensure they actually improve performance.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 238.734ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2504 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 320.019417ms
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing text_search()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 67.107083ms

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using find_all() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!


=== Cleanup ===
Deleting example collections...

✅ All done! RAG system demonstrated successfully.

✓ Using search results to enhance AI responses (RAG)
✓ Building a self-improving knowledge base
✓ Dynamic search configurations per conversation

ekoDB provides everything needed for AI-powered applications:
  • Vector search (semantic similarity)
  • Text search (keyword matching)
  • Hybrid search (best of both)
  • AI functions (Chat, Embed)
  • Flexible querying and filtering
  • All in one database - no external dependencies!


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.227s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.485s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.209s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.001s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.403s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.215s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.973s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.091s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     What is database normalization?

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves a few key principles:

1. **Database Normalization**: As mentioned in Context 1, database normalization is a process that reduces redundancy and improves data integrity, which can lead to better performance. By dividing large tables into smaller ones and defining relationships between them, you can reduce the amount of data that needs to be retrieved and processed, improving memory efficiency.

2. **Choosing the Right Database Type**: Depending on the needs of your application, you might choose either a NoSQL or SQL database. As per Context 2, NoSQL databases are great for flexible schemas, horizontal scaling, high write throughput, and unstructured data. On the other hand, SQL databases excel at complex queries, ACID transactions, and structured data with well-defined relationships. Your choice can impact the performance and memory usage of your database operations.

3. **Optimizing Queries**: This can involve a range of strategies, from using indexes to speed up data retrieval, to carefully structuring queries to retrieve only the data you need. Avoid using SELECT * queries and instead specify the columns you need. If your database supports it, you can also use caching to improve performance. 

4. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks but also offer performance advantages as they're parsed once but can be executed multiple times with the same or different parameters.

5. **Connection Pooling**: Instead of opening and closing a database connection for each user, use a pool of connections and reuse them. This can significantly save resources and improve performance.

6. **Avoid N+1 Query Problem**: In ORM libraries, N+1 query problem is a common issue where the system executes N unnecessary queries. It can be avoided using techniques like eager loading.

Remember, always test your database with realistic loads to ensure it can handle the demands of your application. Also, regularly review and update your code as necessary to adapt to changes in your data and usage patterns.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2124 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.211s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves a few key principles:

1. **Database Normalization**: As mentioned in Context 1, database normalization is a process that reduces redundancy and improves data integrity, which can lead to better performance. By dividing large tables into smaller ones and defining relationships between them, you can reduce the amount of data that needs to be retrieved and processed, improving memory efficiency.

2. **Choosing the Right Database Type**: Depending on the needs of your application, you might choose either a NoSQL or SQL database. As per Context 2, NoSQL databases are great for flexible schemas, horizontal scaling, high write throughput, and unstructured data. On the other hand, SQL databases excel at complex queries, ACID transactions, and structured data with well-defined relationships. Your choice can impact the performance and memory usage of your database operations.

3. **Optimizing Queries**: This can involve a range of strategies, from using indexes to speed up data retrieval, to carefully structuring queries to retrieve only the data you need. Avoid using SELECT * queries and instead specify the columns you need. If your database supports it, you can also use caching to improve performance. 

4. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks but also offer performance advantages as they're parsed once but can be executed multiple times with the same or different parameters.

5. **Connection Pooling**: Instead of opening and closing a database connection for each user, use a pool of connections and reuse them. This can significantly save resources and improve performance.

6. **Avoid N+1 Query Problem**: In ORM libraries, N+1 query problem is a common issue where the system executes N unnecessary queries. It can be avoided using techniques like eager loading.

Remember, always test your database with realistic loads to ensure it can handle the demands of your application. Also, regularly review and update your code as necessary to adapt to changes in your data and usage patterns.

=== System Statistics ===
Total conversations: 4
Total messages stored: 14
All messages are indexed for vector search ✓
All messages are indexed for text search ✓
All messages are queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybrid_search() - Semantic + keyword search
  • client.text_search() - Full-text search
  • client.find_all() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.936s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.224s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.259s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.238s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.345s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.213s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.124s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.264s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.270s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.206s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.093s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe high-performance database code requires a combination of understanding the database system you're working with, the structure and normalization of your data, and the specific needs of your application. Here are some guidelines:

1. **Understand Your Database**: If you're using a SQL database, understand how it handles memory and resources. This often involves understanding how the database uses indexing, how it handles transactions, and how it manages connections.

2. **Data Normalization**: As mentioned in Context 1, database normalization can help to reduce redundancy and improve data integrity. This can lead to performance improvements, as less redundant data means less data to process and store.

3. **Query Optimization**: The way you write your queries can have a significant impact on performance. Try to write efficient queries that only fetch the data you need. Avoid using `SELECT *` and instead specify the exact columns you need. Use JOINs judiciously and understand their impact on performance. Indexing is also a key part of query optimization. Use indexes for columns that are frequently used in WHERE, ORDER BY, and JOIN conditions.

4. **Use Appropriate Database Type**: As per Context 3 and 4, the choice between NoSQL and SQL can have significant implications for performance. SQL databases are better for complex queries and structured data with well-defined relationships. NoSQL databases can offer better performance for certain use cases, especially when dealing with large amounts of unstructured data, or when you need high write throughput or horizontal scaling.

5. **Manage Connections and Sessions**: Each database connection consumes resources. Opening and closing connections frequently can cause performance issues. Use connection pools to manage and reuse database connections. Also, be mindful of long-lived transactions as they can hold resources and cause performance issues.

6. **Memory Management**: Be mindful of how your code uses memory. Avoid loading large amounts of data into memory at once. Use streaming or pagination to handle large result sets. Also, ensure you're properly closing any resources (like database connections or cursors) that you open.

Remember, the key to writing high-performance database code is understanding your specific needs and constraints, and then applying these principles as needed.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.320s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2388 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.058s
✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe high-performance database code requires a combination of understanding the database system you're working with, the structure and normalization of your data, and the specific needs of your application. Here are some guidelines:

1. **Understand Your Database**: If you're using a SQL database, understand how it handles memory and resources. This often involves understanding how the database uses indexing, how it handles transactions, and how it manages connections.

2. **Data Normalization**: As mentioned in Context 1, database normalization can help to reduce redundancy and improve data integrity. This can lead to performance improvements, as less redundant data means less data to process and store.

3. **Query Optimization**: The way you write your queries can have a significant impact on performance. Try to write efficient queries that only fetch the data you need. Avoid using `SELECT *` and instead specify the exact columns you need. Use JOINs judiciously and understand their impact on performance. Indexing is also a key part of query optimization. Use indexes for columns that are frequently used in WHERE, ORDER BY, and JOIN conditions.

4. **Use Appropriate Database Type**: As per Context 3 and 4, the choice between NoSQL and SQL can have significant implications for performance. SQL databases are better for complex queries and structured data with well-defined relationships. NoSQL databases can offer better performance for certain use cases, especially when dealing with large amounts of unstructured data, or when you need high write throughput or horizontal scaling.

5. **Manage Connections and Sessions**: Each database connection consumes resources. Opening and closing connections frequently can cause performance issues. Use connection pools to manage and reuse database connections. Also, be mindful of long-lived transactions as they can hold resources and cause performance issues.

6. **Memory Management**: Be mindful of how your code uses memory. Avoid loading large amounts of data into memory at once. Use streaming or pagination to handle large result sets. Also, ensure you're properly closing any resources (like database connections or cursors) that you open.

Remember, the key to writing high-performance database code is understanding your specific needs and constraints, and then applying these principles as needed.

=== System Statistics ===

→ Querying database statistics...
  • Using findAllWithLimit() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybridSearch() - Semantic + keyword search
  • client.textSearch() - Full-text search
  • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.210s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.179s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.478s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.518s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.284s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.254s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.394s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.257s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.235s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.099s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several practices and principles:

1. **Understand Your Data**: Design your database well using principles such as database normalization (as mentioned in Context 1) to reduce redundancy and improve data integrity. This can lead to more efficient use of memory and faster queries. 

2. **Choose the Right Database Type**: Depending on the nature of your data, you may choose between SQL and NoSQL databases (as mentioned in Context 3 and 4). SQL databases provide strong ACID properties, work well with structured data and complex relationships, and support complex queries. NoSQL databases provide flexible schemas, can scale horizontally, and handle high write throughput and unstructured data.

3. **Optimize Your Queries**: Poorly written queries can consume excessive memory and CPU cycles. Use indexing wisely to speed up data retrieval. Avoid expensive SQL operations like wildcards at the beginning of LIKE patterns, unnecessary joins, or returning more data than necessary. Use LIMIT to restrict the size of the result set.

4. **Use Connection Pooling**: This reduces the overhead and latency of establishing a new connection for every query.

5. **Avoid Memory Leaks**: Ensure you close all database connections and free up any resources after use. Use garbage collection and memory profiling tools to detect and fix memory leaks.

6. **Batch Operations**: Rather than writing or updating one record at a time, batch your writes and updates to reduce the number of individual database operations.

7. **Use Appropriate Data Types**: Using appropriate data types not only save memory but also improve performance because operations with smaller data types are usually faster.

8. **Test and Monitor Performance**: Use tools to monitor and analyze the database performance. Regularly check slow query logs and optimize those queries.

9. **Concurrency Control**: Use transaction isolation levels appropriately to prevent issues like dirty reads or phantom reads. This ensures data integrity while allowing for concurrent access, improving overall performance.

Remember, achieving high performance while maintaining memory safety is all about balance and depends heavily on the specific requirements of your application. Always profile and measure before and after making changes to ensure improvements.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2373 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.284s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing TextSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.048s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using FindAll() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.Embed(text, model) - Generate embeddings
  • client.HybridSearch() - Semantic + keyword search
  • client.TextSearch() - Full-text search
  • client.FindAll() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Kotlin RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.77s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.222s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.378s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.212s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.216s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.223s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.296s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.313s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.469s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.239s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.09s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.042s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using findAllWithLimit() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 13
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybridSearch() - Semantic + keyword search
  • client.textSearch() - Full-text search
  • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mRAG Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw across 5 languages:[0m
  ✓ Embeddings generated via ekoDB Functions
  ✓ Hybrid search (semantic + keyword)
  ✓ Text search with stemming
  ✓ Cross-conversation context retrieval
  ✓ Simple client helpers wrapping powerful AI

[36mMission: AI for All 🚀[0m - Making RAG accessible to everyone!

✅ [32mAll RAG examples complete! Output saved to test-examples-rag.md[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
🌐 [36mTesting SWR (Stale-While-Revalidate) Pattern Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (Zidh_d9dxpb79kGfnPO2cQ1tKWDAJS-oG4bAm2VD5YqvzxmA-On8-mUTyLTV8XyC6_upRnHmFK0AnrhEJk6nEw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "data": {
        "value": {
          "html_url": "https://github.com/torvalds",
          "public_repos": 9,
          "created_at": "2011-09-03T15:26:22Z",
          "company": "Linux Foundation",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "bio": null,
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "type": "User",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "user_view_type": "public",
          "following": 0,
          "hireable": null,
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "public_gists": 1,
          "site_admin": false,
          "id": 1024025,
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "email": null,
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "blog": "",
          "twitter_username": null,
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "followers": 269925,
          "gravatar_id": "",
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "updated_at": "2025-11-24T04:16:14Z",
          "name": "Linus Torvalds",
          "url": "https://api.github.com/users/torvalds",
          "location": "Portland, OR",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "login": "torvalds"
        },
        "type": "Object"
      },
      "cached_at": "2026-01-04T19:42:02.672Z",
      "id": "Vaf67TRBOPX1c8VYZsJzgj2JwQXOqFwue86JEozWop28_hdQGo89PshLFR6bAO0ry2BufP8yWHgupvGUZVowFw"
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 86ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (UdpE2tfrzZ5_CnQY4A7SmvfA0k8SreKLhd4XGaxmkek00HEIey2cAmGZDPi-Z6dGvKNCQPVSxzePWFopcHN_FA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_data": {
        "type": "Object",
        "value": {
          "meta": {
            "createdAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "title": "Essence Mascara Lash Princess",
          "sku": "BEA-ESS-ESS-001",
          "rating": 2.56,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "id": 1,
          "returnPolicy": "No return policy",
          "discountPercentage": 10.48,
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "shippingInformation": "Ships in 3-5 business days",
          "stock": 99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "brand": "Essence",
          "minimumOrderQuantity": 48,
          "warrantyInformation": "1 week warranty",
          "reviews": [
            {
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "comment": "Very satisfied!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon",
              "rating": 4
            },
            {
              "rating": 5,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "category": "beauty",
          "availabilityStatus": "In Stock",
          "weight": 4
        }
      },
      "enriched_at": "2026-01-04T19:42:02.963Z",
      "id": "a1gcga3iAwVu9z8vGQF437_8dIW77iXmKy2h70hcKFsthBCtH14I4c6fscE7p9o5NJwEER7M3V3i8TcY6CrEiw"
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: FRPdsHuMBUzWvJkxaN8wEhNzJPLa261DXJRz-rXLxbApKrvZZG0XLT_IClJKd8KXRfdPWLwprBIng7OvwbXW0g

Call 1: Cache miss (fetches from API)
Response time: 583ms
Result: {
  "records": [
    {
      "data": {
        "value": {
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "elevation": 32,
          "longitude": -73.99309,
          "utc_offset_seconds": 0,
          "current": {
            "interval": 900,
            "temperature_2m": 1.6,
            "time": "2026-01-04T19:30"
          },
          "timezone_abbreviation": "GMT",
          "generationtime_ms": 0.023484230041503903,
          "latitude": 40.710335,
          "timezone": "GMT"
        },
        "type": "Object"
      },
      "id": "Yqw3O_wHXRmiULbhDxZswUvE_OdaceNLgtSXlpvLojwgH9e2g8TXhbfJwdkcSxqeYDrNIuoUgKzSG37e08XMTg",
      "cached_at": "2026-01-04T19:42:03.428Z"
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 539ms (1.1x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-04T19:42:03.428Z",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "time": "2026-01-04T19:30",
            "interval": 900,
            "temperature_2m": 1.6
          },
          "elevation": 32,
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.016927719116210938,
          "longitude": -73.99309,
          "latitude": 40.710335,
          "timezone": "GMT"
        }
      },
      "id": "AiNkOtX9Xuf9ncA4gj4B_7ImIEQvWAQ9nHMIec6qJ0fabt7NDGJK_cid1z4jPrnd-mNGC9KsCDrGxPta2yEVXQ"
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

✅ [32mTypeScript SWR examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.7.1
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (GqHopheX1-QgsxMvgtlNev3SkO4c41xwkWETyimWWSZyWOdJ7ULA-HSI-JZsESICbZhI33OumsqY8D3Ivl7Dbw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 77ms
Result: [
  {
    "cached_at": "2026-01-04T19:42:02.672Z",
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 269925,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 9,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2025-11-24T04:16:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "Ts2R6yKFfa5-ylafKMkoJLTMW8rxLn8aME61GJMbkRRgPSh3Stuxi1l6JrMVqUnm4liKHm6nQwCN_KhjLN6PNQ"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 89ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (vz5z9cDryldbh5ZA52mP-xtlM4DXotxS-LkbeQ1PHoOzmZ2dRJDC-GdKi03bg2FABtzmpvGgwDiQxt7APLUzeg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767555726",
    "enriched_data": {
      "type": "Object",
      "value": {
        "availabilityStatus": "In Stock",
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "discountPercentage": 10.48,
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "minimumOrderQuantity": 48,
        "price": 9.99,
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "weight": 4
      }
    },
    "id": "hgpDJQLLGDmo-YM8x3i_uTHmAk2wekGimN_OENJBIkzH_w5d8i-x0BJsk21DzC8W8jdI2z6Sq_jLNOrJDG0mkQ"
  }
]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mPython SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (YC9K0d6Vj9KYi5JqrtQptyExwT0T-gMiLyMDso9gAw6CVqKATLaJpOtd5JZIW8R59PF0s5ZXj1AIklQ7IZokMw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 86.235334ms
Result: [
  {
    "cached_at": "2026-01-04T19:42:02.672Z",
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 269925,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 9,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2025-11-24T04:16:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "o2Nr6gY1PLEDfqqv9TR4M5iAQ9-iOnbYrIE2e8P6zJhTyk3BEW--WA4Xx2WEB1CR7jCGopXSAgVegVJYC_THOA"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 80.125917ms (1.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (Hz0P9EP5gIY-T0hnT9ODUdNcoM6MByACWYSH4ukpuII-unoakhct9jD6TfYBmO_AcyiY0RA9Odj7setd8g6NqA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "2026-01-04T14:42:07-05:00",
    "enriched_data": {
      "type": "Object",
      "value": {
        "availabilityStatus": "In Stock",
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "discountPercentage": 10.48,
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "minimumOrderQuantity": 48,
        "price": 9.99,
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "weight": 4
      }
    },
    "id": "N17h2k9H9QHY5sn7p661H8TrDUAPU-JPW0uKHtMDCcU-x2BEXP95kHGnFV3wtRdSlAA0XvDxG-znro5JNZZ2Ew"
  }
]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mGo SWR examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (6etHqPoI0_Bd_1Cet8cmYVuQpQ4-Aq4edzAwNZ5m2uBhujSFIwIA0ryHaN12kX2do3Ah2rHsRpNeJd9KehHdDA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 95ms
Result: {
  "id": "ycRPnsesKLdSTCGJ0yLjdq7LYh5Miv9cwvGLqHzyovzv8IJq1oe3LNDenbx6uz3QfpSMYoviD417DtcT6D3yvg",
  "data": {
    "value": {
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "public_repos": 9,
      "name": "Linus Torvalds",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "created_at": "2011-09-03T15:26:22Z",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "following": 0,
      "gravatar_id": "",
      "bio": null,
      "login": "torvalds",
      "blog": "",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "followers": 269925,
      "html_url": "https://github.com/torvalds",
      "company": "Linux Foundation",
      "twitter_username": null,
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "url": "https://api.github.com/users/torvalds",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "site_admin": false,
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "type": "User",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "public_gists": 1,
      "updated_at": "2025-11-24T04:16:14Z",
      "location": "Portland, OR",
      "id": 1024025,
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "hireable": null,
      "user_view_type": "public",
      "email": null
    },
    "type": "Object"
  },
  "cached_at": "2026-01-04T19:42:02.672Z"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 91ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (Txs4IAczB8QkLxANOCgfH2CHEMb3ZB1gQdcKi2K4gKEQBxEhFb36ECGKgK6jzD_1SwCAOOaKGjrp-mJ43T9ieQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "id": "MGE6ZRd2hToas4Qw-vem1fDdlt3ykr_jBA5k5TvCX1mvn8umntNVigfoagetishcdI55vTzmClxxh3o3kgOfvQ",
  "enriched_data": {
    "type": "Object",
    "value": {
      "price": 9.99,
      "availabilityStatus": "In Stock",
      "weight": 4,
      "id": 1,
      "rating": 2.56,
      "warrantyInformation": "1 week warranty",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "minimumOrderQuantity": 48,
      "shippingInformation": "Ships in 3-5 business days",
      "meta": {
        "createdAt": "2025-04-30T09:41:02.053Z",
        "updatedAt": "2025-04-30T09:41:02.053Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "barcode": "5784719087687"
      },
      "stock": 99,
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "returnPolicy": "No return policy",
      "dimensions": {
        "height": 13.08,
        "width": 15.14,
        "depth": 22.99
      },
      "sku": "BEA-ESS-ESS-001",
      "category": "beauty",
      "brand": "Essence",
      "reviews": [
        {
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "rating": 3,
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Would not recommend!"
        },
        {
          "reviewerName": "Lucas Gordon",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "rating": 4,
          "comment": "Very satisfied!"
        },
        {
          "comment": "Highly impressed!",
          "reviewerName": "Eleanor Collins",
          "rating": 5,
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        }
      ],
      "tags": [
        "beauty",
        "mascara"
      ],
      "discountPercentage": 10.48,
      "title": "Essence Mascara Lash Princess",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ]
    }
  },
  "enriched_at": "2026-01-04T14:42:07-05:00"
}
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mRust SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Kotlin SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (YgTjWOeGUu_qPzJ8cRvsOC9-CJEWFbmitlULmiZ-fi8wssEe6MQsNzfZJwtgos0_q690QRxW7g36hcGbhv2rhw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 92ms
Result: [{"data":{"value":{"updated_at":"2025-11-24T04:16:14Z","name":"Linus Torvalds","login":"torvalds","email":null,"public_gists":1,"hireable":null,"followers_url":"https://api.github.com/users/torvalds/followers","received_events_url":"https://api.github.com/users/torvalds/received_events","site_admin":false,"url":"https://api.github.com/users/torvalds","html_url":"https://github.com/torvalds","following_url":"https://api.github.com/users/torvalds/following{/other_user}","created_at":"2011-09-03T15:26:22Z","organizations_url":"https://api.github.com/users/torvalds/orgs","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","twitter_username":null,"events_url":"https://api.github.com/users/torvalds/events{/privacy}","bio":null,"node_id":"MDQ6VXNlcjEwMjQwMjU=","location":"Portland, OR","public_repos":9,"repos_url":"https://api.github.com/users/torvalds/repos","user_view_type":"public","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","blog":"","gravatar_id":"","id":1024025,"type":"User","following":0,"followers":269925,"company":"Linux Foundation"},"type":"Object"},"cached_at":"2026-01-04T19:42:02.672Z","id":"taxZsRXXYYRvcmtEd_iv0SufK4-kAGXGWJbmuzGnV0ECLUFPaWUAXZvUuOUb5nSAp9vh1LWmx8D0thlLh1xCOw"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 87ms
Cache hit was 1.1x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (nhhNDoLUkRxhwerzPPZPOzLiQ-l4HZtZ90a_i0rOhdvLPWIqe9WxL2gxiYut7CCVk9JYrSaJIdHldBj5-XRuTA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"discountPercentage":10.48,"tags":["beauty","mascara"],"returnPolicy":"No return policy","reviews":[{"reviewerEmail":"eleanor.collins@x.dummyjson.com","rating":3,"date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins","comment":"Would not recommend!"},{"reviewerEmail":"lucas.gordon@x.dummyjson.com","reviewerName":"Lucas Gordon","date":"2025-04-30T09:41:02.053Z","rating":4,"comment":"Very satisfied!"},{"reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z","rating":5,"reviewerName":"Eleanor Collins","comment":"Highly impressed!"}],"dimensions":{"width":15.14,"height":13.08,"depth":22.99},"id":1,"meta":{"barcode":"5784719087687","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","updatedAt":"2025-04-30T09:41:02.053Z","createdAt":"2025-04-30T09:41:02.053Z"},"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","availabilityStatus":"In Stock","price":9.99,"minimumOrderQuantity":48,"title":"Essence Mascara Lash Princess","warrantyInformation":"1 week warranty","images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"brand":"Essence","description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","shippingInformation":"Ships in 3-5 business days","sku":"BEA-ESS-ESS-001","weight":4,"category":"beauty","rating":2.56,"stock":99}},"enriched_at":"2026-01-04T14:42:07-05:00","id":"7dnEQn1jLC6NIPOgLtwtyCZNcoatAf7EYLqJdiNLW7TpMhpRpRVr9-evMeZ7RzmNLmLVm9NFB3r-wrX8VrQ9yg"}]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!


BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll SWR Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw - ekoDB as Edge Cache:[0m
  ✓ FindById → Check cache
  ✓ If/Else → Conditional execution
  ✓ HttpRequest → External API calls
  ✓ Insert with TTL → Auto-expiring cache
  ✓ Sub-millisecond cache hits
  ✓ No Redis, no CDN, no cache invalidation logic needed

[36mYour DATABASE is your EDGE! 🚀[0m


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
🔗 [36mTesting Function Composition Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: Some(Object({"value": String("User 1"), "type": String("String")}))
   Department: Some(Object({"type": String("String"), "value": String("engineering")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 82.597ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82.296042ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: None
   Department: None
   Record count: Some(Integer(1))

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ All composition examples completed!
✅ [32mRust function composition examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 92ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 86ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
✅ [32mTypeScript function composition examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.11s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.1
    Uninstalling ekodb_client-0.7.1:
      Successfully uninstalled ekodb_client-0.7.1
Successfully installed ekodb-client-0.7.1
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 82.1ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 84.6ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: None
   Department: None
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
✅ [32mPython function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 90.355916ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 92.061875ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: null
   Department: null
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
✅ [32mGo function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning JavaScript Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 83ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 76ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
✅ [32mJavaScript function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll Function Composition Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw - CallFunction composability:[0m
  ✓ Reusable Script building blocks
  ✓ Scripts calling other Scripts
  ✓ Clean SWR patterns via composition
  ✓ Multi-level nesting (arbitrary depth)
  ✓ No code duplication
  ✓ Single source of truth

[36mBuild complex workflows from simple pieces! 🚀[0m

