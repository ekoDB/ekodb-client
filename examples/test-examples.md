make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'DJo69H27J0nCROhUDHl_2cJ8NTFyuWubZ9GF7tKSgEIEVYs7F-vJR4_Tia-yV2i97jNtdbBB4Hcy4eJaWsn24A'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'DJo69H27J0nCROhUDHl_2cJ8NTFyuWubZ9GF7tKSgEIEVYs7F-vJR4_Tia-yV2i97jNtdbBB4Hcy4eJaWsn24A', 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: [{'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'DJo69H27J0nCROhUDHl_2cJ8NTFyuWubZ9GF7tKSgEIEVYs7F-vJR4_Tia-yV2i97jNtdbBB4Hcy4eJaWsn24A'}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'DJo69H27J0nCROhUDHl_2cJ8NTFyuWubZ9GF7tKSgEIEVYs7F-vJR4_Tia-yV2i97jNtdbBB4Hcy4eJaWsn24A', 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: zQECmSh2ns8ZLfEQScqatfFFhkooJ39oZ1dU-amJnKZTKXj0Mm24Mpp9ICJ0TTqm9HAmZnkQWnmC_cfXHj99Lg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "512558460",
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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "zQECmSh2ns8ZLfEQScqatfFFhkooJ39oZ1dU-amJnKZTKXj0Mm24Mpp9ICJ0TTqm9HAmZnkQWnmC_cfXHj99Lg",
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

✅ Script saved: bQkeChV2KmM7BVZtZEzhj0YkRNfT75S1w9shI39cfF8NJPsTC2W9HtQOMNuHw1d5NCQ7X-2XUqYH0-QoGTQhmw
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: hQaKk8ebAwFvg7OtN5dZt0pIojg7UjNv8XiWrcGFo7tP7TVfsW9UhSmGj-buUdGmJa-U7XbUv9F5eGYWi0AFsA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: mPqrvot8T-VmheDzgRwWYGqeETDTGmWmLMhOznS6aZBLeG-sP4p5CQDowqF78AdtOr2usMNBBjIzBBS5PtuI5w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'avg_score': 50.0, 'count': 15, 'max_score': 90}
   {'max_score': 100, 'status': 'active', 'count': 15, 'avg_score': 60.0}

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
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
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
Collection created with first record: MAG3gThmGtXRZUKspk-MaeMVI_wLI-AOrNLkjxhWU39blxMH1dxv5mN_CgmMwI5sXhXDWJykqUEcbqor-tTSLQ

=== List Collections ===
Total collections: 13
Sample collections: ['demo_collection', 'chat_messages__ek0_testing', 'products', 'ttl_cache', 'users']

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
Created Alice: $1000 - ID: zwzXOx3JfKhjxAnIiVKPTjr4vIwWa5LfABvMt0aOmWc83h9anpDGr7hZMck_36TAbcUc3b7opnbNpmAZ3Mqayw
Created Bob: $500 - ID: HvTa16Azy7ncxaSlSb-hBQAIENSKgo5M6Hx7nEk1hq8jRdVEviahpW78gLSncdj190cVm5uXRI6LRVTp9j5WzA

=== Example 1: Begin Transaction ===
Transaction ID: 364a81bb-65c6-4239-9f6d-9883648e3452

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: cb4c99c3-b035-46e3-813d-b194d1d254a8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KfsvHZURoE1E11CRgZBfLMEi__jOuLsT9kRUAkTlHzS7ViAFiR6HNRaa7ZmWbVTph-7o_vLv8t3eAdhSBeFYeg
Created Bob: $500 - ID: 5FwV21mfn0DjHuUn8TzlKTqAlKfIxImdVfNp7AaCFfrNrg6OGNx58EwH7COMT3VlkphXasKHjyytwZ_-LMUVtg

=== Example 1: Begin Transaction ===
Transaction ID: 9986d504-b80f-4b35-a7e0-602ff6688c35

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: VKqSLgeNoA77WkCdOYzqwGYMKxIsaZXpD2U6pIR1ZVD529U8yZOM6K5QQIJMBeuR45tRvW1Hj5J8DAboMDkFyw
Created Bob: $500 - ID: e1LjdLsyoQ_dA9qj6Ye1IjOgXw4k5qEMNdTSbtYXO7Erj8Og90_qzen3Wx3jJ3Oz-3hlqTbD1dAap6VPptaECg

=== Example 1: Begin Transaction ===
Transaction ID: 79ac30b8-491b-4091-b835-d34c15a70d7f

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: f5e2f320-53fb-4ace-996e-fefa6fe39d1a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 64l19L7oDDncoQbYt28AYtrSguR8frhr-Dlflc3JPt6UCrI8flW-9zw9uLxC88UHHG0-GAkxy-p-v9MG12NWZg
Created Bob: $500 - ID: E2ieWt6XWM1DPnyPYu6Zj_LvdP2cbZg22JwxpMUwlTMkdh742X_6A3q0qprMd86wap-SQGmzG4DDCcdAIa-mTw

=== Example 1: Begin Transaction ===
Transaction ID: 756e1a1f-2a76-48e2-b3c1-336c1586d7b6

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QH_yKI60FSh-vnaeB3PPwMIe0Wq2WF9CkECHqjDV5bOMtYXWkR51D6PduEKI53Y91SoXWyxywmdKThGPdEkAnA
Created Bob: $500 - ID: jPk-MpURkHZpdnZuRPC36T25L9IHuAallS2MENAnC21_EaHV1C0QSvosEdd9lj30KqMmEVCFKZ9XUKpJ8SLPbw

=== Example 1: Begin Transaction ===
Transaction ID: 79dad448-342b-414b-8891-0defa8454d5a

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 94433bb0-6247-4c9d-b5be-3198f4ef41e4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: xA3NqiXZJvnvkdZVjLRB9MLPKRmsQLK0XGPbm2keEIU7a6XTfGc57ofk_4PubjQ34pveMHFbNGpNH4Kyb-5aqw
Created Bob: $500 - ID: HQPfuFJVSR4B4zfUHCRqFWzZEhFWoLnEB59KlhX4KBN-nLz2Yyqw0U6Jhn9yFIT6t76iCtJdVj_aoY2QrgNGVA

=== Example 1: Begin Transaction ===
Transaction ID: 8b2aff04-5f3c-4b1b-9eef-07ba5fd45b70

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

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
   ✅ Script saved: KBnKQF2-Pz_XMRPbqOJGHJ9Q1HrOgBR2ouqUSxnixn11dm9VMSVZkASLPRabss83FymDo5i5-puZgWA1D8GdwQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 51pRYTP7jtqUYO7sy2VoMFrns9QTtGEZUsv9r5Gx9ZapT-TIdGt01GSdFbdgt6moJft-uXgfTlegR6kVLfekTw
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Hf_U24PP9TwbhGZQsNBVrevLLWOWar08PcaNU3rgkbZcZgfl5K6e24eVVq0lQ6AXR7yLFWFLOotJtgDepWIcig

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: QQn8CUKspO-TgSefyp7YlwvEXqEZvtBQ5vFp9fvJK7GC2KcuI0E6ktRcr_rzx0HbZuWUqmxtTh22Uy3xk-XU7Q

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 9A-3hooQsi7wB20YLvXBwXNkjtcXFgE27-BoUj0vd8KtwKbFr_LAWFSzBB0pPQad28O6fuE7rN9yG0f6Rs0bjA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: KBnKQF2-Pz_XMRPbqOJG...
   ✅ Deleted script: Hf_U24PP9TwbhGZQsNBV...
   ✅ Deleted script: QQn8CUKspO-TgSefyp7Y...
   ✅ Deleted script: 9A-3hooQsi7wB20YLvXB...
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
  Output: Document ID = f97PTyqThT12FdOhNujb3Tt_n2wJSFKopxxNNIW1UCw8JxxZNDuApYhIuWdBGFZP-btTeWVTdropgqKjakdxeg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(f97PTyqThT12FdOhNujb3Tt_n2wJSFKopxxNNIW1UCw8JxxZNDuApYhIuWdBGFZP-btTeWVTdropgqKjakdxeg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(f97PTyqThT12FdOhNujb3Tt_n2wJSFKopxxNNIW1UCw8JxxZNDuApYhIuWdBGFZP-btTeWVTdropgqKjakdxeg)
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
  Output: Document ID = n2WV5xO3qHP2qHYNsTUejiJqL_ry-GWtfQLpfYuKODQQSnwdrqBs3jQRtGcNQDtfHpd-Rw9lRLENKWGX0pCmgw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(n2WV5xO3qHP2qHYNsTUejiJqL_ry-GWtfQLpfYuKODQQSnwdrqBs3jQRtGcNQDtfHpd-Rw9lRLENKWGX0pCmgw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(n2WV5xO3qHP2qHYNsTUejiJqL_ry-GWtfQLpfYuKODQQSnwdrqBs3jQRtGcNQDtfHpd-Rw9lRLENKWGX0pCmgw)
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.14s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.2
    Uninstalling ekodb_client-0.15.2:
      Successfully uninstalled ekodb_client-0.15.2
Successfully installed ekodb-client-0.15.2
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'Bf6_NGIUbjxPQv8r9nQ7eKUHP112Fu3MVLjLMR9VSB__d4hnKdikiH88BBHKTQEeQPz7JOnVecBLRBWi_9srLQ'}

=== Find by ID ===
Found: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'Bf6_NGIUbjxPQv8r9nQ7eKUHP112Fu3MVLjLMR9VSB__d4hnKdikiH88BBHKTQEeQPz7JOnVecBLRBWi_9srLQ', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'value': {'value': 42, 'type': 'Integer'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'created_at': {'value': '2026-04-03T10:30:57.291827', 'type': 'String'}, 'price': {'value': 99.99, 'type': 'Float'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-03 10:30:57.291827
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'id': 'Bf6_NGIUbjxPQv8r9nQ7eKUHP112Fu3MVLjLMR9VSB__d4hnKdikiH88BBHKTQEeQPz7JOnVecBLRBWi_9srLQ', 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record', 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-04-03T10:30:57.291827', 'price': 99.99}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'id': 'Bf6_NGIUbjxPQv8r9nQ7eKUHP112Fu3MVLjLMR9VSB__d4hnKdikiH88BBHKTQEeQPz7JOnVecBLRBWi_9srLQ', 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'price': {'type': 'Float', 'value': 99.99}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'created_at': {'type': 'String', 'value': '2026-04-03T10:30:57.291827'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: o9NFKSzUpZMafkbR0AR_l0EJ6i_DDk8riUBwLHqPN7VXznTCQa2kYxlwNiL_-SHUew1MrpAfr-73y0ZDbyJC2Q

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
Collection created with first record: "CoRVONY9ms8FOEnk-i7DlsU45pT0-rqXQnJYelkZkJ84WkmzohANFuTbdGEva9zA29h2An-MIOwClcA6EAIurg"

=== List Collections ===
Total collections: 12
Sample collections: ['chat_messages__ek0_testing', 'products', 'ttl_cache', 'test_collection', 'scripts__ek0_testing']

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
  cache:product:1: {'name': 'Product 1', 'price': 29.99}
  cache:product:2: {'name': 'Product 2', 'price': 39.99}
  cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

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
Created Alice: $1000 - ID: dVh3mMW8FDaH41A6_SUhVOQ8LB0hQep7Am181sAsuBub3UjxZmmliJHgh63PdgJtJpmwDPjN4Ze5yALlAQWd3g
Created Bob: $500 - ID: G-WlMdrMjU5IBs8j_cAfLDnHV4heEwmg6WZS4blw6vKT6mhfg5_bhfr6IddB96FX9aylalGlCWm7appAiremJA

=== Example 1: Begin Transaction ===
Transaction ID: 91f62744-08d2-48cb-9c04-2bb5a1f04e07

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6ac908dd-6839-44cd-8461-c8d8a17bc43d
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
  1. Score: 25.740, Matched: name.value, name, email, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: title, bio.value, bio, title.value
  3. Score: 26.400, Matched: bio, bio.value, title, title.value
  4. Score: 26.400, Matched: title.value, title, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title.value, title
  2. Score: 39.600, Matched: title, bio.value, bio, title.value
  3. Score: 39.600, Matched: bio.value, title, title.value, bio
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.759
  2. Score: 0.740
  3. Score: 0.727

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.380, Matched: title, content.value, content, title.value
  2. Score: 1.363, Matched: content.value, content, title.value, title
  3. Score: 0.370, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title.value, title

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
✓ Inserted document: 7mySnTFOyXLhKW2D-6jBf19IcGKuaQYglSz2j-hRf4E3lgyOTdDactWpvNTPKlU9jy28qfjSOUMrIbBIFFtTTA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Vra_qlDlmMd3kYz24_N8P3Ng9JMtiImJkZSRYj5kSFljcuj-oBSaDTTomx5E18DFWlZKw5U_e2KcHh38534Zeg

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
✓ Inserted document with TTL: AM0EoC8I8xyGcHvQ93yMKLAYx_z8jm3NY9p1EYn1CU1ialzQeZVLkfc2T_THo-jSJG3W3yR0_H_Yv78ZQ3DLoQ

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
✓ Edge cache script created: qNj1rHoDJiiX_Fk1SgIAF_3nTZrE1kP6QBIY_hG6jDujl7ImEMWicJqh1ROOWOJDRqjfCULHwF0vrWHzd0Cf0g

Call 1: Cache miss (fetches from API)
Response time: 559ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 7.1,
          "time": "2026-04-03T14:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02014636993408203,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
      }
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
Response time: 41ms (13.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 7.1,
          "time": "2026-04-03T14:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02014636993408203,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
      }
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

✅ Script saved: OZZasZEhqQsZj-6k8whg6QY_hlrZNg9y5NSx4Uop3qxlqYxXS7R8dyAHbj6wneMXNWUtOlRo07qk_23QtnH_1g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: f9H9zbhAkmKHxbedvR2g9MeaKcxnHZGjTGuY1U0Kngil0l7P9VD0ICYnknPt_FzKUyHn2xcR4qQ7OeOAzgz5-g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: oB-Siola58suAR7qP2L-3TFLrqmMjjdDAITbnHcgSIPQ7Evc2F6wnJTJ-8cbHBeX83gH9D7ld20L5Zs5JHmqiw
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 38.4ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 38.2ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
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
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: Ov6JkDqfjIm13GYT5Eok2SmujLwBVkHCLDyzrE_vnuZ9exzRXXE9nwJ1yDF63c26oCTJYfsXUkNUQHUnhGwTaw
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

✅ Inserted order: Q4qInHL0IK0JVu9pdOA-occQCdN72iJqZ_3Gmf9B7oMVtDeW-5ujnAn1GlJ5a0Lumpnzz9CYeBymYRfCcWXaaw
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
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: 8N_Tpz0oQAcb-qI4uU5VFml3Ge-nxE8S_jsi7BIObORRXREQEzsSiUey5bNYquEvaAsRJuDZV53mvUZiyCffyQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-03T14:31:00.087192+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (hHe0leeHjgNuGH6g7RjN4toLhngzBRsNCRvcV8rNJ1hN7Y7QGEWdxFmIlYMtlp87qQies5yj9OlfojSyMtWT4A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
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
Response time: 40ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Python Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (KpuwV5hR-CgYrsXyD3EEECumMcNwmNfNYXuQRgqwDx4Uda-KZPL9aSiEsnTDWFatO-d7z2U1BczgflBfcqE1MQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 129ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 6.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (NpHFAN9JBtfShbClaAX-c4nOc6n4Lr_4z-PQiXV3cWNovLF4o2NcxFvcjhxh2TRwwwNY2OEgw5mxcqDvPw-QZw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (VPzkSap43eQbxmbwvd3093BJ4VzisMj05KNusjzM76qkhNCdei1_2CEswTTuluNlAelf0Kr9OtgS7WdFSlmR-A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (CUC-4dnFKhy7EVSdyUqxLhF6EQomdL_ERxZXS0_RnYufQo8wsZ61PwO24A4Byx7pXWpLLHCa__fcATDBGJS9rA)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
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
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel at finding similar items in high-dimensional spaces, making them ideal for tasks like image, video, and text retrieval.

2. **Scalability**: Designed to handle large datasets efficiently, allowing for rapid querying even as data grows.

3. **Support for ML Models**: They are optimized for use with machine learning models, especially for embedding and feature extraction applications.

4. **Real-Time Performance**: Vector databases provide real-time capabilities for applications requiring immediate results, such as recommendation systems.

5. **Flexible Data Types**: They accommodate various data types, including text, images, and audio, in a unified framework.

6. **Advanced Indexing Techniques**: Utilizing algorithms like Approximate Nearest Neighbor (ANN) for faster search times compared to traditional databases.

7. **Ease of Integration**: Often come with APIs that integrate seamlessly with existing applications and workflows.

These features make vector databases appealing for AI-driven applications and data-intensive operations.
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
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
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
✓ Created session: wfABG2pu6JA9StMRL4p3vfEt3VSkQjhx8ZQqiXaHzazDalnKBMVsf8_-a8xEJLSi3PpwZt3sVMzie1vAinUA6A

=== Sending Chat Message ===
Message ID: gNx2PuEPCLnhaSJrsoYAcOUOYthcUMp0xlvwGu1fbeczrEC-TVE4m0FNWdS8grzxA8Z6dyov-UPsf4PEKFUssg

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Pro**
   - **Price:** $299
   - **Description:** Enterprise edition product with advanced features

2. **ekoDB**
   - **Price:** $99
   - **Description:** A high-performance database product with AI capabilities

3. **ekoDB Cloud**
   - **Price:** $499
   - **Description:** Fully managed cloud database service product

Execution Time: 2568ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 95
Total tokens: 1393

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: tGGNmnGiFX4b2bdkljRfH8oLAQdJO1_kC23JLHr1mZuaW1udbqfsijepvd_dngWajcZIRd_TnbDKECWxO1fZ2Q

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you have any more questions or need further information, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'updated_at', 'llm_provider', 'role', 'content', 'created_at', 'llm_model', 'chat_id', 'token_usage', 'id'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: rhjy7d4dLnGAML7eqyrN7O3KOusPh8Rb_cdUHeNgVg6xcNp3TDeQHkXFKp8b5D4t42mv7Shjn4pZGlw4z1pLjQ
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Q-XooHMM3eZ6M7nOW9eTQ8OuZ5cEvNVKiDwYB5jtRvATiEsC5RuJRw63dhCpsiiXqZo4ZevBfgAltd11elm_aw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more details or additional products, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: sb4FesmdIcXm1L6eaGXh48HiNTc9M9dsOvH_vOUmSv6ipRvcLhYU6otEtCXueHfw_F3F0J6FLwEFPONPpKgzlg
  Parent: Q-XooHMM3eZ6M7nOW9eTQ8OuZ5cEvNVKiDwYB5jtRvATiEsC5RuJRw63dhCpsiiXqZo4ZevBfgAltd11elm_aw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: sb4FesmdIcXm1L6eaGXh48HiNTc9M9dsOvH_vOUmSv6ipRvcLhYU6otEtCXueHfw_F3F0J6FLwEFPONPpKgzlg (Untitled)
  Session 2: Q-XooHMM3eZ6M7nOW9eTQ8OuZ5cEvNVKiDwYB5jtRvATiEsC5RuJRw63dhCpsiiXqZo4ZevBfgAltd11elm_aw (Untitled)
  Session 3: rhjy7d4dLnGAML7eqyrN7O3KOusPh8Rb_cdUHeNgVg6xcNp3TDeQHkXFKp8b5D4t42mv7Shjn4pZGlw4z1pLjQ (Untitled)
  Session 4: wfABG2pu6JA9StMRL4p3vfEt3VSkQjhx8ZQqiXaHzazDalnKBMVsf8_-a8xEJLSi3PpwZt3sVMzie1vAinUA6A (Untitled)
  Session 5: vCYe82Jg-pOeZZIHObPhq_7GmYPvpDBTrTVkCnR0TIAzSQ1PijqVblarLs6OvWmTuSDzO8eyi8HYbAwcLiYcyw (Untitled)
  Session 6: iaBlsAKJ7QB_nFVEf5jdEb-WhsDQYfsVz6ukmMZS1btgaQmVPlZzEBLnbPkBTg0U0ehNKYADpoXccf-fxuPFKg (Untitled)
  Session 7: 07fRg7thu1zdanTV9Y1iGdPQKtd_ydNh6iIWtBx7NdEteHa5RSoQr6F8ws67b1vQXslB27kZ7iebo7IfcIQn5Q (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: sb4FesmdIcXm1L6eaGXh48HiNTc9M9dsOvH_vOUmSv6ipRvcLhYU6otEtCXueHfw_F3F0J6FLwEFPONPpKgzlg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'R9f7XW050HYXhz-76_Iu9-PJVVhCiW0YS2z226qolb_aiU4H0dYK3KxahU9AKaO46y0SkR8BIUaeYMeDb7GZ7Q'}

=== Upsert Operation ===
✓ Upsert (update existing record): R9f7XW050HYXhz-76_Iu9-PJVVhCiW0YS2z226qolb_aiU4H0dYK3KxahU9AKaO46y0SkR8BIUaeYMeDb7GZ7Q
✓ Inserted second record: 9wys_aF7g3Z-rUxP_1Lsrn955qZhaqeuTFoOMcAh7HkVZklHDHxSt9lnyOzpN9_v2hzD4O98s3U7At7UGzxVhg
✓ Upsert (update second record): 9wys_aF7g3Z-rUxP_1Lsrn955qZhaqeuTFoOMcAh7HkVZklHDHxSt9lnyOzpN9_v2hzD4O98s3U7At7UGzxVhg

=== Find One Operation ===
✓ Found user by email: {'id': 'R9f7XW050HYXhz-76_Iu9-PJVVhCiW0YS2z226qolb_aiU4H0dYK3KxahU9AKaO46y0SkR8BIUaeYMeDb7GZ7Q', 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'age': {'value': 29, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}
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
   Inserted with ripple: {'id': 'ovn910AKsIMIUkaCG3c_Qkz74sjGwD4eK71C9ZHs6p3cWR4_Gs0Ul0CPSoYfsAOmFReROfsm4RmgNSy4FnWaQw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'W9_YSlY8P9hhm_RA6PzGCNE1GRmbZXfBmdmCVsIZSGPNBbotWvb4BjUJNrVMBDbO-8_vPoMPsEMBgTyIx29dMg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'ovn910AKsIMIUkaCG3c_Qkz74sjGwD4eK71C9ZHs6p3cWR4_Gs0Ul0CPSoYfsAOmFReROfsm4RmgNSy4FnWaQw', 'price': {'type': 'Integer', 'value': 150}, 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'price': {'type': 'Integer', 'value': 500}, 'id': 'ovn910AKsIMIUkaCG3c_Qkz74sjGwD4eK71C9ZHs6p3cWR4_Gs0Ul0CPSoYfsAOmFReROfsm4RmgNSy4FnWaQw'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'name', 'id']
  First user: {'value': 'Bob Smith', 'type': 'String'} <{'type': 'String', 'value': 'bob@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['id', 'age', 'bio', 'email', 'created_at', 'avatar_url', 'status', 'name', 'user_role']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['avatar_url', 'name', 'id', 'user_role', 'age', 'secret_token', 'status', 'email', 'api_key', 'created_at', 'password', 'bio']
  Projected query:
    - 3 fields per record
    - Fields: ['id', 'name', 'email']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Python Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4-mini', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4-mini', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']

=== Get Anthropic Models ===
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: m7BYKcRq3lof-yiA0DnJAtG1hI5VsGwtB6ukLi5hD01geFx5vFP1XjLrQ4ZgZsSzN1XaU_6aNMZS4lBl_qhDDg

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users: Get Active Users (Updated)
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - get_verified_user: Get verified and validated user
  - get_active_users: Get Active Users (Updated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_py: Get Active Users
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_py' exists: False

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_py' exists: True

=== Count Documents ===
Document count in 'collection_utils_test_py': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: False

=== Cleanup ===
Deleted collection 'collection_utils_test_py'

✓ Collection Utilities example complete
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
Inserted: map[id:_isTTaB7p1M6VryANn8N4BEkHQTF09UEJE4yCxr51VfZ7z0Cz7hlPvD9nNuhQZtRr-NK4B4KI_vITp1byHZpOA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:_isTTaB7p1M6VryANn8N4BEkHQTF09UEJE4yCxr51VfZ7z0Cz7hlPvD9nNuhQZtRr-NK4B4KI_vITp1byHZpOA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:_isTTaB7p1M6VryANn8N4BEkHQTF09UEJE4yCxr51VfZ7z0Cz7hlPvD9nNuhQZtRr-NK4B4KI_vITp1byHZpOA name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:_isTTaB7p1M6VryANn8N4BEkHQTF09UEJE4yCxr51VfZ7z0Cz7hlPvD9nNuhQZtRr-NK4B4KI_vITp1byHZpOA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: lUlQM26VA_9SBAQRSYliTF_R_2Dvl7JS1tjAnF2wWMJmdARQq0EoSn35oHxL07L9JZUizhY1Oau6-kO7k6EU1A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775226701033494000",
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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "lUlQM26VA_9SBAQRSYliTF_R_2Dvl7JS1tjAnF2wWMJmdARQq0EoSn35oHxL07L9JZUizhY1Oau6-kO7k6EU1A",
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
        "id": "zQECmSh2ns8ZLfEQScqatfFFhkooJ39oZ1dU-amJnKZTKXj0Mm24Mpp9ICJ0TTqm9HAmZnkQWnmC_cfXHj99Lg",
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

✅ Script saved: la3Q4k78Njq8I2JvcBoPakJs8H36Eo4-iMnXOi-iOUiwLo1DCBryq-msXqfse97iF-LZBsn07uINkZ70sPHQSA
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: EVW6IQbtrJqKnGeVwd_wJ9KGKvGsW6wHBUVwAth1tpE7czaazViphfr0gVTmA0ste1tEuxtoSm4Ain4iDxtUkQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: xvxisyqXxWU2Cn-Xeq1PLqj5ekU2ir95ADhjLzSzWjSlKdpuNy0n3H_5e-cZY0q3Qcmj7sY4o3SaS0fy2dNoiA
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
Collection created with first record: cPogG7OP6REHKZ7u2hvHxR5F_Cu-F5KlIVyTt_TyKj2DT03RnNnKpn0SY4DOtXVRATgzNcknf54X-hkHIX35uw

=== List Collections ===
Total collections: 17
Sample collections: [schema_documents_client_py enriched_users demo_collection schema_users_client_py chat_messages__ek0_testing]

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
Created Alice: $1000 - ID: WqLrMfjpFOynwUWtXQSwfDGgWfZso7jpjAxgIqod_MSyoiy-S14UsNSNRMcsMQEeqnBpOU1oDTumr6xYK3HSxw
Created Bob: $500 - ID: Wjaa9mfv0lyHw7LB4_C4pQK4xfYjoJoQbHMpKpS4koc_XH89mFHSodaP2Khv0aOWfaQmoNbONoS_eL680tj_Nw

=== Example 1: Begin Transaction ===
Transaction ID: 77754581-9069-49a6-b532-c9643f1f2f01

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: map[type:Integer value:800]
Bob: map[type:Integer value:700]

=== Example 5: Rollback ===
New transaction: 0e7be821-ce97-4e06-8629-014a254610e4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

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
   ✅ Script saved: CLcEr9JtA72lqWnTxD8GAz-EXLvClCk_AhRhoQYO6djC0gMu--h3YMrUFomNwLoRKw57yn58djxnZbKt94dyEA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: HirONuWbk2JFYgaoekpSqWeOj0AXNC6_e-Wz22ChiZzIRnLOqX9xiLQrpoEGNGXMIp1A7TFwbOOT-0qxD8uuGw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: K0vn3ecBJqJYjTbVK3-zAKSZ3sc4qUvNdd7e8n0NFAg1GD4dRZhGcKhwFU3GV1tXTGKsH2SyP6kVVrFCK2jFrg

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
   ✅ Script saved: S7OLFLr6IM2KUa7PisfAiJxLVjrWBZ_sZxqGX41lurCuK_2FS-GsbZn50b2bNMEO1AFZdi4g1o_crpoFPgWXYw

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
   ✅ Script saved: 7YR07LjtR-sw95O_TiosCTNLgAZwjT4HLvN2LUylr9EpGPZgmdBL_Y19uNXN4QVjXLQNNLjCfyFCM3DqgxZAiQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: CLcEr9JtA72lqWnTxD8G...
   ✅ Deleted script: K0vn3ecBJqJYjTbVK3-z...
   ✅ Deleted script: S7OLFLr6IM2KUa7PisfA...
   ✅ Deleted script: 7YR07LjtR-sw95O_Tios...
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

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = cbbC_nGqP8zuD2ZbYqYqMYOi0zd1n7FzZRJzLeQjGqd4Qq9tpQpdwvsGZnWCAjGawEj2fN2ilc23nnPJm24m4A
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(cbbC_nGqP8zuD2ZbYqYqMYOi0zd1n7FzZRJzLeQjGqd4Qq9tpQpdwvsGZnWCAjGawEj2fN2ilc23nnPJm24m4A)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(cbbC_nGqP8zuD2ZbYqYqMYOi0zd1n7FzZRJzLeQjGqd4Qq9tpQpdwvsGZnWCAjGawEj2fN2ilc23nnPJm24m4A)
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

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = yXGcmCcHzOHGK-8IpyBVmAFtKhGrlU-Fy8WrR6Ph-xr_PhR2pWkULkHvXLERJNza3fUcLCn6hb_b_OQOm3OpNQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(yXGcmCcHzOHGK-8IpyBVmAFtKhGrlU-Fy8WrR6Ph-xr_PhR2pWkULkHvXLERJNza3fUcLCn6hb_b_OQOm3OpNQ)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(yXGcmCcHzOHGK-8IpyBVmAFtKhGrlU-Fy8WrR6Ph-xr_PhR2pWkULkHvXLERJNza3fUcLCn6hb_b_OQOm3OpNQ)
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
Inserted: map[id:IieawdsapsGW39zQNBHMq9k8Oh6QDNskjnfow5plgImJ5vvoCuTLowiMjUwlS_Cn85RypZoPuce0Iuh79FYg2A]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-04-03T14:32:11Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:IieawdsapsGW39zQNBHMq9k8Oh6QDNskjnfow5plgImJ5vvoCuTLowiMjUwlS_Cn85RypZoPuce0Iuh79FYg2A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-04-03 14:32:11 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-04-03T14:32:11Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:IieawdsapsGW39zQNBHMq9k8Oh6QDNskjnfow5plgImJ5vvoCuTLowiMjUwlS_Cn85RypZoPuce0Iuh79FYg2A metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-04-03T14:32:11Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:IieawdsapsGW39zQNBHMq9k8Oh6QDNskjnfow5plgImJ5vvoCuTLowiMjUwlS_Cn85RypZoPuce0Iuh79FYg2A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: ZLeWxbo8tBtGJHDQPk-I9ytvtUJ51wbnd2glRU7q0hYKGYnGrUz7rjyB2KERzJMyWrZkXMNtCTvYIR282dNA9g

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
Collection created with first record: hoseN_YkvVfnOT0hrpD-MQzd8fkd8hfEvakVwrtnei62LVMBAU96aZWX0oTr7bBcvE7wulYs0xr6v-4-d0Izww

=== List Collections ===
Total collections: 16
Sample collections: [schema_documents_client_py enriched_users schema_users_client_py chat_messages__ek0_testing ttl_cache]

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
  cache:product:1: map[name:Product 1 price:29.99]
  cache:product:2: map[name:Product 2 price:39.99]
  cache:product:3: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

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
Created Alice: $1000 - ID: rC4kEpz-zUfL90gRWpagE16RKRyuO1CJhlByyhuIa6RxqxvJYuEE3dt3cu4mNYHdezXDfDVMu4K6lKuQ5axU1Q
Created Bob: $500 - ID: D2Ds1_4doKyl60e8p5Vb44qQeSPEBjAkHjvHomY7FoV9h_gKppduHTGH8l87tZf3qBRVOCbDDLWgqrI8eA9GYA

=== Example 1: Begin Transaction ===
Transaction ID: 048bddaf-cd37-4b67-ab4b-5f601a806877

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 699b1f95-5fe9-4e4f-b75b-8a302fb26fd4
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
  1. Score: 0.767
  2. Score: 0.755
  3. Score: 0.755

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702
  2. Score: 1.507
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
✓ Inserted document: aWCs002FJEwLc5PLzY-gukEQ15cuPz3jisK5NK9i7Zq4oBOcdKoBgIvwqRQJdBE0bBZGsjbDlDXaSh0eLMocXA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4CQNTuhFj9rCcvthxY-XXKdyHmp-QfWyekpAm4DQ3YD65nbxM8VM-49ot8GWW3JEM-0mJfAYqtXj0_RebWoXWA

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
✓ Inserted document with TTL: t7BqT0azE51OO4TuMtPLPAeilJ1D2X3UOn_zJ007IY65LeZGX-_ruahfsqGJDhXDC1EIdq6Sq5ZFNMe1ZJ2B_w

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
✓ Edge cache script created: HmMka_Dw3Y42dJxCHMTJHSuTawXdWm84JIChbBC_o8kMZnS_T6pNoUEFNzewirq3OMt2EVsPAI3PtovlYmD7Og

Call 1: Cache lookup
Response time: 14ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 14ms
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

✅ Script saved: tNKKbhqvbzYTtd4AuuW1XwggYSQgdPQsGvkSVIrOYz4acQTf7HbBiw8RM30Z4QeyoCIGSoi2vg-I_kYYj6ZkIg
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 41.837583ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...

Second call (cache hit - from cache):
   ⏱️  Duration: 37.464459ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...
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

✅ Script saved: cSRdSGwhDArvpe-WjKkTWmF3ADpqg48tfqgENl2g2hiQxqOBkEHurF16HsPf1VheHwu__e0Cmg6xTpQRyfAiDA
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

✅ Inserted order: FtnzjexwO_fNsnrOL4vk4DQRcw9slLZJLVlMOoiSAHqrgLHNBOO8O0DpyGB2D488F7CIeJyaw0BzWnZVG5Ix0Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: _evLh-mtkLMYnQIXMzYAUSvW3uEun_GF1aUVxWYfI-ntEteLhDp6W2unQQp2vhC5wZy1Z1f1vu0zLpjOhXZtjw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: CxQ_uHZVP-lnrLnaWJqloBSp1lVhs2vfu08PbkxhgvRSG3HjymnsDmoTucXhdh4DhMT-IMgUCSM5lyZMpcEo_g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: Zh08YmcElikFvronYgI3hjVXoukXXrWp5cm0cotAa1LzRs2HBONY8WCDG88aMQOrnlbSIEMrstLouSFWsW0tkA
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
✓ Created SWR script: fetch_api_user_go (dACSjKnwI0MtBbbANSkSsB2qWSLI1nMe9P-z3_pwUPZTdSQO5Zm3T6zpYaC8K62mqCSiJtpY6hulCsiUrv1b4g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-04-03T10:32:17-04:00",
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
      "id": "1"
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
Response time: 44ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Go Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (tlLU74gOjbqDrt-eoxXbuuY6-qIcUlXzfApWx-9AhEzf7xha3v8d2alLJvdNBnXlOZg2vUsTdVYps1eXtd9nyg)

First call (cache miss - will fetch from GitHub API):
  Response time: 14ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 14ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (f-eebm4UGwsn92IbZlRkq9CX-GNG5fkbrwUvLZbK1pb7f685YPawGzjb9nJUtTg5-IbNzCJzw3h39dtxfG2WlQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (IU6LLpnl2tlPQTRdLeL-iJ82oGNegIA50sDDpIS-hPKX4CmZ-Cy92Fco63CrwUW31EZesKPpZ4EwE1aqTqym2g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (utFxSFETOOcjM4KOgkPQM7KR1lVu5etrgPynUn5f2WLNuS943H5cYP7HfZMYhamtR7R7wKfVSFn0AcLcElfiaA)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
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
   1. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   2. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   3. map[type:String value:Database Design Principles] (map[type:String value:Database])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   map[category:Database count:3]
   map[category:AI count:2]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 2fK6YIWVrxfkHp49rcnkCpUVif4R7CdjOPIjns1j8lazSs3PDEWUJ03Exa-HOcGNKCO6Lbf2no8Cc8eTrBnPfw

=== Sending Chat Message ===
Message ID: G2nWj-nQUFaqf31Fv1U52rsi657JnpJtNC30mQN3_0OqQiRHV-QfJ6u_P2K0UW8T_Wrv8JAbHhFWOLA0hRUgOQ

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:8aVSkRS-LACR2Iwb6pQapVxp6jIqpysF_aOhSkXP162xnv4-9ugQJ-Z85-PE5eOeskwLlluwrj9fLf4TtBW4Ig name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:2qfukXOUlR3Ve94MkK29I1owno1gcj3KI1P4-owf8YchLlRblLzova43bJ4GLcB1stI4__hYpUMCAXkObLN-ww name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:pvI0ksNuoHO_r8Xgy4p-afNjDFWrmxvHSKcfrM0Tc2bM0YzWcDySZxYQaUWCA056316KfGhYWklu6dIRLnq9wA name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2757ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: MrU4lJN84bWLybzr3Gw54Oj6lp9RU8vniif0B8GyUpQBok4kFBc7RVI0gYJrSxaXnYkgFVIpc7Wmj5SVM58imw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99 

If you would like to know more products or details, just let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: X1YaK9zkzOvX5JG8Ma4c37086VzIogvN7du-5hV8p5dlee0vSPBYRRVcndmTS0l9jjTM4Jbqx05XyEzVKdMzkg
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: V27O2IYkA-pS1ycp6WPrAHKsQ13UVW3vdQYJ0B2rpstg36Mdt_coi2ly3d--01TDPSjMBBIv7vcmLr6TSNQNQQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, let me know!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: xhzHMjCZ0iw4CuU8PpsticMxrZd8eZF15AFeO1IADKELq5cUwGVraUkGjnUxV2M_Sd4OSN2KzHxMZK21cot9dw
  Parent: V27O2IYkA-pS1ycp6WPrAHKsQ13UVW3vdQYJ0B2rpstg36Mdt_coi2ly3d--01TDPSjMBBIv7vcmLr6TSNQNQQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: xhzHMjCZ0iw4CuU8PpsticMxrZd8eZF15AFeO1IADKELq5cUwGVraUkGjnUxV2M_Sd4OSN2KzHxMZK21cot9dw (Untitled)
  Session 2: V27O2IYkA-pS1ycp6WPrAHKsQ13UVW3vdQYJ0B2rpstg36Mdt_coi2ly3d--01TDPSjMBBIv7vcmLr6TSNQNQQ (Untitled)
  Session 3: X1YaK9zkzOvX5JG8Ma4c37086VzIogvN7du-5hV8p5dlee0vSPBYRRVcndmTS0l9jjTM4Jbqx05XyEzVKdMzkg (Untitled)
  Session 4: 2fK6YIWVrxfkHp49rcnkCpUVif4R7CdjOPIjns1j8lazSs3PDEWUJ03Exa-HOcGNKCO6Lbf2no8Cc8eTrBnPfw (Untitled)
  Session 5: Q-XooHMM3eZ6M7nOW9eTQ8OuZ5cEvNVKiDwYB5jtRvATiEsC5RuJRw63dhCpsiiXqZo4ZevBfgAltd11elm_aw (Untitled)
  Session 6: rhjy7d4dLnGAML7eqyrN7O3KOusPh8Rb_cdUHeNgVg6xcNp3TDeQHkXFKp8b5D4t42mv7Shjn4pZGlw4z1pLjQ (Untitled)
  Session 7: wfABG2pu6JA9StMRL4p3vfEt3VSkQjhx8ZQqiXaHzazDalnKBMVsf8_-a8xEJLSi3PpwZt3sVMzie1vAinUA6A (Untitled)
  Session 8: vCYe82Jg-pOeZZIHObPhq_7GmYPvpDBTrTVkCnR0TIAzSQ1PijqVblarLs6OvWmTuSDzO8eyi8HYbAwcLiYcyw (Untitled)
  Session 9: iaBlsAKJ7QB_nFVEf5jdEb-WhsDQYfsVz6ukmMZS1btgaQmVPlZzEBLnbPkBTg0U0ehNKYADpoXccf-fxuPFKg (Untitled)
  Session 10: 07fRg7thu1zdanTV9Y1iGdPQKtd_ydNh6iIWtBx7NdEteHa5RSoQr6F8ws67b1vQXslB27kZ7iebo7IfcIQn5Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: xhzHMjCZ0iw4CuU8PpsticMxrZd8eZF15AFeO1IADKELq5cUwGVraUkGjnUxV2M_Sd4OSN2KzHxMZK21cot9dw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:FoozAFYwAxfLzn5FbREc8WzY7GvyRMjk2lyPjcQoSDnHhDf-1JBHIcZDR2wdOuruew7xrYMdjg5wDgPO3ZxI1A]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:FoozAFYwAxfLzn5FbREc8WzY7GvyRMjk2lyPjcQoSDnHhDf-1JBHIcZDR2wdOuruew7xrYMdjg5wDgPO3ZxI1A name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:FoozAFYwAxfLzn5FbREc8WzY7GvyRMjk2lyPjcQoSDnHhDf-1JBHIcZDR2wdOuruew7xrYMdjg5wDgPO3ZxI1A name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:hZBZteIkO02vnqYai6biQQUDJkikBPiGsSY3tUJG9UCvHclwWR-ftrinLgLIxftPfG2_JvUau9U9gHtozkDGsQ]
Inserted with bypass_ripple: map[id:pVEOvYaNbwrkzRh-na_Iw8sI8iIlDKt11Y9guOOf7YbtKr1AwYV1bSBK_RjaJ-djyi3TneHHnJbUgpXeC5n0cg]
Inserted with TTL and bypass_ripple: map[id:sBtlGRxhwCIaLdkzgzwpPngSOKpD6UGM3EDeJ7K7XfkcQm9kF1PHHEReg2bCoIVbFn-H-On5MPN9F08Z6VaIvQ]
Updated with bypass_ripple: map[id:hZBZteIkO02vnqYai6biQQUDJkikBPiGsSY3tUJG9UCvHclwWR-ftrinLgLIxftPfG2_JvUau9U9gHtozkDGsQ name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: [age avatar_url bio created_at email id name status user_role]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: [age api_key avatar_url bio created_at email id name password secret_token status user_role]
  Projected query:
    - 3 fields per record
    - Fields: [email id name]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Go Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.29999999999999999 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: [gpt-4-0613 gpt-4 gpt-3.5-turbo gpt-5.4-mini gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 dall-e-3 dall-e-2 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 gpt-4o-audio-preview gpt-4o-realtime-preview omni-moderation-latest omni-moderation-2024-09-26 gpt-4o-realtime-preview-2024-12-17 gpt-4o-audio-preview-2024-12-17 gpt-4o-mini-realtime-preview-2024-12-17 gpt-4o-mini-audio-preview-2024-12-17 o1-2024-12-17 o1 gpt-4o-mini-realtime-preview gpt-4o-mini-audio-preview o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 gpt-4o-realtime-preview-2025-06-03 gpt-4o-audio-preview-2025-06-03 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-3.5-turbo-16k tts-1 whisper-1 text-embedding-ada-002]
Anthropic models: [claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514 claude-3-haiku-20240307]
Perplexity models: [sonar sonar-pro sonar-deep-research sonar-reasoning sonar-reasoning-pro]

=== Get OpenAI Models ===
OpenAI models: [gpt-4-0613 gpt-4 gpt-3.5-turbo gpt-5.4-mini gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 dall-e-3 dall-e-2 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 gpt-4o-audio-preview gpt-4o-realtime-preview omni-moderation-latest omni-moderation-2024-09-26 gpt-4o-realtime-preview-2024-12-17 gpt-4o-audio-preview-2024-12-17 gpt-4o-mini-realtime-preview-2024-12-17 gpt-4o-mini-audio-preview-2024-12-17 o1-2024-12-17 o1 gpt-4o-mini-realtime-preview gpt-4o-mini-audio-preview o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 gpt-4o-realtime-preview-2025-06-03 gpt-4o-audio-preview-2025-06-03 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-3.5-turbo-16k tts-1 whisper-1 text-embedding-ada-002]

=== Get Anthropic Models ===
Anthropic models: [claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514 claude-3-haiku-20240307]

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: 2PpT-uWSz6YGwhe-e5Iu5pwhfvoDhLNIdpckZ74KJaAmZ6FzsQkjT-iB56lCmwFi-355Kgye2_RTfh17K0iRkw

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 19 user functions:
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - get_active_users: Get Active Users
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists

=== List User Functions by Tag ===
Found 19 user functions with 'users' tag:
  - get_verified_user
  - fetch_and_store_user
  - fetch_and_store_user
  - fetch_slim_user
  - fetch_slim_user
  - swr_user
  - swr_user
  - validate_user
  - cache_api_call_py
  - get_active_users
  - validate_user
  - fetch_and_store_user
  - fetch_user
  - fetch_user
  - fetch_slim_user
  - fetch_user
  - swr_user
  - get_user_wrapper
  - validate_user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test' exists: true

=== Count Documents ===
Document count in 'collection_utils_test': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test'

✓ Collection Utilities example complete
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 953ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'XFBls3IkMNTSTt8_elpTZnxBt12DUTbmd0Sr9y7cwIeU_kI4oMUOZ6R2Ng50yNn5A85Z3A-GoI7DsE7xzSl84Q'
}

=== Find by ID ===
Found: {
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { type: 'String', value: 'Test Record' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  price: { type: 'Float', value: 99.99 },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { type: 'Boolean', value: true },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  id: 'XFBls3IkMNTSTt8_elpTZnxBt12DUTbmd0Sr9y7cwIeU_kI4oMUOZ6R2Ng50yNn5A85Z3A-GoI7DsE7xzSl84Q',
  value: { value: 42, type: 'Integer' },
  created_at: { type: 'DateTime', value: '2026-04-03T14:33:01.543Z' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-03T14:33:01.543Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  data: 'aGVsbG8gd29ybGQ=',
  metadata: { key: 'value', nested: { deep: true } },
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'XFBls3IkMNTSTt8_elpTZnxBt12DUTbmd0Sr9y7cwIeU_kI4oMUOZ6R2Ng50yNn5A85Z3A-GoI7DsE7xzSl84Q',
  value: 42,
  created_at: '2026-04-03T14:33:01.543Z',
  categories: [ 'electronics', 'computers' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  created_at: { type: 'DateTime', value: '2026-04-03T14:33:01.543Z' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  name: { type: 'String', value: 'Updated Record' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: 'XFBls3IkMNTSTt8_elpTZnxBt12DUTbmd0Sr9y7cwIeU_kI4oMUOZ6R2Ng50yNn5A85Z3A-GoI7DsE7xzSl84Q',
  value: { value: 100, type: 'Integer' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: m4dXK6DDDsZleqA54HvWXcAf2FPis1EnNfWqMuRhzW-tAHYT03perCormfIJQgRPfPa26UTlzD_r6gIdPnsk9Q

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
Collection created with first record: HxPDlNpQm9MtZ6fNGwrMWKan8jgAW8mkPhrmeMaT3DeYkcHAO3dPZaehCTlot_S1mY_OG6w7pf3yoAApiIsTSg

=== List Collections ===
Total collections: 22
Sample collections: schema_documents_client_py,enriched_users,schema_users_client_py,client_collection_management_ts,chat_messages__ek0_testing

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
  cache:product:1: { price: 29.99, name: 'Product 1' }
  cache:product:2: { name: 'Product 2', price: 39.99 }
  cache:product:3: { name: 'Product 3', price: 49.99 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

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
Created Alice: $1000 - ID: 7f4AJao-0RPUpdj_YrG4Y0rcZfBG4JQhtW5l2Z0RqZB4d0pNgzeBVReuWuO2O-DaKRfd7yfn09HgEUtLtVgZ-w
Created Bob: $500 - ID: PCVmKv19UjlMS78N29qTUvIp8tE0Id0gtUrKLWuxSZlO7_UGZjsbVvkc734lXN6QNuFcLGgTjItraKAit4ZpUw

=== Example 1: Begin Transaction ===
Transaction ID: 46d9b7a6-ae8f-453e-8ef0-c8bb72f0a208

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: aaff538c-4c6b-421e-8b0f-d493a10d19dd
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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: title.value, bio.value, bio, title
  4. Score: 26.400, Matched: bio, title, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: bio, title, title.value, bio.value
  4. Score: 39.600, Matched: bio.value, title.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.761, Matched: 
  2. Score: 0.749, Matched: 
  3. Score: 0.720, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.704, Matched: title, content, content.value, title.value
  2. Score: 1.499, Matched: content.value, title.value, content, title
  3. Score: 0.288, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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
✓ Inserted document: 0JrEoWhMz91TX_dZSa7pO-QsvX3aWucTDzuYSQOklb43DZO6mezgJwMUr5xdVbux3VnAx2hUL-PnKW_9f0fJVw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: tnLCfeHv3XnJmk6Ht-5Ds7FSe77dQKIlIozp3kqYwJRehFQcf_rQxXKty8D4LbXtdZPpKrMPCfZmWF2FuFT6Ig

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
✓ Inserted document with TTL: OZH7Q5LIJ3d6wmJjYJVKkgA6ipW1LtganHZeQAAztg9YwYp8dqkjWANqBpAdkrFjJGVzXXNUsF0rrgJ2uPnPbg

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
✓ Edge cache script created: 5zA40FI_aCZEBqzbHM04mZBPBK7HIWHZCuW5nMg6t98DBhYCab7GIKtPkdkXblmTUEFbffliVAlYtNGbmKXDuA

Call 1: Cache miss (fetches from API)
Response time: 150ms
Result: {
  "records": [
    {
      "value": {
        "email": "Sincere@april.biz",
        "website": "hildegard.org",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "name": "Leanne Graham",
        "address": {
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "city": "Gwenborough"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "username": "Bret"
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
Response time: 40ms (3.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "email": "Sincere@april.biz",
        "website": "hildegard.org",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "name": "Leanne Graham",
        "address": {
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "city": "Gwenborough"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "username": "Bret"
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

✅ Script saved: 1sAl0emWhMod1ZX1eUGJMkwgSAT32k7QeocXiigClMXSqlUCtgmOlmLk1JWnTgAkZU0_MWq8h3f6Z7rfhl_jGg
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: d9Y2_StdxxoBur1JLq7RtXDGLuf2sUB52Mqx8eTFy9zuukPE8SvfrFhbArgkjAjwERp60axv_R4RrxzJlZvBKw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: L-xC7Ai9yTIJ8Jzr_cwbQv_-3cR8X4exc8JHibHVLVFNgu9kdf8s2pb5IzHIbtkVfUzYoihrcWRVnz6x4SR_-A
📊 Statistics: 3 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"count":5,"avg_score":60,"status":"null"}
   {"status":"active","avg_score":60,"count":5}

📝 Example 4: Script Management

📋 Total scripts: 22
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "username": "Bret",
    "address": {
      "suite": "Apt. 556",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "zipcode": "92998-3874...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "username": "Bret",
    "address": {
      "suite": "Apt. 556",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "zipcode": "92998-3874...
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
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: x-_IO6wMjEKwwLJdAgIvWS0ODMwp9JanH_3t2IXjj_kGSTFj0JHawN_Z1g1bWLFhylv14EAln7Bp9gNRNL7m-g
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: NLyksw8BzTTC-pUTfe1IsS9-4UJNpfG3Yxfcf0-hOW5bk9Ka6AjaL8Q2COgKBOssq1PoJKC2EnbrKBdZYSwkww
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: i-yQCAtC3DobPk2UFuv0OwAkfcwkAAcfqekaxleAL0JRjQDzhFYbYoUdWRiWrG15UMumwiEqmpqoqjJOv1AqVw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: BI-noATw5mIF1rJ4vkdERrRCkULENTZU7pCROY4SFjA6FeCvq979cYTlnkcNtXVdpL_n4CT6eORBuCgGP_xb7Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: Cqe7Rzv8relrjN3uKcCcq7op2gyHWTLpj9ORxQU3U9O_1XBJ36B_wCJnjMLqO0M4Ic5QeAQkpCuXYlJLiU2gRg
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
✓ Created SWR script: fetch_api_user (avwhxRs7M-uRKBVLL3qH51pKI9xduZPC8RKeIZgiMzDNwUc1BrmFYmLdZKbNJ9NVyg_XJoVOTYYMHM-IiAcbsw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "address": {
          "zipcode": "92998-3874",
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
        },
        "id": 1
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
Response time: 40ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "username": "Bret",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "address": {
          "zipcode": "92998-3874",
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
        },
        "id": 1
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (QJ0LqaBmw3nDhOalzHrUb09bWNld20pnmjovtLfq47sPKhi35yEmWXXq2aZAIqeY8ihPeswivOh8oa6yBBWK4Q)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "stock": 99,
        "discountPercentage": 10.48,
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "returnPolicy": "No return policy",
        "meta": {
          "createdAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "availabilityStatus": "In Stock",
        "warrantyInformation": "1 week warranty",
        "weight": 4,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "rating": 2.56,
        "reviews": [
          {
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 3,
            "comment": "Would not recommend!",
            "reviewerName": "Eleanor Collins",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "comment": "Very satisfied!",
            "rating": 4,
            "reviewerName": "Lucas Gordon",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "lucas.gordon@x.dummyjson.com"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 5,
            "reviewerName": "Eleanor Collins"
          }
        ],
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "minimumOrderQuantity": 48,
        "id": 1,
        "price": 9.99,
        "sku": "BEA-ESS-ESS-001",
        "shippingInformation": "Ships in 3-5 business days",
        "tags": [
          "beauty",
          "mascara"
        ],
        "title": "Essence Mascara Lash Princess",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ]
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

🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (7lSrp2ihcOoF-FXCKLdKyIl0mIfITvbxio3X97PwOJ3fuQj3RuYmLvOPhmq0MtrVVUVlNvu6u8xVL78rL30K5w)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 16ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (qFv9rdyDf2Rv-C0vfNYRnQfRxjCpiKnV-g5MM-WXlcBERK7J-IgsSa5-lWk-iK1puYukvVqhv6y1J0RDujT5Dg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (p4sQKDYClrbzBhAaGUiPoGjxvIdT_jhyzflHLa5mCs-C2cJm_iCwpk6-avAeRB9u55IQTRtF4L6RkIO8HANTjg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (ns3ngS-SEFZ2h9fYdc6BMKqVmB0sjSEDmm7bcQyJp6-KXE1jQ_n3u-OeFECdvGP3uMiUbz5ApWlwX50awZnXbg)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
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
   {"count":3,"avg_price":365.6666666666667,"category":"Furniture"}
   {"avg_price":367,"category":"Electronics","count":5}
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
   Vector databases offer several benefits, including:

1. **Efficiency in Similarity Search**: They enable rapid retrieval of similar items based on vector representations, essential for tasks like recommendation systems and image retrieval.

2. **Scalability**: Designed to handle large volumes of data and high-dimensional vectors, making them suitable for big data applications.

3. **Advanced Machine Learning Integration**: They work well with machine learning and AI models, allowing for seamless integration with embedding techniques.

4. **Flexibility**: Support for various data types and structures, accommodating unstructured data like text and images.

5. **Enhanced Performance**: Optimized for operations like nearest neighbor search, improving query response time.

6. **Real-time Analytics**: Capable of real-time data processing and analytics, which is critical for dynamic applications.

7. **Non-Euclidean Metric Support**: Often support advanced distance metrics beyond traditional Euclidean distance, allowing for more nuanced similarity measures. 

These benefits make vector databases particularly powerful for applications in AI, natural language processing, computer vision, and recommendation systems.
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
   {"role":"admin","count":3,"avg_score":20}
   {"avg_score":70,"count":7,"role":"user"}
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
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: cVJund49jxJ49ZJ62Ez4AOw2mv2BL6yOW1CDpzkjGwn1zybbFwvoX3t0fxtyRFeWX2FeN168tsNhiFr0UPaWng

=== Sending Chat Message ===
Message ID: c8lyMTf_lPBbG1w4ipGEAD9DP9mBR1bHWRt4XkrfTSoDWOvIQcwZhSG6cp131qKfXtOU_YE4S8rQZqtmyzr95A

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB** - $99
2. **ekoDB Cloud** - $499
3. **ekoDB Pro** - $299

If you need more information about any of these products, feel free to ask!

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '4MTA5l-cV-rLKcnDwV8wLf4AWPyBRB7XA3muDnx5pX6-g6BSvvsQQ-l_K5L9QqqhNE3vLIMELGbABa2UteacCQ',
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'JOc2A3j9ElfIfCB7dZFHIDF3vhR-PwG8glbwnht6dCMYT2xnmOMNc5axhuMC7mVWpvEfuAJ1u6eBymBfzCu0Sw',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'SIFwwnLfWNKCb6CPVkYVZU3n8DCHpdgMdeZFdI3xcAQae3haAx97aDL54dYOkj8pYFXrLul9FlYgw6R9p41yzQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 12079ms

=== Token Usage ===
Prompt tokens: 2772
Completion tokens: 69
Total tokens: 2841

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: snQEQB84cLZI3RBlsgET8piAvV44ddXrxqES4VjFL-dT6qHyJt8HVeeI8g1heAt6QuNZaGBTvkHFStcvVDqZRw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you're looking for more products or specific details, feel free to ask!

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
✓ Created second session: KJ2Vm-XGaYFT_Mjbw663wXs8yVSmkFNPdtMfZ93FnDWGf4amRPmeGkaUnV1LJEEw4qAjD-E20o29js71iN_hiA
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rBcttguKjtUmQaP0Z26C6gzi94TQ0VBYNnAZF-8z-xgA2IFaI-S4GbCjDOd4-v7q31NIp-Z8edZQ56lcIq-stA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or additional products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: WRrOlpW2rekawJTYjEKBwI4E0OwFSJBigSSK47dJophNctz0Hs8XZZHzsllT2LsnBXWlcEEmfHIWUYT4DbwfbQ
  Parent: rBcttguKjtUmQaP0Z26C6gzi94TQ0VBYNnAZF-8z-xgA2IFaI-S4GbCjDOd4-v7q31NIp-Z8edZQ56lcIq-stA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: WRrOlpW2rekawJTYjEKBwI4E0OwFSJBigSSK47dJophNctz0Hs8XZZHzsllT2LsnBXWlcEEmfHIWUYT4DbwfbQ (Untitled)
  Session 2: rBcttguKjtUmQaP0Z26C6gzi94TQ0VBYNnAZF-8z-xgA2IFaI-S4GbCjDOd4-v7q31NIp-Z8edZQ56lcIq-stA (Untitled)
  Session 3: KJ2Vm-XGaYFT_Mjbw663wXs8yVSmkFNPdtMfZ93FnDWGf4amRPmeGkaUnV1LJEEw4qAjD-E20o29js71iN_hiA (Untitled)
  Session 4: cVJund49jxJ49ZJ62Ez4AOw2mv2BL6yOW1CDpzkjGwn1zybbFwvoX3t0fxtyRFeWX2FeN168tsNhiFr0UPaWng (Untitled)
  Session 5: V27O2IYkA-pS1ycp6WPrAHKsQ13UVW3vdQYJ0B2rpstg36Mdt_coi2ly3d--01TDPSjMBBIv7vcmLr6TSNQNQQ (Untitled)
  Session 6: X1YaK9zkzOvX5JG8Ma4c37086VzIogvN7du-5hV8p5dlee0vSPBYRRVcndmTS0l9jjTM4Jbqx05XyEzVKdMzkg (Untitled)
  Session 7: 2fK6YIWVrxfkHp49rcnkCpUVif4R7CdjOPIjns1j8lazSs3PDEWUJ03Exa-HOcGNKCO6Lbf2no8Cc8eTrBnPfw (Untitled)
  Session 8: Q-XooHMM3eZ6M7nOW9eTQ8OuZ5cEvNVKiDwYB5jtRvATiEsC5RuJRw63dhCpsiiXqZo4ZevBfgAltd11elm_aw (Untitled)
  Session 9: rhjy7d4dLnGAML7eqyrN7O3KOusPh8Rb_cdUHeNgVg6xcNp3TDeQHkXFKp8b5D4t42mv7Shjn4pZGlw4z1pLjQ (Untitled)
  Session 10: wfABG2pu6JA9StMRL4p3vfEt3VSkQjhx8ZQqiXaHzazDalnKBMVsf8_-a8xEJLSi3PpwZt3sVMzie1vAinUA6A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: WRrOlpW2rekawJTYjEKBwI4E0OwFSJBigSSK47dJophNctz0Hs8XZZHzsllT2LsnBXWlcEEmfHIWUYT4DbwfbQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: '1qAIvY-PCS9gTfi0gXO6cwS7YvX7w-FUFqbXY9dpCuxNp4UuMnGK_ZD9bvWqUeBX3t-vx8dMOiQRGasc1FDcEg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: '1qAIvY-PCS9gTfi0gXO6cwS7YvX7w-FUFqbXY9dpCuxNp4UuMnGK_ZD9bvWqUeBX3t-vx8dMOiQRGasc1FDcEg',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' }
}
✓ Second upsert (insert): {
  id: 'G3NScTzk7GUY_zxMlDftL37J44KX6_otbzNjszYnuhJWjfzk96O_LD36WFcP_dD02Gdy88TzB5adJ4Zd2a9XRA'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: '1qAIvY-PCS9gTfi0gXO6cwS7YvX7w-FUFqbXY9dpCuxNp4UuMnGK_ZD9bvWqUeBX3t-vx8dMOiQRGasc1FDcEg',
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
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
   Inserted with ripple: {"id":"rL605YRy15Ol8wu6XzZ2Z_2EAc7N1O1JSKAGoeqx_Tk9oUg_uc47oqWGiGfUzucn827S6-eI8VgfT9BP74EKIQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"2ZB3Sh9Sv93XpvnJqD3YTuHNrF-aoq042dFZpoUGrFZ1JfBhDX23rwRjgCsLTJwFVg0u7T45C-NY7Dhzj_SN_g"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"name":{"type":"String","value":"Product 1"},"id":"rL605YRy15Ol8wu6XzZ2Z_2EAc7N1O1JSKAGoeqx_Tk9oUg_uc47oqWGiGfUzucn827S6-eI8VgfT9BP74EKIQ"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"type":"String","value":"Upsert Product"},"price":{"type":"Integer","value":500},"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","email","name"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role","created_at","age","name","avatar_url","email","status","bio","id"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - [object Object]: [object Object]

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["avatar_url","api_key","user_role","email","id","created_at","secret_token","password","bio","age","status","name"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","id","name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  anthropic:
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
    - claude-3-haiku-20240307
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4-mini, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: _0mYz6TjOsRl34zPVJV1NnM1K-dwCs0ecQjMaxJrCrnLPz3PXY72-ock39eHNZeC_E2UlA8R_2DIEuhPZ7H_wQ

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_api_user: Fetch User with Cache
  - get_active_users_ts: Get Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - cache_api_call: Cache External API Call

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

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
  id: 'kaVvsrwFI7TUPuv32fKwyn4DWu7Ht1ThM_zjAcHzNkuPO41ptDH6kJANFCw6Ma2bu5n0Iq3FrxUSL95870ANyw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'kaVvsrwFI7TUPuv32fKwyn4DWu7Ht1ThM_zjAcHzNkuPO41ptDH6kJANFCw6Ma2bu5n0Iq3FrxUSL95870ANyw',
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    name: { type: 'String', value: 'Test Record' },
    id: 'kaVvsrwFI7TUPuv32fKwyn4DWu7Ht1ThM_zjAcHzNkuPO41ptDH6kJANFCw6Ma2bu5n0Iq3FrxUSL95870ANyw',
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true },
  id: 'kaVvsrwFI7TUPuv32fKwyn4DWu7Ht1ThM_zjAcHzNkuPO41ptDH6kJANFCw6Ma2bu5n0Iq3FrxUSL95870ANyw'
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
✓ Inserted test record: HPFYo2M67cO94fNUfFqgtC7m1KvU2xls33OzEPtGN3NQFLQlQgVADWblalBpFwA0N6soz6l8YdwU3YbDCRqGfg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775226860974",
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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "lUlQM26VA_9SBAQRSYliTF_R_2Dvl7JS1tjAnF2wWMJmdARQq0EoSn35oHxL07L9JZUizhY1Oau6-kO7k6EU1A",
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
        "id": "HPFYo2M67cO94fNUfFqgtC7m1KvU2xls33OzEPtGN3NQFLQlQgVADWblalBpFwA0N6soz6l8YdwU3YbDCRqGfg",
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
        "id": "zQECmSh2ns8ZLfEQScqatfFFhkooJ39oZ1dU-amJnKZTKXj0Mm24Mpp9ICJ0TTqm9HAmZnkQWnmC_cfXHj99Lg",
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

✅ Script saved: 4JUkKT74JRdrZj6MiiGuXir96RZa5pS7uNT38__I9nvW-b5RZZ_9qhtenOEFYjDHzNKa3EyvNXBCkUU2SguLOQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: _tITlIlwR9wmvcu52SwNRLp_Eh7fIgO2kL2lLZpfssodHUHL412bvguWU2DtXxP8yeeiaKk2MT6g9BTm5QEXpQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: v3G6OFGYhDbwteHFjWhjW6bmcyf8C8NWyzoiGMKuxqHCAwV67f7u7jUqJVs_9eCrMgFjetO1jBDLeza--bdUWg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"status":"inactive","avg_score":50,"max_score":90}
   {"max_score":100,"status":"active","count":10,"avg_score":60}

📝 Example 4: Function Management

📋 Total scripts: 33
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
Collection created with first record: 9iAFMzMlaRYRXQ-c8erUGiHDCH1OJyMVQzsW39oU82Op7IvCME5SZJhfrJMs_Qz1mMDo5s22jglQu65QyPqtEg

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_documents_client_py',
  'enriched_users',
  'demo_collection',
  'schema_users_client_py',
  'chat_messages__ek0_testing'
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
Created Alice: $1000 - ID: Xkv45ap8gbOY7d_XN_vqhBYA99Brz_hGCUTCQQXSpuP80fctsmFSWuldCgxf6Tv_vqs0bgcAOzSU_T1fZhbv8A
Created Bob: $500 - ID: DzsSJiUqN3p81m22aAZUs25no_zeCK-dTmdG2daCdlIxoA0jxUgBXwF77LtogT73cFb0Kh2eAlUQPogytIC7TA

=== Example 1: Begin Transaction ===
Transaction ID: 10cfb125-be0e-46fd-8273-c069fb3a1878

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"value":800,"type":"Integer"}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: e6ab6f2f-2cb7-48d7-a081-c7f35bc2bc81
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

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
   ✅ Script saved: 81MfO_-0l3NsRXg92oDM40KNx7e1aYJ9574IsWAAp0L0hf6WslbGp8p442rOGlm-6HtUVkVaqujxgvtIFdMwLQ

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
   ✅ Script saved: ROBx0AnvQ48f_BHj3-pB5k4b9TDrkhzl2Ynv20fF_QvNU2_EjqTij090O69mQqBK0cWjuSJrDoi-ntHIA7wqvA

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
   ✅ Script saved: 8ACgkDwUsH9QTad9Glr0llP-BwPUhqZtD_uS4uRl7etXS9h55Zha5S9BLeK-M_N-b3SmBYjQVCRwW1FiMjebBw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ucR6Y3r8AzgXUKnA6whFwnDnA_lQ6HNVbV0u9UiOQPSaRDnvJLUTsVjGzOV4RHaID9Lx8GpUxWy6eKjTlCucbA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 81MfO_-0l3NsRXg92oDM...
   ✅ Deleted script: ROBx0AnvQ48f_BHj3-pB...
   ✅ Deleted script: 8ACgkDwUsH9QTad9Glr0...
   ✅ Deleted script: ucR6Y3r8AzgXUKnA6whF...
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = fjgKhLOxCA8N6cSKhD9O104blTQuOjbOuNQX6Vtm5aKfVxUFrVv76jqFQPbI_liqdJlRtKNWoh368LAH821LMA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(fjgKhLOxCA8N6cSKhD9O104blTQuOjbOuNQX6Vtm5aKfVxUFrVv76jqFQPbI_liqdJlRtKNWoh368LAH821LMA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(fjgKhLOxCA8N6cSKhD9O104blTQuOjbOuNQX6Vtm5aKfVxUFrVv76jqFQPbI_liqdJlRtKNWoh368LAH821LMA)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = FHC8cSOhjNPecFnluXuHy4zQzJPCOB3wfGt-mP9XJ1rZsdaziJ3uTASd_l4LdSKf_LL5YzVrwV_ZMqpmv3r-4w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(FHC8cSOhjNPecFnluXuHy4zQzJPCOB3wfGt-mP9XJ1rZsdaziJ3uTASd_l4LdSKf_LL5YzVrwV_ZMqpmv3r-4w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(FHC8cSOhjNPecFnluXuHy4zQzJPCOB3wfGt-mP9XJ1rZsdaziJ3uTASd_l4LdSKf_LL5YzVrwV_ZMqpmv3r-4w)
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

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'S5vS4l5Hnx3xX-p0--0XcjsB9BNcDEeV99MK8rD3fXhrPzrw1Xl0SQy9gugsvr7YqcUTtMC3FMaEPL_puVdrDA'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'S5vS4l5Hnx3xX-p0--0XcjsB9BNcDEeV99MK8rD3fXhrPzrw1Xl0SQy9gugsvr7YqcUTtMC3FMaEPL_puVdrDA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'S5vS4l5Hnx3xX-p0--0XcjsB9BNcDEeV99MK8rD3fXhrPzrw1Xl0SQy9gugsvr7YqcUTtMC3FMaEPL_puVdrDA',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: DBkSXNM2bCPxwvDXfd5XdbEIZT-rdYZw8uwrawn3JTgTkW_Nf2OIhAEtnYEmqNbDfzluj6o0vGCUWYBEnlqpzg

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
Collection created with first record: ONZEDXQbo5EZBpUHv-UmWazGJJ-04RY27HIGO1NqFFz_dg7GVKoDsVv2K9GhSD6gX1AgLSTjHc-S0Xs4uaV_hA

=== List Collections ===
Total collections: 24
Sample collections: schema_documents_client_py,enriched_users,schema_users_client_py,chat_messages__ek0_testing,ttl_cache

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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 15

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: rPmBg_tvXNdsjjT-YA85eT3BZ5ir7usNiGIVy4r_KbD2_vSGkKG_bumlds5dN0Lo2zBCcgm6Cx4levVPCtXYkw
Created Bob: $500 - ID: QuH-hYB5x9sNhKwlMtpBEKxbML-GJ0Cw5_qJVuMCPWdGen-1x5xoCDwDnyUHxcPh57KE-ecMN_KOl1A3tLDySQ

=== Example 1: Begin Transaction ===
Transaction ID: 0f900436-197b-4773-b325-9bdb909af724

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 2e8c3a9c-dae2-417b-91b6-108d61b3e85e
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
  1. Score: 26.400, Matched: title, bio.value, title.value, bio
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: bio, title, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: title.value, bio.value, title, bio
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: title, bio.value, bio, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.743, Matched: 
  2. Score: 0.739, Matched: 
  3. Score: 0.736, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694, Matched: title, title.value, content, content.value
  2. Score: 1.496, Matched: title.value, content.value, content, title
  3. Score: 0.297, Matched: 

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
✓ Inserted document: Ufr6B22t82wlSmp1Xr_adURY6JgqkODEmQ1hq8i9s44LL80VKj2CxRPFxHH0NX1n9qrVrmUc6e6902H4-VHFGQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4hVGqUdtTjkxnvWRdIZAPpgXOyL5b2yFhBrB3orx-aMyRgnN7j5a3GcIyXPHBzo6ECN4XoOr1GZjvUL7QrHx-w

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
✓ Inserted document with TTL: 8WnuU9j8dimBxyJnpvDrLQ_LRR2rsxiMIbl7KdT06PLHbQbdSB0_LcfiUYUekbmBLP56j96orovGBzz-9D2ggg

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
✓ Edge cache script created: RMayYq78UvUgCti3Fzv5R1aBPPsZHIrEjVBkBuhHYEluO9puZnkz9lyVj45TDvmiJqzf5eQLCwcf__Erv_1p4Q

Call 1: Cache miss (fetches from API)
Response time: 43ms
Result: {
  "records": [
    {
      "value": {
        "elevation": 32,
        "timezone": "GMT",
        "generationtime_ms": 0.02014636993408203,
        "latitude": 40.710335,
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "current": {
          "temperature_2m": 7.1,
          "time": "2026-04-03T14:30",
          "interval": 900
        },
        "longitude": -73.99308,
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "°C",
          "time": "iso8601"
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

Call 2: Cache hit (served from ekoDB)
Response time: 43ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "elevation": 32,
        "timezone": "GMT",
        "generationtime_ms": 0.02014636993408203,
        "latitude": 40.710335,
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "current": {
          "temperature_2m": 7.1,
          "time": "2026-04-03T14:30",
          "interval": 900
        },
        "longitude": -73.99308,
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "°C",
          "time": "iso8601"
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

✅ Script saved: tFu6bwuHKVUnV7817Ya0_d-PlzBpQ9ipVw4TArW_d2zPePsh-vERtz3t3Z2keLCwvMpS-Ifiu6XtrInnR4ywiQ
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
   {"avg_score":60,"status":"active","count":5}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 36
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 172ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "username": "Bret",...

Second call (cache hit - from cache):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "username": "Bret",...
   🚀 Cache speedup: 4.3x faster!

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

✅ Script saved: RnyoQfHRvJgimS8cU1YT5EEq7QLFw0ZzG0SWLz4LCyGPL_IeFlKUx7ecDN_q_cTaGQeDB8dhzShykb6OclFyEw
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"category":"Furniture","count":2,"avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"category":"Electronics","count":3}
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

✅ Inserted order: gaTJaPkAAjy03sGD2bnkHLqWu1CNBERctU3Jnu2SR6XBf9HZFyokYD_yTM--8TysEpytBcWG4RZS_MCv2QKKpg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: _E9WIEeNNtUy9TarNlFBlrF8KjwiETFLk5qXWXO1sK0GksDWEVqT4oaFR-agVVDlwrrGpoFfWZO8cC1k9AG4GA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: MtoAV3tZBhwaPVzDv5OniCZgnB-ztIg2RaqZ2kfFDIRBmRijWWnykPXBQTp6dNrz_5xOJU4TwLH9n2lwJcccAg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 0AahoMEoKMLyZEkwILhKvpRQhqsNSaeKv_nFxYV5bGP3wTPSZW-zPHCHq-Y0BaGn2qesFMbEgLSDsdewkbM-7A
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
   1. User 9 - Score: 90
   2. User 8 - Score: 80
   3. User 3 - Score: 30
   4. User 6 - Score: 60
   5. User 10 - Score: 100
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
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
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
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Vector Databases Explained
   4. Natural Language Processing
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Vector Databases Explained
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
   1. Standing Desk - $599 (⭐4.7)
   2. Wireless Mouse - $29 (⭐4.5)
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
   1. Enhanced Representation: Vector databases provide better representation of physical features like roads, boundaries or utility lines, maintaining its geometric characteristics.

2. Scalability: Data scaling doesn't affect data quality in vector databases. You can zoom into the finest resolution without the loss of detail.

3. Lesser Space: Vector databases consume lesser storage space when representing sparse or simple data compared to raster data.

4. Precise Geoprocessing: They are capable of complex geospatial operations like network analysis.

5. Attributes Storage: Each geometric element in a vector database can store associated attributes in a separate database table.

6. Seamless Data Integration: Vector databases allow easier, seamless data integration at various scales and themes. 

Note: In certain contexts, such as storing extensive continuous data (like satellite imagery), a raster database could be more beneficial due to its simplicity and less processing requirement. The choice often depends on the end use of the data.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Draft Article" - 1536D vector
   3. "Advanced Query Patterns" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (-hAq8EvrrGA42nzmPpxKK9m_o9USG_k0CunTzeKCrKrbXbPNRrtEAwBlJ4c_k5nev-yYoxW8LqzoxqFfbXBiNw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "address": {
          "zipcode": "92998-3874",
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
        },
        "id": 1
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
Response time: 43ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB JavaScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (G5Jq9pzhZPhbAZDrt6auZ2_MnhTUBAougKwUw2jQhrxTTcrW1P4fnJV4GCwndrAw1XY5nS1bt_f_mbRm_HzW_g)

First call (cache miss - will fetch from GitHub API):
  Response time: 18ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 18ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (R9x9OY380d74NRQkztxHjKkhKPvH-b39XEYms-OrExSBPfhwEsF3lmb1MuFfcStcNEtusjo-OCSNRvC5xk_lZg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (iPfn6sw-eDU3JcDki3sagYUOEYMptejdynFaBTrCqP1tu1mxs9JYSAQrXOFrhhroL4kZJ1TSOz5yU0nP5hXTow)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (OidsbzJoDVZUUAk4fIZXSj9ewWi72FATp1QSU0LksjjPmP5IWyb55MHvNFTF4Arbiot6tLH1cCkH4l5O05SiwA)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: TACevgeaAEK8DCC6behQNb0-cSKX5ShegnEJwpFGJuK0XuguAsOd2Wtwuy0dEd6zRf0OpRx-b5yvyur87RmVMQ

=== Sending Chat Message ===
Message ID: YM6nIHsZYCtUxMIPAH8DqXtlUQ1KF-L4CqV_-0dvbkx751kTWTPIY0ZCy_ceQQAjXMGSb5lpnTsJnfOUJxnnsg

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299,
    id: '-uzbCPnyr4272EM5JnKvah4vStJBAPCyVV3jVU-jDm9fMAoLO0IT4Bs-ftjdKZ7XSMM7aDFpINzn9RKeIq214A'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 'bwTXfR6-5V5qgGmd4n31plDWQ6L2o6M_pnbOsHz2BmBKjVXRCU_iP6qOXg__Q2mQSyfMw3118SDmwKW83T8TRQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: '67AZPtLzyw5O7Cq6jZT1Hwy3jlpygvilgZKHPucQnVWj5sTAz-PT2xeHLYlfCUHDkh1tLB93QNbduZ-I4t7dqg',
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1844ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: YRU_d09f5vtMc6Ymp5sPTZ0mPTCVSK3TL0ZXnTDT6IuP8EVrMbhRzfuu-HYO6j-auN6dOdo2VoJSjmKvay0zmQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

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
✓ Created second session: dzx_V2zuWyw-ttvB-MQMGQbv6nK_X40--I9J7rl1dfng1WYUIxFEsyTxo1q7S_1T7RcQZ7vLTstAmiR-DAp0uw
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: FfUQiwUld04uVa9LrYA4QI2a9NNUpOkcG15SFYBsmWxZPUUYVMkpi1buICKkWghzSYpk8Lwq8yonsiOssZyfIQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

### ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more details or further information about other products, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: K5qzKlXXLtf4UmXEEEg51BOBy78srrgcYzFUvsEBe7oU1sZox56nFynl1iIixtZFV1hH9KoZda5IXwfOnxVb5Q
  Parent: FfUQiwUld04uVa9LrYA4QI2a9NNUpOkcG15SFYBsmWxZPUUYVMkpi1buICKkWghzSYpk8Lwq8yonsiOssZyfIQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: K5qzKlXXLtf4UmXEEEg51BOBy78srrgcYzFUvsEBe7oU1sZox56nFynl1iIixtZFV1hH9KoZda5IXwfOnxVb5Q (Untitled)
  Session 2: FfUQiwUld04uVa9LrYA4QI2a9NNUpOkcG15SFYBsmWxZPUUYVMkpi1buICKkWghzSYpk8Lwq8yonsiOssZyfIQ (Untitled)
  Session 3: dzx_V2zuWyw-ttvB-MQMGQbv6nK_X40--I9J7rl1dfng1WYUIxFEsyTxo1q7S_1T7RcQZ7vLTstAmiR-DAp0uw (Untitled)
  Session 4: TACevgeaAEK8DCC6behQNb0-cSKX5ShegnEJwpFGJuK0XuguAsOd2Wtwuy0dEd6zRf0OpRx-b5yvyur87RmVMQ (Untitled)
  Session 5: rBcttguKjtUmQaP0Z26C6gzi94TQ0VBYNnAZF-8z-xgA2IFaI-S4GbCjDOd4-v7q31NIp-Z8edZQ56lcIq-stA (Untitled)
  Session 6: KJ2Vm-XGaYFT_Mjbw663wXs8yVSmkFNPdtMfZ93FnDWGf4amRPmeGkaUnV1LJEEw4qAjD-E20o29js71iN_hiA (Untitled)
  Session 7: cVJund49jxJ49ZJ62Ez4AOw2mv2BL6yOW1CDpzkjGwn1zybbFwvoX3t0fxtyRFeWX2FeN168tsNhiFr0UPaWng (Untitled)
  Session 8: V27O2IYkA-pS1ycp6WPrAHKsQ13UVW3vdQYJ0B2rpstg36Mdt_coi2ly3d--01TDPSjMBBIv7vcmLr6TSNQNQQ (Untitled)
  Session 9: X1YaK9zkzOvX5JG8Ma4c37086VzIogvN7du-5hV8p5dlee0vSPBYRRVcndmTS0l9jjTM4Jbqx05XyEzVKdMzkg (Untitled)
  Session 10: 2fK6YIWVrxfkHp49rcnkCpUVif4R7CdjOPIjns1j8lazSs3PDEWUJ03Exa-HOcGNKCO6Lbf2no8Cc8eTrBnPfw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: K5qzKlXXLtf4UmXEEEg51BOBy78srrgcYzFUvsEBe7oU1sZox56nFynl1iIixtZFV1hH9KoZda5IXwfOnxVb5Q

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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=Llsd2_ZmgWWTqrNrM5MYLWdsdKFFzy84tyT9_NLfxbVi7vAisfnUrOYt-r_pTPnLifVMOpWt-_KeSiLr1oWsLw)})
  User ID: Llsd2_ZmgWWTqrNrM5MYLWdsdKFFzy84tyT9_NLfxbVi7vAisfnUrOYt-r_pTPnLifVMOpWt-_KeSiLr1oWsLw

=== Read ===
✓ Found user by ID: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), id=StringValue(value=Llsd2_ZmgWWTqrNrM5MYLWdsdKFFzy84tyT9_NLfxbVi7vAisfnUrOYt-r_pTPnLifVMOpWt-_KeSiLr1oWsLw), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Fri Apr 03 10:36:20 EDT 2026)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Fri Apr 03 10:36:20 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: name, data, price, metadata, email, age, user_id, active, embedding, id, tags, created_at, categories

=== Update ===
✓ Updated user: EkoRecord(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), id=StringValue(value=Llsd2_ZmgWWTqrNrM5MYLWdsdKFFzy84tyT9_NLfxbVi7vAisfnUrOYt-r_pTPnLifVMOpWt-_KeSiLr1oWsLw), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Fri Apr 03 10:36:20 EDT 2026)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), created_at=ObjectValue(value={value=StringValue(value=Fri Apr 03 10:36:20 EDT 2026), type=StringValue(value=String)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), id=StringValue(value=Llsd2_ZmgWWTqrNrM5MYLWdsdKFFzy84tyT9_NLfxbVi7vAisfnUrOYt-r_pTPnLifVMOpWt-_KeSiLr1oWsLw), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)})})

=== Delete ===
✓ Deleted user with ID: Llsd2_ZmgWWTqrNrM5MYLWdsdKFFzy84tyT9_NLfxbVi7vAisfnUrOYt-r_pTPnLifVMOpWt-_KeSiLr1oWsLw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"data":[{"id":"ZabZeP8VYjQo5KbsaxeJUafjgfBfxOspADQAOIQJkEU1EPLWvkvmz64YlJQpTdwMlZwTlAe-zgpq9SAbY5NiZg","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]}

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
  IDs: O6EcYGYboduoHJpkJQ0F30IAP-xqr-CB8URXlDwa4U-xoZbg40gV5xOCuObHl7tdbOMl44HNOTTUjY35G02fZg, nmxs77gURnp6tgUvkWsZxVjf90iuNbrXxdWjEuZfUGE76aS3RgsyfQCjknJOn8-kX6UnWUemvSrqSURlF2n0LA, px_9RXzupv52-J4Yjt-WCcDyZmP3WD3A4SD8qiYTRfAw9n_VLSYLX8lWn6fPsSsxDD2vbHbgxOL-MYgaiO1F-Q...

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
✓ Found 28 collections
  - schema_documents_client_py
  - enriched_users
  - schema_users_client_py
  - schema_users_client_js
  - schema_documents_client_js

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-04-03T14:36:36.528557Z","last_modified":"2026-04-03T14:36:36.528557Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","role":"admin","name":"Alice"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1775227001205}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"port":5432,"host":"localhost"}}
  config:cache: {"value":{"enabled":true,"ttl":3600}}
  config:api: {"value":{"retries":3,"timeout":30}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 16

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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: yigp3EiJOkJCuWqBWWD82A9bGKaYTejEX8QzCMdVLWr8d2ZZLK4HAiOGAXWlcx1vybscnZRDso94qxWqQ7HaKw
Created Bob: $500 - ID: ca8BhLne37fSXJ322K8OhDNKUgEn_0h0fwgyKLJtlpzTjzEOdunZCg79vvEHQbDm3PvZd-UpwExNwuR0imNN1w

=== Example 1: Begin Transaction ===
Transaction ID: 4526e972-aefa-4fbc-a5af-b42567839fc3

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 867bc6aa-9568-4be6-859f-8e53cf475c9e
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
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
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
  {"results":[{"record":{"tags":{"value":"programming,rust,tutorial","type":"String"},"id":"78gLo6eJI_YmOKM0L0WzoC1q0cskndAhD4t08GGtzix2pYmvxWZYPQbcD9oUK6vRxjnmKAnfqC5DaoGuRYYRDw","title":{"type":"String","value":"Rust Programming"},"views":{"type":"Integer","value":624},"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."}},"score":19.8,"matched_fields":["tags","description","title"]},{"record":{"title":{"type":"String","value":"Python for Data Science"},"id":"l1kIcYd0nxJ2xMnhbPwjKgQ-4pvZkHVG_eTcCBZRXrnKriDpsIcVMFLRrOb-EfNBPXuBGNc8vYHmeNMWkzH11g","tags":{"type":"String","value":"programming,python,data-science"},"description":{"value":"Master Python for data analysis, machine learning, and visualization.","type":"String"},"views":{"type":"Integer","value":50}},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":{"value":"programming,javascript,web","type":"String"},"description":{"type":"String","value":"Build modern web applications using JavaScript, React, and Node.js."},"title":{"type":"String","value":"JavaScript Web Development"},"views":{"type":"Integer","value":50},"id":"rgzfyfdIclPuSwZGw7Mn6OeHkOMO8bc2_htdu9hB5xaPoDKq1F8l7Pmzxd_h_h196Fv1yuOgua8ltYxYSXIgog"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"zD85dwEKFCNHyt543GYbTcgwnT-nrhhoTeICdC5GNBK9F9z7Yv77xAjI5GQUyS7i0tXy_-04dLdaGL2ehKCaNw","tags":{"value":"ai,machine-learning,python","type":"String"},"views":{"value":894,"type":"Integer"},"title":{"value":"Machine Learning Basics","type":"String"},"description":{"type":"String","value":"Introduction to machine learning algorithms and neural networks."}},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"title":{"type":"String","value":"Python for Data Science"},"tags":{"type":"String","value":"programming,python,data-science"},"views":{"type":"Integer","value":50},"id":"l1kIcYd0nxJ2xMnhbPwjKgQ-4pvZkHVG_eTcCBZRXrnKriDpsIcVMFLRrOb-EfNBPXuBGNc8vYHmeNMWkzH11g","description":{"value":"Master Python for data analysis, machine learning, and visualization.","type":"String"}},"score":1.0,"matched_fields":["description"]},{"record":{"title":{"type":"String","value":"Rust Programming"},"tags":{"value":"programming,rust,tutorial","type":"String"},"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"id":"78gLo6eJI_YmOKM0L0WzoC1q0cskndAhD4t08GGtzix2pYmvxWZYPQbcD9oUK6vRxjnmKAnfqC5DaoGuRYYRDw","views":{"type":"Integer","value":624}},"score":0.5,"matched_fields":["description"]},{"record":{"id":"krD5-mg36CVx-e4h6JO1kq9GZz0PY9O9ct1TGoDTB1hrLPQ8XCCcHg7rS0U3qc8CUvAdFO287NbESq508sbjyw","description":{"value":"Learn database design principles, normalization, and query optimization.","type":"String"},"views":{"type":"Integer","value":178},"title":{"value":"Database Design","type":"String"},"tags":{"type":"String","value":"database,design,sql"}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted user 1: StringValue(value=eynPLhLbbxEVqKO_aoUUs64vKx_rOhZDY8K7CA2SpKkFSYg3NWONWC-yX42foRxpvtKUalvqlLvLC9qYk3UNvg)
✓ Inserted user 2: StringValue(value=jOOPTpGo09WcPI6CdPF8BRUdCjm0FCDTsMvpVMjKhhsA1IoBjtJjMsPFbsQhFpm49WUh7GEUQkJ3FDs3sCp2Yw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 29
  Sample: [schema_documents_client_py, enriched_users, schema_users_client_py, schema_users_client_js, schema_documents_client_js]

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: tLDmByW72bgoj00rGhRyscgbCr73qpDtxJGPyndJ7S21LzfzNy2sor9IAOQPkl8V2pLNW5CKwE3NggjvI3r7Hw

=== Verify Document Exists ===
✓ Document found: user_id, created_at, id, session_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: Oz0HF7EpwYQFD5RIoPc_egnyH4xniAttAgfV7YzBB3-05_YlJHsEnYLL4SxAuJisnoM8fMJrOM3QcvrffMFJyw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: id, value, cache_key, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

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
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=oA0OvRBDpKuPNL8oMr5qyNrDgtJqF4MaRQXbYPvDVn2IxLEaDYzA9dbNGlnNW88JU3c-U4bV7LLaUlwQW2OgQw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1775227035687},"id":"oA0OvRBDpKuPNL8oMr5qyNrDgtJqF4MaRQXbYPvDVn2IxLEaDYzA9dbNGlnNW88JU3c-U4bV7LLaUlwQW2OgQw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-04-03T15:37:15.789656Z","value":{"type":"Integer","value":42}}]}

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
✓ Edge cache script created: _e0TkPp1zMRwg7C4E9xZVjix9JfGardhLcE51s3N2T6lh3hERz2Bjq8RQgxynZQgQVLmXhd77WMUhmCnNdJVqQ

Call 1: Cache lookup
Found 1 cached entries
Response time: 27ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 18ms

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
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: rOv2oM-B10doTMiJfDxTJzTbNdnxMYj_i4ZmBjBM6bwMyRzkKunZoY-zu9ct7_Jcaq9xuS4xCocnBMpBbDAWzg
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"status":"inactive","avg_score":50.0,"count":5}
   {"count":5,"status":"null","avg_score":60.0}
   {"count":5,"avg_score":60.0,"status":"active"}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 36
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
✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

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
🚀 ekoDB Kotlin Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: mEkPZdrfGYzWgcinLm1b_glkLx8utQXn2AuQrQ_-q09fzh95BI0BwZ2EQ9XkGCtmVgNXBbJUnHSCoLNEssUDaA
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

✅ Inserted order: StringValue(value=z1mR081R4nhPS1dFkiXQwJIfpN_OrtebB8BFu6k6xJs69DBOPCSpD6d2tUKi4BwTjikqn-VpWu_12k2HE1q1TA)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: lhqtRWDdaQYx4-kZ4DVodPoYVQYP_8H4IDwJJ-tu2RE2SFmDBIrnI8C0a6HlCjyWKCG_SZQdXWm1nVKjfw85Jg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: KCbTlNzusKhKLGKgkAWKU0go4mI_m9uW_488g6qc7eq4ZGRlwK8asvH2UX8RF_R0rm8tGWn6fi2MKi690OLoVg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: kR9eD4stRlOtuO-4gujAt9-j9ilKZr4TBhJcoIqJF5Fup0uvRUJyoFoQZuZSL3aLrCZNc9QwvyFR8bKGdQx6KA
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
✓ Created SWR script: swr_cache_lookup_kt (-mddefO4dg-fft7vUv-fotdbjWsTxtoRPUinfL7f6ZdnpJ5EE2J8SsSlEv0xXRsLIoP9CzU2qpbY-CsTuwSLkQ)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 17ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

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
🚀 ekoDB Kotlin Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (spuRpIeYeAWrvOVR7y4b7SUqBVaknGN_IcATf-EfxNyXH_VQC6zK0C2vznkL5F32jvFziEixTjBVZ4m23iDxiA)

First call (cache miss - will fetch from GitHub API):
  Response time: 29ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 19ms
  Speedup: 1.5x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (Z_fpAfqWCkOwmVlNH5OyHYTI0WGnOkpBJUpyNwH0gdA-1QcNrsW3ounccawskrK4YsSIWOiXJdnHIWirZMzMfg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (lawDQfWLsxaduASmETjb9kmgEw-vC6pBRk0N_zSorB0xD9ZKuk5euuMmbu4NZHRSD0bz4uVIijTGyQfcWe3sdA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (GdjTKZwvGAs6ioyzmMLQg41z3JIaeLukSjgyQz5uwoWkm-RUBneWTfQ-ISESGgk-DjD8GQM_X02XUE6uf7XG4g)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!

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
   {"count":3,"avg_price":365.6666666666667,"category":"Furniture"}
   {"avg_price":367.0,"count":5,"category":"Electronics"}
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
   {"tokens_used":258,"response":"Vector databases offer several benefits:\n\n1. **High-dimensional Data Handling**: Efficiently manage and query high-dimensional data, ideal for machine learning and AI applications.\n2. **Fast Nearest Neighbor Search**: Optimize similarity search using advanced indexing techniques like Annoy or HNSW, enabling quick retrieval of similar vectors.\n3. **Scalability**: Can handle large datasets with millions of vectors, making them suitable for big data applications.\n4. **Rich Query Capabilities**: Support complex queries beyond simple keyword matching, such as semantic searches.\n5. **Integration with ML Models**: Easily integrate with machine learning workflows for tasks like recommendation systems and image retrieval.\n6. **Real-time Processing**: Facilitate real-time data processing and querying, essential for applications needing instant responsiveness.\n7. **Multi-modal Data Support**: Can store and query various data types (text, images, audio) in a unified format, enhancing versatility.\n8. **Improved Accuracy**: Offer better results in similarity searches by capturing nuanced relationships in data.\n\nThese features make vector databases particularly valuable for AI-driven solutions and applications requiring contextual understanding."}
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
   {"status":"active","count":7}
   {"count":3,"status":"inactive"}
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
   1. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   2. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
   3. {"value":"Database Design Principles","type":"String"} ({"value":"Database","type":"String"})
   4. {"value":"Getting Started with ekoDB","type":"String"} ({"type":"String","value":"Database"})
   5. {"value":"Introduction to Machine Learning","type":"String"} ({"value":"AI","type":"String"})
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
✓ Created session: lmoQi-kUem7b7Fcc1ncAL_9NVBLzI8YoRMTN6PnLXInCfHKR08Nj1xACEhHPAAhf2qXd1frG7yEMsM-Vqdk6gA

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "uz8h8i5MxeMNJV4jFsyQcVbXJjAsyNVbHR2aIvMT3I0y30dib4RMd54RIxpFqD07X9ekdpRsj_-nzOtzXUuUYA"
  Responses: ["ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. It supports various advanced features, including:\n\n1. **AI Chat Integration**: The chat feature allows users to query the database using natural language and receive AI-powered responses with relevant context.\n\n2. **Search Features**: ekoDB supports full-text search, vector search, and hybrid search, which includes automatic context retrieval, enhancing the search experience.\n\nThis combination of features makes ekoDB a robust solution for managing and querying data effectively."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 12s
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
✓ Created session: XEK0fdOZPYoVikBb67mOytmk9eNhwGpl-yHqpOsCrqUxM2dN2GHrP0xy2TQx6lzyIySzj_am0YTjsGySZL1zpg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name**: ekoDB\n- **Description**: High-performance database product\n- **Price**: $99\n\nIf you need more information or have further questions, feel free to ask!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Currently, we have the following product available:\n\n- **Product Name**: ekoDB\n- **Description**: High-performance database product\n- **Price**: $99\n\nIf you need more information or have any other questions, feel free to ask!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: eyu6kcYhapfthpoq63J2hP3EXgK22n3Oj2S1IrCDZTSIaoRAPNZLXyOzA9xl7gdybw366bz-5l_zhp_jQzQqJQ

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 2

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 23s
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
✓ Created session: s8chFYbGLuhH5hVRCQPQvmu7DDXd-4OUT95jLD0NCNlk78DVuFd_PCqzLhtejFCsznL2k-EyWbuG5vovb-cwwg

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product:** ekoDB\n- **Description:** A high-performance database product with AI capabilities\n- **Price:** $99\n\nIf you need more information or want to explore additional products, let me know!"]

✓ Message 2 sent
  Responses: ["The price of ekoDB is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"s8chFYbGLuhH5hVRCQPQvmu7DDXd-4OUT95jLD0NCNlk78DVuFd_PCqzLhtejFCsznL2k-EyWbuG5vovb-cwwg"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"uwO9YOJdh2ygsYiFXnze_NuLj2MvXrAgysRwvEJORQ5wzv12JLFPO4jhOhFjFd5ZqabvRzARDVpNrJfxHXyJzw","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-04-03T14:38:48.910836Z"},"id":"S4tysI8eBTqJ4ncU4rukgHIGmzdethkct0vobofQ2FreHQaZKxjfSZNKtahyJE79QsasyMBeMA0BJspxSe8nrA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":81,"prompt_tokens":2529,"total_tokens":2610}},"updated_at":{"type":"DateTime","value":"2026-04-03T14:38:48.910836Z"}},{"chat_id":{"type":"String","value":"s8chFYbGLuhH5hVRCQPQvmu7DDXd-4OUT95jLD0NCNlk78DVuFd_PCqzLhtejFCsznL2k-EyWbuG5vovb-cwwg"},"content":{"type":"String","value":"The available product is:\n\n- **Product:** ekoDB\n- **Description:** A high-performance database product with AI capabilities\n- **Price:** $99\n\nIf you need more information or want to explore additional products, let me know!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"uwO9YOJdh2ygsYiFXnze_NuLj2MvXrAgysRwvEJORQ5wzv12JLFPO4jhOhFjFd5ZqabvRzARDVpNrJfxHXyJzw","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-04-03T14:38:48.913449Z"},"id":"Sy6aS8fOC6hwISb_qdZozOaiS1s8hTCRsgti_Z8PSxQauKZHKBjZ6AxxnndQqN9a_vHhKNA6Z2GG_BsrHPxRsQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":81,"prompt_tokens":2529,"total_tokens":2610}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","limit":10,"select_fields":["product","description","price"]},"id":"call_zVvhi6OsYnFHMO8ujm21UtMs","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"uwO9YOJdh2ygsYiFXnze_NuLj2MvXrAgysRwvEJORQ5wzv12JLFPO4jhOhFjFd5ZqabvRzARDVpNrJfxHXyJzw","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_zVvhi6OsYnFHMO8ujm21UtMs","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-04-03T14:38:48.913449Z"}},{"chat_id":{"type":"String","value":"s8chFYbGLuhH5hVRCQPQvmu7DDXd-4OUT95jLD0NCNlk78DVuFd_PCqzLhtejFCsznL2k-EyWbuG5vovb-cwwg"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"uwO9YOJdh2ygsYiFXnze_NuLj2MvXrAgysRwvEJORQ5wzv12JLFPO4jhOhFjFd5ZqabvRzARDVpNrJfxHXyJzw","price":99,"product":"ekoDB"},"score":2.112195121951219}]},"created_at":{"type":"DateTime","value":"2026-04-03T14:38:50.168129Z"},"id":"3AnlIzu1iKJXuYCWVAGNXvZ7qODGfylKWf-ybMMcnn9hjsYHsB4nbok4dBpdYu42B6MQun3z33_liwzljt_1jw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":6,"prompt_tokens":1284,"total_tokens":1290}},"updated_at":{"type":"DateTime","value":"2026-04-03T14:38:50.168129Z"}},{"chat_id":{"type":"String","value":"s8chFYbGLuhH5hVRCQPQvmu7DDXd-4OUT95jLD0NCNlk78DVuFd_PCqzLhtejFCsznL2k-EyWbuG5vovb-cwwg"},"content":{"type":"String","value":"The price of ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"uwO9YOJdh2ygsYiFXnze_NuLj2MvXrAgysRwvEJORQ5wzv12JLFPO4jhOhFjFd5ZqabvRzARDVpNrJfxHXyJzw","price":99,"product":"ekoDB"},"score":2.112195121951219}]},"created_at":{"type":"DateTime","value":"2026-04-03T14:38:50.170793Z"},"id":"lmNYUFODdyXzSx3HWubOufvzHFwKkmWB3pbS1_n0To99QqhjoLjH4IQ2GG44XKPVO8JNu28NPq8QVTRJyQghOg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":6,"prompt_tokens":1284,"total_tokens":1290}},"updated_at":{"type":"DateTime","value":"2026-04-03T14:38:50.170793Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: h8F6ekGgqT9KtgNE7zbLcBhp_hXRiN8FT2FMICaKwfVFDlUainTU16OyaEPuwCs3xsX8774mK6R-KpQysH3D3g

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
=== ekoDB Kotlin Client - Chat Models Example ===

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  anthropic:
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
    - claude-3-haiku-20240307
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get OpenAI Models ===
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4-mini, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

=== Chat Models Example Complete ===

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
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: Y39JSbOfmk7QTQGYZUK2J8EKrC8lgKWS6juZb-tDJ1bp37wQq9qIfu6OmsCEYMtt_IEvKwzmUhw2OER7DM3OJQ

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 31 user functions:
  - "fetch_and_store_user": "Fetch user from API and cache in KV"
  - "fetch_api_user": "Fetch User with Cache"
  - "fetch_and_store_user": "Fetch user from API and cache in KV"
  - "fetch_slim_user": "Validate and slim down user"
  - "fetch_user": "Fetch user by code"

=== List User Functions by Tag ===
Found 31 user functions with 'kotlin' tag:
  - "fetch_and_store_user"
  - "fetch_api_user"
  - "fetch_and_store_user"
  - "fetch_slim_user"
  - "fetch_user"
  - "fetch_slim_user"
  - "fetch_slim_user"
  - "fetch_slim_user"
  - "fetch_and_store_user"
  - "swr_user"
  - "swr_user"
  - "fetch_user"
  - "validate_user"
  - "validate_user"
  - "cache_api_call_py"
  - "fetch_product_with_reviews"
  - "validate_user"
  - "fetch_and_store_user"
  - "fetch_and_store_user"
  - "fetch_user"
  - "cache_api_call_js"
  - "swr_user"
  - "fetch_user"
  - "fetch_slim_user"
  - "get_active_users_kt"
  - "fetch_user"
  - "swr_user"
  - "validate_user"
  - "swr_user"
  - "validate_user"
  - "cache_api_call"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=M5_Rfwyagb2sh1VdEvPnGduw2UcirlcHIudviLY54JQ9sT9L-LV1I86jBjTAcr1Lsi95AsqHG0-UbXtTeuYXTw)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), id=StringValue(value=M5_Rfwyagb2sh1VdEvPnGduw2UcirlcHIudviLY54JQ9sT9L-LV1I86jBjTAcr1Lsi95AsqHG0-UbXtTeuYXTw), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=biWs7KzJGVIAYaXLX2bD5KPq74JazHNuqwuaiCRSxaqSgbFkSCvh07mmNLujZrE_YVdvljCeiM0CA-Ad9vToUw)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), id=StringValue(value=M5_Rfwyagb2sh1VdEvPnGduw2UcirlcHIudviLY54JQ9sT9L-LV1I86jBjTAcr1Lsi95AsqHG0-UbXtTeuYXTw), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)})})
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

BUILD SUCCESSFUL in 10s
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=xS8DXAuI-7mwnNjF5Br1sYMQ7rExskGz4P2lyVGNYkMaafnWX_4Xt78lHlFJJruWoCeeAejl2TiM4TCGP2cOKQ)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=gBlSYjhJoJ-4nMlpkVeMaa3qGGIxnwnaafSi4UKkadTcQxRaXhV63u4PydFO7DsNcQAq3O1JjU56YceQnutfcA)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={id=StringValue(value=xS8DXAuI-7mwnNjF5Br1sYMQ7rExskGz4P2lyVGNYkMaafnWX_4Xt78lHlFJJruWoCeeAejl2TiM4TCGP2cOKQ), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=xS8DXAuI-7mwnNjF5Br1sYMQ7rExskGz4P2lyVGNYkMaafnWX_4Xt78lHlFJJruWoCeeAejl2TiM4TCGP2cOKQ), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)})})

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
  Password field excluded: true

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 3 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: YcMK6XRX_ZrRSslI0lkQB0z6tIQk2YA0sLDcoz40NVPGZjKmqlBRrWzuzMLB5Z657X6tr6AfMep7v5VOHw-doA
Created Bob: $500 - ID: IqKj5SuxnSC-u8kNN_W32fJTpm52RO1-yBdoFFK78fDMPQvh_F-srMzzs6nSjamcTUzQAKRoaDcLNzQanvE4Lg

=== Example 1: Begin Transaction ===
Transaction ID: f2a5e9bf-c58e-432d-8635-52c0c240860f

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9bd235d8-46c7-458c-9a6b-4167cef4f612
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 597.580625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 514.900292ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 619.087833ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.142717708s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 378.849125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.277440084s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 396.655958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.22568925s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 566.211875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 389.439666ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 294.357917ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 584.917584ms
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
    ✓ Generated embedding: 1536 dimensions in 590.817542ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 91.58025ms

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

Writing memory-safe, high-performance database code involves several principles and best practices that ensure the application manages memory efficiently while accessing the database securely and quickly. Here are key strategies to achieve this:

1. **Use Prepared Statements:**
   - Prepared statements help prevent SQL injection attacks and can improve performance by allowing the database to cache execution plans.

2. **Connection Pooling:**
   - Instead of opening and closing database connections for each request, utilize a connection pool that reuses a fixed number of connections. This reduces the overhead of establishing connections.

3. **Handle Exceptions Gracefully:**
   - Implement robust error handling when interacting with the database. Ensure that exceptions are logged and that resources (like connections) are closed or returned to the pool.

4. **Batch Operations:**
   - When performing multiple inserts, updates, or deletes, use batch operations instead of executing individual statements. This reduces round trips to the database and can significantly boost performance.

5. **Indexing:**
   - Create indexes on frequently queried fields to speed up query performance. Be careful not to over-index, as this can hinder write performance.

6. **Limit the Amount of Data Retrieved:**
   - Use pagination for queries that return large datasets. Return only the fields you need instead of selecting all columns.

7. **Database Normalization:**
   - Normalize your database schema to reduce redundancy and improve data integrity. However, also be aware of when to denormalize for performance needs in read-heavy applications.

8. **Use Transactions Judiciously:**
   - Use transactions to ensure data integrity but keep them as short as possible to reduce the chance of deadlocks and contention.

9. **Memory Management:**
   - In languages like C or C++, manage memory explicitly. Use smart pointers or RAII (Resource Acquisition Is Initialization) to automatically manage resource lifetimes.

10. **Profiling and Monitoring:**
    - Continuously monitor and profile your application’s database interactions. Identify slow queries, and optimize them as necessary. Use tools to visualize and analyze query performance.

11. **Security:**
    - Ensure that all user inputs are sanitized. Utilize ORM (Object-Relational Mapping) frameworks that can help manage data access securely.

12. **Concurrency Control:**
    - Implement techniques like optimistic or pessimistic locking (depending on your application needs) to manage concurrency without causing excessive delays.

13. **Use Asynchronous I/O:**
    - If applicable, use asynchronous database operations to avoid blocking threads and improve application responsiveness.

By applying these practices, you can write database code that is both memory-safe and high-performing, leading to a more reliable and efficient application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.275380167s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2905 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.204072625s
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
  ✓ Text search completed in 55.794541ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.535s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.644s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.587s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.277s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.532s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.403s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.433s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.602s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.496s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.405s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.564s
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
    ✓ Generated embedding: 1536 dimensions in 0.507s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.073s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.331] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.300] From conv_database_design
     What is database normalization?

  3. [Score: 0.300] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.256] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.188] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several best practices across different domains of programming, database design, and system architecture. Here's a comprehensive guide to help you achieve that:

### 1. Use Prepared Statements
- **SQL Injection Protection**: Always use prepared statements or parameterized queries to protect against SQL injection attacks.
- **Performance**: Prepared statements can be optimized by the database engine, improving performance when executing the same query multiple times.

### 2. Manage Connections Efficiently
- **Connection Pooling**: Use a connection pool to manage database connections efficiently. This reduces the overhead associated with frequently opening and closing connections.
- **Limit Open Connections**: Use appropriate limits on the size of your connection pool to avoid exhausting database resources.

### 3. Optimize Queries
- **Use Indexes**: Ensure that your queries leverage indexes effectively for faster lookups. Analyze slow queries and consider creating indexes based on query patterns.
- **Minimize Data Retrieval**: Only retrieve the necessary data by using specific fields instead of `SELECT *`. This reduces memory usage and improves performance.
- **Batch Queries**: When performing multiple inserts or updates, use batch operations to reduce the number of transactions.

### 4. Normalization and Database Design
- **Normalize your Schema**: Follow database normalization principles to reduce redundancy and improve data integrity. However, be mindful of over-normalization, which can lead to complex joins that degrade performance.
- **Use Appropriate Data Types**: Choose the most efficient data types for your tables to save space and improve performance.

### 5. Transaction Management
- **ACID Transactions**: Use ACID-compliant transactions to ensure data integrity. Commit transactions only after all operations succeed to maintain a consistent dataset.
- **Avoid Long-Running Transactions**: Keep transactions short and focused to minimize locking and waiting times.

### 6. Consider Concurrency
- **Use Optimistic Locking**: Implement optimistic concurrency control to handle multiple users accessing the same data, reducing the likelihood of deadlocks.
- **Row-Level Locks**: Favor row-level locking over table-level locks to increase concurrency, allowing more users to access the database simultaneously.

### 7. Monitor and Optimize Performance
- **Use Query Profiling**: Regularly analyze and profile your queries to identify bottlenecks. Use tools like `EXPLAIN` in SQL to understand query execution plans.
- **Monitor Resource Usage**: Keep an eye on memory usage, CPU load, disk I/O, and network throughput to identify performance tuning opportunities.

### 8. Error Handling
- **Graceful Error Handling**: Implement robust error handling strategies to respond to database exceptions appropriately without crashing the application.
- **Logging**: Log errors and performance metrics to help diagnose issues.

### 9. Memory Management Considerations (for languages like Rust, C, or C++)
- **Use Safe Libraries**: Choose libraries/frameworks that manage memory automatically and provide safety features (e.g., Rust's ownership model).
- **Escape Unsafe Code**: If using low-level memory management, carefully review and test unsafe code paths to prevent memory leaks or undefined behaviors.

### 10. Code Reviews and Testing
- **Regular Code Reviews**: Conduct thorough code reviews with an eye on performance and memory safety.
- **Automated Testing**: Implement rigorous testing, including unit tests, integration tests, and performance tests to ensure reliability and efficiency.

### 11. Use Appropriate Data Architectures
- **Microservices**: Consider using microservices to separate database interactions based on functionality, improving maintainability and performance.
- **NoSQL vs. SQL**: Select the right database architecture (SQL for complex queries and structured data, NoSQL for flexible schemas and high throughput) based on your application's needs.

By following these guidelines, you can create memory-safe, high-performance database code that is efficient, maintainable, and secure. Each application may have unique requirements, so continuously evaluate and adapt your practices to fit your specific use case.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.850s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 4334 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.562s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices across different domains of programming, database design, and system architecture. Here's a comprehensive guide to help you achieve that:

### 1. Use Prepared Statements
- **SQL Injection Protection**: Always use prepared statements or parameterized queries to protect against SQL injection attacks.
- **Performance**: Prepared statements can be optimized by the database engine, improving performance when executing the same query multiple times.

### 2. Manage Connections Efficiently
- **Connection Pooling**: Use a connection pool to manage database connections efficiently. This reduces the overhead associated with frequently opening and closing connections.
- **Limit Open Connections**: Use appropriate limits on the size of your connection pool to avoid exhausting database resources.

### 3. Optimize Queries
- **Use Indexes**: Ensure that your queries leverage indexes effectively for faster lookups. Analyze slow queries and consider creating indexes based on query patterns.
- **Minimize Data Retrieval**: Only retrieve the necessary data by using specific fields instead of `SELECT *`. This reduces memory usage and improves performance.
- **Batch Queries**: When performing multiple inserts or updates, use batch operations to reduce the number of transactions.

### 4. Normalization and Database Design
- **Normalize your Schema**: Follow database normalization principles to reduce redundancy and improve data integrity. However, be mindful of over-normalization, which can lead to complex joins that degrade performance.
- **Use Appropriate Data Types**: Choose the most efficient data types for your tables to save space and improve performance.

### 5. Transaction Management
- **ACID Transactions**: Use ACID-compliant transactions to ensure data integrity. Commit transactions only after all operations succeed to maintain a consistent dataset.
- **Avoid Long-Running Transactions**: Keep transactions short and focused to minimize locking and waiting times.

### 6. Consider Concurrency
- **Use Optimistic Locking**: Implement optimistic concurrency control to handle multiple users accessing the same data, reducing the likelihood of deadlocks.
- **Row-Level Locks**: Favor row-level locking over table-level locks to increase concurrency, allowing more users to access the database simultaneously.

### 7. Monitor and Optimize Performance
- **Use Query Profiling**: Regularly analyze and profile your queries to identify bottlenecks. Use tools like `EXPLAIN` in SQL to understand query execution plans.
- **Monitor Resource Usage**: Keep an eye on memory usage, CPU load, disk I/O, and network throughput to identify performance tuning opportunities.

### 8. Error Handling
- **Graceful Error Handling**: Implement robust error handling strategies to respond to database exceptions appropriately without crashing the application.
- **Logging**: Log errors and performance metrics to help diagnose issues.

### 9. Memory Management Considerations (for languages like Rust, C, or C++)
- **Use Safe Libraries**: Choose libraries/frameworks that manage memory automatically and provide safety features (e.g., Rust's ownership model).
- **Escape Unsafe Code**: If using low-level memory management, carefully review and test unsafe code paths to prevent memory leaks or undefined behaviors.

### 10. Code Reviews and Testing
- **Regular Code Reviews**: Conduct thorough code reviews with an eye on performance and memory safety.
- **Automated Testing**: Implement rigorous testing, including unit tests, integration tests, and performance tests to ensure reliability and efficiency.

### 11. Use Appropriate Data Architectures
- **Microservices**: Consider using microservices to separate database interactions based on functionality, improving maintainability and performance.
- **NoSQL vs. SQL**: Select the right database architecture (SQL for complex queries and structured data, NoSQL for flexible schemas and high throughput) based on your application's needs.

By following these guidelines, you can create memory-safe, high-performance database code that is efficient, maintainable, and secure. Each application may have unique requirements, so continuously evaluate and adapt your practices to fit your specific use case.

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
    ✓ Generated embedding: 1536 dimensions in 0.517s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.612s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.496s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.423s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.539s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.610s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.367s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.550s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.510s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.637s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.480s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.421s
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
    ✓ Generated embedding: 1536 dimensions in 0.463s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.068s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.180] From conv_database_design
     What is database normalization?

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe and high-performance database code requires a combination of best practices in programming, database design, and query optimization. Here are key considerations to keep in mind:

### 1. **Choose the Right Database Type**
   - **SQL Databases**: Great for structured data with complex relationships. Ensure you understand normalization and create efficient schemas.
   - **NoSQL Databases**: Suitable for unstructured data or where flexibility and horizontal scaling are needed. Choose based on use case—e.g., document stores, key-value stores.

### 2. **Implement Database Normalization**
   - Normalize your database schema to eliminate redundancy. This reduces storage requirements and improves data integrity.
   - Be aware of the trade-offs, as over-normalization may lead to complex joins that can impact performance.

### 3. **Use Prepared Statements**
   - Prepared statements prevent SQL injection attacks and increase efficiency by allowing the database to reuse execution plans.
   - Example in many languages: 
     ```sql
     PREPARE stmt FROM 'SELECT * FROM users WHERE id = ?';
     EXECUTE stmt USING @userId;
     ```

### 4. **Optimize Queries**
   - Use **indexes** wisely to speed up query performance, but avoid over-indexing.
   - Write efficient queries by:
     - Selecting only necessary fields.
     - Using proper filtering conditions.
     - Avoiding SELECT *.

### 5. **Batch Operations**
   - Use batch operations for inserts and updates to reduce the number of database calls. This minimizes the overhead associated with multiple trips to the database.

### 6. **Connection Pooling**
   - Implement connection pooling to manage database connections efficiently. This reduces the overhead of establishing connections frequently.

### 7. **Avoid Memory Leaks**
   - Ensure that database connections, cursors, and result sets are properly closed and cleaned up after use.
   - Use languages or frameworks that provide memory management features (e.g., garbage collection).

### 8. **Transaction Management**
   - Use transactions to ensure data integrity during multiple related operations. Be aware of transaction isolation levels (e.g., read committed, repeatable read) to balance consistency and performance.

### 9. **Caching**
   - Implement caching strategies (e.g., in-memory caching) to reduce database load for frequently accessed data, thus improving response times.

### 10. **Asynchronous Operations**
   - Where applicable, use asynchronous database operations to avoid blocking the main application thread, allowing for better performance and responsiveness.

### 11. **Regular Optimization and Maintenance**
   - Regularly analyze and optimize database performance using tools and techniques (e.g., `EXPLAIN` for SQL queries).
   - Monitor and tune database parameters based on workload.

### 12. **Profiling and Logging**
   - Profile database operations to identify bottlenecks.
   - Maintain logs to track performance issues and analyze query patterns.

By following these guidelines, you can write memory-safe, high-performance database code that reduces errors, optimizes resource usage, and provides a better overall user experience.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.602s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3202 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.383s
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
  ✓ Text search completed in 0.045s
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
    ✓ Generated embedding: 1536 dimensions in 0.397s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.474s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.505s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.466s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.516s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.584s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.492s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.595s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.548s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.530s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.388s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.330s
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
    ✓ Generated embedding: 1536 dimensions in 0.349s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.049s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.180] From conv_database_design
     What is database normalization?

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several best practices and considerations across multiple areas, such as language features, database interactions, and system architecture. Here are key strategies to achieve this:

### 1. **Use a Memory-Safe Programming Language**
   - Consider using programming languages that offer strong memory safety guarantees, such as Rust or Go. These languages have built-in protections against common errors like null pointer dereferences, buffer overflows, and data races.

### 2. **Leverage Strong Typing and Compile-Time Checks**
   - Utilize strong typing to enforce correct data types, reducing runtime errors. Languages that support generics can help create reusable and type-safe database access patterns.

### 3. **Use Object-Relational Mapping (ORM) Libraries**
   - ORMs can abstract away low-level database interactions and help manage memory more safely by handling object lifecycles and conversions. Always choose well-maintained and efficient ORM libraries.

### 4. **Optimize Queries and Use Prepared Statements**
   - Use prepared statements to prevent SQL injection attacks and improve performance by caching the query plans. This also helps in managing memory allocations efficiently during query execution.
   - Optimize your SQL queries to reduce execution time and resource consumption. This can involve indexing, avoiding complex joins when possible, and selecting only necessary columns.

### 5. **Connection Pooling**
   - Implement connection pooling to reuse existing connections instead of creating new ones for each request. This reduces overhead, lowers latency, and improves memory usage.

### 6. **Manage Transactions Carefully**
   - Use transactions to ensure that a series of operations on the database are completed successfully. This avoids partial updates and maintains data integrity.
   - Be mindful of transaction scopes to avoid holding onto resources longer than necessary, thus preventing memory leaks.

### 7. **Monitor Resource Usage**
   - Implement monitoring to track memory usage, cache hits/misses, and database performance metrics. Tools like APM (Application Performance Management) solutions can help you visualize how your database interactions affect overall performance.

### 8. **Indexing Strategy**
   - Create indexes on frequently queried fields to speed up read operations. However, be cautious, as too many indexes can lead to increased memory usage and slow down write operations.

### 9. **Batch Operations**
   - When performing multiple insertions or updates, use batch operations to minimize round trips to the database. This can significantly enhance performance and reduce memory overhead by consolidating operations.

### 10. **Use Efficient Data Structures**
   - Choose meaningful data structures that fit your use case. For example, in-memory caches can improve read performance, but ensure that they are efficiently managed and that their size is bounded to prevent excessive memory usage.

### 11. **Cache Results Where Appropriate**
   - Use caching strategies to store results of frequently accessed queries, reducing the load on the database and speeding up response times. Ensure your caching layer is thread-safe and optimized for rapid retrieval.

### 12. **Cleanup and Resource Management**
   - Make sure to close database connections and clean up resources promptly after use. Utilize RAII (Resource Acquisition Is Initialization) principles to automatically manage lifecycles of resources.

### 13. **Unit Testing and Profiling**
   - Implement extensive unit testing to catch potential memory and performance issues early. Use profiling tools to identify bottlenecks in your code, and optimize accordingly.

By integrating these practices into your development workflow, you can create database interactions that are both memory-safe and performant, ultimately leading to a more reliable and efficient application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.580s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3955 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.540s
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
  ✓ Text search completed in 0.066s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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
    ✓ Generated embedding: 1536 dimensions in 0.454s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.348s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.126s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.569s
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
    ✓ Generated embedding: 1536 dimensions in 0.505s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.491s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.486s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.433s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.585s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.521s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.814s
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
    ✓ Generated embedding: 1536 dimensions in 0.579s
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
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     What is database normalization?

  3. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.618s
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
  ✓ Text search completed in 0.044s

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

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (lmtknDu2WEmiLEmAP0aJsXCTlQqdhk9LSugFiUJrPCXj8XfOVl2XiJkM9DZJ1_Ws32l4V5mvJysZev7nxwsk1g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "address": {
          "zipcode": "92998-3874",
          "street": "Kulas Light",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "city": "Gwenborough"
        },
        "id": 1,
        "username": "Bret",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
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
Response time: 40ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "address": {
          "zipcode": "92998-3874",
          "street": "Kulas Light",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "city": "Gwenborough"
        },
        "id": 1,
        "username": "Bret",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (r-XiZg4luRQUDOUNONekCHVP-d0eDUNHu6JMMOtsunq93l75ZIh3Z_1k_hsnBqAbGbZXFGEybLfP3fTgmZ1gTA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "stock": 99,
        "discountPercentage": 10.48,
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "returnPolicy": "No return policy",
        "meta": {
          "createdAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "availabilityStatus": "In Stock",
        "warrantyInformation": "1 week warranty",
        "weight": 4,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "rating": 2.56,
        "reviews": [
          {
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 3,
            "comment": "Would not recommend!",
            "reviewerName": "Eleanor Collins",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "comment": "Very satisfied!",
            "rating": 4,
            "reviewerName": "Lucas Gordon",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "lucas.gordon@x.dummyjson.com"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 5,
            "reviewerName": "Eleanor Collins"
          }
        ],
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "minimumOrderQuantity": 48,
        "id": 1,
        "price": 9.99,
        "sku": "BEA-ESS-ESS-001",
        "shippingInformation": "Ships in 3-5 business days",
        "tags": [
          "beauty",
          "mascara"
        ],
        "title": "Essence Mascara Lash Princess",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ]
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
✓ Edge cache script created: SQNoJS7xCJzUWctYtvtSbIEJzpfXmycAfKjPNziTqRD85rgrLNcsfweNzN__hlsyj-w1ovXWy-H6QdXSVR6Zuw

Call 1: Cache miss (fetches from API)
Response time: 156ms
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "website": "hildegard.org",
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "username": "Bret",
        "email": "Sincere@april.biz",
        "id": 1
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
Response time: 42ms (3.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "website": "hildegard.org",
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "username": "Bret",
        "email": "Sincere@april.biz",
        "id": 1
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.15.2
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (fkYHyIXh6DqhQP0kfsn-SJJvNaIZK46dZhJdoCea88Uratuz3CZ01O1mP0s-qffLjJLS7wqo3K9HsfsHZSEy3g)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 179ms
Result: [
  {
    "cached_at": "1775227315",
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
        "followers": 294652,
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
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 45ms (4.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (pJPNuFYTq4Qkt6k3TOUdRy9mjFLdN4KV7j096M0Igpj2kOwJJXMvgfcERYTxBh_CHPWYxktnP-r_Fu7HY5y2aw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1775227315",
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
    "id": "1"
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
✓ Created SWR script: fetch_github_user (aI6nL8JpfhOjHG3J5Ti0vx9a8NfL6kRoL9zI1jC8qXDoPSiZ1dxrO2cRR6VjR2EQkMpbayytfId67wWAXKfy3g)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 38.904417ms
Result: [
  {
    "cached_at": "1775227315",
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
        "followers": 294652,
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
    }
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 40.96125ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (8DuA5NLzGUcAIpqlqVu7UeMcsQlnUG88c2ZQoNZUqxodR8rCIyp3hMLSVCVRz0XEc580C3tM4RUb8x1T_2JbWw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
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
    }
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (aaC7LxuvcS_Sqg5UP-ws8OWOybEiAaFCNBl8D7SOjoRJ1n7vWLfY4eVLLoFOope9SFDYfcgYT1AdOF9hx327TA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 39ms
Result: {
  "data": {
    "value": {
      "email": null,
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "gravatar_id": "",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "twitter_username": null,
      "hireable": null,
      "user_view_type": "public",
      "html_url": "https://github.com/torvalds",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "site_admin": false,
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "public_gists": 1,
      "followers": 294652,
      "name": "Linus Torvalds",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "url": "https://api.github.com/users/torvalds",
      "updated_at": "2026-01-13T07:10:05Z",
      "login": "torvalds",
      "company": "Linux Foundation",
      "location": "Portland, OR",
      "bio": null,
      "created_at": "2011-09-03T15:26:22Z",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "public_repos": 11,
      "following": 0,
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "id": 1024025,
      "blog": "",
      "type": "User",
      "received_events_url": "https://api.github.com/users/torvalds/received_events"
    },
    "type": "Object"
  },
  "cached_at": "1775227315"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 41ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (uLsfpyHD1oR-Flvt3xbiMn1MNXWamuRpFR2d_2s5Fln6WLTmGxSTZMzh9Scngb9iwbSf67OZFe001osV3JXSaQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "shippingInformation": "Ships in 3-5 business days",
      "category": "beauty",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "sku": "BEA-ESS-ESS-001",
      "id": 1,
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "discountPercentage": 10.48,
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "price": 9.99,
      "weight": 4,
      "warrantyInformation": "1 week warranty",
      "stock": 99,
      "meta": {
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "barcode": "5784719087687",
        "updatedAt": "2025-04-30T09:41:02.053Z"
      },
      "title": "Essence Mascara Lash Princess",
      "brand": "Essence",
      "dimensions": {
        "height": 13.08,
        "width": 15.14,
        "depth": 22.99
      },
      "tags": [
        "beauty",
        "mascara"
      ],
      "returnPolicy": "No return policy",
      "rating": 2.56,
      "minimumOrderQuantity": 48,
      "availabilityStatus": "In Stock",
      "reviews": [
        {
          "comment": "Would not recommend!",
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 3,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        },
        {
          "reviewerName": "Lucas Gordon",
          "comment": "Very satisfied!",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 4
        },
        {
          "rating": 5,
          "comment": "Highly impressed!",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "reviewerName": "Eleanor Collins"
        }
      ]
    }
  }
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
✓ Created SWR script: fetch_github_user (Zz5RJAZyb58kGzIL-NV-ZnkX4AZpKpIwFxKLq_Gllc8cvs8EQQT3Mt4z-q-ohS6ekixs2FXqdFxrpyDhu2ZAZA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 54ms
Result: [{"cached_at":"1775227315","data":{"type":"Object","value":{"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","hireable":null,"url":"https://api.github.com/users/torvalds","name":"Linus Torvalds","followers":294652,"id":1024025,"bio":null,"events_url":"https://api.github.com/users/torvalds/events{/privacy}","following":0,"user_view_type":"public","created_at":"2011-09-03T15:26:22Z","public_gists":1,"company":"Linux Foundation","site_admin":false,"organizations_url":"https://api.github.com/users/torvalds/orgs","node_id":"MDQ6VXNlcjEwMjQwMjU=","following_url":"https://api.github.com/users/torvalds/following{/other_user}","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","email":null,"gravatar_id":"","login":"torvalds","received_events_url":"https://api.github.com/users/torvalds/received_events","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","location":"Portland, OR","html_url":"https://github.com/torvalds","followers_url":"https://api.github.com/users/torvalds/followers","public_repos":11,"repos_url":"https://api.github.com/users/torvalds/repos","type":"User","blog":"","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","twitter_username":null,"updated_at":"2026-01-13T07:10:05Z"}}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 44ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (s3vSwUhGnBXCuoKn95yqznE_TfxQIFt_uXJ8kLjwT7xZDo1LHNle9y39AxJ-7kAikvml5FGGC5Q5Taxa2zFRRw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"value":{"meta":{"qrCode":"https://cdn.dummyjson.com/public/qr-code.png","barcode":"5784719087687","updatedAt":"2025-04-30T09:41:02.053Z","createdAt":"2025-04-30T09:41:02.053Z"},"minimumOrderQuantity":48,"reviews":[{"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","rating":3,"comment":"Would not recommend!"},{"reviewerEmail":"lucas.gordon@x.dummyjson.com","comment":"Very satisfied!","reviewerName":"Lucas Gordon","rating":4,"date":"2025-04-30T09:41:02.053Z"},{"reviewerName":"Eleanor Collins","date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","rating":5,"comment":"Highly impressed!"}],"shippingInformation":"Ships in 3-5 business days","rating":2.56,"stock":99,"returnPolicy":"No return policy","id":1,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","brand":"Essence","availabilityStatus":"In Stock","sku":"BEA-ESS-ESS-001","warrantyInformation":"1 week warranty","category":"beauty","dimensions":{"depth":22.99,"width":15.14,"height":13.08},"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","weight":4,"title":"Essence Mascara Lash Princess","tags":["beauty","mascara"],"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"discountPercentage":10.48,"price":9.99},"type":"Object"}}]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   ⏱️  Duration: 154.926709ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "suite": "Apt. 556",
      "geo": {
        "lat": "-37.3159",
 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 40.566833ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "username": "Bret",
    "phone": "1-770-736-8031 x56442",
    "name": "Leanne Graham",
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layere...
   🚀 Cache speedup: 3.9x faster!

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

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "street": "Kulas Light",
      "zipcode": "92998-3874",
   ...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "street": "Kulas Light",
      "zipcode": "92998-3874",
   ...
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.17s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.2
    Uninstalling ekodb_client-0.15.2:
      Successfully uninstalled ekodb_client-0.15.2
Successfully installed ekodb-client-0.15.2
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 38.9ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 39.2ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 46.560958ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...

Second call (cache hit - from cache):
   ⏱️  Duration: 40.886042ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 44ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "street": "Kulas Light",
      "zipcode": "92998-3874",
   ...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "street": "Kulas Light",
      "zipcode": "92998-3874",
   ...
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


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
📡 [36mWebSocket Subscription Tests[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mThese tests verify real-time WebSocket subscriptions by:[0m
  1. Authenticating and connecting via WebSocket
  2. Subscribing to a collection
  3. Inserting records via REST to trigger notifications
  4. Verifying MutationNotification push messages arrive
  5. Unsubscribing and cleaning up

🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m

🦀 [33mRust WebSocket Subscription Test...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_f43e6059c44b49c8a99f5807799eafc9)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "v5cyjtux4wvA2QE8fHTr_o4aGLvHZnxDrYw6mmcLtvAS2bKE7PJjoWadwD0y0gR9lu1u77N_5bfZVFb5oq6oZw"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["v5cyjtux4wvA2QE8fHTr_o4aGLvHZnxDrYw6mmcLtvAS2bKE7PJjoWadwD0y0gR9lu1u77N_5bfZVFb5oq6oZw"]
     Timestamp:  "2026-04-03T14:42:12.817648+00:00"

Inserting record 2...
✓ Inserted: "xhW-wQr-HQ9Ms5z32oUFcUiN8RkkX0HoOj3KQIRUEYrpZpl4cuBJd9eZHvaqMOJ3hJau96vehlzxK6xkKGWPDQ"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["xhW-wQr-HQ9Ms5z32oUFcUiN8RkkX0HoOj3KQIRUEYrpZpl4cuBJd9eZHvaqMOJ3hJau96vehlzxK6xkKGWPDQ"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mRust subscription test complete![0m

🔷 [33mGo WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_d7b957863f454c57a02f3ccec7a6495d)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: T1UhMpxQFHS64l6eH-EhTFFKiYpar-s8tA8WhfaUPrRU3B1DHpf99T87twlNXmK8XjZ6uZNMkCjb1ZbqPmxd_w

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [T1UhMpxQFHS64l6eH-EhTFFKiYpar-s8tA8WhfaUPrRU3B1DHpf99T87twlNXmK8XjZ6uZNMkCjb1ZbqPmxd_w]
     Timestamp:  2026-04-03T14:42:13.098329+00:00

Inserting record 2...
✓ Inserted: vHPvkJ_6ozY7sDW0By91nXjH07R1n4AjelbRLRM0-731ElKfiH2OTqd1JNeZjQg6SDWQG4QJoQqm0kp2wkkZ7A

  📡 Notification received:
     Event:      insert
     Record IDs: [vHPvkJ_6ozY7sDW0By91nXjH07R1n4AjelbRLRM0-731ElKfiH2OTqd1JNeZjQg6SDWQG4QJoQqm0kp2wkkZ7A]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mGo subscription test complete![0m

🐍 [33mPython WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_cc934cc593bf474fa759e3f182153107)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: rUySA1XYyKuWyrk5TCmkGcLrXnTfHnkkiYQf09t0pc6bin0vuq3OWGcNxG8fHxgMDYCG4v6OvQh53mj6zWbLNQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: rUySA1XYyKuWyrk5TCmkGcLrXnTfHnkkiYQf09t0pc6bin0vuq3OWGcNxG8fHxgMDYCG4v6OvQh53mj6zWbLNQ
     Timestamp:  2026-04-03T14:42:13.251219+00:00

Inserting record 2...
✓ Inserted: H0iUQJ9uF5Mnq5XGJ4keFFeIh0yukex1iV0UsuR2OpN22_yJ_3K-2wf_lizSStNV5Z73K0blGDSnrRzdkyfLAw

  📡 Notification received:
     Event:      insert
     Record IDs: H0iUQJ9uF5Mnq5XGJ4keFFeIh0yukex1iV0UsuR2OpN22_yJ_3K-2wf_lizSStNV5Z73K0blGDSnrRzdkyfLAw

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

✓ WebSocket subscription example completed successfully
✅ [32mPython subscription test complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m

📘 [33mTypeScript WebSocket Subscription Test...[0m
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_65faa94ac9224115913cb54e2c451da6)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: V26UmJBxjcENN2yMrT6FG3B77EyyTne_SgMowUJJjILLl5f2IfQQkoU4wbQgUU84D8N7sffm6e-GJR_xlqWGCw
     Timestamp:  2026-04-03T14:42:15.802317+00:00
     Records:    {"active":true,"id":"V26UmJBxjcENN2yMrT6FG3B77EyyTne_SgMowUJJjILLl5f2IfQQkoU4wbQgUU84D8N7sffm6e-GJR_...
✓ Inserted record: V26UmJBxjcENN2yMrT6FG3B77EyyTne_SgMowUJJjILLl5f2IfQQkoU4wbQgUU84D8N7sffm6e-GJR_xlqWGCw

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: OcD8_jsFc4_wQM1ShOGUo8eAyG0v2jK8S7rEeS4i-gqIGnTzt6VwIIGZrlSYYrSgHjSpm2Gxm4IJNC4qV9ouoQ
     Timestamp:  2026-04-03T14:42:16.826292+00:00
     Records:    {"active":true,"id":"OcD8_jsFc4_wQM1ShOGUo8eAyG0v2jK8S7rEeS4i-gqIGnTzt6VwIIGZrlSYYrSgHjSpm2Gxm4IJNC4...

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✅ [32mTypeScript subscription test complete![0m

🟣 [33mKotlin WebSocket Subscription Test...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== WebSocket Subscription Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_kt' ===
✓ Subscribed (subscription_id: sub_cdf973853379475aa6178823a8660f08)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "gCM3w4CfX3NQ0Qbj0hXMfleQuWSDVgWcUt8h8deoPu9NfrPuA01nf9LywCHotqKulDN0dgLe3LSllHt822SStg"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["gCM3w4CfX3NQ0Qbj0hXMfleQuWSDVgWcUt8h8deoPu9NfrPuA01nf9LywCHotqKulDN0dgLe3LSllHt822SStg"]
     Timestamp:  "2026-04-03T14:42:21.845991+00:00"

Inserting record 2...
✓ Inserted: "mZNRfXJNNBSomTA_T8PLLXmm5cP9FeV4J5uJE28saSaB_M80ih4twB7iwt3jXIfRQFdD7KO-N2abSHTdfWuMkA"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["mZNRfXJNNBSomTA_T8PLLXmm5cP9FeV4J5uJE28saSaB_M80ih4twB7iwt3jXIfRQFdD7KO-N2abSHTdfWuMkA"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin subscription test complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll WebSocket Subscription Tests Passed![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
