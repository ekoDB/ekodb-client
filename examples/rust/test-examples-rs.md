make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
   Compiling ekodb_client v0.7.0 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.29s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("9iqF6jlVozCIW7OT_9AoJHH8uePAkx7SMLYE4Xkvn27peUzjT9Oq4Np0pl-xr2rJVEvkc-gpP8hJLsqHUQHlAw")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("9iqF6jlVozCIW7OT_9AoJHH8uePAkx7SMLYE4Xkvn27peUzjT9Oq4Np0pl-xr2rJVEvkc-gpP8hJLsqHUQHlAw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("9iqF6jlVozCIW7OT_9AoJHH8uePAkx7SMLYE4Xkvn27peUzjT9Oq4Np0pl-xr2rJVEvkc-gpP8hJLsqHUQHlAw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("9iqF6jlVozCIW7OT_9AoJHH8uePAkx7SMLYE4Xkvn27peUzjT9Oq4Np0pl-xr2rJVEvkc-gpP8hJLsqHUQHlAw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "ZZP8j_9ycdafEWgY6Gv4o5YVw02nFkt05huGnFkyCMirV19KgWpN2J4AepMNXW6mBJu2DEx-dtNYUnc9a7rKIw"

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
        "id": "ZZP8j_9ycdafEWgY6Gv4o5YVw02nFkt05huGnFkyCMirV19KgWpN2J4AepMNXW6mBJu2DEx-dtNYUnc9a7rKIw",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "UaqLbdBuemFSE2dQL8n1k7DSRaNBOSiv5m4wzwxM2tfmUdyG75Nd1VddJR-jnsBA-vVH2CfTyZHjfam8ZwkvuA"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "test_collection", "websocket_test", "demo_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "4LOUhE9IoPo0aIZqG8sSmRV5MgxVP_W0zzFro-5Bq9J_73X4ZWH2wpDwEX8bhawIGMmKxQbuJmDXpBH0ydYAww"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "mr8MDBWSyo4VmRjVRXkIjou84qPBjFU5gDkFx6V3MX0wDGwXVLDBHus5xvsy5ahXMYnWKY0mq80czouEHvvjXQ"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "BNoLbs2ws_AZ0JwEsvoD5r-EuB605lIuN65re9rsJPw39-7imSG0IWDB3zfeC8RjfihbOcepnSzUnOeCDw_mzg"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("Q1SUjR9Htybe-4xoFLZGONHSSFklRsml9oM7UfBdvhWwx_8jE1FdVklqoOGjOGVHvUZ3ewoHHbbW4KmxbBOlwA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: xRXvp7wR6GHxXPB-V_dupia2fJ0R3TUSQ-ZEImS28IzeK9tz_6C7eJJzhbG30sdybnY1yqxvYh3yiRyOVVZvKw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "DXD0BXUij2HdMB6wkFcy7G8l7d8FKyUckbbNWxF13WI3FwB0__nMdKJo2i3kf0AsYTwvUhV8hpUUJmNdFAyiDQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: Zk6OYM5Mb5KuznMqJ6B8UHrgJO1yBaq3zLJHmpR07_4HPfC3IFIFHQGZcXj_OqS0QMohYzirChX-VY9WXXSxjA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: rqkkhuaNl15QWxvBgHCMdrgAhpy3hhZ9bFPRrpaWiOlRcNcL-D0u10REtLOaMB1SY25U54Cdix7_a6S8ryraZg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: S2o6DO0XvFsBS9OEpCqJDidcKPnR0gM6Pn4SZCbZin5P9S2Go49AnaYh1Kk_sLX-UolyupjZdNIsom1KCp9XXg
Created Bob: $500 - ID: uQ9_jDXmlRGIcbnB3zDywvDCHmA8YyExgUVTwOfnXdjMfj_B-aRfz_WGYn1pIEIVgMHU7QEvB_U7wvQdHuq5zg

=== Example 1: Begin Transaction ===
Transaction ID: 34022d6d-c727-492d-87ee-81cf9410c8bc

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
New transaction: 77a924f3-4f7a-4d12-aea7-52564e18889f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("j5BAKL9lIy9tk5D_8b1oAbZk_SmfPg0__fSQNr6Ydlkax7nA9zbazbO5dT2lIZt1xVwPEA8Wto4_bSBnkufC2w")} }

