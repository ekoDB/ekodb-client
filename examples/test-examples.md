make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'b2a9PAJf4xLLn7FR4bMCTYOulr7F7ZjqPchHIUrlPaQU68z-s8J6PHHqBldg1QWVzkV1f-q8Kx_L6XyTGRfO-w'}

=== Find by ID ===
Found: {'id': 'b2a9PAJf4xLLn7FR4bMCTYOulr7F7ZjqPchHIUrlPaQU68z-s8J6PHHqBldg1QWVzkV1f-q8Kx_L6XyTGRfO-w', 'active': True, 'value': 42, 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'b2a9PAJf4xLLn7FR4bMCTYOulr7F7ZjqPchHIUrlPaQU68z-s8J6PHHqBldg1QWVzkV1f-q8Kx_L6XyTGRfO-w'}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'b2a9PAJf4xLLn7FR4bMCTYOulr7F7ZjqPchHIUrlPaQU68z-s8J6PHHqBldg1QWVzkV1f-q8Kx_L6XyTGRfO-w', 'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: xqlWFz-xcGkzGrnXOkLAPRhrmrsYJH2KINL4W-8E2QvjvbU_O7jY19A6D8om-LLjx6BB85QBiBvAz0MPk3ORjg

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "xqlWFz-xcGkzGrnXOkLAPRhrmrsYJH2KINL4W-8E2QvjvbU_O7jY19A6D8om-LLjx6BB85QBiBvAz0MPk3ORjg",
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

✅ Script saved: AJFRfWZmwnzeY37Slt7VbeklV1rt1KnDPSPl_lx8MxbM2_kt1bHnswmcUoTio0eP1PumEZViyLX7KjkMZTnKPQ
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: UgBdcEnHAO3MiRrRjIFG2_HCHnGpb1tkEVHt4sMKSexnEI-g4jtybRy7V3mZDc60N2RFvdtQeDlgE3GgNZZxwA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: DkP1rlWoRL1Ul1y-3m5IkUGGLm66-k1fR2DujLuNLAKSbJU3SQ4cLtu0CS1nX0GxpaO1ZHjH6sGtC0pL--HLBA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 50.0, 'count': 15, 'status': 'inactive', 'max_score': 90}
   {'status': 'active', 'count': 15, 'avg_score': 60.0, 'max_score': 100}

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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: Qt4XcDYpJQRr9ZuM3VgENn--bkjoILfw47PLcERuW5Xk7uhDZcujE9YDWWY34YgVMd2LaXSXFk1yZR2u4h85ow

