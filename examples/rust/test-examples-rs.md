make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("0iZILTSGZu-8NE8rhV_ImVu-5vxLho79yR_RschgSXNmkjhCijKP5UKXaQt70lDz1bLQZH7FPV_KL7potQYOgg")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("0iZILTSGZu-8NE8rhV_ImVu-5vxLho79yR_RschgSXNmkjhCijKP5UKXaQt70lDz1bLQZH7FPV_KL7potQYOgg"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("0iZILTSGZu-8NE8rhV_ImVu-5vxLho79yR_RschgSXNmkjhCijKP5UKXaQt70lDz1bLQZH7FPV_KL7potQYOgg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("0iZILTSGZu-8NE8rhV_ImVu-5vxLho79yR_RschgSXNmkjhCijKP5UKXaQt70lDz1bLQZH7FPV_KL7potQYOgg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "XynUeQSPaTfp_nzYBxA4ynDonvZIjV5Ok9f_VEPpIkitQWuzrXR_nHJ9cd9NPM7dOh-40i2EZzq0MpmA7LiPeQ"

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
        "id": "XynUeQSPaTfp_nzYBxA4ynDonvZIjV5Ok9f_VEPpIkitQWuzrXR_nHJ9cd9NPM7dOh-40i2EZzq0MpmA7LiPeQ",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
Collection created with first record: "0-CYxfoFw77jmPIS6dfmcbaMdCOgn6xL1bbu1bjYZ2niByJKCs6rclOGDOjxzpV9FicLLgUv0_sCqg7p8n6b-g"

=== List Collections ===
Total collections: 4
Sample collections: ["demo_collection", "batch_users", "test_collection", "websocket_test"]

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
✓ Inserted document: "YB7A_L2-ZbuHUDbmMdf4T6FIkBLHmnSqQhaT48A9JSOv-cVQbF4trFSYlbwHG41xPwx89qLJoKQDGBKJPu2jvg"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "nUms1UMa_z90NqvG6FNZ02YCAZbnyxAxVgDbS8qq4g9XjZZc9Ol7lQ0JfAtsZMwgeNrM8ZvQU4K22dPktf9Ang"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "G4Ezr93wWSQcJ6Fhli-o02d-7h5GhfEDMrurI5VjI40yfyrjkObLPiSYAYw7TqeYNScmU9PdRnu8J0obGUkuXw"

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
✓ Inserted document with TTL: String("WlUM5roAhBn1ux_U9iidHdPGDpPg8eMDehoCBLqHaRedtZX0ylK_lQHl-Rvp8sGQcUwp8FdPE0LtyosYJn8wuA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "WlUM5roAhBn1ux_U9iidHdPGDpPg8eMDehoCBLqHaRedtZX0ylK_lQHl-Rvp8sGQcUwp8FdPE0LtyosYJn8wuA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-26T06:55:49.415703Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: uLICpyF24Du5zGyE1sp-wjWVr-jXOKsmA-FHILLSVjKiPpD5oYsFbjO5Lc1aDYt6BOQd_d24JXwKoYMG-9yTpw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "ICjpmnb-omudtmdI9Z9dwTYQX1EdHtPt1OGBUa_mCRRiS73ajdiZWD6x6eOT8XL9kDq2ZL_X-R3usnpe3KYQqQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: Pka2yRKOYtQzV8bO6ojZRAVHiZ67uWUVQ7pgTrhFKUBIibOmeQOQ-YSmHdPJDIL_3ZliR1dmloiH81QzWsJF7Q
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: WIKm9jmHKuQe3w1lZKbHMOyARe07GsbGaZRWQH3KBmWsGZ2-UG7OAV11sYxqep7M4FPy3O69QNDop97Dq074pA
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
Created Alice: $1000 - ID: dO-AB4xBrdYAP9757edbDQYYjlQAZ9QeDe9zj48vNE_jzPX0rfgjoGD6HC-tN2_hk-E0926HukitL8mCar5dvg
Created Bob: $500 - ID: Z0B7sglsFfcbRS9-CyejI0-MYlkTGTSWj8sBrU4BpfYycqm1y9TH9kDi7B1i_XT7mfWt2CV0KWwarD5OJ7_mNA

=== Example 1: Begin Transaction ===
Transaction ID: 239f4096-d097-4af1-bac2-83e77ac543fa

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
New transaction: 28275181-45e1-4881-b8e9-98cd252a6cb3
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
Inserted: Record({"id": String("4nRjjfpqoCQaWdLuTf_0Jq0ka1I8-S6LhWAt6iKDQrORJ0DNGwaOFmx3aYNLmwWYuvbLC-lBBO-Vgyfw0gfn-Q")})

=== Find by ID ===
Found: Record({"value": Integer(42), "price": Float(99.99), "metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "active": Boolean(true), "name": String("Test Record"), "created_at": String("2026-01-26T05:55:54.616551+00:00"), "tags": Array([String("tag1"), String("tag2"), String("tag3")]), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "categories": Array([String("electronics"), String("computers")]), "id": String("4nRjjfpqoCQaWdLuTf_0Jq0ka1I8-S6LhWAt6iKDQrORJ0DNGwaOFmx3aYNLmwWYuvbLC-lBBO-Vgyfw0gfn-Q"), "data": String("aGVsbG8gd29ybGQ=")})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-26T05:55:54.616551+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"price": Object({"type": String("Float"), "value": Float(99.99)}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "created_at": Object({"value": String("2026-01-26T05:55:54.616551Z"), "type": String("DateTime")}), "id": String("4nRjjfpqoCQaWdLuTf_0Jq0ka1I8-S6LhWAt6iKDQrORJ0DNGwaOFmx3aYNLmwWYuvbLC-lBBO-Vgyfw0gfn-Q"), "value": Object({"value": Integer(42), "type": String("Integer")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])})})]

=== Update Document ===
Updated: Record({"value": Object({"value": Integer(100), "type": String("Integer")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "created_at": Object({"value": String("2026-01-26T05:55:54.616551Z"), "type": String("DateTime")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "id": String("4nRjjfpqoCQaWdLuTf_0Jq0ka1I8-S6LhWAt6iKDQrORJ0DNGwaOFmx3aYNLmwWYuvbLC-lBBO-Vgyfw0gfn-Q"), "name": Object({"type": String("String"), "value": String("Updated Record")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: kehLYDlzAaDiJbSu7Fkw17uVSnM42-GlDZbCq0JfIq-5TkgO4tZwZSBSfbXD2QM3nz17I7fKjmPXkH4QoChHGA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  id: "kehLYDlzAaDiJbSu7Fkw17uVSnM42-GlDZbCq0JfIq-5TkgO4tZwZSBSfbXD2QM3nz17I7fKjmPXkH4QoChHGA"
  name: "WebSocket Test Record"
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "HuT6-1Ja9Idyi6O17a71mK8HtNYeqGRJHALTri41rGMcAeOfEQDh1phGwlv6G6hnY9LHAswPdQsKpSQUu5t9Bw"

=== List Collections ===
Total collections: 9
Sample collections: ["scripts__ek0_testing", "batch_users", "ws_ttl_test", "test_collection", "ttl_cache"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
  cache:product:1: Record({"price": Float(29.99), "name": String("Product 1")})
  cache:product:2: Record({"name": String("Product 2"), "price": Float(39.989999999999995)})
  cache:product:3: Record({"price": Float(49.989999999999995), "name": String("Product 3")})

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: -MgkYXARWibB14fwx_iR9OysQnrj6miCgIuGVm8lQRU2QyHkbKlMt6YYXfZmP5jZpg53ahC-lLMdx2UUCatK_w
Created Bob: $500 - ID: hGoh5IuPDD4fotp4t53HnyBjZJq_WYIcRjWVOTiTYNB650dfjq97gUo8-WHiO6lWQG-ICwQ8pKVCYpjvtXRGlA

=== Example 1: Begin Transaction ===
Transaction ID: 8a7da328-59cc-4d2e-980a-c896ff4e6276

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 364c1ad6-c9a2-4b97-95d1-3b556dc6f9df
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
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

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
     Matched: ["description.value", "title.value", "title", "description"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(String("Database Design"))
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
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("JwXlAszZ61lQ1bqJngLmcNEqo8KA2gCSsxnQeWBDsuoNpmkRm7ot4LpdfFZw_4pUbhyUvNmiq7T6R0k8Ry7w1w"))
✓ Inserted user 2: Some(String("dcMjSYzIGr1lyPoniSZ5eA8qMmix8secsdL179pJ0pnFY5cchRD3NXsNYK80pCa1mQCZJzCGsVBk128WVetGVg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - age: Integer
  - status: String
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["scripts__ek0_testing", "batch_users", "ws_ttl_test", "test_collection", "schema_client_rust"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "rHhvC5tErQIXndJZjOmquhCcWZAeoXS8-N8Z1FpOwpxSqOLF-JEo_BGDPQDfNGLN95mpV38GEhy9aymO9LPYBg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("DvowPqVyCSaSf1shgQiwjRTxwoRyTRLltGcj_y9-cdwP9HapRXg7hElrsg3PJgJMjYRxD5YfKTw5hfPlNnswgg"))

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
✓ Inserted document with TTL: Some(String("DyhFuhvUFDxsgGKrCFdLnXdor-tAklq__Ox8bMzjRQMw9F2rvjlzjPpYcif0WMQtxUQJT7kfPFuRf4dwofl6gw"))

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
✓ Edge cache script created: 7QZLkkCN2X9kRSTIzChtIvd5GUibD5gecO2dENqZmSJa-KrS10h9wXUYKTvWDuhy0TgVVbZ4qmV80cqoWXU0YA

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 2ms
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

✅ Script saved: ko0w0EnpRq15Ogf6WnNq83U7HiyJV1--PCjfyxFh54xVfeo_OCh5t9tiYH5zBsxd-hRhzQn7kaVjgAAJiDIWKA
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: L6p0c9whKbaBVir7iNu3PZmJz6oJgzI0VmAobJyjn_X6bypG62bRFio9xGaiUwEO-B8IjDV-4HCZlBZ3fILSHA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: r3Mw4sIFvOF_tSpnZaBqAdelyp7YQvWebjSF3QEzk0HIA7jWH9mXrcJq4N_akUtMditzdtWbvZafeVphJua0Qw
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
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 153.47075ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 73.576334ms
   📊 Records: 0
   🚀 Cache speedup: 2.1x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: WI-5jv0kGyny-pxtrHg6Uvh2vApe15g86dBPZIqe30gg3wPUEfmVNIf1Jwo-_YBEkEhGlRpF2W3ivZzATbuW0Q
📊 Found 2 product groups
   Record({"count": Integer(2), "avg_price": Float(474.0), "category": String("Furniture")})
   Record({"category": String("Electronics"), "count": Integer(3), "avg_price": Float(575.6666666666666)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"category": String("Electronics"), "count": Integer(3)})
   Record({"count": Integer(2), "category": String("Furniture")})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("ALGtrgxH9nZAOSSo4-Dn2V2ReO0ii4Ejr3cL04Q0Im4ugI-lZwEEfAzI2LzS7IpE_Y7FrZYhz9XpI3Q7wn_uGw"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: PypS0A8FcDDtsyl1Fclb2sU1cid7KAd1mKlw-4Ii6JdxijJYU0llIH8fG4v2oHaaIMWiiJP_2_v6gWmKm_ExCA
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: OZ5rv9dUatosUXp1HHtfwNJWEkLxeUsbcnfB0PswgChlly9D08KQeyR3wCYOWN3AW1aZZCjKmNFzKCWq5Vuhpg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: yEOVY3vn2E_J6LmDCX5GNQU5_4dzJe_5NjssIe-Ba5kQnQsWD3dWAF5Y-JemWccF0fmZd_RmZo25bnfAnS2vjw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (gS0JjDs4dB05Z5jfhpuoXlHgNyNMz7pklv41Xv62FA9PNcAqFEPJ6VKP837nXyyIpg_sIzm_dUqCTF_-28kw9A)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 79ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (ZUy3BMIM0ApMgMt8haEhP664jpGPzmpmuDcEDX8Mt6v8C1cQy0roxaIOtf09n82MNDR9PGcdrLQfGgMByk_Fcg)

First call (cache miss - will fetch from GitHub API):
  Response time: 134ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 14ms
  Speedup: 9.6x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (dz2cGvFMg__JTsd_1Kr8AzQPIecy4FPy0cKgICn00CX3z_ycHO-OzjVsf8aLKtcIFZD4GcaB7_z5ZLK6AK1XkA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (Ez59VRZPbnJkouzsz-FFp7Iknohr99HnbZXGCn9LcVscA7ujeGq6rDZFwJSF0jFxhBhizr8KYTfViZzpdjFTmg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (NXcoqAXa7k60aVryXXxO4iaU8YLkbYHC8qOZUREF_QXSvlZHB-Hia7KGvtAT7JMeql-1ysknG99lgHWqLdaOYg)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
   Record({"avg_price": Float(367.0), "count": Integer(5), "category": String("Electronics")})
   Record({"count": Integer(3), "category": String("Furniture"), "avg_price": Float(365.6666666666667)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits such as high precision in data representation, capability to store complex geographical features (like points, lines, and polygons), easy updatability, efficient data management, and they support various types of spatial analysis.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
   Record({"count": Integer(3), "status": String("inactive")})
   Record({"count": Integer(7), "status": String("active")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: twO1BB1nRCiwMXP9x5q1aLkLr3v9Hx_eQLeKCnTTvf2LQClZQ3jyYvmcuFPlzPfVdBYPZw-czz__aLo_WgAbBg

=== Sending Chat Message ===
Message ID: SiNqOLQQILaD3YY7UXg0xWXVKoNEl7Cx8XOCxEmXurjv2MVdzhJ5C2LJd6oddJMr6M94pxDWKY7bmv-NikgWqw

=== AI Response ===
Response 1: ekoDB is a high-performance database that comes with intelligent caching, real-time capabilities, and AI integration. It has several notable features. One of them is the AI Chat Integration, which allows you to query your database using natural language and get AI-powered responses with relevant context. Another important feature is its search capabilities. ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval. These features make ekoDB a versatile and highly efficient database solution.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("UhEvXGCzOCj-j7VNyQUbcXLih7Js3Ajrx5oEhnLtwoGbSj1SsaUN1FJwbzK8arb3H-q3dm3iU9SPikPiCa1sqw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("w3tqumP_lT_eKp1xofP0rHTGn1TsWQmoEhWOo4Mh5vjyDcha9nXXXezG3CFvQFeW02vr4BjxLzpN_-Gg5d0O9w"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("LOueh9UWQYbLy9f0LIokNurXE0J0UG_zDuhjwsvb3fG1J3KZX45LFstBIE4FChIXfHzr3BLSMgNpyvi2POXBUw"), "title": String("Introduction to ekoDB")}

Execution Time: 2774ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: i1WL4SuiVp0EFRJSKXD9oQ2crVjpghGBNhgBuK-VE0P4yLzbt5xDtWaL_K9-JjcM1Oh_hsyJ1tQxwISaoJVAbA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, we have a product called ekoDB. It is a high-performance database product priced at $99.

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
✓ Created second session: gnRXJNHkYunqfd9H8WAnDUhdM_bv_6oBpprg16bG5k95zOLdYvzkww8npnALyOvXgr2HoKc7v1m9f-T9raeLGQ
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
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wNWC_Xk3eqKojZcNLJI7ju-2qUVlYBXfFDTXA8MF4cqBWoe0Vh8TkGCxiYsb0gwbZeQUPFasl6EsXuhSfp3kzQ
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is ekoDB, a high-performance database product with AI capabilities. It's priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product with AI capabilities, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1NNMw68muX8sw6rR2rYsDW6lqZHC83Ld9W0CMrfNhIWMTJouNe02BQDdtOIZpVDAzcLV1wk9LSx_J9JSyZZElA
  Parent: wNWC_Xk3eqKojZcNLJI7ju-2qUVlYBXfFDTXA8MF4cqBWoe0Vh8TkGCxiYsb0gwbZeQUPFasl6EsXuhSfp3kzQ

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: 1NNMw68muX8sw6rR2rYsDW6lqZHC83Ld9W0CMrfNhIWMTJouNe02BQDdtOIZpVDAzcLV1wk9LSx_J9JSyZZElA (Untitled)
  Session 2: wNWC_Xk3eqKojZcNLJI7ju-2qUVlYBXfFDTXA8MF4cqBWoe0Vh8TkGCxiYsb0gwbZeQUPFasl6EsXuhSfp3kzQ (Untitled)
  Session 3: gnRXJNHkYunqfd9H8WAnDUhdM_bv_6oBpprg16bG5k95zOLdYvzkww8npnALyOvXgr2HoKc7v1m9f-T9raeLGQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1NNMw68muX8sw6rR2rYsDW6lqZHC83Ld9W0CMrfNhIWMTJouNe02BQDdtOIZpVDAzcLV1wk9LSx_J9JSyZZElA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("ir8T_fLPKqThdpdW0q7LsZRjZQ-3tzWTtpQn5RYohgRYz-9OwqbL-SCkpxoqSw1s6G2A1gmVc_-TUHrBXR_eAA")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"name": Object({"value": String("Bob Smith"), "type": String("String")}), "age": Object({"value": Integer(35), "type": String("Integer")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "email": Object({"type": String("String"), "value": String("bob@example.com")}), "id": String("ir8T_fLPKqThdpdW0q7LsZRjZQ-3tzWTtpQn5RYohgRYz-9OwqbL-SCkpxoqSw1s6G2A1gmVc_-TUHrBXR_eAA")})
✓ Second upsert (update): Record({"email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "age": Object({"value": Integer(36), "type": String("Integer")}), "id": String("ir8T_fLPKqThdpdW0q7LsZRjZQ-3tzWTtpQn5RYohgRYz-9OwqbL-SCkpxoqSw1s6G2A1gmVc_-TUHrBXR_eAA"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"type": String("String"), "value": String("Bob Smith")})})

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
   Inserted with ripple: Record({"id": String("_MnfFgpt533mSf1Jvli8GFWF2uu8JCoqYULuDkcwn899BAikpu-UGd5h13bWWfy4FNS8N_ABXybGAYKlKl9opQ")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("BbIo-1xZRTsDk_NVvAPztmE2hMPZ_Qvb3ph6GUh-YTzEtJ05ZI6zmujt0VlTbH2CPMVKw7gLtuHXwbPBitI0yw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("_MnfFgpt533mSf1Jvli8GFWF2uu8JCoqYULuDkcwn899BAikpu-UGd5h13bWWfy4FNS8N_ABXybGAYKlKl9opQ"), "name": Object({"value": String("Product 1"), "type": String("String")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("rrehpxbmxoISAmj9bxP3BzaBoM51MdJKHVarcKZYem6DZLnv9KgG6QACWQG-FCqfHFvmi4GbJeyYwk_fC47wLw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: N/A

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
