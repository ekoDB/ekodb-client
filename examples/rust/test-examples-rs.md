make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("mUmWVzzOCzZGDlddhPjs8fososeuYiZJSMBc8kv8WRCYI5KJHfNJHdTZQMnYoDWWnn0y-KnCk77g29cxlto77Q")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("mUmWVzzOCzZGDlddhPjs8fososeuYiZJSMBc8kv8WRCYI5KJHfNJHdTZQMnYoDWWnn0y-KnCk77g29cxlto77Q"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("mUmWVzzOCzZGDlddhPjs8fososeuYiZJSMBc8kv8WRCYI5KJHfNJHdTZQMnYoDWWnn0y-KnCk77g29cxlto77Q"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("mUmWVzzOCzZGDlddhPjs8fososeuYiZJSMBc8kv8WRCYI5KJHfNJHdTZQMnYoDWWnn0y-KnCk77g29cxlto77Q"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "oJm5UDTCfxwFeKzYk0FPOeqI3I76oqgMzD9mKIGAVecxCXdowuT5fEPyBstw_M5Ym1RAJ-tWx-mzm-V2sens6A"

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
        "id": "oJm5UDTCfxwFeKzYk0FPOeqI3I76oqgMzD9mKIGAVecxCXdowuT5fEPyBstw_M5Ym1RAJ-tWx-mzm-V2sens6A",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "vdPoq1WL2iBCcDPyxXbfQGT4EGA-tRvnTz_ttIT_fDfA_MIn8P_AS5lZpU8Ld43T0qh0Jt4-g88l8FisKxzL0g"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "websocket_test", "test_collection", "demo_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "4y2Sa73sqseYosMCRddglS8M8FVsjHjuV6cN4T0aK5MrrZyfA8F11hxBJoYuve6HaV3DiVnd1rDP7nOIfGl59g"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "jXqGrEcsOXLV3Cp97YBLx3PfOCno7PIydRa-CVv_C8axmblbFRyWt_DrIenJ_w4-XouS9RgZAVLwphfoO731eg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "FJyAj5-c7D-wk7muy4yrcv_MTPs_nP9Qk8Sd4cuIpdOgWvrcgE8u5jSVnDdpCWo7vHNFVSUhX0pQ7aBjzOpWMw"

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
✓ Inserted document with TTL: String("uYhx74yFE5AkBhJIAjj3whGGRif38H0Z-K9mjqIpK9K7PyeavzSskYIY6oGPlJKTwaQzLH8oxDcdR4Vp1uBIKA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "uYhx74yFE5AkBhJIAjj3whGGRif38H0Z-K9mjqIpK9K7PyeavzSskYIY6oGPlJKTwaQzLH8oxDcdR4Vp1uBIKA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-28T01:10:13.796169Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: wvgwOQZPpfOXh-rpV7oQgXt4bFBf0-zixJN7H6ApB25t6cISWXIOZZLVBjBFFThnnenKKMXrJ8Wcw1aFrbPZtQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "x2Y45C_8dLhTDacn0NARO_-h44A5mA_M5gDSgnYBsE0JiYi_yKNA6aC49YAXpJdEFsyC9zYN-qUTMgmQe1EMbw"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: 8An1dRuzimANRQY4b6Dc796YbjwqfkucNc9vX2QrRdV8rUpXOovEaP75mv5b0X4JvQ3l-ydo3KXQLv6K6c4XsA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 8OkihqY0MrD0aROb15Ya1b45ekltunjhZr88XDxJAc37shIVAt2Ynt7cRj9iFQB78-cUED_iN0Un4awkzGJixA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: g502OEL1cXLkWxE7wgXIf0m2Kd1X_sMQbkwTJj3XvIbzJuKQz-27HBucTflxzCK65c-ITDRYR1iAvirXEyCRzA
Created Bob: $500 - ID: nm8RrtnYgb6KGcS0AzY2mGu9OsL2VdPd-L_hTAi_I-EKMgBAV0bNL6Oy91HFqB6YWl6jtw3U6lT2bN7jnpL4VQ

=== Example 1: Begin Transaction ===
Transaction ID: 1f98b67a-2e1a-4746-b1a0-e9a875b0b704

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
New transaction: ff8a643f-9b75-47ae-8e0b-ad43cab90a3f
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
Inserted: Record({"id": String("9LIjlQGf4dvR5bUoa9OigZpcjnp1j4XStHh0K2ZKqWJoBSs4GW7tlTFuBgHJJ0pup6m5Cy9IBHwNcemgJpv6xA")})

=== Find by ID ===
Found: Record({"active": Boolean(true), "metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "name": String("Test Record"), "price": Float(99.99), "value": Integer(42), "tags": Array([String("tag1"), String("tag2"), String("tag3")]), "embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "categories": Array([String("electronics"), String("computers")]), "created_at": String("2026-01-28T00:10:19.395341+00:00"), "data": String("aGVsbG8gd29ybGQ="), "id": String("9LIjlQGf4dvR5bUoa9OigZpcjnp1j4XStHh0K2ZKqWJoBSs4GW7tlTFuBgHJJ0pup6m5Cy9IBHwNcemgJpv6xA")})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-28T00:10:19.395341+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "id": String("9LIjlQGf4dvR5bUoa9OigZpcjnp1j4XStHh0K2ZKqWJoBSs4GW7tlTFuBgHJJ0pup6m5Cy9IBHwNcemgJpv6xA"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"value": String("2026-01-28T00:10:19.395341Z"), "type": String("DateTime")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")})})]

=== Update Document ===
Updated: Record({"metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "id": String("9LIjlQGf4dvR5bUoa9OigZpcjnp1j4XStHh0K2ZKqWJoBSs4GW7tlTFuBgHJJ0pup6m5Cy9IBHwNcemgJpv6xA"), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-28T00:10:19.395341Z")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "name": Object({"type": String("String"), "value": String("Updated Record")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 5bHbm_iufQMa5OrmGSsQilzZ7GpqWjWA0HyateHsWnhfphSAy6aY08E2QGVpP3LrH2q8H4jtq58-oY2I81bWdQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  value: 42
  id: "5bHbm_iufQMa5OrmGSsQilzZ7GpqWjWA0HyateHsWnhfphSAy6aY08E2QGVpP3LrH2q8H4jtq58-oY2I81bWdQ"
  name: "WebSocket Test Record"
  active: true

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
Collection created with first record: "22zzraBKzHbey4Rbtudk-MWQlgHmg9H-3oAy31FWcZ3QXmMLBf6aUfbQwV2gt593lIDidm52rM3Luj4IFohXcw"

=== List Collections ===
Total collections: 9
Sample collections: ["test_accounts", "ttl_cache", "batch_users", "client_collection_management_rust", "users"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: eQUVBG-Qq_G4pQUza7kdYnaHU9d1KAEnJUL9y_6EvRDVrAStOrJXFcorzLYREorkaOYkg1dggI58gGGJath7Iw
Created Bob: $500 - ID: qa8AU6yo9_TI3p3NoxiWeAXedDhKHN5O18bntmlzq17siFABboVarWYDuQ3tVSaZHObo12U9IuwZqOwdaW_y_Q

=== Example 1: Begin Transaction ===
Transaction ID: 3cce55b7-caba-4eff-b232-2a4c85d03df1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 42b47b71-183e-4d5a-a2dd-9c813e9efde9
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
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
     Matched: ["description.value", "title", "description", "title.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(String("Database Design"))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("EAltqdgve7ZG-RPop0_DD_N0Raq8pmf2tu3nJRM1V7BQK4cHeZNyWmSPCywF2iEz1YNlwcvygfz-3A3TgsIzCg"))
✓ Inserted user 2: Some(String("tAq2irkjaY9GDVspDzYC05r10FrUTua6fnjrjBln13tHsAAZfXSj4HB6CwV50koH8Ms_UOxwFO7uTCIz_Vg87Q"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - title: String
    (required)
  - status: String
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["test_accounts", "ttl_cache", "batch_users", "users", "websocket_test"]

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
✓ Inserted document: "-RVkTBnWInjCVAWUemLDD4CgD9tvm-Dwp1G3eKG-Cyb0ov8HPYuWYNIU3KD7Wy888lQ9g1PCT99BeZyIZwbkuA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("bDsys9YMf4ZYYb0_P5ST4H_p-1niFQyTs2CvCZA50p3LXtzvTjz_PN9-gAwn4i5llTFfxNnwxBem_wZ5AFzohQ"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("nZbGSgU6tChje6u3DTN2aWTZZl2BU2fiPvqgskVTCvXAyVwUT_165yBrIoRAYtbhiPvjHj9WvICkKoGQXgnNpA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: 2zsDH1RxCIPk4N-5qaCyI4DB3kBwg5tOOY2KZkQUiXC2j_uKySs8OPKW0_RNbc6l2zAWtB1KqPw5NKVlqQ6V6g

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
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: V4892quXmBHhUN5zAJPqJP2QDYsyqTshURo34iqDr93ANXlB0Xf5_qhTIqFqyJrG3xSVEVM-HpiMd-mJL_EcQQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: skVWGjZTCPJfHAukIsrxQxt_Pc8p7QIcpYWxMyjQNQdWCVMPSGHXxcPTC_F2cNokGGpBbYRpR-tkYjpqTOhyLg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: TqlSxRDMzC7gYpvgLdx6SPt7gcUCDA-jrXlwpyvBgInuVwb-31Id0CgZaVDQv9gQzfTyJiJgF0JrQ7zmfgzwig
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
   ⏱️  Duration: 128.810417ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "id": 1,
    "name": "Leanne Graham",
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.196417ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "username": "Bret",
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "address": {
      "street": "Kulas Light",
      "geo": {
        "lng": "81.1496",
        "la...
   🚀 Cache speedup: 64.0x faster!

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

✅ Script saved: 5CVTuFURoinILFj1MVJ64V7c8-se2cqsYp7WEzCHT--vzFcTKCV3ah2ELhC7Ko2j5KFObev3Cam-b4tBDtsP5Q
📊 Found 2 product groups
   Record({"count": Integer(3), "avg_price": Float(575.6666666666666), "category": String("Electronics")})
   Record({"avg_price": Float(474.0), "category": String("Furniture"), "count": Integer(2)})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("LayH2pbLtK-TXd30F_Ro883p54zprfClUigCuFWmcZ7bX9vuN8EcHVIPFCwhNmCc8_pUG0pkHwkPl-MQ9oaWQQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 5W7M-cRr1qaBXkYZkJ30OoKF-oLTwSa4ldGIdcnnretgNcmlrPSG_qmKJ3hNV9Ro1BXkHCnpfHAvThAghfBUcw
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: SL7dgpdpJijN_1ftkr-gbDvrVMJdTkvnrTQZyvbCS3Er6DsGOrpYlntRODxGy3D4rQbgl9FNGdu6_Be_hjisqQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: h-3kBM4o6YUSyO-3_9m1VcfyXwkByp27Fhl_YtuptgwEbyGsovbaavrYwF33bkSC-IFanDfFuz1d0lEGbFLs8w
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
✓ Created SWR script: fetch_api_user_rs (26WAxDZ0dPavpCMGHxHBS2iFwNY7fbVqSnmF07r5eCkdvK2lrOmheSkT0FS-kFDY8myaxeKNfkPGkct9pcpk4w)

Step 2: First call - Cache miss, fetches from API
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (Ecd6fOeSjkIrI3rnJR-1_Fuew6-eWQX5wvvWyTmdq1S54-0nijzLblVfRGyyl2H2hoe5wMWYqAfhs7ks02KZDg)

First call (cache miss - will fetch from GitHub API):
  Response time: 123ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 6ms
  Speedup: 20.5x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (_t1YRWYrht9VegeK7JK97eCEY2mkFcypgkfgfLvoM0bJd_ImThyx1YQ7h1_58pMT-tCd_dctuDzDRVsOudmKpQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (BaKxjV4NvuXjNnox2s__zW9iiA1NhvmrQJ1utfMZI4XY2TDrJXMevklR03SmckdZslpMiCjrXxKm2ciezmfpNQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (xHFfrf3UnwEybG1OkenR9ZTk-ZwTVMuL0zfqWFUjCLQ9jCzQrf0nrkUxlSV1LofJvP2LcwKqZclGSq8D28ZyDw)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
   Record({"avg_price": Float(365.6666666666667), "category": String("Furniture"), "count": Integer(3)})
   Record({"avg_price": Float(367.0), "category": String("Electronics"), "count": Integer(5)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer benefits such as precise geographical representation, efficient storage for complex structures like road networks or boundaries, support for topological analysis, and high-quality map output. They are ideal for applications requiring detailed analysis and manipulation of geographic data.
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
   Record({"count": Integer(7), "status": String("active")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"count": Integer(3), "category": String("Database")})
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
✓ Created session: 1C-L1tqnYk1eKUb3g3_TcFE6VIqGbd8bFPOUYLln1Uu4Fcu5rTWtTyI4UqGIYCE5ksDdntSQVJfrdS4sBBSpNg

=== Sending Chat Message ===
Message ID: 774-kgTz0lrHb2HnX3fklgYkquNLl4Jp-8XjE3EWmo8Fqz5uyc7UwyG7vK4P6dl-mErWQKjuULIeEowhiZbN4Q

=== AI Response ===
Response 1: ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. It offers unique features such as AI Chat Integration which allows you to query your database using natural language and receive AI-powered responses with relevant context. It also supports full-text search, vector search, and hybrid search with automatic context retrieval, providing a wide range of search possibilities for users.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("JfPT7Y8zVxTBzA1Re1NvMO7eXMrxjCQ_n2ZyA--jw1cpABSnNy7x99tA1Ioez9QqC6PJ9HlNlrNHloSXAZZoDQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("ZoehGIqVJn5hQHam4YEccVA4CFn7HtVfMhAj_KImqCnaNA-paqiKq0xKr-9SreKcU7Zf4xDTkWv0lNqE1LUx6Q"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("Y64hu_-_HRF66kw1mpkDft5frzKl6035KnO12tTZq6QjnpVS9xlyeJZqLheYHHWYMb01XlJbAc1VrTaTRIBFZg"), "title": String("Introduction to ekoDB")}

Execution Time: 2742ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Ene0ASmLMkj9LAkYWEb2W3vizgHCKHauinN3gEdICekInqpazyW_lHcK0CKBsPMogVGd9-uKDUClzWDtLj47ag

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, there is a high-performance database product called ekoDB. The price for this product is $99.

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
✓ Created second session: p-ZCLCGlwHgVx28M8OWhlqK2IUXGNo3BEjAUu1504UE3P7KKhpdpHzzfIE8pn0ivZxDV81GWHmCLZzzHlEGMmg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: vd5EO4q7AGtzgy6rMLWE_cVehblRhF5qYGcg6b_LzAk5HEuCM4Py_U3E7SJQDW_euOhnS4KcItAtKBXvLKkibg
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product that comes with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 9i9_hSIrj5m1kr--V9I5ZSUWUU9g9qHCSEfdsMCmw1DfAUeSrO-VkFY9jc4OzuL2iLvBHnJyPiwsTZp-YbtQcQ
  Parent: vd5EO4q7AGtzgy6rMLWE_cVehblRhF5qYGcg6b_LzAk5HEuCM4Py_U3E7SJQDW_euOhnS4KcItAtKBXvLKkibg

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: 9i9_hSIrj5m1kr--V9I5ZSUWUU9g9qHCSEfdsMCmw1DfAUeSrO-VkFY9jc4OzuL2iLvBHnJyPiwsTZp-YbtQcQ (Untitled)
  Session 2: vd5EO4q7AGtzgy6rMLWE_cVehblRhF5qYGcg6b_LzAk5HEuCM4Py_U3E7SJQDW_euOhnS4KcItAtKBXvLKkibg (Untitled)
  Session 3: p-ZCLCGlwHgVx28M8OWhlqK2IUXGNo3BEjAUu1504UE3P7KKhpdpHzzfIE8pn0ivZxDV81GWHmCLZzzHlEGMmg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 9i9_hSIrj5m1kr--V9I5ZSUWUU9g9qHCSEfdsMCmw1DfAUeSrO-VkFY9jc4OzuL2iLvBHnJyPiwsTZp-YbtQcQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("FrkdtpaGMkJzlhR_w3xr-85JuC2U5PkhWod8y37se8S5MS9CwHUhRKzv5krRxNonDWDVmSPuURwPFCaxblYNaQ")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"name": Object({"type": String("String"), "value": String("Bob Smith")}), "id": String("FrkdtpaGMkJzlhR_w3xr-85JuC2U5PkhWod8y37se8S5MS9CwHUhRKzv5krRxNonDWDVmSPuURwPFCaxblYNaQ"), "email": Object({"type": String("String"), "value": String("bob@example.com")}), "age": Object({"value": Integer(35), "type": String("Integer")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})})
✓ Second upsert (update): Record({"name": Object({"type": String("String"), "value": String("Bob Smith")}), "id": String("FrkdtpaGMkJzlhR_w3xr-85JuC2U5PkhWod8y37se8S5MS9CwHUhRKzv5krRxNonDWDVmSPuURwPFCaxblYNaQ"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "age": Object({"type": String("Integer"), "value": Integer(36)}), "email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")})})

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
   Inserted with ripple: Record({"id": String("gWQ_qSjdsSwi3F9bJ3jfWAA1DnQMfWlTy5bUamLSyasZHY2VKdH9MoA9RC90n4lSJHXaUB5M4lM44A7UpGlqWA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("XFXr03mJUR1qNoDQQ_ONsFInpP_9B1xHJrC4Bnm3-6dtCqWl10HMUQnr0A-Os0OeaIMFWZ4kOzHtcL-OI2jU5w")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"value": Integer(150), "type": String("Integer")}), "id": String("gWQ_qSjdsSwi3F9bJ3jfWAA1DnQMfWlTy5bUamLSyasZHY2VKdH9MoA9RC90n4lSJHXaUB5M4lM44A7UpGlqWA")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("6Axw0l2853m_0tyzVxj5h9IqVdAh2LaDqR-8EAm_hy_IwWUM9uAN65xT5-pfVv1P8RqJ2R-t8jQA8uy65DJVIQ")})

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