=== List Collections ===
Total collections: 13
Sample collections: ['chat_configurations__ek0_testing', 'chat_messages__ek0_testing', 'demo_collection', 'scripts__ek0_testing', 'batch_users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: True


============================================================
🔥 TEST 1: Original Config (durable=True)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: DKAODFSqVWzfuFMFeQNqF_BgzYaPQkMiSHnvMBYQosiM67e6HJ0638fKiaVot5DiV1V-HhvyX1FjbtOZn54cjQ
Created Bob: $500 - ID: SsGfPdpe1wgvsANrpMVcbNibaLZaU4bBXswiE1gvW-MoeMmEFrPE_K05XCsH9ug0nNboT08NitWn-gT1wUOW-Q

=== Example 1: Begin Transaction ===
Transaction ID: 2214f0c4-c815-40c1-a0c0-d19cdcd9255b

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
New transaction: 38cd3186-8002-4875-bda0-94a0067f103d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: NRe8z46Q7OK7K-vAlP7YOO15NXCwwQI0zteCSD4DTr-6Z5z3o_c05IdYYAKu3Sd2B3QAUt1k9KhJ77i2EORJJw
Created Bob: $500 - ID: zeb6xV5H3EMUhEDcv0kRxJGjrC-5Fv4LpLwDVnLIJksoj0PKDLMvS-iiKZKUZZR8xaFo8CkF8c4JBDEjF-3zLQ

=== Example 1: Begin Transaction ===
Transaction ID: d1fd19d4-2455-455d-9e66-38d58fe7cfc0

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
Created Alice: $1000 - ID: IsmVagbSmD0EUt7zntcOr934pSULojp-7qTzSTIAoNUntHa302-TN5WcQT3t6dF7i4LXHfYk7L-4ejEHImFpqg
Created Bob: $500 - ID: ZsJypxq8YxB3EgC6ggb2TfGqmybCax1ad7_WW_nIUbk5pjaRwKwDftXA5mhrtFm_NmEDlB6mbIIrUFLrOHeDbw

=== Example 1: Begin Transaction ===
Transaction ID: a5077aa2-c28f-4651-8f44-e78fb7d2ba86

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
New transaction: 51432369-79ca-4b28-8eab-a6f074dfb76a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: zHaLkWX2zXrBDhPpdtHqLEMfssPQ81EqfMDLzPNYIUn_-7A5H400YcPR_2giV1OU-13vzszmZDBtZmU7SMXDxw
Created Bob: $500 - ID: 2D2LewRgiYEZJwkVbKo8SA-r7O6aQaiRTxZCtwKDpf4otuJxhYpr7iEO8tbhs8x0Huc2yWCWveREElvlsXF97g

=== Example 1: Begin Transaction ===
Transaction ID: 39407dcc-c96b-4cfe-9613-d6af9a74f3d6

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
Created Alice: $1000 - ID: BB27UmnUpB2RhH8rILZr89KwhJ2np86JXryJOdwSwr3m-ZjmRT-GRVgL_I3nXppLz55NTWA6BP4bRIdRJx8lCw
Created Bob: $500 - ID: Xv3uc1Y8jfL77Mn9BZ6lzJVyX6S88Z9tZDJkIG44trFlHJNJCOFfIxPtDA0OJA8SCm4tGZ945QBTuK7KOrtTdw

=== Example 1: Begin Transaction ===
Transaction ID: f277e804-7c86-403a-8a26-2c45423c1009

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
New transaction: 55bc3766-659f-4543-8217-610bae52dfbe
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: diOidDIBcVd2gwS3en-bsBbktZwyJG7n_cVGU3vrgwyifbPiy-iFARZRpc3uR6JoALKypz_uVXDFgZv6fJBLKQ
Created Bob: $500 - ID: DXRK2UzjJMs2LPAO5t6x79JP6eJ6UsOCHJYEFE3gZMaypZN84-IEHOc22ZGxmcDgc4rY0p8BrLwKk1qyLnutbA

=== Example 1: Begin Transaction ===
Transaction ID: 80a79e82-7319-470b-94c8-1051314bb5d7

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
   ✓ Config restored: durable_operations=True


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
   ✅ Script saved: SJvvWkF9SUvkTEe1MBo1MXbjkri_-4qP0sNFi2zAq1q0ieyWrfZcmM9nvRA2474XJ08YFYB9V4wzE8azzamj1g

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: xhZtvlL4StLCtvE3HqHZ-yF7AxqWSnBsVM_HSypuQWI55iBrRAQttjeDo6H4D1FEdX8q5vpVfLpxOtkj_TmJcw
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: epWuBeWMtVqo_FivTVy3_meRdq1MOastSVdy3KXNhIAdTJiur-4GICR1jl-t8I5HWmbP-PobMYraPqeTaPuIFw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: heN2pt9qLfc9AwUP3MGW1PMOhhLkUxZc-XTz6_G4Rjl-h-pIoQ-StgD8QN-wR41XDARAMu9wWFky-dDZw7-O2Q

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
   ✅ Script saved: lsJlHLg83ORYecl4lTnVKa4KyexfHWLrxRZ2228_e_5c25UxU62b5E1rFJqJVBYDYdcB0Ll2u8fWAC79D_cy8w

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: SJvvWkF9SUvkTEe1MBo1...
   ✅ Deleted script: epWuBeWMtVqo_FivTVy3...
   ✅ Deleted script: heN2pt9qLfc9AwUP3MGW...
   ✅ Deleted script: lsJlHLg83ORYecl4lTnV...
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
  Output: Document ID = WLvu5PLwz7ZA-tFS1eOjBafhUOy7gBCCL7ip0_1DFr-vUrqGknP8MEUrlY2Ai8wyKsRAZGRr5-1l6ElAlu53RA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(WLvu5PLwz7ZA-tFS1eOjBafhUOy7gBCCL7ip0_1DFr-vUrqGknP8MEUrlY2Ai8wyKsRAZGRr5-1l6ElAlu53RA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(WLvu5PLwz7ZA-tFS1eOjBafhUOy7gBCCL7ip0_1DFr-vUrqGknP8MEUrlY2Ai8wyKsRAZGRr5-1l6ElAlu53RA)
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
  Output: Document ID = LR5U5LBObbGp84EN1mHwLlXkkzNRyfD2r9cIeoLY2jsB2oRXp6FMYNqCJS5X8FqW4bEVj-qbmgh1KIAe0E-NEw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(LR5U5LBObbGp84EN1mHwLlXkkzNRyfD2r9cIeoLY2jsB2oRXp6FMYNqCJS5X8FqW4bEVj-qbmgh1KIAe0E-NEw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(LR5U5LBObbGp84EN1mHwLlXkkzNRyfD2r9cIeoLY2jsB2oRXp6FMYNqCJS5X8FqW4bEVj-qbmgh1KIAe0E-NEw)
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
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.8.0
    Uninstalling ekodb_client-0.8.0:
      Successfully uninstalled ekodb_client-0.8.0
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'sJruZHeF8YWLEFdoBrhkmBu5d2K_sq2yARMn9gUmNlrMdNmpEARqLceWDQUOvdLW5-E55iY9kGXC3V6-A-9zZQ'}

=== Find by ID ===
Found: {'tags': ['tag1', 'tag2', 'tag3'], 'value': 42, 'categories': ['electronics', 'computers'], 'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'id': 'sJruZHeF8YWLEFdoBrhkmBu5d2K_sq2yARMn9gUmNlrMdNmpEARqLceWDQUOvdLW5-E55iY9kGXC3V6-A-9zZQ', 'created_at': '2026-01-26T00:57:21.704239', 'price': 99.99}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 00:57:21.704239
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'tags': ['tag1', 'tag2', 'tag3'], 'value': 42, 'categories': ['electronics', 'computers'], 'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'id': 'sJruZHeF8YWLEFdoBrhkmBu5d2K_sq2yARMn9gUmNlrMdNmpEARqLceWDQUOvdLW5-E55iY9kGXC3V6-A-9zZQ', 'created_at': '2026-01-26T00:57:21.704239', 'price': 99.99}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'price': {'value': 99.99, 'type': 'Float'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'created_at': {'value': '2026-01-26T00:57:21.704239', 'type': 'String'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'sJruZHeF8YWLEFdoBrhkmBu5d2K_sq2yARMn9gUmNlrMdNmpEARqLceWDQUOvdLW5-E55iY9kGXC3V6-A-9zZQ', 'name': {'type': 'String', 'value': 'Updated Record'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: VSesnppyjS1mppFXklqJixW2CBJK24668cbRFg_X_RYLPy-ENZo92lUTNV5ZVGxP6AIlPICxwYUJHlBGIbayrw

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
Collection created with first record: "QpoeNHFr7ad0pnPoWE8mB4U4MWmAbUiptmWPyvd6SXL2KvuWz8w-BuhrYxQeSVFbqwxev5fSwBpfxn2KV4qSxQ"

=== List Collections ===
Total collections: 12
Sample collections: ['client_collection_management_python', 'chat_configurations__ek0_testing', 'chat_messages__ek0_testing', 'scripts__ek0_testing', 'batch_users']

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

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: {'price': 29.99, 'name': 'Product 1'}
  cache:product:2: {'name': 'Product 2', 'price': 39.99}
  cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 05Rk6Pa-o3wTA4BufUa7KBgCp0aKVB4xU74x52KAmVTixyc-umlkYEQh_J9HKHmCKmJPKKsy-iYyHP3sty8OKw
Created Bob: $500 - ID: N8MeMLVWS65SisZTmXsOjQtuPW8pNEz_3KJjy0YJGsQAr7GBvdcVZl12-pz9jQmvM_zOmyF8BgWYrFB2xZ1Clg

=== Example 1: Begin Transaction ===
Transaction ID: 929c208e-35e6-4b42-aff4-b6306dc0189b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: e3651b1f-3029-44bf-96ea-0b20a61db80e
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
  1. Score: 25.740, Matched: email.value, email, name, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio.value, title, bio
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: bio, bio.value, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, title, bio.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: bio.value, title.value, title, bio
  4. Score: 39.600, Matched: title.value, title, bio, bio.value

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
  1. Score: 0.773
  2. Score: 0.749
  3. Score: 0.735

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.374, Matched: title, content.value, title.value, content
  2. Score: 1.368, Matched: content.value, title.value, content, title
  3. Score: 0.387, Matched: 

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
✓ Inserted document: M3QeiogMzpAHl-Z1Sfu8WsN6QUdrJZjdWmjCza9j8mndHU-rj8JK_LJRfrDF2Ryvtzv39DFWCIxMnXYBgliGbg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: NpHiFixvaOJK7wyM4Suh3t6ymTUoCS7Yp1ex18nBqyHZkjC6SwYijEtVVND6bTM9d06pXA0HOqy73eGi83o7cg

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
✓ Inserted document with TTL: Jwcms98lUTnzZPMChqcSzNWLXP-XnsWsePNg6nf2v4h7KBHgXGnCp2ePTuss_5HFyBJqzHyCK_BvhCfHF07T8A

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
✓ Edge cache script created: t02gQvZ02RxairmreyJ92cwdY5ArJjBSCqM2EM9f5N5rEjUe-VQM0GzlFxu-waOpt-006P03AUXMHUMgnUykYw

Call 1: Cache miss (fetches from API)
Response time: 2ms
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 1ms (1.2x faster!)
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: ge2AzVH6tdNpOXMuiMb_GEf8ax7m08-xA7zXkPy5p3VInrryVHv_1Zo98GWaxeuFh1K8TgekaOoxRomrHXRPGw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: qjq9-yE8hHfegCyejlJ10uZNi5MnV-99aIOu62cuEAPzXDmFRllUT4qbB4mpgPr1yqXf1O0hucbj2vY5gaX7vw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: JaO0Bqk9Xho_waAsCucNMlVFn9cBtsfbkPzN-R_YWEMu8IKI3fWetKRrExKyg5yD-IBPqPdK6uZn-Sea-_0bog
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 67.6ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 62.2ms
   📊 Records: 0
   🚀 Cache speedup: 1.1x faster!

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
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: O2TPSTA5NskUp_0tgoUkjZraM5TQfU4DlXj40HR_JV3RqyVP_FLc499Ioprl9WiaXKABL69QrX5u6Vh1Z_3eVw
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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: LUMGr9PTXV3y621Navbvqjk14n6AIG2G343KeYKTmlgbA73KIa-tiTk2_GYUejkTcFOMWOzwS33ZsXNKwClNFg
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

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

✅ Inserted order: wKNnS6A9pRc6TaQ6ImyOhlryMk3fXSDaTlyQDA05a772Pl9Ot6ukBWDrX16lxk2gPiLP68nVG6t-Eq0efEfDdA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T05:57:23.675841+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (-bCxmxQaOB6n9sBtG9sEczBUQWoANrKfHu6GbE2UIFrh0NUXcOSfbjNSeKlFjoBNd8EvwaImyhG7u9B1GIeJdw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
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
   Vector databases offer several benefits such as high precision, scalability, and flexibility. They allow complex geometric operations, support multi-user editing, and enable topological analysis. Furthermore, vector databases are good for representing discrete features and maintaining spatial details.
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
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
   1. Vector Databases Explained (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'AI', 'count': 2}
   {'category': 'Database', 'count': 3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: qO-Z-fYTrhxTnTF_RGGmi6IGq6Cw3n5Wtt4IzSVHy-EGRp8ZpGeXPf6yW9lO2L5vAbnhl3E9xRUrLPDM6uHP9g

=== Sending Chat Message ===
Message ID: E4s8ActqRx0yvNKTC7QacIu8XLyk7RgmHeKXb_OIv56nht4lb_2AeN-pDxl-E3HxzcFBkvwjcSC_WGQ6QmF3Cw

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

Execution Time: 2583ms

=== Token Usage ===
Prompt tokens: 449
Completion tokens: 89
Total tokens: 538

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: C72dvLKIhLq4pet_FQ_mAPmEt9wwoTG9o6XU2p-DavmqZB_W40LqP9t2-Ec0DdyL7yPYYMKw29gd1ik1r918Bw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['chat_id', 'content', 'context_snippets', 'role', 'token_usage', 'id', 'created_at', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of the ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: hfFbxSDUNbfSKyxC79AJeZd7GCrQRPZm1gkmWSev-oxoy7vEStTatUY4EFqw212ot2-7w95yye9aE4-_D1Ioaw
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
✓ Created session: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, there is a high-performance database product available named ekoDB. It costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Et5UKK7wuG-JvSWMb2ZfwSt2YUMfwNTA4xZ2uOBY1Wmm6DSfzZy3eSpZ8eb4KM9kIStgpH9JKq0CmIuo5phhVQ
  Parent: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: Et5UKK7wuG-JvSWMb2ZfwSt2YUMfwNTA4xZ2uOBY1Wmm6DSfzZy3eSpZ8eb4KM9kIStgpH9JKq0CmIuo5phhVQ (Untitled)
  Session 2: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg (Untitled)
  Session 3: hfFbxSDUNbfSKyxC79AJeZd7GCrQRPZm1gkmWSev-oxoy7vEStTatUY4EFqw212ot2-7w95yye9aE4-_D1Ioaw (Untitled)
  Session 4: qO-Z-fYTrhxTnTF_RGGmi6IGq6Cw3n5Wtt4IzSVHy-EGRp8ZpGeXPf6yW9lO2L5vAbnhl3E9xRUrLPDM6uHP9g (Untitled)
  Session 5: wNWC_Xk3eqKojZcNLJI7ju-2qUVlYBXfFDTXA8MF4cqBWoe0Vh8TkGCxiYsb0gwbZeQUPFasl6EsXuhSfp3kzQ (Untitled)
  Session 6: gnRXJNHkYunqfd9H8WAnDUhdM_bv_6oBpprg16bG5k95zOLdYvzkww8npnALyOvXgr2HoKc7v1m9f-T9raeLGQ (Untitled)
  Session 7: twO1BB1nRCiwMXP9x5q1aLkLr3v9Hx_eQLeKCnTTvf2LQClZQ3jyYvmcuFPlzPfVdBYPZw-czz__aLo_WgAbBg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: Et5UKK7wuG-JvSWMb2ZfwSt2YUMfwNTA4xZ2uOBY1Wmm6DSfzZy3eSpZ8eb4KM9kIStgpH9JKq0CmIuo5phhVQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'xsFxoJzBLluuOz-1ONWAXA3_xurXViXyNedk79W4Ya-JG9y_-PngEA4nwExgnoCdsB1RDmqLyBKsvO5pIIg0Mw'}

=== Upsert Operation ===
✓ Upsert (update existing record): xsFxoJzBLluuOz-1ONWAXA3_xurXViXyNedk79W4Ya-JG9y_-PngEA4nwExgnoCdsB1RDmqLyBKsvO5pIIg0Mw
✓ Inserted second record: 44u5e1bin8VTGeyuWQBrNjDWKl86jQVBriM9UTshGqxlrvl-8RSj41ySykeTPOLFCXddl64jvK5UrElW2wvPNQ
✓ Upsert (update second record): 44u5e1bin8VTGeyuWQBrNjDWKl86jQVBriM9UTshGqxlrvl-8RSj41ySykeTPOLFCXddl64jvK5UrElW2wvPNQ

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'id': 'xsFxoJzBLluuOz-1ONWAXA3_xurXViXyNedk79W4Ya-JG9y_-PngEA4nwExgnoCdsB1RDmqLyBKsvO5pIIg0Mw', 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'age': {'value': 29, 'type': 'Integer'}}
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
   Inserted with ripple: {'id': 'CdezZUXFEb_6PkuHpvWvsIx2REAYJnyeC1h6ptOVdUzJCGKuKMIzDbD-b7jH7MgtlhsxoSEAEO9dZAnsjCcZvA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'iQOMjLx74K_g15LzDwj4pbkxDXfNMr44-4eHahx8gqbGenl1iJauKw45th30Ax-7LUvYGLsAFfsM5xPndErp0A'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'value': 150, 'type': 'Integer'}, 'id': 'CdezZUXFEb_6PkuHpvWvsIx2REAYJnyeC1h6ptOVdUzJCGKuKMIzDbD-b7jH7MgtlhsxoSEAEO9dZAnsjCcZvA', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'CdezZUXFEb_6PkuHpvWvsIx2REAYJnyeC1h6ptOVdUzJCGKuKMIzDbD-b7jH7MgtlhsxoSEAEO9dZAnsjCcZvA', 'name': {'value': 'Upsert Product', 'type': 'String'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: False

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: {'type': 'String', 'value': 'Alice Johnson'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
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
Inserted: map[id:69V6yq5bGe_6HFnbT1aZUaqHC1dzAz0xw-31lKJta1kpuwI2d2ZJQ-oXhp63n9Brbqk4j0C8zzQSVozrOhU7pg]

=== Find by ID ===
Found: map[active:true id:69V6yq5bGe_6HFnbT1aZUaqHC1dzAz0xw-31lKJta1kpuwI2d2ZJQ-oXhp63n9Brbqk4j0C8zzQSVozrOhU7pg name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:69V6yq5bGe_6HFnbT1aZUaqHC1dzAz0xw-31lKJta1kpuwI2d2ZJQ-oXhp63n9Brbqk4j0C8zzQSVozrOhU7pg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: A4qF-pqdwkFsJuqfXLHpKbs6oAE_YPGEhnmgThECK-CXukWAgzfuHETj4unrK6t9mwdM3JK25MnDvj8OHX8BYw

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "xqlWFz-xcGkzGrnXOkLAPRhrmrsYJH2KINL4W-8E2QvjvbU_O7jY19A6D8om-LLjx6BB85QBiBvAz0MPk3ORjg",
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
        "id": "A4qF-pqdwkFsJuqfXLHpKbs6oAE_YPGEhnmgThECK-CXukWAgzfuHETj4unrK6t9mwdM3JK25MnDvj8OHX8BYw",
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

✅ Script saved: H-pncSqeFifxzF2x9lAnqeoubXyAt5k9YsM9a2CE5HGN1TwEZMIM1-Pijg1CE4o-n6yfhPgIkOduvArXRpMqVg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: e7XqNIWQkitlagqHPP4VY26nqQaFu3U1O2hRm6y475KLbTSUyggoDz2NYqfOu3XfhhI8NZ2tVd3lFhhkJZ7lSw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: NNdmO0hWHPnnjSCB1sPgRLcJYSjis9wxMJprx_CJ806jWOSO_5zS_x_1lQcYH0Cs-meOWjkcZueZSFO8_9H7kA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

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
Collection created with first record: eqqXHWFuLdiJWmLOlwOU4uXiinklvcpAxE-VlSIUzvU-TULdnlTOd4TfGSigf8TeRShqkF00Nbt8yu7BGktyUw

=== List Collections ===
Total collections: 16
Sample collections: [chat_configurations__ek0_testing chat_messages__ek0_testing demo_collection scripts__ek0_testing batch_users]

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
Created Alice: $1000 - ID: sYR3daRPfT1X3py5SBTKy8_lTktOmGQovUwv3h68tjdgnHOnYiS4pw8GnSsdsEzn6xnxcPGtvZ990Hz0nxkjYA
Created Bob: $500 - ID: 4I7lx6jY5svWuGeD9n7mnb3CAg0xlgKDq8Yd4ZXWHXB9D7HViReWrGp5SETLYNo9Js6REIt19J1ExlyIpXkZpA

=== Example 1: Begin Transaction ===
Transaction ID: 89217c6c-d943-4249-9279-7e06433f2042

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
New transaction: f4cad4af-e39b-4f6b-8f70-90dd4336186a
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
   ✅ Script saved: UiT8dpZiW3NJHE7wn1rQsDuY3YKjAtiwKutQUlKyV-usgdqcu8oBJzBocYIKdy37GohJYSB_Og-4uHnlHtd64w

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: dC6OVOdidTuRcOyLN0oBkXkBeeEpFejWx0N1KJXC-dhabmWuSlNylmc5cM21JdMZIlC_dnYIkwHWyznSL1HSKw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: CCx6z4LNs6a9bw0tkmdTYrCLOwZki_1uk5njfVRANPaLDxHT_V1BZt0N4pGyQluVQba2AOy_2Ak5anHQlTLAHA

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
   ✅ Script saved: awGidDOzQEhACQnDkhX9cYz6L2dpb__eRvrkl5r1a9eU6XfjEemb2bOGkoDJRN0eddRjyeYHtvgYcIiU306T2Q

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
   ✅ Script saved: wRLrmjzYQ7IsGCjtryNoqu7W7Su1GfL0-WQHPuG1ni2aVg_Umz0JEnQKcm6_1xUrj_aNdwoP7V97n8smI_2Opg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: UiT8dpZiW3NJHE7wn1rQ...
   ✅ Deleted script: CCx6z4LNs6a9bw0tkmdT...
   ✅ Deleted script: awGidDOzQEhACQnDkhX9...
   ✅ Deleted script: wRLrmjzYQ7IsGCjtryNo...
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

2026/01/26 00:57:52 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = bcEDAysuy7ybf18LRmRMQPAhCrjqPEU_SEnlAl1SjaefnE75aiQyfJs8t8tCAappwKIPAIhGMnODjysMg7_wHQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(bcEDAysuy7ybf18LRmRMQPAhCrjqPEU_SEnlAl1SjaefnE75aiQyfJs8t8tCAappwKIPAIhGMnODjysMg7_wHQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(bcEDAysuy7ybf18LRmRMQPAhCrjqPEU_SEnlAl1SjaefnE75aiQyfJs8t8tCAappwKIPAIhGMnODjysMg7_wHQ)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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

2026/01/26 00:58:00 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = X2BEoxnHjlyoxxueLGSjpvlfa1JTI06ca_jb3riWrlPtsXa_y6hoypJIgZvzSo46bPwhWSU4NUDSpZ3UVn3Egw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(X2BEoxnHjlyoxxueLGSjpvlfa1JTI06ca_jb3riWrlPtsXa_y6hoypJIgZvzSo46bPwhWSU4NUDSpZ3UVn3Egw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(X2BEoxnHjlyoxxueLGSjpvlfa1JTI06ca_jb3riWrlPtsXa_y6hoypJIgZvzSo46bPwhWSU4NUDSpZ3UVn3Egw)
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
Inserted: map[id:M9roSnfUtNDjzpGYTp-k2WaNHs7rZ7o79a2EpfW-yBc44CO8HQtLkJrq-X1lpFYiE_o60wTsaBzAnq9Sd3zClg]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T00:58:07-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:M9roSnfUtNDjzpGYTp-k2WaNHs7rZ7o79a2EpfW-yBc44CO8HQtLkJrq-X1lpFYiE_o60wTsaBzAnq9Sd3zClg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 00:58:07 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T00:58:07-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:M9roSnfUtNDjzpGYTp-k2WaNHs7rZ7o79a2EpfW-yBc44CO8HQtLkJrq-X1lpFYiE_o60wTsaBzAnq9Sd3zClg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T05:58:07Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:M9roSnfUtNDjzpGYTp-k2WaNHs7rZ7o79a2EpfW-yBc44CO8HQtLkJrq-X1lpFYiE_o60wTsaBzAnq9Sd3zClg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Qth_mTKGsUcvFslTiAfKUa38l-UPokyLE2nC9Q9Nd7rAIA-SB8bP4GaQFzUy9SgW_2ylcYa1k9EhyJ7GyEnIvw

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
Collection created with first record: 89rmEPfljTomOs7dsfFYAws3RiKbmRFHvyylrSOPrkmHftl-dkRqCiB-lfx3smJl4TuJrefLzMk85O67A2p91g

=== List Collections ===
Total collections: 15
Sample collections: [client_collection_management_go chat_configurations__ek0_testing chat_messages__ek0_testing scripts__ek0_testing batch_users]

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

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: map[value:map[name:Product 1 price:29.99]]
  cache:product:2: map[value:map[name:Product 2 price:39.99]]
  cache:product:3: map[value:map[name:Product 3 price:49.99]]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: -iPY6igk_L62Zufssk-b_Fiyow9AB0Bt6wGXKkL5L6YGpN-JoyGkkxWaTZWBqK7uR56kUnKLlEUa66KAfS2y4A
Created Bob: $500 - ID: XLF5w8HGq0tnLdJOiYHVoSg-g5iaizSjt60sQ7l3oHbgI2Ua9dAIFWAtUaz3Q_SQ4vh2kQFCzhpRU_NvXpg6nQ

=== Example 1: Begin Transaction ===
Transaction ID: 9d05a29a-83bf-4412-9e36-9b7cae36aa42

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b408289d-68d6-4d8d-b6d8-595c00fb1fe7
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
  1. Score: 0.756
  2. Score: 0.747
  3. Score: 0.724

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.690
  2. Score: 1.499
  3. Score: 0.302

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
✓ Inserted document: ch9sSYxvjeefumK7Nt_vl-VtT-CTUKxfxodAJ8e6YEfktYAZMMU_B529rJUb6j6N8eegyVORbEXat_40G2XpLg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: gk-sXu4y4JpYFTCWUHzZ7faTngAN3TbohuItmyL3Ral1-k-FBdIWIarfJa4aa1R5MdtuencE3NU4m7QPTPlmVw

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
✓ Inserted document with TTL: HdOVOSg16Mw9jOiPyGzS3G2Q2FQnlkiRxidThF9-1kN2ud9wKQDqI9zyLNeriKWkQQJKqR90NnpZINd8abBTFw

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
✓ Edge cache script created: zeVv2IaBttuB4TufXrKgJnGxOxZvdn-260RFJkpgiu7ePC_10l3XUu5viM_f5wm5554mpQ5OIoccc-GLajEffA

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

✅ Script saved: 7tr-OuGiLXA3oW9uvDbj0uJg6nDbIGpYn-6AOqmqqjH91c1D3dxN8Da75HEwfTDyjbJknpGFc2ihUapKNU9khw
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 86.753417ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 75.477958ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Go Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: fA9F7tfAWX6ko00-5Mju3ZLdBV8atoc9aSILKj0hvkXc3RUvTELwgxOnGxxvl2EETu2QhxvRNiR9VhIEoPFv5A
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

✅ Inserted order: ryIP9njGyYbzySuqkrCx9d1MO0oq-3wrawF0-Pvr2DKRk636pa6GiDbbCRaZzZF7eP_CpIcyLUU3eCPyy64lGw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: OLJNLs7OLaetxsHuFROj2T-FMTfGjmCI6LdCABA5D4hl7IxQFp_ZfelWCou9aTzLjLEj1bHeV9_WtZMEv5-PqQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: pKRZvHH_MObA93RpF2_Io1gkCVumvI9AkwEP3d3DWuWCMb0fRtjzDz9E0gRbEccMBjKhgLaXnLvP755ZPlmbJg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: 3vHdxwShnboVmQQxPLhyXUJMWOrFaVWgbDMo2TsBc__4unM9c2AvscxlJP24WPguzMjtUuIYCdMVTfO8h7V8Rw
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
✓ Created SWR script: fetch_api_user_go (Vokbp_FJYn0E8yWxF2JAn5s0v4XVovqkXJPFDBZ3ztauC1VlPGIlTFERWhq4Jr75JkdCwWwiC3Nzt1_QSKlPkw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T00:58:11-05:00",
      "data": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        }
      },
      "id": "L2XsKpQT56eVRRoLqwtdrrzutAjGNSzGLDfLnWwRvMCOSpoXWC514msKC-YjyRane3L25XNuMqyPN21ueFy-0w"
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
Response time: 79ms (served from cache)
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
   map[avg_price:365.6666666666667 category:Furniture count:3]
   map[avg_price:367 category:Electronics count:5]
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
   map[count:3 status:inactive]
   map[count:7 status:active]
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
   1. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
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
✓ Created session: iOoS8YtheulgTtAeIuuE8D3bhrehIrH7-gMf9kcjqZ1WhiK4u5IFgblmUK0mnvBgQq4b_DIusk1CVO0wAc81Ew

=== Sending Chat Message ===
Message ID: 8LKM6t00JKIl8h5S9896vlSFUQrFguNMXxv_S4NYYUiALswkmHoZ0tMwyzb_0Xpv-jfqkRNpNdIqmC7tqK-1fQ

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:GhCgogDBsMY9HbreV6oZpxr8JmYT4eqM5QRjvxZ-OoKPbYVjtDIqwYhkX16TWg0HSjikb2ozxYnkTOH6pVKahQ name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:XUXJCmMhLyiZf5nW6-Xn6ti8D0-f-Aqd45sdRhISp2kcEO9AW_7MxEMREy_X6ZTp26GDIUtzYeeHL8YIVcAtoQ name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:9KhDQRlu-yPcjXYCl3yQDUIUfmPHtLAViGz2Y0_Hw4MXo0cZKE-4KJiMBpYEoCjdGlvyKeNBuumTHYf4kK7fCw name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2977ms

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
✓ Created session: LmSQtJ0PSHNRi8IZpwtyZ_5VPFUn8o2s06I2qNHLI_Jo_kdPx1ckb2Vw1nqJKyjiDVnU-ceYaPESBSp_ulhR0A

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, there is a high-performance database product available named "ekoDB". It is priced at $99.

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
✓ Created second session: ltoCE9Mp-YHRJtsl5hQz6NbEq3eQLlHVQko1hD7fAv4kWIRFPEQ8lYFYcwbEKxKjiqcO4ZH5mj5uSn5vyDHDEw
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
✓ Created session: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the provided context is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 2XJhpFO91hH0Wfu7q2gnBsw5ktT8aWSSesecGcTv1dFofBm5_JsPFFszfeIPphjbgk_ic8n3rcI8qEhW7yzUQA
  Parent: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 2XJhpFO91hH0Wfu7q2gnBsw5ktT8aWSSesecGcTv1dFofBm5_JsPFFszfeIPphjbgk_ic8n3rcI8qEhW7yzUQA (Untitled)
  Session 2: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ (Untitled)
  Session 3: ltoCE9Mp-YHRJtsl5hQz6NbEq3eQLlHVQko1hD7fAv4kWIRFPEQ8lYFYcwbEKxKjiqcO4ZH5mj5uSn5vyDHDEw (Untitled)
  Session 4: iOoS8YtheulgTtAeIuuE8D3bhrehIrH7-gMf9kcjqZ1WhiK4u5IFgblmUK0mnvBgQq4b_DIusk1CVO0wAc81Ew (Untitled)
  Session 5: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg (Untitled)
  Session 6: hfFbxSDUNbfSKyxC79AJeZd7GCrQRPZm1gkmWSev-oxoy7vEStTatUY4EFqw212ot2-7w95yye9aE4-_D1Ioaw (Untitled)
  Session 7: qO-Z-fYTrhxTnTF_RGGmi6IGq6Cw3n5Wtt4IzSVHy-EGRp8ZpGeXPf6yW9lO2L5vAbnhl3E9xRUrLPDM6uHP9g (Untitled)
  Session 8: wNWC_Xk3eqKojZcNLJI7ju-2qUVlYBXfFDTXA8MF4cqBWoe0Vh8TkGCxiYsb0gwbZeQUPFasl6EsXuhSfp3kzQ (Untitled)
  Session 9: gnRXJNHkYunqfd9H8WAnDUhdM_bv_6oBpprg16bG5k95zOLdYvzkww8npnALyOvXgr2HoKc7v1m9f-T9raeLGQ (Untitled)
  Session 10: twO1BB1nRCiwMXP9x5q1aLkLr3v9Hx_eQLeKCnTTvf2LQClZQ3jyYvmcuFPlzPfVdBYPZw-czz__aLo_WgAbBg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 2XJhpFO91hH0Wfu7q2gnBsw5ktT8aWSSesecGcTv1dFofBm5_JsPFFszfeIPphjbgk_ic8n3rcI8qEhW7yzUQA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:7ElMQh_pFXhuKT0ikDOvs0DIPn_ZphvsT8tMSmwChuMbIZffIB-k9HPIkXY2smjJbVf7IoZVMUOIGr5i629RDQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:7ElMQh_pFXhuKT0ikDOvs0DIPn_ZphvsT8tMSmwChuMbIZffIB-k9HPIkXY2smjJbVf7IoZVMUOIGr5i629RDQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:fIqYCnfz15-XuI2rJM6Ssa6C5BAzKYPYV7juGERzIm3nJk3K-mRYIXgIkhZEGbgQHbYy0VBa42CUqgM2FZypXw]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:7ElMQh_pFXhuKT0ikDOvs0DIPn_ZphvsT8tMSmwChuMbIZffIB-k9HPIkXY2smjJbVf7IoZVMUOIGr5i629RDQ name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:Xp8Htgw-P8qITHymxU-iUiHpwrkK1ud7M7AMLH2UdWid-MmLJKcqnFVGE2eD30YooJSD5UDFhvjGXCjNYyrMbw]
Inserted with bypass_ripple: map[id:M78OU5s1Xb9ZdaFzx6olFeR2OwHCwbzj8SlBqIZzslhFLLbLN7o71sAgsEqvA_S40ypP0ymKGSUrD6rG-veMGA]
Inserted with TTL and bypass_ripple: map[id:4r7Tz_IJq9mNLWJVAY562vzdXj1qy_kUmToL_Xd4_N83dWB6gZJZRUrvjgz8DtaeArAIB-oip85Etr5HXv09Tg]
Updated with bypass_ripple: map[id:Xp8Htgw-P8qITHymxU-iUiHpwrkK1ud7M7AMLH2UdWid-MmLJKcqnFVGE2eD30YooJSD5UDFhvjGXCjNYyrMbw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:ek8U5vWD0Q218oC4mOIas4tiy8zIQyzO8CCPEEvsjvH-5BtzOBUOzwdRxkOnOZeeMr_NyHqwdUx9qp2FbV8q9w]
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
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 823ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'WckLvHDOQyGa4b7tn4Kz8iSML-6_TPlH04k4F33Tc9blPuCyRz2-YnQelbaOGevORxzG46n8W9BIHpErzSxDkw'
}

=== Find by ID ===
Found: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  created_at: '2026-01-26T05:58:27.729Z',
  name: 'Test Record',
  value: 42,
  metadata: { key: 'value', nested: { deep: true } },
  id: 'WckLvHDOQyGa4b7tn4Kz8iSML-6_TPlH04k4F33Tc9blPuCyRz2-YnQelbaOGevORxzG46n8W9BIHpErzSxDkw',
  price: 99.99,
  categories: [ 'electronics', 'computers' ]
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T05:58:27.729Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  created_at: '2026-01-26T05:58:27.729Z',
  name: 'Test Record',
  value: 42,
  metadata: { key: 'value', nested: { deep: true } },
  id: 'WckLvHDOQyGa4b7tn4Kz8iSML-6_TPlH04k4F33Tc9blPuCyRz2-YnQelbaOGevORxzG46n8W9BIHpErzSxDkw',
  price: 99.99,
  categories: [ 'electronics', 'computers' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: 'WckLvHDOQyGa4b7tn4Kz8iSML-6_TPlH04k4F33Tc9blPuCyRz2-YnQelbaOGevORxzG46n8W9BIHpErzSxDkw',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  active: { value: true, type: 'Boolean' },
  created_at: { type: 'DateTime', value: '2026-01-26T05:58:27.729Z' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  price: { type: 'Float', value: 99.99 },
  value: { value: 100, type: 'Integer' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: fHl_mTxPFmbcRP-ayDGwL0PA5lTPvxzBa-d93R8X5NGBohwnzNtWy2swxdZnLgn1nE07aTVI63VWDmDWuloLHA

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
Collection created with first record: 9ir_VwtvPhy2a10aKc7sV5F7ifj5oOwNm4V2YjGtyxtvwNQaaa1gB6_4ojqDBInlZ7CAReOMU4VQKi-SC4zPZA

=== List Collections ===
Total collections: 21
Sample collections: chat_configurations__ek0_testing,chat_messages__ek0_testing,client_collection_management_ts,scripts__ek0_testing,schema_products_client_go

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

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { value: { name: 'Product 1', price: 29.99 } }
  cache:product:2: { value: { name: 'Product 2', price: 39.99 } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ZZ6A08NhQC86sq4dUVhzG-GZKNjzaJ7w8n37AgnGHf24NUWPZpiP6W8KbXeppqO60BfLV8lpYOxvAWmLS3Mi-A
Created Bob: $500 - ID: WG6qs8rTzbVVBwJYcMPgMFelxDSPrDNxOjhcxv8HxRXQR9LouF6mt6qhpDOrcjCYDEakCpId7YRBF5WXJhKA-w

=== Example 1: Begin Transaction ===
Transaction ID: 43b2fe63-1ac0-4bab-8e9b-9eb0416e94eb

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6c91efa3-dc5b-4bd9-8a10-4fa07caabc84
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
  1. Score: 25.740, Matched: email, name, email.value, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title.value, bio.value, title
  2. Score: 26.400, Matched: title, bio.value, bio, title.value
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title.value, title, bio, bio.value
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.768, Matched: 
  2. Score: 0.764, Matched: 
  3. Score: 0.748, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706, Matched: title, title.value, content.value, content
  2. Score: 1.499, Matched: content.value, title, title.value, content
  3. Score: 0.307, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio, skills, bio.value

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
✓ Inserted document: maNto13JmSTc1IQLWcyR9qSdogQV7VunN6KRGhxT5h67URfwiLk2s4P3rN_uxOvzdlB840C-zRx9_J6qF3EYKQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: RFKWKr634oN7rJAdHR-49Uk6f1s3F5jpcnbblBAgisn9_gghICZj0d7-idSXK26vTLGHO_-3bDycA0x10cAf2Q

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
✓ Inserted document with TTL: m3AioByz4SBjEzEI4L8XVJ6TKoc2lSo7TRFMWKrBJab675GIQZ05t_8Oeb5RFtOzjbIKr4rPUe_IpM1T3A_zEQ

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
✓ Edge cache script created: t8cVRhzELa-H-mXx_qFFYe_UHB0M_oGSHhrAGDNchTxfkWyU_MLQcltUQvNp5uNWDjItQXLhH6CnACayLuyOvA

Call 1: Cache miss (fetches from API)
Response time: 3ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 4ms (0.8x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: RpK02iRQXhcwX1lf1ra9Q0MOrrZCQpvJRbFdPZJXDVcNOxV636bELj9FDwswc7v_WUp_5RbRnjRSuKbyLkMu6w
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: DvGOVRyFPnDD5QSVWfXA6Qjnn_yM3HH8cKJUXHpw81U_EXRinkVrHzuwnfedcjo8-ojuzxwtTAsd0Q0r5yoj8Q
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: cOe3FoH2qIhIrUEnn8nlI1Xy6cf-bXKsNd7SufBdNsGHmsJ8wplw7FTNULrm9m3ry1TeGi_jbt8wzPTHaJztzQ
📊 Statistics: 3 groups
   {"avg_score":60,"count":5,"status":"null"}
   {"count":5,"avg_score":60,"status":"active"}
   {"count":5,"status":"inactive","avg_score":50}

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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 69ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 73ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

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
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: K9eb4IoX35bqq3dngg4odrGx4STfYyMK5d8Yjf2s_MURtIwDwW5csrJm6hANJbVrF0XeFTmtmjOr-mPyutqbEA
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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: SflxRp20_a4nYFXktDmg04UU8U-k4-v-MPBzTHHeFOvd_5Svg8kG1rncrxr5rvzg7vH9BsTVv0W2xGI8bXL0jw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: FNGczFMPqV1nOUwUbMz706IdeGc-bGonX7VB-0_MqfI3wrcGml1r07LEX_DrLsIiWjeYF8fHboJBcudlYOaoLQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 5F-trZmmQNFDKKHY_uVzUfhcx5YibIFHnWIbF14qltQfRAiuG8Mk-SNn_y4Jt6BsWM3FcZoSgiKkubscAvEe6w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: -N2n0NujOy0kQEw5lhJDB15wLflJ2S17f__GQPNpWOBukNt1yr_uaOzViWudtgouS8G8USltZPUjYx9-RijvTw
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
✓ Created SWR script: fetch_api_user (4Jc6Ifajdlz3EQjXUipVagoee5eVDJ_Hq1PGqEXT3Lmx04R-Qe_8ApS0D47ZDlnnEY9OGn_oF-rd-drTyO4a7w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (jlndLWMiCeDY-Q7n-0zv8FR614Ko9q8Os3q_6e622YOsDkmxeEHQTF-z1Bq2C8UirkqJsm5AkY-CfhnghTywIw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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
   {"count":5,"avg_price":367,"category":"Electronics"}
   {"avg_price":365.6666666666667,"count":3,"category":"Furniture"}
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
   Vector databases have several benefits such as:

1. High Resolution: They provide high-quality data representation, preserving the precise location and shape of geographical features.

2. Scalability: Vector data can be easily scaled up or down without losing quality.

3. Less Space: They can often use less storage space than raster data for large, less detailed areas.

4. Flexibility: They can represent different types of data including points, lines, and polygons.

5. Attribute Storage: Vector databases easily store attribute information, providing more descriptive capabilities.
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {"avg_score":70,"role":"user","count":7}
   {"role":"admin","avg_score":20,"count":3}
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
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"AI","count":2}
   {"category":"Database","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: cCTbi43y62OYHyTbqEf8g4kL6ATnISdC8ofkyh4kcPbUARjm81LRW8sfyFGLX7T7xvbZLD_08Uci_raZrIgmVw

=== Sending Chat Message ===
Message ID: 3teCjjXtIB0GexpRs5V1BxoUdJTMiQW3747t0JLXhO3PqMalyTXq26_q9I_1rTmkoEOXjdLKNPN0KLzKl8QTRA

=== AI Response ===
We have three products available:

1. "ekoDB Pro" is an enterprise edition product with advanced features. The price for this product is $299.

2. "ekoDB Cloud" is a fully managed cloud database service product. The price for this product is $499.

3. "ekoDB" is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: 'tboN-YAXg4HWj0qPPqUpBoY0cDyRFSjBXLS95Ert81SYKYgZZ-4jp0f3w7EeOMTqeVltH3zyTFcodMkf_nTEgQ',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: 'FmhpLJWbr43xksidhHJTDkrWDTF_g9HsUI2_QaYukkmV6YLCqHxMhYyKdAs2Tesp6yxlqoyrv0DhdpNbDKkkDQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'RcyNU6yhcOSv6H7ZjBWwcH6jkbz6AEjdK0fc_CtmNZHBdy8IYZjctFOKtAbTUm4rwVeqJ3sLkr9xfy3QFZO_Wg',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2258ms

=== Token Usage ===
Prompt tokens: 448
Completion tokens: 81
Total tokens: 529

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: oQCVs41ExdTpWbFgvFvZarwqvwTib3gX4vP_COaZjkxg6TTxykbH9NyEbmT5MFDLHiGanx52WsZucIeh60iing

=== Sending Initial Message ===
✓ Message sent
  Response: The product available is called ekoDB. It's a high-performance database product priced at $99.

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
✓ Created second session: zLj1i9P9gFeDCksqf0ll8FGRv6gHp6Afp1qLA45CyG3Iorxhlt0XLEsoDaei5TNm6rWTg2ArXM9kdbmY06GxVA
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
✓ Created session: zI2BbtcsCgihdjDA9HAtYTbM1DUUPzL-XQruo1fAU8qhPzCDzdL5MAxLnDSwC7AL6V4BmzZjU6K2Nl7YGC1eNQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the provided context is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fX7F5zuk46Iga53FFWBe26rcSPvY6AmjHrBdsmKxQGWgsw09BgQTaxHCUN62Mt9xXkOdbMUJFlcGNpnrDSg20g
  Parent: zI2BbtcsCgihdjDA9HAtYTbM1DUUPzL-XQruo1fAU8qhPzCDzdL5MAxLnDSwC7AL6V4BmzZjU6K2Nl7YGC1eNQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fX7F5zuk46Iga53FFWBe26rcSPvY6AmjHrBdsmKxQGWgsw09BgQTaxHCUN62Mt9xXkOdbMUJFlcGNpnrDSg20g (Untitled)
  Session 2: zI2BbtcsCgihdjDA9HAtYTbM1DUUPzL-XQruo1fAU8qhPzCDzdL5MAxLnDSwC7AL6V4BmzZjU6K2Nl7YGC1eNQ (Untitled)
  Session 3: zLj1i9P9gFeDCksqf0ll8FGRv6gHp6Afp1qLA45CyG3Iorxhlt0XLEsoDaei5TNm6rWTg2ArXM9kdbmY06GxVA (Untitled)
  Session 4: cCTbi43y62OYHyTbqEf8g4kL6ATnISdC8ofkyh4kcPbUARjm81LRW8sfyFGLX7T7xvbZLD_08Uci_raZrIgmVw (Untitled)
  Session 5: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ (Untitled)
  Session 6: ltoCE9Mp-YHRJtsl5hQz6NbEq3eQLlHVQko1hD7fAv4kWIRFPEQ8lYFYcwbEKxKjiqcO4ZH5mj5uSn5vyDHDEw (Untitled)
  Session 7: iOoS8YtheulgTtAeIuuE8D3bhrehIrH7-gMf9kcjqZ1WhiK4u5IFgblmUK0mnvBgQq4b_DIusk1CVO0wAc81Ew (Untitled)
  Session 8: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg (Untitled)
  Session 9: hfFbxSDUNbfSKyxC79AJeZd7GCrQRPZm1gkmWSev-oxoy7vEStTatUY4EFqw212ot2-7w95yye9aE4-_D1Ioaw (Untitled)
  Session 10: qO-Z-fYTrhxTnTF_RGGmi6IGq6Cw3n5Wtt4IzSVHy-EGRp8ZpGeXPf6yW9lO2L5vAbnhl3E9xRUrLPDM6uHP9g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fX7F5zuk46Iga53FFWBe26rcSPvY6AmjHrBdsmKxQGWgsw09BgQTaxHCUN62Mt9xXkOdbMUJFlcGNpnrDSg20g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'Nvw-1P28ZkOrEYUsN_vDsc-RmA2WHcfVp4n8MSZBVzH3bU8pkdfoEpCssILbY1Lgh4_ZpuxDfbH9w-xJ9sByNg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'Nvw-1P28ZkOrEYUsN_vDsc-RmA2WHcfVp4n8MSZBVzH3bU8pkdfoEpCssILbY1Lgh4_ZpuxDfbH9w-xJ9sByNg',
  age: { type: 'Integer', value: 29 },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'Rign7R0oxx2gG9rlWM4IEup-nl4gIxvz8ReDSlHsTqYr30oSDfUiL6NqwzByOEoUi_qIp9X8r6-UfowzW9qltQ'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  age: { value: 29, type: 'Integer' },
  id: 'Nvw-1P28ZkOrEYUsN_vDsc-RmA2WHcfVp4n8MSZBVzH3bU8pkdfoEpCssILbY1Lgh4_ZpuxDfbH9w-xJ9sByNg',
  name: { type: 'String', value: 'Alice Johnson' }
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
   Inserted with ripple: {"id":"-DUoKnBmQ28ImH8caMTjWSQ2mBbqsGRQinIbZBbDnCyrbtSiMzow2IVBYrYHQC1gv6U9PXVfcDUE71sIFBcVYQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"DONR9Mk0xF07IVp72TIf2LBe0eK59W9K7sfjsv6LC0FvOhlhYVAImky04nueOAk8-eaXRxU3sRYUcIyNsi0H5g"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150},"id":"-DUoKnBmQ28ImH8caMTjWSQ2mBbqsGRQinIbZBbDnCyrbtSiMzow2IVBYrYHQC1gv6U9PXVfcDUE71sIFBcVYQ"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"ZydUgTtlF11M7KIbOxOxyGl5WUOJdtoY7higeq7t7uRfv9dYqmWKVGt1ncBapzQv7hQhOTIjx6fzxgRIxBCupA"}

✅ All bypass_ripple operations completed successfully!
✓ Client created

Setting up test data...
✓ Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: [object Object]

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

🧹 Cleaning up test data...
✅ Cleanup complete

✅ All projection examples completed successfully!
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 826ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'zEOSe9-pdnYWlF8kkg1ag04_c_5uyQmJPO3AaDV0p4ndjgeVB3aFB2EuYySCxyg47aFRdKrtvWO9RtTc4C2Skw'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'zEOSe9-pdnYWlF8kkg1ag04_c_5uyQmJPO3AaDV0p4ndjgeVB3aFB2EuYySCxyg47aFRdKrtvWO9RtTc4C2Skw'
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    id: 'zEOSe9-pdnYWlF8kkg1ag04_c_5uyQmJPO3AaDV0p4ndjgeVB3aFB2EuYySCxyg47aFRdKrtvWO9RtTc4C2Skw',
    name: { value: 'Test Record', type: 'String' },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'zEOSe9-pdnYWlF8kkg1ag04_c_5uyQmJPO3AaDV0p4ndjgeVB3aFB2EuYySCxyg47aFRdKrtvWO9RtTc4C2Skw',
  name: { type: 'String', value: 'Updated Record' }
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
✓ Inserted test record: ybK_WRQ5hbJNMIFZgLlTce556xKGqzbq75nO-uNmYeky9iuUpNyx3Gzc3hf4tjIB9jOVR0LBeH-L2sPZNZ_qKg

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "ybK_WRQ5hbJNMIFZgLlTce556xKGqzbq75nO-uNmYeky9iuUpNyx3Gzc3hf4tjIB9jOVR0LBeH-L2sPZNZ_qKg",
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
        "id": "xqlWFz-xcGkzGrnXOkLAPRhrmrsYJH2KINL4W-8E2QvjvbU_O7jY19A6D8om-LLjx6BB85QBiBvAz0MPk3ORjg",
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
        "id": "A4qF-pqdwkFsJuqfXLHpKbs6oAE_YPGEhnmgThECK-CXukWAgzfuHETj4unrK6t9mwdM3JK25MnDvj8OHX8BYw",
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

✅ Script saved: kY0NUhTwl_Rf9XK-_A1NqlMPe1v56eIfNSTbs5RtcOdlciUC3NBdUZlgjurJtuad4NsvxESH1kbw0db9_OkM1g
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: svFkeAdtUM4j2GMDK2F_TLoU3hPZxFtQFNzVbX5g6eiaL5xUyjZd-BKJ9D2GjVjY0mpVeHKkb8EWxtAMTnTBiw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: zlggHCuwheVqu8N_EIZ_nUwetvo-Xf4nYk5bwCQwSW-EnlU3J1taoHW8cfbZH9xfqoeofpC1aAYdFzfIH2q4rA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"status":"active","max_score":100,"count":10}
   {"status":"inactive","avg_score":50,"max_score":90,"count":10}

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
Retrieved value: { username: 'john_doe', userId: 123 }

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: CsrTd7fvgR_rpHxFE-J3dohQYq6_R6BDKvZnec-HkujsCzwDqRJD6MYALAF85Mh_pVblAzlMhmVqjsIs0s6LVw

=== List Collections ===
Total collections: 24
Sample collections: [
  'chat_configurations__ek0_testing',
  'chat_messages__ek0_testing',
  'demo_collection',
  'scripts__ek0_testing',
  'schema_products_client_go'
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
Created Alice: $1000 - ID: n7OV7PBRn05g5zYo5hcMEgxzl6GW2VLVEXlka-Flpk0UIdQKTrgFSXqxE3YeZNf9iw0OHfbbU95tMjPTKCV1rQ
Created Bob: $500 - ID: uxSKqX9dYnhldkjJxAU7Ev8m2iKi7kAByVNYtf8_FHTZwn14XFfORMgZq-F9Vn4ACS2ULyfDcUGpCLf8ky-NgA

=== Example 1: Begin Transaction ===
Transaction ID: 08f3209b-a8c8-43a6-b205-dc939e3d8b83

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
New transaction: e47430ef-ac67-438b-8d0f-84fa02609049
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
   ✅ Script saved: HW2LOWHVHlbn7RdBf4R0gVL_6rOxvDXe3LsEbWYOL5eRT53vNQ0JBBjOS1Mu-qYcZWBpnKRs2KgshMd_zDw91w

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
   ✅ Script saved: unCNZzucH7mmglOITJoTJwx20tz58kpi-RWr_BIXe9Wtt7MyOLtw2J9BKXdSsBizg1g9_uPSPE-IP2VH2MkC_w

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: fVzHHp__BLJ3IeB6fF9yrSdR9dDGKFaxv_5nuffy6AhcNUqOJn1rhR0uX36XoV87J0QO6kywcVxatqYOFDtW0A

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
   ✅ Script saved: pOweenChVuzktl86CDrMzdxSoORkA_SaD3xDOWHJwc5FOp_J8MOeW5foCZG7RI8Mx9GsvNIIs4InVy5VUAY6Vg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: HW2LOWHVHlbn7RdBf4R0...
   ✅ Deleted script: unCNZzucH7mmglOITJoT...
   ✅ Deleted script: fVzHHp__BLJ3IeB6fF9y...
   ✅ Deleted script: pOweenChVuzktl86CDrM...
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
  Output: Document ID = LPhTO954HfDJUseExMwvMRNal1t7eoPRnyf1FVb516SsfSfRdEvO5kIAtx417GnW3S7yEGPKKImEMWTgV0tjQg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(LPhTO954HfDJUseExMwvMRNal1t7eoPRnyf1FVb516SsfSfRdEvO5kIAtx417GnW3S7yEGPKKImEMWTgV0tjQg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(LPhTO954HfDJUseExMwvMRNal1t7eoPRnyf1FVb516SsfSfRdEvO5kIAtx417GnW3S7yEGPKKImEMWTgV0tjQg)
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
  Output: Document ID = 7rR8JggRSCxThA2MIy9koekJwaKXLzOL6S50JSbU5Lfx8kYtTtEFAwbaaBxvf6iNqAuDAhjulX--cCCEuyxZgQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(7rR8JggRSCxThA2MIy9koekJwaKXLzOL6S50JSbU5Lfx8kYtTtEFAwbaaBxvf6iNqAuDAhjulX--cCCEuyxZgQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(7rR8JggRSCxThA2MIy9koekJwaKXLzOL6S50JSbU5Lfx8kYtTtEFAwbaaBxvf6iNqAuDAhjulX--cCCEuyxZgQ)
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

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 842ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'sxdSzpJXp3zEoC1o9T0O5sq5BR-DzHuryzNvF86WpWmgW6Qy0LTcfIe7xugnnYwfhADN5U8ufcp2cldy_CtDPQ'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  value: 42,
  id: 'sxdSzpJXp3zEoC1o9T0O5sq5BR-DzHuryzNvF86WpWmgW6Qy0LTcfIe7xugnnYwfhADN5U8ufcp2cldy_CtDPQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'sxdSzpJXp3zEoC1o9T0O5sq5BR-DzHuryzNvF86WpWmgW6Qy0LTcfIe7xugnnYwfhADN5U8ufcp2cldy_CtDPQ',
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: QVSJWpc-1-cst19kCbd7K1TnnFHVbttKPkeO0ozV70XxN9raPSaYSif5wpNK89tVsG4V4BE4W3mp8SguJJyRNA

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
Collection created with first record: lhMfdCNxyR_OXrdfqfuy8aDhmsxV0zCp6hLfysFT0HXCjGKPPjDq8t1bM2aiBE5r3H95YSCSZqEDd3IZy3RBKQ

=== List Collections ===
Total collections: 23
Sample collections: chat_configurations__ek0_testing,chat_messages__ek0_testing,client_collection_management_js,scripts__ek0_testing,schema_products_client_go

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 10

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tC_XIY2jfXd9dDJlXCKA7wVQImisbn2up6_x1uU5vIjCHSFRhcxizPSgqo2SmfkAxAKANoI4ciDsBSEVODeAsQ
Created Bob: $500 - ID: s3rbbeiAkTFsH9-Cn3vmtXAeg-2okHlgSbVONwJ_feyL396HaKDgBP4q2OxjomCb1Z_D9qL9zocwVX7Kl4qI6Q

=== Example 1: Begin Transaction ===
Transaction ID: dd9553ac-b8f4-4c71-9bd5-29cd178e735c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: fee1d621-a873-4344-b697-9fb0e1c41edd
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
  1. Score: 25.740, Matched: email, name, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio.value, title, bio
  2. Score: 26.400, Matched: title, bio.value, title.value, bio
  3. Score: 26.400, Matched: bio, title.value, bio.value, title
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: bio, bio.value, title.value, title
  4. Score: 39.600, Matched: title.value, bio, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.767, Matched: 
  2. Score: 0.764, Matched: 
  3. Score: 0.752, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706, Matched: content, content.value, title, title.value
  2. Score: 1.507, Matched: content.value, title, title.value, content
  3. Score: 0.301, Matched: 

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
✓ Inserted document: P-DZV1gCKsFUf5P_SGc41HTK_y42jZDbAk_NKMFvHpMC3xAHyrqxbz4Kh0Mwwz0Otg_YTwyA-ay1o68EUZHxGw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 84Sz7_RTTfN8hv6bTIVI3qBrO7S1c2ZCDBnQ6XiwJHKDVGQNMf0cY1WyfIxYHW249d0nK96A8pp1va0uta0eSg

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
✓ Inserted document with TTL: y4NrwTcfzcYiw3mcz0nW_W7LRmVLq06dDLSDEPVnIn783ZVjevsTMJRO3zx9PA9KNYHuTeeygGhV1R5qLquXQA

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
✓ Edge cache script created: kMiH9zmXyZ2ZB0N067PRdsz3n-Z8F7TpYPGkENwP7GjmWiC_7quPF1YUaMbxyBd1ezkLgGs1THv2wDmZBW2JOQ

Call 1: Cache miss (fetches from API)
Response time: 5ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 5ms (1x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: 3x4r3S5bkAxFrGHHcLv7W_V_otNzEG0pzfOVzREZeMEbTvcCiKVKlZCBp3pVsovrEduDSRatD_EGamFlMBSucg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":60,"count":5,"status":"active"}
   {"avg_score":50,"status":"inactive","count":5}
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
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 68ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82ms
   📊 Records: 1
   🚀 Cache speedup: 0.8x faster!

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
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: juhpsSqzfn5jRqNaNKXO_N8HghwOYHeQNezomPzFXLJteYrXmybuyJtEDkgevNJJSV3F_0zwru7UR8ab0GDKTg
📊 Found 2 product groups
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: DuBAEGM_Le6l4dihNXLUjzPEaD61wvqT2DqCBM-4ZcTKBv7x_AsAHaTgYaOD_rJenoQ6joYgQ6KWEwpZKPyTyg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: YQLOaDrDzhmyWVgqG28CKSqEQjI1FbWVjMo5Orj12Kcmy7FEB0XbzMov_uB5rbOhzyU-WvXtXqPPqox2nPaozg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Jk0VZhAROBExc5q81--tABnK5_g7W-TFz4pD7s77keB5GGWxrQBkZsJPH8HhfhRKL9TmjkquDTGjqpWMDceWgA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 2ojroaPHVBMCuc3ABf_md8NjYT7YE5l7rtygE-m_TbmGF-AlCpgzjnPbtJK5spRbsUCnJnNO12VcftkVY8QIqg
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
📊 Users (showing first 5 of 5):
   1. User 4 - Score: 80
   2. User 2 - Score: 40
   3. User 3 - Score: 60
   4. User 1 - Score: 20
   5. User 5 - Score: 100
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
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
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
   1. Database Design Principles
   2. Introduction to Machine Learning
   3. Natural Language Processing
   4. Vector Databases Explained
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Introduction to Machine Learning
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
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
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Bookshelf - $149 (⭐4.1)
   2. Laptop Pro - $1299 (⭐4.8)
   3. USB-C Cable - $19 (⭐4.3)
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
   Vector databases have several benefits:

1. High Precision: They provide more accurate and detailed data representation as they use points, lines, and polygons.

2. Scalability: Vector data can be easily scaled up or down without losing quality.

3. Less Disk Space: They typically require less storage space compared to raster databases.

4. Data Manipulation: Vector data can be easily manipulated and analyzed. It supports topological analysis like network analysis and proximity operations.

5. Better Visualization: They allow for better visual representation of data, especially for features like roads, buildings, and other infrastructure.
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (na8gZdN8em110bNrownyqNejjOGdZupMUtkB3iVm3kPNYAb-3b75aGqOnyxiwnuRdbrdjBA-m1SQAloRSjNodw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (served from cache)
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
✓ Created session: rabMxHA-ANqjuJ_pCJzsGobYdjEFwnzVErBEqmcNUqRNqsaxFiQ2tOog_mxWfE4V0JzY3kvmelmSFoIFM3LvCA

=== Sending Chat Message ===
Message ID: o45nf6qS5Cf0g58q13DmS-SFHQCST1HQ-hutihLDvP24XGTi8DZKQiZPAPEsPEb2fI_e7GSuL70ZFA-9rUmzjw

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.
2. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.
3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    id: 'ckWtyjpJLYF7n031vAw-3HzGcilNSSjqi3YPymEJm67GZ6G1VuxxVSxjPd1ztSROfzSSe4jXORO15eMOK9Mc0g',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: 'rkTkFV3AaRhSbdl2Gsk9dznI5Idy5J0noTpIl3JwLm6PnXGxQTaHAZW0lyFcKUpZQa6NAdNtdBh9S8fwOKt5Fg',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    id: 'Fvbx52qBxGNyv2h11tj1SJMylfgZ4o6_fQWYyC265DB3xr7ufQZkjrBLajsaxz1OWpVrU3n1CKTTlf-Y7KklRg',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2436ms

=== Token Usage ===
Prompt tokens: 456
Completion tokens: 78
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: kE4YBFaG8DBMiEQmia79C4Rwgwt2JMUI2bXqK71SyC_13m0gRmnIK-t1FXMgZi_7nCCDTaL3iKHoXTngfQ3Yuw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, "ekoDB" is a product that is available. It is a high-performance database product and it is priced at $99.

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
✓ Created second session: EsXVr0JjLOljobaQLDhs2c9zU2cehpHJnQaKF2XjfYGrUNZxk1yhZrDhlDIH2eUB6DtFii78sTN4BVCCVOS6lQ
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
✓ Created session: B8NDRQztgVLRCEzVIjKSZ_g4ISm0L0KZEtvnqwADmejdHPROpsUaCrnohlwYnsYKKRry8a-UMkSUZ5bJvDvRRA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, one product that is available is "ekoDB". It is a high-performance database product and it costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: h2NLXfMd5VLN67oLUuLpS0MwRBYAUrIlcf980Z-43sel5nj_y6D6wvLAhBQlVDGNovBQ4xuLSgVNPhCK3zBvdw
  Parent: B8NDRQztgVLRCEzVIjKSZ_g4ISm0L0KZEtvnqwADmejdHPROpsUaCrnohlwYnsYKKRry8a-UMkSUZ5bJvDvRRA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: h2NLXfMd5VLN67oLUuLpS0MwRBYAUrIlcf980Z-43sel5nj_y6D6wvLAhBQlVDGNovBQ4xuLSgVNPhCK3zBvdw (Untitled)
  Session 2: B8NDRQztgVLRCEzVIjKSZ_g4ISm0L0KZEtvnqwADmejdHPROpsUaCrnohlwYnsYKKRry8a-UMkSUZ5bJvDvRRA (Untitled)
  Session 3: EsXVr0JjLOljobaQLDhs2c9zU2cehpHJnQaKF2XjfYGrUNZxk1yhZrDhlDIH2eUB6DtFii78sTN4BVCCVOS6lQ (Untitled)
  Session 4: rabMxHA-ANqjuJ_pCJzsGobYdjEFwnzVErBEqmcNUqRNqsaxFiQ2tOog_mxWfE4V0JzY3kvmelmSFoIFM3LvCA (Untitled)
  Session 5: zI2BbtcsCgihdjDA9HAtYTbM1DUUPzL-XQruo1fAU8qhPzCDzdL5MAxLnDSwC7AL6V4BmzZjU6K2Nl7YGC1eNQ (Untitled)
  Session 6: zLj1i9P9gFeDCksqf0ll8FGRv6gHp6Afp1qLA45CyG3Iorxhlt0XLEsoDaei5TNm6rWTg2ArXM9kdbmY06GxVA (Untitled)
  Session 7: cCTbi43y62OYHyTbqEf8g4kL6ATnISdC8ofkyh4kcPbUARjm81LRW8sfyFGLX7T7xvbZLD_08Uci_raZrIgmVw (Untitled)
  Session 8: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ (Untitled)
  Session 9: ltoCE9Mp-YHRJtsl5hQz6NbEq3eQLlHVQko1hD7fAv4kWIRFPEQ8lYFYcwbEKxKjiqcO4ZH5mj5uSn5vyDHDEw (Untitled)
  Session 10: iOoS8YtheulgTtAeIuuE8D3bhrehIrH7-gMf9kcjqZ1WhiK4u5IFgblmUK0mnvBgQq4b_DIusk1CVO0wAc81Ew (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: h2NLXfMd5VLN67oLUuLpS0MwRBYAUrIlcf980Z-43sel5nj_y6D6wvLAhBQlVDGNovBQ4xuLSgVNPhCK3zBvdw

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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=bIlZbaEyOY5WcfBheQ7LDIwZqEDoQG66u3CRNGDCjPJi0IJBylY199fsAYkjgbDqKIqhJ2aauIURAMr8j-Db3g)})
  User ID: bIlZbaEyOY5WcfBheQ7LDIwZqEDoQG66u3CRNGDCjPJi0IJBylY199fsAYkjgbDqKIqhJ2aauIURAMr8j-Db3g

=== Read ===
✓ Found user by ID: EkoRecord(fields={price=FloatValue(value=99.99), metadata=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), id=StringValue(value=bIlZbaEyOY5WcfBheQ7LDIwZqEDoQG66u3CRNGDCjPJi0IJBylY199fsAYkjgbDqKIqhJ2aauIURAMr8j-Db3g), created_at=StringValue(value=Mon Jan 26 01:00:01 EST 2026), categories=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), embedding=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), active=BooleanValue(value=true), age=IntegerValue(value=28), email=StringValue(value=alice@example.com), tags=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), data=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), user_id=StringValue(value=550e8400-e29b-41d4-a716-446655440000), name=StringValue(value=Alice Johnson)})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Mon Jan 26 01:00:01 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: price, metadata, id, created_at, categories, embedding, active, age, email, tags, data, user_id, name

=== Update ===
✓ Updated user: EkoRecord(fields={active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), id=StringValue(value=bIlZbaEyOY5WcfBheQ7LDIwZqEDoQG66u3CRNGDCjPJi0IJBylY199fsAYkjgbDqKIqhJ2aauIURAMr8j-Db3g), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Mon Jan 26 01:00:01 EST 2026)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), created_at=ObjectValue(value={value=StringValue(value=Mon Jan 26 01:00:01 EST 2026), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), id=StringValue(value=bIlZbaEyOY5WcfBheQ7LDIwZqEDoQG66u3CRNGDCjPJi0IJBylY199fsAYkjgbDqKIqhJ2aauIURAMr8j-Db3g)})

=== Delete ===
✓ Deleted user with ID: bIlZbaEyOY5WcfBheQ7LDIwZqEDoQG66u3CRNGDCjPJi0IJBylY199fsAYkjgbDqKIqhJ2aauIURAMr8j-Db3g

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
  {"payload":{"data":[{"id":"HcE33xTAP6AqRGslRR1407fP2lHNh1MBo1RVZbKbQIFpS3PKQ3TMXsxaIckQnUR7BjKBqqgyGouYmjTeODwEYw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
  IDs: Lb7ew0z9cceYlWfyxQ2VAVJEG38Tcr-ySJeyRTcDvP8sVkqma1HTJk1tI30DyTUA7JwWBcUoD73P1JEMdMR9Aw, s4-aUdnsET8hVwZwTt2SyMVhySVb-KyhMEtZSstgKPOMC6gv8ypk9jvQnboq4cdrQab0WCLSUaOE1nUrQLCl4A, q04938ddxFaIeGHt07IToMYcChUOjdSws4zo4Lay_OsBHFXeD-5WTjlMByuS1t4WDt1LOQ32cLut5RJI-ytgKw...

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
✓ Found 27 collections
  - schema_users_client_js
  - schema_documents_client_js
  - chat_configurations__ek0_testing
  - chat_messages__ek0_testing
  - schema_products_client_js

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-26T06:00:14.353302Z","last_modified":"2026-01-26T06:00:14.353302Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Session value: {"created_at":1769407218549,"user_id":"123"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"ttl":3600,"enabled":true}}
  config:api: {"value":{"timeout":30,"retries":3}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 11

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  config:db: deleted
  config:cache: deleted
  config:api: deleted

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
Created Alice: $1000 - ID: E4rhZ_uoa3sLOu5Dn-908_nXoBc-G6wSj8Ok95n_UJ2KlEwryhg5ovWWCuWiFVJH59UYrB0QfLyEtDIugGdjKg
Created Bob: $500 - ID: W7wkPbGqF9Kq7Tpv8UZ16nRErGycZBg-t9O7JrWKMFr95KObnrH8GZ3nANVsVmcXP4FFWb9gbMXpnEHV_Clotw

=== Example 1: Begin Transaction ===
Transaction ID: 064b5f04-9f6f-437d-8a82-7c6cc7e78b42

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 19f3f6cc-be45-4518-8661-7402847e9146
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed

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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","id":"CTyJp0lIniVf51Tqkdf5EKUJbbYNzHIQEMcElE7xQGaJzDJMpC2bXemDsDEzszMFebu6D_6pNYFpzMua-XroWw","views":600,"title":"Rust Programming","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"views":352,"description":"Master Python for data analysis, machine learning, and visualization.","id":"wFT-8t80IidLrFSN1L-WAf1iKl4IkPd1KMkOKOk8UanaYIPdGm0cWrqSCmSnp6FLT185BRmY9O1eqCFQ0toMAQ","title":"Python for Data Science","tags":"programming,python,data-science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","views":347,"id":"St1J3NF4AFSSDd_uMDtmfd1o2jN3yWrqTXXTWSaskSKLcKPUzIEommYeOiz0EyCWst-3oHDDidyZ2aMB6YGv_A","tags":"programming,javascript,web","title":"JavaScript Web Development"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"QQ7fML5GT75y-_2oHz5n2nfFb55oy40M08R1WEidm9vjXGDZ0AC19OW0Uu2-iqld8-y8L0RAa4XlokgH8Ffu6A","title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","tags":"ai,machine-learning,python","views":414},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","id":"wFT-8t80IidLrFSN1L-WAf1iKl4IkPd1KMkOKOk8UanaYIPdGm0cWrqSCmSnp6FLT185BRmY9O1eqCFQ0toMAQ","title":"Python for Data Science","views":352},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Database Design","views":182,"tags":"database,design,sql","id":"INAVdbNW07C2NrVbhBELmn_pbJlElQlmAZjfD2wEB_MBP2rSdHRz95KX_jzXl8zNTHMRcnxlVafX1VeUMRt8zA","description":"Learn database design principles, normalization, and query optimization."},"score":0.5,"matched_fields":["description"]},{"record":{"views":600,"id":"CTyJp0lIniVf51Tqkdf5EKUJbbYNzHIQEMcElE7xQGaJzDJMpC2bXemDsDEzszMFebu6D_6pNYFpzMua-XroWw","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted user 1: StringValue(value=MMv9jR2ANAuRdudOEvVv5znMjDDHnRnhPChNtS3mgqXLxOwT3m1FS6LqdxbBI32pMiB_BYYIqTFRmPVcEyKsfg)
✓ Inserted user 2: StringValue(value=ewWqQJgoWEp34hSwxRUzqsl_SCjMQSbjfmjExfxm0V-kJ2RJZXDl-aqpboxKgD0x5PCKK51Fa7YkOw6MUnyY2Q)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 28
  Sample: [schema_users_client_js, schema_documents_client_js, chat_configurations__ek0_testing, chat_messages__ek0_testing, schema_products_client_js]

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: 7JRs_y9HszpjjgTasipIsDDJw2CvPyip2Vp8IkSA-72cMkfW4PSLAm7AwREE80ZnZ-WsfkQrKmjfkgD22KiQEg

=== Verify Document Exists ===
✓ Document found: session_id, created_at, id, user_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: fZriNHZDXWb0jyPMzx7F7A1rorHEBp9aqKyQCxeNITnPCl3WWNtPhF8Ax-Fp0vUOJjxlut5vfwovxpPEbJAaEA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: value, id, cache_key, ttl

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
✓ Inserted document with TTL: StringValue(value=BI0-RtVwwtnOcdDyy1emTSg-wLLU8oKJPKItBieLr4rlUxjnR4w6v_e-KPYRixU9F3_o9r8vJNVsWncTcKgl-A)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1769407248485},"id":"BI0-RtVwwtnOcdDyy1emTSg-wLLU8oKJPKItBieLr4rlUxjnR4w6v_e-KPYRixU9F3_o9r8vJNVsWncTcKgl-A","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-26T07:00:48.603653Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Edge cache script created: aAuq39vQ091q0ZjeGzH_tgv47V_u7N8ktP5nJUw8VL2k56KR5EBkPn7mSZSyqqlV3LyGsYosKOYqXdtedVBHyw

Call 1: Cache lookup
Found 1 cached entries
Response time: 12ms

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

✅ Script saved: djE9Xnt6YXf7aT30D-9nk6ew_9G1f1rEpUVhmacOeaZ3qv5ttdFdaCIfScrZjoawn-8lpOVb_ffpSXeDc3eJnA
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
   {"status":"active","count":5,"avg_score":60.0}
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

✅ Script saved: LODZvsFCOQj7u-Fo5dOhJNszx-x1qZvZI_JKVh2G7KL_ZlP2GphAPBUyOMKLe12BfAOoT0Kei8UF6YvRoGPbwA
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

✅ Inserted order: StringValue(value=aBSR0DtdtogWk0GDDn3l-H_w_uLk1DWx36z_1de8lVX0iONRakCARo-nQlhBiDe2HXIvIsLrgZIEEN6sRCOXhA)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: JGD9fyBJ4Z_5HmWx0D-yYVu3JTUbFqTt5LrTgACf-CWyt4-Y5W3RJOJIqHZb-QJ7CQ8WgRnSPVWmDXACtu_7sw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: y5NvF49HpiBu5L9xXTiltajreuP4tqWGyTEhDUx1wKD84Tq3rSfIndpGw_TZzrWKHyJ_ZONKRxtnhzX1-y9GpA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: 4YHVTAHVf69hyF21n5eS1Nkb2R_bRWkzwt7pJmVaSEeR8EZI2TvnUKzF6gagsMRUuyGFhj-ngQGrnrbqePBb0g
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup script
✓ Created SWR script: swr_cache_lookup_kt (EEIfGWzRZuCRLp511G_jxT-0Saub3W8M9sEQt_EoMlMrZS8uEmLSyS_T5RFKbf2cij8qAs2ZsKBbCStqLSqcwg)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 8ms (served from cache)
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
   {"avg_price":367.0,"count":5,"category":"Electronics"}
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
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
   {"response":"Vector databases offer several benefits including high accuracy, scalability, and detailed data representation. They allow precise representation of geographic details like boundaries and allow for efficient data management. Additionally, they are flexible in terms of data manipulation and can support a wide variety of analysis operations.","tokens_used":97}
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
   {"count":3,"status":"inactive"}
   {"count":7,"status":"active"}
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
   1. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   2. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
   3. {"value":"Vector Databases Explained","type":"String"} ({"type":"String","value":"Database"})
   4. {"value":"Introduction to Machine Learning","type":"String"} ({"value":"AI","type":"String"})
   5. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"category":"AI","count":2}
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
✓ Created session: p4S1kJZM8xCNvJ62UUo2gwj534ov16UhvufI2zLxosLLaX6KP5J66G10zgrC-LqMGVUEplcwS_ZSIJP4SEV_0Q

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "kwu_Ac2K3aavpfaAeLaHqwckDlxXJfX5atSMHpabCW5MfjalV0yYczKFzwKSpG6ts42p_RDQexlpnSh7a1vzFA"
  Responses: ["ekoDB is a high-performance database that boasts intelligent caching and real-time capabilities. It also integrates artificial intelligence for efficient data handling. It has unique features that enhance its functionality. \n\nOne of these features is the AI Chat Integration, which allows you to query your database using natural language. This feature provides AI-powered responses with relevant context, simplifying your interaction with the database.\n\nAnother notable feature is its robust search capabilities. ekoDB supports full-text search, vector search, and hybrid search, all equipped with automatic context retrieval. This makes it easier to find specific data, boosting efficiency and productivity."]

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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2WOBf0nyhjvVmr-K6cg9Je5M4U7kMjRLrJeGmxIUKbuTdWqueNldZVL5Ue_x3R2A72YRF0-rHRMDtgPPdJOZPA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is ekoDB. It is a high-performance database product and is priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Currently, we have a high-performance database product available called ekoDB. It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: EfrNZRk40ys2kn90av-wakoKLAQOUEFZDd_mRCIoVAH2NjXzB6ty8bkdnmQB3K_tp_WXn7qHtAX8nXxdNzNN7w

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
✓ Created session: 3v2f_mpQXCEvfwSkStBX1toA4QzeZ0Af3rzQLZykfpVrDj5X9b642xUoksF5WgTaiDIpK6KePkYoSFoNH1bEcA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the available product is called \"ekoDB\". This is a high-performance database product that comes with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB, a high-performance database product with AI capabilities, is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"3v2f_mpQXCEvfwSkStBX1toA4QzeZ0Af3rzQLZykfpVrDj5X9b642xUoksF5WgTaiDIpK6KePkYoSFoNH1bEcA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"LtO_BlOjk3ddY11IldlzCAV2s-5WYvWnYQIcwgVT4BFhjvJxg-BcjIuVTzZe84kqHykwCK6JEGsmaqm2Sf2n-A","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-26T06:01:51.125359Z"},"id":"w1nZNa_zUAylM5PViNHJKpDpTSUGyXz2N7Aj-CzNqFcY7CmbVi9n3vIdNpYFTyuqisAJkX4Idf-a5N5Yg5NALg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":35,"prompt_tokens":173,"total_tokens":208}},"updated_at":{"type":"DateTime","value":"2026-01-26T06:01:51.125359Z"}},{"chat_id":{"type":"String","value":"3v2f_mpQXCEvfwSkStBX1toA4QzeZ0Af3rzQLZykfpVrDj5X9b642xUoksF5WgTaiDIpK6KePkYoSFoNH1bEcA"},"content":{"type":"String","value":"Based on the information provided, the available product is called \"ekoDB\". This is a high-performance database product that comes with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"LtO_BlOjk3ddY11IldlzCAV2s-5WYvWnYQIcwgVT4BFhjvJxg-BcjIuVTzZe84kqHykwCK6JEGsmaqm2Sf2n-A","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-26T06:01:51.130587Z"},"id":"lfX_zUtT4ST98SLkwKGb1Sv4YAt1eN6mLyIgSCs3lj1xurr532gvUApTCPuWtwZ9hFJU2TJNKLfU9WbTz_IqhA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":35,"prompt_tokens":173,"total_tokens":208}},"updated_at":{"type":"DateTime","value":"2026-01-26T06:01:51.130587Z"}},{"chat_id":{"type":"String","value":"3v2f_mpQXCEvfwSkStBX1toA4QzeZ0Af3rzQLZykfpVrDj5X9b642xUoksF5WgTaiDIpK6KePkYoSFoNH1bEcA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"LtO_BlOjk3ddY11IldlzCAV2s-5WYvWnYQIcwgVT4BFhjvJxg-BcjIuVTzZe84kqHykwCK6JEGsmaqm2Sf2n-A","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-26T06:01:52.436678Z"},"id":"XXlRUsCMXlScy-vzgonsYCFqJpwEbzdl2HMBW-G7MCgAMlEQPDH76avYtWxxbvabxh6th9HvCsdEjBYMXW3pGA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":173,"total_tokens":194}},"updated_at":{"type":"DateTime","value":"2026-01-26T06:01:52.436678Z"}},{"chat_id":{"type":"String","value":"3v2f_mpQXCEvfwSkStBX1toA4QzeZ0Af3rzQLZykfpVrDj5X9b642xUoksF5WgTaiDIpK6KePkYoSFoNH1bEcA"},"content":{"type":"String","value":"The price of the ekoDB, a high-performance database product with AI capabilities, is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"LtO_BlOjk3ddY11IldlzCAV2s-5WYvWnYQIcwgVT4BFhjvJxg-BcjIuVTzZe84kqHykwCK6JEGsmaqm2Sf2n-A","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-26T06:01:52.445017Z"},"id":"1omUh10op_g8ElpLD0U6pLdcfnPMuDprJANnWA1BWRfS8mwaGxwfnSLQJmFgKUjoUn1JRub1WPMTssQb3C4Y1w","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":173,"total_tokens":194}},"updated_at":{"type":"DateTime","value":"2026-01-26T06:01:52.445017Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: G3yItH4JnJ88kQpqbNbeF60KQfXhYCHMg2IlpGO9o-6H1fJPGa1dEjLnDCImpmRB3IqK_l5xMrefLXPvzc4UhQ

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
✓ Created record: EkoRecord(fields={id=StringValue(value=hrNkW-NTUA-YdsxM1TN8l1lbvwQBAbNaCiulkKE-K-D_QJZ_O3ubt2L6VMlG8hTH3VcQR0j7gv6uv2ncRmBiVA)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={error=StringValue(value=Record not found)})
✓ Second upsert (insert): EkoRecord(fields={error=StringValue(value=Record not found)})

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
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=7TKiKQa3mesid0xdUfDtaBswy0eMPjeOvUlPZ_pvNv6CimY16EreC3orfAkmvG92pJSehxidaskSRuwCxw7ZUA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=bo8yvdT4XRT_4Jvv74NkgheVFFUB48UCA_Mj62_3ecPH59acuTc4xR3EDHrMG_Q9ZFdPNrnSwT7mnrj2Di90LQ)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={id=StringValue(value=7TKiKQa3mesid0xdUfDtaBswy0eMPjeOvUlPZ_pvNv6CimY16EreC3orfAkmvG92pJSehxidaskSRuwCxw7ZUA), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), id=StringValue(value=7TKiKQa3mesid0xdUfDtaBswy0eMPjeOvUlPZ_pvNv6CimY16EreC3orfAkmvG92pJSehxidaskSRuwCxw7ZUA), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)})})

