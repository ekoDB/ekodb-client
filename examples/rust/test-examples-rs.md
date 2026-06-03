make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("HVnl0j_zylqKGRIzGx_abG1sjTtBAJYNcnyLA-_jmLHLfz3jwgr6ZqoxCoBB16U8SO5BKzceoB9lk78nsXt6iQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HVnl0j_zylqKGRIzGx_abG1sjTtBAJYNcnyLA-_jmLHLfz3jwgr6ZqoxCoBB16U8SO5BKzceoB9lk78nsXt6iQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HVnl0j_zylqKGRIzGx_abG1sjTtBAJYNcnyLA-_jmLHLfz3jwgr6ZqoxCoBB16U8SO5BKzceoB9lk78nsXt6iQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HVnl0j_zylqKGRIzGx_abG1sjTtBAJYNcnyLA-_jmLHLfz3jwgr6ZqoxCoBB16U8SO5BKzceoB9lk78nsXt6iQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "yUKfe5gayo-nz3tFoxV7DkJpdK6NtxseDcnXmbywpf4An4h7dCXpPeM61DAfE14McsWdNePC96131lbVK03DeQ"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1780487924813096000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "yUKfe5gayo-nz3tFoxV7DkJpdK6NtxseDcnXmbywpf4An4h7dCXpPeM61DAfE14McsWdNePC96131lbVK03DeQ",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "F5F95Dn-1mWUSXBAw018j2X8VkHyj_Q2fVsiuk37vipzGI52cc0YLaqDnLe63q9M1dnLGBynqb9Z16RUXkvGVg"

=== List Collections ===
Total collections: 5
Sample collections: ["batch_users", "test_collection", "demo_collection", "audit__ek0_testing", "websocket_test"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "c5Zrz0Tj9a8E1mnHrAhBTau2dsoq7_kGzvFXuLDqPxWsV2gwW988ODC6u3qRKq7xxaAhU44zpZ_WdZsEw5yJ1w"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "h-W3MGGlNaqdddxy_d_4ehqVeKuzyfMuytCIc-Dshbi4zU_ezX6_udERdLX8FnwYJstVXYFfNZgaDBS5ukYdxg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "dldMlPNCjK8zU9I8cBHwSi1YkvYYp9cUlpYnqVw_VNRP8tQFa4V3qX-NEQSlyLZSkf21zW5oHyLrkGbegtPs7A"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("1bHtpSKNjQWhtHeMeajul912hFZar4iT1wxEOSwnjDej2FizriGGMTWdkST8jk90SB1gMFDY8dpOtBHfjAWcyQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "1bHtpSKNjQWhtHeMeajul912hFZar4iT1wxEOSwnjDej2FizriGGMTWdkST8jk90SB1gMFDY8dpOtBHfjAWcyQ"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-06-03T13:00:47.190972Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: jMZTASq2cg9eFur2y5hA2oBBWwzx3Z_zcwGXjdhbBVXwZ_9s3RhlLpD17BdUI56i4J35XbDa_aZNDyJpMH229A
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: c9-zBY6CuUFuok7j_bvTiKA7tYKhbdu3bW3dHsI8c8ba1sV6wwkM8uL1QyfncciuCK5Q11zpWKIWxrO3-c_SbQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: QdIB8v6pJ3T6YPB1HIUXo1uvOXMQNYK0egZ0tc9M5GJlHg5uVqqH_BcNmtkqU6VgV9CwjIQ8Xh6Zhdqb_XtwBg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 1ezP5mdmhtUTf27DDZot2JMRBl5CeU5iCO_10zLyfjJF7r-1-1RUlqmK0_-WeYz4Z1ASuzA6m9CcJlVUdHwSng
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ISHj4wavAOc2MxXlr3toNslax3IvmrDTbQwI2JnWJA5aq8ADzDEIqiaHX7fOKGIXl_cmzbCezYviK-1MSdGL6g
Created Bob: $500 - ID: WxKGH0ceA8HtrDFpF7pSbIHyNfWLDfW31vk9KFgVd8nYmzDbU_CLqQnmQ40m42C951pKIvp18t31qW05WLqUsQ

=== Example 1: Begin Transaction ===
Transaction ID: 4cfb5ca5-8183-4c6d-9db2-04fa2b699328

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
New transaction: d2896178-96f3-409f-91be-dd2738b4976e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("14sXrfGhqH81il6Mja2p-tR_Qe4u_azmcnrMsseJKr7hqTGcQGchmUvxAcYEhJvY_NlV5Vz3KWlnHGc94aYrHw")})

