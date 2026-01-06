make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("a_aYQoNx3-F6jJSth0QHL6f4nadWMy7dLqq5jZppj0nc0vECP0jXLUPCQTvlgtVedQ1lGyU2902Vumw7QhB6MA")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("a_aYQoNx3-F6jJSth0QHL6f4nadWMy7dLqq5jZppj0nc0vECP0jXLUPCQTvlgtVedQ1lGyU2902Vumw7QhB6MA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("a_aYQoNx3-F6jJSth0QHL6f4nadWMy7dLqq5jZppj0nc0vECP0jXLUPCQTvlgtVedQ1lGyU2902Vumw7QhB6MA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("a_aYQoNx3-F6jJSth0QHL6f4nadWMy7dLqq5jZppj0nc0vECP0jXLUPCQTvlgtVedQ1lGyU2902Vumw7QhB6MA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "5yKdWQgJ0eshje0NLVcqSLxJGHZEoxIMmSlenatXgi9XL08hgn5_57TrAMXSKiD0F8B_VHoOI3uK8BOuAr4dVQ"

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
        "id": "5yKdWQgJ0eshje0NLVcqSLxJGHZEoxIMmSlenatXgi9XL08hgn5_57TrAMXSKiD0F8B_VHoOI3uK8BOuAr4dVQ",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
Collection created with first record: "ASooY2Dni75fH4OqtxJ8WClo9f8mnfs_W9gcF_VTeM30AqwYvpJV5MFuUkbYw5hfxHnr3MRwEphF2ZcyjHELqw"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "websocket_test", "demo_collection", "test_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "vxUuc9fNI7xGz72Hu9PDmfGKlDyE_12xECip-YSvLBrOM5TPur3qx18fjc6tTr60nCze9qPDryouNZEkW6RQyg"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "DXgK21fYxNLWGtsXyPF07XqasOQv0rtTsFlvcH7wpEelANOtf33dlz7GuUpdGyUgUfPB_rHf227vLcOi_pwJKg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "5uE6mZtHhSpI0yy7gi85pI191fnaExTopCB2LW5sDfSFdCWxYzbcTjbSWqhwHDGRDqrmLKvKz-7dbcN-536wsA"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("Pdrpz_yT7e3nIl2inpU17Yx_s7qMimr-UJJQzCL8ZC8fZIAJYj_sI_SrbYnko_kjouRP4j_YiA006BR6hpGZDw")

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

✅ Script saved: Y2j-gi0DqYhfj2XEcs3yan-5553nZJyBJE7bsqSBVInUhtCOzMJ4Bm-EehXh9ygLKmMrQyzHSpQ4A_Js4E02hA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "w4VdYHtgziuFpHT_y3Yaozedt2S9Yb0Hv2q-8mw6Y0qcOyYXYgKEgA1_Cg3DWmvFd6HDYGi2aNE2w5UbwunvBA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: V_tIALXxGSCSHoiG6RokUVc_94zOKIRWOesRZo1ssqPqaWVdn6pmqRbFx4opLT57Q5D3fiXC4wPcKhNUZGT0lw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: pzC2jORIgU7O0mdejzgWwu0-m2GzZd5f7oefMkSB4xEsTmk7Z1pdphVHkcKq_UjaYRiuuVtdqKXjpsX7-klzXg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: T0BO1w9wh82Pe3z_LOcAU4Zp99aVBFPOk9F10ihQJNUoAH2FsVr66fMKaGEBZ-aY-Jvmanrf4bwvW2W5qQhL-Q
Created Bob: $500 - ID: VRBTM0s5hwA4Z3EoTpp78960VYKESMRuk-m6ooOGd4wLNcpkPHcPGSzpFrNXFb1MwM7TnKkPaaT48EmWAA8qCQ

=== Example 1: Begin Transaction ===
Transaction ID: 06aec92a-8ac6-4d47-84e3-db0b9c45898f

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
New transaction: aedbdf39-7d8f-4da6-81f4-7f19bcfe05a6
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("RSM7MFOnsYFPj5g8k-Fwg7qkN2D_uAvrp9dwj332104izIc_aCol1Ic44fwFVGV-0CD3cAiWrSjikZZpMn4jVg")} }

