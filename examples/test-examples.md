make test-examples
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("BE5LW4xt_MY6ttSQq8O3bIZXLu7YethdCiQe8aYIKVO86GvU0lnwagTMGU0GlhWjeV9wzLGf1PFExBp-BoIfYQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("BE5LW4xt_MY6ttSQq8O3bIZXLu7YethdCiQe8aYIKVO86GvU0lnwagTMGU0GlhWjeV9wzLGf1PFExBp-BoIfYQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("BE5LW4xt_MY6ttSQq8O3bIZXLu7YethdCiQe8aYIKVO86GvU0lnwagTMGU0GlhWjeV9wzLGf1PFExBp-BoIfYQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("BE5LW4xt_MY6ttSQq8O3bIZXLu7YethdCiQe8aYIKVO86GvU0lnwagTMGU0GlhWjeV9wzLGf1PFExBp-BoIfYQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ"

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
        "id": "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "mELODJy5QM0Sgl3-cO9IeIx4ZTjnUMQJOnevW4UbwZRMvExwmcljGUErwq6SYQDNJ-9V0hc_CojEmxXpZ7mxOg"

=== List Collections ===
Total collections: 4
Sample collections: ["websocket_test", "test_collection", "demo_collection", "batch_users"]

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
✓ Inserted document: "uZSslftNADdNNPoV5gRCAANvYhZyJsIxGIF4iMHwc0Vf-fq_lUyG45gdoFpO3rXBO3n5XmtfQcdqNVCk9mrf9A"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: null

=== Query Documents ===
✓ Found 1 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("Lr21rQWHKxrXXDtLD14AyTvjrOmIdxXcYz9tOl-dUbvszElS1Eyr_bWrPMEj6nbCwMfaQF6---0DW_ksu1fNvQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: zzyOwGG1Xpe7TT3iG6j1QSlIb_NLKzvKcMSb7uc6JFE0nsst79n99zyAel_ViVRmdBqa_c3ntD5TWxzysi-3nQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "bHJ3psbUHPc5oOj8Qo4oG--ORTbNifND5ph8xDHI9rA2yLQS-d40QHa9RUmMtC7-pC7Xr303E7LU_1KEHOt_fA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: PL8K60KmZZZJlTQVgEe3kC9W5qS_9TUM-GLj3AsQD0VVIlTc3Csz2qnPdnYAkEZluDB96HvsVoGjbAka1Rcz5w
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 6cQUcQOzDFZYc5tVYibPmWypMU1vnKMYkB2Z-nxu3F2N1wcrNRpqO94kCjhWJaHLA_xkcQ7vKuVfZcekjPAOhg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 9R36PyqSzI-p5q4y114aZrdpVPr2FXT1taD9aUSQYJDUFRGiFO8kOHvuZlaYYyp18i2wxFsAZKAFn2eQYUPRHg
Created Bob: $500 - ID: 77GmdKHCJXdVa2QREJyZKuTJ8teeV8iyMMnCKLnIumBSMxPpGcXvTyEXreM9Cedbehv9Q6OEI9E9oeGi1VqgKQ

=== Example 1: Begin Transaction ===
Transaction ID: c7806726-5946-491f-bb77-a3a6ea848244

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
New transaction: 380e2dee-424f-4ff7-a610-a8280e7413ec
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("RXDrkwddTCODc1IeUfedXccN2dlNZN3JIYlw6JOhxF6srdCmunSb_-dWZwoGGOzHp4PupFcax7bsziw_Qar7xQ")} }

=== Find by ID ===
Found: Record { fields: {"name": Object({"type": String("String"), "value": String("Test Record")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("RXDrkwddTCODc1IeUfedXccN2dlNZN3JIYlw6JOhxF6srdCmunSb_-dWZwoGGOzHp4PupFcax7bsziw_Qar7xQ"), "value": Object({"type": String("Integer"), "value": Integer(42)})} }

=== Find with Query ===
Found documents: [Record { fields: {"active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("RXDrkwddTCODc1IeUfedXccN2dlNZN3JIYlw6JOhxF6srdCmunSb_-dWZwoGGOzHp4PupFcax7bsziw_Qar7xQ"), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"type": String("String"), "value": String("Test Record")})} }]

=== Update Document ===
Updated: Record { fields: {"name": Object({"value": String("Updated Record"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "id": String("RXDrkwddTCODc1IeUfedXccN2dlNZN3JIYlw6JOhxF6srdCmunSb_-dWZwoGGOzHp4PupFcax7bsziw_Qar7xQ")} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "iY_4Ud8qoe5qh3fCCT1YZFdpfAR6ElkqqINFYpUwABaQANLtz59e-iSp9Eh6OR_307rNggBUnBFCjoPjMuHYKQ"

=== List Collections ===
Total collections: 9
Sample collections: ["websocket_test", "ttl_cache", "scripts__ek0_testing", "users", "client_collection_management_rust"]

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
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "tTJK-QARo3_jIXTJIsp0nhIHcK97aPHSmxziMBmhLgSBbTtnNx4y-s0Ptb29EOZqTksRlzi-E9rAu73_RonMdQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("XXhHm6TwS1uwzuoadAOcLmmfs1xt4H-o0khqxGhR5beRWAAkW87njGIms0Qp8tATwEfIoTsKUPVBpG6iPvd6eg"))

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
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: WNoy3IBzN5dHdljlLScnFTwHYRT6IlCXmZ-BuoXngioj-b105rNlenlNpzRHq_bqbWg-NYBQqwkWo96uH8abXQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("yUBE96AyZZ2A8t8KyfHIll9HsjJBe4I3RMSYsxR9R-CCwn20kM35S3pD2kOwLl_4CLUO5P6B--QTuCjaxyrd8A"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
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
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

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
     Matched: ["title", "description", "title.value", "description.value"]
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
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("fBPT5mtvVsJ3j0u-2FqwM_0hnOvJ8UDHjjBjWjxa1zpyUWo06xbg7TDFW-NSROIM1nBZFx9MedTnLyJKGjum8Q"))
✓ Inserted user 2: Some(String("swjVqrRO40P8-WdfPfF3HAQfwAsVUeoQ2gwhN5EXzK1FKIWnD55fyIbYsJQhIS-I1rETPJYJ9LT2oOxVC9WkpA"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - age: Integer
  - email: String
    (required)
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["websocket_test", "schema_client_rust", "ttl_cache", "scripts__ek0_testing", "users"]

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
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: HWO_Ih0OA303BqPhCgWr6vgXp3n0ydpNFOkFouLtHWO6i67yp0lhJOAGkd3BR00nWokmyL7toIpPhMwWY0WFsA

=== Sending Chat Message ===
Message ID: HKz-TOs0JZR7bQglgqWH2gSsdCe3HU-aM5LKz8U3Rv1YRcwEnd2QvSHEWQ6eG3yWnAqGyB1uqViwzggSWIw0Mg

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. One of its notable features is the AI Chat Integration. This feature enables you to query your database using natural language and receive AI-powered responses with relevant context. In addition, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("K1J-2gseRTKXZx8-Y8lPEVaSA7EoDZ0GBKMUls2dT0hsKRh1ud8UdJnrR2a6tjrUd51DYzo_Zns05M78fTPV-g"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("-G-VoGLFeg6pNRRMsTANvg2iMQoAPtd1TGFj_GX-n8Hv5sODTJ89TCn_aQ49PRXENxtVsDqE6rKZh0gWI3rt_A"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("cx3uMl5KJiqVw_rUldvotzjGW6WR8slSrVmVL8JqSXRVpo7us06lIUc8b7C1r8Q8b-A_sYXei-Qukhg3M7a9cg"), "title": String("Search Features")}

Execution Time: 3639ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called ekoDB. It's a high-performance database product with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: nZ_a9sOPtVTmctloK9S6p9GELh4sWfMeaXlMPvJka0Q4y8n2BNihi4yS4lWCNGfT0CLWnG62sRjeGi8ZZjsGDg
  Parent: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: nZ_a9sOPtVTmctloK9S6p9GELh4sWfMeaXlMPvJka0Q4y8n2BNihi4yS4lWCNGfT0CLWnG62sRjeGi8ZZjsGDg (Untitled)
  Session 2: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw (Untitled)
  Session 3: HWO_Ih0OA303BqPhCgWr6vgXp3n0ydpNFOkFouLtHWO6i67yp0lhJOAGkd3BR00nWokmyL7toIpPhMwWY0WFsA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: nZ_a9sOPtVTmctloK9S6p9GELh4sWfMeaXlMPvJka0Q4y8n2BNihi4yS4lWCNGfT0CLWnG62sRjeGi8ZZjsGDg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Gu1j2Tc-IIp4pe26C6pXsVFBbnTw-JmA72g1oaBnfTcjApklLsfPHyQ7HT6DRIEVzsDnB6c7ESLCPJq9onjB4w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB. It is a high-performance database product.

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
✓ Created second session: FCexZoTUlOX6KTMmjH8vgzqs3v-JG_bq367FxD8kJTuYbPFS0SQ--AfP1uiO3tCjYL3IFjfW2XJv984i2JMbig
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
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: ECJCHT3xyhXxRd_qe7u6Xxvf7-t3zehReKLFPtsNGpsE2bGI4kJpmg4I-v_hcifq8e6rNMYyLlPpoJel_xmlyw
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 1MaDeRcjd33pOXQRJkmFTPFb1KdIzwb05eipLjXwFCfolFeK9J6TfkypxZ6XXNc_benEYmsXQF_z7Q_AdtaDTw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: zbWqlDM0zEFY0Wbx6BCy4HBfmd-mPp5lpOR6ZK658T5Htaavd6QKRbMdOKxTvpezJ3Yrek_wBz0foJm5Yi9jiA
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
Inserted: {'id': 'dYkRme2ONpAMZ0V4rqmQWSiK1shYjM7UIfJZA3E5ZaAE6slxCyJFbaGQkaKXfhKYRT69VgBF3Z6UmwKp7MQfFw'}

=== Find by ID ===
Found: {'id': 'dYkRme2ONpAMZ0V4rqmQWSiK1shYjM7UIfJZA3E5ZaAE6slxCyJFbaGQkaKXfhKYRT69VgBF3Z6UmwKp7MQfFw', 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'dYkRme2ONpAMZ0V4rqmQWSiK1shYjM7UIfJZA3E5ZaAE6slxCyJFbaGQkaKXfhKYRT69VgBF3Z6UmwKp7MQfFw', 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: IclmydnNxR1P-E0R7bBIuBKTF1Ka65Q1s32FLDl28hTOjoHAqNCeuCWCZsfDHQ4J5KOp9vNZEg0ayYtaSPXtQA

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
        "id": "IclmydnNxR1P-E0R7bBIuBKTF1Ka65Q1s32FLDl28hTOjoHAqNCeuCWCZsfDHQ4J5KOp9vNZEg0ayYtaSPXtQA",
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
        "id": "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ",
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
✓ Verified: Record updated with status="{'type': 'String', 'value': 'active'}"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'username': 'john_doe', 'userId': 123}

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
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: kdUsjAoHTxnMFqefKTxIZTNeQQQ-87MQ6i1qbFF-n7_oNPlzJO2LLuAjrXTIHMOzqfl6bw-Qj7olG0zv2yUzog

=== List Collections ===
Total collections: 11
Sample collections: ['websocket_test', 'ttl_cache', 'scripts__ek0_testing', 'users', 'test_accounts']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: P0jcOruyuqVvOIrYqsdZxTGw79msBR0MPtKr0uu7j9TOKndWLWWFYi8e0c63g9UXhaiY1RxvREpt4i4VFHNsfg

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
✓ Inserted document with TTL: E9U_dgrjTub1oBfND5qI_tgUM0hfYcg_6Sf-bAFEOCdateAknF7_0g6E2fsk79pIrDawY1YeNXYMfVotle7NmA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: keNtsEK2NmdyZD5JIq5L_V5xKnAAkiYjk7IQpqnJT6t_w8M7nyvTrBH0ZFKvtJvum61FvfAyHJaca5NjIhOIEg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Dag48tk9SacyZAMlo-jPspCCV7hjeIrf0kfr75KmqKCYyk2uxhPAgYuOqs0xiZYPThA6TMUbYt0_VL1tXuuUwg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: PJc7HNHLc8_3E87VNbziiA7NZmZBFRxlr8Y8rpc85J1uLauUTIyIG1JH89-DmqMbXiZsZGUEwjVLZtO6ook4Og
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 90, 'count': 15, 'avg_score': 50.0, 'status': 'inactive'}
   {'avg_score': 60.0, 'status': 'active', 'count': 15, 'max_score': 100}

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
Created Alice: $1000 - ID: Ql-l5GqNqbAdQPFsMFSmrHXTQ7xrD-EMRyzIAu-VOR3A3Oepa4BkOIZ04Yi3QAmZbzrDPi_4lmSi2gDKFYJLMw
Created Bob: $500 - ID: SRd1UIDt1ZoR3ByH_5C2o1NWoR_Y_4LiGZC9pi_HupevAN4aPRUmtB2ujUtaQitdeOOBfUG3wUPao52LW2D3nw

=== Example 1: Begin Transaction ===
Transaction ID: b4ead9d8-a4b7-4a59-8edc-6a6dee920fe7

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
New transaction: 16ecbb3a-2658-4617-8371-815b05309215
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ogpYjocuGnFhAj-VFLk6tQ_JcoVcRTl5M4QNEDyyO3JBWzA2f50xGUsrfscfZXJYWJcIdOCPMPsY673SHQaJ8w
Created Bob: $500 - ID: t21AE_svF9ADqTdIcQoq54WmzKYT5877WNqrYg08RtN05Ku2hMxWeXjJGRES88MsOzFD_OIh6m0a03sP27UDGg

=== Example 1: Begin Transaction ===
Transaction ID: 25ed54c6-730d-4951-a321-e764d5e4284e

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
Created Alice: $1000 - ID: hw53OjTENCv8YBSqKxLxo_Thh9bMe-Y5DbN7UEwuaIsm103S6w5z0UsO69Vx_ifoUGgPIvzkz_3oNif5h8sb5Q
Created Bob: $500 - ID: DpPKB2jwWpg8pHtooQ0EpYeLLGm-APN9VW-ixTvFDX0Iw4088kLmbgP724GZkgySTiXR62c9QZovxdyxF3B40Q

=== Example 1: Begin Transaction ===
Transaction ID: 677b95c9-0c01-4df8-8017-4d18b15bf191

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
New transaction: 6cacc338-4039-4bfd-a426-2d7fc8245e5d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CW1YK8w6bZt7fYVOCIDbjps9iMI8gXY9hFet-c94__WAAV3Uw_4Qjj53lY0DpEoysqQTqUt3Pxs60NUD15RC9g
Created Bob: $500 - ID: p6sBE4TGPArnxOa-RgGTTXxrmnkJ1VgKyIL8oP6GNcHRCQC3y66OkFUoVHVT7mTyjv4tZY59hshmTZVl68qm7Q

=== Example 1: Begin Transaction ===
Transaction ID: b3d462b7-1833-439a-8077-7d169a0f77c2

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
Created Alice: $1000 - ID: Ptd_cSzzcCk4YJOgjO05pK5IJceiXSPf9S_AMVZ7LGMhrrXqmV1TMdSlX1cE7_PAYp0luW6OmcrvMM3dcdX3UA
Created Bob: $500 - ID: HROi5yFep4rCONGqrpCkjzGe1JU9tc-UD_RkiOkpcOdifRE7Jno92AzIZbS6Wk22yR34e4M5-G4Ag1PZKfTIwQ

=== Example 1: Begin Transaction ===
Transaction ID: 96b2f3cc-cc22-4a4e-b8ed-3adc54a5da32

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
New transaction: 36beb015-5609-4350-9d15-b682cb8d08cc
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Q1sSOKcI9si96e_WfhUsH33jV-seHZxpAqNiCdXmTfKBtlN2kELmwfFXPvbJWl_8YiKqm8C50UkVvVwOWo9Wcw
Created Bob: $500 - ID: h4qreTRaLMlBGG2FceVu5prblRHVPIvIlXZQAQ97Ma2X9oJ7xbG3JJKl0hp3R_ko66LdGrmzpGkicYu6KtVKsQ

=== Example 1: Begin Transaction ===
Transaction ID: 0b3c3103-6a44-4bdf-a68b-04208e871cdb

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
   ✅ Script saved: vW5o2ivvOBx6SEsxXcs-qXWCX-LHzm609O-fCKr8Qj-NuormlDuB_FJ8e4Rq1FlXmXHBANb2wejeD_XeyGx9hA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: CU4eIM24HsufiWuQE6C3FVwhSHKveAUOm0XgA732Zs0Cdy1QtIIiReF_LfCRj2t-iFuy_o7i84OYLoTcCSwUDA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: hSFFUBZAoQetK4TZWpLyidvcXN0jIMY6oOLrP2Qq85-MYAuxNmfC_Xc_khA9OhK3dHDs9sXO1hFfE4EU5v33Vg

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
   ✅ Script saved: dn87ZG9sZxDbV689tQFlPVNplHSAzCgEH-gwCuzNBfDguVdGEPX4CIGPfzhxIH0jmOya6viJx1DLoGexDzjXzA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: YEO9mdYIDlsE1cjoOZTtEOJ8oHm8ZHzsOt8g0n3Mkgpjh3TpoSUqozplWUEPtosnXK6W7yTrN6urV2434zU7dw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: vW5o2ivvOBx6SEsxXcs-...
   ✅ Deleted script: hSFFUBZAoQetK4TZWpLy...
   ✅ Deleted script: dn87ZG9sZxDbV689tQFl...
   ✅ Deleted script: YEO9mdYIDlsE1cjoOZTt...
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
    Finished `release` profile [optimized] target(s) in 0.10s
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
Inserted: {'id': 'WtiKbYriuwdqnryTPXlPxHIRnymN_f4E4n2RcmAGtno9RMRQ2WAKa_KdmrSS4dqfPg4c0hNVI20x5KqRVJDRfQ'}

=== Find by ID ===
Found: {'id': 'WtiKbYriuwdqnryTPXlPxHIRnymN_f4E4n2RcmAGtno9RMRQ2WAKa_KdmrSS4dqfPg4c0hNVI20x5KqRVJDRfQ', 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'WtiKbYriuwdqnryTPXlPxHIRnymN_f4E4n2RcmAGtno9RMRQ2WAKa_KdmrSS4dqfPg4c0hNVI20x5KqRVJDRfQ', 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}}

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
Collection created with first record: "CTx4--hpL99g3d6QG-LMq71MMmz33USLMcstKlmB__Szg3wfEUr9FpqEVrk6DEYM6CFP7an2IgjeeK9B0PX5Dw"

=== List Collections ===
Total collections: 10
Sample collections: ['websocket_test', 'ttl_cache', 'scripts__ek0_testing', 'test_accounts', 'test_collection']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: AGxkCAS-cowEh6Bwbe3WI_GIyAQpwbn7s3AgRvIFSqxRZeFKo9BRitCFEYrwdAy_3y7mJ9Rl4O9qXvz49IEzbg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: BRqcem6UFhssRxhstGZ2bYAoXhf31LBN0ENr8KAn_3lk9DjEr-Zh5AT-2-O_Z4vj2425-SCHFMJ0BbB9h89eHg

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
✓ Inserted test record: 2nUxZa88wwhd92pccaRwW7aEkHn8mfOVmUXD3wr8Bjc9L4U0cEgMbnEalW7gfO4G2hmHAJQPxEpvxoB1RHEOqw

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
✓ Inserted document with TTL: S8hgmmy1GH_ilpbv5MLEeZazP9tXmebliOQCGY6Tarz5pZOeBp8mJbFi9KZ2CDpucGdHDUVmSkrq8N_fo0wEfA

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
  1. Score: 25.740, Matched: email.value, name.value, name, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio, title, title.value, bio.value
  4. Score: 26.400, Matched: title, bio.value, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, bio.value, title
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: bio.value, title.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.732
  2. Score: 0.728
  3. Score: 0.723

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.706, Matched: content.value, title, title.value, content
  2. Score: 1.110, Matched: content, title.value, content.value, title
  3. Score: 0.513, Matched: 

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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: JehJVzy0fGzjb5QdDQEKy5wozk9uYujxIkjq0V5YTjSRI0ad3j65tSRgHdY1djrSCKaYCyqxoZ0QuX97tozN6Q

=== Sending Chat Message ===
Message ID: F_v60-_9ZxrmR9YOoenAE0TVyklJByAkcuFne3BpAf5kLS_-yGBHHReqrfq88LkQXZkIrGeJOAXf6BQ83oeSAw

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

Execution Time: 2876ms

=== Token Usage ===
Prompt tokens: 608
Completion tokens: 84
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information I have, one product available is ekoDB, a high-performance database product. It's priced at 99 units.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: VavaZWaynMzXrFwMAuN0JYWgFdRWfVT769hoCsK6xWG5F5-VM7Cv9BkJ_Bw6_1ZTv69O5gcWR2EVjuy4Im-h7g
  Parent: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: VavaZWaynMzXrFwMAuN0JYWgFdRWfVT769hoCsK6xWG5F5-VM7Cv9BkJ_Bw6_1ZTv69O5gcWR2EVjuy4Im-h7g (Untitled)
  Session 2: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw (Untitled)
  Session 3: JehJVzy0fGzjb5QdDQEKy5wozk9uYujxIkjq0V5YTjSRI0ad3j65tSRgHdY1djrSCKaYCyqxoZ0QuX97tozN6Q (Untitled)
  Session 4: FCexZoTUlOX6KTMmjH8vgzqs3v-JG_bq367FxD8kJTuYbPFS0SQ--AfP1uiO3tCjYL3IFjfW2XJv984i2JMbig (Untitled)
  Session 5: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw (Untitled)
  Session 6: HWO_Ih0OA303BqPhCgWr6vgXp3n0ydpNFOkFouLtHWO6i67yp0lhJOAGkd3BR00nWokmyL7toIpPhMwWY0WFsA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: VavaZWaynMzXrFwMAuN0JYWgFdRWfVT769hoCsK6xWG5F5-VM7Cv9BkJ_Bw6_1ZTv69O5gcWR2EVjuy4Im-h7g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dPNKEv_dfqJL8x3sqR3r2tpGbSp43UYC7alf9KFGbpMnykDYjj3QbyVu8N4qZRHMEeI_2znmmGZjwj7e8oQcyg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product "ekoDB" is available. It is a high-performance database product and it is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['updated_at', 'created_at', 'chat_id', 'context_snippets', 'token_usage', 'id', 'role', 'content'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: w3uwW8-60AB64JP2kQHBpGnrWGOXfXp289TQ8dj3GhYDE9QI6-wPz9Ctj-OMc9v_ttISNovA-FnkX8mqAqCyCA
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

✅ Script saved: RO4ee1NjBSkvQ9zm4b4RmQTKkTukNqwk7bF1K_Sn17dCfBbBaPEzwizIUCCMNSI1SjTPRUOO9YthRYQvi9z4iQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 97iyx4o_mg9KNrI-_D9vAQR5cGE_Bhi-j0g9yBlVerx_ZbBQ6Qs2wqKhmjgCWPU5HQNFKllPD2AXRFZ4JG9ZLQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 2Q0sGYYFqUHJ_JAwK3klk-JwvT3_hGkBu7WzufwEYbRnf23fdH2C4slINQGae1p9zIvClxBNGI_l4DXw8MKbTQ
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

✅ Script saved: ilqfGx2BoJsjcxFfmC4BcuEaQgYfxdeXHpPSPuojRZOrWuwM8E2EALe_bELPytqsjAq5-OQspWL8EUaSDStMxQ
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
Inserted: map[id:Y8FKg6f9KyR7VMMK9ebVoR_aXZXCKlBnFyN1qz-g5qhB519nZYYfbXlAelousMRwnzrnqupk3fX9tSat0zjZ7Q]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:Y8FKg6f9KyR7VMMK9ebVoR_aXZXCKlBnFyN1qz-g5qhB519nZYYfbXlAelousMRwnzrnqupk3fX9tSat0zjZ7Q name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:Y8FKg6f9KyR7VMMK9ebVoR_aXZXCKlBnFyN1qz-g5qhB519nZYYfbXlAelousMRwnzrnqupk3fX9tSat0zjZ7Q name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: EHRclmzT3prR90JJIQl3JV0OnzQ_zdm5GHUZwl1JG0Dv33EQi6Elici_xRW6FSSDrWR4Yjl4f0a7s25WdNchNw

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
        "id": "IclmydnNxR1P-E0R7bBIuBKTF1Ka65Q1s32FLDl28hTOjoHAqNCeuCWCZsfDHQ4J5KOp9vNZEg0ayYtaSPXtQA",
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
        "id": "EHRclmzT3prR90JJIQl3JV0OnzQ_zdm5GHUZwl1JG0Dv33EQi6Elici_xRW6FSSDrWR4Yjl4f0a7s25WdNchNw",
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
        "id": "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ",
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
Batch insert response: map[failed:[] successful:[o-OIpgSkzcKq6rzMP7ENJOrwHRKXYoYTcPx5r2RtI5jO64iyATu6OMAMXNNPai0W1ER8HJom7eYgbhpq-3m6_A CdqAfMnNOUbuppm5Z9FCc-bSDu6BIaR5TvA7-dWy_b5ekm_mYrpH-HpckcKtLu5uUSMndxq7bs0XaXwfoNKqPQ vWGWGozqwLS2hAeB3r13TXzI_7pye9pGNm-54T5n8hqmmqyPIi53RiwbaeQYByx7rpiE57-l8Ck964Y9otSyZQ vUbjWZ5n6WIq7-gj2MA20XDxUoc2QqMeN-dBlq0cqKGQoae_L833Kk9X_7ZBXMSjzvHGTUEPPWOyZvpPxIXwog Z88usFFBqZBhsxc3sQORW2-Oim-eSqc6r7cQfwPGXbBlj-mrhgRzYfh2Q8d9CMccMqKbgW-KLbhWlVHb5U9LHw]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[BXI3_WHUuiNf9ryzxMGabatr_xZYkdLge4wgTwqIoZnhQHM89L-RdvG6spb4r3rai7kTa8WPyvi3TWOVnOSGHA kKFZnPw6UORaQ3JdRirXzJg5onRF-_ikhoSLvTgTaGMfMgwFXYjCcJ6ecFVQWwUsop2RF-sP0aDqzZAsy_HSvw VGeFUgkc6hwTIT80taX_cKL8Lfk82AiyJyQnQvOfvSiIyspy4JTg01dizRp3UYK4yLKS0T6HXUB28jGPL_G9qQ]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[BXI3_WHUuiNf9ryzxMGabatr_xZYkdLge4wgTwqIoZnhQHM89L-RdvG6spb4r3rai7kTa8WPyvi3TWOVnOSGHA kKFZnPw6UORaQ3JdRirXzJg5onRF-_ikhoSLvTgTaGMfMgwFXYjCcJ6ecFVQWwUsop2RF-sP0aDqzZAsy_HSvw VGeFUgkc6hwTIT80taX_cKL8Lfk82AiyJyQnQvOfvSiIyspy4JTg01dizRp3UYK4yLKS0T6HXUB28jGPL_G9qQ]]
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
Collection created with first record: 6yg94eSOflTCrjJWdLpAotYockSlE_gHOtFJ9Lg5pOBX5HJ71bhZitKQFx80v_Mt141JRqFvf1T0chL_c1k_EQ

=== List Collections ===
Total collections: 15
Sample collections: [websocket_test ttl_cache schema_employees_client_py scripts__ek0_testing users test_accounts test_collection chat_messages__ek0_testing demo_collection schema_products_client_py ws_ttl_test schema_documents_client_py chat_configurations__ek0_testing schema_users_client_py batch_users]

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
✓ Inserted document: OwEAdE5wlWrZ4vj1gKcg-1DM7DgSSUEkXCgD471X8ebpAyaaifqcpVRiL9GYC25yNH6gEtlJeGn_8TkzWRKcdg

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
✓ Inserted document with TTL: 3Qvlu7sRUFQe1VliCtGbZDYgPMkE888dYMeDGi6s5UxEg_3BSYikTEXITqfFh68xpuwohf97IyFWfmLtWd51VA

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

✅ Script saved: WxqBvWoFmJyDNndQtPGDFViMedUpGBtKL_m-otK66wOJbf2PopXxNZ3ms2A38AjPdxbKICmgddY67N-_m-rccA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: iOgWp9RqkpWBwZI_TdERo_tlrt1QRSxxyRQvCxGYI7xEK35X4iUwKx_K1PSKgn8jpR-6PIvepNLIdSVz87nAtA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: f39VXPsnS648G3VZuQw0EewfHyMMDKePcA6sBLobYPVeyzw-6s5rQK00xaY_W9HAN7xTZdiznYJH0m4LiFXwIQ
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
Created Alice: $1000 - ID: 6tuk1-ogCKuUnxnw0Y6kDhQx1We-dmT1dE-2sRLoewsIXsdX_bkZ2yUlzda8ZjXqr-T9RWQ2-gFnLa92V8Ct1A
Created Bob: $500 - ID: O6EZbzaPAbFJvkMH49nUjBgWUzgW4IR-W0z8sSu6OXU42OYpd74OFQ10L0GlSvvgP926kFIHIZTS_PEeAbkQNA

=== Example 1: Begin Transaction ===
Transaction ID: 0f4014cd-9bd3-49b1-a61e-9123428cca6b

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
New transaction: 866ab7c1-e5dc-4d67-aaa1-bfadba597d07
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
   ✅ Script saved: zwRK1GgTsHj_7vatX4Tij24H-Kpv796CTwxdtTnsuvpCWtYVRCRqKklOmz6JlS7kwWVig6VwxxJWLtCY9IP-ww

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: cY5mRt2XWFIrbS3JMobcVqBqnzAEfyjG427mCUD5Fgtp-3BLPnS4fCO1apWwoDOrlf2oheRKWi8a9EK3i7nykQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: MXPgwDPbJ9jAMAUSDlO5vupytBhv9IhYUHSQGOPvMpoVZE_mLWIvF-XPuZI9dz6P3idrcggJ91W2lcu5Hl1ejw

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
   ✅ Script saved: 38VAUcWDDVgsA4p9rnLJgLmVDq-gW-jSAf5K1J2zsMi2fh2Ea2e2Zx9i92jxyJjnGevLAsvQbd1fkvXwMO4MUQ

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
   ✅ Script saved: N67jrfCFhTK18SRtlBv6M_ijCjeGe5iq6AESob7B_d1pzE8fqVudQ9o4bmCmds_rhG1jdb8Va9-1cxKKFjne8g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: zwRK1GgTsHj_7vatX4Ti...
   ✅ Deleted script: MXPgwDPbJ9jAMAUSDlO5...
   ✅ Deleted script: 38VAUcWDDVgsA4p9rnLJ...
   ✅ Deleted script: N67jrfCFhTK18SRtlBv6...
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
Inserted: map[id:NhoI2JCCw_7uQdS0k5QMRTPKtqekd5E0-MgKUGfAKqhkaxKEQ0AOjsO28f_UwNSy8Ks8vMM4QUVElGiAgvs-Mw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:NhoI2JCCw_7uQdS0k5QMRTPKtqekd5E0-MgKUGfAKqhkaxKEQ0AOjsO28f_UwNSy8Ks8vMM4QUVElGiAgvs-Mw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:NhoI2JCCw_7uQdS0k5QMRTPKtqekd5E0-MgKUGfAKqhkaxKEQ0AOjsO28f_UwNSy8Ks8vMM4QUVElGiAgvs-Mw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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
Collection created with first record: OFz5NdyLxAc_EsAKBHGM7mpvws_LGoYtrkub-OdNaZW-yNxzoxF0hdMkAgGE6QxgBSmPCDC12V2453tu8i3cyg

=== List Collections ===
Total collections: 14
Sample collections: [websocket_test ttl_cache schema_employees_client_py scripts__ek0_testing test_accounts]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: pnLsOSRBwoWoaZXY43rbj-naHts_hEKguyRaJ_9rjcwTISuE94Se1Fg5RnSXkHXsxrgsdNJQxondxU8_a2O_2g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 5q0m_DYEB1lzDQ2z2Rjw8-hrpxbZJrtcK1S5gBrT7kO0wmMzSRgQtr6_tHPntiKRKGtlhuaovhre5-XmF0kxXg

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
✓ Inserted test record: pjQ1BvqnjONb7fBPOOkkvSfmQayUWIt5nln3jD6K8Ze-_5lSFTKwUpn8Ohgg2Mm3QJUU_jZMm6of2lDcQG73DQ

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
✓ Inserted document with TTL: -R1rxlBUSivVIY-kIqV49_qt1YyUnMjuaOViA2Hmjy998OhI3hAnc7leokZT73bJ59Ij8AmxOZP2nMTBikHQWQ

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
✓ Created session: BFktfJOMJ7YljgfQGEQDsMgJIZ-0uhKET-2tVlgR9fOhkzOHWvpdYvlxe2DZcxoZd1fmY8aweNcjmyR3ZbNI-A

=== Sending Chat Message ===
Message ID: o5Qa0Kucc4gTDzEw2DKsfLKLwp9Aq90swocF0OBKV0rCQxWfRVsHyrH140ptz7nuLnnsetg-TLF_XaGcJNNXCg

=== AI Response ===
The available products along with their prices are as follows:

1. ekoDB Cloud: It is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB: A high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:fRcvIiN9GQdWlg4Nds2p8bfh4xnAxl5Yd_2NP5N6qnGiVxP_XMs6dbtX8PJ8dS1o5V9EqmOVOOpndG7BBys2wg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:Yz-PUvMHTM7LNvqeCqhoUYzSglDOtPYTtP935tYuT1shI9tbElgJPeNnw6zgHSxYmrhB8YkrEj7sPZ_o1QV2fw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:SGjq3diLpx3bIGR0TDsTmVQsq2cr9FLhOxcw66pqtB4Fs7gKg_X1gWyCTKYm-WECjV3s-LbP4LatCL4EVlxEIg name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2256ms

=== Token Usage ===
Prompt tokens: 614
Completion tokens: 87
Total tokens: 701

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: CssK2C9NwDqB6gnxHjQmOEWnlPFpUh7XiAwg7Wvu-k6ZAZLAZe5IkReQusVCWlsU96NjKPvgu-n6XzcfI2kidw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product that is available is ekoDB. It is described as a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: -tnnQtOaa4IvPZEJpHUZR2YW9b8JT1MW43Gfl_uAWknkvRgu1FMJCepHu-XTfGxDMEqTyFrkTWq1OM6RTxxXTQ
  Parent: CssK2C9NwDqB6gnxHjQmOEWnlPFpUh7XiAwg7Wvu-k6ZAZLAZe5IkReQusVCWlsU96NjKPvgu-n6XzcfI2kidw

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: -tnnQtOaa4IvPZEJpHUZR2YW9b8JT1MW43Gfl_uAWknkvRgu1FMJCepHu-XTfGxDMEqTyFrkTWq1OM6RTxxXTQ (Untitled)
  Session 2: CssK2C9NwDqB6gnxHjQmOEWnlPFpUh7XiAwg7Wvu-k6ZAZLAZe5IkReQusVCWlsU96NjKPvgu-n6XzcfI2kidw (Untitled)
  Session 3: BFktfJOMJ7YljgfQGEQDsMgJIZ-0uhKET-2tVlgR9fOhkzOHWvpdYvlxe2DZcxoZd1fmY8aweNcjmyR3ZbNI-A (Untitled)
  Session 4: w3uwW8-60AB64JP2kQHBpGnrWGOXfXp289TQ8dj3GhYDE9QI6-wPz9Ctj-OMc9v_ttISNovA-FnkX8mqAqCyCA (Untitled)
  Session 5: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw (Untitled)
  Session 6: JehJVzy0fGzjb5QdDQEKy5wozk9uYujxIkjq0V5YTjSRI0ad3j65tSRgHdY1djrSCKaYCyqxoZ0QuX97tozN6Q (Untitled)
  Session 7: FCexZoTUlOX6KTMmjH8vgzqs3v-JG_bq367FxD8kJTuYbPFS0SQ--AfP1uiO3tCjYL3IFjfW2XJv984i2JMbig (Untitled)
  Session 8: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw (Untitled)
  Session 9: HWO_Ih0OA303BqPhCgWr6vgXp3n0ydpNFOkFouLtHWO6i67yp0lhJOAGkd3BR00nWokmyL7toIpPhMwWY0WFsA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: -tnnQtOaa4IvPZEJpHUZR2YW9b8JT1MW43Gfl_uAWknkvRgu1FMJCepHu-XTfGxDMEqTyFrkTWq1OM6RTxxXTQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: z7-WuIfx0TZWO_wVEJRYrpdLkAUwYQxau_EP1WTjtv_oP0ZLgYclsIc4klT8aYQoqwK26WQFBgPsixYseADITg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is the ekoDB. This is a high-performance database product.

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
✓ Created second session: EoxFsNGvDGoht1Q_yKy8wRxk6vja0w6JhYWhzZrain-YcVDqpDEU6v2mJoT9pNfcSopyQbQVaAcUDJECztvOhQ
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
  1. Score: 0.769
  2. Score: 0.756
  3. Score: 0.747

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702
  2. Score: 1.499
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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: bzAFy24MAWHkU7f68A5CI7bvK68cJ4Pz4a3OBTk1h5rIyVteMGDGeBytV1otf0AothI0WaAUrLf1u-ttF58yQg
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


up to date, audited 7 packages in 876ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 688ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'TprlvND5ZqYFy5G9T_5XBCZLX01OGtG80kgoS4AzS1GabrrSsG0xJl2wu4eiuxw2InwXpwel9ESBqmkbH7mqxw'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' },
  id: 'TprlvND5ZqYFy5G9T_5XBCZLX01OGtG80kgoS4AzS1GabrrSsG0xJl2wu4eiuxw2InwXpwel9ESBqmkbH7mqxw',
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  name: { value: 'Updated Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'TprlvND5ZqYFy5G9T_5XBCZLX01OGtG80kgoS4AzS1GabrrSsG0xJl2wu4eiuxw2InwXpwel9ESBqmkbH7mqxw'
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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: Bfzo-jFFZJG82d8kY2JqSq4Cm-3LFZheRU5J7Wnlb6ARx-Oir4EMcUKPphx2w9CMMiZvbtlVQBXbc9kiWTy9kw

=== List Collections ===
Total collections: 18
Sample collections: schema_documents_client_go,websocket_test,ttl_cache,schema_employees_client_py,schema_products_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: RZGIcq2CiiIQVXXVrlM4xhuCKwsrStv_JqsZZfQ10pGXqQzTd1t6ZqWtjTWiI6WCzhRSZ49dexFCt_prqoW8-g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: U964hmaeJMfBMqgWcVT4Evuj3j_Hi6wO_anuVl_Pbfsj6VQ7h4WPrDl2Ru_-uQBlhh4vQfUNfp6iaOhUzkBNjg

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
✓ Inserted test record: 4fwDz8KeKlkIFUxWie6dHcS9W5G8HhUvFRmpZJR2kPlR_YnFGEWqVmakJCDyxEvOmS8YQfrbFkwj33UATFuWgQ

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
✓ Inserted document with TTL: aM3-h0D5FIKno-lC0VNBZu_idQCzRFaymbScHXGKVyLEjD93yDkEOrHPwIwzwot_5hSZskGZSAZy8BmSJD3nhQ

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
✓ Created session: tyjQSPLhUkrwuy7usi4J-ACs56k7JtISW1NKu4Mc01d4HdqAS5UtDhYgML4vZfENohr1kMxZe-cYyaKgyv-kqQ

=== Sending Chat Message ===
Message ID: Kl16o9mTM6QBl3fPeJTNMzxIQUQydgWwwKQry5I0PsKURJcQSi4jubnX2bdlaNTslGBwMisEJrrD4hCUCV_IHg

=== AI Response ===
The available products and their prices are:

1. ekoDB Cloud: This is a fully managed cloud database service product and it is priced at $499.
2. ekoDB: This is a high-performance database product with AI capabilities. It has a price tag of $99.
3. ekoDB Pro: This is an enterprise edition product with advanced features. It is available for $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'vebaNzUFFc9RWte8_6j8Y1X4u2sUAXIr0nqZTjjjgNXKbKwgMjd19uBksSOo15c6xEElr9C4QvwpmtW5RKgwQA',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'JFEfrStWoRTnAYIp_XHZaPxcFV7cBPX6wt0GfON17-xR9bD4Mj0Ld02ykGpAjtr04ecF2Fmkn7kzEUyywHsBEA',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    id: 'DlkCgptWj1sIj03dMvJZiIIHEiCC9aNJCowpV5bCB9-kY_p63TIfIhk60C-ZeuSqjFyUzOAgjB6vFEKQ4qrMiA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3300ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 82
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: TE0B1tmmDPi7xvN_2zBCap9AL__HdNuTlsKjDaz76LJRSRZpDD26mE2I6II1jdl38xe6PHv29rZ104cGauOIlA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is "ekoDB". It is a high-performance database product. The price for ekoDB is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5qDuAePdPKipmjsIsZ5ov0i7-dEuGLxWNONFXYeYtYqF2NjChjGGdj0BAUfyS4u6lYCwBscAKjCwnyosnbBmcA
  Parent: TE0B1tmmDPi7xvN_2zBCap9AL__HdNuTlsKjDaz76LJRSRZpDD26mE2I6II1jdl38xe6PHv29rZ104cGauOIlA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 5qDuAePdPKipmjsIsZ5ov0i7-dEuGLxWNONFXYeYtYqF2NjChjGGdj0BAUfyS4u6lYCwBscAKjCwnyosnbBmcA (Untitled)
  Session 2: TE0B1tmmDPi7xvN_2zBCap9AL__HdNuTlsKjDaz76LJRSRZpDD26mE2I6II1jdl38xe6PHv29rZ104cGauOIlA (Untitled)
  Session 3: tyjQSPLhUkrwuy7usi4J-ACs56k7JtISW1NKu4Mc01d4HdqAS5UtDhYgML4vZfENohr1kMxZe-cYyaKgyv-kqQ (Untitled)
  Session 4: EoxFsNGvDGoht1Q_yKy8wRxk6vja0w6JhYWhzZrain-YcVDqpDEU6v2mJoT9pNfcSopyQbQVaAcUDJECztvOhQ (Untitled)
  Session 5: CssK2C9NwDqB6gnxHjQmOEWnlPFpUh7XiAwg7Wvu-k6ZAZLAZe5IkReQusVCWlsU96NjKPvgu-n6XzcfI2kidw (Untitled)
  Session 6: BFktfJOMJ7YljgfQGEQDsMgJIZ-0uhKET-2tVlgR9fOhkzOHWvpdYvlxe2DZcxoZd1fmY8aweNcjmyR3ZbNI-A (Untitled)
  Session 7: w3uwW8-60AB64JP2kQHBpGnrWGOXfXp289TQ8dj3GhYDE9QI6-wPz9Ctj-OMc9v_ttISNovA-FnkX8mqAqCyCA (Untitled)
  Session 8: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw (Untitled)
  Session 9: JehJVzy0fGzjb5QdDQEKy5wozk9uYujxIkjq0V5YTjSRI0ad3j65tSRgHdY1djrSCKaYCyqxoZ0QuX97tozN6Q (Untitled)
  Session 10: FCexZoTUlOX6KTMmjH8vgzqs3v-JG_bq367FxD8kJTuYbPFS0SQ--AfP1uiO3tCjYL3IFjfW2XJv984i2JMbig (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 5qDuAePdPKipmjsIsZ5ov0i7-dEuGLxWNONFXYeYtYqF2NjChjGGdj0BAUfyS4u6lYCwBscAKjCwnyosnbBmcA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: h-c1tRfU3JxVU0Yr00lcyT10BDn4NYC3nCDIceXpqwWmpQuwiMlumXt2Og8gjZgeKkvtg9lUPx20V8rwX4pfRA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the available context, the product "ekoDB" is available. It is described as a high-performance database product and it is priced at $99.

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
✓ Created second session: SYGB6sQ1IYqqkigD9f7pPdYxL6NQcPSn80b89Q0DIvHyTL3FNly2qLQ1q31a4xZdPvPtNT473MpSqRhsIJxrfQ
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title.value, bio, title, bio.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title.value, bio, title, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, bio.value, title
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: bio.value, bio, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.731, Matched: 
  3. Score: 0.719, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.708, Matched: content.value, title, title.value, content
  2. Score: 1.488, Matched: title, content, content.value, title.value
  3. Score: 0.293, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, bio, skills, skills.value

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

✅ Script saved: 1iuG_Dy8qAo-JnGTJFLe8DPz_AQaNWuTWjKxWoNm3SQp_Duljm9GWv0taZdXDGKbq4QiaXgs2YAt1sRMY805cA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: E-vv9T6zVKPHGxZTkm80F1CRYarPIO7_eWjjVqXeQtHDcQY9YIh0HL4YtD7HHfUH0GttLvp3M-Q-owczbQweLw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: UYZ0v10iWx7fykOm-QRK3TKHu-aiLJvNIsKgUl54oGjXcNCm6SgA2O-FgxDpZKQgs7hSbfQs-bQjsmungi7F6Q
📊 Statistics: 2 groups
   {"avg_score":60,"count":5,"status":"active"}
   {"count":5,"avg_score":50,"status":"inactive"}

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

✅ Script saved: MNXERYth0xZn0P7Lsq3GWCNOFA3kBz5dlf_snzER25f2I91DSgYJ8Yjg0ePN-Ax3smsfyf2xiftck4V31VW3cw
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"count":2,"category":"Furniture","avg_price":474}
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
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 640ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:75641) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:75644) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'HExkc71eokHYIpompSBj1e6mkszJI5d4SjOJ2D-stwBm3GTFV6Lb5pJCT_v5dtbN-ecFHgcqHv54UvHeFY-BQA'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  id: 'HExkc71eokHYIpompSBj1e6mkszJI5d4SjOJ2D-stwBm3GTFV6Lb5pJCT_v5dtbN-ecFHgcqHv54UvHeFY-BQA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' },
  id: 'HExkc71eokHYIpompSBj1e6mkszJI5d4SjOJ2D-stwBm3GTFV6Lb5pJCT_v5dtbN-ecFHgcqHv54UvHeFY-BQA',
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:75657) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: DPEkqZIh-i23QxrxTy9w5u3GHzN4R4xthmy2IGmxlMQOc7Ui8r2egTNy5jwHONuAeiAishu1GBMezwJqfwWegQ

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
        "id": "IclmydnNxR1P-E0R7bBIuBKTF1Ka65Q1s32FLDl28hTOjoHAqNCeuCWCZsfDHQ4J5KOp9vNZEg0ayYtaSPXtQA",
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
        "id": "EHRclmzT3prR90JJIQl3JV0OnzQ_zdm5GHUZwl1JG0Dv33EQi6Elici_xRW6FSSDrWR4Yjl4f0a7s25WdNchNw",
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
        "id": "DPEkqZIh-i23QxrxTy9w5u3GHzN4R4xthmy2IGmxlMQOc7Ui8r2egTNy5jwHONuAeiAishu1GBMezwJqfwWegQ",
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
        "id": "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ",
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
(node:75660) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:75674) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2025-12-31T16:20:54.765Z',
  userId: 123,
  username: 'john_doe'
}

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
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:75677) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: u5FEveCERNtg9a4Dv5rOA1H9V_2HlzVVD8p-fVJ_wqLlNqNORqPlNFca6zuwen44sx-PVzTE9qXeslgSCTI10A

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_documents_client_go',
  'websocket_test',
  'ttl_cache',
  'schema_employees_client_py',
  'schema_users_client_ts'
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
(node:75680) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: retYL8l1z7i4GWviJXvLONp-thw6XxRj3TsjIdCQADrF418OntsRn631amABxHHYMliQ7L2jsuVCTGuW_zUznQ

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
(node:75693) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: pPJxHIus6ZtR9_5hRjPWgyf9zwwcknEc4bM2V5Mase92CpxoJ9ICzEvGmjuPS47Bke6LAqngjIvQxxF9lISm1Q

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

✅ Script saved: OFuMWZq8Zoa7mWig38AowNIcs17nGXYqcb1IRSUuiD4fJLEQ_rbvXhSz9YT0DD5A-vQERfuYKhjVByapxKevqg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 5eS2EGqH7JgBg57FB1gd_o8qIVI5s_hnBWAiE3bIi9INafELjytyNcDER1vYJaK2AXvCf7AMcEQPW9zBLuY2fA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: sPcBdbgaOTwRmuvR9NUN8UKuEY-dZBiYIzggd8ZUmXfIYJ_gRQ5V1wBMGhCNTwnKwx9nI05KNj0MeKo-_agACw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":50,"max_score":90,"status":"inactive"}
   {"max_score":100,"avg_score":60,"status":"active","count":10}

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
Created Alice: $1000 - ID: M3gxo-A4kX3hYfkpSZYQZXKPCQIZAGQ6e2V32lVQz_OKzBiTJLUODbYvpdVlAJFTFGMrp0gLgQPabbi-GWb4lA
Created Bob: $500 - ID: Pe50VxMvVmQszvJ7jcNVqOk2PhGua1OrBM8Z-vO0eybTccUoCu-k_1CaYWfnGmD_F-HKE5bGkHUDxzyx0T5qLQ

=== Example 1: Begin Transaction ===
Transaction ID: 133c1327-856b-4107-a22c-78cc3afe258a

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
New transaction: bd7280a2-a17b-47af-984c-30e1909b725c
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
   ✅ Script saved: fvUS4vRJID5roMA8N5fvIIR8Cd4cre-gyeoWzPnv5TsiuuynmxLlBRn--npKyjFUXJu9dlB6tjuKSV7N0LmIRg

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
   ✅ Script saved: eqgNPlJYGPgCFhFO6_REsG8MG4eC_bx5eiKNvWP43Xr8hpb7ENfdYjzkxx4e_qIUA3hPXACO1izJ1eE--yqTyw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: AfFwn6tJcsj68aw-Dw1yP_VoYBw_ZebZ4YaTesP-J3Ijw8V_BSrgVQt7PCaUxkFDTO86lumstcNAd6PouvIlGQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Pkzastp935yfHC-kRQEcgoNHhGksx5xkNPOQ8kiMdhwspyYE85TqVioPRURv3JrIHyTVrduKQY2bV2m5lqgcPQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: fvUS4vRJID5roMA8N5fv...
   ✅ Deleted script: eqgNPlJYGPgCFhFO6_RE...
   ✅ Deleted script: AfFwn6tJcsj68aw-Dw1y...
   ✅ Deleted script: Pkzastp935yfHC-kRQEc...
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


up to date, audited 7 packages in 798ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 624ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'IcvqG-d4OGn_0p0LHfDp9t827lKUsKv9OGB1U-9MRrA3fof7KimyTTvjgGWpdcZ0S8lM83gf7SyQ4i8w0WnrFA'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' },
  id: 'IcvqG-d4OGn_0p0LHfDp9t827lKUsKv9OGB1U-9MRrA3fof7KimyTTvjgGWpdcZ0S8lM83gf7SyQ4i8w0WnrFA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  id: 'IcvqG-d4OGn_0p0LHfDp9t827lKUsKv9OGB1U-9MRrA3fof7KimyTTvjgGWpdcZ0S8lM83gf7SyQ4i8w0WnrFA',
  active: { value: true, type: 'Boolean' }
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
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 6Dm0FDIChSdIo1w7LTKnLBbN7oKm-l8AFwPGXyZSHSRxWltmdHW_vdkzcalCU5zAEo0-XKAkhwEdxIBmsSUIQg

=== List Collections ===
Total collections: 22
Sample collections: schema_documents_client_go,websocket_test,ttl_cache,schema_employees_client_py,schema_users_client_ts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: FFBL9K9IFSYpSRFCzvXW1wWAF5w41Wc5B4SK9EuCTTM2DS9PVH1-REG8NFoPsJALmIDrOwH1VpUcurY-fQebQg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: SBhFCs-muIASj8Q0wOe_cCTCmY1rs47-cNvzObXRuIPg3P5mAbPBqf-MgQQHWcjfDeJ-LO2-clxmdNQVILe-sg

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
✓ Inserted test record: PqgiUDC5XrejtedsiPNOn2PrX8NQ_Qz-uSfNCdvznHBe74Iij845NbRwQayZl3IROMkUn39tUYnzEwKhFqY4Zg

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
✓ Inserted document with TTL: 0ZQwQvshN1itz-CVzHv07JTt11GzxVJDwwRdceEI7JeuSC1biPfgGD_2VmV50ydwNTErIB2qcx9gp0paaA7Ryw

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

✅ Script saved: V2OmV7Y_UUPB233XnSHN37rXUfkcjsc3kogJmtGB_CHU_9C1wbcP1oZXZx_Q0jwBXS4wOUFPsXsKkfH7FAUvPg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: MJ31QwISP4VeaOzkR49ExNpSOaRPKzrttPTEu3e-1OW1On1UCuRItSLnQ021eo_7SIEmehp-GBy82j1dZPJl3A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: u4EMnO60ImpgKXIDiokHH8I0bppzozgMvmHihWT1xutLf-wKnYqsKpkgR4OS5Egh4vDE_AsA2qF6jnl3EyIEHQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":90,"count":5,"status":"inactive","avg_score":50}
   {"avg_score":60,"count":5,"status":"active","max_score":100}

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

✅ Script saved: N1N7I4mWd-lqpCAq7ObrRZlHkzwkRwC7HaMN7jdnWc3rs7Rqy7h0I5vEHRs64YwZq3ZAEokyup4H1c24UEBmdw
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","count":10,"avg_score":60}
   {"count":10,"status":"inactive","avg_score":50}
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
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Wireless Mouse - $29 (⭐4.5)
   2. Monitor 27" - $399 (⭐4.6)
   3. Office Lamp - $79 (⭐4.05)
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
   active: 7 users
   inactive: 3 users
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
   1. User 4 (user4@example.com) - Score: 40
   2. User 7 (user7@example.com) - Score: 70
   3. User 9 (user9@example.com) - Score: 90
   4. User 5 (user5@example.com) - Score: 50
   5. User 2 (user2@example.com) - Score: 20
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
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
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
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Natural Language Processing
   4. Database Design Principles
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
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
   1. Scalability: Vector databases can handle large amounts of data and are highly scalable.
2. High Precision: They support a high level of detail and precision, with no loss of quality when zooming in or out.
3. Flexibility: These databases can handle various types of data, including points, lines, and polygons.
4. Fast Processing: Vector databases allow for quick data processing and analysis.
5. Better Visualization: They provide better visualization options due to their ability to represent data in a graphical manner.
6. Efficient Storage: Vector databases use less disk space compared to raster databases.
7. Easy to Update: Updating data in a vector database is relatively easy since it involves changing the attributes of the vector features.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Draft Article" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
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
✓ Inserted user: Record(fields={id=StringValue(value=p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ)})
  User ID: p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ

=== Read ===
✓ Found user by ID: Record(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), id=StringValue(value=p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)})})

=== Update ===
✓ Updated user: Record(fields={id=StringValue(value=p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ)})

=== Delete ===
✓ Deleted user with ID: p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: 5ax711-g0zZYaRNQYOFsQCk1oYxDF7falmW6R79l8XPf7n86PHsTtJT4p0Dfmg_A19FGRFRC6DG6aS2j03aWgw, qu94s4iuFJRxSfMTe2s6jkatiLfBILlEB_BM0wMiicFAdbTex333OIRcYSOdGIdbxNZtV_s4_7kg1PRnc30Dfw, 8G_BnKGPp30uT9n9TKPRm-jwVD6HngYh-GEGkyrY28BAIiiHAq5qXsyxZCqhqfbrfBjFJ6kHOGXs-auALffjZQ...

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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1767198102441}
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
  - schema_documents_client_go
  - websocket_test
  - ttl_cache
  - schema_employees_client_py
  - schema_users_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-31T16:21:46.845307Z","last_modified":"2025-12-31T16:21:46.845308Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: 5vfrMrnPORxpBQ9glrtw_Hwnd6Cm1Mxmxo-2CFDJcyy1-t4LObalUDJC8DDvxCNfHvIC3utD0tSzdKCwvdyiRQ

=== Verify Document Exists ===
✓ Document found: [user_id, created_at, ttl_duration, id, session_id]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: GdckQ-D2l7BdSrP2xXHOv1Jb3AOAupmy-Z1Y-paGU-91dlGvP5qihs_eqL2GQx9-n95PmxnCgLMP1VFLXbRzrw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [cache_key, ttl_duration, id, value]

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"8xcI2zGumqpcLdoIKwynveQUUoglXk8NaH-glXcKUCd6Ozd0I7eFyYIl_k4GbcruLpCrHfoGhxiQkp_u7wH9Kw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=4osdzy62ytkXtf44VIMjB8gWi-pReJ0ytidXxNJrpC5j7FMUdwKRy3EBnuSZEioIDpMC-XLZhEj93ZUQ8rC48Q)
✓ Inserted user 2: StringValue(value=p99RB6fvpXsemtnJC6Y_do7xXuxDCcUTYrVUaf8U2XI6-IEOM1L4hy5KDX1Xm91swXc9NPTcYTZEK6PAZ_Qnkg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_documents_client_go, websocket_test, ttl_cache, schema_employees_client_py, schema_users_client_ts]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"tags":"programming,rust,tutorial","title":"Rust Programming","views":166,"description":"Learn Rust programming language with hands-on examples and best practices.","id":"kJ_-dooEl6BA2D5_VBfuho6g-VxDWoGIGAVO0qxDOIR9wfbkm3cpWEI7aslDaoQVf6iYvUICUUFK2zymCZ1BXg"},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"tags":"programming,python,data-science","id":"FQD2wA29uVAH-hTW249hR2WDOkhMaSyZWvqf_CFuMAyYJvIQrPNUahD60FdbXsfn7vNnLKej0WNUPCL3GAwNnQ","description":"Master Python for data analysis, machine learning, and visualization.","views":944,"title":"Python for Data Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","views":109,"id":"2zuyHw0lrKegqkFq4wtKZrXb3dRcLiFN-C6TPeSjjqs6D8hQNM22kTo03UpdvlUObv3-QuL8RqbAPG0If0v4vA","title":"JavaScript Web Development","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":"ai,machine-learning,python","title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","id":"Gu78MI6tNv9fdfN23HAUERR0bCYKYK7--FlN1k8nIgM7bDRv7LLqxktzwIfCM2Rhww69kT8tYltG8v_HPZ7HYg","views":505},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","views":944,"id":"FQD2wA29uVAH-hTW249hR2WDOkhMaSyZWvqf_CFuMAyYJvIQrPNUahD60FdbXsfn7vNnLKej0WNUPCL3GAwNnQ","title":"Python for Data Science"},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"database,design,sql","views":958,"description":"Learn database design principles, normalization, and query optimization.","title":"Database Design","id":"UIJ5g90F9kaGySXrFFeZjr3l6PrEK0PDFsZGAFKx-Myec5HwQM_XEhOGUA005NZDece7w-GlnjyIMGRlF5PAAw"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"kJ_-dooEl6BA2D5_VBfuho6g-VxDWoGIGAVO0qxDOIR9wfbkm3cpWEI7aslDaoQVf6iYvUICUUFK2zymCZ1BXg","views":166,"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=BHGPVMx3ziYui5tdwEEfwr4M7f7vk29yfJj1zEbczmqYtYJExi0mblEzPZH5hlBy7cQif_Cf41l9uES3AB5bWw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767198128450},"id":"BHGPVMx3ziYui5tdwEEfwr4M7f7vk29yfJj1zEbczmqYtYJExi0mblEzPZH5hlBy7cQif_Cf41l9uES3AB5bWw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Created session: XhpBPYD5xKIGjrz74UBzkNMucxgd_0CV0XplH5Ia_Iee37DWov-zqptaGc1RucE5Ms7jFADdrn4XibIIvOOwZg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "kBzSC9fniUyYmjvMG0Hwe2AsQ0IhLnwKwEZ_4mItuN_hgYcma4WyT5H9w3T6Zyuc1MA4n6M-1xlgq_PKEqIqqw"
  Responses: ["ekoDB is a high-performance database that is designed with intelligent caching, real-time capabilities, and AI integration. It offers several unique features such as:\n\n1. AI Chat Integration: This feature allows you to query your database using natural language and get AI-powered responses with relevant context.\n\n2. Search Features: ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval, making data searches efficient and contextually relevant."]

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
✓ Created session: pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Answer: The product available is \"ekoDB\", a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"VqRaDRb71FbsLLfRWDfFrmBaYC_AULM38UEoNrm35dR-AfE5OqgEtR2SZ29LoHHpDzFNDD851CgfeygG7sBMgg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-31T16:22:24.935673Z"},"id":"lx2OkVGpCDDmOtbpUr0Z2P9muT1bg237oxPw3LaQ0Y31NP98RPl6c5ZpsSNxmfiiqarJiZEE45JF3Mc8KrGUSw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":235,"total_tokens":261}},"updated_at":{"type":"DateTime","value":"2025-12-31T16:22:24.935673Z"}},{"chat_id":{"type":"String","value":"pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ"},"content":{"type":"String","value":"Answer: The product available is \"ekoDB\", a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"VqRaDRb71FbsLLfRWDfFrmBaYC_AULM38UEoNrm35dR-AfE5OqgEtR2SZ29LoHHpDzFNDD851CgfeygG7sBMgg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-31T16:22:24.938569Z"},"id":"y8vIAgwhHZv8ZIi03GkKSRBIoxRard7D0caDODhbeGN_95venpL-asKwiY-BuMX2bObKXNMmLAaZmIEi9O-a6A","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":235,"total_tokens":261}},"updated_at":{"type":"DateTime","value":"2025-12-31T16:22:24.938569Z"}},{"chat_id":{"type":"String","value":"pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"VqRaDRb71FbsLLfRWDfFrmBaYC_AULM38UEoNrm35dR-AfE5OqgEtR2SZ29LoHHpDzFNDD851CgfeygG7sBMgg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-31T16:22:25.672782Z"},"id":"eZw6q_Dzb5GXSiDufUeq1oJQ_vNuohFsAYf6quoAVcyxQELCV4UL4Bwvf74iG3Y0EdQb4HzKTUd9TP5bG-1DVg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":235,"total_tokens":247}},"updated_at":{"type":"DateTime","value":"2025-12-31T16:22:25.672782Z"}},{"chat_id":{"type":"String","value":"pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"VqRaDRb71FbsLLfRWDfFrmBaYC_AULM38UEoNrm35dR-AfE5OqgEtR2SZ29LoHHpDzFNDD851CgfeygG7sBMgg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-31T16:22:25.676416Z"},"id":"x_o6BODQlPFM3gjHf1OMPC88T2PSRxuVL8mqUACBao8TVZj5SQXJyaDyjxpYXVnhk91sqw0SiqUIT-9O8oM6Xw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":235,"total_tokens":247}},"updated_at":{"type":"DateTime","value":"2025-12-31T16:22:25.676416Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: tM_iDzK9Wzrc7tXgZ75UJWg2qDmvlaGc2OaDrShVYpgLSlvQcls81Ik4La2D6w4TS6qGXGhM2Prww_GyhmajtQ

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 5s
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
✓ Created session: i6SLBNbQGQC3KHDnxsDJ_C_7m7CyqaXwjxHs8cw-zuCeyXWJPXJ359Vk7HH17BhUEqbOs1xhMJxNnZ_4_AjA5g

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the provided context, the available product is ekoDB, a high-performance database product. It is priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the available context, the product that is available is a high-performance database product named \"ekoDB\". It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: -OB-lUza4K56Q1JHnvRlFGh_pOedrl-mUI2kGVi7iIYtqwu5-8NeaYu4HYYlUy1EOVAsO2zQ5lNwIxDexndubA

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

✅ Script saved: y4F3uBlBnsepWUCPRPawRp3lms3UUPRDMNd_W72CqxY3yD3mTv_few7g92r103dXIRgznD517rzlECb6ejXVIw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"status":"active","avg_score":60.0}
   {"avg_score":50.0,"count":5,"status":"inactive"}
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
    ✓ Generated embedding: 1536 dimensions in 365.475ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 346.206875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 546.901125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.026774917s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 991.867333ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.312477583s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.035720667s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.015554958s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 215.696875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 224.172584ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 964.769583ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 551.804834ms
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
    ✓ Generated embedding: 1536 dimensions in 184.080416ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 105.089083ms

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

Writing memory-safe, high-performance database code involves a number of best practices and considerations across multiple areas such as query optimization, database schema design, and coding practices. Here are some strategies you can follow:

1. **Query Optimization**: Always ensure your queries are as efficient as possible. This can involve using appropriate indexing, avoiding unnecessary columns in SELECT statements, and avoiding complex joins where possible. Remember, SQL is better for complex queries, so if you're using NoSQL, you might need to structure your data differently.

2. **Normalization**: As mentioned in the context, database normalization is a process that organizes data to reduce redundancy and improve data integrity. This can also improve performance, as you're reducing the amount of redundant data that your queries need to process.

3. **Appropriate Use of SQL or NoSQL**: Based on your application's needs, choose the appropriate type of database. As discussed earlier, NoSQL is suitable for flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

4. **Memory Management in Code**: When interacting with a database in your code, always ensure that you're managing memory effectively. This includes closing database connections when they are no longer needed and being mindful of how much data you're loading into memory at once. If you're querying large amounts of data, consider using techniques like pagination or streaming to handle the data in smaller chunks.

5. **Concurrency Control**: When multiple threads are attempting to write to the database at the same time, you'll need to implement proper concurrency control mechanisms to avoid data inconsistencies and to ensure memory safety.

6. **Caching**: Keeping frequently accessed data in memory can drastically reduce the need for expensive database queries, improving performance and reducing memory usage over time.

7. **Use Prepared Statements**: Prepared statements not only help in preventing SQL injection attacks, but they also improve performance as the database server can cache the query plan for a prepared statement and reuse it when the statement is executed again.

8. **Database Configuration**: Adjusting the database's configuration settings can improve its performance. For instance, increasing buffer pool size, log file size, or adjusting the query cache type and size in MySQL. 

Remember, it's important to maintain a balance between memory safety and high performance. Always test your code thoroughly under different scenarios to ensure it behaves as expected.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 339.750167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2732 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 229.041667ms
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
  ✓ Text search completed in 61.892875ms

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
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.691s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.216s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.119s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.392s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.236s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.436s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.356s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.715s
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
    ✓ Generated embedding: 1536 dimensions in 0.559s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.083s

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

Writing memory-safe, high-performance database code involves several practices:

1. **Optimize Your Queries:** As mentioned in Context 5, optimizing your database queries can significantly improve the performance of your database code. This includes using indexes effectively, avoiding full table scans, reducing the amount of data transferred, limiting the use of certain types of joins, and avoiding nested queries where possible.

2. **Normalization:** As discussed in Context 1, database normalization can help improve data integrity and reduce redundancy. This can also help to improve the performance of your database code by reducing the amount of duplicate data that needs to be processed.

3. **Appropriate Database Choice:** As per Context 2, choosing the right type of database (SQL vs NoSQL) depending on the use-case can also affect the performance and memory usage. SQL databases are better for handling structured data with well-defined relationships and complex queries, while NoSQL databases are better for unstructured data and scenarios requiring high write throughput and horizontal scaling.

4. **Memory Management:** In the context of coding, memory-safe means that the code does not cause memory leaks or other types of memory-related issues. This involves properly closing database connections, using appropriate data structures, freeing up memory space after it is no longer needed, and handling exceptions properly to avoid crashes that could lead to memory leaks.

5. **Concurrency Control:** Implementing proper concurrency control mechanisms can help to avoid conflicts and ensure data consistency, which can improve performance in multi-user or multi-process environments.

6. **Caching:** Implementing a caching mechanism can also help to improve the performance of database code. Caching involves storing the results of common queries in memory so that the same queries can be answered more quickly in the future.

7. **Batch Processing:** Where applicable, use batch processing to handle large amounts of data. This is more efficient than processing each record individually.

8. **Profiling and Monitoring:** Regularly profile and monitor your database performance. This can help to identify bottlenecks and other performance issues, which can then be addressed to improve the overall performance of your database code.

Remember, the different approaches should be tailored to the specific needs of your project and the particular database system you are using.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.333s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2496 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.378s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.254s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.301s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.336s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.232s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.458s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.305s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.397s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.408s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.398s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.415s
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
    ✓ Generated embedding: 1536 dimensions in 0.605s
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
  1. [Score: 0.000] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.000] From conv_database_design
     What is database normalization?

  5. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe high-performance database code involves a combination of best practices from both database management and programming. Here are some tips:

1. **Optimize your queries**: As stated in Context 1, optimizing your database queries is crucial for high-performance code. This includes techniques such as using indexes effectively, limiting the amount of data retrieved with each query, and avoiding unnecessary queries. 

2. **Choose the right database system**: Depending on your needs (as outlined in Context 2 and 3), you may need to choose either a SQL or NoSQL database system. SQL is generally better for structured data and complex queries, while NoSQL provides advantages for unstructured data or high write throughput.

3. **Database Normalization**: As per Context 4 and 5, Database normalization can also help improve performance by reducing redundancy and improving data integrity. This can also contribute to memory safety, as it ensures that your database operations are working with the most accurate data possible.

4. **Memory Management in Programming**: In terms of memory safety, this is more related to the programming language you use to interact with the database. Some languages like C/C++ require manual memory management, which can lead to issues like memory leaks if not handled correctly. Using a language with automatic garbage collection, like Java or Python, can help mitigate these issues.

5. **Concurrency Control**: Proper handling of concurrent transactions can also improve performance and memory safety. Techniques like locking, versioning, or using ACID transactions can help maintain data integrity when multiple users are accessing the database.

6. **Use Prepared Statements**: These can increase performance and help protect against SQL injection attacks, improving both efficiency and security.

7. **Caching**: Implementing a cache for frequently accessed data can greatly speed up your application and reduce database load.

