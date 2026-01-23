make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
   Compiling ekodb_client v0.8.0 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("1ieM4QtxS8SmKyZDBFHLR1Akotmnb8KQu0Eti0k1f_Ds_e0Vj_6WtuWzdI8D2tYK5oQPrtCM37lDi9nNFOzinQ")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("1ieM4QtxS8SmKyZDBFHLR1Akotmnb8KQu0Eti0k1f_Ds_e0Vj_6WtuWzdI8D2tYK5oQPrtCM37lDi9nNFOzinQ"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("1ieM4QtxS8SmKyZDBFHLR1Akotmnb8KQu0Eti0k1f_Ds_e0Vj_6WtuWzdI8D2tYK5oQPrtCM37lDi9nNFOzinQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("1ieM4QtxS8SmKyZDBFHLR1Akotmnb8KQu0Eti0k1f_Ds_e0Vj_6WtuWzdI8D2tYK5oQPrtCM37lDi9nNFOzinQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "ajOM9grEuxTgPap_lS26dHJIjqMNVtCi0kY26-XoI-_V6fIVLgzgWGF1AbVnsyVjJ7f6teDdL69dUR6se7jbXA"

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
        "id": "ajOM9grEuxTgPap_lS26dHJIjqMNVtCi0kY26-XoI-_V6fIVLgzgWGF1AbVnsyVjJ7f6teDdL69dUR6se7jbXA",
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "fURD7e8Hac6Jy1MXoh3Gfa1NFRyRfcasmToiAu7dFAIT1R07tokNdQwxVJnGdM3U1LvFH9KZqCNfe1B2haqqRw"

=== List Collections ===
Total collections: 4
Sample collections: ["test_collection", "demo_collection", "websocket_test", "batch_users"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "7UbFP-gwpecCPlKj5EdyLGiAtc2etLt1KB153cNvaYXs_J3eaeuJacKPLhvwfqnIxbifpJAFZqTIx6If26Qd4Q"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "nYT95s3Df49R16X7u0CKutHsECxw6yRVvoWRVETQNSpYuBDj0qJ53DXhZNV_NIddRB3zbPfF08ewskyZKThp1w"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "Ri2WxvxMc4XbFUL8TjM363dnyomhLfKGQiVkjkhT_hw6lQHYmuSgNDj1HJrW4IP-V25ocp4EjEhipahmITWGGw"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("78B3_vOy-NUA0klMbNZEXwFuVSwOBodwco3XZ2OsPtUtAlU7FwGjcKf4B3Dtzu7Ht2rag93UII0pMVaUVENFvg")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "78B3_vOy-NUA0klMbNZEXwFuVSwOBodwco3XZ2OsPtUtAlU7FwGjcKf4B3Dtzu7Ht2rag93UII0pMVaUVENFvg"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-23T18:25:08.854884Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: UP2E7SCk-qgUv4WO9Wez2X-NZf33rf6uhy04z0ySbgbpbMvJJPCsGws0FYNXUGNH50wJPIA85p1kwrNqkvxJ4g
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "18yzP1PoaaAImE1CFOznyWWoXDTEGKOZCcHbCtYdA17DkDyXMMbvkHLGTMzILJgqrzwG58oGdydRFSgI_Ebifw"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: lkJRz3inyzSsIO4rmb93dF_HiBXxviTe0V_ay3Du_W-RrlS7luGFKaRxO5RbMe9QP0Jgn1T2ScJTuz1OVvF3yg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: IImqzBuahQ5SQ4KAOwyvgKDULNuC8F8vXG0fbmt0IVsXPB4V7XrQ_tmeUetX7IRPGzeJ7-z_BbgL_GMZ98MDlg
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: vF5Fr1BP1e8I1Jd1PQZF8vIjpTBkPnFLaE6tTt1-LuPA4JY4TQ_8QvReIT3uo26mCMthU4h2LqF4vAclan833w
Created Bob: $500 - ID: PO-eUJwZLeJYYX5Q6vPBKnGfojAnDS8d26sRwrclb1xNZMX0jp7ObP5EFsqKPiGtzBO8fXKrHucGwawBM84DFw

=== Example 1: Begin Transaction ===
Transaction ID: 1fa295ab-f43c-408f-b887-256e46f0af85

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
New transaction: 42d2da91-f6cf-48a0-86b9-3101089c1a88
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("ofAjCRpg5TQBpwMO2CjlWT8o6d0TjvsQ5HO4uVYJe8Aupt66CWUPMAXz7kSdsp0uqs-6D-5e31ANER_STSqnVg")})

=== Find by ID ===
Found: Record({"name": String("Test Record"), "metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "active": Boolean(true), "tags": Array([String("tag1"), String("tag2"), String("tag3")]), "created_at": String("2026-01-23T17:25:14.777399+00:00"), "value": Integer(42), "id": String("ofAjCRpg5TQBpwMO2CjlWT8o6d0TjvsQ5HO4uVYJe8Aupt66CWUPMAXz7kSdsp0uqs-6D-5e31ANER_STSqnVg"), "price": Float(99.99), "categories": Array([String("electronics"), String("computers")]), "embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "data": String("aGVsbG8gd29ybGQ=")})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-23T17:25:14.777399+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"created_at": Object({"value": String("2026-01-23T17:25:14.777399Z"), "type": String("DateTime")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "id": String("ofAjCRpg5TQBpwMO2CjlWT8o6d0TjvsQ5HO4uVYJe8Aupt66CWUPMAXz7kSdsp0uqs-6D-5e31ANER_STSqnVg"), "value": Object({"value": Integer(42), "type": String("Integer")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")})})]

=== Update Document ===
Updated: Record({"id": String("ofAjCRpg5TQBpwMO2CjlWT8o6d0TjvsQ5HO4uVYJe8Aupt66CWUPMAXz7kSdsp0uqs-6D-5e31ANER_STSqnVg"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "created_at": Object({"value": String("2026-01-23T17:25:14.777399Z"), "type": String("DateTime")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: JKULjfLkpZv36xLstMOPeo-gSZuw2jTg2r-C5icG6zRaOnZt2w22Y-QVT-EuSmhT6U_BOeL45W61v_t8C8_oBw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  name: "WebSocket Test Record"
  value: 42
  id: "JKULjfLkpZv36xLstMOPeo-gSZuw2jTg2r-C5icG6zRaOnZt2w22Y-QVT-EuSmhT6U_BOeL45W61v_t8C8_oBw"
  active: true

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "SGnvuqrCUQJf-k8J_-77S4HqQSNVak-nQOk69YZBzHK5jAUpDd2Y2mOH6VevIJI8zInZF28lc8xI0Dw2BqOFQw"

=== List Collections ===
Total collections: 9
Sample collections: ["client_collection_management_rust", "ws_ttl_test", "test_collection", "websocket_test", "test_accounts"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
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
  cache:product:2: Record({"price": Float(39.989999999999995), "name": String("Product 2")})
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: abrrLaOAFMgdx_yjTzEyNpJFgck-bJ4S_ZXynXTilJihv_W008YcXi9hy8DiiE3J6-tYRJGz0sLQ4xg8gM390g
Created Bob: $500 - ID: tKt6eP7fNLf5l-qQBat7llk15qnuUQH6QfWUmXigROAuEuxGCmKW_1JEXSmOMP-vwslifZH2yqmnAmNw2VUeEQ

=== Example 1: Begin Transaction ===
Transaction ID: 583f77d5-a26d-4cc9-b48c-12a6bc13699c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ae568a74-f2b6-4432-b018-0f92fb4401e7
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(String("Rust Programming"))
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))
  3. Score: 6.6000 - Some(String("Python for Data Science"))

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(String("Rust Programming"))
  2. Score: 0.5500 - Some(String("JavaScript Web Development"))
  3. Score: 0.5500 - Some(String("Python for Data Science"))

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(String("Machine Learning Basics"))
     Matched: ["description", "title.value", "title", "description.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description.value", "description"]

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
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("zhVWTKlofwCzCEqlir4p4DZ4A4ukDVRAlw8itA-pbzaCEZTJehG3mNGQ90BtH5YzCfe8eYhndwK5fpIcAkpOFg"))
✓ Inserted user 2: Some(String("HrEajxDHPMkTMQ3HountneH6R7GsTikkesrhMVLlA6aT0xuO-5B-6AM6nRH__CiQcB66krRNVwBsbL0sZ1RidA"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - email: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["ws_ttl_test", "test_collection", "websocket_test", "test_accounts", "schema_client_rust"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "izw1qVNhTSVfkwjnELgTmcFhvSWrmdtBXoxoxp3JBw_v8BHkRTDD1VM22mRI87cTPCUUk8sgqhdQY_n3w_yqgw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("B4ndnRLLG1OfIveCH0T6itB9neukKm1jVCq6Q7nQSb9zkLl8JQz8Jgc4aAR-YN2AIILsr7JRuFSdDhtXTkLSSw"))

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("DQWyqRfCyHrOSZRM2Zzl49Nno1XmDYMQRTPFQ199V691oofpUHy1DsN9OdkYYctl_FQwKiH8g47YWqHqEequaA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: dy9L7POdgVNl-Dk0bgqfwxkZiykBiQjjBtKKJEGhiL1sWzeWYwqll2zUnQnGoEJJb1OaqBAv26aqrKTexQlJKA

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: kl6mLHFJCUt_D6cBSYMR8YYq685AX8BWoyM3vqJbav5me7vxctwSh0iziJ2Lv5qqRcgg427h2GfvHYndVFgb0A
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: atOWHX7ApYOu2rfWbtfIsPUnoJ7NB5okRlspgNCc8SUqd_pnlc6HSDY0hKKyk-aZrgpeea4CsiI2m5or7nMoOw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: BZ2GGJVGYh0I4nO8cSMi3qR6pLdI2FNP9_GFV0fzoZWr0omQUfWJylgzrVOUsTwwfgEsE0MoSqlRTd5repxt0Q
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.75s
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 156.163666ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 62.682ms
   📊 Records: 0
   🚀 Cache speedup: 2.5x faster!

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: LzahZSB8yo6W8CG83LgCjeR3-Xlejgwimuwu3SHMJKwdXSSLXu5iW6OnN_KHukfyW2nyMjy09V8552W4sKXZzQ
📊 Found 2 product groups
   Record({"count": Integer(3), "category": String("Electronics"), "avg_price": Float(575.6666666666666)})
   Record({"count": Integer(2), "avg_price": Float(474.0), "category": String("Furniture")})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"count": Integer(3), "category": String("Electronics")})
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("ulOshV9BsOjtga9sUMttxUT3DunPNerHw-J8rUGbfIhyd13PCx7oqpePtukP5kXrl2iYSMr5_W9RQYTXQRVYbw"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: f_1mjaEImpwwtk7AD9P_02TFGFpuF8ec4Wv856pH80yWMg8liXxK4Y7O02IsD-dxqiyWkwNNYSquIvH5SexnUQ
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 5OeG-NsmjpVfBOKEObi1rxuCoJ9Nt4WY9G_10Y_hG2Te_eW6plDlFSv43oRhG3NRHWoXgdiIbpt0vECTtcLrrw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: Wc3ocBT1g80D-ftENcLO0_3v48ZaFyPilcyAPWwzsxEZNXaCjNeWbDMq4ic2AzbceP5brMYyqq2NzTfohU77hQ
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_rs (uGP_6CA1oa3YERMEmoh44UFfQO9wxGm0MrjRhrpET5dpCXMkIPC1GwDmfXxkj0fkxnj8_hsZrSZcrWDihnqxSw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 68ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (7YFAxNw_RRgdae0KWDrnbZCFrzchcnJISvh1Mkl0uZkPdKr2_LQRi9OwTIq9ZPVhxgf79jJEOwzpZFNp2XPdhg)

First call (cache miss - will fetch from GitHub API):
  Response time: 62ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 20.7x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (veJtdjAoQuxWUYUnQ6ewOYkEmeExDwBLuUHdSdivAvx7BgcX3Ao278tQFBK9Izk_391xYfKBH1oUjJn_rUEpVw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (Zi51ozuTQphpeHVGn_CL_J9ZkRDpVnb3kCbv2EY_iCre-iJbZBeCL8qE6VurvGhl7j3mp1n5oYBYGWas-4P3YA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (pRBM1AvmnozR0mBHqM20QK37MWeWp5tJzysDgvHl2qIV9YAwV-D9qjm8lpT5WOfqDTUtmYi3T_nBi4V_G8TIsQ)
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

   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.81s
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
   Record({"category": String("Furniture"), "count": Integer(3), "avg_price": Float(365.6666666666667)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.83s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer benefits such as efficient spatial analysis and manipulation, accurate representation of data, high-quality cartographic output, and support for topological modeling. They are also scalable and allow precise geographic operations like tracing, network analysis, and proximity analysis.
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
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
   Record({"status": String("inactive"), "count": Integer(3)})
   Record({"count": Integer(7), "status": String("active")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"count": Integer(2), "category": String("AI")})
   Record({"count": Integer(3), "category": String("Database")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.75s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: crBexqJwKDpcsGuB3rJQOwiUxiEf2h11eXAFX6UNxZ6Ubw7DoeufXGKOj9ak8CZ38Y6jh3U1t1xtDGRI_nGmsw

=== Sending Chat Message ===
Message ID: sFYmiy2v5Zy9pY6StDX3LxEdaIaQDvbcztOhvzbE0OgVzqy2JL8Jr_PQBn9v5T4v7YGrHNb7ufnpWa3rHugkYA

=== AI Response ===
Response 1: ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. This database is designed for high performance and includes a variety of features to support your data needs.

One standout feature of ekoDB is its AI Chat Integration. This feature allows you to query your database using natural language and receive AI-powered responses with relevant context. This makes it easier to interact with your database and retrieve the information you need.

Another important feature is ekoDB's robust search capabilities. ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval. This makes it easy to locate and retrieve specific pieces of data, even in a large, complex database.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("4Xh1vRTmZUKyBOyBK2JXeGQxahAMeKiNwP6mcPB1_clN3e2-2uFjKtfghfxlEo3428HtS3E5QhGkuaAWZqyiYg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("qW8fqYJ-BuMrZwLMcj4zzUz5auajvJnlFw60TU-EvUxuJ6isi7IywHnC0wggzLpAjJuR2bOdk-fEWq0v_9Dv4w"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("G0lzfyaGX09ZeF4-t1p0oHzkVUANqRYLEY4EOgvzjlnDCaDr6qT837HY5i3cR_PaJ1KZb8byXBIKZjO1_xlppQ"), "title": String("Search Features")}

Execution Time: 6919ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cSQKQneCnNdHCYmuPmxeLGmG0JZu2DRNRnAzH5Be4tXABMYRXzmxGKAj1phC7Z7DZIuf2HXAu8RpQsmODcN7Pw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product that is currently available is "ekoDB". It is a high-performance database product and it costs $99.

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of the ekoDB, a high-performance database product, is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: -4ud5ChnACwEjh1N7E3gYEQ33gMpz4YAMtz_zYGQwtZblZRjNJc7BSsYHmrBEoM4Mx_Z58rUDmdMickMiP7dzg
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: DC2g7WWPHsV5bOOghEpM4TZLVy4ViAFTeIHUeBiROZLxtVeH0tzTN51YS3Q-ke8HI91cEfTmVyI_oXSOkPD65A
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is "ekoDB". It is a high-performance database product with AI capabilities. The price of the product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: qASx8q_OVTOqO3rYUzlenbrfvAiwnjZD0zkdY0mCuH7HBYOCky6A2fZ0O55jJaGHJaSuGvmamvJYO0aylY2jxw
  Parent: DC2g7WWPHsV5bOOghEpM4TZLVy4ViAFTeIHUeBiROZLxtVeH0tzTN51YS3Q-ke8HI91cEfTmVyI_oXSOkPD65A

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: qASx8q_OVTOqO3rYUzlenbrfvAiwnjZD0zkdY0mCuH7HBYOCky6A2fZ0O55jJaGHJaSuGvmamvJYO0aylY2jxw (Untitled)
  Session 2: DC2g7WWPHsV5bOOghEpM4TZLVy4ViAFTeIHUeBiROZLxtVeH0tzTN51YS3Q-ke8HI91cEfTmVyI_oXSOkPD65A (Untitled)
  Session 3: -4ud5ChnACwEjh1N7E3gYEQ33gMpz4YAMtz_zYGQwtZblZRjNJc7BSsYHmrBEoM4Mx_Z58rUDmdMickMiP7dzg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: qASx8q_OVTOqO3rYUzlenbrfvAiwnjZD0zkdY0mCuH7HBYOCky6A2fZ0O55jJaGHJaSuGvmamvJYO0aylY2jxw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("DPgtx4pbEYNyKWt76ys62yz1aqavExbaa907SxE3Lu8N2_MZNfRdTVJG9Fj1_ZW97N3rBFJtS64LcwV5aICT6A")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"age": Object({"type": String("Integer"), "value": Integer(35)}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("DPgtx4pbEYNyKWt76ys62yz1aqavExbaa907SxE3Lu8N2_MZNfRdTVJG9Fj1_ZW97N3rBFJtS64LcwV5aICT6A"), "email": Object({"value": String("bob@example.com"), "type": String("String")})})
✓ Second upsert (update): Record({"name": Object({"value": String("Bob Smith"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "age": Object({"type": String("Integer"), "value": Integer(36)}), "id": String("DPgtx4pbEYNyKWt76ys62yz1aqavExbaa907SxE3Lu8N2_MZNfRdTVJG9Fj1_ZW97N3rBFJtS64LcwV5aICT6A"), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")})})

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("YBOlQeqjfLJkH3AyuKg6H-FT166oKCZotRh6BcqUCAU9G0zYjpcVDjUyaH7ySLjx7_AfF7QriQbJwIgMwG2_0w")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("BXKEO6a2E5JDi4KepfUN3IJ7Ls9JSY65OAIpSXx7mM6FVacouq4puzDEVEr307B_GmXr307RQp0zMQMwApMaOA")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("YBOlQeqjfLJkH3AyuKg6H-FT166oKCZotRh6BcqUCAU9G0zYjpcVDjUyaH7ySLjx7_AfF7QriQbJwIgMwG2_0w")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("5BACYIhbbc7EnHSzqhHBd0Qx4DaNZ3Y2h7gRx_iWHqe7A24Tom8eaQ10BBaCgFo6iFYRT3wcASulvt8HdlkyOA")})

✅ All bypass_ripple operations completed successfully!
✅ [32mRust client examples complete![0m