✅ All bypass_ripple operations completed successfully!

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
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: m1_Cuq5KJ2Gdw86qD_jZGs7LD81vmhyxaY4YWYF6w3xtyU9znGYdU_BMau0NCC0iT-zN5TW3LVkn3mSgLpgG2g
Created Bob: $500 - ID: K-Fc9xjRJJLCU9XTBd9o3XZYOnwpmuuF7er6MJ0KKVmaRZUnnSfHRaiKB7QuIc_hwEitiWUtb8ITVCUJUCtgBA

=== Example 1: Begin Transaction ===
Transaction ID: fefc82b9-6d4a-4542-b906-b3465143b044

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6a7af0d5-dd6d-4ec8-a29c-33f2bedc61ce
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 417.27575ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 536.165208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 324.382458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 303.872458ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 314.031125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 337.784291ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 653.510292ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 276.511417ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 499.632167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 252.314375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 235.930541ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 369.916959ms
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
    ✓ Generated embedding: 1536 dimensions in 508.284833ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 110.746208ms

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

Writing memory-safe, high-performance database code can be achieved by considering the following principles:

1. **Understand your data**: Before you start writing your database code, thoroughly understand your data and its structure. This will help you choose the right type of database (SQL or NoSQL) based on your needs. For instance, if you have unstructured data and need high write throughput, NoSQL might be the better option (Context 1).

