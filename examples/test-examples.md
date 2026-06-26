make test-examples
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
✅ [32mAll Rust integration tests complete![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'Xah1Rbe6OAMilzKoLhrgu-SDg9RgFjwHpwMKG9a_Xpi3pd0Fw3PrO8xlwLTEOoqQA8uN233BawHkhaTLbiGzVQ'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'Xah1Rbe6OAMilzKoLhrgu-SDg9RgFjwHpwMKG9a_Xpi3pd0Fw3PrO8xlwLTEOoqQA8uN233BawHkhaTLbiGzVQ', 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'id': 'Xah1Rbe6OAMilzKoLhrgu-SDg9RgFjwHpwMKG9a_Xpi3pd0Fw3PrO8xlwLTEOoqQA8uN233BawHkhaTLbiGzVQ', 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'Xah1Rbe6OAMilzKoLhrgu-SDg9RgFjwHpwMKG9a_Xpi3pd0Fw3PrO8xlwLTEOoqQA8uN233BawHkhaTLbiGzVQ'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: WPqvrkImxJdJQGknbhfZE_zv7HPfMd2R_AWgTwv8FDfhndwQgMgaizHru0D74aM7v4u_IW-2BVaM0HqFXLcpUg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "WPqvrkImxJdJQGknbhfZE_zv7HPfMd2R_AWgTwv8FDfhndwQgMgaizHru0D74aM7v4u_IW-2BVaM0HqFXLcpUg",
        "value": {
          "value": 42,
          "type": "Integer"
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "OZdVF92oGdfsvvOSD4yyE45G1-Ue61ATHNGkDBRj_10Z-cgCfY2PomLVC0mMdOXbhG8OuJzrg8y4cBsbOzDxyw",
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      }
    ]
  },
  "messageId": "1779559684"
}
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: oZOlBzsfRFsH-3nCZAt5FFAjUjh_9wsQQNckRRBPYAcKr4kPlWpekb3DwjKe83_CO9-bl-SDfzgpk2-mDhw9NA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: get_active_users_paginated
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: H4PHLxzCH3yP6_oZ4M7yPaCkvqqdu7eyag-SehZpTYfW5rMvK3FEHkAGrg0ZSGeb7JxWOy-JNbl9AIKMVNcTlQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'value': 90, 'type': 'Integer'}, 'count': {'value': 15, 'type': 'Integer'}, 'status': {'value': 'inactive', 'type': 'String'}, 'avg_score': {'value': 50.0, 'type': 'Float'}}
   {'count': {'value': 15, 'type': 'Integer'}, 'max_score': {'type': 'Integer', 'value': 100}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'status': {'type': 'String', 'value': 'active'}}

📝 Example 4: Function Management

📋 Total scripts: 13
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
Collection created with first record: Y3Es3n-9NzZ3pC3V6NkoNg7QWk8IBGclUbBzRcpHOVr3mbEtq3Fs0spugeEPFkYewz8c_WRgGXiM4VcpiIrHlg

=== List Collections ===
Total collections: 14
Sample collections: ['test_collection', 'functions__ek0_testing', 'demo_collection', 'ttl_cache', 'chat_messages__ek0_testing']

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
Created Alice: $1000 - ID: 33iKPKjE_SRhXWIKNlSb6DvUw4zMMyjV9kZl6JIq2ZjAz8Z4xIh008XWZBsQ2I89qPRxMu8CCxdIrTRYMHeXAw
Created Bob: $500 - ID: 8LoQDvfBoGmtA3ecQoocnayNJQrNKgzCYclvjIARIyHNPTOOm01vEMGkHGPND3UukLx5AJZVKMMIoaUuezYemg

=== Example 1: Begin Transaction ===
Transaction ID: 57ea0f39-b81b-4468-b189-1fa75c13f15c

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
New transaction: ebba06ec-594e-489f-a4f5-d14ea43e81bd
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: j8522t_c3zGV0wkPjjBkTKzKruVkP4TCZzw2Mn45Vg0v3ZvjZF3pMCXHY2DHZtAw_hnM0mYKW8nzkwihwmGolw
Created Bob: $500 - ID: rx5t2ADsAsAn0GO9eIXy87be2nBQb7VtmS1xfBSNzvy3CHyxjp-AexX4IloG5xcxJPGWbqq1aT55u5Mz-xjYYg

=== Example 1: Begin Transaction ===
Transaction ID: 79dcf7ca-441e-4426-8cf6-892f3128fe09

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: TXqjz44JJ-2JFes1bIep6ctRyF3iTxzu1nZFbQ8YROX9L0XeP4sA9xrZjlGu2XPjm1fGv5E0RvRO_0LBex3fQA
Created Bob: $500 - ID: Q7nY2DGv5xjDbOIRTAtbQO07mQ3bPwlK65jDsWlxDWhmBiSjBseDEsiTgVjGqv3LCVYsiZZinfb9lYvlRR6r-w

=== Example 1: Begin Transaction ===
Transaction ID: b5682218-3438-40c6-8a49-00156e7cc85c

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
New transaction: 71237a39-baad-4d8a-9a11-14ca5bd3db4e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: TmH1dK6reIBglvDWbHudbIESK839VKNfTgUme_Y2xQUjP0tFkwkxrm7nUBfIo9D11-8Hh9JVTvvDreKBRPQCtA
Created Bob: $500 - ID: 077Z6t-bJeW4-DDdPU084NcGiMOm5514lnoDPqv66d57d8SyzUTJ57MLQjUyYQ05Nk8xMEdPEd-Ip2CpaACACg

=== Example 1: Begin Transaction ===
Transaction ID: 73126efc-8888-4227-a1b7-f58b3f8d5595

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: z2hjIVqfX_44iI3eulRUM-FrMemLzc4Rw9G-UAHLcp7cnnGcVZcvCTp7XumAg7a0czVeGwBatJ3r4YpMfbRJ1A
Created Bob: $500 - ID: Gh990eS12EM_jPsf2HbBc1q_95zhsdgfcbxRuMWIgoJSy7Y6GMzhuF2oc7Cq5Gce7JbJiippslwFLGaMar2LZg

=== Example 1: Begin Transaction ===
Transaction ID: 36ffc907-d843-4e6e-bb1f-7f77e644dc69

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
New transaction: 99ae94c8-1011-41d9-8ecb-da4496022da5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gyTktnnINiB41eCV_aAkMBnN8cBegDjslkJmShUSSmEIsZgr7tD0ts7sywxkGEP8fRTsudiFk3bk3OaHVeUEIA
Created Bob: $500 - ID: E-9tfOIgARELqmywfyoJ3-H7GsfOAIqfMDcJgzs8OAfw_IJhz3dkC826Qa37Xj5jSqS2AV5GgAGhARzS8x84EA

