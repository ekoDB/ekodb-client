make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("2cXnMAqxO2TuMG41lCEUt_wtGe2Qik1kZkKmDXZXnveLoRtdvw5zbP0UWrrWDmvUaEuHkbnRxLIUkzoCWUd2pA")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("2cXnMAqxO2TuMG41lCEUt_wtGe2Qik1kZkKmDXZXnveLoRtdvw5zbP0UWrrWDmvUaEuHkbnRxLIUkzoCWUd2pA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("2cXnMAqxO2TuMG41lCEUt_wtGe2Qik1kZkKmDXZXnveLoRtdvw5zbP0UWrrWDmvUaEuHkbnRxLIUkzoCWUd2pA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("2cXnMAqxO2TuMG41lCEUt_wtGe2Qik1kZkKmDXZXnveLoRtdvw5zbP0UWrrWDmvUaEuHkbnRxLIUkzoCWUd2pA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "OZdVF92oGdfsvvOSD4yyE45G1-Ue61ATHNGkDBRj_10Z-cgCfY2PomLVC0mMdOXbhG8OuJzrg8y4cBsbOzDxyw"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1782492633060443000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "OZdVF92oGdfsvvOSD4yyE45G1-Ue61ATHNGkDBRj_10Z-cgCfY2PomLVC0mMdOXbhG8OuJzrg8y4cBsbOzDxyw",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "l2TeIqbieHOcK0HyPolynSRX1aUQf4xxSyIsjttYJTvZnp_uX4wWpNBDvYYYY0M_WRPQ41krCPatWpnWj5J38Q"

=== List Collections ===
Total collections: 5
Sample collections: ["test_collection", "demo_collection", "audit__ek0_testing", "websocket_test", "batch_users"]

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
✓ Inserted document: "SLowaV8xkKt4dIEscBGVb4QJ9o4DsyCAdi1YS41V5I1WfnJj_a7EFrHPY9lce16fXnZaP6Lng8i6hxIm2asEsA"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "pzAofAhhTEmQx1Sck3ONRUzTo2AivvPacR4OjuD97XPAJu8Yzm2p-UA1mpqj2_YXyfoKZRy1kwRTkB3nhCEQ5Q"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "xMWCn42UM0_DTgDirSKtmbjnwZ9i-wkypTtQw27YI1q9rWZ9Iajuf2t51IHYA1BLGbYULwTO7HlQy5JRlN0I-g"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("Lu91l1gZgBXtDBFcwqqS2Y1uOeXsyHk-iltutkQjPhoRRBqyNeGc9RZBfPvHFBuKzKwoP3oiv4VLacWTWcU5sA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "Lu91l1gZgBXtDBFcwqqS2Y1uOeXsyHk-iltutkQjPhoRRBqyNeGc9RZBfPvHFBuKzKwoP3oiv4VLacWTWcU5sA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-06-26T17:50:34.262916Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: LkuqvItPR6xmPSqoqKrJjwOTXweAOL34hAwsMRrOpTVVW9yzIztr36OVa5RJrCb1oiAHzhYTlQUOO0GhaDXARA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: AxQuoeQrvFAUqgudJgbi1H84Doq1LHy7fvM4xf0cPVTldMACD6nLu-MMGnhRm6gKNBcOhTuR7I16KucxmJgqYA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: q6svnVTyFgZs5GCHZw5iYidm8SBIn2OOcqyKHv1RjTK-KSqfNwwpqnsJviIt6ipT6X-fTeWGsBmv8zZsF2RrsA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: XIJSxeA-Ved8bQPrnLg1kI8-FY8XJCBp7Iga1I1tKuiYQ1zmvTh7Enf68vEqmCJd609oNNIjbJZdDmwPYuhD-Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}
   {"avg_score":{"type":"Float","value":50.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: NOsYMTX0pBKXfvoNNGw42obSbqS03XoR8JZGoWWrDTH9FY4NhFHxTPiYiTLcFh5Bwc5qrymx4iBJ60g9ig8xdA
Created Bob: $500 - ID: K4fyptIH_ZmnrJjiQErbRBVDckLVOSQYGLKQuV9rHlLfGILHZmt98hx-Nmleeal0ZvlmJb28ZELXn61qy0g5gw

=== Example 1: Begin Transaction ===
Transaction ID: f0219ef8-820f-4d61-8297-1cda0797f39a

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
New transaction: c376dab3-d5bd-4138-8209-dce4df66559e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("19mNayM34LFKLY-n9GaSs6x1-5grlQeConqrZuIvgEeB6T5Sf0qHdi0b3NqU-flhGZG2d6pi5TJDVRFiIe9QZA")})

=== Find by ID ===
Found: Record({"price": Object({"value": Float(99.99), "type": String("Float")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "id": String("19mNayM34LFKLY-n9GaSs6x1-5grlQeConqrZuIvgEeB6T5Sf0qHdi0b3NqU-flhGZG2d6pi5TJDVRFiIe9QZA"), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "created_at": Object({"value": String("2026-06-26T16:50:35.304601+00:00"), "type": String("DateTime")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-06-26T16:50:35.304601+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"name": Object({"type": String("String"), "value": String("Test Record")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "created_at": Object({"value": String("2026-06-26T16:50:35.304601+00:00"), "type": String("DateTime")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "id": String("19mNayM34LFKLY-n9GaSs6x1-5grlQeConqrZuIvgEeB6T5Sf0qHdi0b3NqU-flhGZG2d6pi5TJDVRFiIe9QZA")})]

=== Update Document ===
Updated: Record({"embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-06-26T16:50:35.304601+00:00")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("19mNayM34LFKLY-n9GaSs6x1-5grlQeConqrZuIvgEeB6T5Sf0qHdi0b3NqU-flhGZG2d6pi5TJDVRFiIe9QZA"), "name": Object({"value": String("Updated Record"), "type": String("String")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "price": Object({"type": String("Float"), "value": Float(99.99)})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: VinBGYhFht3Z5G8nsRc7KYEpHMkbpdEMdV1kkXkU6JTRAcYKqbVJTSKQQlnr6R4YSmSnq_8XOrGfpdGs3Azrnw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  id: "VinBGYhFht3Z5G8nsRc7KYEpHMkbpdEMdV1kkXkU6JTRAcYKqbVJTSKQQlnr6R4YSmSnq_8XOrGfpdGs3Azrnw"
  value: 42
  name: "WebSocket Test Record"

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
Collection created with first record: "f0hP9EC07kwk9-0hK5Mt-UUHuwiLvJksQsAWfKYvtcHebrIWYYoBEmZGi7ZUp1Lh8wgbAJWGTEWdxObniZF3eA"

=== List Collections ===
Total collections: 11
Sample collections: ["test_collection", "functions__ek0_testing", "ttl_cache", "audit__ek0_testing", "agent_function_versions__ek0_testing"]

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
  cache:product:1: Record({"name": String("Product 1"), "price": Float(29.99)})
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
Created Alice: $1000 - ID: q4FXqIHGYORISSkbcdicaoXfcVpd6PnCam9nVGF257kg9MjrxzyaDW1NcGKk5XdQeck9pp0diVtwGBGTCPcHTQ
Created Bob: $500 - ID: NdeQkNDY_Wl83eUfENHCROy2aivmk0qctJIAxzoYCLybWCrWkBUFxF185vcrkS5-b0uJQ1s_KAxbI7CLWhV3TQ

=== Example 1: Begin Transaction ===
Transaction ID: 401e97b1-e19d-45f3-947a-809f45cea458

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: bda137af-5c84-4e16-8a28-ff61700739b8
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

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
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 2.2000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["title", "description"]
  2. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description"]
  3. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description"]
  4. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  2. Score: 3.3000 - Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Database Design")})

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Search with a metadata pre-filter (category = programming) ===
✓ Found 2 results in category 'programming' (database/ai excluded)
  1. Some(Object {"type": String("String"), "value": String("Python for Data Science")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))
  2. Some(Object {"type": String("String"), "value": String("Rust Programming")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("6y0isusr-YK8CHgjud-BzYXTv04cjoB8nzkIUMd-iuQAl3L0_pCnVaYRTsWdUBUlwM4deB2OkiE8j4C_blNB-A"))
✓ Inserted user 2: Some(String("FByhEvehvvMlmKsZ7C2h4zJRgMcp_f81qUmvgnv2noqsOP7-2pMUbr95vXrhlcnSihfo0Sjsv3XqDgfjLj30bw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - email: String
    (required)
  - title: String
    (required)
  - status: String

=== Listing Collections ===
✓ Total collections: 11
  Sample: ["test_collection", "functions__ek0_testing", "ttl_cache", "audit__ek0_testing", "agent_function_versions__ek0_testing"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "kA3-TtlRCQq20s97SorGGRNPom-joPFyOFBq4bq4PQxqO_0a0i4Sfu4IoBsTzm0MB_g1Zuwbyfae8256lRo5dA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("kzbtxYte3jjvxb1ysCER4kw6CAoEOvwa61jMRWgYp26uaSwJMdriYGG-Kcvcg-eZvkYaSyMF8HwIIHZIUYV6dQ"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("g5AvpmRC3zfOJewEUO3kpNM-eEBdwmftbBoHTexyr4MteZG0Asf-rdZssJJqNCtdmzfv9EmVb6am4KZnqHZplg"))

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
✓ Edge cache script created: GKK3Ht45nN7qDirsfbvKIU-ZTVJfKjYXZ3a5W4f3Ya7TufNz1hfLNmYm7Hi96bBB0-N84-quh1hYk0trZZKZZw

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
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: abu7y_64VpKfKHjlk1CvTIMrBz-BZwt8Sd5gAQ4iKskiLX9V2RYYJBdaBe_q8y6yxoMeFwGIQLiYxzynQvPH1g
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: 6TDM-tnGxBVF8NqtI9_X0YN0U7ex4sK0R0oL1FFZ3Sid9JU-9TArmRokqEX-Eq7UFuS64ZUC6bqRodTKCkRVug
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: Zifg1DrlbMtK1i2RIyjtGvwevNyeGY-BfL2hiVxixB-rzNJ0Ngz020B87XI49X1ESuA1AJfWEQZayTfDS145Ng
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 5
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 113.261958ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.298292ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "suite": "Apt. 556",
        "zipcode": "92998-3874",
        "g...
   🚀 Cache speedup: 56.5x faster!

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
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 7Yt1X7azjDE9p6H6ae1f-8wI5UAkmSQA_TO1QKRniea2dsaGxtSu0Wmon1mNSRnPPcAijD4ihTMnNJisEK4wyw
📊 Found 2 product groups
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"type": String("String"), "value": String("Electronics")}), "avg_price": Object({"value": Float(575.6666666666666), "type": String("Float")})})
   Record({"avg_price": Object({"type": String("Float"), "value": Float(474.0)}), "category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"category": Object({"value": String("Electronics"), "type": String("String")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"type": String("Integer"), "value": Integer(2)})})
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("mG4kKV3NBCyonJtbibALei9JprVYaRlwPytJV7aPcBrFWtmb78pt45YonkNCaLr4_Dkvj3zhhIgPfhROjZyjPA"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: Eae4JcHk5O_3lFlRLtWNi-OP7GHa-C0jirXrr9NQb_g3yPSx6EH0XkmJlom-LOnm33Dkg3tP0uVzZuBUN0xMiA
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: e1CcoZsCRsksYaW2khBWQ-TBJ-rBM-ytC6fJUU6hE8k_rr8IgkEkc88YyNRgf_aBITeAaxVm-KlneosqXoPq9A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: U8B95EMZIVV9DU91Uf-0xeDR14LCbhxC5leZMy0bA6SG5TTjw0LeoHsFYxe1r9ZvNNqVosDfWvLcNlBAtOGV6g
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (C3zeFKI8j_2Wn9qRhoINx0KZ6eN2B5N8FGDjoV9oOip5TWBJ8a9Sg0yRQA2jdt7YM43k9w-Xg8EYxoFFUHIiow)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (6ePFEjLfnsmt1Xg3Ngw4cedcGK0ZwmSUKTCLgaNcaJPs-1ThWKp5DjgRKF29h90HpxLiT2gDmj-4Tm9gaFXH4A)

First call (cache miss - will fetch from GitHub API):
  Response time: 106ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 53.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (Klk4f8HtqORmnSafsVw2DocMZUnAMoJ0920Y_ctIItc1lm_856-6Wjc6RKYKsqso1T8KYoF17uqLmNkZkxU2qA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (pAWmOFECB8QoGPXyHETL6OZiKpWAV_adO9stGAqq-jg0UGgbR4E5BCtUELqkvFNU5Ii8CtfGfr-qeXsW7YKDog)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (R7LK7xcqyIDnnkNO3kZ4LvLAjDEHnmtSFvj1X2_PL9Tah1PSx7k6LQ5aYHTTTOeYqfoprNKRgnbiahww18TcPg)
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
🚀 ekoDB Rust Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "avg_price": Object({"type": String("Float"), "value": Float(365.6666666666667)}), "category": Object({"value": String("Furniture"), "type": String("String")})})
   Record({"count": Object({"value": Integer(5), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Electronics")}), "avg_price": Object({"value": Float(367.0), "type": String("Float")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
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
🚀 ekoDB Rust CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   Record({"status": Object({"type": String("String"), "value": String("active")}), "count": Object({"type": String("Integer"), "value": Integer(7)})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "status": Object({"value": String("inactive"), "type": String("String")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("AI")})})
   Record({"category": Object({"type": String("String"), "value": String("Database")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
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
✓ Created session: ieawk8LiucOs5nKI_C_wHeCLjw92fUJnpMz3PFAce2AWqQtREuB4lsOSg9wo8gn3QNFF03_aSZGLtGlGMSasFw

=== Sending Chat Message ===
Message ID: W9iQtgWyGEwvYbzQ-7FUm3IypW5pI8hWMsaqp8fSa2CK2vwUKx9uavggTb8H8LRba9Q4DWQq_gLieunClb-38Q

=== AI Response ===
Response 1: ekoDB is a high-performance database designed to integrate AI capabilities, intelligent caching, and real-time processing. It supports various search functionalities such as full-text search, vector search, and hybrid search, which provide automatic context retrieval for improved query responses.

**Key Features of ekoDB include:**
1. **AI Chat Integration**: Enables querying the database using natural language, allowing for AI-powered responses with relevant context.
2. **Search Features**: Supports multiple search methods that enhance querying capabilities and provide contextual information automatically.
3. **Performance and Caching**: Utilizes intelligent caching for improved performance and efficiency in data handling.

For more detailed information, you can explore the specific features highlighted in the documentation.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("PImTf51SLzx3Eq766DPmoZ1O-1IN1U2-brJo0yY4skchsiT3cSgL5Tc6D9xp4UjUBVjvlETG4x8Zlug-KijaKA"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("7nDuolpTvOY0L2JoR3EtcIqVYw6gUlwf9nWxaGQ4Q0tpk_tGjWHQQgIWbUtgBpBVeKBKCsRhoNaGmKOrXD6RlQ"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("Iy11RkmS5gcAT9FrttfmAg_8AvX3LaHJYzMeVSE8eNJAzwfvyYBDBDsrJSxmxbLdvxEcExdkLMt-2RcGUGdfrQ"), "title": String("Introduction to ekoDB")}

Execution Time: 12652ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 16mWuRCFdRmvect_SoEBqqO-bNpkD5ydPCH9aBbYuhxn5lV-XGCrBmlFbI4ldc8L9SHYfsqdtoerHo_jqaMjXw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have another question, feel free to ask!

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
✓ Created second session: FjhdyrsoiSxi3Wvhn_BjeTxxYLupEHEXgBlIAm75KSmSVRD5jV6FYxG0fBI4F5AEpPH2IIcsnreDVjMJLcTrcQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6iCjiw0uTvLDoU1C2PDmYU20G9zcAOYlEnu-WLoDMnO2zotdLwhU4slQ0dBk3ZXelzYuE0Suy_FNS2QhZ3KUPw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99 

If you need more information or have questions about other products, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3K4oluQ1wZhMl9dxjMWAVHX6edI4Ksy42-zRInRkROvxYA6hThhblx6cVUSWUAvPhzSQJRXRPpJfOIVX3e9fdw
  Parent: 6iCjiw0uTvLDoU1C2PDmYU20G9zcAOYlEnu-WLoDMnO2zotdLwhU4slQ0dBk3ZXelzYuE0Suy_FNS2QhZ3KUPw

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: 3K4oluQ1wZhMl9dxjMWAVHX6edI4Ksy42-zRInRkROvxYA6hThhblx6cVUSWUAvPhzSQJRXRPpJfOIVX3e9fdw (Untitled)
  Session 2: 6iCjiw0uTvLDoU1C2PDmYU20G9zcAOYlEnu-WLoDMnO2zotdLwhU4slQ0dBk3ZXelzYuE0Suy_FNS2QhZ3KUPw (Untitled)
  Session 3: FjhdyrsoiSxi3Wvhn_BjeTxxYLupEHEXgBlIAm75KSmSVRD5jV6FYxG0fBI4F5AEpPH2IIcsnreDVjMJLcTrcQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3K4oluQ1wZhMl9dxjMWAVHX6edI4Ksy42-zRInRkROvxYA6hThhblx6cVUSWUAvPhzSQJRXRPpJfOIVX3e9fdw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (120):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  - gpt-4-0613
  - gpt-4
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
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
  - omni-moderation-latest
  - omni-moderation-2024-09-26
  - o1-2024-12-17
  - o1
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
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - gpt-5.4-mini
  - gpt-image-2
  - gpt-image-2-2026-04-21
  - gpt-5.5
  - gpt-5.5-2026-04-23
  - gpt-5.5-pro
  - gpt-5.5-pro-2026-04-23
  - chat-latest
  - gpt-realtime-translate
  - gpt-realtime-2
  - gpt-realtime-whisper

Anthropic models (9):
  - claude-fable-5
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805

Perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

=== Get Models for Specific Provider ===

openai models (120):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  ... and 115 more

anthropic models (9):
  - claude-fable-5
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
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
Created user function with ID: xeW6x1NHCr9Zj3Qcnnqf_4H61wl2jIeAeZmr4r5v_qNaTnTjGhM_685PLk1elIstfcdvaQ1i1HXncv1hNSHzdQ

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 12 user functions:
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - get_active_users_rs: Get Active Users
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("YgiOWdOKVGsH7ZF-Ss3-VJcL26NFnWxlw4NowGwLFUCluHYQRT7l44eYw6hO0tpStpx1tZgPxTOpuTrLAlHeGg")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("EYK8aAow0A9-ixl9ls-uzEy6_ViTYd8zdbTvlWoeg6LeTrM-sMpUrOCaITtAwDn2IDLX83JfmECbk3z7l1G2oQ")})
✓ Second upsert (update): Record({"id": String("M3lOk9_abEpGX2M_RuGMJelv9W-xC_nD5QQofKqWP8KGOg3WY44PZ5_U0vxGqCKdZBsuJlFQVSzLQIklnPpkRw")})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"type": String("String"), "value": String("Alice Johnson")}), "id": String("YgiOWdOKVGsH7ZF-Ss3-VJcL26NFnWxlw4NowGwLFUCluHYQRT7l44eYw6hO0tpStpx1tZgPxTOpuTrLAlHeGg"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "email": Object({"value": String("alice@example.com"), "type": String("String")}), "age": Object({"type": String("Integer"), "value": Integer(28)})})
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
   Inserted with ripple: Record({"id": String("90JRmacxsVPm7s-Gz7LLHpH1Q_Q8KUP-uui9D1Fsg0fuxDI3FPiAYQuMV1srr8CRLRtFLkcUbcfxRltB3lbBng")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("jJN-srpy9yaGKnZeX8KoFtj5BSS_jxja36pnqddpuNJsbsXnNjST3gEHXgDq7VnVS4m48vQf2-7dnMZvGNihsw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("90JRmacxsVPm7s-Gz7LLHpH1Q_Q8KUP-uui9D1Fsg0fuxDI3FPiAYQuMV1srr8CRLRtFLkcUbcfxRltB3lbBng"), "name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"value": Integer(150), "type": String("Integer")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("_LDKFT1Ch-2MkBIXCdvoVt8qO1BVhw2MK5PenEjINM-DwIB0lPsRoUrw4VAkEBX76ZCPvgIMO48MBmCgPuarFw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
  Fields returned: ["name", "avatar_url", "age", "id", "bio", "created_at", "user_role", "email", "status"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["password", "age", "bio", "email", "user_role", "created_at", "secret_token", "avatar_url", "id", "name", "api_key", "status"]
  Projected query:
    - 3 fields per record
    - Fields: ["email", "name", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_jwt_auth_flow`
✓ Client created
✓ rs_users_register saved
✓ rs_users_login saved
✓ rs_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/rs_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking — the {{env.JWT_SECRET}} placeholder reads from that whitelist, NEVER from the function definition.

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_crypto_stages`
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

All crypto-stage demos defined. Invoke any of them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_concurrency_stages`
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_path_routed_function`
✓ Client created
✓ rs_route_admin → GET /api/route/users/admin
✓ rs_route_user_by_id → GET /api/route/users/:id
✓ rs_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ rs_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mRust client examples complete![0m