2. **Normalize your database**: If you're using SQL databases, normalize the database to reduce redundancy and improve data integrity (Context 3). This helps in writing efficient queries and also in maintaining the consistency of the data.

3. **Optimize your queries**: Whether you're using SQL or NoSQL, ensure that your database queries are optimized. This can be done by using indexes appropriately, avoiding full table scans, limiting the result set to only what is needed, and avoiding unnecessary joins.

4. **Use Transactions**: Use ACID transactions when necessary for data consistency and integrity. But remember, while transactions ensure that your operations are atomic (i.e., all-or-nothing), they can also slow down performance if not used wisely.

5. **Use Connection Pooling**: Connection pooling can save the overhead of establishing a new connection every time a database request is made, thus improving performance.

6. **Proper Error Handling**: Always include error handling mechanisms in your code. This will ensure that your application can gracefully handle any errors or exceptions and continue to operate, rather than crash or hang.

7. **Memory Management**: Be mindful of how your code uses memory. Avoid memory leaks by properly closing connections, result sets, and other database objects after use. Also, be aware of the size of the data you're working with to avoid out-of-memory errors.

8. **Test Your Code**: Regularly test your code under realistic conditions to identify potential performance issues and bottlenecks. This can help you optimize your code for better performance and memory usage.

9. **Keep your code clean and maintainable**: Well-structured, easy-to-read code can help you and others identify and fix performance issues more easily.

