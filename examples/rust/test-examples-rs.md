make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("QTe9kjvAj7Os2snR-BlO4rfQA0Vg6JppqisQmw16ztsehZ-tSYm76zlhxyc3hOBCBQkEkSevvKc68GWxa12ePg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("QTe9kjvAj7Os2snR-BlO4rfQA0Vg6JppqisQmw16ztsehZ-tSYm76zlhxyc3hOBCBQkEkSevvKc68GWxa12ePg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("QTe9kjvAj7Os2snR-BlO4rfQA0Vg6JppqisQmw16ztsehZ-tSYm76zlhxyc3hOBCBQkEkSevvKc68GWxa12ePg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("QTe9kjvAj7Os2snR-BlO4rfQA0Vg6JppqisQmw16ztsehZ-tSYm76zlhxyc3hOBCBQkEkSevvKc68GWxa12ePg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "nrFky7TUpLwkxOlhaCDSUe0PCao8kfz0iZYaDndFzDJ96Kl38Z2qDWBwYQYTAPkaH5E4IT8NKcJugA_gXfF4vA"

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
        "id": "nrFky7TUpLwkxOlhaCDSUe0PCao8kfz0iZYaDndFzDJ96Kl38Z2qDWBwYQYTAPkaH5E4IT8NKcJugA_gXfF4vA",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "qtJj7Sfr_pNjxPUaT46__kAoE1IhVtrVhFS6ndpAQHnEbG7PWGjEF-G7f38c1-xcJuTHSdW2C6V2JvPYaYF2YA"

=== List Collections ===
Total collections: 5
Sample collections: ["demo_collection", "batch_users", "audit__ek0_testing", "websocket_test", "test_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "WfewFnDwkaJ1Jg5bsxfu6nK-8TG5mlejZPdz0j7pWK0GrMW_GZ1-Vj8LgQ8Otw2mAVc9Y8A9XsI9yv8lrr0EWg"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "1z7fcAGuPH8U3WCwbd97TfuPRFeeOSPvVpAmuXJYhuztsUsSTaBTIcp0X5btJAbskcTpshyKS1nmxr-xY2vahg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "AuOyj9bEWBNOaQvL2aWeTXE0EhUbpuOUnBU4W_6MjR5FdEs4zzEWfoufLsBO3zNYGbW_dy9u7UoMrXPNn2sofg"

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
✓ Inserted document with TTL: String("NGi9vyyjRAOPbDznNp3hQUcT1jeTscA-QHTlMxaiNO2Sk1X7ex35OTkGWKOKBdTBcI-DFplcck9t5q5qyjJEUw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "NGi9vyyjRAOPbDznNp3hQUcT1jeTscA-QHTlMxaiNO2Sk1X7ex35OTkGWKOKBdTBcI-DFplcck9t5q5qyjJEUw"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-03-22T17:44:48.135594Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: -MKEcsIIIdlfemv8Lqp-9lk119Vl0ol_pCy4rryrIdZBJ7_Trrv0_NqC9QVEpf7S_AZb2kyXblZBE50wzt8b6w
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "-WKOg69-f7_QxUcyZ__ABVsLQ5ziyw-Iwgt1lnm_e86GSkRrizgCsTXTfitdpdFBNxHVbooasYX-UVgfnlcbiQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: Iqb-GvH63a-6gU0xltZfRVtgiboI05dGtBYeMc-VLpHXiJZwO6oXYq0jQ5y65-6RvyZJQzvPc6Ygp-I_Fa44ng
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: qzcoryJLq_L-txQK0xBYiHZXWALpv8E3i2npbteTkiTnhHsOhc-c5c5Wghyk9K9lU_2XjXo_7Ux9uOTUj4eKBA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: FT2IQ5LiRakTdt6KvluAwDR7FuiD_qAmgb3Js6tHMRgD-cM4HL17xd3KMJ70xdEAJ3r8QUSuMLmO3D-8SflQNg
Created Bob: $500 - ID: MK9Rg7gjP3Tkzc9yX1mZzTXXoqo0v1949bObfJfC3SvgaBgTa9y6SBQB_8H6YJT18HL8G17X49h0nWdRE1v6sA

=== Example 1: Begin Transaction ===
Transaction ID: 0bb9530d-07fc-4b02-ab87-521b8defa8ce

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: 890c57e2-071d-403a-80dc-1b60b477eeaf
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("G_Bzq2dYZ_UpBMNPHnDuIIcEIh1RpAAJrgGPl7OqdSZMRPcGRcwPWOt4TbuHx-BG3hhV7EzdeGN911oWAE7A_A")})

=== Find by ID ===
Found: Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "id": String("G_Bzq2dYZ_UpBMNPHnDuIIcEIh1RpAAJrgGPl7OqdSZMRPcGRcwPWOt4TbuHx-BG3hhV7EzdeGN911oWAE7A_A"), "created_at": Object({"value": String("2026-03-22T16:44:52.250910Z"), "type": String("DateTime")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-03-22T16:44:52.250910Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"value": String("2026-03-22T16:44:52.250910Z"), "type": String("DateTime")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "id": String("G_Bzq2dYZ_UpBMNPHnDuIIcEIh1RpAAJrgGPl7OqdSZMRPcGRcwPWOt4TbuHx-BG3hhV7EzdeGN911oWAE7A_A"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})})})]

