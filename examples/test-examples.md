make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'twoxWMb2mwijONFhaYC8hfML5_pOKwq-HB5KKfmv6WZEMh8s3IrsVA44mzLu05_EVF7bbAGHwebjhXaJLP8Eyg'}

=== Find by ID ===
Found: {'value': 42, 'active': True, 'id': 'twoxWMb2mwijONFhaYC8hfML5_pOKwq-HB5KKfmv6WZEMh8s3IrsVA44mzLu05_EVF7bbAGHwebjhXaJLP8Eyg', 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'id': 'twoxWMb2mwijONFhaYC8hfML5_pOKwq-HB5KKfmv6WZEMh8s3IrsVA44mzLu05_EVF7bbAGHwebjhXaJLP8Eyg', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'twoxWMb2mwijONFhaYC8hfML5_pOKwq-HB5KKfmv6WZEMh8s3IrsVA44mzLu05_EVF7bbAGHwebjhXaJLP8Eyg', 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: GVRDwJ1ctChqDW-nHlur23wm5ZxClykHrnNvvUfdGMGCmcMMLu6HmAOOGOX7dcCRGFO11nhRc7UzzQMSIvLOHw

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
        "id": "GVRDwJ1ctChqDW-nHlur23wm5ZxClykHrnNvvUfdGMGCmcMMLu6HmAOOGOX7dcCRGFO11nhRc7UzzQMSIvLOHw",
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
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: kEWSqn-dbSmjmrZ77xxu9MR_lb7H8H_r6jFV1jim64jR4XwitOsM2T0bTFxttnbg_J8icj6FfJudiyv8nNXczg
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: AkYBmBoprcsmrzi-e34ToNIGvc0NOiky_WaXcjvtl2DnMhLIaFzDBpkulih9U4c6AMC9jj0lZ6rFL-lvTmQ3yA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 36lqyvsQSlgzEOD2C1dyp-uBYZw29_g-9YlHpbYvH5TXpEwHwlSgoiOdH5HD4k2VqCKNtQWEeXx37m4RblbRSA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'status': 'inactive', 'max_score': 90, 'avg_score': 50.0}
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
Collection created with first record: Ele5YcDMXK6UicQadaaVJKMd-2evpfelNJw7d_vQdlTXykxKlen9ji30DlRoFN3IlGGHHmQDUv7ii523v-BMAA

=== List Collections ===
Total collections: 13
Sample collections: ['chat_messages__ek0_testing', 'github_cache', 'ws_ttl_test', 'test_collection', 'products']

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
Created Alice: $1000 - ID: DCHCq-BrpDqSnxYiJ2GWaqG6b6xMvOqPmgxsU4OY0K2riR6_jjKrX5dra7NL0tgHGLO9_-D1tur04-OGAJTtIw
Created Bob: $500 - ID: bPdQIcoRYKQUIdKhyqA9qPosWlNNyycS8ESpy-ASlJbHpRo3HsT44k62lrIaOXMXu5ZuG9gGcMafoF1UZ2fGbg

=== Example 1: Begin Transaction ===
Transaction ID: 153aa5b8-8035-45bd-93de-449fa6751d61

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
New transaction: 7f8418e7-c2a7-4e52-9e34-78b3f0964571
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tA-_YZLK0WRvaEJ7V9HzMNWhZa0gI7iy4KV122amfNeZMjbVGmObJc_a0EDfT8siAojswgDOPvH85l3fF2BIbw
Created Bob: $500 - ID: uZHNJDL5Ulqs4sbI57DlVp0CPxGwKATcnpzcds403Twm9ZnNnVqXKLOY-ta26acUztu5Lp5OOVd4ro94Uj7HBg

=== Example 1: Begin Transaction ===
Transaction ID: 39044c63-4817-4b8c-9028-832ac7172c64

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
Created Alice: $1000 - ID: -WhlIQs8rYl9JVlDqP7skCxR0esvEqcd2yI9ua-NFQgtt52lnH2-aVPFRbOYSYUcwkjo-91vQCYgrpVQUIdgyQ
Created Bob: $500 - ID: a2VSJEBLMNHc2EbjVX5f6XK2CVl9bCCu6cqvv-OoqgaGq1rYtmaMZEghkSAm3Q2DY8Rd8uPSN5ir2rAbMcuCrA

=== Example 1: Begin Transaction ===
Transaction ID: 2b04206f-3839-4787-aeb3-a7b43297a638

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
New transaction: 7ffca086-2727-499f-b968-0e4819022588
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: WTqmkypvFBTKymW7bdWtASgXkFYgh7PzDcqgGFnVO1LzkL3yNhIfqQzJtdZpo917U8sPc85uIPik0FM1Cw4LVA
Created Bob: $500 - ID: d8G0bINpGE-EXdlK3t-htxr2sue6tlR098j2GDjp_dlEQ9OhJzC1SReXcckcWM35mpxj1vnE7MPqYoZTd2W5Rg

=== Example 1: Begin Transaction ===
Transaction ID: 03be6cfc-6b8b-4b6c-aac5-7c369a17d542

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
Created Alice: $1000 - ID: 6a8HzzuI9crTxjD4XjnY7VOmYwDQuhhV8jCp8N7TSnxp6kHh0lIkA5k2jqAYu_BltWIr8GwKK3_H8L3ZwqR9AQ
Created Bob: $500 - ID: 9H4YaSih8zjEuasrWV56wGvs3JPZkG-mN2YpVxp6-tVPf3SCjCCodlB4SOBqb-TzfUm2HiBXJaDTwfdL9QeG8w

=== Example 1: Begin Transaction ===
Transaction ID: a95d6223-ed51-4128-9cff-f5f447aa01d6

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
New transaction: 10bda475-29c1-4f1b-ada6-448f1ba941b0
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: a5TMmdDa7301t4lUN3FYpDVFiDUgurHFHP3bryisPE2pGzP92Fu63VUf2ykEwlBxKDOHIvUokpNSCIOmw_dEbw
Created Bob: $500 - ID: Sf5V9VvHGwLxUxaqNBmE8h3R8vvYvQox97qXC-hxczjO_JzhEY4WvrMbegDpCdax_g_HiL978BuAxx9C9_7lbA