10. **Stay Updated**: Database technologies and best practices are always evolving. Stay updated and be ready to refactor your code to incorporate new improvements and features.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 282.94875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2412 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 268.208333ms
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
  ✓ Text search completed in 72.702ms

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
    ✓ Generated embedding: 1536 dimensions in 0.252s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.538s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.291s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.482s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.236s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.299s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.779s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.226s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.318s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
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
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.101s

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

Writing memory-safe, high-performance database code involves a variety of techniques and best practices:

1. **Use Prepared Statements**: Prepared statements not only help prevent SQL injection attacks but also improve performance by reducing the time taken to parse, compile, and execute SQL statements.

2. **Optimize your Queries**: Try to reduce the complexity of your queries. Avoid using "SELECT * " and instead specify the exact columns you need. Use JOINs effectively and avoid subqueries where possible. Use indexing for faster data retrieval.

3. **Database Normalization**: As discussed in Context 2 and Context 3, database normalization can help improve performance and data integrity.

4. **Use Appropriate Database Type**: Depending on your specific needs such as flexible schemas, high write throughput, complex queries, ACID transactions, etc., choose SQL or NoSQL as per the guidelines provided in Context 1 and Context 4.

5. **Data Caching**: Implementing a cache can drastically improve your application performance by reducing the number of database hits.