Remember, writing high-performance, memory-safe code is an iterative process that involves continuous monitoring and optimization. Each application is unique, so it's important to profile and benchmark your specific use case.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.349s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2210 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
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
  ✓ Text search completed in 0.052s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe high-performance database code involves a combination of best practices from both database management and programming. Here are some tips:

1. **Optimize your queries**: As stated in Context 1, optimizing your database queries is crucial for high-performance code. This includes techniques such as using indexes effectively, limiting the amount of data retrieved with each query, and avoiding unnecessary queries. 

2. **Choose the right database system**: Depending on your needs (as outlined in Context 2 and 3), you may need to choose either a SQL or NoSQL database system. SQL is generally better for structured data and complex queries, while NoSQL provides advantages for unstructured data or high write throughput.

3. **Database Normalization**: As per Context 4 and 5, Database normalization can also help improve performance by reducing redundancy and improving data integrity. This can also contribute to memory safety, as it ensures that your database operations are working with the most accurate data possible.

4. **Memory Management in Programming**: In terms of memory safety, this is more related to the programming language you use to interact with the database. Some languages like C/C++ require manual memory management, which can lead to issues like memory leaks if not handled correctly. Using a language with automatic garbage collection, like Java or Python, can help mitigate these issues.

5. **Concurrency Control**: Proper handling of concurrent transactions can also improve performance and memory safety. Techniques like locking, versioning, or using ACID transactions can help maintain data integrity when multiple users are accessing the database.

