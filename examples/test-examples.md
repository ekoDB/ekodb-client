make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '6S9N7IIuTqqLdeHFcjGKjtf2UTX5m10knQGujwhzKC_oXG949LojxqOa9S1Gu_q9fuRm-4gBiD1cBxg19ysETg'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': '6S9N7IIuTqqLdeHFcjGKjtf2UTX5m10knQGujwhzKC_oXG949LojxqOa9S1Gu_q9fuRm-4gBiD1cBxg19ysETg'}

=== Find with Query ===
Found documents: [{'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'id': '6S9N7IIuTqqLdeHFcjGKjtf2UTX5m10knQGujwhzKC_oXG949LojxqOa9S1Gu_q9fuRm-4gBiD1cBxg19ysETg'}]

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '6S9N7IIuTqqLdeHFcjGKjtf2UTX5m10knQGujwhzKC_oXG949LojxqOa9S1Gu_q9fuRm-4gBiD1cBxg19ysETg'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: fQW6SBGgSvKuNS5AKtPgbx9f8znVSScfJfJFeZectmHoGisu5TIZw-ftWvtZkUwKd3podcIrg1_b_HMlYHimDA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "20246520",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "fQW6SBGgSvKuNS5AKtPgbx9f8znVSScfJfJFeZectmHoGisu5TIZw-ftWvtZkUwKd3podcIrg1_b_HMlYHimDA",
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
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 2Iqh9SsdncTNBnb5iotzQfo6tg1FbJtDY_EEgtPN5X5wuEsuivqsDW7vbryXUYNQBM2D1uChm2kr0TTYtzJE5w
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: TREBBKcC6YcPe7UDtl1InssjcshYF_NGfKKnoirU_yozCvMo5F0DDZCJNEuSOoO67XzxaKWWCHeEB8oG8bXKCw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: gy0OinNwg5E03Kgcs4vCQQ9qfNl_oryoXxqeAlbjK8hhu6YSkMp4XGCFZK7LKqOLldstI9P5cYInzxLGq1HaaQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 100, 'count': 15, 'avg_score': 60.0, 'status': 'active'}
   {'count': 15, 'max_score': 90, 'avg_score': 50.0, 'status': 'inactive'}

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
Collection created with first record: sTI05o4W8wVVR-rEAd_0YNAHF6IHymkRlWG4CVq1PdT-FdzGjuCfWUtcmvJMozcT6KFoz9nRobGDjwvb494P0w

=== List Collections ===
Total collections: 13
Sample collections: ['ws_ttl_test', 'audit__ek0_testing', 'products', 'batch_users', 'ttl_cache']

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
Created Alice: $1000 - ID: jVVWBZLCMriVKKacyR_3nGNj20pFdbAe51WzwM5JrSxI5qQLW_LlKAAmkgUpQ4ZMcwigZgAixGJkIhZGC0ETKQ
Created Bob: $500 - ID: 6IFIeEPFcnpfqdD02dHCF0Xsy5XTPJVICu8xWyWlrb2HeoRKolG0zQkF6fzOSEXOex9fpCa31dHd3fMTKwxRnw

=== Example 1: Begin Transaction ===
Transaction ID: 961cda7b-5bb6-45a8-9445-7e9ca30ce875

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
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: 609dd2ed-e1bc-4b6b-9e9b-6df261d2aa4c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 4A7_JNzYfiGrR8QXYjeS42A-WdWCaWVnqRvarfWKfC-QOL7Zv3aTnkXMWs4er8eXyize-g9_EoJU5e6PcmQ5kw
Created Bob: $500 - ID: LEeI7tJRunOpr58EDQfjTlCXMUlp-f4WaP6fb7fojyjisoW7JIqp8jwk1jiQtA44-NaI-yn5XtrI-7WsEFDyew

=== Example 1: Begin Transaction ===
Transaction ID: 3c961792-bd5d-4d48-baec-a1c1ebfc77c8

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


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Xa1ctCz6MhBGppd_WsKuggaZV1usmKQes_R-bOJS_LCTt63t2XX0fVmqsvEJZIooFyae35rnYdqJoRPHYiP23g
Created Bob: $500 - ID: mbTQlOYHcmFygtzsmPsmZYWXPnvhaudPhKe25bIl6FxfiA6BnJMgRFLV5-08mnUaVl660jrqJV3gw6_UvhFEMg

=== Example 1: Begin Transaction ===
Transaction ID: ab8da08f-cfd0-44a0-9841-90160db309ce

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
New transaction: 346efe27-8ba8-4075-afd2-a18ca31dcecf
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: V71RoGUOU3q5R6quQ-eeFOigPJm-bMJpyl-Brss4DSgJ4eBe3cU9JIXOEqwdQwLiNaKu0U_IT22jR0KoZ13B6w
Created Bob: $500 - ID: ktPFxVDpE24E1_dJFZeby_1HPFtySSBz5glaYEIbTINq37EP7NRiHlIaS7j7VVdVY5XQ4MjIgOo0PHLAEWnSeQ

=== Example 1: Begin Transaction ===
Transaction ID: 83d021f5-8a26-4e49-8355-96677f3e5ace

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
Created Alice: $1000 - ID: HvZU5vEr49SXA_fx_HxmIte6pNv9yaRRRvMsaMDrCFNCy-rFCUui5tqdinJ0_p51fsa_naEVcfHJwoZNyy59Sg
Created Bob: $500 - ID: bOGCBjghiaLRZSU_tXB7PfFkPbNZ8G4R-jFCIOjkjseZyiPFVAI4F7Y_zNKdGl8WTw1tYlq6rzhkaHh_dfbylw

=== Example 1: Begin Transaction ===
Transaction ID: c81c1940-8b24-4458-9142-00edd3f6722a

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
New transaction: 5befadfc-34f3-44f8-9a99-fe51f7d585c5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: IwgMEIvRHV8G65ZCnhVBMywZwWmIuixRbsnSIS1j7tHuD57po5s7YOJCc9k5czvLOgPkDuQ4JCllwv9_WRO3pg
Created Bob: $500 - ID: lJOleciyM-ol0rMbIwawnlFmVUhPKwN6Xxt8cpl761XDOiqDIm7iA8sbOvjYW3rWzOTBVdRI11f05fhKlmWo7w

=== Example 1: Begin Transaction ===
Transaction ID: 9488d70d-1e2b-4c7d-ba16-651b717c7819

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
   ✅ Script saved: nXbOVRT8lht-8eI-3opmAw9N-X26M8sui6PelJdskJVdDregEQ-qOzdFRwvRlAQlnFgshiMws-h_L1Xfa0fWeQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 8hpsxUcGZJodOf-wx_2jInvYvobY2QJQi7lF7cc8xshVl5e-h7-8UoGnsFinZlEeCrulRyGDS60Qq-iiiqY76w
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: wNd3YU9v4U0k_hOdbQIch4dwwEV6DzNoPu8UVbIs3G2rjSQaHEkv84bGRA10uFmE66cuWYots6DXFK2QZleZ2Q

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
   ✅ Script saved: -5-UmGAMbVwxjDErlXukUsAjUpqvJ_-A2cTO7sPY1zqjV16hnzZXWMkSdnSRO1ssXHfUrkWPUFlo7LbeKNWQBw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 7OBTuCtfIVyhtj4CyKfc4EV0LAR8nRf1fSvZ7vd6JnGmKyc2dGNriTlXBMoW7CWR56lweMpNsJ3kz1XiPnqC9g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: nXbOVRT8lht-8eI-3opm...
   ✅ Deleted script: wNd3YU9v4U0k_hOdbQIc...
   ✅ Deleted script: -5-UmGAMbVwxjDErlXuk...
   ✅ Deleted script: 7OBTuCtfIVyhtj4CyKfc...
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
  Output: Document ID = kEeFFLcFgwmSKvCNJu-42MnMrlOJgQWBY_DZdRdosfwvSvYKwPpxnBRiMPR-etoUMYich7JxoPbFuR85segr3Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(kEeFFLcFgwmSKvCNJu-42MnMrlOJgQWBY_DZdRdosfwvSvYKwPpxnBRiMPR-etoUMYich7JxoPbFuR85segr3Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(kEeFFLcFgwmSKvCNJu-42MnMrlOJgQWBY_DZdRdosfwvSvYKwPpxnBRiMPR-etoUMYich7JxoPbFuR85segr3Q)
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
  Output: Document ID = rzyLOvjJ5Mpv18hsDje9qbZl3ApZyYSHEeg7O_w_AttXlCHukYP_6ERFZ4TGtx018Y0xIIYaJbNsZ5R6qzyPSw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(rzyLOvjJ5Mpv18hsDje9qbZl3ApZyYSHEeg7O_w_AttXlCHukYP_6ERFZ4TGtx018Y0xIIYaJbNsZ5R6qzyPSw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(rzyLOvjJ5Mpv18hsDje9qbZl3ApZyYSHEeg7O_w_AttXlCHukYP_6ERFZ4TGtx018Y0xIIYaJbNsZ5R6qzyPSw)
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
Inserted: {'id': 'Fwx0q4m7idTsv67GYT9Cw2D-VQ4T1Wi1TEuV6bEOAIz8Y-fuNdTMEYydJp8l_MqKxcTx7475kqdmlWrwNgvrHA'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'price': {'type': 'Float', 'value': 99.99}, 'id': 'Fwx0q4m7idTsv67GYT9Cw2D-VQ4T1Wi1TEuV6bEOAIz8Y-fuNdTMEYydJp8l_MqKxcTx7475kqdmlWrwNgvrHA', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'created_at': {'value': '2026-03-28T17:45:45.242501', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-28 17:45:45.242501
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'name': 'Test Record', 'price': 99.99, 'id': 'Fwx0q4m7idTsv67GYT9Cw2D-VQ4T1Wi1TEuV6bEOAIz8Y-fuNdTMEYydJp8l_MqKxcTx7475kqdmlWrwNgvrHA', 'tags': ['tag1', 'tag2', 'tag3'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'created_at': '2026-03-28T17:45:45.242501', 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers']}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'value': True, 'type': 'Boolean'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'value': {'value': 100, 'type': 'Integer'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'price': {'value': 99.99, 'type': 'Float'}, 'created_at': {'value': '2026-03-28T17:45:45.242501', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'id': 'Fwx0q4m7idTsv67GYT9Cw2D-VQ4T1Wi1TEuV6bEOAIz8Y-fuNdTMEYydJp8l_MqKxcTx7475kqdmlWrwNgvrHA', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 2uiRVSdq0YJiHR0nO_ARlSIaQbrxzBLcWyBxz2hYq2kHQ6PQyBiTG4hvPbFx1FxZPoUwIPZB7wHTc76hR1tsOQ

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
Collection created with first record: "rI0GJE__97QGC9056MmhHBXmY4lOmGkhmTCiAWbhUxhRmjO0mMrMK7P1vFWUbYdt7Sc2jo6ZQ6QuX6LQ1dPe5w"

=== List Collections ===
Total collections: 12
Sample collections: ['ws_ttl_test', 'audit__ek0_testing', 'products', 'client_collection_management_python', 'batch_users']

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
Created Alice: $1000 - ID: o6zolw3BE4TxYYPQySupzilSKdemFZ6ulIa9vdkxTuD_YLcb6o3Y8amX0haWL5xDHecvQ0qDQvDgHZRlRpGcCA
Created Bob: $500 - ID: 85XtuZuw-QZ8Qo-4HoNtIBcordJgjI15vORjeA9F-UwGdgaxBuRLv0R9UqYTh8UU8y1MhTSrPU2SN6xe2IXS-Q

=== Example 1: Begin Transaction ===
Transaction ID: cd53eb68-653c-4344-9cef-7e18d8fd6af5

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f9375919-9977-4a9a-b28e-0ff0f2e2fdbc
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
  1. Score: 25.740, Matched: name.value, email.value, name, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, title.value, bio
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: title.value, bio, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, bio, title
  2. Score: 39.600, Matched: bio, bio.value, title, title.value
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: title.value, title, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.755
  2. Score: 0.747
  3. Score: 0.740

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.370, Matched: content.value, content, title, title.value
  2. Score: 1.377, Matched: content, content.value, title, title.value
  3. Score: 0.374, Matched: 

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
✓ Inserted document: httLk51BHC3Ea2WvLn6xuKRLa9VcAOvgIymVCaZ11jHBqgFPEWhYAC5W1j3gGMBCQohO7tvZLgpqcYOtLnc44A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: CElVjWKUB7A7r3RXpuoVh27W8IVFrqGE_uqYDVhXC7cHmd3MiaJQDBOjjgeDz8Bhx9QmCWfnA6BwdhwfuneQ_g

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
✓ Inserted document with TTL: JjpnWFlqcCV6QohVX_E7LcO6WY65uHtrJkkPk9hzPTsP2YOchmJP-pdTmUQ2xNbidXon6hdyJCobArsznXpvOQ

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
✓ Edge cache script created: wjWH3dn6SGZkTaxDq7BcGVeBwma0WNy1d5DHDf_AZvMyOVGgY1BDwDMvbPw6uXT8xXBle4YgUOlw3pLLhiWhOg

Call 1: Cache miss (fetches from API)
Response time: 720ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 5.5,
          "time": "2026-03-28T21:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.022292137145996097,
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
Response time: 41ms (17.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 5.5,
          "time": "2026-03-28T21:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.022292137145996097,
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

✅ Script saved: 9yV8VoV5QPp2xAtsMLjUq0vDNlRnLG0-Kjdw81PtHo_BXUpHdfLMtTD3QJjyLSCvO53S8EZXIV-GMU-Pnw8ePg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: oCbxCaUIc3zuKO_T4TV7_JdUG0r3dveGPN0d5A5oiMtBC6fjo1wX6Uh0QOSk1IPDn77XU5PltnHmv-meaTZTow
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 5HYUlIdaz8K6iYFha37qaIIyJShQmBWE_vw5tx9C1KVUAGSdVpmLRDWlQu_fCRiJTvLTSMjimRX9EdSEFHibZA
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

✅ Script saved: mNMwvTSpr5uzzTLAD68iMSgQlmiX9CoP0H5CPFV-d_lNE_rY1nucx7CWzs7m8ovgUR8N5aRB0FIfgWQo5DVBKg
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
   {'category': 'Furniture', 'count': 2}
   {'category': 'Electronics', 'count': 3}
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

✅ Inserted order: BGwjya0qcitnfitDCCgVooL6TvKMn7wSB5cSwwfWfpq_hQHnyRpR-2XTMELp73hFA9XFFB4lVLEkEfCNrDiajQ
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

✅ Inserted order: ACZH3fpDknVU0Si8hm1zxiYGQtH_XdYToWlG19pD_s0XdbYmg26la9NUmNHtkuIZroaAWep1ovYWBWEkAV5xog
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-03-28T21:45:48.142210+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (9NJolS3KOISlUQbOFDeICvUKuHn7sn7cpEmv-OVSY8enIY4iN-J6MKO_GmOaoVJhyQZRG0dzCPlCHBo4pC8kOA)

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
✓ Created native SWR script: github_user_native (1rpDO0Lx37c1En7K8XlUgK_npX-GypvUvQnE2gRMtwX31AN2ivA82gURD1-JcuJ6mc3hajH_vK-62_wVXtF6tA)

First call (cache miss - will fetch from GitHub API):
  Response time: 123ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 5.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (G0RaPFc6ZJdQOL1ZnEhRBlFa60Kb2rNjqwZRLohvSczau4po9Buw7v7j6QbQZYlkWGR7svuUWrYUvSOTK9rrQg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (jji1guiyJ6EUBek4EVtE32rZjl5KWTEbWuRjjMPAw0FR_Dil4nEsLbo_tHUj-bclQWsx_sviESgNpgDp-YzeLA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (KBe0nfjL3Zv77Mua0A_CiPts51PES6YLbjXKsPF_FlSiKjtiai7omgKWFQVqNPmGkJdiXUszS4LnDtxCheuVHQ)
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

1. **Efficient Similarity Search**: They excel at searching for similar items using vector representations, making them ideal for applications like recommendation systems and image retrieval.

2. **High-dimensional Data Handling**: They can efficiently store and query high-dimensional data, which is crucial for machine learning and natural language processing applications.

3. **Scalability**: Many vector databases are designed to scale horizontally, allowing them to handle large volumes of data.

4. **Real-time Search Capabilities**: They often provide fast response times for queries, suitable for real-time applications.

5. **Integration with ML Models**: They can easily integrate with machine learning workflows, facilitating the storage and retrieval of embeddings from models.

6. **Flexibility**: Vector databases can handle diverse data types, such as text, images, and audio, in a single database.

7. **Advanced Indexing Techniques**: They utilize specialized indexing structures like HNSW and Annoy to improve search speed and efficiency.

8. **Support for Vector Operations**: Many offer built-in support for operations such as vector arithmetic, making complex queries more straightforward.

These features make vector databases particularly valuable in AI and data-intensive applications.
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
   1. Vector Databases Explained (Database)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'Database', 'count': 3}
   {'category': 'AI', 'count': 2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: oSTbHI3HIfQPb12V5a71f57bJiy6odGOwleAbeB35d0aLyVVKHQDLIJaJDqfBEi770Qav9ZcpgPyr9k7AywD8A

=== Sending Chat Message ===
Message ID: jd1CrAdIVspDwjvTtC0XS14vV4M2rfvCoy7VxkLg_FBEyW5vhc7AhsetFZHQEvo8hGGbG6zoCD0utiCMHeuRBg

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

2. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

3. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

Execution Time: 2245ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 89
Total tokens: 1387

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dIbCSxMLcmrPoG3nbNPQW4gjqIZNxPtO6e2UED3r_Pf17GKPr_cvFft2SdYJy_y4l-GB13WWUnNKkoMHX7RLsA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Price**: $99

If you would like more information or additional products, please let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['token_usage', 'content', 'llm_model', 'updated_at', 'context_snippets', 'llm_provider', 'chat_id', 'created_at', 'id', 'role'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: jaaT3Vfrj3B45QAf8_r32GkVnIiUv6kshpVNj2mn7m7EavDP9Lsw_ae_nu16Y5DI3ZQ8BiHlV3Q4VbHiytX-NA
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
✓ Created session: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99 

If you need more information or further assistance, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: l6R5rQcWaI_JzS252WqlnguXi418Kost6-_MeDwXsdZnkp9yzaNCGO2TFbR7tJoRL3koCveZNEyC8mpkGbKBOw
  Parent: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: l6R5rQcWaI_JzS252WqlnguXi418Kost6-_MeDwXsdZnkp9yzaNCGO2TFbR7tJoRL3koCveZNEyC8mpkGbKBOw (Untitled)
  Session 2: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ (Untitled)
  Session 3: jaaT3Vfrj3B45QAf8_r32GkVnIiUv6kshpVNj2mn7m7EavDP9Lsw_ae_nu16Y5DI3ZQ8BiHlV3Q4VbHiytX-NA (Untitled)
  Session 4: oSTbHI3HIfQPb12V5a71f57bJiy6odGOwleAbeB35d0aLyVVKHQDLIJaJDqfBEi770Qav9ZcpgPyr9k7AywD8A (Untitled)
  Session 5: gyNEy24WI3QCrsiEhkrIDESRQGTfXjb8K19yYzbV1N5bBqzVbySN-E2I81QMuassG-3hZTEl13vLPX-FKSHxAQ (Untitled)
  Session 6: dtK53BYTkysAiNRMWldOmIovwo_wnvgyHbNFMIfz9_pCa33c3eD0iQORSq9S1c2pWDBc9IrosRYBY20zvb8qmg (Untitled)
  Session 7: Rfr0sJJyydcNjx_m5eGjQmvOoovWjppLeco4coWFC92v51wT_xTauDziN2zIy1rsoZA5Hm_iGIXay4z-Fzbjww (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: l6R5rQcWaI_JzS252WqlnguXi418Kost6-_MeDwXsdZnkp9yzaNCGO2TFbR7tJoRL3koCveZNEyC8mpkGbKBOw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'OIsNQ4WzMpKYiUHJXY139bxFTIcNsNhqC1B8hv3e9i0zAWIR9CANmZBkHNymleZ74ies_i3m3p4Ows9Xcyi_Zg'}

=== Upsert Operation ===
✓ Upsert (update existing record): OIsNQ4WzMpKYiUHJXY139bxFTIcNsNhqC1B8hv3e9i0zAWIR9CANmZBkHNymleZ74ies_i3m3p4Ows9Xcyi_Zg
✓ Inserted second record: x3X_DbtCSJ22f83y_6PK9XSlA8Uwb9E0WrNL0m3mtkZQ6H5J1_gNE7zrXqII8BKHk_029-LM_iuD23cjssCNXg
✓ Upsert (update second record): x3X_DbtCSJ22f83y_6PK9XSlA8Uwb9E0WrNL0m3mtkZQ6H5J1_gNE7zrXqII8BKHk_029-LM_iuD23cjssCNXg

=== Find One Operation ===
✓ Found user by email: {'id': 'OIsNQ4WzMpKYiUHJXY139bxFTIcNsNhqC1B8hv3e9i0zAWIR9CANmZBkHNymleZ74ies_i3m3p4Ows9Xcyi_Zg', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'active': {'type': 'Boolean', 'value': True}}
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
   Inserted with ripple: {'id': 'VUZ3qg7qw4nMfyCgWMiZrxrr_y-PGPm0N_OIr-c2leqzPYNpaK1rknty7tVi9NPKmiiWyx6ILR2uPwETVRXf1Q'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': '8WKIJZZKmtJ6QaKv7beRcNMI4_rDZNiNStMPUfo8fIGijd8PqwZgIapw4VEJgcZK_wDV6h7-3iXdNqZHPe7NDQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': 'VUZ3qg7qw4nMfyCgWMiZrxrr_y-PGPm0N_OIr-c2leqzPYNpaK1rknty7tVi9NPKmiiWyx6ILR2uPwETVRXf1Q', 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': 'VUZ3qg7qw4nMfyCgWMiZrxrr_y-PGPm0N_OIr-c2leqzPYNpaK1rknty7tVi9NPKmiiWyx6ILR2uPwETVRXf1Q', 'price': {'value': 500, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Upsert Product'}}

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
  Fields returned: ['avatar_url', 'email', 'user_role', 'status', 'age', 'bio', 'id', 'created_at', 'name']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'value': 30, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['password', 'created_at', 'age', 'id', 'name', 'email', 'avatar_url', 'user_role', 'api_key', 'bio', 'status', 'secret_token']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'id', 'name']
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
Created user function with ID: 6fuHOb3-Z2HNrrxGlaj_bBTtENotSBmia2tdBvBKEUvLQaZWp8b01kcgJuo9vUI13i7lilROqo-9QvyZBLeO9A

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - get_active_users: Get Active Users (Updated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - cache_api_call_py: Cache External API Call
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - get_verified_user: Get verified and validated user
  - get_active_users: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_py: Get Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user

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
Inserted: map[id:0zZaN4N7TkGo8zTBDyWg1BMuCsvdGaTrrnTebnVFfWbinCj77ivRsaDPIxjasXwE6PN81ClmvkLk7HUSYamU2A]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:0zZaN4N7TkGo8zTBDyWg1BMuCsvdGaTrrnTebnVFfWbinCj77ivRsaDPIxjasXwE6PN81ClmvkLk7HUSYamU2A name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:0zZaN4N7TkGo8zTBDyWg1BMuCsvdGaTrrnTebnVFfWbinCj77ivRsaDPIxjasXwE6PN81ClmvkLk7HUSYamU2A name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:0zZaN4N7TkGo8zTBDyWg1BMuCsvdGaTrrnTebnVFfWbinCj77ivRsaDPIxjasXwE6PN81ClmvkLk7HUSYamU2A name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: ZODatZcbUzL0VY5XwPCFMuuff3xb33htMyj5AetliQ0GoBfkKJNfjwrL9qqem_ISoiT2b-KFCkj1wZV2c4g9yg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774734391680845000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "fQW6SBGgSvKuNS5AKtPgbx9f8znVSScfJfJFeZectmHoGisu5TIZw-ftWvtZkUwKd3podcIrg1_b_HMlYHimDA",
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
        "id": "ZODatZcbUzL0VY5XwPCFMuuff3xb33htMyj5AetliQ0GoBfkKJNfjwrL9qqem_ISoiT2b-KFCkj1wZV2c4g9yg",
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
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: xmObR7D8NZe0QnoEnAwa0uOxq_jHi_ZaJdzKz3mzd2qYkdKeybo1-QT-WOHZsGPcIY9G5URwXrMEzbXKQlyNeA
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 7WjGPJhqKFNW4dcFhHuRMElZ2yH2S2bR5Z8xL7Z1gihPrx_3tQ65DeWoZfWzO1UG5FbGkXyDeXTjCOWhIaRKoA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: U1Xk_HGx_iwe2oV4oLvSa4aL-0rJcyfcOrno6sQTUy6Ew2c6GLBIsvDPI9OAj-9VVOfJcuDld7HKVYFnhyGCAQ
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
Collection created with first record: p03Mdx5fkr3vkHldIPVRWcqt3UF7dVnhRmZV9pG1J28YKcNxNdOgiVashGNn0gxK2wW727RkrK1HTN26cFcm0Q

=== List Collections ===
Total collections: 17
Sample collections: [ws_ttl_test audit__ek0_testing batch_users ttl_cache chat_configurations__ek0_testing]

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
Created Alice: $1000 - ID: sqcOKOsS2crNtPHfVrytJGsSSLXzKBfC5JInhUZ1TODDNWjB3qTMcZn9jS5PC86ZG2oTRxftzOan4LaSmFO5mA
Created Bob: $500 - ID: rNf0KV-QWXzlSR75bhaMr-mPiskRIU3JTlDhtV2EWMr_cVJwQeHjA300Bs8km4CvY_59w3_r9mI_Ycar4GZq4Q

=== Example 1: Begin Transaction ===
Transaction ID: c44e2ca3-0479-44e5-b8f8-6f824dbbb620

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
New transaction: 021a0723-2c2d-4511-881b-b3447bce7dc9
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
   ✅ Script saved: -xqCN9xPbN6x_uziK5ugB_kDx5-Zy-nQGxUa-HNoya6a6Ni6LrWro0S2utvdxwXnG0jInqv1hUDZo0HFY2Tyqw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: nqQH00H5wXGyZnJwSF7SEYpwAxiKJ3z_NWVpAnhSfdfyhPXB75rwgH4KsdMOI5DIauI_gVzTIGOD802uQxJzVA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: JvOs-H1pjH04bChCqa8UkDIS3NUMkPEKRoRzsqyBI6jNeAf4uNAC7YioAnhuUcEkqg7-QK80rRomSp0aObEBIw

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
   ✅ Script saved: _jqC7c2CMsSPTv3WOHoPoc_LacsfcwHociiup9fXJq7tChD50nD1NAJEiQ8Tr91wYSkYgRGjx061wUscReOFTA

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
   ✅ Script saved: xYovCLPb_Fgs9XTAWRcTDxdjTq-UYfSxfod78fDOIdbeBFLnMu9jIYZE5zWHFHQQCmcw_a3nwkEostiTNPvw6w

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: -xqCN9xPbN6x_uziK5ug...
   ✅ Deleted script: JvOs-H1pjH04bChCqa8U...
   ✅ Deleted script: _jqC7c2CMsSPTv3WOHoP...
   ✅ Deleted script: xYovCLPb_Fgs9XTAWRcT...
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
  Output: Document ID = iHIWhr4mc1rsj8bP9gIdBqBbVGHgiLaQ181GM4WqNrzjhh6FZuvEV313rkCWv3rg3Ojxl8Zz0boi-xNhD_xTJQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(iHIWhr4mc1rsj8bP9gIdBqBbVGHgiLaQ181GM4WqNrzjhh6FZuvEV313rkCWv3rg3Ojxl8Zz0boi-xNhD_xTJQ)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(iHIWhr4mc1rsj8bP9gIdBqBbVGHgiLaQ181GM4WqNrzjhh6FZuvEV313rkCWv3rg3Ojxl8Zz0boi-xNhD_xTJQ)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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
  Output: Document ID = r7P-FftCtKtLMY9jle5DKwB5cVNmNH7HZ5FccUJ_jZ1kvS4uSJLUFCNMfhHVm-GX9B6Ac1CkIycmZjWMUv2ExA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(r7P-FftCtKtLMY9jle5DKwB5cVNmNH7HZ5FccUJ_jZ1kvS4uSJLUFCNMfhHVm-GX9B6Ac1CkIycmZjWMUv2ExA)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(r7P-FftCtKtLMY9jle5DKwB5cVNmNH7HZ5FccUJ_jZ1kvS4uSJLUFCNMfhHVm-GX9B6Ac1CkIycmZjWMUv2ExA)
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
Inserted: map[id:uellK-te-vvuokbNC421Cl9zrHSmntrB4-6_ied6Y4m0-53fx2sHbx6Yqb235cgxRWBerHTD-O6rG5JgVuc0Sw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-03-28T21:47:01Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:uellK-te-vvuokbNC421Cl9zrHSmntrB4-6_ied6Y4m0-53fx2sHbx6Yqb235cgxRWBerHTD-O6rG5JgVuc0Sw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-03-28 21:47:01 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-03-28T21:47:01Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:uellK-te-vvuokbNC421Cl9zrHSmntrB4-6_ied6Y4m0-53fx2sHbx6Yqb235cgxRWBerHTD-O6rG5JgVuc0Sw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-03-28T21:47:01Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:uellK-te-vvuokbNC421Cl9zrHSmntrB4-6_ied6Y4m0-53fx2sHbx6Yqb235cgxRWBerHTD-O6rG5JgVuc0Sw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Q_r-fQbOo3gtuQYMaHdcpcu8kjw9glj7dbDNUoS4gBfdwPbl9jkHJ64byiLi-xlUliohrvFfyd8T0pTI6X3iig

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
Collection created with first record: dU2YEhF84HngPeTwWg2kVgMtX_X6s6V_pGF3WnHfv9pjXhXJk79hcVgiWIiegjqQ08dpxWirKOe5uh0iKOnFeQ

=== List Collections ===
Total collections: 16
Sample collections: [ws_ttl_test audit__ek0_testing batch_users client_collection_management_go ttl_cache]

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
Created Alice: $1000 - ID: 6YnxOaJCIk22vsOtQphQpc262BIspih5AYll2byg9jKKI8_sUkV6_UPHwKwqH6Kz8CnV5IaDv04uiL0YFhhOig
Created Bob: $500 - ID: aMpw8vmgWL1i-zqpeT-8LE192QH_2I3Tp2UjHlwMvDs9TA5RTx76mdfGMoVmBJv2vDak8resY6uQ2ecbJlAq-w

=== Example 1: Begin Transaction ===
Transaction ID: fb8f08b5-a0f0-4b6d-87ec-9c20b7d26c05

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 63f8e5f5-e48c-4137-9331-7f4d8786949b
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
  1. Score: 0.769
  2. Score: 0.746
  3. Score: 0.745

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698
  2. Score: 1.498
  3. Score: 0.308

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
✓ Inserted document: htHG0EORX6NLjHGtK0QRq4HSOR6Nwuc-jX4edUzLWbOMIKMOaT2kEo_OKEftNP1sCYGfkR9wZBUHT-7SwoBW_w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: uUKKBRf4Nk_4tEoItCBJkPG7DPnfar2XQq46HNBlw7TBA-_ufOWkiOvCOk0RJqLkY7X_Dxl2dOsjfu1UJyqnPg

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
✓ Inserted document with TTL: AHjV1yfBYY-j-I22NRGXqTkQZtfG_eR1ofGOfsQX7dGhqkoVnZN29wYMtZXhFPgHZ5fuvHV4EpYkUedbrUDo_g

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
✓ Edge cache script created: zh0bAuSsI4rURqKCkoNTgJdfJUTPfg_DBGCWQ2rTRYxgt5dO2GmD2q1hT4H2FBIURCFboAqHqfrkn7epZtQNaA

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

✅ Script saved: QxrXcKzoUikf9jUoE3P6e9JvdSlsNQmiNi9i8r2nyAH_dty1ccOM17p0NjFiyeP-BBCpAVoxBFDzFYOstsNlIw
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
   ⏱️  Duration: 38.2385ms
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
   ⏱️  Duration: 38.947042ms
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

✅ Script saved: bz4jqGT8R3tV076n5insmSdH61q13BfzHiOCD-8nMkgpu2cGIvJy6v5ExzjBHZ2kY2dr-Nt46tDqxJ_pWqL00g
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

✅ Inserted order: 2JFGhRUihvfuXczaj4XKNYhWZrXYgzTLdvx89kPE2W50hlepiNC2Qg8ucsiwD0CBfeS-Ww0ZNA4mq5QCoxFQuA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: kY7XVOrYpwK61le5L6xsTTtavNKskSzuzQLFfX0gbTLK4Sh3AM1J1Yc_wLWM2Fm3sBIRo9130bWGc2xvay4dRQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: zRFBM12KxveAvXGJ_CaMe0cMz-Dw6fjJIem8MKAS8K3UXWhZx3UV45eOwBIEjWIETaBCOAaPiKbS-FJYxbLQYw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: m6XGLUsfCfOtogZOQII63Rk5TRJIW2-IiRYc2qkwZpoXmKviIfz-Uq8NKrOV75us3u02Q2izlF_L97PX3WX6IQ
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
✓ Created SWR script: fetch_api_user_go (mRaI7nGmWMuTgOFDL6VMgSZjQ9qXSJO8eF-wyWec0CdT-7DIR8gXZ6_G5nx0lK5Rnoy9-h_7DiDNL8w8cuIQEA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-03-28T17:47:04-04:00",
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
✓ Created native SWR script: github_user_native (rbD8hW0d8cXYihWH1ydGxlEjFXt0ouBLw1PM87D4DIO2HO78bh3EwW1Q79zpT4WhRXO8k_5KKfbADVftpzsbrg)

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
✓ Created SWR script with audit trail: product_swr_audit (uMWgcRP-PWfboLlPq320E3FblwWfCRCqhbCgiIjsNHUpwAONq_0C6ezv4LSbxWKyvOkfohywJYDOCq5fY5I9Lg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (FiwlUrcZrK7Dzq-KBmSbZwn8sLSlgN29klopOibVYkal6BEWnOSX_liGDfHe0SEaDRW9sI80USHtCyafPpWsBQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (NuN7LIvQAEAkLg8O8lssrHBA1Y9gnhvVBAh7mUMna_wNAg9x-xS64HS2izzabVh7QL0z3k9brIF6CXqzIFZxNA)
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
   1. map[type:String value:Database Design Principles] (map[type:String value:Database])
   2. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   3. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   4. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
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
✓ Created session: pzOmm3GrdXAkvNrQvEDpLh7CvHMf-1VaM0kvPSika5LbfNwnJcNf5IQaEpCvYaancEPji9cden_C4jNJTfDDcQ

=== Sending Chat Message ===
Message ID: OqaK7LLhGPiGQapOQoVGJx-E00yKWOTNqqvG1WjvrMIhVvSXcXc_r_9cz4LnVdQliqRPjWWERkvEWm459Wa0cA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB** - $99
2. **ekoDB Pro** - $299
3. **ekoDB Cloud** - $499

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:qtxUDSr_MCC8fIbZMa5MxbiGmmFwwkaGXTmhnkcJrNhg3A7VUj376kJIJ6ZrgVSYfLShbHvsWO7mSIQrteRfjg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:FIoZ47C4os662MlxA3pSNCZQ96BVasH7LkC3Tv2LUwCCefYKbinFz-DSPNWXW5EEH36u5bGO0ldL7Y44683tNA name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:CvT0GV9Fxa3v0j_oWwSY56k7-hIOPt-Ho3_hUwT5aUNGRyH1bdSGbnsLg-gmTs9bdPZgkoQs-oTqj_ieirmdCA name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2488ms

=== Token Usage ===
Prompt tokens: 2772
Completion tokens: 48
Total tokens: 2820

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: jAObF0x1dz90biR4_nYdFVmEYMb19dyaPSQWlx2B13mJUoph0zdpUQtg8tWV_MZcSdooCNBVYHFWQjDcGZhaeQ

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
✓ Created second session: 1gjmtoMHROg0KCa8IbrFbAQEV4A-em9sQzlFUQeQgdL-rCg0DPsL2p7KS9KY8zml-OlqB47tmELFAaSLOkddqw
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
✓ Created session: k1E8eIVqGqE-lO_7PkwnwtlQtLl2BUNR-B2feKtveL_8ak_ZAuzJeBKfLFkf_zI-KDxeX90IC5wEWVusJSscMA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: yPXMqfzWWh6B_ADGaMSz60VOUaMCLCV5E628b61UKM34AayxiUIv_2_Q5WXmieKcrnjf2kktw1Og8a8DvQ3yTQ
  Parent: k1E8eIVqGqE-lO_7PkwnwtlQtLl2BUNR-B2feKtveL_8ak_ZAuzJeBKfLFkf_zI-KDxeX90IC5wEWVusJSscMA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: yPXMqfzWWh6B_ADGaMSz60VOUaMCLCV5E628b61UKM34AayxiUIv_2_Q5WXmieKcrnjf2kktw1Og8a8DvQ3yTQ (Untitled)
  Session 2: k1E8eIVqGqE-lO_7PkwnwtlQtLl2BUNR-B2feKtveL_8ak_ZAuzJeBKfLFkf_zI-KDxeX90IC5wEWVusJSscMA (Untitled)
  Session 3: 1gjmtoMHROg0KCa8IbrFbAQEV4A-em9sQzlFUQeQgdL-rCg0DPsL2p7KS9KY8zml-OlqB47tmELFAaSLOkddqw (Untitled)
  Session 4: pzOmm3GrdXAkvNrQvEDpLh7CvHMf-1VaM0kvPSika5LbfNwnJcNf5IQaEpCvYaancEPji9cden_C4jNJTfDDcQ (Untitled)
  Session 5: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ (Untitled)
  Session 6: jaaT3Vfrj3B45QAf8_r32GkVnIiUv6kshpVNj2mn7m7EavDP9Lsw_ae_nu16Y5DI3ZQ8BiHlV3Q4VbHiytX-NA (Untitled)
  Session 7: oSTbHI3HIfQPb12V5a71f57bJiy6odGOwleAbeB35d0aLyVVKHQDLIJaJDqfBEi770Qav9ZcpgPyr9k7AywD8A (Untitled)
  Session 8: gyNEy24WI3QCrsiEhkrIDESRQGTfXjb8K19yYzbV1N5bBqzVbySN-E2I81QMuassG-3hZTEl13vLPX-FKSHxAQ (Untitled)
  Session 9: dtK53BYTkysAiNRMWldOmIovwo_wnvgyHbNFMIfz9_pCa33c3eD0iQORSq9S1c2pWDBc9IrosRYBY20zvb8qmg (Untitled)
  Session 10: Rfr0sJJyydcNjx_m5eGjQmvOoovWjppLeco4coWFC92v51wT_xTauDziN2zIy1rsoZA5Hm_iGIXay4z-Fzbjww (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: yPXMqfzWWh6B_ADGaMSz60VOUaMCLCV5E628b61UKM34AayxiUIv_2_Q5WXmieKcrnjf2kktw1Og8a8DvQ3yTQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:GGa-OljCWzRhOyPKG-ypmBYpopWIg1u7kxAPefXf9wrwFsYcyAMhrm29O9qUTlmHaPeCfJ13l9d1rCoPzmRKOQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:GGa-OljCWzRhOyPKG-ypmBYpopWIg1u7kxAPefXf9wrwFsYcyAMhrm29O9qUTlmHaPeCfJ13l9d1rCoPzmRKOQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:GGa-OljCWzRhOyPKG-ypmBYpopWIg1u7kxAPefXf9wrwFsYcyAMhrm29O9qUTlmHaPeCfJ13l9d1rCoPzmRKOQ name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:UxoSMU_xzq7dDx33z0NjEd3B3SynMKrNchxAsijFAsHnUzBdNPoghRsZIUmlpck68kjUpwMWSAyIWFSn-5UVwg]
Inserted with bypass_ripple: map[id:AO0-IPOs5nTA0N7pdV3BEpA3Jg_l0vw4wpyNEU7TjyK_89SLbWfxn7P9kzn-ZTFYG4gMUCrg29cg_VUpYJx5Aw]
Inserted with TTL and bypass_ripple: map[id:p4HvDPGdTSPZCRcSyxwjZjQt31W8hHBIBudvBEkDWgkfYmRRS8Lu9HUpw-Ik3vgOeYGFc9FXIxpAkcM3dT5ynw]
Updated with bypass_ripple: map[id:UxoSMU_xzq7dDx33z0NjEd3B3SynMKrNchxAsijFAsHnUzBdNPoghRsZIUmlpck68kjUpwMWSAyIWFSn-5UVwg name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Dave Brown <dave@example.com>

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
Created user function with ID: 6KH4A0aVO3a7nOy_JEZsokvhFHZHzCE-c9GH9dt0sFpJuiOv324B65ixLoY5sPHVd5ZY4VSE1dGESJcm_DO2hQ

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 19 user functions:
  - cache_api_call_py: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - get_active_users: Get Active Users
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user

=== List User Functions by Tag ===
Found 19 user functions with 'users' tag:
  - cache_api_call_py
  - get_user_wrapper
  - fetch_slim_user
  - fetch_user
  - fetch_user
  - get_active_users
  - validate_user
  - get_verified_user
  - fetch_slim_user
  - fetch_and_store_user
  - swr_user
  - swr_user
  - fetch_user
  - swr_user
  - validate_user
  - validate_user
  - fetch_and_store_user
  - fetch_and_store_user
  - fetch_slim_user

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

added 1 package, removed 1 package, and audited 15 packages in 956ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'u1sMXaINTOFp8k4yTW2z9UBlvadSKujT3iVmcyvidix2Kf6zdsoWUUQyou6F0ZUNMHCVWUiJNU2mhJauYuIMOQ'
}

=== Find by ID ===
Found: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  created_at: { type: 'DateTime', value: '2026-03-28T21:47:36.334Z' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  price: { type: 'Float', value: 99.99 },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  active: { type: 'Boolean', value: true },
  id: 'u1sMXaINTOFp8k4yTW2z9UBlvadSKujT3iVmcyvidix2Kf6zdsoWUUQyou6F0ZUNMHCVWUiJNU2mhJauYuIMOQ',
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-28T21:47:36.334Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-03-28T21:47:36.334Z',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  price: 99.99,
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  name: 'Test Record',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  active: true,
  id: 'u1sMXaINTOFp8k4yTW2z9UBlvadSKujT3iVmcyvidix2Kf6zdsoWUUQyou6F0ZUNMHCVWUiJNU2mhJauYuIMOQ',
  metadata: { key: 'value', nested: { deep: true } }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  active: { value: true, type: 'Boolean' },
  created_at: { type: 'DateTime', value: '2026-03-28T21:47:36.334Z' },
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: 'u1sMXaINTOFp8k4yTW2z9UBlvadSKujT3iVmcyvidix2Kf6zdsoWUUQyou6F0ZUNMHCVWUiJNU2mhJauYuIMOQ',
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: V5SLQnzdVjChHCF0XHmAyMzMC_dBQN78uBLsziSfOF3kGUMBDs5LrPSilhXqveeT4USwy_ulhxBKbRFzAzA3Ww

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
Collection created with first record: CzhJ05GyE2f02XCYLHlYQA_-HvKKxTjaekfAoZLyoGL0-NsvEHw6OfxWcPShHt2n-yw9GyXfT67JeIvE2h9Dfw

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_go,ws_ttl_test,audit__ek0_testing,products,batch_users

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
Created Alice: $1000 - ID: q46NtavRFLqcsjDgMSPonGnxIn6_jFye9O5x6UB_DCsgobGFCA5h_YZ3EOMwjfAIuOgRypf_mUVQ31i9xFdVWQ
Created Bob: $500 - ID: eKObUvPpe0K5yEFCokPVCFib64L1aMqqfPYpEEXgBjqCG0H4fCEnZfjc7cfHJBP66fDBq5YtJAj9C1fWTGngnA

=== Example 1: Begin Transaction ===
Transaction ID: b76e8fc5-afd3-4e07-a689-07638652a16c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a27bf9b2-6a1a-4c15-b1ee-d1a5d3c1ab15
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, title, bio, title.value
  4. Score: 26.400, Matched: title.value, bio.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, bio, title
  2. Score: 39.600, Matched: title.value, title, bio, bio.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.769, Matched: 
  2. Score: 0.750, Matched: 
  3. Score: 0.731, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.708, Matched: content.value, title, content, title.value
  2. Score: 1.493, Matched: content.value, content, title, title.value
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, bio, skills.value, skills

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
✓ Inserted document: fypTY-dQI353wWgIco4n_UTnbfmLSLLie40kIgTh4oqVXK3I47duMcQooFJ7DawiV3u0GuS0_I4bMn7sphxvvQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: lDS_uI4rFLBvZX2goA6zBPUwpBS9TPgWKBMN99v6Z1fFFVe8ANivb4y7ymwu8WEqgCHm8ToL9banGmMkNAbz7A

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
✓ Inserted document with TTL: pmLDaA5f_ykQ-jo2D2tUWqnvcwQLVYNBv1uFVmXxrOcIsoHobDGL8fYwM2JnqmG9204U_C8QyIK29G8TuoLXNw

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
✓ Edge cache script created: sc_EoIyfc-7-c3CpSVhzEyYW0AbRhD7LvYbRC3co3Y4G_1q-9dp21IK3EG2McxWXGw7SkeELUITlt3eVqI7wPA

Call 1: Cache miss (fetches from API)
Response time: 147ms
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "id": 1,
        "name": "Leanne Graham",
        "username": "Bret",
        "address": {
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
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
Response time: 41ms (3.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "id": 1,
        "name": "Leanne Graham",
        "username": "Bret",
        "address": {
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
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

✅ Script saved: Oe0JQr0idliWpDbumTkA1JgYgqA3Y-pUzxyWkqannFduZvmtzPa3xkyTBzqwS3ojopUjStjQQJL57B3ViH_Rxg
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: uyqRDUC9bImHkwvzoybWUTNt1nqtVfm8NXkFX_ADrB5hMeF2UIP5sSKQrcQRo2cqlHiM1Tu77IFCmw511iDjSQ
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Hcz8mRRU48nWKoE26-G0IBWPApkzE_kxewfiVfkBjFP0PDV2qQNvtEpJ8KKjggP3AiHQgBtyns-39mbM3MPWew
📊 Statistics: 3 groups
   {"count":5,"avg_score":60,"status":"null"}
   {"count":5,"avg_score":60,"status":"active"}
   {"avg_score":50,"count":5,"status":"inactive"}

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
   Name: {"type":"String","value":"User 1"}
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
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: WaWGEs-KmAuwED84v9KI2OUZVT-Uv651JcZJmX5V7Qzx_r3LmGLA6Rh2IiYxYr5f4xUc1S_NBpoj5TKGWWMe1g
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: g__UU1ijGxdFjgg5etAVNiUdvsVEvy6ShUtar_csMhw6YQbvFrRgrQy31nQRDX4J92iTNCUsQ-jK770R3Z1Buw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: g2GQTt-fZ46fIVgleD-1YT53WZeYE0hOhVKdW8deKXIVhVj_iueRWcxFOAFS72N6b3Uif17jHdEh8mhaG_ZxrQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: fGjh_GI94XGueUA2Kc216M04filfB9bTifgpzALWe3j9hHK2GfDyVsteJaV1_ilt4VTiRGnCY8puALET5lt8jA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: GsxIyimQLYKDVIUghOZ6HzCqrhzIXaDTzjz_B6pVKKM6ERVeWKymH-YAjE01PCuqg2uJO3kg9T9FNBed4lQyog
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
✓ Created SWR script: fetch_api_user (rH2a7sWu4lXbTuGICPYLgBTWC9IVVO5cZ6q1w2DrUXT20V8l30eFaIJPu2OHC-MQLw3r2MIwTTF1MXCf0n1c2A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 46ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (MfddG2eZwXnT22UIVxKaCvVcUbH5pksm0_KJRlhxGjVOSkng_G6YTv2sCavaV0gsoRWi2LoxjEs4s6T69r4vhg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "rating": 3,
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          },
          {
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "rating": 4,
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerName": "Eleanor Collins",
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          }
        ],
        "sku": "BEA-ESS-ESS-001",
        "price": 9.99,
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "availabilityStatus": "In Stock",
        "meta": {
          "barcode": "5784719087687",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "warrantyInformation": "1 week warranty",
        "weight": 4,
        "rating": 2.56,
        "brand": "Essence",
        "discountPercentage": 10.48,
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "returnPolicy": "No return policy",
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "category": "beauty",
        "minimumOrderQuantity": 48
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
✓ Created native SWR script: github_user_native (g5eUjaQhYsz6jr6LCwDhoDF6xVp365e4KjsaRB3jslhtp0bSaYateWXNgKcVvdWpN9nNlF1DPMNPm6K8Z964yg)

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
✓ Created SWR script with audit trail: product_swr_audit (igPgyTjE8TcYJpzaByJV32ucmDpvmuv4_0WV50dkxO_vw1AT81_rdiR3NYOqJb8mHfioS8Y0h--EUUQ34-KAmg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (OV8Hq90QlAFyMVBcB7jlBPqx1n5KxNRX8-qp9mutkir6EXB_jTaMDUhsXryoZYf6yo22TvtYgNYXf1xAhSNX1g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (Sj7gljL-fStJ1VuMnPLt3PvJcZmOFQa8VaTOQpf3urKddXat1S_Dl5pzTnMAVEchAVx6z-GvkwQcgGecc2x9Bg)
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
   {"avg_price":367,"category":"Electronics","count":5}
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items based on vector embeddings, which is crucial for applications like recommendation systems and image search.

2. **Scalability**: Designed to handle large volumes of high-dimensional data effectively, making them suitable for big data applications.

3. **High Performance**: Their architecture allows for optimized storage and querying of vectors, improving performance in ML and AI applications.

4. **Flexibility**: They support various data types, such as text, images, and audio, enhancing the breadth of use cases.

5. **Integration with Machine Learning**: They easily integrate with machine learning workflows, facilitating the storage and querying of feature vectors generated from models.

6. **Advanced Indexing Techniques**: Use of techniques like Approximate Nearest Neighbor (ANN) to further speed up search times.

7. **Enhanced Retrieval Accuracy**: Provide better retrieval accuracy compared to traditional databases when dealing with complex, high-dimensional data. 

8. **Real-time Analysis**: Support real-time querying, which is beneficial for applications requiring immediate results. 

These advantages make vector databases particularly valuable for AI-driven applications.
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
   {"role":"admin","count":3,"avg_score":20}
   {"role":"user","count":7,"avg_score":70}
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
   2. Introduction to Machine Learning (AI)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: i-BUzL5C2HrCbpnbd0AFuVafXbkVVZTFFUmBhONDsxEu6V3nsJBaxZIsxWxd1we4-zkOhR_yG4V4GWuPhJdl3Q

=== Sending Chat Message ===
Message ID: HQQwzwb5ggooNl41olApnqdvxTRPTE9FnFhdKyHpMYIq0lZKr_UM2uY0dXvCulI8jbRPmHmkrTHW1ZFJPPud2A

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

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    name: 'ekoDB Cloud',
    id: 'fSSZkG0sl_shw7Zg7-RYhZBbYBRtJV7ZbTbtwEqtwkxbr-9LdoxZkhVO4Ox8BFMPCA5bier2vqILNr1n-TQdSQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'V6cYON17TARNGR69rek7ccmBRyfq98SHkcQsJZphNWMn8wURGmcin0c-VOwi3KfyngCtVlTFauU3T8D6f7c8WQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'U8A2G8opXjZtCOJg7IfRCV2dvRjNhEimz6oD5grk3kWS1hnNXtszVJN15yIlLu09q3cHWZojKwD1w6tzfcHzNQ',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2032ms

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
✓ Created session: AFeU8FkylYyVbIyeg97wuoH5lA0wFLgv3hV5WRINRbv9kTFBCgA1smAndTMczYQyVgkG5ePxWRiLGvpj1hAJsw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or would like to see additional products, let me know!

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
✓ Created second session: cEDZEwnAiPqPjGQvqTYxwxrjJZyEqAy9ETnuVnZGbmvmU11zpr2pTDfnhAhbxgZn-OUc3iEiklbm49mxRRazVQ
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
✓ Created session: sx_rMnfqErh-mtyjHAyXFsabMqi00LrFPcyYpdKVjsVkC2qDvI96XR7o-5mkJ-n1g--B4q9yh2ZvVSS9th69DA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you would like more information or details about additional products, please let me know!

✓ Message 2 sent
  Response: The price of **ekoDB**, the high-performance database product, is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: X8TTrHDVeFd6iAWoUI1DDYB61iKXjpSGZhdm4-4FFlvvaC63RF1GE0deQucwiNxv1sPdmmAe1w5n_5oh9Dmf6g
  Parent: sx_rMnfqErh-mtyjHAyXFsabMqi00LrFPcyYpdKVjsVkC2qDvI96XR7o-5mkJ-n1g--B4q9yh2ZvVSS9th69DA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: X8TTrHDVeFd6iAWoUI1DDYB61iKXjpSGZhdm4-4FFlvvaC63RF1GE0deQucwiNxv1sPdmmAe1w5n_5oh9Dmf6g (Untitled)
  Session 2: sx_rMnfqErh-mtyjHAyXFsabMqi00LrFPcyYpdKVjsVkC2qDvI96XR7o-5mkJ-n1g--B4q9yh2ZvVSS9th69DA (Untitled)
  Session 3: cEDZEwnAiPqPjGQvqTYxwxrjJZyEqAy9ETnuVnZGbmvmU11zpr2pTDfnhAhbxgZn-OUc3iEiklbm49mxRRazVQ (Untitled)
  Session 4: i-BUzL5C2HrCbpnbd0AFuVafXbkVVZTFFUmBhONDsxEu6V3nsJBaxZIsxWxd1we4-zkOhR_yG4V4GWuPhJdl3Q (Untitled)
  Session 5: k1E8eIVqGqE-lO_7PkwnwtlQtLl2BUNR-B2feKtveL_8ak_ZAuzJeBKfLFkf_zI-KDxeX90IC5wEWVusJSscMA (Untitled)
  Session 6: 1gjmtoMHROg0KCa8IbrFbAQEV4A-em9sQzlFUQeQgdL-rCg0DPsL2p7KS9KY8zml-OlqB47tmELFAaSLOkddqw (Untitled)
  Session 7: pzOmm3GrdXAkvNrQvEDpLh7CvHMf-1VaM0kvPSika5LbfNwnJcNf5IQaEpCvYaancEPji9cden_C4jNJTfDDcQ (Untitled)
  Session 8: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ (Untitled)
  Session 9: jaaT3Vfrj3B45QAf8_r32GkVnIiUv6kshpVNj2mn7m7EavDP9Lsw_ae_nu16Y5DI3ZQ8BiHlV3Q4VbHiytX-NA (Untitled)
  Session 10: oSTbHI3HIfQPb12V5a71f57bJiy6odGOwleAbeB35d0aLyVVKHQDLIJaJDqfBEi770Qav9ZcpgPyr9k7AywD8A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: X8TTrHDVeFd6iAWoUI1DDYB61iKXjpSGZhdm4-4FFlvvaC63RF1GE0deQucwiNxv1sPdmmAe1w5n_5oh9Dmf6g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'lqKnXY38T-oEOgAHZEifKxRuP8KC6yvQaC1gZBd0_aaekoVgAPm3kaqdtTMvg1yTlTk1jCQGgT-lAg7PWnJSYQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { value: 29, type: 'Integer' },
  id: 'lqKnXY38T-oEOgAHZEifKxRuP8KC6yvQaC1gZBd0_aaekoVgAPm3kaqdtTMvg1yTlTk1jCQGgT-lAg7PWnJSYQ'
}
✓ Second upsert (insert): {
  id: 'IPLC8WvgenR0aFyVnTNyo9tY3eB5n4OFHD4t0dtMyJ6OTc9earE5Hb3yYvQUoDFJTO8MT8Y2TGAovmGSMZy-eg'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { value: 29, type: 'Integer' },
  id: 'lqKnXY38T-oEOgAHZEifKxRuP8KC6yvQaC1gZBd0_aaekoVgAPm3kaqdtTMvg1yTlTk1jCQGgT-lAg7PWnJSYQ',
  active: { value: true, type: 'Boolean' }
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
   Inserted with ripple: {"id":"8SheOZd3PtHT1Q-wEle5MlgVo5tYv6g_Kddr5jhogVy4w7aeeNaWW76E9PpPrbL2yym0rVDj2SMe_d52u5lz5Q"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"GkgX9Cuixc3kB3-Bp_l4W1P5xyei_QfiPcyKPuXTZJ4unGMbeJ8B-m0MXa55FP3Nw19zZAmNXT1F_RO0UdjK_A"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"},"id":"8SheOZd3PtHT1Q-wEle5MlgVo5tYv6g_Kddr5jhogVy4w7aeeNaWW76E9PpPrbL2yym0rVDj2SMe_d52u5lz5Q"}

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
  Fields returned: ["id","name","email"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["avatar_url","email","status","bio","user_role","name","created_at","age","id"]

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
    - Fields: ["status","avatar_url","bio","age","secret_token","id","user_role","name","api_key","created_at","password","email"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","id","email"]
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
Created user function with ID: 2RD27jvfTYwuHWVsrHfkUCYLqV5l60_j54JtICKwJh1AvzUGDqxjE_WX38DOl00LZb07cqaUq51AoB1PPHo1xg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - cache_api_call_py: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_api_user: Fetch User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_ts: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call: Cache External API Call
  - get_verified_user: Get verified and validated user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user

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

added 1 package, removed 1 package, and audited 9 packages in 954ms

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
  id: 'G0jxqQepD_lv5xfLhu6nVjEd4dO-jHFHPoDAeynywslxh-2KtetwO_7cBhrGIuyHWsJMypImEraqf8XU8AYLLw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  id: 'G0jxqQepD_lv5xfLhu6nVjEd4dO-jHFHPoDAeynywslxh-2KtetwO_7cBhrGIuyHWsJMypImEraqf8XU8AYLLw',
  active: { type: 'Boolean', value: true },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    value: { value: 42, type: 'Integer' },
    name: { value: 'Test Record', type: 'String' },
    id: 'G0jxqQepD_lv5xfLhu6nVjEd4dO-jHFHPoDAeynywslxh-2KtetwO_7cBhrGIuyHWsJMypImEraqf8XU8AYLLw',
    active: { value: true, type: 'Boolean' }
  }
]

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  id: 'G0jxqQepD_lv5xfLhu6nVjEd4dO-jHFHPoDAeynywslxh-2KtetwO_7cBhrGIuyHWsJMypImEraqf8XU8AYLLw',
  active: { value: true, type: 'Boolean' }
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
✓ Inserted test record: 2gCiK1gMfgUB6nocfeRqWpNCqZhSvD7khDx4uZs91BkRA9ju9_4q3AkFAKoC1hXqE1tQW4MJReSZbDfbX8-plg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774734496185",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "fQW6SBGgSvKuNS5AKtPgbx9f8znVSScfJfJFeZectmHoGisu5TIZw-ftWvtZkUwKd3podcIrg1_b_HMlYHimDA",
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
        "id": "2gCiK1gMfgUB6nocfeRqWpNCqZhSvD7khDx4uZs91BkRA9ju9_4q3AkFAKoC1hXqE1tQW4MJReSZbDfbX8-plg",
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
        "id": "ZODatZcbUzL0VY5XwPCFMuuff3xb33htMyj5AetliQ0GoBfkKJNfjwrL9qqem_ISoiT2b-KFCkj1wZV2c4g9yg",
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

✅ Script saved: Pr0ajsDlrQ0ieNevElMRXOykVN3zn9qPg2yPVHRKcLCnYa521LGxoWtdgFv77g14PxYzf8hzpfLD0FlY5PeWbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: rVyWof_Gu4QNQEnv4xW-51kt3u9tOoChcTyoARqMlBfoSicBeE9czxkBBE7HwU-mHWWbvZRQPdFHWb0ofpPYSA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Amja3rAo718P2Eo_e420WuZ_y9NF1tKF_TGa9yy-M8esU6nGMvVGBG-e6X_nfi9GQVj6p0Aq2IVSUu9tezNUcA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"max_score":100,"count":10,"status":"active"}
   {"count":10,"max_score":90,"avg_score":50,"status":"inactive"}

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
Retrieved value: { username: 'john_doe', userId: 123 }

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
Collection created with first record: gBn8bIqDQ1mszVLYY_Ikn6_1Y_jBKtdDLicpbmWHfHoKmMNeIkZclpEsoIun2VXF-xwNBq8Z77CB3i5FxQgxqw

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_products_client_go',
  'schema_employees_client_ts',
  'ws_ttl_test',
  'schema_products_client_ts',
  'audit__ek0_testing'
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
Created Alice: $1000 - ID: 991ivDZcPS4MOkiike8T2xtKfi01pn26CwKGmC_xDIvtWC0TrJkMROX05xy7fgN26rB8F93Ty7Me-DKUCnUI5Q
Created Bob: $500 - ID: _74D5uE04Etz8EUxGMRcFg2cWqi8Nrf8lL6xOKTZFkO1TKhl1B0RdnUVjjqS3kwAtZxzW-9DUSh9pwRM9oRXZg

=== Example 1: Begin Transaction ===
Transaction ID: af844926-9c4a-4011-b81f-5c201faf820b

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
New transaction: 99ba88e2-30db-42c3-97a3-c3f579f670ef
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
   ✅ Script saved: rLSa_agY78qTRn3dN6O6ou8b2SjmaiTXW_lgnkggd-mI8MznnLo-wqr-FnuJdnBv2fS7RKv-un504V41An39QA

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
   ✅ Script saved: QGjlNaQc1eb7wJFuwuDh17cy61EG1u9O89HD1KJmAJv-cCOV2fFm2QOqpm4NxIOPdnqbNYefYgJjlV5y71oSaw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: PyNaRTaJLJxvSKCT0YUIg39kv8JnIKz14_4oGtiFwECqENu3KQlXwT_JBy1fFkZijVivZAbqum66JfRtlCw4fg

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
   ✅ Script saved: bGnbH0o_Biylienaez-0wJpNb8guQgWzYZcc0sxSoKUEhDbX0vnEKXmqCMXV7bQ24Og4KY2hzaDTe9lSHjD70g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: rLSa_agY78qTRn3dN6O6...
   ✅ Deleted script: QGjlNaQc1eb7wJFuwuDh...
   ✅ Deleted script: PyNaRTaJLJxvSKCT0YUI...
   ✅ Deleted script: bGnbH0o_Biylienaez-0...
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
  Output: Document ID = fr2G_8jcbzXHhm_E-P_UqOh5m-KZ_d1lKFDpyYFKrjkCs4ga-2QxSWbJS4P4o-WgDZ6yEicGjTOoMetsevKfdQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(fr2G_8jcbzXHhm_E-P_UqOh5m-KZ_d1lKFDpyYFKrjkCs4ga-2QxSWbJS4P4o-WgDZ6yEicGjTOoMetsevKfdQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(fr2G_8jcbzXHhm_E-P_UqOh5m-KZ_d1lKFDpyYFKrjkCs4ga-2QxSWbJS4P4o-WgDZ6yEicGjTOoMetsevKfdQ)
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
  Output: Document ID = mkku4AtYfbwmnXjEU9fqAWMRZQcBUR3_Z8flb6xBFNrs233U0DX6Rd9s3T3m1enmOLWYS-DabHxOLrRi8WfrZA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(mkku4AtYfbwmnXjEU9fqAWMRZQcBUR3_Z8flb6xBFNrs233U0DX6Rd9s3T3m1enmOLWYS-DabHxOLrRi8WfrZA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(mkku4AtYfbwmnXjEU9fqAWMRZQcBUR3_Z8flb6xBFNrs233U0DX6Rd9s3T3m1enmOLWYS-DabHxOLrRi8WfrZA)
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

added 1 package, removed 1 package, and audited 9 packages in 954ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Xgo-MULamZmRVjQmHb_QFErpRnymeRuikg4nasr6BB86DvfHXyoPsT9fZD-zRVz0G0uU1eqO_br1pYdjPhFACw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'Xgo-MULamZmRVjQmHb_QFErpRnymeRuikg4nasr6BB86DvfHXyoPsT9fZD-zRVz0G0uU1eqO_br1pYdjPhFACw'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  id: 'Xgo-MULamZmRVjQmHb_QFErpRnymeRuikg4nasr6BB86DvfHXyoPsT9fZD-zRVz0G0uU1eqO_br1pYdjPhFACw',
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 1ErhYsCvBAOD9MLiDTLbVE--vhsstiLQuc0qEIxTqhdZJSlgLf5H9HvchfvAmM88s68aCm_NadWeXs4gJAoPjA

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
Collection created with first record: Cz2w7S4QQs5Q3bxtrTvpafCdgfOfb6D_01sjH5y4-x5YvfU4l5JEmP16wX2pGStUJQ2h4UrF4ei2_HwsNMSdxQ

=== List Collections ===
Total collections: 24
Sample collections: schema_products_client_go,schema_employees_client_ts,ws_ttl_test,schema_products_client_ts,audit__ek0_testing

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
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
Created Alice: $1000 - ID: G9wA4CRUiCL5OkPoGCHANAXW_zKfJk16gg10Jg4VS-hc5H_EdHy888btOwkdiRHIMe7wG6K0NrnL9RBxqhGH7Q
Created Bob: $500 - ID: I2myDs8sm10KHNCNIT65_ylb35gB_Xqbp2rYC607dQ7-3D2u6IiTfNrwdhyGvZ-kyAVwWoQ9XEUqdlUja2kg2Q

=== Example 1: Begin Transaction ===
Transaction ID: 60c3f931-9716-49a9-9e76-ff009e1ec701

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 2bc76593-c59d-4d13-a912-b7cd23442168
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
  1. Score: 25.740, Matched: name.value, name, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, bio, title.value
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: title, bio.value, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.737, Matched: 
  2. Score: 0.734, Matched: 
  3. Score: 0.725, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694, Matched: title, title.value, content.value, content
  2. Score: 1.495, Matched: title.value, content, content.value, title
  3. Score: 0.290, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, skills, bio

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
✓ Inserted document: Yj6nruk5ZWWn5STGKYv7_IDYS2Stj1ttKlqa_mdM9uT8MtxWp5IEv4qOLVOVmCKP1fzCSkqjJIF2aPrcv-8WdQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: AuYDTTdnb7u-05nplayLc9Z8vH_M-Wl6zC2vCmGh2YSi-bdDF1CZVt9a0XkqMGGogcai1x6KJGbem3IGvTgicw

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
✓ Inserted document with TTL: V7bVGzRnIelDM95FId3aAzxyzuSKziWEB8v_fSGTYe2S8yP99OkEdxL8ubfiZSyppWPCx2h3znRw7z_W7PZPgA

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
✓ Edge cache script created: U62GOKAehmq2W1obs3dw87DZrBnu_tDLSuY7ZQlUORoZ83ULpspnihDg-u2JdUz-ZY6ph0H8DYxvPTIWmvGYiQ

Call 1: Cache miss (fetches from API)
Response time: 41ms
Result: {
  "records": [
    {
      "value": {
        "longitude": -73.99308,
        "timezone": "GMT",
        "current": {
          "interval": 900,
          "time": "2026-03-28T21:45",
          "temperature_2m": 5.5
        },
        "elevation": 32,
        "latitude": 40.710335,
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "°C",
          "time": "iso8601"
        },
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.022292137145996097,
        "timezone_abbreviation": "GMT"
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
Response time: 41ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "longitude": -73.99308,
        "timezone": "GMT",
        "current": {
          "interval": 900,
          "time": "2026-03-28T21:45",
          "temperature_2m": 5.5
        },
        "elevation": 32,
        "latitude": 40.710335,
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "°C",
          "time": "iso8601"
        },
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.022292137145996097,
        "timezone_abbreviation": "GMT"
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

✅ Script saved: GLECVK5jZJAl-Km2P1Gt_ld6Y9CMpMjHs2Ax1sGIhDns6em2-JFHP-U57Ph4Hug1DRN0Ff7aJh9rKi-0R4gTbg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","avg_score":60,"count":5}
   {"status":"inactive","avg_score":50,"count":5}
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
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
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
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: plgJcDlgjw2MDn1pCZxfONUGtVo8gRqIi7Bo9lgWCbKlmZWP_M6zU4r7Tw55Jr6rHk9fTeYeR-VaJK8JJiMEkg
📊 Found 2 product groups
   {"count":2,"avg_price":474,"category":"Furniture"}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: xKcvvo6BICBPbe7mBsNqKZjcxi9XibnIeRC1n7uDDAPnhu7TSrtTRl9pUD634CGKW4jQR8rvp6LdFRzo_q3haQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: R0RxRLFXFMgmmW47E99d-FyRZMwfT8i4aQcLqrb97Njry07lvwQ-9eClBtmDT4Rd593sfFyz7uNH5OEw9HXdew
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: ZTwj-PZCmLWyHoxDW9rEZ2OwCHH2h1az0hHFsQ7CprLqsqzuUXTPToLpHeAminSDep7hJ0IyDrZFmDBGygM95w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 0KfhwlSEIhhUQgH4gfODp0fIevoSuPGESoC0uvi_K0gVJF7m9hNjOdF_FNcFuvCNLRyO5TQaPBbDuN2air0qgQ
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
   1. User 5 - Score: 50
   2. User 10 - Score: 100
   3. User 2 - Score: 20
   4. User 3 - Score: 30
   5. User 8 - Score: 80
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
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
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
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
   1. Database Design Principles
   2. Getting Started with ekoDB
   3. Vector Databases Explained
   4. Introduction to Machine Learning
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Getting Started with ekoDB
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
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
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. USB-C Cable - $19 (⭐4.3)
   2. Keyboard - $89 (⭐4.4)
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
   1. Precise Geometry: Vector databases provide more accurate and detailed presentation of geographic features.

2. Scalability: In the vector model, data remains accurate and sharp when zooming in or out.

3. Less Disk Space: Vector databases can take up less disk space compared to raster databases.

4. Data Integration: Vector databases can facilitate the integration of different datasets.

5. Data Analysis: They allow for robust spatial analysis and mathematical modelling.

6. Maintenance and Update: It is possible to update vector databases without affecting the entire dataset.

7. Attribute Assignment: In vector databases, attributes can be assigned to individual vectors, providing more detailed and comprehensive information for analysis.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (ZkUmbtQ8pVwjkN7sb09ErSqSGQog-OniVDnwzGLWC0otcJK3krnwaOY66tMvlOAWzYqwCKGcGjG1kaX6zEfFQA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 42ms (served from cache)
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
✓ Created native SWR script: github_user_native (Wmzo1ZuJ4zFFQwun1FFkCYG41odeM8jh5MVDayO58VBFotLOAsbDHLWcgISSi907PrXoDfFX7QkdLPjgNfgf3g)

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
✓ Created SWR script with audit trail: product_swr_audit (ph3Indp8U1Ke98YgWF8s6GXVlDT12luB_cAuURT7ms1tGfntHLARj0MCKILN3YUO8NOrM2rQN6txIT0gFrMxvg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (Uj8TF9ylFJpjLIv_rMfvBJsXphlC2QTYdxkLnC-gp6pI-AB3FFN-G9ZyACwuaYEdqYmhbKu5VAJffw9IdUasfg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (PtE8YUG8yOqqxfZXjMOlaCt4OVbtz0_FCR22K9kGTfM74T8h5DmjpXNPvcacqBaBulIWt-VVd6t31qpO3om5XQ)
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
✓ Created session: Qnx-in5M8OsQ2S-X3KzbbkhGC2xu1cmZiW-isnnLRf6kmdWOTKNu6XGWXNsXT4SrTga-bt2G-7WOvSboQPYgwg

=== Sending Chat Message ===
Message ID: FGfkj-373eFhugb7Wruh3GeT33-96-KEVNkorwYDZ-VQ3AmIR5SSwbf7Q-ZH4qeUbLTNs_jiXpAuTthE_acznQ

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: '-quARG-K8B5yvYIQMFlj4uCSAFFIOH3EV5lOPZvxABzb72J3nFTHdyQ248VubtD8Mi4i6iGdgjS7Zn7Xbh9Hqw',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    id: '22hyEuuG7S-binvZNERtF21NyGARh-eAOPelAUlof4mh-4JsN105Bj7bDZVW4o54qJ4Pc14Eua17HNsa5ecf-Q',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'OJKe8Om0jH6Z-nZmyaymn2Q49LINXaCZvDfDCmk2FaFoMAK4aHhwu9WlKkbOnSMzuxmKxQyzdbmeysbKdUaX1g',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2383ms

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
✓ Created session: X8hX5_OmADq8xs09n7MdwlePVqCWMErem75cGjK4ccKVaRGYsTkH7YmNnVkLen1ksIDkUhGLEvB5w_QRASsQqw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or have further questions, feel free to ask!

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
✓ Created second session: tVnKKouZwLf5Vf205Vr9fmvExnDZvs6FYdmxEH1EhIMbDEL1UUhWr_UUT0yGw06rkAw5fvw0ZNwux7DkthW8Aw
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
✓ Created session: aH41tlisMQ2Mbdy1X_P5pp1QvjOINzzB2yyR9shYSLUJCgT2sASSfkiSFdoUOCzVswcUDXDD8rKsXOrRXvWZ7w

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you would like more details or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: REMeZEWkRUIvKGpkV9X5w0nPhoGqWMZDvEoyzbi1nI7xaJTpZTYTa6HbuqGRK-T7nGJBMyDtYmpMB_EdYKhbMQ
  Parent: aH41tlisMQ2Mbdy1X_P5pp1QvjOINzzB2yyR9shYSLUJCgT2sASSfkiSFdoUOCzVswcUDXDD8rKsXOrRXvWZ7w

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: REMeZEWkRUIvKGpkV9X5w0nPhoGqWMZDvEoyzbi1nI7xaJTpZTYTa6HbuqGRK-T7nGJBMyDtYmpMB_EdYKhbMQ (Untitled)
  Session 2: aH41tlisMQ2Mbdy1X_P5pp1QvjOINzzB2yyR9shYSLUJCgT2sASSfkiSFdoUOCzVswcUDXDD8rKsXOrRXvWZ7w (Untitled)
  Session 3: tVnKKouZwLf5Vf205Vr9fmvExnDZvs6FYdmxEH1EhIMbDEL1UUhWr_UUT0yGw06rkAw5fvw0ZNwux7DkthW8Aw (Untitled)
  Session 4: Qnx-in5M8OsQ2S-X3KzbbkhGC2xu1cmZiW-isnnLRf6kmdWOTKNu6XGWXNsXT4SrTga-bt2G-7WOvSboQPYgwg (Untitled)
  Session 5: sx_rMnfqErh-mtyjHAyXFsabMqi00LrFPcyYpdKVjsVkC2qDvI96XR7o-5mkJ-n1g--B4q9yh2ZvVSS9th69DA (Untitled)
  Session 6: cEDZEwnAiPqPjGQvqTYxwxrjJZyEqAy9ETnuVnZGbmvmU11zpr2pTDfnhAhbxgZn-OUc3iEiklbm49mxRRazVQ (Untitled)
  Session 7: i-BUzL5C2HrCbpnbd0AFuVafXbkVVZTFFUmBhONDsxEu6V3nsJBaxZIsxWxd1we4-zkOhR_yG4V4GWuPhJdl3Q (Untitled)
  Session 8: k1E8eIVqGqE-lO_7PkwnwtlQtLl2BUNR-B2feKtveL_8ak_ZAuzJeBKfLFkf_zI-KDxeX90IC5wEWVusJSscMA (Untitled)
  Session 9: 1gjmtoMHROg0KCa8IbrFbAQEV4A-em9sQzlFUQeQgdL-rCg0DPsL2p7KS9KY8zml-OlqB47tmELFAaSLOkddqw (Untitled)
  Session 10: pzOmm3GrdXAkvNrQvEDpLh7CvHMf-1VaM0kvPSika5LbfNwnJcNf5IQaEpCvYaancEPji9cden_C4jNJTfDDcQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: REMeZEWkRUIvKGpkV9X5w0nPhoGqWMZDvEoyzbi1nI7xaJTpZTYTa6HbuqGRK-T7nGJBMyDtYmpMB_EdYKhbMQ

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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=gJCqt-Srwhv8koubQbluHKcq4VRzXFg4sz4FJnupti3U4W9CqIEz-s9lZRr1xvFXuv34Rx0KRvE-ovcqAvN8zA)})
  User ID: gJCqt-Srwhv8koubQbluHKcq4VRzXFg4sz4FJnupti3U4W9CqIEz-s9lZRr1xvFXuv34Rx0KRvE-ovcqAvN8zA

=== Read ===
✓ Found user by ID: EkoRecord(fields={user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), id=StringValue(value=gJCqt-Srwhv8koubQbluHKcq4VRzXFg4sz4FJnupti3U4W9CqIEz-s9lZRr1xvFXuv34Rx0KRvE-ovcqAvN8zA), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Mar 28 17:49:39 EDT 2026)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Mar 28 17:49:39 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: user_id, metadata, id, email, categories, tags, embedding, name, age, created_at, price, data, active

=== Update ===
✓ Updated user: EkoRecord(fields={embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Sat Mar 28 17:49:39 EDT 2026), type=StringValue(value=String)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), id=StringValue(value=gJCqt-Srwhv8koubQbluHKcq4VRzXFg4sz4FJnupti3U4W9CqIEz-s9lZRr1xvFXuv34Rx0KRvE-ovcqAvN8zA), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), id=StringValue(value=gJCqt-Srwhv8koubQbluHKcq4VRzXFg4sz4FJnupti3U4W9CqIEz-s9lZRr1xvFXuv34Rx0KRvE-ovcqAvN8zA), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), created_at=ObjectValue(value={value=StringValue(value=Sat Mar 28 17:49:39 EDT 2026), type=StringValue(value=String)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)})})

=== Delete ===
✓ Deleted user with ID: gJCqt-Srwhv8koubQbluHKcq4VRzXFg4sz4FJnupti3U4W9CqIEz-s9lZRr1xvFXuv34Rx0KRvE-ovcqAvN8zA

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
  {"data":[{"id":"iY6iJgiw4taNwr6aBhETlzFisNnmgZN92nhQrLIvilM52Z5kG8MiWbNgsKZLo04ZtePz3NFtvwU78RF2z5WR3A","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]}

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
  IDs: IxmNyef5xBJVm4dN41Qp_HMthifYEqBRmOwgzk19GZ07bn8IUr_yt7zfAFh8P6ctYLqPSjfPkA56mTv5w5mgaQ, B8qxt9O2HVbFwk2ie6W_yJ2RKuPUpCspVtNTgVyt4BmmKwu3MK9ivdEW-G-P4GbvnBfwHRzEUuNCwVlfyFFkAw, EAv_0n6IYdMDivwndS29uTVpKLVLFC6fkPoF0oO94YTHXl2v16aCp03RDZDGkI8bQ31ehbGkVE1AaViWEbAvxA...

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
  - schema_products_client_go
  - schema_products_client_js
  - schema_employees_client_ts
  - ws_ttl_test
  - schema_products_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-03-28T21:49:56.045437Z","last_modified":"2026-03-28T21:49:56.045437Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"role":"admin","name":"Alice","email":"alice@example.com"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1774734600335}
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
  config:api: {"value":{"timeout":30,"retries":3}}

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
Created Alice: $1000 - ID: XO367I0HrAM4wv6WFiBW__6zz9MFqnrUrjTDTvOpH9CCYE3Pz3djvdv8t2cm7R1_GBo4ybFuKAeCWxL8cB594w
Created Bob: $500 - ID: 3NUdnXht8pmVu1AmBJ1rVJbpRanGzJVo_VrNKPaqspCDM1b9xgBKP0x5D2wNutQm--NdsmWWHKja7g9tBmEPvw

=== Example 1: Begin Transaction ===
Transaction ID: c13f2093-2e0e-426f-ab9d-9f9ab2a996d2

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0275c1fb-edc4-404f-b073-cd6822872f18
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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Charlie)})
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
  {"results":[{"record":{"tags":{"type":"String","value":"programming,rust,tutorial"},"views":{"type":"Integer","value":551},"id":"_nNkYbRNhwlb7VDegIj0GExapUKpEn3s4ZtZbSye3jPbw57_Xkiwk7avEbXV0M0eN_hCdKjJ0-qBZTn_KpLFfw","title":{"type":"String","value":"Rust Programming"},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"}},"score":19.8,"matched_fields":["description","tags","title"]},{"record":{"views":{"type":"Integer","value":677},"description":{"value":"Build modern web applications using JavaScript, React, and Node.js.","type":"String"},"title":{"value":"JavaScript Web Development","type":"String"},"id":"6cDbG8imc7J21Ty9zKw9eeXaHnu5MbKOkBoK2Rnuew_5TxS_Qfep4abmRD-iSQjahwVH_NgSPVEpPcKrRZzHSg","tags":{"value":"programming,javascript,web","type":"String"}},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":{"value":"Python for Data Science","type":"String"},"tags":{"value":"programming,python,data-science","type":"String"},"views":{"value":700,"type":"Integer"},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"id":"eazDudqZWoDYYzRo6hmbW5ouaaeGY2X2NyVLHf7addekkBpLFkDjYf6nG7R6_wqodqVcHgvGSxy1iZRl835qrg"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":{"value":"ai,machine-learning,python","type":"String"},"views":{"type":"Integer","value":421},"id":"WdF84UgTIc1xp2U-7Y-pU1osTBNWCEbdzgUv4U-Y8z9KsIfWnNp57a43Q4yehSiZYRQI5Xsalmq43hofgFgLoQ","title":{"type":"String","value":"Machine Learning Basics"},"description":{"type":"String","value":"Introduction to machine learning algorithms and neural networks."}},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"views":{"value":700,"type":"Integer"},"id":"eazDudqZWoDYYzRo6hmbW5ouaaeGY2X2NyVLHf7addekkBpLFkDjYf6nG7R6_wqodqVcHgvGSxy1iZRl835qrg","tags":{"value":"programming,python,data-science","type":"String"},"title":{"value":"Python for Data Science","type":"String"}},"score":1.0,"matched_fields":["description"]},{"record":{"views":{"type":"Integer","value":366},"description":{"value":"Learn database design principles, normalization, and query optimization.","type":"String"},"id":"2mNToBE7CndhFLe7TC6sGwW_Rnsp9iYUY6W8Poq3Wdn2FVmg_ZsKc-wFINkb_Kq_V2ow-bM9Wz_oLx3nF1e_-Q","title":{"value":"Database Design","type":"String"},"tags":{"value":"database,design,sql","type":"String"}},"score":0.5,"matched_fields":["description"]},{"record":{"tags":{"type":"String","value":"programming,rust,tutorial"},"title":{"type":"String","value":"Rust Programming"},"views":{"type":"Integer","value":551},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"id":"_nNkYbRNhwlb7VDegIj0GExapUKpEn3s4ZtZbSye3jPbw57_Xkiwk7avEbXV0M0eN_hCdKjJ0-qBZTn_KpLFfw"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":0}

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
✓ Inserted user 1: StringValue(value=S2Mz1HoWWMw6vZeVO94C-Qp-u08RwMB_WlK7QkAsEP0iQk5u18hIQgGTGfsidLW2nRX4soCHnDAqa6DguWKcGg)
✓ Inserted user 2: StringValue(value=I1CL5wi-cf9qZrW82-px35RknAJkjVPyMwBd42nzhJ62KXpBJwasJvmZp9MSIWPh1uPxERGb9ec3CAPxUNjP2w)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 29
  Sample: [schema_products_client_go, schema_products_client_js, schema_employees_client_ts, ws_ttl_test, schema_products_client_ts]

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
  Document ID: F1R5bc0wZ35Jr9MMkBVYuSYa7Um-uXvvUCtUu1Ziyh4xdXBFbaTIN1nwXbY-IL8puORAOGRvbHy3mEtFPy2aoA

=== Verify Document Exists ===
✓ Document found: created_at, id, user_id, session_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: Sgqka6NYLnvGOG-RP-qlDSWwC1TVW2xe9hTdTg1Q1VSIuNiXujinkaOMBkdOtjobkvzR06Xtw2u_TOiuFqsrQQ

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: id, cache_key, value, ttl

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
✓ Inserted document with TTL: StringValue(value=ukqnbutHBe7M3RpMrEABIhn6TL8vGRYE6NtUnG7PFfhkzFv-5M0mWx-mmTKE2MNC7pymw_oF8Yqv8bD62vRdIw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1774734633624},"id":"ukqnbutHBe7M3RpMrEABIhn6TL8vGRYE6NtUnG7PFfhkzFv-5M0mWx-mmTKE2MNC7pymw_oF8Yqv8bD62vRdIw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-03-28T22:50:33.732158Z","value":{"type":"Integer","value":42}}]}

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
✓ Edge cache script created: WmYe68ydVbRrlyILTXWcTWIPEYbL9_X2R6rTFnGgwPzCFPnaLrKImpIEifCCRi6Rpn-Iy3mhOGPHiSswWDJqeA

Call 1: Cache lookup
Found 1 cached entries
Response time: 28ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 17ms

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

✅ Script saved: T_ZYDJbY4SwbGAFc6MQaZt-wKQiIk8oXJtSJyFWWI5YH57g1UCVlvdFjQso0Uo7Y-5ngoKYfTpMEpWLll2dQTA
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
   {"status":"inactive","count":5,"avg_score":50.0}
   {"status":"active","avg_score":60.0,"count":5}
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

✅ Script saved: 77CmR5NpR6dgftT6vY-24rWw-pi0_X5UkymdNFDnzBKjxMxDkRH4W2XtIrT7aKvpg38ne8evq9hAURCCfXIlrg
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

✅ Inserted order: StringValue(value=Bh8LlLl7SNMyHxaZzWmMgdLZdeL0iLxqZqSa0ohE4UP0D6txJphVpZthdlu9xqeY-LjheHK7s9g-DBd-jqqnWg)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: D4uE6IP0v-3j_NBKKaOnxEGL6cS1OQSByCwz97InyiVarU9L0H08Gm0J0J5tj4lBqpBIiB0N1art0NFCW9oc7A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: zzMrNc5zmnh8BthqQxKRnCMj1DHtiNj7-aZOzNTqwEeQWFuURnrkJ-09RiLFTkBdp-y1k0MCR6o9EBq3zv1bVw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: W49OarivxQtICzeIOm14MAmnFlns0melVkf-psFBuh9Nl6WpHGTnhaiuKm_HjxxFbloAc2EFoHlBdhYLMmn5gg
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
✓ Created SWR script: swr_cache_lookup_kt (0CMqN3-NGySvXzZ29sF5HWtcrBP-Upsteck6vU4uTpxTLpq3IM7hzWjEeSG68NynVrO8QNGxFKr8c1lyQ37zIQ)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 20ms (served from cache)
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
✓ Created native SWR script: github_user_native (RefkXqrRRvnxlJXuChUsMECHzM2zq8rzYKMOy1jG2ijC0Fmr2ZptFt8fuQV7mT9a1GHbKa20kMdAFeIDl2onSw)

First call (cache miss - will fetch from GitHub API):
  Response time: 29ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 18ms
  Speedup: 1.6x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (VosA4J8tellFXF9t7No8D0bZWIv0GHXBDuCYd7J1d7YWMZtlWSP2Dqa61mj7Os2mNOKfcH4xGoa4eh8a_8riyA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (ancY3agzIHUXKy5szKjuzXxhalhlLDvHebDfZvE5n9475i1uciIwm8ApRnXMT2ukig6NF3YlGbho8J5Oo4aBBA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (gzPJuHYJulKybuum1SLxpXW18YdhR-SBRpMiCceGZQAbnoeRbCuLw2HGaBA8NKz-DlcGOy-m-clFFjLzjTG4xA)
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
   {"count":5,"avg_price":367.0,"category":"Electronics"}
   {"category":"Furniture","avg_price":365.6666666666667,"count":3}
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
   {"response":"Vector databases offer several benefits:\n\n1. **Efficient Similarity Search**: They enable quick retrieval of similar items based on vector embeddings, enhancing search capabilities.\n\n2. **High Dimensional Data Handling**: Ideal for managing complex data types like images, audio, and text by representing them as high-dimensional vectors.\n\n3. **Scalability**: Can efficiently scale with vast amounts of data, maintaining performance for large datasets.\n\n4. **Support for Machine Learning**: Integrates well with machine learning workflows, allowing for easy implementation of AI models that use embeddings.\n\n5. **Semantic Search**: Enhances search relevance by understanding context and meaning rather than relying solely on keywords.\n\n6. **Real-time Processing**: Facilitates real-time querying and updates, crucial for dynamic applications.\n\n7. **Flexible Data Types**: Can handle various data types, making them versatile for different applications.\n\n8. **Improved User Experience**: By providing more accurate and relevant results, they lead to better user interactions and satisfaction.\n\nOverall, vector databases enhance data retrieval, support advanced analytics, and improve application performance.","tokens_used":252}
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
   {"status":"inactive","count":3}
   {"status":"active","count":7}
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
   1. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
   2. {"type":"String","value":"Getting Started with ekoDB"} ({"type":"String","value":"Database"})
   3. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
   4. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
   5. {"value":"Vector Databases Explained","type":"String"} ({"value":"Database","type":"String"})
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
✓ Created session: imWdQr6dAlbvJPM94v-YCDfOEEH-KbD700wSLrpulMDcFgp4BU6AoYWEAA5RMeBbAjvnvFTeFLUEIzg4Z2DoWQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "PJaDioPZIXjkt2rkA3DBCReu2elI88-m6br0f-_jHN-UMs6GB-p_Xsx07RUJYWCXzOD3QLH17SQG2yd0_1E1Nw"
  Responses: ["ekoDB is a high-performance database designed with features that enhance data storage and retrieval, integrating intelligent caching and real-time capabilities alongside AI functionalities.\n\n### Key Features of ekoDB:\n1. **AI Chat Integration**: The chat feature allows you to query your database using natural language, providing AI-powered responses with relevant context.\n   \n2. **Search Capabilities**:\n   - **Full-Text Search**: Enables users to perform text-based queries over their data.\n   - **Vector Search**: Allows for semantic search capabilities, finding relevant data based on meaning rather than exact matches.\n   - **Hybrid Search**: Combines both full-text and vector search, automatically retrieving context to ensure the results are relevant.\n\n3. **High Performance**: ekoDB includes features that optimize performance, making it suitable for applications requiring rapid data access and processing.\n\nOverall, ekoDB emphasizes intelligent real-time data management, making it a robust choice for modern applications that require quick and contextual data retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 14s
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
✓ Created session: zRRBRwt0QRWb-0pRoIJ68c5Ltm3TNtXZmPeQ9_jXsB2sZXVQVBgThAEbsy8QUvlAFG4YJsPRXfT05r-rdJ-Elw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name**: ekoDB\n- **Description**: High-performance database product\n- **Price**: $99\n\nIf you need more information or have other questions, feel free to ask!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Currently, we have the following product available:\n\n- **Name**: ekoDB\n- **Description**: High-performance database product\n- **Price**: $99\n\nIf you need more information or details about additional products, please let me know!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: 232oPwBMiPujbijHs-T1VGa-KmKvfy_U4pAiE9v5uu30rcsrWeMfkpiJxdpTLq2kxzVxs5-Gdb0-NosTP6i-kw

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 2

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BNFzGlUcdlcM4B3W9Ic4VmdXLSz3YDDY3o9mUcg0YUY03VZaVwqTeTVQc0SEpr5r6GvsiKIsq8xt6jQglMPZeg

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Price**: $99\n\nIf you would like more details or additional products, please let me know!"]

✓ Message 2 sent
  Responses: ["The price of ekoDB is **$99**."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"BNFzGlUcdlcM4B3W9Ic4VmdXLSz3YDDY3o9mUcg0YUY03VZaVwqTeTVQc0SEpr5r6GvsiKIsq8xt6jQglMPZeg"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"4726vpg5zbnvipDBUlSuUtxsKX439JcdkBzGjdDL_L-8-AtsELRy-Y3Ts3uDbfS13M0gbqyVehTv8shehHOYrg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-03-28T21:52:00.196279Z"},"id":"Q96VzleeZmuxUHfAmST2aitgtzHLJJ4fzcDNtNiGX74HC4WtEoBNeo65c7rt1yAAoH2FZMy9q7160eESAvZGHw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":58,"prompt_tokens":2504,"total_tokens":2562}},"updated_at":{"type":"DateTime","value":"2026-03-28T21:52:00.196279Z"}},{"chat_id":{"type":"String","value":"BNFzGlUcdlcM4B3W9Ic4VmdXLSz3YDDY3o9mUcg0YUY03VZaVwqTeTVQc0SEpr5r6GvsiKIsq8xt6jQglMPZeg"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Price**: $99\n\nIf you would like more details or additional products, please let me know!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"4726vpg5zbnvipDBUlSuUtxsKX439JcdkBzGjdDL_L-8-AtsELRy-Y3Ts3uDbfS13M0gbqyVehTv8shehHOYrg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-03-28T21:52:00.206534Z"},"id":"0VSI-AxMvbiMG0mXviL_RV1iDdbnllhjpRnqdFNKz7hvNuL0ZvP-DW2P3MZQAcsFQRGfggkm9z7jvod2ZWFAzg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":58,"prompt_tokens":2504,"total_tokens":2562}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","limit":100,"select_fields":["product","price"]},"id":"call_xTaXGjysYwcu2BSb3AeUWBxm","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"id":"4726vpg5zbnvipDBUlSuUtxsKX439JcdkBzGjdDL_L-8-AtsELRy-Y3Ts3uDbfS13M0gbqyVehTv8shehHOYrg","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_xTaXGjysYwcu2BSb3AeUWBxm","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-03-28T21:52:00.206534Z"}},{"chat_id":{"type":"String","value":"BNFzGlUcdlcM4B3W9Ic4VmdXLSz3YDDY3o9mUcg0YUY03VZaVwqTeTVQc0SEpr5r6GvsiKIsq8xt6jQglMPZeg"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"4726vpg5zbnvipDBUlSuUtxsKX439JcdkBzGjdDL_L-8-AtsELRy-Y3Ts3uDbfS13M0gbqyVehTv8shehHOYrg","price":99,"product":"ekoDB"},"score":1.364516129032258}]},"created_at":{"type":"DateTime","value":"2026-03-28T21:52:03.063982Z"},"id":"RWeaH7KXYPML9JUMPXLBYJxj0VGvm7EKaFJ90gs07jfAN4HBLmeOpkKZHp6z4mmKlny2PY6z_LC9fEs3jKyu9A","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":7,"prompt_tokens":1263,"total_tokens":1270}},"updated_at":{"type":"DateTime","value":"2026-03-28T21:52:03.063982Z"}},{"chat_id":{"type":"String","value":"BNFzGlUcdlcM4B3W9Ic4VmdXLSz3YDDY3o9mUcg0YUY03VZaVwqTeTVQc0SEpr5r6GvsiKIsq8xt6jQglMPZeg"},"content":{"type":"String","value":"The price of ekoDB is **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"4726vpg5zbnvipDBUlSuUtxsKX439JcdkBzGjdDL_L-8-AtsELRy-Y3Ts3uDbfS13M0gbqyVehTv8shehHOYrg","price":99,"product":"ekoDB"},"score":1.364516129032258}]},"created_at":{"type":"DateTime","value":"2026-03-28T21:52:03.070545Z"},"id":"f55EsxIvgTQVDCdjCGpy9401uagP3QHrSFxdCtcIjiNIVt6p5X7Twq49oo7zJcn_NWyKySAbQlQT1F7xPhXNAQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":7,"prompt_tokens":1263,"total_tokens":1270}},"updated_at":{"type":"DateTime","value":"2026-03-28T21:52:03.070545Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: PzQSjLVq2ZUIOAQp_BfBjop5ddhiNKodiHDPuypveMlyPbboZKkQEvwwCFmMu46n0BXmxQPhVZbj7SUhaNfP7Q

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
Created user function with ID: Qs8yNAYZ80rh52DujM8bMWU8yp9smO6GoHJPNXqck2xP45O6NqeVKWs302rrHAg9ulo9F_jlTW_x5xchmeKL6Q

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 31 user functions:
  - "cache_api_call_py": "Cache External API Call"
  - "swr_user": "SWR pattern for user data (KV-based)"
  - "fetch_api_user": "Fetch User with Cache"
  - "fetch_and_store_user": "Fetch user from API and cache in KV"
  - "fetch_user": "Fetch user by code"

=== List User Functions by Tag ===
Found 31 user functions with 'kotlin' tag:
  - "cache_api_call_py"
  - "swr_user"
  - "fetch_api_user"
  - "fetch_and_store_user"
  - "fetch_user"
  - "cache_api_call_js"
  - "fetch_and_store_user"
  - "fetch_slim_user"
  - "validate_user"
  - "fetch_user"
  - "fetch_user"
  - "fetch_user"
  - "validate_user"
  - "fetch_and_store_user"
  - "swr_user"
  - "fetch_slim_user"
  - "fetch_slim_user"
  - "swr_user"
  - "cache_api_call"
  - "fetch_user"
  - "swr_user"
  - "validate_user"
  - "fetch_slim_user"
  - "fetch_product_with_reviews"
  - "swr_user"
  - "validate_user"
  - "validate_user"
  - "get_active_users_kt"
  - "fetch_and_store_user"
  - "fetch_and_store_user"
  - "fetch_slim_user"

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
✓ Created record: EkoRecord(fields={id=StringValue(value=90yHWTlpaTb06mOybuBpbLJLenEk80CfISXiBUTozZty2ADEs6spMczlqaSnr9832oA_GYd61kUjk5tROfm9Nw)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=90yHWTlpaTb06mOybuBpbLJLenEk80CfISXiBUTozZty2ADEs6spMczlqaSnr9832oA_GYd61kUjk5tROfm9Nw), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=NZI8FsIbe1tz8uHPZZA2yMw-FgsIQl-2s-nJvhoEWskQTn52a-Qta_VXn0TInsQt8lZ6a5biwEBQ2FkzKopAEg)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=90yHWTlpaTb06mOybuBpbLJLenEk80CfISXiBUTozZty2ADEs6spMczlqaSnr9832oA_GYd61kUjk5tROfm9Nw), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)})})
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=iNFu5FhgyXIHUteJa-HQB_aTK3zmGiZ1Vlp77ykxU8gBjiHCAjEe94rPG5WQDEpwmqK5odFFxpWKKe6blCqOYw)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=ZbtjE0yOdrH5yXEol0XMkoCbdqNHf6-aCwAk0Woo8YS1btbqBgtF5tMuVzn_GUyqJn636gkU9Jh2YeXv7VaZyQ)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={id=StringValue(value=iNFu5FhgyXIHUteJa-HQB_aTK3zmGiZ1Vlp77ykxU8gBjiHCAjEe94rPG5WQDEpwmqK5odFFxpWKKe6blCqOYw), price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)}), id=StringValue(value=iNFu5FhgyXIHUteJa-HQB_aTK3zmGiZ1Vlp77ykxU8gBjiHCAjEe94rPG5WQDEpwmqK5odFFxpWKKe6blCqOYw)})

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
Created Alice: $1000 - ID: 2E7JRW178FADXCqoGH9QjNxB0bC62Uy9Z64kl7c8X5wCxk5huuBhDyv-wGI8Mpg9c_AzQGoNdBrbgpPXAD086A
Created Bob: $500 - ID: wTvXv-50kWLe8_4gAaq1VsaKjtydM5XQTA_Kq31QhEDr9np5GcBzXkyLF4HrQVf0Pc5XAEFH4fRSIhtpsQ8r3g

=== Example 1: Begin Transaction ===
Transaction ID: 72fb996f-4595-4b1b-b943-6f6d7ce6df69

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c1e69370-00e9-4aee-b854-11faea723003
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
    ✓ Generated embedding: 1536 dimensions in 728.812958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.010184417s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 661.50475ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 535.629125ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 676.596ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 699.983958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 388.979583ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 575.590583ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 379.529209ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 327.352625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 589.277ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 564.855708ms
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
    ✓ Generated embedding: 1536 dimensions in 556.504958ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 101.070625ms

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

Writing memory-safe, high-performance database code involves multiple best practices across different programming paradigms. Here are several key areas to focus on:

### 1. Language and Database API Choice
- **Use Safe Programming Languages**: Opt for languages with strong memory safety features (e.g., Rust, Go, or managed languages like Java and C#). They often include built-in protections against common pitfalls like buffer overflows.
- **Choose Efficient Database APIs**: Use database drivers that are well-optimized and maintained. For example, native drivers for specific databases often provide better performance than generic ones.

### 2. Connection Management
- **Connection Pooling**: Utilize connection pooling to minimize the overhead of establishing connections. Most database libraries/frameworks support connection pools.
- **Properly Close Connections**: Always ensure that database connections are closed properly, utilizing language features like `try-with-resources` in Java or RAII principles in C++ to enforce this.

### 3. Query Optimization
- **Use Prepared Statements**: Prepared statements prevent SQL injection attacks and can also optimize performance by allowing the database to cache execution plans.
- **Batch Operations**: Instead of executing single insert/update statements, batch multiple operations together to reduce the number of network round trips.
- **Indexing**: Create the necessary indexes based on query patterns to speed up data retrieval. Regularly analyze and refine your indexes as application needs evolve.

### 4. Data Validation & Security
- **Input Validation**: Always validate and sanitize inputs to prevent SQL injection and other security vulnerabilities.
- **Limit Data Exposure**: Only fetch necessary fields from the database. Avoid `SELECT *` queries unless absolutely necessary to limit data transferred and improve performance.

### 5. Use of Transactions
- **Atomic Transactions**: Wrap related database operations within transactions to ensure data integrity. This includes using features like distributed transactions where needed but keeping them local when possible for performance.
- **Isolation Levels**: Choose the right isolation level based on your application needs to achieve a balance between performance and data accuracy (e.g., Read Committed vs. Serializable).

### 6. Error Handling
- **Graceful Degradation**: Implement error handling to manage database failures gracefully. Consider retry strategies for transient errors, especially in cloud environments.
- **Use of Logging**: Implement structured logging for database queries and errors to monitor performance and catch potential issues early.

### 7. Profiling and Monitoring
- **Database Profiling**: Regularly profile your database queries using tools (like EXPLAIN in SQL databases) to identify performance bottlenecks.
- **Application Monitoring**: Use application performance monitoring (APM) tools to track database query performance and identify long-running queries.

### 8. Efficient Data Structures
- **Choose Appropriate Data Structures**: Use data structures that are appropriate for your workload and database design to minimize memory usage. For example, using object-relational mapping (ORM) selectively can manage data more effectively.

### Conclusion
By combining the above practices, you can create high-performance, memory-safe database code that is robust, secure, and scalable. Each application may require additional specific techniques based on unique workloads or architectures, so iterative testing and adjustments are crucial.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 371.420292ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3591 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 343.767959ms
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
  ✓ Text search completed in 62.260583ms

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
    ✓ Generated embedding: 1536 dimensions in 0.534s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.429s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.447s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.514s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.501s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.755s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.822s
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
    ✓ Generated embedding: 1536 dimensions in 0.698s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.563s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.617s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.470s
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
    ✓ Generated embedding: 1536 dimensions in 0.681s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.088s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.331] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.300] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.300] From conv_database_design
     What is database normalization?

  4. [Score: 0.256] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.188] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe high-performance database code involves several best practices and techniques that help ensure both safety and efficiency. Here are key strategies to consider:

### 1. **Use Prepared Statements**
   - Avoid SQL injection by using prepared statements or parameterized queries. This not only enhances security but can also improve performance as the execution plan can be reused.

### 2. **Connection Pooling**
   - Implement connection pooling to manage database connections efficiently. Reusing a pool of connections minimizes the overhead of establishing new connections and improves performance for high-traffic applications.

### 3. **Batch Operations**
   - Group multiple operations into a single batch transaction to reduce round trips to the database. This can significantly boost performance, especially for insert/update operations.

### 4. **Indexing**
   - Use appropriate indexing strategies to speed up query performance. Analyze query patterns and create indexes on fields that are frequently searched or filtered. This reduces the amount of data scanned during queries.

### 5. **Query Optimization**
   - Write efficient queries by:
     - Selecting only necessary columns instead of using `SELECT *`.
     - Avoiding complex joins and subqueries when simpler operations suffice.
     - Analyzing query execution plans to identify bottlenecks and potential improvements.

### 6. **Resource Management**
   - Properly manage resources by closing database connections, statements, and result sets once they are no longer needed to prevent memory leaks.

### 7. **Data Validation**
   - Validate input data before it reaches the database layer. This can prevent unexpected input and help maintain data integrity while also reducing the risk of errors.

### 8. **Concurrency Control**
   - Implement mechanisms for managing concurrent access to the database, such as locks or optimistic concurrency control. This minimizes race conditions and potential data corruption during simultaneous transactions.

### 9. **Use of Transactions**
   - Utilize transactions to ensure that a series of operations are executed as a single unit of work. This helps in maintaining database integrity, and in case of failure, the database can be rolled back to its previous state.

### 10. **Monitoring and Profiling**
   - Continuously monitor database performance and profile your application to identify potential memory issues or performance bottlenecks. Tools like profilers or APM (Application Performance Monitoring) can be crucial for identifying issues.

### 11. **Scalability Considerations**
   - Design your database schema with scalability in mind. Consider aspects like sharding or data partitioning for handling larger datasets.

### 12. **Use Language-Specific Features**
   - If your application is developed in a language that provides memory safety features (e.g., Rust), utilize these features to prevent buffer overflows and memory leaks.

### 13. **Review and Refactor**
   - Regularly review and refactor your code to incorporate learned best practices and improve efficiency. Code reviews can also help in identifying anti-patterns and safety issues.

By integrating these practices into your development process, you can create database code that is both memory-safe and performs optimally under typical load conditions.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3359 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.435s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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
    ✓ Generated embedding: 1536 dimensions in 0.540s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.646s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.610s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.422s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.593s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.360s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.536s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.527s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.517s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.553s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.445s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.362s
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
    ✓ Generated embedding: 1536 dimensions in 0.570s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.074s
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

Writing memory-safe and high-performance database code involves several best practices across various layers of your development process. Here’s a comprehensive approach:

### 1. Use Prepared Statements
- **Why**: Prepared statements can help mitigate SQL injection risks and improve performance by allowing the database to optimize the query execution plan.
- **How**: Use parameterized queries in your database libraries.

### 2. Connection Pooling
- **Why**: Creating a new database connection for each request can be resource-intensive. Connection pooling reuses existing connections, improving performance.
- **How**: Use a connection pool manager provided by your database library or framework.

### 3. Error Handling
- **Why**: Proper error handling ensures your application can gracefully handle anomalies without crashing or leaking resources.
- **How**: Always check for exceptions when executing queries and handle them appropriately, using retry logic when necessary.

### 4. Resource Management
- **Why**: Not closing database connections or result sets can lead to memory leaks.
- **How**: Use `finally` blocks or try-with-resources statements in languages like Java or similar constructs in Python to ensure resources are released.

### 5. Batch Operations
- **Why**: Performing database operations in batches reduces the number of round trips to the database and can significantly improve performance.
- **How**: Use batch insert or update methods offered by your database API.

### 6. Indexing
- **Why**: Properly indexing your database tables improves query performance, especially for large datasets.
- **How**: Analyze your queries, identify frequently searched fields, and create appropriate indexes.

### 7. Code Optimization
- **Why**: Inefficient code can lead to excessive CPU or memory usage.
- **How**: Profile your application to identify slow queries or inefficient loops, and optimize these areas.

### 8. Use Transactions Wisely
- **Why**: Transactions ensure data integrity but can add overhead if not managed properly.
- **How**: Group changes into a single transaction when appropriate, and commit only when all operations are successful.

### 9. Cache Query Results
- **Why**: Caching frequently accessed data reduces the need for repetitive database queries.
- **How**: Use in-memory caching solutions like Redis or Memcached for high-read scenarios.

### 10. Monitor and Tune Performance
- **Why**: Continuous monitoring helps identify bottlenecks in your database access patterns.
- **How**: Use profiling tools specific to your database (like SQL Server’s Query Analyzer or PostgreSQL’s EXPLAIN ANALYZE) to analyze query performance and adjust as necessary.

### 11. Choose the Right Database for Your Use Case
- **Why**: Selecting between SQL and NoSQL based on your data structure can impact performance.
- **How**: Use SQL databases for structured data and complex queries; use NoSQL for unstructured data and high scalability demands.

### 12. Consider Schema Design
- **Why**: A well-normalized schema can reduce redundancy and improve integrity, while denormalization can help performance in certain read-heavy scenarios.
- **How**: Follow normalization principles but also assess when denormalization is beneficial for query performance.

### 13. Optimize Query Structure
- **Why**: Well-structured queries can optimize execution time and resource usage.
- **How**: Avoid SELECT *, only retrieve the fields you need, and structure WHERE clauses effectively to make use of indexes.

### Example of Safe and Efficient Database Code (in Python)
Here’s a simple example using Python with SQLite:

```python
import sqlite3
from sqlite3 import Error

def create_connection(db_file):
    conn = None
    try:
        conn = sqlite3.connect(db_file)
    except Error as e:
        print(e)
    return conn

def execute_query(conn, query, params=()):
    try:
        cursor = conn.cursor()
        cursor.execute(query, params)
        conn.commit()
    except Error as e:
        print(e)

def main():
    database = "example.db"
    conn = create_connection(database)

    with conn:
        # Sample table creation
        create_table_query = """CREATE TABLE IF NOT EXISTS users (
                                    id INTEGER PRIMARY KEY,
                                    name TEXT NOT NULL,
                                    age INTEGER
                                );"""
        execute_query(conn, create_table_query)

        # Inserting a user safely
        insert_user_query = "INSERT INTO users (name, age) VALUES (?, ?)"
        execute_query(conn, insert_user_query, ("Alice", 30))

        # Fetching users
        select_users_query = "SELECT * FROM users;"
        cursor = conn.cursor()
        cursor.execute(select_users_query)
        users = cursor.fetchall()
        for user in users:
            print(user)

if __name__ == '__main__':
    main()
```

Following these practices should help you achieve memory-safe and high-performance database operations in your application. Remember to keep monitoring and optimizing your code as your application grows and changes.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.824s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 5140 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.359s
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
  ✓ Text search completed in 0.054s
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
    ✓ Generated embedding: 1536 dimensions in 0.432s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.476s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.390s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.612s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.563s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.375s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.630s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.635s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.898s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.359s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.401s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.536s
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
    ✓ Generated embedding: 1536 dimensions in 0.559s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.075s

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

Writing memory-safe, high-performance database code involves several best practices that ensure efficient use of resources while maintaining data integrity. Here are key strategies you can apply:

### 1. Use Parameterized Queries
- **Prevent SQL Injection**: Always use parameterized queries to avoid SQL injection attacks. This not only secures your database operations but also optimizes query planning.
  
### 2. Connection Pooling
- **Reuse Connections**: Use connection pools to manage database connections efficiently. This reduces the overhead of establishing connections multiple times and improves performance.

### 3. Efficient Query Design
- **Indexing**: Ensure proper indexing on frequently queried fields to speed up data retrieval. Analyze query performance and create indexes as needed.
- **Batch Operations**: Use batch insert/update/delete operations instead of single-row operations to minimize database round trips.
- **Selective Data Retrieval**: Only select necessary fields instead of using `SELECT *`. This reduces the amount of data transferred and processed.

### 4. Utilize Transactions
- **Atomic Operations**: Use transactions to ensure that operations either complete successfully or fail without affecting database integrity. This is crucial for maintaining a consistent state in your database.

### 5. Memory Management
- **Resource Cleanup**: Ensure that all database connections, readers, and commands are disposed of properly after use to free up resources.
- **Handle Large Datasets Efficiently**: Use pagination or cursors for large datasets to avoid loading everything into memory at once.

### 6. Optimize Data Access Patterns
- **Caching**: Implement caching strategies to reduce the number of database hits. Use in-memory caches for frequently accessed data.
- **Read/Write Splitting**: Depending on your application needs, use read replicas for read-heavy operations to balance the load.

### 7. Data Normalization
- **Avoid Redundancy**: Organize data through normalization to eliminate redundancy and improve data integrity. Only denormalize if necessary for performance gains.

### 8. Monitor Performance
- **Profiling and Monitoring**: Use database performance monitoring tools to identify slow queries and resource bottlenecks. Optimize or rewrite those queries based on findings.

### 9. Utilize Asynchronous Programming
- **Non-blocking I/O**: Where applicable, use asynchronous methods to handle database operations to keep your application responsive.

### 10. Follow Language-Specific Guidelines
- **Leverage Language Features**: Use features offered by your programming language or framework that promote memory safety (like Rust's ownership system) and performance (such as Go's concurrency model).

By combining these practices, you'll be able to write database code that not only performs well but is also safe and maintainable. Always evaluate the specific requirements of your application to tailor these strategies effectively.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.788s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2986 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.488s
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
  ✓ Text search completed in 0.054s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices that ensure efficient use of resources while maintaining data integrity. Here are key strategies you can apply:

### 1. Use Parameterized Queries
- **Prevent SQL Injection**: Always use parameterized queries to avoid SQL injection attacks. This not only secures your database operations but also optimizes query planning.
  
### 2. Connection Pooling
- **Reuse Connections**: Use connection pools to manage database connections efficiently. This reduces the overhead of establishing connections multiple times and improves performance.

### 3. Efficient Query Design
- **Indexing**: Ensure proper indexing on frequently queried fields to speed up data retrieval. Analyze query performance and create indexes as needed.
- **Batch Operations**: Use batch insert/update/delete operations instead of single-row operations to minimize database round trips.
- **Selective Data Retrieval**: Only select necessary fields instead of using `SELECT *`. This reduces the amount of data transferred and processed.

### 4. Utilize Transactions
- **Atomic Operations**: Use transactions to ensure that operations either complete successfully or fail without affecting database integrity. This is crucial for maintaining a consistent state in your database.

### 5. Memory Management
- **Resource Cleanup**: Ensure that all database connections, readers, and commands are disposed of properly after use to free up resources.
- **Handle Large Datasets Efficiently**: Use pagination or cursors for large datasets to avoid loading everything into memory at once.

### 6. Optimize Data Access Patterns
- **Caching**: Implement caching strategies to reduce the number of database hits. Use in-memory caches for frequently accessed data.
- **Read/Write Splitting**: Depending on your application needs, use read replicas for read-heavy operations to balance the load.

### 7. Data Normalization
- **Avoid Redundancy**: Organize data through normalization to eliminate redundancy and improve data integrity. Only denormalize if necessary for performance gains.

### 8. Monitor Performance
- **Profiling and Monitoring**: Use database performance monitoring tools to identify slow queries and resource bottlenecks. Optimize or rewrite those queries based on findings.

### 9. Utilize Asynchronous Programming
- **Non-blocking I/O**: Where applicable, use asynchronous methods to handle database operations to keep your application responsive.

### 10. Follow Language-Specific Guidelines
- **Leverage Language Features**: Use features offered by your programming language or framework that promote memory safety (like Rust's ownership system) and performance (such as Go's concurrency model).

By combining these practices, you'll be able to write database code that not only performs well but is also safe and maintainable. Always evaluate the specific requirements of your application to tailor these strategies effectively.

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
    ✓ Generated embedding: 1536 dimensions in 0.456s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.446s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.479s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.561s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.343s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.812s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.515s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.622s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.508s
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
    ✓ Generated embedding: 1536 dimensions in 0.521s
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
    ✓ Generated embedding: 1536 dimensions in 0.643s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.086s

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
    ✓ Generated embedding: 1536 dimensions in 0.608s
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
  ✓ Text search completed in 0.043s

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
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (yaKaabt3yoIBB59l9xYIVGZyDGVwCliVCaMrNW1wRNLD8b1g-6Lyh5etQfRkJ5spwUGoyYnOLzxEm-9pOlCPJg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "username": "Bret",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
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
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
Response time: 41ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "username": "Bret",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
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
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
✓ Created enrichment script: fetch_product_with_reviews (lkGEDuVPxpQT4cjl_1dvOpkjN-w5t2rVDaPoZ_mf_NYs5iovkxzTRBdNhN76siSHbW2tziFiRTO3ioaqXKpOIA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "rating": 3,
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          },
          {
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "rating": 4,
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerName": "Eleanor Collins",
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com"
          }
        ],
        "sku": "BEA-ESS-ESS-001",
        "price": 9.99,
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "availabilityStatus": "In Stock",
        "meta": {
          "barcode": "5784719087687",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "warrantyInformation": "1 week warranty",
        "weight": 4,
        "rating": 2.56,
        "brand": "Essence",
        "discountPercentage": 10.48,
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "returnPolicy": "No return policy",
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "category": "beauty",
        "minimumOrderQuantity": 48
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
✓ Edge cache script created: 0nrkq8lDR0NfE4KM-U1QDoUxNmcDFjAERiaQobHkMwcxVop76la0MDgQO8ihQdhwJmofmuoMrAPBd_w5K0nSgQ

Call 1: Cache miss (fetches from API)
Response time: 165ms
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "id": 1,
        "name": "Leanne Graham",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "address": {
          "city": "Gwenborough",
          "zipcode": "92998-3874",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "street": "Kulas Light"
        },
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442"
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
Response time: 42ms (3.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "id": 1,
        "name": "Leanne Graham",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "address": {
          "city": "Gwenborough",
          "zipcode": "92998-3874",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "street": "Kulas Light"
        },
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442"
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
    Finished `release` profile [optimized] target(s) in 0.09s
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
✓ Created SWR script: fetch_github_user (H5fBDjk2AybbX31RgTmt4D3wxT8_3eZTO50xoX0dBeFnfTdWTOmCGlIV_wdBxa1fLJJQrC0lWR03qGKMjA8UCg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 181ms
Result: [
  {
    "cached_at": "1774734878",
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
        "followers": 293467,
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
✓ Created enrichment script: fetch_product_enriched (jR0aNURePoYoKffMPdPJAK6vFs3SxTFM76OneN4xgJsQJjqgVIWm3xevWrEMx9cPyX_4oWywwA0rlZxWBO36QQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1774734878",
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
✓ Created SWR script: fetch_github_user (_a2xjJuhLczb4GtoB1gGpS-cP4Azs5DFDlGeGX1Oef8fSVI0kiZB4_EkBHRaWQrWjXUiMKUN6E6F91_w7b-jNQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 39.571083ms
Result: [
  {
    "cached_at": "1774734878",
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
        "followers": 293467,
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
Response time: 40.710125ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (hXAE9GJ6vtavt4ERzsNSWQpavwJe3BUeoe8FxYrjiY5Vv0nTtq0vin9DtQnBqT1wWwXVNjM6r_RYPpKKEQZI6A)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (HkPArW8JUmJ5Kc0i1TdIN1PiXr9yPSdX6FhNrjEsorha8-byaL0FeSy63uddeNaOmRDMX28DPb0rYEUt6xGBLg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 41ms
Result: {
  "data": {
    "value": {
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "url": "https://api.github.com/users/torvalds",
      "site_admin": false,
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "hireable": null,
      "user_view_type": "public",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "html_url": "https://github.com/torvalds",
      "location": "Portland, OR",
      "bio": null,
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "company": "Linux Foundation",
      "twitter_username": null,
      "gravatar_id": "",
      "login": "torvalds",
      "updated_at": "2026-01-13T07:10:05Z",
      "created_at": "2011-09-03T15:26:22Z",
      "public_repos": 11,
      "type": "User",
      "following": 0,
      "id": 1024025,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "name": "Linus Torvalds",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "blog": "",
      "email": null,
      "followers": 293467,
      "public_gists": 1
    },
    "type": "Object"
  },
  "cached_at": "1774734878"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 42ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (Yfy9Sd1DZdTXi3dCqCef7-Vznn9C8EcQU4mG-g91nC_KaI88hTEGjiLz0xhkpPQRCeCOC8I3OzSlgtUR0PRfGg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "value": {
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "sku": "BEA-ESS-ESS-001",
      "weight": 4,
      "id": 1,
      "category": "beauty",
      "discountPercentage": 10.48,
      "reviews": [
        {
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "rating": 3,
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins"
        },
        {
          "reviewerName": "Lucas Gordon",
          "rating": 4,
          "comment": "Very satisfied!",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com"
        },
        {
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Highly impressed!",
          "reviewerName": "Eleanor Collins",
          "rating": 5
        }
      ],
      "tags": [
        "beauty",
        "mascara"
      ],
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "warrantyInformation": "1 week warranty",
      "rating": 2.56,
      "meta": {
        "updatedAt": "2025-04-30T09:41:02.053Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "barcode": "5784719087687"
      },
      "shippingInformation": "Ships in 3-5 business days",
      "minimumOrderQuantity": 48,
      "availabilityStatus": "In Stock",
      "price": 9.99,
      "title": "Essence Mascara Lash Princess",
      "stock": 99,
      "brand": "Essence",
      "returnPolicy": "No return policy",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "dimensions": {
        "height": 13.08,
        "width": 15.14,
        "depth": 22.99
      }
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
✓ Created SWR script: fetch_github_user (lhn1I-CyDsw-kYa-NN31aA3u78K612LD4S3jXXbbc0Aw5zV98eI9fDVySXN2c9M0J0QV0_2mmhLswfNZLHNGXg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 56ms
Result: [{"data":{"value":{"url":"https://api.github.com/users/torvalds","events_url":"https://api.github.com/users/torvalds/events{/privacy}","company":"Linux Foundation","type":"User","email":null,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","html_url":"https://github.com/torvalds","location":"Portland, OR","login":"torvalds","node_id":"MDQ6VXNlcjEwMjQwMjU=","updated_at":"2026-01-13T07:10:05Z","user_view_type":"public","twitter_username":null,"following_url":"https://api.github.com/users/torvalds/following{/other_user}","organizations_url":"https://api.github.com/users/torvalds/orgs","public_gists":1,"following":0,"created_at":"2011-09-03T15:26:22Z","followers":293467,"id":1024025,"name":"Linus Torvalds","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","gravatar_id":"","public_repos":11,"site_admin":false,"received_events_url":"https://api.github.com/users/torvalds/received_events","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","followers_url":"https://api.github.com/users/torvalds/followers","repos_url":"https://api.github.com/users/torvalds/repos","blog":"","hireable":null,"starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","bio":null},"type":"Object"},"cached_at":"1774734878"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 44ms
Cache hit was 1.3x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (KxWP4-bGVRlPSk9LFmZJpFVQamJp39IvJxIWzO5R5C0bHRGXdUNWiHD-ooNOHAtXwuPeVBTcolF2HamCXB9ZWw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"value":{"tags":["beauty","mascara"],"brand":"Essence","warrantyInformation":"1 week warranty","weight":4,"returnPolicy":"No return policy","images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","reviews":[{"rating":3,"reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z","comment":"Would not recommend!","reviewerName":"Eleanor Collins"},{"rating":4,"reviewerEmail":"lucas.gordon@x.dummyjson.com","reviewerName":"Lucas Gordon","comment":"Very satisfied!","date":"2025-04-30T09:41:02.053Z"},{"comment":"Highly impressed!","rating":5,"reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","date":"2025-04-30T09:41:02.053Z"}],"stock":99,"price":9.99,"dimensions":{"depth":22.99,"height":13.08,"width":15.14},"title":"Essence Mascara Lash Princess","description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","id":1,"availabilityStatus":"In Stock","rating":2.56,"category":"beauty","meta":{"qrCode":"https://cdn.dummyjson.com/public/qr-code.png","updatedAt":"2025-04-30T09:41:02.053Z","createdAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687"},"minimumOrderQuantity":48,"discountPercentage":10.48,"shippingInformation":"Ships in 3-5 business days","sku":"BEA-ESS-ESS-001"},"type":"Object"}}]
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
   ⏱️  Duration: 155.844458ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "address": {
      "suite": "Apt. 556",
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 42.088542ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "address": {
      "city": "Gwenborough",
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      }...
   🚀 Cache speedup: 3.7x faster!

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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "id": 1,
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romag...

Second call (cache hit - from cache):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "id": 1,
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romag...
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
    Finished `release` profile [optimized] target(s) in 0.08s
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
   ⏱️  Duration: 40.1ms
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
   ⏱️  Duration: 40.2ms
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
   ⏱️  Duration: 40.367833ms
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
   ⏱️  Duration: 40.236667ms
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
    "name": "Leanne Graham",
    "id": 1,
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romag...

Second call (cache hit - from cache):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "id": 1,
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romag...
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
✓ Subscribed (subscription_id: sub_e10897dbc9244a64ab2eb50219e8ce5e)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "oZxs6tYVOLL7Y2vWauzw5x1iDMzeFWlGxptipJb6YfbQQs1SRaXp1WbrJQcZf4uSssMF1BfOjSRArhJL9wrOJg"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["oZxs6tYVOLL7Y2vWauzw5x1iDMzeFWlGxptipJb6YfbQQs1SRaXp1WbrJQcZf4uSssMF1BfOjSRArhJL9wrOJg"]
     Timestamp:  "2026-03-28T21:54:52.554598+00:00"

Inserting record 2...
✓ Inserted: "_6NbordpNj2Es55o3Tk0JIVL27-72_gkAjWAbH5QRdFPHJIXXe48AiP3A0q4mY8EFvxfnDKGU0luef2k3pRdCg"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["_6NbordpNj2Es55o3Tk0JIVL27-72_gkAjWAbH5QRdFPHJIXXe48AiP3A0q4mY8EFvxfnDKGU0luef2k3pRdCg"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mRust subscription test complete![0m

🔷 [33mGo WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_5ebb204562b54cbfb2952410e4045bc7)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: yQFvlPuye-t-OmnEV0PdrzCHETMyQeBx1EF2InXZJwR6GwqaMuUodEQCBFzysXzdRkTTPPx9sexA3oa6TwpkeQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [yQFvlPuye-t-OmnEV0PdrzCHETMyQeBx1EF2InXZJwR6GwqaMuUodEQCBFzysXzdRkTTPPx9sexA3oa6TwpkeQ]
     Timestamp:  2026-03-28T21:54:52.680071+00:00

Inserting record 2...
✓ Inserted: RK6raHGOu40mSR18KvqF0U09HNP9Zs8Iub0lkh4hOCb2sibWGcMfsbxNDcuJdK-vS9bF0WWloLH7CxFJ7EE20g

  📡 Notification received:
     Event:      insert
     Record IDs: [RK6raHGOu40mSR18KvqF0U09HNP9Zs8Iub0lkh4hOCb2sibWGcMfsbxNDcuJdK-vS9bF0WWloLH7CxFJ7EE20g]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mGo subscription test complete![0m

🐍 [33mPython WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_b2073a32613845fcb84fccf6fd40ee21)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: kypf9x-JAivDa5mk_hZUkBXn1vcpIs7jlkJg4y9n73FbN5hYMIpR4_OcYqeKerKiv_Inq0ll0R4h3QyWwKJRZg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: kypf9x-JAivDa5mk_hZUkBXn1vcpIs7jlkJg4y9n73FbN5hYMIpR4_OcYqeKerKiv_Inq0ll0R4h3QyWwKJRZg
     Timestamp:  2026-03-28T21:54:52.803359+00:00

Inserting record 2...
✓ Inserted: xrUscaKzBl5px0joafrz8OjL9OxHEXSAtKa72YC55RR9UV8X11RSmEXMPl496ZCQaFtbh5ZlDa1m91L5l311SQ

  📡 Notification received:
     Event:      insert
     Record IDs: xrUscaKzBl5px0joafrz8OjL9OxHEXSAtKa72YC55RR9UV8X11RSmEXMPl496ZCQaFtbh5ZlDa1m91L5l311SQ

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
✓ Subscribed (subscription_id: sub_432b7e67f0174bbe88e0b07d02ce26a2)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: uPzFvejb4nNzAC0qDeojCqN9pX-WtSjVV87fCg_KSYBdOEvC81bpAK-Q9jXuv0AAb8VGGgu9Dz_rwmw_RV5cXA
     Timestamp:  2026-03-28T21:54:55.283866+00:00
     Records:    {"active":true,"id":"uPzFvejb4nNzAC0qDeojCqN9pX-WtSjVV87fCg_KSYBdOEvC81bpAK-Q9jXuv0AAb8VGGgu9Dz_rwmw...
✓ Inserted record: uPzFvejb4nNzAC0qDeojCqN9pX-WtSjVV87fCg_KSYBdOEvC81bpAK-Q9jXuv0AAb8VGGgu9Dz_rwmw_RV5cXA

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: 6kZtalji12q6D-eAvbgWe7zyikhs7iJ1hx-oZ4DcP9U6DB-gQ_JDW5hLGh8Inp6rKrAc7Z__LyC23txFU2AEuA
     Timestamp:  2026-03-28T21:54:56.306679+00:00
     Records:    {"active":true,"id":"6kZtalji12q6D-eAvbgWe7zyikhs7iJ1hx-oZ4DcP9U6DB-gQ_JDW5hLGh8Inp6rKrAc7Z__LyC23tx...

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
✓ Subscribed (subscription_id: sub_0d6e11fc5a624582807aec4d0d28ded8)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "OjzcUNk_jTSzuG6pdm9S0JpHhjUxLZTiOM4a2GeV-NPp8HnFVRg9oQSWp86JXbq07UQcrgTQ6KXr6ny07sJ8oQ"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["OjzcUNk_jTSzuG6pdm9S0JpHhjUxLZTiOM4a2GeV-NPp8HnFVRg9oQSWp86JXbq07UQcrgTQ6KXr6ny07sJ8oQ"]
     Timestamp:  "2026-03-28T21:55:01.121204+00:00"

Inserting record 2...
✓ Inserted: "0nSDmXfidnv3yA0ZApN-2J0qi_dYVRK-4z3wZJg_DRUJCxYlUn_SKO5vTL9Y5v1AnNeFNXbQlsRYSquHd48NpQ"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["0nSDmXfidnv3yA0ZApN-2J0qi_dYVRK-4z3wZJg_DRUJCxYlUn_SKO5vTL9Y5v1AnNeFNXbQlsRYSquHd48NpQ"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin subscription test complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll WebSocket Subscription Tests Passed![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