6. **Connection Pooling**: Connection pooling can help you reuse existing connections to the database, reducing the overhead of creating a new connection every time a user requests a connection.

7. **Use Batch Processing**: When performing a large number of operations, try to group them together in as few calls to the database as possible.

8. **Memory Management**: Always close connections, result sets, and statements as soon as you're done with them to free up system resources and ensure memory safety.

9. **Concurrency Control**: Implement concurrency control mechanisms to deal with conflicts due to simultaneous transactions and to ensure the ACID properties of the database.

10. **Monitoring and Tuning**: Regularly monitor your database performance to identify bottlenecks and areas for improvement. Use database profiling tools to gather detailed performance data.

Remember, the key to high performance is to balance load, reduce unnecessary work, and make everything else as fast as possible.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.285s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2088 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.279s
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
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.328s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.319s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.369s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.224s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.341s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.308s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.385s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.485s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.902s
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
    ✓ Generated embedding: 1536 dimensions in 0.314s
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
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     What is database normalization?

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of good coding practices, database design techniques, and thorough testing. Here are some general guidelines:

1. **Use Database Normalization Techniques:** As stated in Context 1, database normalization helps in organizing data in a way that reduces redundancy and improves data integrity. This can have a direct impact on the performance of your database operations and can help in writing efficient code.