6. **Use Prepared Statements**: These can increase performance and help protect against SQL injection attacks, improving both efficiency and security.

7. **Caching**: Implementing a cache for frequently accessed data can greatly speed up your application and reduce database load.

Remember, writing high-performance, memory-safe code is an iterative process that involves continuous monitoring and optimization. Each application is unique, so it's important to profile and benchmark your specific use case.

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
    ✓ Generated embedding: 1536 dimensions in 0.209s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.312s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.235s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.346s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.312s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.248s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.448s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.219s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.710s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.479s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.308s
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
    ✓ Generated embedding: 1536 dimensions in 0.279s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.082s

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

Writing memory-safe, high-performance database code requires a combination of good programming practices, understanding of the database system you're working with, and effective use of database features. Here's a few general tips:

1. Understand your database: Whether you're using SQL or NoSQL, understanding how your database handles data, its limitations, and its strengths is key. Use the database's features to their fullest. For example, SQL databases usually provide mechanisms for transaction management, indexing and stored procedures, which can greatly enhance performance and safety if used correctly.

2. Use appropriate data structures: The choice of data structures can significantly affect memory and performance. For example, using arrays where a hash map would be more efficient can lead to performance issues.

3. Normalize your database: As discussed in Context 1, database normalization can help to reduce redundancy and improve data integrity, which indirectly can lead to better performance and safer code.

