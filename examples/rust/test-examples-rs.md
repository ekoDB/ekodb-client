make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("QijvJVGRC5OnKo3dv3ywhMbt4FHKn3NUoTQEXABaFlmf7yEWiSsMF9Vo6rVZzXK2D7jWLEPz_CsPtcBN8T6FPg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("QijvJVGRC5OnKo3dv3ywhMbt4FHKn3NUoTQEXABaFlmf7yEWiSsMF9Vo6rVZzXK2D7jWLEPz_CsPtcBN8T6FPg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("QijvJVGRC5OnKo3dv3ywhMbt4FHKn3NUoTQEXABaFlmf7yEWiSsMF9Vo6rVZzXK2D7jWLEPz_CsPtcBN8T6FPg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("QijvJVGRC5OnKo3dv3ywhMbt4FHKn3NUoTQEXABaFlmf7yEWiSsMF9Vo6rVZzXK2D7jWLEPz_CsPtcBN8T6FPg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "gFEqTXq3N-9Rjq25gBAgck5lECyiNA4m7S70bsLetAO7GjvryzHjPOF5SCbfMLKA7rf5fjVQkU9GUpQFU5H-cA"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774734222253268000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "gFEqTXq3N-9Rjq25gBAgck5lECyiNA4m7S70bsLetAO7GjvryzHjPOF5SCbfMLKA7rf5fjVQkU9GUpQFU5H-cA",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "K23J9mzKlrTTKuozC9Hnwl5JOj9aszDYpvU6qYEo9cz2p24WhMl4Dzq5YmqRFve1tzj2xZxrs0AvdMZCBunu3Q"

=== List Collections ===
Total collections: 5
Sample collections: ["audit__ek0_testing", "batch_users", "test_collection", "demo_collection", "websocket_test"]

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
✓ Inserted document: "Bdx-XhMeA7_tJQRZ908FFRj4ayzwiN0qRFwQnNm6C25NLXry_o3A_UxwH5nc8wh0fsvaFPK5J77UMp6SXY2VBQ"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "ARCXMavSfwqvb3SWomoqSsUGKmwtTToRHfdOOw-QN0JQGDQ6s5Ib4R0joV0tyJtSdgE7jm0RGzbV0vCOf1Z1-A"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "a2qgMqxE7KkegJn3fAex06EbnAJUloIBMtFuXXIzSwp2qb5wSriKFN7gU7IEgBvLCT5QSdQO52FEr8bxV8JY6g"

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
✓ Inserted document with TTL: String("38Dfk33QTAVaJNcBw-iqkw7iYixlo7m88W33-rvd9SiVEjI7jd8ARw3iisOskEBSuqyijZ4L4akCu3z5mFxC3w")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "38Dfk33QTAVaJNcBw-iqkw7iYixlo7m88W33-rvd9SiVEjI7jd8ARw3iisOskEBSuqyijZ4L4akCu3z5mFxC3w"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-03-28T22:44:04.104193Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: mY_024UkisJA8-VevHfE87x-WlG-04XzHrkRCnVOnzQt_SKZ2JmtsJcOO5S_0ygF0LFBB63vORP9to5cuHbeYw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "J2RI9ODuRVwzqsnMr0QKM2EkGsSEj_SxMzamXy5SkDZKAJNyKdIvfqjsI5cJ7uNTIv8aGLfWlr-Hk75pQFrmmA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: CvJDGvbW0pND3D5WPIusq9TX970ldgxJa49DFQz-A_Q1ealBvZY3KsxGqtjqa2rNvx1zYI2fqa29O8-uzMJluA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: mc5NmX48Gk8e2_ILWdbWyuCgSk14w0WuqS4VhbZc1F70bDzFqPF0T0MI7t2vHJIlpSQOn-lc59CcFQh8hK3rUA
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
Created Alice: $1000 - ID: dGMmCx3EoAs8MWk6vRUHdq2PFwL2DNw7T7OHma05zVy-ZmWg2zSc8IN_ZpzQ6lBx2ljPSO66uCDIQb_K1wNEcA
Created Bob: $500 - ID: VrC0zRSBLzGgl0eaHN07hoN_EGhLs0eBGGoaCvhTsn_vqBxsWicCEl1TDG38-DsI3H1GiP9UYDTA_LKUe56jKg

=== Example 1: Begin Transaction ===
Transaction ID: 36dfa3b3-03bc-4aeb-952d-ff5d4a1bc502

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
New transaction: 42f31f91-eadc-490c-97a0-727e612a3428
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("eJgCjxCS3CmY-BlwhkNeh3mmGR_jHim2sBR8WhwYKK6AiTnXPiGsQ8IoM_yciSYMR0fCMKmZGJM3fmsy3yBfYQ")})