=== Example 1: Begin Transaction ===
Transaction ID: b3b06dda-c697-4e4d-8b0a-627588b18ad1

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
   ✅ Script saved: PpkyRCS_ZU96Wgv67lWP1f1ZfFXWJMlLf1knmpfYQhKC_E_O6pDchqV67dZkE2Vmc-O_QM0hfbezKLyxcZvOJQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ekK2AGha6Da5znGukJccmy1HytBM2oXk_BR1GKEofrJiH1NYdo4qW_-1dIj2jM6ODOsaV5TmcD3Aw8U8z1xFHA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: k-DWLI6nINYvDxRx0E5QfAziRosEy-emXyeLZsg7t-RGM_xbHdsHm--AVsbZ3orzeOhoR1rKGcfKFkxqF4_B9A

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
   ✅ Script saved: vymFuSaUnShVR59kiGXeZRC8lTP3UuHynKIWm3OI7sWa3gifiNDsNqmen-2ye5Xq8LTbaNcMF8VOR6K1FKjyEA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: s9Re-jxm5cfAmrunTGDMTGz6-dBWAWwiUC7ckadwf5reyOrI3Rwjkns1YAGD5Q36XcWh4uxV0tL5DqxKBihXpw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: PpkyRCS_ZU96Wgv67lWP...
   ✅ Deleted script: k-DWLI6nINYvDxRx0E5Q...
   ✅ Deleted script: vymFuSaUnShVR59kiGXe...
   ✅ Deleted script: s9Re-jxm5cfAmrunTGDM...
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
  Output: Document ID = yEARhX_Z2hCRsrOit6NtEVGSwUCqPa6Ph5DjLOeuhsyJ2CaVzupXvKdF2kOSIaRIViq3XLRtoae8rr4nT4PyBQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(yEARhX_Z2hCRsrOit6NtEVGSwUCqPa6Ph5DjLOeuhsyJ2CaVzupXvKdF2kOSIaRIViq3XLRtoae8rr4nT4PyBQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(yEARhX_Z2hCRsrOit6NtEVGSwUCqPa6Ph5DjLOeuhsyJ2CaVzupXvKdF2kOSIaRIViq3XLRtoae8rr4nT4PyBQ)
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
  Output: Document ID = 1MZVRDELHveTjb3GkhoZ_s71ct0YBAW1Dg4MSgPGjAv9OQ6hRe1liR5KMFTDOuRBgaCxqYRIDdi9G7_FRGcwXA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(1MZVRDELHveTjb3GkhoZ_s71ct0YBAW1Dg4MSgPGjAv9OQ6hRe1liR5KMFTDOuRBgaCxqYRIDdi9G7_FRGcwXA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(1MZVRDELHveTjb3GkhoZ_s71ct0YBAW1Dg4MSgPGjAv9OQ6hRe1liR5KMFTDOuRBgaCxqYRIDdi9G7_FRGcwXA)
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
Inserted: {'id': 'a38iXBgDCa7pOIl7rQyPVYsgazGCUrYDxiQS-IYkY34aC1ITFdMHa0vPMg-UGkfRJFDD_-GnrJR-XQv9_0uyKQ'}

=== Find by ID ===
Found: {'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-01-23T12:27:02.630193', 'data': 'aGVsbG8gd29ybGQ=', 'id': 'a38iXBgDCa7pOIl7rQyPVYsgazGCUrYDxiQS-IYkY34aC1ITFdMHa0vPMg-UGkfRJFDD_-GnrJR-XQv9_0uyKQ', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'name': 'Test Record', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'price': 99.99}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-23 12:27:02.630193
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-01-23T12:27:02.630193', 'data': 'aGVsbG8gd29ybGQ=', 'id': 'a38iXBgDCa7pOIl7rQyPVYsgazGCUrYDxiQS-IYkY34aC1ITFdMHa0vPMg-UGkfRJFDD_-GnrJR-XQv9_0uyKQ', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'name': 'Test Record', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'price': 99.99}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'price': {'type': 'Float', 'value': 99.99}, 'id': 'a38iXBgDCa7pOIl7rQyPVYsgazGCUrYDxiQS-IYkY34aC1ITFdMHa0vPMg-UGkfRJFDD_-GnrJR-XQv9_0uyKQ', 'value': {'type': 'Integer', 'value': 100}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'value': '2026-01-23T12:27:02.630193', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: NEaehovgojfrlz7In_B2jUYLJJPoJ6D-_QsbqwyZKll7rn833nKWBwYaQR2ztTWD3sWQjA-A2lZ9jKow4TzTvQ

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
Collection created with first record: "Lf2_MMhN9FmWqTrKRlRkXxRQvnctUm9smpi3UlPdDIyY4C5D42scQNHPujfM0KyiPVCeSWcRCianEzKg4hF5-Q"

=== List Collections ===
Total collections: 12
Sample collections: ['chat_messages__ek0_testing', 'github_cache', 'ws_ttl_test', 'test_collection', 'products']

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
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

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
Created Alice: $1000 - ID: dBbHXAndgzlgIrO7edWNQIjg8hV-Q9AE9QH0K3EtCihKeOpzGMDMwaRT76NmQUdIdRTIjy6a6nVOkDUyAidLZA
Created Bob: $500 - ID: 1fhz3MZxkzj6RU98jQ-qx56IRi0I0-44K4kCLjTn_mEe2nuUxzoaMAcjRF1UdbPc3VyaGLD7QwhRA95haYRjHg

=== Example 1: Begin Transaction ===
Transaction ID: b3ad17bb-1915-44e8-941a-61c4555a4a29

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 21b04eba-5562-44f9-bbf5-691c6b69ef3a
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
  1. Score: 25.740, Matched: name, email, name.value, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: bio.value, title, title.value, bio
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
  4. Score: 39.600, Matched: title.value, bio.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.765
  2. Score: 0.763
  3. Score: 0.757

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.382, Matched: content, title, title.value, content.value
  2. Score: 1.379, Matched: content, title.value, title, content.value
  3. Score: 0.382, Matched: 

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
✓ Inserted document: 2EAXOumjbXqi645KAYRfNtMoPmOzbE9cxP-4RC2W8uob_Hr_IL1WWhIwt3jDIxHSFLitZreRCYB_yJEZoZcuRw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: eJ39od71GyTldxsHQ3j6PZ_Qqmv2gv-P2aPDhchFtNsMJVJ9nBk8VJCmV8It59N00m_W1uLnPQzvKVuR1tlMoQ

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
✓ Inserted document with TTL: 87ES_SeNEwZcKWoMlAqSBaRZaxjcjy5RlyVfIUaqKok9sYrBwHEqK5JERFk1waTgK8dIYo0cZ9sd7e90dGURmg

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
✓ Edge cache script created: 2uawugjJDGm6J-2lGGKvubnCB2xKYOMDBGCesqRu8-XpOWQGe49VC7yW4A0qgEb7rWuuLPTAmaQalLdiCE17nA

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
Response time: 2ms (1.1x faster!)
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

✅ Script saved: ymSqbX4jFUO2TpCuPldFACl5hwuIZ9Cq-Ru7eKh_LgWZWzK43ss9tw_o6Nh7MTsZpYMr0qTbbbe9Lpx75UCCmg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: e3y9LhfQLc1dw4NzuyaaIPFE55I4UGGk97A_tJNPRr7qSGl2J6WqP-kenvBL03ikUUS9pkGGYDwkUcTASTIVZA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: LgmStIBxqM_W9J4z5a1EFY4sIeQFVomWcVY9BAkuj0EflXgKv7aqJmrIqx_lSyj9O3RAGEQjdpJiURuQHpj3zQ
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 127.2ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 78.9ms
   📊 Records: 1
   🚀 Cache speedup: 1.6x faster!

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

✅ Script saved: xja5OEnP-hhxH6355k7vBJu-FgwT57xJLmuVSUDFRsG4kdQBld8Xcw1HbkYecFd_dHQwJONCwYsKacnqaqiKbA
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

✅ Inserted order: W3l6PGA8gje4C-R6qpmKoCXFArD4gUalH8PeSNkDPwjkH_Khxud9EbknS19u3qXHjNVIs5pYelyPz0IJhRlDfg
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

✅ Inserted order: nxxZY15qP_MLaqGgDMybZIGCljFyP0P70zBlajO1KUB-ujJ17NHQSltp6dUIrT3Z9Wa5v2rd7iojykGBrSuffg
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-23T17:27:04.374352+00:00"}'}

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
✓ Created SWR script: fetch_github_user_py (8vCY1cigdt97nr3AgHEA1ZFHJMjVZOpfo_lBOTQyp1xO1YeXsJkSUh1SIalcRXKNW1jayQr0EBTo39_diQSt0Q)

Step 2: First call - Cache miss, fetches from GitHub API
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
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
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
   Vector databases offer several benefits including high precision, support for complex structures like points, lines, polygons, and scalability for large datasets. They also facilitate accurate geographic analysis and spatial querying.
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
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
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
✓ Created session: deBMeaW6ZyOXZgSVS4I7CjcdHEUe5mNs3ujERxw6PbgqG52yubBVJplrqnvQgRwQHBfnGxO5nWYgAy5o7IQrMw

=== Sending Chat Message ===
Message ID: vTbbdrtwkc8mWSvmbHhowTXOFa0K1dHQs2wiBNWTDBV8D7vKVdEhwqUt9kuIMXiwgWhfppuICPVpbAJngolN6Q

=== AI Response ===
We have three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

2. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

3. ekoDB Pro: This is our enterprise edition product with advanced features. It is priced at $299.

Execution Time: 2579ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 78
Total tokens: 532

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: olRDG0h-RBsVX_CRaDRRgLSQON-Btl8bZ_b-YyDDUVjxDBSHt_-tUEbk14GDCutEM9n8QJRpGvWmTRp6vq3rHw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. It's a high-performance database product and it's priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'context_snippets', 'chat_id', 'token_usage', 'updated_at', 'created_at', 'role', 'content'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: zOR29wOGIutIB8YS2Abnb1ahQ69bxObd1qN0vAehTVcUeLY3FKlJVFqey_muiTzCTeh6hkk8hdQVGYFPMuvPoA
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
✓ Created session: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is "ekoDB". It is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Bywkt8qlKN7sL69jUopYLVOJg9z_wfiWrl-TOdRF_q2o-7bXti07QS3ic8VuA_fqtQP5xZ5d9NSU4MfxVGG7Fw
  Parent: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: Bywkt8qlKN7sL69jUopYLVOJg9z_wfiWrl-TOdRF_q2o-7bXti07QS3ic8VuA_fqtQP5xZ5d9NSU4MfxVGG7Fw (Untitled)
  Session 2: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg (Untitled)
  Session 3: zOR29wOGIutIB8YS2Abnb1ahQ69bxObd1qN0vAehTVcUeLY3FKlJVFqey_muiTzCTeh6hkk8hdQVGYFPMuvPoA (Untitled)
  Session 4: deBMeaW6ZyOXZgSVS4I7CjcdHEUe5mNs3ujERxw6PbgqG52yubBVJplrqnvQgRwQHBfnGxO5nWYgAy5o7IQrMw (Untitled)
  Session 5: DC2g7WWPHsV5bOOghEpM4TZLVy4ViAFTeIHUeBiROZLxtVeH0tzTN51YS3Q-ke8HI91cEfTmVyI_oXSOkPD65A (Untitled)
  Session 6: -4ud5ChnACwEjh1N7E3gYEQ33gMpz4YAMtz_zYGQwtZblZRjNJc7BSsYHmrBEoM4Mx_Z58rUDmdMickMiP7dzg (Untitled)
  Session 7: crBexqJwKDpcsGuB3rJQOwiUxiEf2h11eXAFX6UNxZ6Ubw7DoeufXGKOj9ak8CZ38Y6jh3U1t1xtDGRI_nGmsw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: Bywkt8qlKN7sL69jUopYLVOJg9z_wfiWrl-TOdRF_q2o-7bXti07QS3ic8VuA_fqtQP5xZ5d9NSU4MfxVGG7Fw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'HXB1N9QYZ_zC7Bf0Bva0ScDUkKOuDgHxcyeQlVIWI6613ebcGawMNN4Amh_0QzV2hExT8F91qvnpla2Vzhp0-Q'}

=== Upsert Operation ===
✓ Upsert (update existing record): HXB1N9QYZ_zC7Bf0Bva0ScDUkKOuDgHxcyeQlVIWI6613ebcGawMNN4Amh_0QzV2hExT8F91qvnpla2Vzhp0-Q
✓ Inserted second record: OmkhFthSylaQZtKOqr4HDC9xyHNZfoGaY3tNWR72SeaKEiZGL-qooyrFYsmmgFtAPKG4PdzFYmJIbAPoBf6OZA
✓ Upsert (update second record): OmkhFthSylaQZtKOqr4HDC9xyHNZfoGaY3tNWR72SeaKEiZGL-qooyrFYsmmgFtAPKG4PdzFYmJIbAPoBf6OZA

=== Find One Operation ===
✓ Found user by email: {'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'value': 29, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'id': 'HXB1N9QYZ_zC7Bf0Bva0ScDUkKOuDgHxcyeQlVIWI6613ebcGawMNN4Amh_0QzV2hExT8F91qvnpla2Vzhp0-Q'}
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
   Inserted with ripple: {'id': 'iQLERUaAlzjuMaDGhvZSZ2Rn7JXIWD7Cwko15cdVc-1Ueb5wHUp0cn0HX6ikGe89sR1-N2CI6p2bpShnzjWANg'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'OfoK_Os8lozEIJT0POKCor0pp4xOGm6HreYWiHmx3ySUz2AnxpHyiJLt3JP6gcrHuP46TYx_HmoLEdAdaFqdzQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'id': 'iQLERUaAlzjuMaDGhvZSZ2Rn7JXIWD7Cwko15cdVc-1Ueb5wHUp0cn0HX6ikGe89sR1-N2CI6p2bpShnzjWANg', 'price': {'value': 150, 'type': 'Integer'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'id': 'iQLERUaAlzjuMaDGhvZSZ2Rn7JXIWD7Cwko15cdVc-1Ueb5wHUp0cn0HX6ikGe89sR1-N2CI6p2bpShnzjWANg', 'price': {'type': 'Integer', 'value': 500}}

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
Inserted: map[id:P7VgiWRlk-2Gx9bVuZG1HwBMcRDnrzgOOqmfQDI0vZEmWwS4pVKdIkgt2xZLtLbyvJdMm-wSk_Nn0svrwKYJiA]

=== Find by ID ===
Found: map[active:true id:P7VgiWRlk-2Gx9bVuZG1HwBMcRDnrzgOOqmfQDI0vZEmWwS4pVKdIkgt2xZLtLbyvJdMm-wSk_Nn0svrwKYJiA name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:P7VgiWRlk-2Gx9bVuZG1HwBMcRDnrzgOOqmfQDI0vZEmWwS4pVKdIkgt2xZLtLbyvJdMm-wSk_Nn0svrwKYJiA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: GEv26dPSRPt4XxnEv67qCooubqCkgLUZqEpx5QsGzWhwRUF_PZGZzp29lz7Zn0D3Vf0Doykedto4X2Ei1hZe2w

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
        "id": "GEv26dPSRPt4XxnEv67qCooubqCkgLUZqEpx5QsGzWhwRUF_PZGZzp29lz7Zn0D3Vf0Doykedto4X2Ei1hZe2w",
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
        "id": "GVRDwJ1ctChqDW-nHlur23wm5ZxClykHrnNvvUfdGMGCmcMMLu6HmAOOGOX7dcCRGFO11nhRc7UzzQMSIvLOHw",
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
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: ejjPIOkBEKs-WTxpwPfFm7_C9A6dz3GctYuNNkA8NGkXr2uobFtTVsYuYlcBXhFMqFCa8emtBYBClRFxkZiMxA
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: X29w4ehZCT_KALrIXJ96ideHA-8VYh6WATwxidYPLdNEsVirFEbiIBXm19xL2bJkdiUQHI9T893n-ICxBhg3BA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 4lpoE1AWSsJ9wIRLmC8KP-WAbDXXJ8nype0yzycfGqifu6tyrVpWLuloedceJ3FqGScXZ3PaeS7qQccZy4sImA
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
Collection created with first record: rwrliQXQAqaEt0JEF59YNhHQBUf64RFTR7G-eeU0IPBmHa4hrJqN_qQ3aShgPjL17O92WMfTdAR5oAj6zw9iHw

=== List Collections ===
Total collections: 16
Sample collections: [schema_products_client_py chat_messages__ek0_testing github_cache ws_ttl_test test_collection]

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
Created Alice: $1000 - ID: UGXoVSK5zbKzQf-56qSfwjnVGf0-Y5AaBDU61d_gc6qxwdGh7JmZUBa0FVmQ3S6DxPavmCUHvJkKDcxg5eF4IQ
Created Bob: $500 - ID: aQV2kVDNFIoaSHpE33NmnAcZFclNA60KnZhuBRRhC1lu3VWGGyDuByaA5EZeiL_MwfKqRdKcg-5pAWOyb-ONGA

=== Example 1: Begin Transaction ===
Transaction ID: d10e17a2-53c4-4812-9ed7-c663a7863d89

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
New transaction: dd091a9e-556f-4bd7-991d-4bd0d3537cb8
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
   ✅ Script saved: IcLdGUWzJVbVgcWxR9NEFY7pkpdR6FqIXonfejxbbf0zXj-Dtb9Tpt5mCD-rqqR6fv8P0gd-Zb1JPYQYu_AO0Q

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: j4U8TSieG5mihJF5GKoUe5hSqck2uGDVayo1hJGdEtQXmjMoKaY2PGRCDUGCS60fOXceA1eg30iML1VQDow-LQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ANt1llVfi21JG_9ytSbYjmF4gqZTfTgdRljalCFfzf6DiNjI9W47bnE7DjkRbMzTFXbsGOvPSjggGv3UvXU73A

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
   ✅ Script saved: jETWc17CmVoEeEjgOZEYfwaa_aQ-apGk3UD3m3V0bx-E0phR6DkGYk76VgiDqajfzgVDa5o4hqRJOSgTQ1WrPQ

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
   ✅ Script saved: 7eXra-z0E1pc8sy8OFgNsYMmUN2SmZ8dIQHTMUJmZMvoCthRLbm9SjaBLzrAwLxe3CyqsEXHsHeDplnS-JDKVg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: IcLdGUWzJVbVgcWxR9NE...
   ✅ Deleted script: ANt1llVfi21JG_9ytSbY...
   ✅ Deleted script: jETWc17CmVoEeEjgOZEY...
   ✅ Deleted script: 7eXra-z0E1pc8sy8OFgN...
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

2026/01/23 12:27:39 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = PfM4Q7Eot1IaF8_NCMaISEYkNer7HKvuWM5RL7EqVhcO_oNM_5gNgmv8cLYZjDI_e1Osk6CHZMNuYX8Kk2Ub5w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(PfM4Q7Eot1IaF8_NCMaISEYkNer7HKvuWM5RL7EqVhcO_oNM_5gNgmv8cLYZjDI_e1Osk6CHZMNuYX8Kk2Ub5w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(PfM4Q7Eot1IaF8_NCMaISEYkNer7HKvuWM5RL7EqVhcO_oNM_5gNgmv8cLYZjDI_e1Osk6CHZMNuYX8Kk2Ub5w)
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

2026/01/23 12:27:47 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = WddD7oGdJ4JcLmQci4xGozEaMzvOVmszZmIZLwu-BqkA0E24RtYV027oTLvkymTNMKe_6O_e4hwfxq9DDg4rcQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(WddD7oGdJ4JcLmQci4xGozEaMzvOVmszZmIZLwu-BqkA0E24RtYV027oTLvkymTNMKe_6O_e4hwfxq9DDg4rcQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(WddD7oGdJ4JcLmQci4xGozEaMzvOVmszZmIZLwu-BqkA0E24RtYV027oTLvkymTNMKe_6O_e4hwfxq9DDg4rcQ)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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
Inserted: map[id:retr1TCNXs8OxHItWt0_abieIdnwSsAjxDah7836KvplXnYc-ILCh3wszOVwRhW59lGyGTL-E5v6jOQQLvoM9w]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-23T12:27:54-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:retr1TCNXs8OxHItWt0_abieIdnwSsAjxDah7836KvplXnYc-ILCh3wszOVwRhW59lGyGTL-E5v6jOQQLvoM9w metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-23 12:27:54 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-23T12:27:54-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:retr1TCNXs8OxHItWt0_abieIdnwSsAjxDah7836KvplXnYc-ILCh3wszOVwRhW59lGyGTL-E5v6jOQQLvoM9w metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-23T17:27:54Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:retr1TCNXs8OxHItWt0_abieIdnwSsAjxDah7836KvplXnYc-ILCh3wszOVwRhW59lGyGTL-E5v6jOQQLvoM9w metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: hTiC0Ej3hTpQ4tGjuiMTCyV-tPnrp8O3EyXWwKPOyjbZOqnxdg4SU_u1b8FWD7KIE7QgnTwcIGcenyLwkrx7BQ

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
Collection created with first record: kNPaBdU4pEe-BHS-AeK3z0reVS6K4SPtSAi4jeBEDO4vwOP5QGCd3xQ_mcjDOdV-eXA7TIazWzB76LsrIfF4BA

=== List Collections ===
Total collections: 15
Sample collections: [schema_products_client_py chat_messages__ek0_testing github_cache ws_ttl_test test_collection]

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
Created Alice: $1000 - ID: 6kRsHr3VXGqL0QM-XULMoxrIsFHsAP2DO610SUMe7xS1aaDkqiWOHMIVitIAYXxBgWh9CufChMVREhMfEOLZaA
Created Bob: $500 - ID: nF8pp_FSqfzfBV37VcRoEqGxcMT1LxfVlT9KqZwPcG8sT29HZiOKtYk1_nSTk8VJymeMERdhkef6rVyGjDg7RA

=== Example 1: Begin Transaction ===
Transaction ID: 6ec7b52c-9f9e-40db-9dc2-63f362cb387b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: d4d0bef9-a0a4-454d-b508-1d61b6aacce4
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
  2. Score: 0.751
  3. Score: 0.749

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700
  2. Score: 1.511
  3. Score: 0.300

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
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

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
✓ Inserted document: u7jR6e-22jznIBZeyT60XF7mxABrgE27Du7LijmxYj2cZsGWmTjgvrlApZn7mfZ4hy8SQ7iTAAYrt3NhmZV_3w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: R_pyJUX9Zki6JfZUo7ovOx2mKjoR6K0pmyDewvSRFT0BtG54HUlVM2W2-eUm1vlkRqW3eLLHQn0unzXQnhgK3Q

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
✓ Inserted document with TTL: iK78lLppZXhWIZwn5yDcbbeYR416r3oJZt0-Gofl7htyNaGqycRMJGzD1w7JjUlLAFnEFUtQlUxNEXSmS5O2-w

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
✓ Edge cache script created: YYfJeXm9Qa_IE8_08nxMslGDSWItH-1qef46Tmn4f74abye6bM8wAAUOedE5F9Ar1lV74eK_scPbYBXcxnGHOw

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 56BuegBS2Hbz-mppiQ-1YdHkl5x_vU4R9GDgMCrSRISFrRH8BDwgz0Ig_6VQmZtPXLeG_eTsGDcbvalqScaozw
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 165.409667ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82.577167ms
   📊 Records: 1
   🚀 Cache speedup: 2.0x faster!

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

✅ Script saved: 6MCoP74yguQZOgCEWOoRHpcOT9BKjPQ6bv_c3nJ_NASAPUH03yHkhzE75-UL5_GK54N5yPjWpI71wR_ETaQrGQ
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

✅ Inserted order: gL4QWdO_WLlNe-r8lJe_HqXOHfWhNvhfeW9XUyriLak3-t1MdAL2ri6js4iWEVbnW7yc_QwRmgwzL8TH3lbLAQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: TTkh0N2EIIr4zDoKqA_Tmb6b4IN_1dpfOz7bb9oH9ZI2WUJ1qHRDO75X4uwswqLl5VRkkhjcq4sWlXI-eeimCg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: QdRRbxEuJa8YEwDlxTjR3NtFGi3P1ivYFba0witj3ECNQQhCwEC2K3ct4KT_XvHwjbuGYcfJrB35xzEbgF_dRg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: 1EmzfOA6-fnvepnLJpV9s-Nr9WhIxKteHNFmXzeshWBIzBd8kMvRYYQ8ExVvuEJnkOzlFkzefR8hu0wID0qrSA
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
✓ Created SWR script: fetch_github_user_go (kT2HqowY323u4Y45Y5XiNEtK5zZ70ZWUwzKwIStuKTGi3qosXhLP8NOTPpuiT0JYKrRS3UOkQ1spMVmc2dxnmg)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-23T12:28:03-05:00",
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
          "followers": 279915,
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
      "id": "ObakaR3uMqws5oF8fMNFtT6X0Fw4nrmyqY9BvxKscH6LcZX_NuMYOtLOB2Bn7dm8QioKjmOuNuJ64in6PcA4Rw"
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
Response time: 70ms (served from cache)
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
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   5. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
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
✓ Created session: GFi9nITIlmmDiWZfKTsqe6vfG12LC7fx6h3DUO-A8GnXij598gQSbfMBnLLFF371mGI3CoYy-ndoWqrvEIJSvA

=== Sending Chat Message ===
Message ID: lzAKtS-AefggWhd5afaX2V0Jn-ZJAMLVy-PTBFpXNhd9zozCw7CovIyzrbZfhyZMoKS_Sd1jT7rqtC8ZMdqlRw

=== AI Response ===
The available products are:

1. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

3. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:8Vaz2-2O2Cp0F35gGXQNgBV6ws21cPheVwIKEHuk9WV2t8wBP7O1ycjSri_Ka3DI2YzSGz7wzACLvNpEbDy4Gw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:cxIpm_KDIyl010jRQp2mWzNBHmuGDkZLvEM5g0so65w3IRZjnSeBUQr1-55ISKUuxTFQ7V0XQKeMpZHA51-Zlw name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:36MmNCrVmYyk43S4jnTT6Q7fSr34z7MlPcPW2rlJOCD4f1jWz4zVhVJO-I79DUOFGqS1PnKtI8eOlPLaEjREgg name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 3324ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 77
Total tokens: 531

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: vyFOQdM_-mtCbS08tXqiTw5oRiyWm5ddmT2M0arG49vrwddPbRyLue-FfPxBqQjcz5Iy0y3okLk6eopP_oUiIA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is the ekoDB. This is a high-performance database product priced at $99.

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
✓ Created second session: 8djIhno3iGEPxBoj-qxsGY1Zpx_yFB-AEO3e3JulWsFnGJE73TCbQ-BN95kEtrpkrz4vmqG1UpPQSqVXz7qukA
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
✓ Created session: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, there is a product named "ekoDB". It is a high-performance database product. The price for ekoDB is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ryewnfyMy-d70KkjeqbmqMaWpXyFiZzpAW9lOoHCtjmxKQ84gVi3lmSezoEl0U33Oea3A-nRyrlK1SweydmOPA
  Parent: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: ryewnfyMy-d70KkjeqbmqMaWpXyFiZzpAW9lOoHCtjmxKQ84gVi3lmSezoEl0U33Oea3A-nRyrlK1SweydmOPA (Untitled)
  Session 2: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ (Untitled)
  Session 3: 8djIhno3iGEPxBoj-qxsGY1Zpx_yFB-AEO3e3JulWsFnGJE73TCbQ-BN95kEtrpkrz4vmqG1UpPQSqVXz7qukA (Untitled)
  Session 4: GFi9nITIlmmDiWZfKTsqe6vfG12LC7fx6h3DUO-A8GnXij598gQSbfMBnLLFF371mGI3CoYy-ndoWqrvEIJSvA (Untitled)
  Session 5: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg (Untitled)
  Session 6: zOR29wOGIutIB8YS2Abnb1ahQ69bxObd1qN0vAehTVcUeLY3FKlJVFqey_muiTzCTeh6hkk8hdQVGYFPMuvPoA (Untitled)
  Session 7: deBMeaW6ZyOXZgSVS4I7CjcdHEUe5mNs3ujERxw6PbgqG52yubBVJplrqnvQgRwQHBfnGxO5nWYgAy5o7IQrMw (Untitled)
  Session 8: DC2g7WWPHsV5bOOghEpM4TZLVy4ViAFTeIHUeBiROZLxtVeH0tzTN51YS3Q-ke8HI91cEfTmVyI_oXSOkPD65A (Untitled)
  Session 9: -4ud5ChnACwEjh1N7E3gYEQ33gMpz4YAMtz_zYGQwtZblZRjNJc7BSsYHmrBEoM4Mx_Z58rUDmdMickMiP7dzg (Untitled)
  Session 10: crBexqJwKDpcsGuB3rJQOwiUxiEf2h11eXAFX6UNxZ6Ubw7DoeufXGKOj9ak8CZ38Y6jh3U1t1xtDGRI_nGmsw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ryewnfyMy-d70KkjeqbmqMaWpXyFiZzpAW9lOoHCtjmxKQ84gVi3lmSezoEl0U33Oea3A-nRyrlK1SweydmOPA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:aC5BVegqTvO6SHCW0S3_kCTSVzlky9LNEdFBZbndoJlEoChxO33bFx5EP8H6Aa2swmM1FHoMZVSjLbq6ZGFDkg]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:aC5BVegqTvO6SHCW0S3_kCTSVzlky9LNEdFBZbndoJlEoChxO33bFx5EP8H6Aa2swmM1FHoMZVSjLbq6ZGFDkg name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:G9rJXkFK86EhDYOPvU5IuNwc8POMc-_G5ip0LjpJQw6w1ux1PsW_4PYZqNyz9ogSIAXfoFscSuWnWR4Tsa8WYw]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:aC5BVegqTvO6SHCW0S3_kCTSVzlky9LNEdFBZbndoJlEoChxO33bFx5EP8H6Aa2swmM1FHoMZVSjLbq6ZGFDkg name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:mJ8ksz9vRSydj1UZpplgm3u0MIGLGFNKAzZBsar2YDnZKZS8Zb5M3yNvCdi6EKJHIsqK-7gaf-650-9FlMtGfw]
Inserted with bypass_ripple: map[id:fJH4dqsxiUn30yrI0ETAeLhoYjf_Jv6YVnARvMF_tEoo8u3UPnOo43kwSMr582lcW0-24tyWKkr6paCpwtvLQg]
Inserted with TTL and bypass_ripple: map[id:mLCPjUDcyCoVvvoQSCXd9C53pTpNHf-VuJY_LN7EAOUb4_D_BBjQUiswnCbl5d4Rq-oaj1pBDV2Gcr4qM2WfaQ]
Updated with bypass_ripple: map[id:mJ8ksz9vRSydj1UZpplgm3u0MIGLGFNKAzZBsar2YDnZKZS8Zb5M3yNvCdi6EKJHIsqK-7gaf-650-9FlMtGfw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:KDCS95jFtz5bMiRo_tFLeXLzGGu9OQImIymApwfI_u6f2wWwf1Z1VgUdtNPk_-EK74qpFrQ1ka-AU6cBjc11Sw]
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

added 1 package, removed 1 package, and audited 13 packages in 849ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'qLesd_k_z1VhCq7Fq3ttcTJu7Yy8mT0jQCZD1kWaq0zov9tW2Sdhlf49oolE7OKuYgdaFg-jhIMUT5e55lFjIw'
}

=== Find by ID ===
Found: {
  metadata: { key: 'value', nested: { deep: true } },
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'qLesd_k_z1VhCq7Fq3ttcTJu7Yy8mT0jQCZD1kWaq0zov9tW2Sdhlf49oolE7OKuYgdaFg-jhIMUT5e55lFjIw',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  price: 99.99,
  name: 'Test Record',
  value: 42,
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  created_at: '2026-01-23T17:28:26.655Z',
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-23T17:28:26.655Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { key: 'value', nested: { deep: true } },
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  id: 'qLesd_k_z1VhCq7Fq3ttcTJu7Yy8mT0jQCZD1kWaq0zov9tW2Sdhlf49oolE7OKuYgdaFg-jhIMUT5e55lFjIw',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  price: 99.99,
  name: 'Test Record',
  value: 42,
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  created_at: '2026-01-23T17:28:26.655Z',
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  price: { value: 99.99, type: 'Float' },
  active: { value: true, type: 'Boolean' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  created_at: { type: 'DateTime', value: '2026-01-23T17:28:26.655Z' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  id: 'qLesd_k_z1VhCq7Fq3ttcTJu7Yy8mT0jQCZD1kWaq0zov9tW2Sdhlf49oolE7OKuYgdaFg-jhIMUT5e55lFjIw'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: wxlj5ADdj183wgWjK3sdeW2AiUIXy7GDfaSfw89xtzWA6KxJHqi-dABZVUTzlZpv2W3Ga_ut1nb-HI2HD5VIwg

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
Collection created with first record: mJ5pkgrPPzdPWnqAwIJ4tj5hurs_yfK2mC0wbtO2BipEcW1W-1oXKuXhBuGt4l1Yt_g71u3iPZ08gS1DxGPIGA

=== List Collections ===
Total collections: 21
Sample collections: schema_documents_client_go,schema_products_client_py,schema_users_client_go,chat_messages__ek0_testing,github_cache

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
  cache:product:1: { value: { price: 29.99, name: 'Product 1' } }
  cache:product:2: { value: { price: 39.99, name: 'Product 2' } }
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
Created Alice: $1000 - ID: GtGgIEXnlL0Kpxp4fYVQ-F2pWxioLCyaf8VTETuOTulMEvCuU-mWUPeSQUWWKK4EOJmfvsQLDWwvt2wVebA-FQ
Created Bob: $500 - ID: 76lIXOSsM9ros-jG25WpH5QOgrWQVl-c0P8iLS4ncsXY46KwBHwfSjWVGwGnaw9JTD0rTNH952_Wy6BMevEePA

=== Example 1: Begin Transaction ===
Transaction ID: 6dfd2d87-a85b-4745-8113-325faed49e6e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 36357267-f0ce-4ed6-b870-b8716d472a53
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
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title, bio.value, bio, title.value
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.746, Matched: 
  3. Score: 0.725, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.708, Matched: content.value, title.value, title, content
  2. Score: 1.498, Matched: title.value, content, title, content.value
  3. Score: 0.290, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, skills, bio.value, bio

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
✓ Inserted document: eKFgihhIUoSZ9cEhqb8QeG3t-tqqFd6QuTUyb9YHKSBCzLbeuJDd60VoTFyAUVrvsnDd7rVfHNNYNbbl9nPb6g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: rNMasSX0wzjRkgKv_tvOVVbvLe7rECdh_jIBIlkTJrP-Fcj9QC5UWX-HZeS3-HO3S2lwegFgRlsgrht61lillA

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
✓ Inserted document with TTL: pW7ezbVYUSY2tx00VsBji8jKVy4-0rkFdSM745ssG-sFuUmy1-8rMT3zFRJ2QOw5TGn4dMU305FyTTDhgtPavA

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
✓ Edge cache script created: kLWiXEjuFhHJOyncprTu9vC3eMFF1HXlsKnRgB_apiqh40_YYKawnC1_3b-RhnHM5cqcAqz8Wp55JjgWCzQofA

Call 1: Cache miss (fetches from API)
Response time: 4ms
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
Response time: 3ms (1.3x faster!)
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

✅ Script saved: zCcj7Yh_u28JPKUBYyZBZd_f77jdwr-4Z9-p8msveWeX8cV8SsAVDEERhQYFI8_kUE-sjmXif7IEZBoC7VjrHw
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: izB1IlvuqNNpdIgQCEGyoPbpx10ednXa_CbKehPv_lpARheNpO54LD3Mirz6lgcp5ygvuh1HkP7rIzezG0fIWg
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: TbvJ6xJ9jwmj4dWYjcd9EV1gH5tYa-SqyhDb-RBdYP9FSWpiXxdSF0VGLReUogu30smkm0hm4KtbLu9uqtG4YA
📊 Statistics: 3 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"avg_score":60,"status":"active","count":5}
   {"avg_score":60,"count":5,"status":"null"}

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
   ⏱️  Duration: 111ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 66ms
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

✅ Script saved: VTxNX8t3U0OGzhuPrXp2LY5jyWkaiYX9eQ3JzL8NVKPhH6CZcjQ3thZLJ98WwhaQ_ZqoqPefNe7jqxadjpoh2w
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: f5rPeQZICCMzIuh0gMg7SDTfgG9gw76Odx5bZQ2S1wnH60mBnjjoWbaQ9tcd459dvIe0covkbIhFwqdLAzsmUw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: Ux4Ovv63FKgBkzV214qI8GausUCOYBuZErQNnDOXknHLHCWd7DELal23sSNQAJtmnEcg4Zjd76dhbpTddJlS4w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: apCzuWMNt-vSq7soGoJKjh4ADWdi985vd1OKJ6baHnbC2SM86YEALHKfjOQo-hmG6eGkGinNDhwfpTBX3sgaLA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: weBWujmb4Bcqh6mJGn9Kx5nk7DWvwMYUSxEOB3IHwOoHJN27GOv4ocYi2foJ9KPFaYfnk4Vru8ab3MFzIE6myA
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
✓ Created SWR script: fetch_github_user (xz2Ryb-OTYJ5zuU3Dk10EjNgJMQDU4Q3qhZ-Vkk1sAXOzoPm_g7r7zvJ03H6BfhWS6tqvDTyxaOPIxt_PAU71w)

Step 2: First call - Cache miss, fetches from GitHub API
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
✓ Created enrichment script: fetch_product_with_reviews (qKZubPZ0_RasWjqh21EZD6X542q-U9sojg8M1uirege3GViuoWGyzacJVQmHYcUguKBh2vGRLbQzWIprhDqXHw)

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
   {"category":"Furniture","count":3,"avg_price":365.6666666666667}
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
   Vector databases offer benefits such as efficient storage, accurate representation of data, high quality graphics, and the ability to perform complex spatial analysis. They allow for easy modification and manipulation of data, and they preserve detailed geographic information.
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
   {"avg_score":70,"count":7,"role":"user"}
   {"avg_score":20,"count":3,"role":"admin"}
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
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":3,"category":"Database"}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: pOGOOxmPEbbRsJQtqdm6xQIKSSnyBzYzNv2ua9eCwXmKq3v8FBzOS04-JYqwjkZtfjTvWdttPNU8rzG222inrw

=== Sending Chat Message ===
Message ID: ew9ZyQ2M1AVQcem6Unf-25OVOaHmh0KIJD3dzTL14D22PZyIiKw3jcyvI7eKH35TgwmhHU06sbNIuYKllszDvw

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Cloud: This is a fully managed cloud database service product, and it is priced at $499.
2. ekoDB Pro: This is an enterprise edition product with advanced features, and it is priced at $299.
3. ekoDB: This is a high-performance database product with AI capabilities, and it is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'm7AORkjyNAi3lZZxu7TkW3nowUrpDzczScSffhssul5C3fyy6N7dH1bR5rld-0F7KhNknO7MqiTWLIHsAx8CxA',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'MF8qWQw_L1JXsP3H_PejnLqwIcpl02glJDs1R1oM3cKlBohfmjbk21EvoU7ByTEhpwz9Io2a06guAIIsLVdlCQ',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    price: 99,
    id: 'u3GqLVum5hAdEWq78pXa_mgiJXZU0XG2-sY48JJaYXSTErvAgoW8P4BFSGJpShggU7DKn2trEKGY-WWw9aPyhw',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2297ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 85
Total tokens: 539

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ktqC05OF4cf6AtcyXHkFgjdF6QDeiV-gPhXCX22I8qK-TZOqOB4elWQqc_CGQm3v95nDec8ZSo7wpceqtNhOpA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: 99Pn4ItmLj9yNXdJ9MHXaR_S6DJOS752ZPS7m0yTb1fLPcBFhVyB3y20uMRIHqLljr2_IjUyTEO3fa5MBjqN5w
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
✓ Created session: N7x7pp55YXw3bqdV-U7D_qqytgmhA6oUr6eEbffzzFO5Fpem9r1yPKwYwlDbOSWNFwpwNJ6GxPfwL_VjF7Y-7A

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is "ekoDB". It's a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: sZYkMaEjhIGpW3bngpaLj13oVv8s6D4hCducfEDhglLfhL7vK3fWAgwpLolyIqRzSJ6HZT7LPDo4XFrpMW2rxw
  Parent: N7x7pp55YXw3bqdV-U7D_qqytgmhA6oUr6eEbffzzFO5Fpem9r1yPKwYwlDbOSWNFwpwNJ6GxPfwL_VjF7Y-7A

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: sZYkMaEjhIGpW3bngpaLj13oVv8s6D4hCducfEDhglLfhL7vK3fWAgwpLolyIqRzSJ6HZT7LPDo4XFrpMW2rxw (Untitled)
  Session 2: N7x7pp55YXw3bqdV-U7D_qqytgmhA6oUr6eEbffzzFO5Fpem9r1yPKwYwlDbOSWNFwpwNJ6GxPfwL_VjF7Y-7A (Untitled)
  Session 3: 99Pn4ItmLj9yNXdJ9MHXaR_S6DJOS752ZPS7m0yTb1fLPcBFhVyB3y20uMRIHqLljr2_IjUyTEO3fa5MBjqN5w (Untitled)
  Session 4: pOGOOxmPEbbRsJQtqdm6xQIKSSnyBzYzNv2ua9eCwXmKq3v8FBzOS04-JYqwjkZtfjTvWdttPNU8rzG222inrw (Untitled)
  Session 5: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ (Untitled)
  Session 6: 8djIhno3iGEPxBoj-qxsGY1Zpx_yFB-AEO3e3JulWsFnGJE73TCbQ-BN95kEtrpkrz4vmqG1UpPQSqVXz7qukA (Untitled)
  Session 7: GFi9nITIlmmDiWZfKTsqe6vfG12LC7fx6h3DUO-A8GnXij598gQSbfMBnLLFF371mGI3CoYy-ndoWqrvEIJSvA (Untitled)
  Session 8: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg (Untitled)
  Session 9: zOR29wOGIutIB8YS2Abnb1ahQ69bxObd1qN0vAehTVcUeLY3FKlJVFqey_muiTzCTeh6hkk8hdQVGYFPMuvPoA (Untitled)
  Session 10: deBMeaW6ZyOXZgSVS4I7CjcdHEUe5mNs3ujERxw6PbgqG52yubBVJplrqnvQgRwQHBfnGxO5nWYgAy5o7IQrMw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: sZYkMaEjhIGpW3bngpaLj13oVv8s6D4hCducfEDhglLfhL7vK3fWAgwpLolyIqRzSJ6HZT7LPDo4XFrpMW2rxw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'vWyyTVb8qswLl22JQDFynl_o_yqExzqe-SrNWDLJKhpZt1kBEbXLaPFy3L8rogYgDgdzPv5LAsC0lxRxnPJ0bg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'vWyyTVb8qswLl22JQDFynl_o_yqExzqe-SrNWDLJKhpZt1kBEbXLaPFy3L8rogYgDgdzPv5LAsC0lxRxnPJ0bg',
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 }
}
✓ Second upsert (insert): {
  id: 'mqcvGgoVi4xJHwMxjlWSKj_QWcunLF4i4lBVJtCwpxsKW0z-g6coLlD6eq_rT4iEsZUmAfzUFEmPnuD_b_xEtw'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
  id: 'vWyyTVb8qswLl22JQDFynl_o_yqExzqe-SrNWDLJKhpZt1kBEbXLaPFy3L8rogYgDgdzPv5LAsC0lxRxnPJ0bg'
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
   Inserted with ripple: {"id":"WmzOIXqMs1kPwttTT5b7TNixVrKPsl0qQNKWzuBu4lriSS9hH_6pesaabqNYNCMVE_5S2TuntLYcg4j0CXcgJA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"XdhS5gWK0GOMlqC8uaTjW-h2ctaIm1HB3IFSxdQVu9pw4904IZmXSlklugz5aW5-qzV8lXTsLEP0cHw0kF53AA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"value":"Product 1","type":"String"},"id":"WmzOIXqMs1kPwttTT5b7TNixVrKPsl0qQNKWzuBu4lriSS9hH_6pesaabqNYNCMVE_5S2TuntLYcg4j0CXcgJA","price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"BMLy6fJByX4wbIaqpmfs-zC8V6KO1-PXIFz-ZxSVrTGz1aglzxvK7PfJAb2MNQyR8xliC1m5RAeZO7liwZ7Ptw"}

✅ All bypass_ripple operations completed successfully!
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 871ms

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
  id: 'YCr-OwPkGLvhvowS44pRR7IZFEDmsDu0MblW4o9PV41gFrPTamjBcAacenHajkQt8AyE20F1ec2GtwiNJ6wPrg'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  value: 42,
  id: 'YCr-OwPkGLvhvowS44pRR7IZFEDmsDu0MblW4o9PV41gFrPTamjBcAacenHajkQt8AyE20F1ec2GtwiNJ6wPrg'
}

=== Find with Query ===
Found documents: [
  {
    value: { type: 'Integer', value: 42 },
    active: { value: true, type: 'Boolean' },
    name: { type: 'String', value: 'Test Record' },
    id: 'YCr-OwPkGLvhvowS44pRR7IZFEDmsDu0MblW4o9PV41gFrPTamjBcAacenHajkQt8AyE20F1ec2GtwiNJ6wPrg'
  }
]

=== Update Document ===
Updated: {
  id: 'YCr-OwPkGLvhvowS44pRR7IZFEDmsDu0MblW4o9PV41gFrPTamjBcAacenHajkQt8AyE20F1ec2GtwiNJ6wPrg',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 }
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
✓ Inserted test record: hsBEYksk_N6zkkzB9-tkqrnaCLvYVuUYMFtehcwaVrw5rIH6TnPg6Xlr0e05XeeTkwHIzA2-KNLTC-MbbVbxEQ

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
        "id": "GVRDwJ1ctChqDW-nHlur23wm5ZxClykHrnNvvUfdGMGCmcMMLu6HmAOOGOX7dcCRGFO11nhRc7UzzQMSIvLOHw",
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
        "id": "hsBEYksk_N6zkkzB9-tkqrnaCLvYVuUYMFtehcwaVrw5rIH6TnPg6Xlr0e05XeeTkwHIzA2-KNLTC-MbbVbxEQ",
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
        "id": "GEv26dPSRPt4XxnEv67qCooubqCkgLUZqEpx5QsGzWhwRUF_PZGZzp29lz7Zn0D3Vf0Doykedto4X2Ei1hZe2w",
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

✅ Script saved: S4PgIgAOSRsBLSHvFg7K6p2xW0RVAuMuGYOJ8agGpKtv5602HlLqBfbuomQ2D1RfST8TWKRxIloBlGtZlIHFiA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Qo6DQhdEZkvGdLIWI_BykHDJ6U15LVPphwvpeLdrbeCjcwVt1z09gJCB72xKRUSloym5IjXiQpjyVnC77OeOXA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 9SfcaJnUB8IIcWWs-2v05VcKe7Vo7GyTqHmrVR4zftnFZE_NCpPssMQu9xA_ILqfeyPZaindZ0EgYPfMLKozvw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"status":"active","max_score":100,"count":10}
   {"avg_score":50,"max_score":90,"status":"inactive","count":10}

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
Collection created with first record: LINAd-q2gqo-cryTJGSc_Xw9WDyYJNMjJmqV82uN1CEIY7AzbV4HXqoA_9NAs_mR3o8JuRNocS_jt_f-vwWmbw

=== List Collections ===
Total collections: 24
Sample collections: [
  'schema_documents_client_go',
  'schema_products_client_py',
  'schema_employees_client_ts',
  'schema_users_client_ts',
  'schema_users_client_go'
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
Created Alice: $1000 - ID: BoIPmiOW-2fubryEBjKum3LAt2Iv741Xtg3N7-nhFZAUd2CnGQOYfHLyrFwQU3MPkbOVwW9jexU4xIZXEoiofg
Created Bob: $500 - ID: vpJb7Sz6eQzYsL9WQsr0i8z0ENDv11eOzPBEdakaVKTfT2xteV48YMuOTJndsMV0PMUeXWv6Y_-ISsqKbc3QeA

=== Example 1: Begin Transaction ===
Transaction ID: c47f55ec-bee4-4ac1-ba89-62ab0af14432

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
New transaction: a3cdccda-b062-4403-8439-c7d223dd9f3a
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
   ✅ Script saved: -n5AJQqkgWtN04RCiJB1nyJoGBHYLsDvOQ3OIrnVa5ffYIJbqa3_OliVvOApLSocjISozN2Yl58LXcQeytieUQ

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
   ✅ Script saved: kXIyoUuGr9i4E3iwLXhgd3n9L0-9rNsomYAIiCuQ2EaSZCGqTdcJPjQKEv8LfOa7RbzEsnPAlJieTqLz80bWfA

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
   ✅ Script saved: 0UU-0FmprFEocJSyYrDcWyrzNODvigoA_GSzTAHrYp4r2OzOdLxmRzHy0auJyEUo0L3hwzYBVxXciYhoAWJUXw

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
   ✅ Script saved: sqyuMhslLR0A9VWURxKE2eh8tElCY9Gt8lhvMSDzQxDQhXb_x2afbAzCC5UC6Ca86rpOlm-AFBqQbtOJTGnLSA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: -n5AJQqkgWtN04RCiJB1...
   ✅ Deleted script: kXIyoUuGr9i4E3iwLXhg...
   ✅ Deleted script: 0UU-0FmprFEocJSyYrDc...
   ✅ Deleted script: sqyuMhslLR0A9VWURxKE...
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
  Output: Document ID = aTbrAd-oAfJsat0bSfCsbup1vMVoPUDV7Kzrzc4FaLlJU-eKobvYsbj54MQyzwViyt7ew3A9umF9deA8zgKCbA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(aTbrAd-oAfJsat0bSfCsbup1vMVoPUDV7Kzrzc4FaLlJU-eKobvYsbj54MQyzwViyt7ew3A9umF9deA8zgKCbA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(aTbrAd-oAfJsat0bSfCsbup1vMVoPUDV7Kzrzc4FaLlJU-eKobvYsbj54MQyzwViyt7ew3A9umF9deA8zgKCbA)
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
  Output: Document ID = gTSrgQXowYMHsFJ8rLhf1aaQEcrxOObrvZnGyLs6OfYxG_fO4tloqhnS06IQKQlIpnOsgPhyc02OaxpCayeMqg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(gTSrgQXowYMHsFJ8rLhf1aaQEcrxOObrvZnGyLs6OfYxG_fO4tloqhnS06IQKQlIpnOsgPhyc02OaxpCayeMqg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(gTSrgQXowYMHsFJ8rLhf1aaQEcrxOObrvZnGyLs6OfYxG_fO4tloqhnS06IQKQlIpnOsgPhyc02OaxpCayeMqg)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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

added 1 package, removed 1 package, and audited 9 packages in 830ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'hnDaYX2CuaSlgpWJ4FeEm3hNfYdZJBlguzfWYcy2um58QW6_ttFZAHQw0wToi7ziVFozpLIY1KENsuLgp4TTvw'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'hnDaYX2CuaSlgpWJ4FeEm3hNfYdZJBlguzfWYcy2um58QW6_ttFZAHQw0wToi7ziVFozpLIY1KENsuLgp4TTvw'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'hnDaYX2CuaSlgpWJ4FeEm3hNfYdZJBlguzfWYcy2um58QW6_ttFZAHQw0wToi7ziVFozpLIY1KENsuLgp4TTvw',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: aUEKQyqsrH66zsDGmplb04FVMFxzugsSOL9mW9NP7B8Ct4OHMge8qu6eKQBMvXmQxIEoC097OKhl1bbMdkZB2g

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
Collection created with first record: MdFA0RUBh3VsFEJFU5tmuR0cVu5UYzTGnd3APTVT_gt3JtN5KednI3QJt7WNqNySSZ-dp_XHRNKPXbMcipNdhw

=== List Collections ===
Total collections: 23
Sample collections: schema_documents_client_go,schema_products_client_py,schema_employees_client_ts,schema_users_client_ts,schema_users_client_go

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
Created Alice: $1000 - ID: 28MJ3cLMfEHiRN1hkTsu4ekAToBiosdloFOhurNsyRM5JeIMrCwrWul_LUzB0cng17B8jj-TyRiQiB7M3Kz1sw
Created Bob: $500 - ID: 26tkvYhp0BEbpIAwKBii8TSGQocsLO9fOl8wZATOmwjQY73LkBrK-yA5FNq8bSgJtAGnjQVPUdSnR6a-GLoqIw

=== Example 1: Begin Transaction ===
Transaction ID: 79910039-85e5-4c8f-aebc-f719068be8ab

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a2f538f2-f411-4746-a765-1a61952a1543
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
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: bio, bio.value, title, title.value
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: title.value, bio, bio.value, title
  4. Score: 39.600, Matched: title.value, bio, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.761, Matched: 
  2. Score: 0.743, Matched: 
  3. Score: 0.737, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.705, Matched: content, content.value, title, title.value
  2. Score: 1.497, Matched: content.value, title.value, title, content
  3. Score: 0.295, Matched: 

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
✓ Inserted document: XnhBxlAbnq3T7E_CWGs6ZDcXX_PNYg5uFub4oFQKMapksNdOXJWJGPKD3AIRBDbEEE-Rv0P6njfXpAAI8-ePMA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: i1nO9Uqmz22uhrS2TsE3wC6QkN1A7lmAQNbsORPEWWTiB_2uNOXEE6VBv-St_tKIT_hUisheBkJH0qlwscjsEQ

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
✓ Inserted document with TTL: 8wuCeaeuJt-YXksF7LVI5U5xD8EK1PB_4PTn4bIuGtUIFNFbzOdhS-JyBD5_3VMwHzYPO1S2uuUrVt-_0X_KnA

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
✓ Edge cache script created: 4adZNQlC5AIKw_ucvCfFUHOhveTV2WrVy6b5JXXZ_JvnbcWZhoW8X9eZYQgjleQ5KWgDTbadAdzxz3sPPgFHRA

Call 1: Cache miss (fetches from API)
Response time: 4ms
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
Response time: 4ms (1x faster!)
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

✅ Script saved: B6Gg3LZp9NsSMl_CKpBzTfbJ6JbeGndPyOpHoHMTpTwgAOKDSfRvLCuik_CxzCRJaCPWmMy6_AKnlK2tA2o7wA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"inactive","avg_score":50,"count":5}
   {"count":5,"status":"active","avg_score":60}
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
   ⏱️  Duration: 131ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 69ms
   📊 Records: 1
   🚀 Cache speedup: 1.9x faster!

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

✅ Script saved: FnfZTLBct-9pANsfjF6aAN5KDK6xH7X4DXh4XYq6KmERni4dp35wCqw95RTekV4O8gc3Ad0ta_Rjq6cuK_rArA
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
   {"category":"Furniture","avg_price":474,"count":2}
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

✅ Inserted order: Pkiuhtl6dl8IzvWL54LBgAry0QTNTyZNT4jyPR8i6nEjX3RHEkVVhL5EP3QXAEK-yoDmI_OwSprd5tliCwGZ6w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: qU4TNYJZrU5Om70Lnkw6waUzPmaM8rBSnjToJHCw9qBjt0fb0--numGYAigwTLFzR6gF8oxeqxQxtH7IE1UjFg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: bXPdbk7zDA82dgPs0t_JWqhx_5njbC2cICuicG6yvgkeBLwnL_PsWTwi56QLpSKVfqg_Hm-vmcEEb4xNEUPS8A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: wBntr-b8TfnENpSuPJSiDvOv_2wiWzE77OMoBFzVBVHviD8cZh0q2hhxz0I_G3ki7242aZukbxf03t9SU-lkfg
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
   active: 7 users
   inactive: 3 users
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
   1. User 1 - Score: 20
   2. User 2 - Score: 40
   3. User 4 - Score: 80
   4. User 3 - Score: 60
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
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Vector Databases Explained
   2. Database Design Principles
   3. Getting Started with ekoDB
   4. Introduction to Machine Learning
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Database Design Principles
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
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
   Furniture: 4 items (avg $294.00)
   Electronics: 6 items (avg $325.67)
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
   2. USB-C Cable - $19 (⭐4.3)
   3. Webcam HD - $119 (⭐4.5)
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
   Vector databases offer several benefits such as:

1. High Precision: They provide accurate and detailed data representation as they use points, lines, and polygons.

2. Scalability: They can be easily scaled without losing data quality.

3. Flexibility: They allow complex geographic operations like calculating the shortest path, overlaying layers of data, etc.

4. Data Integration: They can integrate different types of data on the same map.

5. Less Storage: They typically require less storage space compared to raster databases. 

6. Interactivity: They offer the ability to add, delete, and modify features without impacting the entire database.
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
✓ Created SWR script: fetch_github_user_js (MZEOoECXDYq7pTBr1z-KAWz2hj1h0_mIClpyBM03jzPFwkeI7nwIYfLn2YmI1lLQheeFih0PaxXTsJ8Z_9p2QQ)

Step 2: First call - Cache miss, fetches from GitHub API
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
✓ Created session: AFoDIZhNX5jrNT2zdex209rENlsik0TMuGzK1WIK3YAuo8DzR8V0J5eZAV0r-TUkngiWwxAmIfZ5-9HP2ytnDQ

=== Sending Chat Message ===
Message ID: 9-VVFkywqKSf384wCucTRhJVWqOzw_SPoM9SbTqJeT5vs-x6f3gX7XKatDThWw1r0Q1VMoLUUSaDcmvmpnZhKQ

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. The price is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '_Vo3ujp6qeTb8IEDw3sULflq5_TEu5ydocdiDpa_8_cabzFi_8smyXxhAMjd79PFQOqmUTJAzYnUibxHWB282A',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'Qn78GdPiSb2LC_sQD47Xyy4wjecBs5rBpdqmjgxwCOHdYrG5zeEW7nKmrUcWWoD56DOTyTMRSRrtyLWxE2eomQ',
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    id: 'VkxKoyI_0XnYEa7nD0akyHvxNWfJoJ8YXJziV820pw7NPU_LOfR9canPTrmgJI7lxW8ZFDthkuy1ITT4SjxZIg',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4969ms

=== Token Usage ===
Prompt tokens: 445
Completion tokens: 75
Total tokens: 520

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: w3Vqd-86slnazs08_8-kNlhCjQ1em5RBl2WLrXWO5m5wzPACGu8iFJQ_ZzfMChj4kNoFlcPquRaPr21ROCLCNQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one available product is the ekoDB. It is a high-performance database product priced at $99.

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
✓ Created second session: gFZcgrXsqr51sNN9ZOZXiask5Tk-D33f2FxNAAn67FNdvLnBCoVnlHd7eG_XhKvT5YxRkZbIxSXpEEKyfA7f6w
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
✓ Created session: dDyPuL0ArR-2xoiQr1VhNJfkjEN0RL2Q4OjX-9nrbrYKMl_x72o58cw4K38ar6OZTFyf7YgCGE4zsUweuIj1EQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is a high-performance database product named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 292rI2955FBXmE7idanvPrzX_Wiceb8OcQbFBiPnErWSVWmqGmFx-ajUWuRCEkCWQ6kE-twc6u8xQ2vMhpsykg
  Parent: dDyPuL0ArR-2xoiQr1VhNJfkjEN0RL2Q4OjX-9nrbrYKMl_x72o58cw4K38ar6OZTFyf7YgCGE4zsUweuIj1EQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 292rI2955FBXmE7idanvPrzX_Wiceb8OcQbFBiPnErWSVWmqGmFx-ajUWuRCEkCWQ6kE-twc6u8xQ2vMhpsykg (Untitled)
  Session 2: dDyPuL0ArR-2xoiQr1VhNJfkjEN0RL2Q4OjX-9nrbrYKMl_x72o58cw4K38ar6OZTFyf7YgCGE4zsUweuIj1EQ (Untitled)
  Session 3: gFZcgrXsqr51sNN9ZOZXiask5Tk-D33f2FxNAAn67FNdvLnBCoVnlHd7eG_XhKvT5YxRkZbIxSXpEEKyfA7f6w (Untitled)
  Session 4: AFoDIZhNX5jrNT2zdex209rENlsik0TMuGzK1WIK3YAuo8DzR8V0J5eZAV0r-TUkngiWwxAmIfZ5-9HP2ytnDQ (Untitled)
  Session 5: N7x7pp55YXw3bqdV-U7D_qqytgmhA6oUr6eEbffzzFO5Fpem9r1yPKwYwlDbOSWNFwpwNJ6GxPfwL_VjF7Y-7A (Untitled)
  Session 6: 99Pn4ItmLj9yNXdJ9MHXaR_S6DJOS752ZPS7m0yTb1fLPcBFhVyB3y20uMRIHqLljr2_IjUyTEO3fa5MBjqN5w (Untitled)
  Session 7: pOGOOxmPEbbRsJQtqdm6xQIKSSnyBzYzNv2ua9eCwXmKq3v8FBzOS04-JYqwjkZtfjTvWdttPNU8rzG222inrw (Untitled)
  Session 8: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ (Untitled)
  Session 9: 8djIhno3iGEPxBoj-qxsGY1Zpx_yFB-AEO3e3JulWsFnGJE73TCbQ-BN95kEtrpkrz4vmqG1UpPQSqVXz7qukA (Untitled)
  Session 10: GFi9nITIlmmDiWZfKTsqe6vfG12LC7fx6h3DUO-A8GnXij598gQSbfMBnLLFF371mGI3CoYy-ndoWqrvEIJSvA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 292rI2955FBXmE7idanvPrzX_Wiceb8OcQbFBiPnErWSVWmqGmFx-ajUWuRCEkCWQ6kE-twc6u8xQ2vMhpsykg

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
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: EkoRecord(fields={id=StringValue(value=gk6637YhliXgWG-xt7ekeUVw7_B6WPAUAzjsvYgyh4XQuuVHE7LESNnLMBpmiUxVeRFtRUiu3ipe3ZF9Z75fHA)})
  User ID: gk6637YhliXgWG-xt7ekeUVw7_B6WPAUAzjsvYgyh4XQuuVHE7LESNnLMBpmiUxVeRFtRUiu3ipe3ZF9Z75fHA

=== Read ===
✓ Found user by ID: EkoRecord(fields={data=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), user_id=StringValue(value=550e8400-e29b-41d4-a716-446655440000), categories=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), metadata=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), email=StringValue(value=alice@example.com), name=StringValue(value=Alice Johnson), active=BooleanValue(value=true), price=FloatValue(value=99.99), created_at=StringValue(value=Fri Jan 23 12:30:18 EST 2026), age=IntegerValue(value=28), tags=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), embedding=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), id=StringValue(value=gk6637YhliXgWG-xt7ekeUVw7_B6WPAUAzjsvYgyh4XQuuVHE7LESNnLMBpmiUxVeRFtRUiu3ipe3ZF9Z75fHA)})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Fri Jan 23 12:30:18 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: data, user_id, categories, metadata, email, name, active, price, created_at, age, tags, embedding, id

