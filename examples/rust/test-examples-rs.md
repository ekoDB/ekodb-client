make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("0X-3Ts9ShtWRVCGHxelepQvdiBOO2IaItBkUHOweRE5kyBz7VUAB8RGsxEaPq7_Fdo94CcnRlxbW4gJ98d2o0Q")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("0X-3Ts9ShtWRVCGHxelepQvdiBOO2IaItBkUHOweRE5kyBz7VUAB8RGsxEaPq7_Fdo94CcnRlxbW4gJ98d2o0Q"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("0X-3Ts9ShtWRVCGHxelepQvdiBOO2IaItBkUHOweRE5kyBz7VUAB8RGsxEaPq7_Fdo94CcnRlxbW4gJ98d2o0Q"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("0X-3Ts9ShtWRVCGHxelepQvdiBOO2IaItBkUHOweRE5kyBz7VUAB8RGsxEaPq7_Fdo94CcnRlxbW4gJ98d2o0Q"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "-Y1QkUwIUNwv7K_DDT9HDqnUZnG4p2GiYfp32sra_Et5SNn0w6eMDDWj5odCNCHBH6pvyV46cJHNCiTKCU7Qxw"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775454913804153000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "-Y1QkUwIUNwv7K_DDT9HDqnUZnG4p2GiYfp32sra_Et5SNn0w6eMDDWj5odCNCHBH6pvyV46cJHNCiTKCU7Qxw",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "J1V2yzOhyM36qBZYesDUbFHF5bcWs7f7bds_4GHY8hAuBPadqPYFwitJuXkHBHUZypiUztgKbAbCKK2V3-G5lg"

=== List Collections ===
Total collections: 5
Sample collections: ["test_collection", "websocket_test", "demo_collection", "audit__ek0_testing", "batch_users"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "ZdyZ2P22XrXrmsvUz2d8r1CXyDZvjpJB12hDLlArF-KoTKvC0lwdfSHxD_NcKJv9YPTyKt_jbFGx60ANNxw5-g"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "vyVTGO1-gOYjqJdCiHvnMFGyIdsfiXlUWwb_fQAWglGSFdGojZjyW-gE72FMav0pbDX2plnUjNNX9Djqj0uSgQ"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "ZN30IXyGRzJaBs3NWp5qSWke-xfMaR1yqyLfBc-n-bnR2CREx_wSVGOedLCsjpe86fidwE7vOiFW23kKeqrEqg"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("uYaogE4JkVg6kQmpjvteQfS-xcmP65wRHw6uFQIFNAwdJjHLLj8016ZntLn5hnAKWeEZU1mAWE2fzRueFxGwDA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "uYaogE4JkVg6kQmpjvteQfS-xcmP65wRHw6uFQIFNAwdJjHLLj8016ZntLn5hnAKWeEZU1mAWE2fzRueFxGwDA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-04-06T06:55:29.139447Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: EV6NZFOG9i_FUX7H-ERiGKHjsKvRr3IY31BDOoZbHz_NmLNLmNm-69TKGAe57nM3bc5S5_M_TzUwkzPxtarDaw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: "0zUyeblIM0YuzX_WIYHptlKQ8MwnEfdZiAqWJEPoFDR9rBeeMyKprUp1pPEx6KaL86tNdgp72wvvcq0RLdkiZg"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: RoGIECJdggka3HDbdDYnJNqd9NTOigdPWXPXN580WWf-l9st0eRYLxjRRrPi-U5ZKij9xKepydKwRDsnDwjoxg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: BZaZJRMMgGLU4H_uR7FCCH9q5Us5lflj54xadIeMu8pNOdoQ3mZ4s3KwDC9kQyYGLjO8dwWO3naIyU5BPQBsHQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: _p-j2Vj0A11U0HAAelV8Xe0J384sO1AqWZnLDReWd5B1ABNKteuyEoLkKKjeglGwHPeOxOazgXkgtvV2kodOEQ
Created Bob: $500 - ID: 0zrATDtrfiL6O1TkoRTJf6QFgGsB_TE8DSEBhb876aAtzyFcdyS2h8LEjdXqHUOimt4__Y5rfxfZnmqqlVHYnA

=== Example 1: Begin Transaction ===
Transaction ID: a6b178a5-7dac-4bf9-82b3-8684dc793143

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
New transaction: 31975d22-44dd-4b65-bccf-ec452ac9ea20
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("s5e-DMWFYJyeKdvJBpYGoGfMsthhaeRHg0tw2xyW4JfYXsGH37uKfbSho1-PXi7lYamfyFslG-f59Z4fckxGYA")})

=== Find by ID ===
Found: Record({"value": Object({"value": Integer(42), "type": String("Integer")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "id": String("s5e-DMWFYJyeKdvJBpYGoGfMsthhaeRHg0tw2xyW4JfYXsGH37uKfbSho1-PXi7lYamfyFslG-f59Z4fckxGYA"), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-06T05:55:43.822127Z")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-04-06T05:55:43.822127Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-06T05:55:43.822127Z")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "id": String("s5e-DMWFYJyeKdvJBpYGoGfMsthhaeRHg0tw2xyW4JfYXsGH37uKfbSho1-PXi7lYamfyFslG-f59Z4fckxGYA")})]

=== Update Document ===
Updated: Record({"embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("s5e-DMWFYJyeKdvJBpYGoGfMsthhaeRHg0tw2xyW4JfYXsGH37uKfbSho1-PXi7lYamfyFslG-f59Z4fckxGYA"), "created_at": Object({"value": String("2026-04-06T05:55:43.822127Z"), "type": String("DateTime")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: jyUhwfpGDv4XYf73IM1m2LSRRikTDD0ECmrFfY3Ex9nbPSprqvCAw5Y5iiqzTpdXhcJwsRvZr7HCV4r3im5aKw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  value: 42
  id: "jyUhwfpGDv4XYf73IM1m2LSRRikTDD0ECmrFfY3Ex9nbPSprqvCAw5Y5iiqzTpdXhcJwsRvZr7HCV4r3im5aKw"
  name: "WebSocket Test Record"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "EJQDHPyXLJJc6CscQxhB1U9SIW7uSCRhed3rR5ciX-X8OqZVRqmzZfDM6mHeMP0ILkEuSe8QyBtmvJZ5LLQ35g"

=== List Collections ===
Total collections: 10
Sample collections: ["test_collection", "websocket_test", "ttl_cache", "functions__ek0_testing", "users"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: K1Ik_CrfFFuUuzrAWwSg6-T2Kdksg7j_FxX8qEQEshtMI7tWFX00vhbrburoyeRYPoUBER7pJu9ed3XaWRDRAg
Created Bob: $500 - ID: Blt49ZFJTf6U-VcgnJYB0_wJ3di7LwxnNiXNuqOhF2VhjsCJdZm8NXrI759VX3Bp9vyp2REtnfeDmPhSuNv6eQ

=== Example 1: Begin Transaction ===
Transaction ID: df7b2000-29a6-4b56-9431-522d2eaf158b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 43615663-96f1-48de-8454-239fc33fe55c
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
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
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
     Matched: ["description", "title", "title.value", "description.value"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
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
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("lPvJWBA0OSFVdgEmafikHd3nZM_MotMRpIy0OaiFdgVO9uRYu7_PPJFem4TuSN0ZWOf3jf6qLr05qchwX0vRWw"))
✓ Inserted user 2: Some(String("j54ZI9tRhWYoz_bUga0_l2I5mwbn4sBwn4VbfGfSrALvOXyjqYyRlI1tGhhQ-LiRO3nyixqTeJOXzLYp-u8svg"))

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
  Sample: ["test_collection", "websocket_test", "ttl_cache", "schema_client_rust", "functions__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "jfG9PAT7a6Z_ads72b63V8KtshBozIi3V6oahJC1NuNXmembYR5F_XnZvR1TLaClLkfFJGA1ZONF_dzmSuVIOg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("bgH1VjlZUPYzv1BpDUBYqzK48MVlYZYQnA3BZ5Z6Qe9fC4CkNXAGBCCXbtKcpvtS0HmyaB09h5S16AJPNIUSuQ"))

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
✓ Inserted document with TTL: Some(String("8B2XrWiUk6zHYQFkso74vi9ym9k9VWe2jAipmQhc45ZASruxXmW2VtF36W2A_0UtoS82tZdOXGt-UIgmIFDVXw"))

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
✓ Edge cache script created: eco4nRNqYuSF_QR9tcA1Ukh71nWWjpAP0l_1fjmYNEoFLASfysTsMlL2claWAKU0nB0a4BlW6ck6YbwvyGj8lg

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

✅ Function saved: ZpVII4T3tXP4mmdfoGH9LBVI0XJMdY-E99AsefBHz_TRVTfJgUkxZ8siANl720063pxUzG7mSxWTwci8f1CbyQ
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: xJXRgsMAMB6fR_Y_Z8dHd2fqqr8SFpjkhmZrvvHncu-wxbc8G11xk9UE3x9YWnLMHnVo7kzH7TOhOY_OXZONZw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 8_c00mvBPUUni4M2ZtPjKD-8LFmMOPXiF8enK7irqkW1JS8X2gajAzTPvyvefZ9kSl8rKx6oczvUGKZ8lQmdJg
📊 Statistics: 2 groups

📝 Example 4: function Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

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
   ⏱️  Duration: 226.021541ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "id": 1,
    "username": "Bret",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "geo": {
        "lat": "-37.3159",...

Second call (cache hit - from cache):
   ⏱️  Duration: 40.5535ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-m...
   🚀 Cache speedup: 5.7x faster!

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

✅ Function saved: dUpJ5zjrFzKiRVe9IxjV8Hzl2kb-v4t1M97wj5EPaGnM1wYFyt-MDxS9rUOKxh79obD2Ra9UkbR42iqo1M2Nlg
📊 Found 2 product groups
   Record({"avg_price": Float(575.6666666666666), "category": String("Electronics"), "count": Integer(3)})
   Record({"category": String("Furniture"), "avg_price": Float(474.0), "count": Integer(2)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"category": String("Electronics"), "count": Integer(3)})
   Record({"count": Integer(2), "category": String("Furniture")})
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

✅ Inserted order: Some(String("8LmaoZq1dFMpHxN8EhqsN3q8JdHDcTIp5nIJ9pm9H7mRh8tOb2Z5TG82iYnNW3CybOAUipHA7sj_GmdfWUrHkw"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: 5ItsqxlLdtSnUyUaUMYZEWjRnRTHlcZ2gBdDSjbcBj7CPmLyamksJIRQj2eMaiSWl-p4KJaLuC7sXCwPB70BrQ
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: q3ZaPQB6Dv1zL6U88fQoqM47JgaPk-03rWkPWfKreHmCYBhpuMuQ_Xn-Xlk8YdwDDLpEeBHisyzURLdHiYrZrA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: USWBvt9bAwqdBxI9qxGe6LNqBw_4YJYj7JcWKT6BlSz6mAWrtrbOj6JUY2JMkkA33Y168jZWN4HkwIUMg00dWw
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
✓ Created SWR script: fetch_api_user_rs (bKHGNFMAYYBwZty8CQwY-kyDTB9sr0wkPwLjWhYvnqlXTY3F2m5qc7Pounr-VXyAX95NHnYb47IlEqa9mkC1MQ)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (HX4yWLv5j2buHBulmnnN8G4OblKoBaDfQ0jJ0tLbWETbWeQ3xo8p3mTeuRAzOUr8CXllh17jlMppy6yw-AJAYw)

First call (cache miss - will fetch from GitHub API):
  Response time: 116ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 14ms
  Speedup: 8.3x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (GYKAZjlM-0ZD07hflHpsiNZ0kLxSDYwccXptIcu5siC_TuZ1y0oEWY8OUoRmWMXXOmhfYB9jwESidg6e9L8yjA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (1hvtkEWSWVQ7e7qZc8l3ah8U8cF7N7e_AdQAMa7W4u35H9Cd6eDdygZQOGbu3J6zSEseBjSsKKBYLJJIyFEXMQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (5hvfzF496KaiMofv6fRe2JpWQOddcOYea3XvUGTZeB4PZuuNrGI5r_Vf63i1su-bHEjT6Ohm0dn0m-hC7Kv8Yg)
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
   Record({"avg_price": Float(367.0), "category": String("Electronics"), "count": Integer(5)})
   Record({"count": Integer(3), "category": String("Furniture"), "avg_price": Float(365.6666666666667)})
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They excel in nearest neighbor search for high-dimensional data, making them ideal for applications like image, audio, and text similarity.

2. **Scalability**: Designed to handle large volumes of data and scale easily with the addition of new vectors.

3. **Fast Query Performance**: Optimized for rapid querying and retrieval of similar vectors, enhancing performance in real-time applications.

4. **Robustness**: Support for fuzzy matching, accommodating noise and variations in data.

5. **Integration with Machine Learning**: They work well with machine learning models, particularly those leveraging embeddings for representation.

6. **Multi-modal Data Handling**: Capable of storing and querying various types of data (text, images, audio) in a unified manner.

7. **Simplicity in Querying**: Often provide intuitive query interfaces for complex vector manipulations.

These benefits make vector databases particularly valuable in areas such as recommendation systems, natural language processing, and computer vision.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
   Record({"status": String("active"), "count": Integer(7)})
   Record({"count": Integer(3), "status": String("inactive")})
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
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Integer(3), "category": String("Database")})
   Record({"count": Integer(2), "category": String("AI")})
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
✓ Created session: nx9B8dMhSoBFDc7TMj9VcUBs_3AYQiGlUruZhrVe7vbfO7ifnTm513v7gMsDU51k9hKAmiBWs2frRI8IlGklFA

=== Sending Chat Message ===
Message ID: HNwwIjp5Rgz_3ZtaPG3Vd4q40_0DkjyveJTMao6fM2Y4zEyunzovIhOGtntxqZfFRD3Hoxw9DQ45VDI6j0TXOA

=== AI Response ===
Response 1: ekoDB is a high-performance database characterized by intelligent caching, real-time capabilities, and AI integration. Here are some of its features:

1. **AI Chat Integration**: The chat feature allows you to query your database using natural language and receive AI-powered responses with relevant context.

2. **Search Capabilities**:
   - **Full-Text Search**: This enables quick searching through the document’s contents.
   - **Vector Search**: ekoDB supports searching based on semantic meaning, allowing for more contextually relevant results.
   - **Hybrid Search**: Combines full-text and vector search, automatically retrieving context for searches.

These features enhance the usability of ekoDB, making it suitable for applications that require real-time data access and intelligent querying capabilities.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("DjSveJQLjSV9g0FTd9RNykPW3fMmLMFyt7VLPEfYP-mLvnbX3_n-20Yizvgf3IpIssRtlyDeF5JcxuUbTOz4Ow"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("31hov8ZDxkjDfkOhivc4go_XUEe_R1PWfq1CN7MW9Dcl4XqXjL08I2xQeSQFaNaBD1dDaXq0LGe2JuG4dwajjg"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("baGYaXHLdoH8oCuOdPNzAFKbs44oI0edYVsLogiJXdEsDeowthCRUIAe5lSo6Q8yuN-Q11oFbnUoZMCfK9gIRg"), "title": String("Search Features")}

Execution Time: 5721ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5Ab0M1h0BAfblL-tOfStJSTcStCFBzQL0b_vi9xEgeYA-NLH6hfDpp7rPJw6SfaK8g73jTb_pWymnrh5bTnWqQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have other questions, feel free to ask!

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
✓ Created second session: QJ1tRdmF2ChzbOxluHr79WKe9w7hPGSMmFlnfNS8_JUdgNjjWGNUx9eOei4nxDZR7AJOMdwOKkju_CtWylJYow
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
✓ Created session: 3YM7IzSN6M0w3JrCh-00l84pGH06JmyfF15U__UKiWFuRd_e89HQL65ebo7Fr65ncJMNcpjpys4lQp84BIe85w
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name**: ekoDB
- **Description**: A high-performance database product with AI capabilities
- **Price**: $99

If you need more information or other products, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: tqqf9GgyBqQHNq0yVE93HN_N52_ZW5h4Y-5kVoC4Lpc0yXrsgeo_t1DkKOfOpMMYHeELSwrcMi0BckgOdEJGGA
  Parent: 3YM7IzSN6M0w3JrCh-00l84pGH06JmyfF15U__UKiWFuRd_e89HQL65ebo7Fr65ncJMNcpjpys4lQp84BIe85w

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: tqqf9GgyBqQHNq0yVE93HN_N52_ZW5h4Y-5kVoC4Lpc0yXrsgeo_t1DkKOfOpMMYHeELSwrcMi0BckgOdEJGGA (Untitled)
  Session 2: 3YM7IzSN6M0w3JrCh-00l84pGH06JmyfF15U__UKiWFuRd_e89HQL65ebo7Fr65ncJMNcpjpys4lQp84BIe85w (Untitled)
  Session 3: QJ1tRdmF2ChzbOxluHr79WKe9w7hPGSMmFlnfNS8_JUdgNjjWGNUx9eOei4nxDZR7AJOMdwOKkju_CtWylJYow (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: tqqf9GgyBqQHNq0yVE93HN_N52_ZW5h4Y-5kVoC4Lpc0yXrsgeo_t1DkKOfOpMMYHeELSwrcMi0BckgOdEJGGA

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: wEyobUAVH4sQ2rroyIA6-EzJ73cIqEe1qp79yPjq_hRS7_diKiWoedyDRDl6BUMOu-UeH87M4CEHzfe0tdU7Zg

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_active_users: Get Active Users (Updated)
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_rs: Get Active Users
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("bV1aR7QR0gFP0DlsLbzwdf-vBUU83wTOSU6xNqQB1AAfjR5CzMdReSfevaVzGdUY4_nO2z8ZOuM5xVzUEn9aYQ")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("4ZdZWQZPuJ-BWawRde3UwzYKOZ4JosahwgdqB2rE6wlgtYxu_-TkkJq1SrtQQ2iXQWmuAyqVXyNEFH5BklNReg")})
✓ Second upsert (update): Record({"id": String("RufP3Drti9cVROycGBYr9ynw377VM9T0GtHIK78WwD9D_Bc9QsH4Wiuef9Mpo-PLQ8r_QIPre2hT4JGvImJw2g")})

=== Find One Operation ===
✓ Found user by email: Record({"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "age": Object({"value": Integer(28), "type": String("Integer")}), "name": Object({"value": String("Alice Johnson"), "type": String("String")}), "id": String("bV1aR7QR0gFP0DlsLbzwdf-vBUU83wTOSU6xNqQB1AAfjR5CzMdReSfevaVzGdUY4_nO2z8ZOuM5xVzUEn9aYQ")})
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
   Inserted with ripple: Record({"id": String("e3OxxqVfnUxucBkJSTXMymilE4qE0XZAzGqxvY1Ida8wLYeJ8QdZhhCDIwRUFobMCcCMx_B06xT12rryMDoi8g")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("dyylkrWF3AdycL1GFLvE49I8Ksv2Jrb3wpF9Q6WQ6JhDFtj8ipGo2HGPLbDVVEb00uvd6m6E6fs39L8nx0QPwg")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("e3OxxqVfnUxucBkJSTXMymilE4qE0XZAzGqxvY1Ida8wLYeJ8QdZhhCDIwRUFobMCcCMx_B06xT12rryMDoi8g"), "price": Object({"type": String("Integer"), "value": Integer(150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("sxWRgacJmN4npjVt-X653PxtwUd0q8OHbkk-ww7LdweCQkX4bLlwtutlMjvbhCk9auPi6Qf4gf1vdE8l6BySxQ")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "name", "email"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["avatar_url", "created_at", "name", "age", "id", "user_role", "bio", "status", "email"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["bio", "email", "api_key", "status", "created_at", "name", "user_role", "age", "secret_token", "avatar_url", "id", "password"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "email", "name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
