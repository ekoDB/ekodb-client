make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'vvctkCIpd-JQPPt9qcCfzmfk9Y5-8GxwJJADvEMJBOUUdHjee_tJbjKVcqbWJEE9BQIIObVv-c8ka-z2xo1FNQ'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'vvctkCIpd-JQPPt9qcCfzmfk9Y5-8GxwJJADvEMJBOUUdHjee_tJbjKVcqbWJEE9BQIIObVv-c8ka-z2xo1FNQ'}

=== Find with Query ===
Found documents: [{'id': 'vvctkCIpd-JQPPt9qcCfzmfk9Y5-8GxwJJADvEMJBOUUdHjee_tJbjKVcqbWJEE9BQIIObVv-c8ka-z2xo1FNQ', 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'vvctkCIpd-JQPPt9qcCfzmfk9Y5-8GxwJJADvEMJBOUUdHjee_tJbjKVcqbWJEE9BQIIObVv-c8ka-z2xo1FNQ'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: NijRrca-9YBQBeXU6j280I4rPcUwOpFnrXCbwAWzKK-LMaIattQNOpF3zXfRLzEbyzyKyf4XMcJ6kC1iBfqmHw

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "NijRrca-9YBQBeXU6j280I4rPcUwOpFnrXCbwAWzKK-LMaIattQNOpF3zXfRLzEbyzyKyf4XMcJ6kC1iBfqmHw",
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
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: UxEiwmXd_VYy-fIfeY893eVU0LxvqNzVnhSsPM547mFfRwF-h_ig4uHx0sEoEPco5hj9OZYhPZIr7iiPmkrA3Q
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: pb6_jWm4IZ88lb5by0W0jhMai9zs1eWWSwQ_NJLcSKqtIxax8zM3Yw6lOAy9wsPpi1WE4OaDZ3jC9r0ML5AXUA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 2Kk0HiiN9MWf4EmSh7DHWUsuGYg4P19JgwlfA4cLmS7rBVBAsjCjxP-KENEbNFN7nWjOEuDZy6uFRIm6x_AycQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 60.0, 'status': 'active', 'count': 15, 'max_score': 100}
   {'count': 15, 'avg_score': 50.0, 'status': 'inactive', 'max_score': 90}

📝 Example 4: Function Management

📋 Total scripts: 15
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: Pi08dMDlitOqNEz8K_6XIVRDwB7u5fiy0yl4mWTfN2MoMqySj5RUfxyA3Hibp3DbOVL47moBPHM2ieKfx6m5Sg

=== List Collections ===
Total collections: 13
Sample collections: ['batch_users', 'ws_ttl_test', 'products', 'scripts__ek0_testing', 'github_cache']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: False


============================================================
🔥 TEST 1: Original Config (durable=False)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gxmtaUSKEAHKMJn8VaaKA5_SpagLajigkcRWJUy29aJXpWaYc9fxcWGD-KsEJX7xwvQ2VOLPBG-UvgbICtGzPQ
Created Bob: $500 - ID: l1qqBPLN4p5JqqYyaCpE6UJpFNlU0gxzM07Y4y2FwmBmbi_zTengJS922Y1D_kMGN69lhHVpItafQ5jrTrgFlA

=== Example 1: Begin Transaction ===
Transaction ID: 36cd4141-fcaf-45cf-b23e-e8ab75037810

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
New transaction: 61e29575-7375-4c0b-9c07-b434d8f5f0b9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: uwwf94q0_nUy1LC3DG2BMhxUYkUOzQFwauspnmrvubE7DdDG7PcjO1rXI0-0D6df-mgL-tgPbjwkNmIWmdQM7g
Created Bob: $500 - ID: y_ouGPOSq2fLgFVaIbu7DFwUfJfe9AF7zR5qDJfn3ntp9MIYphnKhItVtEmKH9SXASdAwmX5L73JH8eOzQfa0g

=== Example 1: Begin Transaction ===
Transaction ID: 8288d17f-06ff-40e4-8acf-cef8f9f089a5

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
Created Alice: $1000 - ID: iGegGZkjsy1UHlbkJJKCI871KhQRXGhmfH4HfKxFmA8_0v5pvXUuka9bUySlSyChChkyiaKHc_G6fV9biMNMBA
Created Bob: $500 - ID: zzZaGGh37Gq8wUOEKxBma8yx_OeLwZdriGC04uqRqU8o561HY-N0xvaxJyobTvYficby9E4EcBMrDAq4VIsquQ

=== Example 1: Begin Transaction ===
Transaction ID: 33e802ed-ba14-4e28-8b70-7bf5ef1be2a9

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
New transaction: 315b8a1e-4ab8-437c-a82f-0f98d1b43ce7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: HU_KI9HL-vN_iPYFf_CmU6h31xyKAqTHnz-yvlV_eC1pWJZxmKAXvgrBAi62bO8tfggOWQLxVeAKJ_B2RhV9UQ
Created Bob: $500 - ID: 6Bn7VtruwYJUrTv-RLxsIIH6G1YwxSNdOFwRPohKTpncC_Ylc-DHqW0tUOPnkazDoQyXD34V0VK6sNd4RS02gQ

=== Example 1: Begin Transaction ===
Transaction ID: dde6d0bf-1dd3-441a-b239-560a99750b53

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
Created Alice: $1000 - ID: ei7XuiyllwmCFGCL7tsLRJcGtvonyfiledr14w5-HBHG1VIjM5HJoOrIZ2Na9EwPYAf1HuCUYKUU10y8RoAcag
Created Bob: $500 - ID: twD5BxMX2IvrssM69hYrHVfwT5WXyVUKRwGDYQSveVLE7HEdHd8cV78hK2hUwHF9GVqri9c30Fomc8QPub7jdg

=== Example 1: Begin Transaction ===
Transaction ID: 317d5be7-e0bd-4240-8f02-9716fecab985

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
New transaction: 7a452e88-1eea-4ea2-b15f-c4c8dba3989e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: dK18BtxlEd2mq3vCA4y2GAr2oP6Rd63CVQJbljzJuCl6wt74W4xqWorVsFRCnB7YHtjYDy8om4lBNcJBDvlhBw
Created Bob: $500 - ID: NOq-ke2SLSjYRcMLJCSuzHr_2ms2pQEhNjwJG6rpcLsaAtgoirQNyAQnl9gY1J5dSOf6zSkjSwfQ5HxWMedm5g

=== Example 1: Begin Transaction ===
Transaction ID: 074266b7-b900-47e1-ae96-1c1571deb557

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
   ✅ Script saved: 7wUzRP5EFAO4qbzUrjWKqpgju45S3EN_D0bgzlb5gphYqzPmn3JdkS3FdT3hL9Y2nQD-AhRmpLJcpkrn2Rjx_g

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: DiS29mQgPVpccZlFYlq5zbDIHLmnO_eh_DtRmSXNSkidPAL3ZQUw1VImmZHGtM9H4UEmaavvn_7DUpdysLooGQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Cr2cflX01rrt4EZFK_EuF0ukwO8adg9pfR9n6gWvrFjT47xenlMUWLFrsk7A2MEdaN6qf_xYxeZkGvq1InMaaw

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
   ✅ Script saved: s_AwOdEyEL1dxRanL2Z6uncVWg0nuEkvIBL6H-ioH8gARynkJGr8t5k-WLgXI0T5ucB0A3lZv-iLS0ml3akfEw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 2WgctZ9wF1_h8Luj7tOhgvVE9WrnjpmjSk2H4bEQi6S88XDECz9zOwYfB5FgJ6zQGA40NXAAfKQyijWZSzeCdA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 7wUzRP5EFAO4qbzUrjWK...
   ✅ Deleted script: Cr2cflX01rrt4EZFK_Eu...
   ✅ Deleted script: s_AwOdEyEL1dxRanL2Z6...
   ✅ Deleted script: 2WgctZ9wF1_h8Luj7tOh...
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
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = pOAJg1ng7NqI1O88_oOFrDwcTRCXkOyC-kX3vNmy_A3t1uchguSHb4C_mnlsjHBNo9HMkzaar3nPiMhOgNurFg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(pOAJg1ng7NqI1O88_oOFrDwcTRCXkOyC-kX3vNmy_A3t1uchguSHb4C_mnlsjHBNo9HMkzaar3nPiMhOgNurFg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(pOAJg1ng7NqI1O88_oOFrDwcTRCXkOyC-kX3vNmy_A3t1uchguSHb4C_mnlsjHBNo9HMkzaar3nPiMhOgNurFg)
  Output: Error (expected) - Find failed: Record not found
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = x-XDiqhar8f0jaJSRDJGuz29_xFMs-ZdFtrFOABCzrKYKSAGCj0NyM4Nj3b8lAU3v2v7k0y16LwjrUCvM_LkUg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(x-XDiqhar8f0jaJSRDJGuz29_xFMs-ZdFtrFOABCzrKYKSAGCj0NyM4Nj3b8lAU3v2v7k0y16LwjrUCvM_LkUg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(x-XDiqhar8f0jaJSRDJGuz29_xFMs-ZdFtrFOABCzrKYKSAGCj0NyM4Nj3b8lAU3v2v7k0y16LwjrUCvM_LkUg)
  Output: Error (expected) - Find failed: Record not found
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return None for expired documents
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
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
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
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_scripts.py ===[0m
[32m✓ crud_scripts.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
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
Inserted: {'id': 'nDJQBL-DCMhpYvoxnDQz4E4mNoa6uGiTYqfp0AtbwFyqkLxMRu7_RlBDCVdDhv39X4AFlNLG1m5NHRXmjUEY5w'}

=== Find by ID ===
Found: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'id': 'nDJQBL-DCMhpYvoxnDQz4E4mNoa6uGiTYqfp0AtbwFyqkLxMRu7_RlBDCVdDhv39X4AFlNLG1m5NHRXmjUEY5w', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'value': '2026-01-06T11:10:31.001245', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-06 11:10:31.001245
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': 'nDJQBL-DCMhpYvoxnDQz4E4mNoa6uGiTYqfp0AtbwFyqkLxMRu7_RlBDCVdDhv39X4AFlNLG1m5NHRXmjUEY5w', 'categories': ['electronics', 'computers'], 'active': True, 'created_at': '2026-01-06T11:10:31.001245', 'value': 42, 'price': 99.99, 'tags': ['tag1', 'tag2', 'tag3'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'data': 'aGVsbG8gd29ybGQ=', 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'nDJQBL-DCMhpYvoxnDQz4E4mNoa6uGiTYqfp0AtbwFyqkLxMRu7_RlBDCVdDhv39X4AFlNLG1m5NHRXmjUEY5w', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'type': 'String', 'value': '2026-01-06T11:10:31.001245'}, 'price': {'value': 99.99, 'type': 'Float'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: J-af5-hidEzKWmfZHMHNq5D2azRl6aROuiiNMMH7FobjNJuQsAAiKdrv2hLvij97rzzaFJeOTFI2SBMxKcw4vw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: "M3H281WCEcaUcfNcX3WOwd8O46BhSpyLJj7bUFjxLHYAXK0zi6tJkEgDQn74-UpGh-C4zYdidVhdikBy-8YdFg"

=== List Collections ===
Total collections: 12
Sample collections: ['batch_users', 'ws_ttl_test', 'client_collection_management_python', 'products', 'scripts__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
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

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Qa543jqYtUcPrKgY6aetfcYLoITy9opHiSSeDkVh1ahOR5L-oDuoR2P_EiVC6jBZbuhy6R9y2ORoSrKpxgLB3g
Created Bob: $500 - ID: cgethdgQFuRI5UYcLYozJJ053gCfSkH6-kX8yveBlcoHfu6AHv6J0DneunA1olUA_leyx9iQj5hwnrgQciVVfA

=== Example 1: Begin Transaction ===
Transaction ID: fdb2be7a-9e3c-4208-a8d2-e057152cbeb7

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9d689106-622f-4d5e-bc31-0d439aa96ea7
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: bio, title, bio.value, title.value
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, title, bio
  2. Score: 39.600, Matched: title.value, bio.value, title, bio
  3. Score: 39.600, Matched: bio.value, title.value, title, bio
  4. Score: 39.600, Matched: title.value, title, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770
  2. Score: 0.760
  3. Score: 0.758

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.380, Matched: title, content.value, content, title.value
  2. Score: 1.379, Matched: content, content.value, title, title.value
  3. Score: 0.385, Matched: 

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 8k4nGTsuebOPMHd_hihFI8Bwcevwo7TugwgkfwGGqQLxNWSExui7bczQFLXOMU8zIRKX9xCUh0_irSweepLHOQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: DrDNtvaEZjAXS-mNd9blGJ4jOWLjuYpb06xXoOi5FfpZm6_oiLe5l4X9Au3-Ei3p24XQ1dOTN0MJbl4YzPc3cA

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

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: UC0GbSSdHensWgotZCmzTW9BDYoAw6x-9zbdH-qETOIP3IJiZCm_FIMK4M3dq8mBR6JfJZjqm-GkrFnpUIjz8A

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: KVIDZ59JkiFDLFPil9MP3omAY5VtZZGQl0d01HdMo20OPQ0Q8IN4Ls14Wf2kQfugA9DnQCEIqkvjEXMb3ADuow

Call 1: Cache miss (fetches from API)
Response time: 573ms
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T11:10:32.406726",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 5.1,
            "time": "2026-01-06T16:00"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.026106834411621097,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      },
      "id": "WiHktZ0s7L6qMPppBt4iNCvUR_e07RU72wbX82uPtlXOCBLq1XNgsNceBMw9dSv3HRAAmRPgHk1tFJCaKZmNeA"
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 539ms (1.1x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T11:10:32.406726",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 5.1,
            "time": "2026-01-06T16:00"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.01728534698486328,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      },
      "id": "bB9hNj3KGYOrnKAW0oS7nK9DhrrW-3YFAi5ymvRIj6aOh0peFibNI0kDjhRK7Xh_u6xw2_nke4IO8Ue5y6sLyw"
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
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

🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: KB3HSBfoL8VM3BxFKsWKyMwq3aJjaTa3aMhMqiA4Cmfelvcqk_-JGC4AgKHwfLfhA227iJKU04ASSDGs7dyZIg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: MtRX8iuWpw8IqBLBlHVHIfQI48T1i2enrHdfGCjeVI06jbutoBX1kjnotk36arPYnvB9Ouaa93wtJ786ju0Ikw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: H6sNBrNrnTh8zCenOnqmOjC73UKQMUp659-4bcv8my-DNnPhej34zGcy5uBFrlc6ComMRSUa2zjyQnxqifZW7A
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

📝 Example 4: Script Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
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
   ⏱️  Duration: 135.5ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 81.0ms
   📊 Records: 1
   🚀 Cache speedup: 1.7x faster!

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
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: ULs8m6JRUJAo76Zyzi8TdlVKkz5D1gVfk9T_m8hJIlfKBaljIcOySiapmM2-cgaWZ49Bv7Y0fT4wVUGsxtaUlg
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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: 6IiKKt1GUcAFozYCEqWHWvUi4FwIz-q5MY6CY1TucNrJLBwdYIcxkI_o81-1PdMzZ0B06J0WsBTuoI1qsMkzaA
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Wireless Mouse
   • Laptop Pro

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"role":"admin","userId":"user_abc"}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session: None

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: KCW8I6nQIp0VsR4V4HdKL8q24gfGx7T8Fc-5lmoEcOg6sckDswtYidUJoyBTw24k4xo625mJ2vXSjSsnYYByeQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-06T16:10:34.247979+00:00"}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_py (B2odpP768id-fn1uQ6q342HE0OrboNc1HOrwJ_YqguoEqF0b3uHZMvIwdh4hB3gOOCns7eIXM8O4N7StGj7riw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T11:10:34.415692",
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
          "followers": 270299,
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
      "id": "8kYielVMNhuowHFsDrNPQMIli5gl4sRTwxE4i3KYhctinSKGL4nh1zom2mVVuk5o4dLbCQgY6032BZs1NOyEpw"
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 84ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Python Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Script saved
📊 Total products: [{'total': 8}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases have several benefits including:

1. Precision: They provide more accurate and detailed data representation.
2. Scalability: The quality of vector data is not affected by scaling or zooming.
3. Flexibility: It's easier to modify and manipulate data.
4. Less Space: They typically require less storage space compared to raster databases.
5. Topology: Vector databases have the ability to maintain topological information like adjacency, connectivity and relationships between features.
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
🚀 ekoDB Python CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'Database', 'count': 3}
   {'category': 'AI', 'count': 2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: ld9rVKp4aH4JOkUnFVYGKj4j8r2wHCUzMhFDp6pPQ_uKiRHIPwF3xlBjj_jGZnNHXzBvJCFoL7eFsx1FGxpNYw

=== Sending Chat Message ===
Message ID: uZgJ0NvEWH-hJHdl3KNIB70EMLykYRt2nhr3gwHGLlLliH0wX_20Qqlp5jXzz0Q11MHTm7FJkb6pncDeQPsbgg

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

Execution Time: 4377ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 84
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1gzm5zfXlhBQ621uv3rfxJkKBEv77lqME2WaMW3ZVk8B-zc5MMMhFLfMeiarWBIANE1U9qucF-nET8F9eZxKCg

=== Sending Initial Message ===
✓ Message sent
  Response: Currently, we have the "ekoDB" available. It's a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'role', 'updated_at', 'created_at', 'context_snippets', 'token_usage', 'content', 'chat_id'])
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
✓ Created second session: Vwv_dS-QtANBp_pTXuP31-g0Z56KtYJkKqzGlcNeqgBv68RCtjOiUv2gnGQKhO03ZeTUkAZZVX49FONaJsaS-w
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: CmkXUGl8N39PNfVRr_wfp_s9gXhLtpKaLd7K8JE8JEizguq4G_Ih6TWm_fI7mh0etUGyvOC3ZG1uYG-OnM22RQ
  Parent: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: CmkXUGl8N39PNfVRr_wfp_s9gXhLtpKaLd7K8JE8JEizguq4G_Ih6TWm_fI7mh0etUGyvOC3ZG1uYG-OnM22RQ (Untitled)
  Session 2: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw (Untitled)
  Session 3: Vwv_dS-QtANBp_pTXuP31-g0Z56KtYJkKqzGlcNeqgBv68RCtjOiUv2gnGQKhO03ZeTUkAZZVX49FONaJsaS-w (Untitled)
  Session 4: ld9rVKp4aH4JOkUnFVYGKj4j8r2wHCUzMhFDp6pPQ_uKiRHIPwF3xlBjj_jGZnNHXzBvJCFoL7eFsx1FGxpNYw (Untitled)
  Session 5: jSz7-Rn7Gm2dKTCBQP-Na7ElcHYIhA-KdHJAC4xfo_HHUe9692grvVeO8tsjgtikoNoDpkcoo-Y6Pr3IWt5sxQ (Untitled)
  Session 6: q06liewa9ga2qYakyZ96N2GGsnUAwOqtrCRMLPHiWGhF1EO3wrHr_bbAT4z9mta5NnhSvf-MPBdyCFSYGNIzqw (Untitled)
  Session 7: FbI9y0Ydck_SMHJ_B-YqCwwxEsLnxwrSKcGyg2Kr74MXEWlYYCdWZ_1x1xSwLF5OunKuwVUWMmSzsLvZo_3JOw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: CmkXUGl8N39PNfVRr_wfp_s9gXhLtpKaLd7K8JE8JEizguq4G_Ih6TWm_fI7mh0etUGyvOC3ZG1uYG-OnM22RQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'Xws4uumYwVnn9YKf6874w_eAs3owy-V133AuFim4SKoZJeaJCMc6PXv8xmrQ8xXO53WP1Jplj41hzMVnvZKX1w'}

=== Upsert Operation ===
✓ Upsert (update existing record): Xws4uumYwVnn9YKf6874w_eAs3owy-V133AuFim4SKoZJeaJCMc6PXv8xmrQ8xXO53WP1Jplj41hzMVnvZKX1w
✓ Inserted second record: 24oBPzZQiB-LQQuTdTxpXyTvHEwjSBfQU4yb_cMt7aMbGbXE6GFbmUKaTJAP_Fw2uTa5rioSu1gMxb4GGa8qXA
✓ Upsert (update second record): 24oBPzZQiB-LQQuTdTxpXyTvHEwjSBfQU4yb_cMt7aMbGbXE6GFbmUKaTJAP_Fw2uTa5rioSu1gMxb4GGa8qXA

=== Find One Operation ===
✓ Found user by email: {'name': {'type': 'String', 'value': 'Alice Johnson'}, 'active': {'type': 'Boolean', 'value': True}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'id': 'Xws4uumYwVnn9YKf6874w_eAs3owy-V133AuFim4SKoZJeaJCMc6PXv8xmrQ8xXO53WP1Jplj41hzMVnvZKX1w', 'age': {'type': 'Integer', 'value': 29}}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: True
✓ Fake record exists: False (should be False)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'Iq4y-QTSX0nfbyMV9UvBQ9KhZbeaTVi9w8o8LH_bDrLtfBlhTVVOuRM6ZRLZ9FmjS0VnVlVFXal1CHs84JArJw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'WI5ZRbnGyvEuQL-PRrOnXBFTeEvO-jY01CRoZV8yPEPMgpodRYF6Wx0Tz5wRgDaO4Mw4uFzJYpj_2X7xYeJU-g'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'Iq4y-QTSX0nfbyMV9UvBQ9KhZbeaTVi9w8o8LH_bDrLtfBlhTVVOuRM6ZRLZ9FmjS0VnVlVFXal1CHs84JArJw', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'Iq4y-QTSX0nfbyMV9UvBQ9KhZbeaTVi9w8o8LH_bDrLtfBlhTVVOuRM6ZRLZ9FmjS0VnVlVFXal1CHs84JArJw'}

✅ All bypass_ripple operations completed successfully!
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
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: map[id:GtvC4o3Bc5mJnTAMgN9xCC0JrjRr--K1RCJgqrqWPNIAx7pQ3_U_KAOsD4Y73NopJbldHQdHnNluXEktQYgHJg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:GtvC4o3Bc5mJnTAMgN9xCC0JrjRr--K1RCJgqrqWPNIAx7pQ3_U_KAOsD4Y73NopJbldHQdHnNluXEktQYgHJg name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:GtvC4o3Bc5mJnTAMgN9xCC0JrjRr--K1RCJgqrqWPNIAx7pQ3_U_KAOsD4Y73NopJbldHQdHnNluXEktQYgHJg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: wOhw36APY7lqulIUv8F8sgCT5KKowLEVn1StWe5ITSbhiGFihQjS0yKngcE0-d6Pqs0oe1bokPr8hlOsg3kr1w

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "NijRrca-9YBQBeXU6j280I4rPcUwOpFnrXCbwAWzKK-LMaIattQNOpF3zXfRLzEbyzyKyf4XMcJ6kC1iBfqmHw",
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
        "id": "wOhw36APY7lqulIUv8F8sgCT5KKowLEVn1StWe5ITSbhiGFihQjS0yKngcE0-d6Pqs0oe1bokPr8hlOsg3kr1w",
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
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: WfrHxBOLEk6l9FCCGEpqMyCpkDGJbf3Wt7Bgt4jaL3TGXi4NfP6g20mI-RUwewHQXD937EzlMYqjnbCceu61dQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: M1cWUj1uxy8xTE-mGatv1xPNRL9QhHtR1W8pj3nsjFrHB7UxvrmuLo2BSyfzWEpAbTpMCxhvDMFGdTNlX14-3Q
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: vUmiTU4O5NyergoE0FKzK38guyBvff0kqBGmDOuf-AbVRw5UrgXtF6DsJyR5BFNs-JCC9TlG3lH1rQzAE6qcPg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 27
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.go completed successfully[0m
[34m
=== Running batch_operations.go ===[0m
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
=== Key-Value Operations (Direct HTTP) ===

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
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: -M1C0L_gKDgCKxu7SYtfOrTTEh2rK1z1lw36SJSe9K9-U6tldqm30O5rUztaHun75V-k0kBFDv3qC67IteUd2A

=== List Collections ===
Total collections: 17
Sample collections: [schema_products_client_py batch_users ws_ttl_test scripts__ek0_testing github_cache]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: YfnT1BKDk6F0Piu0TmhZEklRyO6JZGeb7HK7EEviqSgbPU0QhkGqV6JbHi1WwqQ83SuwmZQZ3oBu4VnExqMRNw
Created Bob: $500 - ID: S-SGjflhx8vINIG0ELSTVtvtYbWMBJzZgBPqyMp8t9XCka236mPcG-MgJLn_dOYFsgddFSbrZuXUDrTb28TGEg

=== Example 1: Begin Transaction ===
Transaction ID: c063f661-2246-44c2-8bbd-f55d438137a5

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
New transaction: 10cdd397-eebc-49b5-9ca4-988365ee9324
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
   ✅ Script saved: DXACOIbS7F5bJcv527cxaRp4sFNJOLncanEwYWihpczEs8hv4Xcdcx_UsN4SXoe5EShloFgde0umCQO9e8nb1Q

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: a_SLe39ycIQtKONzpi31IbaXudH8ENMjySEAVe-QmTvj6UnIdxq_uH0GiJ6Kxx9a8l_M9c3PHnIbwuS2XmjAXg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: KU3pQjjM3CRfUr2GrNa83S2O2nsEGJwhiAW3I9pu0QS2zAXGd4fGy12zX7LpPPPCXRPPiVr-m2zWKuzYISM9pQ

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
   ✅ Script saved: nXVwp-xC9X5hrBkJk5YEJg1bqD3KeOUqXn6XDiukN94_5t8hK6T-XDuLEapRNWtVAXRa8bpeRBRdWf4UDbGxog

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
   ✅ Script saved: FmN3GhptZAeBewb4gJHLZxhh6WVr7lHqN2QVsnuwJS8FqeunVTCce8LKeQ1KMHR6RlKGDNONBKKY7-FvOXgRyA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: DXACOIbS7F5bJcv527cx...
   ✅ Deleted script: KU3pQjjM3CRfUr2GrNa8...
   ✅ Deleted script: nXVwp-xC9X5hrBkJk5YE...
   ✅ Deleted script: FmN3GhptZAeBewb4gJHL...
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
=== Running document_ttl.go ===[0m
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

2026/01/06 11:11:09 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = j0vcTiKIMl145jHOvy9Zh_CzTvgKOjgy0PqtcVEr9_qtlcmKEJvnrX_mlaC7Bgz7qIO_wSq_TrMw2YJjr8AiFg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(j0vcTiKIMl145jHOvy9Zh_CzTvgKOjgy0PqtcVEr9_qtlcmKEJvnrX_mlaC7Bgz7qIO_wSq_TrMw2YJjr8AiFg)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(j0vcTiKIMl145jHOvy9Zh_CzTvgKOjgy0PqtcVEr9_qtlcmKEJvnrX_mlaC7Bgz7qIO_wSq_TrMw2YJjr8AiFg)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

2026/01/06 11:11:16 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = 0b9zite4PD1qms7TM_TQkxzN796qCKFJOKStOwrP-WFbmjC8ui8Mq2J3Vu-mhv5XYgKMtYWiIF5iHvtLHD2vug
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(0b9zite4PD1qms7TM_TQkxzN796qCKFJOKStOwrP-WFbmjC8ui8Mq2J3Vu-mhv5XYgKMtYWiIF5iHvtLHD2vug)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(0b9zite4PD1qms7TM_TQkxzN796qCKFJOKStOwrP-WFbmjC8ui8Mq2J3Vu-mhv5XYgKMtYWiIF5iHvtLHD2vug)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return nil for expired documents
[32m✓ websocket_ttl.go completed successfully[0m
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
Inserted: map[id:oVViHmI8H8ncZ2xskStkzLGfqiZ2-j0sU_YsNmbXVrqEW_NAjTgpcorVC21E9wQ0uPShFIk2D0Fk4EVOtf5AQg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-06T16:11:23Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:oVViHmI8H8ncZ2xskStkzLGfqiZ2-j0sU_YsNmbXVrqEW_NAjTgpcorVC21E9wQ0uPShFIk2D0Fk4EVOtf5AQg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-06 16:11:23 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-06T16:11:23Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:oVViHmI8H8ncZ2xskStkzLGfqiZ2-j0sU_YsNmbXVrqEW_NAjTgpcorVC21E9wQ0uPShFIk2D0Fk4EVOtf5AQg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-06T16:11:23Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:oVViHmI8H8ncZ2xskStkzLGfqiZ2-j0sU_YsNmbXVrqEW_NAjTgpcorVC21E9wQ0uPShFIk2D0Fk4EVOtf5AQg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: eeuT0NsnhfNX0O6W3_bH-25pa_Ea8Umx6qdYBNqJUnSznkaykfUyFlEO1nA7t_13z5nidJJgqNgtQYnxDO7Mbg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: af2BT3SbLVWqtk1U5TSB_BkytuJHO0MjWeO7Dv4SpG1-B7jpl_e_d_oj-gIst6vlsLrcOKtmxBTynymOx1HZqA

=== List Collections ===
Total collections: 16
Sample collections: [schema_products_client_py batch_users ws_ttl_test scripts__ek0_testing github_cache]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
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

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: liYne5FCqips4jUB1rVW2IUPOOs7rQ48NatyEhTQxvNTZUji8O6DQNQGd23txgKMvX7gW3QVKV_MwEo-nGbE1g
Created Bob: $500 - ID: 0giMybQdb9faSGNV-z1eU1hr-1imxdDLCJbcSrUJIYTyLzJdzkv58Zbh-FBnj6kFunmG8wcjPYDK7UJ-1Ea2sw

=== Example 1: Begin Transaction ===
Transaction ID: 76c44efb-9996-48f1-bd42-a0b9618683a1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: bdfbbd94-6a2b-46c8-a4d8-bfb3eccf78d6
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
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
  1. Score: 0.777
  2. Score: 0.762
  3. Score: 0.730

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.692
  2. Score: 1.511
  3. Score: 0.305

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
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: oBeM1aNMsiHxrJLdwxGTKTBKEAKsjt-ZYlFmxN-ayUCInB8_Ib8rqf7J8CKoUH4EP4wDcSq-TgHeM4LPttrKXg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: fzaLSga9Wsl1gqKNgMBABjl2GV0DQOXQc8KyaPiS0AeM764MWvPkzOEA4yaeWXNLeABiNEeI1KdNFRoqRaIfAA

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

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: rQaWiGqxdaEIYLY9521ihhBDBuxGcZ9kDXsvLZtWkeBBuxyJBA8LPAwGJZBNRvfS5uPo7Z5q-pmceFNzW7Z7VA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: QU_fGh9Wl0NekHYRi1VKJDmIZ1BKAQZnFX_1vWTLmAh8owDyZ7y3sI0-u5_KIZWDzVCVLz96g77bXqCozUTG_Q

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 1cLf-HMj8QqaESy4YplqtUAOo9ibGH_sED_52K2YtyZPNdNAdSAsIW2uskB9wUvEgKz22TYgXCAFr-6fWKhWkw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:60 count:5 status:active]
   map[avg_score:50 count:5 status:inactive]
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 29
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
   ⏱️  Duration: 127.805417ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 84.312084ms
   📊 Records: 1
   🚀 Cache speedup: 1.5x faster!

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
🚀 ekoDB Go Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 1lsr8xJttUkCkSlifxNwQCg64iWmZqT4GWbvULM9Aw654BqLHbCfObDeIHrPTICgS0nA-AzBSF7hAqJA9LdJVw
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Script management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Am1V7jZt0Rj2GSGQfU_e7AQ5z5oOSez2MY3JnA_EmZ8apmATJSg-PzCgRqUl1445AWD2U9PNo-RTsIXUlwij-w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: BvFoFdMUeY-alGxirGL1lVx2hXTBPn1BDoQRfobjjULiMOPncyATb7xzTXfOxKT7IIp-67MpuWTKfdWQHguGoQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: 5WmFdvBTlYH_slC66OUQFn4A1lYZLicFyVmYZksVffjB9BaF7TC8l9-XHc_kR5pvnnzmN5ZSeJAC9tCfSjTUgg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: MLepfCjB1LX_YaMpFKQD-jPHwiUoyTlGJG7q9MvdxTYoqviCGPegER0x_XfH2KZi4cbsC05tltGpC0BHnNlxwg
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field* helpers for type-safe wrapped values
   ✅ FieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ StageKv*() functions work within scripts
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_go (uf1ZzsAV_DvEP4fkFB1uHN2Yt0lMQQEiLw_1l6TyMnqsXeiOBl1FEvRPU6DVMywVRsQ5_P8Ya22qR4_w3OREBQ)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T11:11:26-05:00",
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
          "followers": 270299,
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
      "id": "IAMfC6bghidublLByrRDNUi6O1FZtDvyuMvwGnMDcdNd01vbzRWi_Hk8kUd_Yhx7suzLP3j3pj2i-ZrDBRzdIA"
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
Response time: 71ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Go Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   map[avg_price:367 category:Electronics count:5]
   map[avg_price:365.6666666666667 category:Furniture count:3]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
📊 AI Response generated
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Go CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   map[count:7 status:active]
   map[count:3 status:inactive]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   3. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   4. map[type:String value:Database Design Principles] (map[type:String value:Database])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   map[category:AI count:2]
   map[category:Database count:3]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: X9t5cttsehSZsRMNaQJaFNuP86HLbljgFO9jHItG45p3NzgjeeiOKF4tY97d975r8UT0y_xwUsC38ihAeBwYgg

=== Sending Chat Message ===
Message ID: DzS7bCJTXxuAEkfr5a4Wb_TCSHTtsvIxkPKu9O8FHilnx6IJbzN9lkrivr7WY9LNdgKy_fl6Y0aGeJFJ01b3tQ

=== AI Response ===
The available products are:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is the enterprise edition product with advanced features. It costs $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:a5X_bVCevT_5kT-9kc08WXQqzGPQTl3xGb6Qik-ZXHFmU7EiROindGn3EHFP-MzWqUnn_VlNUMYdcTpEbYJNzw name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:cvmwX0VwJ2UpzpFd4wvVrqnNGNHZdBRzhvlSrCZUArJ9w_C-_BkQA4EI9z3IEb3Bt9eDlTMQnFxrm0IMI5dSiw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:1cfsKrkp0eVU8KS8jraNI6YnwDkmpUPHj8GQ9_JF-TDn0P0FzdD-ZGEnFQ6NZyG7vJ6njeKsoowrcnmrc_D7vg name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 3123ms

=== Token Usage ===
Prompt tokens: 455
Completion tokens: 77
Total tokens: 532

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sK6VuJhhK34wgmqXdtlIjrsmMj_aX7Or3SxLW2e3t2WDM5u6mujVgEdEbB52Q5WCCcP-61rjaxlEbK2IKB38og

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is "ekoDB", which is a high-performance database product. It is priced at $99.

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
✓ Created second session: EB3jEuzG-LxkynD65O1rXeJuvg8QIZkvVwPEHVz2k56v6PiRdnSL7oN61GeamNEYGSrhJJXdcEuHNcMobxBfPg
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, one product available is ekoDB. It's a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: GICodxKr0jQ8jniyEi_Rz5fHY22XdSnRqdLCOU0PGdm2XU_7JZ_X3rw9ZcpIQGuukUSd1m2_vAN57IrTnMrmbA
  Parent: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: GICodxKr0jQ8jniyEi_Rz5fHY22XdSnRqdLCOU0PGdm2XU_7JZ_X3rw9ZcpIQGuukUSd1m2_vAN57IrTnMrmbA (Untitled)
  Session 2: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg (Untitled)
  Session 3: EB3jEuzG-LxkynD65O1rXeJuvg8QIZkvVwPEHVz2k56v6PiRdnSL7oN61GeamNEYGSrhJJXdcEuHNcMobxBfPg (Untitled)
  Session 4: X9t5cttsehSZsRMNaQJaFNuP86HLbljgFO9jHItG45p3NzgjeeiOKF4tY97d975r8UT0y_xwUsC38ihAeBwYgg (Untitled)
  Session 5: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw (Untitled)
  Session 6: Vwv_dS-QtANBp_pTXuP31-g0Z56KtYJkKqzGlcNeqgBv68RCtjOiUv2gnGQKhO03ZeTUkAZZVX49FONaJsaS-w (Untitled)
  Session 7: ld9rVKp4aH4JOkUnFVYGKj4j8r2wHCUzMhFDp6pPQ_uKiRHIPwF3xlBjj_jGZnNHXzBvJCFoL7eFsx1FGxpNYw (Untitled)
  Session 8: jSz7-Rn7Gm2dKTCBQP-Na7ElcHYIhA-KdHJAC4xfo_HHUe9692grvVeO8tsjgtikoNoDpkcoo-Y6Pr3IWt5sxQ (Untitled)
  Session 9: q06liewa9ga2qYakyZ96N2GGsnUAwOqtrCRMLPHiWGhF1EO3wrHr_bbAT4z9mta5NnhSvf-MPBdyCFSYGNIzqw (Untitled)
  Session 10: FbI9y0Ydck_SMHJ_B-YqCwwxEsLnxwrSKcGyg2Kr74MXEWlYYCdWZ_1x1xSwLF5OunKuwVUWMmSzsLvZo_3JOw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: GICodxKr0jQ8jniyEi_Rz5fHY22XdSnRqdLCOU0PGdm2XU_7JZ_X3rw9ZcpIQGuukUSd1m2_vAN57IrTnMrmbA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:xeUcSaOVntITF4gxRTEQSjhro_k2YnOKnB2BRTNU80FvghweSGzbxc0hsZ_Hd_YP2-LHdjKeTmjBKURwbENYDQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:xeUcSaOVntITF4gxRTEQSjhro_k2YnOKnB2BRTNU80FvghweSGzbxc0hsZ_Hd_YP2-LHdjKeTmjBKURwbENYDQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:tGQRoAnUZV1PNoeGZapmE3zL5YAqEKaephnZf5UqsCAwjgwsUBeBGqQHNUxKIzTQjWSab5LACg9NSBkqroOqmQ]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:xeUcSaOVntITF4gxRTEQSjhro_k2YnOKnB2BRTNU80FvghweSGzbxc0hsZ_Hd_YP2-LHdjKeTmjBKURwbENYDQ name:map[type:String value:Alice Johnson]]
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
Inserted with ripple: map[id:bG903Jycy5xUgCtu0-u1SbpBE52R2h0fAyO7x0XRwetOHI107Cteh4bNG3hnUpsgz85gEgzqiuaLlM68eojD3w]
Inserted with bypass_ripple: map[id:PR8v2Q175HDtiRuxwvq-1VdBafAf16hV9P8uSsU2dpWgA_TAVwReDo-R9jcEUrZ8ZdQCingxh4U4u-GniQspQg]
Inserted with TTL and bypass_ripple: map[id:7oyKwdNqr1Wx-I8GMa8J-BRax6v8x29CwfJbBpPTAo7o-17ppxGg0TQlnfpRaDUoBHNtrlZRueyPx7hT4XMdow]
Updated with bypass_ripple: map[id:bG903Jycy5xUgCtu0-u1SbpBE52R2h0fAyO7x0XRwetOHI107Cteh4bNG3hnUpsgz85gEgzqiuaLlM68eojD3w name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:j2rmCXDLAcarVN974h-8E7X92yH6NlJcH9syeC0WnE_aYVFEzlZ8P7BVVazYCD3IIHqJtPMOCBlZwk0N1is3rQ]
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

added 1 package, removed 1 package, and audited 13 packages in 846ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'NkOTwK2mvpfZxGQAoqd_hWqa0yiUhKEwu0EN4aAFrNedIwm2g1dMYc6juZ2pUwiJkpoApRja-WysZ_mbTpmxcQ'
}

=== Find by ID ===
Found: {
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  id: 'NkOTwK2mvpfZxGQAoqd_hWqa0yiUhKEwu0EN4aAFrNedIwm2g1dMYc6juZ2pUwiJkpoApRja-WysZ_mbTpmxcQ',
  name: { value: 'Test Record', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  active: { value: true, type: 'Boolean' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-01-06T16:11:46.507Z' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { type: 'Integer', value: 42 },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-06T16:11:46.507Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  metadata: { key: 'value', nested: { deep: true } },
  id: 'NkOTwK2mvpfZxGQAoqd_hWqa0yiUhKEwu0EN4aAFrNedIwm2g1dMYc6juZ2pUwiJkpoApRja-WysZ_mbTpmxcQ',
  name: 'Test Record',
  price: 99.99,
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-01-06T16:11:46.507Z',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  categories: [ 'electronics', 'computers' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  price: { value: 99.99, type: 'Float' },
  value: { type: 'Integer', value: 100 },
  id: 'NkOTwK2mvpfZxGQAoqd_hWqa0yiUhKEwu0EN4aAFrNedIwm2g1dMYc6juZ2pUwiJkpoApRja-WysZ_mbTpmxcQ',
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  created_at: { value: '2026-01-06T16:11:46.507Z', type: 'DateTime' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: BjA_o7c7bxFRX8luTpTzZoP-uVOCNarFDg63x5tgulG5F2t5I_2FMcHz12gMZXZokH_ipPOilm1MREkt0spWcQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: 8XkrVe2X7tEAuzDpHbCCdbaiQOsN41M15mzK5CwPua-Lp-7XIiEmCtGLjKez9FQAMGzrbWF_tPO4LsYfQsQOtQ

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_py,client_collection_management_ts,batch_users,ws_ttl_test,schema_users_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
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

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Jphp2U5n6aDtHHZDrtuO1XQMJi9Zdv138p19siK65KfDxJzrWqMAXB-yF-uwPOdK2a-I3D8pTwWfXeYhbxHt6Q
Created Bob: $500 - ID: H5D7vGHHH_a872fGpHI03S-FzOBqXC8bSSB3PuYBKPvqvzuoPyDJe0-p3pd6V7MMRvhb3dE1X8uaF17yOQGmQA

=== Example 1: Begin Transaction ===
Transaction ID: 49c2ad8e-ef80-4523-804b-a31a33baee08

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: af7e0a6e-b026-4b7b-ae84-c5bf57072ba2
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
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
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio.value, bio
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: title, bio.value, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, bio.value, title.value
  2. Score: 39.600, Matched: title, bio, bio.value, title.value
  3. Score: 39.600, Matched: bio.value, bio, title, title.value
  4. Score: 39.600, Matched: bio, title, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.769, Matched: 
  2. Score: 0.764, Matched: 
  3. Score: 0.762, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706, Matched: title, title.value, content, content.value
  2. Score: 1.505, Matched: content.value, content, title.value, title
  3. Score: 0.308, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, skills, bio

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: lmEHQ1wiMOqa7dc5EA1o0h8pzCvEKrcspbD6EjEr06Yf0HhJeH5USIRMs4AqY-TQ_hXvO2rqr03BQoitzMgXHw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Vj5Rq3xHf72piBSA6AQ4eTGhtJ9vcQkvgAVSLOT6Mb01_zBI-2EbCiMitoIj0N6twSNwrT_BeIbg2y7nBTkRwA

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

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: d9zW7p-z6gvuILHuMcPA36CnhfQKnP8FpjQxQ0RnbDJpZkkzA-imkYxDXdWUYLHzed6R0JayzuPFvhuADcsY4w

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: FQ07VMy6_2GZvmkOI3fP1a5bauP2I2XlMqxq2OEvFLXSnb9RmbsDBvYfGjyk8G1RSk3V6Qb6ryDQB_2oCAwEvA

Call 1: Cache miss (fetches from API)
Response time: 85ms
Result: {
  "records": [
    {
      "id": "cgbg1_3ca3qP14OjTTOlQOMJSTLgiQkp4l5JJ7NNMdkcJz7uoIsHdLJJnHve0P_-iLQd6rFeRy3LYgegq1RpSA",
      "cached_at": "2026-01-06T16:11:51.980Z",
      "data": {
        "value": {
          "username": "Bret",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "id": 1,
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham"
        },
        "type": "Object"
      }
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
Response time: 62ms (1.4x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T16:11:51.980Z",
      "data": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "id": 1,
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "website": "hildegard.org",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          }
        }
      },
      "id": "D3kCYiP6RdCwQccHbl86nY26Fx_4VOi5g22Ehdf9POf1UR07FMQem5vp7QN6O6y8udDd6KAb87Uh-yXbC2uZlA"
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

🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: Td_ihLoFjFlRTD24nR5tDXqlNdq2N4XJK2ZaE8-CcibdKB8EXT1ySOv5XXF6G-AJy6I2fVrGeTZnMHkBPUCc3Q
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: CtqLp-0SspWSvbmWHQE5WXc_KhTpt1plsS1Cq8mBR0wLRaVrNLhwcfc-6XShAN89a71wSvfmWDo5kq9NPDhAZw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: KKOvQE7FQRC4fAKcgfX9Enic6VwAzuRJ0lkL-GSEzTZBCTBXepOCs3yZRQwFUldtKrxXiNXUPGdAkSTB9CSw1w
📊 Statistics: 3 groups
   {"count":5,"avg_score":60,"status":"null"}
   {"avg_score":50,"status":"inactive","count":5}
   {"status":"active","avg_score":60,"count":5}

📝 Example 4: Script Management

📋 Total scripts: 41
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
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
   ⏱️  Duration: 143ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82ms
   📊 Records: 1
   🚀 Cache speedup: 1.7x faster!

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
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: Efex87mRKT3hyc3lfTS7smLOs6AxuhJYZIb5WMXVlEYfOZifdxASp1nbHdot9jCnBB3ySSudS-MKeMu3PJi2aw
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"category":"Furniture","count":2}
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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: 51ANrvHZIR5mzg-g9JUG0msry_j132qFpgk51-xJJCIf8pC599_Woo12eg5WybqF9kf47kPsIR_vA4JxraUdpg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: P_kP3vgLvn1wFEmepzuhoe_tKDpFy60SaP6JMqAymQJt_5eft4MsdihGW_eLBuTvKjwESlq5uAHfpWo74NsOog
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: bv9pM8WsYgrGhSdlOVsBeeFkd5tgeHCgc8LW9NYpXc6IePFRjz2zufJMif7YAvBMcAJyioubWQqQAWgyj3BgFw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 3kSNAgvLTNSLsfW8wwpa6z7XDxI5_ZaoArPsoftjg-xhYMcQUGeGbnIL9Wauhjt3qcr6bumJpuNZUq_Ta7ohCg
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (WNtHJ6kKcLy6_hOsLGO6fERKT_5e1TKbnmSmgXwVOq7Gc1Z8jUiSbZd869ZGBuz9wiKUxZKfsh_qCtiHYjlHjA)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "id": "ZJEYqgbXQF55FpeNz8mHjnVhUPPiz4LG8nPcUHpK8k_ZcRuMiYzZLlC4CeA53DmQzyXw2GSe9qPrSroRlsiIfQ",
      "data": {
        "type": "Object",
        "value": {
          "company": "Linux Foundation",
          "created_at": "2011-09-03T15:26:22Z",
          "twitter_username": null,
          "public_repos": 9,
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "id": 1024025,
          "name": "Linus Torvalds",
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "bio": null,
          "followers": 270299,
          "type": "User",
          "url": "https://api.github.com/users/torvalds",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "site_admin": false,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "email": null,
          "html_url": "https://github.com/torvalds",
          "hireable": null,
          "blog": "",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "gravatar_id": "",
          "location": "Portland, OR",
          "login": "torvalds",
          "following": 0,
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "user_view_type": "public",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "public_gists": 1,
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "updated_at": "2025-11-24T04:16:14Z"
        }
      },
      "cached_at": "2026-01-06T16:11:54.462Z"
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
Response time: 76ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (MVWoOkLEmUN9XeKwaGIDx2A1cS8lnrqHawEw1unsUN0OvdLy7Z5WyEXU-QdcNvVc7HDDK54GEDS8F6SaEMLRtA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2026-01-06T16:11:54.675Z",
      "enriched_data": {
        "value": {
          "meta": {
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "createdAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687"
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "rating": 2.56,
          "returnPolicy": "No return policy",
          "warrantyInformation": "1 week warranty",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "dimensions": {
            "height": 13.08,
            "width": 15.14,
            "depth": 22.99
          },
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 3,
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            },
            {
              "rating": 4,
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com"
            },
            {
              "rating": 5,
              "comment": "Highly impressed!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            }
          ],
          "id": 1,
          "brand": "Essence",
          "discountPercentage": 10.48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "price": 9.99,
          "shippingInformation": "Ships in 3-5 business days",
          "availabilityStatus": "In Stock",
          "sku": "BEA-ESS-ESS-001",
          "stock": 99,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "minimumOrderQuantity": 48,
          "title": "Essence Mascara Lash Princess",
          "category": "beauty",
          "weight": 4
        },
        "type": "Object"
      },
      "id": "hnnkprsSa_6ADATIICz9g-OVIXQQVW2dLmEEIQEG8XMUNbAwOi48AnBOP6fE4aHU0R1hAXI1AFWk0-jJE5rJzQ"
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

🚀 ekoDB TypeScript Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {"category":"Electronics","avg_price":367,"count":5}
   {"category":"Furniture","avg_price":365.6666666666667,"count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits such as accurate representation of geographic data, ability to scale without losing quality, efficient storage and manipulation of complex spatial data, and ability to support various types of geometric operations. They also allow for easy editing and modification of data.
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
🚀 ekoDB TypeScript CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {"role":"user","count":7,"avg_score":70}
   {"count":3,"role":"admin","avg_score":20}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":2,"category":"AI"}
   {"count":3,"category":"Database"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 0OAikv6tJLqRDgUuWi9Pnl56j2x8xE7bzmwiLTgU9RMca2eAvrP-r175vrCL8NMFU-sg7NLECYq90SvuxK--_Q

=== Sending Chat Message ===
Message ID: AWow5_xsd4ujaD1AS0dy4ob5NHcgwTi8aQymSkOBU-i55hGyw_aJJPGYfXTHE1mwCrQdMs_nnS_24PeJK4PWXg

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. The price for ekoDB Pro is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'cSvFdR6v6AQ5ZwGpdlXD0uq0XHrI1h6t9Zhh2oATeVVkWsQiAQR_OUY7CdhcNcFcx2YXTKD39JHF9N9tlubfWA',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    id: '3iFPjMrT1zzUEUVIPWFhQ0DGC1794z9LaUerFeQrnai2sdTG7rwmGQH49mW6sQiK4hn-ehbjXJmPSs4p2qvxsg',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'S_ynNVqNmr2ZVUdVlc5mDRnyloersYTe89wg8vyheS6YoVZgMrhzMeiifcH6Ap0ckl8b7kNMHGfDw-EUeRT5sQ',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 7699ms

=== Token Usage ===
Prompt tokens: 453
Completion tokens: 93
Total tokens: 546

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Y2q-D-6ez9NzP7rIljtrPcF7qIgkxoYnDz9nl7syRY2cOA0QbaUEMai47E5HKZgyFcs45cbZkiSttELNuPsE-w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a high-performance database product available named ekoDB. It is priced at $99.

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
✓ Created second session: 9lzCblLUBkxRFffds-OVpG9c64OgZjfh7QP7knhbj8J8Zy-tt4xi24dUxNlQJojSy0TRk1HH_cqAGuZ5sFlChQ
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: mokgXfm_deB8_nvB7m2cxF9e2-pijcG8V-souy39OJNjzlaLN_fLpWv_EUrNc5TCBYqoJB7Gako8ntad_ThuOQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 0dcPElXXqozuaR7GyPxnkvHWjPHq0m6it4SdEu5eI40L8MQ_ysBD1UM09wsiOge7y8V1CzbK7slx7tkQ1flTQQ
  Parent: mokgXfm_deB8_nvB7m2cxF9e2-pijcG8V-souy39OJNjzlaLN_fLpWv_EUrNc5TCBYqoJB7Gako8ntad_ThuOQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 0dcPElXXqozuaR7GyPxnkvHWjPHq0m6it4SdEu5eI40L8MQ_ysBD1UM09wsiOge7y8V1CzbK7slx7tkQ1flTQQ (Untitled)
  Session 2: mokgXfm_deB8_nvB7m2cxF9e2-pijcG8V-souy39OJNjzlaLN_fLpWv_EUrNc5TCBYqoJB7Gako8ntad_ThuOQ (Untitled)
  Session 3: 9lzCblLUBkxRFffds-OVpG9c64OgZjfh7QP7knhbj8J8Zy-tt4xi24dUxNlQJojSy0TRk1HH_cqAGuZ5sFlChQ (Untitled)
  Session 4: 0OAikv6tJLqRDgUuWi9Pnl56j2x8xE7bzmwiLTgU9RMca2eAvrP-r175vrCL8NMFU-sg7NLECYq90SvuxK--_Q (Untitled)
  Session 5: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg (Untitled)
  Session 6: EB3jEuzG-LxkynD65O1rXeJuvg8QIZkvVwPEHVz2k56v6PiRdnSL7oN61GeamNEYGSrhJJXdcEuHNcMobxBfPg (Untitled)
  Session 7: X9t5cttsehSZsRMNaQJaFNuP86HLbljgFO9jHItG45p3NzgjeeiOKF4tY97d975r8UT0y_xwUsC38ihAeBwYgg (Untitled)
  Session 8: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw (Untitled)
  Session 9: Vwv_dS-QtANBp_pTXuP31-g0Z56KtYJkKqzGlcNeqgBv68RCtjOiUv2gnGQKhO03ZeTUkAZZVX49FONaJsaS-w (Untitled)
  Session 10: ld9rVKp4aH4JOkUnFVYGKj4j8r2wHCUzMhFDp6pPQ_uKiRHIPwF3xlBjj_jGZnNHXzBvJCFoL7eFsx1FGxpNYw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 0dcPElXXqozuaR7GyPxnkvHWjPHq0m6it4SdEu5eI40L8MQ_ysBD1UM09wsiOge7y8V1CzbK7slx7tkQ1flTQQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'Ehk_CVjoj7BNideUXs3VgakYwDC7rsEEcUFUMr7QnZ5mrwFEY-0pbkXvvTtjRFDQTnWKMl-Ie5MkhBHfru0wkg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { value: 'Alice Johnson', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'Ehk_CVjoj7BNideUXs3VgakYwDC7rsEEcUFUMr7QnZ5mrwFEY-0pbkXvvTtjRFDQTnWKMl-Ie5MkhBHfru0wkg'
}
✓ Second upsert (insert): {
  id: '5qMgRQGFC3WLB87k-kyjWEaNuJFNLrzbG-DR5DbpcPSxw1F3OGF-twJ2NKFa2N1r2z5rsRXCGRPCnB67YMZbyw'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { type: 'Boolean', value: true },
  id: 'Ehk_CVjoj7BNideUXs3VgakYwDC7rsEEcUFUMr7QnZ5mrwFEY-0pbkXvvTtjRFDQTnWKMl-Ie5MkhBHfru0wkg',
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { value: 'Alice Johnson', type: 'String' }
}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"aNvVLsPmGEtG0T9miQl4NLiI5uqJEnjx7r5Ms-bHU8qS1xpM-oc1piwh2qviQ_dgrXgLAnL4KP3z1qhJWn1faA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"jPwtgFlNwpBdUDG3zuApWRR6WhWutzQYlj1jMjnRB_yatesA7Gmgct_ht1jp4HJEvYBGLsvJQnUz8tS2tDXHig"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"aNvVLsPmGEtG0T9miQl4NLiI5uqJEnjx7r5Ms-bHU8qS1xpM-oc1piwh2qviQ_dgrXgLAnL4KP3z1qhJWn1faA","price":{"type":"Integer","value":150},"name":{"value":"Product 1","type":"String"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"kxnE6nSdXfBtBP_3niB279pEJyzRDM1pVxUuDDePUwUsHfj37Vd0fTcCPXSB7aWY9iUdwxF8AMcEl0YIYNmgUg"}

✅ All bypass_ripple operations completed successfully!
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 839ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:46690) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: '6O_JOopTOadlVOLXNjB8GEF4w74YyoEvihvUMf_z0cyuQ9eRcrScdaIlSZwiTHy_LOsBJU4e9kF22CCXv89u6g'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' },
  id: '6O_JOopTOadlVOLXNjB8GEF4w74YyoEvihvUMf_z0cyuQ9eRcrScdaIlSZwiTHy_LOsBJU4e9kF22CCXv89u6g'
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    id: '6O_JOopTOadlVOLXNjB8GEF4w74YyoEvihvUMf_z0cyuQ9eRcrScdaIlSZwiTHy_LOsBJU4e9kF22CCXv89u6g',
    name: { type: 'String', value: 'Test Record' },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  id: '6O_JOopTOadlVOLXNjB8GEF4w74YyoEvihvUMf_z0cyuQ9eRcrScdaIlSZwiTHy_LOsBJU4e9kF22CCXv89u6g',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 0qSV5P8zfDGeaCJNHEoNmqH5U7p22lSpMlkdHQ1VVKOYpqerHRO6pQBLCQGkoRKe1c1csOal0o8GYy9EJdRHnQ

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "0qSV5P8zfDGeaCJNHEoNmqH5U7p22lSpMlkdHQ1VVKOYpqerHRO6pQBLCQGkoRKe1c1csOal0o8GYy9EJdRHnQ",
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
        "id": "NijRrca-9YBQBeXU6j280I4rPcUwOpFnrXCbwAWzKK-LMaIattQNOpF3zXfRLzEbyzyKyf4XMcJ6kC1iBfqmHw",
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
        "id": "wOhw36APY7lqulIUv8F8sgCT5KKowLEVn1StWe5ITSbhiGFihQjS0yKngcE0-d6Pqs0oe1bokPr8hlOsg3kr1w",
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
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: bXRmxtZ2u9BnqkiVJvHRTpu5L3EN6wnp4yf7zkaodiRJLaz8wpsul4pMOB0WstVN459BWHawqNdM73fzAwrqlg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 0vsEuLQ19I7h6lVDbrxOb5iMiCaDWnKBtF0LPn6bH3e5s7xFoT8iaT4Qx7WSyfnAPtKw4ohaudptsyVYQ-DYYw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ZJ0_IJDOpwGXksNMRctZ_avpC48byXxpUcdSFTZiQtAmvntk2AIXoc0PN0AeqZ3WufSEgvuDR52NW6UtzYmCNA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"active","count":10,"max_score":100,"avg_score":60}
   {"avg_score":50,"status":"inactive","max_score":90,"count":10}

📝 Example 4: Function Management

📋 Total scripts: 52
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: pFVo_vBM-dldzNueqyii0Z6MS8CQCeF9BWBo4vw45KQHAwmtzJny8zdhtHo6-D-7DRtK4rt7YW-x8kWj-2Xdeg

=== List Collections ===
Total collections: 27
Sample collections: [
  'schema_products_client_py',
  'batch_users',
  'edge_cache',
  'ws_ttl_test',
  'schema_employees_client_ts'
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: EoP9mwolhGBSGAFj2GxzoShPtTwJABgoKqO5UJi3I1JSqN-1zS_sOnO26r9-y_kkgqvvs38ohzi-vwFVo0bm2g
Created Bob: $500 - ID: 2-CYZ6rYrrTjuRyoTo1mq-ubvgd8N8NwPQ-qwfHLUtyzHV_p_MB18MVhIGF0CqUaiSWjfF87_C6oT0KdISqirA

=== Example 1: Begin Transaction ===
Transaction ID: 32238d4f-13df-4081-b41b-91c3237812be

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
New transaction: 37b07649-9ffe-47ee-bede-9cf27a5da51d
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
   ✅ Script saved: ERIPGjwmPV_X2_5JzgCM0ipUB_90btFUH3Mfok9WxvzPLVnUzudeRzsjVwY_ip632aG8fMzjt5jmVETN1lm8SQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: yo0gPJshz_O1sVRhXg3_2ErCUclDcvQQZmLFTHrd4dGXjfw2svaXY7q6p9sOf-5bSbJFlmNmZpVa-CTfsXJvBQ

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ZAm26s0szdeQKNx3XeumqvBIAbalpuS87nCWhSUtVktSPNRAEpEk8pylOK3iymcRYyOLfc-vA0lhErxSKVMoCQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Nv11yEfLIHlzJh9nRibhSNQVS9rKhZFWpSrY8F3xwOLZTldcNLiMsbVQmMTuoa8AQ-TBO8EJEyYeqtvYw1OSzA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ERIPGjwmPV_X2_5JzgCM...
   ✅ Deleted script: yo0gPJshz_O1sVRhXg3_...
   ✅ Deleted script: ZAm26s0szdeQKNx3Xeum...
   ✅ Deleted script: Nv11yEfLIHlzJh9nRibh...
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl/document_ttl.js ===[0m
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = Fyfl5CYAj6aDI-Cj_RBBy3HvVSRJhDoIiGRM-ioexKFLb0iCpkWGYup0Xx8euKd-OtsVsm7NxDzFb9pSA6c5_w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Fyfl5CYAj6aDI-Cj_RBBy3HvVSRJhDoIiGRM-ioexKFLb0iCpkWGYup0Xx8euKd-OtsVsm7NxDzFb9pSA6c5_w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Fyfl5CYAj6aDI-Cj_RBBy3HvVSRJhDoIiGRM-ioexKFLb0iCpkWGYup0Xx8euKd-OtsVsm7NxDzFb9pSA6c5_w)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl/websocket_ttl.js ===[0m
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = icddADD8h2x03_Y3oOXYIDrYaoreaCwlFDEuvvTXrFBbaQQyJ0zHne-j-LxRN9pK5JvoCiykoAVC0HIA18O3sQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(icddADD8h2x03_Y3oOXYIDrYaoreaCwlFDEuvvTXrFBbaQQyJ0zHne-j-LxRN9pK5JvoCiykoAVC0HIA18O3sQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(icddADD8h2x03_Y3oOXYIDrYaoreaCwlFDEuvvTXrFBbaQQyJ0zHne-j-LxRN9pK5JvoCiykoAVC0HIA18O3sQ)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return null for expired documents
[32m✓ websocket_ttl.js completed successfully[0m
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


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 820ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '-7KkEgCLh462UpHQk1QHVGptM52VLdvyT_5eawQWQGmxueHbjiZ_K_R7SCE8lGQZ9HTZ97x1XXTFX7rO_ahJnQ'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: '-7KkEgCLh462UpHQk1QHVGptM52VLdvyT_5eawQWQGmxueHbjiZ_K_R7SCE8lGQZ9HTZ97x1XXTFX7rO_ahJnQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: '-7KkEgCLh462UpHQk1QHVGptM52VLdvyT_5eawQWQGmxueHbjiZ_K_R7SCE8lGQZ9HTZ97x1XXTFX7rO_ahJnQ',
  name: { value: 'Updated Record', type: 'String' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: eGcuOIrfm984P1ZmOMIQiJ92gZRubLFFfuxAzwzzqi4Cs1l3j8gm4tKh6pzyTu-jnSZogJnbWmVkZ1k8Gx0nJA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: MCsjLO24D5fhOjv7HEC-LnmEEquLozjq5QyRDHv-YCJWiyi0T8IO1tC_a9bhV1GGbDUCmALL8nZ2wqEVAv5NtQ

=== List Collections ===
Total collections: 26
Sample collections: schema_products_client_py,batch_users,edge_cache,client_collection_management_js,ws_ttl_test

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 6

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: bXA9so30w0NsfBoCentwz3cAXLY0B7CvFto4ZQQobg23hWMA4hz1e28f9k2juj_9_uKKwBlY5TKxnV4JAsp38Q
Created Bob: $500 - ID: GHSYB56vR6K_JbZqIh1mqQfWVExB0yIUzChcaLxo9KMTek90bvGOrc-6TfYmyDV21cNyvBryuspHR2jPLWpBbQ

=== Example 1: Begin Transaction ===
Transaction ID: ff66fcea-5ced-4892-82c0-a710c6306414

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: e3665c6c-9f72-4c77-8490-30fe151eba2e
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
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
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, bio.value, title.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: title.value, bio, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.785, Matched: 
  2. Score: 0.778, Matched: 
  3. Score: 0.743, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.714, Matched: title.value, content.value, title, content
  2. Score: 1.497, Matched: title.value, title, content, content.value
  3. Score: 0.311, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio.value, bio

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: fN9XvDPTi-OudxlbdpqjmrYzTnX1l5VSguE-fAv28hy5MobiGPvMpUo1KmNA-4hzli6desTihqF7QMCpnwN8jA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 02A0dH7_VzwuX1Uq-YBJUQrSru4l_IiZvw7hxYvu04t0fL0-z01onYYVWiM6gsWVtcray2gcd-jQ2TgoFp7Z-w

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

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 7oRTnXdjRqd52329OolXkR4Aff23n5drVijpz85saEwk9ifaSmj58CtLxuNL-ewp6z3q8A4k7VlEHwOlnLXA9w

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: igYtXlYIazbJ607SEuWPCHUNQZe_LihEEgaNM1HvHMs8rIRr30VX405gx2ybj1Nd1DvohKd0plKUsVnXShhv0Q

Call 1: Cache miss (fetches from API)
Response time: 549ms
Result: {
  "records": [
    {
      "data": {
        "type": "Object",
        "value": {
          "generationtime_ms": 0.028848648071289062,
          "timezone": "GMT",
          "utc_offset_seconds": 0,
          "longitude": -73.99309,
          "current_units": {
            "time": "iso8601",
            "interval": "seconds",
            "temperature_2m": "°C"
          },
          "elevation": 32,
          "timezone_abbreviation": "GMT",
          "current": {
            "temperature_2m": 5.1,
            "time": "2026-01-06T16:00",
            "interval": 900
          },
          "latitude": 40.710335
        }
      },
      "cached_at": "2026-01-06T16:13:03.744Z",
      "id": "eat5wDHaHufQMzEPx6CLGk0Na0OV12ovOGLhwDk2-zNe29PWTOuRVsjELbaYnzPyDB2KY3AHTWiG_GGSF5urgA"
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
Response time: 594ms (0.9x faster!)
Result: {
  "records": [
    {
      "id": "94LBJN1WBZsprioJKgy8JAKc8nxaHKXNtBnYfs9f3UhTqDE8agdTCbwzf0ez10X035JMUYM6IVT6km-wMBUlQQ",
      "data": {
        "type": "Object",
        "value": {
          "timezone": "GMT",
          "generationtime_ms": 0.024557113647460938,
          "current": {
            "temperature_2m": 5.1,
            "interval": 900,
            "time": "2026-01-06T16:00"
          },
          "current_units": {
            "temperature_2m": "°C",
            "time": "iso8601",
            "interval": "seconds"
          },
          "elevation": 32,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "utc_offset_seconds": 0,
          "timezone_abbreviation": "GMT"
        }
      },
      "cached_at": "2026-01-06T16:13:03.744Z"
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

🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: rrOqnW6-sSTGqNsMv3zZmDsjIIpH5tjQf4H1H9j4WJcnoJSfxL-DKh49cluvMkvVzSAQOXK8V8n1oiU3Oks8UQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"avg_score":60,"count":5,"status":"active"}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 55
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
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
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
   ⏱️  Duration: 141ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 91ms
   📊 Records: 1
   🚀 Cache speedup: 1.5x faster!

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
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: iLhI-HDerNCzv62VdQkr6294UtFTjxNN-plrN_jjoJvDgvRV-MmQ6VGa5nH0qlk4HWS0McOZ2GMVpd4pmY4cug
📊 Found 2 product groups
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
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
   {"category":"Furniture","count":2}
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: V6zQNuh_YmROvez5xErXBdry37eRH0bYJCBktQRgsEizG587YnsVr_CoJ3BJUDBfv4JVysLi-KdwhGyVlqdjqg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: RBBTMepn2wT6XocKHtVCZ2aupsE9eCltpiXfeNJqyMJgecrkVUaYmZ7EDS5-wjvJHB9HU4UsLjL3_HidiEfAyQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: mIy4R9LyMfw1o_qSiKjI0H5b0ytaHwZmmu39yUEXVGINkmG_N7fSYXm8jMQvjQURpMN8tzKbC263QKoo0zo1FA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 5QXnVx7FRjVgOVKtPnDD9RcCIIT1D_UZF0eLEp_eGY5sXMRIJpW2RaVqD3u-_7x3Ue1G3XXXV8xTdykwAxeNqQ
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
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
📊 Users (showing first 5 of 10):
   1. User 2 - Score: 20
   2. User 5 - Score: 50
   3. User 9 - Score: 90
   4. User 8 - Score: 80
   5. User 3 - Score: 30
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
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
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
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
   1. Vector Databases Explained
   2. Getting Started with ekoDB
   3. Database Design Principles
   4. Introduction to Machine Learning
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Getting Started with ekoDB
   3. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
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
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Webcam HD - $119 (⭐4.5)
   2. Laptop Pro - $1299 (⭐4.8)
   3. Monitor 27" - $399 (⭐4.6)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. High Precision: They can represent data more accurately, as they use points, lines, and polygons.

2. Scalability: The quality of the data doesn't degrade when zooming in or out.

3. Data Representation: They can store complex geographical data like road networks, drainage systems, etc.

4. Attribute Storage: They can store a large amount of attribute data.

5. Analysis Capabilities: They support complex geographic operations like network analysis and topology.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Draft Article" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_js (aERTbukYdmMpCP-l0DOkpmSszvwyAkNoZs3EABPMht3OkBXiXRDom_6DBJi6VSM2HlM-lzgV3meznnayRIpp_g)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T16:13:13.302Z",
      "id": "ZuBNuxlOIKr0cuQbpt3DLDptQJyKl_PKCXpex4ZMSxzVWbLFEQAt2qLUBWJBOCgwzB36J0nTP2Ehc-12CgR8Cw",
      "data": {
        "type": "Object",
        "value": {
          "email": null,
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "blog": "",
          "updated_at": "2025-11-24T04:16:14Z",
          "gravatar_id": "",
          "public_repos": 9,
          "bio": null,
          "public_gists": 1,
          "login": "torvalds",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "location": "Portland, OR",
          "twitter_username": null,
          "user_view_type": "public",
          "created_at": "2011-09-03T15:26:22Z",
          "hireable": null,
          "following": 0,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "name": "Linus Torvalds",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "url": "https://api.github.com/users/torvalds",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "type": "User",
          "company": "Linux Foundation",
          "html_url": "https://github.com/torvalds",
          "id": 1024025,
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "site_admin": false,
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "followers": 270299,
          "followers_url": "https://api.github.com/users/torvalds/followers"
        }
      }
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
Response time: 87ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: pRivlIJEaoy3S6kCtqxMOuO2GXmR7rkWT8BsDj8KY3ILfqZb4ZERjVfvP5U1tBMtMStIsKls1_xGx9WfDYSb0w

=== Sending Chat Message ===
Message ID: 0YH4HQU1w3kwb_KNvRnE6ytorBfi13XXSVysqwrnplEoFXGsOw5XogSPJky0pFWyR2q3kiw80YoAVz6pJWBBSQ

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It's priced at $99.
2. ekoDB Cloud: This is a fully managed cloud database service product. It's priced at $499.
3. ekoDB Pro: This is our enterprise edition product with advanced features. It's priced at $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB',
    id: 'M6zBKUETuYcu8QVkEJ6naLWv2Ef1MgLeOcNabHcdpkzvXR5BAong79Y-2c0UIVtU8Z7Jpgbj-R_3sYLAE4lHtw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'z__YtF3bmS-PLWyy1phbPOOh7JuFTUwp2NUmtcYDtxLmUeOs_LwYg33CBLC9UTC3iQRW093PdaIp-7sXks48ZA',
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'SMJxCE7CT1AsmFjvLG8h36EOHR95JYhitBEpUojjNbVPSOIxbSNJfT3D7lg5h-q2y5OnUspZXPvs622gUcKttw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3884ms

=== Token Usage ===
Prompt tokens: 447
Completion tokens: 78
Total tokens: 525

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: lKLUqic3BfSkc5G4dFHedhyhXRpVfOmIC8ZQExHz9eeUWY8hN1KMwobk1eIaE2G19jUq2kl2Dn026O36o3bwSw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is "ekoDB". It is a high-performance database product and is priced at $99.

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
✓ Created second session: CgafTDcw3gdix761YQ59PLeo7Z6twbp_I546rAhBXkIS8MnWhkGATSFX77x-_9va_3iVoTIbJT3VgT3T7gzZ8g
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 4Fo0DeaY1SJIMXfsb6JXjPqtIeiJ93jI0KM-53GLBR0nJZDMhrcLO9Qg902il80eA9u0JNTLKdOxFTul72GfQg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". This is a high-performance database product and it costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: tfEgPxtOUjCzVSjKQI8fmJZOeRLYk3iD9PsyNTWzdeOKro2fjnRFj2HcdZJddw3y5ZN2d2ID8lFPHSybOr8Uxg
  Parent: 4Fo0DeaY1SJIMXfsb6JXjPqtIeiJ93jI0KM-53GLBR0nJZDMhrcLO9Qg902il80eA9u0JNTLKdOxFTul72GfQg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: tfEgPxtOUjCzVSjKQI8fmJZOeRLYk3iD9PsyNTWzdeOKro2fjnRFj2HcdZJddw3y5ZN2d2ID8lFPHSybOr8Uxg (Untitled)
  Session 2: 4Fo0DeaY1SJIMXfsb6JXjPqtIeiJ93jI0KM-53GLBR0nJZDMhrcLO9Qg902il80eA9u0JNTLKdOxFTul72GfQg (Untitled)
  Session 3: CgafTDcw3gdix761YQ59PLeo7Z6twbp_I546rAhBXkIS8MnWhkGATSFX77x-_9va_3iVoTIbJT3VgT3T7gzZ8g (Untitled)
  Session 4: pRivlIJEaoy3S6kCtqxMOuO2GXmR7rkWT8BsDj8KY3ILfqZb4ZERjVfvP5U1tBMtMStIsKls1_xGx9WfDYSb0w (Untitled)
  Session 5: mokgXfm_deB8_nvB7m2cxF9e2-pijcG8V-souy39OJNjzlaLN_fLpWv_EUrNc5TCBYqoJB7Gako8ntad_ThuOQ (Untitled)
  Session 6: 9lzCblLUBkxRFffds-OVpG9c64OgZjfh7QP7knhbj8J8Zy-tt4xi24dUxNlQJojSy0TRk1HH_cqAGuZ5sFlChQ (Untitled)
  Session 7: 0OAikv6tJLqRDgUuWi9Pnl56j2x8xE7bzmwiLTgU9RMca2eAvrP-r175vrCL8NMFU-sg7NLECYq90SvuxK--_Q (Untitled)
  Session 8: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg (Untitled)
  Session 9: EB3jEuzG-LxkynD65O1rXeJuvg8QIZkvVwPEHVz2k56v6PiRdnSL7oN61GeamNEYGSrhJJXdcEuHNcMobxBfPg (Untitled)
  Session 10: X9t5cttsehSZsRMNaQJaFNuP86HLbljgFO9jHItG45p3NzgjeeiOKF4tY97d975r8UT0y_xwUsC38ihAeBwYgg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: tfEgPxtOUjCzVSjKQI8fmJZOeRLYk3iD9PsyNTWzdeOKro2fjnRFj2HcdZJddw3y5ZN2d2ID8lFPHSybOr8Uxg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
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
✓ Inserted user: Record(fields={id=StringValue(value=1SiNh4_EaE_HDENDRB_buvyCgL0VpWed6QWgzRfG68eLEQA5Q4N8v4moB11xy09jmcM0yFfJ2L70OySlHWB6SA)})
  User ID: 1SiNh4_EaE_HDENDRB_buvyCgL0VpWed6QWgzRfG68eLEQA5Q4N8v4moB11xy09jmcM0yFfJ2L70OySlHWB6SA

=== Read ===
✓ Found user by ID: Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Tue Jan 06 11:13:35 EST 2026), type=StringValue(value=String)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), id=StringValue(value=1SiNh4_EaE_HDENDRB_buvyCgL0VpWed6QWgzRfG68eLEQA5Q4N8v4moB11xy09jmcM0yFfJ2L70OySlHWB6SA), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Tue Jan 06 11:13:35 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [name, tags, categories, created_at, embedding, email, active, price, data, metadata, id, user_id, age]

=== Update ===
✓ Updated user: Record(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Tue Jan 06 11:13:35 EST 2026)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), id=StringValue(value=1SiNh4_EaE_HDENDRB_buvyCgL0VpWed6QWgzRfG68eLEQA5Q4N8v4moB11xy09jmcM0yFfJ2L70OySlHWB6SA)})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=1SiNh4_EaE_HDENDRB_buvyCgL0VpWed6QWgzRfG68eLEQA5Q4N8v4moB11xy09jmcM0yFfJ2L70OySlHWB6SA), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Tue Jan 06 11:13:35 EST 2026)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})})})

=== Delete ===
✓ Deleted user with ID: 1SiNh4_EaE_HDENDRB_buvyCgL0VpWed6QWgzRfG68eLEQA5Q4N8v4moB11xy09jmcM0yFfJ2L70OySlHWB6SA

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

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
  {"payload":{"data":[{"id":"GG9-vxac-8-u05moAuZje9xp7Kizc4PtWqMcVF3QoklSQw3nCYdUoGdwRSRI2oxRE2gf14pbf5V7rxk72IyNnQ","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: 5fkwvR5R-H51Q9dD51CfPznaPKzGBDDfjbKbFwZRwOTVDvBDOXklFwAcuIObVfIdevrlmVnIO306MPL4OoJSFg, Qa5ZORGdLTqxP2pbANpGqwbIHyKo1QvbIkAr4TbxfVzxqftYQEDsUvS4swnF8D9e24ud48BdNlJqEdwoofQ-6w, QJNeHM7FWMdjUNu7TExooDK1GrjNcaWvwTOX7A_RmdcgawmzfIFvAJKpA6PeKfRc5kx6BWk-n2afScvfXuEkdw...

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
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 31 collections
  - schema_products_client_py
  - batch_users
  - edge_cache
  - ws_ttl_test
  - schema_documents_client_js

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-06T16:13:49.009047Z","last_modified":"2026-01-06T16:13:49.009048Z","bypass_ripple":false,"primary_key_alias":"id"}

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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"name":"Alice","email":"alice@example.com","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1767716033327}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 7

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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1c9gPtSgP1vlUyXTw-fEkvZsZNSTBGP4k4rZ7d7B9cjONb9ayInLRXlX0ZfodB4ITjQd-NfzVvzKIoSeRPpYtw
Created Bob: $500 - ID: UZ39SMIma_LDcMgC3fYG0h3I2Iu8-wOcd5k_7U4R5TMxJgIddugNtvv3AR_qZVE8_4t6-04tIvjs18QkTTB0Rw

=== Example 1: Begin Transaction ===
Transaction ID: e3eb7bf2-7c98-4323-ae03-4450c198b3f1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6ab34eb3-5c33-4a73-8eb8-b6a3927f405c
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed

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
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","id":"d3x8FoCGFgrjiaxRmZPusHqbPPZbpNBMNR2g37mFc5cyQG0fXXeqhafGgmWWCgq_fVH3AJOU09gUDi9vWyRZ5g","views":977},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"title":"JavaScript Web Development","tags":"programming,javascript,web","id":"f1TBp84faiBKgF76drhGOcLcv5xXDERrbMXkms91Abb2IvutWsO7IgKEWycluddK3go27LwiVi0Bnqy8r7f9XQ","description":"Build modern web applications using JavaScript, React, and Node.js.","views":649},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":"Python for Data Science","tags":"programming,python,data-science","id":"J-9e1nNc4zK37sgGjhrMcb-wpwGpLvJfz3s8EW9_VWRstMICDS9NjoUbLBFm113g7OoS2r4l3NlTSTq-Ru77iA","description":"Master Python for data analysis, machine learning, and visualization.","views":55},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"description":"Introduction to machine learning algorithms and neural networks.","views":624,"title":"Machine Learning Basics","id":"rGWdEAYHNi3MtXliEi-EO0K1WPt6m-wMvS-gu7NZFwtRGnyW4AHh0scDeUqLZrSid7G4PUhN_hVRmGWiD7iPHQ","tags":"ai,machine-learning,python"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","views":55,"tags":"programming,python,data-science","id":"J-9e1nNc4zK37sgGjhrMcb-wpwGpLvJfz3s8EW9_VWRstMICDS9NjoUbLBFm113g7OoS2r4l3NlTSTq-Ru77iA"},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"database,design,sql","title":"Database Design","description":"Learn database design principles, normalization, and query optimization.","views":39,"id":"gtBpqzBVul2zSKdlFqKnf87p5Y_Kk3OhFJlxGVRUfCp-gLxkGtgOhMeyAfuyC03Y1AGmyPMQ8PlTL6DGnzNKjA"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"d3x8FoCGFgrjiaxRmZPusHqbPPZbpNBMNR2g37mFc5cyQG0fXXeqhafGgmWWCgq_fVH3AJOU09gUDi9vWyRZ5g","tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","views":977,"title":"Rust Programming"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=vFMPD_TU1xw6GE9qqfFczcPc3IXw7E4ltcrpj-gvNCUpCKUY14BD1lomzXpsckwCStiei9ZeZHq8Gmti_Oo-iA)
✓ Inserted user 2: StringValue(value=UyqIpacUoYWh1D4GsG1UBz363eOJtwIZdyh1SDukDvzQQ7AaDYnVFJy3ojX5LsFr1kkI1uuHpJOiWuFpWKeZCQ)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 32
  Sample: [schema_products_client_py, batch_users, edge_cache, ws_ttl_test, schema_documents_client_js]

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
=== ekoDB Kotlin Client - Join Operations Example ===

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: ObLbiPqEkS0YxRS9XiI0UPXa33dx2K9C2RGAC6kPnMDL4-LvPl1o_YMNA-9kvnc1KJ5jrqMAR5yfZLbgpjPuGA

=== Verify Document Exists ===
✓ Document found: [session_id, id, user_id, created_at, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: -Nqu9VDUXYimXL0fDNetcXFtCIfz8-WSvHVO-M5Hlbk3r76BTmqD8TX2NjhzdPcccARJlLVqDL1wjrsJdaphJw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [cache_key, id, value, ttl]

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
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=wbPYsJEsAUoTO7ZHXVSX1_uTYeWgh8sHCsmrmtSvZEa-hGzyvSaPns31d6Tnwywir5-nJ92e9RU3hmtrG67ayw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767716063863},"id":"wbPYsJEsAUoTO7ZHXVSX1_uTYeWgh8sHCsmrmtSvZEa-hGzyvSaPns31d6Tnwywir5-nJ92e9RU3hmtrG67ayw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-06T17:14:24.006049Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: YEqtApq0Xfhj96E0eULbAHuS5O239AiL-yoIFGxLc6ji19NibnYBva9_cPIAfRHlYzYPa8QHSkEgNMVEbD5Sig

Call 1: Cache lookup
Found 1 cached entries
Response time: 13ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 7ms

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
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: wqxNFZkqNuQ02CWEjgH5zZTvSAJe8W6vbSin5JBilvOV2x27gaaap3pz6IMzsEG8pRW2Oil6DHvO56A0It5uow
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"status":"inactive","count":5,"avg_score":50.0}
   {"status":"active","avg_score":60.0,"count":5}
   {"avg_score":60.0,"status":"null","count":5}
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
📊 Total user count: 15
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
🚀 ekoDB Kotlin Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: NDijbars3al7hBXUZOpwSLy1LDzPb-9bxxxd3BMMmN9kTeUgu6puxhjGHYYi-NWDSEJVIDP5KYXHriO5ueyGCg
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Script management (save, call, delete)

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
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=OqQxZ1T-kQ_yRhvtOQoG21y_XTCrxeD2Equ6kHJ3Js1Vo1YokIoq0ZAV11uYXT80gBMs6D4K69DXvONPKSVeuA)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: O1klXf20Siw5w3Aq3uVwg1C4-ro6Od4BxA5McyNcBaIoriWbRFfuLEsM3T1RRM1x3GMMuS6F80r0uIeJZrt2pw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: nrzZHlfgfcI_V7LuhBbmuGhB3jMZSb_qykF3pD6Kvn8kcysfNyIk1IRKRuMWXT39IiMaUWg36vEmnDsHuvO29Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: AG3_JLzRV5cQ1LtBcMnHEhUHIE-XHwMprw38XbjeWZi_4YVDfeSDa0_f8FLonb6X3VQbp9L3BkxGWI3g1NxqWQ
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field* helpers for type-safe wrapped values
   ✅ fieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ FunctionStageConfig.Kv* classes work within scripts

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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup script
✓ Created SWR script: swr_cache_lookup_kt (CE2sHx8tdWOAB9m1Df4HgKhMSBExTFGgS7VRiVWj-8KoF1JOlYOR68ycg6j8Rg2vxKdmus-DYYTbK7GEMZ4-jg)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 7ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

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
🚀 ekoDB Kotlin Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {"category":"Electronics","avg_price":367.0,"count":5}
   {"category":"Furniture","count":3,"avg_price":365.6666666666667}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!

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
🚀 ekoDB Kotlin AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   {"tokens_used":99,"response":"Vector databases offer several benefits including high precision in data representation, ability to store complex geometric objects like points, lines, and polygons, flexibility in the data structure, and capacity for topological analysis, which makes them ideal for geographic information systems (GIS) and computer graphics."}
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
🚀 ekoDB Kotlin CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   {"status":"inactive","count":3}
   {"status":"active","count":7}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!

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
🚀 ekoDB Kotlin Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. {"type":"String","value":"Getting Started with ekoDB"} ({"value":"Database","type":"String"})
   2. {"type":"String","value":"Introduction to Machine Learning"} ({"type":"String","value":"AI"})
   3. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
   4. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   5. {"type":"String","value":"Natural Language Processing"} ({"type":"String","value":"AI"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!

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
✓ Created session: gyjCATU0CYI5N40MYOUZf1ndubsVbDAE299ICM8LRkk4KCEtivpBt_uURuXgnmtu2lFTFdBMOytoGZg930LNpg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "3H71Zf1HwolCQassWj5oqalh71jd7yxGeag5-8i3KZMbqv4cOSGuS_dSy0AOJwvDJHh1Qe2zj06NYu9RQsXynQ"
  Responses: ["ekoDB is a high-performance database that offers several unique features. It includes intelligent caching and real-time capabilities, enhancing the efficiency and speed of data handling. Additionally, ekoDB is equipped with AI integration which adds to its innovative functionalities.\n\nOne of the main features of ekoDB is its AI Chat Integration. This feature enables users to query their database using natural language, and the system provides AI-powered responses with relevant context. This makes data retrieval easier and more user-friendly.\n\nMoreover, ekoDB supports various search types such as full-text search, vector search, and hybrid search. These options allow users to fetch data accurately and swiftly from the database. The system also offers automatic context retrieval which enhances the search results by providing more relevant data."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 8s
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
✓ Created session: JjwWnZCyW_We_2vfYEL817_SllSWDlfdsiOdmwmrmLAR8rj6qxfxLNpm1uVDIQukKOoZ37aY5qogSopP4jAKbw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is ekoDB. It's a high-performance database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the context provided, the available product is ekoDB. It's a high-performance database product priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: LrvH5jF3Ct7IXiyz_kO8LGDti2xpmcB64qkmGmqpxzD1KWlkcdLoPGGj4hOGt9tZ1LMX9cnsli8ml5uIMFUurQ

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dFh8mRXTlL3TjtbEFv0Hl8CpTXA3B9ibVF9Qdkg3W5WQEQwd0EC7q1qNF-co124_1_wwTfoYZlhvK2npY0TDJg

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the product available is \"ekoDB\". It is a high-performance database product with AI capabilities and it is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"dFh8mRXTlL3TjtbEFv0Hl8CpTXA3B9ibVF9Qdkg3W5WQEQwd0EC7q1qNF-co124_1_wwTfoYZlhvK2npY0TDJg"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"HJILhJ6Mf3f1R-YHq9qDZ1TlHWZ8gux8hOPayzLpxBnCqL_cSXH-__-UUauC3wwVbK40BeBzAz5Xo0BdMXodaQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-06T16:15:29.881428Z"},"id":"qeRuQJzjPmPKmcmiVs3movIbJnFwuPcgUhKP35Ne_heCY4G5uBaew4f8kLLjG78BBGBPO3FiwHIRbbZIufaRLw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":178,"total_tokens":210}},"updated_at":{"type":"DateTime","value":"2026-01-06T16:15:29.881428Z"}},{"chat_id":{"type":"String","value":"dFh8mRXTlL3TjtbEFv0Hl8CpTXA3B9ibVF9Qdkg3W5WQEQwd0EC7q1qNF-co124_1_wwTfoYZlhvK2npY0TDJg"},"content":{"type":"String","value":"Based on the information provided, the product available is \"ekoDB\". It is a high-performance database product with AI capabilities and it is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"HJILhJ6Mf3f1R-YHq9qDZ1TlHWZ8gux8hOPayzLpxBnCqL_cSXH-__-UUauC3wwVbK40BeBzAz5Xo0BdMXodaQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-06T16:15:29.885012Z"},"id":"9BbDxnirUqJ9rvTwx3u96d3Pw_RkWld14o5byoSi41aAo_UR78RScB69Feqj26ZdAdJwWiyy2Ne_6ylj2Q6oeQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":178,"total_tokens":210}},"updated_at":{"type":"DateTime","value":"2026-01-06T16:15:29.885012Z"}},{"chat_id":{"type":"String","value":"dFh8mRXTlL3TjtbEFv0Hl8CpTXA3B9ibVF9Qdkg3W5WQEQwd0EC7q1qNF-co124_1_wwTfoYZlhvK2npY0TDJg"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"HJILhJ6Mf3f1R-YHq9qDZ1TlHWZ8gux8hOPayzLpxBnCqL_cSXH-__-UUauC3wwVbK40BeBzAz5Xo0BdMXodaQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-06T16:15:30.846462Z"},"id":"V1SNpLKyPzccB2Lj1_BFxfcn8MdlaZc91VV7QFnWzAbJM1Sgkn0HMuA-CAuWq2EDov_S9HWKecxjwB9w_ESzkQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":178,"total_tokens":190}},"updated_at":{"type":"DateTime","value":"2026-01-06T16:15:30.846462Z"}},{"chat_id":{"type":"String","value":"dFh8mRXTlL3TjtbEFv0Hl8CpTXA3B9ibVF9Qdkg3W5WQEQwd0EC7q1qNF-co124_1_wwTfoYZlhvK2npY0TDJg"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"HJILhJ6Mf3f1R-YHq9qDZ1TlHWZ8gux8hOPayzLpxBnCqL_cSXH-__-UUauC3wwVbK40BeBzAz5Xo0BdMXodaQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-06T16:15:30.849118Z"},"id":"PNLaxfrzEoWxgfwTUrwcDfN-W8hibeM5C3nupIjjuIprgI8ERoCX61QYIxf3d6YPU11Zq7NH9tDfMItpy8NRPw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":178,"total_tokens":190}},"updated_at":{"type":"DateTime","value":"2026-01-06T16:15:30.849118Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: lkFgescQJwacCUpQCqQvCs20orAXw8SRLn5OeO47NBo6dM7SN401Pn29VzQJAWvOqt_V4nRdP6piC9Js3I9oIQ

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: Record(fields={id=StringValue(value=tgs0PyMhfymeJgZ_7jiqpwriEO0FoJWAf4psHfJAZ89kseYKOghtOPxWEkU5-jjgdGmU3nkG9gnQQTuvlf48bA)})

=== Upsert Operation ===
✓ First upsert (update): Record(fields={error=StringValue(value=Record not found)})
✓ Second upsert (insert): Record(fields={error=StringValue(value=Record not found)})

=== Find One Operation ===
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: false
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 6 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!

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
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: Record(fields={id=StringValue(value=1M4bN_83NwpsuZDoO3qBWC8PBE0OMK4FgJmqLucBySZ3Qyk-m6hRx3AZLeFQEdxa6wlvrECBy60dwoEPJyRIzA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record(fields={id=StringValue(value=2UHwfFreUIjLNElCTqukB5VxcKbOL-xVXEjyl_8yvRzUCgUDG1YKfnDIFYdU4rIxwWzepkXSqEES9YEQu1OPng)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record(fields={id=StringValue(value=1M4bN_83NwpsuZDoO3qBWC8PBE0OMK4FgJmqLucBySZ3Qyk-m6hRx3AZLeFQEdxa6wlvrECBy60dwoEPJyRIzA), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record(fields={price=ObjectValue(value={value=IntegerValue(value=500), type=StringValue(value=Integer)}), id=StringValue(value=1M4bN_83NwpsuZDoO3qBWC8PBE0OMK4FgJmqLucBySZ3Qyk-m6hRx3AZLeFQEdxa6wlvrECBy60dwoEPJyRIzA), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: --aovo4jRi5HWVi8YaFlErjWB54m7C1p4Vw5-WBTgfoANXrRTUBa2Be3NRpbq95xrKycxKTe5tJmuvY1YSxmlw
Created Bob: $500 - ID: nHlIPlx9Bj9AiX_948I04R-3XLqLB9ukVUBakM3iWKZs5XlQZcUhSxqhiJDj4hj6UpG0UfYL0pKhhMnKbGwPqw

=== Example 1: Begin Transaction ===
Transaction ID: afcb8b7e-03e6-4988-a5e4-692a5ad0a304

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: bb25edc3-bcb3-4f07-8665-595542f9f92a
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
    ✓ Generated embedding: 1536 dimensions in 490.391959ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 504.572958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 287.452334ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 209.763084ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 398.472625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 288.644209ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 615.235458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 503.225375ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 370.245167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 274.402042ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 308.598875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 299.113584ms
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
    ✓ Generated embedding: 1536 dimensions in 945.135791ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 100.799917ms

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

Writing memory-safe, high-performance database code involves several practices:

1. **Normalization**: As discussed in context 2 and 3, normalizing your database is important to reduce redundancy and improve data integrity. A well-organized database will inherently speed up many queries.

2. **Choose the Right Database Type**: As discussed in context 1 and 4, the choice between SQL and NoSQL can have a significant impact on performance. If your data is structured and has well-defined relationships, SQL databases can be more efficient. If you have unstructured data, or need high write throughput and horizontal scaling, NoSQL can be more performant.

3. **Indexing**: Indexes can dramatically improve query performance by reducing the number of disk I/O operations. However, indexes require additional disk space and can slow write operations, so it's important to use them judiciously.

4. **Optimized Queries**: Write your queries in such a way as to minimize the amount of data that has to be read from the disk. Avoid using SELECT *, as it requires more memory to store the results. Instead, specify the exact columns you need. Also, use WHERE clauses to filter data as early as possible in the query. 

5. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks, but they can also improve performance by allowing the database to cache the query execution plan and reuse it across multiple instances of the same query.

6. **Connection Pooling**: Establishing a database connection is a time-consuming process. Connection pooling allows you to reuse existing connections, reducing the overhead of establishing a new connection for every query.

7. **Profiling and Monitoring**: Use profiling tools to identify slow queries and performance bottlenecks. This will help you understand where optimizations can be made. 

In terms of memory safety, ensure you are properly closing connections and cleaning up resources after use. Also, avoid loading large amounts of data into memory at once, especially in languages that don't automatically manage memory for you. 

Please note, these are general tips. Depending on the specific database system and programming language you are using, there might be other specific practices you should follow.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 516.738417ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2286 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 506.1ms
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
  ✓ Text search completed in 62.196167ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several practices:

1. **Normalization**: As discussed in context 2 and 3, normalizing your database is important to reduce redundancy and improve data integrity. A well-organized database will inherently speed up many queries.

2. **Choose the Right Database Type**: As discussed in context 1 and 4, the choice between SQL and NoSQL can have a significant impact on performance. If your data is structured and has well-defined relationships, SQL databases can be more efficient. If you have unstructured data, or need high write throughput and horizontal scaling, NoSQL can be more performant.

3. **Indexing**: Indexes can dramatically improve query performance by reducing the number of disk I/O operations. However, indexes require additional disk space and can slow write operations, so it's important to use them judiciously.

4. **Optimized Queries**: Write your queries in such a way as to minimize the amount of data that has to be read from the disk. Avoid using SELECT *, as it requires more memory to store the results. Instead, specify the exact columns you need. Also, use WHERE clauses to filter data as early as possible in the query. 

5. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks, but they can also improve performance by allowing the database to cache the query execution plan and reuse it across multiple instances of the same query.

6. **Connection Pooling**: Establishing a database connection is a time-consuming process. Connection pooling allows you to reuse existing connections, reducing the overhead of establishing a new connection for every query.

7. **Profiling and Monitoring**: Use profiling tools to identify slow queries and performance bottlenecks. This will help you understand where optimizations can be made. 

In terms of memory safety, ensure you are properly closing connections and cleaning up resources after use. Also, avoid loading large amounts of data into memory at once, especially in languages that don't automatically manage memory for you. 

Please note, these are general tips. Depending on the specific database system and programming language you are using, there might be other specific practices you should follow.

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
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.219s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.380s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.298s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.411s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.259s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.438s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.593s
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
    ✓ Generated embedding: 1536 dimensions in 0.593s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.294s
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
    ✓ Generated embedding: 1536 dimensions in 0.304s
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

Writing memory-safe and high-performance database code involves several best practices and guidelines. Here are some:

1. Use the appropriate database: As discussed in Context 1 and 4, choosing the right database for your needs is crucial. If you need to deal with unstructured data, high write throughput, or horizontal scaling, NoSQL databases are typically a better choice. For structured data, complex queries, and ACID transactions, SQL databases are more suitable.

2. Normalize your data: As mentioned in Context 2 and 3, database normalization can help improve data integrity and reduce redundancy. This can lead to more efficient use of memory and better overall performance.

3. Indexing: Use indexing wisely to speed up your queries. But remember, while indexes speed up retrieval times, they slow down write times because the index also needs to be updated. So it's a trade-off which must be managed.

4. Use prepared statements: Prepared statements can be compiled and cached by the database management system (DBMS), which can lead to performance improvements. They also help prevent SQL injection attacks, which can lead to memory safety issues.

5. Limit your data: Do not retrieve more data than necessary. Use LIMIT in SQL, or its equivalent in NoSQL databases, to avoid loading more data into memory than you need.

6. Use efficient queries: As asked in Context 5, optimizing your database queries is crucial for performance. Avoid using SELECT *, use JOIN instead of nested queries, and make use of aggregate functions for calculations instead of retrieving and processing data in your code.

7. Handle memory explicitly: If using a language like C++, manage memory allocation/deallocation explicitly and use smart pointers to avoid memory leaks.

8. Use proper error handling: Ensure that failed database calls do not lead to memory leaks by implementing proper error handling.

9. Connection Pooling: Repeatedly opening and closing database connections can lead to a significant performance loss. Connection pooling allows you to reuse existing connections, reducing the overhead of establishing a new connection for each query.

Remember, the best practices can vary depending on the specific DBMS and programming language you are using. Always refer to their specific documentation for more in-depth guidelines and best practices.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.389s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2354 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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
    ✓ Generated embedding: 1536 dimensions in 0.485s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.389s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.429s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.241s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.513s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.314s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.296s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.399s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.806s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.208s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.388s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.390s
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
    ✓ Generated embedding: 1536 dimensions in 0.415s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.087s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.000] From conv_database_design
     What is database normalization?

  5. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code requires an understanding of both coding best practices and database architecture. Here are some tips based on the context given:

1. Understand the Database System: You need to understand whether you are using a SQL or NoSQL database, as the design considerations and optimization techniques can differ. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships. In contrast, NoSQL is more suitable for flexible schemas, horizontal scaling, high write throughput, and unstructured data.

2. Optimize Your Queries: You can optimize database queries by using indexation, pagination, and limiting the fields returned by the queries. Avoid using "SELECT *" as it can slow down your database by returning unnecessary data. Also, make use of JOINs instead of multiple queries when you need data from multiple tables.

3. Normalize Your Database: Normalization is a process that organizes data to minimize redundancy and improve data integrity. It can help enhance the performance of the database by reducing the amount of data that needs to be read from the disk.

4. Memory Management: When writing code, ensure you're effectively managing memory. Avoid memory leaks by properly deallocating any memory that was previously allocated when it is no longer needed. In languages like C++, this is a manual process, but in languages like Java or Python, this is handled by the Garbage Collector.

5. Use Prepared Statements: Prepared statements not only provide protection against SQL injection but can also improve performance as they are parsed only once but can be executed multiple times.

6. Connection Pooling: Connection pooling can significantly reduce the overhead of creating a new connection every time a database operation is performed. By reusing database connections, you can improve the performance of your application.

7. Caching: Implement a caching strategy to store and retrieve frequently accessed data, which can help increase your application's performance and responsiveness.

Remember, writing high-performance database code is an iterative process and involves regular monitoring and tuning based on the specific needs of your application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.458s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2256 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.257s
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
  ✓ Text search completed in 0.051s
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
    ✓ Generated embedding: 1536 dimensions in 0.289s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.211s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.382s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.654s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.286s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.513s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.381s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.431s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.224s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.402s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.254s
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
    ✓ Generated embedding: 1536 dimensions in 0.407s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.089s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.000] From conv_database_design
     What is database normalization?

  5. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several steps and techniques. Here are some of them:

1. **Optimize Database Queries**: This is an important step for improving the performance of your database code. This can be done through proper indexing, limiting the data that you are querying with more specific WHERE clauses, or using JOINs instead of multiple queries. Also, take advantage of stored procedures, views, and triggers where appropriate.

2. **Database Normalization**: This process helps to reduce data redundancy and improve data integrity, which indirectly contributes to performance enhancement. By organizing data into smaller tables and defining relationships through foreign keys, you can speed up queries and make your database more efficient.

3. **Choose the Right Database Type**: Depending on your needs, either SQL or NoSQL may be more appropriate. If you need flexible schemas, high write throughput, horizontal scaling, or are working with unstructured data, NoSQL might be a better choice. On the other hand, if you require complex queries, ACID transactions, or have structured data with well-defined relationships, SQL is likely the better option.

4. **Memory Management**: Be mindful of the data types you use, as this directly affects memory usage. Use appropriate data types that take up less memory but still satisfy your needs. Also, be careful with how much data you load into memory at once. Try to process data in chunks rather than loading large datasets into memory all at once.

5. **Error Handling and Security**: Properly handle database errors and exceptions to prevent memory leaks and ensure the stability of your application. Also, use parameterized queries or prepared statements to avoid SQL injection attacks, which can compromise your database.

6. **Use Caching**: Caching can help to reduce the load on your database and make your application faster. By storing the results of common queries in memory, you can avoid having to execute those queries repeatedly.

By following these practices, you can write memory-safe and high-performance database code.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.339s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2127 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.818s
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
    ✓ Generated embedding: 1536 dimensions in 0.383s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.578s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.294s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.35s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.356s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.608s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.454s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.501s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.238s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.284s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.332s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.284s
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
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.083s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.000] From conv_database_design
     What is database normalization?

  5. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.704s
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
  ✓ Text search completed in 0.047s

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
✓ Created SWR script: fetch_github_user (pgDIoQp3JunUq2c8_iTkxBojj2Ow03XoQ-lFtD8RSC3CHVetlxdrxqeSjhVr2oPD0p8mFHDEl7kcsd-NqOBuJw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T16:11:54.462Z",
      "data": {
        "value": {
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "user_view_type": "public",
          "created_at": "2011-09-03T15:26:22Z",
          "html_url": "https://github.com/torvalds",
          "public_gists": 1,
          "type": "User",
          "url": "https://api.github.com/users/torvalds",
          "company": "Linux Foundation",
          "following": 0,
          "hireable": null,
          "location": "Portland, OR",
          "updated_at": "2025-11-24T04:16:14Z",
          "followers": 270300,
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "gravatar_id": "",
          "login": "torvalds",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "email": null,
          "bio": null,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "id": 1024025,
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "blog": "",
          "name": "Linus Torvalds",
          "public_repos": 9,
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "site_admin": false,
          "twitter_username": null
        },
        "type": "Object"
      },
      "id": "cUDmZ_PbM9MzTMbKhN24KpKlnhiWvMf5vAw3x5dsWh6mubPWe69kw1U4iBI2-y3lFFwwJDEMCsarPGApzxPyxA"
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
Response time: 76ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (jpolsrPDuwcGcgmC76frgAdX8SAaZTz8uIPQI-zZfdxpclcJiXQ7mQPQe-lnKa5Tmy61jZWoXWafhEyFXHfTfg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "id": "gs68D2t2ai9wKb90oQNSWQSLrrkvo52bb_OfnPBY81b0xRq2txGSoZSUJ7h4CexvTI5Uj5_b-MccSxPf5HzVSw",
      "enriched_at": "2026-01-06T16:17:37.271Z",
      "enriched_data": {
        "type": "Object",
        "value": {
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "sku": "BEA-ESS-ESS-001",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "discountPercentage": 10.48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "meta": {
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "createdAt": "2025-04-30T09:41:02.053Z"
          },
          "minimumOrderQuantity": 48,
          "reviews": [
            {
              "rating": 3,
              "comment": "Would not recommend!",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon"
            },
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            }
          ],
          "brand": "Essence",
          "id": 1,
          "availabilityStatus": "In Stock",
          "warrantyInformation": "1 week warranty",
          "returnPolicy": "No return policy",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "title": "Essence Mascara Lash Princess",
          "rating": 2.56,
          "category": "beauty",
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "shippingInformation": "Ships in 3-5 business days",
          "stock": 99,
          "weight": 4
        }
      }
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
✓ Edge cache script created: t4MTV5kz8kFZ9MygmrntryJfNIZYeyWKH9JCIkKP9iAvkg5OS_UdfNlojfDOEBMwp2uVRD_D3CLSnFJwcCWa8w

Call 1: Cache miss (fetches from API)
Response time: 82ms
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T16:17:37.670Z",
      "data": {
        "type": "Object",
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "username": "Bret",
          "id": 1,
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874"
          },
          "name": "Leanne Graham"
        }
      },
      "id": "9KGNlS87rqwzME8n1FRvPuOKJAoFx50A8cWC7LQBHXbZ5IwtGj7fXUbIkp8iN-DChKdBh7d2PftoSdgR2iUL9A"
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
Response time: 62ms (1.3x faster!)
Result: {
  "records": [
    {
      "data": {
        "value": {
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light"
          },
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "website": "hildegard.org"
        },
        "type": "Object"
      },
      "cached_at": "2026-01-06T16:17:37.670Z",
      "id": "TLMXl475jAKqut_FrSeHtWfdoCtEh9EQlipmEDrY00gB3JEoOtL8ho3PS6QyZmwbA8A148nfFhhUAP-GVZMLOw"
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
    Finished `release` profile [optimized] target(s) in 0.09s
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
✓ Created SWR script: fetch_github_user (f2FVhwdZUTgPnDsjmUFA0WiVAkTnnMakR-7Fyya6fD1Q-ag-zIJQ_g0oc26FyrRSmRSaZExde_REVbVsSbmO4w)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 121ms
Result: [
  {
    "cached_at": "2026-01-06T16:11:54.462Z",
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
        "followers": 270300,
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
    "id": "vjVny7BVMGvo87RhBb3rtncXOkGxLXtmGSYqCahKy7uGa01q76jvoo1NTnRwFugHylKA4EST124lqfRTzVvX2g"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 84ms (1.4x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (wHy2k8bYC5650OLPDrvNrXlISuF0HtKCDNszgeF6eiC1w1I_RWLCwV1Rtid9I6oW_H_a3UK6GJOKV2-YIoUMVA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767716259",
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
    "id": "t1U55imEELr9gKGJTTLGv_5HVOdAnOolkExm-mxx5-V-Q5HFUNzEffFYlCYh9mqdz-jHOK9PKa5C5IHVuhC-dA"
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
✓ Created SWR script: fetch_github_user (J_sbI8cy_bQFKxZdlTih4LksD9z6UnkPWQPH5oViw6dXhq__NAvWS_gC49Sf6Zxa360xn077rM4rGb-voxE_LA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 85.235125ms
Result: [
  {
    "cached_at": "2026-01-06T16:11:54.462Z",
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
        "followers": 270300,
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
    "id": "d39uFqzHtHhFQmDSldFHoGlxBgNpxkmlggV3vQHxjJdYvqGWf_06z4doISl_DTjJsXOclVV28-bgakUhWmPJeQ"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 75.922042ms (1.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (7Yy3yHLGQ-xKx7o-27HxjdCWtG8s5Jd0LZe9Q64SqyeLXrIyxqJcgNMYyXI6R5ruDuqtWzwkWnRg32mb-TyYHg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "2026-01-06T11:17:40-05:00",
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
    "id": "ZVOS80z5uy5vWwOn8yNxg1IUqirZqKbgH6aXc-3psukB28zgaeIiWhaMlHqj6n1CHMsb5NM6GjTsG0qzgtrfCg"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (fIBfcgJeRPM7nNCOezxtOOFEEJCTe9jVnh7UrF2psPic3z8YIcorg9-ArlLcFC7NAL6fh57zeZtsI-GmG9cKAg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 79ms
Result: {
  "id": "sxCizUfV58fXbpoh22RzFHSdE64Fk2tChmrRoTP4RWXFT0snzCAcb410lFJSbBfDm-ieqbSPIQLysHmifi4yaQ",
  "cached_at": "2026-01-06T16:11:54.462Z",
  "data": {
    "type": "Object",
    "value": {
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "hireable": null,
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "type": "User",
      "email": null,
      "followers": 270300,
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "gravatar_id": "",
      "bio": null,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "url": "https://api.github.com/users/torvalds",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "id": 1024025,
      "location": "Portland, OR",
      "following": 0,
      "login": "torvalds",
      "public_repos": 9,
      "created_at": "2011-09-03T15:26:22Z",
      "site_admin": false,
      "company": "Linux Foundation",
      "name": "Linus Torvalds",
      "blog": "",
      "twitter_username": null,
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "user_view_type": "public",
      "public_gists": 1,
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "updated_at": "2025-11-24T04:16:14Z",
      "html_url": "https://github.com/torvalds"
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 97ms (0.8x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (S_m0MFxdT1sEOeip0WJQ6SaMMjH9hHFqyvc9Eoj8cgvHTcxkBsC8ExaQQ-FyWS5-MDQTDZUTrcIDbb29JrE2eQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_at": "2026-01-06T11:17:40-05:00",
  "enriched_data": {
    "type": "Object",
    "value": {
      "rating": 2.56,
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "warrantyInformation": "1 week warranty",
      "id": 1,
      "stock": 99,
      "availabilityStatus": "In Stock",
      "title": "Essence Mascara Lash Princess",
      "dimensions": {
        "depth": 22.99,
        "height": 13.08,
        "width": 15.14
      },
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "sku": "BEA-ESS-ESS-001",
      "shippingInformation": "Ships in 3-5 business days",
      "returnPolicy": "No return policy",
      "category": "beauty",
      "reviews": [
        {
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "rating": 3,
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins"
        },
        {
          "rating": 4,
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Very satisfied!",
          "reviewerName": "Lucas Gordon"
        },
        {
          "rating": 5,
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Highly impressed!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "reviewerName": "Eleanor Collins"
        }
      ],
      "weight": 4,
      "brand": "Essence",
      "price": 9.99,
      "minimumOrderQuantity": 48,
      "meta": {
        "barcode": "5784719087687",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "updatedAt": "2025-04-30T09:41:02.053Z"
      },
      "discountPercentage": 10.48,
      "tags": [
        "beauty",
        "mascara"
      ]
    }
  },
  "id": "0YhvW-j_xd6Ny4_cnrmlPxGY_z-0d3R9Kp52xXEBj4mFy2UkW0O_qOFPjwGstEBDfksrLVD0uS3TNU9plzGkBg"
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
✓ Created SWR script: fetch_github_user (qY2Wjay32QbR5cONDyvbwC_xI4Q4CR3n1LzhpR1IvElvc9makEKnK8yA9VKaQ4D-mugsXbSdBlRq-81YNto-4Q)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 95ms
Result: [{"id":"7BX1o-81X6Il3XXHgDWqXjz4_kuX8M14vgAgLZ4bWHwUDmMsFvrBVGh5ebO0LoFJCaQsBPdzjZ3719aEuVjsYg","cached_at":"2026-01-06T16:11:54.462Z","data":{"value":{"email":null,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","html_url":"https://github.com/torvalds","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","followers":270300,"name":"Linus Torvalds","site_admin":false,"public_gists":1,"following_url":"https://api.github.com/users/torvalds/following{/other_user}","gravatar_id":"","following":0,"organizations_url":"https://api.github.com/users/torvalds/orgs","followers_url":"https://api.github.com/users/torvalds/followers","public_repos":9,"starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","events_url":"https://api.github.com/users/torvalds/events{/privacy}","user_view_type":"public","repos_url":"https://api.github.com/users/torvalds/repos","twitter_username":null,"subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","url":"https://api.github.com/users/torvalds","login":"torvalds","company":"Linux Foundation","blog":"","id":1024025,"received_events_url":"https://api.github.com/users/torvalds/received_events","created_at":"2011-09-03T15:26:22Z","location":"Portland, OR","node_id":"MDQ6VXNlcjEwMjQwMjU=","bio":null,"type":"User","updated_at":"2025-11-24T04:16:14Z","hireable":null},"type":"Object"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 80ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (B2ggaghc25-KAIv0CTsukadMuoA2Exs8N0xRzUi5_TpzsJUlUqW5XLshQy4uXZr3PHiScjM_cMMFsucWey8Qvg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"id":"6Ndj2tGsW02-AUknVKsq9msiBdzzQeAOlZnvn0II54fYygiLn8aaLiFNwR-QoLxr65eAdbV3w3fJEpUg23h4Uw","enriched_data":{"type":"Object","value":{"shippingInformation":"Ships in 3-5 business days","id":1,"price":9.99,"returnPolicy":"No return policy","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","availabilityStatus":"In Stock","sku":"BEA-ESS-ESS-001","title":"Essence Mascara Lash Princess","discountPercentage":10.48,"stock":99,"minimumOrderQuantity":48,"dimensions":{"height":13.08,"width":15.14,"depth":22.99},"warrantyInformation":"1 week warranty","weight":4,"tags":["beauty","mascara"],"reviews":[{"date":"2025-04-30T09:41:02.053Z","rating":3,"reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","comment":"Would not recommend!"},{"reviewerName":"Lucas Gordon","date":"2025-04-30T09:41:02.053Z","comment":"Very satisfied!","reviewerEmail":"lucas.gordon@x.dummyjson.com","rating":4},{"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","comment":"Highly impressed!","rating":5}],"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","meta":{"barcode":"5784719087687","updatedAt":"2025-04-30T09:41:02.053Z","createdAt":"2025-04-30T09:41:02.053Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png"},"rating":2.56,"category":"beauty","brand":"Essence"}},"enriched_at":"1767716268405"}]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   Name: Some(Object({"type": String("String"), "value": String("User 1")}))
   Department: Some(Object({"type": String("String"), "value": String("engineering")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 88.03975ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 86.534042ms
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 85ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 88ms
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
   ⏱️  Duration: 79.8ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 84.2ms
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
   ⏱️  Duration: 76.808708ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82.254125ms
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
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 84ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 92ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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

