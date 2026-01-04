make test-examples
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("vGXEqfYsL5SBGzVMb3QLvx_fd7qeJNH6c_fbq1YPJ0sphbCCE0rffdQI_iaKyQy4g6MQzrH73THCfO1hVsMwLA")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("vGXEqfYsL5SBGzVMb3QLvx_fd7qeJNH6c_fbq1YPJ0sphbCCE0rffdQI_iaKyQy4g6MQzrH73THCfO1hVsMwLA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("vGXEqfYsL5SBGzVMb3QLvx_fd7qeJNH6c_fbq1YPJ0sphbCCE0rffdQI_iaKyQy4g6MQzrH73THCfO1hVsMwLA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("vGXEqfYsL5SBGzVMb3QLvx_fd7qeJNH6c_fbq1YPJ0sphbCCE0rffdQI_iaKyQy4g6MQzrH73THCfO1hVsMwLA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "v_o_VMTYzOkploU1BbN3TCNs9Kbc8H7BASVbe-rp30wX4gyOWAjC8jUwlFCsg2pqbeSVbXBiD-nK4oMNdpJJEg"

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
        "id": "v_o_VMTYzOkploU1BbN3TCNs9Kbc8H7BASVbe-rp30wX4gyOWAjC8jUwlFCsg2pqbeSVbXBiD-nK4oMNdpJJEg",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "goBQeoDCu_1MVrIy9-0ndud0IrP2qaFnTNbRHnw9WZ81nQ6-_OpD_G1w32o7xVHqagbJ-qUAL4FNwwCX3h9odg"

=== List Collections ===
Total collections: 4
Sample collections: ["demo_collection", "test_collection", "batch_users", "websocket_test"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "Hj5PCYhFJwqLTv9ZTL_QGcDTzeV1p2iM70feVLYJTGYwh2s88-E_b-cnD20H29e70_XmTv7oEyRuo83PXpg_hQ"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "oJjxZtX4vHS4ax-SAZ5G6BecVfP8DSECpUbdkKJE4Kj2Tjw2a_Kmd-gQteZlNTc4Doli58d0sWg4Z6_n-XFkTw"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "0Ln73ns5Mo_3yGfm7R6jV_EriIeSJs1LhF5fUrJ4Pxt9QR_4M2FMleXvqYCydUdC8pa_6KhEtZfH5IXCleVLdA"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("zKWzMCojiSbBauCccuQgda7Inra-nw0OuAF-yfnbwgCGebMEeK3UmOYDcVoaKLlLX-5XHZmNFDSzX73ZFQRtRA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: w7fkU_VzTncBuSXmdDaImvnETFLTFUvns61_Mb05CGR6rtLxYUlMiW9FoxvKmJ23FY02sOTwFR5-wE2h-qmiTw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "p6fhF3RaU_XX0KBvrH0Jr5ma3tfnf4enfZU8ZA-9R-DxbtuWjiMvnRoF_o1EegAnVNgrl9XaNyDkADSYhp_HIg"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: XULBjPYPwaaMuyHvK_clonfdDMpx8CZK581-WhZV726FV1DCMYFcTqUPU83L1pLnUHatApVIX59ALjW_z_BNkw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: rrXu_Hwi7siBfgLmdWelvTZySTXjVDnYEzsOAZnp1ZRdNWBz4j04RXA3n2TA8-E1jbEAw-Rz-s5CKsKatnWNcw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: jR9LtRBNJpjTOZGL6PCRVMdjNDAGtCbMY3rX5oN7XhFyMCemBhdp-XgOHVbBXRuwvovY_RoDGaYOiHnvTbXjVg
Created Bob: $500 - ID: 1Uq59C8tBn76nyHEq05YdjVEnC2jhAbAtMP1KICUf6BLbzX6uZT65paBSaDc-zeMHdHW0Pt2kbL6jcPuwBHnyg

=== Example 1: Begin Transaction ===
Transaction ID: cb158131-1762-4615-ae4b-2104c3391436

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
New transaction: 9fbe82d8-c367-466b-a329-10b77f93cbd5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("Sb0ExP49xJHGSFF7iUxLYbW5W8_QcAo_ufM4dwpR0nA9UqVfMnvJ2lbR8xo_Yuq2zzlocUJE5hzBq0HJK1VnSA")} }

=== Find by ID ===
Found: Record { fields: {"created_at": Object({"type": String("DateTime"), "value": String("2026-01-03T23:53:50.742008Z")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("Sb0ExP49xJHGSFF7iUxLYbW5W8_QcAo_ufM4dwpR0nA9UqVfMnvJ2lbR8xo_Yuq2zzlocUJE5hzBq0HJK1VnSA"), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"type": String("Float"), "value": Float(99.99)})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-03T23:53:50.742008Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"price": Object({"value": Float(99.99), "type": String("Float")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("Sb0ExP49xJHGSFF7iUxLYbW5W8_QcAo_ufM4dwpR0nA9UqVfMnvJ2lbR8xo_Yuq2zzlocUJE5hzBq0HJK1VnSA"), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-03T23:53:50.742008Z")})} }]

=== Update Document ===
Updated: Record { fields: {"value": Object({"type": String("Integer"), "value": Integer(100)}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "created_at": Object({"value": String("2026-01-03T23:53:50.742008Z"), "type": String("DateTime")}), "id": String("Sb0ExP49xJHGSFF7iUxLYbW5W8_QcAo_ufM4dwpR0nA9UqVfMnvJ2lbR8xo_Yuq2zzlocUJE5hzBq0HJK1VnSA"), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "pC4XsqptK8DbF2NXEqRHVAqkSaiTOlngoIZhqgRiQSDmIZrBaRDEcdAwWd5y_xj9xR4650InUboUzjaJRupNFg"

=== List Collections ===
Total collections: 9
Sample collections: ["ttl_cache", "client_collection_management_rust", "scripts__ek0_testing", "test_collection", "batch_users"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "Edw2MzfyE6tzt4tsq2ULK4dkZOMxzEAmEA0vvAp-ZvmsZjF0ox4Qynt-w6nR4MavyLdPwgJ2PXSjqcJD61V5Pg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("P9PNo9XG32MjYfOSMqfBWzGDHvdRubfnIdk8Q7dY-PXp1bcFqxitII6hIm8_S0EgRQEeaCxymRhRn_IaQ2LQcg"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 3I3naWmdPqpGfH3FYfazneFH6-OuQdxFFH8It72Uu1quNiuWxmWNyRq74Af6mtFBTHvab6YiWTODL4tPpsKhcg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("VPEyGlmBmFitGi_R2KJnRWEcRvgeSPAEQ7mzl_G9WWqhRJ45L_qlIAkq6oynprl0nLRMcskAaHwWNK1ysFbUbA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("7-8NUqqQbRm-5GAW_twVbABozGh-Iztvq-q1xZsCCrLOXd5eAFSzdBrBUZMyacIP0PxjoD_bmliirKa2AO8DLA"))
✓ Inserted user 2: Some(String("QG8-yO4VlyM5YmYGaCZZdvjnLmyU0gTg4rOFCfxQyUQAt9OfKhVWiS07ZDFSqIxMmJ3kfz1NmVRitUx4_eS1gQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - age: Integer
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["ttl_cache", "scripts__ek0_testing", "test_collection", "batch_users", "websocket_test"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: peLCg_SRnDYQAjqcOCWo_IJvvhygPTvZtRKGBFOslnP8qxJdAALGQnFev7fga-KnBOZNPjrLWG6uIi8ibXIUuw

=== Sending Chat Message ===
Message ID: FZbG0123qjCFq_lOH7e7GtcMXMSBaLYI74yE2CaSiTZOyfCDCW1CLyiEqcf5T7zHjwEdhSMuvaL77014UHupjw

=== AI Response ===
Response 1: ekoDB is a high-performance database that incorporates intelligent caching, real-time capabilities, and AI integration to facilitate efficient data management and retrieval. Some of its distinctive features include:

1. AI Chat Integration: This feature allows you to query your database using natural language. You get AI-powered responses with relevant context, which simplifies data querying and enhances user interactions with the database.

2. Search Features: ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval. This diversifies the ways in which you can search for information in the database, ensuring quicker and more accurate results.

In summary, ekoDB combines performance, real-time capabilities, and AI integration to provide a comprehensive database solution.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("Pe8nLVa2a5ZvGV3SHg6rtqC68nmi9Lo5QH2OxlaM7QfDdTwePeyeQiNo4Mj-cvUuvmzOAzqpguheDQblCPcong"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("eJfaYNAKBPl14nPg21NzHt5zjv9Y8Q42tY2-9rJvRoN70sBawOGQjOYWuAHtq2wEd-dEnww350mldQHIJ5ETlQ"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("iSY5W8uSJvGhiipvgEyH48rQ2fcXUAgf444hRv9tONoIs1AMcKDU1AMTW0qQW4L_z5FaIQsBrOQ1bxVQNq3iPA"), "title": String("Introduction to ekoDB")}

Execution Time: 3680ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: CG2PChDTWEdZekN5sWiZVrf_WLAtDRjp3PJxpTJjFYSqQCVmjeLDQWziHPt0aaTvTUT6V0Yt986-P4qwCZzNRw
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available according to the provided context is "ekoDB", which is a high-performance database product with AI capabilities. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: UBJkrt-IqZ_gDEXkmLMHxnkx7L_zFEyvLdMjfv-d-qqgEKlBCIHI8p_s-X-94BAmydN8hM9FFGn23dKVfGSd5w
  Parent: CG2PChDTWEdZekN5sWiZVrf_WLAtDRjp3PJxpTJjFYSqQCVmjeLDQWziHPt0aaTvTUT6V0Yt986-P4qwCZzNRw

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: UBJkrt-IqZ_gDEXkmLMHxnkx7L_zFEyvLdMjfv-d-qqgEKlBCIHI8p_s-X-94BAmydN8hM9FFGn23dKVfGSd5w (Untitled)
  Session 2: CG2PChDTWEdZekN5sWiZVrf_WLAtDRjp3PJxpTJjFYSqQCVmjeLDQWziHPt0aaTvTUT6V0Yt986-P4qwCZzNRw (Untitled)
  Session 3: peLCg_SRnDYQAjqcOCWo_IJvvhygPTvZtRKGBFOslnP8qxJdAALGQnFev7fga-KnBOZNPjrLWG6uIi8ibXIUuw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: UBJkrt-IqZ_gDEXkmLMHxnkx7L_zFEyvLdMjfv-d-qqgEKlBCIHI8p_s-X-94BAmydN8hM9FFGn23dKVfGSd5w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: RlD5iRAEpkqYmqABWHW6wi_Y5KQn9I3kUrooAPmsh7LCBwLBEyglCTWOQzN_TdfoM_F5OqWQfOMwGFY1RndZPA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product according to the context provided is ekoDB. It is a high-performance database product and costs $99.

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
✓ Created second session: Z2pocQ1N977hiiEZksfO2WyAC5l-VC1h9TJzhg0S1xbXfB2ONM-Qk_Zqb6aWaumPAg1PXd6Wrerc4mYNkdmKdw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: qy4Ur8VLNOImM9wjVgGvrxQ_MQRFt_Q_cPUqlnuNhkozTEh_aOReJNj8q2I_aWqG7OIQWavQDY-wTcEGjl211Q
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: MvWMEfMrCIyRpymT8vhKs0lXE9YrCXl0Rz6sCU8sNoMqo9rP3WlCc6LF4wwLe9EyHNy2mPxwn-p50D_P-N3BAQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Gp1GvuUdAKQ5Qt8mUQaX2pw4JP4HuiV0dK-_3JmtFKvPb0xIlrFEf4BxIn5cGbKe5Ht7b54-lyfsFYuupmZjCg
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KmPi7I1cXylpsDpatvkolX5VKR5fbFo7vhT_sL6eLBF9fRVsKwS-6BTqQxSEreoSxSR4Ji_xgc5edHP0zQn_1Q
Created Bob: $500 - ID: YWhSyN7KrdAvkaEiUgww-E5qtKVa5Ck2Nf9yYdr513siXtdf3IIb-gsAcpRLG9GqyeVtaOP_iq2jgqQ8v9ozqg

=== Example 1: Begin Transaction ===
Transaction ID: cee41269-1b1b-4f9b-a554-abde9fc6b09d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 62953328-a760-48c5-85ef-93a74d10fbb5
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
Inserted: {'id': 'VpZPIX6jbB1QS4jgfD8XwH1b9IDluid5qQ-I6o7RZaws4I6JO-NnATr42CLGuQBEAjbWa25C3fwygTO89qU5GQ'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'VpZPIX6jbB1QS4jgfD8XwH1b9IDluid5qQ-I6o7RZaws4I6JO-NnATr42CLGuQBEAjbWa25C3fwygTO89qU5GQ', 'value': {'value': 42, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'VpZPIX6jbB1QS4jgfD8XwH1b9IDluid5qQ-I6o7RZaws4I6JO-NnATr42CLGuQBEAjbWa25C3fwygTO89qU5GQ', 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: um21JmrqMHTOaWiaJQsXKEO81RaRGKGHACyQBVWU8yNrbkOgM0K0tZKnikMWm0v1nnEdc0jxGdcOI5tZuN4HRA

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
        "id": "v_o_VMTYzOkploU1BbN3TCNs9Kbc8H7BASVbe-rp30wX4gyOWAjC8jUwlFCsg2pqbeSVbXBiD-nK4oMNdpJJEg",
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
        "id": "um21JmrqMHTOaWiaJQsXKEO81RaRGKGHACyQBVWU8yNrbkOgM0K0tZKnikMWm0v1nnEdc0jxGdcOI5tZuN4HRA",
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
✓ Verified: Record updated with status="{'type': 'String', 'value': 'active'}"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: rff2Hcwwa55gH61SeoUj_KIawT2FOjInRezxIeZBCMP9XUO11KTHk28HMxWK-Fz5MlG1ykiqYtkq-zjxbBmjpQ

=== List Collections ===
Total collections: 11
Sample collections: ['chat_configurations__ek0_testing', 'ttl_cache', 'scripts__ek0_testing', 'demo_collection', 'test_collection']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: zQtCv-l0T0mBlyA4VA9cWX7cE421ZjhP7xT7LST6eFUs5myQtJduro7cUIVE0V1Hjh8M97Mf6lXAvhEd8Cj3YQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: l1vrLKJ55IDg4TwPzdGCSx2PijAZrc9LtappUeVQHsGDlDwKkO1733B-O5gq6yNjEZUIlX0HPe-wi7WKnW1AjA

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: gPJ1Lyh94ujA-CV2TrEnCc0_qr1FCukeLLyyoJePYFapsQgmpQJOIdZICP3zkHwbdK4UDKrUA7NdmlFcW3Z1oQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: DhEdyznYZEAAdVFLJIXIgGk3DqCQSJ7CJFaXFR7GzfWCIXFdHNfzqp_VtBwWHzungPBTlTCquuM8Dp6rUopPzw
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: r90VyPsFrUYpwgcVaB1hDCu1dwoQDYRt8kwf5arBI2rPmMTUOB2SBqDPY5TTQrUHOqoAlLFNG3Ynv85fBWSGMA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: AB72k8P3SwQxBBDSC_LkZsbtU4hwqL4u5leefU52Jc47sMBTnAefWQMIELKMwf0qBQjUr9csRhEBNL4RPPIuSA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 50.0, 'status': 'inactive', 'count': 15, 'max_score': 90}
   {'count': 15, 'avg_score': 60.0, 'status': 'active', 'max_score': 100}

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
Created Alice: $1000 - ID: buifY4YUnxMDABtCcWqRhBy-cXFVXZQqHECogebTV2tcxCo6SPgJFuHUoGWYurZrK0rl6tomhnSrIZwEIdzbSQ
Created Bob: $500 - ID: P83D0AVNs1CTtNKtR1fo2IRtTcQbzMkbe7EhQybqxqlkH8N-MwussL7tI7VfbTJgx4dQSeAAovhHxXrqUHI4Xg

=== Example 1: Begin Transaction ===
Transaction ID: 5e0b3d6e-48a8-42d9-9b0f-e3e425f30fa7

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
New transaction: be77e5c6-d507-4e06-937a-45d99e7e308d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: YX4tBCbvsOtyUXc6HvMHDlHrGcaRMWgtDqTVIl6ADF01QwsyyGcjPHAehCkPLXbtW0mYGoEd0fvrkDyxT_Gp2A
Created Bob: $500 - ID: rD5qvnXGwyGK1rrHu04JFWpRPxOuMUC9Qcoi7I7sQctOW6sZ-wocFslX0ijlmPPKkZCcZU0obBgTMsqP7oKGJg

=== Example 1: Begin Transaction ===
Transaction ID: 64940ec8-ddce-4892-b6f5-570bb9446834

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
Created Alice: $1000 - ID: yRYG0Y0AwET0XuD7rDHN7jq88vEYBscSDshkvNV2BiArc8ACbCRFCGtpirgVSZewkzVdYGdcmC-GN8X3pb1-Mw
Created Bob: $500 - ID: -iVaI4iXnf4YdpOJ_JthqV0VA8X_Hn-e64gjKfNYb51NIYnfiiOqTP_CnCVC0TR1Yn9_zvmbxvY02tjHMv3mpA

=== Example 1: Begin Transaction ===
Transaction ID: 3c3718b6-ea54-4b8d-adac-4105c7baf2ca

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
New transaction: 5e64f15f-5015-4e00-b87b-71aeca22b16c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: mxoBoZLf8AB2thF1VmtpJZq7k78V4MEKdaB1Q9gzm90YSw1o8aV0igEmiO_EaEIWvt03nGPrqo-G_3fsGaUObQ
Created Bob: $500 - ID: Pc4cKkDrgEuuvqtYtQkqdCO4hUp_C3SbhNyVEQbxspjseH12krP07-qEl3EJCnruEv-E4GUspb-QWfg5VG0--A

=== Example 1: Begin Transaction ===
Transaction ID: 0ed33c1d-a271-493e-b984-5fc41e970dd3

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
Created Alice: $1000 - ID: Luc9V6R9vQTyOWpNbR3O6vsaDBxKFC7muf2iW-6yAFQ-4dCAHGT8VIUGI7ExYHRYQCy5jwPAbM6MDICNjGeioA
Created Bob: $500 - ID: iRoirefKcSSCr9V9qBaGTj_z-JQ8T8bFEA1CEmhx7vXyDa_XIBSCUlJfAoy_Z05zM7PKJ7FI_1OdXv4MUikpEQ

=== Example 1: Begin Transaction ===
Transaction ID: 4615dafa-078f-414e-8725-98f1f993f8be

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
New transaction: f26b8c2a-574a-48f8-93bf-fe6249c9a1e9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QyHR9Drp8xQeea3QKR6_RsJ9dzmkmAhEf8d2KeGM5b-8_9Sg0ihApV4Rl9xGB9URJgSp_JyrNJ_rSaX0QusUqg
Created Bob: $500 - ID: ibqblKItBnN77gT761IX06geMdzbZOcaCtMC0Akm9J5UwcE--zNvBSMN6Se4O9y8-U1VbZLe3tKWW27O_uW1Ew

=== Example 1: Begin Transaction ===
Transaction ID: 9e55f12a-811d-4e45-8ecf-7da50bcdeba4

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
   ✅ Script saved: UJDeobuujAng8eUUAqa3erd-sbAAuxsCCmyh4QS0AmhQqkrKhFrbeoElm3lwjEmEPiRMpGVMY-l95gz6znRBCg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: FcNv0eXVUg1wEVbLyeYX39N2rANfgdTewfnzTvIRrXOVrmvLxQ-b94fVEWsAe659ke_sm9yFJV2bbtBeCnp45Q
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: KC53LsabOSy6s91QCclaXn8z1CSw3OG0ZNM0y_87vLaCijMxVa6Jo-HVjYSqkX01wEYEyG278xwoakZfWh1B9A

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 2n3od9xM92TjEXtHJrMd_U5bEqkozC1KphnbFwhmM9Y6yUKi2ankqScxCxUbzk_bF1pm1c35vt0yMUBECB_sAg

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
   ✅ Script saved: kNcuNqaQB07KxIi9TxmD0DcnzV177b6APtJlgt9rNexptuntCO7GHYvc1xFwGEyeN2P1T0h1RKOfcqG2mOrtpw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: UJDeobuujAng8eUUAqa3...
   ✅ Deleted script: KC53LsabOSy6s91QCcla...
   ✅ Deleted script: 2n3od9xM92TjEXtHJrMd...
   ✅ Deleted script: kNcuNqaQB07KxIi9TxmD...
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
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.0
    Uninstalling ekodb_client-0.7.0:
      Successfully uninstalled ekodb_client-0.7.0
Successfully installed ekodb-client-0.7.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'sxNtMS0bqKj42LbupIMYM0R3up9fcfjcC0EE8s9j2w65EWgZLnM9tb5ykEoyCJ5nYWp8_tkCGS8jxXD-pG7y6w'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'sxNtMS0bqKj42LbupIMYM0R3up9fcfjcC0EE8s9j2w65EWgZLnM9tb5ykEoyCJ5nYWp8_tkCGS8jxXD-pG7y6w', 'value': {'type': 'Integer', 'value': 42}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'active': {'value': True, 'type': 'Boolean'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'value': 99.99, 'type': 'Float'}, 'created_at': {'value': '2026-01-03T18:54:38.697986', 'type': 'String'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03 18:54:38.697986
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'name': 'Test Record', 'id': 'sxNtMS0bqKj42LbupIMYM0R3up9fcfjcC0EE8s9j2w65EWgZLnM9tb5ykEoyCJ5nYWp8_tkCGS8jxXD-pG7y6w', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'price': 99.99, 'created_at': '2026-01-03T18:54:38.697986', 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'created_at': {'type': 'String', 'value': '2026-01-03T18:54:38.697986'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'id': 'sxNtMS0bqKj42LbupIMYM0R3up9fcfjcC0EE8s9j2w65EWgZLnM9tb5ykEoyCJ5nYWp8_tkCGS8jxXD-pG7y6w', 'value': {'value': 100, 'type': 'Integer'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'price': {'value': 99.99, 'type': 'Float'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

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
Collection created with first record: "5LO-OpOmyIf0SfzonohO8ssyIecRBSvJapPHm26ufUfYECi3YNvRX1qqzK5XcmrpjZSgUXaB2cb6Q3nt4hy4Lw"

=== List Collections ===
Total collections: 10
Sample collections: ['chat_configurations__ek0_testing', 'ttl_cache', 'scripts__ek0_testing', 'test_collection', 'batch_users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ln3kMnT9OgnaDWbYg2VfEucW0qgSD0m70a28JqAJ_azE3s22Wkypg_Y2DLHtac3YV6PMHtkSAf8DxbaZcWYwkQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: XnwvfC4RqXCV_Y-Uwka87sj-aCTTcvVq2nyyAdMl8xBOy-yfZbgVg5V60MwJiM8ODhr7UGdeNNg5h0HWOx7A0Q

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
✓ Inserted test record: 6fnoBkzr3PkB2IYmWnzocDcCtkcpm652OO4vzD75D_QQvYpEQesF-fAOUKpTF7k_G6hDssP0pzZcKJD4xeeqCg

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
✓ Inserted document with TTL: 2pNI-VlnuR9GF4VdunKk6Wzagxq-WKCmAa_VLNNw0TgzLg88rGjUYkUotdVMbesY1lcyQlsqMymsXtQujckARg

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
  1. Score: 25.740, Matched: email, email.value, name.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio.value, bio
  2. Score: 26.400, Matched: title.value, title, bio.value, bio
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: bio.value, title.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: bio, title.value, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.764
  2. Score: 0.762
  3. Score: 0.743

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.734, Matched: title.value, content, content.value, title
  2. Score: 1.120, Matched: content.value, content, title, title.value
  3. Score: 0.535, Matched: 

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
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: l4g7zNZrVeaCo5zuQPZks0Yqia4jcciEDUMpkyHfX0jkRBqSmIH-phsZn7hXgLEJV4kU7DCyyt3ahLVBx7oCHQ

=== Sending Chat Message ===
Message ID: 9W28QzNDqEVlz0dU7czZwcVy1K4LA6Q-pLscxZxIPdgKK0VF8xpU34GEsc_wjdbK2LnlTvxl07QKbqWoNVUhLQ

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Cloud: This product is a fully managed cloud database service. The price for this product is $499.

3. ekoDB Pro: This is the enterprise edition product with advanced features. The price for this product is $299.

Execution Time: 2491ms

=== Token Usage ===
Prompt tokens: 603
Completion tokens: 84
Total tokens: 687

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product "ekoDB" is available. It's a high-performance database product and it's priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ES97t9jsi5usrA6BVEgzOW9AT_eGyIm7_9lLzcklPxeMOErA5gccTnUy2v8Q0_7r8WU8QBaH0ldy-2i3hEP_Jg
  Parent: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: ES97t9jsi5usrA6BVEgzOW9AT_eGyIm7_9lLzcklPxeMOErA5gccTnUy2v8Q0_7r8WU8QBaH0ldy-2i3hEP_Jg (Untitled)
  Session 2: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA (Untitled)
  Session 3: l4g7zNZrVeaCo5zuQPZks0Yqia4jcciEDUMpkyHfX0jkRBqSmIH-phsZn7hXgLEJV4kU7DCyyt3ahLVBx7oCHQ (Untitled)
  Session 4: Z2pocQ1N977hiiEZksfO2WyAC5l-VC1h9TJzhg0S1xbXfB2ONM-Qk_Zqb6aWaumPAg1PXd6Wrerc4mYNkdmKdw (Untitled)
  Session 5: CG2PChDTWEdZekN5sWiZVrf_WLAtDRjp3PJxpTJjFYSqQCVmjeLDQWziHPt0aaTvTUT6V0Yt986-P4qwCZzNRw (Untitled)
  Session 6: peLCg_SRnDYQAjqcOCWo_IJvvhygPTvZtRKGBFOslnP8qxJdAALGQnFev7fga-KnBOZNPjrLWG6uIi8ibXIUuw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: ES97t9jsi5usrA6BVEgzOW9AT_eGyIm7_9lLzcklPxeMOErA5gccTnUy2v8Q0_7r8WU8QBaH0ldy-2i3hEP_Jg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: XdbNvcAnUcEO6MDPRDXMuLU5Lu-nWKFCsZb5LtERJvPYctWEiLW0578Uz8p49EbI4Y9y4owTDcbuSIzh6tToCg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one available product is "ekoDB". It is a high-performance database product. The price for this product is 99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'role', 'content', 'context_snippets', 'chat_id', 'created_at', 'updated_at', 'token_usage'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: pJAkPJ50ceg34xRRKItswW6PzC6DxtbaJCb0cxXJ-vky-Pz_nkfgehDNTp6ms__wEKw2RaPUOav2-ltIrOmPqg
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

✅ Script saved: 4skv9xRiuGH9r41MelfcTs15CpXREdqL8vM6wNBdiCStqkqXV2uaQLZKXQzthJ7761dfldNfrH8b9aZJTA6Ymw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: UMGtfvd048Uom5mGg1ZmK0IcD-F4lmcfpQljtCr92_edtJpzOvBVRVlh4HvbIVZz-7qX8XeDOrXJw4kXdR2F1Q
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: bMa5TrbyIPyYpU7_l4XjIzBuwQcAqI5LbLHDfJeBwUSc6lkZQBZgZhBvxf7ycWI6Qn3o-v5DW9ZQQIq7xOr0Yw
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

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

✅ Script saved: k5oJZfVlQxQcRpZZtKEooqqTghxivyGrARP0MkWYeGbFj-gH9GwFAXZ-5qAm5XyojnagJ-jrMakoE6BydZ4-zg
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {'category': 'Electronics', 'count': 3}
   {'category': 'Furniture', 'count': 2}
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
Created Alice: $1000 - ID: TLMHcMwYWJ9cvwyuYqOHIG5xN0BdcXwsvxAbhh5m82ExzpHArYu20PbOlUwRaO_Q-CwpVfv97u8wbPsdsluOmA
Created Bob: $500 - ID: DV_ZVQYtWl25PvwtPBFGrHMFq46HnBu1uDm_pIYaNWtFlfYwfOT6zB68sMxj0st7VEKf6_oQYVCr1ZLLVpCvFw

=== Example 1: Begin Transaction ===
Transaction ID: a0278f3e-d6e6-4892-a80d-f1ecd150886d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f0b72de0-b554-4338-846f-92a9732146a5
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
Inserted: map[id:rivp_cfjb9XdnFqiWhmzErX9cuzEvHzcK4Z7lEqj9ETr_XK2FML1aQOkLUiBDNhq-1fby9m5NocFjCxC2cDKcA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:rivp_cfjb9XdnFqiWhmzErX9cuzEvHzcK4Z7lEqj9ETr_XK2FML1aQOkLUiBDNhq-1fby9m5NocFjCxC2cDKcA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:rivp_cfjb9XdnFqiWhmzErX9cuzEvHzcK4Z7lEqj9ETr_XK2FML1aQOkLUiBDNhq-1fby9m5NocFjCxC2cDKcA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: FTsekewxgB5micEApFUENESf1T4V1PoJc7ytPFR4kyIC6AB3mCv18L8t7JYVDT6tkCGtqwz8jXgnOOsmcpkPkw

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
        "id": "FTsekewxgB5micEApFUENESf1T4V1PoJc7ytPFR4kyIC6AB3mCv18L8t7JYVDT6tkCGtqwz8jXgnOOsmcpkPkw",
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
        "id": "v_o_VMTYzOkploU1BbN3TCNs9Kbc8H7BASVbe-rp30wX4gyOWAjC8jUwlFCsg2pqbeSVbXBiD-nK4oMNdpJJEg",
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
        "id": "um21JmrqMHTOaWiaJQsXKEO81RaRGKGHACyQBVWU8yNrbkOgM0K0tZKnikMWm0v1nnEdc0jxGdcOI5tZuN4HRA",
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
Batch insert response: map[failed:[] successful:[_LBxl4142w9mkofhH5lo6AGbVb181xv2nYRMPCUW0nO-00aD2OROMXZtdqodCWrwKQSOjXqEGIUUnk2Dg4DEug 9XGEAXrAlz-O7rT0r_dPN5J_wFctqZNoboVcPoF4gog6SNkL_CQZr2y8ny12kri1byJkKEMSB3TgW_WrlZIz0Q p4uDRUuHALuX0_Im-b-ISzDdUFqrUdPyipLCxcgvSUt3__UYc16Ai83JZ7eko56jxeyVI9RgXeP2DAz6E1V40A ZDOsULdKSkZWVD4KMaRPlMvO3W-drXmWnuRTFJbSYoglZxeGlGVFFU5MV6S7iSNms-610luKTMaFNgDWGcJHLQ elbhM3sbtA-pqfkJ6fmgZEF5ATJUduAY47fcD1V_MojZcbNp2hJIzDlK0fbEjYakF-vQlrtcmshrFd_E6jaa7g]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[O4AMWe6VAYE3camhpyenX8pHsZaq1y2nfoaNhh_HQ3Im6GYWcJz_q4mToq2rzMhwn4SJmSbByF8o73XsLSIeGg SwVa4BmarvjXX8gzwE8hv10OdLQTr5VrNyjQo-tNXVamD4QBnEA3zvBLxt4r8bfHoCZ8BvelCNpjqNhZ6BKz1w 8vo0NhPbRqvVjEkbyeyJarLcRU0VP3vS1GwxmnkPJgWdicKHgVvDDLGtf9zbB_jh6Iln9I3nnB9EP5Gb6PK_KA]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[O4AMWe6VAYE3camhpyenX8pHsZaq1y2nfoaNhh_HQ3Im6GYWcJz_q4mToq2rzMhwn4SJmSbByF8o73XsLSIeGg SwVa4BmarvjXX8gzwE8hv10OdLQTr5VrNyjQo-tNXVamD4QBnEA3zvBLxt4r8bfHoCZ8BvelCNpjqNhZ6BKz1w 8vo0NhPbRqvVjEkbyeyJarLcRU0VP3vS1GwxmnkPJgWdicKHgVvDDLGtf9zbB_jh6Iln9I3nnB9EP5Gb6PK_KA]]
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
Collection created with first record: 3PGG0gfFDbHTu2d812Asn_7Rg7sVl4lg1t3W2WkOGjzPVzmDwEvpzZt5nCjWtdDKku4wjnlyExzhLxcn0PHVvQ

=== List Collections ===
Total collections: 15
Sample collections: [chat_configurations__ek0_testing ttl_cache schema_documents_client_py scripts__ek0_testing demo_collection test_collection schema_employees_client_py batch_users websocket_test schema_users_client_py ws_ttl_test users schema_products_client_py chat_messages__ek0_testing test_accounts]

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
✓ Inserted document: c-1Sl2P2mgxY9hZRdfI3OVmrOKM68cxU6hkCGzDGfhLCxQrYiw684eoE_j3vKatWsaSTAhloTw2kbDknEA4xmQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: p06aBl87fjpRMr5kpvCSmh9V0aGcifikRrSOXMf4t2wHx9-Iqc6CI40wLhh2FpzDjNNpDU7nmNX9nIfxipDF_Q

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
✓ Inserted document with TTL: zcSJH14KnWyS_gKb4ZSnzAHQf_2BD7hv3x3XL-Wnu36W3sR7GfvjSv4bnoZgjoL9sGzaw9H-1Xo4TJDcsRG2ZQ

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

✅ Script saved: XIIXBUK_0SlbzLxX76dvsfM-kgqbpw-ItOyTjxYyEnnPK_fzExcG0N5MnF3RmjzDkQ2B2vbDfI3MszZQfSxMQA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: soNJqPCr1yqm-ugzq0mQuAp75ibpydU5XYUygfHktk_YPRD0YKePt-UtR6FLF2h5kfocjmVl9ZaNzMw081NwqA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: VdG0jPwxmakttVoQETr90NMRMHRdn0cYHHfmUFhaVvTar35RenSmvuDFplXVisdJcDwDaRnTqZtneyOLPfeJ-Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

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
Created Alice: $1000 - ID: LgHgiDUd1-r9T92h4K5TsUu33JwJkJ56Zntge7A8JKFddvx3qbcpuQU8HJxQ5YzspDiP2awID6cu-SZ8Ul04TA
Created Bob: $500 - ID: xLZ0pwVoFhVa1hQkxpM1Lw55S0NmqG93Ih-X4u8MudwPB6JbfM4uE4gT4Y6wtQ4RMgaqgZn5V9VmrVLjhPZzbw

=== Example 1: Begin Transaction ===
Transaction ID: 9ad7ba83-14f3-4463-834d-3a9d422310a9

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
New transaction: f11dcf2f-45bb-4202-aaf6-9c17ebfbe56e
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
   ✅ Script saved: 7GYcMVHox8X2VlmfwKStu9N465WuTvL0pWRhTPrW4jh3R8sXwISqj8KATFEz-aF_f4aWdnmZWfw5KGx7QE09_Q

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: WopDK0CyvngpqvRn0stF4KSX6CsBVRhwDZ4kWpMAN7jkYn28j9bLo45aCi-MzvNBQSzKAe1M-7WeOGpcoLD49g
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: vJgzEQmKxafgelpkVWAW80oIVMMERTc-P-FROqsxthAZzaK31m4wm2CF4G-RPPd1VO9YLRehxUYKXLu-dN4pww

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
   ✅ Script saved: LJvq_V1YJBU7ljCG5zvvi5hCqCbhHy20RikzDHVW0Pz2wbpCHmg-VTwgqlKTzNBHoDaRLE5HDhz8-1hoRZgcOA

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
   ✅ Script saved: gC05wL746QvX1zQNcxXHmxyJSCEoVrKoNgb250z_lUEEOlOwAW3AafDm9J00UL5ePSh5r6tXzxTrUFw9wLKg2g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 7GYcMVHox8X2VlmfwKSt...
   ✅ Deleted script: vJgzEQmKxafgelpkVWAW...
   ✅ Deleted script: LJvq_V1YJBU7ljCG5zvv...
   ✅ Deleted script: gC05wL746QvX1zQNcxXH...
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
Inserted: map[id:8Xjm9XIaBnfJi42ZPq3zTxyo9ZM6CjKJ228zhw2VlDgRoxfuboJhqDbhen7S8uCMaioZiCU_4IgRhLULGYVbLg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T23:55:11Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:8Xjm9XIaBnfJi42ZPq3zTxyo9ZM6CjKJ228zhw2VlDgRoxfuboJhqDbhen7S8uCMaioZiCU_4IgRhLULGYVbLg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-03 23:55:11 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-03T23:55:11Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:8Xjm9XIaBnfJi42ZPq3zTxyo9ZM6CjKJ228zhw2VlDgRoxfuboJhqDbhen7S8uCMaioZiCU_4IgRhLULGYVbLg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T23:55:11Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:8Xjm9XIaBnfJi42ZPq3zTxyo9ZM6CjKJ228zhw2VlDgRoxfuboJhqDbhen7S8uCMaioZiCU_4IgRhLULGYVbLg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: QxRnPpdUrtS5UIJhxanRdSId0Y0pFaAr0xuuRkWIaEjXrR93D59P4LuAJT95rRUXA21V-y2yrFK-XaoYs2s8tA

=== List Collections ===
Total collections: 14
Sample collections: [chat_configurations__ek0_testing ttl_cache schema_documents_client_py scripts__ek0_testing test_collection]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: l2f0a9zNgkizL-DvmJUflGhGp5idRClogdXEV2-fgRkR1N7cG8I4GodpFdjJ0B4baaYbf9OmpGW5XfFWVTVgbw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 6FFq1lvd2WJNHuyPbNZBqMdhlwzLz34Rmgt0l3G7RFqLtotbSKG-P2lv8SLqvCrxz0LG2qHilcHdgSSdfeNfCw

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
✓ Inserted test record: v_LF3I3j6JS1LmTRSLG6_Pr8rPvskd0_wXrriIp6VD9NTw78mNdn5cqNAbrDlUGRdsWEUrt4NVVqpQcCpojPGA

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
✓ Inserted document with TTL: bVpQU5MegOFfcyAm7_ueqFzvrtBQMyq-Gpi6nOna49kIpllavTAdMxXnadEtPcyFU1aL8xI4FVqT2BNBa1sCYA

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
✓ Created session: Dogxv1Bte8IojZZSfjozm5Q7Bh1PRfDRm7Kts0TLSDNhwVoTkZU9B7Zyryl4unpGYGP_brfhd1cvzWqBLSZoxw

=== Sending Chat Message ===
Message ID: 0xn0dFJD1VfCm64zI5IvKGMU6P6lyQdOL4dx-M2qG1paMKzMaDRKkLvigo1zTN99wpXHxAXDjUUGhmXZT917ow

=== AI Response ===
There are three products available:

1. "ekoDB Cloud" is a fully managed cloud database service product. The price is $499.

2. "ekoDB" is a high-performance database product with AI capabilities. The price is $99.

3. "ekoDB Pro" is an enterprise edition product with advanced features. The price is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:FEizPpFdbvIdbKCs2rhrOER_sKRXxNnS9uj6oWcJbxxh31d8MAERwrg51VtjKM3PiKIAHyxXuq1Kv1Uc1z8Xag name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:SgxOV0qcwe_YWcPUIaOFCkJp3ca9kOm9heY8lWI1eNerdy-r9Cb4sLt1BVJwE_LjAeRactV2MO2l6xnwfDUmLg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:Ks42Fw2HcdyxMIXiE6Q2vcOtNYonBFfkm51ven6kxkNi_wP3DyWpZ0AJJCrNTIY0U0h7XPzrjMJ2Fx8dh0d2Wg name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2312ms

=== Token Usage ===
Prompt tokens: 613
Completion tokens: 72
Total tokens: 685

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: KnySy_WEfsp_X7M9HFbWUQFUF0_e0-g83hQ4ruClauV_v3Hx3qsidknLRmcrBq9N9LjEYo3Qf3E-Zf9vp1BW0A

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is the ekoDB, which is a high-performance database product. It is priced at 99 units.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: pXATh_AppcMEk36vCsV1Qc0Ulh_huI_7CZBq5JqUfdvSMC3YkbjN-l_VqVR3TzFIwofDcZofSCmvSMi-rZ0UKw
  Parent: KnySy_WEfsp_X7M9HFbWUQFUF0_e0-g83hQ4ruClauV_v3Hx3qsidknLRmcrBq9N9LjEYo3Qf3E-Zf9vp1BW0A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: pXATh_AppcMEk36vCsV1Qc0Ulh_huI_7CZBq5JqUfdvSMC3YkbjN-l_VqVR3TzFIwofDcZofSCmvSMi-rZ0UKw (Untitled)
  Session 2: KnySy_WEfsp_X7M9HFbWUQFUF0_e0-g83hQ4ruClauV_v3Hx3qsidknLRmcrBq9N9LjEYo3Qf3E-Zf9vp1BW0A (Untitled)
  Session 3: Dogxv1Bte8IojZZSfjozm5Q7Bh1PRfDRm7Kts0TLSDNhwVoTkZU9B7Zyryl4unpGYGP_brfhd1cvzWqBLSZoxw (Untitled)
  Session 4: pJAkPJ50ceg34xRRKItswW6PzC6DxtbaJCb0cxXJ-vky-Pz_nkfgehDNTp6ms__wEKw2RaPUOav2-ltIrOmPqg (Untitled)
  Session 5: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA (Untitled)
  Session 6: l4g7zNZrVeaCo5zuQPZks0Yqia4jcciEDUMpkyHfX0jkRBqSmIH-phsZn7hXgLEJV4kU7DCyyt3ahLVBx7oCHQ (Untitled)
  Session 7: Z2pocQ1N977hiiEZksfO2WyAC5l-VC1h9TJzhg0S1xbXfB2ONM-Qk_Zqb6aWaumPAg1PXd6Wrerc4mYNkdmKdw (Untitled)
  Session 8: CG2PChDTWEdZekN5sWiZVrf_WLAtDRjp3PJxpTJjFYSqQCVmjeLDQWziHPt0aaTvTUT6V0Yt986-P4qwCZzNRw (Untitled)
  Session 9: peLCg_SRnDYQAjqcOCWo_IJvvhygPTvZtRKGBFOslnP8qxJdAALGQnFev7fga-KnBOZNPjrLWG6uIi8ibXIUuw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: pXATh_AppcMEk36vCsV1Qc0Ulh_huI_7CZBq5JqUfdvSMC3YkbjN-l_VqVR3TzFIwofDcZofSCmvSMi-rZ0UKw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BEqv94BGuoMeQYX3pPbBjU1uRADxKTwYXUobq_-J6uV5oV0zw0peMRacpEdA3llV_MT-14zp-6c_gwSMTogkUQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one of the products we have is the "ekoDB". It is a high-performance database product. It is priced at 99 units (the specific currency isn't mentioned in the context).

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
✓ Created second session: o_BggN1qIUogt0S1WPe31EPA0f2Y0eNckWn74WiDxXaSXeoZx_KvkCI0z7b3iqRZc9GJPCmI5xLqZFA-U-BXkg
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
  1. Score: 0.769
  2. Score: 0.757
  3. Score: 0.744

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.697
  2. Score: 1.503
  3. Score: 0.308

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
  - Mouse ($0) by Alice Johnson
  - Laptop ($0) by Alice Johnson

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

✅ Script saved: jE714NoFoty0Gvf-0woxTzOwW9OO_DXrNBKjHfIvMjT9m8MJiyVHwKtomCvV9ZUsDg_nrsXTSJYhrQDgukODvA
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
Created Alice: $1000 - ID: cwU6n-UvHoFMAQFtAVXtAT2H2_CsZN1JQXHJdIHKQVIeUL8PgLfvjPxP5RqMHz1P22AyKViBsf9NqtM1N3RD1A
Created Bob: $500 - ID: 9xxK_saVdjc8rP2LxJFNTXbbA7aCR9bHrKmxmi6egYouCB19aTRGTm2X4rFWRM8RMVSs-3p9-fnwXjWgiBkwag

=== Example 1: Begin Transaction ===
Transaction ID: c471465e-bb82-4883-83bb-af9b0efec628

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9b06fa74-4694-4eb4-96af-038eaa089e93
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 797ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'MvxyMuRnSODOCglgKyfT8ipnOZxaEihEZAHvQzfM0COQ5x2j2rb-7H9RBpBnSDbYiTa8jXIk86ad_kfmDDghAA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  created_at: { value: '2026-01-03T23:55:33.190Z', type: 'DateTime' },
  id: 'MvxyMuRnSODOCglgKyfT8ipnOZxaEihEZAHvQzfM0COQ5x2j2rb-7H9RBpBnSDbYiTa8jXIk86ad_kfmDDghAA',
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03T23:55:33.190Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  metadata: { nested: { deep: true }, key: 'value' },
  created_at: '2026-01-03T23:55:33.190Z',
  id: 'MvxyMuRnSODOCglgKyfT8ipnOZxaEihEZAHvQzfM0COQ5x2j2rb-7H9RBpBnSDbYiTa8jXIk86ad_kfmDDghAA',
  categories: [ 'electronics', 'computers' ],
  value: 42,
  active: true,
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'MvxyMuRnSODOCglgKyfT8ipnOZxaEihEZAHvQzfM0COQ5x2j2rb-7H9RBpBnSDbYiTa8jXIk86ad_kfmDDghAA',
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  created_at: { type: 'DateTime', value: '2026-01-03T23:55:33.190Z' },
  price: { type: 'Float', value: 99.99 },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { value: true, type: 'Boolean' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] }
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
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
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
Collection created with first record: rpAh7ElzSljzYzWHh9Yjxqp7dESPfEyO8GRF-lAETeoq9hcgdBeMqhTYq_1B0FT7ba6Wskv5mfalKU2aLYYhUw

=== List Collections ===
Total collections: 18
Sample collections: chat_configurations__ek0_testing,ttl_cache,schema_documents_client_py,schema_products_client_go,schema_users_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: yD-DlYdmV3clotFc4Ib4t8I9beXd3OGNKWSLSrlGP5_yUbFOL-85IbSYQn4ll07wg0V_UtO6KQQ59vSc9104FQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: UE80CJqooBI52iloRFunNzVlxy6nLrlNrnG_PO31a0luD8hc2qu-vRxSy8PdONQ7N7D7OonD5DbrXlLVhVYqNA

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
✓ Inserted test record: 3dK618MpMRBxvO9jB4NTrmfiu-zgudKZ1uOlQPdf_AQz5S05EVisdNwCQ8G5fNvzoWvGk3zPS-SGrN75h0dzPw

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
✓ Inserted document with TTL: zmZLVFVTWeYYa_nsiutYcobYLoiwIY0-3GlZzfFzxhjPils_Ql4BCODMV1Gs-VPDqMRQYRxvIKwnRzmAhAPG9Q

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
✓ Created session: KoYojp58i-Wk3pORhJffUTNtd0Ki68ZOkImoEKp6JE1zVBIyRy5o5psjVUzGT_ZnZGvdN_ihaq7n22yiVEE8Ng

=== Sending Chat Message ===
Message ID: UsrvSwpWqoi0RZx6RjosrIzLBRp4ivxWewaPJ87UFb134WzrbZP9yBZpaTnd3E6QV0Doud28EunihOBIvlDT1g

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Cloud: This is a fully managed cloud database service product priced at $499.
2. ekoDB: A high-performance database product with AI capabilities and it costs $99.
3. ekoDB Pro: This is an enterprise edition product with advanced features and it is priced at $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'ZHigRoalnzdfENV2s5stMLszWZEZz5AMKEKHsBIqkyAg_aIn_umQm-RglF3pIfpgK8zGwRhtp8z4rFNWk4noXw',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: 'u51-1uWMTccZD2inEEONLbbld0poR8TAtVHtbpjua0Blu_wqz5XGEe8pPAeL-XlxbzaMZiAIyfHItG1wbkIVvw',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: 'wRKQo9RqBVUWF5S88qWrL8_Xzi1mR52ld4vozfK2lR3TfqP-5aPYKISG44E3UiBRZtOuaEfmGs27JgconIFVGg',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1849ms

=== Token Usage ===
Prompt tokens: 602
Completion tokens: 75
Total tokens: 677

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: IdJ7XroAEsW30zhOxwJTaoxysS1ptJV8WRE7EqkoXMcHsfpjbvhigBhR0ulgYhAOTif7rq_sHpLUrtDIuQmleQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available according to the provided context is "ekoDB", which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: smpGicIdxDseC3WtZbVuXszqXXnw74pxWOJ8YKSyE2aJlEWbqkdo3t2cpOQFC-38zg-pu8GgC18HyudbqtqH8w
  Parent: IdJ7XroAEsW30zhOxwJTaoxysS1ptJV8WRE7EqkoXMcHsfpjbvhigBhR0ulgYhAOTif7rq_sHpLUrtDIuQmleQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: smpGicIdxDseC3WtZbVuXszqXXnw74pxWOJ8YKSyE2aJlEWbqkdo3t2cpOQFC-38zg-pu8GgC18HyudbqtqH8w (Untitled)
  Session 2: IdJ7XroAEsW30zhOxwJTaoxysS1ptJV8WRE7EqkoXMcHsfpjbvhigBhR0ulgYhAOTif7rq_sHpLUrtDIuQmleQ (Untitled)
  Session 3: KoYojp58i-Wk3pORhJffUTNtd0Ki68ZOkImoEKp6JE1zVBIyRy5o5psjVUzGT_ZnZGvdN_ihaq7n22yiVEE8Ng (Untitled)
  Session 4: o_BggN1qIUogt0S1WPe31EPA0f2Y0eNckWn74WiDxXaSXeoZx_KvkCI0z7b3iqRZc9GJPCmI5xLqZFA-U-BXkg (Untitled)
  Session 5: KnySy_WEfsp_X7M9HFbWUQFUF0_e0-g83hQ4ruClauV_v3Hx3qsidknLRmcrBq9N9LjEYo3Qf3E-Zf9vp1BW0A (Untitled)
  Session 6: Dogxv1Bte8IojZZSfjozm5Q7Bh1PRfDRm7Kts0TLSDNhwVoTkZU9B7Zyryl4unpGYGP_brfhd1cvzWqBLSZoxw (Untitled)
  Session 7: pJAkPJ50ceg34xRRKItswW6PzC6DxtbaJCb0cxXJ-vky-Pz_nkfgehDNTp6ms__wEKw2RaPUOav2-ltIrOmPqg (Untitled)
  Session 8: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA (Untitled)
  Session 9: l4g7zNZrVeaCo5zuQPZks0Yqia4jcciEDUMpkyHfX0jkRBqSmIH-phsZn7hXgLEJV4kU7DCyyt3ahLVBx7oCHQ (Untitled)
  Session 10: Z2pocQ1N977hiiEZksfO2WyAC5l-VC1h9TJzhg0S1xbXfB2ONM-Qk_Zqb6aWaumPAg1PXd6Wrerc4mYNkdmKdw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: smpGicIdxDseC3WtZbVuXszqXXnw74pxWOJ8YKSyE2aJlEWbqkdo3t2cpOQFC-38zg-pu8GgC18HyudbqtqH8w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sN8lTnbo0eczBZScGPE0q8sUUXc0QVUpCLtHoARXhheKGupZri6GvXgJxZMsOMZWeGInTURM__ZGCQZ83O_asA

=== Sending Initial Message ===
✓ Message sent
  Response: The product available is ekoDB, a high-performance database product. It is priced at $99.

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
✓ Created second session: IxBgwdbCT9z9Mtjl5_Kv-MoF93OLRm9xaxJZAwE0yrJ-C4joZ_2CyNIegYiej98z3vBnvoYFledtEVJ4sgBXHw
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
  1. Score: 25.740, Matched: name.value, email, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.755, Matched: 
  3. Score: 0.750, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: content, content.value, title, title.value
  2. Score: 1.508, Matched: content, title, content.value, title.value
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, bio, skills

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

✅ Script saved: lbEfbEVqj3ejQnJeVcv5emBlwxmi6o-jzNZYPmdSWgG_fUKaVQhKdX0K3kK6Bwt9LPPeLuLIUtv3y02BaWBD3g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 7XEZe-vrtE8hq4GPixPEtgYV96113yiDN2JEqTWTbUo8Z48KTI8SJoJeWqvTKBXlF_7m-LfYT-F8O6w2P9KX7w
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: mcAW1pqmlSTW7Pj4zDhIiD_c77pRXfC8peTmDFUmy-vB3L_exRwF5NjVDZ8Fvt1ybUtzmyDDOl4flABmddCFBg
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":5}
   {"status":"active","avg_score":60,"count":5}

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

✅ Script saved: XK_1kYWMtaBPQj7GtDlKp_ICd6juKxZCvFrWr2WTH6_HOG304ieKch9UubjoBPCsczzCjngp-8CU9ITw8WWA7g
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
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
Created Alice: $1000 - ID: DTtj-IFwRN_JdxgyGk3jUoODWWuydl-p-A3CZAHeHVJouY6sVC8Tx9BPE643TR8hsyVyvjDSmMZGxUH7vUkkLQ
Created Bob: $500 - ID: db4gZt12fnbsFaCzHz2gjJJlRXpVKghPmyjQEyPHWFAlna8oyNsHSMKtvUoypNgBHMN_AenNCNWT3R4QFvtCqw

=== Example 1: Begin Transaction ===
Transaction ID: 170de70a-773e-43c5-a586-2bc6c7f44156

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6f3f4253-a414-44ea-a914-ff193e024364
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 785ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:63719) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:63722) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'SYZ1qTh0Sw4yQLPMQSsB7XikiIzYeYf6a8ZK40BZQvc_xD8Mn_Il-TrIwhuOihtnF3-NXKBemUTnM2eE5Tf0gQ'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'SYZ1qTh0Sw4yQLPMQSsB7XikiIzYeYf6a8ZK40BZQvc_xD8Mn_Il-TrIwhuOihtnF3-NXKBemUTnM2eE5Tf0gQ',
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  id: 'SYZ1qTh0Sw4yQLPMQSsB7XikiIzYeYf6a8ZK40BZQvc_xD8Mn_Il-TrIwhuOihtnF3-NXKBemUTnM2eE5Tf0gQ',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:63750) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: Pa7R3PiGt3p4YlYpqhrD2eixvMIcEmIj35VogIHxpACw8ZSuFt-YKnbU3UVubX2Ul2LmPIVfDBCsvsdmtwvm4g

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
        "id": "FTsekewxgB5micEApFUENESf1T4V1PoJc7ytPFR4kyIC6AB3mCv18L8t7JYVDT6tkCGtqwz8jXgnOOsmcpkPkw",
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
        "id": "Pa7R3PiGt3p4YlYpqhrD2eixvMIcEmIj35VogIHxpACw8ZSuFt-YKnbU3UVubX2Ul2LmPIVfDBCsvsdmtwvm4g",
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
        "id": "v_o_VMTYzOkploU1BbN3TCNs9Kbc8H7BASVbe-rp30wX4gyOWAjC8jUwlFCsg2pqbeSVbXBiD-nK4oMNdpJJEg",
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
        "id": "um21JmrqMHTOaWiaJQsXKEO81RaRGKGHACyQBVWU8yNrbkOgM0K0tZKnikMWm0v1nnEdc0jxGdcOI5tZuN4HRA",
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
(node:63753) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:63771) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2026-01-03T23:55:55.634Z',
  userId: 123,
  username: 'john_doe'
}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
(node:63774) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: N90CNCySk2MviTVtaHQStwtewhrTXm3OL6LCOWM1v67kWoX6vT1wJ00ryA1UYYS-T_CYiJkyWxs0bVYzLCKDKA

=== List Collections ===
Total collections: 23
Sample collections: [
  'chat_configurations__ek0_testing',
  'ttl_cache',
  'schema_documents_client_py',
  'schema_products_client_go',
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
(node:63781) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: TbyXfjzonHNqpJSuKgdjN_8_0-QA6mTWppnrFBdAQoMJhlzqIclckVckgiKh7k7Rp3hR9bV5Z27Wf41TNth3vQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: TBRjQKeo82t7U4XdDkspbtYrO7YqNxC2yN6Wa07golFN1vP3Gk1P11YRpe5r-lrlDa2hXwCOM4IomKLsjBxAjA

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
(node:63794) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: yUlYb72KZdbfjdjp7Rs7ATc5SFuQVDCU7hAq8D2I4WMsZps1D3IDV-XS9DFNb5vdzWtsgZN0EZsmp3OsgiBO-A

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

✅ Script saved: 9a2aJf8FbjxrN5s1rqPIfCa7NOpMG6DKCyVmUvDGChL9iSi9P6JpXb3IHmWGG6lMgjL_LHV05WycBzDHYwGL3w
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: muMcH8W4Z1VEbEKAlortntWmyk_WMsxUzAoT2DFnBq2SRWJDvQ90Sq1d2SKzhaRuSItU2begWOQIkbRGbR8BYw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: L6ng3kPi6HgMTjRuSvBinF_rnVavucBXQgOLWoGsOrlF20XvPDOTNycuO8gEIiJaKOKoBIMXQ4wd9De90MUh_Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","count":10,"avg_score":50,"max_score":90}
   {"count":10,"avg_score":60,"status":"active","max_score":100}

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
Created Alice: $1000 - ID: O1zTjj8jE1m4qYXCGY8k2JRHs8dvw20GAnmAdeanlZUdhh4ycasXeNBph9l9-gvXCautzZ6SbY_cA4UvP3ybNg
Created Bob: $500 - ID: rK7g8fMB2xYMhKJQCwUwv38VdWH8EcRKk33EJgG5JYk3Hd0RknBuiVaozUfSuUFnygY473JvpjSDFdTTCz_fLQ

=== Example 1: Begin Transaction ===
Transaction ID: 09b38310-9548-4e45-aca8-8067c5d9eaec

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
New transaction: a1c3c9d2-537e-452a-afda-068bf39570c5
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
   ✅ Script saved: tVdexwN51KiAsI9PODtABctJrxz4jPR8-6HL2sdLw1-_Bovhe-YZAGPd5Ol413GIJ5z8BsYSyHi0iUrawqYurg

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
   ✅ Script saved: tO8cqTKmx5y8CvhFevzRiSJX7BBzrOL74vve6fd0PF5IENVyqbZAFptFzVEHudgVwsW9roFokKuTCXsIP3IMoQ

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
   ✅ Script saved: vS77pK3n_MjnZToV9jtJ1SCL1yYTmJoZ65H_vOMMe0YAn1fXAghWQPqxSQk0FzOLVsWb9izdCYPWRcpETj2-7g

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 7ZsLnO6AjMHOhKl9fFGHFjIkEuKdN1YI5MscygY7monRdZKu8DXcGjqyMyg8inv7Qyfym_sjh7RErSFCW34EQQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: tVdexwN51KiAsI9PODtA...
   ✅ Deleted script: tO8cqTKmx5y8CvhFevzR...
   ✅ Deleted script: vS77pK3n_MjnZToV9jtJ...
   ✅ Deleted script: 7ZsLnO6AjMHOhKl9fFGH...
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

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 818ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'bW5kJIQriWHsPlRmQh32eRMVAsgq7SeyGCkC5VK2mOf3aEGhCB7lGnTm1pUxFHe2gXbCFBpP2z714fdiPrc8LQ'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  id: 'bW5kJIQriWHsPlRmQh32eRMVAsgq7SeyGCkC5VK2mOf3aEGhCB7lGnTm1pUxFHe2gXbCFBpP2z714fdiPrc8LQ',
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'bW5kJIQriWHsPlRmQh32eRMVAsgq7SeyGCkC5VK2mOf3aEGhCB7lGnTm1pUxFHe2gXbCFBpP2z714fdiPrc8LQ',
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' }
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
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
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
Collection created with first record: _YenXnTXAsZ4EKQdAP-iCK5EA6Ijpqik5waHkS-RfkqQf4gDdiIiOhIBSMwvnRjhgemJIvuhJSJani1Td_1sMQ

=== List Collections ===
Total collections: 22
Sample collections: chat_configurations__ek0_testing,ttl_cache,schema_documents_client_py,schema_users_client_go,schema_products_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: RjBpRSQRVYXh4hmPfyc1bekbLPBXswv_-FPT_7qH-VtW4FCiJ2yr5BJwMQgjlNahQfcDOLjlYAXZ2NSWjpZ0iQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Ca2sdieU5CQuoDopdENeFPJKrXb_jh80ikNbaGfT24xEk7kxfRcBrwd0Utsram_WuSrXrtfUN4G9l9tPRgxx4g

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
✓ Inserted test record: kJvaulA8lBQmtH1JXM5fCyTQg5ub6wFPVFdkOR7tw0dklxh-BK-Dken14Q_-GchLbchIsklp0mA2F8fwjWiYyA

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
✓ Inserted document with TTL: 7ukUmLOxAUeo7EMFuWFWiM8Ugj5OmHuQSnPveC0p4JZEMND-2JUgz6kDRq0h3rNnQUR3anTlT-SYGvdS8kavag

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

✅ Script saved: vGRqsGmi1VPmRx2_Ta8VEL4YCdxlmXDcNa7rHgxlfQirO1PDoSWla3zM1fFN_CGL8YKOeCYh16RVTyhioKkUJw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: X8Q0THXGLI1GED_Q4r4C9O6XV6iU-bwbJPlijvo4gY_ep3SEblCGiE9qhooT0znkdO2HEDOoWw9Cbm_NCYNpNQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: LVVWpO--8b5hTmE2EOtmOtAr-i3xnkrMMZu2bptAmP7doE-PWmCF9243uKJzzNncWJ-BcKhV0FBVWn2xiVmqXg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"active","count":5,"avg_score":60,"max_score":100}
   {"max_score":90,"status":"inactive","count":5,"avg_score":50}

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

✅ Script saved: A94XyKzBhWTC5J3mo4_UFgSdusCeL7zi2_RIn7e1a65TFFfyPMbh9K6TMHy8EV6PU5-tR4xdSzTf3MIynmN_tA
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":10}
   {"status":"active","count":10,"avg_score":60}
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
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved
📊 Category analysis (2 categories):
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Laptop Pro - $1299 (⭐4.8)
   2. Monitor 27" - $399 (⭐4.6)
   3. Bookshelf - $149 (⭐4.1)
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
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 0):
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
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
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Natural Language Processing
   4. Getting Started with ekoDB
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
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
   1. Scalability: Vector databases can handle large amounts of data and scale as needed.
2. Speed: They provide fast data retrieval and processing capabilities.
3. Flexibility: They can handle both structured and unstructured data.
4. Accuracy: Vector databases allow for precise computations on spatial data.
5. Real-Time Analysis: They can perform real-time spatial analyses and operations.
6. Storage Efficiency: Vector data requires less storage space compared to raster data.
7. Detail Preservation: They preserve details even when zoomed in, unlike raster databases.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Draft Article" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
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
Created Alice: $1000 - ID: cUFPmpy2FkbzwNgpTQw21bm7Vr1OKboLc7qcH_VqDLBxGp8sxB2JVfhdFfVcMuZJLEw7bOX4k7uSM3eXCaw02Q
Created Bob: $500 - ID: 1fUT3FhQ8kSJlJBhXoBNIgtIOn46zwlfaPGlh27x4zTCW_Yf_oPu7hzPchGMDpk2FtEWX-cJrnznyNigIs-FyQ

=== Example 1: Begin Transaction ===
Transaction ID: 73366db5-7624-49c1-a52d-eb295ed7e0fe

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9edcc8ef-0258-4573-9dde-43170feefe0e
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
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: Record(fields={id=StringValue(value=9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw)})
  User ID: 9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw

=== Read ===
✓ Found user by ID: Record(fields={price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), id=StringValue(value=9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 18:56:27 EST 2026), type=StringValue(value=String)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Jan 03 18:56:27 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [price, embedding, data, id, created_at, email, name, tags, categories, active, metadata, user_id, age]

=== Update ===
✓ Updated user: Record(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 18:56:27 EST 2026), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), id=StringValue(value=9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Jan 03 18:56:27 EST 2026)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), id=StringValue(value=9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw)})

=== Delete ===
✓ Deleted user with ID: 9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: Sv7fRPoWskaSh7MOY8DeRKZq1zvKqc5eCeQPTV7mq4Tdag2NiZznQgOeuaZnWisxMSrQ3dG34EkoDDKfsAKyZw, SyhLpjHsT4e09N9k_Z2V4PCv0HAIwz2ktPdw3p1zeokxzFaHv8AxKnYlOwuuQOC7kBSYIynAt43eFd-H2i3-ZA, 6mqmWGBbEZKa3HecFT181IFqkgPLh8I2J7o32jOaj5mEcSeesU1u4tCNVnhqwDRKhOjcUdwODDEd_mvSYgHKng...

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
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={value=StringValue(value=Charlie), type=StringValue(value=String)})
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Diana)})

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
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1767484600356,"user_id":"123"}
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
  - chat_configurations__ek0_testing
  - ttl_cache
  - schema_documents_client_py
  - schema_users_client_go
  - schema_products_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-03T23:56:44.601002Z","last_modified":"2026-01-03T23:56:44.601002Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: f6x8ipn2I5sVEOKeviOtTWey53RnWoFgeCkfDiJg23sfuTVyA6aoXhx9GHvnwiq1JmIXnYrb2Sd-B1C1pOMnvA

=== Verify Document Exists ===
✓ Document found: [session_id, user_id, id, created_at, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: YPbOOfyf1hb8aeS82knXRdcLOFp1iEqMtnibc3HZI_Pqhmv0kaHDXb6PGxnTXVvNTTUMcajEhpaApK1bihgJaA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [id, cache_key, value, ttl]

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
  {"payload":{"data":[{"id":"XyWZtPLmGvgb9g6C12H9eKS-T4d2uLphbqDNsYKptbVo-OEqa6CgSBqhjGKGlRIlrgQJ08N6Zvr4Gf4DNXpNdg","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=1kTn_oK6KiXvCAWB4OPplxlvnweFNEeEFeHuInZu1z4wUOIw6SG3UCJKsufTnVVSuLEommw5evP47FkQwJ7BNQ)
✓ Inserted user 2: StringValue(value=WJT38RqAihJSmu0HEZHOMP86bz-7FSkWF3RlCXPfgM6XYTBXPcLISq2Cq1D99B3RKp2Wsx0bA9urV8Bs7sXL1Q)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [chat_configurations__ek0_testing, ttl_cache, schema_documents_client_py, schema_users_client_go, schema_products_client_go]

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
  {"results":[{"record":{"id":"dMCXNBQC4VAcp61_2b82NoJtnJZKaPF2wtbVaCUNQtLkhDmS22POHf8jJfWfmwXeDcyH-EJgBRnZgiW4Blu8aw","views":811,"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["description","tags","title"]},{"record":{"title":"JavaScript Web Development","id":"UX-DJtsX-7NlzrSaIt88EBzR92dP6ouoG_5VLh27UOzgda9NrevXhUtm6W1ysDTh-qD76hli_jY5Nga7lztk_A","views":98,"description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","title":"Python for Data Science","id":"9ZFsUa7nwEJRbx3Kk2MnvZ_YX7GNGS3i5yMk6F79L8tYYZ2aYVNtgEWiIbWctGOQhDWKQaQclVN7VI07quKmdA","views":534,"description":"Master Python for data analysis, machine learning, and visualization."},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"views":363,"tags":"ai,machine-learning,python","description":"Introduction to machine learning algorithms and neural networks.","id":"eXGkyzXIbd_i8uVtyEMb3ihrHOrdBYkAr0yk0G6BBCjLpAfB-MVkqm_fL9oNkfFA-FcvuwieM_TR9O9oOy6ddQ","title":"Machine Learning Basics"},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"id":"9ZFsUa7nwEJRbx3Kk2MnvZ_YX7GNGS3i5yMk6F79L8tYYZ2aYVNtgEWiIbWctGOQhDWKQaQclVN7VI07quKmdA","tags":"programming,python,data-science","views":534,"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"views":798,"tags":"database,design,sql","id":"31jK_X2y-2LE5AIl2ebh5yzRGf_2MeZOq-0QKWkkZUcHD0lH8jW25ysVNz7KtYoL_F5lVPK9PPDx9BZkvQMsVQ","description":"Learn database design principles, normalization, and query optimization.","title":"Database Design"},"score":0.5,"matched_fields":["description"]},{"record":{"views":811,"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial","id":"dMCXNBQC4VAcp61_2b82NoJtnJZKaPF2wtbVaCUNQtLkhDmS22POHf8jJfWfmwXeDcyH-EJgBRnZgiW4Blu8aw"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=RoBNWzKKjSRbVQfAPQLCVLQNBFftOO_co95MnieYIGDAUCHI0JVDWVlsR8B0DEjE9zDAXU4CfbxBfmfB0NEO_g)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767484625727},"id":"RoBNWzKKjSRbVQfAPQLCVLQNBFftOO_co95MnieYIGDAUCHI0JVDWVlsR8B0DEjE9zDAXU4CfbxBfmfB0NEO_g","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-04T00:57:04.859161Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

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
✓ Created session: 8SpPtv3cwJSd3xsRpUIzj0mNUTDsgG8nI5wxfWokyjmtYUSb6sDYkuBl-kCq6_RB7nXNspuQusgJJ9RWuMwgUQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "a2XTWZ_gGnOfdroVp00eujhEG9Z1gll7Z6mcL561kPo65YzaEOMFQeTW4PBJVQr2edC2SMokNjd4hPhaYdkNrQ"
  Responses: ["ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. Some of its main features include:\n\n1. AI Chat Integration: This feature allows you to query your database using natural language and get AI-powered responses with relevant context. \n\n2. Search Features: ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval. These features make it easier to find specific information within the database."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the available product is ekoDB, a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"S1yEU3LWbs1t7t9Tx19Xj36RoP5yHZl_K7KeYpq-NuuPnEks8GzDvO444PheKC0il_8c5VOdorSftf9ELwN8iA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T23:57:22.726197Z"},"id":"09zruanWWuyP522Ejqp6HG6Q6THP_YPM0bABoakhZ4BRanBYpQign7tdxneSYpl96i2-RgHe3JU-qDAAhSPLjQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":242,"total_tokens":274}},"updated_at":{"type":"DateTime","value":"2026-01-03T23:57:22.726197Z"}},{"chat_id":{"type":"String","value":"JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ"},"content":{"type":"String","value":"Based on the information provided, the available product is ekoDB, a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"S1yEU3LWbs1t7t9Tx19Xj36RoP5yHZl_K7KeYpq-NuuPnEks8GzDvO444PheKC0il_8c5VOdorSftf9ELwN8iA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T23:57:22.730324Z"},"id":"RWOpaoIaQlQDRlbdaKZhAo9EoYEf1D8-noPieH6zGXLNvLI05qLMZ5xLnlHHsrgoa5i0R30VNgDR118ApwuuMQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":242,"total_tokens":274}},"updated_at":{"type":"DateTime","value":"2026-01-03T23:57:22.730324Z"}},{"chat_id":{"type":"String","value":"JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"S1yEU3LWbs1t7t9Tx19Xj36RoP5yHZl_K7KeYpq-NuuPnEks8GzDvO444PheKC0il_8c5VOdorSftf9ELwN8iA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T23:57:23.513431Z"},"id":"6oKRRjewuTuCjhMaIwj7MhxW_oBDWdN9fBi2wKg4E7u-BbUbKbTnIPieBZ2aXfFHYUTqz8H22YFV_b3efuyWzg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":242,"total_tokens":254}},"updated_at":{"type":"DateTime","value":"2026-01-03T23:57:23.513431Z"}},{"chat_id":{"type":"String","value":"JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"S1yEU3LWbs1t7t9Tx19Xj36RoP5yHZl_K7KeYpq-NuuPnEks8GzDvO444PheKC0il_8c5VOdorSftf9ELwN8iA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T23:57:23.517534Z"},"id":"_qb_7dLn9MAROm_JYSG6RHKlDrdz9im_vQM9_wLe3vyEgganAckM_RbdCJHVaiL6aeeeuEcQA9aUDyvhlvEgEA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":242,"total_tokens":254}},"updated_at":{"type":"DateTime","value":"2026-01-03T23:57:23.517534Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: Ly1oztPgN77vzMgmT6VOakL7wqhN6nEdmbq6kL3QIY3f-2DrlgqPvJKel4Cs9FNYHWDryGeDnbZL0bactAa9kw

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 5s
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
✓ Created session: 1xn6wLlYnWZvsIsaGcBNTfuomCNL9ttoRtiBfVpFNoSuTnMUVlUJ7pvsbP8CPCAsf49h0d6WvIl52xzJYpKO-w

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is ekoDB, which is a high-performance database product."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The available product based on the provided context is ekoDB, a high-performance database product. It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: icKUeFOq6a-1Zrapw-ZR_uHoCXvyDcoi_yZRN-i83ZUh0cPKWdspPX892bXdhV_XtMolJ_Qv-2ILQd7UpxUj8g

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

✅ Script saved: MWCSWskkBe0mi9-OAGzx7RjDcJ1xHlP6jZlRzJKMNLHKSZLa2Bb0j0ukhdGyalyYEpvbXk9lB2R0pL0CGzeIbg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","avg_score":60.0,"count":5}
   {"count":5,"avg_score":50.0,"status":"inactive"}
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
Created Alice: $1000 - ID: anP3HCboqjozxp5ySPVacF0WKg4vKrhpSnU7FWGFcAMm_nDKQ-_gEvChb2AtbTtRii5QG7oz4py665uyqy9qXg
Created Bob: $500 - ID: zzG1JPdbcZE91shIwUl4-iaX4ysx3o6o1jWsVhS-SylnSB53IBGf9O5xxdrYFV9zvynqQRd9iBeqEBo8AcHXow

=== Example 1: Begin Transaction ===
Transaction ID: 03e71918-3873-4fe7-bfb3-54c244d3f1ee

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 83fb6ed5-ce2b-418b-92ab-09e668dfed8a
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 277.420458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 528.7515ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 316.235958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 230.590834ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 450.457709ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 603.13025ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 503.608917ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 515.421791ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 315.298916ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 254.053ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 625.889625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 257.17725ms
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
    ✓ Generated embedding: 1536 dimensions in 443.753041ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 101.981625ms

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

Writing memory-safe high-performance database code involves various strategies and considerations in both the design and implementation phases. Here are some tips:

1. **Choose the right database system**: Depending on your needs, SQL or NoSQL may be the better choice. As discussed in previous contexts, if your application requires flexible schemas, horizontal scaling, high write throughput, or dealing with unstructured data, NoSQL would be a better choice. On the other hand, SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

2. **Normalization**: As mentioned in Context 3, database normalization can help improve data integrity and reduce redundancy. This not only makes your code more efficient but also can help prevent memory issues by eliminating unnecessary data duplication.

3. **Optimize your queries**: Efficient queries can significantly improve the performance of your database code. This may involve things like using indexes effectively, minimizing the use of joins where possible, and making sure to only retrieve the data you need.

4. **Use Prepared Statements**: Prepared statements not only provide protection against SQL injection attacks, but they can also improve performance by allowing the database to optimize the execution of the SQL query.

5. **Connection Pooling**: Reusing database connections rather than creating a new one for every operation can save a significant amount of resources and improve performance.

6. **Memory Management in Code**: If you are writing code in a language that allows manual memory management, be sure to properly deallocate any memory that was allocated once it is no longer needed.

7. **Monitoring and Profiling**: Regularly monitor your database performance and use profiling tools to identify any potential bottlenecks or memory leaks. This can help you to address any issues before they become bigger problems.

8. **Caching**: Implementing a caching mechanism can also help to improve performance and reduce the load on the database.

Remember, what works best can depend greatly on the specific requirements and constraints of your project.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 299.307834ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2172 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 420.553416ms
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
  ✓ Text search completed in 61.65875ms

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
    ✓ Generated embedding: 1536 dimensions in 0.247s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.559s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.291s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.451s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.562s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.495s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.286s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.267s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.241s
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
    ✓ Generated embedding: 1536 dimensions in 0.772s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.089s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of best practices in both database management and software development. Here are some steps to consider:

1. **Choose the Right Database Type**: Depending on your needs, you might choose SQL or NoSQL databases. SQL is better when you need to perform complex queries, have ACID transactions, or structured data with well-defined relationships. NoSQL is ideal for flexible schemas, unstructured data, high write throughput, or when horizontal scaling is required.

2. **Normalize Your Data**: To improve data integrity and reduce redundancy, use database normalization. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

3. **Optimize Your Queries**: Use indexes effectively, especially on columns used in joins and WHERE clauses. Avoid SELECT * and instead specify the columns you need to cut down on unnecessary data transfer. Use LIMIT to reduce the amount of data returned. Also, avoid N+1 queries where a query is run for each row in a result set.

4. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks but can also be more efficient as they reduce parsing time as the database can reuse the query plan for a statement.

5. **Connection Pooling**: Instead of opening a new database connection every time an application needs to write data, use a connection pool. It saves the overhead of establishing a new connection every time a database transaction needs to occur.

6. **Caching**: Implement caching mechanisms to store frequently accessed data in memory, reducing the need to access the database for every operation. 

7. **Concurrency Control**: Make use of concurrency control mechanisms to prevent conflicts and ensure data integrity during simultaneous access by multiple users or sessions.

8. **Memory Management**: Regularly check your database's memory usage. If it's using too much, it may be due to inefficient queries, or you may need to allocate more memory to your database.

Remember, every database and every use case may require different optimizations. Regular monitoring and testing can help identify bottlenecks and areas for improvement.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2243 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.320s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

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
    ✓ Generated embedding: 1536 dimensions in 0.332s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.226s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.256s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.442s
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
    ✓ Generated embedding: 1536 dimensions in 0.237s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.333s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.323s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.487s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.200s
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
    ✓ Generated embedding: 1536 dimensions in 0.454s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.082s
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

Writing memory-safe high-performance database code involves various techniques, but the specifics can depend on the database system you're using (SQL, NoSQL, etc.). Here are some general tips:

1. Understand your database: You need to know how your database system works. This includes understanding how it uses memory and works with disk storage. For instance, in SQL databases, understanding concepts like indexing, normalization (as mentioned in Context 1), and ACID transactions can be vital.

2. Efficient Queries: As per Context 5, optimizing your database queries is crucial for high performance. Select only the necessary data, use joins wisely, and apply proper indexing. Also, avoid using wildcard characters at the beginning of a query, as it can lead to full table scans.

3. Connection Management: Use connection pooling to manage connections efficiently. It reduces the overhead of creating a new connection for each operation.

4. Use Prepared Statements: They can increase performance and also protect against SQL injection attacks, thus providing safety.

5. Concurrency Control: Use transactions to ensure data integrity while dealing with multiple concurrent operations.

6. Memory Management: Be mindful of how you store and retrieve data. Large objects can consume significant memory and might need to be handled differently (like streaming data). Also, ensure to close the database connections, statements, and result sets after usage to free up memory.

7. Profiling and Monitoring: Regularly monitor your database performance and identify bottlenecks. Use profiling tools to understand where you can optimize.

8. Use Appropriate Data Structures: Choose data structures that provide efficient memory usage and performance for your specific use case.

9. Use Batch Processing: If you are writing or reading large amounts of data, batch operations are usually more efficient than individual ones.

10. Hardware Considerations: The performance is also dependent on the hardware where the database server runs. Consider faster disks (like SSDs), more memory, or a more powerful CPU if the database operations are the bottleneck.

Remember, the choice of SQL or NoSQL (as per Context 3 and 4) can also affect how you approach writing high-performance database code. For instance, NoSQL databases might be a better choice for high write throughput scenarios, while SQL databases can be better for complex queries and well-defined data relationships.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2467 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
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
  ✓ Text search completed in 0.053s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

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
    ✓ Generated embedding: 1536 dimensions in 0.226s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.216s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.424s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.447s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.218s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.289s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.294s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.414s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.502s
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
    ✓ Generated embedding: 1536 dimensions in 0.250s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.083s

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

Writing memory-safe high-performance database code can be a complex task. Here are some general guidelines:

1. **Use Prepared Statements:** Prepared statements not only help in preventing SQL injection attacks, but they also allow the database to cache the execution plan for a query, which can significantly improve performance when running the same query multiple times.

2. **Normalize Your Database:** Based on Context 1, normalization is a process that organizes data to reduce redundancy and improve data integrity. This can also lead to performance improvements because it can reduce the amount of data that needs to be read from disk.

3. **Optimize Your Queries:** As suggested in Context 5, optimizing your queries is crucial. This can involve avoiding SELECT *, using JOINs instead of multiple queries, using indexes effectively, and minimizing the use of functions that prevent the use of indexes like NOT, % (for LIKE queries), and OR.

4. **Choose the Appropriate Database Type:** Depending on your needs, SQL or NoSQL may be more appropriate (Context 3). SQL databases are generally better for complex queries and structured data, while NoSQL databases can provide higher write throughput and are better for unstructured data.

5. **Manage Connections Effectively:** Creating and closing database connections can be resource-intensive. Use connection pooling to reuse existing connections and avoid the overhead of establishing new connections.

6. **Use Transactions Wisely:** Transactions can lock database tables and make your application slower. Use them only when necessary to maintain data integrity.

7. **Avoid Memory Leaks:** In terms of memory safety, ensure that all resources (like database connections, result sets, etc.) are closed properly after use. Use tools and language features that automatically manage memory for you, if possible, or else be very diligent about cleaning up after yourself.

8. **Testing and Monitoring:** Regularly test and monitor your database performance. Use profiling tools to identify bottlenecks in your code. 

Remember, every database and application has different requirements and constraints, so what works best will depend on your specific situation.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.397s
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
  ✓ Text search completed in 0.053s

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
    ✓ Generated embedding: 1536 dimensions in 0.336s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.382s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.222s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.222s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.324s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.47s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.219s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.678s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
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
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.085s

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
    ✓ Generated embedding: 1536 dimensions in 0.29s
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
  ✓ Text search completed in 0.048s

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

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (GL9zXKjvEOyJNpOvFy7xZSVpe3R4iX9hVvaL96HFP8Zsymjy16ZKT2WT9W36sCjVYYmKXGHspblXNnC9wB1mKw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269740,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
      "id": "6mBT8b2vz8UcRMtOuFivHOpkMpzgXUo8UDKZJlRAkXclEQOxQugs7kiFe5ufcthd5CZsMqmRNh2gByu1_e8iLg",
      "cached_at": "2026-01-03T23:58:54.986Z"
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
Response time: 80ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (7cbCtIR1SLcOA3vkkpSYZw61sIYTmS_eyymq4jctfsdbe8gvn9TTrgF1fnLpjQ-Y9CxPqex2xs6UuPbPk-RPMw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
      "enriched_at": "2026-01-03T23:58:55.244Z",
      "id": "SGBUWnYgXsBsKWgujCT2RkzCt0489i3lRkgfTq98BcbcRYql0TyJu_ik8zvs-amC5AZtcgjFvnntA8uU2cdMKQ"
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
✓ Edge cache script created: ANrJ1EYR009KnpPPSXsd2N6hT1B-35tzQOCusFlJutmxHZ2TYELQwKqHOqppWytEYAby9uZgHZ1gcwY2JkhaoQ

Call 1: Cache miss (fetches from API)
Response time: 594ms
Result: {
  "records": [
    {
      "id": "qUWy4LQSGvmPfYa3tg0as6V9zUmbc2DU2p2NswX_3gnwhq16y32VtatZu24pKBXq9U3BDu-7ih6rN7Ryfm3ocQ",
      "cached_at": "2026-01-03T23:58:55.751Z",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.037670135498046875,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2026-01-03T23:45\",\"interval\":900,\"temperature_2m\":-2.2}}"
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
Response time: 572ms (1x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-03T23:58:55.751Z",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.024557113647460938,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2026-01-03T23:45\",\"interval\":900,\"temperature_2m\":-2.2}}",
      "id": "Dlu76NpxgM2CjIpLn1r-jlGJXk1y1mGKZ-2jtXC3uUiWQbqXXU6fE67UdnPdnpu1os0eBUqXH0TfJYbs2OmGfQ"
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
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.7.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (bEQCcOsHNNMQO1YRycseV2kwj7HPFn4KHTnZEqb8ak6ob3anX12hAQBaCpAq5nzHJhilBNM2idtyKVGvb3GBFg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 73ms
Result: [
  {
    "cached_at": "1767484738",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269740,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "ou6u8CSIE7CkI7_PdZOBFFt52SHQsLnN1siYNiCY8GwmSDZpnp0fcL6djJ5hkkIOD2xdoR1i9ZSmJsK6F2upSg"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 78ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (kkY5R3_BAVq_Rq_q22zcBm-HdxmM7O0f97Pi0Qmuy5eSvKLr2SMje-_Ss80pJ0z8ERxhplcUOUC-NxQmMSrH1g)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767484738",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "a1YuP2BVdZhVBThCoqbWOw51sPG0_Uz60qiKsWGxlWfasEMP4ZPdW4y2Zylc1xrIWGqQowxAmE3hPyZYFVZwkg"
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
✓ Created SWR script: fetch_github_user (iErdPiTPvlZmnmgRd5_mUb_mZGF9wwutWx91MRmD-Ld4S7M_idmMpOXtdxzamD6XrgbNj0XY0xWykOItrQRwAQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 83.805333ms
Result: [
  {
    "cached_at": "1767484738",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269740,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "umFeuAzPmHY88p2DMydt-kcrV3_uXOXV17gBFmGFw6bwQm0BCNWuezl3_VukKOLvlkG6i8V6swJHngF8Yq226g"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 86.367125ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (0aYpLwGvzCaPEhtWpglslcy1kVUiRX7-5q_7DD2cBrKBfPmYnSEDt9csx2Q7F2CSmAoKKQS3Z1I6eLLLTHfHYA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767484738",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "YAwWqg1qywNo_p9pl06vgpitP3-O6ZpA06DswEcEc1GobBGnDozhcrd-OUCXyIdYOiNvixpw2od0tlnYDoKnLg"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (hj1etc_9X6KxksEYzX1eS9VA1j65fa3fCuSS4_KgLQfEolaSHX8pJmkfXks9RpIWDx-lG8PrdA7uuInLrozALw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 86ms
Result: {
  "cached_at": "1767484738",
  "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269740,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
  "id": "CoA5bwaE-ClXm9En7-vOJHdhxmWX9YYv-q52l4vBYkndxRQnDRYhBx4MJLPNLNfguApzgv2OL2vpuOgtJIe2uA"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 83ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (XjfP1nYISP1f7NPhuK8klyStv53Hj9umiiR6YOhZf3yTgx1CnFWQoLg2ZEa7FMfIVcs7jCZUW9qO7YsAlph8_g)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_at": "1767484738",
  "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
  "id": "JAkOxsiN-ITGkfSwW9_zIusODzR6F_WuYk1AtAynZ3RE7o--I4g41SCc-bCsg0xQS5ces3VExhnoGoyGQvmBzw"
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
✓ Created SWR script: fetch_github_user (uNZSncGYyP7kBt3zkViPE5OFjV7CyFeFjyBwMER87_Rw4tc4oxeMefT4-Tzw-MJlaQgkf06jsSkto91QQegB3g)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 96ms
Result: [{"cached_at":"1767484738","data":"{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269740,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}","id":"Mlvrl9Q06t1-RPWAZFECivkbkr_BLRL9vOhMrkyYIj4vWiLgbNlA5OCtvRSAs1w4h2JKdcHryNzdcKTbkAw2Ew"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 82ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (WjQrFuby6nqbrzvu4IsIoUQXN3cdZeKdFX4y5UL7ulV5_wNyK6VRVDhCu8_ra8ZV_7b1_a6kLDBHvuSWm3dnxg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"id":"8cy2Zn1sX1TfgKpCzFJUVE9TQLV6hS6ISW4DD-O4iNw6WdMFDlb3HGdxMP6l2opEzPv5S7cHLo_9DoIaFul8BA","enriched_at":"1767484738","enriched_data":"{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}"}]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
   Department: Some(Object({"value": String("engineering"), "type": String("String")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 81.633125ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 86.27775ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 58 packages in 979ms

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 86ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

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
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.0
    Uninstalling ekodb_client-0.7.0:
      Successfully uninstalled ekodb_client-0.7.0
Successfully installed ekodb-client-0.7.0
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
   ⏱️  Duration: 82.0ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 88.1ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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
   ⏱️  Duration: 77.749541ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 85.496583ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 88ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 84ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

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