=== Find by ID ===
Found: Record({"data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "id": String("eJgCjxCS3CmY-BlwhkNeh3mmGR_jHim2sBR8WhwYKK6AiTnXPiGsQ8IoM_yciSYMR0fCMKmZGJM3fmsy3yBfYQ"), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "value": Object({"value": Integer(42), "type": String("Integer")}), "created_at": Object({"value": String("2026-03-28T21:44:08.840083Z"), "type": String("DateTime")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-03-28T21:44:08.840083Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"name": Object({"type": String("String"), "value": String("Test Record")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "id": String("eJgCjxCS3CmY-BlwhkNeh3mmGR_jHim2sBR8WhwYKK6AiTnXPiGsQ8IoM_yciSYMR0fCMKmZGJM3fmsy3yBfYQ"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-03-28T21:44:08.840083Z")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")})})]

=== Update Document ===
Updated: Record({"metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "id": String("eJgCjxCS3CmY-BlwhkNeh3mmGR_jHim2sBR8WhwYKK6AiTnXPiGsQ8IoM_yciSYMR0fCMKmZGJM3fmsy3yBfYQ"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "created_at": Object({"value": String("2026-03-28T21:44:08.840083Z"), "type": String("DateTime")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: AYgf7S-EWI9xvKlVghRBYSfhZMk4Y4LxRLIycoBb9KgDcIKJUy_tY5nwRUPW9XSlR7boJAJ5HNCRS2222EN7-g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  value: 42
  id: "AYgf7S-EWI9xvKlVghRBYSfhZMk4Y4LxRLIycoBb9KgDcIKJUy_tY5nwRUPW9XSlR7boJAJ5HNCRS2222EN7-g"
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
Collection created with first record: "3J5jfI9IheZ10dNFEFgRl8GPX-VEe9pBnmCp2MskeXSRSFm4AsxkKKt1wS_YLUoEY-mJ9-jjfHULnZoDq-8HGg"

=== List Collections ===
Total collections: 10
Sample collections: ["ws_ttl_test", "audit__ek0_testing", "batch_users", "ttl_cache", "test_accounts"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: B7Kv_zlta5xXfUUQqc-4BH74KjjNVysC4QaRhiHo2hZBiGKDRivzODvp-VJaM1Y3GMsWD2qxY38tzo0Tahz4IA
Created Bob: $500 - ID: EpS_1tesciWYfxbKt0QzTRzilkNU37CKKYwuAMrkvWCmDQRkraqkXnnoNgEXkT-XPrPPQlbNrlk0mm4_l5kAyg

=== Example 1: Begin Transaction ===
Transaction ID: b07522c4-251d-46b8-8e4f-0fc9b213a9f4

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 120ad00b-ef41-458c-960d-9fdcaade4ff1
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
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
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
     Matched: ["title", "description", "description.value", "title.value"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description", "description.value"]
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
✓ Inserted user 1: Some(String("fMbCEZugkwWlPo7r1z6foz8SO3e-dgg0D5REhIIni-vVvspfE-Q_oxcbpNE-SU48xy4uX2f9aHq8XXMb6Cwvjw"))
✓ Inserted user 2: Some(String("U2EvKLgP_n3Rm77TMcLXrORnHGoLOByR4Vb1_Mw0lmn3a0thO8DynSPrCLmx4AEYrhbQUfIWzF2AqRW02vVDpw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - age: Integer
  - email: String
    (required)
  - status: String

=== Listing Collections ===
✓ Total collections: 10
  Sample: ["ws_ttl_test", "audit__ek0_testing", "schema_client_rust", "batch_users", "ttl_cache"]

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
✓ Inserted document: "hQ8DXx83qs2dsZYPccWvpPNbbzab-yGo0kYaAYeD_mdIMqpQbbZEkETB6GWCuRvk6ZXT4d5GkNLcA-xdlz_uKw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("LD0I78mRSWqrQs8npE2rgfJEAotKmgQzg-czlCbn9U7-mxJIYkMuQTxxpero8cb_HkynuIaaDNuLUxOLP3OswA"))

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
✓ Inserted document with TTL: Some(String("VeGG161B-K7yyD1ZkuX-Rd-IvFsTa6SuNzMt6XfnaQnu62JTJO_sjXOqvIx4xACsfaecPSeLswIoo870nXBO5A"))

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
✓ Edge cache script created: a10hZfRWjD54OIuBnBQFcgLQYWjAYjU50ZzjyrOcRQUmVGAY0gzNS5enDW07dYD1c2maW2Lp35BDb48hRW0oVw

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

✅ Script saved: JlUankBFsqhwOWSBTaFEAma22cs37ln2CPUa80synwvIUSlcOyXspIBssbJ4k0yf-m4-uvlxr8xlqwY68J5d9A
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 8FKOFL_ovPqZQPn98UF2DKUf1h45Ee5PZsUphlNhHpHE3rgUCsdjlbHm2sq4yEAh2utDW4dXe9oDw30xCTrq4w
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: hsN3_z6dYYvcwHhVUFAi0XNIqqXoaODjZLtIYjJXbIzBtKAhYCslKt15eLqKNBn094MculL_VbApDzz7XQ5I3A
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
   ⏱️  Duration: 207.566208ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-markets...

Second call (cache hit - from cache):
   ⏱️  Duration: 39.460416ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-markets"
    },
    "name": "Leanne Gra...
   🚀 Cache speedup: 5.3x faster!

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
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: zSHrgZqDvPwFUJwx_7LgSdSSLszfpNprl092JMq6FFKjVg2PREZrYDuWo-CwNH6s5Eihh-237fyZCxXaokmulg
📊 Found 2 product groups
   Record({"category": String("Furniture"), "count": Integer(2), "avg_price": Float(474.0)})
   Record({"avg_price": Float(575.6666666666666), "category": String("Electronics"), "count": Integer(3)})
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

✅ Inserted order: Some(String("Ok4NG2G6EqtmMmfL4YwDeip_4o2fpj_boS_T6O1K8Q-Ihezgv-UNfGM6gFRFyCCWWM9ySs_AffgzXoqHSXTl-Q"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: aslCMdmM9904wX1DrH4MbG7cNb90rdajPCNhYbCyrtC-eyIMUvcjxP_pwTDOS789XNBLqJ-dxdhYnHp_QVC6oA
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: G6XLN9gj4KJMXDSXyn-mxXZjH1Hco3RtrAp0gnOrZxNHN3umwDM5Mvv_yv8Wa1l1XSeyGJgU9uooxQPAGkPCqQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: _qEMSC1CUlylAVDUXrnHQez1rvfqmf8YJmnVB39hsITC-CBEJRvgHK7_L3EtGicAyi_D3KXNHJHVbsj6MHgABQ
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (9GqVOlKNpyXWZRk8h5VEQLLTbOu8iwhslSQyZdKynNO1xL-7PR3EeFJ-xuzgYG7hZRAITMB36xnMt4mL_Lb6Qg)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (Ry8fX_zzYvknFE3q6pSX2v66ihmkhjcYlA2zrxNbyAvFiVXrKFX3yq18nwktV0j6JS6CKJ_4k64L_wKlRdk6kQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 144ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 25ms
  Speedup: 5.8x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (GsIDrf_w4DLbXDC2Ip_TcwxKgro2K6qy-Z7Vw2klV2bAQck_LpzzDL13q9-56FRC6iSB6oEw6ELW1kvUtt09Sw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (lGGhYbmfTXa5Fkcf5c108Y8wOBBnDxo5ZZO1W1dYUbR4NUtSEgx2S8_LBPHcYoz2iwcuKWYQFV-ZFXkQejpScg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (S_TnCSuOPoL3J6IShTIOe7ze1iXW6PnBz1Z_GWB2n9o-5pMm2HpBPSqOK7RDZUMdUOcDUR9k_9tejsfvfsHuxQ)
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
   Record({"category": String("Electronics"), "count": Integer(5), "avg_price": Float(367.0)})
   Record({"avg_price": Float(365.6666666666667), "count": Integer(3), "category": String("Furniture")})
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
   Vector databases offer several benefits, including:

1. **High-Performance Similarity Search**: They efficiently perform nearest neighbor searches for high-dimensional data, crucial for applications like recommendation systems and image retrieval.

2. **Scalability**: Designed to handle large volumes of vector data, vector databases can scale horizontally to accommodate growing datasets.

3. **Handling Unstructured Data**: They excel at managing unstructured data, such as text, images, and audio, by converting them into vector representations.

4. **Real-Time Analysis**: Many vector databases provide real-time querying capabilities, enabling immediate insights from data.

5. **Integration with Machine Learning**: They're often used in conjunction with machine learning frameworks, facilitating the deployment of AI models.

6. **Flexibility**: Support for various data types and models allows for adaptability in diverse applications.

7. **Enhanced Search Capabilities**: Vector databases improve semantic search by considering the meaning of data rather than relying solely on traditional keyword-based approaches. 

8. **Robustness**: Built to handle complex queries and data relationships, providing resilience and reliability in data management.

These advantages make vector databases particularly valuable in fields like AI, natural language processing, and computer vision.
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
   Record({"count": Integer(3), "status": String("inactive")})
   Record({"count": Integer(7), "status": String("active")})
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
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"category": String("Database"), "count": Integer(3)})
   Record({"category": String("AI"), "count": Integer(2)})
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
✓ Created session: Rfr0sJJyydcNjx_m5eGjQmvOoovWjppLeco4coWFC92v51wT_xTauDziN2zIy1rsoZA5Hm_iGIXay4z-Fzbjww

=== Sending Chat Message ===
Message ID: Pqq-HhiHAnZRvGort8AyyuLHhUmr5OCv0VUZa5RoVw0rZ-AD4IS_Ik-C_Twptw8TCvUjlx7-UJGX5LoCpojSQg

=== AI Response ===
Response 1: ekoDB is a high-performance database that integrates intelligent caching and real-time capabilities, offering powerful AI features. Here are some key aspects and features of ekoDB:

### Overview
- **Performance**: ekoDB is designed to offer high performance, making it suitable for various applications requiring quick data retrieval and processing.
- **Real-time Capabilities**: ekoDB supports real-time operations, allowing for immediate interactions and updates.

### Features
1. **AI Chat Integration**: 
   - The chat feature allows you to query the database using natural language. You can receive AI-powered responses with relevant context, enhancing user interaction.

2. **Search Capabilities**: 
   - ekoDB supports various search functionalities, including:
     - **Full-text Search**: Enables searching through text fields for relevant content.
     - **Vector Search**: Uses semantic search to retrieve data based on meaning rather than exact matches.
     - **Hybrid Search**: Combines full-text and vector searches for comprehensive querying with automatic context retrieval.

These features collectively make ekoDB a versatile option for developers looking to build applications that require efficient data handling and intelligent interactions.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("oqcA7vRyi93fmmLdWppgJIbq06T9kMYYT82mvuALK-vTyJuSW-V7zlh-3AymzKpv7-Rg9yI2EcSfbOIYbs1mjQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("OSLOuhNtxg7np4VXPRgTLdz0dQI7XhVYiMJgE8Z51CHFiCWMq0dGn0J-HrOSZ7-sxLAaa3qmJqGX08B7lbXsBA"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("dqBD5X8SclhoPiEiv85GCY0XA7XFb6QFkfxDJ-6r33L7EprrFJtjuvug1tcSngy_6jjjYqhW5pBhtIDZOnjLbg"), "title": String("Introduction to ekoDB")}

Execution Time: 10578ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1bQHbCIT8bwqjxWRSYCAnJ1ES4sWxtGpSCYFS8Jz9dUX2Z17L3F8CGZUiVEkdX2mRa3OVItLbQ4_L8syuOMhFQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or further assistance, feel free to ask!

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
✓ Created second session: dtK53BYTkysAiNRMWldOmIovwo_wnvgyHbNFMIfz9_pCa33c3eD0iQORSq9S1c2pWDBc9IrosRYBY20zvb8qmg
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
✓ Created session: gyNEy24WI3QCrsiEhkrIDESRQGTfXjb8K19yYzbV1N5bBqzVbySN-E2I81QMuassG-3hZTEl13vLPX-FKSHxAQ
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product**: ekoDB
- **Price**: $99
- **ID**: 9n51IDS5nt30_ecBA5-_yqrlbccg_joYANka1F2F9WbtvGei8bxhVeCmMcabK7v0V2n6PNb6p8hIEXUWbBQw3Q

If you need any additional information or more products, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 8UOiChCpvRXz8eWCYbajoZfp-rPHiibCwvrUjB1np9JhwSa8icdxUA1Fpvhz_6xnSyyGhlMSfddtFwgIBVPOAQ
  Parent: gyNEy24WI3QCrsiEhkrIDESRQGTfXjb8K19yYzbV1N5bBqzVbySN-E2I81QMuassG-3hZTEl13vLPX-FKSHxAQ

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: 8UOiChCpvRXz8eWCYbajoZfp-rPHiibCwvrUjB1np9JhwSa8icdxUA1Fpvhz_6xnSyyGhlMSfddtFwgIBVPOAQ (Untitled)
  Session 2: gyNEy24WI3QCrsiEhkrIDESRQGTfXjb8K19yYzbV1N5bBqzVbySN-E2I81QMuassG-3hZTEl13vLPX-FKSHxAQ (Untitled)
  Session 3: dtK53BYTkysAiNRMWldOmIovwo_wnvgyHbNFMIfz9_pCa33c3eD0iQORSq9S1c2pWDBc9IrosRYBY20zvb8qmg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 8UOiChCpvRXz8eWCYbajoZfp-rPHiibCwvrUjB1np9JhwSa8icdxUA1Fpvhz_6xnSyyGhlMSfddtFwgIBVPOAQ

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: ZhepzwwKqNi07E6d-6AEP-uKQ-QdT6Ujq6NWXgb1E6eoQdHJdqpO3qy5qF-d4Y2jVWco0gVGGGEQL62qpziSgg

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_rs: Get Active Users
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - get_active_users: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("fV-YCz5U-wH7jmNGf7DkslETkdo4MMoL4N_76hzSXlx9O6QX--VhpT6lQDkYkRJg1K1zUSYur7G8ua1KY2Vaww")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("RBC3cO8IT2cD1flatrdqAsH-QuFv6Y4VkqVftJtAgJA0M4Yqu_Z6BI9apBaQ_meG0r2HJM1PXjPUoo10xo57nA")})
✓ Second upsert (update): Record({"id": String("Sk4yczRXUFV1xHfd7LtvRdWD7fzuoM2Mj14u77_5Gd396kI_K0xcwK-F6jo51qxG427WtOT_Ooqo56iuqw7qLw")})

=== Find One Operation ===
✓ Found user by email: Record({"id": String("fV-YCz5U-wH7jmNGf7DkslETkdo4MMoL4N_76hzSXlx9O6QX--VhpT6lQDkYkRJg1K1zUSYur7G8ua1KY2Vaww"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "email": Object({"value": String("alice@example.com"), "type": String("String")}), "name": Object({"value": String("Alice Johnson"), "type": String("String")})})
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
   Inserted with ripple: Record({"id": String("DBMcDqaBn0PjBJyiEmF0OJ-b1356Git80nuK6urjAzu1WrU36Mv_jYVyCn6sUfG0HlfXDRCvxeywKZ7dURX-iQ")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("gchinfVJN7hxoc-kg0uA0coeXje5KrBD_lHoNyzSmhM6fRHfdTlTC4gmFwj1rAb8FzROhf2vD8sOL4abZe51eQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("DBMcDqaBn0PjBJyiEmF0OJ-b1356Git80nuK6urjAzu1WrU36Mv_jYVyCn6sUfG0HlfXDRCvxeywKZ7dURX-iQ"), "name": Object({"type": String("String"), "value": String("Product 1")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("Lz8q0X4YJ7Ci_axEa1eKye4c0w8X2t9Vd_r9FyG-OfgLwQ_iPfjKAvbMopFWwuy_bhNXZCUe1DNo_51NlfDtWw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "email", "name"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["id", "user_role", "email", "age", "bio", "name", "created_at", "status", "avatar_url"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["id", "user_role", "email", "status", "secret_token", "created_at", "api_key", "name", "age", "bio", "avatar_url", "password"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "name", "email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