=== Find by ID ===
Found: Record { fields: {"created_at": Object({"value": String("2026-01-04T00:29:23.757185Z"), "type": String("DateTime")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "id": String("j5BAKL9lIy9tk5D_8b1oAbZk_SmfPg0__fSQNr6Ydlkax7nA9zbazbO5dT2lIZt1xVwPEA8Wto4_bSBnkufC2w"), "name": Object({"value": String("Test Record"), "type": String("String")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-04T00:29:23.757185Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "id": String("j5BAKL9lIy9tk5D_8b1oAbZk_SmfPg0__fSQNr6Ydlkax7nA9zbazbO5dT2lIZt1xVwPEA8Wto4_bSBnkufC2w"), "created_at": Object({"value": String("2026-01-04T00:29:23.757185Z"), "type": String("DateTime")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"value": Float(99.99), "type": String("Float")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})})} }]

=== Update Document ===
Updated: Record { fields: {"metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-04T00:29:23.757185Z")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "price": Object({"value": Float(99.99), "type": String("Float")}), "id": String("j5BAKL9lIy9tk5D_8b1oAbZk_SmfPg0__fSQNr6Ydlkax7nA9zbazbO5dT2lIZt1xVwPEA8Wto4_bSBnkufC2w"), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "name": Object({"type": String("String"), "value": String("Updated Record")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

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

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "61kfaeqZYssoTFJlGw_wgMAErt64IvEiawpOAJPI7jBfGQGCbpzDB5QuO-4r5gG-9Euo3FBAoe4uvv4XhBrN6g"

=== List Collections ===
Total collections: 9
Sample collections: ["ttl_cache", "users", "batch_users", "test_collection", "ws_ttl_test"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "N6Dcp0jBYter3vvEhKc_9J2JH1ANnBRT24RV47Q5_ScsAndmhAN0Qp-lWGovEIAFuhSphuctEYH69qTxb6E7WQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("IVDuvCL3V7SQAkrTYMi84w-fEk0HJrFQf0Ko4yES6Gi4A5RAx6tWBK4ei-WjQc9WdZOMlJwIox5zhV2_6ZHomA"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: ZEsbe0excMcHJY3fHZk6KR6p_akNrsHZIFNcq7uSqOTGjvKDAG4LocZfZJdWXjeUr2TBYTmKERcRA-tEKsvGzg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("yCpSNe7uek630QC_MCVBC3GOxuYg3srThxxI-efOK5622hzWCKEflBa0Ro-M2UmGZK-BOsBBcP73dx9PWrSn_w"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
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
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(String("Rust Programming"))
  2. Score: 6.6000 - Some(String("Python for Data Science"))
  3. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(String("Rust Programming"))
  2. Score: 0.5500 - Some(String("Python for Data Science"))
  3. Score: 0.5500 - Some(String("JavaScript Web Development"))

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(String("Machine Learning Basics"))
     Matched: ["description.value", "title", "description", "title.value"]
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
  2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("vrz5XUqCH6BLm2XMK0Vu9zpskEyS2ZG5KNXuKVxBGFZvNgolPHtNzhQgWaSfShps_gcdzxsCV7_WGdh9b9cEgA"))
✓ Inserted user 2: Some(String("Ct2fHIpblc9X1uqTYLCNhyYwbn7751wCiBwUCu70ETUnqgyie4077--haJTdvVPR5caD3t4NfFiY3eyszMwvNQ"))

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
  Sample: ["ttl_cache", "users", "batch_users", "test_collection", "ws_ttl_test"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: iqrOsUYHuTFlzfksowEe8ciD6bzA2e94cOOZ7GyKEsD15VCgG1hxJsYOpdB_3OLEg2L7-kw21Vk-ZKuf-Et5zg

=== Sending Chat Message ===
Message ID: NIUeEfHPBE7MufIS7uWu53rSrc9OEDiDIRWjGuTtSZw2WJwM11OLBJaaDXSRq4VXR3CAYLmA0oz7XB9O14jnhg

=== AI Response ===
Response 1: ekoDB is a high-performance database that comes with intelligent caching, real-time capabilities, and AI integration. One of the key features of ekoDB is its AI Chat Integration. This feature allows you to interact with your database using natural language and receive AI-powered responses with relevant context. Another feature is its advanced search functionality. ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval, providing a comprehensive and efficient search experience.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("V95AVraH1xINV9ubryVcl1WXGnWN3etnz5oZMUy5mn39ccBwXaFJ_9lxHf0jxXf3TWvzc1EcUyNBWqFBrbNpMw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("d7GZMag0l8N1xH_JvrF9uo2Hw5hNIGGiMLNs3Fb47B1eOGxVU7lcy6Buohg8dj-6S-SkR1YO1x95X_JgBNWW6A"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("xQ98jcbAAsIOZV-plHAwu28GNiVqQHmZHZfSGhGds5uvkOSC9jruxzbVDHZTMPk2cogDE6NXdhBCFaLbFxPeKA"), "title": String("Search Features")}

Execution Time: 3067ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fabuYeYu50GbJZH1jgr2QWukNk_sPLaArrv4BWAEY-PAj_WKkMnZMeNrYn43vbRJcrsUksRWuKDz013FF4LMuA
=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The available product is a high-performance database called ekoDB. It is equipped with AI capabilities and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: yWUpwVYu--b_Pn4pITLdSFVSgraoyED5DPP7b_da-9RfaqncNDMNa-TFKSK6vAlfmJ76UfNb8zZOC1iHyjjVxQ
  Parent: fabuYeYu50GbJZH1jgr2QWukNk_sPLaArrv4BWAEY-PAj_WKkMnZMeNrYn43vbRJcrsUksRWuKDz013FF4LMuA

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: yWUpwVYu--b_Pn4pITLdSFVSgraoyED5DPP7b_da-9RfaqncNDMNa-TFKSK6vAlfmJ76UfNb8zZOC1iHyjjVxQ (Untitled)
  Session 2: fabuYeYu50GbJZH1jgr2QWukNk_sPLaArrv4BWAEY-PAj_WKkMnZMeNrYn43vbRJcrsUksRWuKDz013FF4LMuA (Untitled)
  Session 3: iqrOsUYHuTFlzfksowEe8ciD6bzA2e94cOOZ7GyKEsD15VCgG1hxJsYOpdB_3OLEg2L7-kw21Vk-ZKuf-Et5zg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: yWUpwVYu--b_Pn4pITLdSFVSgraoyED5DPP7b_da-9RfaqncNDMNa-TFKSK6vAlfmJ76UfNb8zZOC1iHyjjVxQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Po05jgaFBcAoeeguSakXdwTWk6_iOhPii6-C2iT1xoZlDZMwEgp4rZjOGBD3oRXMejUYzSiCpwb-E6u5rx0jvg

=== Sending Initial Message ===
✓ Message sent
  Response: The product available based on the context provided is the "ekoDB." It is a high-performance database product.

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
✓ Created second session: nG-we3gQtp1gSDq6fq6xMPlJ-4gSKMIesqqHvUqGbz21aNYku8Q_W2jLmV6iL1vTODEep_b5rEryHUaH2yHB-g
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: tddzsqilEZpxWXUFnSSlRPw4Zy3Ao9kXUhEgP8rUgprADvfErecFHCY5gaVl58MFmN0G1E2RFUV-FIkBOOckHg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: SN2ohf7YvsDAQ9g-1WQvU-onZ3g_cbUC2y5QzN9jxidYEU7tvRYfkNEcn8lIg913HjFQnxFpgFIYatJYuNgXkw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: seZ9Hmg-3waa_1uIpHvHm8msyNe8Dpujh5pgs6s0L6YKCilIZAO_y9PejxQINVDFQlmSQ9h5B3KmTnRZB3bBlA
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: O38tpYM63YGPT9VnLvvVJRMxkCzD9FJaXEfguFAejCeCfk6f3OE-45ewCIYkU_Qy1zsX7Zbe5S2kOecK9iYbNg
Created Bob: $500 - ID: r0s1fm3Jp7je9Yt-x4h13z8uuCckibBZTb2xYCqjremCfL17yxC95t5ACkBfAUsc9aV1Q6dgcYDlrC6MyGit-Q

=== Example 1: Begin Transaction ===
Transaction ID: 37148357-d899-488b-abaf-a09709981ec5

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ae075761-282f-487c-8580-33b4af3ac237
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mRust client examples complete![0m
