make test-examples
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("LlBYoDIs9rx0-GtIVpJqZt-GUy6Mwcept_CaWrIUVLOF1yasokxov4pzezK5Om7MFErtCAfIrQtpM7Lw4T66Aw")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("LlBYoDIs9rx0-GtIVpJqZt-GUy6Mwcept_CaWrIUVLOF1yasokxov4pzezK5Om7MFErtCAfIrQtpM7Lw4T66Aw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("LlBYoDIs9rx0-GtIVpJqZt-GUy6Mwcept_CaWrIUVLOF1yasokxov4pzezK5Om7MFErtCAfIrQtpM7Lw4T66Aw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("LlBYoDIs9rx0-GtIVpJqZt-GUy6Mwcept_CaWrIUVLOF1yasokxov4pzezK5Om7MFErtCAfIrQtpM7Lw4T66Aw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "uECQwVBLmwZzmL6GZrW53lL84vBuwZLnKCf76m2k1zWzCWK2lhqx5kfniy4ncQB0Ggm7bX1kOBzBkTnjpJKOHg"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774741672748275000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "uECQwVBLmwZzmL6GZrW53lL84vBuwZLnKCf76m2k1zWzCWK2lhqx5kfniy4ncQB0Ggm7bX1kOBzBkTnjpJKOHg",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
Collection created with first record: "aKfQwlL830cWHwm1FBkmu_ZyuCawWGb25iqrvX0pcvCxOQEI-YkHSxOVjkUkUQhWOPB-p3Xs9H9wr7C7_9wyyg"

=== List Collections ===
Total collections: 6
Sample collections: ["demo_collection", "websocket_test", "batch_users", "audit__ek0_testing", "test_collection", "distributed_locks"]

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
✓ Inserted document: "xXP6wIxbynTxf-Z9yL7K6nBh8862f0wWQgisIVSIfiM32i__J6Rp_mzM9VJ5xQjlVRau0-aAoSSQalRhra2izQ"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "5IlU1MNhYdbK4aMbjjOzAUilzahs6UnZnnKWKSD_dqaqE-VWE78s5OlL1FMZG8dfxo5z7BQkSeipExkWKEO3Xg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "eG8SqpyqBV46IZwLFdcg3kQTiwVB9EocpsBbsBWolNdJhcRnsgj4VKPinL3wfxM_k_TfXGwGE7IOVcdkaY-lCw"

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
✓ Inserted document with TTL: String("3dHIjVvA6y1dEkKztO6gpcaHDjYmqh_iEJ8J8ekL7BJdaJkno91qH6IbK-tl3vqu8yseHOp284uCdIa1H9QbLQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "3dHIjVvA6y1dEkKztO6gpcaHDjYmqh_iEJ8J8ekL7BJdaJkno91qH6IbK-tl3vqu8yseHOp284uCdIa1H9QbLQ"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-03-29T00:48:03.514796Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: ypINnfQ1MOzd6FjgHWQ2roSi0DsEd3JLBGYJF59VFMInZzBUD4ut60MenVADpdhr0EXualyr_OYQacF7ElYU-g
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "wbL8apcEQXFga8g1ZEcDdPEuabsLd-ii53AUj3hETSPlQ9xPPkVLTsJ_ZcSPvYAiC_ZfpIK6ISzISPw6xnT_9A"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: cDfbpRw2esBl1LXiJbdHTfdHf4esVJukG7d6KSZgktXxS0GwgH87yxstxTqglw9AYr-6R3yBHKfIJwV_IBxthg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: z3GQADYM1_bsFDYV-WPMI3_UvI9AFxAV3LT_Mb1Wp7EbXg1fOtPRJdj1CIjJA8ckYb5aq2qNY0J7j6ljyCQ9mQ
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: VKEYcGDCfa1Xa9ipTMKnI8CCq5jZuV4T9AXHXQSXyzP3un7TH1jrgfuHy-a0i56pkKHqwphJNFZsFdpovT_vpA
Created Bob: $500 - ID: ctvMhPbh0w04dVaxKkSWvKAyxK0jJHJ6StwmspFxyUJ1GccLoZkQGIVyVqyTBAOgcMWdjln2VnkcDF9O5PICyg

=== Example 1: Begin Transaction ===
Transaction ID: 4ec15ec2-f2fe-4e1c-b718-2193ea3a2f6b

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
New transaction: c711cec9-e61c-443b-a9ce-0b451e65fcb7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("nbVqtK0AVdlOb_gXeHSaL1YKo6ED-YWjnRrscvScI116Hc39NsvXJu2F7JcUoG9PZKIWxhWi7zrYyJjjWKKeqg")})

=== Find by ID ===
Found: Record({"created_at": Object({"type": String("DateTime"), "value": String("2026-03-28T23:48:08.235432Z")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "id": String("nbVqtK0AVdlOb_gXeHSaL1YKo6ED-YWjnRrscvScI116Hc39NsvXJu2F7JcUoG9PZKIWxhWi7zrYyJjjWKKeqg"), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-03-28T23:48:08.235432Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "created_at": Object({"value": String("2026-03-28T23:48:08.235432Z"), "type": String("DateTime")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "name": Object({"type": String("String"), "value": String("Test Record")}), "id": String("nbVqtK0AVdlOb_gXeHSaL1YKo6ED-YWjnRrscvScI116Hc39NsvXJu2F7JcUoG9PZKIWxhWi7zrYyJjjWKKeqg"), "price": Object({"type": String("Float"), "value": Float(99.99)})})]

=== Update Document ===
Updated: Record({"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "id": String("nbVqtK0AVdlOb_gXeHSaL1YKo6ED-YWjnRrscvScI116Hc39NsvXJu2F7JcUoG9PZKIWxhWi7zrYyJjjWKKeqg"), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-03-28T23:48:08.235432Z")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"value": Integer(100), "type": String("Integer")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"value": String("Updated Record"), "type": String("String")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: _YW54EpargzSRnIkicCWqRqyGsdwZ4xBEO9-U_7ET5pYwwTJINdSc4WwXPRx6_OyjDMMiHwpOOa8xqq6gtY_Lg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  name: "WebSocket Test Record"
  id: "_YW54EpargzSRnIkicCWqRqyGsdwZ4xBEO9-U_7ET5pYwwTJINdSc4WwXPRx6_OyjDMMiHwpOOa8xqq6gtY_Lg"
  active: true
  value: 42

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
Collection created with first record: "hZKkihIRsGRX4s4CRgr6whsEHNbXuR_BvXGnNQGULQL0Wkv62XIBXOWJBBKfbuh1CTsrCR7nSHO2ssvz-qAi5g"

=== List Collections ===
Total collections: 11
Sample collections: ["users", "ws_ttl_test", "scripts__ek0_testing", "ttl_cache", "websocket_test"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: u8eO_b_JsLKaSgGkKqVxZe14K5f9cAFHEx8nyiflu1VCQxItFmB9d28ZijAutXj0ztcPazlevOyvS4j-wffJvw
Created Bob: $500 - ID: pzzbvBhXmIq9zEC1LSPE9ut4joc6R_DSoGqpzytq5VV4hqnQRFQs2mmOzok3mbHWnS_8aBkuMJm5Hk1ibgM-sA

=== Example 1: Begin Transaction ===
Transaction ID: da3775b8-f525-4d3b-9653-ff85f8ae9739

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3d940e44-a7bd-49d7-a7da-df1b176a4eaf
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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
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
     Matched: ["title.value", "title", "description", "description.value"]
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
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("owrY7lb4wRaYaKaTo13VDRYETl0_9rQennyx9sVqKGSjkinK_NEASr-yegrMFicJt67PiBoMCzaujcpJvyhY5g"))
✓ Inserted user 2: Some(String("PpF7EztI28KHDGnKUel0Q360lGax5LXe1RNpBrYe48wYCO0k5e9FM-qnAioaqSs-KxxB2IAA6-x50v1smCHOIQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - age: Integer
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 11
  Sample: ["users", "ws_ttl_test", "scripts__ek0_testing", "ttl_cache", "websocket_test"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "h8kqkvUzNB4jLDuS1OwDojjRKzSS3JuSLQM7b-0m6aWUwyQ710lCCvwxyWvrFjNBVyH7xOrxUPdtZDejwphlDw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("8yejJQWlwzpKtrM_hCA-yy5a8zeP2FqO8CaGXrYeVLo9jPVIucbWlHeW5pLdZp0ASbDrEqyJZxiVsfHh-dpsLw"))

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
✓ Inserted document with TTL: Some(String("_LFU7HGrD8ay_PR7Y_QZOqwaw_a_rL4Qa7Dd6n--0Dm1hRoyTdP6vHAybqT5TalDwf2-wIN_as-xcwDJ6_uDNg"))

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
✓ Edge cache script created: kshqqBU_b42axxBeLXVRaqTmS8CO4IUSpy1m097qwn4iYpaV2G05zZmefxM-dopc0Vj0GuVEQuVEgiTX4o-mVA

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: c3Dc6E1nFl9w17xvb-uYPdFRI9aWSIBs60xaM2LsXUrZ4skwF39VO-Cy5LsxIb-G9mLTA4CotRgGF5nedzbplw
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: AugC8xCxUUEscpwXF1HE-8dDxl6GH8e3uwvhGao208c9uAaSxW5OpG1geFTs077ax3mCiJN0m3Rcagmq-thVlg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: bapNSFw29V-_x915VvcVn4IE04AZm7wTrRc83Pyg1oycg4lOeswICvX3Yvyo-83OpoqZJQ6gqFJKC4_kBvU5Lw
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 186.83125ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "email": "Sincere@april.biz",
    "id": 1,
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-...

Second call (cache hit - from cache):
   ⏱️  Duration: 39.450417ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "h...
   🚀 Cache speedup: 4.8x faster!

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
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 3BJQZ7YWfS19Nq7x7XM2Yrx6pxxmXYkE7EEs52zsZgE0n4QjFh69eTvagj2gT7YoclBgUY_1Z_hf3SHNWg58gQ
📊 Found 2 product groups
   Record({"count": Integer(2), "avg_price": Float(474.0), "category": String("Furniture")})
   Record({"count": Integer(3), "category": String("Electronics"), "avg_price": Float(575.6666666666666)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"category": String("Furniture"), "count": Integer(2)})
   Record({"category": String("Electronics"), "count": Integer(3)})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("AxPsuC2PE37gmB05ykasic_OfxtAm7lcktmsgSZgIvTI54dFSIlOU85rcyg7FVIsKQbFoooQO-uNGuiD7HjJpA"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: thJAOmtAU_gvXrn9wQ6bI01RfvHduqDx6iP4iPf5GJIUFhgmWUz_R8caLasmbHrekxUYt7YNAqnjSY3G5JC_MA
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: M1qhU3pTlqpzkDHrtNJpdtQ0RIljN-YUVjXINm5fs_t-oI_kKRV-4QkdwqtL2WeJxBrO4uqa-5JVd73kpOVpAw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: naIVIQ6dX829aWUjMQfgd2345gxTE5SAZUHmURGLDYZCWlDlkm2LlnviRRiAeORTnDRsrZMkWeAAQOntG1aBbw
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
✓ Created SWR script: fetch_api_user_rs (ywNIrX70NyfWLZNAX5pCabNcd2qVg1ffS5JvTt631v_e0kLoesvxQelQDtB41yKiSOx7gVNdQ9ZkcQzvMKZ3yA)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (HaQwFZgCqcPhtiME-uWaabKmAUV9jhdvZL1_rpCbA9kURVyZoC0iZlbE9uWdRkznQ9CRF3y1JdCgoMfk3RlRAQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 123ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 6.2x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (3BQRKT_37-cxP1jbQdB31kwXuYwA46JIzZ6lXcRk0gSj5GfHdypdFg70qk5q_tQX9DYXS5Yw09iVjcHJJ49G-A)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (kI1WhMt44WZDaBjoHz18_5LVUL8NllDX7Gh9_GuCAjCRxD8PWFAZ2W-uq1BV99FXtW43B8qbDZUvwPqsd99csw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (6lSqADrtRc9FtjRiHL4iHTlrmAJg9ZBi_eupO1r79pxRJ8D00TFj43x8ItzyZZhOPio75IstLkC8vnpfkJyG2g)
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
   Record({"count": Integer(5), "category": String("Electronics"), "avg_price": Float(367.0)})
   Record({"category": String("Furniture"), "count": Integer(3), "avg_price": Float(365.6666666666667)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items based on vector embeddings, which is ideal for tasks like recommendation systems and image recognition.

2. **High Dimensionality Support**: They handle high-dimensional data effectively, making them suitable for applications in natural language processing and machine learning.

3. **Scalability**: Vector databases can scale to handle large datasets and can efficiently manage vast amounts of unstructured data.

4. **Real-time Performance**: They provide quick query responses, essential for applications requiring real-time data access.

5. **Advanced Indexing Techniques**: They utilize indexing methods (e.g., HNSW, IVF) that optimize the search process for nearest neighbors, enhancing performance.

6. **Integration with ML Models**: They seamlessly integrate with machine learning models, allowing for direct storage and querying of model outputs.

7. **Versatility**: Vector databases are applicable in various domains, including e-commerce, healthcare, and social media, supporting complex data types.

8. **Support for Multiple Query Types**: They allow for not just similarity search but also ranges and other complex queries on vector data.

These features make vector databases an attractive solution for modern data-driven applications.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
   Record({"status": String("active"), "count": Integer(7)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
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
✓ Created session: XYf4K4p27uG-Khw_d6gCgyk8l5FE5HJ_sBKkrVPC_B8yIv1SlstejAemAqE_4HfUcKJn6PifBgFYFG6Fc0qMxQ

=== Sending Chat Message ===
Message ID: eyAeokcNS-iSrp_U_bYPErC9jtQeTqBDIdfheHYjXwXIIuh4NeTtc9bMzglVXWthfl3w2YDsc3f2booJIezw2Q

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. Some of its key features include:

1. **AI Chat Integration**: The chat feature allows you to query your database using natural language and receive AI-powered responses with relevant context.

2. **Search Capabilities**: ekoDB supports:
   - **Full-text search**: Allows for robust text searching within the database.
   - **Vector search**: Enables semantic search capability using vector embeddings.
   - **Hybrid search**: Combines both text and vector search for more comprehensive results, along with automatic context retrieval.

These features make ekoDB versatile for handling various data querying needs efficiently.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("-TKge4euzRnlhYkUYgeB_zfJ9cSx7Xf3yqLM87x863FhfBnO5iXbAmV-vOTpKQ8rcO4huf2VnN9mAq5O97OA8A"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("sElIbIzyYIOUZf2rcaeRLTGUc7r8yRujCulD4cGeKlhuDA2xfkoWmcl-ympvkTKiCxQBjY5GXOG1Sy2xMASMFQ"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("W5CvqeAr_ljY__5nrTVFgQ0EMfhn8FGItO7KAdLmenkJSRPIr6joyeo9dnoorUkYUqwDlbnpq-GmOygnmecRrQ"), "title": String("Search Features")}

Execution Time: 3722ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: vRsqOlMjFORPXJQqJiSDy4IRyYW_Pmc8-ZFduwb-jwsRylXXThEpG6dxdEXfZR9FWm8PZWQdH2mCtb_NK73pdA

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
  New response: The price of ekoDB is $99. If you have any other questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: pFPnqAtp08oCL4UUqkRYoHA97h5__QHkMVXMvvIXdGr734rhuDDvY0KBEmhOVYCwLeX-bA_SNR8bLQydscbghA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 9Jr3Qe-CxxNhxTsCRmrM9rNvW0r0x2fzl446Rb9ns4tzaWZvzDxApSQxUtkA8cJvKf_fROyiqNfWYwWM4ahQmw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more information or if there are other products to explore, just let me know!

✓ Message 2 sent
  Response: The price of the ekoDB product is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: cvE4hUcCl2LII9fwerPBPIPwGwTFZuFmmcdHR6QE9ko4PikbILkD5LlJtCG5kcj7Fcx_cjDQ5RlpFiVKIwAGWQ
  Parent: 9Jr3Qe-CxxNhxTsCRmrM9rNvW0r0x2fzl446Rb9ns4tzaWZvzDxApSQxUtkA8cJvKf_fROyiqNfWYwWM4ahQmw

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: cvE4hUcCl2LII9fwerPBPIPwGwTFZuFmmcdHR6QE9ko4PikbILkD5LlJtCG5kcj7Fcx_cjDQ5RlpFiVKIwAGWQ (Untitled)
  Session 2: 9Jr3Qe-CxxNhxTsCRmrM9rNvW0r0x2fzl446Rb9ns4tzaWZvzDxApSQxUtkA8cJvKf_fROyiqNfWYwWM4ahQmw (Untitled)
  Session 3: pFPnqAtp08oCL4UUqkRYoHA97h5__QHkMVXMvvIXdGr734rhuDDvY0KBEmhOVYCwLeX-bA_SNR8bLQydscbghA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: cvE4hUcCl2LII9fwerPBPIPwGwTFZuFmmcdHR6QE9ko4PikbILkD5LlJtCG5kcj7Fcx_cjDQ5RlpFiVKIwAGWQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
Created user function with ID: qTTQBGaYvQS7aNeI-iFOZB4xHHu50aOwS3HEZ75MzBNf0cYRLp_KUf-XaAeZUO8NlGKpbcjoQFZTeUvVVlVAyQ

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_active_users_updated: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - get_active_users_rs: Get Active Users
  - get_active_users: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - get_users_by_status: Get Users By Status
  - get_high_scoring_active_users: Get High Scoring Active Users
  - validate_user: Check if user exists

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("gmnDu0v7ZzWK5xtCwewI6iFNKfy6TyGldPezrYDwU67e9m1a1bikksPlKC9NFMGVz1k5NdL790U0qRZX-6nv5g")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("88LOL5XrOuG_Fqmbg1Yzffyc0srpAqJJucv8M9mj3f6WE07Aywb_i734zGj_nM3E4NAf6PKnkDmyqaz_eakeFw")})
✓ Second upsert (update): Record({"id": String("xXPhFHuwQCKoS9MamdryOYu2TSNZPga3UgNMZw7dJiSRGAtotiRmbSZeF60uTHVVJsboviHNeuYpnvGtX0akSQ")})

=== Find One Operation ===
✓ Found user by email: Record({"id": String("gmnDu0v7ZzWK5xtCwewI6iFNKfy6TyGldPezrYDwU67e9m1a1bikksPlKC9NFMGVz1k5NdL790U0qRZX-6nv5g"), "age": Object({"value": Integer(28), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Alice Johnson")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "email": Object({"value": String("alice@example.com"), "type": String("String")})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("KSJywZLjTYAxb9xybdx7TCbGUvIFaZ7QUI8aDiT8VKfVQwtidsgzQKABRUVDpEyxLgpIC3-gDhIu94mZkMOTmQ")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("xwXBM1X-NZAj7MR8MN4iCt4ZDcXKWveh3CDYKF2qIeYHGlscpK3LfL2vozIe6iJpJNbnz3gt_g0MJg8bC-ujSw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("KSJywZLjTYAxb9xybdx7TCbGUvIFaZ7QUI8aDiT8VKfVQwtidsgzQKABRUVDpEyxLgpIC3-gDhIu94mZkMOTmQ"), "price": Object({"type": String("Integer"), "value": Integer(150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("4bLkdaL00025Fl-C4S85nMdF12cB3j62wnccstYOCopPGlBEPMNO2UbhA6qizZ6VSi7goltR3DXQd0uOsQ7Puw")})

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
  Fields returned: ["email", "age", "bio", "created_at", "name", "status", "avatar_url", "id", "user_role"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["api_key", "password", "age", "avatar_url", "email", "name", "id", "created_at", "bio", "status", "secret_token", "user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "id", "email"]
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
Inserted: {'id': '5Shu-6q0hCwe3FKCrq9AdT1KmahrPhYEoT9t1HvRqcilnDJBuozN573dF0gnA14KASF2QyeXXGwB73CGkpAh1A'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'id': '5Shu-6q0hCwe3FKCrq9AdT1KmahrPhYEoT9t1HvRqcilnDJBuozN573dF0gnA14KASF2QyeXXGwB73CGkpAh1A'}

=== Find with Query ===
Found documents: [{'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 42, 'type': 'Integer'}, 'id': '5Shu-6q0hCwe3FKCrq9AdT1KmahrPhYEoT9t1HvRqcilnDJBuozN573dF0gnA14KASF2QyeXXGwB73CGkpAh1A', 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': '5Shu-6q0hCwe3FKCrq9AdT1KmahrPhYEoT9t1HvRqcilnDJBuozN573dF0gnA14KASF2QyeXXGwB73CGkpAh1A'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: _S5yYpl3xBCh_YyPvDcmKVdwN76GYR1Oecn1I3cEbjoR2IwpNSv7WkqMKf8ONNyBKeTkGJG7Gfwp6qJ1Ag1tbg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "27657813",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "_S5yYpl3xBCh_YyPvDcmKVdwN76GYR1Oecn1I3cEbjoR2IwpNSv7WkqMKf8ONNyBKeTkGJG7Gfwp6qJ1Ag1tbg",
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
        "id": "uECQwVBLmwZzmL6GZrW53lL84vBuwZLnKCf76m2k1zWzCWK2lhqx5kfniy4ncQB0Ggm7bX1kOBzBkTnjpJKOHg",
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

✅ Script saved: 7lj4T79UX9YvVXEzL3ySifTkjZYCfr5dpfscn1LZ366AtB0YVtcSVR_OWZQUffidoe4LEQJrv4OtOnQvgQ5SsA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: ZvoL9zhMYNSe3XARt-m3qNc2Qq7NPSb2P7zBeQgmI-fZ7jwoFbSs5I46yF_vPGiJBwljYGZppN1nV1SHmjpMlg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: l0aYjdkLXGquL6xWi2TX57r8nOW1mwxe6ndrG_nEmGFd_l9nq8vObca52FTu7OYRWXXR3emp6bdIpw2NCTMKbg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 90, 'count': 15, 'status': 'inactive', 'avg_score': 50.0}
   {'avg_score': 60.0, 'count': 15, 'max_score': 100, 'status': 'active'}

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
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
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
Collection created with first record: RU6Vno86YtlLs1UIROH1StPagwX_7oZQJYso09JPOxh8V7HvngOvbo6oq4TyskThFrRuuiHKCAlhQURGJlLpvA

=== List Collections ===
Total collections: 14
Sample collections: ['users', 'chat_configurations__ek0_testing', 'demo_collection', 'ws_ttl_test', 'scripts__ek0_testing']

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
Created Alice: $1000 - ID: Fe86tL-w2j0h0eWaSwWIGuIXFDJT9F2Aqr7qGRh-SRwO1MpbAgoMpxaJWbW1fOImzPHShkUqj77sY2kRTA5v_w
Created Bob: $500 - ID: BHHV1plBnlwlSf2xC4_80-xQVhdUlBTaewHrLzGuAAciUH90RXTtNBZlqSUme9Z646wJ-iMFvnttjw9Ht4ve3w

=== Example 1: Begin Transaction ===
Transaction ID: f6fbfcc6-db34-40d8-96ef-5285542ed11d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 00a2c50c-8260-4afd-be4b-9b1156615a19
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tv6tAZsT4dw9miU6g0acOp_u2RSVjj8fQODGmHIdZ_mUEzD0Oc743dIApEYj-g-brTI1DBF3oLhfs2aSy5zLUQ
Created Bob: $500 - ID: GD7irzWb7TJOJSXJQD_YkPo71F1wioJgD2F6P_I3FKLCdkXQxzgemN_4XZcOxN2xoXFTegVST6A8jRBqxmwMqQ

=== Example 1: Begin Transaction ===
Transaction ID: 81e3651c-319b-4719-9b9e-fd8867f248fe

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


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: fW2ZSwgb3F0GWc9zU1muB82SRjx0lef_w_VMo0l1v5yEv3NFSQqMygqR4IHixlWE0GiLzAS38u_sdfCd-zwj_A
Created Bob: $500 - ID: XqikHCspggLjRQBm8RpEQBFGIpUiil2AGTatEhXiwyI_eRWaIiFs7Zb3hkArIV2qcS1llo4c8hONCYTU1E1sWA

=== Example 1: Begin Transaction ===
Transaction ID: e6045b93-fae7-4f05-8c09-8359a3715e07

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: 48e7162d-b1c4-497b-afec-452aa7e6537b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Jh4WHQkFWNI5cbPd0ZQ9mWiYfJdO7_DdFPB7xubreFKOKBhUrK4uIApjL0zcU6kGRyIQubUh_9aEmhZxOs8o3Q
Created Bob: $500 - ID: BCRjvdUHa71OzCtExyKfVJmcL31IkdAofzX3fJkAz41ydEFj1hORKzzTyLADOLrWmdF68BX42gj1AQ1R27I7Dg

=== Example 1: Begin Transaction ===
Transaction ID: 301ba09a-ae2f-4952-bfc7-c7174c7cc0e7

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: DZ4PijrrFjN7azx774N3hzz1loWk5tDWHAc7Vwo0TzoxXYqZN8jOCL4AR1ygUkmi6woi5O6-U89boz1ATfEsuQ
Created Bob: $500 - ID: bcHVQPL9v-yEnjR82NJFkEKJ7rngjdMpoID2n84hW8yvmleshmDMWSteueEWFlg6L4l6Of_AvJWnRHh5mf-q6w

=== Example 1: Begin Transaction ===
Transaction ID: 93bd2422-5897-4190-a94a-5f0f51312cf9

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: a97d6bd9-7527-4075-9a0d-85b25a1acf86
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6E2jb_Jt8eteq1wD2gksTRgZI9kjwq1cArozwy3hWmO_PnvdkE0yxu0aDOJ5mJcAxVc6wUYPFU0AqX3PPJbkOA
Created Bob: $500 - ID: 7HAuw9ditPHv0lSUWIzmhehNdSG-YbZRbEuuCzOcNguDU2cI-mHgRKArqurpKNOVklR3K-2Xh-o-c5fPZj5WFA

=== Example 1: Begin Transaction ===
Transaction ID: 5290d51d-f0dc-45fd-9107-b163915e44c6

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
   ✅ Script saved: dE6VVoUrp3nZschORKFb_TH4OZd4OnmbjCPS_mpF02dT-e7tHwylZuoay57ApZMN3p-ClJw6BMmYrFbqIPOxBw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: KCRBE4yGWdImsrAzBRMZfgAuOk1lfKfx7oxeak8Bl2r7gHi_0tOVOGIQ-ODpCoM5SR_K1KcLEUkuxq_XbvKTQQ
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: gkPn4X2mmjI5NHS7atgsxyAvOJ89rieF3Yu-jLMUC8KJhHKkA4xHG9oLcxDcy6pbB97yeyAmlKFYFPOw7n9F4g

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: mQH2wCPrNUD76lbIQ0JlLEjfLpq3eNqd9ONdMDRX5HMTn7H1SAD41ltAQFo_ZHYcUQjNVtMFTU6cxgvKEw0whQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: gyeXjIUlmTzNgKG-GA4cYfGz0iQ7jyZUwU-Q28CmvAVd9O-wZine9TuTaD-MvA6UZMGagUrXcMLWEtJVEBtr8w

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: dE6VVoUrp3nZschORKFb...
   ✅ Deleted script: gkPn4X2mmjI5NHS7atgs...
   ✅ Deleted script: mQH2wCPrNUD76lbIQ0Jl...
   ✅ Deleted script: gyeXjIUlmTzNgKG-GA4c...
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
  Output: Document ID = Nrt4c-WDl1QtquMfl5dRNZn0qOofDHY79sk_cc2xnXXKxv9CuHmDBfSqsy7a-c5WrDdI9rM3UTJnY-qe492yfQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(Nrt4c-WDl1QtquMfl5dRNZn0qOofDHY79sk_cc2xnXXKxv9CuHmDBfSqsy7a-c5WrDdI9rM3UTJnY-qe492yfQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(Nrt4c-WDl1QtquMfl5dRNZn0qOofDHY79sk_cc2xnXXKxv9CuHmDBfSqsy7a-c5WrDdI9rM3UTJnY-qe492yfQ)
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
  Output: Document ID = CWmdMMNLBtQtc3NYWZiuUP91CIlEVUYcjIaPlB5AAZOQ_91pX2sOEbE4_WE1RJ9374FZbA0WSO3dohO-GdRzig
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(CWmdMMNLBtQtc3NYWZiuUP91CIlEVUYcjIaPlB5AAZOQ_91pX2sOEbE4_WE1RJ9374FZbA0WSO3dohO-GdRzig)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(CWmdMMNLBtQtc3NYWZiuUP91CIlEVUYcjIaPlB5AAZOQ_91pX2sOEbE4_WE1RJ9374FZbA0WSO3dohO-GdRzig)
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
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.1
    Uninstalling ekodb_client-0.15.1:
      Successfully uninstalled ekodb_client-0.15.1
Successfully installed ekodb-client-0.15.1
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'tMJ1mv6nSokhC3LnDF3tZJzLUMN7MMa8ZlIV-FV15FAvMdIb6p-X9t5zg3_p2-mwmtH_cZ242SnMwdbRZzqz3w'}

=== Find by ID ===
Found: {'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'price': {'type': 'Float', 'value': 99.99}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'id': 'tMJ1mv6nSokhC3LnDF3tZJzLUMN7MMa8ZlIV-FV15FAvMdIb6p-X9t5zg3_p2-mwmtH_cZ242SnMwdbRZzqz3w', 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'value': '2026-03-28T19:49:16.642779', 'type': 'String'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-28 19:49:16.642779
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'id': 'tMJ1mv6nSokhC3LnDF3tZJzLUMN7MMa8ZlIV-FV15FAvMdIb6p-X9t5zg3_p2-mwmtH_cZ242SnMwdbRZzqz3w', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'created_at': '2026-03-28T19:49:16.642779', 'name': 'Test Record', 'value': 42}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'value': True, 'type': 'Boolean'}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'tMJ1mv6nSokhC3LnDF3tZJzLUMN7MMa8ZlIV-FV15FAvMdIb6p-X9t5zg3_p2-mwmtH_cZ242SnMwdbRZzqz3w', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'created_at': {'value': '2026-03-28T19:49:16.642779', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: zMk-8BdxPC6py4gWyrqmrpsJkRrrqxa8r2-kkxiIhqraapDZ1eibKonzvqzJEMggxpPrqrR4codHReSJWaRd-Q

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
Collection created with first record: "jYAS4nBxTyf-6WUEli1_diAt1g0-7iEkpWaI5bQH1ru2ikG7FVhhvjmzdDGn7Bi3ZebSAhWz1xMlONJ78znHgA"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_configurations__ek0_testing', 'ws_ttl_test', 'scripts__ek0_testing', 'chat_messages__ek0_testing', 'ttl_cache']

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
Created Alice: $1000 - ID: nhoA-Bj3vCoTmOgn9FR6Bv4WqLbdKFz_WRmmkhxgYFaEbj96rLw3pogq4GIdHf2RmZ1b5rb8JsS-_9feui60tA
Created Bob: $500 - ID: x8i3nyQmVyMZ4gA-o9_mqxhPF2qHMSvrORz-Vd-s6c6JIU23G3VhDMPnfnITL82PXNFVZyl9GMN1gtDXEYt44Q

=== Example 1: Begin Transaction ===
Transaction ID: 7873b47f-d32d-4c29-bdea-859291288ae1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 27a31446-baab-4a28-a21f-74c0814e48b5
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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio, bio.value
  2. Score: 26.400, Matched: title.value, bio, title, bio.value
  3. Score: 26.400, Matched: bio.value, title.value, title, bio
  4. Score: 26.400, Matched: title.value, title, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title, title.value
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.794
  2. Score: 0.761
  3. Score: 0.748

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.397, Matched: title, content.value, content, title.value
  2. Score: 1.374, Matched: content.value, title.value, content, title
  3. Score: 0.380, Matched: 

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
✓ Inserted document: knRt6NQxICH1R1lrpVK0Zc6sxj2OMMhqI1cJC-AraJKKXbElJ29YBdhhJtYD-GscGt67CtCE7lyseJuRGjQysQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: kJAwzMy-Zr_HuLSgg9vuq7pyrpQpVxfiYnRgj8kZdxxF8o07QVSXL6Tqjr0louRBeENPpeGLPxwJSHQ8hCJOuA

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
✓ Inserted document with TTL: pBsd06CYP45sc5LZoWxSnnCOXnUQ6w1MLwgxeBKzQg4AdxUPn7l7m-5wAwXZ-5DT9n_7yCTsGaRF5lku-bgRzA

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
✓ Edge cache script created: _Im6JtjzTeiVqXIBxZqQMWZKZC51Vu3kG5060HaoXhvVJIjTxoeCKzLIMiB7nLEunNxF9WNg5FJ-Yfwrnvi89A

Call 1: Cache miss (fetches from API)
Response time: 569ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 3.5,
          "time": "2026-03-28T23:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.016570091247558594,
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
Response time: 40ms (14.2x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 3.5,
          "time": "2026-03-28T23:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.016570091247558594,
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

✅ Script saved: xbMTwRjnj8mh217Ty7_mvmWvZcCCtiTF7Za3veYidiiizhOj8zR6PZgKZIBl5Fe4QmqMrZCbBM_JR_duJiWVoQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: jg2H1MChcknNNV_4E_4Au6n_zLseFsx7HICl83aJUjaHPH_mRgFuwGAiBnUw4oeFxo87Kv1EB7YBGntnQmtz8A
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: rJnywIfQOBx4jjQGpaKKoyJGaNuMqDsW2UgYIXncZgrg_i2syuMER93gX_CqeKV22tclmEyDaZ21VgSNbD2UOQ
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
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

Second call (cache hit - from cache):
   ⏱️  Duration: 38.6ms
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

✅ Script saved: 9CMjG2PbupNCEUgbGrg1qCK1DHZf1stwKMobwzgH8aZv8_Nfgda29o6RZjuActsNCHQES4ABDz9bXM44Hdd1MQ
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

✅ Inserted order: 3RQXOJXiSJh6rH1XBrIWxhNWMRbQFpTMcdhvmqmFTnJM-zHBcEJTTSYzHfE3P3Mmi_uHGZ8dwEaRqKprrjAyAw
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

✅ Inserted order: zjgq6EVKFNSgbFgHt5TVwwISiMaVF917sTNU32G1x1uHUMUyq9wumyP0KHKnVo78G5bdUAtArgOF2fnnNQrNtg
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-03-28T23:49:19.365590+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (VwQXgal6qYtgijtfG_j0aoBl7UboNQsB5hMddl-vgGSYj-tCkGzkDm6t0kNIMfsOZJaXd-2EvWn_tH4X-KhIzw)

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
Response time: 39ms (served from cache)
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
✓ Created native SWR script: github_user_native (AUceEZNiKeKDPGiIPnPaThvet78mlAFLrawx9VwdF2Oz0gTmz69YaTMlQuh_vF9PWgyxfiKcl3LczOeefzx3RQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 133ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 24ms
  Speedup: 5.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (ykp3RxfkGqPy-aV4IgwlHFPGcmB8kzUFCLUy96qWcH8SFN32mln-9NqM3dIrPvVbfqOnf2FNEETLRM_8ot4BYg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (zrCjqHvlf4xWTpkX6LZ5dJM49Cz3L5KlH13QOZuvG6FCyFiZLAwz-vfQ9tAxKJcAs4i4UUhZYSsJjnwozc-4SQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (zvIZ2kFZcCw5BvnNGx-og0lzmapxOwxfuHfLTTqyaUjpUfCknVJji2MmoTd_JEtI33HkFo9o03QZQ3pnb705uQ)
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items based on vector representations, ideal for tasks like recommendation systems.

2. **Handling High-Dimensional Data**: They manage complex, high-dimensional data effectively, such as images, text embeddings, and more.

3. **Scalability**: Many vector databases are designed to scale horizontally, accommodating large datasets without performance degradation.

4. **Real-time Querying**: They support real-time similarity searches, which is crucial for applications like chatbots and dynamic user experiences.

5. **Integration with Machine Learning**: They facilitate seamless integration with machine learning workflows, optimizing embedding retrievals.

6. **Flexible Data Models**: Vector databases can handle various data types, including structured and unstructured data.

7. **Enhanced Accuracy**: By using advanced algorithms for vector search, they improve the accuracy of results in finding relevant items.

8. **Support for Multi-modal Data**: They can manage and query across different data modalities, such as text, images, and audio.

These features make vector databases particularly valuable in AI and machine learning applications.
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
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
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
   1. Getting Started with ekoDB (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Vector Databases Explained (Database)
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
✓ Created session: pkmE2E5xs52sdcOVdKgIqWPVAlHqOnXmvl-MXprvOEhMLgAYzuH2XSnmx3V8T12WZMq9xqrfTcTglJQ1Zx4k5A

=== Sending Chat Message ===
Message ID: P2ay3EJ2P9-dmlXVuUwc83RCu5C_sfyk_ZjnYUI90MFfUrnUiyEMzW1xyLqWa87I0Cgc39VaBcV3ufEQ-Mn1Jg

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

2. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

3. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

Execution Time: 1817ms

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
✓ Created session: AY7Fr3fvV83ax7wkv6K-tBliPSBbCwABlJgLLa8xv0qXUyKrYjk0g4f4XrhneDxvRyDR_ToCd7Ei05TvoqVSFQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have other questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['role', 'token_usage', 'id', 'updated_at', 'llm_provider', 'context_snippets', 'content', 'llm_model', 'chat_id', 'created_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any further questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: OUGtIBJszHl4Zgo0ENTpTwyDMiuwPxQJxonyTQx-Reb-PqdBsA46sl1sJvJjUxkaNkDjoy2bL1pg1hFCFOcazQ
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
✓ Created session: QXOdFl0w4yhb1lF1zrevRN2vRu-5DRX9LZ6plyVt-UCxEnXgZTHp7c87IvQFBE69ZvueD4h8qjQvtxzoUsiKJw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Price**: $99

If you need more information or want to explore additional products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 35PIdX_nDYoy19Xk8ygPd2IR_WTFhCg-SrMIlyFoakU8KIoz672BTnNDxqtQCtLNJRoucE2r9elM9u5_m-Ovsw
  Parent: QXOdFl0w4yhb1lF1zrevRN2vRu-5DRX9LZ6plyVt-UCxEnXgZTHp7c87IvQFBE69ZvueD4h8qjQvtxzoUsiKJw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: 35PIdX_nDYoy19Xk8ygPd2IR_WTFhCg-SrMIlyFoakU8KIoz672BTnNDxqtQCtLNJRoucE2r9elM9u5_m-Ovsw (Untitled)
  Session 2: QXOdFl0w4yhb1lF1zrevRN2vRu-5DRX9LZ6plyVt-UCxEnXgZTHp7c87IvQFBE69ZvueD4h8qjQvtxzoUsiKJw (Untitled)
  Session 3: OUGtIBJszHl4Zgo0ENTpTwyDMiuwPxQJxonyTQx-Reb-PqdBsA46sl1sJvJjUxkaNkDjoy2bL1pg1hFCFOcazQ (Untitled)
  Session 4: pkmE2E5xs52sdcOVdKgIqWPVAlHqOnXmvl-MXprvOEhMLgAYzuH2XSnmx3V8T12WZMq9xqrfTcTglJQ1Zx4k5A (Untitled)
  Session 5: 9Jr3Qe-CxxNhxTsCRmrM9rNvW0r0x2fzl446Rb9ns4tzaWZvzDxApSQxUtkA8cJvKf_fROyiqNfWYwWM4ahQmw (Untitled)
  Session 6: pFPnqAtp08oCL4UUqkRYoHA97h5__QHkMVXMvvIXdGr734rhuDDvY0KBEmhOVYCwLeX-bA_SNR8bLQydscbghA (Untitled)
  Session 7: XYf4K4p27uG-Khw_d6gCgyk8l5FE5HJ_sBKkrVPC_B8yIv1SlstejAemAqE_4HfUcKJn6PifBgFYFG6Fc0qMxQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 35PIdX_nDYoy19Xk8ygPd2IR_WTFhCg-SrMIlyFoakU8KIoz672BTnNDxqtQCtLNJRoucE2r9elM9u5_m-Ovsw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'nbAYgGyRU1PwX5rapxTzJV43fZEOAKBcuBjNYY7zQ5K7t5UWswtu-jN9DU6oKylGeb0GJRa5cdfnjsEgf_-KnA'}

=== Upsert Operation ===
✓ Upsert (update existing record): nbAYgGyRU1PwX5rapxTzJV43fZEOAKBcuBjNYY7zQ5K7t5UWswtu-jN9DU6oKylGeb0GJRa5cdfnjsEgf_-KnA
✓ Inserted second record: JSk0vnWmRyItWs5t3MWOeFcescWJ3WqbBqmC_oCwDfZqE6Ml4DvOxM1VFva-FTlNMSSPkLaJxF4KP7k48NapfA
✓ Upsert (update second record): JSk0vnWmRyItWs5t3MWOeFcescWJ3WqbBqmC_oCwDfZqE6Ml4DvOxM1VFva-FTlNMSSPkLaJxF4KP7k48NapfA

=== Find One Operation ===
✓ Found user by email: {'age': {'value': 29, 'type': 'Integer'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'id': 'nbAYgGyRU1PwX5rapxTzJV43fZEOAKBcuBjNYY7zQ5K7t5UWswtu-jN9DU6oKylGeb0GJRa5cdfnjsEgf_-KnA', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}
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
   Inserted with ripple: {'id': 'jOImT9h5ZzgrsxQQthtybU9m73YcTFptT4UPckHAkclm0BwbLTpaSq5gm948rdWHs-F3wPgLjcUBYR8-UOd80A'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'lLacRyJOYyiIjz115l6hH-BCrWaLVr2baXWRFDjQ8E8ILWjTtAZLm_WZBYwjitOz9qh9doG6oHU6KQF0VaimwA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'jOImT9h5ZzgrsxQQthtybU9m73YcTFptT4UPckHAkclm0BwbLTpaSq5gm948rdWHs-F3wPgLjcUBYR8-UOd80A', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'id': 'jOImT9h5ZzgrsxQQthtybU9m73YcTFptT4UPckHAkclm0BwbLTpaSq5gm948rdWHs-F3wPgLjcUBYR8-UOd80A', 'price': {'value': 500, 'type': 'Integer'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['name', 'id', 'email']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'value': 'alice@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['status', 'email', 'bio', 'id', 'user_role', 'created_at', 'name', 'age', 'avatar_url']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'value': 30, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['secret_token', 'created_at', 'id', 'avatar_url', 'password', 'email', 'status', 'name', 'age', 'user_role', 'api_key', 'bio']
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
Created user function with ID: toHZ34wgW9Xjb5bETP210Q6Q47WAAJGTbP5XYsloy_hkIAPrxPw_6mr2NcIeGE9Gq2TShevbAMNvEDAFE9k4Pw

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - get_verified_user: Get verified and validated user
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users: Get Active Users (Updated)
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - get_active_users: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_users_by_status: Get Users By Status
  - get_users_by_status: Get Users By Status
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_user: Fetch user by code
  - get_active_users_py: Get Active Users
  - validate_user: Check if user exists

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
Inserted: map[id:2UO1rTVfMzn-zneyEaBvuv_YL8OvPOCGS6Ko7H0ceQIHQuQAsp7HTgDMCSjckA2kOW4kGx6SD5jx-o6gJihMbg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:2UO1rTVfMzn-zneyEaBvuv_YL8OvPOCGS6Ko7H0ceQIHQuQAsp7HTgDMCSjckA2kOW4kGx6SD5jx-o6gJihMbg name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:2UO1rTVfMzn-zneyEaBvuv_YL8OvPOCGS6Ko7H0ceQIHQuQAsp7HTgDMCSjckA2kOW4kGx6SD5jx-o6gJihMbg name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:2UO1rTVfMzn-zneyEaBvuv_YL8OvPOCGS6Ko7H0ceQIHQuQAsp7HTgDMCSjckA2kOW4kGx6SD5jx-o6gJihMbg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: mmpnpqj8wl8y2nCQsIu6Gr0V-dAmd7j7kPjU6Xqi3-P9by1YQW8f19sL-wubi6PxdiWatf2aQJbUal9HOFNuXQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774741792406614000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "_S5yYpl3xBCh_YyPvDcmKVdwN76GYR1Oecn1I3cEbjoR2IwpNSv7WkqMKf8ONNyBKeTkGJG7Gfwp6qJ1Ag1tbg",
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
        "id": "mmpnpqj8wl8y2nCQsIu6Gr0V-dAmd7j7kPjU6Xqi3-P9by1YQW8f19sL-wubi6PxdiWatf2aQJbUal9HOFNuXQ",
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
        "id": "uECQwVBLmwZzmL6GZrW53lL84vBuwZLnKCf76m2k1zWzCWK2lhqx5kfniy4ncQB0Ggm7bX1kOBzBkTnjpJKOHg",
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

✅ Script saved: kB8XQPKbOmQ7DDYippc5oveqABKMj3XAzzZHq9AW0zKHaUvWAXPXTG0qGRShLfNfbYVAQgKVHxj4Vci3ofUObQ
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: lb9RF8dwYhRPqJzT5uZHnWBWU7LpggabwtowHnMuEqOpAQLHvQSN6LY8GJf8D_zXZwVdzeLNwtk6CTzQ2j3XOw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: bD1G8oaJFVzeQC8k4eqnw56bTuebOQfmmiNh_xmefPlo_SU3RlbPREWazGCKsO41AFSultQYx204Jc0NBj0_nA
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
Collection created with first record: _kiG7741ayUsMpWge2U0dzopaQNeGWdgnLt1uv_Lw_03JFB7rMHEU231P037KecDxsXKSreAeSJSW2-KU4bbrw

=== List Collections ===
Total collections: 18
Sample collections: [users chat_configurations__ek0_testing demo_collection ws_ttl_test scripts__ek0_testing]

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
Created Alice: $1000 - ID: _pfJWns76BkLEx0eqUyuJXjs6G2Wwkgp_89o4lB6GUqpCZcYApgm-pmK1OBOAgbP3zV3pHCaJeedOnOANNoa5w
Created Bob: $500 - ID: VLF4IXYRU1HD8V-pvg5faODhi_-3fbV68w6zWhT_cvibsFGVscqbnJor_ekYX2Vxc9LOAqjn-lnWnwrYBhJvLQ

=== Example 1: Begin Transaction ===
Transaction ID: 9117587c-5a24-4126-a9c3-ff4edeb61986

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
New transaction: 5fe53354-a7dc-4234-9953-2a37c6dadf8d
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
   ✅ Script saved: Zrfu8T7luwOoFfh0szrqNtcFWcT1ZRfCrREIr3oUCebtUjJ7L4M_fax0LLKz3kOo_3ceWbV2IdOBqBDCR5QJxA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: Q0jaCC8ZPypVfzuzQYjTQO1ghx-z62MPlBwzKHMNPgKH_BgdRJUMp-6SR9NCpmSeAkNBxgReq0SVMEsaVRS85A
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 7UzZQWvr0n2uTDl3ccMfHMKCD3wiApwvEPcSBtu-rn6GjohCPpM1Me66LyA0-LHZ6mWEQMGqRUbAQXR6LPNddw

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
   ✅ Script saved: tJm5v23cC4udbQQBdjUsSFBdWMTK9Yr_Oxvn3ZfIp63i4XgWfxPnWIVLs2ppAHJJC9QrWP64MKzykroJ1jIDqw

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
   ✅ Script saved: l6DWV9i9orliBn-DUlxxVWFDmEwtmB4K_4MYGvqtGX6dwJWq0pL1IC70QAzaIyd4_lbQtAbAUYcUtcll3_Pp0Q

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: Zrfu8T7luwOoFfh0szrq...
   ✅ Deleted script: 7UzZQWvr0n2uTDl3ccMf...
   ✅ Deleted script: tJm5v23cC4udbQQBdjUs...
   ✅ Deleted script: l6DWV9i9orliBn-DUlxx...
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
  Output: Document ID = t5JbAGVKE1qDxRqVSzfdtwvuVh3z5h0D89uZ9hFEepcuzQjsPlirV_IU4oaT1c-qiFgpsoboyj4sZBEJRdWnpw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(t5JbAGVKE1qDxRqVSzfdtwvuVh3z5h0D89uZ9hFEepcuzQjsPlirV_IU4oaT1c-qiFgpsoboyj4sZBEJRdWnpw)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(t5JbAGVKE1qDxRqVSzfdtwvuVh3z5h0D89uZ9hFEepcuzQjsPlirV_IU4oaT1c-qiFgpsoboyj4sZBEJRdWnpw)
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
  Output: Document ID = V_YOU0uzOvqRQaOFdMptDFUU5XwgHCVJP7ZU5kwu1XfFBd8HbufzFnMENrA4QPExfUJzpAs_pGZn-p5HphuwWw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(V_YOU0uzOvqRQaOFdMptDFUU5XwgHCVJP7ZU5kwu1XfFBd8HbufzFnMENrA4QPExfUJzpAs_pGZn-p5HphuwWw)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(V_YOU0uzOvqRQaOFdMptDFUU5XwgHCVJP7ZU5kwu1XfFBd8HbufzFnMENrA4QPExfUJzpAs_pGZn-p5HphuwWw)
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
Inserted: map[id:NompErPKZ6uui8oceJ-lOFbBUnURf-wfGX-gfjkEHZnrE6OLQpJ0pcAdGudR-PZQqg5WF80tGXMX9TvOuPtWqQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-03-28T23:50:21Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:NompErPKZ6uui8oceJ-lOFbBUnURf-wfGX-gfjkEHZnrE6OLQpJ0pcAdGudR-PZQqg5WF80tGXMX9TvOuPtWqQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-03-28 23:50:21 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-03-28T23:50:21Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:NompErPKZ6uui8oceJ-lOFbBUnURf-wfGX-gfjkEHZnrE6OLQpJ0pcAdGudR-PZQqg5WF80tGXMX9TvOuPtWqQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-03-28T23:50:21Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:NompErPKZ6uui8oceJ-lOFbBUnURf-wfGX-gfjkEHZnrE6OLQpJ0pcAdGudR-PZQqg5WF80tGXMX9TvOuPtWqQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: ozMIUez9xiPR10l2CgDTUY7PzEZaxZdf4v_SAafWjjRThd1WDxK69p71NYSckWx5b_2P-fItlF_kkMt5zxsidw

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
Collection created with first record: 0YvXyHO3-nU-6NweUuYt_PJWBnAqEWNfQO5fPl-EQprOrGyec0OKwgw1BAtwc50VoA1go9prUm0RDHifr_L8LQ

=== List Collections ===
Total collections: 17
Sample collections: [chat_configurations__ek0_testing ws_ttl_test scripts__ek0_testing chat_messages__ek0_testing ttl_cache]

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
Created Alice: $1000 - ID: RrQUWvyCgUmdMhTeq37GbftFaO2G1ebVuxmKu9ycPp1Xw4QQ42gjqMPuZBGIyoGnoloJwq0E5lnVWYWk1RThEw
Created Bob: $500 - ID: K2rdpnGHIb7pPXyzxEUHuZVM09KYRq8Kw39AKtiSyJdzY9j9RaJtEzgoKzqs57MWd1lZOJhI5dfzyRMWgn8Qdg

=== Example 1: Begin Transaction ===
Transaction ID: ea84c120-b925-4a2b-8e5e-3348bfc7e13a

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 76c59434-3a7d-4d74-973e-3f77f9ba8fd7
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
  2. Score: 0.760
  3. Score: 0.752

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.704
  2. Score: 1.501
  3. Score: 0.307

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
  - Mouse ($0) by Alice Johnson
  - Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: MQemyezbSVzsrHUvfaC4uv8lsS_LNJ_09JN_6IbAQg5m8uJnsFPZFONXAAGpjQpS8LVYTy_DrE0e0FLzxSqSdA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: QPl46ZM0dSABUac67Jq6ydMQJ9_uzi8XiHivSfevtjkn8jb0mZuQlxXdCmSSjqxo9Gg_jon4uDQueAZmhJHeFg

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
✓ Inserted document with TTL: sAARkuX5nGMZ2vPF-12GxIIkLQZXkZd2TN53GhmjuBrGDNxB5inQvQCB5XffCd4wnlrJ3nr-4G7-m5jxgTYPBw

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
✓ Edge cache script created: nGAiKjnk1HeRJT53x7aJs6t5Dvc9h4J8GVOGc6Jlo1pt3qkh5SU4tKB6wCARJi-jSGh4ep6YPzHlamdRmfP3fg

Call 1: Cache lookup
Response time: 15ms
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

✅ Script saved: 8lPXKCnjXIrh9OMHTUUL6PyO_excQg9g9XYUFTZKQnP_n1PZoVSgTuld0I6tESXlG3FJX-VnWfBtGA_8wgjlxg
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
   ⏱️  Duration: 38.330083ms
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
   ⏱️  Duration: 38.235458ms
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
   🚀 Cache speedup: 1.0x faster!

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

✅ Script saved: eYe5mySQX5sMXoWt4Tc-dlejSxrvi_nYaGShDGW-0-Wy7_jtZ_1GkwtiV6a_GPeWYpP6UlqXly7e5zzIwwLLZQ
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

✅ Inserted order: R9PgaAc25wxP0XQ8emGEXNGq6Spz2SbKlkAkWLU2NfHRbBhV0sE7edDxrpNFqFbtTxnSSDKF7if02eiPTnr7Bw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: DyTI6X7q4EO3z4QjCxrKgeVgl7Nzy15zTc434L8TSf1MzD2EDBTuiQhTEClCkchG2gwI0aUeQWmmIYDCQsFxoQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: FVcr-ONhAAOMB5S31gSkT6grUDNnoAfFWbaFaYIzjv3FdcmI0JAr2HFblkbCWb8ioM7EToSbFC5CBA0y9uDA9g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: 7bEfnFSN0B9ifBF_EPKKohGHe-Qa3bXzQCn1VjWBpTlKz__91O-sd_KcS31xQHEloiaVsojFG9d5vqj9rXlufA
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
✓ Created SWR script: fetch_api_user_go (mm__tdkMHi0Pn21gGUnIIKg6_bb8c4mA5rOxZo3IMA2iBI-ELQSRGV2-P0PxPE1lTEQrE8owC1ARjZ9_jGJ2yQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-03-28T19:50:25-04:00",
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
Response time: 40ms (served from cache)
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
✓ Created native SWR script: github_user_native (vjS2V9qk16I2TphQtJvRrCwOY86n_NUOIk3xbPeZ8NeCvtNKYRvurPZn-NbnixbTogOWmO4Yf2BWbZyND17HiA)

First call (cache miss - will fetch from GitHub API):
  Response time: 13ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 19ms
  Speedup: 0.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (nB64azMHwA-1UbohWrDU7rl59N2dwEmJrDpHLakYM32rLOOGyQ9NgIwTiOcHzOfnh_tdGp37OZv8KKqEC32DGw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (DCtN37kJ_gzhPgjwPEE8eqk1Txj1Arq66RpL7RBvqEAw_Nacz8SnP71MZ85m5zrzD6HLzIe86IMOrWtmcQ_CTg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (n-637Woi-RrUVjFZN6HNh3BWaaFrfkXimMY2HiFtGjyaTyOlI73bCjy1TCtEvplD9V-RDKsmWOaUeIVA87FY3Q)
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
   2. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   3. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   4. map[type:String value:Database Design Principles] (map[type:String value:Database])
   5. map[type:String value:Natural Language Processing] (map[type:String value:AI])
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
✓ Created session: zseqPJi_FKBvWhcszt4lzHwF2aO_Zdq5_3yK9_zLocfnKymTh48UuUu-whYP4Hua5y6KoPXDmXBOkXpwWCjc7A

=== Sending Chat Message ===
Message ID: mFQ6cPgOEJYmMPPrIppGsWBA3rW-wsB_dtTb1FpBPNqPc_cXmn-OLsCX-BZh9p2UguT8DLeXsdPqozCd8O6YOw

=== AI Response ===
The following products are available along with their prices:

1. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

2. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:Z3yzzVWNahLlgMUI--TnQQ5QbPoMTQHLeSxX6m3JONf8cCMjxmiqw4xm0c19qoTqnhv-cbwFm_59Tn54K3E2Xw name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:P9GmE1-nIF3qROekTZh5-gDF7P-s-QUMKJxXtLvOVwSGPJ3f1kX8Hc-JrmJsv5-lS4t5w0W36ItqttHgJCFGOg name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:Orj-mX5tNiIXTDljxS-uetxUFOAGH9ALPgjUUMvhkPCI1umS3d9cp4JQD_sjf5uPAWaGZYdn5gu5C-qFsPNRBA name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2297ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 92
Total tokens: 1390

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: xGrUr2pl65mxw6Nlz3zydi3glZuIDBBH2vUVHPfnw7hMyTNBYgWuWG5DZGsNM5qnxloiD1h1vorNVw9X7880gA

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
  New response: The price of ekoDB is $99. If you have any other questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 1DKL_oeh4pxE-pQex_XVsXEX71hWlyhbI0SLwhPUot4_QRG0ga4H5XJJWkezKWNg1SrwpXKK3H8GhONB65JNaQ
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
✓ Created session: b2GLulGwg3Lo3R1RniwXyBoFHoTgBniyXUqPk9Br72wCTWPjZMc5Uj83W3mC-Kve6hL_pEaV-_07Ul9mdYKLMQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, please let me know!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: QHEC5fRR8nxyeMNdpSUUhXD98rgEHXoJyH4S9BhntOk9S5jKmI4rV6FNyN2RF-Ogw972oo1GIfQw6sNCPS-3lw
  Parent: b2GLulGwg3Lo3R1RniwXyBoFHoTgBniyXUqPk9Br72wCTWPjZMc5Uj83W3mC-Kve6hL_pEaV-_07Ul9mdYKLMQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: QHEC5fRR8nxyeMNdpSUUhXD98rgEHXoJyH4S9BhntOk9S5jKmI4rV6FNyN2RF-Ogw972oo1GIfQw6sNCPS-3lw (Untitled)
  Session 2: b2GLulGwg3Lo3R1RniwXyBoFHoTgBniyXUqPk9Br72wCTWPjZMc5Uj83W3mC-Kve6hL_pEaV-_07Ul9mdYKLMQ (Untitled)
  Session 3: 1DKL_oeh4pxE-pQex_XVsXEX71hWlyhbI0SLwhPUot4_QRG0ga4H5XJJWkezKWNg1SrwpXKK3H8GhONB65JNaQ (Untitled)
  Session 4: zseqPJi_FKBvWhcszt4lzHwF2aO_Zdq5_3yK9_zLocfnKymTh48UuUu-whYP4Hua5y6KoPXDmXBOkXpwWCjc7A (Untitled)
  Session 5: QXOdFl0w4yhb1lF1zrevRN2vRu-5DRX9LZ6plyVt-UCxEnXgZTHp7c87IvQFBE69ZvueD4h8qjQvtxzoUsiKJw (Untitled)
  Session 6: OUGtIBJszHl4Zgo0ENTpTwyDMiuwPxQJxonyTQx-Reb-PqdBsA46sl1sJvJjUxkaNkDjoy2bL1pg1hFCFOcazQ (Untitled)
  Session 7: pkmE2E5xs52sdcOVdKgIqWPVAlHqOnXmvl-MXprvOEhMLgAYzuH2XSnmx3V8T12WZMq9xqrfTcTglJQ1Zx4k5A (Untitled)
  Session 8: 9Jr3Qe-CxxNhxTsCRmrM9rNvW0r0x2fzl446Rb9ns4tzaWZvzDxApSQxUtkA8cJvKf_fROyiqNfWYwWM4ahQmw (Untitled)
  Session 9: pFPnqAtp08oCL4UUqkRYoHA97h5__QHkMVXMvvIXdGr734rhuDDvY0KBEmhOVYCwLeX-bA_SNR8bLQydscbghA (Untitled)
  Session 10: XYf4K4p27uG-Khw_d6gCgyk8l5FE5HJ_sBKkrVPC_B8yIv1SlstejAemAqE_4HfUcKJn6PifBgFYFG6Fc0qMxQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: QHEC5fRR8nxyeMNdpSUUhXD98rgEHXoJyH4S9BhntOk9S5jKmI4rV6FNyN2RF-Ogw972oo1GIfQw6sNCPS-3lw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:eTUxU-xKVJ4OH-iyyyyvluBBaiPNhLG3hqnx0EADcL_ruSfltYW5jeHjealhjeOZZqQp3qOjrNADHY_Jkr5d1Q]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:eTUxU-xKVJ4OH-iyyyyvluBBaiPNhLG3hqnx0EADcL_ruSfltYW5jeHjealhjeOZZqQp3qOjrNADHY_Jkr5d1Q name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:eTUxU-xKVJ4OH-iyyyyvluBBaiPNhLG3hqnx0EADcL_ruSfltYW5jeHjealhjeOZZqQp3qOjrNADHY_Jkr5d1Q name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:VmZR3Y24lK-0mvXpt9bv4CvBxzkCZWHPFqvif-wAdd7-RcFL8vuoymjwFdkqdu8MjA63FQULCt0nm4i1qug4Mg]
Inserted with bypass_ripple: map[id:vZEHpOc2cMB4BYRDZNxmj9emPibeGnRhXJC7Xwjrx2UxR0FoEQ4rVDiCD-44d-7cd7q0L2BxRdPQD-v-yS6kTQ]
Inserted with TTL and bypass_ripple: map[id:RRCa6ynx8KI7k9RSU1mgPWT9QoyxK3PQm9SzNsu18auCxp1fNRuEwjy4VdmVMCZsKe6bzaMhOwbvowkXY4SqPw]
Updated with bypass_ripple: map[id:VmZR3Y24lK-0mvXpt9bv4CvBxzkCZWHPFqvif-wAdd7-RcFL8vuoymjwFdkqdu8MjA63FQULCt0nm4i1qug4Mg name:map[type:String value:Product 1] price:map[type:Integer value:150]]
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
Created user function with ID: WQLhdjzEVeKd-uzmmzSRDpAd8vSsSeUj38jXj3G-AxMjBf49kysWJWpKYBf09NxOzQLc-sE4il6REmhZGrpr1g

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 19 user functions:
  - validate_user: Check if user exists
  - get_active_users: Get Active Users
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists

=== List User Functions by Tag ===
Found 19 user functions with 'users' tag:
  - validate_user
  - get_active_users
  - validate_user
  - cache_api_call_py
  - swr_user
  - swr_user
  - fetch_and_store_user
  - fetch_user
  - fetch_slim_user
  - fetch_slim_user
  - fetch_and_store_user
  - fetch_slim_user
  - fetch_and_store_user
  - swr_user
  - get_verified_user
  - fetch_user
  - fetch_user
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

> @ekodb/ekodb-client@0.15.1 prepare
> npm run build


> @ekodb/ekodb-client@0.15.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 975ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'R2tx8KnEDAuvRPVE6YY0pgnkHbIvbehHq8HK3M77k5Ub42csjCkOV8QcKlAy1hD-UQ1Xv5m6AXL4pMls-iYU-A'
}

=== Find by ID ===
Found: {
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  name: { type: 'String', value: 'Test Record' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: 'R2tx8KnEDAuvRPVE6YY0pgnkHbIvbehHq8HK3M77k5Ub42csjCkOV8QcKlAy1hD-UQ1Xv5m6AXL4pMls-iYU-A',
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 42 },
  active: { type: 'Boolean', value: true },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  price: { type: 'Float', value: 99.99 },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { value: '2026-03-28T23:51:12.276Z', type: 'DateTime' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-28T23:51:12.276Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test Record',
  data: 'aGVsbG8gd29ybGQ=',
  id: 'R2tx8KnEDAuvRPVE6YY0pgnkHbIvbehHq8HK3M77k5Ub42csjCkOV8QcKlAy1hD-UQ1Xv5m6AXL4pMls-iYU-A',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  active: true,
  categories: [ 'electronics', 'computers' ],
  metadata: { nested: { deep: true }, key: 'value' },
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-03-28T23:51:12.276Z'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { value: 100, type: 'Integer' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  name: { value: 'Updated Record', type: 'String' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-03-28T23:51:12.276Z' },
  id: 'R2tx8KnEDAuvRPVE6YY0pgnkHbIvbehHq8HK3M77k5Ub42csjCkOV8QcKlAy1hD-UQ1Xv5m6AXL4pMls-iYU-A',
  active: { value: true, type: 'Boolean' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: SiB9Q9UxVhC0nPIPXT5ogxRz73tc7X27f_XePQzAfO6aCw7NlAdsTyfhMbFPNbq2nQTGMixJDVn9lWnMPnLDxw

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
Collection created with first record: -MZs5LlQutodqs1LDG2eYv4wktTcRsC7bIj4sY28EN00xDtN13rOq85Gi35Rt7DERc8GelDxmLk8L9Loe099fA

=== List Collections ===
Total collections: 23
Sample collections: users,chat_configurations__ek0_testing,ws_ttl_test,scripts__ek0_testing,chat_messages__ek0_testing

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
Created Alice: $1000 - ID: l7F-FTdMVvF4xIm7GlAhUwPsOFM4lhCEtdicFl8IUtYQm1yPPBDSs9mpuPQC4sKG36IXVEwzHoWZW7z70UeleQ
Created Bob: $500 - ID: AdaVz3Tp1Mf_7Y-rzW5alTMG6nHyQo_F6xvxmRJteRhhgbI3JvrqWUlfbRNnusgp0cE-AIBUEiqJAymAMWqmyg

=== Example 1: Begin Transaction ===
Transaction ID: e21db25b-83de-4908-a507-51eba1777c18

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 8500d0a0-8a84-417b-8a5d-c91ca7fc7a40
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
  1. Score: 25.740, Matched: name, name.value, email, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio, bio.value
  2. Score: 26.400, Matched: bio, title.value, title, bio.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, bio, title.value
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: bio.value, title.value, bio, title
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.753, Matched: 
  2. Score: 0.748, Matched: 
  3. Score: 0.729, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.701, Matched: content, title, content.value, title.value
  2. Score: 1.492, Matched: title, content, content.value, title.value
  3. Score: 0.299, Matched: 

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
✓ Inserted document: X3yLdgO127MYRXqPMjmmTbvsWJZIDUmmaVOAhwWBsU51GRqWnbU-XEeY2NRu9e3TYz64XGDTRtSBFOgTt8wNYg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: pOPIVZFRi6j5IwTDAg6kQN0LsSQznjGBssIKFDJF7iOTUfK2l3XIfwduqfD1jmKqCyCgSkmhvjFNaL3epx2rvQ

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
✓ Inserted document with TTL: rddt6lYxBvbLxj38CL4TJcwisTOy_8dTBqkEQSU-pEYfj7FkmdAvh--RCE9f85P5vbRHqRBXy7rKVyVJHn1L5g

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
✓ Edge cache script created: PHnTTNCwd_BR6gkrL1CffoB0H6aBlaIr82b9sAi5QpSL3Hwi5GRXHF0cZ7C1bc0e07TYe0oSCA7LFwhwSA_IuQ

Call 1: Cache miss (fetches from API)
Response time: 165ms
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "address": {
          "street": "Kulas Light",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1,
        "website": "hildegard.org",
        "username": "Bret",
        "email": "Sincere@april.biz"
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
Response time: 41ms (4x faster!)
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "address": {
          "street": "Kulas Light",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1,
        "website": "hildegard.org",
        "username": "Bret",
        "email": "Sincere@april.biz"
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

✅ Script saved: jRaTyPZVgtSt6lL8xZ-rs7UKXmEjkizl7u6zDeljoJt0ngEF3v3w9Uxt4-0Vekbg6h3eHh-0XVxIh6397LzrwA
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: Sqc2XdQ-wtaYvir84BUeWWp_LrWPG5TSLdkowXNGI_6Uv2VvUil42Rrkj7emVN8mDuLejBlvU3Skz6lmJkaJPg
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: nle9KJmnlVFRH5lE6T-PtbxeMXbiJGkYACK5Cbr2dSVJ6B5vEfP1RIjW6Xyvfrvk6J2vguOpLv9TjrgnBxcmUA
📊 Statistics: 3 groups
   {"status":"active","count":5,"avg_score":60}
   {"status":"inactive","avg_score":50,"count":5}
   {"avg_score":60,"status":"null","count":5}

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
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
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
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: Nl8F_QJJMIbYKrxJ4NNvGXogUpJe7c-7zmv2FMSYHqe-IydRKfqwtmdgGOBeWMEnlF74e0z9p5JSpmeLSfsbrQ
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
   {"category":"Furniture","count":2}
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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: U8hnsM6pEwju8PdNknOjQ6VQxNScTPgzWOZ8F_tVJJbyWW5sUpWlMCPwRsN-5OKkN8x699i_T-xVdElEAzlOvQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 0ftuTBVv8U8M4IoCPH1U2WQ8bETPZcClpihKBHLqa4cyAwL4PEAM2rdzqUKQGGYihavMth8MRahQNIvQFXBh1Q
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: SFnBMELLghjxOrvSTlVmNCSeg64EFjtwBuFz_FOZSqc5Wz0XYfNsFx79sysLUFzii-i54p0XgBRvqfXpl-wY9A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: dGEE1_1HZQiJA2WwdVNJtZ0R7pG-W8hDXNFl_aQPZdV9a6EnvitajlrGu4R4DXU-8A-bX_s1YjKR0t1ogXDfyA
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
✓ Created SWR script: fetch_api_user (t0iA-B-ZzUAvfQMGKjwbkDHB3cbGAEPk8Q6RoZW1Bq6j7Js5sHfLeBx32xTv_qV5pnF-AGJ_CcF2akCu6HG1Aw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "street": "Kulas Light",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "username": "Bret",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
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
Response time: 51ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "street": "Kulas Light",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "username": "Bret",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
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
✓ Created enrichment script: fetch_product_with_reviews (kpgOxEY5pP_EVjaimq_8ez9G6OPGyGYr54jY6ffw6iQzTultRkX_K6xp8nqC82dOSrDpvc0msftdkZ7xecTdkA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "shippingInformation": "Ships in 3-5 business days",
        "id": 1,
        "sku": "BEA-ESS-ESS-001",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "warrantyInformation": "1 week warranty",
        "stock": 99,
        "category": "beauty",
        "brand": "Essence",
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z"
        },
        "discountPercentage": 10.48,
        "tags": [
          "beauty",
          "mascara"
        ],
        "reviews": [
          {
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerName": "Eleanor Collins",
            "comment": "Would not recommend!"
          },
          {
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon",
            "comment": "Very satisfied!",
            "rating": 4,
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "rating": 5,
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "minimumOrderQuantity": 48,
        "weight": 4,
        "dimensions": {
          "height": 13.08,
          "depth": 22.99,
          "width": 15.14
        },
        "title": "Essence Mascara Lash Princess",
        "availabilityStatus": "In Stock",
        "price": 9.99
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
✓ Created native SWR script: github_user_native (MJdSuqhyVjjk4g2igAT-yXG_HWLG0RdopwYPayzTW6mKKJnK7dVFPSbi2tdjHmc4kULRO7vfo23EJLV-ijW5Pw)

First call (cache miss - will fetch from GitHub API):
  Response time: 15ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (ZIqa-aMwN8BlbTk8nJNacnNlGBF953m9rwBAjXozMzld7GYwf7eXwQ98Yw_sgBnokM0lhI2OwIhMQPW4GUhfdw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (h_Se5jbOztLM-FzjB70GodUmiXNhhc2CV4GYAPwFB4cUaxqMrYfeaWmUJv9CkqSWP-rJYwNUzBiZVZtBgRYDDA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (0FyLeid9bfdNQHKqKgdItDd98HuWoypIUTO467UoHo1OTop4twz5bzDM3BLqnbIZaqhyPdXDyA4hiDdHRM6hOA)
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
   {"count":5,"category":"Electronics","avg_price":367}
   {"category":"Furniture","count":3,"avg_price":365.6666666666667}
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
   Vector databases offer several key benefits:

1. **Efficient Similarity Search**: They enable quick retrieval of similar items based on vector representations, making them ideal for applications like recommendation systems.

2. **High-dimensional Data Handling**: They excel in managing high-dimensional data, which is common in machine learning and AI applications.

3. **Scalability**: Vector databases can scale to handle large datasets while maintaining performance.

4. **Real-time Processing**: They support real-time data processing, allowing for immediate insights and responses.

5. **Integration with AI Models**: They easily integrate with machine learning models, facilitating seamless data input and output.

6. **Flexibility**: They accommodate various types of data (text, images, audio) that can be represented as vectors.

7. **Enhanced Performance**: Optimized for specific queries (like nearest neighbor search), they can significantly outperform traditional relational databases in certain tasks.

8. **Support for Sparse Data**: Capable of efficiently storing and retrieving sparse vectors, which are common in NLP and image processing. 

Overall, vector databases are a powerful choice for applications focused on semantic search, AI, and machine learning.
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
   {"role":"admin","avg_score":20,"count":3}
   {"avg_score":70,"role":"user","count":7}
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
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: tBdAM8UVRfWjR-8yVcnRq0e-bBpTeEqgdJI23IdFSQlVpuutnXMeIJ5seAKBOmFSevre9FF1vn7iSui3CY0-dA

=== Sending Chat Message ===
Message ID: fAm6UjvvomTjwdiIimJos2ekueh9A3AofmT6OsPAVqn5hjvRTEWoovopQIYZRtyY81sRFt1IcrwNvUI1ZVZqcQ

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
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'JRidv3TILk-qQTAiaR46zmxuEJHLObE1VnMwmnaeAPkSUZfU3o8QWPzg-qBJDi8app1t-3p2BcRfK_HfOF8JEw',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    id: 'imKBx35qdx41d_wKC7OKq3Bfj5HDOsauZqUhSEfinV1pHKC7c7Bs72ITyVY8RE7Q6LWu66PKLiLAYMtwPgioLg',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '2CknhcuBTYBGOymMd8rscRtgAiOOPJJbkMw7bAXwjfbRHZPPMStQDwm1Hh0ytD5TR6I0SLlinU7J7xxzybioIQ',
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2952ms

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
✓ Created session: wI5pmuwWgB4u6UPlkgZdkgWgbQ4z36eKIhzk_pBGerqjJPafISYplsGdgaCkffQbk_NmvdZ8fZK9lEsJUyb-Xg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more details or have other questions, feel free to ask!

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
✓ Created second session: eLCZB-7_ZJTmrc8Qd8pEq_5M9a2M0OB-ythxrdBQj4m2_zO_0SYQgy8YHaDGFqysaY2Gk1BZ9ZCUv8wpVAGnig
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
✓ Created session: 7DzhItH6Yj6JtcBJvapGuPhG11MJ_C-dyH_Rscif0fUE5ZQ5TahmcWWnAPwXWKeRZtzHDvouVu2s27xNRYKIWA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more products or further details, just let me know!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: qjU-3_7t-JnrOJkQVU4uVa2PeuPf3-GcnjdO_5Gl81QT8dYOhEzTWEcF2z1k_Ot6zTS4JmlTyfYiorop57ZAMg
  Parent: 7DzhItH6Yj6JtcBJvapGuPhG11MJ_C-dyH_Rscif0fUE5ZQ5TahmcWWnAPwXWKeRZtzHDvouVu2s27xNRYKIWA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: qjU-3_7t-JnrOJkQVU4uVa2PeuPf3-GcnjdO_5Gl81QT8dYOhEzTWEcF2z1k_Ot6zTS4JmlTyfYiorop57ZAMg (Untitled)
  Session 2: 7DzhItH6Yj6JtcBJvapGuPhG11MJ_C-dyH_Rscif0fUE5ZQ5TahmcWWnAPwXWKeRZtzHDvouVu2s27xNRYKIWA (Untitled)
  Session 3: eLCZB-7_ZJTmrc8Qd8pEq_5M9a2M0OB-ythxrdBQj4m2_zO_0SYQgy8YHaDGFqysaY2Gk1BZ9ZCUv8wpVAGnig (Untitled)
  Session 4: tBdAM8UVRfWjR-8yVcnRq0e-bBpTeEqgdJI23IdFSQlVpuutnXMeIJ5seAKBOmFSevre9FF1vn7iSui3CY0-dA (Untitled)
  Session 5: b2GLulGwg3Lo3R1RniwXyBoFHoTgBniyXUqPk9Br72wCTWPjZMc5Uj83W3mC-Kve6hL_pEaV-_07Ul9mdYKLMQ (Untitled)
  Session 6: 1DKL_oeh4pxE-pQex_XVsXEX71hWlyhbI0SLwhPUot4_QRG0ga4H5XJJWkezKWNg1SrwpXKK3H8GhONB65JNaQ (Untitled)
  Session 7: zseqPJi_FKBvWhcszt4lzHwF2aO_Zdq5_3yK9_zLocfnKymTh48UuUu-whYP4Hua5y6KoPXDmXBOkXpwWCjc7A (Untitled)
  Session 8: QXOdFl0w4yhb1lF1zrevRN2vRu-5DRX9LZ6plyVt-UCxEnXgZTHp7c87IvQFBE69ZvueD4h8qjQvtxzoUsiKJw (Untitled)
  Session 9: OUGtIBJszHl4Zgo0ENTpTwyDMiuwPxQJxonyTQx-Reb-PqdBsA46sl1sJvJjUxkaNkDjoy2bL1pg1hFCFOcazQ (Untitled)
  Session 10: pkmE2E5xs52sdcOVdKgIqWPVAlHqOnXmvl-MXprvOEhMLgAYzuH2XSnmx3V8T12WZMq9xqrfTcTglJQ1Zx4k5A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: qjU-3_7t-JnrOJkQVU4uVa2PeuPf3-GcnjdO_5Gl81QT8dYOhEzTWEcF2z1k_Ot6zTS4JmlTyfYiorop57ZAMg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'g1Idw5Z_BZM2DshapVMCEXI8Tv-fQgp-YWaeZFoKq_crbn3_er-7Wh-lY0kqwgR-Yz-GQV_WUJkgkVyG2SPa3w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'g1Idw5Z_BZM2DshapVMCEXI8Tv-fQgp-YWaeZFoKq_crbn3_er-7Wh-lY0kqwgR-Yz-GQV_WUJkgkVyG2SPa3w',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 }
}
✓ Second upsert (insert): {
  id: '6soWBe1RA_L7Gs8jdQbKyZsnOAKOu8jvp3klvQa78lBLhr-mulyD0tXM0GLW1pLLls-OI6e7zon5nmuZreccDQ'
}

=== Find One Operation ===
✓ Found user by email: {
  id: 'g1Idw5Z_BZM2DshapVMCEXI8Tv-fQgp-YWaeZFoKq_crbn3_er-7Wh-lY0kqwgR-Yz-GQV_WUJkgkVyG2SPa3w',
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
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
   Inserted with ripple: {"id":"G-pefKkHzD3UQ1bBihGOer-k9zipeuJYFb6araF8vU9nZ5PTdSbHDTghIs_dO-Otoak11Udi66WdhcraZAhkzg"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"lA2Qa-eWZNsdLI4fyYza-7kjECnbsMJJAqYHIeei6LbcWVFC_5VdDmSuMO_-wUZFDznGNUCUeDaQO8v2uUymVw"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"id":"G-pefKkHzD3UQ1bBihGOer-k9zipeuJYFb6araF8vU9nZ5PTdSbHDTghIs_dO-Otoak11Udi66WdhcraZAhkzg","price":{"type":"Integer","value":150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"type":"String","value":"Upsert Product"},"id":"custom-id","price":{"value":500,"type":"Integer"}}

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
  Fields returned: ["email","created_at","id","name","avatar_url","user_role","bio","age","status"]

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
    - Fields: ["status","user_role","api_key","id","avatar_url","secret_token","bio","email","created_at","age","password","name"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","email","id"]
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
Created user function with ID: MoSBPJ-wTI4GNuKAvVK0s6SqebS_WOFpc7vnD4bkQ-yfsD7VJNs6cwoGcx-2BPIli66m5WLNqW42f97QLU-GLA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - fetch_api_user: Fetch User with Cache
  - get_active_users_updated: Get Active Users (Updated)
  - cache_api_call_py: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - get_active_users_ts: Get Active Users
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)

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

added 1 package, removed 1 package, and audited 9 packages in 983ms

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
  id: 'tHGA0pH2jWeeBUJ7gHtAmEKORB3X0YhT4NsGb2OUIjmNqzqnMAf1NlPCbmMuENq8B1dTjn8W8BAekN_eMsnYxw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'tHGA0pH2jWeeBUJ7gHtAmEKORB3X0YhT4NsGb2OUIjmNqzqnMAf1NlPCbmMuENq8B1dTjn8W8BAekN_eMsnYxw',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    id: 'tHGA0pH2jWeeBUJ7gHtAmEKORB3X0YhT4NsGb2OUIjmNqzqnMAf1NlPCbmMuENq8B1dTjn8W8BAekN_eMsnYxw',
    name: { value: 'Test Record', type: 'String' },
    value: { value: 42, type: 'Integer' },
    active: { type: 'Boolean', value: true }
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  id: 'tHGA0pH2jWeeBUJ7gHtAmEKORB3X0YhT4NsGb2OUIjmNqzqnMAf1NlPCbmMuENq8B1dTjn8W8BAekN_eMsnYxw'
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
✓ Inserted test record: nelTg6jXoWDgdaNxfywOhxSl74geEBzCjmsbTQ_9sINcutq4kwCjQ0kpvqeFKIVgCdP4uxlTe0flG9xEQXAzNw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774741938999",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "nelTg6jXoWDgdaNxfywOhxSl74geEBzCjmsbTQ_9sINcutq4kwCjQ0kpvqeFKIVgCdP4uxlTe0flG9xEQXAzNw",
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
        "id": "_S5yYpl3xBCh_YyPvDcmKVdwN76GYR1Oecn1I3cEbjoR2IwpNSv7WkqMKf8ONNyBKeTkGJG7Gfwp6qJ1Ag1tbg",
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
        "id": "mmpnpqj8wl8y2nCQsIu6Gr0V-dAmd7j7kPjU6Xqi3-P9by1YQW8f19sL-wubi6PxdiWatf2aQJbUal9HOFNuXQ",
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
        "id": "uECQwVBLmwZzmL6GZrW53lL84vBuwZLnKCf76m2k1zWzCWK2lhqx5kfniy4ncQB0Ggm7bX1kOBzBkTnjpJKOHg",
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

✅ Script saved: SMCx65J8denZLsUsG79eqnbbnMxMIKV6LpIPzFLpDsj1xNoUSvqsfbKHx9weVQoFo2HWSsRsIz0cthwkRkfxbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 5PTJHAqB51J97t21tVcx9IPxeYcDryyjg_Zd6iG0zeABU_MG5NV1nHjZ0ttTFh4-uF6DNaFkBPEO1Zk6sDE22Q
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: jCNVB7fDUeAAsBtQU6_IHWhXkpEJ4RwSwGzkvQskvXneLGm9iSpe9M8iPXb5NrgXF4_QjxgunN8E87WGLpBfSA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":50,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

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
Collection created with first record: wMVexxkcuPpseFXX_ISH1_TzF6RrUEwZIRGvz-9yZ5y6cJ3rqzlCOmLI28WQr4UXt0rSZU9jkeyQa1TJspoKkQ

=== List Collections ===
Total collections: 26
Sample collections: [
  'schema_documents_client_ts',
  'schema_products_client_ts',
  'users',
  'chat_configurations__ek0_testing',
  'demo_collection'
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
Created Alice: $1000 - ID: voigQzoEL6rQ4JvTXP2WOmSRuelPHjtwcfDWEflow59a0FkbmMTpWxuv9cblWPLrgov8rXpsPvuL_2bbeJWcMQ
Created Bob: $500 - ID: 47LDNbYK31fYCywpPmZ4EWtp4SEqZZ22WONM3niaKXtcWMu_rhFJ87w-535TXhTc4dwYxrohE0ueFgPSxUwEow

=== Example 1: Begin Transaction ===
Transaction ID: 24935fa6-25a7-4a21-a25a-4d728c6ca42c

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
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: 32f9ba28-eef3-4ef7-b7cb-c398bab21be6
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
   ✅ Script saved: 2MwQNFkKl6aceqJfKHvWZl3nccGOi5ocx1h3kLECglEvPNSluIjxqaC-CNIzKnSusLUt2Hzn3VDCZ9Pi3HuGIA

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
   ✅ Script saved: xG8r8MybiYdt2wWRGkQuW9g9XBAYgoVpqZfK6GcuPBPIAO7xoUrgqA51MYbn25EImzE9Qpo47yHmD3iEBmLApw

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
   ✅ Script saved: APaKpiGen96ZOE7eCut2ZY0KtE4fuKQbYweeweZCAeLV3vdEXuhSzce78KplUA9y1KP-yzVkFkqQD1OVNAGIFA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: yeSceQanrSvBUGX3NPl3UHlK_3tqcgwQv7nfYUx_ObbyW_kGn1jtR2PhPq3Ox0sa6LRzJ6YUAPtwX_FdXNR9Pg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 2MwQNFkKl6aceqJfKHvW...
   ✅ Deleted script: xG8r8MybiYdt2wWRGkQu...
   ✅ Deleted script: APaKpiGen96ZOE7eCut2...
   ✅ Deleted script: yeSceQanrSvBUGX3NPl3...
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
  Output: Document ID = o7fNYtg10atH9YbUUqKb1T5F6KPdVTIRrm62UstLMPCFaS2q450oqQs7jC44hLHikcy8lzLjx1PuHC6zgBv7WA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(o7fNYtg10atH9YbUUqKb1T5F6KPdVTIRrm62UstLMPCFaS2q450oqQs7jC44hLHikcy8lzLjx1PuHC6zgBv7WA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(o7fNYtg10atH9YbUUqKb1T5F6KPdVTIRrm62UstLMPCFaS2q450oqQs7jC44hLHikcy8lzLjx1PuHC6zgBv7WA)
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
  Output: Document ID = mtQfVin15v1A0Yg21i2Ntyrtgn4J-VSrNRZEtyXOQvLXgLRGw0Z63danGQFKoOEiaVAU05xWfH1gRaVvPk7D7w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(mtQfVin15v1A0Yg21i2Ntyrtgn4J-VSrNRZEtyXOQvLXgLRGw0Z63danGQFKoOEiaVAU05xWfH1gRaVvPk7D7w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(mtQfVin15v1A0Yg21i2Ntyrtgn4J-VSrNRZEtyXOQvLXgLRGw0Z63danGQFKoOEiaVAU05xWfH1gRaVvPk7D7w)
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

> @ekodb/ekodb-client@0.15.1 prepare
> npm run build


> @ekodb/ekodb-client@0.15.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.1 build
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
  id: 'QWM_7kSU1vRf-8zK8rkj9U5OBW2tYtZAmtbzv3Z2Ruc4dV_5ldtDvw2J4ur8XKzFy4ipUCCo0HE3zc0SdGOnkQ'
}

=== Find by ID ===
Found: {
  id: 'QWM_7kSU1vRf-8zK8rkj9U5OBW2tYtZAmtbzv3Z2Ruc4dV_5ldtDvw2J4ur8XKzFy4ipUCCo0HE3zc0SdGOnkQ',
  active: { type: 'Boolean', value: true },
  name: { value: 'Test Record', type: 'String' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'QWM_7kSU1vRf-8zK8rkj9U5OBW2tYtZAmtbzv3Z2Ruc4dV_5ldtDvw2J4ur8XKzFy4ipUCCo0HE3zc0SdGOnkQ',
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: iolMaYKywrOq70yczt_SkVsmuYpGSuO60YJ3huK20Dx1XDdlrPKNOqp_CinLajRgv0-iBhYnOO7IxBDYVIi3Vw

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
Collection created with first record: sBcCQkbXktH40B0yY9EhaymlEqFKmakaKJu4sMMGtbJNnwzEiUog7dntyQWJuhk4qOCCssrYDGtN1VA-6FzoOQ

=== List Collections ===
Total collections: 25
Sample collections: schema_documents_client_ts,schema_products_client_ts,chat_configurations__ek0_testing,ws_ttl_test,scripts__ek0_testing

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
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CkgvqA6qoXNdwcAgerD7Q6KhpfETG0nGT4sUq-XQYKjWKUP4N762yE-XW1CoyB9ajju312gT_0IdCTeUEU13iw
Created Bob: $500 - ID: 45qBkUHq3wII_NDLDEzY2I76cpZqsCl2ULJGJ43isFs2AgdPQnooHoSUf0eA5WLdEDsXr-1GlY3Br5_Fo-M73g

=== Example 1: Begin Transaction ===
Transaction ID: 6f706766-c472-4f8d-bc9b-427db8005190

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 5fcc1905-db75-4b0e-8331-aaa7a52a8295
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: bio.value, title, title.value, bio
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: title, bio, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: title.value, title, bio, bio.value
  4. Score: 39.600, Matched: bio, title.value, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.756, Matched: 
  2. Score: 0.735, Matched: 
  3. Score: 0.729, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: content.value, title, title.value, content
  2. Score: 1.492, Matched: content.value, title, content, title.value
  3. Score: 0.294, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, bio.value, skills, skills.value

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
✓ Inserted document: bo-DyL4YjNYrFUy_fe2LjVPRXFXdnS4sNoMQjc76uaRHMXRaFE-gmqofiiahgttwj_W2DZh1M5qOKcOp9YgNCw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: YQgRI-HLCwLM4s_ZJPDTBzQghVRghb8OaZZLdS6prAdk1QMDmrduejgFIid0VDvWRsz4jsED5-b7GwkxCo_j4w

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
✓ Inserted document with TTL: Cjg0pvWVSUBLNtRDmH43xfnBYmMIMyYDRgL3267UHXfTHBZmtS4BxMK13LHem4VmsDdv_MUqYwdX30BsPCyr0g

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
✓ Edge cache script created: muGJidL5_OKi45rktu8FVDNNvMS1DFdyqXEfn8KaRTRrjYZ9Bi09165MB2EF9msCe4YY9_HCyGMGi4_drIwOUw

Call 1: Cache miss (fetches from API)
Response time: 42ms
Result: {
  "records": [
    {
      "value": {
        "latitude": 40.710335,
        "longitude": -73.99308,
        "current_units": {
          "time": "iso8601",
          "temperature_2m": "°C",
          "interval": "seconds"
        },
        "timezone": "GMT",
        "elevation": 32,
        "generationtime_ms": 0.016570091247558594,
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "current": {
          "temperature_2m": 3.5,
          "time": "2026-03-28T23:45",
          "interval": 900
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
Response time: 42ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "latitude": 40.710335,
        "longitude": -73.99308,
        "current_units": {
          "time": "iso8601",
          "temperature_2m": "°C",
          "interval": "seconds"
        },
        "timezone": "GMT",
        "elevation": 32,
        "generationtime_ms": 0.016570091247558594,
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "current": {
          "temperature_2m": 3.5,
          "time": "2026-03-28T23:45",
          "interval": 900
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

✅ Script saved: MZzn90eoYDe5uCuWasyeOkVvRfrb6ahFQ63u_ld-zoIwOb6TcuejXrlKyjV_StgplMl1M2hFNmX_9Elay7rwoA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","count":5,"avg_score":60}
   {"avg_score":50,"count":5,"status":"inactive"}
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
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
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
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: e8urAQY3iaD_-6krp8V-hu1uPLVCs2aXh1htmYIprBX-8ptkFhW061pumMYdyVMiV5NeKdVKoQZyXN9YopTMNw
📊 Found 2 product groups
   {"avg_price":474,"category":"Furniture","count":2}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: cTOJ4sQdcHqesiVXrZv7KlkBr3-ssE0aehY5UpTFJpuc1uiAnkQPb_pocvDMzQmhrYGbJ7bE4-3iaggfd86OWA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: hzcLImcwKw7dbM1Geh085hlAMgPL2MWqF4Z5xYbaAOnuwNdsSj0O-PeF1ZpndzPMY02aC2Wez_Ba2c1f3sX6Fw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: rfEDmMOskWNH6_MNFZqupguS1LerlTg8RCAtJNKsOTVNH4YIFmLBiiXoyjP0f44He3IpruV74AYkoHyg7C9qlw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 7uRRL8uf_AF_VQ0ATIQl0mgdOgXsg-A8Q7KnZb6TNWdSP2ygf2WIblSiZ2VJ1VFs-nX18DELU8GYYTrAc5nBrA
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 10):
   1. User 2 - Score: 20
   2. User 10 - Score: 100
   3. User 1 - Score: 10
   4. User 7 - Score: 70
   5. User 9 - Score: 90
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
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
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
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Natural Language Processing
   4. Getting Started with ekoDB
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
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
   1. USB-C Cable - $19 (⭐4.3)
   2. Laptop Pro - $1299 (⭐4.8)
   3. Standing Desk - $599 (⭐4.7)
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
   1. High Precision: Vector databases store data through points, lines, and polygons which allows accurate representation of geographical features.

2. Scale Independence: They can be zoomed at any level without losing the quality of data, which is not the case for raster databases.

3. Topology Support: The data in vector databases preserves spatial relationships, such as connectivity and adjacency, for geographic features.

4. Space Efficient: They store only the coordinates of the features hence occupy less storage space compared to raster databases for the same geographical coverage.

5. Rich Geographic Analysis: Vector data is well suited for complex spatial analysis and computations, such as shortest-path or network analysis.

6. Attribute Association: It is easier to associate attributes with vector data allowing better spatial queries and data analysis.
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
✓ Created SWR script: fetch_api_user_js (McEensJ2VpLKqAOgQigRjALyDQovTeVcIMk5PyM6wKjUgVW3Hl4ArwDoLTjPXFUszhJBSVtlLbsxbwerDBacHA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "street": "Kulas Light",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "username": "Bret",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
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
Response time: 41ms (served from cache)
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
✓ Created native SWR script: github_user_native (71BzLnMGDCi8PvNkrV4s2ZAj5LUjawmA_wwSclA8RJLuEMuORle5d90eE79dX9oS2I9GUenzz_sv0nS1nWd3rQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (DaL-MlfjrumJ_22l0_u2mzavHO1AEAsuF3nokUfnKe9pgTKQZV2wQrvYg6aopgB_0Lc7Nw9l67UpAqcCvZIxXQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (JXIQe8FolRtV6ySLUTVhNkk30X6U7gFvi4defGZQvXVNkyvrEf7oI_JTiunAL7_rmUUGufsxv6HNGY3stasYMQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (naWmVDRr11C4tjxtQmHd-8op943RKkvvgnDjxtaUVTDlb6egrjvEWkWdUbyFEG3VDN3HU6EFtorl--zFugMpmg)
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
✓ Created session: 1vdGO88s69jfwoMV9EVC6epMu5_AM_cEYfxY96zVmExzRO0-t5CvAnd-qmF_jZLGSC4BPTNB-vnAZbzFzh2vbg

=== Sending Chat Message ===
Message ID: NWR_gOj4Z3AojMghVmFUsnN2gZ-MNqJ8GTRWmvHeNrg-jGKu0zVtSuTVd2xK5qXHRerMgH2Iz5oWAXCMyVfFbw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'KQcRIwAec8Pp0qbZ2AW25Qcr9PBruWFw3jRz-yJuev2X6iMRd39dEt3GKYt7GPKp1hKTqOA5xtzuh4TOxT-H9g',
    description: 'Fully managed cloud database service product',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'SbauXn4R_CGyXDIQ82SHjbxHvg00e5T9ZiLIy_VIIxB8Gu7Gh1YEA-4Za_r19POVWsIGTl9ZDlFmPzM-1U53nQ',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'bfflr7cgU9S37HxXOPvpfMKpImc0Sq7JUrwOWB6qJoc7LFtcHEkFgo4PS-zG2wwm4_ngSeoY8NF4dSMrGm0qNw',
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2447ms

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
✓ Created session: M5CIJEsiTBu9Wq68305hMbYvwT6VzOeWEMVe3bPPDrzKX65scQR-Xyl3vPjYKesLT_hRCWCp3UGKerB2Sd7K3w

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or want to explore other products, let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: RIlDdF6Ajxurie3mTW0MCvjPqSV899FsfZTHKvbM-vvSlsVy0jxzaZD-1MR9716KB6SjXZHJ0HbcZgnvCd0KiA
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
✓ Created session: smzWzGqW6CfHOIK0YAga94tYAUrFq29lKFdcNxi7xnw7EOe-j3GckuddzzV_xTBzvUrRzkVolHjZ7hVniBY9_g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need information on more products or have further questions, let me know!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: UGhI6kT5jFNiaey-yCo_K-yA715YeqHquSJ5QYqzRuIoELF2ybHg1NS2Ysj7b963CKV-oNUsxaLqNIBd7uXc9w
  Parent: smzWzGqW6CfHOIK0YAga94tYAUrFq29lKFdcNxi7xnw7EOe-j3GckuddzzV_xTBzvUrRzkVolHjZ7hVniBY9_g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: UGhI6kT5jFNiaey-yCo_K-yA715YeqHquSJ5QYqzRuIoELF2ybHg1NS2Ysj7b963CKV-oNUsxaLqNIBd7uXc9w (Untitled)
  Session 2: smzWzGqW6CfHOIK0YAga94tYAUrFq29lKFdcNxi7xnw7EOe-j3GckuddzzV_xTBzvUrRzkVolHjZ7hVniBY9_g (Untitled)
  Session 3: RIlDdF6Ajxurie3mTW0MCvjPqSV899FsfZTHKvbM-vvSlsVy0jxzaZD-1MR9716KB6SjXZHJ0HbcZgnvCd0KiA (Untitled)
  Session 4: 1vdGO88s69jfwoMV9EVC6epMu5_AM_cEYfxY96zVmExzRO0-t5CvAnd-qmF_jZLGSC4BPTNB-vnAZbzFzh2vbg (Untitled)
  Session 5: 7DzhItH6Yj6JtcBJvapGuPhG11MJ_C-dyH_Rscif0fUE5ZQ5TahmcWWnAPwXWKeRZtzHDvouVu2s27xNRYKIWA (Untitled)
  Session 6: eLCZB-7_ZJTmrc8Qd8pEq_5M9a2M0OB-ythxrdBQj4m2_zO_0SYQgy8YHaDGFqysaY2Gk1BZ9ZCUv8wpVAGnig (Untitled)
  Session 7: tBdAM8UVRfWjR-8yVcnRq0e-bBpTeEqgdJI23IdFSQlVpuutnXMeIJ5seAKBOmFSevre9FF1vn7iSui3CY0-dA (Untitled)
  Session 8: b2GLulGwg3Lo3R1RniwXyBoFHoTgBniyXUqPk9Br72wCTWPjZMc5Uj83W3mC-Kve6hL_pEaV-_07Ul9mdYKLMQ (Untitled)
  Session 9: 1DKL_oeh4pxE-pQex_XVsXEX71hWlyhbI0SLwhPUot4_QRG0ga4H5XJJWkezKWNg1SrwpXKK3H8GhONB65JNaQ (Untitled)
  Session 10: zseqPJi_FKBvWhcszt4lzHwF2aO_Zdq5_3yK9_zLocfnKymTh48UuUu-whYP4Hua5y6KoPXDmXBOkXpwWCjc7A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: UGhI6kT5jFNiaey-yCo_K-yA715YeqHquSJ5QYqzRuIoELF2ybHg1NS2Ysj7b963CKV-oNUsxaLqNIBd7uXc9w

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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=7iQfFvHfW59b1vsjF72mjiAy3fvkPrVuHqxs5NlksLBM-bLdg1_PvYl25isciIyLtfVxQltYNNrB9zMol6_EaQ)})
  User ID: 7iQfFvHfW59b1vsjF72mjiAy3fvkPrVuHqxs5NlksLBM-bLdg1_PvYl25isciIyLtfVxQltYNNrB9zMol6_EaQ

=== Read ===
✓ Found user by ID: EkoRecord(fields={embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), id=StringValue(value=7iQfFvHfW59b1vsjF72mjiAy3fvkPrVuHqxs5NlksLBM-bLdg1_PvYl25isciIyLtfVxQltYNNrB9zMol6_EaQ), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Sat Mar 28 19:53:55 EDT 2026), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Mar 28 19:53:55 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: embedding, active, age, email, tags, categories, id, user_id, price, data, created_at, name, metadata

=== Update ===
✓ Updated user: EkoRecord(fields={email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), id=StringValue(value=7iQfFvHfW59b1vsjF72mjiAy3fvkPrVuHqxs5NlksLBM-bLdg1_PvYl25isciIyLtfVxQltYNNrB9zMol6_EaQ), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Mar 28 19:53:55 EDT 2026)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), id=StringValue(value=7iQfFvHfW59b1vsjF72mjiAy3fvkPrVuHqxs5NlksLBM-bLdg1_PvYl25isciIyLtfVxQltYNNrB9zMol6_EaQ), created_at=ObjectValue(value={value=StringValue(value=Sat Mar 28 19:53:55 EDT 2026), type=StringValue(value=String)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)})})

=== Delete ===
✓ Deleted user with ID: 7iQfFvHfW59b1vsjF72mjiAy3fvkPrVuHqxs5NlksLBM-bLdg1_PvYl25isciIyLtfVxQltYNNrB9zMol6_EaQ

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
  {"data":[{"id":"R4tSOwiYQgmxB5SPr9-1PRNgk_L66PrsSOUuBvDznlShhEZPzKUPRdV1XCxrwCfig_cDKfrps4gaN9qEO1BtWA","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]}

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
  IDs: jXrpZhDuG8zBYFKet-m2ue5axA7_gvajS1yzEwtNa0ge3X6bZ9Zb5xZ6ihD29B-j-QPpvEPkcvUHYaVUWXHPNg, eV_TbFsFtMtvqIItK13FLVphIh3drCjaAR6WRQlQbnqVGv4aKE8IYmYxHFLXPhUlznBG213z14msleiVxASEwQ, veGADC-qROeaDqR6fnfcnKJyOE7IVAeZJOgmuA5FWRKfnmQg4sfd3ShnjBMoNQyL3a6mGTBP2C8LGpSKG46O_A...

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
✓ Found 29 collections
  - schema_documents_client_ts
  - schema_products_client_ts
  - users
  - schema_users_client_js
  - chat_configurations__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-03-28T23:54:11.504542Z","last_modified":"2026-03-28T23:54:11.504542Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"role":"admin","email":"alice@example.com","name":"Alice"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1774742055680}
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
Created Alice: $1000 - ID: CdSKv9UtsSOKqpdK_bbY_A0FKqRlPRatDDWQnMhnSg4CFAK67pBQQAK6Nx7lND759vX6pzFjr1PyCtGJiIALIQ
Created Bob: $500 - ID: BfmyreJE5uWDbq_WPVSs1qPLtxkcDLjzBZKWixjXAQ6LdkGC32gTTrtmcGaR6crGC3uekkzL-24oAYSD6ep3AQ

=== Example 1: Begin Transaction ===
Transaction ID: 835857c7-631a-460c-99f4-13f58518b442

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 120a6edf-7eb7-4074-b94e-e5ddfe081ef0
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
  {"results":[{"record":{"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"id":"SR07co5YN7Gvs7Cbzrt3umjyokIIh_BbujJz0NwAzHLct8tcJh-jsBs0ih2Rto7QJGgcCFLGbanSk5umyujP5Q","title":{"type":"String","value":"Rust Programming"},"tags":{"type":"String","value":"programming,rust,tutorial"},"views":{"type":"Integer","value":35}},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"id":"A2NnfQsM0UDkqxKNK_e3ywyOQCdEyLF2FfKSRjm34P2-k8Dl-t-_W48az74U51tHQf96iLOZjgg1i0QV6H0nIA","tags":{"type":"String","value":"programming,python,data-science"},"views":{"type":"Integer","value":393},"title":{"type":"String","value":"Python for Data Science"},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."}},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":{"value":"Build modern web applications using JavaScript, React, and Node.js.","type":"String"},"id":"1XxSXjiRKZZjUnQMgbh9t_tOiX9X4yzS-MZF-4X3iRHEGEA25Ra1B6HfoTMOQf2lr7ntNsb8vSwqTwkKLNBCLg","views":{"type":"Integer","value":921},"title":{"value":"JavaScript Web Development","type":"String"},"tags":{"type":"String","value":"programming,javascript,web"}},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":{"type":"String","value":"ai,machine-learning,python"},"views":{"value":16,"type":"Integer"},"title":{"value":"Machine Learning Basics","type":"String"},"description":{"type":"String","value":"Introduction to machine learning algorithms and neural networks."},"id":"nhf9U1qem58Sr4K-XtnNdOHcWGUZHpb962BSUg8tbtq0jKK7_DFqePRuzqOeZ_1cgGJIAoLur-JrqBw49njkMw"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"views":{"type":"Integer","value":393},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"id":"A2NnfQsM0UDkqxKNK_e3ywyOQCdEyLF2FfKSRjm34P2-k8Dl-t-_W48az74U51tHQf96iLOZjgg1i0QV6H0nIA","tags":{"type":"String","value":"programming,python,data-science"},"title":{"type":"String","value":"Python for Data Science"}},"score":1.0,"matched_fields":["description"]},{"record":{"title":{"type":"String","value":"Rust Programming"},"id":"SR07co5YN7Gvs7Cbzrt3umjyokIIh_BbujJz0NwAzHLct8tcJh-jsBs0ih2Rto7QJGgcCFLGbanSk5umyujP5Q","description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"tags":{"type":"String","value":"programming,rust,tutorial"},"views":{"type":"Integer","value":35}},"score":0.5,"matched_fields":["description"]},{"record":{"title":{"type":"String","value":"Database Design"},"views":{"type":"Integer","value":241},"description":{"type":"String","value":"Learn database design principles, normalization, and query optimization."},"id":"3VGim91O5sJtF9Is4Ji_UWq_ztBSyTVTtHhdxBc77ZayQD8LnvI2darSIH91EEDyVk77CDQ9cXLFK9mgTAlq2A","tags":{"value":"database,design,sql","type":"String"}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":0}

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
✓ Inserted user 1: StringValue(value=vZjXVLaf3H3pdXqk9xJAEABoIrKCL9pTg-jPG1J8pjXzI2lMpmSkkevCLIq7fds_OBF1-SgC6EoSsKR60mniEw)
✓ Inserted user 2: StringValue(value=_5f7qA2zojI6mIj7cX4p25qCCTLZEliQsHY559m6FJuC0Pi_jcZ_-a2UB1oznOABMFJ-WbAytMHHOvQORzy66w)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 30
  Sample: [schema_documents_client_ts, schema_products_client_ts, users, schema_users_client_js, chat_configurations__ek0_testing]

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
  Document ID: V5MJ3L4X7Elg1_Vk01YpObj3qrstsX4XgakB1-zX9vto_4-fV11q1xa5Ayn6NAN5hRxAzMSTlH8NAu8MilXxYA

=== Verify Document Exists ===
✓ Document found: id, user_id, created_at, session_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: TghGh6fPl7Ie8uXfM0gTMycamH58yaPiSwN6Q-4nkPPQpCSkaROejzEPBuvQCOvS4JMEkfJW1zhHlncSrb5_7w

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
✓ Inserted document with TTL: StringValue(value=FPaYJpRk6DVQTqU4HoteNeU3xCUSDwrph44v0CWWkEaWhqCxM8pTfbh46DyC3k8FTYL74GaINelndIP6UbpVGg)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1774742088793},"id":"FPaYJpRk6DVQTqU4HoteNeU3xCUSDwrph44v0CWWkEaWhqCxM8pTfbh46DyC3k8FTYL74GaINelndIP6UbpVGg","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-03-29T00:54:48.898488Z","value":{"type":"Integer","value":42}}]}

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
✓ Edge cache script created: ia5ei5VF-T_lo571f6qkrPlG18eCUMMgCGPIgjWVJUT24SA70gAwrEpHc5nSqenkw3h1NEFrwiI-erVdb8r55g

Call 1: Cache lookup
Found 1 cached entries
Response time: 31ms

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

✅ Script saved: 3FoYF8W1juPhZ3flP-2AAnjaZJ6ZOMZu6KWfQPOe_4CrpbrhuOLqMbp33gENDLxBM5xcjvq46heh3udn8BWSqw
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"status":"null","count":5,"avg_score":60.0}
   {"avg_score":50.0,"count":5,"status":"inactive"}
   {"status":"active","count":5,"avg_score":60.0}
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

✅ Script saved: WXLVuf8L77TORo3ESjFpTZ3z1EcY_lazt84pv7UfltY3IHhjH3QjlY6nrQHCNJ_vVaoGaJsZGHnMjNGwL69ptA
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

✅ Inserted order: StringValue(value=Dzr-Qei9FaktGP8vStp78Ei9hanPEMNQuti9peKGY_hEQVhjyBZNy78zLtfyn6q1kB5-fHUHc_GSSMDknooDqQ)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: bDvsVNzjcpR9eUzoHD6d-u-W3mhw1o1jpmBSyUlY1GVQzyHOp66CSI8Q1WxkU3iSPQo0fZJrlTHCe_SY81hMEQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: YP1qBkj_BwktOABWksUPHuWbZeD7DPAVR0a-6U52w3nJjps06A0ym-cOqA9xdbUWwNTdlJSB2SWkYUuhX_OU0A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: cI5S25KlICjUdG3d8xpQtArNF5WGIy2aikgQmUrAIvUx0B6V-ZVwzAErq5JsMnvgbhFqmaUlTTtUYtNifEcLrQ
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

Step 2: Create SWR cache lookup script
✓ Created SWR script: swr_cache_lookup_kt (q8aHKj8GLiV076FQf29P-rO-Jrl56K9Nw7Qj4HRiu62H5PfiaokRg9JkhkA21qi7htokKRaoLcWjnpvHcCO0Fg)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 18ms (served from cache)
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
✓ Created native SWR script: github_user_native (uCHQ5SprPCSh1L_NeJ0fFoHEw1nBm1n3A6_o_zycLNfoto0B6UIStvgL-FuABCVFjg-N8RKonC31HtpDlE8ijw)

First call (cache miss - will fetch from GitHub API):
  Response time: 29ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 18ms
  Speedup: 1.6x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (GGBW6TNs5dSatdM7VYYDTGBikZlr4Uw1Fn49_eU5oCnqAwwcVqZBnM7CXf0Bddd7B8n0G1OPRwiAmMfERFOmpA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (HDNT2A3Uv7etz02SWtJEz3UPilIs76uiqzMY7VwPMlNPakUdlSwyflL2ovv7hGJrVWJgy-rmbugN8EvOLbAg9Q)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (Io2mAKAXRXh2byUJeBxnUX5gnnZg6qc7Od4fJZRT-2DxrhDPIN8OZAk1WhFCG5Uk_SY2RlPsjcrRIQG_18_7DQ)
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
   {"category":"Furniture","count":3,"avg_price":365.6666666666667}
   {"category":"Electronics","avg_price":367.0,"count":5}
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
   {"response":"Vector databases offer several key benefits:\n\n1. **Efficient Similarity Search**: They enable fast retrieval of similar items using vector representations, which is ideal for applications like recommendation systems and image retrieval.\n\n2. **High-dimensional Data Handling**: Vector databases are designed to manage and query high-dimensional data effectively, which is common in machine learning applications.\n\n3. **Scalability**: They can handle large datasets and scale well, accommodating growth in data volume without significant performance loss.\n\n4. **Flexibility with Data Types**: They support various data types, such as text, images, and audio, making them versatile for different use cases.\n\n5. **Integration with AI/ML**: Vector databases easily integrate with machine learning models, facilitating the use of embeddings for tasks like natural language processing and computer vision.\n\n6. **Improved Performance with Indexing**: Advanced indexing techniques (like HNSW or Annoy) optimize search speed, reducing latency in retrieving relevant results.\n\n7. **Real-time Processing**: They allow for real-time updates and queries, which is beneficial for applications requiring immediate responses.\n\nThese features make vector databases particularly suitable for applications in AI, machine learning, and large-scale data analysis.","tokens_used":275}
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
   {"status":"inactive","count":3}
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
   1. {"type":"String","value":"Introduction to Machine Learning"} ({"type":"String","value":"AI"})
   2. {"type":"String","value":"Vector Databases Explained"} ({"value":"Database","type":"String"})
   3. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   4. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   5. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":2,"category":"AI"}
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
✓ Created session: Ucup6loNBzWcW-e0C5movt_K9GlAjAMv1G-oyhiH0Q58k1pjWnKWfjy4U7xBmTRPFaQ1zjWFPWuDmHRLgM5RGw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "VJGkn4vKuLytMlgxEy85iGM0KbH4zj8wFvlN2o3XXukw_HNjkVUFlEdBIobe1hAqOEfCeJcKmnEfP2cueIWW7w"
  Responses: ["ekoDB is a high-performance database designed to support intelligent caching, real-time capabilities, and AI integration. Here are some of its key features:\n\n1. **AI Chat Integration**: The chat feature allows you to query your database using natural language and receive AI-powered responses that include relevant context.\n\n2. **Search Capabilities**:\n   - **Full-Text Search**: Supports searching across all text fields for relevant content.\n   - **Vector Search**: Utilizes vector embeddings for semantic search, allowing for meaning-based queries.\n   - **Hybrid Search**: Combines both full-text and vector search capabilities for optimal retrieval of relevant information, including automatic context retrieval.\n\nThese features make ekoDB versatile for various applications, enabling efficient data management and retrieval through intelligent technologies."]

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
✓ Created session: vEDqD5OZAmbuGQt4X7IhWH4dGJJZanJw7XAfVZBX9jc7HuPkoQqp4ZENtAHTeTwpLSpokeTCOBGU5lL1kOoAgQ

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Available products include:\n\n1. **Name**: ekoDB\n   - **Description**: High-performance database product\n   - **Price**: $99\n\nIf you need more information or details about other products, feel free to ask!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["One product available is the **ekoDB**, which is a high-performance database product. It is priced at **$99**. If you need more information about this product or additional options, feel free to ask!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: 1_pLghScGjhgDoKtmbFor5ndaVPUK0c-roTZFve4A_PqmUQaFk7fYBPTWABVfrDfal6EBhgvwm6heriId6RxIw

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 2

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 9s
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
✓ Created session: ThjMeAu279MpbwM4CTWiSYjze1ym8uY8yZfDuPlaCwf541EjbACGgVjA8Cp62cblVqKI783llm8yj0ZrFuWBAA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Price**: $99\n\nIf you need more information or other products, please let me know!"]

✓ Message 2 sent
  Responses: ["The price of ekoDB is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"ThjMeAu279MpbwM4CTWiSYjze1ym8uY8yZfDuPlaCwf541EjbACGgVjA8Cp62cblVqKI783llm8yj0ZrFuWBAA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"M45IKwUSIpmdIZU7cg19JsIKPX9IIk49IYD1ujXHwJqArxx40SUs_osdPjwP_U59VjL1t6lTDhNMS5Dbk2IXug","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-03-28T23:56:05.293539Z"},"id":"y1VP2OtXhKVpVkepcVtYNtH82qEFsf3-hdfRgM03yx6s4HA-viAKZ7WUayA-evdKTwALS2Fd3C7x4iT423pnuA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":56,"prompt_tokens":2504,"total_tokens":2560}},"updated_at":{"type":"DateTime","value":"2026-03-28T23:56:05.293539Z"}},{"chat_id":{"type":"String","value":"ThjMeAu279MpbwM4CTWiSYjze1ym8uY8yZfDuPlaCwf541EjbACGgVjA8Cp62cblVqKI783llm8yj0ZrFuWBAA"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Price**: $99\n\nIf you need more information or other products, please let me know!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"M45IKwUSIpmdIZU7cg19JsIKPX9IIk49IYD1ujXHwJqArxx40SUs_osdPjwP_U59VjL1t6lTDhNMS5Dbk2IXug","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-03-28T23:56:05.298625Z"},"id":"NbM3jfD42zKVf0_gSx79kByjXrLgoxYvpiM4PVaaZCzF_vmeHU5XR8I0dc9dDghDhK085WIlixmh4D2V_xkZqQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":56,"prompt_tokens":2504,"total_tokens":2560}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","limit":10,"select_fields":["product","price"]},"id":"call_WmXqu90g3FoWtcv4P6ZglX0e","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"id":"M45IKwUSIpmdIZU7cg19JsIKPX9IIk49IYD1ujXHwJqArxx40SUs_osdPjwP_U59VjL1t6lTDhNMS5Dbk2IXug","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_WmXqu90g3FoWtcv4P6ZglX0e","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-03-28T23:56:05.298625Z"}},{"chat_id":{"type":"String","value":"ThjMeAu279MpbwM4CTWiSYjze1ym8uY8yZfDuPlaCwf541EjbACGgVjA8Cp62cblVqKI783llm8yj0ZrFuWBAA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"M45IKwUSIpmdIZU7cg19JsIKPX9IIk49IYD1ujXHwJqArxx40SUs_osdPjwP_U59VjL1t6lTDhNMS5Dbk2IXug","price":99,"product":"ekoDB"},"score":1.37}]},"created_at":{"type":"DateTime","value":"2026-03-28T23:56:07.269877Z"},"id":"thcnpXa3mYF00PMD2f39OgV-w3KiycqcFYhNARnRoo89QTzCLG05xBS319qvF0Oj8_avLM331AvKJQr2iFUKIA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":6,"prompt_tokens":1261,"total_tokens":1267}},"updated_at":{"type":"DateTime","value":"2026-03-28T23:56:07.269877Z"}},{"chat_id":{"type":"String","value":"ThjMeAu279MpbwM4CTWiSYjze1ym8uY8yZfDuPlaCwf541EjbACGgVjA8Cp62cblVqKI783llm8yj0ZrFuWBAA"},"content":{"type":"String","value":"The price of ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"M45IKwUSIpmdIZU7cg19JsIKPX9IIk49IYD1ujXHwJqArxx40SUs_osdPjwP_U59VjL1t6lTDhNMS5Dbk2IXug","price":99,"product":"ekoDB"},"score":1.37}]},"created_at":{"type":"DateTime","value":"2026-03-28T23:56:07.275306Z"},"id":"CWuefNXfFi8ne8dAUn1oD4kwHtREYtPp-eMJXf1en2IkOu03l3exq41ygvxfWSqduGQFzCPA2wP1Ci3qjqJuHQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":6,"prompt_tokens":1261,"total_tokens":1267}},"updated_at":{"type":"DateTime","value":"2026-03-28T23:56:07.275306Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: hPT05lA7Vh9yTsNXZaOmFYsfhY_bWkEGMabgXwkzJYPOsz1Yc5DPwXRkva9gGChVQWWV1LUOagY4MAiHAhWdtg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
Created user function with ID: 2XWrbA-m5ng1TDf9kxkKvrUdAKxPOtgZ14ERsqPjSnOajzSHuo3S3WbhTZB-cIWpnNaAZBfflDJNKh0Suglr6g

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 31 user functions:
  - "validate_user": "Check if user exists"
  - "validate_user": "Check if user exists"
  - "validate_user": "Check if user exists"
  - "get_active_users_kt": "Get Active Users (Kotlin)"
  - "fetch_api_user": "Fetch User with Cache"

=== List User Functions by Tag ===
Found 31 user functions with 'kotlin' tag:
  - "validate_user"
  - "validate_user"
  - "validate_user"
  - "get_active_users_kt"
  - "fetch_api_user"
  - "cache_api_call_py"
  - "swr_user"
  - "cache_api_call"
  - "cache_api_call_js"
  - "swr_user"
  - "validate_user"
  - "fetch_and_store_user"
  - "fetch_user"
  - "fetch_user"
  - "fetch_slim_user"
  - "fetch_slim_user"
  - "fetch_and_store_user"
  - "swr_user"
  - "fetch_slim_user"
  - "swr_user"
  - "fetch_and_store_user"
  - "swr_user"
  - "fetch_and_store_user"
  - "fetch_slim_user"
  - "fetch_slim_user"
  - "fetch_user"
  - "fetch_and_store_user"
  - "fetch_user"
  - "fetch_user"
  - "validate_user"
  - "fetch_product_with_reviews"

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
✓ Created record: EkoRecord(fields={id=StringValue(value=NrQ92I54hM4cEHU8IDJdJ56rBRDI5nWrOdagVPr90Q2wU3ZATU8eD5qOKMvEqrRS6iDjyzCQrLuzCIF7HCGcNA)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={id=StringValue(value=NrQ92I54hM4cEHU8IDJdJ56rBRDI5nWrOdagVPr90Q2wU3ZATU8eD5qOKMvEqrRS6iDjyzCQrLuzCIF7HCGcNA), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=qBoje1TyXDCbIUnIwIdBmFKCJp7VKunEOVJqkGH7U7gbbZdH9hKucjiXoKxv7fBzHCxZgXOACTttvW28-j4y9A)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=NrQ92I54hM4cEHU8IDJdJ56rBRDI5nWrOdagVPr90Q2wU3ZATU8eD5qOKMvEqrRS6iDjyzCQrLuzCIF7HCGcNA)})
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=cdhYekql9c1MI7NIhEcr165ADq6RsLRxuT9Pq6xW1vwERJXdMp2J2hF2b95t7Hcj28supinO9Gdkn_vteaQEmQ)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=1AL_gZv6IIhdx8IZ2iQZmTMWzHODMSFJlJ40jUlolVfxXd25O7JjkbmhmE2TB_REk2pmYJZZFoRgAXlgCAe4Wg)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)}), id=StringValue(value=cdhYekql9c1MI7NIhEcr165ADq6RsLRxuT9Pq6xW1vwERJXdMp2J2hF2b95t7Hcj28supinO9Gdkn_vteaQEmQ), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), id=StringValue(value=cdhYekql9c1MI7NIhEcr165ADq6RsLRxuT9Pq6xW1vwERJXdMp2J2hF2b95t7Hcj28supinO9Gdkn_vteaQEmQ)})

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

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hqOc0mnOPa-Tp7yjjKwODVbSdW-8FLL30RN0EKePRVv5YV4XFEva_ZPJAReK53nUcbJzDETF6CI985UzJ2m_mQ
Created Bob: $500 - ID: 7iwUTdBh64GD1zAw7FAqFMOIHrhMw8rEnRhu6IBDVTFBlNFUb9y0F2f1PPVrlwgjYTcOLlgcPcuWPqrEmiDd-w

=== Example 1: Begin Transaction ===
Transaction ID: 63dadd95-a61d-40b1-b18f-cc39d3cfc440

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f9073f0b-abb2-49cb-a1f2-af82a9e4b7e8
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 890.584208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 481.103792ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.194574959s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 601.39425ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 578.449542ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.102178833s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 788.498709ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 591.336709ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 538.216334ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.098693834s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 533.493584ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 473.908917ms
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
    ✓ Generated embedding: 1536 dimensions in 687.03ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 104.170208ms

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

Writing memory-safe, high-performance database code requires a combination of good architectural practices, efficient use of data structures, and optimizing your database interactions. Here are some key principles and techniques to consider:

### 1. **Choose the Right Database Model**
   - Use **SQL databases** for structured data with complex queries, ACID transactions, and well-defined relationships.
   - Use **NoSQL databases** for flexible schemas, high write throughput, horizontal scaling, and handling unstructured or semi-structured data.

### 2. **Proper Database Design**
   - **Database Normalization**: Organize your data to reduce redundancy and improve data integrity. This can lower the chances of anomalies when inserting, updating, or deleting records.
   - **Indexing**: Create indexes on frequently queried fields to speed up lookups. Use the right type of index (BTree for range queries, Hash for equality).

### 3. **Efficient Data Access Patterns**
   - **Batch Operations**: Use batch inserts/updates to minimize the number of database round trips, which often results in significant performance gains.
   - **Pagination**: When retrieving large datasets, implement pagination to limit the number of records fetched at once. This reduces memory usage and speeds up response times.
   - **Lazy Loading**: Retrieve data only when necessary. Don't load all related entities at once if you don't need them.

### 4. **Optimize Queries**
   - **Prepared Statements**: Use prepared statements to avoid SQL injection attacks; they can also improve performance by allowing the database to cache execution plans.
   - **Query Profiling**: Analyze your database queries for performance bottlenecks. Use tools like `EXPLAIN` in SQL databases to understand the execution plan.

### 5. **Memory Management**
   - Use **memory-safe languages** or frameworks that provide safety features (like Rust or languages with strong typing).
   - Keep track of resource usage, ensuring proper handling of connections and other resources. Use connection pooling to efficiently manage database connections.

### 6. **Concurrency and Transactions**
   - Use appropriate isolation levels in transactions to avoid deadlocks and ensure data integrity.
   - Apply optimistic or pessimistic locking strategies depending on the operations performed to minimize conflicts in concurrent environments.

### 7. **Testing and Monitoring**
   - Implement rigorous unit tests and integration tests to ensure your database code behaves as expected.
   - Use monitoring tools to observe database performance in real-time and identify areas that may need optimization.

### 8. **Use Proper Libraries and Frameworks**
   - Choose well-established libraries or ORM (Object-Relational Mapping) frameworks that support efficient data access and manage complexities behind the scenes.

### Summary
By combining the right database technology, thoughtful design, optimization techniques, and safe coding practices, you can create a database application that is both memory-safe and high-performance. Focus on profiling, testing, and iterating on your code to adapt to changing requirements and data patterns.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 569.528875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3184 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 617.520916ms
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
  ✓ Text search completed in 65.473834ms

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
    ✓ Generated embedding: 1536 dimensions in 0.666s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.303s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.585s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.196s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.445s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.355s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.364s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.340s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.575s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.586s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.574s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.997s
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
    ✓ Generated embedding: 1536 dimensions in 0.567s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.084s

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

Writing memory-safe, high-performance database code involves considering various aspects of programming practices and database management. Here are key strategies you can employ:

### 1. Use Strongly Typed Languages
- **Choose languages** like Rust, C#, or Java that enforce strong type safety at compile time. This reduces the risk of type-related bugs and memory leaks.

### 2. Adopt Memory Management Techniques
- **Garbage Collection**: Use languages with automatic garbage collection (like Java or Python) to manage memory, avoiding memory leaks.
- **Manual Management**: If using lower-level languages (like C or C++), ensure you are diligent with memory allocation and deallocation. Use smart pointers (in C++) to manage dynamic memory safely.

### 3. Database Connection Management
- **Pooling**: Use connection pools to manage database connections efficiently. This reduces overhead by reusing existing connections rather than opening new ones for every database operation.
- **Explicit Cleanup**: Always close connections, statements, and result sets explicitly in your code to prevent resource leaks.

### 4. Optimize SQL Queries
- **Use Prepared Statements**: They prevent SQL injection and can optimize performance by allowing the database to cache query plans.
- **Normalized Data**: Utilize database normalization to minimize redundancy, which can improve query performance and maintain data integrity.
- **Indexes**: Create appropriate indexes to speed up query performance. Be cautious about excessive indexing, as it can slow down write operations.

### 5. Batch Processing
- **Bulk Operations**: Instead of executing multiple single-row inserts or updates, use bulk operations to reduce the number of database round trips and utilize resources more efficiently.

### 6. Asynchronous Programming
- Use asynchronous database drivers or libraries to avoid blocking operations that can slow down your application, especially in I/O-bound scenarios.

### 7. Monitor and Profile Performance
- Utilize profiling tools to analyze database queries and application performance. Look for slow queries and optimize them, and monitor memory usage to identify potential leaks.

### 8. Error Handling
- Implement robust error handling to deal with exceptions gracefully without leaking resources or crashing the application.

### 9. Use Transactions Wisely
- Utilize transactions to ensure data consistency, especially for operations that affect multiple records, but avoid holding long transactions open to minimize lock contention and conflicts.

### 10. Regular Maintenance
- Perform routine database maintenance tasks such as updating statistics, rebuilding indexes, and cleaning up obsolete data to keep the database performant.

### Example Code Snippet
Here's an example of how you might implement a database access layer in Python using a connection pool and prepared statements:

```python
import psycopg2
from psycopg2 import pool

# Set up a connection pool
connection_pool = psycopg2.pool.SimpleConnectionPool(minconn=1, maxconn=10, 
                                                      dsn="dbname=test user=postgres password=secret")

def fetch_user(user_id):
    connection = None
    try:
        connection = connection_pool.getconn()
        with connection.cursor() as cursor:
            cursor.execute("SELECT * FROM users WHERE id = %s", (user_id,))
            return cursor.fetchone()
    except Exception as e:
        print(f"Error fetching user: {e}")
    finally:
        if connection:
            connection_pool.putconn(connection)

# Usage
user = fetch_user(1)
print(user)
```

This example demonstrates safe memory management through use of context managers and connection pooling while employing prepared statements for optimal performance.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.699s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3779 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.644s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.833s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.414s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.518s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.545s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.671s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.586s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.575s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.557s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.429s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.683s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.098s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.477s
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
    ✓ Generated embedding: 1536 dimensions in 0.584s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.082s
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

Writing memory-safe, high-performance database code is crucial for creating robust and efficient applications. Here are several strategies you can implement:

### 1. **Use Prepared Statements**
   - **Why:** Prepared statements help prevent SQL injection attacks and can reduce parsing time when running the same statements multiple times.
   - **How:** Use parameterized queries instead of string concatenation for crafting SQL commands.

### 2. **Connection Pooling**
   - **Why:** Establishing a database connection can be expensive, so pooling minimizes overhead by reusing connections.
   - **How:** Use a connection pool library relevant to your database and programming language to manage connections efficiently.

### 3. **Batch Operations**
   - **Why:** Minimizes round-trip times to the database by sending multiple operations in a single request.
   - **How:** Use batch inserts/updates where possible (e.g., `INSERT INTO table (col1, col2) VALUES (val1, val2), (val3, val4);`).

### 4. **Indexing**
   - **Why:** Proper indexing significantly speeds up data retrieval queries.
   - **How:** Analyze your queries and create indexes on frequently accessed fields. Use database analysis tools to recommend optimal indexes.

### 5. **Data Normalization**
   - **Why:** Reduces data redundancy and improves data integrity.
   - **How:** Organize your database into smaller, related tables, and use foreign keys to maintain relationships.

### 6. **Use Transactions Wisely**
   - **Why:** Transactions ensure that a series of operations either all complete or none do, maintaining data integrity.
   - **How:** Utilize transaction management (begin, commit, rollback) to handle groups of operations safely and effectively.

### 7. **Memory Management**
   - **Why:** To prevent memory leaks, particularly in languages where manual memory management is required (e.g., C/C++).
   - **How:** Always free allocated memory and handle error cases gracefully. In managed languages (e.g., Java, C#), ensure objects are disposed of properly.

### 8. **Asynchronous Operations**
   - **Why:** To avoid blocking the main thread, which can improve responsiveness in UI applications.
   - **How:** Use asynchronous database libraries or calls that allow you to perform other operations while waiting for a database response.

### 9. **Monitoring and Profiling**
   - **Why:** Identifying bottlenecks in database operations can lead to performance improvements.
   - **How:** Use database query profiling tools to monitor performance, and analyze logs and slow query reports to optimize.

### 10. **Data Pagination**
   - **Why:** Retrieving large datasets at once can consume excessive memory and slow down response times.
   - **How:** Implement pagination techniques (e.g., `LIMIT` and `OFFSET` in SQL) to fetch only the required data at a time.

### 11. **Microservices Architecture**
   - **Why:** It allows for building applications that scale better by separating database responsibilities.
   - **How:** Use different services for different functionalities, each possibly having its own database.

### 12. **Configuration Management**
   - **Why:** Ensure optimal performance without hardcoding settings.
   - **How:** Use configuration files to manage settings like database URLs, connection limits, and timeouts.

By following these practices, you can enhance the safety and performance of your database operations. Always be sure to review and test different configurations to determine the best settings for your specific use case.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.820s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3537 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.389s
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
  ✓ Text search completed in 0.053s
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
    ✓ Generated embedding: 1536 dimensions in 0.361s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.369s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.573s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.487s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.785s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.431s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.518s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.557s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.651s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.551s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.584s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.568s
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
    ✓ Generated embedding: 1536 dimensions in 0.422s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
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
     What is database normalization?

  3. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
2026/03/28 19:58:55 Network error, retrying after 3s...
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several key practices and principles, including efficient resource management, proper data structures, and best coding practices. Here are some approaches to achieve this:

### 1. Use Connection Pooling
- **Connection pooling** allows you to manage multiple database connections efficiently by reusing existing connections rather than creating a new connection for each request. This reduces overhead and improves performance.
  
### 2. Optimize Query Performance
- **Write efficient queries**: Avoid SELECT *; instead, return only the necessary fields. Use WHERE clauses to filter data as much as possible.
- **Use indexes**: Properly index your tables to speed up query execution, especially on frequently queried fields.
- **Batch operations**: Use batch inserts/updates to minimize the number of round-trips to the database.

### 3. Handle Memory Management
- **Use memory-safe languages or libraries**: In languages like Rust, memory safety is built into the language. For languages like C++ or Java, use tools (like smart pointers in C++) to manage memory automatically.
- **Avoid memory leaks**: Always free up resources and employ techniques like RAII (Resource Acquisition Is Initialization).

### 4. Implement Proper Error Handling
- **Graceful error handling**: Ensure that your application can handle database errors gracefully without crashing. Use try/catch blocks (in languages that support them) or equivalent error handling mechanisms.

### 5. Employ Lazy Loading and Caching
- Load only the necessary data when needed (lazy loading) instead of all at once, which helps save memory.
- Use caching strategies (like in-memory caches) to reduce database load and speeding up data retrieval for frequently accessed data.

### 6. Apply Database Normalization
- **Design your database schema**: Normalize your database to reduce redundancy and improve integrity, which can also help in optimizing queries and performance.

### 7. Use Asynchronous Programming Models
- Asynchronous database interactions can help improve the performance of I/O bound applications by allowing other processing while waiting for database operations to complete.

### 8. Profile and Monitor Performance
- Utilize profiling tools to monitor the performance of your database code. Identify and address bottlenecks by analyzing query execution times and server metrics.
- Consider using tools like database query analysis and system performance monitoring tools.

### 9. Use Transactions Wisely
- **ACID transactions** help ensure data integrity but can take additional time. Use them where necessary, but assess if some operations can be done outside of transactional scope to improve performance.

### 10. Learn and Adhere to Best Practices
- Stay informed about best practices in database interactions for your specific programming language and database system (e.g., PostgreSQL, MySQL, MongoDB).
- Regularly update your libraries and frameworks to leverage performance improvements and security patches.

### Conclusion
By combining these principles, you can develop high-performance, memory-safe database applications. Always consider the specific context of your application and database system to choose the most effective strategies.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.799s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3289 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.565s
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
  ✓ Text search completed in 0.052s

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
    ✓ Generated embedding: 1536 dimensions in 0.522s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.543s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.468s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.553s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.42s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.461s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.513s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.532s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.564s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.451s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.54s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.537s
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
    ✓ Generated embedding: 1536 dimensions in 0.453s
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
    ✓ Generated embedding: 1536 dimensions in 0.574s
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
  ✓ Text search completed in 0.04s

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

> @ekodb/ekodb-client@0.15.1 prepare
> npm run build


> @ekodb/ekodb-client@0.15.1 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.1 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (_L-dj1o4Ezujq5XsKmt2nZgDyk5sKZ9Rde0ZyaILbrHZBkR2Pxb9UOFJgzT0yUuMZrExC_lSiK1hcy-CUor5hw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "username": "Bret",
        "address": {
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "zipcode": "92998-3874",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
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
Response time: 47ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "username": "Bret",
        "address": {
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "zipcode": "92998-3874",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
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
✓ Created enrichment script: fetch_product_with_reviews (A7okGEueTRGwrAnXfnKt9pFmL6RV9rKP1gE1wC3Wkvs2swlUwNhemZ9Yc-97kcWXhMzhgLnjcBtf74yAmJPRrw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "shippingInformation": "Ships in 3-5 business days",
        "id": 1,
        "sku": "BEA-ESS-ESS-001",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "warrantyInformation": "1 week warranty",
        "stock": 99,
        "category": "beauty",
        "brand": "Essence",
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z"
        },
        "discountPercentage": 10.48,
        "tags": [
          "beauty",
          "mascara"
        ],
        "reviews": [
          {
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerName": "Eleanor Collins",
            "comment": "Would not recommend!"
          },
          {
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon",
            "comment": "Very satisfied!",
            "rating": 4,
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "rating": 5,
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "minimumOrderQuantity": 48,
        "weight": 4,
        "dimensions": {
          "height": 13.08,
          "depth": 22.99,
          "width": 15.14
        },
        "title": "Essence Mascara Lash Princess",
        "availabilityStatus": "In Stock",
        "price": 9.99
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
✓ Edge cache script created: yv3SKHwLNCJ1uTm2X0TeFjfGPJGdvC9zfnDYjKPQJ6Tg3T6SD8Iez771su11p5a-a8qs7-cBe_y3Gicfid3wFg

Call 1: Cache miss (fetches from API)
Response time: 166ms
Result: {
  "records": [
    {
      "value": {
        "id": 1,
        "name": "Leanne Graham",
        "email": "Sincere@april.biz",
        "username": "Bret",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
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
Response time: 42ms (4x faster!)
Result: {
  "records": [
    {
      "value": {
        "id": 1,
        "name": "Leanne Graham",
        "email": "Sincere@april.biz",
        "username": "Bret",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
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

✅ [32mTypeScript SWR examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.15.1
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (RheowiP6mT3ycmvTSeZ1rmjECtfeF8dhwf4LrtIJ0832rvUtwre2Yzak1KNEg_OIhmiONP9Ph-De0GCyxyc-Vg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 298ms
Result: [
  {
    "cached_at": "1774742380",
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
        "followers": 293476,
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
Response time: 54ms (5.5x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (F5RsuQprDVSeuG5k_BlU1uj9yKsOM_ZHW8JgtYUP-woDKgGPar7j1OtHJaE1_NJjAU981G1d0BqRVOzwJ8vjLA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1774742381",
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
✓ Created SWR script: fetch_github_user (YKQAQ7LXc2XscYANzSEDjutkBB-Fv0bsOuHizj74fYL_sMP7PTNaaTM7PvNluHiYNaqcDCUa50sHc6ENGaBNXA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 39.462125ms
Result: [
  {
    "cached_at": "1774742380",
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
        "followers": 293476,
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
Response time: 40.882333ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (dkySWW8OjciqIfweYrqv8cqhhblzF1dhHD-9CHQK-94SZqHg_vONlvsf2l01MPM36uEof0aQKJ6WzolS1aua2w)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (fgB_RwKXYTX5drQ8ye_JJ7aYSInN1kmvysA3o6IlMvIioJSJJlqwcDEt8CfGutEnzzooY-IVEtRR9f0obFnb0g)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 40ms
Result: {
  "cached_at": "1774742380",
  "data": {
    "value": {
      "id": 1024025,
      "hireable": null,
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "blog": "",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "site_admin": false,
      "followers": 293476,
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "created_at": "2011-09-03T15:26:22Z",
      "public_repos": 11,
      "gravatar_id": "",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "following": 0,
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "twitter_username": null,
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "public_gists": 1,
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "company": "Linux Foundation",
      "login": "torvalds",
      "location": "Portland, OR",
      "name": "Linus Torvalds",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "html_url": "https://github.com/torvalds",
      "type": "User",
      "bio": null,
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "email": null,
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "user_view_type": "public",
      "url": "https://api.github.com/users/torvalds",
      "updated_at": "2026-01-13T07:10:05Z"
    },
    "type": "Object"
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 41ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (FjrSSmt0_HU5ujMr8khPTqKpuA1F0yMftqbSV_fbuRAjVB0vwiZShSPcGjONEMLztm8Nzx06Z46k35f-hiCh3A)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "value": {
      "minimumOrderQuantity": 48,
      "id": 1,
      "sku": "BEA-ESS-ESS-001",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "weight": 4,
      "brand": "Essence",
      "shippingInformation": "Ships in 3-5 business days",
      "category": "beauty",
      "reviews": [
        {
          "rating": 3,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Would not recommend!",
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z"
        },
        {
          "rating": 4,
          "comment": "Very satisfied!",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Lucas Gordon",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com"
        },
        {
          "rating": 5,
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Highly impressed!"
        }
      ],
      "warrantyInformation": "1 week warranty",
      "meta": {
        "createdAt": "2025-04-30T09:41:02.053Z",
        "updatedAt": "2025-04-30T09:41:02.053Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "barcode": "5784719087687"
      },
      "price": 9.99,
      "tags": [
        "beauty",
        "mascara"
      ],
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "returnPolicy": "No return policy",
      "discountPercentage": 10.48,
      "title": "Essence Mascara Lash Princess",
      "rating": 2.56,
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "dimensions": {
        "width": 15.14,
        "depth": 22.99,
        "height": 13.08
      },
      "availabilityStatus": "In Stock",
      "stock": 99
    },
    "type": "Object"
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
✓ Created SWR script: fetch_github_user (MzYlEyUPbqPTLvcglFDQWVEgke8Qcl5U6Cl45dsE8IZZeaOK1TydKOsOkeaetCy17bnCbOXDTUT1zZ0O0NGfig)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 54ms
Result: [{"data":{"type":"Object","value":{"url":"https://api.github.com/users/torvalds","gravatar_id":"","public_gists":1,"repos_url":"https://api.github.com/users/torvalds/repos","node_id":"MDQ6VXNlcjEwMjQwMjU=","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","updated_at":"2026-01-13T07:10:05Z","html_url":"https://github.com/torvalds","email":null,"following_url":"https://api.github.com/users/torvalds/following{/other_user}","company":"Linux Foundation","followers_url":"https://api.github.com/users/torvalds/followers","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","organizations_url":"https://api.github.com/users/torvalds/orgs","id":1024025,"following":0,"created_at":"2011-09-03T15:26:22Z","received_events_url":"https://api.github.com/users/torvalds/received_events","twitter_username":null,"site_admin":false,"user_view_type":"public","events_url":"https://api.github.com/users/torvalds/events{/privacy}","name":"Linus Torvalds","hireable":null,"public_repos":11,"type":"User","blog":"","followers":293476,"bio":null,"avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","location":"Portland, OR","login":"torvalds"}},"cached_at":"1774742380"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 46ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (9o3i-_wr0zRVgjHtG3gCcXK3YI3O8EA4FniclJNkRHh5BSUiNQABO81QRIfdZHRyWzMnrBdiDAjzqQ0f5c5NFg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"discountPercentage":10.48,"availabilityStatus":"In Stock","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","meta":{"createdAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687","updatedAt":"2025-04-30T09:41:02.053Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png"},"shippingInformation":"Ships in 3-5 business days","price":9.99,"sku":"BEA-ESS-ESS-001","reviews":[{"reviewerEmail":"eleanor.collins@x.dummyjson.com","rating":3,"comment":"Would not recommend!","reviewerName":"Eleanor Collins","date":"2025-04-30T09:41:02.053Z"},{"rating":4,"date":"2025-04-30T09:41:02.053Z","reviewerName":"Lucas Gordon","reviewerEmail":"lucas.gordon@x.dummyjson.com","comment":"Very satisfied!"},{"reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins","comment":"Highly impressed!","rating":5}],"tags":["beauty","mascara"],"category":"beauty","rating":2.56,"stock":99,"warrantyInformation":"1 week warranty","description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","minimumOrderQuantity":48,"id":1,"dimensions":{"depth":22.99,"width":15.14,"height":13.08},"returnPolicy":"No return policy","weight":4,"brand":"Essence","title":"Essence Mascara Lash Princess","images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"]}}}]
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
   ⏱️  Duration: 169.65ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "username": "Bret",
    "phone": "1-770-736-8031 x56442",
    "name": "Leanne Graham",
    "address": {
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "A...

Second call (cache hit - from cache):
   ⏱️  Duration: 41.538708ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "suite": "Apt. 556",
      "street": "Kulas Light",
      "city": "Gwenborough",
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "...
   🚀 Cache speedup: 4.1x faster!

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

> @ekodb/ekodb-client@0.15.1 prepare
> npm run build


> @ekodb/ekodb-client@0.15.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.1 build
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...

Second call (cache hit - from cache):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...
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
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.1
    Uninstalling ekodb_client-0.15.1:
      Successfully uninstalled ekodb_client-0.15.1
Successfully installed ekodb-client-0.15.1
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
   ⏱️  Duration: 40.8ms
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
   ⏱️  Duration: 41.7ms
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
   ⏱️  Duration: 39.853ms
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
   ⏱️  Duration: 40.166834ms
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
   🚀 Cache speedup: 1.0x faster!

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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 43ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...

Second call (cache hit - from cache):
   ⏱️  Duration: 46ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
✅ [32mClient build complete![0m

🦀 [33mRust WebSocket Subscription Test...[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_12e001e3ff0d4c6faeef034e5df09b19)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "bTmjzropS9Gbe6iPLxTDfHQHSsG0J29cKXMfcD3RvyY8nil83glaPmUF4AeaDLWSFTdxmhfTYelKaHeQa3mHLQ"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["bTmjzropS9Gbe6iPLxTDfHQHSsG0J29cKXMfcD3RvyY8nil83glaPmUF4AeaDLWSFTdxmhfTYelKaHeQa3mHLQ"]
     Timestamp:  "2026-03-28T23:59:54.706011+00:00"

Inserting record 2...
✓ Inserted: "xcfbTqlUGHFVlgoOc0VqEFrKAdmcO-pKKpjkE6uh2Pe2wraLmpV45QNuK0ZZIpSnvuLhZsf2hIHLK22BcflWkg"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["xcfbTqlUGHFVlgoOc0VqEFrKAdmcO-pKKpjkE6uh2Pe2wraLmpV45QNuK0ZZIpSnvuLhZsf2hIHLK22BcflWkg"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mRust subscription test complete![0m

🔷 [33mGo WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_ebf72ad3cf634b0aad928532a44c4b83)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: M7Y9xOFzfWtaS6MTKt5BQ8VL9LOSCjT7h28twcsoOKNI-qsmSWwtfWE9lkqtGns3ctKEL36q5c2bAotTHYPBFw

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [M7Y9xOFzfWtaS6MTKt5BQ8VL9LOSCjT7h28twcsoOKNI-qsmSWwtfWE9lkqtGns3ctKEL36q5c2bAotTHYPBFw]
     Timestamp:  2026-03-28T23:59:54.834657+00:00

Inserting record 2...
✓ Inserted: Lu3BkXwOGWtx33t0-gknTWF7qy37ed2ps02agj1xNNiYZG-rSqucqa69t9fSjFSz4_SkIiSw5sjzrW6cQRxTUA

  📡 Notification received:
     Event:      insert
     Record IDs: [Lu3BkXwOGWtx33t0-gknTWF7qy37ed2ps02agj1xNNiYZG-rSqucqa69t9fSjFSz4_SkIiSw5sjzrW6cQRxTUA]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mGo subscription test complete![0m

🐍 [33mPython WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_3e77a686ab5f4d8db4caab93629c2a38)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 8clJ_pqkXl62Xj28KFpQNdWT0z7rFT6db6ZRS_2Z9vm-r9bWMUpBvRjlWWhocAVPJR1KpBLiNznfQaTunWqKPA

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: 8clJ_pqkXl62Xj28KFpQNdWT0z7rFT6db6ZRS_2Z9vm-r9bWMUpBvRjlWWhocAVPJR1KpBLiNznfQaTunWqKPA
     Timestamp:  2026-03-28T23:59:54.970799+00:00

Inserting record 2...
✓ Inserted: RzBax55GbM_2d86epcSAXXeISat1i2R9pNkmFbwh-J9fxOkP5d2qrGO5eUb2f633KhekSj6qN48Fi-oUq5MO9g

  📡 Notification received:
     Event:      insert
     Record IDs: RzBax55GbM_2d86epcSAXXeISat1i2R9pNkmFbwh-J9fxOkP5d2qrGO5eUb2f633KhekSj6qN48Fi-oUq5MO9g

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

✓ WebSocket subscription example completed successfully
✅ [32mPython subscription test complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.1 prepare
> npm run build


> @ekodb/ekodb-client@0.15.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.1 build
> tsc

✅ [32mTypeScript client built![0m

📘 [33mTypeScript WebSocket Subscription Test...[0m
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_960624b9d60b4eaba05a6d8a90b3c90d)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: iidINHXwPEehmGitPDGFPIxVwTsgpMPHOcZAgq-921PCa1S-W-r8CYMPuAtMtjO1k-7QfeeQ-zAK94z-3ml1Uw
     Timestamp:  2026-03-28T23:59:57.547161+00:00
     Records:    {"active":true,"id":"iidINHXwPEehmGitPDGFPIxVwTsgpMPHOcZAgq-921PCa1S-W-r8CYMPuAtMtjO1k-7QfeeQ-zAK94z...
✓ Inserted record: iidINHXwPEehmGitPDGFPIxVwTsgpMPHOcZAgq-921PCa1S-W-r8CYMPuAtMtjO1k-7QfeeQ-zAK94z-3ml1Uw

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: eZktyz5e0G2UmUijfrfBO9sU_o3VAoV8jx4fdf44Vwl7Mt7f6XSU3plPBLhkvbqlRApXKur_nJxRBofFXLJrTQ
     Timestamp:  2026-03-28T23:59:58.562927+00:00
     Records:    {"active":true,"id":"eZktyz5e0G2UmUijfrfBO9sU_o3VAoV8jx4fdf44Vwl7Mt7f6XSU3plPBLhkvbqlRApXKur_nJxRBof...

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
✓ Subscribed (subscription_id: sub_59adddf9d30d4577964ce6befd1a4ebf)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "qZ2RVGEuFkVAwso4DL3dBpi5raF9ktripTI0BoRFoni0YvC9zhoOH0CFVoBHORwxN1rcOVu_sZunFJUP5J9niA"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["qZ2RVGEuFkVAwso4DL3dBpi5raF9ktripTI0BoRFoni0YvC9zhoOH0CFVoBHORwxN1rcOVu_sZunFJUP5J9niA"]
     Timestamp:  "2026-03-29T00:00:03.535528+00:00"

Inserting record 2...
✓ Inserted: "NYPi4mzs6W9Rm827PyWp3NLve6aNELjhgogNLmHtpLqpTMdwKPYKb0biJ9O5urlJwaZMV7aLo-GIDjmJMIFdbg"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["NYPi4mzs6W9Rm827PyWp3NLve6aNELjhgogNLmHtpLqpTMdwKPYKb0biJ9O5urlJwaZMV7aLo-GIDjmJMIFdbg"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin subscription test complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll WebSocket Subscription Tests Passed![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