=== Update ===
✓ Updated user: EkoRecord(fields={active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Fri Jan 23 12:30:18 EST 2026)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), id=StringValue(value=gk6637YhliXgWG-xt7ekeUVw7_B6WPAUAzjsvYgyh4XQuuVHE7LESNnLMBpmiUxVeRFtRUiu3ipe3ZF9Z75fHA), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), created_at=ObjectValue(value={value=StringValue(value=Fri Jan 23 12:30:18 EST 2026), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=gk6637YhliXgWG-xt7ekeUVw7_B6WPAUAzjsvYgyh4XQuuVHE7LESNnLMBpmiUxVeRFtRUiu3ipe3ZF9Z75fHA), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)})})

=== Delete ===
✓ Deleted user with ID: gk6637YhliXgWG-xt7ekeUVw7_B6WPAUAzjsvYgyh4XQuuVHE7LESNnLMBpmiUxVeRFtRUiu3ipe3ZF9Z75fHA

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 14s
2 actionable tasks: 2 executed
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
  {"payload":{"data":[{"id":"xc7eYGB2VOJoZvxCPOR1V-IiAjZSeFVNAeGV1tRkBis9iPTbRteem9bG72Rc5zQfhY9oQ0-E1UWzEnBYkw7PGw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
  IDs: 23oTRuc9aRk1St_JIoZlHiVCMnD_aglmGaLgf-tnovFZ1_5AkEDq7B6u25RquCDzTWCzHARvpu0Qf1HRUhe_3A, slrU8hyJADsH0_t7MRrufYE6CcdVlGbp7VvBgdTeNPJPYLKlF0PXL4YA_3NnsJuiPxzBpQVYTx9y_FqX2KzCdg, q36x9_VtE0JXgBWtNCeT5P1nr7tDhmKxPA9fjIy5XuxvEh_-JBaD6rA7Md8ImwpICshdlI7OAm14-NyGVYzpaQ...

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
  - schema_documents_client_go
  - schema_products_client_py
  - schema_products_client_js
  - schema_employees_client_ts
  - schema_users_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-23T17:30:32.229198Z","last_modified":"2026-01-23T17:30:32.229198Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1769189436623}
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
  config:api: {"value":{"retries":3,"timeout":30}}

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
Created Alice: $1000 - ID: 5b-3XRFqVHzJMaj2RTI23qG0Ic8xeqeI19C5vRjOeWUyaz2XZkQhnNMKvl3P8Vawb0GXw5mmJ1eVNeRuZ8tHuA
Created Bob: $500 - ID: 71PA9OUCWFEEcxI-s1P48_Wyum0WHFoOCrXwOnS5oPznZy0BiPflwiJZrUAK_ySxNK5dZ1ptUb6zCe9E6zHmwg

=== Example 1: Begin Transaction ===
Transaction ID: beb48c19-ece4-4664-9af2-fc681d2ec4e2

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 1288e9e1-ecbe-441e-bf06-9916026be511
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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=95)})
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
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
  {"results":[{"record":{"views":663,"description":"Learn Rust programming language with hands-on examples and best practices.","id":"ddlkiBtRDMfyUdhWHJE0vOs4f0C-YrmhslOfY_s3GC-n9qaHrwZ0wZl-pws5ldjFBiyc_zGmXPe73X9cxJfVJA","title":"Rust Programming","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"description":"Master Python for data analysis, machine learning, and visualization.","views":608,"tags":"programming,python,data-science","id":"F5VpuO9CLjUzR-lw346VcLV3i3JxsoPM2i0Ej8l9tiqyOnEy2U5D7qByalxKkcNH8K8GCF2Yrj6yTrhXfPjUTg","title":"Python for Data Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,javascript,web","description":"Build modern web applications using JavaScript, React, and Node.js.","title":"JavaScript Web Development","id":"oaCYtNix08CeWIZZNZoWZgM10J5CD8iTQzSXUisfZI5XCFqIgGbRz--fGlgxZtqeX64lECBrXnspbobIofNkNg","views":649},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","tags":"ai,machine-learning,python","id":"JYI6LYmn53y4yEcS1KALBC10qi03BuZDCvXqdyEWxA6Pi2Q6gOB3555Jn6_6J-GiX9VeBUMm2Y4ZcjWBePD8_g","views":916},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"description":"Master Python for data analysis, machine learning, and visualization.","views":608,"id":"F5VpuO9CLjUzR-lw346VcLV3i3JxsoPM2i0Ej8l9tiqyOnEy2U5D7qByalxKkcNH8K8GCF2Yrj6yTrhXfPjUTg","tags":"programming,python,data-science","title":"Python for Data Science"},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial","views":663,"id":"ddlkiBtRDMfyUdhWHJE0vOs4f0C-YrmhslOfY_s3GC-n9qaHrwZ0wZl-pws5ldjFBiyc_zGmXPe73X9cxJfVJA"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"VqtADsO9G46f78MOAmY425mPjZMsV00hu4oXrASMW8WBmZu_1YcEoZPE_GJs064BIH51Frmpmh0Mps7VLWAY-g","views":50,"description":"Learn database design principles, normalization, and query optimization.","tags":"database,design,sql","title":"Database Design"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted user 1: StringValue(value=nFvvMjvVMvEzhbBweUSy9baC3OSsG_pdGZKRcpBDZ-eUb5lK8NPNmTEHGzD3deM1LYM_ZVSsUUKYsJSHfnhfkA)
✓ Inserted user 2: StringValue(value=7d17E0QQxIdPiw9Y-8Uc7YP9qT98oHiAdAbP-Y2n-PeYIZy6MhLK-w_UXPjQ5T2WbAiOi0mnkaSpbahAVbP6yg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 28
  Sample: [schema_documents_client_go, schema_products_client_py, schema_products_client_js, kotlin_schema_example, schema_employees_client_ts]

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

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
  Document ID: n5isOGBHTk-ZIy1cmxARR-KjXb0MhRoJn3pVDfSzNmUMGcRKpa0-2e_v755QRqU61cjTuc0QBPrt4CCX-HOTiQ

=== Verify Document Exists ===
✓ Document found: created_at, id, session_id, user_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: QeSulIJf9zsRP8TTKBT_FWyT7mKRNBiY0u74aiPkqCm3mYcAr5Db4HYdm9HB62GUmTrybjgNL2FPD7VRq3A5xg

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: cache_key, value, id, ttl

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
✓ Inserted document with TTL: StringValue(value=Winx5yehhDX9wACe3LyzFulembASc1461TtHpnYg3lYgBR1LIZuxq2NNH6o0-OaR3-n6SvWW9wiDjCSm-j6G8g)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1769189466928},"id":"Winx5yehhDX9wACe3LyzFulembASc1461TtHpnYg3lYgBR1LIZuxq2NNH6o0-OaR3-n6SvWW9wiDjCSm-j6G8g","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-23T18:31:07.046805Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Edge cache script created: 2Oolg49Ul77z2-Bx5xKZ0Z7F28bFBKsqvJP-lfOSMOJitBx9JOMEiHU-OGU-gceWc3VN7lzmjDFjDFN7R935iw

Call 1: Cache lookup
Found 1 cached entries
Response time: 16ms

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

✅ Script saved: V1TpkV7ShP0x5fhlQlOLNAgtfQhbV5CKjrHdqa5mzcTu_xd0Mh3EM2phFaNvU377uUduTzjPjYUl6y_i9to6Tg
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"avg_score":60.0,"status":"null","count":5}
   {"count":5,"avg_score":60.0,"status":"active"}
   {"status":"inactive","count":5,"avg_score":50.0}
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

✅ Script saved: 7rNyvynqUdDFGKSuc5_WPKndWXA3TDXfSN_Kf8XsQnkOlH7UE-JsYlkSelpcE-uB2yZgKi2JD-v8XJer-wplWw
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

✅ Inserted order: StringValue(value=CCJ-vuqPEY9Na7FpvXqMNGtUKFAmTPfaOa07XuUQB92e4fRmQIiDMKy24LxAHcMsMCZlyyrt9BG2CTmwygKu8w)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: l-13r1mwO8WETBGBWf6ZsZwynM9Qgiwl7qIPMlWcKZ9VZONAbJerfoHPTQ6-5c_9FOsrwsZy08LbXB42WR_Hxw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 1MA6IzmRevT5p-zQJgnkBpi9aTolqBtamKUglDUPpPEsOY8fgiwu2MxWa6QKD1ViAjrgx7B2Cav0Z4Dwu4lfWg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: UP4hR95e01OmucklgTuq5LwIfNbsNVB9j5puL97jjEiBRJC0-Hu0_4yYXhbabRUs_g6UBEtTCmm0ImLAuvPWPw
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
✓ Created SWR script: swr_cache_lookup_kt (v0CNV_ZACY0kp0FTaPbJvfw8z96qN2r5szudHwaK9XzT-ZdB9EqcGMciy70XZiWEAZMQ2eSLlcOOMRl-w69Luw)

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
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
   {"category":"Electronics","count":5,"avg_price":367.0}
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
   {"response":"Vector databases provide benefits such as efficient data management, accurate data representation, high-quality spatial analysis and network analysis. They also offer precision in preserving spatial details and allow for easy modification and editing.","tokens_used":83}
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
   1. {"value":"Getting Started with ekoDB","type":"String"} ({"type":"String","value":"Database"})
   2. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
   3. {"value":"Introduction to Machine Learning","type":"String"} ({"value":"AI","type":"String"})
   4. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   5. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":3,"category":"Database"}
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
✓ Created session: kkXUiEPvbSblDD9ky3XE9sAYP7NX057rihDY1P1Bjj5i8DDJNXn1inw017W7IGNDjZPjE_HEuKoIcI26pXr7Yw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "BRJw0fOLucAbvWOnrwaFSbkBwBGSorAoJxnSnsbKveACfg5Tf6Sg0Dgo4wrVLTlxv1axNAlYFff_WaRjOZsQgw"
  Responses: ["ekoDB is a high-performance database that comes with intelligent caching and real-time capabilities. It also features an AI integration which enhances its performance. Some of the main features of ekoDB include an AI Chat Integration and diverse Search Features.\n\nThe AI Chat Integration allows you to query your database using natural language. This means you can get AI-powered responses that provide relevant context for your queries. \n\nThe Search Features on the other hand, support full-text search, vector search, and hybrid search with automatic context retrieval. This makes it easier to find specific information in your database."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: KKmDHFUhpOoB9SbQ51zW8dKxDbznJ2SyL0C0v9m1KNN-ErWY7gDWloT4PZ-r4Pk3UYtlPphMT8VrOw0SHFTwIA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is \"ekoDB\". It is a high-performance database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information available, we have a product named \"ekoDB\". It's a high-performance database product, priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: L82XU1o0IFvZQO1wyQn0mUxKigZwlUdX4rPrnyKEOGUCO79XGC_FkUiwFGzS2ofYgVZ0Zxb5GElHFaWx3_Eq2A

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0ACBsSZm4-YBNesdqGpd6OyWljqRQbyxw1xrdmEU-Pcw23R2XGajXsd9a15RpAqyLD9xSaCB9WeiI_nIhfKSwg

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is ekoDB, which is a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB, a high-performance database product with AI capabilities, is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"0ACBsSZm4-YBNesdqGpd6OyWljqRQbyxw1xrdmEU-Pcw23R2XGajXsd9a15RpAqyLD9xSaCB9WeiI_nIhfKSwg"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"jybmCKrp2R8aB9CGUluFtcMvv2881uMok_ti7ZlRspYPaGTE_Clsuo_r-4x3bskQjbrtNi_wVcd1ZsYQgt-dIQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-23T17:32:12.476270Z"},"id":"yWn4eHLncjGb7mNsoUp6bWMbphPyF4jQ6uYa2fUtFhY5bGoCwokWaKgLmW5xG97dGxmLWL1aXacTzB17l3R-HA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":170,"total_tokens":196}},"updated_at":{"type":"DateTime","value":"2026-01-23T17:32:12.476270Z"}},{"chat_id":{"type":"String","value":"0ACBsSZm4-YBNesdqGpd6OyWljqRQbyxw1xrdmEU-Pcw23R2XGajXsd9a15RpAqyLD9xSaCB9WeiI_nIhfKSwg"},"content":{"type":"String","value":"The available product is ekoDB, which is a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"jybmCKrp2R8aB9CGUluFtcMvv2881uMok_ti7ZlRspYPaGTE_Clsuo_r-4x3bskQjbrtNi_wVcd1ZsYQgt-dIQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-23T17:32:12.479238Z"},"id":"h7opQoQ2kpAqt-zX17AFVHFyLavuXC8A7oTNcST8BLCXhFFpKJYdYJLEZin0880u1V_4XKAoPwcNolJ6UZU2Tw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":170,"total_tokens":196}},"updated_at":{"type":"DateTime","value":"2026-01-23T17:32:12.479238Z"}},{"chat_id":{"type":"String","value":"0ACBsSZm4-YBNesdqGpd6OyWljqRQbyxw1xrdmEU-Pcw23R2XGajXsd9a15RpAqyLD9xSaCB9WeiI_nIhfKSwg"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"jybmCKrp2R8aB9CGUluFtcMvv2881uMok_ti7ZlRspYPaGTE_Clsuo_r-4x3bskQjbrtNi_wVcd1ZsYQgt-dIQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-23T17:32:14.318734Z"},"id":"O2eWPGy3gEmsF8tjd0uAvYH-AVyyiUOUNTOKNC0-3HduL3mgv79xaG_HdJvfsuqZPFjDhjH0gpa3G2OD4F3ISw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":170,"total_tokens":191}},"updated_at":{"type":"DateTime","value":"2026-01-23T17:32:14.318734Z"}},{"chat_id":{"type":"String","value":"0ACBsSZm4-YBNesdqGpd6OyWljqRQbyxw1xrdmEU-Pcw23R2XGajXsd9a15RpAqyLD9xSaCB9WeiI_nIhfKSwg"},"content":{"type":"String","value":"The price of the ekoDB, a high-performance database product with AI capabilities, is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"jybmCKrp2R8aB9CGUluFtcMvv2881uMok_ti7ZlRspYPaGTE_Clsuo_r-4x3bskQjbrtNi_wVcd1ZsYQgt-dIQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-23T17:32:14.320942Z"},"id":"ATJHg0ZX6qxU3K6nx2s0LwfXZ59Wk8ZK_9sGI52bYQ3QkNhzCPcNlBfzuX54C3zHzz5Ipdo9hF8Cr17G-QMwWw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":170,"total_tokens":191}},"updated_at":{"type":"DateTime","value":"2026-01-23T17:32:14.320942Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: wwtJVUdU0qSmbjIco18OQF_nzdeA7IuWuYkyZCt9yeL5DBZMbSfkswhOk8EK4x_bv9M8zlvU-tx3f5OmP9zG_g

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=E6GcbHTLmwmOy-k0WAB4jGc9oUH43CgcwqWGW6A1eVtf4OQeBmgCEPx_e8yaTtRRd8iCEPA2hmgSQKk42ZMp3g)})

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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=hTla5GyJLrcHWTXgWABLUDzkhNnKVfYqCJfggbpdBpPfji-ghdNjQyhjLoWn7GisrGmItu9_ZeYn8lvVr2fGtA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=_T593oo2FQUEJ6P00lmemh7-vqmo9HBC614I45RlbNRlbT8logAfN982IEhogjx1V_eNcZ9fQ1JPooUpjs3LLg)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), id=StringValue(value=hTla5GyJLrcHWTXgWABLUDzkhNnKVfYqCJfggbpdBpPfji-ghdNjQyhjLoWn7GisrGmItu9_ZeYn8lvVr2fGtA)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=hTla5GyJLrcHWTXgWABLUDzkhNnKVfYqCJfggbpdBpPfji-ghdNjQyhjLoWn7GisrGmItu9_ZeYn8lvVr2fGtA), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)})})

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
Created Alice: $1000 - ID: qnBH9s3lbL-sdbJKbLpbxsOBsjexmXG9_Zkqp0_4HjrI1uACg8YqG03b2IvTLYFkIYR6gmWef5cSIyr8o_ODAQ
Created Bob: $500 - ID: a_vcNTwAoOU1VHkd--6GWwWL5mjiG078cPxlTNKMhdpAC_OVCWtN3wH_wVYcuE7NiQ92wGC7Dl6PzNtqorJciA

