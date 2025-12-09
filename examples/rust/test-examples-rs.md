make test-examples-rust 🧪 [36mRunning Rust examples (direct
HTTP/WebSocket)...[0m Finished `dev` profile [unoptimized + debuginfo] target(s)
in 0.10s Running `target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("dhOOtWqhD_rFGIGyi9UnXaDCF35Ib8Fyge2_iNUv7Q2aoTKDrG1AR8e6P6_V63ra5QeSIx3qqfOpSNNDjdpV7A")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("dhOOtWqhD_rFGIGyi9UnXaDCF35Ib8Fyge2_iNUv7Q2aoTKDrG1AR8e6P6_V63ra5QeSIx3qqfOpSNNDjdpV7A"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("dhOOtWqhD_rFGIGyi9UnXaDCF35Ib8Fyge2_iNUv7Q2aoTKDrG1AR8e6P6_V63ra5QeSIx3qqfOpSNNDjdpV7A"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("dhOOtWqhD_rFGIGyi9UnXaDCF35Ib8Fyge2_iNUv7Q2aoTKDrG1AR8e6P6_V63ra5QeSIx3qqfOpSNNDjdpV7A"),
"name": Object {"type": String("String"), "value": String("Updated Record")},
"value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized
+ debuginfo] target(s) in 0.08s Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"ChsE4hhtNg8BkHvjsfyGVK_RCbVHQx8IaoOx1Vo0vx5l_0yuETorylmHzekR4I61txQOtNqmD1ntnmwC4zO4Nw"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ChsE4hhtNg8BkHvjsfyGVK_RCbVHQx8IaoOx1Vo0vx5l_0yuETorylmHzekR4I61txQOtNqmD1ntnmwC4zO4Nw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.09s Running `target/debug/examples/batch_operations` ✓
Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/kv_operations` ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: Object {"userId": Number(123), "username":
String("john_doe")}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: Object {"name": String("Product 1"),
"price": Number(29.99)} cache:product:2: Object {"name": String("Product 2"),
"price": Number(39.989999999999995)} cache:product:3: Object {"name":
String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.08s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"W87MEHvTLJHZcpJmswqvjzGvo8Ta5YyAooOfP8gkZ64UmBV1Ys7Uk1cBWkjStNB8H\_\_uQgCOrzI42Rej_ZRtjw"

=== List Collections === Total collections: 4 Sample collections:
["test_collection", "demo_collection", "websocket_test", "batch_users"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"BqOmf2l6NkYyqiualTGHVL1JHH0-8wCZnazGi52cI95DthS380XzVCDwHPDL2EC8evZzQa-hfQIX1PWySsslEA"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("5PY1faZP6u2NUUGhStuthfyTyKiPJuiWxaFfLO_akbwAO0gAeKPdYyq1AUqcJ_OugwbikmhepzFj37a-ppi-BQ")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully warning: function
`complex_filter_function` is never used --> examples/http_functions.rs:206:10 |
206 | async fn complex_filter_function() -> Result<(),
Box<dyn std::error::Error>> { | ^^^^^^^^^^^^^^^^^^^^^^^ | = note:
`#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ekodb-examples` (example "http_functions") generated 1 warning
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/http_functions` 🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
qG6WltbvbX6s_zgkIAtU3bGU6vljI7ba3qaUkAVpUB3lEudYpKZk2IL-GDflkJNHzA1s37gqt37dnZ64kX3phw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
"4vXYOFYSNrROaAGa23H1to2RcVRIwIZTcL4tWboMbgBuEiKmncKIo0AXzVGl2v2EmY_Ew46woxY-luTa0Id-Hw"
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
xvgLG4Evw7NN9bPyw6PbMGjuqXatrgdvd4HZXwQUUp4wsY7_uAzMSAhy5oiY29N13ac_8Bh1AqgwjJAWbbzkJQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}
{"avg_score":60.0,"count":5,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 3 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.10s Running `target/debug/examples/transactions` ✓ Authentication
successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
WCsHaANZe2serZq8feQupb9usDNxAzPpaiam_I21PLWQHMajcGsU1gDA1U0Unj51HqDMR4KJlwrjPGMmmK17dw
Created Bob: $500 - ID:
t5TcTrYemsTjv-KjCOD-pTQcD-s563BIu8RAdjOi22g1Hs-px3lfwCW2Bv0lfRTBf1dXskCCvzrzTnMtsd56xA

=== Example 1: Begin Transaction === Transaction ID:
da989f9d-2cc0-4a16-82d5-21966339a498

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: "Active" Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"type":"Integer","value":700}

=== Example 5: Rollback === New transaction:
e17be52e-ebd9-4046-abfb-55629f7bb14f Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed ✅ [32mRust direct examples complete![0m 🛠️
[36mBuilding client library...[0m cargo build -p ekodb_client Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s ✅ [32mClient build
complete![0m 🧪 [36mRunning Rust client library examples...[0m Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("DBmHbbG5uifFBbGqYbEwmSBXxPRTN47rk1TcUJOv190ZzHO96wCpVmWey5qdzGy9L-3-7fyUOUpW2DSelNTpag")}
}

=== Find by ID === Found: Record { fields: {"active": Object({"type":
String("Boolean"), "value": Boolean(true)}), "id":
String("DBmHbbG5uifFBbGqYbEwmSBXxPRTN47rk1TcUJOv190ZzHO96wCpVmWey5qdzGy9L-3-7fyUOUpW2DSelNTpag"),
"name": Object({"type": String("String"), "value": String("Test Record")}),
"value": Object({"value": Integer(42), "type": String("Integer")})} }

=== Find with Query === Found documents: [Record { fields: {"name":
Object({"type": String("String"), "value": String("Test Record")}), "id":
String("DBmHbbG5uifFBbGqYbEwmSBXxPRTN47rk1TcUJOv190ZzHO96wCpVmWey5qdzGy9L-3-7fyUOUpW2DSelNTpag"),
"active": Object({"value": Boolean(true), "type": String("Boolean")}), "value":
Object({"type": String("Integer"), "value": Integer(42)})} }]

=== Update Document === Updated: Record { fields: {"id":
String("DBmHbbG5uifFBbGqYbEwmSBXxPRTN47rk1TcUJOv190ZzHO96wCpVmWey5qdzGy9L-3-7fyUOUpW2DSelNTpag"),
"name": Object({"type": String("String"), "value": String("Updated Record")}),
"active": Object({"value": Boolean(true), "type": String("Boolean")}), "value":
Object({"type": String("Integer"), "value": Integer(100)})} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized
+ debuginfo] target(s) in 0.10s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_kv_operations` ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: Object {"userId": Number(123), "username":
String("john_doe")}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: Object {"name": String("Product 1"),
"price": Number(29.99)} cache:product:2: Object {"name": String("Product 2"),
"price": Number(39.989999999999995)} cache:product:3: Object {"name":
String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"KzKFoSQXphAgx36BhKjxK2fjOp3TUAze9eSLaMA6iZJYWJ9PTmxb5l2GRDK0h2JfUQYnSJEaK9uaPfkEuAhSZw"

=== List Collections === Total collections: 9 Sample collections:
["test_collection", "ws_ttl_test", "ttl_cache", "websocket_test",
"scripts__ek0_bench"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.13s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"cMtSDKS9T1KoeJ-qsLlUghMKB_D1_PzeSs7FNcnSVmoQz2LLXx8pyEQ9voXBGJNCThCz2AKAcUxJ3Q1BKq0Byw"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("ReAhOTGZqKWLQh7G1gjttKV8F11D8zPc09IVUxmYBVdSya2SYmQbQRmwONv04zfF3J6HmpI0BXptjaDDS9KX6w"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
4FtYjKcOCvlBvM5qB6TEQI7UHxQen2njoV4ApX7LYNj0hN0jhmNYxj-y2XN1v7n5gSHrH57jFnzYq7-szK29vA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("A01YtYBYMqlG-6ZHMR3f39ywxQw5n7qOx3GZ6B9I0lM4luC6BkP7y0FHfMmvqGJV-rQv4L5prEgNa8iQewD3Dg"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/examples/client_query_builder` === ekoDB Query Builder
Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"value": String("Eve"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("David")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"type": String("String"), "value": String("Bob")}))
2. Some(Object({"value": String("David"), "type": String("String")}))
3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator === ✓ Found 4 users NOT inactive

=== Complex Chained Query === ✓ Found 3 users with all conditions

=== Cleanup === ✓ Deleted collection

✓ All query builder operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_search` === ekoDB Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Found 3 results for 'programming'

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("Python for Data Science"))
3. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Fuzzy Search === ✓ Found 3 results for 'progamming' (typo)

1. Score: 1.6500 - Some(String("Rust Programming"))
2. Score: 0.5500 - Some(String("Python for Data Science"))
3. Score: 0.5500 - Some(String("JavaScript Web Development"))

=== Field-Specific Search === ✓ Found 4 results in title/description

1. Score: 4.0000 Title: Some(String("Machine Learning Basics")) Matched:
   ["title.value", "title", "description.value", "description"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description.value", "description"]
3. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description", "description.value"]
4. Score: 1.0000 Title: Some(String("Database Design")) Matched: ["description",
   "description.value"]

=== Weighted Search === ✓ Found 2 results with field weights

1. Score: 23.1000 - Some(String("Python for Data Science"))
2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options === ✓ Found 1 results with stemming

1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit === ✓ Limited to 2 results (requested 2)

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup === ✓ Deleted collection

✓ All search operations completed successfully Execution time: 2ms Finished
`dev` profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("IBnU75j14AX0rZrOOdKwyNl1BgmyHUJTPkHP2z14l7ZERlnI0-6D8pgH826os_3xdfWZpfhyuPzQQMK-fL8m_g"))
✓ Inserted user 2:
Some(String("lWxKRJaIaftdF-uKJUzQQSe7CICXIxUiQPfO26EIDHa0FyXiSrpVFP_EGhTBXkaRD-V4uI228dmUJEAiRmiCnA"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- title: String (required)
- age: Integer
- status: String
- email: String (required)

=== Listing Collections === ✓ Total collections: 9 Sample: ["test_collection",
"ws_ttl_test", "ttl_cache", "websocket_test", "scripts__ek0_bench"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_joins` === ekoDB Rust Client - Join Operations
Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Bob Smith: Sales
- Alice Johnson: Engineering

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
pya1NJ3sd_plhrdNkxjV2UR64G7fwYPWoMMYOLD9nKmXIYUpiHjhWAWx9vDsNgnjfiUhdknf53UZWjg0-GZM8Q

=== Sending Chat Message === Message ID:
6mUbAnWWP7xySzErHBfLJFLyMnn5sEB6dq1wvFvpZXW0Kxf1A12w--OLr5s6wYDROi1OXtgN9lY6qm_lBFTz-Q

=== AI Response === Response 1: I'm sorry, but the context provided doesn't seem
to include any specific information about ekoDB or its features. Could you
provide additional details or context?

=== Context Used (3 snippets) ===

Snippet 1: Collection: client*chat_basic_rust Score: 0.6333 Matched Fields:
["content", "category", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("aR*-YWMJxfsMMocF_HftJzsh8H5IeQB1yqwEKOS0QStZ59KG-QNGFSZ3rzVsTuxCQK33g7OdcKGg7ITH11WQ5A"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["category", "content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("dg9lDAysuqoWyQSgEfkuSHYRSw-Gf8rDJbsmqdwZCw5x0_YaIdu-EsFX8qovy_gKI9LCS5r5-dLEMjurpYsVKg"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("ANEo73JoIvE_hw0LKIy96Buwwmd_9iVu6tPu9zFg5x_yVZHibwbxGBZFOej5foZaRTKra4YvZxmyg42AfQnSIA"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Execution Time: 1746ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.10s Running `target/debug/examples/client_chat_sessions` ===
ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
Ols-WO2aHQOC7MeahkVVK-T9Tc-ADE9GzB9ZhrbriEtpGVorP3vLjzQyHBQ_eURfu6VZqSeNJB5cO1Ge2zKbGg
=== Sending Messages === ✓ Message 1 sent Response: Apologies for the
inconvenience, but the context provided does not include specific information
about any available products. Could you please provide more details or specify
the type of product you're interested in?

✓ Message 2 sent Response: Apologies, but the context provided does not include
any information about the price. Could you please provide more details or
specify which product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
D0AXRCt_BDhCQLvFj_OcguQ6qvLyHxhYnl1yCK2eQgEvMht1H7hUygEpobfgTATt8qnw9IF685Gu1CKjqLIIrA
Parent:
Ols-WO2aHQOC7MeahkVVK-T9Tc-ADE9GzB9ZhrbriEtpGVorP3vLjzQyHBQ_eURfu6VZqSeNJB5cO1Ge2zKbGg

=== Listing Sessions === ✓ Found 3 sessions Session 1:
D0AXRCt_BDhCQLvFj_OcguQ6qvLyHxhYnl1yCK2eQgEvMht1H7hUygEpobfgTATt8qnw9IF685Gu1CKjqLIIrA
(Untitled) Session 2:
Ols-WO2aHQOC7MeahkVVK-T9Tc-ADE9GzB9ZhrbriEtpGVorP3vLjzQyHBQ_eURfu6VZqSeNJB5cO1Ge2zKbGg
(Untitled) Session 3:
pya1NJ3sd_plhrdNkxjV2UR64G7fwYPWoMMYOLD9nKmXIYUpiHjhWAWx9vDsNgnjfiUhdknf53UZWjg0-GZM8Q
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
D0AXRCt_BDhCQLvFj_OcguQ6qvLyHxhYnl1yCK2eQgEvMht1H7hUygEpobfgTATt8qnw9IF685Gu1CKjqLIIrA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
II_0e-ST7101df8Llt2jXRrlC0NaIYztnpvpt4fnqreoJWYa61XKxcczpdTh0owcs9tz3mM2T07oPh1i9zi4BA

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't contain specific information about any available
products. Could you please provide more details or specify the type of products
you're interested in?

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: Apologies, but the provided context does not include
information on the price of ekoDB. I recommend checking the official website or
contacting the seller for the most accurate and up-to-date pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
Y9oeEuI0Dmbz-mFP_WrCEsZDbKYoKGSpek0DIqaEDWl5ia3L5pUftCBVIKBw4dtX5sMlhoqqhoK5Dj-TltDRgg
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_functions` === ekoDB Rust Client - Scripts Example
===

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
p71bhspCeVh_y7rxLwV_0DODan3UtHtjqpJsoa3eZvjtc6ZaDMjxZ0eQnIr\_\_dpxUtnLQNXBQH4tG4duIm3dzg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
70mgZLd7nqYym7mpi5T9m3lVxNobCL447lB3yEf5rCyOpXIck0okWXJUZtOYX1rM4optyf1JCErqyI2RfSt4Sg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
zi5_XmAPbt4abXIvxUq70qG_rfz-P-EtvPGsx7DLfVRSFN-i7fei53qICxVHYR9KR9Hkv9kr_Q8Bo182Fyw3Fg
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 5 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! ✅ [32mRust client examples complete![0m
