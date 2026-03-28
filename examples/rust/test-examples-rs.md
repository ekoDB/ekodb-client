make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("-kDZ65_XCkhVQZxWFb0xaJSQSUw_gsks-OgY5fsnuRWeuU9U63xgklI-W1YfQG2wV2S_b6ssGKrw1TDwM1sxUQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("-kDZ65_XCkhVQZxWFb0xaJSQSUw_gsks-OgY5fsnuRWeuU9U63xgklI-W1YfQG2wV2S_b6ssGKrw1TDwM1sxUQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("-kDZ65_XCkhVQZxWFb0xaJSQSUw_gsks-OgY5fsnuRWeuU9U63xgklI-W1YfQG2wV2S_b6ssGKrw1TDwM1sxUQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("-kDZ65_XCkhVQZxWFb0xaJSQSUw_gsks-OgY5fsnuRWeuU9U63xgklI-W1YfQG2wV2S_b6ssGKrw1TDwM1sxUQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "aKdQ63M13kgIZbKZYZYTPFYm5-7xdsO5vUT2KRNbXKxwYoqBTDCGw4lYYqVpoGBpk8sWjYSKmgNryV2xruD46Q"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774661270678325000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "aKdQ63M13kgIZbKZYZYTPFYm5-7xdsO5vUT2KRNbXKxwYoqBTDCGw4lYYqVpoGBpk8sWjYSKmgNryV2xruD46Q",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "53EiSfMcMJ0IjamiIqM5rOTNJnt_WlWDDJL3amgDVUvXzoJjg69Qab6gFYhCwu4HLpQkgySFIj-J224oi1CJrQ"

=== List Collections ===
Total collections: 5
Sample collections: ["batch_users", "websocket_test", "demo_collection", "test_collection", "audit__ek0_testing"]

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
✓ Inserted document: "4MbfFhNFArPqJss1uzq90IsYqd3ULmnNTBS_qrv4-B8KUd3gFNFgLuOXDVybpsi_B0-XqregO9W3VrLJSXF7Jg"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "fDZupKD6ZqlPfN_V0AxhETDtMaL8Xz24hKHMrhEfIKFgvkP_ABX7dx3lNoPSSnN4xqP7fLoS0niN167gLnPnhQ"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "lLn62dk8hjfcxmxkTe5KiSxi-53G6XSaKJXSwxgY7ltn6PVVqefPcMduuXCsB5Z4ZYK5_NGCf389b9wfDDBOCQ"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("RJr3PnDl0HCq9WR6NTvZovXvJr48PLH27OWMlHhvl_PyG3fmSnQO4PYejsAI-ty5jtoYBlVgEL6jIM25FEYIGQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "RJr3PnDl0HCq9WR6NTvZovXvJr48PLH27OWMlHhvl_PyG3fmSnQO4PYejsAI-ty5jtoYBlVgEL6jIM25FEYIGQ"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-03-28T02:28:18.592454Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: VweI33zm2uG8Z0CLnun2_0Nr7B1cnQWvzCBUMGgEhTFqHCtXQ8wdYBCq-Vvzyttdf4I-AvP2qmAEuWrns8tAVg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "l0BAmDtxj84EAiBpgzRMjQxcEocV-E0IzagLZg35rF3F7oGGf6oR_t3qElOs8MvY2ar4-Bt4EUtOx410gRrFXA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: jwqguEl53dXSQgYYNgiHErZt8QfR3i7dejxKNLv8y7BGRygQH_j5ILkUwzRU3g_fzEC4B89XciPvQzyNVgBO3w
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: -K5TOIIUWUR21jnjwtXRXmyhe0auBrTXe7t1BTRriOvZmQ_RgsTgwXJdtMMujHU8UwYEwTgOWx4RrIgomV8b6w
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: p4bzYvdMZVQVPJlfLHEy3CcM476RlpN8No0Wzpb7lFM7nUNlayfnSQnbwg4TtpessoBr9V-6YTp95E8B9ydbIg
Created Bob: $500 - ID: XGVHToHal6njWGkEXNvRV2ET_EVO0knFGGtRGEl2aMyb1VPaTfsOZOEF2Tbu3UaRvolJ2xhCUJbkkSGdnm2L4g

=== Example 1: Begin Transaction ===
Transaction ID: ebc5306e-5b87-4ec4-9db2-faf131fe262e

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
New transaction: b61831a4-e0f3-40b8-8dcf-f23f392153f7
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("Ijccj1K1oxN1URvWGPqa0PEuB-z5wchM0GdCzyTN23BxPJFU00LqTHaezVf1h0COICQHfVuU3MrVm3uxB9gHZA")})

=== Find by ID ===
Found: Record({"data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"value": String("Test Record"), "type": String("String")}), "id": String("Ijccj1K1oxN1URvWGPqa0PEuB-z5wchM0GdCzyTN23BxPJFU00LqTHaezVf1h0COICQHfVuU3MrVm3uxB9gHZA"), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-03-28T01:28:22.199782Z")}), "price": Object({"type": String("Float"), "value": Float(99.99)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-03-28T01:28:22.199782Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "created_at": Object({"value": String("2026-03-28T01:28:22.199782Z"), "type": String("DateTime")}), "id": String("Ijccj1K1oxN1URvWGPqa0PEuB-z5wchM0GdCzyTN23BxPJFU00LqTHaezVf1h0COICQHfVuU3MrVm3uxB9gHZA"), "name": Object({"type": String("String"), "value": String("Test Record")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")})})]

=== Update Document ===
Updated: Record({"data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "created_at": Object({"value": String("2026-03-28T01:28:22.199782Z"), "type": String("DateTime")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("Ijccj1K1oxN1URvWGPqa0PEuB-z5wchM0GdCzyTN23BxPJFU00LqTHaezVf1h0COICQHfVuU3MrVm3uxB9gHZA"), "price": Object({"value": Float(99.99), "type": String("Float")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Mww81qwUoO9nrk9TBWwuOwB79lhwVnxHbeKo2587JtTBZKjjOPsNKBU8ivokOVjypZzgZfc6s1-hf1BytxZ5YQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  value: 42
  active: true
  name: "WebSocket Test Record"
  id: "Mww81qwUoO9nrk9TBWwuOwB79lhwVnxHbeKo2587JtTBZKjjOPsNKBU8ivokOVjypZzgZfc6s1-hf1BytxZ5YQ"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "X_IN0LpftlJ01esFuIJC82jkP1qCwqJKeE4yE9Ik4bBsULv8rDjoluFdXQ9exiGwLU8KGgRqkwWFscPkYxZhgg"

=== List Collections ===
Total collections: 10
Sample collections: ["client_collection_management_rust", "batch_users", "websocket_test", "ws_ttl_test", "test_accounts"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
Created Alice: $1000 - ID: iB6-6TQ1yXH7o835Ovgqdju-CWm_iO03XoaUFGh1W1ssV4ork9HB2nujUfbG36FHD9F_Vf1vv5aN2r8IUpwY_w
Created Bob: $500 - ID: UBj1MxF_ruOrvSSZn3ZXvNzTdeBN4V1mHHc0HgiliFvKpEiJxRdAG5GCIULf2rmMCEmNJE806zxpKQrt2eO4wQ

=== Example 1: Begin Transaction ===
Transaction ID: 7fd5b0e7-fbb3-40de-95e2-da8f83c1a411

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 51965e74-c1a0-4e25-b714-096d3b7eb30b
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  3. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  3. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["description", "title", "title.value", "description.value"]
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
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("Dgk-2u5lXi3rUuCJtRnnafQXW53W5bOBT3ZZ5pmRniQqRGbvWdmj-_SfWgK3nQwqDeu91kjM5SyxFZabGv26Uw"))
✓ Inserted user 2: Some(String("SRiHEDIxtgyt2ur2prShE_Di6CzGsfWslikfqDitz4mLqAS9ZPGdasSwNkSjX5NNAtQihdCTDUjEA3h8pNFIfA"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - title: String
    (required)
  - age: Integer
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 10
  Sample: ["batch_users", "websocket_test", "ws_ttl_test", "test_accounts", "scripts__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "ieGuPdL7LBLBAfzhPgscQrnwbNgBXNDX_2iHr43jYcSGdYA-2oC-TSUmxCHnNsB5P-2dXGs2XfeG9HJ3lAQHMw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("HA2NUQEZ78kQNxzUKw4skzUMXj-xwqipms7PsU6TOfgYcvJXy29nQTVbJLR5sD58yUqk_xX1TxpmqIxu0S2PCA"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("JvIRNERv_nxFzpObvH7_R7wVDf2PMND9KRYgJL9_eE4bimOSwRl9lOT2IHbrbc_GUS6Bcb9OQOPr0jdlNMq-PA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: MB54BwkEFEUgEydGPtPwLojHUCRRFecEmnjUg3AOtkJW9kHQynsFfpyys2zFS-3DQ4sY1zNkQ2896M-IQ8_cnw

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: OlV5_ilFsp3uND122m660Cqno_o3kBcU_2F8Hx7y5O4iCSgDh0kc-NCVKSuSUKf_vXDJzbhNNETtZnaTbtR-zg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: ZXmlV8eZxjehv3alKrJBD-3khv3MP6qZqob6wT1N6z4900dL3eMuRJBui8vMlvtRQ9RygqO7TH_n6tf80pxx7Q
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: aENQ6wbLWobKZDxrrfNb_Sg2ndAsA8-sQLTKtSDJBM9xSQ7WV07ulaCCaciIZiqYBlMazIjsWXi77R_ihmmUQg
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
   ⏱️  Duration: 199.701584ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-markets"
    },
    "id": 1,
    "email...

Second call (cache hit - from cache):
   ⏱️  Duration: 41.433791ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "name": "Leanne Graham",
    "username": "Bret",
    "website": "hildegard.org",
    "email": "Sincere@april.biz",
    "id": 1,
    "address": ...
   🚀 Cache speedup: 4.9x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 0XiKLqv1I90sGajPt5CMV3_cUQuW5_00iwZbCrnXkus0S0eyUf47nN8CgVoMLzSuEZPA6Z6YXbVryopSgb7w3A
📊 Found 2 product groups
   Record({"category": String("Furniture"), "count": Integer(2), "avg_price": Float(474.0)})
   Record({"count": Integer(3), "avg_price": Float(575.6666666666666), "category": String("Electronics")})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("pHHvf_veT5CLW1Arzd-BZVcsvw9kILwnoSv-F85PYsnnCZZNqDR1Jfk1nTm0i7IZg4G7a-aQI-Yp0u6hW-ve2A"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: h4gVUGbDQXqu_UmMSQRglyPmcXy6Vbw4A0tcdqqlRw5HbzJxXcPssddJUOB6eGW2wBzN4GObV0I6BTk1Hvx2mQ
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: D0YIWWoDQZ-ZMjs4YpkqGfsi2-ptIMdrr50Vhw7jW9njVH6OIVeQUCCQdVfYikkc6e2A9u364kSeVJIewlxOOA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: TPe3OQ025KB0_Ta246kGYv5OhUSxPdG9YCSCjjjV4BJtiJHRy01_R75K2PJ_ZPjokEf_fMFttJKxKcc0wiHDaA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (Rjr1B8Te_fWcOa6xSVwN1DGWbMDPwPxbW8kdnMIban-InazOkMla5bS10RNUvV_ZMdeJOVxwoAANTdwN90cd2w)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 41ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (pn79K-TVUCfiVdsQ6If52iq3a5q9mXcWbCfdf9syKcAgYMJX8L3dBW5XicGVcQvN-8mfubhbKxucQEfswDK4Cw)

First call (cache miss - will fetch from GitHub API):
  Response time: 185ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 10.9x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (z0QNELrRTvzx8Z0IQbYZDZMrVGu7KF3IGBJDYIF8TbCY2c6ae9R_m_cqAs_wxCeGjgH9I0_wUf6CsVUykALboQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (bWTBbDGcpICZj0XbSXcHnOvuSUZXEu6VNf8H4cRyVxcsoDGB43VJiWoIr8M23m3PFzzSky7piTIlyzsG5bdLHw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (yHXBevO6cBY82P1nwyckT_qWy2NK8UhogfEb00htqrSbqS5fi4Ji6cl-txeftSmpV0sz6DW6RmQkTQiAXESKVw)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
   Record({"category": String("Electronics"), "avg_price": Float(367.0), "count": Integer(5)})
   Record({"category": String("Furniture"), "avg_price": Float(365.6666666666667), "count": Integer(3)})
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
   Vector databases offer several benefits, including:

1. **High Dimensional Data Storage**: Efficiently store and manage high-dimensional vectors, which are crucial for applications like machine learning and natural language processing.

2. **Fast Similarity Search**: Provide rapid retrieval of similar items using techniques like nearest neighbor search, enabling quick access to relevant data.

3. **Scalability**: Designed to handle large volumes of data, allowing for seamless scaling as datasets grow.

4. **Flexible Data Handling**: Support various data types, including text, images, and audio, making them suitable for diverse applications.

5. **Improved Search Relevance**: Enable semantic search capabilities by uncovering patterns and relationships beyond keyword matching.

6. **Enhanced Performance**: Optimize query performance through indexing methods tailored for vector data.

7. **Integration with AI Models**: Facilitate the implementation of machine learning models by providing a suitable infrastructure for embedding storage and retrieval.

8. **Real-time Processing**: Allow for real-time analysis and inference, which is critical for applications like recommendation systems and fraud detection.

9. **Support for Distributed Architectures**: Often designed to work in distributed environments, enhancing fault tolerance and load balancing.

Overall, vector databases are pivotal for applications that require efficient handling of complex data relationships and rapid information retrieval.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   Record({"status": String("active"), "count": Integer(7)})
   Record({"status": String("inactive"), "count": Integer(3)})
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
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
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
✓ Created session: j_uBHtijrLDRNENy69rB5f_tVH2i8h8M5OwCFueyR6GMcZTzHG6yN-FTyLFXDv0_XdYoTH_0vN6_HZHSBYrcqg

=== Sending Chat Message ===
Message ID: z1KMK32fXY4hf0IeOYzmDz7pFc_W9wmzS3NQ7nKubyez5JABU2lCYl_acwxMWHWe2UxTpPf_MlZMoS1pIFUjgw

=== AI Response ===
Response 1: ekoDB is a high-performance database known for its intelligent caching, real-time capabilities, and AI integration. It supports various advanced features, including:

1. **AI Chat Integration**: Users can query the database using natural language and receive AI-powered responses with relevant context.
2. **Search Capabilities**: ekoDB offers full-text search, vector search, and hybrid search, which includes automatic context retrieval.

These features provide a robust and flexible environment for managing and retrieving data efficiently.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("vk21ZUDJ0Gq2Y_qoi0WP4ZHlnH3vNgLqzsW2kQcGuN8nFQEIMUlB8iuj5z7TtspwKvmt72obj_SqaDW5kQ_2XA"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("Msa0Lx4EIHx4i3P9Hd14hgwwy6_fLeBEG9xQ6kAhd8AykhKnZTapn_bQr4Jp4t_4aMYyDVDZjxPBtamOwdchMA"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("uAKx8Wn8XjT5PUPRG6elUYJ5L1w5ygUUriW2AckZ4PpQOlbiwp2hiT7HKyLWdKieMm2cRpiTxWcocOBBx_rifA"), "title": String("Search Features")}

Execution Time: 6009ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: aPbQo61nVVU9hEwDaZKjafXlN34siydtdsOcjiCZ1DkMtGVPr6pt9pQH6aWxqnlq01wmRdMg5X8Ei70kjYTb_w

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

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
✓ Created second session: dc_xaMhvbWw5t-t3j5q8wtArZ4_fW84kPvVgOuSLbdULuIoRcaHb8VKeV1OJ5xhXd2ZhIhxLdz4ILldzRqsf8Q
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: s_2yjRosdEte6cS567XdIu9SOHSPa86dimWJmzOYOSTXQRNMWxnUejuwWGRs6wKqgRURPvfQlBU8IcvqS00JBQ
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more information or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: x462sRVOf0zK8LWoXWZKk5DbHr0OSf5vfjUKJpZw6oBcWxLXHmiwrbgluDQg_0rSuVStrOri3_wE0-GFXq6gOQ
  Parent: s_2yjRosdEte6cS567XdIu9SOHSPa86dimWJmzOYOSTXQRNMWxnUejuwWGRs6wKqgRURPvfQlBU8IcvqS00JBQ

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: x462sRVOf0zK8LWoXWZKk5DbHr0OSf5vfjUKJpZw6oBcWxLXHmiwrbgluDQg_0rSuVStrOri3_wE0-GFXq6gOQ (Untitled)
  Session 2: s_2yjRosdEte6cS567XdIu9SOHSPa86dimWJmzOYOSTXQRNMWxnUejuwWGRs6wKqgRURPvfQlBU8IcvqS00JBQ (Untitled)
  Session 3: dc_xaMhvbWw5t-t3j5q8wtArZ4_fW84kPvVgOuSLbdULuIoRcaHb8VKeV1OJ5xhXd2ZhIhxLdz4ILldzRqsf8Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: x462sRVOf0zK8LWoXWZKk5DbHr0OSf5vfjUKJpZw6oBcWxLXHmiwrbgluDQg_0rSuVStrOri3_wE0-GFXq6gOQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
Created user function with ID: hi9hBxyNjLsIwUBfyr-FPyHEep4KsiNG1JZDiiCsqhoAl9x19bUnvo2fEpRT6vOJWwDY7jmrYH4o-9XGPfBsZw

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users: Get Active Users (Updated)
  - get_users_by_status: Get Users By Status
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - get_active_users_rs: Get Active Users
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("K-7oplB3tjXq7pw8TWX6kGCoVPHyoFqie1UrT8pkG5QrXyT2A7kGhd2RjL2DBdh4TT9S-LjiFHRFr_HXQxd8wA")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("Nl1E38lv52LjZGBKWUM4IzMh33QEc9Mxd80L6ND_QAAr7gbjp2X9YMaHIANClOXfRdkZAN7_05x676OXdAJuAg")})
✓ Second upsert (update): Record({"id": String("5borOqqifm082dIYtBf6RQjbR4PJNRZnDj4BnnlGfLfCGJByB2fb_TwAsT9wL7jHMjWUM_Plp27NM7tDD8bELQ")})

=== Find One Operation ===
✓ Found user by email: Record({"email": Object({"type": String("String"), "value": String("alice@example.com")}), "name": Object({"value": String("Alice Johnson"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "id": String("K-7oplB3tjXq7pw8TWX6kGCoVPHyoFqie1UrT8pkG5QrXyT2A7kGhd2RjL2DBdh4TT9S-LjiFHRFr_HXQxd8wA")})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("VZQmcTr61RGOHzOjYirOPB2hC_c_5YW7z7qcNIKXeGCGVoBLPaBO_eGm0EKw2plq-Mcn6drb5bldIfKBG7lQ2A")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("Rb7cGxHmD9Sym1r_igw73gAJ9rZdFXM7ld46JVn7Zk70zz4-HgKd1Sft05GKrkO-G7fRAJE9CEFam8uEohfMog")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("VZQmcTr61RGOHzOjYirOPB2hC_c_5YW7z7qcNIKXeGCGVoBLPaBO_eGm0EKw2plq-Mcn6drb5bldIfKBG7lQ2A"), "price": Object({"value": Integer(150), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Product 1")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("uqyqzE5BgffJnkpcMeTj5uFk6hvfGEAJHU_bhISp2JCArk1n98h3-i9Na5knDkKuDto7epQjD-GF8G2-nWNmlw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name", "id", "email"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["bio", "age", "user_role", "name", "email", "avatar_url", "status", "id", "created_at"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["api_key", "secret_token", "id", "name", "avatar_url", "created_at", "age", "status", "email", "bio", "user_role", "password"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "name", "email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