=== Example 1: Begin Transaction ===
Transaction ID: 97855d15-43d9-4f76-9fe8-e82d637721eb

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0118d67c-cd84-4d86-bcff-ceb115717aca
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
    ✓ Generated embedding: 1536 dimensions in 872.64375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.080852334s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 571.619666ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.319123291s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 271.844875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 337.399125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 274.013916ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 338.374167ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 653.878542ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.04349375s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 204.233166ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 429.498208ms
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
    ✓ Generated embedding: 1536 dimensions in 1.099089792s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 97.784ms

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

Writing memory-safe, high-performance database code involves a number of strategies:

1. **Use Prepared Statements**: Prepared statements help prevent SQL injection attacks. They separate SQL code from data, reducing the risk of accidental or intentional corruption of the database.

2. **Normalize Your Database**: As discussed in context 2 and 3, normalization is a good practice for reducing data redundancy and improving data integrity. It can also improve performance by reducing the amount of duplicate data.

3. **Use Indexes**: Indexes can significantly increase query performance by reducing the amount of data the database needs to go through to find the relevant records.

4. **Optimize Your Queries**: Avoid SELECT * queries, as they can slow down your application by returning unnecessary data. Instead, specify the exact fields you need. Also, consider the order of conditions in your WHERE clauses. The database will process them from left to right, so place the most restrictive conditions first to filter out records as quickly as possible.

