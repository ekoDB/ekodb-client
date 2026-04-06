make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("FsUZknnGZurTxrZGPQhj7xgrp4ces1RxBpU_oXK4BYQuc2PKkCl8p0AuDcGzk0L7tkKki_-z1XH9JGCt3AIjMw")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("FsUZknnGZurTxrZGPQhj7xgrp4ces1RxBpU_oXK4BYQuc2PKkCl8p0AuDcGzk0L7tkKki_-z1XH9JGCt3AIjMw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("FsUZknnGZurTxrZGPQhj7xgrp4ces1RxBpU_oXK4BYQuc2PKkCl8p0AuDcGzk0L7tkKki_-z1XH9JGCt3AIjMw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("FsUZknnGZurTxrZGPQhj7xgrp4ces1RxBpU_oXK4BYQuc2PKkCl8p0AuDcGzk0L7tkKki_-z1XH9JGCt3AIjMw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "-f0h1copZhZ1-1UgYMQ-hXlAMY2TRqZRvIjv-flcOdAOifuyhypOS0w1NG7mqVGDd1BEUcdA_hwXHEBHJ__3og"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775485780734627000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "-f0h1copZhZ1-1UgYMQ-hXlAMY2TRqZRvIjv-flcOdAOifuyhypOS0w1NG7mqVGDd1BEUcdA_hwXHEBHJ__3og",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "fkX3ysIB6dmUPmiEVsbNiHKcJ4uRNqStt32UEB7Fiz0eZm-GqFR9lmHEPhQ52L8t1Fx8jK9ypzbUG3zCHa8Taw"

=== List Collections ===
Total collections: 5
Sample collections: ["test_collection", "demo_collection", "websocket_test", "batch_users", "audit__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "XfgwCxk5ZJZURqliPhVZoHkoM7WlAuHiC2wpwbu_WuuQO7hrc4_y_pT5Xa9h-vR6NTtVfIyMyk04ohjtFWsoLg"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "QAUWbjNOaJWDjU1ctkLBOVpoeME-0EvtY0eO1qU_LKCQSgiJgQgkynBWezx1rLpMESIomUBTE3DMZXt_BmajcA"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "WJGvvv0DsGc4MyVTu5hTiZA-QBm_HvV8ieksisC2KO524OHuYi9PreFnBI5cCCG6tSl8E-FOhVz4XB_x8Egr4Q"

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
✓ Inserted document with TTL: String("jc7DlqywGDI3ZYmAYmr9S-MY5M2ffwrG3OdO_KqlzebcHQRoldNnRQouqQqKFEd47zF8h8PZxT4BK6PFOWGZ3Q")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "jc7DlqywGDI3ZYmAYmr9S-MY5M2ffwrG3OdO_KqlzebcHQRoldNnRQouqQqKFEd47zF8h8PZxT4BK6PFOWGZ3Q"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-04-06T15:30:39.904535Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: y-rO52eUhKeCFoIm95-yVH4OJoSu3BTKu53lTemoLefAuInNGDnhLw_CvRHW0t9izkn9_yPh-3CK3esdggf-mA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: "XN1vGkrC928X8BFH3B6keNHMqCx2T8Vzy_7b-PqJerpfJLtkmVjt-6gslY5247nhIAz_Z-CCCRlJIxeq3AFTmw"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: -hPFPvbyeKN7obVn5z3lImhX624rab7qKncUTmR5k2D6_o5Iynv-jnFRxnxzCjmoPkdVtV8Ypsq7BJa06BTCEQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: sgFbc9NhMa0sYG412KmU5hj9ZtfTSTjauZyDeoGwvZ34-1Vh0DgycQagBjZFAiFlM-NGKcLAb_1xwzW_PcQHXw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}

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
Created Alice: $1000 - ID: WREPLsKEufebT6x1OLwKSqM1EEoWYLaYctFHhfcDmg9784V7J5pbo0SVIFgxhrJeiv-gcQ-dVmLeLuXsiXHNIQ
Created Bob: $500 - ID: dBp4Q-Vwf0qyEWfFgKKePt64cGChWsf6UMTrq1Ry5RRmJMz70Q7PQKB_g1IheN5mlZ-EglvbVf2Rl-07aiofRw

=== Example 1: Begin Transaction ===
Transaction ID: 586aba28-a162-4506-854a-3721b36606c8

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
New transaction: 846a85d8-0392-4ce4-b56a-22c48bf7532c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("UslzYhq07yIqFCTlxa2KDFF8hBQfcmUA61krLDiZCmnlv0weg51Yq_8AgeSq6ADgOgEjrJfemJ381DxFkDl9hg")})

