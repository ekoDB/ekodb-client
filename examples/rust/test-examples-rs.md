make test-examples-rust
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