5. **Use the Right Database**: As mentioned in context 1 and 4, the choice between SQL and NoSQL databases depends on your specific needs. SQL databases are generally better for complex queries and ACID transactions, while NoSQL databases are better for flexible schemas, horizontal scaling, and high write throughput.

6. **Caching**: Implementing a cache for your database can dramatically increase the speed of read operations. A cache stores data in memory for quick access, reducing the need to query the database directly.

7. **Monitoring and Profiling**: Regularly monitor the performance of your database. This will help you identify and fix bottlenecks early. Profiling tools can show you which queries are taking the most time, so you can focus on optimizing those.

Remember, safety and performance often go hand-in-hand in database code. By following these practices, you can create code that is both fast and secure.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 410.126625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1989 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 293.6045ms
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
  ✓ Text search completed in 54.563ms

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
    ✓ Generated embedding: 1536 dimensions in 0.470s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.413s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.406s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.312s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.327s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.334s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.534s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.250s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 2.439s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.468s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.213s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.351s
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
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.085s

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

Writing memory-safe, high-performance database code requires a combination of robust programming practices, understanding of the database system being used, and awareness of the specific data needs of your application. 

Here are some general tips:

1. **Efficient Querying**: In both SQL and NoSQL databases, you need to ensure that your queries are optimized. This means using indexes effectively, reducing the amount of data transferred, limiting the number of round trips to the database, and making use of the database's built-in functions where appropriate. For SQL databases, this might also mean writing queries that take advantage of the relational model, such as using JOINs effectively.