2. **Choose the Right Database Type:** Depending on your needs, you may want to choose either SQL or NoSQL databases, as described in Context 2. SQL databases are better for complex queries, ACID transactions, and structured data with well-defined relationships. NoSQL databases, on the other hand, are more suited for flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data.

3. **Optimize Your Queries:** Inefficient queries can lead to poor performance. Use techniques like indexing, limiting the number of returned results, avoiding wildcard operators, and reducing the use of joins where possible to improve query performance.

4. **Manage Your Resources:** To write memory-safe code, avoid memory leaks by properly managing your resources. Close database connections when they are not in use, and use garbage collection if your language supports it.

5. **Use Prepared Statements:** Prepared statements not only help prevent SQL injection attacks, but they also can improve performance by reducing the amount of time it takes to execute the same query multiple times.

6. **Test Your Code:** Use profiling tools to identify any bottlenecks in your code. This can help you find areas that need optimization.

Remember, the specific techniques and practices you'll need to use can vary depending on the specific database system you're using, as well as the requirements of your application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.339s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1922 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
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
  ✓ Text search completed in 0.067s
✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.250s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.354s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.319s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.240s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.235s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.331s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.595s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
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
    ✓ Generated embedding: 1536 dimensions in 0.361s
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

Writing memory-safe, high-performance database code involves multiple strategies, including adhering to database normalization principles, choosing the right database type (SQL or NoSQL) based on your requirements, and optimizing your queries. Here are some tips to achieve this:

1. Database Normalization: By dividing large tables into smaller ones and defining relationships between them using foreign keys, you can reduce data redundancy and improve data integrity, as per Context 1. This can lead to better performance and less memory usage.

2. Choosing Database Type: According to Context 3 and 4, if your data is unstructured or needs high write throughput, using NoSQL can provide better performance. However, if you are dealing with complex queries, ACID transactions, and structured data with well-defined relationships, SQL databases may be a better choice. 

3. Query Optimization: As per Context 5, optimizing your database queries can significantly improve performance and reduce memory usage. This can be done through techniques such as using indexes, avoiding full table scans, reducing the amount of data retrieved, and using the EXPLAIN tool to understand how your queries are being executed.

4. Writing Efficient Code: Beyond the database itself, ensure you're using efficient coding practices. This involves things like properly managing connections, using prepared statements to prevent SQL injection, limiting the use of temporary objects, and releasing resources when they are no longer needed.

5. Caching: To reduce the load on the database, consider using caching systems to store frequently accessed data. This can significantly improve performance by reducing the need for expensive database calls.

6. Monitoring and Profiling: Regularly monitor your database performance and use profiling tools to identify potential bottlenecks or memory leaks. This will help you continuously refine and optimize your code.

7. Regular Database Maintenance: Regularly update statistics, rebuild indexes, and perform other routine maintenance tasks. This can help maintain high performance and prevent memory issues.

Remember, there is no one-size-fits-all solution, what works best will depend on your specific use case and requirements.


=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.289s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2256 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
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
  ✓ Text search completed in 0.069s

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
    ✓ Generated embedding: 1536 dimensions in 0.436s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.446s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.332s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.353s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.316s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.456s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.28s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.248s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.293s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.337s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.481s
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
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.098s

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
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 2.073s
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

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (GBuobEzcKiy_gj62QXyQUT-HHtSDtREdEqpQsQFccTxi2qF2SGvOpoZeKRfK5yQ43CjBErvZYklYrMJHG1ufsw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 6ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (6EEqsRqdyFUBxxsh4PB65HKuJUu0LcQaxmVnOevWPA63IubiF0fuX3BlPHN0dG-y7wQBHNoyctxycUdYqTqMcQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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
✓ Edge cache script created: sMCR9D795ZgPU3K8OyebnQ1pX50FWfHO7OPHWrhTsCy8byV2nzjkRYtAezp6rT23q-mJuNMukKNUmYel3u0MOg

Call 1: Cache miss (fetches from API)
Response time: 5ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 5ms (1x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (-YDIAMxOJImp3VsUIOPx84BvIIUDYH_D8Qc6hxYsulCNPx453jQgX8j1uLpRcZ6GAuHxm-0eGRa5kz3c2VSMyg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 134ms
Result: [
  {
    "cache_key": "torvalds",
    "cached_at": "1769407424",
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
        "followers": 280327,
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
        "public_repos": 11,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-01-13T07:10:05Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "ab38OQO1I3Evy-_YmtdXTXnFffxpd7-XR25LplnfomFSph2WF1kPmwAqFK5Cn02F34sGnJhdFDCixc5f-fXpSg"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 81ms (1.6x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (hQTsvMqQ7WceaPMH-5o8_C7FrVjljaad5e1wOAxto9JBCMe0V-edqCI1hPVk9bbX0iVwRnsKBKtq-p2p-5-vVQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "cache_key": "1",
    "enriched_at": "1769407425",
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
    "id": "-u9ZjG5vhRxZK9nHGP_XdbiSz6IfStGPAeLzwyhoUmnzmPn2LYi-a3-CIzoFQKct_J1sU0uJ-2qNDVw1uWbEfA"
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
✓ Created SWR script: fetch_github_user (-zhve80ehoawSek-JeJ4ZHXBWFVtW89T2jy9Kwes6xrTxjibh0aZdYgkDPusN3_pEf2np6s5me5n3M_0uIY81A)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 66.901958ms
Result: [
  {
    "cache_key": "torvalds",
    "cached_at": "1769407424",
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
        "followers": 280327,
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
        "public_repos": 11,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-01-13T07:10:05Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "4SH7oULt09tBcPcAqpuQXZYZoNXTG1NuDrqHaZz9GOD41p7nVrkkO8nukwv15JnjZsUSdGiFAB764229w_Hlmg"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 60.169375ms (1.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (SzX5ojTfwi3i54lb2HxCMbSAlMN6UJnXP5Gk_0dWWK8Cnhgq-HRoPjfYR_RaGi3T2dB_pYM3nzLUa09dl8WuIQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "cache_key": "1",
    "enriched_at": "1769407425",
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
    "id": "4EvcNoomL_Q6KUZ-plEvygwsnRh3qYY50cHpArzpOFZs_xRIPPccgPrSYLMM23G3mFyvUJG-if-H4M6S0UKLkw"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (szg9rAx7FsvtNV30iG7lTqlpzInLjrXtJArOrtU4R4IaB8iyWHqGpoOHC1n4etdvVsHGAIJIDCZhfbP8ILPmdg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 70ms
Result: {
  "data": {
    "value": {
      "followers": 280327,
      "gravatar_id": "",
      "bio": null,
      "created_at": "2011-09-03T15:26:22Z",
      "id": 1024025,
      "url": "https://api.github.com/users/torvalds",
      "public_repos": 11,
      "type": "User",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "user_view_type": "public",
      "login": "torvalds",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "hireable": null,
      "email": null,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "site_admin": false,
      "html_url": "https://github.com/torvalds",
      "name": "Linus Torvalds",
      "twitter_username": null,
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "updated_at": "2026-01-13T07:10:05Z",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "public_gists": 1,
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "location": "Portland, OR",
      "company": "Linux Foundation",
      "blog": "",
      "following": 0,
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "organizations_url": "https://api.github.com/users/torvalds/orgs"
    },
    "type": "Object"
  },
  "cache_key": "torvalds",
  "cached_at": "1769407424",
  "id": "bSRaf3Pqo6bhJufhI8U8SiBdZRBe2GwuNnig0wrUvzVnx0k7z7vPk7I7Oms4tVmrPgOExuXipk9pBVpnYNRdsg"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 71ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (OKfnIxAPtLYXQaz4DtNfePAVi19pMR-i4gbyzBKvu_vPpRVaxQ9JlzcnTGp7Xtom_dfkp4P90oSVga-ftr0dgQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "value": {
      "warrantyInformation": "1 week warranty",
      "stock": 99,
      "rating": 2.56,
      "meta": {
        "updatedAt": "2025-04-30T09:41:02.053Z",
        "barcode": "5784719087687",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
      },
      "id": 1,
      "availabilityStatus": "In Stock",
      "returnPolicy": "No return policy",
      "dimensions": {
        "height": 13.08,
        "depth": 22.99,
        "width": 15.14
      },
      "price": 9.99,
      "weight": 4,
      "category": "beauty",
      "minimumOrderQuantity": 48,
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "discountPercentage": 10.48,
      "shippingInformation": "Ships in 3-5 business days",
      "reviews": [
        {
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 3,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Would not recommend!"
        },
        {
          "comment": "Very satisfied!",
          "reviewerName": "Lucas Gordon",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 4
        },
        {
          "comment": "Highly impressed!",
          "rating": 5,
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        }
      ],
      "tags": [
        "beauty",
        "mascara"
      ],
      "sku": "BEA-ESS-ESS-001",
      "title": "Essence Mascara Lash Princess",
      "brand": "Essence",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"
    },
    "type": "Object"
  },
  "id": "1HFddONLhUQBcPXt3vhIMuOL285sIrYEf18YPy90Rr4r6E9A3lZRchYiIHNWfPV8pdrqVwRN9IZ30ldFBVhlMA",
  "cache_key": "1",
  "enriched_at": "1769407425"
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
✓ Created SWR script: fetch_github_user (CAGmQ5yeA7cid8c94CvQUTY11_sPhQLFROTjGh8DIsy4hEe8YCi3Lyh4SGuB7Cw85ibCxZ3Ox02BT3BIymP9kQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 75ms
Result: [{"cached_at":"1769407424","cache_key":"torvalds","id":"e0WQ7ZUh8FB0Ujeq1TlbLUZo2KYAbHwYc9iriJpQ2spYQJRVo4gNeTKWZD3NC2ECrx8agwMy8nUWlglylqAbew","data":{"value":{"following_url":"https://api.github.com/users/torvalds/following{/other_user}","url":"https://api.github.com/users/torvalds","bio":null,"login":"torvalds","repos_url":"https://api.github.com/users/torvalds/repos","received_events_url":"https://api.github.com/users/torvalds/received_events","twitter_username":null,"location":"Portland, OR","created_at":"2011-09-03T15:26:22Z","node_id":"MDQ6VXNlcjEwMjQwMjU=","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","site_admin":false,"updated_at":"2026-01-13T07:10:05Z","blog":"","organizations_url":"https://api.github.com/users/torvalds/orgs","public_gists":1,"hireable":null,"public_repos":11,"starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","name":"Linus Torvalds","html_url":"https://github.com/torvalds","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","email":null,"id":1024025,"type":"User","company":"Linux Foundation","user_view_type":"public","events_url":"https://api.github.com/users/torvalds/events{/privacy}","following":0,"followers_url":"https://api.github.com/users/torvalds/followers","followers":280327,"gravatar_id":""},"type":"Object"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 65ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (3te-xwGToUlxx4uIPbLhbfVbeM3xH_FQ3YYeaUql4BE5frvk_emAQMxV2tWSdVdYpJcJkw0rRZnGqOMH9sN_2g)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"id":"9jQAql7WYcJVQ_iDYzvsScMEjo4HloX8CPr45mKvv-Un684emKNK70k07_1_esoEg6jxu1T8-aXyhfW3rKRVZg","cache_key":"1","enriched_data":{"type":"Object","value":{"availabilityStatus":"In Stock","id":1,"meta":{"barcode":"5784719087687","createdAt":"2025-04-30T09:41:02.053Z","updatedAt":"2025-04-30T09:41:02.053Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png"},"tags":["beauty","mascara"],"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","category":"beauty","minimumOrderQuantity":48,"rating":2.56,"shippingInformation":"Ships in 3-5 business days","sku":"BEA-ESS-ESS-001","returnPolicy":"No return policy","stock":99,"warrantyInformation":"1 week warranty","price":9.99,"title":"Essence Mascara Lash Princess","brand":"Essence","reviews":[{"rating":3,"comment":"Would not recommend!","reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","date":"2025-04-30T09:41:02.053Z"},{"reviewerName":"Lucas Gordon","rating":4,"comment":"Very satisfied!","date":"2025-04-30T09:41:02.053Z","reviewerEmail":"lucas.gordon@x.dummyjson.com"},{"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","comment":"Highly impressed!","rating":5}],"weight":4,"dimensions":{"depth":22.99,"height":13.08,"width":15.14},"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","discountPercentage":10.48}},"enriched_at":"1769407425"}]
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 101.584792ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 80.641583ms
   📊 Records: 0
   🚀 Cache speedup: 1.3x faster!

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
✅ [32mRust function composition examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 71ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 72ms
   📊 Records: 0
   🚀 Cache speedup: 1.0x faster!

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
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
✅ [32mTypeScript function composition examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.8.0
    Uninstalling ekodb_client-0.8.0:
      Successfully uninstalled ekodb_client-0.8.0
Successfully installed ekodb-client-0.8.0
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 86.4ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 90.8ms
   📊 Records: 0
   🚀 Cache speedup: 1.0x faster!

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
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 83.787ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 79.741583ms
   📊 Records: 0
   🚀 Cache speedup: 1.1x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 68ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 72ms
   📊 Records: 0
   🚀 Cache speedup: 0.9x faster!

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
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


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