4. Optimize your queries: As per Context 5, optimizing your database queries can significantly improve your database performance. This can include techniques such as avoiding full table scans, using indexes effectively, and minimizing the use of complex joins where possible.

5. Handle Errors Gracefully: Ensure your code is able to handle database errors gracefully. This will not only make your code more robust, but also safer.

6. Use Connection Pooling: Connection pools can greatly enhance the performance of your database operations. They reduce the overhead of establishing a new connection for every database operation.

7. Memory Management: Be mindful of memory management in your programming language of choice. For languages where the developer has control over memory allocation and deallocation, be sure to handle memory correctly to avoid leaks or overflows.

8. Use ACID Properties: If you are using SQL, take advantage of ACID properties (Atomicity, Consistency, Isolation, Durability) to maintain data integrity and consistency.

Remember, the approach might differ depending on whether you're using SQL or NoSQL, as discussed in Context 3. SQL is generally better for structured data with well-defined relationships, while NoSQL can be a better choice for unstructured data, or situations requiring high write throughput or horizontal scaling.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2394 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
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
  ✓ Text search completed in 0.053s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.329s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.223s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.327s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.257s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.432s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.295s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.464s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.603s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.267s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.525s
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
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.104s

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
    ✓ Generated embedding: 1536 dimensions in 0.387s
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
  ✓ Text search completed in 0.041s

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