2. **Database Normalization**: If you're using a SQL database, it's important to normalize your data to reduce redundancy and improve data integrity as per Context 3. This can help to improve the performance of your database by reducing the amount of data that needs to be stored and retrieved.

3. **Connection Pooling**: Establishing and closing database connections can be a costly process. Connection pooling allows you to reuse existing connections, reducing the overhead associated with establishing new ones. This can be particularly beneficial in high-traffic applications.

4. **Caching**: Caching can significantly improve the performance of your database by storing frequently accessed data in memory. Be aware, however, that caching can also introduce complexity, particularly when it comes to ensuring data consistency.

5. **Use Appropriate Data Structures and Algorithms**: The choice of data structures and algorithms can significantly impact memory usage and performance. For example, using data structures that minimize memory usage can help to reduce the memory footprint of your application.

6. **Memory Management**: In languages where memory management is manual, like C++, make sure to properly allocate and deallocate memory to avoid leaks. In languages with garbage collection, like Java or Python, be aware of how memory is managed to avoid unintentionally holding onto large amounts of data in memory.

7. **Concurrency**: Use proper synchronization when accessing shared resources to avoid data races and inconsistencies.

8. **Testing and Profiling**: Regularly test your application under realistic workloads to identify any potential performance bottlenecks or memory issues. Tools like profilers and memory analyzers can be invaluable in understanding how your code is behaving in practice.