=== Find by ID ===
Found: Record({"tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "name": Object({"type": String("String"), "value": String("Test Record")}), "id": String("14sXrfGhqH81il6Mja2p-tR_Qe4u_azmcnrMsseJKr7hqTGcQGchmUvxAcYEhJvY_NlV5Vz3KWlnHGc94aYrHw"), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "created_at": Object({"value": String("2026-06-03T12:01:31.688410Z"), "type": String("DateTime")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-06-03T12:01:31.688410Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "id": String("14sXrfGhqH81il6Mja2p-tR_Qe4u_azmcnrMsseJKr7hqTGcQGchmUvxAcYEhJvY_NlV5Vz3KWlnHGc94aYrHw"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-06-03T12:01:31.688410Z")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "value": Object({"value": Integer(42), "type": String("Integer")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "name": Object({"type": String("String"), "value": String("Test Record")})})]

=== Update Document ===
Updated: Record({"metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "id": String("14sXrfGhqH81il6Mja2p-tR_Qe4u_azmcnrMsseJKr7hqTGcQGchmUvxAcYEhJvY_NlV5Vz3KWlnHGc94aYrHw"), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "created_at": Object({"value": String("2026-06-03T12:01:31.688410Z"), "type": String("DateTime")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: nPImtgI03tNzyWYOg8AdR5u_VQjRLxIacdgHLWlbQAWAEnNpRP2haIjxtlaL17S7qM-YyKN-XgId7j63rwObYw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  value: 42
  id: "nPImtgI03tNzyWYOg8AdR5u_VQjRLxIacdgHLWlbQAWAEnNpRP2haIjxtlaL17S7qM-YyKN-XgId7j63rwObYw"
  active: true
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "IKQRSrng_XiNQV2fF_UOBfdWBwi3J5fm5IIG6vHlSbiRNkTDwXXcthQSk_teioAB3QI3yitt0VgZXpOcV_IwvQ"

=== List Collections ===
Total collections: 11
Sample collections: ["test_accounts", "batch_users", "functions__ek0_testing", "ws_ttl_test", "test_collection"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: cjtXpZDVKbNgZ4teEfrV-jNmlX1oJ2D-OKBcTlIZesDG4JiggPEmQ5Mrm8zzjWsa4G0EFFte3Jm3gz0DkyP8Gw
Created Bob: $500 - ID: NWew4OIxQE2jPdQpbOXxPSj1yfbNNKZP59SnQ1I-PRE2UxmE9dQDkgCXslPIolvaaEwqJnHC4HG72KqDSooFbg

=== Example 1: Begin Transaction ===
Transaction ID: 5188a926-9e56-49a1-b15e-44155de1721b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: cb123a20-5cd4-4a71-a314-d4a110f88c67
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
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
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
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
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
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
     Matched: ["title.value", "description", "description.value", "title"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
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
Execution time: 12ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("CEVBS7bCcWTOVstnOiknA7W3iKrgredw9CTrazCkPg6vadlVh8Is0xM7Bq0dWHQlY4Nr4QSRFBJAti0X_-aGdQ"))
✓ Inserted user 2: Some(String("TxlUK_mU-UTtIp4bz8_p3-LAiD81F1MFP5U4m5u2euj6an_o5488_fjooqUdG2h-NlXBSlRmxzSsayShWVqwIQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - email: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 11
  Sample: ["test_accounts", "batch_users", "functions__ek0_testing", "ws_ttl_test", "test_collection"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "uiOd18FcRj0QIV-Z1ApIiTu1peZOPL5hyRN1YwDkaxKodCcBqNgZOf8Njy6H6owiyn_pYccEFLGvFvaOmlkPXA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("NQPFoIQqsX8NK1_aSq0avCCVZJkDdarH0lsspDzOZQTOnJ3wixOBT7XoOjh0hTS8IsCezDsDbJ8sWxxi9tDmaQ"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("QnXaJndOoTbvNjS3yaqfU2Rc9ET-VcJc6hNMJBVBAy4QsRMWW9LRArZjr9JFVHaQo2P5xLQmQrEgxkM3qbYHOA"))

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
✓ Edge cache script created: wi37kixH2YcF1grxsQA1ROCVrvO--NRx9-QwvH2ZpYceCme462ETQUhbxvEnBf0Y2AYjq9cxkDRswj198gx1VA

Call 1: Cache lookup
Response time: 32ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 15ms
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
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: k1h_Q72Btz4PaGNICLPRa6e4e0KDNgbScqqpybrIIe5oiN6WhnhZxj4OAilPqKhc5yWIsICed2YwhTUmpP6Lfw
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: QEeIWCtgpd0ybFoCkWmdQ6tSSSLi-4uOJGjY8kryZNkexg05TgsNFH8Yru5TVLTzeasrV11u9rzPGx_M16STwg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 5i-l99iEUFh3diuUQUdQMD5wNtB5rkb9HX-d8bjx8xj3dcn5CfmckTE1RA1tK8xiao3iPDsI3pHXUSX2nPtvXA
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 5
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
   ⏱️  Duration: 257.725791ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "zipcode": "92998-3874",
        "geo": {
          "lng"...

Second call (cache hit - from cache):
   ⏱️  Duration: 42.991333ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "username": "Bret",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Cr...
   🚀 Cache speedup: 6.1x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: E_NV85nexyDtv4fk9T0-9xeZdGPJVuyM4e-aIVE7ysHQh-7wH-PjJpFk-fsRUSRiD2TklGz4KZo_zOBjSzyk0w
📊 Found 2 product groups
   Record({"avg_price": Object({"value": Float(575.6666666666666), "type": String("Float")}), "category": Object({"value": String("Electronics"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
   Record({"category": Object({"value": String("Furniture"), "type": String("String")}), "avg_price": Object({"value": Float(474.0), "type": String("Float")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("hqqtYpqCYOf8cf8P_zPc_6zMJAlQO_WOIR2uJns61VGFDZeZ1nibZSNigcDHB1S8pK7AotmaWydN-pic4emdNg"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: mjI-QJiRNeVPCraUvXPHDC5B8CyK7O5bv6OPxVnOs5_FJseP4C7Qgw2r_VOAEuw_i80nsPNUpdNa5-hbdx1IKg
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: j03k4gU9uD7idMG9Tp0nROkHlRzZpnYeFL1w1JVysL1Ek8KuCEEqBpcqrbYuhefBSMXjR7cys7L4A2Kj7ufoCg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: X1OzwB5AKybG_hzBs_WSPWlBaLdqgFZavv7vVbVfrMjzyykV-09lrwXV4ejJF_mxySrSaCtNDNTOzNNkBOkLOw
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
✓ Created SWR script: fetch_api_user_rs (IwWP2s0Ho6h74m4jjQrbqno8Uw9sUCCVANzFWgsPSC1roXR3r_IR4zVZoR4V44aIestN9D7y9mJj5eumTRUB2Q)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 49ms (served from cache)
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

✓ Created native SWR script: github_user_native (moYFrpFy_EKQTcK67hX2ct_OWdfvzPDFoWbh5bl-o2syuAe6_oi-5WWkMgI6ETNTnLW2OiHNnXZeErSw4plWNQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 181ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 35ms
  Speedup: 5.2x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (Zzx7SQx3d4MBVoge8PWXj8aulUggaksYuSUZlLojrwGN2Tj2g0ZIsMr8an8LrjI58cDeJmbznsqj6crDg63iuQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (O4KCq1SuC-1PH3qBaN1HAy2_Eb6-dUzj1-NcmJb5YFCJ9dWHGvATWlvRZ11jifUmFPtla6e-gBPxUEKL2Xa_Gg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (jk2NNBNYPmTR43vrxJcmmVJKzZ54t315LBail2UE8y4E8E8tckeHF2IRgvrlFldmdlqUAVr54DmuXS6sfJN76w)
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
   Record({"category": Object({"value": String("Electronics"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(5)}), "avg_price": Object({"type": String("Float"), "value": Float(367.0)})})
   Record({"avg_price": Object({"type": String("Float"), "value": Float(365.6666666666667)}), "count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
   Record({"status": Object({"value": String("inactive"), "type": String("String")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(7)}), "status": Object({"value": String("active"), "type": String("String")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Getting Started with ekoDB (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"type": String("String"), "value": String("Database")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"type": String("String"), "value": String("AI")})})
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
✓ Created session: 1kCDSQLf0CZKIdZWJ87PeCN_wAxGnjZ1ZtnJsd9_xucJCKqu9A1wsa4XwoyhCe4V0Zkvuzk8JtfTHqqXLP0Yrw

=== Sending Chat Message ===
Message ID: LBvupgbMw4LeFxZSeWZcQ8h__vTBsgWd05_x9PI-CM0YjQjuG5EARaW49gWcLb7Ef6Cslk1sLclO6mSTPhQL2g

=== AI Response ===
Response 1: ekoDB is a high-performance database that integrates intelligent caching, real-time capabilities, and AI features to provide efficient data management and retrieval. 

Some key features of ekoDB include:

1. **AI Chat Integration**: The chat feature allows you to query your database using natural language and receive AI-powered responses with relevant context.
2. **Search Capabilities**: ekoDB supports full-text search, vector search, and hybrid search, enabling automatic context retrieval for efficient information access.

These features combined make ekoDB a powerful tool for managing and accessing data effectively.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("FfHJ8ShBH88Irn74y-ZXnEgFOX22FJbJ42tIB7y7i7cAfufWNxSx3Xn9cf6GnOoSQG83JdYhCJudiC0hi-zCPQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("h-qlY5Z0QrV2POefF6NnYx2BRPfB_url0bNYoxrryTlHYGhGpWiyQMxmoIvB1tPRbNRhE39eZ9NcDXJ8z6XA6g"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("GBILAVgQmmh48CBTFbrfrLI6KG1IdYsNM7UFv4GQq_eTtELgtRBFQ4OtN5JDNCft79wmrtMSP7MFZevlk4unSA"), "title": String("Introduction to ekoDB")}

Execution Time: 11385ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1WUhqU3-zNE9IJvc1-tJo4bi4qJ7fqdB3fPtCX5Er1vGFc27EyDwnyq49os_iffgEX_aeL5IkrOk86eWvS7a2A

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any more questions or need further assistance, feel free to ask!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: X1XPk6Um2QpQcjjvhp87Y7-vbyDSFPgOyr4aOT09Qit8sPQEBknbZIw13dxL3Noc_woKklsKDWSJb0oGe0C0XA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Ugyl71pwWgqEAYkNKt1oukam5QXK3PoxRBFeZA4rQgJIUBBEyEHpyaLliBC6zOyG3otAcamLQhGFdklYNOHtHQ
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name**: ekoDB
- **Description**: A high-performance database product with AI capabilities
- **Price**: $99

Is there anything else you would like to know about this product or any other products?

✓ Message 2 sent
  Response: The price of the product "ekoDB" is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5r8HQAMWZdJHg-Iw467P8vNDKYfHXWvkbToaLbmGvEBTPITrJQrmtyYsehgiU_sFOyf5C7-ZtLcr05Ff7NUtLw
  Parent: Ugyl71pwWgqEAYkNKt1oukam5QXK3PoxRBFeZA4rQgJIUBBEyEHpyaLliBC6zOyG3otAcamLQhGFdklYNOHtHQ

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: 5r8HQAMWZdJHg-Iw467P8vNDKYfHXWvkbToaLbmGvEBTPITrJQrmtyYsehgiU_sFOyf5C7-ZtLcr05Ff7NUtLw (Untitled)
  Session 2: Ugyl71pwWgqEAYkNKt1oukam5QXK3PoxRBFeZA4rQgJIUBBEyEHpyaLliBC6zOyG3otAcamLQhGFdklYNOHtHQ (Untitled)
  Session 3: X1XPk6Um2QpQcjjvhp87Y7-vbyDSFPgOyr4aOT09Qit8sPQEBknbZIw13dxL3Noc_woKklsKDWSJb0oGe0C0XA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 5r8HQAMWZdJHg-Iw467P8vNDKYfHXWvkbToaLbmGvEBTPITrJQrmtyYsehgiU_sFOyf5C7-ZtLcr05Ff7NUtLw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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

Anthropic models (10):
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805
  - claude-opus-4-20250514
  - claude-sonnet-4-20250514

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

anthropic models (10):
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  ... and 5 more

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
Created user function with ID: ZnkVO5CCtTArKqULT34zQIWXoV-N38X1M4jmeqP3zcAa8wSgMTS6J1meNDlF-r8KdA40O9Hg62SVvR-U8U67dQ

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 12 user functions:
  - get_verified_user: Get verified and validated user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - get_active_users_rs: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("Ec3MAIdIdW41k8CdliEhiHgLfC4_UOaVbhV-tfZG1n2bFBqzcxlWly6G3ud4pIlldUUS3z2NVCiYw4SW4sc70w")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("4djNRzqVYjxkUR1PfC5-as80vS0iQAe8hofOHrDA_ndxoASwzY4I8E7MOOqOET2HwLO2hTrgAdm0tyKog5nPZA")})
✓ Second upsert (update): Record({"id": String("AGccffOTGz0s15qqkIS3oMcyT9G-xfS0Hdz28I7cm7Dr4oO4bdDUlgxzA09OqhpS0k3uNGV6seEMF3xhfdeEsw")})

=== Find One Operation ===
✓ Found user by email: Record({"id": String("Ec3MAIdIdW41k8CdliEhiHgLfC4_UOaVbhV-tfZG1n2bFBqzcxlWly6G3ud4pIlldUUS3z2NVCiYw4SW4sc70w"), "name": Object({"value": String("Alice Johnson"), "type": String("String")}), "email": Object({"value": String("alice@example.com"), "type": String("String")}), "age": Object({"value": Integer(28), "type": String("Integer")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})})
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
   Inserted with ripple: Record({"id": String("vVL7eDb5jawrTudvd8AETO5Le1HA2ckh-hebJ2n97vLmesy7ni96sCp1a1bWwIlHJV2oSz9cSA1d9ujRxIXRLg")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("9UqUQh8jMxUWXRwk3Cssythm1m1eaGRER5yg3i3soRH8UfFBL0psDEooTopAUIjggACRk6JzIpE74ZAqcGJSEQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("vVL7eDb5jawrTudvd8AETO5Le1HA2ckh-hebJ2n97vLmesy7ni96sCp1a1bWwIlHJV2oSz9cSA1d9ujRxIXRLg")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("Jocm-dgYx7mr9KbUOgkcCxsprny-MKhocsyVzBtPGfNZteGPOtjTN8KkcEGQ2LpKrFpWV4azF7KlnAKy_aZJew")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email", "name", "id"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age", "status", "user_role", "avatar_url", "id", "created_at", "email", "name", "bio"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["api_key", "password", "name", "user_role", "status", "email", "secret_token", "avatar_url", "bio", "id", "age", "created_at"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "id", "email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
