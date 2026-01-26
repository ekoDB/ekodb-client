make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("ehi_iDX2-8bOHDncpT7YvkEJC-BTxS-Z3bs1v9s6YaU5ZTbndafGEMUlxblyKlFn4dzQGqP2ahoHzUm8H6lq8w")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("ehi_iDX2-8bOHDncpT7YvkEJC-BTxS-Z3bs1v9s6YaU5ZTbndafGEMUlxblyKlFn4dzQGqP2ahoHzUm8H6lq8w"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("ehi_iDX2-8bOHDncpT7YvkEJC-BTxS-Z3bs1v9s6YaU5ZTbndafGEMUlxblyKlFn4dzQGqP2ahoHzUm8H6lq8w"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("ehi_iDX2-8bOHDncpT7YvkEJC-BTxS-Z3bs1v9s6YaU5ZTbndafGEMUlxblyKlFn4dzQGqP2ahoHzUm8H6lq8w"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "OxC0Iqm6dKscDmJ4lvOfrE5ULerFFSPFfDhxbdGMxPG7DjUSbfjfBCkLZHOrBWGwBrzGmKldyS-4fge4dR1o9Q"

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
        "id": "OxC0Iqm6dKscDmJ4lvOfrE5ULerFFSPFfDhxbdGMxPG7DjUSbfjfBCkLZHOrBWGwBrzGmKldyS-4fge4dR1o9Q",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
Collection created with first record: "oIUZCgOY2N9YJT-cB6Be2z5aINkc7xFFz5HfcZuHUUjSQ0Y1zKnCALMnjbecjIUaAnxzTClWWcbHaMUJk-bmcQ"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "demo_collection", "test_collection", "websocket_test"]

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
✓ Inserted document: "yW_A1O2aXDY_sA-Mq6l-LIcmNpMc-C8Ky5iXNHn5W7HRiCsz044eBOOwDIxA3yJrQStZem30AR6IlfJ7M5hI3A"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "3FpnvfljTCBmXN6GD6eHXJnNXqgAosrp3GcU_zG9nO4vr7--jv-9M1ddLreW32UK5E3Pshijky1JiQiD7YIfwQ"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "6NYLefBKovKyzCdehhnmYdy1L3qHUIbmN_xI-xGxJLdnPKoS7dgXsPlVjhXeCq1bLjO0MGzQ5BaTdo3fs7jYFw"

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
✓ Inserted document with TTL: String("qqC2cE03vcgqIWc5p68fFgpiJ6Vg82UEpoDuVOTJ3TmEcSmd2KDblO-XCd07oOIzsZP2d0FprYl5g-REWK9I7g")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "qqC2cE03vcgqIWc5p68fFgpiJ6Vg82UEpoDuVOTJ3TmEcSmd2KDblO-XCd07oOIzsZP2d0FprYl5g-REWK9I7g"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-26T23:53:12.181881Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: Pch2YogZgigQkQcGsjBSfbiXMKkiHiyHBRk2yW51JV4U51dDqOiF1KhIK64h0Y6OlSskLhGClYYayNhdDd-mXg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "-H1eqkFUyrjN1xr9vB2W9BE2ojJMkTFeUgZ9Yy6LhARXKD0mwXoYQY1VFaMqGfPwAQgaMKopxyo8FEbKchnLJA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: dYaM_bp0bbmnV3sx-DRcTyympVyDoSeY-2Y6iwpp6Vmt9wje-ALH02czMgbOI58ISBIGxwqoGmaGAfohgvxTAA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: IW2ulvcEpKTvbUjPCBR-iJzHUIVVj2gsIvuKjFIDrbgestYcNK7cfLf0e8syBxFkXDZQ77o3CWP4yaOwsE3h7A
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
Created Alice: $1000 - ID: g61aLrFirsFX4d3YONvY1YdOpHgwwGAKhZwi6YGDc1M2BM-9Ign05w91jEOXZsKpNrC9ohedyorFEFap4Z3n_Q
Created Bob: $500 - ID: IO3BvYeXJjWCvQBzbGXndi24GzHh0iZtL_hXilnenaDQFjCzCqNtI7v8xYdF-ODzPgq8-OUYmLklULKGqv3c9A

=== Example 1: Begin Transaction ===
Transaction ID: 486f4a2b-90a2-4c25-83df-636be38dcb6d

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
New transaction: 3591015c-68aa-4e5f-b3e1-09049ae55be2
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("y1fla_JUxigXLaKBXQBF7K2gxgkvpDE08O5gTwNSQlGD66nVDPPPBd_xyKRYchC_FogERty-7cANK7o0NQ0BJg")})

