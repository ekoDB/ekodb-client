make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("jAJyK5FtBTaokQ_qJe1oaZh_UnxmLwtO6ZT9pX-VJqNoBjFzxKFfYb5NA7-TbNIFbIM2BGNDYqvDVrpH4WBuYg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jAJyK5FtBTaokQ_qJe1oaZh_UnxmLwtO6ZT9pX-VJqNoBjFzxKFfYb5NA7-TbNIFbIM2BGNDYqvDVrpH4WBuYg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jAJyK5FtBTaokQ_qJe1oaZh_UnxmLwtO6ZT9pX-VJqNoBjFzxKFfYb5NA7-TbNIFbIM2BGNDYqvDVrpH4WBuYg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jAJyK5FtBTaokQ_qJe1oaZh_UnxmLwtO6ZT9pX-VJqNoBjFzxKFfYb5NA7-TbNIFbIM2BGNDYqvDVrpH4WBuYg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "qki0j-AbPiqn-W9E9_Co8X9oaL0jTuJ8TS557tLD_PXUotvNSLVZyvEMADozG_Uoz8jqy69EW3PmfjChDGxv6Q"

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
        "id": "qki0j-AbPiqn-W9E9_Co8X9oaL0jTuJ8TS557tLD_PXUotvNSLVZyvEMADozG_Uoz8jqy69EW3PmfjChDGxv6Q",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "JNVjS0IpfI1PRFbI32GWPhBh2ctsG61EB_foYxoKuG8cek4P0rMfv27DY8KmM_LLJaK8S87y128HZgIQ_TC_1A"

=== List Collections ===
Total collections: 5
Sample collections: ["test_collection", "batch_users", "audit__ek0_testing", "websocket_test", "demo_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "2siEIiuKLWq2hrulYdtc0BJq2tZJxpzUZGmKiHUlTEtoEvqphjxABDZ8ew5tOJhoSPgFAIuGYbf8aJieILmudA"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "ymV3PJJbAuxPiH8muX1feaHx7g5n2WGPZ-fWUrP-pO9NLvb1T6u0Ceui-nf_zgwhvftPIJUsw1Z-RUIG9Gno_Q"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "DFPR9AqC0v6D9KFSyf9x0bBIZsn7F3I2rPxGO1S6NtYTIZ7AKPlaGGzpoHuP1r9HQ9MGyQyThqJ4NgT2bQYoWQ"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("D1YAA_b3rRDv-Wbbrj_tjO2sy1aO06p3JL2rOybvYU-w2f9mv8fCvbrR3mYjnO_sjwu7YMK89tNm3xOWF0UV_g")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "D1YAA_b3rRDv-Wbbrj_tjO2sy1aO06p3JL2rOybvYU-w2f9mv8fCvbrR3mYjnO_sjwu7YMK89tNm3xOWF0UV_g"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-03-20T06:05:48.204285Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: caXDNDIl7ujizK-XvXAiZOrEpms_fvV5z8NrlVYeidxNrrqa_jTCdm2f4YW0z2_5pLMLfo9St4_9yKzQFezdqw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "BavaHXHFmf98eQtXAVCYyvxFkfSZdn20Jl_Js73DfcQMVeSUn2jRtXxibKP15Upi1wBflDnV4_0jf8OTU4VzNA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: FQYiOC_60ld6wrbGVOKv6yFsXdQ0ueNYohQjl-TM5K6L1pWp79DGk0edmlU1v_hAdvPaizEb0e88FMcURkx6aA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 9CkP-PgXuPLpQ-5J4hD4UYP8SzQL-crrBYlGiBNL6qf7ozQUTn5SrTgd2UjlQT0hK5voae_oQisZU1f2sTVvEA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: C3UFfdf12qowQLbBYQS7zEr301O7_M_PTKwVb-CHiDPVCqncVEDl6Qw8DP13NQ-FVPYohoEyTyoQ_XnOFWMQoQ
Created Bob: $500 - ID: mkiGV-_ELZtP2PfAy8Yuoj8xLbUQx0duKSQeznFd_Jwl4uWp6Hhk3sIKkpSq7WyvFbC7JzgT5dzazdz-xlUUsQ

=== Example 1: Begin Transaction ===
Transaction ID: 045b28eb-7ef2-4ddc-84b2-93f7d9eb483b

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
New transaction: 07fd933e-4638-46fc-bbe4-5b440207f049
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("i1Ov0hh-eK8ycSPvDcQTiRTDaPPyyHWJ9BEucJFkxBRVCx7KZLkqGHmrQVr4GvypcqfjSBS9da7pv5OWOY7D6w")})

