make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
     Locking 1 package to latest compatible version
    Updating ekodb_client v0.5.0 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client) -> v0.6.0
   Compiling ekodb_client v0.6.0 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.05s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("1TekNmQcl19jZx2DvtD2TOXNfjzailh3i2CUuFWl7GeBgzl4Gkns5mNHT_lZle0yU6DA0DXbH-7224Lw03Y2wA")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("1TekNmQcl19jZx2DvtD2TOXNfjzailh3i2CUuFWl7GeBgzl4Gkns5mNHT_lZle0yU6DA0DXbH-7224Lw03Y2wA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("1TekNmQcl19jZx2DvtD2TOXNfjzailh3i2CUuFWl7GeBgzl4Gkns5mNHT_lZle0yU6DA0DXbH-7224Lw03Y2wA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("1TekNmQcl19jZx2DvtD2TOXNfjzailh3i2CUuFWl7GeBgzl4Gkns5mNHT_lZle0yU6DA0DXbH-7224Lw03Y2wA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "WaBLKHHaUr5EzU2ATdkHLvXtzFbO8-_txXCCsCgLbh3hqPL8p-jLVoUzdHAgJhCQlO1uSOqvXl_I5gzOCfGMxA"

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
        "id": "WaBLKHHaUr5EzU2ATdkHLvXtzFbO8-_txXCCsCgLbh3hqPL8p-jLVoUzdHAgJhCQlO1uSOqvXl_I5gzOCfGMxA",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "702U-G9XeJr89PELj2D7VDv9DaPa9hW1eB7Fm3cafU9USGnvpiCKguw8EIqC9FF0DGmnYiI7yR9bWVgik_-aBA"

=== List Collections ===
Total collections: 4
Sample collections: ["websocket_test", "demo_collection", "batch_users", "test_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "-iooECEa4wtmPfYQiNlDML3JBaBoFke9WD7Cn2y1bW0MazDMA9kMX6ch1YRV2o7_5P5ZwGYO1Er7oFGeEWvamg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: null

=== Query Documents ===
✓ Found 1 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("A_Pnoo4a9styLmoe7RY9ck9uHsko9C-GJ8zE4bC6JZvYaJ9EKvBT4jYnjXYJS6bRjuxTeFqYOXmYvAI3BBZTwQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: bSznThWvo4JNaNusykZAhABmkm5QvUWtaoXIhXN8P4u0trmmO667BZM7gZjC44Ic8FeAFyuV9kRLqzxrkX6zzA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "WzawA4rOoZph5ZygSMDnCXEOGSQtNEmLgdWMvJCR9WJ6dIrlJ5yY_idsETYuAXDQE-_Rpazr8yOXYoWcDgspUQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: XMLVwZ4KAD2T6U8C9kQKj10MFuzknGD9QqdpFJ62y17E2HUzUOXJ3MIzXydG1Ttkf_p-UUjTww6IGcoBZe1CTQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: vbJLxhVv2KVyQE5DH-KsQDeIjJkNkZJj0u4iVaZqi6MljOK3vt-vae6EPsMvzFSHlIQZVRLXxTH-758d262bkg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gG2GSjxfH0mm0XcIkXFxAu9HLWJXjrXRfcO3Z8w-PbmpcmwylEk26Z_weVlKSWJZ6Qq0FWGuX056eeHOuy4Q5w
Created Bob: $500 - ID: WJ_wYC4NjjYjCIRshIQtuTAyKAHX7K8tNwADo5eJNINM-hBaOCRbAxrupS8A4S2dA36K3WUj6fiPFlFuQ6uOWQ

=== Example 1: Begin Transaction ===
Transaction ID: 8594b6d6-4eb5-49cc-a4c6-75bdc4e33203

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
New transaction: 77449552-146a-4bc4-9211-da404daeba4c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("q_WiSjjCCMnEzK5SxfGKsZPK-IFN7ol7XooD90kgA5KjHwXLfOIk_x5qfO_r16H3tmWBUT0iR5NABrw4QKjZNQ")} }

=== Find by ID ===
Found: Record { fields: {"value": Object({"value": Integer(42), "type": String("Integer")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("q_WiSjjCCMnEzK5SxfGKsZPK-IFN7ol7XooD90kgA5KjHwXLfOIk_x5qfO_r16H3tmWBUT0iR5NABrw4QKjZNQ"), "name": Object({"value": String("Test Record"), "type": String("String")})} }

=== Find with Query ===
Found documents: [Record { fields: {"name": Object({"value": String("Test Record"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("q_WiSjjCCMnEzK5SxfGKsZPK-IFN7ol7XooD90kgA5KjHwXLfOIk_x5qfO_r16H3tmWBUT0iR5NABrw4QKjZNQ"), "value": Object({"type": String("Integer"), "value": Integer(42)})} }]

=== Update Document ===
Updated: Record { fields: {"value": Object({"type": String("Integer"), "value": Integer(100)}), "id": String("q_WiSjjCCMnEzK5SxfGKsZPK-IFN7ol7XooD90kgA5KjHwXLfOIk_x5qfO_r16H3tmWBUT0iR5NABrw4QKjZNQ"), "name": Object({"type": String("String"), "value": String("Updated Record")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.86s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "iy67YWaA5AbaVM4uA536fbTmHlHiLnSXBX0sRZ4zpiMVu2ZHaBFTOsDVvqQBTM2_uz_IaO7KRIQcqrcQOthm0w"

=== List Collections ===
Total collections: 9
Sample collections: ["client_collection_management_rust", "test_accounts", "websocket_test", "users", "ttl_cache"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.90s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "TV1VzrvzzCkx36HFxE6eDvF9vALOpGUiA_rurpxxXBrsswG_OakA44timti1cP_RlJQx64mS8j-newoxHD3_wQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("HiTchGtRd-g9P4fECQz4CLCP9PcFgVT9rHOwrMmAU0nlfvUMfJ0Q5gr-_04Rphpv5LwwR8EiemWHl9rjXHoqog"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.16s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: h6ad659_tbMnpk-lDnXAOXoOSZGsnJnnk46mIe4BdI9PRT3wwNBCsh4zmOvoH2tPeBHzWTw-Xz2bdvAcZWI70w

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.13s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("AQqRemIzjnImWoThgWtFO9ZkoUtuiXefgAfp1IK4YCaZI8alvxwRutALXgVFKNtugeQUqFVpUekeM--L1JNH7g"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.90s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.87s
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
     Matched: ["title", "title.value", "description", "description.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description.value", "description"]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("7MXY8AX6cNk-A1IUVTJYJ5rnFQ80UM-0kfvr02eN5qiIKtAit5fD0c_oSYLFwiJLE5uWcvs11i1wI_5DaweQnQ"))
✓ Inserted user 2: Some(String("iCo5B6D7cCrSg8rqqKOp80FG87d-JvVzyo5dUe5yqSNrtdgh2kPlly3Ct7QXl9SLqxYM6o_BR1liFdNQ5k7vbw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - email: String
    (required)
  - status: String
  - age: Integer

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["test_accounts", "schema_client_rust", "websocket_test", "users", "ttl_cache"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: LsrAmQyiFnof-6HfYut53SABCHrDvt2bS-ATTomrdt03ouabmWG3DHX-iGnYDsyVIYmH3pxEdrdeqf0cTWaG8A

=== Sending Chat Message ===
Message ID: _uKjCBRChUhHxmG0fqc2nTdqxD2_Jdv-E1g9VzJvNALFxK8PE5yQGo85M1tUTf5JJ5nmt5vTu_0SNXtF1MwIGQ

=== AI Response ===
Response 1: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. Some of the notable features of ekoDB include a unique AI Chat Integration which allows you to query your database using natural language and receive AI-powered responses with relevant context. Additionally, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("1IW8RBchREERdbfYSMId5skeBRk31dLpzFmoy8EcDX0bF4sukrODFcX1M7RgLbCnctLeNwLfuhne7yjl_gHQdA"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("eBXEk4xBlzcsZvYy4q8DVPCQmCFrZMoPh0bIwtAAc2r1wv9hL7jpcnfml-uaC7z1g1lQKcDXUUQsqhKQ2GTTDQ"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("l1wV-E60vtbTt-3Uc4dhK9SMOp5713XV8EU5DLLw22TI_KYkp7HPu7tI-SFrgL4doggqYCE0g9ie7ZDcn1-uNg"), "title": String("Search Features")}

Execution Time: 3553ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: G4DhRHz9-8gw8a7-nQBcBLd29RZmU5B9ERthrec45Dhw07EDfqLMUnzuBtpz7-EnRrin2N2AEJOh2lhgCH-HZA
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product with AI capabilities. The price of the product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 2hCJF5m0Iw44tAWmiuyDO7AcQe8BOCv17GsciKI3lBCZ9Z4g12ELH5sVmTOIQDBPSp-HimqofH7yO9Cdhl_BcQ
  Parent: G4DhRHz9-8gw8a7-nQBcBLd29RZmU5B9ERthrec45Dhw07EDfqLMUnzuBtpz7-EnRrin2N2AEJOh2lhgCH-HZA

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: 2hCJF5m0Iw44tAWmiuyDO7AcQe8BOCv17GsciKI3lBCZ9Z4g12ELH5sVmTOIQDBPSp-HimqofH7yO9Cdhl_BcQ (Untitled)
  Session 2: G4DhRHz9-8gw8a7-nQBcBLd29RZmU5B9ERthrec45Dhw07EDfqLMUnzuBtpz7-EnRrin2N2AEJOh2lhgCH-HZA (Untitled)
  Session 3: LsrAmQyiFnof-6HfYut53SABCHrDvt2bS-ATTomrdt03ouabmWG3DHX-iGnYDsyVIYmH3pxEdrdeqf0cTWaG8A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 2hCJF5m0Iw44tAWmiuyDO7AcQe8BOCv17GsciKI3lBCZ9Z4g12ELH5sVmTOIQDBPSp-HimqofH7yO9Cdhl_BcQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Qkp1QtnL6TebZAb1w_TQzSPQ9qjTUgN-h5TiyKX04eRxQLO1YQb8S7j17wE5_ODByMJ2mLuKsgKYARRHaNO-UA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product that is available is ekoDB. It's a high-performance database product priced at $99.

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
✓ Created second session: OUSEtgQ_-7kKWGMt4AmVYrdXPPGik3263KCnNt9ueJAKP1k4kK_PUO1YW4JlGYF-TG2Tmcwoh-qhMRAInCKHkA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: lpmQIrAjIg_2S3adqpCATGHcxyRcKzIj0mYhKSdStT07oleSM1Y-q-IlKox6W2Pot_h1fDGK9Hiinat_SeYOUA
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: YdoFVmOWaFhVta1Z66EXrezOX-duheILWwWxWiZbEZ63gmAz9nnRTX-YQ4uFdWCGv-gsiSuLkUB5WFvpelZhEw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: hMUYh4SzDdOKV530xZfx7VX5-rj94FYhxg22sVE3-hv9hrjDp_LVaQzBWvoSdqG5Qia45Z_8PsEVU74a2mr_fg
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
✅ [32mRust client examples complete![0m