=== Find by ID ===
Found: Record { fields: {"name": Object({"value": String("Test Record"), "type": String("String")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"value": String("2026-01-06T16:09:10.959554Z"), "type": String("DateTime")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("RSM7MFOnsYFPj5g8k-Fwg7qkN2D_uAvrp9dwj332104izIc_aCol1Ic44fwFVGV-0CD3cAiWrSjikZZpMn4jVg"), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"type": String("Integer"), "value": Integer(42)})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-06T16:09:10.959554Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "name": Object({"value": String("Test Record"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "id": String("RSM7MFOnsYFPj5g8k-Fwg7qkN2D_uAvrp9dwj332104izIc_aCol1Ic44fwFVGV-0CD3cAiWrSjikZZpMn4jVg"), "created_at": Object({"value": String("2026-01-06T16:09:10.959554Z"), "type": String("DateTime")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")})} }]

=== Update Document ===
Updated: Record { fields: {"value": Object({"value": Integer(100), "type": String("Integer")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "created_at": Object({"value": String("2026-01-06T16:09:10.959554Z"), "type": String("DateTime")}), "id": String("RSM7MFOnsYFPj5g8k-Fwg7qkN2D_uAvrp9dwj332104izIc_aCol1Ic44fwFVGV-0CD3cAiWrSjikZZpMn4jVg"), "name": Object({"type": String("String"), "value": String("Updated Record")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: FrKlFLGuiqyUbBA2lBokNIbSwrP4ltJ1ETEf_20VX-vr_xurFTaUG7Jcwqgl8pJ5iv9JN6mtlWkvjA9WxsPXOA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "iMaEVtBGhRPMTKEzsfvW0-W6cZzW1jM1twRhoQ1bIc9C6qparvPe_ZUU9N_H5kDxVAk6uudODT6vdAvPeXCUsg"

=== List Collections ===
Total collections: 9
Sample collections: ["batch_users", "ws_ttl_test", "client_collection_management_rust", "scripts__ek0_testing", "websocket_test"]

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
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hNwq3UYI8BKnc113l3aMqh9HoiRE5RvKFzPYYexwDwKWmKoMoaLvuZWBsmCsuxiYKukhC_y4z8x7NyIsvM4zzg
Created Bob: $500 - ID: R8Q3dpTMVm7Oe4Bzf8JAklF_H7G41boQ9KXaXQNxrv8i4NdGfxyqf3tEX3jhFn-sQ95Y5MhEkl47g-4pkMaVcQ

=== Example 1: Begin Transaction ===
Transaction ID: 5a87e0b1-7f31-4569-925c-1ff9c5e48fee

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a4bd3e08-679a-407d-b5f6-0483a2c67a77
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
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
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(String("Rust Programming"))
  2. Score: 6.6000 - Some(String("Python for Data Science"))
  3. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(String("Rust Programming"))
  2. Score: 0.5500 - Some(String("Python for Data Science"))
  3. Score: 0.5500 - Some(String("JavaScript Web Development"))

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(String("Machine Learning Basics"))
     Matched: ["title.value", "description.value", "description", "title"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description.value", "description"]
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
  2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("OS5OQsfpFrwiORTwY1fgmzKBA1t0yvQg1jhjGZ_N6uGc5xbZdOgxYc-x3FGOg0AdVLzJBkMwntamLaUnFa_bWw"))
✓ Inserted user 2: Some(String("B_oP7N2G3XELeDkXGe6-ZG_xpSy5IqQQ4eCVs0ks-9x8xEEBrbZGXp-0teK18fuMscX2nJrLggDT8Ms387s6Jg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - age: Integer
  - email: String
    (required)
  - status: String

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["batch_users", "ws_ttl_test", "scripts__ek0_testing", "schema_client_rust", "websocket_test"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Alice Johnson: Engineering
  - Bob Smith: Sales

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "T74UrVW-PdknJH_JHGgZtCLyOlIUS22uAEwVtcMAXnML6vtFaCozOjRB7LmRbU7mx5tOabwOuxtnrKz0OUBvSQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("IZLa0YQ589qBITB6vP7SFySzrr04F9cEhphhGsyJI4JCxclgg4Y10-PHIGp2M4vw2eqY3CKXepfcSCCqg7EhVw"))

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
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("KMcLd9MEZ81or386qkCoUWbEoHLWCqeLUqsrBf4vWoXjzCx_bKvBY5H-ts2L9nxyqmlfsbTX01s6jvT3XEIvZw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: LacSLNQc4U04jT4mcf9L2HiWG7hmm46UWPBJ4HsR-76TDyMsbOMy_MNCMT2B_FMEqzWmk8QELnjyQEdSn7JnfA

Call 1: Cache lookup
Response time: 1ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 1ms
Found 1 cached entries

🧹 Cleaning up...
✓ Cleanup complete

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: ZvEhKtR5uKrgtqDKaevkeOHIIHIOTG31eky4pYQFRWyLgR5MOlyd5636RP7GAIGQUfjJsq-Ok5p57NdGrCPdhw
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 35F3LACMafqmmLF2qimlp6jTH4NrviNmbBQMJId_WHXnFvXHa1hAyq1DusxUIxXYvNBnKmxTRlRBTxYvb1GQxQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: NtU_TCzJuZLZADYsz83uf_UbjZ2fjvO0OUhtOAm8RxoijHFCGPE006qKatVcpxHsgzL0ww3CVLarCzrWzMZc9g
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
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
   Name: Some(Object({"type": String("String"), "value": String("User 1")}))
   Department: Some(Object({"value": String("engineering"), "type": String("String")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 154.380125ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 76.7365ms
   📊 Records: 1
   🚀 Cache speedup: 2.0x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: MPGSd5eRKL1E6hsGzWqCcdgLQZPJdHyTP-4_d8A6fK-g5odYQxJ2dL9Rq2KtQNT8psOfnW5WC9FUMHiahORQ8A
📊 Found 2 product groups
   Record { fields: {"count": Integer(2), "category": String("Furniture"), "avg_price": Float(474.0)} }
   Record { fields: {"category": String("Electronics"), "avg_price": Float(575.6666666666666), "count": Integer(3)} }
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record { fields: {"count": Integer(2), "category": String("Furniture")} }
   Record { fields: {"category": String("Electronics"), "count": Integer(3)} }
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("jiCNBPSYAJSYDAYvZaVGXWZedohnte6xSzOuZ7qR_eWrEKtCvfOasbA3-fQMKAaLkhYx-y62lQu_Nu67fDkF5Q"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: o_ZYe4Qqk3Q79RGT_hgkH7VuTyT1Gqws35qz8jiZfBSp8cDrEBA1k1zOmoh1nRISQic3TKbnSaaTXKxNACIEIQ
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 4P2s6crh6xRhxv8seczaNsuvaTVqeZ0HIU1pDsjidEBGZDgwblkI75HSSOhlL1zSAKqFQ4MObCqePMoEVEP2dg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: jUI3_-I8uLnYidUTOkVMdom6dk6AJqgvDvrpFpC6u3ZKv2kHdG6rTP0bpQWTgJiLebI0BZG0pOpDjDUTuECfCA
📊 Processed order with caching
⏱️  Stages executed: 1
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use FieldType variants for type-safe wrapped values
   ✅ FieldType::Decimal preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ KV operations work within scripts
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_rs (uHwv432NTnGaQ_tza3Cb3z6tcCALVW1oetN8wZweFT74puUhwaTcrGnRCnh1BCBkIzuYkgKIONE5Tzth6I3kog)

Step 2: First call - Cache miss, fetches from GitHub API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 77ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_advanced`
🚀 ekoDB Rust Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   Record { fields: {"category": String("Furniture"), "count": Integer(3), "avg_price": Float(365.6666666666667)} }
   Record { fields: {"avg_price": Float(367.0), "category": String("Electronics"), "count": Integer(5)} }
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases have several benefits: they provide precise and accurate representation of geographic data, allow complex spatial analysis, support topological data models, and enable easy editing and updating of data. They also allow for efficient storage by eliminating redundancy.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_crud`
🚀 ekoDB Rust CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   Record { fields: {"count": Integer(7), "status": String("active")} }
   Record { fields: {"status": String("inactive"), "count": Integer(3)} }
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. ? (?)
   2. ? (?)
   3. ? (?)
   4. ? (?)
   5. ? (?)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record { fields: {"count": Integer(3), "category": String("Database")} }
   Record { fields: {"count": Integer(2), "category": String("AI")} }
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: FbI9y0Ydck_SMHJ_B-YqCwwxEsLnxwrSKcGyg2Kr74MXEWlYYCdWZ_1x1xSwLF5OunKuwVUWMmSzsLvZo_3JOw

=== Sending Chat Message ===
Message ID: Hcw_zCzm4WrokEl_9W7xpDPPv2Yr5Pfn_znKRmenxdKL_PG-ZO9eN-keY28e6O5GGe7VyHUmaQFMcIwMSb77jQ

=== AI Response ===
Response 1: ekoDB is a high-performance database with features like intelligent caching and real-time capabilities. It also has AI integration which makes it quite advanced. One of the key features of ekoDB is its AI Chat Integration, which allows you to query your database using natural language. The responses you get are AI-powered and provide relevant context. In addition to this, ekoDB supports various search features including full-text search, vector search, and hybrid search, all with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("0Wu-_H7ak08tCAXWXLqeJEtJ3lGgn_XpbrRkhFzoOZp7bPhAdIjS55-q2MlcZkDw4K77Q7CmzrOGA5eor9kNrw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("v4_lAwDgYWWZaB4lgkDm2DzYfWrLkVom0Ts7dXdV8boVw7F15yraWPGr-pUCGyl9F0tL1hasyWK53Ty4iRjh7Q"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("8d0wp2JDHzqa6vIyl-yKV8E-y7xmHwx-_q7fQbllunRgU8klCikbRuQZIhFcdTGbjD7RixFvRFYWnvONNRvo3w"), "title": String("Search Features")}

Execution Time: 3912ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Fwa8M6t6PUt0xJDekPzlhIms4g-V-wDI7KkG0d49_uLylWh5IVKx2n3NYrmATCbJd6SYAMRib99bH4M3IJ7HYA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the product available is called "ekoDB". It is a high-performance database product that is priced at $99.

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
✓ Created second session: q06liewa9ga2qYakyZ96N2GGsnUAwOqtrCRMLPHiWGhF1EO3wrHr_bbAT4z9mta5NnhSvf-MPBdyCFSYGNIzqw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: jSz7-Rn7Gm2dKTCBQP-Na7ElcHYIhA-KdHJAC4xfo_HHUe9692grvVeO8tsjgtikoNoDpkcoo-Y6Pr3IWt5sxQ
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is "ekoDB". It is a high-performance database product with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product with AI capabilities, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: NZojVSTgYPV1Y0c940SoFSVgNl0f55Nzy5uDcFUiDT4THolrOMl5-zhnRqWsPsIpy3YvWJiUHHFo1U8FFqNCNg
  Parent: jSz7-Rn7Gm2dKTCBQP-Na7ElcHYIhA-KdHJAC4xfo_HHUe9692grvVeO8tsjgtikoNoDpkcoo-Y6Pr3IWt5sxQ

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: NZojVSTgYPV1Y0c940SoFSVgNl0f55Nzy5uDcFUiDT4THolrOMl5-zhnRqWsPsIpy3YvWJiUHHFo1U8FFqNCNg (Untitled)
  Session 2: jSz7-Rn7Gm2dKTCBQP-Na7ElcHYIhA-KdHJAC4xfo_HHUe9692grvVeO8tsjgtikoNoDpkcoo-Y6Pr3IWt5sxQ (Untitled)
  Session 3: q06liewa9ga2qYakyZ96N2GGsnUAwOqtrCRMLPHiWGhF1EO3wrHr_bbAT4z9mta5NnhSvf-MPBdyCFSYGNIzqw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: NZojVSTgYPV1Y0c940SoFSVgNl0f55Nzy5uDcFUiDT4THolrOMl5-zhnRqWsPsIpy3YvWJiUHHFo1U8FFqNCNg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record { fields: {"id": String("NPy99GTZcuEfzjmM1nhASffEGMZ9uLs-0EQsVhP4dXuKaiLIh6v0orrVPJ4_2dnlZpACz12WuEkYgY3PeSgk-w")} }

=== Upsert Operation ===
✓ First upsert (insert): Record { fields: {"name": Object({"value": String("Bob Smith"), "type": String("String")}), "age": Object({"type": String("Integer"), "value": Integer(35)}), "email": Object({"value": String("bob@example.com"), "type": String("String")}), "id": String("NPy99GTZcuEfzjmM1nhASffEGMZ9uLs-0EQsVhP4dXuKaiLIh6v0orrVPJ4_2dnlZpACz12WuEkYgY3PeSgk-w"), "active": Object({"value": Boolean(true), "type": String("Boolean")})} }
✓ Second upsert (update): Record { fields: {"name": Object({"type": String("String"), "value": String("Bob Smith")}), "id": String("NPy99GTZcuEfzjmM1nhASffEGMZ9uLs-0EQsVhP4dXuKaiLIh6v0orrVPJ4_2dnlZpACz12WuEkYgY3PeSgk-w"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")}), "age": Object({"value": Integer(36), "type": String("Integer")})} }

=== Find One Operation ===
✗ User not found
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 6 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record { fields: {"id": String("MLMXdv6nv6cWltV0vYqYYVpWD32gOHVv2auPFhhC2ZC_h2DAKze4gOpGbM7kZU4Xoy6ShKG1bn3d0pvGQeHkFw")} }

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record { fields: {"id": String("dGCqTXEDYd1AXtOyNAYS6gQ98q6lbY6ofdSEKGkjv_1vGVCMlRJf4S20Evsq0-MGJWUkJu35h8DZ48jKsTACtg")} }

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record { fields: {"id": String("MLMXdv6nv6cWltV0vYqYYVpWD32gOHVv2auPFhhC2ZC_h2DAKze4gOpGbM7kZU4Xoy6ShKG1bn3d0pvGQeHkFw"), "name": Object({"value": String("Product 1"), "type": String("String")}), "price": Object({"value": Integer(150), "type": String("Integer")})} }

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record { fields: {"id": String("hUm11Yp3ax6TLacnCHFn-V0_z8vsZ-IoQ8lIip1I1EzLdEY3dB8S-c0wC6fruJkW9JsLuUQ1HSeAVNjWBNsEww")} }

✅ All bypass_ripple operations completed successfully!
✅ [32mRust client examples complete![0m