=== Find by ID ===
Found: Record({"created_at": Object({"value": String("2026-03-20T05:05:52.437014Z"), "type": String("DateTime")}), "id": String("i1Ov0hh-eK8ycSPvDcQTiRTDaPPyyHWJ9BEucJFkxBRVCx7KZLkqGHmrQVr4GvypcqfjSBS9da7pv5OWOY7D6w"), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "value": Object({"value": Integer(42), "type": String("Integer")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-03-20T05:05:52.437014Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "created_at": Object({"value": String("2026-03-20T05:05:52.437014Z"), "type": String("DateTime")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "id": String("i1Ov0hh-eK8ycSPvDcQTiRTDaPPyyHWJ9BEucJFkxBRVCx7KZLkqGHmrQVr4GvypcqfjSBS9da7pv5OWOY7D6w"), "value": Object({"value": Integer(42), "type": String("Integer")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")})})]

=== Update Document ===
Updated: Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "price": Object({"value": Float(99.99), "type": String("Float")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "created_at": Object({"type": String("DateTime"), "value": String("2026-03-20T05:05:52.437014Z")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "id": String("i1Ov0hh-eK8ycSPvDcQTiRTDaPPyyHWJ9BEucJFkxBRVCx7KZLkqGHmrQVr4GvypcqfjSBS9da7pv5OWOY7D6w")})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: RQw0wCUMY2LG2MyThn5N3nw8MKAPRXEiUnQ2INn31Ae72nZnAw-IybK_9ZUPGbEHTQg_1M_vLpkOPZUWKfrUsA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "RQw0wCUMY2LG2MyThn5N3nw8MKAPRXEiUnQ2INn31Ae72nZnAw-IybK_9ZUPGbEHTQg_1M_vLpkOPZUWKfrUsA"
  value: 42
  active: true
  name: "WebSocket Test Record"

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "NiVLw4VIQ7s2st3SEgjdc6n4RAgflx8OWrpqUypYgfo89dYiIsGmtS-6NrZONw_vTVF-xjFNM1Iqegjjg9nNDQ"

=== List Collections ===
Total collections: 10
Sample collections: ["test_collection", "batch_users", "audit__ek0_testing", "websocket_test", "ttl_cache"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
  cache:product:2: Record({"price": Float(39.989999999999995), "name": String("Product 2")})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KjsxX5Imo2Jp07SVBIUHXL3Qo-7woHcm_EVMTWtMD6j4vIUV8z1JuMFsrcOaiLRtm8-wmJhmQ1LnzaYAxN6P8g
Created Bob: $500 - ID: ggNQl3uD1sV_C_s2VGvFbBTd4QbswTjyU_RrIvFtBQerijVP-3tw5dtCJCzA_Ix4fc-Tm6FJKf9F4uVyxAWCJA

=== Example 1: Begin Transaction ===
Transaction ID: d8389d30-7ca2-435a-ab6e-64ff35f967c3

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: fb6d304f-47b6-4dc7-b565-2f5ccdc8877b
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
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
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
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

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
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description", "description.value"]
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
Execution time: 4ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("2FI1-fuj5Pa1rfDhppnBvkZqd2OeDU0j77M-TzovseIiE5tULR9ag6eh5NsQ8IPGf4Es4DaHRSZNRU_8flRJ4w"))
✓ Inserted user 2: Some(String("4Dv5-Qr_VdwkNyHvrqZIkhcUzMhytekKihsFEyi0k5EemJLjpzo3DdK5ZFaVbTQMG2QEfmBIaKf8YSlttZKgXg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - email: String
    (required)
  - age: Integer
  - status: String

=== Listing Collections ===
✓ Total collections: 10
  Sample: ["test_collection", "batch_users", "audit__ek0_testing", "websocket_test", "schema_client_rust"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "TORh55_rNZYPYQsEsow4IiFs2IIIFYtQsR1nJwYTgR_QKn_fbIyxbKgjDYvUsylMqFX5tL5B5ApfWDxGdUhC7g"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("4-hvRLY7niF5395NEWXKMuKxTvmKzwxi09udfDWqvIAQB0pq-yKnfpXZzsC25HtG1GovYR2iRETw5GlmGCDw_Q"))

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
✓ Inserted document with TTL: Some(String("KS9g_jmNvUD4uRJu_k8anIcRNca13eQ5sxTraU2jS7yK8jD8FVgHpdsYFjKWVOx5--u4_pucGupgFP9Voqzv3Q"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: 45ubA_qKM4hkOG6VKTPHc2Py6Po9Ayp1gWuA9rUI5_x4ltaNVZKdDegTz6z2c0LLpnSm5Jz5QzzbeHSuLwXONQ

Call 1: Cache lookup
Response time: 2ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: EVojMW3g9jIbwHkkhWxVEZaaFxo6WQyko3x92vE-wfTxJLBfPl9QR3Rd6OvLn12Sy2SdP57fIOkZYBK8AhVa4g
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: Hv70SzxTKGDeRQVxvb4XdeFs6U4A4pF3nFomvaNEuiCP_t2r56xunXRi83VYQwlwM_TFF5-Z2ReE5LJLw9STOg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: MCQqgzzjiQ0hdqLc0GrwxKZb_PPiR95dgSvfGMOBONwrb6Jbb-rgglh8NTj-43X3htMLIhFLQ-g9-eg037l6Yg
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
   ⏱️  Duration: 124.119625ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "id": 1,
    "phone": "1-770-736-8031 x56442",
    "address": {
      "street": "Kulas Light",
      "zipcode": "92998-3874",
      "suite": "Apt. ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.128333ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "email": "Sincere@a...
   🚀 Cache speedup: 62.0x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 4J1WtWD_yesPthIM0xIfBUvPvP02IroTWOma7NPgzH6rqCYzxhiuStAblDsHwCVNe79tULED_8HfZ3gaGU5riQ
📊 Found 2 product groups
   Record({"avg_price": Float(575.6666666666666), "count": Integer(3), "category": String("Electronics")})
   Record({"avg_price": Float(474.0), "count": Integer(2), "category": String("Furniture")})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("_jKXN3FPuiFaZjXKbaijPf8AFKyl-n9plFXy8nVBLTOZCgGyJwlxO8aNrGUuF1Ohn6CESpRWgJQ2BCg9Hn2rtQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: d2egVZpKk3ogSZipjb4CsK2VKLC68IweVGvqxzdvL69X_L7Y1TjvvsKajhc-vduxNPqCpMNCElNBkEldiocsBw
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 3AY_S-OkTHsA3SAaAlfsUz21qh0gXMFA9Thon100woTA_ZRnUEjLSiGxCcMfnFE31ADVkC2N0013AGjL-hgSQw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: jnvuw2aKFIYs_zHUg4EIv1Q47fECqVekKIbVUqc5x91aGH2YSraqd2Fblx6YqCOgDLSHg0NfDn5ZnlDOF-EPvg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (AzDu-AlhCP6I7LJjPCYXMVDYpsR61bd2aeJa0TZnpEbGRqFZrPcK3qO8mYm8abMst_GSLg3B1ccz5TZbDVS5yQ)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (YSITeqVQ3pTcEgQSeIu4xd7P_OaSpUU6weUZfh0PhFhh5C6Hp5ZpPiZ4FQDhlQScVsEV3H2f7sJsZjjPJm69XQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 123ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 61.5x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (xFu4XEmLPVyy7bFsOCaOZFmrSBhvw2gqzJbG9PqQQBf0KQ7OiEOgFvaO_F2VZCZDCEHu_Cu-TNlVzotajgEoXw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (gizBCIY7Pa55BOqV_GaObwShh_UnlACjG8CxfMIfWV7Vp4b1Z_v6vlkIqHlCygdOCb4EpEhq9OhOCwBaI-NKMw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (jrFiUSjt6-GkO6UVpKy_k_OI12NORB-5W4wUJCm3BLQX_lzobFc7vJzX1md1Ty5uT-YE4LO24feUwRTavB0iHA)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
   Record({"category": String("Furniture"), "avg_price": Float(365.6666666666667), "count": Integer(3)})
   Record({"avg_price": Float(367.0), "count": Integer(5), "category": String("Electronics")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in nearest neighbor search, enabling fast retrieval of similar items based on vector embeddings.

2. **High Dimensionality Handling**: They are designed to manage and index high-dimensional data, which is common in AI and ML applications.

3. **Scalability**: Vector databases can scale to handle large datasets and user queries efficiently.

4. **Integration with ML/AI**: They seamlessly integrate with machine learning models to store and retrieve complex data representations like text, images, and audio.

5. **Multi-modal Support**: They can handle various data types (e.g., images, text, audio) by converting them into a uniform vector space.

6. **Real-time Processing**: Many vector databases support real-time data ingestion and querying, making them suitable for dynamic applications.

7. **Enhanced Search Capabilities**: They enable more meaningful and context-based searches compared to traditional keyword-based databases.

8. **Lower Latency**: Optimized algorithms reduce the time required for data retrieval, enhancing user experience. 

9. **Clustering and Classification**: They provide tools for clustering and classifying data points based on their vector representations. 

Overall, vector databases are particularly advantageous for applications like recommendation systems, image and video search, and natural language processing.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
   Record({"status": String("active"), "count": Integer(7)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"category": String("Database"), "count": Integer(3)})
   Record({"count": Integer(2), "category": String("AI")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 5NA7fc-gGQ8ik8Oj5SrFhHvgpazi_4_utQPew-GP7y5Q4HW1vdV2REbIcKCpQLiFLAnG3VwOmgTdMa25kozH7w

=== Sending Chat Message ===
Message ID: 1KbqQ-jsqiYvrOyuV8YD5MJCF2xsUsUqutU6zKV-cxCRyd7kvuSzX3J44KYep4mrMxbhKL5hNGmfmX5AruI5FQ

=== AI Response ===
Response 1: ekoDB is a high-performance database designed with intelligent caching, real-time capabilities, and AI integration. It allows for efficient data management and retrieval, providing users with the ability to utilize advanced features seamlessly.

### Key Features of ekoDB:

1. **AI Chat Integration**: 
   - The chat feature enables users to query the database using natural language, receiving AI-powered responses that include relevant context. 

2. **Search Features**: 
   - ekoDB supports various search capabilities including full-text search, vector search, and hybrid search, which allows for automatic context retrieval.

These features make ekoDB a versatile and powerful tool for managing data and enhancing user interaction through intelligent querying and search mechanisms.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("VWQUN7xrgJVwydHHQH8iWvwOZhaNJhj5F9PTfcNUvA9eYFgp-f5dc6Q81GeSU9_Z86_s5tCJzlHMrl2PzW7-OQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("hJSBw1wRmJI9R7HBsjrHbhXQK5ExyEjI3pxCQ_zTGYDxA3lZ6AjrFow1PNKV75fqTC_fUs79XTwaaa7ck_JQgQ"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("D2TGteu_deUukBzh09sJ7q23oVZnjVoZ3m_O8uMGb9Gbevq-u5R0gen-qrFnLo61DEKift9V6WSFdULBHYzvWQ"), "title": String("Search Features")}

Execution Time: 10313ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rtrp3oqMUT_roui3BuTOtoo9LBv9tvMgHljXMaFqD2TwydQbmySwtAvCnXkAMUEjTUrRthSmMisJbJgRB-hZLQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have other questions, feel free to ask!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need additional information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 4AYqcp-2wqegFsv6KpaPN1YAulh13iugnpP9A6u-EjAhtpngVic51EFAfSADEmBPgLGnVMLuFlX_iDcCKcAXVg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Oa77HTEsMYNabxKMdnZuvuU9MmAKF25EBvw8N1UROa4Wf2fsgoetkmzwseRmHwdWgKsWPx0PFAAtvZODWbENrg
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you have any more questions or need further information, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: _O6AdwtXSA-bnxdbhzEJHEdEhgjPrI58X4QZN-fz9LxVqIZWZcoRohWrgIyQROUewQHUVU2KoLDyl_itCU8iPA
  Parent: Oa77HTEsMYNabxKMdnZuvuU9MmAKF25EBvw8N1UROa4Wf2fsgoetkmzwseRmHwdWgKsWPx0PFAAtvZODWbENrg

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: _O6AdwtXSA-bnxdbhzEJHEdEhgjPrI58X4QZN-fz9LxVqIZWZcoRohWrgIyQROUewQHUVU2KoLDyl_itCU8iPA (Untitled)
  Session 2: Oa77HTEsMYNabxKMdnZuvuU9MmAKF25EBvw8N1UROa4Wf2fsgoetkmzwseRmHwdWgKsWPx0PFAAtvZODWbENrg (Untitled)
  Session 3: 4AYqcp-2wqegFsv6KpaPN1YAulh13iugnpP9A6u-EjAhtpngVic51EFAfSADEmBPgLGnVMLuFlX_iDcCKcAXVg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: _O6AdwtXSA-bnxdbhzEJHEdEhgjPrI58X4QZN-fz9LxVqIZWZcoRohWrgIyQROUewQHUVU2KoLDyl_itCU8iPA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: Dn72NL1LZz6mU3Gl-rKkTLP91dSUupIhYo6hT48QXpkPpevW_N46fV60O5PwzWBolbz691FUKZUQtCeY1SQ1Xw

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_verified_user: Get verified and validated user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_users_by_status: Get Users By Status
  - get_active_users: Get Active Users (Updated)
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_rs: Get Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - get_active_users_paginated: Get Active Users (Paginated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("7CGaNsdNUV755aG_po6aZ3Uy9PSIVJso5K07AQt6CbqpBDqFP0Qvfs3PdtIhhi41Vr89uls7hXpwXp7UZNQ87A")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("Su9dq0HXXv1NRoUaCdB1R83sEHbhGhaw1s0_EgDHrDqUc6soXAdqkajRRQv9tLkGgaFu1elKbn3Bwtk06JRgpQ")})
✓ Second upsert (update): Record({"id": String("cgjLiuPysOceYP2eS4Yk7bmTugi_CMuHcgUvju2E1XQD-GdcNzq2X_fQEh67oy5NtnMwlSs4M26C9ZreuqU6Fw")})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"value": String("Alice Johnson"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("7CGaNsdNUV755aG_po6aZ3Uy9PSIVJso5K07AQt6CbqpBDqFP0Qvfs3PdtIhhi41Vr89uls7hXpwXp7UZNQ87A"), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "age": Object({"value": Integer(28), "type": String("Integer")})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("GBoXMs5MZvF5wGaqTcnPkz6UUZm2SEGERWdlheN72LfXgQjbzheDFQBhZTznLDn5cBS0p9fIfzoc7u7zF1DcaA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("Inb6UHsG_ZHpb9it1_4XJir_ncPJKj0FdvprSweLoV-j7k18SJBySoKDejbxxv7APWxIS4wG1VOszHMyNfXJaw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("GBoXMs5MZvF5wGaqTcnPkz6UUZm2SEGERWdlheN72LfXgQjbzheDFQBhZTznLDn5cBS0p9fIfzoc7u7zF1DcaA"), "name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"value": Integer(150), "type": String("Integer")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("QgbG2KWcknFBXBuA9CW20F7JMKurSMxQ4BEGSgx0aI-Hd1x2UVJEzDqRL0zjZH-swNl3MEUn9U4m8uuulOLdpQ")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
  Fields returned: ["email", "created_at", "avatar_url", "id", "user_role", "bio", "status", "age", "name"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["secret_token", "email", "api_key", "id", "name", "status", "password", "created_at", "age", "user_role", "bio", "avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["email", "name", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