Remember, the best practices for writing high-performance, memory-safe code can vary depending on the specific database system and programming language you're using.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.421s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2689 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.396s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe, high-performance database code requires a combination of robust programming practices, understanding of the database system being used, and awareness of the specific data needs of your application. 

Here are some general tips:

1. **Efficient Querying**: In both SQL and NoSQL databases, you need to ensure that your queries are optimized. This means using indexes effectively, reducing the amount of data transferred, limiting the number of round trips to the database, and making use of the database's built-in functions where appropriate. For SQL databases, this might also mean writing queries that take advantage of the relational model, such as using JOINs effectively.

2. **Database Normalization**: If you're using a SQL database, it's important to normalize your data to reduce redundancy and improve data integrity as per Context 3. This can help to improve the performance of your database by reducing the amount of data that needs to be stored and retrieved.

3. **Connection Pooling**: Establishing and closing database connections can be a costly process. Connection pooling allows you to reuse existing connections, reducing the overhead associated with establishing new ones. This can be particularly beneficial in high-traffic applications.

4. **Caching**: Caching can significantly improve the performance of your database by storing frequently accessed data in memory. Be aware, however, that caching can also introduce complexity, particularly when it comes to ensuring data consistency.

5. **Use Appropriate Data Structures and Algorithms**: The choice of data structures and algorithms can significantly impact memory usage and performance. For example, using data structures that minimize memory usage can help to reduce the memory footprint of your application.

