make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '1AtHKcn9CuPYRp4FSYPwRI7WDgRuuoB81NCIw1e622yAKrZEv3yPJ-oBVkq7BBVlJkALHMNKivZW9fH6hjlDdw'}

=== Find by ID ===
Found: {'id': '1AtHKcn9CuPYRp4FSYPwRI7WDgRuuoB81NCIw1e622yAKrZEv3yPJ-oBVkq7BBVlJkALHMNKivZW9fH6hjlDdw', 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '1AtHKcn9CuPYRp4FSYPwRI7WDgRuuoB81NCIw1e622yAKrZEv3yPJ-oBVkq7BBVlJkALHMNKivZW9fH6hjlDdw'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: OzuHNu3HBCQJv4o_AkTX0qjzwIKg-qfalYHGh1k66UfW3VutE3O01Y0n_pg4otwK3O-pyXBVTeGvX6uut0kfyg

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "OzuHNu3HBCQJv4o_AkTX0qjzwIKg-qfalYHGh1k66UfW3VutE3O01Y0n_pg4otwK3O-pyXBVTeGvX6uut0kfyg",
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
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="{'value': 'active', 'type': 'String'}"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: wHuYll1Qa-v1w_yKPEpdWyfiAn-K-O6TaO5pmVE6uTWwRQZNQHicAaILtbQHToz260AYSdF0Kw86gwrJRwe2Nw

=== List Collections ===
Total collections: 11
Sample collections: ['test_collection', 'chat_messages__ek0_testing', 'test_accounts', 'demo_collection', 'batch_users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 2_6dnLeIxG60BAdyGDdG27djj8yPsEGVMw9KRBUNBK9Aq-Sm9mCLmje4pDHqwhpABv5hpsupov3d1VJEQaAG7w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: zmnKfe3N63yRzSso1TMQRIj_X57MTvaVW7vlFXyoa-zNr5hwOfaqF7LcqniL82EnIITRcOQ04lI-vyyxmpW9tw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: c5GgU_cXToYurBJ8A-JxoiynN8OP63Lb411S0MUCnrklE60Oaj1IeTZs_UzDwDO89h2os3CTn5Q295PAiqvtfQ
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: bZZIjs65RVK28Q8-h3Ucvm0SiWQu2C6VQ7HoeHrcPqeSGJjnUNXh6_fIUkDVIlwTNbUtwh22TozUn_liYzP_KQ
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: xJAvr2GoM6mWIeIcSq8rZNbEB_dMQwf7i1leGx_sUZVC-koJZlQu-tGCqKKPIZIkmu8hUd5z-pCZ8vis9XW-5Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'count': 15, 'max_score': 90, 'avg_score': 50.0}
   {'max_score': 100, 'status': 'active', 'avg_score': 60.0, 'count': 15}

📝 Example 4: Function Management

📋 Total scripts: 8
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: False


============================================================
🔥 TEST 1: Original Config (durable=False)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KnH07QJMEGkVQXjZaf3HOFtPG48D4vPF6iwR0faGKyQoWLycphuANuV42KRhHnTwXZh9hMSwStKvbqyCeFggNQ
Created Bob: $500 - ID: M5LAm3USWgzTaxxcz2PfiA4U0jZLAm08T4h9TJTu3uXJJrnFT7sTASNM_760pjfZlEFUNrnOw9JZBGeymo1RQA

=== Example 1: Begin Transaction ===
Transaction ID: f5acdf8e-496a-4872-ab9a-a8f2a3e60336

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: b38a414f-607e-4275-bb7b-51d215d518e6
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: qAi0kT5o-9tm6ETRB2re68RqXMq4-6MAIoCJLQyxZSzuFJYiNKa_cn_LFvs-3JSPyIO0eSLLzcp9G3-ersEL-g
Created Bob: $500 - ID: 1xSn9z9Tc_UhELrsSEoNdVK0ke6yp-vxnCv8nck_UAq_q3eiOVogHiz82FB8vjucQFfDy7y9ym9Bjfe4SDLNIA

=== Example 1: Begin Transaction ===
Transaction ID: 28f7bba9-1994-4c74-9397-8a9489d99c63

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: y3Pb93skT45r7UkPjQNm_F5BCrtgPD2pig1drMvUKTw2ACbUy76446tZDKqu2UmAKM8idHMQvCx7wu_U_Hdfcg
Created Bob: $500 - ID: PhMHpDOb8OAuEAVDrjjoVUwusRQ1MHvAPgbXrv2vdge4Eg_c8_TOeho7EL7bKIs6Px0RL20WlkoMfZWuTgbPiQ

=== Example 1: Begin Transaction ===
Transaction ID: 523b39db-d669-4ec6-a7c9-cba98717fd7e

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: cccdafab-7028-4660-bea1-2b75798e3b21
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: s4lSmGDXkTDxkjclMnLe21pe0yFvRzBnaOLvfXtDo6W4FCJDIXu4_k0KtNyF6FhJAZbIDkgTITvAjJqzLHctXg
Created Bob: $500 - ID: A9EGmP2spLLdjbBeHbpHtttZXkY95upTqEWSEYWEt6Aw8Kq1hfGxE6KjOAYH25uYdATVM76z86pYNvRkM_PURA

=== Example 1: Begin Transaction ===
Transaction ID: 9c17f7cc-87a7-43f0-9c28-de404f65009d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: g-CPKnyqiIHYG-piHYV2c6dWMMjQrD0j36FH_xgiPSEqNEcP_ZmgBWcIj0rcZXVK7TG8kdNYkhaEHbQqZ7Qrlw
Created Bob: $500 - ID: V5jfjfriXk2Mcx9h4nIq1bjlZZA_Nx_uR71xXxR9IuraWQqcHzNzKnbImI3Fmm_GSjzmKj7Ju0lPcVuaj34WVg

=== Example 1: Begin Transaction ===
Transaction ID: 42d6e1a0-a277-4db0-8f87-130a5d6a9106

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 95231bdf-2041-45cf-9adf-8f0974ae8afb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: NHYFhZqlMXjOE0Z_E_BRQSGUAplA6NH_YK6U1-uM-q3yZKpjfkPL4ERii4EaTLDuuf4Tohh0491D2_UDnPXVVw
Created Bob: $500 - ID: KHLxXlu4wNORXruVJCdLLG8MPMaAOHrFTR7lMQPcy2eGjs9WQ01fAMN6P8aW11eOX7v0A-oQmVEAFVMociGoBw

=== Example 1: Begin Transaction ===
Transaction ID: 71063f42-2aa0-4e02-8945-d078607bfedf

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=False


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
   ✅ Script saved: aVED9GRJX4uSydlNVujB-Fsmiv5Jxy5KVAIwxChkEypzn5x4h_t9zTnfwDUbTvzUWLv2vEiJSZ7FrEveHWVQ7w

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: LmrZ8sgUzOwIE_0qF8o_-Cz3YQLbgLK6vsCqDzC8kFqmpQM3PdXxW0gUibQKYlh8vvWI4e1Ljvdnw_O_n99Cow
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: eOrlt_hLMpxav0uJfMn46ByiqfoIcrIPmf4eCt6b0N0pN-D5gRAoSCAxzG-W9cG7rd0344rvTzSXQtQz-oFT8g

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: PxoFY16zFc70AIaL2LOQEm2qydRvgdXHlK1rfAY7wk2kbi8kTcCRKafZfgRw2yH_YSuhASaPfX8nh604Kpg5Vw

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
   ✅ Script saved: 48quKA2NdX1WiDvB02OK4HZgSOYLFeczDLQQ1OGALCAz6XnUzdnSYgTbcFTO8hM_Ps16u7SxsRBb1tA2vh7TuA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: aVED9GRJX4uSydlNVujB...
   ✅ Deleted script: eOrlt_hLMpxav0uJfMn4...
   ✅ Deleted script: PxoFY16zFc70AIaL2LOQ...
   ✅ Deleted script: 48quKA2NdX1WiDvB02OK...
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
=== Running batch_operations.py ===[0m
[32m✓ batch_operations.py completed successfully[0m
[34m
=== Running kv_operations.py ===[0m
[32m✓ kv_operations.py completed successfully[0m
[34m
=== Running collection_management.py ===[0m
[32m✓ collection_management.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
[34m
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
[34m
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_scripts.py ===[0m
[32m✓ crud_scripts.py completed successfully[0m
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
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.5.0
    Uninstalling ekodb_client-0.5.0:
      Successfully uninstalled ekodb_client-0.5.0
Successfully installed ekodb-client-0.5.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'MV5DJqLR1GLIGEsTNth4caUFZ18x_gxTXBzQoZ55GxOUR1qd63QTH92C7BcWqWtV0N5L066AInykgiwzJRsH4w'}

=== Find by ID ===
Found: {'id': 'MV5DJqLR1GLIGEsTNth4caUFZ18x_gxTXBzQoZ55GxOUR1qd63QTH92C7BcWqWtV0N5L066AInykgiwzJRsH4w', 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'MV5DJqLR1GLIGEsTNth4caUFZ18x_gxTXBzQoZ55GxOUR1qd63QTH92C7BcWqWtV0N5L066AInykgiwzJRsH4w', 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"userId":123,"username":"john_doe"}'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'value': '{"name":"Product 1","price":29.99}'}
cache:product:2: {'value': '{"name":"Product 2","price":39.989999999999995}'}
cache:product:3: {'value': '{"name":"Product 3","price":49.989999999999995}'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "_GWmq746FRcETwQ_Qh1SllxAdZtPDQHYe-OdLQCEiyivtLhhQLLx4PB6dynlTyhBA5VKng2eBiV79fqqzO5LoA"

=== List Collections ===
Total collections: 10
Sample collections: ['test_collection', 'chat_messages__ek0_testing', 'test_accounts', 'batch_users', 'scripts__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Yo5NWLsw45fUB4EyAylYaEVj_RzmQCyzCkmsErafPPgZ853cDJXJ4s0hz9jaFUA66yk3Y2JQ4ZeXTQEzLDujxQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 6tkHzGu1jZ_V7VI9ItSLIUdDPuh5e_dNZR8jwvHUa1fmOzKc9xUdWBY-zRB9BpcMzmWZmvc-0ACmKeIGKwbv9A

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

=== Inserting Test Data ===
✓ Inserted test record: rd95n4vyBYMJ6FGzJ3-iqtYzRGoTkUZ7beZg0U0cQfwgIsauBZKA7tN6k7DSYyhdZzEv2zAlQTjFRmRB8FOh5g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: x6RbkuyF1Z0TQyoErHkRu-we4yXXrQ9ba8fXnfS_rgry3B6fdGRVk8_eJ7014hqfantu6vpKeCruRCtO4bJxuA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
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
  1. Score: 25.740, Matched: email.value, name.value, email, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title.value, title, bio.value, bio
  3. Score: 26.400, Matched: bio, title, bio.value, title.value
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title, title.value
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.768
  2. Score: 0.766
  3. Score: 0.710

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.737, Matched: title.value, content.value, content, title
  2. Score: 1.136, Matched: title.value, content, title, content.value
  3. Score: 0.497, Matched: 

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: aoe6MEuL3E1EmuO5qnfz0zmj9UCxpmR5TGic-qgUuC5Tr0b6ynRYl1hobYCeqXxu4wWURBibPcRgbY8-SCF3RA

=== Sending Chat Message ===
Message ID: cZeWIg4LhqkBTahhFsFOtG90b95gP6D3-53YfV7ocwpaNu8YkTegL6JWiREC5rISOGf-PDI-syMAyEA6aahdbA

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The cost of this product is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. This product is priced at $99.

Execution Time: 3520ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 83
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One available product is the ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5DJRPU683OG1Fl0L2A2OF6whPsszJjmAV327yfLnemyz_lPLyyin_k3-YzCAdaCqqxxDUCY5lTvHKWaDnPMEWQ
  Parent: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: 5DJRPU683OG1Fl0L2A2OF6whPsszJjmAV327yfLnemyz_lPLyyin_k3-YzCAdaCqqxxDUCY5lTvHKWaDnPMEWQ (Untitled)
  Session 2: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw (Untitled)
  Session 3: aoe6MEuL3E1EmuO5qnfz0zmj9UCxpmR5TGic-qgUuC5Tr0b6ynRYl1hobYCeqXxu4wWURBibPcRgbY8-SCF3RA (Untitled)
  Session 4: Y9IlcOV2FYlrt19uHBT3nw_RiNqIdVs6MtG_7rOFYFLhEOfn0b8eJ3leVDMrFWKw2U_9gaJhiyPp1eiGoMGI4w (Untitled)
  Session 5: qBqsUD_jL1fpoveeqUPs3_yDNveMsqdq1TamMm0Xps-3pScyHXiEnh5jYsC0QQlaPWGE1YT6iUAx3PH-0JEQng (Untitled)
  Session 6: 9msyvD3h8zah5g8WdK2SPkPlQqqs_rbgazEBNOjmdoMUNl04wtkPKyhQLv3aRA1BGzDbOwuBnOMc5aT88H9h0A (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 5DJRPU683OG1Fl0L2A2OF6whPsszJjmAV327yfLnemyz_lPLyyin_k3-YzCAdaCqqxxDUCY5lTvHKWaDnPMEWQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: yL7pXRBlCbxDQCriqCUaKTKT-jTMr1BMK4YHE2Afug7_AzMEtOwEKl94HVItuITmj7Xf6LqIi_NOKAxvsWmDoQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, we have a high-performance database product named "ekoDB" available.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'token_usage', 'created_at', 'chat_id', 'updated_at', 'id', 'content', 'role'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: rPOuQWVT2njcPDiEPMECJJCqn_dUz-waRLpPt-MeiwbHxylvroUkBcOh8x-E9Voy0mCLjYHyGVIQIQGvw2Fp1g
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
🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 9D8UuEx1wq72JlHL1UJlYJ_rR4a77dVblYQnWMNOlz-Sx1TxPyL6-2ieXniwVpoNA-S395laN8m0ecOrTXUSQA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: KSVffDnO5fn36sfXVZzaXjbZ-mSOlZAESCurpQPg2ShrX8u4qpjMseaufd-Axgi4fM7Jo5tsa4s-6KcmvOQctA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: bonS1xRyFuAU1UHibWRCzUx55fdMvop83iNgEbp6G6YcSdNyta7F8YduJ__xL6H_NH9Q-O5RlrYL_t1Q3sd44w
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

📝 Example 4: Script Management

📋 Total scripts: 10
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: gKOf1xf9Jnnz_WmcdkUcJLWXnLRnY_2m_-6tv-AX29PcW7lj14aYvG5lhFDb8b9Q45wwd_cVjx-3MTme3CGVtg
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
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
✓ Authentication successful

=== Insert Document ===
Inserted: map[id:95hmCDBSwQLfm124cDJu8JVU9nYonr3aJZBdfm3W5M9yfY6ehtJozWEcZ6PRRdhpyuvgodFdF8S_eAgEONeqPQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:95hmCDBSwQLfm124cDJu8JVU9nYonr3aJZBdfm3W5M9yfY6ehtJozWEcZ6PRRdhpyuvgodFdF8S_eAgEONeqPQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:95hmCDBSwQLfm124cDJu8JVU9nYonr3aJZBdfm3W5M9yfY6ehtJozWEcZ6PRRdhpyuvgodFdF8S_eAgEONeqPQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: rncnCNXtn9TifyjIRBuSS9NwEa4LkEujohxC3uUuEvKmgfRNVOk0eFL9shR9g3HdyKsv0S0RKI11q-cTGLTC6Q

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "rncnCNXtn9TifyjIRBuSS9NwEa4LkEujohxC3uUuEvKmgfRNVOk0eFL9shR9g3HdyKsv0S0RKI11q-cTGLTC6Q",
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
        "id": "OzuHNu3HBCQJv4o_AkTX0qjzwIKg-qfalYHGh1k66UfW3VutE3O01Y0n_pg4otwK3O-pyXBVTeGvX6uut0kfyg",
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
=== Running batch_operations.go ===[0m
✓ Authentication successful

=== Batch Insert ===
Batch insert response: map[failed:[] successful:[psl2OvehUK7aA8F0QodILbJAieiVRFd0DzNUlNeEvVEey8OdkR_W1RnA8ZzaM1eWHYiJKWPI8kbdDbpRx7GoyA jdnm6CRuy7NNkfbbLZLTFhewDtvOgzZ1zjBOnPh2QYhPR8t5BFRIl5NTi9K_FpKjW5EdX0Fm3jsXMVNs98iSBg BWw6UcDH8TcpLf2ryIJXbvXbEX3YKr64E8LUfjIYt5qFQMisg39sby1AfI_NqJO5TTEL9TcvsIl47N_0--0hmQ pzPWFEGPHlZX-i16yT2Lo1RL9QMNtSeemiku9JslyU9RBhOktLHvNdow9eAzM9lkRCfePZun-w22jJ85yrZhGg dTM4Ba_aaKVVbbX5I0vUig2Rx6eAe0m-7UNOlE19tnXbKMNdbH_lTtfHV0BzIj5QeO1Zdb6_S_-5hnJH_q_1cg]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[OZ_su3v-zBxR8FoYQWgF7CYD9VcIipeYahZzTKgg3nHzlIymMfFrNhkUTLcBbPz4csvQB88kbbyag8_4E2jHBQ _1C7jfbuaRRpoQ41s1y3rOBVt-44iHQBijAGp2TpwfedqysNK4N38jLX46K26tb3eWuBkNVNPxz9C2sLKXR__w VMFDa8gJaSv97kiFGG8id5xS1x4AklcC1ER9yWi-LVM-JNWvPDuD1tPnAhY48iww6q0QtpckUkDN1H0x3zAZVA]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[OZ_su3v-zBxR8FoYQWgF7CYD9VcIipeYahZzTKgg3nHzlIymMfFrNhkUTLcBbPz4csvQB88kbbyag8_4E2jHBQ _1C7jfbuaRRpoQ41s1y3rOBVt-44iHQBijAGp2TpwfedqysNK4N38jLX46K26tb3eWuBkNVNPxz9C2sLKXR__w VMFDa8gJaSv97kiFGG8id5xS1x4AklcC1ER9yWi-LVM-JNWvPDuD1tPnAhY48iww6q0QtpckUkDN1H0x3zAZVA]]
✓ Batch delete completed

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
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
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: v4SekgwbP6c2Fh-wnIUb21kBF_slzsQESIRzxGFZpedmBYwUIZ9FIPgZLk3VJ6IpkDKVrNOwQ4utjK1zcDJfOw

=== List Collections ===
Total collections: 15
Sample collections: [test_collection chat_messages__ek0_testing test_accounts schema_users_client_py demo_collection batch_users scripts__ek0_testing schema_employees_client_py ttl_cache schema_products_client_py ws_ttl_test users websocket_test chat_configurations__ek0_testing schema_documents_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: FjVvf0tV8MLadImR-t9leHQGDejGgz4IfoyldkNAUaBEVWYc6kTBOJPQ1lMtOQX6lUfs4ZeGkFeJCDwRQIPYJA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 5cDVDf4j1cEKomnuXpEvFC2IL6zpeBkusr1N8DPUCRhv2CPovhT8xdjM4UuFoAaTdOd85vHywfg0he-PMEENVg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: K_NTH8GamaOMsKqK85fI0x1uuI9LrfRE7i2IpyUxhJ-pm9-IhRZRwIuZWL1SPwAHgCAmh3SzzxxVh6Mqa2YZ2g
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: K3dVs7AwV2ss4OiMbQAsgCVGKg5PgsRVCW9ZFXybqKm-Vnn8miSnGwa16d9wO9iDSqfNSSzS9Tdg3gjb26A7rg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: INekY8BDC6KU05_1mnxX1I_ndowSNwINp2jacq7qjeD4hDEoLYd0NQq5GuFTCtV8RIUpKboh2fWGSuySwYoUgg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 12
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.go completed successfully[0m
[34m
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: GoPktwtTzydm-aAaQo8b5DOqlFORBLNXn12uKF7nlj_MUzkNfl_J9WtLLFlCUZrK4x8_Uz68lJT7ZTweY8CeAQ
Created Bob: $500 - ID: r9Ud2dexDLqUlqjNJYaazv4cApZTo3EpUKThVbo0YnrAvbXVvSkaq5jV2VwGMvs7HxPhFHQstXbaPxwSQtFJvQ

=== Example 1: Begin Transaction ===
Transaction ID: 0c10a4d7-a00b-42ba-bcb1-127bc4426b18

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: 7cb17e63-8848-4ecd-8457-5be3fd264bd5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

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
   ✅ Script saved: skBPSt1LhTHkjzxy4xUQ-d__tslBRaTuv5ju1PRugSPFVx5gPY6du0KkvkKceoljwv6ycnLt-l5z8CVqi3E_qg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: TYDAAOIqApG5_5O-ZX30XC8iBJ-fU3knRqGSpdosiJbcwQf7F43g2eWS7y_V2mZfuX_ptPwX3fzS-UiVaPWvOw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: t9RDaRfycR71lNxTsm3eS2JVDUTPsDLGkg94qf3lzek_ITzpiysm47WaLyhgn7II6Unhq2TfY4TzswkMtSZs9g

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
   ✅ Script saved: tKnuP-gKD0T5uYvfwiRP4wBBFABNSgQkslKFMCtXE54aSUwFZJQMTMj1jPhh46cKtpf-_5NpifNo9plPhfLpOA

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
   ✅ Script saved: mOUZYPYNWEZpnqvLl0KVtnkvfOkuXl7UpS0AKkybLou5QaJne3wmcdlKO3VPx3VceS-t7SOlQcQw3uOA2TAGVQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: skBPSt1LhTHkjzxy4xUQ...
   ✅ Deleted script: t9RDaRfycR71lNxTsm3e...
   ✅ Deleted script: tKnuP-gKD0T5uYvfwiRP...
   ✅ Deleted script: mOUZYPYNWEZpnqvLl0KV...
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
Inserted: map[id:slrxuYPm_Qm67wR-vbxI6cIOMLMHMTyacDlZPkCE6gQ6snuxixNJPlgnYQZi7mik5TZaPdHcL1G3hheo6t3u9w]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:slrxuYPm_Qm67wR-vbxI6cIOMLMHMTyacDlZPkCE6gQ6snuxixNJPlgnYQZi7mik5TZaPdHcL1G3hheo6t3u9w name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:slrxuYPm_Qm67wR-vbxI6cIOMLMHMTyacDlZPkCE6gQ6snuxixNJPlgnYQZi7mik5TZaPdHcL1G3hheo6t3u9w name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: ptq7x_8Its9FgLFMmNN5NiKPqqEMcmXnmphNoie_5f2V8Sysdby14hE6DL76jMMqrEDKKyr9ky8wYsUhWK_NFw

=== List Collections ===
Total collections: 14
Sample collections: [test_collection chat_messages__ek0_testing test_accounts schema_users_client_py batch_users]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Wt3EBFdJtj848cNl36j5KMadp8GdMx_uX1a8YBXDkwy2OoeRs8nuT5lvbX_9Xxr2Oiglv-UfojiX9byYptgoLA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: rl0McGcpha2Rxutc4E_3b9G3XSnnmMIj_6oDVT09HYCa7Z7iSdOd706pwc9PpjXpATx9fMkFU8XQ94PM_lO2mA

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

=== Inserting Test Data ===
✓ Inserted test record: qDX7006IaMXpT9VeGt-WhYEAD4m8R6-l5kUEVILJ0rbXGZgu0P3zunMoubh7JsAyk-Ch2YQo9Qlpe--FiN5zoA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 9gedxrLJaVItPs1XzPm1TQOvZs6FoqFfjO12PIGEuRrKa8YBZEO5u44TxFtccCvchWZ_OEAzyoNvPa7fjxlnfQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 821H3iSPIbH7vew7_PwAtAC9ISMwvMRCdEty66oC-5tcpXoP4JTJmI0I9vxK_eBCrT28f9HnLWB3bWMIGxHKrg

=== Sending Chat Message ===
Message ID: zhA4msUnMaxIUGA9kPK7ii1BErznR8KcUkIrDBT4_suFPAqaGrgxFaeD6ZW2IKs3daiPeBs5sDCJMS4AbzU8Mg

=== AI Response ===
Based on the provided information, the available products and their prices are:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:zASVYNXNvSrAkMIcEPU-wYat1P6ek7HPyI9xS6xTm9D9EX0_cLGspFNqJ3Wuiu592lOKB_SjyLdUQa5qUztpaw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:z85OoP9kh8wY2Knfj01MxnZ8DLSR3PbXbZXlalq0q32qdVsh-VkpjGhf8KBSblDORNDQAgxchpt0yn051RB2Tg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:m2oM6rxhHgF3I4lbBLgzwpTJoOYG6o453XegVAXIwP9stpJPW8jwYxvYMSZZDTXQvHrqASGBqOzfA2NUM5ac3Q name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2519ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 83
Total tokens: 693

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: odv6MLCuEtIj3NUV-uSW6TTFYvnBemkIBtIuafULJVlO8vS_Zg-LszGuoGQcik32p3_p1cfQ2b2uCpQwXpfVIQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The product available is ekoDB, a high-performance database product. It is priced at 99 units.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fRgne_XfPZ6R8iKg83YNhw3oq1CIqp2VtYwyqs4OU78-BMDtN82sGKcMeeKlSbNCtaqJIcoFa9GBasdBua1qfg
  Parent: odv6MLCuEtIj3NUV-uSW6TTFYvnBemkIBtIuafULJVlO8vS_Zg-LszGuoGQcik32p3_p1cfQ2b2uCpQwXpfVIQ

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: fRgne_XfPZ6R8iKg83YNhw3oq1CIqp2VtYwyqs4OU78-BMDtN82sGKcMeeKlSbNCtaqJIcoFa9GBasdBua1qfg (Untitled)
  Session 2: odv6MLCuEtIj3NUV-uSW6TTFYvnBemkIBtIuafULJVlO8vS_Zg-LszGuoGQcik32p3_p1cfQ2b2uCpQwXpfVIQ (Untitled)
  Session 3: 821H3iSPIbH7vew7_PwAtAC9ISMwvMRCdEty66oC-5tcpXoP4JTJmI0I9vxK_eBCrT28f9HnLWB3bWMIGxHKrg (Untitled)
  Session 4: rPOuQWVT2njcPDiEPMECJJCqn_dUz-waRLpPt-MeiwbHxylvroUkBcOh8x-E9Voy0mCLjYHyGVIQIQGvw2Fp1g (Untitled)
  Session 5: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw (Untitled)
  Session 6: aoe6MEuL3E1EmuO5qnfz0zmj9UCxpmR5TGic-qgUuC5Tr0b6ynRYl1hobYCeqXxu4wWURBibPcRgbY8-SCF3RA (Untitled)
  Session 7: Y9IlcOV2FYlrt19uHBT3nw_RiNqIdVs6MtG_7rOFYFLhEOfn0b8eJ3leVDMrFWKw2U_9gaJhiyPp1eiGoMGI4w (Untitled)
  Session 8: qBqsUD_jL1fpoveeqUPs3_yDNveMsqdq1TamMm0Xps-3pScyHXiEnh5jYsC0QQlaPWGE1YT6iUAx3PH-0JEQng (Untitled)
  Session 9: 9msyvD3h8zah5g8WdK2SPkPlQqqs_rbgazEBNOjmdoMUNl04wtkPKyhQLv3aRA1BGzDbOwuBnOMc5aT88H9h0A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fRgne_XfPZ6R8iKg83YNhw3oq1CIqp2VtYwyqs4OU78-BMDtN82sGKcMeeKlSbNCtaqJIcoFa9GBasdBua1qfg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wpgk9XzUe66icqRb5w-9MVGuzBHnJ1IoL_Ei7ot4YIN6P1PCvM0OvARhJtb7AteUIf2y8MNn7dcWflCckLaJqw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product "ekoDB" is available. It's a high-performance database product.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is 99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: EB4Z9QpAgykYsvdkcb07eqv0-Q3ZRYPpC_ElC-i-1fKANvdGOuiJjA7D77TFhWuN5tRHAGcJcQiYD8Bv8xSudg
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
  1. Score: 0.765
  2. Score: 0.757
  3. Score: 0.744

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.703
  2. Score: 1.506
  3. Score: 0.297

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
  - Laptop ($0) by Alice Johnson
  - Mouse ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: edcIIR4w_Vd0dNefJnmRNF_H9SxJcNk-JYoWAosieM3nBr3fRbgNgGb4_KX4JRMtHV-XnNs-YaDl-tfdm_xuTA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:50 count:5 status:inactive]
   map[avg_score:60 count:5 status:active]
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 14
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
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 886ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 790ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '7BuvcTK6tCPvtvHr8bI5zimkSnKoLY4DBXI_gEOuEMHvtSHu8DOg9dkYRCQWg7T4J4bw2o8TQhmCDShn7mEOEw'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: '7BuvcTK6tCPvtvHr8bI5zimkSnKoLY4DBXI_gEOuEMHvtSHu8DOg9dkYRCQWg7T4J4bw2o8TQhmCDShn7mEOEw',
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  id: '7BuvcTK6tCPvtvHr8bI5zimkSnKoLY4DBXI_gEOuEMHvtSHu8DOg9dkYRCQWg7T4J4bw2o8TQhmCDShn7mEOEw',
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: TMx48sRDwHogTHdds2s9KRtFGqbu921DS6hlVe-0aDVUB8wvRxlSSMgxrUE07uRgh2uYHEnwphhUkvycTaAhPw

=== List Collections ===
Total collections: 18
Sample collections: schema_products_client_go,test_collection,schema_employees_client_go,chat_messages__ek0_testing,client_collection_management_ts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: HOKDoMjn4LYz57b81OmlVsE9Y6UbwG8ZHkXXipTMb5L0mrRpAQRzFmZZVR5SJZiw9MZQQorwLsC0lEdL5tqm5w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 6s_71rrFZsulS2nBA-9TjDXlQDHHwXEoQgJvQkVUyp_TIYNT8x_9YeIiq2Bj4BxybdocMqljOAZACE-_Wy-kgw

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

=== Inserting Test Data ===
✓ Inserted test record: BT3xBqbQITDFtoNoNHUOfLPSIqOIab67k-nyP9aVSY9nmUJZeLBH56M1F6MK4NlCVCBVsrFel3PmWgMhfcVqnA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: mc2q7q-OfmTRCPfpG3eKeyhTLiOMnAjuJIEQNaEXrCU_xVgOBZ_rvZHCwA8dLz0Vsc8q4lWnXHA4YUhxudECCw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: EcfuSEeAPKm9boQ_qCcTXAPrdYySTZP5BFOF-ROwtw2VNENWetCHIf89czWfJfBY7_cu-fYnZvtXxSxp1mI2vw

=== Sending Chat Message ===
Message ID: j1zV82txU5K7vRFkEefuKHV4fOlgBqn_bxcQr1-vOmfO3_bIUJZZPAft5o6nZ2iLoAIZ7UNAwMTJrcj6fn1Y3w

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price is 299 units.
2. ekoDB: This is a high-performance database product with AI capabilities. The price is 99 units.
3. ekoDB Cloud: This is a fully managed cloud database service product. The price is 499 units.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'tssPWTB_S0BSkhAjZkIX2CbH1s2-dpg5FmS-AX8UK49shS5QTxiTIVKCyQz6axS92WTwAhZlG0XIxXF5jDW-ZA'
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
    id: 'p3N6fUUKc8WDIW7kTNX36ZE2Vwo_5kT4sa41OvCpaq58XR6yefiAQI3lh8D3ilExZ-bq1bTZnnp4pOvR-bOgtQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: 'De1cwxeYjbHw-AQ9YD5Piv4xRUz1VIaRCJRFTwPC4tot4LvMgiNqH3q6wrHC5DhxSLekT_wk6g-GkNVqWVb8Mw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2970ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 78
Total tokens: 687

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: f3cqw3oxlUv6OBEomautzumrrpf5HNMYsFmODEtlquVqrBk5NfYi1DVUm8muA_djECAEFz2HZoxC-hI1PMZfBw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is ekoDB. It is described as a high-performance database product. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: EBz573b8e0AAiWzHaAYZZ9PP-L2EUv4ZzS6vCZpGcxuGG182MRu80mJd0FYEj_uRBFthIBNjv3I7zoNOMwW7uA
  Parent: f3cqw3oxlUv6OBEomautzumrrpf5HNMYsFmODEtlquVqrBk5NfYi1DVUm8muA_djECAEFz2HZoxC-hI1PMZfBw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: EBz573b8e0AAiWzHaAYZZ9PP-L2EUv4ZzS6vCZpGcxuGG182MRu80mJd0FYEj_uRBFthIBNjv3I7zoNOMwW7uA (Untitled)
  Session 2: f3cqw3oxlUv6OBEomautzumrrpf5HNMYsFmODEtlquVqrBk5NfYi1DVUm8muA_djECAEFz2HZoxC-hI1PMZfBw (Untitled)
  Session 3: EcfuSEeAPKm9boQ_qCcTXAPrdYySTZP5BFOF-ROwtw2VNENWetCHIf89czWfJfBY7_cu-fYnZvtXxSxp1mI2vw (Untitled)
  Session 4: EB4Z9QpAgykYsvdkcb07eqv0-Q3ZRYPpC_ElC-i-1fKANvdGOuiJjA7D77TFhWuN5tRHAGcJcQiYD8Bv8xSudg (Untitled)
  Session 5: odv6MLCuEtIj3NUV-uSW6TTFYvnBemkIBtIuafULJVlO8vS_Zg-LszGuoGQcik32p3_p1cfQ2b2uCpQwXpfVIQ (Untitled)
  Session 6: 821H3iSPIbH7vew7_PwAtAC9ISMwvMRCdEty66oC-5tcpXoP4JTJmI0I9vxK_eBCrT28f9HnLWB3bWMIGxHKrg (Untitled)
  Session 7: rPOuQWVT2njcPDiEPMECJJCqn_dUz-waRLpPt-MeiwbHxylvroUkBcOh8x-E9Voy0mCLjYHyGVIQIQGvw2Fp1g (Untitled)
  Session 8: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw (Untitled)
  Session 9: aoe6MEuL3E1EmuO5qnfz0zmj9UCxpmR5TGic-qgUuC5Tr0b6ynRYl1hobYCeqXxu4wWURBibPcRgbY8-SCF3RA (Untitled)
  Session 10: Y9IlcOV2FYlrt19uHBT3nw_RiNqIdVs6MtG_7rOFYFLhEOfn0b8eJ3leVDMrFWKw2U_9gaJhiyPp1eiGoMGI4w (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: EBz573b8e0AAiWzHaAYZZ9PP-L2EUv4ZzS6vCZpGcxuGG182MRu80mJd0FYEj_uRBFthIBNjv3I7zoNOMwW7uA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dpeT26YlSy6FwuHGUz5lLHXvESg1rJHejtiGT8QwrLy7ILa5TJbwJFcu6XwpHw7lplMbJoqldD4tmVhVt_THyA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one of the available products is "ekoDB". It is a high-performance database product and is priced at $99.

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
✓ Created second session: mLTVcPRJ94iVpBTu-9gVST_Sjqr_WugLWph97L9WTgQpABTg92TuZLeACZDL7W_NYYNEdcUs7iUNd0qWMTJbaA
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
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: bio.value, title, title.value, bio
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: title, bio, bio.value, title.value
  3. Score: 39.600, Matched: title, bio.value, bio, title.value
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.772, Matched: 
  2. Score: 0.769, Matched: 
  3. Score: 0.760, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.709, Matched: title, content.value, title.value, content
  2. Score: 1.504, Matched: title, content.value, content, title.value
  3. Score: 0.308, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, skills.value, bio

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
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: W3vyzCiZzdC4oyvALJNk7JpDBXlQHI7F_uKW3l7w4JToQWfVevTYu62fu5v__6gGGd_hJOMQzOVFIIMsm8m33Q
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: zPn0dKIkooLp1gy7ko9tX5xVbxjb16hwpzUus9FE1aIVJiDeclkoFSQa-BvollaqgyMF6S_C9rWzwLizv-ctuQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: N39QKyGKRJ6-qvtyXwj0woM17Fxm75MPNXTraD0NH0h5B0MXIACfNmoelYitUROh9LlFZI0bRMamek5CpaC-4g
📊 Statistics: 2 groups
   {"status":"inactive","avg_score":50,"count":5}
   {"avg_score":60,"status":"active","count":5}

📝 Example 4: Script Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: M9uyYUdOAq3xhfxhiTbKjGug9BTY59GPXcculM8UiF6P_2dPSggvt8KJ533MDbUMrUTjrML5ndeZuYmwqNu9kA
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"category":"Furniture","count":2,"avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"category":"Furniture","count":2}
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
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 638ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:3711) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===[0m
✓ Authentication successful

=== Insert Document ===
(node:3714) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'Ymbe2onJ7B0eI4-yfIk1UtJ5mzFW52fQRRwKRzXXXZfEME1Y4awpaIahI5AICAvDrwjj_DwX4bDVRiH7YjeVPA'
}

=== Find by ID ===
Found: {
  id: 'Ymbe2onJ7B0eI4-yfIk1UtJ5mzFW52fQRRwKRzXXXZfEME1Y4awpaIahI5AICAvDrwjj_DwX4bDVRiH7YjeVPA',
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'Ymbe2onJ7B0eI4-yfIk1UtJ5mzFW52fQRRwKRzXXXZfEME1Y4awpaIahI5AICAvDrwjj_DwX4bDVRiH7YjeVPA',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:3742) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: 6OxKBx0mTRZ2QZpJVbpnIRwUxR5C4Eys8c6jqNSnrZ3ZK9VrtocO0pq_yGeFdF6OjcLKIT-8YYS1cph6jD-eLw

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "rncnCNXtn9TifyjIRBuSS9NwEa4LkEujohxC3uUuEvKmgfRNVOk0eFL9shR9g3HdyKsv0S0RKI11q-cTGLTC6Q",
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
        "id": "6OxKBx0mTRZ2QZpJVbpnIRwUxR5C4Eys8c6jqNSnrZ3ZK9VrtocO0pq_yGeFdF6OjcLKIT-8YYS1cph6jD-eLw",
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
        "id": "OzuHNu3HBCQJv4o_AkTX0qjzwIKg-qfalYHGh1k66UfW3VutE3O01Y0n_pg4otwK3O-pyXBVTeGvX6uut0kfyg",
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
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js ===[0m
✓ Authentication successful

=== Batch Insert ===
(node:3745) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js ===[0m
✓ Authentication successful

=== KV Set ===
(node:3758) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  username: 'john_doe',
  loginTime: '2025-12-31T07:14:01.454Z',
  userId: 123
}

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:3766) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: qi5aAZ_sgSRjbl17I8n3s5u0kjm5oJI_21h3yZQuez76OPWNNM-3ckvRdwFki0iBgq8BjcXJjosIEA2SQk-vHA

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_products_client_go',
  'schema_users_client_ts',
  'test_collection',
  'schema_employees_client_ts',
  'schema_employees_client_go'
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
(node:3769) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: QnMykBYxUy8ZQV9zLeMzuQlsqwSkyfWzAB7MkZjbmt1AXNQPUJfSVDWab2rYCbLqW0s76_KygYO2DIiBcBee5w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 4 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:3789) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: qOgPd5JqZ_P-0Z9jxSYssIZfU1_b-Vx2Zp51eO2UBxd8D0H2YfjIJN0aJl3o2xO27YP-n16TN1k0yHeyQA47vg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: zceM0jXyoHHXerSXpKFWE4zKrhyEkL04whK49HUnosGaHk29et7V-dIlPjxbLGVkh8gKQ3iXzziArJNQrotb1w
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: KNs3W8Yn5sLoIHd1f1VmYD7qu8lvxA4zGrMAH1kMHDvOwsRjARsdedLNuTiKma_udRR_EW3hjBRja_GGWthGfA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Z2oVRPhugcAf9eTXD-CJ2HymTicpiOrC_CVJQ5bLv-Lh4QNScu4AlNFFYpPNeyyT0U3B9MhLm7xcWvbUnzHPRg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":60,"status":"active","max_score":100}
   {"max_score":90,"status":"inactive","avg_score":50,"count":10}

📝 Example 4: Function Management

📋 Total scripts: 20
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: xNPfd30pxmYE-c-7yGUckTGsWh2cWfLJhmJHxscB6KW_C9v30sVX0rPOUUNza00x2nS2-xZnFxpzvaaiFs3MXQ
Created Bob: $500 - ID: q_Wh87SlJJcNrMToa7Gd5UgMvq8xWQB-nQEWVcSve6IeI3DHjNYY77i10BAhALZhAUHo9U4duJHtxZbKtMhUCg

=== Example 1: Begin Transaction ===
Transaction ID: f0ce5e24-f9ab-4711-b008-690c129717ba

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: 81a43c18-4880-4ddd-b62b-fe5c4ced87e7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

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
   ✅ Script saved: awh6cV0Qi8ltOKyLOPOpsnLUeXPzRvdLFL3V9PqU7FYxPu2od6yw67saaoBs_EbWuEmUsUaqUbsVFNP7LLEtig

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
   ✅ Script saved: sh6cF1JQbFTZEyyH5VaxB9Lvww2ZtQqC263CRusF19a1itCFMrA_-ocD1JAhGO-iLQAP_R5X8uJWhcRnrfvbKg

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
   ✅ Script saved: dcFS6Txfrc6peV10GCC99DKB03ms8l0QCMKDTgVNnly1oj7BTO0ei5rj6tgntlxvBA3WUI7U1RFxhZ5Y6cFOAA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: q4Pm3dCieYj4HBIDOQwESOm1BxoT8gccoonFqhJehvhfmtb_FD8Nd0H_iEMyA3DzrbMKIez5jlulUX3u8z7yBQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: awh6cV0Qi8ltOKyLOPOp...
   ✅ Deleted script: sh6cF1JQbFTZEyyH5Vax...
   ✅ Deleted script: dcFS6Txfrc6peV10GCC9...
   ✅ Deleted script: q4Pm3dCieYj4HBIDOQwE...
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
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 823ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 633ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '9KejwXA3ApHmN871Af1CCUW6YKe_xFDeITDv9jrRGRaR6ZKuN39vsE1H3cvJAlLiowF6YpJ26kCOftKZtWNq5Q'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: '9KejwXA3ApHmN871Af1CCUW6YKe_xFDeITDv9jrRGRaR6ZKuN39vsE1H3cvJAlLiowF6YpJ26kCOftKZtWNq5Q',
  name: { value: 'Test Record', type: 'String' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  id: '9KejwXA3ApHmN871Af1CCUW6YKe_xFDeITDv9jrRGRaR6ZKuN39vsE1H3cvJAlLiowF6YpJ26kCOftKZtWNq5Q'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

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
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: b7_YVVqCvFgGoupxWv14E4F-RTSaXv8P0KGdlSUn9rjjNLQSEdRuG3qNgQNQ7wAx6_nh_u8VHPmNEJqYUdIQtQ

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_go,schema_users_client_ts,test_collection,schema_employees_client_ts,schema_employees_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 0wb_GNAyAB2vJBNjxTLu9zZnM4kg4EaxZdMtAPl6Ej1yk1Xli3AxJMI-iUfXNaBUgqTuCWwLKUY7FeX4tjlVjQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: XqXIuRhDH2Ddv3Psb4rylsEaNffY1tDnC-vT1sNQLdYUlexd8Dgck-yNZdWWeabeJXvT3vHGy5odWDcWKLBezQ

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

=== Inserting Test Data ===
✓ Inserted test record: gc7oF1UPGH_FpY1MruEjccHPylfphX0rwYSRUKXzjFQ_LoB4SDmCzMlBmoF4KJ6FrAzwQooWGgYRF9g_3ppOTg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: fEw09gynuiR7dHZ9AI3yIBvtS8OrvGWKMkGQ6xTNu38DgdLb6LCK_UTlpi1OEnEeFYPYeJroRATbLUrG38sVrw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: _mcO7uO1anLmcPSY28CL_fomR0ke5fpwMLWRtAnEhNY8iBp6BB5eSN4MNMZZRlP1DrXJknhQdWt8Wwjx02qmEg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: kvwNmiCxlYTLlvFaitiU9_fREsAMB3TaZ719LwxzC_PjVVu2WQtpJBTD4aNvupEOGAiHyWGMaBRUMh6oV52YNQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: gNiEzCrTd0RMwT1556ZYonnEbslkM5Gvktcy39yhIcN-OE7X6yhXOKijoT4CafZbhJdecFN4iNVjlLWZm8_f2w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":90,"count":5,"avg_score":50,"status":"inactive"}
   {"count":5,"avg_score":60,"status":"active","max_score":100}

📝 Example 4: Function Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: WrFAnddTIJ8GPKWbEphFwAKINwKhjKVQycLzkN_vXLZRj5pxMHTQicHcpc34qDKfc_OmLu7_weJ7U51CQ-i6TQ
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":60,"status":"active","count":10}
   {"avg_score":50,"status":"inactive","count":10}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 24
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
📊 Total user count: 20
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
   Furniture: 4 items (avg $294.00)
   Electronics: 6 items (avg $325.67)
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
   1. Monitor 27" - $399 (⭐4.6)
   2. Standing Desk - $599 (⭐4.7)
   3. Laptop Pro - $1299 (⭐4.8)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
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
📊 Users (showing first 5 of 0):
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive undefineds: 3 users, Total Score: 180
   active undefineds: 7 users, Total Score: 370
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
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Natural Language Processing (AI)
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
   3. Database Design Principles
   4. Getting Started with ekoDB
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Database Design Principles
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
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. High Precision: Vector data is represented as points, lines and polygons, enabling high precision and accurate representation of spatial data.

2. Scalability: Vector data can be easily scaled up or down without any loss in resolution or detail.

3. Detailed Analysis: Allows for complex and detailed spatial analysis such as network analysis, proximity analysis, and topological analysis.

4. Less Disk Space: Vector data usually requires less disk space than raster data.

5. Clear Boundaries: It provides clear boundaries for features like roads, buildings, and land use parcels. 

6. Attribute Storage: Each entity in vector data can hold multiple attributes, allowing for a rich dataset.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Draft Article" - 1536D vector
   3. "Advanced Query Patterns" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
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
> Task :compileTestKotlin NO-SOURCE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test NO-SOURCE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s
2 actionable tasks: 2 up-to-date
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
✓ Inserted user: Record(fields={id=StringValue(value=aGQP9H7L8R_0I6UCxrh0FpFTwcXK_VQ2KUBmixnvNCxVff8Yfy7aGL3NowH0r7VDDbQ929TUjOzaY_NiHUHrCw)})
  User ID: aGQP9H7L8R_0I6UCxrh0FpFTwcXK_VQ2KUBmixnvNCxVff8Yfy7aGL3NowH0r7VDDbQ929TUjOzaY_NiHUHrCw

=== Read ===
✓ Found user by ID: Record(fields={age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), id=StringValue(value=aGQP9H7L8R_0I6UCxrh0FpFTwcXK_VQ2KUBmixnvNCxVff8Yfy7aGL3NowH0r7VDDbQ929TUjOzaY_NiHUHrCw), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})

=== Update ===
✓ Updated user: Record(fields={id=StringValue(value=aGQP9H7L8R_0I6UCxrh0FpFTwcXK_VQ2KUBmixnvNCxVff8Yfy7aGL3NowH0r7VDDbQ929TUjOzaY_NiHUHrCw), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), id=StringValue(value=aGQP9H7L8R_0I6UCxrh0FpFTwcXK_VQ2KUBmixnvNCxVff8Yfy7aGL3NowH0r7VDDbQ929TUjOzaY_NiHUHrCw)})

=== Delete ===
✓ Deleted user with ID: aGQP9H7L8R_0I6UCxrh0FpFTwcXK_VQ2KUBmixnvNCxVff8Yfy7aGL3NowH0r7VDDbQ929TUjOzaY_NiHUHrCw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

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
  IDs: W1S3n2sIwowbRfzLl3TDDDS2d6JTThKUBR6210igveZJo-Wy0_q3u8TcpA4GXVdpRTqDTFTn86EU8HRfgNriSA, y8lBmN9x9MjFfZH9WGmT4TnVdpJAGqtso-XZHPdo2uzH-tDZTN5dRHUBwCwi6J2v3o8MbD0pFrti792brZsLeQ, 46lWC6jDyqBmAfHeTUpscQ8WbQTdp1N3ENEf9OYGf6ilQrsBXPSbYUFBb-Ix0ET8orqzwR5Zo9pbk-tr7JMWWw...

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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1767165288687,"user_id":"123"}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Confirmed key was deleted

=== Cleanup ===
✓ Cleaned up test keys

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
✓ Found 21 collections
  - schema_products_client_go
  - schema_users_client_ts
  - test_collection
  - schema_employees_client_ts
  - schema_employees_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-31T07:14:53.038204Z","last_modified":"2025-12-31T07:14:53.038204Z","bypass_ripple":false,"primary_key_alias":"id"}

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: aUDCqi8g-MIiRhpdi190FPNx4qJG1jQS0H2526VejgMPoFXjSfomJTmSa2hlR9XHFVU-S62vcnvuo-jxZqQQAQ

=== Verify Document Exists ===
✓ Document found: [created_at, ttl_duration, id, session_id, user_id]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: a9i8NWwWb832ftXMuAjrU-JQSBHHue_o4ds3MfkkXovpO_HmTFvhgoUG-lGWbmPg7lVloe2dCspCuXqp7Jyx6Q

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [value, id, ttl_duration, cache_key]

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"TfRxNyzoB9BpE9OMrmWLnwjzvNBEyiqGx0Iw1XSDr26bi8-cD6KOQ98Ttjuw-u5yP-zeeZn6fDZ0f4U0vFoCmw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=DRgzLB4oi0_oB16iZXXH-2V2MvMqvamxsY3koWG2kjYyoFWTc4HeHBud4FbBEQfosM4v1w7457LRFqdX_Slhkg)
✓ Inserted user 2: StringValue(value=E8yqArAZUsclfYEX0RYzdcjG0ttlM7FW4vuayrKuCURa8jmc4n_BxGR2a96ne0XuwCBbjsH5p5enrZu-BZNEDQ)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_products_client_go, schema_users_client_ts, test_collection, schema_employees_client_ts, kotlin_schema_example]

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"id":"cQ5g4WiFazQBwTaGb8xO6nuFfLA0dqAo02KETx2tzi30yGv9XGCBELe5itFcm1OquqPkWg5StjaXOAa29eKxXQ","tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","views":148},"score":19.8,"matched_fields":["tags","description","title"]},{"record":{"views":349,"title":"Python for Data Science","tags":"programming,python,data-science","id":"YGmrOhZEdBJtoSMWc8hf4u2FUPEVnHvoNV9gnBR3z6nh6nOqO6cI4sFVyBCEk0THnOzB8TI7frIZx-yPgArlpA","description":"Master Python for data analysis, machine learning, and visualization."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":"JavaScript Web Development","id":"frFSTH7WG2JjKWlhFiZIVEMytLLod1qU_viXbV11v0AnG-9MHKS2l6kdAll6BO9sJ8zJo9EtzpxBwp-JPPDZpQ","description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web","views":196},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":"Machine Learning Basics","id":"VVqlyVkhhgtsxPJE7uX9nsoG7a4Zd1OlzvzzoWa9MnojW7WDxNpWfW9oNazobMynLhX2Y7a5MZneRArIgevOZw","description":"Introduction to machine learning algorithms and neural networks.","views":717,"tags":"ai,machine-learning,python"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"description":"Master Python for data analysis, machine learning, and visualization.","id":"YGmrOhZEdBJtoSMWc8hf4u2FUPEVnHvoNV9gnBR3z6nh6nOqO6cI4sFVyBCEk0THnOzB8TI7frIZx-yPgArlpA","title":"Python for Data Science","views":349,"tags":"programming,python,data-science"},"score":1.0,"matched_fields":["description"]},{"record":{"views":323,"tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization.","title":"Database Design","id":"6na2rBdx1EvBibltyNKDmiw8ukbQiBcCAn7xe8mNGvUNqg5ShWWKQyQFn70XLz32iO7T7GJlVqtmXW-h8to7mA"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","views":148,"id":"cQ5g4WiFazQBwTaGb8xO6nuFfLA0dqAo02KETx2tzi30yGv9XGCBELe5itFcm1OquqPkWg5StjaXOAa29eKxXQ","tags":"programming,rust,tutorial"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=E7Ylw-09bKYrI6niza2blOBDVscd8zGk43WBr-zliZV4CPWHdqQ3C1jCKgCJ8cykSnM3eanx3rt9Myj8uu_HdA)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767165314910},"id":"E7Ylw-09bKYrI6niza2blOBDVscd8zGk43WBr-zliZV4CPWHdqQ3C1jCKgCJ8cykSnM3eanx3rt9Myj8uu_HdA","name":{"type":"String","value":"WebSocket TTL Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

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
=== ekoDB Kotlin Client - Join Operations Example ===

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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 1wdyEYaw6q_0I08xfy7KG6b2eBgN-asKZI-IYnIIQWQrzzjN7JF8HMtH9TtBIdn_bvBJJpIW3M432XmOW2gaVQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "brfNdpr6m4JfzIaAUxgUwl55mVn5iWaRWQjtDh12qGC-gITShAwW5kREYHNGut9QHhXuuV3YkRpjYPD8p1sM-w"
  Responses: ["ekoDB is a high-performance database that includes intelligent caching, real-time capabilities, and AI integration. Some of its key features include an AI Chat Integration, which allows you to query your database using natural language and get AI-powered responses with relevant context. It also supports full-text search, vector search, and hybrid search with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: oV4Y9eFK-TlSspXFsI3Qn8bW4iu8B2cMPVPdnkTyMuLX8wQmf_02RzgLVyn1NGRBB3O00t8e06D8-MQck4iB9Q

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the available product is ekoDB. It is a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"oV4Y9eFK-TlSspXFsI3Qn8bW4iu8B2cMPVPdnkTyMuLX8wQmf_02RzgLVyn1NGRBB3O00t8e06D8-MQck4iB9Q"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"wq5U2iWlZszm1u1M33_sbZSmAgbjG2runEneR1i1h0JS9woSWCQlHEQu9uetXiFK4V9zyoiZNzLwyMsMt1EukQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-31T07:15:31.900946Z"},"id":"178Vl8FyP4CFrXPSpcPzv0J-LWaLqvX859ypaIoVkdxP7Bl4Md62sQg_HAx-dR488U16i8rvu55bAtUqqoC0xg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":34,"prompt_tokens":238,"total_tokens":272}},"updated_at":{"type":"DateTime","value":"2025-12-31T07:15:31.900946Z"}},{"chat_id":{"type":"String","value":"oV4Y9eFK-TlSspXFsI3Qn8bW4iu8B2cMPVPdnkTyMuLX8wQmf_02RzgLVyn1NGRBB3O00t8e06D8-MQck4iB9Q"},"content":{"type":"String","value":"Based on the information provided, the available product is ekoDB. It is a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"wq5U2iWlZszm1u1M33_sbZSmAgbjG2runEneR1i1h0JS9woSWCQlHEQu9uetXiFK4V9zyoiZNzLwyMsMt1EukQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-31T07:15:31.908173Z"},"id":"t016XAPdq8qcIh3tzHj1E0Rjxq8swYgG6N6bGmltyPRHxozqfskihgHIDZ3RTTnHDZ0oN6NfY9vqed_mcyoMyg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":34,"prompt_tokens":238,"total_tokens":272}},"updated_at":{"type":"DateTime","value":"2025-12-31T07:15:31.908173Z"}},{"chat_id":{"type":"String","value":"oV4Y9eFK-TlSspXFsI3Qn8bW4iu8B2cMPVPdnkTyMuLX8wQmf_02RzgLVyn1NGRBB3O00t8e06D8-MQck4iB9Q"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"wq5U2iWlZszm1u1M33_sbZSmAgbjG2runEneR1i1h0JS9woSWCQlHEQu9uetXiFK4V9zyoiZNzLwyMsMt1EukQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-31T07:15:32.767601Z"},"id":"t8mAOu5wuuluF_Uo9RPwOzy3Ep3ernMRCFFYXg3LEiJTCOQEaU2wSr7aRInGMRuWXdNFatbN148mjek4Hnzavg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":238,"total_tokens":250}},"updated_at":{"type":"DateTime","value":"2025-12-31T07:15:32.767601Z"}},{"chat_id":{"type":"String","value":"oV4Y9eFK-TlSspXFsI3Qn8bW4iu8B2cMPVPdnkTyMuLX8wQmf_02RzgLVyn1NGRBB3O00t8e06D8-MQck4iB9Q"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"wq5U2iWlZszm1u1M33_sbZSmAgbjG2runEneR1i1h0JS9woSWCQlHEQu9uetXiFK4V9zyoiZNzLwyMsMt1EukQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-31T07:15:32.773624Z"},"id":"pS2o1CEd5miW4PdjVrHK2431D9Zm6b_X7SwpIEDpNb1rdyB0jwL-thc1699Ki9IWLriRLGEXxtOQmvD2Xrmnug","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":238,"total_tokens":250}},"updated_at":{"type":"DateTime","value":"2025-12-31T07:15:32.773624Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: WcfuHjuzNMdJoIlSMv_ogAFfvKNzAwYCOo5BdYIL0Elv__TOUZdX7eSM88nsA9ghAxCsKiXV3xRzgP0pNZjGJg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BkJdQejjLYKDMPvuSzxlLHvJUFFRSike1pDAf_4wmznKDBf0ka73PqQAC7TzRUljVPkZ_tiMSEX1xAMGq1Z-UA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is ekoDB. It is a high-performance database product and it costs $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The product available is called ekoDB. It's a high-performance database product. The price for this product is $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: vKbxgzQCadPdIuCgoWTS_BiqXnjL4lrKHIWV1v5XV5GLvnGdBplVDlAK5Zb5Aeva2lv6e6Yuvu3b4dAau0RtgA

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

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
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: GFPgnRfHvu_Y_hR01pPAOMySQVV378tSWZ66ywG_i7PWfetKrurMQOvqehHze_s1AiStvYDiAUHg7LM6ZBxZUw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":60.0,"status":"active"}
   {"status":"inactive","count":5,"avg_score":50.0}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Script listing skipped (some scripts contain parameter placeholders)
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
⚠️  Script cleanup skipped (some scripts contain parameter placeholders)

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
/bin/sh: kotlinc: command not found
Kotlin example (requires kotlinc installed)
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
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 675.251959ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.20389075s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 389.406792ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.121243125s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 231.574208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.175061458s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.229006167s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 755.930459ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 293.11475ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 460.628875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 602.317584ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 439.726542ms
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
    ✓ Generated embedding: 1536 dimensions in 279.793125ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 107.725959ms

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

Writing memory-safe high-performance database code involves several best practices and techniques. Here are some tips:

1. Query Optimization: Write efficient queries. This is the most effective way to improve performance in your database code. You can optimize your database queries by using indexing, minimizing the use of joins where possible, avoiding wildcard characters at the beginning of LIKE patterns, and using LIMIT to sample query results.

2. Connection Management: Make sure to properly manage your database connections. Keep them open only as long as necessary and close them as soon as you're done with them. This will prevent your application from consuming too much memory and slowing down.

3. Use Prepared Statements: Prepared statements can help optimize your database code by reducing the parsing time of SQL statements. They also help prevent SQL injection attacks, thereby increasing the security of your application.

4. Normalization: As mentioned in context 3, database normalization can help improve data integrity and reduce redundancy. This process involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

5. Choose the Right Database Type: Depending on your use case, you may want to choose between SQL and NoSQL databases. As stated in Contexts 1 and 4, NoSQL is good for flexible schemas, horizontal scaling, high write throughput, and working with unstructured data, while SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

6. Use Efficient Data Types: Use the most appropriate data types for your columns. More efficient data types take up less space in memory and improve the speed of your queries.

7. Batch Operations: Instead of inserting or updating records one by one, group similar operations together in batches.

8. Error Handling: Implement robust error handling to ensure that your application can gracefully handle any issues that arise and recover from failures. This includes checking return values and error codes, and using try/catch blocks in your code.

Remember, every application is unique and some of these tips may not apply to your specific situation. Always profile and test your code to find out where the bottlenecks are and where you can make improvements.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 314.190709ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2328 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 332.032917ms
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
  ✓ Text search completed in 67.143ms

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
    ✓ Generated embedding: 1536 dimensions in 0.347s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.236s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
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
    ✓ Generated embedding: 1536 dimensions in 0.498s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.402s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.130s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.242s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.281s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.204s
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
    ✓ Generated embedding: 1536 dimensions in 0.451s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.085s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe high-performance database code involves several aspects, including database design, query optimization, and the use of appropriate database management systems. Here are some general tips:

1. **Database Design:** As mentioned in Context 3, normalization is a crucial part of database design. It helps to reduce redundancy and improve data integrity. This can have a significant effect on the performance of your database.

2. **Choosing the Right DBMS:** As discussed in Contexts 1 and 4, the choice between SQL and NoSQL can affect the performance of your application. SQL is a good choice when dealing with structured data and complex queries, while NoSQL can provide high write throughput and horizontal scaling. Choose the one that best fits your requirements.

3. **Query Optimization:** As per Context 5, query optimization is an essential part of improving database performance. This can involve using indexes, avoiding full table scans, and reducing the amount of data that needs to be processed by the database.

4. **Memory Management:** To write memory-safe code, it's important to manage resources properly. This includes closing database connections when they are no longer needed, and correctly handling exceptions to avoid memory leaks. In some languages, it may also involve manually freeing memory that is no longer needed.

5. **Concurrency Control:** Efficient handling of concurrent transactions can significantly improve the performance of your database. This involves using appropriate isolation levels and locking mechanisms to prevent conflicts and ensure data integrity.

6. **Caching:** Using caching can dramatically improve the performance of your application by storing frequently accessed data in memory. This reduces the need to access the database and hence reduces the load on your database server.

7. **Batch Processing:** Rather than executing individual queries for each operation, you can group similar operations together into a batch. This reduces the number of database round-trips, which can significantly improve performance.

Remember, the best practices might vary depending on the specific database system and the programming language you are using. Always refer to the documentation for specific guidelines.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.898s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2274 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.421s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe high-performance database code involves several aspects, including database design, query optimization, and the use of appropriate database management systems. Here are some general tips:

1. **Database Design:** As mentioned in Context 3, normalization is a crucial part of database design. It helps to reduce redundancy and improve data integrity. This can have a significant effect on the performance of your database.

2. **Choosing the Right DBMS:** As discussed in Contexts 1 and 4, the choice between SQL and NoSQL can affect the performance of your application. SQL is a good choice when dealing with structured data and complex queries, while NoSQL can provide high write throughput and horizontal scaling. Choose the one that best fits your requirements.

3. **Query Optimization:** As per Context 5, query optimization is an essential part of improving database performance. This can involve using indexes, avoiding full table scans, and reducing the amount of data that needs to be processed by the database.

4. **Memory Management:** To write memory-safe code, it's important to manage resources properly. This includes closing database connections when they are no longer needed, and correctly handling exceptions to avoid memory leaks. In some languages, it may also involve manually freeing memory that is no longer needed.

5. **Concurrency Control:** Efficient handling of concurrent transactions can significantly improve the performance of your database. This involves using appropriate isolation levels and locking mechanisms to prevent conflicts and ensure data integrity.

6. **Caching:** Using caching can dramatically improve the performance of your application by storing frequently accessed data in memory. This reduces the need to access the database and hence reduces the load on your database server.

7. **Batch Processing:** Rather than executing individual queries for each operation, you can group similar operations together into a batch. This reduces the number of database round-trips, which can significantly improve performance.

Remember, the best practices might vary depending on the specific database system and the programming language you are using. Always refer to the documentation for specific guidelines.

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
    ✓ Generated embedding: 1536 dimensions in 0.295s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.435s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.307s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.397s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.294s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.433s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.366s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.431s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
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
    ✓ Generated embedding: 1536 dimensions in 0.463s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.077s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     What is database normalization?

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several principles and strategies:

1. **Normalization**: As mentioned in Context 1, database normalization can help reduce redundancy and improve data integrity. This can improve performance by reducing the amount of data that needs to be read from disk.

2. **Appropriate Database Choice**: Depending on the type of data and the operations you need to perform, SQL or NoSQL may be a better fit. For example, according to Context 2, NoSQL can provide high write throughput and is good for unstructured data, while SQL is better for complex queries and structured data.

3. **Indexing**: Create indexes on the columns that you frequently query. Indexes can greatly speed up data retrieval but remember that they also slow down write operations because the index also needs to be updated.

4. **Optimize Queries**: As asked in Context 5, optimizing database queries is crucial for high performance. Avoid using SELECT * queries, instead, specify the exact fields you need. Also, try to avoid complex joins and nested queries where possible.

5. **Use Prepared Statements or Parameterized Queries**: These not only help prevent SQL injection attacks but also can improve performance by allowing the database to efficiently cache and reuse query execution plans.

6. **Connection Pooling**: Reusing database connections rather than establishing a new connection for every operation can save a significant amount of resources.

7. **Paging Large Results**: When dealing with large amounts of data, retrieve and process them in smaller chunks to avoid using too much memory at once.

8. **Caching**: Cache the results of expensive or frequently accessed queries to reduce the load on the database.

9. **Concurrency Control**: Use appropriate locking mechanisms or transaction isolation levels to prevent data inconsistencies and conflicts while allowing for as much parallelism as possible.

10. **Memory Management**: Be mindful of how your database code uses memory. Avoid loading large amounts of data into memory at once. Always ensure to close connections, result sets, and other database resources after use to free up memory.

Remember that performance tuning can be a complex task that often involves trade-offs. Monitor your application's performance, identify bottlenecks, and iteratively refine your approach.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2377 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.455s
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
  ✓ Text search completed in 0.056s
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
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.831s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.255s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.453s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.305s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.229s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.218s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.315s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
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
    ✓ Generated embedding: 1536 dimensions in 0.248s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.098s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of good practices, optimization techniques, and the appropriate choice of technology. Here are some steps you can follow:

1. **Choose the right database system**: Depending on your use case, you may benefit more from SQL or NoSQL databases (Context 3 and 4). SQL databases are better for complex queries and structured data with well-defined relationships, while NoSQL databases are great for unstructured data, flexible schemas, and high write throughput.

2. **Normalize your data**: Normalization helps to reduce data redundancy and improve data integrity by dividing large tables into smaller ones and defining relationships between them using foreign keys (Context 1). This can simplify the writing of queries and reduce the amount of data that needs to be loaded into memory.

3. **Optimize your queries**: Efficient queries can significantly improve the performance of your database code. This can involve techniques like indexing, using the correct datatypes, limiting the results returned by your queries, avoiding nested queries, and making use of query planner tools to analyze and improve your SQL queries (Context 5).

4. **Manage memory usage**: Look for ways to reduce memory usage in your code. This could involve using appropriate data structures, avoiding unnecessary duplication of data, and disposing of objects that are no longer needed.

5. **Implement caching**: Caching can significantly improve the performance of your application by storing frequently accessed data in memory. However, it's important to manage your cache carefully to avoid memory leaks.

6. **Use connection pooling**: Connection pooling can help to manage resources, improve performance, and ensure memory safety. It allows you to reuse database connections rather than creating a new one every time a client makes a request.

7. **Handle exceptions properly**: Exception handling can help to ensure the integrity of your data and prevent memory leaks. If an error occurs, you should roll back any changes that were made during the current transaction and release any resources that were acquired.

8. **Test your code**: Regular testing can help to identify any issues with memory usage or performance. Tools like profilers and load testers can be very useful.

Remember that writing memory-safe, high-performance database code often involves a balance between optimizing for speed and maintaining data integrity. It's important to understand the trade-offs involved and make decisions based on the specific requirements of your project.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.376s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2609 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.423s
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
  ✓ Text search completed in 0.060s

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
    ✓ Generated embedding: 1536 dimensions in 0.324s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.23s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.239s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.324s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.281s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.341s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.31s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.264s
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
    ✓ Generated embedding: 1536 dimensions in 0.401s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.124s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
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
  ✓ Text search completed in 0.056s

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

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 803ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (FSVEwzqehJHn6Fr17Abyuw365jRwWphouCPjRrcP6q2mAHY3RCaaAlSDd7yGhTkIrH2k57-nhATf3_B6xKc-8Q)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269031,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
      "cached_at": "2025-12-31T07:17:22.150Z",
      "id": "uNaUI-AJcVOXs_OWlF8n0zmf8g4tAg32wNf1j_E9_UvqDU9TAe8ObqeD1ZbgMJrm_86Hx88tyPriNUoF1AD7zw"
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
Response time: 75ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (4yfcug4ZQzJ5RONZLmkMWjWBfVSb7sRTRpKfSwdwu8lhgGBzkyICF8WWNacu3s7F_34JcrcMFVWnVDZhK1xwMw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "id": "ZSVhMKB4IxM3363wvoxwG9yRzm6_SYJDbAIBxsRdMvvpBIHT3oaL7cV933hq3yNUbl5fEXhr63944SEjDufTLw",
      "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
      "enriched_at": "2025-12-31T07:17:22.394Z"
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
✓ Edge cache script created: Jg5C0B1-K1RvP3LGCABA6QzdaEp1UPoewsDYuX4pnHsIE5hOTY5Z12cmyPwz0ttUmDUkUb-6x8qbsZI1ZF3XOA

Call 1: Cache miss (fetches from API)
Response time: 564ms
Result: {
  "records": [
    {
      "cached_at": "2025-12-31T07:17:22.853Z",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.04208087921142578,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2025-12-31T07:15\",\"interval\":900,\"temperature_2m\":-3.7}}",
      "id": "T0dAp01SQndrjhLI-9Hb8MTn5Kk9158ffRRfJhb_GmL71SvqbG5w_FNChLf2GDRBBarmXpre68fTjz_byjWxTw"
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
Response time: 553ms (1x faster!)
Result: {
  "records": [
    {
      "cached_at": "2025-12-31T07:17:22.853Z",
      "id": "QSIK2jI4OBNlF-gGPf6t73LrouHKnQrZ0Sbwm8hC8y0C3StV5ba2L-gih1luMLhAamESxY9elFZ269xoxJEf1A",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.022292137145996094,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2025-12-31T07:15\",\"interval\":900,\"temperature_2m\":-3.7}}"
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
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.5.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (vZUa4fEKpCSwT1ZXI8k7QYOjCEPCiwoowr-EpuW3vtAHCsaYd_iy1c0IoA3tVB2kSbi1zY24mKgw9FKVeLY9Lw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 97ms
Result: [
  {
    "cached_at": "1767165445",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269031,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "ozcSMIhbihHrT4C7ICGv2neGSq42CjP_7cAn4A7fFI72dqWRWHjxnoA23qjooJGaZr-dXgCH38xpN1UHSIr-VQ"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 109ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (FQZKMwNe-6YaqoN9HVNF77S2j2LnghrhlV4fXB6f9_EIM2ZcWzkgqJujFA-dCMHdPeN9q6dMGuSkIGoCShchKA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767165445",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "MPrP1N0EC5Ni_oi51VBQFxcrtpdvQMp9qZUcheApd_FZWrDWk9O7vvPYL7Jp-1DfN9rMwmYDozcDgI78rIjGYQ"
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
✓ Created SWR script: fetch_github_user (nXEO_atl3yvP4xlbqyGINBZZzL3J7EIV5ueeIHjivIFBc9R3nNzTPVAJ1B_aTPqgj1wqUVuDiaeLIkFiknt4tg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 84.120667ms
Result: [
  {
    "cached_at": "2025-12-31T02:17:26-05:00",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269031,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "ZAzaLicoUIN0PNQ4SH8yFciCywOzrqTwXtGTHV2l0X7tnSpQo4sa7TLB8LY4YzUViZhEbdwgqRcvWZvxiMJliQ"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 78.566084ms (1.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (jiujbNmRMMQ1LyOrjuvkbsCJySqcSbUDAWIMtne90gW-EH01hZ-raa-2z7xwsyIk60t2ThIVvLSu3SlxZFyPEA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767165445",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "Xte2fK_NxNFWQqx5bL7kgbKwHvhkcSiqbW8SMMc-Vb_JP_zbgOzLGpVm6oKupmIOdM3FLqQw-2BMttH7zkNy9A"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (gdc83LSnLMIODgPHd-BczasADMBOdGZG2pqIKxy_OrGVJzMw1PuNXyW8gZT-nRkeZCDsHOwRZi3c7d9PvzAqhg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 96ms
Result: {
  "cached_at": "2025-12-31T02:17:26-05:00",
  "id": "9EeaXEo4OtOWdinmb1QfRkS-sc19H8zBxcggVmidJmG4KOE_rmPRGYIXqmrwL8T2ypwE5LUj4kgGQ1Oji85yhA",
  "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269031,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 105ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (7ChA7eI21b_sCCMn_1JfF6qrC-lrcWtK2CcpTaMiP9ob1ur896uUXCTqM0caIDY3qMN8ZIoSWQUieqaMPmHHrg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_at": "1767165445",
  "id": "j9_OB047z7kK9QlX0euZxTphCPdbqPRuHzdVPkabmf01-7SVOAnNQzPoLQO905RSp8JScFO-t7dISMkMSEkSGA",
  "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}"
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
✓ Created SWR script: fetch_github_user (y5cCaiy9rmVMyLOVDlfBK4rXZs1OORHxbHs66AJtdQ30ybsIP11egSX4iXskIKGnSt6evFamxs6YLOaEtANEBg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 88ms
Result: [{"id":"YlyUv03zs32yFdgY4hw7UeznZ-BRdcGxODK0oRcQHZukkBGqPGm499bP4CHxgDAanzH_LGCEB5Lly95E2vhVCg","data":"{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269031,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}","cached_at":"2025-12-31T02:17:26-05:00"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 80ms
Cache hit was 1.1x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (pLJo63pwQnv7TJ0SubFGL_TOvPQalcsig52R_D_rkjLOJvYld8E-GLR-Klgp00hs9lGnlah8TVrj7UeeebY-BA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":"{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}","enriched_at":"1767165445","id":"gvIBFru6_CuMvzmkzKAGi5qi9GMI8Wp37XMJoO2eZCQ7kEKrk1dNbIb311dUyxwn5oTw29s8q5HregOusR4PTA"}]
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