up to date, audited 7 packages in 820ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (AywApl3riaW5GTdw9S001ZTkYLDIdG6g0Pf7NK5lnL0y_8QAjgP1aKHmmritX6TQN2XA5hLrk_7769asSYZPnQ)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "id": "gUu4BGHE250V1t71FxZtacPPiRco-X5_QKsylKZpo668ZGkgiEgTGYUwbcYDhs6CwIa93_teqxjjSr1bfa4pZw",
      "cached_at": "2025-12-31T16:24:11.595Z",
      "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269089,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}"
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
Response time: 76ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (381-S3dN3HKQjfrDcytVajVz444ssJo0e6uo0x5JL7b_cNd88wPzvCs04acMzbp_JqGY9oRFHpfGbBRkh6dkqA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2025-12-31T16:24:11.849Z",
      "id": "8uAVeekHkfvtVC4glbsJahggJ3VfZjcEECGWw9A4w0mVGbmajiw13xl9b5N3GqRNxS2tWYsTiOgHP016eFaw4A",
      "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}"
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
✓ Edge cache script created: NFCVzRCIEKIm6JFZv1B6YyXD7oQPevXcApwmSSaVrgBtEzBXKrxqc7YFNvSg34Ca-hDyjef7yHHKAS--WUq5yg

Call 1: Cache miss (fetches from API)
Response time: 566ms
Result: {
  "records": [
    {
      "cached_at": "2025-12-31T16:24:12.330Z",
      "id": "d4g3FRvQwHph-qd0S99F0CSNy9OzC6SOJCqxMjSuMUrvceNpJB1a98-2Rumz53n_O11fEUboATmyb_KNYhRpUQ",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.02872943878173828,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2025-12-31T16:15\",\"interval\":900,\"temperature_2m\":-0.7}}"
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
Response time: 538ms (1.1x faster!)
Result: {
  "records": [
    {
      "id": "sst_T8g_zx_uOQ0v267Ewi8EH7U0Zr1qUrsskaetKeYohkbV12S03IuUCTTCcqEY1EKqq1IG8XrR2YqEy-XzqA",
      "data": "{\"latitude\":40.710335,\"longitude\":-73.99309,\"generationtime_ms\":0.02658367156982422,\"utc_offset_seconds\":0,\"timezone\":\"GMT\",\"timezone_abbreviation\":\"GMT\",\"elevation\":32.0,\"current_units\":{\"time\":\"iso8601\",\"interval\":\"seconds\",\"temperature_2m\":\"°C\"},\"current\":{\"time\":\"2025-12-31T16:15\",\"interval\":900,\"temperature_2m\":-0.7}}",
      "cached_at": "2025-12-31T16:24:12.330Z"
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
    Finished `release` profile [optimized] target(s) in 0.08s
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
✓ Created SWR script: fetch_github_user (0YUO2x7VzLZ5vZRqwJURYcUBeioiP7xEoHO7e7rOiqveZAYXFuA9C4PSlesbM6PZK8UXFUkyy9NAzMwHp0Y-6A)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 81ms
Result: [
  {
    "cached_at": "1767198255",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269089,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "NeRV8_ZfEW_qG1zoYw153oClyHhMENFq0uFDwuVmrLynzJUG93WnhIbxZ60S9ysi1vDNWuCnmcCnbI4Z41fA8Q"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 90ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (Xt0dXkO6g9zKFMp-uCB9Etlyy07JQ7RXHaHiOYlXMKsHjgZJVKSXDX8Ej7i6BOpIJFNNOmmSRLCfJzmVwVnYBg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767198255",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "lSZQJk32g9_7ZCPm3mPvocW0ZJSfhfoCqJPL4_OIoXYyEQJ4ALGJCa7pRws_C0Pz-zkk78bmttk0RYtbJi4o1g"
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
✓ Created SWR script: fetch_github_user (rhDIC4xMMTVavI5DNYEMVUk1GN1wGYul11D9nU0EvenJ9ZklICTEEj-A17E60JCpuUptFVseE6Pb0iEwM551Fw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 95.802958ms
Result: [
  {
    "cached_at": "1767198255",
    "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269089,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
    "id": "bsDNaZkSwVZlZaI8t1WHUIaP7QQT-370Yxpwqo8QGQ1DMWL2frT_6xSTiQHpC6ibsyiXklC_0_kCMkCShNYB7w"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 89.712875ms (1.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (P1Qd_MolEvJXrPZh2f_Daf_5iWuYvsUQUF6i5mIudLz1dcmqzmwgLo48s0vrSkaohwtPxh1Agoz3MWrNZuMIBA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "2025-12-31T11:24:16-05:00",
    "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
    "id": "AtkHPORU5XD7jdKsG5qWrskbC7-zi5SDnWtPl8W6fJdiaM42JAe7AtZIs70M5B48zoI9apBo6ZGsiu4-7Ac2oA"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (NeZ3fiHrMHT-BwMxlVeIIxLyuFugxCHtqe_rCgC2JckI50dkl2w_rUSHmzpt_02PySzlHaKpomfDBghXiDrwrQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 86ms
Result: {
  "data": "{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269089,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}",
  "cached_at": "1767198255",
  "id": "mNSS1etyKf0N6T4Nl_q5riekm1k3CyHEavIhKr-Q9iKHRZ59Y-2U5FO9ogQXwgDwcCszc0ufl4Cw_g4r22dcew"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 88ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (vdq_S3X7PjmGTxIND2QEaLs6HhrdvlRUo3knXAt8R6dR7iXVxxYVjWXKjxKttAV2d_dac4B_yhp3GHK4iQkV-w)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": "{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}",
  "id": "WoP8IPShKv2SaqEccHuTjImchjCBjIBgDwdZYyLqMyUqmRV-m-xJ0GiUy7Dk1kAm93_bmv2vLOn4C60a9dNC0w"
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
✓ Created SWR script: fetch_github_user (EF1iBTodZcV-qDbSCIwB3TwC6SoOTCq5KGDkybQii2mcZy1AZJcpu1Sflx0GJbM8ixdesyCEk9v8oY-2EOyImA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 89ms
Result: [{"cached_at":"1767198255","data":"{\"login\":\"torvalds\",\"id\":1024025,\"node_id\":\"MDQ6VXNlcjEwMjQwMjU=\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/1024025?v=4\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/torvalds\",\"html_url\":\"https://github.com/torvalds\",\"followers_url\":\"https://api.github.com/users/torvalds/followers\",\"following_url\":\"https://api.github.com/users/torvalds/following{/other_user}\",\"gists_url\":\"https://api.github.com/users/torvalds/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/torvalds/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/torvalds/subscriptions\",\"organizations_url\":\"https://api.github.com/users/torvalds/orgs\",\"repos_url\":\"https://api.github.com/users/torvalds/repos\",\"events_url\":\"https://api.github.com/users/torvalds/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/torvalds/received_events\",\"type\":\"User\",\"user_view_type\":\"public\",\"site_admin\":false,\"name\":\"Linus Torvalds\",\"company\":\"Linux Foundation\",\"blog\":\"\",\"location\":\"Portland, OR\",\"email\":null,\"hireable\":null,\"bio\":null,\"twitter_username\":null,\"public_repos\":9,\"public_gists\":1,\"followers\":269089,\"following\":0,\"created_at\":\"2011-09-03T15:26:22Z\",\"updated_at\":\"2025-11-24T04:16:14Z\"}","id":"97uTJ472vxJoXQjg9BXOcgPLqXYon-Xa1G8Au5GzmNt0BNQT6q9pho465vHsocO5v1nhx2P0e2jmDYxIDMZs8g"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 79ms
Cache hit was 1.1x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (w3g0SxvBdh4xF0BnwO1M189LJJCjdAsxhZW2JF4TVPDqrWzzcgJcbiH7UnJrciZgxv7d4rKRKuYpkIEEPso1Kw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":"{\"id\":1,\"title\":\"Essence Mascara Lash Princess\",\"description\":\"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.\",\"category\":\"beauty\",\"price\":9.99,\"discountPercentage\":10.48,\"rating\":2.56,\"stock\":99,\"tags\":[\"beauty\",\"mascara\"],\"brand\":\"Essence\",\"sku\":\"BEA-ESS-ESS-001\",\"weight\":4,\"dimensions\":{\"width\":15.14,\"height\":13.08,\"depth\":22.99},\"warrantyInformation\":\"1 week warranty\",\"shippingInformation\":\"Ships in 3-5 business days\",\"availabilityStatus\":\"In Stock\",\"reviews\":[{\"rating\":3,\"comment\":\"Would not recommend!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"},{\"rating\":4,\"comment\":\"Very satisfied!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Lucas Gordon\",\"reviewerEmail\":\"lucas.gordon@x.dummyjson.com\"},{\"rating\":5,\"comment\":\"Highly impressed!\",\"date\":\"2025-04-30T09:41:02.053Z\",\"reviewerName\":\"Eleanor Collins\",\"reviewerEmail\":\"eleanor.collins@x.dummyjson.com\"}],\"returnPolicy\":\"No return policy\",\"minimumOrderQuantity\":48,\"meta\":{\"createdAt\":\"2025-04-30T09:41:02.053Z\",\"updatedAt\":\"2025-04-30T09:41:02.053Z\",\"barcode\":\"5784719087687\",\"qrCode\":\"https://cdn.dummyjson.com/public/qr-code.png\"},\"images\":[\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp\"],\"thumbnail\":\"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp\"}","id":"fdFam-P943NBjXafHYXtEOFX16cBhLiNtXvziRdgCVdhywKGPPX-XPIBCSvSDdDFh0RHGVfufnA13A5i8iUQTg"}]
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


BUILD SUCCESSFUL in 3s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.75s
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
   Name: Some(Object({"type": String("String"), "value": String("User 1")}))
   Department: Some(Object({"type": String("String"), "value": String("engineering")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 92.273958ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 86.268208ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: None
   Department: None
   Record count: Some(Integer(1))

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ All composition examples completed!
✅ [32mRust function composition examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 832ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 80ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 83ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: undefined
   Department: undefined
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
✅ [32mTypeScript function composition examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.07s
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 79.5ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 91.8ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: null
   Department: null
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 82.795583ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 85.048ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: null
   Department: null
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 87ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: undefined
   Department: undefined
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


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

