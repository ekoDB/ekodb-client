make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.68s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("dsnxfu6QxFd3hsPPqPYqDh5aMjsIzEC0nulhityFM3pclyJG8jzcPCMOJsDT85angCPrch0VLnziO3CjN87obQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("dsnxfu6QxFd3hsPPqPYqDh5aMjsIzEC0nulhityFM3pclyJG8jzcPCMOJsDT85angCPrch0VLnziO3CjN87obQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("dsnxfu6QxFd3hsPPqPYqDh5aMjsIzEC0nulhityFM3pclyJG8jzcPCMOJsDT85angCPrch0VLnziO3CjN87obQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("dsnxfu6QxFd3hsPPqPYqDh5aMjsIzEC0nulhityFM3pclyJG8jzcPCMOJsDT85angCPrch0VLnziO3CjN87obQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "1keYPJvW6FB7hGaXsqhyCZ7CwoMvaiXk_VaFjue1b3Rx9dZwlmdq3_LXzLW6-GYcNI_dkQ0efAoScYSbPFuB4g"

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
        "id": "1keYPJvW6FB7hGaXsqhyCZ7CwoMvaiXk_VaFjue1b3Rx9dZwlmdq3_LXzLW6-GYcNI_dkQ0efAoScYSbPFuB4g",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "6vaceng8ohMXys-gyTeeWYvN5UGslDLQrzXRWaViLi4K3MOQCtCCm7BPSBb80pKFtHC0RnZjvCnxk6KQ_pLhcg"

=== List Collections ===
Total collections: 4
Sample collections: ["test_collection", "demo_collection", "batch_users", "websocket_test"]

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
✓ Inserted document: "GvtyKd6b3Wmt-yCU3UBxVjXNV42fl3B_3Z1V4IbtQl-j9hhb0mA1P4edIg_oi4IrkCX7bkj7BXtNv5g38M9rcw"

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("opSf6tvTFp7wn6VX5TkDZ9vV6a5NO2xRuA9vwPv5oU1Uaq6TgZww4SP--4R5MCKdokie4cCa_tlwhiSQkS-14A")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: BctjF0LS2MtH-vQf3eWRWeyGLqmnP13Ti_aoaipAc3C2rrl0wOnabRznUW7QoloGY-joOxm4mKOxoDUaZHKGbw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "uurztv3uExPi-3dfjMhb86BH2IuWAol0iN1zcRf9cX_8-LefXZtqpzlNt-tIt25MvxJCrHHyY8aIR1GFUgSg_A"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: C0V2BiLLJyuT1HDgJ6DxQg0AdvdaK0r_aljvB_AgHMNhyDoLPAvIk7qIGAKAEHbu1l_pZwwn0dgk8oNgBwG-TA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: tC-bAskEK-m_VaacUza6J02kGbFZJ2F5XJj9_Zi1Y5bS8bGWvqNRxKc3Dqji16Ekhv3w7j0el6U4tpQLP2i5vQ
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: j1sqtlaxEDKwNqo4ngGZ1JUKNF24j0ruLlQpkZGRC9DJ5xA46NKaY0aAJI6xQKZ_ZAui4BLTm3ZhW9nzZsVmfQ
Created Bob: $500 - ID: SfVxNWAYKJz1pocOtYC7cW_mg0d0YmqL-lkVUoOlkT8edjmNwjosa-qrl8zWm_RxWr9yMRgMnfdFT3lRe_--4g

=== Example 1: Begin Transaction ===
Transaction ID: b8fd2c94-9e3c-44eb-9a47-32d8bd00fad2

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
New transaction: 52b8e8bd-393a-4909-8cc3-bac8752e2a6a
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("UDODhemmTFVl-pMGKx7vabCtnDowEhquQVaFhKTJfzZUA5ZtPY38u_aTGxsUEg4IMiJlMzBrjq_cXAhdMwtyYg")} }