=== Update Document ===
Updated: Record({"price": Object({"value": Float(99.99), "type": String("Float")}), "id": String("G_Bzq2dYZ_UpBMNPHnDuIIcEIh1RpAAJrgGPl7OqdSZMRPcGRcwPWOt4TbuHx-BG3hhV7EzdeGN911oWAE7A_A"), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "created_at": Object({"value": String("2026-03-22T16:44:52.250910Z"), "type": String("DateTime")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: F7xo-wlH7VS0MD2Q9fsOCbCH8JHQDFP1ljdo4sdj70V11EVbtx_x0P7kL7Sx0OZAgMdyVXsyeLWVj4LmBA_igw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  value: 42
  name: "WebSocket Test Record"
  id: "F7xo-wlH7VS0MD2Q9fsOCbCH8JHQDFP1ljdo4sdj70V11EVbtx_x0P7kL7Sx0OZAgMdyVXsyeLWVj4LmBA_igw"

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "H0p4rs7Cs4sYK8-1TUjW-kbMJsZtJvO8DXGZoYe8s-cH9dCANprClyOamVnq1gfTbu2r5bWUZk8YcMTz3yM1DQ"

=== List Collections ===
Total collections: 10
Sample collections: ["batch_users", "client_collection_management_rust", "users", "scripts__ek0_testing", "audit__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"userId": Number(123), "username": String("john_doe")}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: Record({"name": String("Product 1"), "price": Float(29.99)})
  cache:product:2: Record({"name": String("Product 2"), "price": Float(39.989999999999995)})
  cache:product:3: Record({"name": String("Product 3"), "price": Float(49.989999999999995)})

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Hi35L82ak83kI0xtNd8tvK1cXTQWMSIPZ2AyTomq-qlUE4iWJRXxISmE8AhHjQiiiihlHMKDlDYm-M8xk6iepw
Created Bob: $500 - ID: vdxFMAHCj5mvqMtaZlEM3gt4QQ0wMYrwyEdi9wX78Cq5zLiK2tcK3DqaUl92V-yVcewHRxRVYv3xZF-3vXhvIQ

=== Example 1: Begin Transaction ===
Transaction ID: f6721dc2-af32-4e0f-b8a5-0f5aba0efaba

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 2cfa0232-0985-43ba-a282-3f1ade9a2f0c
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

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
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["title", "description", "description.value", "title.value"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description", "description.value"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  2. Score: 3.3000 - Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Database Design")})

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("cv3s5CTqnx7a5fjAFgZTcdbApj4M9SLkcKKsEj53JrH9v6JZBzfhnzXMuMbxev-dr8YGm1Zm9e1qDHACBVirBg"))
✓ Inserted user 2: Some(String("9EFBBOtSJMyOVtz2AffxrAc-zGwLFcnkgHRXRUm5MdFyigpb5KQ9VChZOYbBcbQjG-M-xgY8-VqqBIFwwQax_Q"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - email: String
    (required)
  - title: String
    (required)
  - age: Integer
  - status: String

=== Listing Collections ===
✓ Total collections: 10
  Sample: ["batch_users", "users", "scripts__ek0_testing", "schema_client_rust", "audit__ek0_testing"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "2Eg4V-f3WaBg6nCEZNKV07RbKwi2jPiSrTBIKxtl_RXaMk5IwiQD6fqbEXj5sDj4KqBwEK1m4_rU2ol9QKtSLA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("zX9akCZZNstUaGccRlKAV3uOFJg0ewOkLsx7U7fG_3af0OOkRJYQPUzeIm1eZw2d6i8byxOoaEhCANWJ1BvvdQ"))

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
✓ Inserted document with TTL: Some(String("-YlkdLBfE-hFEhiiZpeBeHNVDWZ0V3qkTRofkisJf84FHgyaAWwFUuAxChyStmiE3Thf1cXjxyYn0bGtTDEojg"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: xoufeXlz2nRgaWA-0Ed4Gv9COZsNYw3iTM-90XiT918rSIAb0RckgXyjqclnLnUKQfFUQFe6wWT38dxtpt55uQ

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: EywVD4M9dfB9bmX1QyT80p7xwgy4hl7jdsP7Ql5pP2GiR6qTq29zYEn63BreIY81nz_pET0fw2KUsKkFsms03Q
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: far2z3NN1n9j3vSbCQGpdtcYrc0rBXsHScAoQ72ZeQEWtNuGAnGkJkiwGKkG26AqroMa5xD8XMYZ9Z7h3k58iQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: xHHiP9UKtgt6hyTvun3iE7RUVVv27yhzIkGjNfGLg1bmqUEEEot-1czhiGuNnbDGCsEnE5ueWMOtDFPGilmGXw
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 117.538125ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "username": "Bret",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "geo": {
        "lat": "-37.3159",
        "lng": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.95575ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "id": 1,
    "phone": "1-770-736-8031 x56442",
    "address": {
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        "lng": ...
   🚀 Cache speedup: 117.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ All composition examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 2TkD5XpUyB7iK1lpq1VVvqbFp3xMZ5pwmmE-xgn_8KqCMgYzYuj5kh0AB4-nEfucq27gEgCdwrVr0dnVHTsKBg
📊 Found 2 product groups
   Record({"avg_price": Float(575.6666666666666), "category": String("Electronics"), "count": Integer(3)})
   Record({"category": String("Furniture"), "count": Integer(2), "avg_price": Float(474.0)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"category": String("Electronics"), "count": Integer(3)})
   Record({"category": String("Furniture"), "count": Integer(2)})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("SngR_hF3Zcs59QE4fjR77wvas-lR_-XiIVsJi0jpn3LiYU-uuhEpG0fKGkdveKasfrleCe_eW893VBfKZLk-Fg"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 8gdfKLxk94ZkKLLMNf5q1LEg894mt8wkhDHi4RAbMRy4u-IKUAR8gt8_fzbIQ3jvT-ifaZZXQxE6P-yyRNw7HA
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 3y-EQjMbgbpFes-oX9fCCgFOslcNxLB_UTMUPkrcxVuqrQ-zmQB4Cz6VJYGAmEIOqF3PCGNwMbdPsTTd8dstJw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: oZyEiTOeZgh17XK3loAI5S2YZbsv1YRebryTGR3NwjBDs7lm1XIJWv2f3ZjKqk5Go5u5u98XdUhaG1wip11E2Q
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (-Tv60yjzMT4RL_GiECdj-AXaHvGtTgbaB-jQZ30KbeDFsACZByj4eg_SkPh8IPjXVq2yCTSkWwZyxIQKUXIvFQ)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (Sgm0RXbvlGXG3Enx_98ymOWoL7AsIOzDAHnX69x1YGTNc7VpVl_WrJ0Zay4_QZacj4uHdGMx8PB6miJP1IVcyA)

First call (cache miss - will fetch from GitHub API):
  Response time: 137ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 45.7x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (N38JWpvvH9MhjdrjYwub9wdUrRJ80iRhkT-gE5imRjeSUsKxVRIlnrvTJSbycLgrVxPUw9nMCgTkiaJj1ZtFYg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (mzEmH5-YvzcwLwapBqDEJoBz8jo44iNNLrF98HyLW3UIl-sRysvrEVOuN_pd3DX6_mjDzuowHxuEOO6_HRKnLw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (pnr1m9ohA9HFOf6MPZ_68EvUviJwBMjDmGHVoRYf8OKMwUPNItKPy7_MzJJOrzsXDcdw_Zz0YkksVQDtK8d76g)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

=== Key Benefits of Native SWR Function ===
✅ Simpler: One function instead of 4 (FindById → If → HttpRequest → Insert)
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Old pattern (manual):  ~4 function calls, complex script logic
New pattern (native):  ~1 function call, handled by server
Result: Simpler code, faster execution, easier maintenance

🧹 Cleaning up test data...
✓ Cleanup complete

=== Summary ===
The native SWR function provides a production-ready cache-aside pattern
with minimal code and maximum flexibility. Use it for:
  • API gateway caching
  • External API aggregation
  • Microservice response caching
  • Edge computing patterns
  • Real-time data enrichment pipelines

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   Record({"count": Integer(5), "category": String("Electronics"), "avg_price": Float(367.0)})
   Record({"count": Integer(3), "category": String("Furniture"), "avg_price": Float(365.6666666666667)})
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
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in finding similar items based on vector embeddings, making them ideal for applications like image recognition, recommendation systems, and natural language processing.

2. **High-dimensional Data Handling**: Vector databases can manage and query high-dimensional data effectively, which is essential for modern AI and machine learning applications.

3. **Scalability**: Many vector databases are designed to scale horizontally, allowing them to handle large volumes of data and user requests.

4. **Performance**: They often provide optimized indexing structures (e.g., HNSW, Annoy) that enable fast query responses even with massive datasets.

5. **Rich Querying Capabilities**: They support more complex queries beyond basic filtering, enabling advanced analytics and insights.

6. **Integration with ML Models**: These databases can seamlessly integrate with machine learning workflows, facilitating the storage and retrieval of model outputs and features.

7. **Multi-modal Data Support**: They can handle various data types, including text, images, and audio, enabling a more unified approach to data storage and retrieval. 

Overall, vector databases are tailored for modern applications requiring efficient and scalable solutions for high-dimensional data.
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
   Record({"count": Integer(7), "status": String("active")})
   Record({"count": Integer(3), "status": String("inactive")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

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
   Record({"category": String("AI"), "count": Integer(2)})
   Record({"count": Integer(3), "category": String("Database")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: E9uFMil8cwvzc9-nF4LPb4OSS-fx9MQ99SOPTRnCswWbl12QCwpijCRClkBNTKD-B96wOOJD8F2BKrpqz-ZFqQ

=== Sending Chat Message ===
Message ID: E2m_k0hjQV73_1EeYQa11ve4yi4nrO5BZ_wOmkokQc-Pat41ioDAVUqFzK5ayz7qc2QopaPcdfvAYNnIk3fmXA

=== AI Response ===
Response 1: ekoDB is a high-performance database that incorporates intelligent caching, real-time capabilities, and AI integration. It is designed to allow users to interact with their data using natural language queries and provides AI-powered responses along with relevant context.

### Key Features of ekoDB:

1. **AI Chat Integration**: Enables users to query the database using natural language, receiving AI-driven responses that include relevant context.

2. **Search Features**: Supports various types of searching, including full-text search, vector search, and hybrid search, all with automatic context retrieval.

3. **Real-time Capabilities**: Offers real-time interaction with data, enhancing user experience and responsiveness.

If you need more detailed information about any specific feature or aspect, feel free to ask!

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("hR50wj8etBVXyFVmZbhhpFz1qdlvrUD1ZQZmuYSwPwI_yoJa6C7LuBwmt9xYViWDh5qpxl28hysuZCmUOPjT5g"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("riPVuguPYuI_kg_rSDhFk8weCJ1PpZjz17pRNfS1wVCfD3elq08TN1u0pYc4mWSnCfpEwH-TOCkQ9fZnGX2wNQ"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("LnsEwKOcoHhREwETiXVNkPNzkWv5_SD1Iuw2t2YaHUSc6kJHfgWt3D2SECGxpbHZd3sNHxHHBIOdwD5ztvHluQ"), "title": String("Introduction to ekoDB")}

Execution Time: 3661ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: XzzNYDZB8nY2Zj-szI_pud6zHL2pSXDlP5QHdgrlbJh-zoCPl-76Rhl8EyQGr6utr5RaBKK7ajRnmu9crWziWg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
  - **Description**: High-performance database product
  - **Price**: $99

If you need more details or additional products, feel free to ask!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 5dn35TDiTniZ5gJZM68VR2VT4wVPUIr6m7UxcTtX0hXVLbCj94a8s8EGMfqxrtyzhxeGW49-fvTXjINDcJFdQw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 6

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 5

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: jFsyq9FBwczTrWifSkqCWnKmY1bYI2q4Uq7YbC71Y3GrRBGN9CE_VoxGT2U-JOPLF0JvBH_R9EZl5r2_DbrCWg
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more information or have further questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: TWnmAkCkO01Q8pWTfAwiIUz3O_l-wcNSX3QRODZLpsWohXLN_ba9kYrkRb1DctBpzZAKemgDhaYRRx2v6x8Ijw
  Parent: jFsyq9FBwczTrWifSkqCWnKmY1bYI2q4Uq7YbC71Y3GrRBGN9CE_VoxGT2U-JOPLF0JvBH_R9EZl5r2_DbrCWg

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: TWnmAkCkO01Q8pWTfAwiIUz3O_l-wcNSX3QRODZLpsWohXLN_ba9kYrkRb1DctBpzZAKemgDhaYRRx2v6x8Ijw (Untitled)
  Session 2: jFsyq9FBwczTrWifSkqCWnKmY1bYI2q4Uq7YbC71Y3GrRBGN9CE_VoxGT2U-JOPLF0JvBH_R9EZl5r2_DbrCWg (Untitled)
  Session 3: 5dn35TDiTniZ5gJZM68VR2VT4wVPUIr6m7UxcTtX0hXVLbCj94a8s8EGMfqxrtyzhxeGW49-fvTXjINDcJFdQw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: TWnmAkCkO01Q8pWTfAwiIUz3O_l-wcNSX3QRODZLpsWohXLN_ba9kYrkRb1DctBpzZAKemgDhaYRRx2v6x8Ijw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (125):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4-mini
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
  - dall-e-3
  - dall-e-2
  - gpt-4-1106-preview
  - gpt-3.5-turbo-1106
  - tts-1-hd
  - tts-1-1106
  - tts-1-hd-1106
  - text-embedding-3-small
  - text-embedding-3-large
  - gpt-4-0125-preview
  - gpt-4-turbo-preview
  - gpt-3.5-turbo-0125
  - gpt-4-turbo
  - gpt-4-turbo-2024-04-09
  - gpt-4o
  - gpt-4o-2024-05-13
  - gpt-4o-mini-2024-07-18
  - gpt-4o-mini
  - gpt-4o-2024-08-06
  - gpt-4o-audio-preview
  - gpt-4o-realtime-preview
  - omni-moderation-latest
  - omni-moderation-2024-09-26
  - gpt-4o-realtime-preview-2024-12-17
  - gpt-4o-audio-preview-2024-12-17
  - gpt-4o-mini-realtime-preview-2024-12-17
  - gpt-4o-mini-audio-preview-2024-12-17
  - o1-2024-12-17
  - o1
  - gpt-4o-mini-realtime-preview
  - gpt-4o-mini-audio-preview
  - o3-mini
  - o3-mini-2025-01-31
  - gpt-4o-2024-11-20
  - gpt-4o-mini-search-preview-2025-03-11
  - gpt-4o-mini-search-preview
  - gpt-4o-transcribe
  - gpt-4o-mini-transcribe
  - o1-pro-2025-03-19
  - o1-pro
  - gpt-4o-mini-tts
  - o3-2025-04-16
  - o4-mini-2025-04-16
  - o3
  - o4-mini
  - gpt-4.1-2025-04-14
  - gpt-4.1
  - gpt-4.1-mini-2025-04-14
  - gpt-4.1-mini
  - gpt-4.1-nano-2025-04-14
  - gpt-4.1-nano
  - gpt-image-1
  - gpt-4o-realtime-preview-2025-06-03
  - gpt-4o-audio-preview-2025-06-03
  - o4-mini-deep-research
  - gpt-4o-transcribe-diarize
  - o4-mini-deep-research-2025-06-26
  - gpt-5-chat-latest
  - gpt-5-2025-08-07
  - gpt-5
  - gpt-5-mini-2025-08-07
  - gpt-5-mini
  - gpt-5-nano-2025-08-07
  - gpt-5-nano
  - gpt-audio-2025-08-28
  - gpt-realtime
  - gpt-realtime-2025-08-28
  - gpt-audio
  - gpt-5-codex
  - gpt-image-1-mini
  - gpt-5-pro-2025-10-06
  - gpt-5-pro
  - gpt-audio-mini
  - gpt-audio-mini-2025-10-06
  - gpt-5-search-api
  - gpt-realtime-mini
  - gpt-realtime-mini-2025-10-06
  - sora-2
  - sora-2-pro
  - gpt-5-search-api-2025-10-14
  - gpt-5.1-chat-latest
  - gpt-5.1-2025-11-13
  - gpt-5.1
  - gpt-5.1-codex
  - gpt-5.1-codex-mini
  - gpt-5.1-codex-max
  - gpt-image-1.5
  - gpt-5.2-2025-12-11
  - gpt-5.2
  - gpt-5.2-pro-2025-12-11
  - gpt-5.2-pro
  - gpt-5.2-chat-latest
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-4o-mini-transcribe-2025-03-20
  - gpt-4o-mini-tts-2025-03-20
  - gpt-4o-mini-tts-2025-12-15
  - gpt-realtime-mini-2025-12-15
  - gpt-audio-mini-2025-12-15
  - chatgpt-image-latest
  - gpt-5.2-codex
  - gpt-5.3-codex
  - gpt-realtime-1.5
  - gpt-audio-1.5
  - gpt-4o-search-preview
  - gpt-4o-search-preview-2025-03-11
  - gpt-5.3-chat-latest
  - gpt-5.4-2026-03-05
  - gpt-5.4-pro
  - gpt-5.4-pro-2026-03-05
  - gpt-3.5-turbo-16k
  - tts-1
  - whisper-1
  - text-embedding-ada-002

Anthropic models (9):
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805
  - claude-opus-4-20250514
  - claude-sonnet-4-20250514
  - claude-3-haiku-20240307

Perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

=== Get Models for Specific Provider ===

openai models (125):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4-mini
  - gpt-5.4
  ... and 120 more

anthropic models (9):
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  ... and 4 more

perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: b5BqhUI1E7i_MLQpvijWXoVED_mAp7oOXXhMm0xXL1OpVNZkTaaRlqusgkQIgeO0FDlR0wqVoG-zlq4900P46w

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_active_users_rs: Get Active Users
  - swr_user: SWR pattern for user data (KV-based)
  - get_users_by_status: Get Users By Status
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users_paginated: Get Active Users (Paginated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("aUbWeOUXYlVPySx4wWgG7P2VJXlqCQmgVYyHKYaM-hsjxGyLnn_70-bbxeRKtTZCOMQHZAGwfY5eFvoKU_5NQw")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("56vkO2VJ7uJ23IA7jpGFtBW1Vv149wTk8-jQ2HWDarCQShatk6cN_vdr0E85OCiB8hMa_JmrJdm3Sp1tEZs9XA")})
✓ Second upsert (update): Record({"id": String("t7aNsBG520R71E7-IYPOBmqH3zfmGGQCDYIGKJfKEufeo3tXwFDON7p3DGO6Yc-ygeUnsfW1f_b5gmVdq7_0qA")})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"type": String("String"), "value": String("Alice Johnson")}), "id": String("aUbWeOUXYlVPySx4wWgG7P2VJXlqCQmgVYyHKYaM-hsjxGyLnn_70-bbxeRKtTZCOMQHZAGwfY5eFvoKU_5NQw"), "email": Object({"value": String("alice@example.com"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "age": Object({"type": String("Integer"), "value": Integer(28)})})
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 8 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("UJX2w-koSafF5ep9AcfUOHsmAMWnR4Kepao9Rmv29nsmMWlA3f2XPPgPUZjBXv8EzyUx3qBlXOJApubImUuuLw")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("FjeKQVBU8n2WadCw1ZIEx3C8reSfWUoR_jNnt5Erd18wG25GSoODTUKRwyyp3WxGzcZ9O7HVlA2Dvss9yukwJQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("UJX2w-koSafF5ep9AcfUOHsmAMWnR4Kepao9Rmv29nsmMWlA3f2XPPgPUZjBXv8EzyUx3qBlXOJApubImUuuLw"), "price": Object({"value": Integer(150), "type": String("Integer")}), "name": Object({"value": String("Product 1"), "type": String("String")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("KAS9BfRqsjpQwM6oADQu4BGLgsmwfw-ch4WchgI0VKtZ659aUiNP1xv86KDyuPGxTAVXFtm70HljtrUUkBsbWQ")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "name", "email"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age", "avatar_url", "id", "email", "status", "name", "created_at", "user_role", "bio"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["user_role", "avatar_url", "secret_token", "api_key", "email", "password", "age", "bio", "status", "name", "created_at", "id"]
  Projected query:
    - 3 fields per record
    - Fields: ["email", "name", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