6. **Memory Management**: In languages where memory management is manual, like C++, make sure to properly allocate and deallocate memory to avoid leaks. In languages with garbage collection, like Java or Python, be aware of how memory is managed to avoid unintentionally holding onto large amounts of data in memory.

7. **Concurrency**: Use proper synchronization when accessing shared resources to avoid data races and inconsistencies.

8. **Testing and Profiling**: Regularly test your application under realistic workloads to identify any potential performance bottlenecks or memory issues. Tools like profilers and memory analyzers can be invaluable in understanding how your code is behaving in practice.

Remember, the best practices for writing high-performance, memory-safe code can vary depending on the specific database system and programming language you're using.

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
    ✓ Generated embedding: 1536 dimensions in 0.411s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.341s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.170s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.610s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.229s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.308s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.226s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.295s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.357s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.437s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.324s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
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
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.084s
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

Writing memory-safe, high-performance database code involves several key practices:

1. Understanding Your Database: Whether you are using SQL or NoSQL, understanding the ins and outs of your database is crucial. This includes knowing how to normalize data (as per Context 1) in SQL to reduce redundancy and improve data integrity, or when to use NoSQL over SQL (as per Context 4) depending on your specific needs like flexible schemas, horizontal scaling, high write throughput, or working with unstructured data.

2. Efficient Query Writing: To optimize your database queries (as per Context 5), you need to write efficient SQL statements. Avoid SELECT * queries; instead, specify the columns you need. Use joins appropriately and avoid N+1 queries. Indexing is another way to optimize your queries. Applying an index to a column used in WHERE clauses can greatly speed up retrieval times.

3. Safe Memory Practices: Ensure to properly manage memory by closing database connections once they are no longer needed, and freeing up memory space used by temporary objects. Avoid memory leaks by periodically checking your application for any unused or unnecessary data taking up memory space.

4. Use Prepared Statements: Prepared statements not only help secure your database queries, but they can also improve performance as they are compiled only once and can be reused.

5. Batch Processing: If you have to insert or update large amounts of data, consider using batch processing. This reduces the number of individual queries to the database, thus improving performance.

6. Monitor and Optimize: Regularly monitor your application's performance. Use profiling tools to identify bottlenecks in your database queries. From there, you can apply appropriate optimization techniques.

Remember, writing high-performance database code is not a one-time event but a process of continuous monitoring, learning, and optimization.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1923 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.392s
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
  ✓ Text search completed in 0.042s
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
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.308s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.445s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.530s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.332s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.092s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.403s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.610s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.406s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.391s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.413s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.324s
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
    ✓ Generated embedding: 1536 dimensions in 0.347s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.085s

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

Writing memory-safe, high-performance database code involves a few key principles:

1. Use Appropriate Database: As per Context 3 and 4, the choice between NoSQL and SQL can impact the performance of your database operations. SQL is better suited for complex queries and structured data with well-defined relationships, while NoSQL shines when dealing with unstructured data, needing high write throughput or flexible schemas.

2. Normalize Your Database: According to Context 1 and 2, database normalization can help improve data integrity and reduce redundancy. This could lead to more efficient database operations by preventing unnecessary data duplication. 

3. Optimize Your Queries: As per Context 5, optimizing your database queries can significantly improve the performance of your database operations. This can be done through methods such as indexing, using appropriate data types, limiting the use of functions in queries, avoiding unnecessary joins, and more.

4. Memory Management: In order to write memory-safe code, you need to ensure that you are properly managing the resources your database operations consume. This includes things like closing connections when they're no longer needed, freeing up used memory, and more.

5. Use Prepared Statements: Prepared statements can help improve performance and security. They are pre-compiled SQL statements that can be executed multiple times without needing to be parsed again, improving efficiency.

6. Use Transactions: Transactions ensure that your database operations are processed reliably and in a memory-safe manner. They group a set of tasks into a single execution unit, where either all tasks are performed or none are.

7. Error Handling: Implement robust error handling. This helps in preventing memory leaks and ensuring that your application can recover gracefully from unexpected situations.

8. Testing: Regularly test your database code under varying loads to identify any potential performance bottlenecks or memory issues. This can help you make necessary optimizations and adjustments to ensure high performance.

Remember, writing high-performance, memory-safe database code is an iterative process. It involves continuous testing, optimization, and adjustment.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.829s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2246 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
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
  ✓ Text search completed in 0.056s

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
    ✓ Generated embedding: 1536 dimensions in 0.477s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.367s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.404s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.331s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.285s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.209s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.395s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.348s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.35s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.344s
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
    ✓ Generated embedding: 1536 dimensions in 0.276s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.097s

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
    ✓ Generated embedding: 1536 dimensions in 0.472s
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
  ✓ Text search completed in 0.05s

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
✓ Created SWR script: fetch_github_user (-thBaxmmw0w1zMD1JOWZSTy3exwCZvWNlSH95_6ml1GLeW4ixRbENtgQRhJ9NKGqwXnh9FwotYta3cXgoKeGug)

Step 2: First call - Cache miss, fetches from GitHub API
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
✓ Created enrichment script: fetch_product_with_reviews (O41rQirJ7J9nAk4yU9SjLYyDWODylpPqhAQjp9xwMtOIKbAV6iAGxXwoBQoKMAMls8KbxrQ6C3IiY7ngNraTTg)

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
✓ Edge cache script created: rncesrM_T4BzyLf1BaPr2_GjxujEnM_VdZt0EManqUbzzJVnEvG4MFvW7q6btIHAnbJNaSsJHjSW5XY6Mf63OA

Call 1: Cache miss (fetches from API)
Response time: 6ms
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
Response time: 5ms (1.2x faster!)
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
✓ Created SWR script: fetch_github_user (Hsb3NF-CdmKqrjp0oOT3om_BkUcEZaN-_mPq85HS1kuNqI2ImQtWqraVKvS5Kugn8EHXcf_zq9sKOMQdnS3lBQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 6ms
Result: []
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (1.2x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (N-CKBGIxFMnFRDlltxD0hdvHaBMt3P8NadbP3CHptvmbOBrAvLxprgdwSqd7hKhB_btq1TVEXDlYUY3WbTI-cw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "cache_key": "1",
    "enriched_at": "1769189657",
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
    "id": "NpcG0xJdWcA_BxbMbI-IKblS4JTL1SI7PWAjLOF1-wtsXquRqOgrtjhA0A6XhkvzMxuw_pzN81u7W5By0OQc0w"
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
✓ Created SWR script: fetch_github_user (WK4jJAh6zgKtA495Tphvhd2J5QC5nJaVFv6dsUJ3bTosUEj2IJtPB-rAcbT17zYFQyAUBNX7lqej-NvTANzSZg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 6.742709ms
Result: []
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5.707917ms (1.2x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (IlfsnS45KYWuxiKEC2jtz-dAbT-jrb9tI5Y8gYMnyO3R3wqJvKgNPFGaNm3brg2yO9Gm1nb_Ekg1d4NvT7pCTA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "cache_key": "1",
    "enriched_at": "1769189657",
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
    "id": "1MRtk0joJbvk_1szC4IA1Zz-R5GJ6SBHCfZ4AOdWKD_hYXO0TQpyL_IVoDMf1QWq-6RmA3_ucUWjK_CXh0DQ1Q"
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (sOkoSM2s5Y6wMivpydiNDNqU-bAEm-AokSN3DSRQVp8l5vw6Kx8zw4IsGbbn1xrrs4Xcaj_Spe6Lczai5c1B4w)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 7ms
Result: null
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 6ms (1.2x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (fE61qJRbPcemF3PZuUDzmuzfYt7foEdUxA-rLsPOJJjEyESlqDveHAavooyg_x_x7vp2QHjhPaES8xqloEXTsw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: null
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
✓ Created SWR script: fetch_github_user (LUdiZIvsFaZc36H4JN4IY1iqIXFrBTSy9o1HCopasMzrq0jmUBsrJV6YuYnwf8r5Mc7Yej_kgGOP-fPkYANgwg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 14ms
Result: []
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 8ms
Cache hit was 1.8x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (_baYG-zESnI5CTTxm-vfisVGA_ZVl63j37rJsIbAi_Xs2cKF8Mrb4Iq-Wm05yNHmL_QRR6ToR7zKyKg_diMHHA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: []
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 127.206959ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 73.617292ms
   📊 Records: 1
   🚀 Cache speedup: 1.7x faster!

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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 65ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 68ms
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
    Finished `release` profile [optimized] target(s) in 0.07s
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 71.4ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 70.1ms
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
   ⏱️  Duration: 71.853792ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 71.513708ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 69ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 90ms
   📊 Records: 1
   🚀 Cache speedup: 0.8x faster!

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