=== Find by ID ===
Found: Record({"metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "price": Float(99.99), "active": Boolean(true), "value": Integer(42), "categories": Array([String("electronics"), String("computers")]), "name": String("Test Record"), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "data": String("aGVsbG8gd29ybGQ="), "embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "created_at": String("2026-01-26T22:53:17.485034+00:00"), "id": String("y1fla_JUxigXLaKBXQBF7K2gxgkvpDE08O5gTwNSQlGD66nVDPPPBd_xyKRYchC_FogERty-7cANK7o0NQ0BJg"), "tags": Array([String("tag1"), String("tag2"), String("tag3")])})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-26T22:53:17.485034+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "id": String("y1fla_JUxigXLaKBXQBF7K2gxgkvpDE08O5gTwNSQlGD66nVDPPPBd_xyKRYchC_FogERty-7cANK7o0NQ0BJg"), "value": Object({"type": String("Integer"), "value": Integer(42)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "created_at": Object({"value": String("2026-01-26T22:53:17.485034Z"), "type": String("DateTime")})})]

=== Update Document ===
Updated: Record({"data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "id": String("y1fla_JUxigXLaKBXQBF7K2gxgkvpDE08O5gTwNSQlGD66nVDPPPBd_xyKRYchC_FogERty-7cANK7o0NQ0BJg"), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "created_at": Object({"value": String("2026-01-26T22:53:17.485034Z"), "type": String("DateTime")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: HHYEVJBF-ByVxThBIXL5E8p8xDNss2qATdbgJ_iCjUt2N6jY_hDNkwvsnyQZPYB3M-thDWHn6XgSKfGJuaseDQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "HHYEVJBF-ByVxThBIXL5E8p8xDNss2qATdbgJ_iCjUt2N6jY_hDNkwvsnyQZPYB3M-thDWHn6XgSKfGJuaseDQ"
  name: "WebSocket Test Record"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "A4xH7oZ5KlXHrF7ZYNMtKOc6yPS9h9ruhbNoB1pDzU2hehqiRaC0hiyE9-Iz4U6nNstycs3czIPwcx184RGfgw"

=== List Collections ===
Total collections: 9
Sample collections: ["batch_users", "ttl_cache", "test_accounts", "users", "client_collection_management_rust"]

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
Created Alice: $1000 - ID: jYjiTl7H8ot1T8J1VASu3BHt3a_jx3CcLHjerubM8EDxflqyyxw2sW2V_gtrtz5doXU6vrDQr8-BggN1YNTBzw
Created Bob: $500 - ID: CO9-Y3d0Rjn6zd4a3drwJzTLhHl2u0nCFTiN6iS4S2y2h7z3YwXlsCW52eLvh_brvdWqOiv0VqRcmyG2MIOspQ

=== Example 1: Begin Transaction ===
Transaction ID: 18c800c4-7cdf-4115-83d3-8e165d0aa396

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a7eb11d3-81f7-4efd-a9f5-65466a5af959
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
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
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
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
     Matched: ["title", "description.value", "title.value", "description"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description", "description.value"]

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
Execution time: 3ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("RyYw6wFk5cZ6FbM7f_INKB0n-s7YHL-peeP1z2OKEhvLl8IdvnV4We0TtSvvdAo2u3LHstrk2oQ0pZZ9LBy2WA"))
✓ Inserted user 2: Some(String("IYIOyryhGvxQZLnww3zcBEe2EiGyGF-9OQ4rroy9KxZxbhrOthNd4WXhjlujvIHywUfQJhkBTr3_EIHPDa-8Hw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - email: String
    (required)
  - status: String
  - age: Integer
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["batch_users", "ttl_cache", "test_accounts", "users", "ws_ttl_test"]

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "3kg56IHBumflRg87z7Uxev7weHWQndkWSW0cHKuiKRua061CzdbaNCe2hnYR3KT7hMfMO3cKmV4O0ZpuQESr-A"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("V6LAwsxpVOlR9APRutS-Cvkfdj0PIBxqeN-an2LbF8apfiV6xHZfBBs_ehUMf1CZ8OC0UTX1vmBW481diM2NOQ"))

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
✓ Inserted document with TTL: Some(String("66fOSgq2F2xrsvc7DPAbiqjzFF67iu3aWP-PQqChBTQDedwGIoqS4Y4Mm-rpDg3LliWMUOVuu9Y7LM1mQ75W2g"))

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
✓ Edge cache script created: IU3gpjh1ExS70717IZ4O05rbqwye8hEDeBAkFm7DaflKuVrurchayW3HpoehhvawguXBhQGsGO5J4Q7NJjBRgg

Call 1: Cache lookup
Response time: 3ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: lZi5d7k4UOePLjOfKoyZIelrkzsLX09C6p9_LelnrZv5qKExcsuIOn72WTbsfDtd7p-Eeta2OtkgoHyoYQsrLQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: cu_siauGAt_dpBbxTbeat0nG72APSbWI4e2u5-7C5vsuGX5hhSHvpHZBp5rJqpAD76z780Aqli1amUbOVtuLKQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 94Nmg6FCEd3mZz5JDywoNO858jouEDzlVJtmc0q6jZoC2Wg32K4IKodG3HTXmwfrRxaNQztnhKeZ1EAnJ0G0_w
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
   ⏱️  Duration: 135.574041ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "email": "Sincere@april.biz",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 3.229583ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "username": "Bret",
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "email": "Sincere@april.biz",
    "id": 1,
    "company": {
      "name": "Romaguera-Crona",
   ...
   🚀 Cache speedup: 45.0x faster!

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

✅ Script saved: FhM3S77GDF8BM6F4cL9cU2su02w4w_JSr-9HlIOB7Ua_dm4WVzKY3Mk1JmmICBtcCbl7fm1aj0CB3M8C13RxLQ
📊 Found 2 product groups
   Record({"count": Integer(3), "category": String("Electronics"), "avg_price": Float(575.6666666666666)})
   Record({"avg_price": Float(474.0), "count": Integer(2), "category": String("Furniture")})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"category": String("Furniture"), "count": Integer(2)})
   Record({"count": Integer(3), "category": String("Electronics")})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("VIS1FUhoDi-eZQFRMBkynH9EGpmyveZyRUXW5EDsgkZsfd0q7_OnpKMTHM9CqsTSLygFpIkpNlaBB3QCdbHvJg"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: cJYfDa239jmOyam0iCvgdzGlcsj1hRq27CeXfi5taXTlPnGuXbtY_vwrwRp5vq2qfHYM1OSOBOunsnancmuulQ
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: IzNCLOl-Z1N5jz0hBgtFfVoueagv2wcjFPuDqx8Ltk40J6U5jx-6_T0b3BxIneOibxFwO2XyPPFkxaz0WDBipQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: s2OK5y28Rl_G9B2nO0EJTUxJpKryS_D5KQvreJQ1l4mNRwsJe5zXpo2EtUYv4K_UT9cXTVF_bTTUW3ZU-UYQow
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (w4xApEYoRSVmv7huwiSArxKKICDFSPAWpDJ5jq-kg2YyggbT6Bxxxk22uhMzyZb5fbRAblw99cLa3AAAOfBPWw)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 81ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (RBXPn2g-7BfFhJOfnels7P6yY4rv5WIK9ZD0Cvm7JCyVUFl7byghJqA4Gm-eccvrGAyFNhSg1zNuXybY5VfH5A)

First call (cache miss - will fetch from GitHub API):
  Response time: 199ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 15ms
  Speedup: 13.3x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (xtr56ycLF_PUpjpO6raxmDQitgxcDVhUsNYbnufkebNXrljVU_s-IUijbcWf8tNAsHOeTbjl0ibWl-qcIrURog)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (BTdfVWhI1g61ayaOuQqQ5ziG0-qkLrjwSg1pDphCnAcTteQICLDhnVGjGPLD4wO42cXilxyD2vul2fRk93Y7FA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (09Oz5duDrEVjRzetHfqbZzkuwcm__0X-NPxgGZabfTscbG8IrRPogbMJQkJlOwswevRLhdQIqcv0oGoSZ1_a1g)
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
   Record({"avg_price": Float(365.6666666666667), "count": Integer(3), "category": String("Furniture")})
   Record({"category": String("Electronics"), "avg_price": Float(367.0), "count": Integer(5)})
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
   Vector databases offer benefits such as accurate representation of geographic data, efficient storage for complex data, support for a wide range of spatial queries, and the ability to handle detailed topographic features. They also provide high quality resolution and are easier to edit and manipulate.
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
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: JJp8VIsGR4wyMvu17JEv-OLpzuXsTUFvjS_alTXh2kLaX0urem-vKy4enOAO1M-uRfBK8cMBjeUSkp6euPePLw

=== Sending Chat Message ===
Message ID: nr4vRB2lKEPs-0E88xg1qh8Bu6ltLR1BQ2LCJvB6L7fXB6MY7tkUidCxsefJBgSWEvVcF5IBThI9JJrVCznZ_Q

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. It also includes several notable features. For instance, ekoDB supports a chat feature that permits querying your database using natural language and receiving AI-powered responses with relevant context. Moreover, there are robust search capabilities, including full-text search, vector search, and hybrid search with automatic context retrieval. These features together make ekoDB a comprehensive and efficient database solution.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("06k6i41DvBhU4B63XupKikwcuumxM2nE9rKn_AYH77PsgCBm1ol3nJEDKIP-mGU8IGnKzZjzBq-XrthOb1gbNg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("oDNhPsu-SlHWhgkxEPe5WaTtofxCkUXvmv7YCc4pQmokk-vdyPfRmsD86eRDI-dY52wl3w8J56McYucSguxVag"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("Q5zW8HK9-BeIEQtnW9DBqCqsyQ3YCDXprvkIJ7cjDkuNTE8SbV9I1Hp4Z9VjUmDc9YjGkLSvI2vUyPJO-Ca8UQ"), "title": String("Introduction to ekoDB")}

Execution Time: 2606ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: c7ZPLnVS8mzCJtfMYlOzvbBJOOj8y8NEwngpgKqg98wUDpsi44tgh8AaSdVLvXMK5OmPkG1pCxbPBVaDD8NvWQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product based on the context provided is "ekoDB". It's a high-performance database product. The pricing for ekoDB is $99.

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
✓ Created second session: pcvxPqW253IbKRKd-rHxq-mW7fHJhDl4VlInFqDEDbDIVNBA5RUJBXB2SDuGmSYhgC8rUos6lwrMWjZ1mnEJVg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BA_ndoTnr5r03-371eTaiV8HcAFSw0mV27F1OLGme-UgwiGVwrPuPA1_Kp9gyHW2wlVLEZqBv0vfCO4udTlanA
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is ekoDB, a high-performance database product with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: -_sBwsqWia7oulBygjZyEb1DnazkULc2Kx5kMzghGwgHb2julARR__rSRbmBsOlmkqKVs4vWj-DCpSDfA-HOyA
  Parent: BA_ndoTnr5r03-371eTaiV8HcAFSw0mV27F1OLGme-UgwiGVwrPuPA1_Kp9gyHW2wlVLEZqBv0vfCO4udTlanA

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: -_sBwsqWia7oulBygjZyEb1DnazkULc2Kx5kMzghGwgHb2julARR__rSRbmBsOlmkqKVs4vWj-DCpSDfA-HOyA (Untitled)
  Session 2: BA_ndoTnr5r03-371eTaiV8HcAFSw0mV27F1OLGme-UgwiGVwrPuPA1_Kp9gyHW2wlVLEZqBv0vfCO4udTlanA (Untitled)
  Session 3: pcvxPqW253IbKRKd-rHxq-mW7fHJhDl4VlInFqDEDbDIVNBA5RUJBXB2SDuGmSYhgC8rUos6lwrMWjZ1mnEJVg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: -_sBwsqWia7oulBygjZyEb1DnazkULc2Kx5kMzghGwgHb2julARR__rSRbmBsOlmkqKVs4vWj-DCpSDfA-HOyA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("drF2vkyY7vpSkN0wTqO8KSa7em89quIBIcnNnDbr5vOiSmlwdIlGtvCkr2X_Xdsk9FiGH-df4Ntg5f25rEwWZw")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"email": Object({"value": String("bob@example.com"), "type": String("String")}), "id": String("drF2vkyY7vpSkN0wTqO8KSa7em89quIBIcnNnDbr5vOiSmlwdIlGtvCkr2X_Xdsk9FiGH-df4Ntg5f25rEwWZw"), "age": Object({"type": String("Integer"), "value": Integer(35)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"value": String("Bob Smith"), "type": String("String")})})
✓ Second upsert (update): Record({"age": Object({"type": String("Integer"), "value": Integer(36)}), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "id": String("drF2vkyY7vpSkN0wTqO8KSa7em89quIBIcnNnDbr5vOiSmlwdIlGtvCkr2X_Xdsk9FiGH-df4Ntg5f25rEwWZw"), "active": Object({"type": String("Boolean"), "value": Boolean(true)})})

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("4BxRYve-09ORm6aNPhO-fJ9Fy-8G6Y6aEkyjARqz6myliAhfl5aciNRsHh6wlPTrxZSPraqLhHZ6rNxzL8Xq2g")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("yvutR2YVGDzJa1FN6qjxygighFCzpH_Hl9LCyxKECzMantD4sgyApLqCCAniJA4CcOwR5nRVVcvXFrlR9bENXw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("4BxRYve-09ORm6aNPhO-fJ9Fy-8G6Y6aEkyjARqz6myliAhfl5aciNRsHh6wlPTrxZSPraqLhHZ6rNxzL8Xq2g"), "name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"type": String("Integer"), "value": Integer(150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("AOcIDLBi_EOW7bm3I_-XbP4OI3wqSquvLM_w-Ae1MlXwlUXZ9RpOIOx7yLEg8cfwiCeoqXDBjnQCoGf-TAAtcA")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: N/A

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