=== Find by ID ===
Found: Record({"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "id": String("UslzYhq07yIqFCTlxa2KDFF8hBQfcmUA61krLDiZCmnlv0weg51Yq_8AgeSq6ADgOgEjrJfemJ381DxFkDl9hg"), "price": Object({"type": String("Float"), "value": Float(99.99)}), "name": Object({"type": String("String"), "value": String("Test Record")}), "created_at": Object({"value": String("2026-04-06T14:31:10.936812Z"), "type": String("DateTime")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-04-06T14:31:10.936812Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-06T14:31:10.936812Z")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "id": String("UslzYhq07yIqFCTlxa2KDFF8hBQfcmUA61krLDiZCmnlv0weg51Yq_8AgeSq6ADgOgEjrJfemJ381DxFkDl9hg"), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"value": Float(99.99), "type": String("Float")})})]

=== Update Document ===
Updated: Record({"metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "created_at": Object({"value": String("2026-04-06T14:31:10.936812Z"), "type": String("DateTime")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "id": String("UslzYhq07yIqFCTlxa2KDFF8hBQfcmUA61krLDiZCmnlv0weg51Yq_8AgeSq6ADgOgEjrJfemJ381DxFkDl9hg"), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "value": Object({"value": Integer(100), "type": String("Integer")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: JZk1Cpas73Ot6DdB0BDMVdLijEM9JRDKLMMBOJqmY9Seqo1WSvJv7qHbWR3xN3qMLTR4wTiZV0M5WnFWKeIdCQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  id: "JZk1Cpas73Ot6DdB0BDMVdLijEM9JRDKLMMBOJqmY9Seqo1WSvJv7qHbWR3xN3qMLTR4wTiZV0M5WnFWKeIdCQ"
  name: "WebSocket Test Record"
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
Collection created with first record: "lgauTYfQvDxDo-K74P4H30NcQMf9NS7jS7HDe7LTCQjLu8p4G66jeFpnqAoiryX9YJgncV_fJMG2JPLkJc_a0w"

=== List Collections ===
Total collections: 10
Sample collections: ["test_collection", "functions__ek0_testing", "client_collection_management_rust", "websocket_test", "batch_users"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: q0wwoWFhbXFGQcbeIJ0rzfiMa6Rof2qxjSftGjqloXsJKviUWpmpQYTKVlR_X5vx0pM_3-pAOvOxfK43-n8Gvw
Created Bob: $500 - ID: qojAMhgh_ijSLydE0DcQH_qMK0-XwKgKHNlt0hY_px_cP60yHlrXatgrsDMMCL4n2mo2ia3o8ljHduA4-i-08A

=== Example 1: Begin Transaction ===
Transaction ID: 361ff5e6-ef9a-4f9f-925d-ddc805af11d6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 013d80f7-9f57-4605-a567-ead5a005659b
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
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

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
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

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
     Matched: ["title.value", "title", "description.value", "description"]
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
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("iHbhHefHArt7Z4SWQwFHvVIGyDkSGwgYmnXdfBxMo64DlU3JsQOFDmL7f5i92HzER0IExkLXBnI4UNrXQtG2QQ"))
✓ Inserted user 2: Some(String("JDM2wIrYugKtDFz4WAxawrK-X2cE_Uc5lL6F9Hw63k0pFn0LqUxrfMThnJsJYkA5WTGECox1M7wAMtT7PZ6YfQ"))

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
  Sample: ["test_collection", "functions__ek0_testing", "websocket_test", "batch_users", "ttl_cache"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
✓ Inserted document: "zGfw0OEbDRzc4rq3BfOE9D8bzJsSkgQ-cwmMJ9MYLVF5uqR9tZYjjFuIoSGQrFYpzVKhhAI23njzp1MLx1_FQw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("FWjcHEzVT6ssZXSriSk1qd1xVVKELD0P-rsrGd2gMZvdBQwqztqwIJEdmTRwhOBaTfN06kUFg_MZ7omklFemhg"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("TL5Irdal1eR540SLnjnKN-tqZEJI2mEEpKFrBhQFj81-QID0iANNIFARduoVCyDHiFMFXAwEcy_zKlF4GhYmzg"))

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
✓ Edge cache script created: lWuhZKV6SCuoaXxFHntRbsNfEjycHkL3i6U3Rmkgg5eVhGwrcthsYwh-T1D185lXPaMQeGTqdgOIGATiFCnK1Q

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
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: -miOStQfa8iC85TrC0PyNts0QGppnnsKH-DXQxNij2_hjZAkcHmPIPPdh8IaLx_q0b-W4yvP7FeqquRbnolcOw
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: zsSr11Z7C11NtXaBOLsaaJ0Go1GamtME0YKeKHb6DRj-QUoR7FADPaWdp1o4WPvhgwsR5cnRUmcHkHNmlrR9tw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: juCTokPffUh4R8KT23AzpDI0dRL7_Yq7Q9YRUPyiwlfWh0ycs-luakufhZfIr1dsXzzVZBlhDJfWdNGvlq6bFg
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 6
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
   ⏱️  Duration: 204.488625ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "username": "Bret",
    "id": 1,
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "...

Second call (cache hit - from cache):
   ⏱️  Duration: 40.284458ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "phone": "1-770-736-8031 x56442",
    "id": 1,
    "website": "hildegard.org",
    "company": {
      "bs": "harness real-time e-markets",
      "catchP...
   🚀 Cache speedup: 5.1x faster!

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

✅ Function saved: sgArKrFLMiuSJwEyFMxM1dFzm7HflaLWcnTamK04GR4tvgsJwL40FKLWzRxZsBacq2lfgd9-VsDcoaVk3pl1zQ
📊 Found 2 product groups
   Record({"count": Integer(2), "category": String("Furniture"), "avg_price": Float(474.0)})
   Record({"category": String("Electronics"), "avg_price": Float(575.6666666666666), "count": Integer(3)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Integer(2), "category": String("Furniture")})
   Record({"category": String("Electronics"), "count": Integer(3)})
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

✅ Inserted order: Some(String("x6kvlMvCPw_Z1wSe42RGoh5iJKhbENuIIZm5zUYxQr9H_VP6aoPg0JsxBmW2b4qS_vh7gLhds7CUh__lzlLdXg"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: GwMUW8ebD0oB7PcRm-EBDr-bzrS-L5viIAXCZoBQ2N4Ythlou5FyM3Dll7elFwjEhLsDLrI4KK4GErBQwP-9EA
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: J-B_lOCqle2BhCxt-iLeer0c3fX_vkxyUQmJ3Su13MvljW-ii4DxRizLTl3FUSdRtVpmYzKvUqiOnnDUYgmn7Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: COjvEadCqELiA6yWYdBrFvA0Am93TUoNVjAJNZ4XmduYTwSCPh9JMGcoD2xfx4qB-XwiUDfi5KhQMvqaLoBMcA
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
✓ Created SWR script: fetch_api_user_rs (qZKSd4qkFfVCaqFwC6Ja5gvQQREs9FEqZcit8H_YulHyj96KiP4kvghzoOvGRJvkDYCyXoV0sFDtp9278LlTTA)

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

✓ Created native SWR script: github_user_native (AvyMuvBb9_epVHttC2K2GPj9NmwsK1dhsp8mXIa0FeIESKnh1_bneGNcJ29h3jDBhdT8veti9ZKz8UjuauvsEQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 138ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 6.9x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (-YyXlQCzWaEEjFLEb2jrJA5OzkUYInHHQwszq6gvoyCksFDhjzS3IgGROoxyPRP6vTDN5T7dQ9o9-Er5KsrOkA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (d_5-1XJ1RNcYzWxhneDWh-gnukFwndwpahbvwJTtjL2iyY9eScjqxwWx6aNXAeBn-BFE7xsQwdyc5DlBqh16dg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (M5XQELaO3_CwAakyOID-IazR8_hjW9t0zNnSgIut4FyHT9HY56pKe9JO3QA4Fi0uBoi2FxT-yMNfGaOAYLtP3A)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   Record({"category": String("Furniture"), "count": Integer(3), "avg_price": Float(365.6666666666667)})
   Record({"category": String("Electronics"), "count": Integer(5), "avg_price": Float(367.0)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast similarity searches for high-dimensional data, making them ideal for applications like image and text retrieval.

2. **Scalability**: Designed to handle large datasets, they can manage billions of vectors efficiently.

3. **Real-time Performance**: Many vector databases support real-time querying, enabling quick responses for applications like recommendations and personalized search.

4. **Advanced Machine Learning Support**: They seamlessly integrate with machine learning models to store, retrieve, and analyze complex data representations.

5. **Flexibility**: Vector databases can handle various data types, including images, text, and audio, making them versatile for different applications.

6. **Enhanced Accuracy**: By representing data in high-dimensional spaces, they improve the accuracy of finding relevant results based on semantic meaning rather than just keywords. 

7. **Built-in Indexing and Optimization**: Many vector databases come with optimized indexing methods, such as Annoy, HNSW, or IVFPQ, to speed up search operations. 

8. **Support for Unstructured Data**: They are suitable for managing unstructured data and perform well in tasks involving natural language processing and computer vision.
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
   Record({"status": String("inactive"), "count": Integer(3)})
   Record({"count": Integer(7), "status": String("active")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": String("AI"), "count": Integer(2)})
   Record({"category": String("Database"), "count": Integer(3)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: ykweq9aT3ctDaaqDgEjReoyIRQ1dhjQNHffFgYGiE3kvyd940dzT2H1H6vbzPBdbe6rREtQ0tSrgTG49AyryPw

=== Sending Chat Message ===
Message ID: gadu7r37_z6j2EX0ZLrlpjbNuStzHNSC_mV0wcHWa1p2svtaf4A-1t612Vrq3CLsvLQ9tvtbX-YTTY0pTBw7PQ

=== AI Response ===
Response 1: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It provides various features that enhance its functionality, including:

1. **AI Chat Integration**: ekoDB allows you to query your database using natural language, enabling AI-powered responses with relevant context.

2. **Search Features**: It supports multiple types of search capabilities, including full-text search, vector search, and hybrid search with automatic context retrieval.

These features make ekoDB versatile and efficient for handling data with integrated AI capabilities.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("7o0M95N0hjrtiyPrHrlegVNdCbnlG7DbosJuBj4ECSB_TZaIHliq6QPGyqWA-pBaRnSpm4wS6-pPk0jJImYxZw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("9AnghsXhJ5ur7x6nM1DoYdEnz8nYyKM2dftlzwdWQ8CzVEhiqDqsgTzm5IrE7nzxuyr8oxFfkfJbvpKLDtB4og"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("7P3MuCmhENBCi9esIPtkcsmDrmDPgyx2JeDcSlnrqnu9ot2QijnC7SqDnrleaj3X7Z72hyXCw-zdafbbnlwKng"), "title": String("Introduction to ekoDB")}

Execution Time: 6459ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: zHvKho9u_dnQj_krqqKbyeNK-IXNhwlbUeDmK7k78ffpmvsMArKITt02Fg10TTa_nGfyG2TUnkDanJRe62D-dg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more details or additional products, let me know!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: cQJxP86LSAdtP34i1wOt-MFh0JOvbneSKhZKwa335Padx_WMjW2BPok1-OdJhBYDBps2uE6lNxJcKarhTtmgtQ
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 3KoXYwwCCITYp_flL5A35UMDsfWBl2GpU2AzgdzxK4Ywi5llJ7V_6jJ2Usemo-334IudDV6eeot_tyLO6j8Grg
=== Sending Messages ===
✓ Message 1 sent
  Response: Currently, the available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more details or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 2YAWL_prVydRBq9kSJ3sqwhuFBZEmIHJh0qQWT0BOoFY1u7ilcq1ZwOjMwFPMk4OxNbFRkAj25Mv7CiSYAWuTQ
  Parent: 3KoXYwwCCITYp_flL5A35UMDsfWBl2GpU2AzgdzxK4Ywi5llJ7V_6jJ2Usemo-334IudDV6eeot_tyLO6j8Grg

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: 2YAWL_prVydRBq9kSJ3sqwhuFBZEmIHJh0qQWT0BOoFY1u7ilcq1ZwOjMwFPMk4OxNbFRkAj25Mv7CiSYAWuTQ (Untitled)
  Session 2: 3KoXYwwCCITYp_flL5A35UMDsfWBl2GpU2AzgdzxK4Ywi5llJ7V_6jJ2Usemo-334IudDV6eeot_tyLO6j8Grg (Untitled)
  Session 3: cQJxP86LSAdtP34i1wOt-MFh0JOvbneSKhZKwa335Padx_WMjW2BPok1-OdJhBYDBps2uE6lNxJcKarhTtmgtQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 2YAWL_prVydRBq9kSJ3sqwhuFBZEmIHJh0qQWT0BOoFY1u7ilcq1ZwOjMwFPMk4OxNbFRkAj25Mv7CiSYAWuTQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: dQZMq75zOuhAsfwUPE9vAPIeAqjnFPHMDzOZSQd8l8WXddn9eAySG6yHvgfqHyQ6MtbM_GOwy4Z8pTJu37eNMA

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users_rs: Get Active Users
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - get_users_by_status: Get Users By Status
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users: Get Active Users (Updated)
  - get_verified_user: Get verified and validated user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("Qm5-O2D-dMzzBOXClpXvMK-k1ALRPSNzzOIagFYCOM1zFnM8ZhDrFNVzFz2as6LXYhTW6LMjvXwHStHscK4D6g")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("BKx7sPs79ZC30NVpUGBI5eBQSF8dcw6eAbJng3h0TxzvveLbELhPfhM4F0n64b6Na1I-EW5HMsgD-zk_p7zgxg")})
✓ Second upsert (update): Record({"id": String("Nn4vB62dK_26wuXgBbVCYkhMUVqOk6PCyQK_iFL1sI4zVblb5BOHfSNWG3e4cDZUyKjnQdChYuHW_l0V8CchRg")})

=== Find One Operation ===
✓ Found user by email: Record({"age": Object({"value": Integer(28), "type": String("Integer")}), "id": String("Qm5-O2D-dMzzBOXClpXvMK-k1ALRPSNzzOIagFYCOM1zFnM8ZhDrFNVzFz2as6LXYhTW6LMjvXwHStHscK4D6g"), "name": Object({"value": String("Alice Johnson"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "email": Object({"value": String("alice@example.com"), "type": String("String")})})
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
   Inserted with ripple: Record({"id": String("5LYdpU-1F-pmm2j7yecULouEWT28-Y4q_z9MZVpVP6wk5vo99n3Veo0rRnkP7wGk_8_X0FEfOYjWWiTv3-XjBw")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("3XK5z8nH7NdeMdlCf2dR5nvskmGBwyo_jtgU1fCRHHDUAYhAScwTNWxw2EnDr0PXL-5XhtGMPZ5vVO3JW2lgOg")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("5LYdpU-1F-pmm2j7yecULouEWT28-Y4q_z9MZVpVP6wk5vo99n3Veo0rRnkP7wGk_8_X0FEfOYjWWiTv3-XjBw")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("VQnX4NAgRzbAsd7jTwdVk9JLs9aAF-xHoBtYN87wheXsAaY7CFiySKutAHZdpbbeo__DG4rtOAPpFL25nnkNmw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name", "email", "id"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role", "name", "age", "id", "status", "created_at", "avatar_url", "bio", "email"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["bio", "password", "avatar_url", "email", "api_key", "created_at", "secret_token", "id", "user_role", "age", "status", "name"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "email", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
