make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("yuB0DHJY9KbLeiL6gDSx749AUYsPUyWzo0PjM71Z3s2AwGKilcGM70Re2uuqg_I5xaZoPVDAP9harVQbqL45lg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("yuB0DHJY9KbLeiL6gDSx749AUYsPUyWzo0PjM71Z3s2AwGKilcGM70Re2uuqg_I5xaZoPVDAP9harVQbqL45lg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("yuB0DHJY9KbLeiL6gDSx749AUYsPUyWzo0PjM71Z3s2AwGKilcGM70Re2uuqg_I5xaZoPVDAP9harVQbqL45lg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("yuB0DHJY9KbLeiL6gDSx749AUYsPUyWzo0PjM71Z3s2AwGKilcGM70Re2uuqg_I5xaZoPVDAP9harVQbqL45lg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "f1ep3MgsAlnio7R9S6GnmVSrYAorrXrI27q9RdGsoyd30671cQvxiWd5TDUy6NRUMVNF5-2ErJVEeT9xTtvSOw"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775226538943203000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "f1ep3MgsAlnio7R9S6GnmVSrYAorrXrI27q9RdGsoyd30671cQvxiWd5TDUy6NRUMVNF5-2ErJVEeT9xTtvSOw",
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
Collection created with first record: "UdLXN5Wq708zVWfh1hlBdKNobg1eCspQedVIDx_4rYzYXK5N2qYMqK-JrVF4YMvpQmLl6yXk6WcXA35oJ8tFrQ"

=== List Collections ===
Total collections: 5
Sample collections: ["demo_collection", "test_collection", "audit__ek0_testing", "batch_users", "websocket_test"]

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
✓ Inserted document: "8zCh_mX6zYgzwxeNUXl7LKdrDx7RYotAIxaeV6CzabiQYUhl478Fvq7j6o030XkErELGPMQWinGpv3VAB_uavw"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "MXblIFHgjgvW5H7qlu5zSzLfR4Iz3aKI8u7Tm9oS2QdCwLoa8ts7JlUlP39wImHjh0A9gQZfsuxz9IC4C5sMPg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "ba9-DIoxQDdAF4zp7XpLLFqk_-rGWhsMzZoONbXMpyzDsdX8ji-SJuYynIhjY2FpYhOy2rFat4PtPih7IpNiaA"

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
✓ Inserted document with TTL: String("p2j_HQXj8l2qjjvRoNwEBE7D2aplQDrywzVTkhCVS9RTWuZrrTBBIqHDDxfiLlqJ8uUOSUyTKBY1aR4X9FJAPQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "p2j_HQXj8l2qjjvRoNwEBE7D2aplQDrywzVTkhCVS9RTWuZrrTBBIqHDDxfiLlqJ8uUOSUyTKBY1aR4X9FJAPQ"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-04-03T15:29:09.802263Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: xScDTbBO8l5xeIJ-oWB2rEDpv1JxS2vX_E86jDJZwyGy9A7SU3qLvuHNDldvSecSZZK1rcoP-cNlfsTNN5x3lQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "y-5tS9Wx1206kF4RolKTDERyc5a-PJUY3q1w1W3SaobFBl-LR64BbLUYvmvG5_3lMvB5dO-TAZLdm0XKRyxoIw"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: opOoHb_2bVieuEjSNcZl9iiXI7az-_8MjUZnm9SVwyN8D6HhzqqPxtoM6ViN28NgVwkRiDgakhZZnVGFn-F8Dg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: WfahoJl08Q355P-xd9X1Vx9N_fLvi7W7JEVClfd-MHvtm_GCpf58H5exEdm4kO8nFBvwhf2FJUGY16Gm3ZYwLg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1I1IVPxpPR1A5st3auiK2VxJ3gMS6bPh6dxm1FF7ps7si16awUjap97stDs-KAcj4rrycM33z7--qeTguKGkXg
Created Bob: $500 - ID: DHE4kQHY6Gh7aeCuvIjK5lgmUC2eTTCJ5rzoUr2VTkuK24vZcOXRAbxbPAzXVHJtn9GQ_T18hrunHS1wZpQmVg

=== Example 1: Begin Transaction ===
Transaction ID: a411a834-9ac2-4611-8ce9-4fcc160ff0fe

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
New transaction: e7e446c6-2b9f-4d4f-8352-0e1a0a9eba9d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("kuriKYd2HPiCn6dUkzEjwU6PY1a8xs5zT9mF9f1ENXb_pa2cWVNQGUVJLgLYX9K3LWiAWkrrgq3l-odiuwbEzw")})

=== Find by ID ===
Found: Record({"price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "created_at": Object({"value": String("2026-04-03T14:29:17.341240Z"), "type": String("DateTime")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("kuriKYd2HPiCn6dUkzEjwU6PY1a8xs5zT9mF9f1ENXb_pa2cWVNQGUVJLgLYX9K3LWiAWkrrgq3l-odiuwbEzw"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-04-03T14:29:17.341240Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"id": String("kuriKYd2HPiCn6dUkzEjwU6PY1a8xs5zT9mF9f1ENXb_pa2cWVNQGUVJLgLYX9K3LWiAWkrrgq3l-odiuwbEzw"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "price": Object({"value": Float(99.99), "type": String("Float")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "created_at": Object({"value": String("2026-04-03T14:29:17.341240Z"), "type": String("DateTime")})})]

=== Update Document ===
Updated: Record({"created_at": Object({"value": String("2026-04-03T14:29:17.341240Z"), "type": String("DateTime")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "id": String("kuriKYd2HPiCn6dUkzEjwU6PY1a8xs5zT9mF9f1ENXb_pa2cWVNQGUVJLgLYX9K3LWiAWkrrgq3l-odiuwbEzw"), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: nNXlR3069M8GZvmv5fs3WUURrpuVNTX7Ai_uH2vCuWjBD5NkXqMOurtOylpNBitjsPMMsI_PZ_KkrU3ssl4Igg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "nNXlR3069M8GZvmv5fs3WUURrpuVNTX7Ai_uH2vCuWjBD5NkXqMOurtOylpNBitjsPMMsI_PZ_KkrU3ssl4Igg"
  value: 42
  name: "WebSocket Test Record"
  active: true

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "hdeMLek1PVtObu4XXo7NnVn4YmIs9lI-c7coRCte1-7Dd8N-gcyuuRrS9rBrzfQ1rNYl5mF052JVbBLh3_LGZg"

=== List Collections ===
Total collections: 10
Sample collections: ["ttl_cache", "users", "test_collection", "scripts__ek0_testing", "audit__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: iCwt39y2hqHreBoc0UL_Qkk58xnQd50GjEZuYyDjaNrkVInsLqfUQ_n5S0n5mpuBkVEGkTIPTV7eM8NMdb_Adw
Created Bob: $500 - ID: K8WVuKGpE-nAbqcCDDiHjP-gJ07kbHcLbXMDoDbK0J3wQ3LgwcrSpE_MWyfKiRp_zCPFkxy81VYMqXAXi8w1wg

=== Example 1: Begin Transaction ===
Transaction ID: eb92dfe0-464b-47d9-a9cb-1e970e7434f3

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3a944f5e-f274-4e05-8781-00e9b9289e11
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

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
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
     Matched: ["description", "description.value", "title", "title.value"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description.value", "description"]

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
Execution time: 3ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("s8XiJqj0gdbGAnLI_5l4bvFjvJPFE1uv3zjdtJ6LgWKp1cXmK-lvbTmgAGr8qPPSXZR25yYzepCBWghSgtaRxA"))
✓ Inserted user 2: Some(String("4ACuYfRojIUQsrXB1oiQsYuf2LXC76qXLXgOnjYUFPNsarbK2ZnafPr8EgWvIV9zcyh-tstsRvzXi2J5PmZmJA"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - email: String
    (required)
  - status: String
  - age: Integer

=== Listing Collections ===
✓ Total collections: 10
  Sample: ["ttl_cache", "users", "test_collection", "scripts__ek0_testing", "audit__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "0mahv8mF0pgqh7JjXMMKlZixRI-nVS9u_yjMedHOFge9hr53lrfzCRGB2fV-AsJ2GaDw5BdjyA0q5GeQDddKcw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("zXyEJ3LE5xqysATUU9tZsVTF-nucMixN3sLeEPP-ZTgyPdJEpnRLfumAkMbmI4P_lhERDiYkv1PEVsaYMNi7fw"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("hffzb6i3-1WeiYO4afcBG9hVJNUQwo0_SktrSMqHso5VFjddPjhNqitRPuKkXaKm1vEub7T35SvLJKwThakA7w"))

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
✓ Edge cache script created: bpuPNne0krohc70zGa-si0b5ZRxlRl4wG-LjiCLkK2XSUqrLTrCM4MTNLPmoeNwaGZx3MkUmCUjqnkUwTJUjRw

Call 1: Cache lookup
Response time: 14ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 13ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: YnamDT0hty_LM3fhD_fp2dgVPNiFxW4-BuoruXfO11CGwdb2rlzMBps42KZUwYynOPnsvLNRUa1Ph6bL2zKA5w
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 5h0LBfCbYTAuHCcI11ySb6dlkojdcDRHLXkBDYeDazXHNqtK6vye0P6D8kpC9tYd2OeyQdcNsGQcurF8Zw7-_Q
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: n_wSLLNfLaGU7swep3bM5DozFmGsLt-UjZlbTIg595d3R3xKBXMadV55gOg0j6paP4uZG2I1LxfRKX64PQUvUg
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
   ⏱️  Duration: 215.435667ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "phone": "1-770-736-8031 x56442",
    "address": {
      "city": "Gwenborough",
      "street": "Kulas Light",
      "zipcode": "92998-3874",
      "suite": "Apt. 556",...

Second call (cache hit - from cache):
   ⏱️  Duration: 39.588417ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "username": "Bret",
    "id": 1,
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neura...
   🚀 Cache speedup: 5.5x faster!

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

✅ Script saved: pqJaeagnVFhcnmwXiOvlAyoJHlXUcLtYbQM4odrjHswkCfjrpKx2JBMTelURDI5E2b2pMQ0uHzzUQYpDeWPBxQ
📊 Found 2 product groups
   Record({"count": Integer(2), "category": String("Furniture"), "avg_price": Float(474.0)})
   Record({"count": Integer(3), "category": String("Electronics"), "avg_price": Float(575.6666666666666)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"count": Integer(2), "category": String("Furniture")})
   Record({"count": Integer(3), "category": String("Electronics")})
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

✅ Inserted order: Some(String("jkM2Bnk8tP9ekcSf0JAKmu5Ug5iQiWw1IcD3z75DtQI7jO_LoxrljG_cL5dJLoDvdZmPooKpB4NLHRO-0yJWbg"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: ZMbXB8tRhLEMvksuJ06p94sPv_r-vuGHpA1qF-hlu8TOzKaaSGOg1QpK3g08f0EIOW3HOdHkZ5Zp321eG9Ytqw
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Gcui5blFJPTXRYBXPT3r-WLt2II4nZ7ZCHO_UBT3woMwbgk1znPrhVDyCjvAshRP_sPkrbYhDspIx5ypxiNZjw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: wisapHSeiyx7i1l44kNiIX-jXM_0Tk0aYAOWVmLqO3HwlS-MOH5KiO8uSyavU_ZSXoS-N0VJzOgER5S1WjoqAA
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
✓ Created SWR script: fetch_api_user_rs (U09qvHYkNHIrCSoA0WzLHYZvzb3SS-NvDnkr--K6PA36gnEjpIW475geipyC2tziYrut9jLT78vJACApkyQ_SA)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 40ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (mVdIgALYDSoJTuJt5DHd3Xrv5eKYOfb6dn8dPNIWPDSraifbNyyJ3NeMvzY6jg-J8AWtM-4cUdc4OxIwXdHOKw)

First call (cache miss - will fetch from GitHub API):
  Response time: 126ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 6.3x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (V5QDj2iEG3j-MWCaRAahRKPjw_WAP_zTK76X9AOBuD0pxfk3Sx7ZamOCDKxU8BSGJ4kBQ73u9XjM-IP_6b0yWA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (WAYvtTW1eFKZip6vQowVMs-40Gj4uITKWCoM7owg-CEh9v9s-IYbOt8pXqAUYGFXSXxzjUPrJLM6iQ9FyIAelQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (Mwo51zMUGYtUXX5wrKKXfuHWigyBw-iPVPgxPUUZc-MZCbKzsRRDnlgDsHNPImUg-pwj9q_902Q55qleoz4j6A)
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
   Record({"avg_price": Float(365.6666666666667), "count": Integer(3), "category": String("Furniture")})
   Record({"count": Integer(5), "avg_price": Float(367.0), "category": String("Electronics")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in retrieving similar items using vector embeddings, enhancing tasks like recommendation systems and image retrieval.

2. **Scalability**: Designed to handle large volumes of high-dimensional data, they efficiently manage and index vast datasets.

3. **High Performance**: Optimized for fast querying and retrieval, often using techniques like approximate nearest neighbor (ANN) search.

4. **Enhanced AI/ML Support**: Ideal for storing and processing data from machine learning models, particularly in natural language processing and computer vision.

5. **Flexible Data Types**: Can accommodate various data types, such as text, images, and audio, represented as vectors.

6. **Semantic Search Capabilities**: Facilitates more relevant search results by understanding the context and meaning of queries.

7. **Integration with Modern Tools**: Often compatible with popular data science and AI frameworks, easing the development process.

8. **Real-Time Processing**: Supports real-time data ingestion and query processing, beneficial for dynamic applications.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Database Design Principles (Database)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"category": String("AI"), "count": Integer(2)})
   Record({"category": String("Database"), "count": Integer(3)})
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
✓ Created session: 07fRg7thu1zdanTV9Y1iGdPQKtd_ydNh6iIWtBx7NdEteHa5RSoQr6F8ws67b1vQXslB27kZ7iebo7IfcIQn5Q

=== Sending Chat Message ===
Message ID: cTA93Ujq97Pjr70tr0akC4vhzp-kfsVtr4OvjEqd_-X7gVUxO84Uu4XId8zFVGkCF8A1D0QV5ROJgFvULUoPXw

=== AI Response ===
Response 1: ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. Here are some key features of ekoDB:

1. **AI Chat Integration**: This feature allows users to interact with the database using natural language queries, receiving AI-powered responses that include relevant context.

2. **Search Features**: ekoDB supports multiple types of searches including:
   - **Full-text Search**
   - **Vector Search**
   - **Hybrid Search**: This combines both text and semantic searches, with automatic context retrieval.

These features are designed to enhance user experience and facilitate more intuitive database interactions. If you have more questions or need further information, feel free to ask!

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("lzcI_SCavV7vWRoiRpxDtaT3FMrGbBued2fENL6g7CImbI6ru7_2lvFOF_3tVmU8rompTkNTiY8urP0VrlV1vA"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("-Dv68q5Y2jveZdbO451Y1bG1vdltbkBuR5QPYM5XCNbRbGoDM6TN8RgTl3DHWGxIcK-plc2XkN-fL5ECr0WjNg"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("tGP2-Va9Xe99AOKqM4woX8ACeXA7qTcaUyp4Uak83wIfjMnqE9gExxhDLZzQ6M2dDOl0Nit2PUGvdEQNqVvz_A"), "title": String("Search Features")}

Execution Time: 6324ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: smDgk8xU0qCBZo66hyK4MyZXgOZyzf3M_QV82Iy4kQIjfGyz8dsJP8RAegQc6rzvMIXPv5T5hFHUlKWWKKGufg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have further questions, feel free to ask!

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
✓ Created second session: iaBlsAKJ7QB_nFVEf5jdEb-WhsDQYfsVz6ukmMZS1btgaQmVPlZzEBLnbPkBTg0U0ehNKYADpoXccf-fxuPFKg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: vCYe82Jg-pOeZZIHObPhq_7GmYPvpDBTrTVkCnR0TIAzSQ1PijqVblarLs6OvWmTuSDzO8eyi8HYbAwcLiYcyw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product**: ekoDB
- **Description**: A high-performance database product with AI capabilities
- **Price**: $99

If you need more information or have specific questions about other products, let me know!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 05JlQuH7qP1SHgpJ-TURKOHDXaj85e2-u4qiSmdwNgMKKtBUizVZhBjLggZo_f8J0dssglAah1TwHu2c-arX6Q
  Parent: vCYe82Jg-pOeZZIHObPhq_7GmYPvpDBTrTVkCnR0TIAzSQ1PijqVblarLs6OvWmTuSDzO8eyi8HYbAwcLiYcyw

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: 05JlQuH7qP1SHgpJ-TURKOHDXaj85e2-u4qiSmdwNgMKKtBUizVZhBjLggZo_f8J0dssglAah1TwHu2c-arX6Q (Untitled)
  Session 2: vCYe82Jg-pOeZZIHObPhq_7GmYPvpDBTrTVkCnR0TIAzSQ1PijqVblarLs6OvWmTuSDzO8eyi8HYbAwcLiYcyw (Untitled)
  Session 3: iaBlsAKJ7QB_nFVEf5jdEb-WhsDQYfsVz6ukmMZS1btgaQmVPlZzEBLnbPkBTg0U0ehNKYADpoXccf-fxuPFKg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 05JlQuH7qP1SHgpJ-TURKOHDXaj85e2-u4qiSmdwNgMKKtBUizVZhBjLggZo_f8J0dssglAah1TwHu2c-arX6Q

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (122):
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
  - gpt-3.5-turbo-1106
  - tts-1-hd
  - tts-1-1106
  - tts-1-hd-1106
  - text-embedding-3-small
  - text-embedding-3-large
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

openai models (122):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4-mini
  - gpt-5.4
  ... and 117 more

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
Created user function with ID: DG64jdN-PX5DbdMmPuHw25n518XfJQyn-dIzGmq92o85dNg5Y48-O3n6QYxUFUfXoAUK7wtvrfzu4K8qe4xnsQ

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_rs: Get Active Users
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - get_active_users: Get Active Users (Updated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_users_by_status: Get Users By Status
  - fetch_user: Fetch user by code

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("ANXX64Uq5EQWZ4M65N6m5YzQLxrC2TM3mB6b7I_1yHCfRMbjVX0MR_F7ZtoxiniZtRn24M_276otRwl8zzGm1w")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("Iz7R21dux819C-mnDSMsPmtyd9mlPbQpAyC1L7PjhW9StR0DKFpl5fOjA0P2XxFlCl4aP9VZzmB6M8hgArssBA")})
✓ Second upsert (update): Record({"id": String("Gc3jImpdHQRuOqdgI_UPS1uS4H1DbIkvSOshONAcrF8wCyw-wJqkHVu5Yb2wsOH-cfEGz9ppKwXVasvBULYA4Q")})

=== Find One Operation ===
✓ Found user by email: Record({"age": Object({"type": String("Integer"), "value": Integer(28)}), "email": Object({"value": String("alice@example.com"), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Alice Johnson")}), "id": String("ANXX64Uq5EQWZ4M65N6m5YzQLxrC2TM3mB6b7I_1yHCfRMbjVX0MR_F7ZtoxiniZtRn24M_276otRwl8zzGm1w"), "active": Object({"value": Boolean(true), "type": String("Boolean")})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("2iqZi04bdtX0k128UjoffIkZWFG6Wn2opXWXC0kF4kAffyvoMR_vAZukxe73l1fL-EkaoVwXpLnsUYVjp3dJ3A")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("-Kd4EOfAgbmPxw1oL69fRJyb6eEybDUqNgZ6nsZb9F5WF_vad1r1mHprMV3bgZJbRdR2ia2dYaZp6Bsp-iVKhg")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("2iqZi04bdtX0k128UjoffIkZWFG6Wn2opXWXC0kF4kAffyvoMR_vAZukxe73l1fL-EkaoVwXpLnsUYVjp3dJ3A"), "price": Object({"value": Integer(150), "type": String("Integer")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("f7p6wWLzGBNbwK8t6piHn5GnvGApuct3VcsQTLaX67TxbdjiEtNXljk3GD-kpKmHmw_9ak8oItc8q4vkyXQhfw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email", "id", "name"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["status", "created_at", "id", "name", "email", "avatar_url", "user_role", "age", "bio"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["status", "password", "secret_token", "avatar_url", "age", "created_at", "bio", "id", "api_key", "user_role", "email", "name"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "name", "email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