=== Example 1: Begin Transaction ===
Transaction ID: 0052185b-0d60-4005-9cdf-0d936b705875

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


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================
============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: bOy3FQIPoZSbp16wgwx-tA3LJ-lDGu80xrO-hmyJH2Cns5TlE-_FF9OktwcwVNHczvnDnAUqzxWboxigMh5pHA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 3uwCKf8J6V8ggCUMgZvA-sNZ8EKRtB70HRic3brpLJht7R5lRpuHahZRRms6PkifeNSNwi9t5nddKHJA07743g
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: YBzOGsEOExfrwgdFJomszigxkb0tWArwGnfbbtIS1Fk6Cpw8tEhoFVD51nugLXK4wyHBmKanS2vTAEih70WgmA

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: zOY-rBaL10PJybdiQ7x6GC_QlIlX3F5-nu2sGreGJDEAKBXuOgZrWaweFcwg4cPbnyovLp-OE3YQaABnYH9pfw

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 5a9xWSLy8xPFusCWwUG-RKfVEkug5c3A_mWp2J_RoPdsi8NhJCYgrslV3qaSnhkW2nnkCYZIPqGShu0udSr9Vg

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: bOy3FQIPoZSbp16wgwx-...
   ✅ Deleted script: YBzOGsEOExfrwgdFJoms...
   ✅ Deleted script: zOY-rBaL10PJybdiQ7x6...
   ✅ Deleted script: 5a9xWSLy8xPFusCWwUG-...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
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
  Output: Document ID = tAXA3_d8JKcbeYtwcAYbOpInHD4v_CI8ZtYn1zMf_dQfx6vAI3FHZOpO1UpjUME6tZhmDwpswVYmJMejO6MtWg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(tAXA3_d8JKcbeYtwcAYbOpInHD4v_CI8ZtYn1zMf_dQfx6vAI3FHZOpO1UpjUME6tZhmDwpswVYmJMejO6MtWg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(tAXA3_d8JKcbeYtwcAYbOpInHD4v_CI8ZtYn1zMf_dQfx6vAI3FHZOpO1UpjUME6tZhmDwpswVYmJMejO6MtWg)
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
  Output: Document ID = e1YQyrd0lNEt8nyeKxFwCt9HB9s-n-nprNIfl3tgvyy6iaNtoSITr8KXwmE4MnHPtYb3vxnujkJlXOdgxq564w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(e1YQyrd0lNEt8nyeKxFwCt9HB9s-n-nprNIfl3tgvyy6iaNtoSITr8KXwmE4MnHPtYb3vxnujkJlXOdgxq564w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(e1YQyrd0lNEt8nyeKxFwCt9HB9s-n-nprNIfl3tgvyy6iaNtoSITr8KXwmE4MnHPtYb3vxnujkJlXOdgxq564w)
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
=== Running crud_functions.py ===[0m
[32m✓ crud_functions.py completed successfully[0m
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
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.17s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.22.0
    Uninstalling ekodb_client-0.22.0:
      Successfully uninstalled ekodb_client-0.22.0
Successfully installed ekodb-client-0.22.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'y7f16jLcg8TSm7H44oLVGkMA0hs2LLXGtU0RoNrk0LFvxnshvAIC-SLWkFy6sg3hkH91SFs7vL6IrSAAum_nyA'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'name': {'value': 'Test Record', 'type': 'String'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'value': 99.99, 'type': 'Float'}, 'id': 'y7f16jLcg8TSm7H44oLVGkMA0hs2LLXGtU0RoNrk0LFvxnshvAIC-SLWkFy6sg3hkH91SFs7vL6IrSAAum_nyA', 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-06-26T12:52:30.515842'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-26 12:52:30.515842
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'active': True, 'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'price': 99.99, 'id': 'y7f16jLcg8TSm7H44oLVGkMA0hs2LLXGtU0RoNrk0LFvxnshvAIC-SLWkFy6sg3hkH91SFs7vL6IrSAAum_nyA', 'data': 'aGVsbG8gd29ybGQ=', 'created_at': '2026-06-26T12:52:30.515842'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'id': 'y7f16jLcg8TSm7H44oLVGkMA0hs2LLXGtU0RoNrk0LFvxnshvAIC-SLWkFy6sg3hkH91SFs7vL6IrSAAum_nyA', 'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'type': 'String', 'value': '2026-06-26T12:52:30.515842'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: m07itEkmkNnBB4tguIeH0CTtBpVJXOQD3zwPRp09zYpfN_T3XVLVNnqVOAutgSfjToXXcbTbX3BYPLCWlwLHXA

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
Collection created with first record: "Ao70H8q0jXfAtIhwMsNYbaINv8rHqcn4KTNugs4CJcXNpeA5DK5uaVMbm84gMvrdBQS1pXF33i04PRG7MaH2sg"

=== List Collections ===
Total collections: 13
Sample collections: ['test_collection', 'functions__ek0_testing', 'ttl_cache', 'chat_messages__ek0_testing', 'audit__ek0_testing']

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
  cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

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
Created Alice: $1000 - ID: hrWWv2f6FiEjMw3wvntrlY-PhzlkYKyTFZ9SfhY2-SlYCGVKzbyz2djRnbtJKBERbdKxQoYrlTnBwq7w74pJUQ
Created Bob: $500 - ID: EgyR8CzdPHQNUzhWLdiebQoiUb5L3V7O6jtikdH1uajVVVWPAes3YjG9vrG4NOmyqqXGM13IRH8XJCjEDvRRpw

=== Example 1: Begin Transaction ===
Transaction ID: fb556514-5238-4465-810c-2f1e962567b1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 18d46038-e320-4656-b2db-686073226f6d
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.768
  2. Score: 0.761
  3. Score: 0.740

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.370, Matched: content, title
  2. Score: 0.880, Matched: title, content
  3. Score: 0.384, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: zUs_JCSaJgiUlRnd4OpTjy44RC4VOKRQ29JkbZYcwoK1HmXYa0WVavHJRFcI3l2lBcD1us3TACO26UIuM-LB3g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: UwdrHn9j80SEpxRlfidfYDWJo3gRaKLKga4hiq91o_Ids-QF38Dqy6jtBVgW9zFZSg4EX5YGQVLzZ4P2oq6Iow

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
✓ Inserted document with TTL: 6SJq7alfnYLwYhlJjhnLi-9Xu1JSViuBUzcNW3ch-_GDoaEHBKW4_wE3Em8y3QvB1n-I_K-OQ6eU3DBke2hz5A

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
✓ Edge cache script created: ES64RkWj0JW7E5FHprrhWmsdOjlud0k1honmbzsy4j4He7k4nfHMOZ73om7DrJVkKDbSH_C-gWQLU2097DNwkQ

Call 1: Cache miss (fetches from API)
Response time: 431ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 25.9,
            "time": "2026-06-26T16:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.011205673217773438,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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
Response time: 11ms (40.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 25.9,
            "time": "2026-06-26T16:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.011205673217773438,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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

🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: get_active_users
📊 Found 10 active users

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved: get_users_by_status
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 74zXRKvj-lgyQn-NqHQtun4iC_V-BSnq_TlbkEJKxNPtbbJnjPFBhjRVQ3C2jDkoFaUe5faIvLa21Dr6wtKtMw
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

📝 Example 4: Function Management

📋 Total functions: 14
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 1.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 0.9x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: iX7EOEvntwRssdwwv1h1IbqLlHWvPPbo4fSdoylGOQfIcGZDXWRVcWUPrAsMfDOPnwTXwd4tppuRl7bo_ApOmQ
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Nmj4NOH6LjHsdLRJVyyWwYYwxsl_SDN74M5VHQJGUZonmsT4aKzO57_k2UetCtAkq5l8HySXbBaQpV-Y0SogDQ
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
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: Oa6XY7e-pDd20255jv4YnvDndvr3DHvfQhik_UnmJOiGTyo2Zmmo4MenmhWPI_cmz7q4NcrBRKEBVaFdL14L-A
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-06-26T16:52:33.418802+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (fowGnGeclSjnKtbaZ46yghCPUxm-npvKSDL0nHOuIc0lDctuuwefLgJOBrqEep35-e8q1Ew8C0FK3c1-jCUlug)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
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
Response time: 2ms (served from cache)
Result: {
  "records": [
    {
      "value": {
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
✓ Created native SWR script: github_user_native (BiSbYurO0DVDar49P4PNltdD61z-FpUIDBWBRbZa-lRvcGVTeGCItEI5Pc7I7LW0-VxhkgQmijdWbPS1vswc5A)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 45.2x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (lHxRbhfK0jkjR3niuNvn9OQuRMiuXb4hhzCxeNAq_Sp01VHI_Nx-2ATVQbspieemCURVLhOmSpdg637FgwM2Og)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (a2Sc3WvhhApUgEj9mKWcxQQ41JV1-tg-st0oHmSoIhLXdSYaIqEx4lliri-9emF3pTMHI2Fvh1D2Bo0tQez6uQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (a04f2U7hOxMCofTGT1GwM1qFGVNnyJBOtUO1jKbkqkJ_bXWRObX7jPJHCyBO1EKTWPcVtXHa_44jauXjA88Emg)
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
🚀 ekoDB Python Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {'avg_price': {'type': 'Float', 'value': 365.6666666666667}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 367.0}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 5}}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': {'type': 'Integer', 'value': 8}}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in retrieving similar items using vector embeddings, which is ideal for applications like image recognition and recommendation systems.

2. **High Dimensional Data Handling**: Vector databases effectively manage and query high-dimensional data, enabling complex queries that traditional databases struggle with.

3. **Scalability**: They are designed to scale with large datasets, accommodating growth in data and queries without significant performance loss.

4. **Real-time Performance**: Many vector databases provide low-latency responses, making them suitable for real-time applications.

5. **Integration with Machine Learning**: They work seamlessly with machine learning models to store and retrieve embeddings, enhancing AI-driven applications.

6. **Flexible Querying**: Users can execute various types of queries, such as k-nearest neighbor searches, to derive insights from data.

7. **Semantic Understanding**: Vector representations capture semantic relationships, improving the interpretation of data in NLP and other contexts.

8. **Enhanced Search Capabilities**: They support advanced search functionalities, such as faceted search and interactive querying based on similarity.

Overall, vector databases are particularly beneficial for applications focused on managing and querying complex, high-dimensional data effectively.
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
🚀 ekoDB Python CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: dNGb9c_qlepdYXRMjHPAGJy-MDa4ulQLp2WDlIqDZw6P0oMFTVsKtL3Kbm7QmxX2GWwRZn8COanmCIfYs3kCBA

=== Sending Chat Message ===
Message ID: XRT7RlUJ9nwJA4R7BS66A2_2cfQ78jOhxogmGJRZwZoIY33ib_uEs7k1f_HtrcUF1zhjNxRX23DyaiOeqjHbOQ

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

Execution Time: 11565ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: l8Irj6iHxXLJiDvqHYzsGL-I9z-X23UGygGkme7y6lNCp4PjTfQsSFnBrTU5U4O3bu92dGpAt_ZaNpgnAlksvg

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no available products in the database. However, based on the context provided, I found the following product:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you have any other questions or if there's anything specific you're looking for, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'chat_id', 'created_at', 'role', 'llm_model', 'token_usage', 'llm_provider', 'content', 'id', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you need more information or have any other questions, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: ukiK-AXE26Q3Zjg_6vlCTPlIKs6tPt_cZ1FJZmS2k3GLm7E9jvnL_X7O14_DOGfmoVXa1X1BaT06zCM8dy9weQ
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w

=== Sending Messages ===
✓ Message 1 sent
  Response: It seems there are no records available in the database regarding products. However, from the context provided, I found information about a product:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you are looking for more specific details or different products, please provide additional context or clarify your request!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1xlbu1PejFWAh14E_CAWeWalbzvJ_Ejjv3mCBa043nrR5ntGIQCemyV9Y85MWjMyt7baxRjchkieocZugla49A
  Parent: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: 1xlbu1PejFWAh14E_CAWeWalbzvJ_Ejjv3mCBa043nrR5ntGIQCemyV9Y85MWjMyt7baxRjchkieocZugla49A (Untitled)
  Session 2: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w (Untitled)
  Session 3: ukiK-AXE26Q3Zjg_6vlCTPlIKs6tPt_cZ1FJZmS2k3GLm7E9jvnL_X7O14_DOGfmoVXa1X1BaT06zCM8dy9weQ (Untitled)
  Session 4: dNGb9c_qlepdYXRMjHPAGJy-MDa4ulQLp2WDlIqDZw6P0oMFTVsKtL3Kbm7QmxX2GWwRZn8COanmCIfYs3kCBA (Untitled)
  Session 5: 6iCjiw0uTvLDoU1C2PDmYU20G9zcAOYlEnu-WLoDMnO2zotdLwhU4slQ0dBk3ZXelzYuE0Suy_FNS2QhZ3KUPw (Untitled)
  Session 6: FjhdyrsoiSxi3Wvhn_BjeTxxYLupEHEXgBlIAm75KSmSVRD5jV6FYxG0fBI4F5AEpPH2IIcsnreDVjMJLcTrcQ (Untitled)
  Session 7: ieawk8LiucOs5nKI_C_wHeCLjw92fUJnpMz3PFAce2AWqQtREuB4lsOSg9wo8gn3QNFF03_aSZGLtGlGMSasFw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 1xlbu1PejFWAh14E_CAWeWalbzvJ_Ejjv3mCBa043nrR5ntGIQCemyV9Y85MWjMyt7baxRjchkieocZugla49A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'LsO2qxhoaheVJwHDUXGQNxQRVB61VnHw9lnDaNv91w0ua9uPO-NRaFMZjVeoUQynwHQaGWQgQOWaV_yoVThg7A'}

=== Upsert Operation ===
✓ Upsert (update existing record): LsO2qxhoaheVJwHDUXGQNxQRVB61VnHw9lnDaNv91w0ua9uPO-NRaFMZjVeoUQynwHQaGWQgQOWaV_yoVThg7A
✓ Inserted second record: xf4rglBbkwz6_e7Xt7Cc5QIA0ztrokM4kudXq0tZen20-1qo87Y1unNQXo7KqGnTsxJgjwmi4Sl_2BoFOaQadw
✓ Upsert (update second record): xf4rglBbkwz6_e7Xt7Cc5QIA0ztrokM4kudXq0tZen20-1qo87Y1unNQXo7KqGnTsxJgjwmi4Sl_2BoFOaQadw

=== Find One Operation ===
✓ Found user by email: {'age': {'value': 29, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'id': 'LsO2qxhoaheVJwHDUXGQNxQRVB61VnHw9lnDaNv91w0ua9uPO-NRaFMZjVeoUQynwHQaGWQgQOWaV_yoVThg7A', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}}
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
   Inserted with ripple: {'id': 'FnrNplAoxFME4VahPjCBgghjFLGQw0FlkGUFYIu_Kco2eU8CCbiRCojp7X-IEiMS770GHe1tJPIGQXYQMpwwLQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'EKsm-3Njx9VFfHhN-SfdxIJbrpWDgyMifH3rgf8Y14d33uxVULd33mmf7rr5Htl6GXTRLSf8OU65-dbu6TX0Yg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'FnrNplAoxFME4VahPjCBgghjFLGQw0FlkGUFYIu_Kco2eU8CCbiRCojp7X-IEiMS770GHe1tJPIGQXYQMpwwLQ', 'price': {'value': 150, 'type': 'Integer'}, 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'FnrNplAoxFME4VahPjCBgghjFLGQw0FlkGUFYIu_Kco2eU8CCbiRCojp7X-IEiMS770GHe1tJPIGQXYQMpwwLQ', 'price': {'type': 'Integer', 'value': 500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'id', 'name']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'type': 'String', 'value': 'alice@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['created_at', 'id', 'email', 'status', 'age', 'avatar_url', 'name', 'bio', 'user_role']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'value': 30, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['created_at', 'id', 'password', 'status', 'email', 'avatar_url', 'user_role', 'api_key', 'age', 'secret_token', 'bio', 'name']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'id', 'email']
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
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper']
Anthropic models: ['claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper']

=== Get Anthropic Models ===
Anthropic models: ['claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: LJTxWUMlbYnNlAiDSrPvLcaH7GsyztWtafldhqMr_XAyz8Sg6SsJA6R7UUKuVHRUOOtXwWbEBswxMs9CdNmCIA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_py: Get Active Users
  - get_users_by_status: Get Users By Status
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - cache_api_call_py: Cache External API Call
  - get_active_users_paginated: Get Active Users (Paginated)

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
✓ Client created
✓ py_users_register saved
✓ py_users_login saved
✓ py_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
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
✓ Client created
✓ py_route_admin saved
✓ py_route_user_by_id saved
✓ py_route_user_posts saved
✓ py_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
Inserted: map[id:rBHS5emvP1jqb01pG8koyTYFl2ok14lRo4Cy-OP_scqbFzODhaRpwkGp0KT8DO3RWoYT1lmkGAKbFlb7MLMjgA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:rBHS5emvP1jqb01pG8koyTYFl2ok14lRo4Cy-OP_scqbFzODhaRpwkGp0KT8DO3RWoYT1lmkGAKbFlb7MLMjgA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:rBHS5emvP1jqb01pG8koyTYFl2ok14lRo4Cy-OP_scqbFzODhaRpwkGp0KT8DO3RWoYT1lmkGAKbFlb7MLMjgA name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:rBHS5emvP1jqb01pG8koyTYFl2ok14lRo4Cy-OP_scqbFzODhaRpwkGp0KT8DO3RWoYT1lmkGAKbFlb7MLMjgA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 3qPv3CEUDmu5T7on85zmtKGt3BFAZyaVEhodwHKMxfq7qiOuKsqIELqjjSv4-KCbxZs4B_7tAJfyw2uSUyK5IQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1782492829425191000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "WPqvrkImxJdJQGknbhfZE_zv7HPfMd2R_AWgTwv8FDfhndwQgMgaizHru0D74aM7v4u_IW-2BVaM0HqFXLcpUg",
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
        "id": "3qPv3CEUDmu5T7on85zmtKGt3BFAZyaVEhodwHKMxfq7qiOuKsqIELqjjSv4-KCbxZs4B_7tAJfyw2uSUyK5IQ",
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
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Functions Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: n_V2DOylszAwxgyK7BshpcSBJwfOlmHgb72QckdqFNOQpja_vbSs9SzKuYFW5b4u-AN8wpbfqRuMEy-cfFhB6g
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: wwNI0lB5p1pDvIztYfiNXK5Yyph0Kb5-IK5DWGqSlloTD15DqVPsfDnduSOC77JIhdl-6B-WjP5m6HJwbuQnGA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: MZJd83U9kKvP40vPACUPQj7o0iHWRomrzx9Oqo4OGXhDghRISO2qBtrLCb8LD_YfY5WeFh5BIy8_zQ87Ha22uw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}

📝 Example 4: Function Management

📋 Total functions: 15
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
Collection created with first record: gb-JcQIxzQc1IopPAk9oXIv-jktier-MDe1q5SKfacc3bmE6ise0vLiKpJo4Z2UyZRAy16XG4vXwbG37Agmwkg

=== List Collections ===
Total collections: 18
Sample collections: [test_collection functions__ek0_testing demo_collection ttl_cache chat_messages__ek0_testing]

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
Created Alice: $1000 - ID: vhsmBUMMCkrUWPLYR36gBTJTR8nTnNEKeM0uzBPSlPOWSFjI7hFF0tVx9BPQNzgacu-nRJLYF7DaH_54Wx5CAA
Created Bob: $500 - ID: nLXZg2Pv_3wGJ5IYnuKQhdoHnaKQDU90Kl1kxFfJJjsslLCLeqI9THnT1LIwwXih7Q1fJXBTMBfB8X6CgqKhlQ

=== Example 1: Begin Transaction ===
Transaction ID: 96222cd0-4dac-4b68-928f-1b9a75cb13c4

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
New transaction: 39ca1eba-ef79-4281-b3f3-3b1a3c548a94
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_functions.go ===[0m
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 28VYMR1JM09iRRaPaPkmm8R8fHfZm9k4YZVxJeVuPnqBpznT78OD4VKgGU945_xSm8Sst3B_SWWd_gPPFd90Vw

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 82SiS8lSo2z5rgXC0MFervbI-b_21ytZbGdXO7ccbT1T1NU93aqpJqPsUMHtcgq6slesVeh8i0Q8Q97-3QZ0lw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: ijT5nqn7J9bMVlpGUHvwsD09hu8PrNGBtzOYfLUwYVbcwpbWVBFO689ZKXixscQSKAtAF0X0BuWQGYU7Yv8wDA

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: zKfKAjiewv28CZiOcPjDgcB39tqBOE7pqdKtfSIB7sacnf6Eo5eYEh7PMvNzpfdnGva2JHejZ1sBHRz4IfrQig

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 63pCDs4vKGOObiqts7XpZbQl9wzWPcx-SjI2lWUtjAw2GT6Q62LrI-wxy-G0ASPBKya0YWPv0MUgdaqOZZMFvA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 28VYMR1JM09iRRaPaPkm...
   ✅ Deleted script: ijT5nqn7J9bMVlpGUHvw...
   ✅ Deleted script: zKfKAjiewv28CZiOcPjD...
   ✅ Deleted script: 63pCDs4vKGOObiqts7Xp...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain steps together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.go completed successfully[0m
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
  Output: Document ID = yvsw9SF-8WCnuxhu9R4Bed4eAINjcqdz9XeYY5JqaLlsyfY4oSkUFOpQvNCQ_QxnZ-o7vKiofVU5mHOgKIjjfw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(yvsw9SF-8WCnuxhu9R4Bed4eAINjcqdz9XeYY5JqaLlsyfY4oSkUFOpQvNCQ_QxnZ-o7vKiofVU5mHOgKIjjfw)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(yvsw9SF-8WCnuxhu9R4Bed4eAINjcqdz9XeYY5JqaLlsyfY4oSkUFOpQvNCQ_QxnZ-o7vKiofVU5mHOgKIjjfw)
  Output: Error (expected) - request failed with status 404: ��error�Record has been deleted
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
  Output: Document ID = o9csafS1HLS3Zns89D0Bv5j5KAMHPuRPCcz1WnokS_KEnjAVvCpV5whHzaTWwfiCBo086904Qq2pDKIgvL2S6Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(o9csafS1HLS3Zns89D0Bv5j5KAMHPuRPCcz1WnokS_KEnjAVvCpV5whHzaTWwfiCBo086904Qq2pDKIgvL2S6Q)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(o9csafS1HLS3Zns89D0Bv5j5KAMHPuRPCcz1WnokS_KEnjAVvCpV5whHzaTWwfiCBo086904Qq2pDKIgvL2S6Q)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
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
Inserted: map[id:UQL1g0PA-kuKSoCkBXTaVVt2asXq0SO8xqMKYg1Z08yJ18dmzdUGxrUrQTHL1uf535YuDxDcVuRBQ0RUnPPzVg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-06-26T16:54:18+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:UQL1g0PA-kuKSoCkBXTaVVt2asXq0SO8xqMKYg1Z08yJ18dmzdUGxrUrQTHL1uf535YuDxDcVuRBQ0RUnPPzVg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-06-26 16:54:18 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-06-26T16:54:18+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:UQL1g0PA-kuKSoCkBXTaVVt2asXq0SO8xqMKYg1Z08yJ18dmzdUGxrUrQTHL1uf535YuDxDcVuRBQ0RUnPPzVg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-06-26T16:54:18+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:UQL1g0PA-kuKSoCkBXTaVVt2asXq0SO8xqMKYg1Z08yJ18dmzdUGxrUrQTHL1uf535YuDxDcVuRBQ0RUnPPzVg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: lWIXlaFMfPfZ9d4jUprZAKrrSoa2DzprtMxFL5mAf4PuVvhEp4NZ6gzihAMtgs4ZcOfoThDduTlAat-Np4UALg

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
Collection created with first record: 3vgV8TcxE6ta-ur6ALrQAittuAD7zAx0aOOdmG6qEAZNI_96a_udJ5_cnzybwu5bbuS4YO37zLHsiCtiQikznQ

=== List Collections ===
Total collections: 17
Sample collections: [test_collection functions__ek0_testing ttl_cache chat_messages__ek0_testing audit__ek0_testing]

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
Created Alice: $1000 - ID: Ib20qEU3ZV0Ix_Tfu9q6I2Zr3p9xEAxW-LSbV-zsG32G-GQ59lwYpDfC9sgjV64UeqsZJ6FwkmLgSSE3_Yjfjg
Created Bob: $500 - ID: 488acEcVoOKk1StzeOolT7rPuidUqclL7zmvDplEAWpkXQ_BPPeE21q9B_zFLUG7kHL6Wj7svn8EosMe1dQTSA

=== Example 1: Begin Transaction ===
Transaction ID: 3f0712b5-f1e0-4c0b-be29-00d6188bad49

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: cce126da-9e4b-4203-85eb-70c30ee0f638
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
  1. Score: 12.870
  2. Score: 6.270

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200
  2. Score: 13.200
  3. Score: 13.200
  4. Score: 13.200

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600
  2. Score: 6.600

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.754
  2. Score: 0.737
  3. Score: 0.733

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.493
  2. Score: 0.902
  3. Score: 0.295

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

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
✓ Inserted document: 4wEb_NE0AOD-eTds_GJ8cwzkd7_iWSi6rN2fV-Sqopp90xWx9FmwNfgLOs97LEzH6EKVbM3cmqRLyDpVZjKSMA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: WOU7XqGLVQrRuNFaoD5KEh5EpB9aQFLscKtmynfVzm-Uk_CYB95arkJlGUM2w5ov2Qmf_wMiIrP_YN_zAPx2lw

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
✓ Inserted document with TTL: nxdD36dqTOAiGSn_E2I_GT-dNE8UShGP0LdakI6WmD6gsCufaI7rnAggCoumoo5LzPdjTH27mb_Jdhl6Srh9lQ

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
✓ Edge cache script created: EeNnmKFs-Pb2UJA-lkZXRGJegg2UgFuxakrM-LYBTJcvUsk-W11ccUwjG8CoKiBz6ydbeqlsLJyTCU4mW0iu8w

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
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: W4wiNIj2agh43bCSaA0Zmiy12uOoumzHNp0LoqfgcO3KWT_DlhIMIZdi66gyXPMIdmLzt_4yWHwip1Iw5ekRpA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 15
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 0
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

Function 'fetch_user' already existed — updated instead
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

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 1.353625ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.101666ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
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
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: L-1FZ8SoeB-uK7wGxJlf5b0GbXnnMoI08H__MrNkZ3VmL7Tktq74FG4Hf84Pjd_kzrEj2jw_88Jbkv_f4Lxejg
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: ri6_1M5SAQTQ1dvj-Mf79VLab6Pb1KMKuFscdikQrcKwUAml3dNpchHc5ZiuVebbRlVxrGD0OkDCJJgiTjxWpw
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: KCpTtny-ExFxPeLHC1NpSrHHzQ-98g3A8yZ0p6Lf5RBe9Z_339Laenb-ZIWyqs-TkazyS4fk2AcehEhigmhBrQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: kKPlqWgx8U6aS1_0eX-rzCT6lvcwODGc6Zfss6bEanQTFC7c_m8RxM3huJvlA-7qjITs5rbJ30fk_sNb0Bl_HQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: _mHDjIOIic__3va4jcVZ7TdKdvUCd6Lmj2hd81QGr5nnf5JFpTQUHanHbn987YHy1Fm-IYtF0LaRWyzYIajjyA
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
✓ Created SWR script: fetch_api_user_go (5e4VEJruKmBEGTWO0PNIEQPjztxJ65RakTMjBhnwnWk7gkxhgp1KN9YBqUoHpA0APx9IdtBbBo7ScHG1dg-QgQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-06-26T16:54:22+00:00"
      },
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
Response time: 2ms (served from cache)
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
✓ Created native SWR script: github_user_native (SADFNC9lM-7pF1N6as-hS_dcEacGcNCT-5sXedw25sykt124kCNAF2yl8WRJ9vYgxfpdh_Hi0qW8MQtGb_jujQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 1ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 8ms
  Speedup: 0.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (OPfi79R6QNa55b7U4yj-N9neN_R3gd0NOkMFqWSWuvsmOlAJnihv_XRoyP4Z_p8x5vJDOftvMw74N3UKGyIBnw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (qNoj1M9MlCwcosGvOPNTQ61vqki80HnPMf4q5F_QkwZyrRIizyS4KsvILrwY4ld_G9G0Pezop1rbfkSDgrFRxA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (_cOxCTpyhvcmYJvIJRJwrcczjoaPrBA-rQLd9kwr4qIWijoWuYLVgz8YEapBu3Ac8yBhbGimA1pLDIP19VYFaA)
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
🚀 ekoDB Go Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Functions Example

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
🚀 ekoDB Go CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   map[count:map[type:Integer value:7] status:map[type:String value:active]]
   map[count:map[type:Integer value:3] status:map[type:String value:inactive]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: dKGks_vWoqc7xBDZc623Z7LgNSCJJ5QFe-FA_xx6sKDPg8-MHLIDLF0rc3NBksrUQ-nv8W3py5ltjB6qOfhwsw

=== Sending Chat Message ===
Message ID: 2xE23smPCF7gqQsxRaHUVHKYklcSNmr7VUa4Qd7HQHeKxIGhSdy3v2EEkGk_eFsN9optJvuWqTIF27DCfifmIw

=== AI Response ===
The available products and their prices are:

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
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:XRJrArvu9xDkpcKcFXjnqeSoRrqKUQLnLOZ24zfII9mDp4GxzIPyUdhRoq7JbW-X8AJwo2hmEq2bMMYbJWosBA name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:ngJcei16NZlSsQOeGFj3tZZnVMufiY-1Odr6Ue9VkkI6l00IffaEVeUZSkVHR-vFhwYTStKYpfZ-KQJxiV-qFw name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:Mi2TyDj5UTHZdSctyfJqpevu0HK-bcNKFnS1jA9pz_MAtT2CimKi3J1JyyzHZ1yHfs__OvLK-0HdSwgp5_nhnA name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 7319ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 88
Total tokens: 3501

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: _AR7Sx91zj6gU8CeP--Gj5P10rfQJmxcDclcGIN83zOTGxGUdIATYrOJ2de_88VXVcVrSI5yte0oDAulvNXGEw

=== Sending Initial Message ===
✓ Message sent
  Response: It seems that there are currently no available products listed in the database. If you have any specifications or if you're looking for something specific, feel free to let me know!

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
✓ Created second session: mqEaN0jW1PYv4-F-nE5E0Y9pe_1D-NBvlwKr9Hqp7Gl520k7RrXGqQpjE3cinyv438pzqdTSH71F20QHVQGxDw
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: X8y4BQWBBNZKMk4RpdETvhBcErKX00VRLLkDmYJCw_loP0dqoXD5MaZ3rwYVtdDUG-uzHCR8Nd5SvG9SjI4_Aw

=== Sending Messages ===
✓ Message 1 sent
  Response: It seems there are currently no products available in the database under the specified collection. However, based on the context provided, we do know about the ekoDB product:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you have any other specific queries or need additional information, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1ILKTgQSelQe2NQ-ZY4t01oqPL34lKxmCi2Los6RUQB_EWAW1ZSEokY-gMPV67MwiaoFwB1gfHuFLDLm2Zq_Ew
  Parent: X8y4BQWBBNZKMk4RpdETvhBcErKX00VRLLkDmYJCw_loP0dqoXD5MaZ3rwYVtdDUG-uzHCR8Nd5SvG9SjI4_Aw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 1ILKTgQSelQe2NQ-ZY4t01oqPL34lKxmCi2Los6RUQB_EWAW1ZSEokY-gMPV67MwiaoFwB1gfHuFLDLm2Zq_Ew (Untitled)
  Session 2: X8y4BQWBBNZKMk4RpdETvhBcErKX00VRLLkDmYJCw_loP0dqoXD5MaZ3rwYVtdDUG-uzHCR8Nd5SvG9SjI4_Aw (Untitled)
  Session 3: mqEaN0jW1PYv4-F-nE5E0Y9pe_1D-NBvlwKr9Hqp7Gl520k7RrXGqQpjE3cinyv438pzqdTSH71F20QHVQGxDw (Untitled)
  Session 4: dKGks_vWoqc7xBDZc623Z7LgNSCJJ5QFe-FA_xx6sKDPg8-MHLIDLF0rc3NBksrUQ-nv8W3py5ltjB6qOfhwsw (Untitled)
  Session 5: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w (Untitled)
  Session 6: ukiK-AXE26Q3Zjg_6vlCTPlIKs6tPt_cZ1FJZmS2k3GLm7E9jvnL_X7O14_DOGfmoVXa1X1BaT06zCM8dy9weQ (Untitled)
  Session 7: dNGb9c_qlepdYXRMjHPAGJy-MDa4ulQLp2WDlIqDZw6P0oMFTVsKtL3Kbm7QmxX2GWwRZn8COanmCIfYs3kCBA (Untitled)
  Session 8: 6iCjiw0uTvLDoU1C2PDmYU20G9zcAOYlEnu-WLoDMnO2zotdLwhU4slQ0dBk3ZXelzYuE0Suy_FNS2QhZ3KUPw (Untitled)
  Session 9: FjhdyrsoiSxi3Wvhn_BjeTxxYLupEHEXgBlIAm75KSmSVRD5jV6FYxG0fBI4F5AEpPH2IIcsnreDVjMJLcTrcQ (Untitled)
  Session 10: ieawk8LiucOs5nKI_C_wHeCLjw92fUJnpMz3PFAce2AWqQtREuB4lsOSg9wo8gn3QNFF03_aSZGLtGlGMSasFw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1ILKTgQSelQe2NQ-ZY4t01oqPL34lKxmCi2Los6RUQB_EWAW1ZSEokY-gMPV67MwiaoFwB1gfHuFLDLm2Zq_Ew

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:oGp1EDNo_nh8UIEeYa3w1n3Nm0lRSISJaIpxODhew_-ASlfp86GAhlJFaQmC-vN46uX-Skfqx4FIppj7rjqkKA]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:oGp1EDNo_nh8UIEeYa3w1n3Nm0lRSISJaIpxODhew_-ASlfp86GAhlJFaQmC-vN46uX-Skfqx4FIppj7rjqkKA name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:oGp1EDNo_nh8UIEeYa3w1n3Nm0lRSISJaIpxODhew_-ASlfp86GAhlJFaQmC-vN46uX-Skfqx4FIppj7rjqkKA name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:Dw5UVDOsuGyqyO_DNJaSuO3ljvve2rIXsaZIuh5w6Bdi3zZQfzHK82cIBrG5jeOnEN-iJx-ue8rY63OPWBZTrw]
Inserted with bypass_ripple: map[id:4RS0M3E87mc1n0_9n8R9qXdo0eCr21CHZb1NpEWJ0veYOuIOBFtXR--m587awP17L0ugOlgCFG2ZGJUO2QLJnw]
Inserted with TTL and bypass_ripple: map[id:6lsR5sA10exV38XW4u-elMkXWSLJAx_is3iYcCGyf06vks4RnRPqlc8ytjSIdsQDZSyF66Z3tLxKKRUHRSBKRQ]
Updated with bypass_ripple: map[id:Dw5UVDOsuGyqyO_DNJaSuO3ljvve2rIXsaZIuh5w6Bdi3zZQfzHK82cIBrG5jeOnEN-iJx-ue8rY63OPWBZTrw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
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
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper]
Anthropic models: [claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805]
Perplexity models: [sonar sonar-pro sonar-deep-research sonar-reasoning sonar-reasoning-pro]

=== Get OpenAI Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper]

=== Get Anthropic Models ===
Anthropic models: [claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805]

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: vbA7zLwSSw6WY_JDyz1_tKspQ2bRm5Vwwo0VKqueaPzC5X9nGTEPlIsAaBP8BQiat4vDgrc32V8W1xbEcZe2jg

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 9 user functions:
  - get_active_users: Get Active Users
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - cache_api_call_py: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)

=== List User Functions by Tag ===
Found 9 user functions with 'users' tag:
  - get_active_users
  - get_verified_user
  - fetch_and_store_user
  - fetch_slim_user
  - validate_user
  - get_user_wrapper
  - fetch_user
  - cache_api_call_py
  - swr_user

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
✓ Client created
✓ go_users_register saved
✓ go_users_login saved
✓ go_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/go_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
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
✓ Client created
✓ go_route_admin saved
✓ go_route_user_by_id saved
✓ go_route_user_posts saved
✓ go_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.9',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.22.0 prepare
> npm run build


> @ekodb/ekodb-client@0.22.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.22.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'b7gVER08TjNxKhJ7e6XJII7nsAWUVayZJSqaRpSepa4QBHI9ShONaUd9VXnr6iDxe2Lmo7dptkLuPrkWIqV3Tg'
}

=== Find by ID ===
Found: {
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  id: 'b7gVER08TjNxKhJ7e6XJII7nsAWUVayZJSqaRpSepa4QBHI9ShONaUd9VXnr6iDxe2Lmo7dptkLuPrkWIqV3Tg',
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  price: { value: 99.99, type: 'Float' },
  active: { value: true, type: 'Boolean' },
  created_at: { type: 'DateTime', value: '2026-06-26T16:55:28.504+00:00' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-26T16:55:28.504Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  name: 'Test Record',
  value: 42,
  id: 'b7gVER08TjNxKhJ7e6XJII7nsAWUVayZJSqaRpSepa4QBHI9ShONaUd9VXnr6iDxe2Lmo7dptkLuPrkWIqV3Tg',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  price: 99.99,
  active: true,
  created_at: '2026-06-26T16:55:28.504+00:00'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  created_at: { type: 'DateTime', value: '2026-06-26T16:55:28.504+00:00' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  id: 'b7gVER08TjNxKhJ7e6XJII7nsAWUVayZJSqaRpSepa4QBHI9ShONaUd9VXnr6iDxe2Lmo7dptkLuPrkWIqV3Tg'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: usAtQmo5ZNMY7c_Pe9EE8Hstau8zvfup7tprfICkw2hCbV8WYKDrK8wIRCHCl2ktQ4YsYS6jGc0XQQQgnxUtkg

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
Collection created with first record: p11izS8mMAuMnu6QlcvB0jCuXiEkTqCPNUKDtgpbHJX4-gXhhkN-u0n11Ba5bwIYHfxXDEl5Uf-z74XMKYfKxA

=== List Collections ===
Total collections: 23
Sample collections: test_collection,functions__ek0_testing,client_collection_management_ts,ttl_cache,chat_messages__ek0_testing

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
  cache:product:1: { name: 'Product 1', price: 29.99 }
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
Created Alice: $1000 - ID: JGkwG3aGEN9Z217DTX3gbZKLan-E_MNq3hvmFfEKaVnHtjiqjcxVhUhI-tgLYfiHv_IzUITNX5oamoeTmmzt6A
Created Bob: $500 - ID: yQF4-WO00cnIX4qXlcCeFARKnCwXEs98Nq2-iYJCKDXZV0FdIQlZMthHTDtIrdz2si9OYA6DbhyjMGu1uzss5g

=== Example 1: Begin Transaction ===
Transaction ID: 45c7a58b-6a6c-4906-8c59-4bedd4fe2fb0

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 107ddf23-1c4c-4858-bc30-f334d0c237b8
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.757, Matched: 
  2. Score: 0.753, Matched: 
  3. Score: 0.752, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501, Matched: title, content
  2. Score: 0.901, Matched: content, title
  3. Score: 0.303, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

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
✓ Inserted document: erZ02cuxpbrIkpCyhhn6vxxFmnCJP8fgf1zUKFaeBiastnDquxSJ__7wXiIjAzBvXETXriHpRt4l-3fYPcJEnw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: wloENxxZliAvwe4PFgoHHsL3RfQbiK3Lj32upwjvGDPUI17iR5j2o9_W6J5mloFAmgtsJbP1ODOnGl7JXCMP-Q

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
✓ Inserted document with TTL: QLgmPxhEuRgVsjzgFSdg7vRlFMYeHmwYs9eP0rvkedFWMZmMMQTjOYcqk1R7hku6PkqC-cuKbW_40qwfB4JhkA

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
✓ Edge cache script created: Aid05EbUbHJDLcXhdrmXadip3yLUjstBqR9YqkmNxyhsCNvfpbTOygmkQZxgBb_YoHwgZwyfdQjzoRyOV-Wg2g

Call 1: Cache miss (fetches from API)
Response time: 66ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "address": {
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "username": "Bret"
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
Response time: 3ms (22x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "address": {
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "username": "Bret"
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

🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: jn8MQkps18LnEmlrpow61xD-HMJLY762QuX2angZX8w9dxW6b0dmal5HNctqVA2vkVCsnzbbt_dcOqY2F0vQuw
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: qQman-_9D5Cte9LRunKUV1YqFb5BHcj74e9lbedGr6OKYa_h0TkTRLVFfrlkf8nhvo2QJHhOZIQewSL7Ef_cJQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: CTg5q-xaA2LdZNMoh5NeEfvYFqELf3f6qXimvWtsRNJZp6jbH8oCrwoxYzD-uiM3AQlZ86bI32fBiALvWFu2_w
📊 Statistics: 3 groups
   {"avg_score":{"type":"Float","value":60},"status":{"value":"null","type":"String"},"count":{"type":"Integer","value":5}}
   {"count":{"type":"Integer","value":5},"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50}}
   {"status":{"value":"active","type":"String"},"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 12
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets"
      }...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": "harness real-t...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: oUlRYYOL_ORP2g4hyeAlRru-__1ibU6jshsQu_SN3RWLyOuDpTkTtTgzWKXPe56WB5_dLM_jzJiL4-7FVPgkFg
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"avg_price":{"value":474,"type":"Float"},"category":{"type":"String","value":"Furniture"}}
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"value":575.6666666666666,"type":"Float"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: y91fFLU0D8GTdMdAfOSOq8E_y4t3ReuMVlDrvqziav0nA7wbAX24gehq5wVUcaErgdPvVI95_2ervw0GTN9pgQ
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: emxo33ZHBgIcvD9NDI8Hl5Jy2IxJ5Q1Gs_KXEA6KKSuj_P37zI4TV8pJcITJll5tqxqg7cifgmhhUu-oxAf6iw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: j4ut2Frp2Zkh0jRyILvPhyf68cI4SDmL-fkCIJYLhrrmMnjEtG3Fwf-TVRKB3MeHCuyEjECp3gbPvduCakjFXA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: eV7-fR14aghUsxKxlk7AZRRUDXRWWH8Qd8oTWOs5PHQs4dHVXL_AHD1eKnZ8QRLmYyZWa2JSzbqXMBpmm5nvpg
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
✓ Created SWR script: fetch_api_user (5e-THDgBamGiP7gRnezIqWY8J9FSf9vR1zGkOhl7lKHLZPfZpRHMV5pXIZ3TurbK6APrlSiE431hn9kTqpmsyw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "username": "Bret",
          "id": 1,
          "website": "hildegard.org"
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
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "username": "Bret",
          "id": 1,
          "website": "hildegard.org"
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (ffhtp7lIxeI3XLrqi0ff-V4rmzC-3ciVs4eLVP-MoobgnD77XeZsFMQIST4KXqCb4gK-Gb_YhqtI-zrod-46IA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "shippingInformation": "Ships in 3-5 business days",
          "availabilityStatus": "In Stock",
          "category": "beauty",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "title": "Essence Mascara Lash Princess",
          "discountPercentage": 10.48,
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "weight": 4,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "reviewerName": "Eleanor Collins"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!"
            }
          ],
          "returnPolicy": "No return policy",
          "minimumOrderQuantity": 48,
          "meta": {
            "createdAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2025-04-30T09:41:02.053Z"
          },
          "brand": "Essence",
          "sku": "BEA-ESS-ESS-001",
          "rating": 2.56,
          "price": 9.99,
          "stock": 99,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "id": 1,
          "warrantyInformation": "1 week warranty"
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
✓ Created native SWR script: github_user_native (SIofGWKfI_vhRZgRbKDtRnrM3w2nCIk_5-VVj1AV2ZKndumP9_9QcMpgHA1PIaPlHQqj1xnEK21MNpxAzlZXFg)

First call (cache miss - will fetch from GitHub API):
  Response time: 3ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 1.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (HSQpegjMlSKvKg2d61WRMPGFXSfz40o9ZFIjLhJkieKctxWW2wFfKzX4zztMnRCA4gQUlUBcAEUHoTX8yxvTmg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (7EwvZxMwHgMedq-AgTrmHIjBvbonO3ThQepzN8FoYxva9r7bcNqjdbauRWuCeW5ed-GGPVOADPz6exuUNM6_mw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (dYZ5acohyO1atVgAN58MKllHAz-6Ch-RBjKd3TVwYuFHqaHgAboMBbXPCHw_MzzZTh431D3lbDjEdaunDVS6IA)
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
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":365.6666666666667}}
   {"avg_price":{"type":"Float","value":367},"category":{"type":"String","value":"Electronics"},"count":{"value":5,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **High-Dimensional Data Handling**: Efficiently manage and query high-dimensional data (like embeddings from machine learning models).

2. **Fast Similarity Search**: Enable rapid nearest neighbor searches, crucial for recommendation systems and search operations.

3. **Scalability**: Handle large datasets effectively, making them suitable for big data applications.

4. **Flexibility**: Support various data types and formats, accommodating unstructured data like images and texts.

5. **Real-time Processing**: Allow for real-time data retrieval and updates, enhancing performance in dynamic environments.

6. **Improved Performance**: Optimize storage and retrieval processes for complex queries compared to traditional relational databases.

7. **Enhanced Machine Learning Integration**: Seamlessly integrate with machine learning workflows, facilitating model deployment and real-time inference.

8. **Better Similarity Metrics**: Provide advanced distance metrics for evaluating similarity beyond traditional methods.
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
🚀 ekoDB TypeScript CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"role":{"value":"user","type":"String"},"avg_score":{"value":70,"type":"Float"},"count":{"type":"Integer","value":7}}
   {"count":{"value":3,"type":"Integer"},"avg_score":{"value":20,"type":"Float"},"role":{"type":"String","value":"admin"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"value":"Database","type":"String"},"count":{"value":3,"type":"Integer"}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 6Wh9bCQ13lb_HCio7nnE5Hr8aCUA5gv-29SxdpvsvDtlSjZoNat3ZAXh1nqwBi-9AsG-HN5SFab1tYsSpenM2A

=== Sending Chat Message ===
Message ID: 4PIw86o6KyJrBx9xvN0OAwpVXwVRs8fSUK3Y0aim1CQehSoLBtOFmDoYbbCWpSnc4kRJg1SChMrhRiIP96Zkxw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

2. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

3. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'bLLCSyD4AAlGJG6DA9xy9pizC9GM__DcGL7IY6U90ig0WuPPANmHgLC4p47oQed6E_5TYQwXMAbvVPPngWJm8g',
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: 'NzdPHKMIIfEFo2-O0388epCmo44qK7dRznx8Q3a_sx8T_UPBs1KcVxF1Z5kr1RIbN8uVNAEL9okDNQP3RdZbeg',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'Ms1CyhPSdItsQ-6UKF2YXdeiVSJHR8FfiSo-Yw6cJn5jtwoWd0CdfrwO5I5AW5aivHcUaqziRGepMxWfpwkCjQ',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 10686ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: n2MeP84jyoIr06DLVd2uA36DjefoG7fbHZ3XoR3r3GuQW6jHmxJ_to9ImOHXJ822GBDGjmTgulzZ5LKumZ1_1w

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no products available in the database. The only product mentioned in the context is "ekoDB," which has a description of "High-performance database product" and a price of $99, but it doesn't seem to be listed as available. Would you like to know more details about "ekoDB" or explore something else?

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
✓ Created second session: 3DvYW-NyW3TeumsTUdqRKqI8_b6YRbC3vM0oRo8oPIOgoQG6XEkf0aLUyfOsJs3gzkw5P8vrdtWSr5pyFr3xIw
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: tl5AgUXw9dsU27owPdl6mLfXHoIZp4TqFqUCIgszFovnGiHlT09d4XUbaEfpWaQXcPP9aneMgArcJcnrJ6QKUQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product.
- **Price:** $99

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: pk1lu9HdT4uShyBbVX5m9rvJVM3zuzL3rGL0b0TR2z7vKG-wyQOu5KrF24FZGrJ0RBLB66b2qR0m2chD-MVTvQ
  Parent: tl5AgUXw9dsU27owPdl6mLfXHoIZp4TqFqUCIgszFovnGiHlT09d4XUbaEfpWaQXcPP9aneMgArcJcnrJ6QKUQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: pk1lu9HdT4uShyBbVX5m9rvJVM3zuzL3rGL0b0TR2z7vKG-wyQOu5KrF24FZGrJ0RBLB66b2qR0m2chD-MVTvQ (Untitled)
  Session 2: tl5AgUXw9dsU27owPdl6mLfXHoIZp4TqFqUCIgszFovnGiHlT09d4XUbaEfpWaQXcPP9aneMgArcJcnrJ6QKUQ (Untitled)
  Session 3: 3DvYW-NyW3TeumsTUdqRKqI8_b6YRbC3vM0oRo8oPIOgoQG6XEkf0aLUyfOsJs3gzkw5P8vrdtWSr5pyFr3xIw (Untitled)
  Session 4: 6Wh9bCQ13lb_HCio7nnE5Hr8aCUA5gv-29SxdpvsvDtlSjZoNat3ZAXh1nqwBi-9AsG-HN5SFab1tYsSpenM2A (Untitled)
  Session 5: X8y4BQWBBNZKMk4RpdETvhBcErKX00VRLLkDmYJCw_loP0dqoXD5MaZ3rwYVtdDUG-uzHCR8Nd5SvG9SjI4_Aw (Untitled)
  Session 6: mqEaN0jW1PYv4-F-nE5E0Y9pe_1D-NBvlwKr9Hqp7Gl520k7RrXGqQpjE3cinyv438pzqdTSH71F20QHVQGxDw (Untitled)
  Session 7: dKGks_vWoqc7xBDZc623Z7LgNSCJJ5QFe-FA_xx6sKDPg8-MHLIDLF0rc3NBksrUQ-nv8W3py5ltjB6qOfhwsw (Untitled)
  Session 8: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w (Untitled)
  Session 9: ukiK-AXE26Q3Zjg_6vlCTPlIKs6tPt_cZ1FJZmS2k3GLm7E9jvnL_X7O14_DOGfmoVXa1X1BaT06zCM8dy9weQ (Untitled)
  Session 10: dNGb9c_qlepdYXRMjHPAGJy-MDa4ulQLp2WDlIqDZw6P0oMFTVsKtL3Kbm7QmxX2GWwRZn8COanmCIfYs3kCBA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: pk1lu9HdT4uShyBbVX5m9rvJVM3zuzL3rGL0b0TR2z7vKG-wyQOu5KrF24FZGrJ0RBLB66b2qR0m2chD-MVTvQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'c4yBR4uNna_v8EDPiVXYQ2Q17JSPS1Cq23GLB0loEsWFfiZfVeRDouVhrAsJHIL5efJo8ZdtsZuBEHtPHVRX_w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  id: 'c4yBR4uNna_v8EDPiVXYQ2Q17JSPS1Cq23GLB0loEsWFfiZfVeRDouVhrAsJHIL5efJo8ZdtsZuBEHtPHVRX_w'
}
✓ Second upsert (insert): {
  id: 'iF-q3QyBkS8COfkoWCXQwZpN8HDrgsZT5su6keY2bh-1k713wVtu8t5-m6OBkpAk7V_rr0m0JKi94MaftJ7UPw'
}

=== Find One Operation ===
✓ Found user by email: {
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { value: true, type: 'Boolean' },
  id: 'c4yBR4uNna_v8EDPiVXYQ2Q17JSPS1Cq23GLB0loEsWFfiZfVeRDouVhrAsJHIL5efJo8ZdtsZuBEHtPHVRX_w'
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
   Inserted with ripple: {"id":"4kLH3VsKBkSPY6ETCPnY9uoSyhOqdoa8d72G_7MPd-_zNm97rAwV6a-k6HVJC8nDtivbwi26hg4K6C3vEMCGXQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"jpmaQUiGl1OTJpDOyo7inz1cPTlLLLKD1KsirwTCU8bSwBZ5dd3RCKnAph6Lsg5OXl6IEBX59qugAQiOEJo03g"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"4kLH3VsKBkSPY6ETCPnY9uoSyhOqdoa8d72G_7MPd-_zNm97rAwV6a-k6HVJC8nDtivbwi26hg4K6C3vEMCGXQ","name":{"value":"Product 1","type":"String"},"price":{"type":"Integer","value":150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"value":"Upsert Product","type":"String"},"price":{"type":"Integer","value":500},"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","name","email"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age","email","name","status","id","bio","avatar_url","created_at","user_role"]

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
    - Fields: ["password","name","email","secret_token","avatar_url","api_key","created_at","status","id","age","bio","user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","email","name"]
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
  anthropic:
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: 2s-xtl8xvcownEbU6PW8JSRpqAVtenaEoEX8ETOfr6JZJWBwq5WhGinqhuxyd_eBGFEbY6ppCchF0XcIoRsLNg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - fetch_api_user: Fetch User with Cache
  - cache_api_call: Cache External API Call
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - cache_api_call_py: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_ts: Get Active Users

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
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/ts_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
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
✓ Client created
✓ ts_route_admin → GET /api/route/users/admin
✓ ts_route_user_by_id → GET /api/route/users/:id
✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ ts_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
(node:24598) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'KL2NCQNxRYOqQLjPCd7dNDDg81U2Si3Pjjd-wMfQDOTDJoDIFydlY6SFyUtlx0W5RiNNOryTK-ch_GfH7dwEbA'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: 'KL2NCQNxRYOqQLjPCd7dNDDg81U2Si3Pjjd-wMfQDOTDJoDIFydlY6SFyUtlx0W5RiNNOryTK-ch_GfH7dwEbA',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    active: { value: true, type: 'Boolean' },
    id: 'KL2NCQNxRYOqQLjPCd7dNDDg81U2Si3Pjjd-wMfQDOTDJoDIFydlY6SFyUtlx0W5RiNNOryTK-ch_GfH7dwEbA',
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  id: 'KL2NCQNxRYOqQLjPCd7dNDDg81U2Si3Pjjd-wMfQDOTDJoDIFydlY6SFyUtlx0W5RiNNOryTK-ch_GfH7dwEbA',
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: NvnjI0I_ysYi0yeaWu9DFtL7JyhxX4s4WzfTTjETVMiUNuftb_oOl7IBxFrD3TepymatBiIErzc1vj29yg0WhQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "id": "WPqvrkImxJdJQGknbhfZE_zv7HPfMd2R_AWgTwv8FDfhndwQgMgaizHru0D74aM7v4u_IW-2BVaM0HqFXLcpUg",
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      },
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "3qPv3CEUDmu5T7on85zmtKGt3BFAZyaVEhodwHKMxfq7qiOuKsqIELqjjSv4-KCbxZs4B_7tAJfyw2uSUyK5IQ",
        "value": {
          "value": 42,
          "type": "Integer"
        }
      },
      {
        "id": "OZdVF92oGdfsvvOSD4yyE45G1-Ue61ATHNGkDBRj_10Z-cgCfY2PomLVC0mMdOXbhG8OuJzrg8y4cBsbOzDxyw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "NvnjI0I_ysYi0yeaWu9DFtL7JyhxX4s4WzfTTjETVMiUNuftb_oOl7IBxFrD3TepymatBiIErzc1vj29yg0WhQ",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      }
    ]
  },
  "messageId": "1782493035790"
}
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: qsz_P34Ljj4Uh3eHR8mIyIsEkpHhoEHgS4saRprQ3l-nQ5IZEyJAEV8CcLhwajiT4tMQjUBpjhJzwns-bQPaKQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: RG8ERv6YzCarsA_aFU0HbFTaCTcOKB3NSpkYRm-0wYLwnJnR5qQsgkt7ody6GQiRBbah9Zm5HhEZjBR34tds4g
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: -yrODI1LLa-14RH7B8MnIyiVK70u4SXkxB7T2SnVZjHdrAaRvJ__IYUOXyFdrTTREWqDofsFrsptE_lzGMnsMQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":100}}
   {"count":{"value":10,"type":"Integer"},"max_score":{"value":90,"type":"Integer"},"status":{"type":"String","value":"inactive"},"avg_score":{"type":"Float","value":50}}

📝 Example 4: Function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
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
=== Running ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 8Nsum8Xm6uYCZ8hJ_RsuW5-QAyMzMGLA1Ml9hdwGNMk0IcuQZAlFSmtwx9KUU_OaTJnRQIPKjedQpLbRyySxFA

=== List Collections ===
Total collections: 26
Sample collections: [
  'test_collection',
  'functions__ek0_testing',
  'demo_collection',
  'ttl_cache',
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
=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: DuZ4zJ3dLEFAw7BPpqDsx0XACe7b4WQjZ1HCk0pkvTxB2sooKi6edIcxU9DniOQ_2dVkdpAFfHborqmIMT8jhA
Created Bob: $500 - ID: y5NOjaqmweZqOSEihC4C_3nfhYIa_j01X6sehqc4LcNFuzS0yVykDWsFhDj4d5L3HBoyJOP9cQq431UPY4uB7A

=== Example 1: Begin Transaction ===
Transaction ID: 4a887cd9-1e46-48b3-96d3-78282a4e9407

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: 5b8a2724-54d2-468e-8101-feaa9a86b9d5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: Yv23M3AY1T5MJnac3AGyTcKRXhod3vmufAJKFYX982ijxEiBwBwdwyUE2EgBbb_n0UZzrk-s7UHreJwh-cRPsA

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 8k1LPAUdwg6yIl79XuNtetev-QFyOts67OBgW-9Yhnc2XeAk6MD3G11SHrqI0XC9BqTaOc_HAfoFo51bUEKAQw

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: azty31JMeLL5o6S8MwZMSqBo-pd19A_VxQGylrSywa4mFLax7SD6yyYPaPl009raWhseGGT8mqbNTQE2wi4cCg

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: D80MC86-Bpuh2eFGoExn5rLvmymKLWHL2eCGC1wSWMYkfGgZMCQZdrTG1uRZ3M2uLvzgiNba4QmfqtLl09oDpA

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: Yv23M3AY1T5MJnac3AGy...
   ✅ Deleted function: 8k1LPAUdwg6yIl79XuNt...
   ✅ Deleted function: azty31JMeLL5o6S8MwZM...
   ✅ Deleted function: D80MC86-Bpuh2eFGoExn...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = r9d0Z894Y5emXbBLDk_-j-lEWEIHQjn6UXYe060phO8jnWH4yV-QxLs63GabxvNHNGTx2IWbS5niVNdMXBMVlA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(r9d0Z894Y5emXbBLDk_-j-lEWEIHQjn6UXYe060phO8jnWH4yV-QxLs63GabxvNHNGTx2IWbS5niVNdMXBMVlA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(r9d0Z894Y5emXbBLDk_-j-lEWEIHQjn6UXYe060phO8jnWH4yV-QxLs63GabxvNHNGTx2IWbS5niVNdMXBMVlA)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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
=== Running ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = NQtswmgYzBQblld47rd2l0wurA_VN1SIQU4h8rhmVdZs9IkNrQOS22er7neffQE4-WsS6Y7UjHmNptnBH96gBg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(NQtswmgYzBQblld47rd2l0wurA_VN1SIQU4h8rhmVdZs9IkNrQOS22er7neffQE4-WsS6Y7UjHmNptnBH96gBg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(NQtswmgYzBQblld47rd2l0wurA_VN1SIQU4h8rhmVdZs9IkNrQOS22er7neffQE4-WsS6Y7UjHmNptnBH96gBg)
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
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.9',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.22.0 prepare
> npm run build


> @ekodb/ekodb-client@0.22.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.22.0 build
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
  id: 'vbk0tjjbiJKVvpYT7YAAdyJKaC-YI8tZI44fCtcXO3bV2W0AzyruLWpDn2-sD79S-roqThHcHku50MWQs8zugw'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  id: 'vbk0tjjbiJKVvpYT7YAAdyJKaC-YI8tZI44fCtcXO3bV2W0AzyruLWpDn2-sD79S-roqThHcHku50MWQs8zugw'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  id: 'vbk0tjjbiJKVvpYT7YAAdyJKaC-YI8tZI44fCtcXO3bV2W0AzyruLWpDn2-sD79S-roqThHcHku50MWQs8zugw',
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: oqMY-lZeeERfOStdHw4w9zhLsS0lGU014BMD-0SZnjhKnL-Ye3LdiBFZgm20QS2YM_0FAKck-rFru9j30Nqk0Q

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
Collection created with first record: uGKLW98b5x5RtMTpcTKGhlzZn3t6tvcYhA9L9kC3wGz0ou4q9mJKHRJ9b4ZkmJVo9whF09qs4ZJriywF_iO0cQ

=== List Collections ===
Total collections: 25
Sample collections: test_collection,functions__ek0_testing,ttl_cache,chat_messages__ek0_testing,chat_configurations__ek0_testing

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
Total keys in store: 13

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Mae0cuXoLb6BE9rbGTvcfLSuFsSzzfLwbn0JpW60iG1iMGkWxHVFSVPvlMpguyso3RCknfgqqRIQrTvlOSKQtQ
Created Bob: $500 - ID: zY0w-973bRXAhWRyaFrwCnJ-QCnRD0GakMgobgWSfvMqHI6Son_0WlML9ZMQDjGEtqKM_mHh1n-eXem5dHnWMQ

=== Example 1: Begin Transaction ===
Transaction ID: 6631afa7-de0e-4a32-9efe-47c277bdb0c7

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 154f31aa-d3ef-425a-9745-b213a77e4c8c
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.769, Matched: 
  2. Score: 0.760, Matched: 
  3. Score: 0.730, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.492, Matched: title, content
  2. Score: 0.904, Matched: content, title
  3. Score: 0.308, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

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
✓ Inserted document: ka3mL2CPHKJhJlQdhp7Qv5e74YUWzHyFQ2HMmw46GmZST-SuO_mLdx_UVxaPgvKiI4dVZzlneGrFAS2vLIwWvw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: -K4US0KfDm_hOlWGJe2mtltGI6DQf-2WIp6GYe9NT6Pd7KK_HHzfJA_mqRwHy8ZjYAbdlUvpL4Tbyv4AQR65rA

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
✓ Inserted document with TTL: 2svFv5ZVYeVNDtGDoRySczOSpwnzBjGPFFxGS5k-jgJKMe0ysC4wmgcBqK4aoCIRPd77PTHfEaSZOLsJq0f3jg

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
✓ Edge cache script created: Qm38EsVKoVxpwi1JfoKHapf9qkRpcp1cKQz-oyI6hPPdfceLRxB7XK__9fWZ44zYfsrWaAbmzxjOaJkGJFkV4g

Call 1: Cache miss (fetches from API)
Response time: 448ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "latitude": 40.710335,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "time": "2026-06-26T16:45",
            "temperature_2m": 25.9
          },
          "generationtime_ms": 0.01633167266845703,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "longitude": -73.99308
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
Response time: 6ms (74.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "latitude": 40.710335,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "time": "2026-06-26T16:45",
            "temperature_2m": 25.9
          },
          "generationtime_ms": 0.01633167266845703,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "longitude": -73.99308
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: BtvQOgJmxcEI3s862PBwWuo7Bz8vnH5E5s0N54HwvkVGzlLqByFBFaXDKnL7P7tZPYS6-QgJGyoXsVzvUFVNGQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"},"status":{"type":"String","value":"active"}}
   {"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50},"count":{"value":5,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 17
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

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

ℹ️  Function 'fetch_user' already existed — updated instead
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

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 75ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Ro...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net...
   🚀 Cache speedup: 18.8x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 5Itb0KezCE7bzkhaf0UPyT8QUgNfaQX650Va4A0TxnRIFoKi5-be6z_p1VcUI8ehWrjRfWfyWRFy5X0Pf2H-xQ
📊 Found 2 product groups
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":474}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's Function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: y9-QZeYiyUWCSZbalpPeIiDzD91xbFzzn3IfYug2UMahQAGmV4XU_fvsJ5FAVpmluN6OONpvKrFMdi8g5OkF1A
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: ZDQwTHhi7KN4HlcFmRE4GyvTm9o705aiRCE1MtZzQa9jO1JVUNX28nOeUMhs6EOR0j57dXV5ik5SNhwCj-r4Pw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: DvvdBodEi7Rq0tIeTl9ULCmKCR7QiM7UV-hm_QcXT1-P-nEz-IemcjD_qmPfzGCr09A_sfoIcpWTDPZeF45l-A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: a9hf6EeP3ZHIzts6SDR8Btq8j96wXYHubV4M13EYqWzJyAO9JuNWcPMrPCD8x7MLXjeZzZGaqfA4IAayfkmLqg
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
🚀 ekoDB CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 5 - Score: 50
   2. User 8 - Score: 80
   3. User 7 - Score: 70
   4. User 9 - Score: 90
   5. User 4 - Score: 40
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Getting Started with ekoDB
   2. Database Design Principles
   3. Natural Language Processing
   4. Vector Databases Explained
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Database Design Principles
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
🚀 ekoDB Advanced Functions Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Function saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Function saved
📊 Category analysis (2 categories):
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. USB-C Cable - $19 (⭐4.3)
   2. Desk Chair - $349 (⭐4.2)
   3. Wireless Mouse - $29 (⭐4.5)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples completed!
🚀 ekoDB AI Functions Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   1. Precision: Vector data models are beneficial when dealing with precise geography or boundaries.

2. Flexibility: They can represent different types of geographical features such as points, lines, and polygons.

3. Scalability: Vector databases permit efficient scaling and zooming, allowing users to view data at various levels of detail.

4. Smaller Size: Vector databases take up less space, making them more efficient for storage.

5. Clear Topology: They can clearly represent topological features such as connections and adjacency of polygons.

6. Attribute Storage: In vector databases, each object can have an attribute associated with it, allowing more in-depth analysis.

7. Realism: Vector data models can effectively represent real-world features like roads, rivers, and buildings more accurately.

8. Computational Efficiency: Certain spatial analyses can be processed more efficiently using vector data models.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 0D vector
   2. "Advanced Query Patterns" - 0D vector
   3. "Draft Article" - 0D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Function saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (I6IjJDOSc39l8hgGK7L2RCIjLL5y1sHib2XfH1D7u1ZAWfwV6_XInsJR3rfD0lxz2ulbPwrLPAKa6NMNj9vK_w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "email": "Sincere@april.biz",
          "username": "Bret",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          }
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
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
✓ Created native SWR script: github_user_native (rAN9KoJSGTAEfDlulENE7L2KYQdj4RgexsabfX8RxN48-ANSWnFcIH9kPpli5qpcOrhGk6lNVIkn_0qGq22zYg)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 2.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (0luTiqqZrHkCiZVyDATbzR72SR0vQduj2tZUSfzA3m8VtEK_hcvK-tGz0e73LsW7E0DBd7vCMN4J-RRE_CO87Q)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (PQN0Lni56joleo8RJ9vpHfsjc5VPQQ0W-dqkSmQ6njlXD7YBRww3QAZoCWWjcp2GxzX45_IRmFgHYwG-OuaVLw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (6F1u9hXGPAWGNqPywS52z7vGSpNbm3lgULQ4zvP-7r1SshgbD2YNBfe5Er-MOyfRNdPHQBcyvjegVpCMt8QnTg)
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
✓ Deleted 4 test functions

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: FmiXG5pAKetUw81PAu4pJbahDNfvgqzQmSmRtc52_sXImga31mnJ39WXR0-kKlZ6pwhlOfgeQaLgMF3MdRZP8Q

=== Sending Chat Message ===
Message ID: a2rbL2RymN4CxmAMfcwbUfSP1DiMZ6TG9RTEy79JfuGwSUGgSaVdm5L-JKmqyQDXhX8a3sc1KeiQh1ybZqB0cg

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'HJ9TvzhjZIl-pLM3RRsWJdocH2cdERAbCJsuAN3M9cAwdtLOgBfqaceCC2aWfRB_PLdv7FWa55mlhZuplnRmcg',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    id: 'iK3IkcFiIZMnMgSVYXNYLZKKbCJCYWd8AyMC-nLOcg4n_keu0BJ3HBHwazugQ8L1jgEW7Eqhg4WKfGMleklE0g',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'mwoi7Oy9ekAO9ECOhav5BPP-Vluq30OIhS2pT2CTGv8FJRGv0lxgBmiDVaYs4RROyF2nwWOlNHp1vzzYMQxhlw',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 5922ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1RqXf6uKMyTD13GQD4Oxzbnx_3VR77caiXUQi3aZZt_CJANR27xW4xBVBAEr5FuQ6gQ9F15mJVSqT8y_z5Jg4g

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or additional products, just let me know!

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
✓ Created second session: 7GIO44WfERGmebZt2qQJh5w9k2caEHH5Dbhxx8kgQ5OWHTcQwsyhkeZN0RGcdaH-e0reKiAqFhwOT9SrnLsKlQ
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fH-aG9uTpvzYoNzRyMn7KqpX_NfRr5CgBrSxJ9ffSj5BL1RQf4gkyHFArNecV9BqBII2s-rB7NplHL-Zhu76aQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

Would you like more information or assistance with anything else?

✓ Message 2 sent
  Response: The price of the product ekoDB is $99. Would you like to know more about it or anything else?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 05q_suoEWD2DgHvJUHYg6D3sADYe42xktB301kKNron_B8rvzLHbvMKKiR_Ek2c1b05n8pO2IULcHJxQmLnWYw
  Parent: fH-aG9uTpvzYoNzRyMn7KqpX_NfRr5CgBrSxJ9ffSj5BL1RQf4gkyHFArNecV9BqBII2s-rB7NplHL-Zhu76aQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 05q_suoEWD2DgHvJUHYg6D3sADYe42xktB301kKNron_B8rvzLHbvMKKiR_Ek2c1b05n8pO2IULcHJxQmLnWYw (Untitled)
  Session 2: fH-aG9uTpvzYoNzRyMn7KqpX_NfRr5CgBrSxJ9ffSj5BL1RQf4gkyHFArNecV9BqBII2s-rB7NplHL-Zhu76aQ (Untitled)
  Session 3: 7GIO44WfERGmebZt2qQJh5w9k2caEHH5Dbhxx8kgQ5OWHTcQwsyhkeZN0RGcdaH-e0reKiAqFhwOT9SrnLsKlQ (Untitled)
  Session 4: FmiXG5pAKetUw81PAu4pJbahDNfvgqzQmSmRtc52_sXImga31mnJ39WXR0-kKlZ6pwhlOfgeQaLgMF3MdRZP8Q (Untitled)
  Session 5: tl5AgUXw9dsU27owPdl6mLfXHoIZp4TqFqUCIgszFovnGiHlT09d4XUbaEfpWaQXcPP9aneMgArcJcnrJ6QKUQ (Untitled)
  Session 6: 3DvYW-NyW3TeumsTUdqRKqI8_b6YRbC3vM0oRo8oPIOgoQG6XEkf0aLUyfOsJs3gzkw5P8vrdtWSr5pyFr3xIw (Untitled)
  Session 7: 6Wh9bCQ13lb_HCio7nnE5Hr8aCUA5gv-29SxdpvsvDtlSjZoNat3ZAXh1nqwBi-9AsG-HN5SFab1tYsSpenM2A (Untitled)
  Session 8: X8y4BQWBBNZKMk4RpdETvhBcErKX00VRLLkDmYJCw_loP0dqoXD5MaZ3rwYVtdDUG-uzHCR8Nd5SvG9SjI4_Aw (Untitled)
  Session 9: mqEaN0jW1PYv4-F-nE5E0Y9pe_1D-NBvlwKr9Hqp7Gl520k7RrXGqQpjE3cinyv438pzqdTSH71F20QHVQGxDw (Untitled)
  Session 10: dKGks_vWoqc7xBDZc623Z7LgNSCJJ5QFe-FA_xx6sKDPg8-MHLIDLF0rc3NBksrUQ-nv8W3py5ltjB6qOfhwsw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 05q_suoEWD2DgHvJUHYg6D3sADYe42xktB301kKNron_B8rvzLHbvMKKiR_Ek2c1b05n8pO2IULcHJxQmLnWYw

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

BUILD SUCCESSFUL in 4s
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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw)})
  User ID: 6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw

=== Read ===
✓ Found user by ID: EkoRecord(fields={id=StringValue(value=6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Fri Jun 26 12:58:59 EDT 2026), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Fri Jun 26 12:58:59 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: id, user_id, categories, created_at, data, tags, metadata, age, active, email, price, name, embedding

=== Update ===
✓ Updated user: EkoRecord(fields={metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), created_at=ObjectValue(value={value=StringValue(value=Fri Jun 26 12:58:59 EDT 2026), type=StringValue(value=String)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), id=StringValue(value=6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), id=StringValue(value=6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Fri Jun 26 12:58:59 EDT 2026)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Delete ===
✓ Deleted user with ID: 6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"data":[{"status":{"value":"active","type":"String"},"id":"RGdPR50WpX51aHVuhUWM9Jsz0aKW4ebgzdwRrzoG_kPqbqrQrMUGeYOmwyioEYHubnL0XFf8ck4XRA_orzBJLg","name":{"type":"String","value":"Test User"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: g3cG_8ginyVZ-cJDX-4c1asW4418FJpmDkn4E9pMf9kzO0Orx3A8TTDeb-tZ0lGLU5czE-uonDE0EyqZWvF42Q, a-2tvgYMv1aEqsWNynnd1o7mU0pmvuXWchjDRe--aSAsJzbRdIdBzg2gOigNmr7d_zKsH30Y56i7kzDAGxQRAw, unCVaQhr0F7o3D42iEwVw-0qIGqdmg2XTuC55QFvdzo6Mp9sL-QF0ifxmy3OzveV4XmEc16qMS0X6f0DImmrgg...

=== Batch Update ===
✓ Updated 3 records

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

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
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 29 collections
  - test_collection
  - schema_employees_client_js
  - functions__ek0_testing
  - ttl_cache
  - chat_messages__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-06-26T16:59:18.120131Z","last_modified":"2026-06-26T16:59:18.120132Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1782493162905}
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
✓ Total keys in store: 18

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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hxOX87hT_LBxYF7ORxAT8QQ9-hqdaOLNwSTvubFSoPa9eP5N9ns0KdyYS3vH_wDsdJVUWuOlDYOSY_Hvaq8Zrg
Created Bob: $500 - ID: cNMhqWnNryFpwxx7Qcq3BMBP-mCUjdbBKR5G-tr5Z9m-XjSo7F_j0xwh_QveKsw6yZCeJ43iYHkJlrGSRGCsXQ

=== Example 1: Begin Transaction ===
Transaction ID: ab73f407-2a2b-497f-9919-4ae6c9ec5111

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9a2c2b52-127a-4243-ba62-92bb0c00a1de
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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"title":{"value":"Rust Programming","type":"String"},"tags":{"type":"String","value":"programming,rust,tutorial"},"category":{"value":"programming","type":"String"},"views":{"type":"Integer","value":259},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"id":"D3XmqmHTNNqj1n9DSkGXmGaMco76FKLdvOjFdFyvHMZpUFsxZnj8Yp0LW67J9NbjtIL34090rHsRhGP5bYGvgA"},"score":26.400000000000002,"matched_fields":["tags","description","category","title"]},{"record":{"description":{"value":"Build modern web applications using JavaScript, React, and Node.js.","type":"String"},"views":{"type":"Integer","value":636},"category":{"type":"String","value":"programming"},"id":"T7W0zuj6NH0Js2kG2MMdElnO4qJYWy6PgO8fFZSY0WbqAInOSEaOtPkCEWtTkCBtZXCpuz94OuIpL9wq5ASIzw","title":{"value":"JavaScript Web Development","type":"String"},"tags":{"value":"programming,javascript,web","type":"String"}},"score":13.200000000000001,"matched_fields":["category","tags"]},{"record":{"title":{"value":"Python for Data Science","type":"String"},"views":{"type":"Integer","value":918},"tags":{"type":"String","value":"programming,python,data-science"},"category":{"type":"String","value":"programming"},"id":"tfXAXOvJBtY0dmarNkyP2ltSmKkFI6x_YpuZatsSUZi7SDOn0qBhwYYEYttxN7FUrhtk4ius5TaZnsiUU74s8w","description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."}},"score":13.200000000000001,"matched_fields":["tags","category"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"views":{"type":"Integer","value":610},"description":{"value":"Introduction to machine learning algorithms and neural networks.","type":"String"},"tags":{"type":"String","value":"ai,machine-learning,python"},"title":{"type":"String","value":"Machine Learning Basics"},"category":{"type":"String","value":"ai"},"id":"XaIqDYE_i9e7YF_8fWvykjFDydXV4PD3hp_4_VmOaVMQvDcn4fThofALMVa_taKxRwwmMgsoUhI9a1jjk5KIwQ"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"id":"tfXAXOvJBtY0dmarNkyP2ltSmKkFI6x_YpuZatsSUZi7SDOn0qBhwYYEYttxN7FUrhtk4ius5TaZnsiUU74s8w","title":{"type":"String","value":"Python for Data Science"},"category":{"type":"String","value":"programming"},"views":{"value":918,"type":"Integer"},"tags":{"type":"String","value":"programming,python,data-science"}},"score":1.0,"matched_fields":["description"]},{"record":{"id":"D3XmqmHTNNqj1n9DSkGXmGaMco76FKLdvOjFdFyvHMZpUFsxZnj8Yp0LW67J9NbjtIL34090rHsRhGP5bYGvgA","description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"category":{"type":"String","value":"programming"},"title":{"value":"Rust Programming","type":"String"},"tags":{"type":"String","value":"programming,rust,tutorial"},"views":{"value":259,"type":"Integer"}},"score":0.5,"matched_fields":["description"]},{"record":{"id":"Fj8WGrfO47Og4Jqgx0BilxEtOjO0nd4QUja_l9nA9Nmzp6df40G1gnWE5FLpYPQbX537RdaUvw5_3O9Mh5JKtw","title":{"type":"String","value":"Database Design"},"category":{"type":"String","value":"database"},"views":{"value":303,"type":"Integer"},"tags":{"value":"database,design,sql","type":"String"},"description":{"type":"String","value":"Learn database design principles, normalization, and query optimization."}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Search with a metadata pre-filter (category = programming) ===
✓ Found results in category 'programming' (database/ai excluded)
  {"results":[{"record":{"views":{"value":259,"type":"Integer"},"tags":{"type":"String","value":"programming,rust,tutorial"},"title":{"value":"Rust Programming","type":"String"},"category":{"value":"programming","type":"String"},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"id":"D3XmqmHTNNqj1n9DSkGXmGaMco76FKLdvOjFdFyvHMZpUFsxZnj8Yp0LW67J9NbjtIL34090rHsRhGP5bYGvgA"},"score":6.6000000000000005,"matched_fields":["description"]},{"record":{"id":"tfXAXOvJBtY0dmarNkyP2ltSmKkFI6x_YpuZatsSUZi7SDOn0qBhwYYEYttxN7FUrhtk4ius5TaZnsiUU74s8w","description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"tags":{"type":"String","value":"programming,python,data-science"},"views":{"type":"Integer","value":918},"category":{"type":"String","value":"programming"},"title":{"type":"String","value":"Python for Data Science"}},"score":6.6000000000000005,"matched_fields":["description"]}],"total":2,"execution_time_ms":0}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=k4Go3QRI_Ytyu6gHmlmKld83nBsyyWTl_xx9wXj9ow3cPlz84lFZEsgITtokInE715kj0mQMP4-3P8Xhxc59BA)
✓ Inserted user 2: StringValue(value=fr2jwunSKKbtoeHmVvSBkP05rzWdGl0gg1BBO7XveCJs_sAti07M8NWHU-QfjBhF6cwvBIyCXqcF2uEpu9TL4w)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 30
  Sample: [test_collection, schema_employees_client_js, functions__ek0_testing, ttl_cache, chat_messages__ek0_testing]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

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
  Document ID: a5EUNZMifPZmAcFKF2CqayUzxHTfBZZ8JB6QwizylSynvd8HboL7qXGPGBCoc0Jaj8y9lPV66h5xWiyS5Oyd7A

=== Verify Document Exists ===
✓ Document found: user_id, created_at, session_id, id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: skgfn404dGX5eepH11kjLSN6Tq5jVYqt_mxYK_GF7iyTWyT64gUGStfuVAONWX067FqYGZfWNAZzRTnCB9vFnw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: id, cache_key, value, ttl

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
✓ Inserted document with TTL: StringValue(value=SkvLPHzTnwFA5CbQC2QoTPWcfg94oCz1eDDEq1YIJQ_ToKyUloBDj4C_ZA7c9Sky8fpvrSD_DaY98aR9kMBjVg)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"id":"SkvLPHzTnwFA5CbQC2QoTPWcfg94oCz1eDDEq1YIJQ_ToKyUloBDj4C_ZA7c9Sky8fpvrSD_DaY98aR9kMBjVg","value":{"type":"Integer","value":42},"name":{"value":"WebSocket TTL Test","type":"String"},"created_at":{"type":"Integer","value":1782493200020},"ttl":"2026-06-26T18:00:00.147957Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: uIKHXfKUvZzhvMKJLw0Ax-lnnvklTuIf1BH3uyL2yridMlntbX4rLt889qpeEt0pkmTicRBc-NJP5h3kLo1h5g

Call 1: Cache lookup
Found 1 cached entries
Response time: 16ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 8ms

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
🚀 ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: S6pH7HIAHKREfPI_pjyHhueeW3gB2EpWC1KcRozZa7dpjJnzfpi5s94TqOkywbhfsCZxbc5DakJRNnCAwtjOvg
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 3 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"value":60.0,"type":"Float"},"status":{"value":"null","type":"String"}}
   {"avg_score":{"type":"Float","value":50.0},"status":{"type":"String","value":"inactive"},"count":{"value":5,"type":"Integer"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60.0,"type":"Float"},"status":{"type":"String","value":"active"}}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

ℹ️  Function 'count_users' already existed — updated instead
✅ Count function saved
📊 Total user count: {"type":"Integer","value":15}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

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
🚀 ekoDB Kotlin Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 479ZWPfyBf-ie73TEVF_qBRnliDkfF1hf7Naw5VSESX1ukJb_mIWMBCv3BEC1obXsTAPvJusi8MPPvhL9ARjpw
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)

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
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within functions
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=Kby30zDLN_1k4hevxPDFO5s1cMKB2YntlVFlnOnpmeLIokqOY-VKX5Ax-ZnqREHP4yUp9IGJ3Qhn--jyJxTv4w)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: KF22Tpy8Rkfzf-vf4K3TkttCbUOWigfMUqYP7jfe6j1b-iOBRQcRJlO8KEXQWK5nJPE0GV-A2rY2M2FEjewNvw
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: mBzJISR9AZ7WcoN1m4koiEIELvBd4lgDzyT-jApNNoZR1LVzWcbd4wyabjQ263izRNfvls77ChhS8KwWxrtWWQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: tfZhtxrOK9Uwx0cqxDs0tnT0VaxEc3cXQ0JpSszgVvaUrIvyXuqVX2fwHgK_BN0lj7rsUoXLjA3D4tSlOfJi4Q
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
   ✅ FunctionStageConfig.Kv* classes work within functions

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

Step 2: Create SWR cache lookup function
✓ Created SWR function: swr_cache_lookup_kt (DeQvnVq9h6qFbIf9Xs0QHp3oVbirbLAb2Go-1ZBLGpJk4jxRH8n8iuBfUG23kGaZroRl9iG00WTNzUD5IyjYfg)

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
✓ Created native SWR function: github_user_native (WRjhuDs220Uy5qQUN8FBdDlaONq1dVKNpqd-ptMgCzIrQP5MBm_LOWZZrG6ke1TBGsYvaoqGJxlSleoUN6XbJg)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 3.2x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit (mQkPu9QANjfTcvs6mzfIItTYAthxUewh43EHNFMzQORmPbQidRw2CSTjzEsP6GKtsdKssARnkrNLDvTJNvKCIw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (6BGGUXOiMWJNVqsjxA5ummyRyEJbR_D8CoW-Ji7eLoyb0MqMDTthohKCVUImWJDYMZyLGahpNWf4uEJRicZWfA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache (4m-folNqmMUKJVy6STYCp0DAoD4t5RRJ0ldX4qxBm6TFmAwxv5s7nLVWMhQt1-m3JDYui4VtcDefWMm0LfsM-g)
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
✓ Deleted 4 test functions

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
🚀 ekoDB Kotlin Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"avg_price":{"value":367.0,"type":"Float"},"category":{"value":"Electronics","type":"String"},"count":{"value":5,"type":"Integer"}}
   {"avg_price":{"type":"Float","value":365.6666666666667},"count":{"type":"Integer","value":3},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

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
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   {"tokens_used":{"type":"Integer","value":265},"response":{"value":"Vector databases offer several benefits, including:\n\n1. **Efficient Similarity Search**: They enable fast retrieval of similar items using vector embeddings, which is ideal for applications like recommendation systems and image search.\n\n2. **Scalability**: Designed to handle large volumes of high-dimensional data, vector databases can efficiently scale with growing datasets.\n\n3. **High Performance**: Optimized for similarity searches and nearest neighbor queries, providing quick responses even for complex queries.\n\n4. **Semantic Understanding**: Capable of processing and understanding nuanced relationships in data, enhancing tasks like natural language processing and image recognition.\n\n5. **Support for Machine Learning**: They facilitate the integration of ML models, allowing for easy storage and querying of model outputs.\n\n6. **Multimodal Data Handling**: Can efficiently manage different types of data, such as text, audio, and images, by representing them in vector form.\n\n7. **Flexible Data Ingestion**: Support various methods for data input, making it easier to update and manage the database.\n\n8. **Rich Querying Capabilities**: Allow for advanced querying options based on vector similarity, improving user experience and data insight.","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed function saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI function examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

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
🚀 ekoDB Kotlin CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   {"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":7}}
   {"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

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
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
   2. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   3. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   4. {"value":"Introduction to Machine Learning","type":"String"} ({"type":"String","value":"AI"})
   5. {"type":"String","value":"Vector Databases Explained"} ({"value":"Database","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"AI"},"count":{"type":"Integer","value":2}}
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: LwPPlE9e3hVrs1nmTWL6mfEV2IBr9QQSMFS4z0IF7OeHxy15o2MANZvZVt353_PS6JoNt--5vV91UO9WbcoAZg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "R-9YFKHrg5NzSTECoBKJRLG4ypGBfp7eBDjoYmK7Us80ApIL7shMjW1tQf9N39XZMb3zBaTuucoBBo0luxql0w"
  Responses: ["ekoDB is a high-performance database that integrates intelligent caching, real-time capabilities, and AI functionality. It is designed to support various features that enhance the user experience and application performance.\n\n### Key Features of ekoDB:\n1. **AI Chat Integration**: The chat feature allows users to query the database using natural language, providing AI-powered responses with relevant context.\n   \n2. **Search Capabilities**:\n   - **Full-Text Search**: Utilizes keyword matching to retrieve relevant content.\n   - **Vector Search**: Leverages semantic search through embeddings to find semantically relevant records, even without shared keywords.\n   - **Hybrid Search**: Combines both full-text and vector search capabilities, automatically retrieving context to improve search results.\n\n3. **Performance Optimizations**: ekoDB includes mechanisms for intelligent caching and is optimized for high-performance operations to ensure quick data retrieval and processing.\n\nThese features make ekoDB a versatile choice for applications requiring efficient data handling and integration of AI technologies."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 36s
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
✓ Created session: TlQi_VN6sI1-Y3yna1MFn3ZrU9Le0WbHOWJGgvh9PJMbHN8JdmYnoZQ-XMcGpCivmRSYsPX5FFMHT5llfQQyYA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99\n\nIf you have any further questions or need more details, feel free to ask!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: 9kV5S1XMdel6NsBju3nN2zarEmDQMLtQjN7EkWTyJ_w2AJdCKGx-ZoSH2yZ-xPDIwq1ZA7uTgDFQbJyjDtAusw

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 19s
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
✓ Created session: OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you have any other questions or need more information, feel free to ask!"]

✓ Message 2 sent
  Responses: ["The price of the product ekoDB is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-06-26T17:02:12.211150+00:00"},"id":"z9ySp70h9BfEZU4ZHR90s9pC2lvSe1x4qlD2nzkCuI7QvtlnVSnP26YAYxGn9p-cCIHmsB_PbsqYmbjiOm-Xcw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":65,"prompt_tokens":6740,"total_tokens":6805}},"updated_at":{"type":"DateTime","value":"2026-06-26T17:02:12.211150+00:00"}},{"chat_id":{"type":"String","value":"OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you have any other questions or need more information, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-06-26T17:02:12.229263+00:00"},"id":"cb5mSqxY5pqTvx8LBis3Wb6AMOWVEcOuKOcsZBkNbSkwSFeZ8PNner8Qob7QAaVuAu4P0py-K22pohyNcXACSg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":65,"prompt_tokens":6740,"total_tokens":6805}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example"},"id":"call_weqo62hudAP1XaZJFzXzlCNQ","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_weqo62hudAP1XaZJFzXzlCNQ","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-06-26T17:02:12.229263+00:00"}},{"chat_id":{"type":"String","value":"OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-06-26T17:02:19.225806+00:00"},"id":"HVwQMFU3q-JCf2rs8ea5e-F2Yl4kJVDy4QB7l1tgzNAt_EtDwjD0-nvVZ8rg0P50RjqW_9Opgj8vIIEp1Ffg7g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3386,"total_tokens":3395}},"updated_at":{"type":"DateTime","value":"2026-06-26T17:02:19.225806+00:00"}},{"chat_id":{"type":"String","value":"OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow"},"content":{"type":"String","value":"The price of the product ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-06-26T17:02:19.260459+00:00"},"id":"0-kO1m4YEXMPb876qNu7xTHzT_gBt2y16WlcLIEzMHay2tKXuqMHrBJN3qLr-rSBi5U1xADOwqDqBNCX2OcuHw","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3386,"total_tokens":3395}},"updated_at":{"type":"DateTime","value":"2026-06-26T17:02:19.260459+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: -as2Cj_AyAuVdvZ83KNS6ZzQAIcVT7Vv23kVOV5TMNId3yzMNpm00mxVd0NfE9yUkICJ_rY3F-hJfWfRyYTuRA

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 24s
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
  anthropic:
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get OpenAI Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

=== Chat Models Example Complete ===

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
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: 3QRfA-G2ex485sl0Rsg9lOMwlTSkpw40oGqt_oFbPkBDb8wYoXjyvNOHLxzgWuxU1res6FTdfwfxrPq3JUYi1Q

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "fetch_user": "Fetch user by code"
  - "fetch_api_user": "Fetch User with Cache"
  - "fetch_slim_user": "Validate and slim down user"
  - "cache_api_call_js": "Cache External API Call"
  - "cache_api_call": "Cache External API Call"

=== List User Functions by Tag ===
Found 11 user functions with 'kotlin' tag:
  - "fetch_user"
  - "fetch_api_user"
  - "fetch_slim_user"
  - "cache_api_call_js"
  - "cache_api_call"
  - "get_active_users_kt"
  - "fetch_product_with_reviews"
  - "swr_user"
  - "validate_user"
  - "cache_api_call_py"
  - "fetch_and_store_user"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=fmUocBmz-eux3-UCP3w9_YPLMpwOctZC41EpH5B70hrMuJZDduHcRBfIDUS7div4CQCqGGJ59F8BfZW22GTvJQ)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=fmUocBmz-eux3-UCP3w9_YPLMpwOctZC41EpH5B70hrMuJZDduHcRBfIDUS7div4CQCqGGJ59F8BfZW22GTvJQ), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=bNTIz9bk2eEWT2AqeYRgoBfa5E1pZK0WttAWxSpQcVcTFODKOl4QgxYKSDRWZWOW4m25P8Y8OVJPQCrsNQKOLA)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=fmUocBmz-eux3-UCP3w9_YPLMpwOctZC41EpH5B70hrMuJZDduHcRBfIDUS7div4CQCqGGJ59F8BfZW22GTvJQ), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})
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

BUILD SUCCESSFUL in 11s
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=qgTG9Ua8tObf4nNzuA7Z22j6c4X4qIzoEWat94WuemMGS_vbhZECgb0sjvOeCZLwfuMhkDBirhoZDRU9R1nBdw)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=XcrADqX53hEydI1YSkmDDxNCBrMNV1NxtpSegYHo0qQeRIA9WthhLQNpWl-Hp2S2dwn8AiwOFTdJu4itbBvp3Q)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), id=StringValue(value=qgTG9Ua8tObf4nNzuA7Z22j6c4X4qIzoEWat94WuemMGS_vbhZECgb0sjvOeCZLwfuMhkDBirhoZDRU9R1nBdw)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=qgTG9Ua8tObf4nNzuA7Z22j6c4X4qIzoEWat94WuemMGS_vbhZECgb0sjvOeCZLwfuMhkDBirhoZDRU9R1nBdw), price=ObjectValue(value={value=IntegerValue(value=500), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)})})

✅ All bypass_ripple operations completed successfully!

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
✓ kt_users_register saved
✓ kt_users_login saved
✓ kt_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/kt_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions

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
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions

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
✓ Client created
✓ kt_route_admin saved
✓ kt_route_user_by_id saved
✓ kt_route_user_posts saved
✓ kt_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: bwC3lLS379PlMeFvV1-tTNAXwxsaFH3hCxw_Ar-1BvwBehoyaufMjmWiG7qxLxiNsUXvb3qgKJdMt7LsQGbo6g
Created Bob: $500 - ID: laaZfryHZl-yyZxJZueQDZ-CL4V6l4By9Vn0onvLJdulOxOWPexep1U8ZMH_PYu4k_A2cSeSGfmuc2O3IUvgaQ

=== Example 1: Begin Transaction ===
Transaction ID: edd396fb-07b6-4392-9942-c347b8014737

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 212db9b0-db9c-4279-96e8-427aa565bf7f
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mAll Kotlin integration tests complete![0m
🐍 [36mBuilding Python client package...[0m
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.82s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.22.0
    Uninstalling ekodb_client-0.22.0:
      Successfully uninstalled ekodb_client-0.22.0
Successfully installed ekodb-client-0.22.0
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m
✓ Rust client built

✓ Python client built and installed into .venv (via build-python-client prerequisite)

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
    ✓ Generated embedding: 1536 dimensions in 413.999041ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 275.963125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 283.052958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 481.219583ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 5.954432708s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 504.697083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 234.45125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 229.576834ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 324.24175ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 4.502987541s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 307.708375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 279.320167ms
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
    ✓ Generated embedding: 1536 dimensions in 275.605ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 94.105042ms

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

Writing memory-safe and high-performance database code involves several principles and best practices. Here’s a comprehensive guide:

### Memory Safety

1. **Use Safe Programming Languages:**
   - Prefer languages that enforce memory safety, like Rust, Swift, or languages with garbage collection (e.g., Java, C#). These languages manage memory allocation and deallocation automatically.

2. **Check for NULL:**
   - Always verify that any pointer or reference is not NULL before dereferencing it, which helps prevent dereference errors.

3. **Bound Checks:**
   - Ensure that any array or collection access does not exceed its bounds to avoid buffer overflows.

4. **Immutable Data Structures:**
   - Utilize immutable data structures where possible, which benefit from being thread-safe and reducing the chance of unintended side effects.

5. **Use RAII (Resource Acquisition Is Initialization):**
   - This C++ idiom manages resource allocation through object lifetimes, ensuring that resources are automatically released when objects go out of scope.

6. **Encapsulation:**
   - Encapsulate database access logic to control and validate state changes, minimizing the risk of unexpected errors.

### High Performance

1. **Efficient Queries:**
   - Write optimized SQL queries. Use `EXPLAIN` to analyze query execution plans and make adjustments based on performance insights.

2. **Indexing:**
   - Utilize appropriate indexes on frequently queried fields, but avoid over-indexing, as it can degrade write performance.

3. **Connection Pooling:**
   - Implement connection pooling to minimize the overhead of establishing database connections.

4. **Batch Operations:**
   - Use batch processing for inserts/updates to minimize round trips between your application and database.

5. **Caching:**
   - Implement caching strategies for repetitive queries using in-memory data stores (e.g., Redis) to alleviate database load.

6. **Optimize Data Types:**
   - Use the smallest data types that can accommodate your data. For example, use `INT` instead of `BIGINT` when appropriate.

7. **Asynchronous Operations:**
   - For I/O-bound applications, use asynchronous database access patterns to avoid blocking your application.

8. **Monitor and Profile:**
   - Regularly monitor your database performance, using profiling tools to identify slow queries or bottlenecks.

9. **Normalization vs. Denormalization:**
   - Normalize your database to reduce redundancy and improve integrity, but consider denormalizing for read-heavy workloads to reduce joins.

10. **Transaction Management:**
   - Use transactions wisely to ensure ACID compliance for critical operations while avoiding long-running transactions that can lock resources.

### Tools and Libraries

1. **ORMs and Query Builders:**
   - Use Object-Relational mappers (ORMs) like Entity Framework, Hibernate, or Sequelize, as they can manage memory safety in object mappings while optimizing queries.
   - Also consider lightweight query builders for more control over SQL.

2. **Profiling Tools:**
   - Use profiling tools or database analyzers (like pgAdmin for PostgreSQL or MySQL Workbench) to evaluate and optimize your queries and schema.

3. **Data Validation Libraries:**
   - Implement validation libraries to ensure that data being inserted or updated meets expected structures and constraints, preventing runtime errors.

### Conclusion

By combining safe programming practices with performance optimizations tailored to your database and application's needs, you can create robust and performant database code. Make sure to continually profile and refine your code as systems and data change.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 352.531084ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3662 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 395.087583ms
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
  ✓ Text search completed in 65.119458ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe and high-performance database code involves several principles and best practices. Here’s a comprehensive guide:

### Memory Safety

1. **Use Safe Programming Languages:**
   - Prefer languages that enforce memory safety, like Rust, Swift, or languages with garbage collection (e.g., Java, C#). These languages manage memory allocation and deallocation automatically.

2. **Check for NULL:**
   - Always verify that any pointer or reference is not NULL before dereferencing it, which helps prevent dereference errors.

3. **Bound Checks:**
   - Ensure that any array or collection access does not exceed its bounds to avoid buffer overflows.

4. **Immutable Data Structures:**
   - Utilize immutable data structures where possible, which benefit from being thread-safe and reducing the chance of unintended side effects.

5. **Use RAII (Resource Acquisition Is Initialization):**
   - This C++ idiom manages resource allocation through object lifetimes, ensuring that resources are automatically released when objects go out of scope.

6. **Encapsulation:**
   - Encapsulate database access logic to control and validate state changes, minimizing the risk of unexpected errors.

### High Performance

1. **Efficient Queries:**
   - Write optimized SQL queries. Use `EXPLAIN` to analyze query execution plans and make adjustments based on performance insights.

2. **Indexing:**
   - Utilize appropriate indexes on frequently queried fields, but avoid over-indexing, as it can degrade write performance.

3. **Connection Pooling:**
   - Implement connection pooling to minimize the overhead of establishing database connections.

4. **Batch Operations:**
   - Use batch processing for inserts/updates to minimize round trips between your application and database.

5. **Caching:**
   - Implement caching strategies for repetitive queries using in-memory data stores (e.g., Redis) to alleviate database load.

6. **Optimize Data Types:**
   - Use the smallest data types that can accommodate your data. For example, use `INT` instead of `BIGINT` when appropriate.

7. **Asynchronous Operations:**
   - For I/O-bound applications, use asynchronous database access patterns to avoid blocking your application.

8. **Monitor and Profile:**
   - Regularly monitor your database performance, using profiling tools to identify slow queries or bottlenecks.

9. **Normalization vs. Denormalization:**
   - Normalize your database to reduce redundancy and improve integrity, but consider denormalizing for read-heavy workloads to reduce joins.

10. **Transaction Management:**
   - Use transactions wisely to ensure ACID compliance for critical operations while avoiding long-running transactions that can lock resources.

### Tools and Libraries

1. **ORMs and Query Builders:**
   - Use Object-Relational mappers (ORMs) like Entity Framework, Hibernate, or Sequelize, as they can manage memory safety in object mappings while optimizing queries.
   - Also consider lightweight query builders for more control over SQL.

2. **Profiling Tools:**
   - Use profiling tools or database analyzers (like pgAdmin for PostgreSQL or MySQL Workbench) to evaluate and optimize your queries and schema.

3. **Data Validation Libraries:**
   - Implement validation libraries to ensure that data being inserted or updated meets expected structures and constraints, preventing runtime errors.

### Conclusion

By combining safe programming practices with performance optimizations tailored to your database and application's needs, you can create robust and performant database code. Make sure to continually profile and refine your code as systems and data change.

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
    ✓ Generated embedding: 1536 dimensions in 0.593s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.285s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 14.264s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 3.200s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
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
    ✓ Generated embedding: 1536 dimensions in 0.336s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.081s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.504] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. [Score: 0.481] From conv_database_design
     What is database normalization?

  3. [Score: 0.479] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. [Score: 0.467] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.446] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several best practices across both the programming and database management layers. Here are some strategies:

### 1. Use Prepared Statements
- **SQL Injection Prevention**: Always use prepared statements to ensure that input data is safely handled rather than directly concatenated into queries, which can lead to SQL injection vulnerabilities.
- **Performance**: Prepared statements can also improve performance, as the database can cache the execution plan.

### 2. Transactions
- **Atomic Operations**: Use transactions to group multiple operations together to ensure they either complete successfully or leave the database unchanged. This ensures data integrity.
- **Concurrency Control**: Use optimistic or pessimistic locking as necessary to manage concurrent access to shared resources.

### 3. Connection Management
- **Connection Pooling**: Use a connection pool to handle database connections efficiently. This minimizes the overhead of establishing new connections by reusing existing ones.

### 4. Indexing
- **Optimize Query Performance**: Create appropriate indexes on the database to speed up query performance. Use the `query_explain` tool (in ekoDB) to understand how queries are being executed and improve the indexing strategy.
- **Avoid Over-Indexing**: Too many indexes can slow down write operations, so balance read and write efficiencies.

### 5. Data Modeling
- **Normalization**: Normalize database schema to reduce redundancy, which also helps ensure data integrity.
- **Denormalization When Necessary**: In performance-critical applications, consider selectively denormalizing data for read-heavy operations to reduce joins.

### 6. Asynchronous Operations
- **Async Database Operations**: Utilize asynchronous database calls when possible to avoid blocking the application thread. This enhances the responsiveness of your application.

### 7. Error Handling
- **Graceful Error Management**: Implement robust error handling to catch database connection issues or execution errors and provide meaningful feedback or logging without crashing the application.
- **Retry Logic**: Implement retry logic for transient errors, such as timeout exceptions, to improve resiliency.

### 8. Profiling and Monitoring
- **Performance Monitoring**: Utilize profiling tools to monitor query performance and identify bottlenecks. Monitor active queries and database load to optimize performance over time.
- **Logging**: Implement logging to capture slow queries and errors, which can help diagnose issues and improve performance.

### 9. Memory Management
- **Batch Operations**: Use batch inserts or updates to minimize the number of database calls and optimize memory usage. 
- **Garbage Collection**: Ensure any allocated resources are appropriately disposed of or released, especially in long-running transactions or services.

### 10. Utilize the Right Tools and Technologies
- **Choose the Right Database**: Select a database system that fits your application’s performance, scalability, and complexity needs (e.g., SQL vs. NoSQL).
- **Leverage ORM Tools**: If using an Object-Relational Mapping (ORM) tool, choose one that is efficient and provides caching or lazy loading features.

### Example in EkoDB Context
For a backend built using ekoDB, you can implement these practices by utilizing features like:
- **Stored Functions** to encapsulate database logic and improve transaction safety.
- **Custom Indexes** to optimize query performance for your specific data retrieval patterns.
- **Asynchronous query handling** to improve application responsiveness.

By integrating these practices, you can write high-performance, memory-safe database interactions that improve the reliability and efficiency of your applications.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.296s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3804 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.497s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices across both the programming and database management layers. Here are some strategies:

### 1. Use Prepared Statements
- **SQL Injection Prevention**: Always use prepared statements to ensure that input data is safely handled rather than directly concatenated into queries, which can lead to SQL injection vulnerabilities.
- **Performance**: Prepared statements can also improve performance, as the database can cache the execution plan.

### 2. Transactions
- **Atomic Operations**: Use transactions to group multiple operations together to ensure they either complete successfully or leave the database unchanged. This ensures data integrity.
- **Concurrency Control**: Use optimistic or pessimistic locking as necessary to manage concurrent access to shared resources.

### 3. Connection Management
- **Connection Pooling**: Use a connection pool to handle database connections efficiently. This minimizes the overhead of establishing new connections by reusing existing ones.

### 4. Indexing
- **Optimize Query Performance**: Create appropriate indexes on the database to speed up query performance. Use the `query_explain` tool (in ekoDB) to understand how queries are being executed and improve the indexing strategy.
- **Avoid Over-Indexing**: Too many indexes can slow down write operations, so balance read and write efficiencies.

### 5. Data Modeling
- **Normalization**: Normalize database schema to reduce redundancy, which also helps ensure data integrity.
- **Denormalization When Necessary**: In performance-critical applications, consider selectively denormalizing data for read-heavy operations to reduce joins.

### 6. Asynchronous Operations
- **Async Database Operations**: Utilize asynchronous database calls when possible to avoid blocking the application thread. This enhances the responsiveness of your application.

### 7. Error Handling
- **Graceful Error Management**: Implement robust error handling to catch database connection issues or execution errors and provide meaningful feedback or logging without crashing the application.
- **Retry Logic**: Implement retry logic for transient errors, such as timeout exceptions, to improve resiliency.

### 8. Profiling and Monitoring
- **Performance Monitoring**: Utilize profiling tools to monitor query performance and identify bottlenecks. Monitor active queries and database load to optimize performance over time.
- **Logging**: Implement logging to capture slow queries and errors, which can help diagnose issues and improve performance.

### 9. Memory Management
- **Batch Operations**: Use batch inserts or updates to minimize the number of database calls and optimize memory usage. 
- **Garbage Collection**: Ensure any allocated resources are appropriately disposed of or released, especially in long-running transactions or services.

### 10. Utilize the Right Tools and Technologies
- **Choose the Right Database**: Select a database system that fits your application’s performance, scalability, and complexity needs (e.g., SQL vs. NoSQL).
- **Leverage ORM Tools**: If using an Object-Relational Mapping (ORM) tool, choose one that is efficient and provides caching or lazy loading features.

### Example in EkoDB Context
For a backend built using ekoDB, you can implement these practices by utilizing features like:
- **Stored Functions** to encapsulate database logic and improve transaction safety.
- **Custom Indexes** to optimize query performance for your specific data retrieval patterns.
- **Asynchronous query handling** to improve application responsiveness.

By integrating these practices, you can write high-performance, memory-safe database interactions that improve the reliability and efficiency of your applications.

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
    ✓ Generated embedding: 1536 dimensions in 19.515s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.436s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.345s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.311s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.289s
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
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
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
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.080s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.474] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.466] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.449] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.434] From conv_database_design
     What is database normalization?

  5. [Score: 0.405] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe and high-performance database code requires a careful approach to programming practices and database interaction. Here are some key strategies to consider:

### 1. Use Strong Typing and Memory Safety Features
- If you're using a programming language like Rust, employ its ownership and borrowing system to ensure memory safety at compile time.
- In languages like C#, utilize features like `using` statements or `Dispose` patterns to manage resource cleanup automatically.

### 2. Optimize Query Performance
- **Use Prepared Statements**: They can prevent SQL injection attacks and optimize performance by allowing the database to cache query plans.
- **Batch Operations**: Instead of executing numerous single-row operations, group them into a batch transaction to reduce round trips to the database.
  
### 3. Indexing
- Ensure that your database tables are appropriately indexed based on query patterns. This improves the speed of data retrieval significantly.
- Regularly monitor and analyze query performance using tools to identify and create necessary indexes.

### 4. Data Access Patterns
- **Lazy Loading**: Load data as needed rather than all at once, especially large datasets. This can help reduce memory usage.
- **Pagination**: For queries that may return a lot of data, always implement pagination to limit the number of records handled at a time.

### 5. Connection Management
- Use a connection pool to manage database connections efficiently, minimizing the overhead associated with creating and closing connections frequently.
- Ensure proper handling of connection lifecycle events to prevent memory leaks.

### 6. Error Handling
- Implement robust error handling to manage database exceptions gracefully. This can prevent cascading failures and ensure the application remains stable.

### 7. Use ORM Wisely
- When using Object-Relational Mappers (ORMs), be cautious of the overhead they can introduce. Optimize queries generated by the ORM and avoid over-fetching of data.
- Understand the underlying SQL generated by the ORM to identify potential inefficiencies.

### 8. Monitor and Profile
- Regularly profile your application's performance and memory usage. Tools like profilers and monitoring frameworks can help identify bottlenecks.
- Use database performance monitoring tools to identify slow queries or resource contention issues.

### 9. Consider Asynchronous Processing
- In environments where high throughput is necessary, consider using asynchronous programming paradigms to prevent blocking operations when accessing the database.

### 10. Use Transactions Wisely
- For multiple related database operations, wrap them in transactions to ensure atomicity. However, be mindful of transaction duration and locking, which could affect performance.

### Example With ekoDB Tools
In the context of ekoDB, you might focus on:
- Using stored functions (`create_function`) for encapsulating operations.
- Ensuring you make use of `batch_transaction` for multi-record operations to enhance performance.

### Conclusion
Combining these strategies will help you develop database code that is both memory-safe and performant. Always iterate and test under realistic load conditions to ensure your approach meets the demands of your application. If you're interested, I can assist with specific implementations or tool usage for optimizing database operations with ekoDB.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.352s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3411 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.250s
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
  ✓ Text search completed in 0.052s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe and high-performance database code requires a careful approach to programming practices and database interaction. Here are some key strategies to consider:

### 1. Use Strong Typing and Memory Safety Features
- If you're using a programming language like Rust, employ its ownership and borrowing system to ensure memory safety at compile time.
- In languages like C#, utilize features like `using` statements or `Dispose` patterns to manage resource cleanup automatically.

### 2. Optimize Query Performance
- **Use Prepared Statements**: They can prevent SQL injection attacks and optimize performance by allowing the database to cache query plans.
- **Batch Operations**: Instead of executing numerous single-row operations, group them into a batch transaction to reduce round trips to the database.
  
### 3. Indexing
- Ensure that your database tables are appropriately indexed based on query patterns. This improves the speed of data retrieval significantly.
- Regularly monitor and analyze query performance using tools to identify and create necessary indexes.

### 4. Data Access Patterns
- **Lazy Loading**: Load data as needed rather than all at once, especially large datasets. This can help reduce memory usage.
- **Pagination**: For queries that may return a lot of data, always implement pagination to limit the number of records handled at a time.

### 5. Connection Management
- Use a connection pool to manage database connections efficiently, minimizing the overhead associated with creating and closing connections frequently.
- Ensure proper handling of connection lifecycle events to prevent memory leaks.

### 6. Error Handling
- Implement robust error handling to manage database exceptions gracefully. This can prevent cascading failures and ensure the application remains stable.

### 7. Use ORM Wisely
- When using Object-Relational Mappers (ORMs), be cautious of the overhead they can introduce. Optimize queries generated by the ORM and avoid over-fetching of data.
- Understand the underlying SQL generated by the ORM to identify potential inefficiencies.

### 8. Monitor and Profile
- Regularly profile your application's performance and memory usage. Tools like profilers and monitoring frameworks can help identify bottlenecks.
- Use database performance monitoring tools to identify slow queries or resource contention issues.

### 9. Consider Asynchronous Processing
- In environments where high throughput is necessary, consider using asynchronous programming paradigms to prevent blocking operations when accessing the database.

### 10. Use Transactions Wisely
- For multiple related database operations, wrap them in transactions to ensure atomicity. However, be mindful of transaction duration and locking, which could affect performance.

### Example With ekoDB Tools
In the context of ekoDB, you might focus on:
- Using stored functions (`create_function`) for encapsulating operations.
- Ensuring you make use of `batch_transaction` for multi-record operations to enhance performance.

### Conclusion
Combining these strategies will help you develop database code that is both memory-safe and performant. Always iterate and test under realistic load conditions to ensure your approach meets the demands of your application. If you're interested, I can assist with specific implementations or tool usage for optimizing database operations with ekoDB.

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
    ✓ Generated embedding: 1536 dimensions in 0.244s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.356s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.304s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.368s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.264s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.338s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.225s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.287s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
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
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.078s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.474] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.466] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.449] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.434] From conv_database_design
     What is database normalization?

  5. [Score: 0.405] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
2026/06/26 13:06:27 Network error, retrying after 128.760895ms...
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of best practices at both the database and application levels. Here are some key principles to follow:

### 1. **Query Optimization**
   - **Use Prepared Statements:** Prepared statements help mitigate SQL injection attacks and often improve performance by allowing the database to cache execution plans.
   - **Indexing:** Create indexes on columns that are frequently used in WHERE clauses, JOIN conditions, and sorting. Use composite indexes when multiple fields are involved.
   - **Batch Operations:** Use batch inserts and updates to reduce the number of individual transactions sent to the database.

### 2. **Connection Management**
   - **Connection Pooling:** Use connection pooling to manage database connections efficiently. This reduces the overhead of establishing new connections for each database operation.
   - **Close Connections:** Ensure that database connections are properly closed after use to prevent resource leaks.
   
### 3. **Data Integrity and Safety**
   - **Transactions:** Wrap multiple SQL statements in transactions to ensure atomicity. This ensures that if one part of a transaction fails, the entire transaction can be rolled back.
   - **Validation:** Validate inputs on the application side before executing queries. This can reduce the risk of corrupting the database with invalid data.

### 4. **Memory Management**
   - **Efficient Data Types:** Use the most appropriate and efficient data types for your fields (e.g., avoid using `TEXT` when `VARCHAR` will suffice).
   - **Batch Fetching:** Retrieve only the necessary data when querying and consider pagination for large datasets.
   - **Avoid Memory Leaks:** Keep an eye on memory usage and avoid keeping large datasets in memory when not necessary.

### 5. **Concurrency Control**
   - **Locking Mechanisms:** Understand the locking mechanisms of your database. Use appropriate isolation levels as per your requirement to ensure data consistency while optimizing performance.
   - **Optimistic Concurrency:** When appropriate, use optimistic concurrency control to allow multiple transactions to occur without locking resources and improve throughput.

### 6. **Monitoring and Profiling**
   - **Use Profiling Tools:** Monitor your database's performance regularly using profiling tools. Identify slow queries and many reading/writing operations and optimize them.
   - **Analyze Query Execution Plans:** Use execution plans to understand how the database executes a query. Look for full table scans and other inefficiencies.

### 7. **Programming Language Best Practices**
   - **Safe Libraries:** Use library functions and frameworks that handle memory safety, such as those built into languages like Rust, which emphasize safety by design.
   - **Error Handling:** Implement robust error handling to manage exceptions that may occur during database operations gracefully.

### 8. **Regular Maintenance**
   - **Database Maintenance:** Regularly perform maintenance tasks such as vacuuming (for PostgreSQL), analyzing tables, and optimizing them based on usage patterns to improve performance.

By integrating these practices, you can write database code that is both memory-safe and optimized for performance, leading to a more reliable and efficient application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.446s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3338 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.414s
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
  ✓ Text search completed in 0.051s

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
    ✓ Generated embedding: 1536 dimensions in 0.367s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.589s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.538s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.503s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.486s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.417s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.31s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.25s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.353s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.281s
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

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.101s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.474] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.466] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.449] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.434] From conv_database_design
     What is database normalization?

  5. [Score: 0.405] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.36s
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
  ✓ Text search completed in 0.037s

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
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.9',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.22.0 prepare
> npm run build


> @ekodb/ekodb-client@0.22.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.22.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
Function 'fetch_api_user' already existed — updated instead
✓ Created SWR script: fetch_api_user (2NrnzcD-AiTAxH9So0-f0yKFPq4wQ6F3QdFD7-cwRPS7DBhUAyc7godUd4wnJmAkiFtwQT3md2qvjgYDwR_iQw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "name": "Leanne Graham",
          "username": "Bret",
          "website": "hildegard.org"
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
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "name": "Leanne Graham",
          "username": "Bret",
          "website": "hildegard.org"
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
Function 'fetch_product_with_reviews' already existed — updated instead
✓ Created enrichment script: fetch_product_with_reviews (Pb9lMMAPxDKFnDOQq0Qx_hc1JN74IZyavYCYg9aFpe5zrXOHTNLlV5OZgsRG1O7jgP4crkgmEP0S2pl8d3Hg6A)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "shippingInformation": "Ships in 3-5 business days",
          "availabilityStatus": "In Stock",
          "category": "beauty",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "title": "Essence Mascara Lash Princess",
          "discountPercentage": 10.48,
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "weight": 4,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "reviewerName": "Eleanor Collins"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!"
            }
          ],
          "returnPolicy": "No return policy",
          "minimumOrderQuantity": 48,
          "meta": {
            "createdAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2025-04-30T09:41:02.053Z"
          },
          "brand": "Essence",
          "sku": "BEA-ESS-ESS-001",
          "rating": 2.56,
          "price": 9.99,
          "stock": 99,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "id": 1,
          "warrantyInformation": "1 week warranty"
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
ℹ️  Function 'cache_api_call' already existed — updated instead
✓ Edge cache script updated: 1tcuQKWrELFM7B7sjJ4zKIhH7oGSc_iab2zEE5wUroJKnAjqmEpt3tUIiM4UpjrwS2cJgVilSAtuKgpkvwUXDw

Call 1: Cache miss (fetches from API)
Response time: 55ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "name": "Leanne Graham",
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442"
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
Response time: 3ms (18.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light"
          },
          "name": "Leanne Graham",
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442"
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
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.22.0
    Uninstalling ekodb_client-0.22.0:
      Successfully uninstalled ekodb_client-0.22.0
Successfully installed ekodb-client-0.22.0
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (3Y4dh6FKqjuRmpv_F7m2fIX6_fYIQOCHZQa6QhSxCR_xPJy-SD1LXCCmcjopzgV-wqRmn6TQso0zZ4ggzBU9bA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 220ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1782493633"
    },
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
        "followers": 308940,
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
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-06-17T17:35:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 6ms (35.2x faster!)
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1782493633"
    },
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
        "followers": 308940,
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
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-06-17T17:35:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (FjNgidUHnmrwIC7PA037j-3w41U1EhEwyn-0oxThuuhnlQ9tWynHcJI9770QLUgFi7vSMVw369gl0_hX9Jujsg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "String",
      "value": "1782493633"
    },
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
Function 'fetch_github_user' already existed — updated instead
✓ Created SWR script: fetch_github_user (fetch_github_user)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 1.851166ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1782493633"
    },
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
        "followers": 308940,
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
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-06-17T17:35:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 1.996083ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment script: fetch_product_enriched (fetch_product_enriched)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
ℹ️  Function 'fetch_github_user' already existed — updated instead
✓ Created SWR script: fetch_github_user (oiD7GT3g4ow3ej4sJbIVfxsK91X_WEsQqpMXpNPXi8Bj0ABgM3eAmUo2Al6W-WtWx5EuYq8FjdXi29uE4oCByg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 3ms
Result: {
  "data": {
    "value": {
      "company": "Linux Foundation",
      "site_admin": false,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "hireable": null,
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "followers": 308940,
      "twitter_username": null,
      "public_gists": 1,
      "updated_at": "2026-06-17T17:35:14Z",
      "bio": null,
      "location": "Portland, OR",
      "name": "Linus Torvalds",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "public_repos": 12,
      "blog": "",
      "login": "torvalds",
      "email": null,
      "user_view_type": "public",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "url": "https://api.github.com/users/torvalds",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "gravatar_id": "",
      "created_at": "2011-09-03T15:26:22Z",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "following": 0,
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "html_url": "https://github.com/torvalds",
      "id": 1024025,
      "type": "User"
    },
    "type": "Object"
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (1.5x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
ℹ️  Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment script: fetch_product_enriched (3_n0eFSdikXIEuxBMcFXMJtbLdqS-ietXtn96lDxkIZ0r4cn-w2VO7nBcpLJ7X4PFLajjz5sGxPzpEMX3tTU6g)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "tags": [
        "beauty",
        "mascara"
      ],
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "id": 1,
      "brand": "Essence",
      "title": "Essence Mascara Lash Princess",
      "rating": 2.56,
      "meta": {
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "updatedAt": "2025-04-30T09:41:02.053Z",
        "barcode": "5784719087687"
      },
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "category": "beauty",
      "discountPercentage": 10.48,
      "sku": "BEA-ESS-ESS-001",
      "minimumOrderQuantity": 48,
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "availabilityStatus": "In Stock",
      "weight": 4,
      "warrantyInformation": "1 week warranty",
      "shippingInformation": "Ships in 3-5 business days",
      "dimensions": {
        "width": 15.14,
        "height": 13.08,
        "depth": 22.99
      },
      "price": 9.99,
      "returnPolicy": "No return policy",
      "stock": 99,
      "reviews": [
        {
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins",
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "rating": 3
        },
        {
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "reviewerName": "Lucas Gordon",
          "comment": "Very satisfied!",
          "rating": 4,
          "date": "2025-04-30T09:41:02.053Z"
        },
        {
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 5,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Highly impressed!"
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
ℹ️  Function 'fetch_github_user' already existed — updated instead
✓ Created SWR function: fetch_github_user (v_l5yu-kiXFPTdysiB0Mcq-uoaEGVLe9ukXIjGxKXa06yWmwLECMsZpC9hu8F-O68oC5dkuLrajQXXGU1EcAbQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 18ms
Result: [{"data":{"type":"Object","value":{"starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","updated_at":"2026-06-17T17:35:14Z","user_view_type":"public","created_at":"2011-09-03T15:26:22Z","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","email":null,"id":1024025,"twitter_username":null,"avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","followers":308940,"url":"https://api.github.com/users/torvalds","company":"Linux Foundation","node_id":"MDQ6VXNlcjEwMjQwMjU=","public_gists":1,"organizations_url":"https://api.github.com/users/torvalds/orgs","bio":null,"location":"Portland, OR","name":"Linus Torvalds","blog":"","html_url":"https://github.com/torvalds","gravatar_id":"","login":"torvalds","followers_url":"https://api.github.com/users/torvalds/followers","public_repos":12,"repos_url":"https://api.github.com/users/torvalds/repos","received_events_url":"https://api.github.com/users/torvalds/received_events","events_url":"https://api.github.com/users/torvalds/events{/privacy}","site_admin":false,"following_url":"https://api.github.com/users/torvalds/following{/other_user}","hireable":null,"following":0,"type":"User"}},"cached_at":{"type":"String","value":"1782493633"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms
Cache hit was 3.6x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
ℹ️  Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment function: fetch_product_enriched (9lZCmYzIa9WsuKAf0Uh6yXxSMuHt-zdmEVWotcodL0tX_ZGP_00V1ONGX6HObX5bWvxcfKI__kvuUy4ytj33Hg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"dimensions":{"height":13.08,"depth":22.99,"width":15.14},"price":9.99,"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"id":1,"availabilityStatus":"In Stock","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","returnPolicy":"No return policy","title":"Essence Mascara Lash Princess","weight":4,"meta":{"qrCode":"https://cdn.dummyjson.com/public/qr-code.png","updatedAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687","createdAt":"2025-04-30T09:41:02.053Z"},"rating":2.56,"stock":99,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","warrantyInformation":"1 week warranty","shippingInformation":"Ships in 3-5 business days","sku":"BEA-ESS-ESS-001","reviews":[{"reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins","comment":"Would not recommend!","rating":3},{"rating":4,"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"lucas.gordon@x.dummyjson.com","comment":"Very satisfied!","reviewerName":"Lucas Gordon"},{"date":"2025-04-30T09:41:02.053Z","rating":5,"comment":"Highly impressed!","reviewerName":"Eleanor Collins","reviewerEmail":"eleanor.collins@x.dummyjson.com"}],"category":"beauty","tags":["beauty","mascara"],"brand":"Essence","minimumOrderQuantity":48,"discountPercentage":10.48}}}]
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


BUILD SUCCESSFUL in 10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
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

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 62.151125ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "company": {
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.259916ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "id": 1,
      "email": "Sincere@april.biz",
      "address...
   🚀 Cache speedup: 31.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
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
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.9',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.22.0 prepare
> npm run build


> @ekodb/ekodb-client@0.22.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.22.0 build
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

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net"
      }...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...
   🚀 Cache speedup: 1.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.11s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.22.0
    Uninstalling ekodb_client-0.22.0:
      Successfully uninstalled ekodb_client-0.22.0
Successfully installed ekodb-client-0.22.0
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2.0ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.8ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 0.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 1.982042ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.910167ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...
   🚀 Cache speedup: 1.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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
  ✓ Reusable Function building blocks
  ✓ Functions calling other Functions
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
✅ [32mClient build complete![0m

🦀 [33mRust WebSocket Subscription Test...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_72fd5ad8ae0b4e60b05d564e73dbcfe3)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "OPROkeHCRNqW5nKI3optYmDyKE63407EIPse7kVLzpFz3JpQs0zzxQWMftwiLZ_J-sWRUhNH5wNzHyXqTUwmng"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["OPROkeHCRNqW5nKI3optYmDyKE63407EIPse7kVLzpFz3JpQs0zzxQWMftwiLZ_J-sWRUhNH5wNzHyXqTUwmng"]
     Timestamp:  "2026-06-26T17:07:35.309712+00:00"

Inserting record 2...
✓ Inserted: "tVUHyfyy3U9-3DxR5Ue0jWmhB_RO0gZ_do-44wbndyNUIAZ2nRjrvcRw5ZWlPEVXtrZY5GdWK8nFZ1WIE1WB7g"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["tVUHyfyy3U9-3DxR5Ue0jWmhB_RO0gZ_do-44wbndyNUIAZ2nRjrvcRw5ZWlPEVXtrZY5GdWK8nFZ1WIE1WB7g"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mRust subscription test complete![0m

🔷 [33mGo WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_8358c99ce0494a2c91954ef918c3b74e)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: PcqURKnRsowk-hs7ud7xEtlnG04DgrOoyYQY5wa56ugNsJ1xr-_X8VRAiNQfOx6dYI61s31_4nVx3Y5g6CaUgw

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [PcqURKnRsowk-hs7ud7xEtlnG04DgrOoyYQY5wa56ugNsJ1xr-_X8VRAiNQfOx6dYI61s31_4nVx3Y5g6CaUgw]
     Timestamp:  2026-06-26T17:07:35.803843+00:00

Inserting record 2...
✓ Inserted: 1QgGLkTPmQdAkuPwqmtqrbp_dFRqxxADlLZQfwCVu6XlWftdHbeCUAwKX3TGhA2rLZ7k26ByRhuDO_-yRMF6lA

  📡 Notification received:
     Event:      insert
     Record IDs: [1QgGLkTPmQdAkuPwqmtqrbp_dFRqxxADlLZQfwCVu6XlWftdHbeCUAwKX3TGhA2rLZ7k26ByRhuDO_-yRMF6lA]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mGo subscription test complete![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

🐍 [33mPython WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_535516634b844c7d80e66f4fd3f2b244)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: cpSk_4h9lBYJhcQSQ30wnv-Ts9tPDQkgpva3xGMu9l1msbPsiAJB0-UhpMLpAvNwzqFjxnavGeYfRkeP3TtQHQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: cpSk_4h9lBYJhcQSQ30wnv-Ts9tPDQkgpva3xGMu9l1msbPsiAJB0-UhpMLpAvNwzqFjxnavGeYfRkeP3TtQHQ
     Timestamp:  2026-06-26T17:07:36.257777+00:00

Inserting record 2...
✓ Inserted: _ZTu7X92coAL3hv3aLtT9z682I6A8QT45CogE8UmDbQR6KpqkbQpi-KauLEuS9GZPwsNoVcblqPTd_c7y5TzFw

  📡 Notification received:
     Event:      insert
     Record IDs: _ZTu7X92coAL3hv3aLtT9z682I6A8QT45CogE8UmDbQR6KpqkbQpi-KauLEuS9GZPwsNoVcblqPTd_c7y5TzFw

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

✓ WebSocket subscription example completed successfully
✅ [32mPython subscription test complete![0m
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.9',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.22.0 prepare
> npm run build


> @ekodb/ekodb-client@0.22.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.22.0 build
> tsc

✅ [32mTypeScript client built![0m

📘 [33mTypeScript WebSocket Subscription Test...[0m
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_84eb481f8ffe4b86a0a32f93ec25fa25)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: _r7sB6ULg9gJAkenkVV3yjaBuabYc3vYalEP8mKo3v1icGbNqzcn1C0A056lB9jbb-ZzYbyNWw76UeYohvJUYQ
     Timestamp:  2026-06-26T17:07:39.524242+00:00
     Records:    {"active":true,"id":"_r7sB6ULg9gJAkenkVV3yjaBuabYc3vYalEP8mKo3v1icGbNqzcn1C0A056lB9jbb-ZzYbyNWw76UeY...
✓ Inserted record: _r7sB6ULg9gJAkenkVV3yjaBuabYc3vYalEP8mKo3v1icGbNqzcn1C0A056lB9jbb-ZzYbyNWw76UeYohvJUYQ

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: YKs6WbZJbzH5jg55gklhyyEnvwXacSNhfFox43ObhLzgLK3o95azbW8X-ArZNn0Nr9I88zC-h33V7KToPGNyKQ
     Timestamp:  2026-06-26T17:07:40.552178+00:00
     Records:    {"active":true,"id":"YKs6WbZJbzH5jg55gklhyyEnvwXacSNhfFox43ObhLzgLK3o95azbW8X-ArZNn0Nr9I88zC-h33V7KT...

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
✓ Subscribed (subscription_id: sub_aaec2c3d180643fb9f1891118bac0aac)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "6UYKkaXH7g-9t7RHQUXD_zwu2l9xrvJM8DJJrMe5PnZSy7D0Gai1oZpuZ9y3bKdXKEWy_6ooL_ninubdEEos-g"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["6UYKkaXH7g-9t7RHQUXD_zwu2l9xrvJM8DJJrMe5PnZSy7D0Gai1oZpuZ9y3bKdXKEWy_6ooL_ninubdEEos-g"]
     Timestamp:  "2026-06-26T17:07:46.298649+00:00"

Inserting record 2...
✓ Inserted: "FDi_w2U4QC6JGdAirDZzmVkWwJGfGWsfcadPNnN7tASQIARpozRleRW6-c8vzqffGM7A4_5vJz9Cdt-uoBH9-Q"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["FDi_w2U4QC6JGdAirDZzmVkWwJGfGWsfcadPNnN7tASQIARpozRleRW6-c8vzqffGM7A4_5vJz9Cdt-uoBH9-Q"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin subscription test complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll WebSocket Subscription Tests Passed![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