=== Find by ID ===
Found: Record { fields: {"name": Object({"value": String("Test Record"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("UDODhemmTFVl-pMGKx7vabCtnDowEhquQVaFhKTJfzZUA5ZtPY38u_aTGxsUEg4IMiJlMzBrjq_cXAhdMwtyYg"), "value": Object({"value": Integer(42), "type": String("Integer")})} }

=== Find with Query ===
Found documents: [Record { fields: {"active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("UDODhemmTFVl-pMGKx7vabCtnDowEhquQVaFhKTJfzZUA5ZtPY38u_aTGxsUEg4IMiJlMzBrjq_cXAhdMwtyYg"), "name": Object({"value": String("Test Record"), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(42)})} }]

=== Update Document ===
Updated: Record { fields: {"name": Object({"value": String("Updated Record"), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("UDODhemmTFVl-pMGKx7vabCtnDowEhquQVaFhKTJfzZUA5ZtPY38u_aTGxsUEg4IMiJlMzBrjq_cXAhdMwtyYg")} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "laiFVkmjWkcTspqsASN4K0KqiQxFabNZvY_OI-t534vsoYRYVYJSK1QPENLvUgvjGStxHkbdpIMqk2MY8qCSzQ"

=== List Collections ===
Total collections: 9
Sample collections: ["test_collection", "test_accounts", "batch_users", "scripts__ek0_testing", "ttl_cache"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "D2dEyKiBnQOPG3Kl6kn7Q0YJJImfVg8DPo5KxPp5Ef9t-iAtnEesPBp4BH3HQFat9JHydaKXGKiCSj9oZhP8Pg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("gtRjPr8yBdepIriNI3RC9_j6HZ2oaQJdZKl9JAjKelP2Ik-YMBlLWJjpb20MNfGE5r4WStoi4mWW4PP4NVP13Q"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: lXR6UTyTSizwLuNgavKdXwc9MN8H3mW6I9zdN47IfMz7IH71iqqweAnl7YtPK3Pca8y1gD0cAVgB2ImgIHxGGQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("_i6D_7nQorTOSKF-0vhV8kW1bXyglQWiWS_ACzT6r0ZP62-2ssBJLWRuf2ehiCs0TT5Hc0BtGAq-0PxWyfmvtw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
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
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
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
     Matched: ["title.value", "description.value", "title", "description"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(String("Database Design"))
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
Execution time: 2ms
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("sNjPm915LnQ0ecuxGINW_hzerXWs3PyAuKKLC-koG550VzX0eIKKcHpQWm88LGOWl4tG5vYFRYCcyLpVHCYmNQ"))
✓ Inserted user 2: Some(String("p78WQUGoEhgC2zFh7LDR960yxSRC0oQJexmytwYLSvs8Pjy1iYmv881CKzE2J1ane5WD-T9wKAtrs2OBMCSeRg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - email: String
    (required)
  - title: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["test_collection", "test_accounts", "batch_users", "scripts__ek0_testing", "ttl_cache"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 9msyvD3h8zah5g8WdK2SPkPlQqqs_rbgazEBNOjmdoMUNl04wtkPKyhQLv3aRA1BGzDbOwuBnOMc5aT88H9h0A

=== Sending Chat Message ===
Message ID: EEARzKax2E4tJo3kp9akWdvsCmFe4fi_jVsx5N7Mnsi9OaiJ8I8CRVZMZcAK9GRBADvfaP_CmuP5_2LqGn2JrA

=== AI Response ===
Response 1: ekoDB is a high-performance database that is characterized by intelligent caching, real-time capabilities, and AI integration. One of the key features of ekoDB is its AI Chat Integration, which allows users to query their database using natural language and receive AI-powered responses with relevant context. In addition, ekoDB also supports full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("0TDxyU4NqrzvhdIh9KZLlkKSgLHi4tpKJQIjJWRHOHsuJHrfEspANkKMnKlrT-NohFAsJeCSoBQZwPSAhHjvaw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("VPO_s8DfhVnV9xc5B-lza5LSlXs73PqXTco-HUHcmCtyG51yKuP02JPcHV0ADmexD-M0NID13znbH91QZ1DDtA"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("Ho2so0QeNUbHb-TNuZvBH-UXdHdNTvGWZ98MBlED8wh_Vqq-jlf8C2y-bTCAuWCCet84gJwX-PKsVgoPCf8VSg"), "title": String("Search Features")}

Execution Time: 3318ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: qBqsUD_jL1fpoveeqUPs3_yDNveMsqdq1TamMm0Xps-3pScyHXiEnh5jYsC0QQlaPWGE1YT6iUAx3PH-0JEQng
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: c66KgeA2jCv9T7x1FLyxg_KZGB7NWCgtIpz_j6sPzEqlKV6DKSGK-6AJ6a6abRc-AO0FPhOulC216hFZR7oGiA
  Parent: qBqsUD_jL1fpoveeqUPs3_yDNveMsqdq1TamMm0Xps-3pScyHXiEnh5jYsC0QQlaPWGE1YT6iUAx3PH-0JEQng

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: c66KgeA2jCv9T7x1FLyxg_KZGB7NWCgtIpz_j6sPzEqlKV6DKSGK-6AJ6a6abRc-AO0FPhOulC216hFZR7oGiA (Untitled)
  Session 2: qBqsUD_jL1fpoveeqUPs3_yDNveMsqdq1TamMm0Xps-3pScyHXiEnh5jYsC0QQlaPWGE1YT6iUAx3PH-0JEQng (Untitled)
  Session 3: 9msyvD3h8zah5g8WdK2SPkPlQqqs_rbgazEBNOjmdoMUNl04wtkPKyhQLv3aRA1BGzDbOwuBnOMc5aT88H9h0A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: c66KgeA2jCv9T7x1FLyxg_KZGB7NWCgtIpz_j6sPzEqlKV6DKSGK-6AJ6a6abRc-AO0FPhOulC216hFZR7oGiA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: f4FTNO5NVLKb1OGgJq0wnwqeA3WRGdBV6sKwlmTdKeMQywOKDKjJx_xxSp-fG62iNMMPFqEU3KacuntplFyM3w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product is the "ekoDB". It is a high-performance database product and it's priced at 99.

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
✓ Created second session: Y9IlcOV2FYlrt19uHBT3nw_RiNqIdVs6MtG_7rOFYFLhEOfn0b8eJ3leVDMrFWKw2U_9gaJhiyPp1eiGoMGI4w
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.07s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: z6bmN-VikPE9JkCOzLCJvHsutCIg1u7SLLk6a1ThZRTvSl_-CT6JxHmj5o3eSl_GCK9mLPvg4uAkdrwoAZTAJA
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: BnhzOYIEkJvNKGl4aDZAtfSZ7MT1YjH3J3PiOTGg3FXJCtJPhfiroNcsUvL9533k__HdpqJaLbS6t_F3wtrkRw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: XTX4tKU2ENtEja0QWUj6S27iFwek0qhY8iLc5wOM5OPzwJA9uh6BUrJ3Bzvs9Ohe28bRrMM4naqN1QEzS3PnPA
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
✅ [32mRust client examples complete![0m
